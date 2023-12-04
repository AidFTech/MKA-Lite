import sys
import os
import time

import pygame as pg
from pygame import Rect
from IBus import *

import IBus
import IBusHandler
import MainMenuHandler as main
import SettingsMenuHandler as settings
import PhoneConnectScreen as phoneconnect
import PongLoopHandler

import threading

sys.path.append("./mirror")
import mirrordisplay

window_width = 800
window_height = 480

period = 0.1

class BMirror:
	def __init__(self):
		pg.init()
		pg.display.set_mode((window_width, window_height), pg.FULLSCREEN)
		pg.mouse.set_visible(False)
		
		self.display_surface = pg.display.get_surface()
		
		self.period_time = time.time()
		
		self.run = True	#True if the program is running.
		
		self.colors = ColorGroup()	#The active color profile. More of an AIBus carryover.
		self.colors.main_font = pg.font.Font('ariblk.ttf', 32)	#The font to use for the menu.
		self.active_menu = main.MainMenu(self.colors, self)	#The active menu screen (e.g. main menu, settings, phone connection).

		self.airplay_conf = open('airplay.conf','rb').read()	#A configuration file to be sent to the dongle.
		self.oem_logo = open('BMW.png', 'rb').read()	#The Android Auto icon to be sent to the dongle.
		self.icon_120 = open('BMW_icon.png', 'rb').read()	#A Carplay icon to be sent to the dongle.
		self.icon_180 = open('BMW_icon.png', 'rb').read()	#A Carplay icon to be sent to the dongle.
		self.icon_256 = open('BMW_icon.png', 'rb').read()	#A Carplay icon to be sent to the dongle.
		
		self.autoplay = True	#Determines whether the phone mirroring begins automatically when a phone is connected. This may need to change for wireless use.
		self.selected = False	#Determines whether the Pi is selected as the audio source.
		self.control = False	#Determines whether the Pi is under BMBT control.
		
		self.carplay_name = ""	#The name of the Carplay device.
		self.android_name = ""	#The name of the Android Auto device.
		
		self.carplay_connected = False	#Determines whether Carplay is connected.
		self.android_connected = False	#Determines whether Android Auto is connected.
		
		self.night = False	#Determines whether night mode is active.
		self.time_24h = False	#Determines whether 24h mode is active (on the IKE).
		self.RLS_connected = False	#Determines whether an RLS is connected.
		self.light_thresh = 4	#The night mode threshold for when an RLS is connected.
		
		self.time_clock = ""	#The set time from the IKE.
		self.date = ""	#The set date from the IKE.
		
		self.gt_version = 0	#The detected version of the nav computer. When it receives the first message with sender ID 0x3B, the Pi will send the query out.
		
		self.mirror = mirrordisplay.MirrorDisplay(self)	#The phone mirror display object.
		connect_thread = threading.Thread(target=self.mirror.startDongle, args=(0x1314, 0x1520))	#The connection thread.
		connect_thread.start()
		
		self.ibus_handler = IBusHandler.IBusHandler(self, 0xED)	#The IBus handler. Currently mostly used for sending messages.
		self.ibus_thread = threading.Thread(target=self.ibus_handler.loop)	#The IBus handler thread.
		self.ibus_thread.start()

		self.pong_looper = PongLoopHandler.PongLoopHandler(self.ibus_handler)	#A thread that sends the "pong" messages from the CD changer and VM.
		cd_pong_thread = threading.Thread(target=self.pong_looper.loopCD)
		cd_pong_thread.start()
		vm_pong_thread = threading.Thread(target=self.pong_looper.loopVM)
		vm_pong_thread.start()
		
		self.sendAnnouncement()
	
	#Loop function, to run while the Pi is running.
	def loop(self):
		if self.active_menu is not None:
			self.active_menu.displayMenu(self.display_surface)
		
		self.run = self.handleEvents()
		
		time.sleep(1.0/60)
	
	#Interpret IBus messages read by the handler.
	def handleIBusMessage(self, ib_data):
		cmd_pass = False
		if hasattr(self.mirror, "decoder"):
			if self.mirror.decoder is not None and not self.mirror.decoder.player.window_minimized:
				cmd_pass = True
				
		if ib_data.size() < 4:
			return
		
		if ib_data.data[3] == 0x1 and (ib_data.data[2] == 0xED or ib_data.data[2] == 0xBF or ib_data.data[2] == 0xFF): #Ping
			self.ibus_handler.sendPong(ib_data.data[0], 0xED)
		elif ib_data.data[3] == 0x1 and ib_data.data[2] == 0x18: #Ping to CD changer.
			self.ibus_handler.sendPong(ib_data.data[0], 0x18)
		elif ib_data.data[3] == 0x4F: #Change in screen control.
			if(ib_data.data[4]&0x01) != 0:
				self.control = True
		elif ib_data.data[0] == 0xF0 and self.control: #From BMBT.
			if ib_data.data[3] == 0x49: #Selection knob turn.
				num_turns = ib_data.data[4]&0xF
				for i in range(0,num_turns):
					if (ib_data.data[4]&0x80) == 0:
						if not cmd_pass:
							if self.active_menu.selected < len(self.active_menu.options):
								self.active_menu.selected += 1
							else:
								self.active_menu.selected = 1
						else:
							self.mirror.sendCommand(100)
					else:
						if not cmd_pass:
							if self.active_menu.selected > 1:
								self.active_menu.selected -= 1
							else:
								self.active_menu.selected = len(self.active_menu.options)
						else:
							self.mirror.sendCommand(101)
			elif ib_data.data[3] == 0x48: #Button press.
				button = ib_data.data[4]&0x3F
				state = (ib_data.data[4]&0xC0) >> 6
				
				if button == 0x5 and state == 0x0: #Selection knob.
					if not cmd_pass:
						self.active_menu.makeSelection(self.active_menu.selected)
					else:
						self.mirror.sendCommand(104)
						self.mirror.sendCommand(105)
				elif button == 0x34 and state == 0x2: #Menu button released.
					self.control = False
				#elif button == 0x20: #Select button.
				#	if state == 0x0:
				#		if cmd_pass:
				#			self.mirror.sendCommand(106)
				#		else:
				#			if isinstance(self.active_menu, settings.SettingsMenu):
				#				self.openMainMenu()
				#			else:
				#				self.openSettingsMenu()
				#	elif state == 0x1 and cmd_pass:
				#		self.openMainMenu()
				#		self.mirror.decoder.setWindow(False)
				elif cmd_pass:
					if self.selected and button == 0x14 and state == 0: #Direction/pause button.
						self.mirror.sendCommand(203)
					elif button == 0x08: #Phone button.
						if state == 0:
							self.mirror.sendCommand(106)
						elif state == 1:
							self.mirror.sendCommand(200)
				#TODO: Audio button.
			elif ib_data.data[3] == 0x47: #"Soft" button press.
				button = ib_data.data[5]&0x3F
				state = (ib_data.data[5]&0xC0) >> 6
				
			#	if button == 0xF: #Select button.
			#		if state == 0x0:
			#			if cmd_pass:
			#				self.mirror.sendCommand(106)
			#			else:
			#				if isinstance(self.active_menu, settings.SettingsMenu):
			#					self.openMainMenu()
			#				else:
			#					self.openSettingsMenu()
			#		elif state == 0x1 and cmd_pass:
			#			self.openMainMenu()
			#			self.mirror.decoder.setWindow(False)
		elif ib_data.data[0] == 0x68: #From radio.
			if ib_data.data[2] == 0x18 and ib_data.data[3] == 0x38: #CD request.
				if ib_data.data[4] == 0: #Request current CD and track status.
					if self.selected:
						self.sendCDStatusMessage(2)
					else:
						self.sendCDStatusMessage(0)
				elif ib_data.data[4] == 0x1: #Stop playing.
					self.selected = False
					self.sendCDStatusMessage(0)
					self.mirror.sendCommand(202)
				elif ib_data.data[4] == 0x2 or ib_data.data[4] == 0x3: #Start playing.
					self.selected = True
					self.sendCDStatusMessage(2)
					self.mirror.sendCommand(201)
				elif ib_data.data[4] == 0xA:
					if ib_data.data[5] == 0x0: #Next track.
						self.mirror.sendCommand(204)
					elif ib_data.data[5] == 0x1: #Previous track.
						self.mirror.sendCommand(205)
					
					self.sendCDStatusMessage(2)
				else: #Default response to the radio.
					if self.selected:
						self.sendCDStatusMessage(2)
					else:
						self.sendCDStatusMessage(0)
			elif (ib_data.data[3] == 0x37 or ib_data.data[3] == 0x33) and self.control: #Radio menu enable message. Must be disabled.
				self.control = False #TODO: Expand!
		elif ib_data.data[0] == 0xD0: #From LCM.
			if ib_data.data[3] == 0x5B and not self.RLS_connected:
				last_night = self.night
				self.night = (ib_data.data[4]&0x01) != 0
				if self.night != last_night:
					self.mirror.setDayNight(self.night)
		elif ib_data.data[0] == 0x80: #From IKE.
			if ib_data.data[3] == 0x15:
				self.time_24h = (ib_data.data[5]&0x01) == 0
			elif ib_data.data[3] == 0x24:
				if ib_data.data[4] == 0x01:
					try:
						self.time_clock = bytes(ib_data.data[6:ib_data.size()-1]).decode('utf-8')
					except:
						pass
				elif ib_data.data[4] == 0x02:
					try:
						self.date = bytes(ib_data.data[6:ib_data.size()-1]).decode('utf-8')
					except:
						pass
		elif ib_data.data[0] == 0xE8: #From RLS.
			if not self.RLS_connected:
				self.RLS_connected = True
			if ib_data.data[3] == 0x59:
				last_night = self.night
				self.night = ((ib_data.data[4]&0x01) != 0) and ((ib_data.data[4]>>4) < self.light_thresh)
				if self.night != last_night:
					self.mirror.setDayNight(self.night)
		elif ib_data.data[0] == 0x3B: #From navigation computer.
			if ib_data.data[3] == 0xA0: #Version message.
				version_data0 = ib_data.data[15]
				version_data1 = ib_data.data[16]

				version = bytearray([version_data0, version_data1])
				self.gt_version = int(version.decode())
			elif ib_data.data[3] == 0x4E: #Ensure the radio is enabled.
				if (ib_data.data[4]&0x1) != 0x0:
					self.ibus_handler.activateRadio()
			
			if self.gt_version <= 0:
				self.ibus_handler.sendVersionQuery(0x3B)

	#Send AIBus, er... IBus messages to change the text on the screen.
	def sendAIBusText(self, cmd, text):
		if self.gt_version >= 5:
			index = 0x40
			if cmd==0x61: #Song name.
				index = 0x41
			elif cmd==0x62: #Artist name.
				index = 0x42
			elif cmd==0x63: #Album name.
				index = 0x43
			else:
				return
			
			text_message = IBus.AIData(8+len(text))

			text_message.data[0] = 0x68
			text_message.data[1] = text_message.size() - 2
			text_message.data[2] = 0x3B
			text_message.data[3] = 0xA5
			text_message.data[4] = 0x63
			text_message.data[5] = 0x1 #TODO: Adjust as per BlueBus?
			text_message.data[6] = index

			try:
				text_message.data[7:7+len(text)] = bytes(text, 'utf-8')
			except:
				for i in range(0,len(text)):
					try:
						text_message.data[7+i] = bytes(text[i], 'utf-8')
					except:
						text_message.data[7+i] = bytes('*', 'utf-8')

			text_message.data[text_message.size()-1] = getChecksum(text_message)

			self.ibus_handler.writeIBusMessage(text_message)
			
			update_message = IBus.AIData(8)
			
			update_message.data[0] = 0x68
			update_message.data[1] = update_message.size()-2
			update_message.data[2] = 0x3B
			update_message.data[3] = 0xA5
			update_message.data[4] = 0x63
			update_message.data[5] = 0x01
			update_message.data[6] = 0x00
			update_message.data[7] = getChecksum(update_message)
			
			self.ibus_handler.writeIBusMessage(update_message)
		else:
			index = -1
			if cmd==0x61: #Song name.
				index = 0
			elif cmd==0x62: #Artist name.
				index = 1
			elif cmd==0x63: #Album name.
				index = 2
			else:
				return
			
			#if index == 2:
			#	text += '\x06'*8
			
			text_message = IBus.AIData(8+len(text))

			text_message.data[0] = 0x68
			text_message.data[1] = text_message.size() - 2
			text_message.data[2] = 0x3B
			text_message.data[3] = 0x21
			text_message.data[4] = 0x60
			text_message.data[5] = 0x0
			text_message.data[6] = index | 0x40

			try:
				text_message.data[7:7+len(text)] = bytes(text, 'utf-8')
			except:
				for i in range(0,len(text)):
					try:
						text_message.data[7+i] = bytes(text[i], 'utf-8')
					except:
						text_message.data[7+i] = bytes('*', 'utf-8')

			text_message.data[text_message.size()-1] = getChecksum(text_message)
			
			self.ibus_handler.writeIBusMessage(text_message)
			
			update_message = IBus.AIData(8)
			
			update_message.data[0] = 0x68
			update_message.data[1] = update_message.size()-2
			update_message.data[2] = 0x3B
			update_message.data[3] = 0xA5
			update_message.data[4] = 0x60
			update_message.data[5] = 0x01
			update_message.data[6] = 0x00
			update_message.data[7] = getChecksum(update_message)
			
			self.ibus_handler.writeIBusMessage(update_message)
		
	#Handle keyboard events. Carryover from the test program.
	def handleEvents(self):
		events = pg.event.get()
		for e in events:
			if e.type == pg.QUIT:
				return False
		
		return self.run

	#Set the connected phone type. This can trigger a light on the BMBT?
	def setPhoneType(self, phone_type):
		if phone_type == 3:
			self.carplay_connected = True
		elif phone_type == 5:
			self.android_connected = True
		else:
			self.carplay_connected = False
			self.android_connected = False
			self.carplay_name = ""
			self.android_name = ""

		self.mirror.setDayNight(self.night)
		#TODO: Alert the MKIV that a phone is connected.
	
	#Set the connected phone name.	
	def setPhoneName(self, phone_name):
		if self.carplay_connected:
			self.carplay_name = phone_name
		if self.android_connected:
			self.android_name = phone_name

	#Open the main MKA-Lite menu.
	def openMainMenu(self):
		self.active_menu = main.MainMenu(self.colors, self)
	
	#Open the MKA-Lite settings menu.
	def openSettingsMenu(self):
		self.active_menu = settings.SettingsMenu(self.colors, self)
		
	#Open the MKA phone connection screen.
	def openPhoneConnectScreen(self, phone):
		self.active_menu = phoneconnect.PhoneScreen(self.colors, self, phone)
		
	#Set the text in the title header in the audio screen.
	def sendGTIBusTitle(self, text):
		if self.gt_version < 4:
			title_message = IBus.AIData(7+len(text))
			
			title_message.data[0] = 0x68
			title_message.data[1] = title_message.size()-2
			title_message.data[2] = 0x3B
			title_message.data[3] = 0x23
			title_message.data[4] = 0x62
			title_message.data[5] = 0x30
			title_message.data[6:6+len(text)] = bytes(text, 'ascii')
			title_message.data[title_message.size()-1] = getChecksum(title_message)
			
			self.ibus_handler.writeIBusMessage(title_message)

			update_message = IBus.AIData(8)
			
			update_message.data[0] = 0x68
			update_message.data[1] = update_message.size()-2
			update_message.data[2] = 0x3B
			update_message.data[3] = 0xA5
			update_message.data[4] = 0x62
			update_message.data[5] = 0x01
			update_message.data[6] = 0x00
			update_message.data[7] = getChecksum(update_message)
			
			self.ibus_handler.writeIBusMessage(update_message)
		else:
			title_message = IBus.AIData(8+len(text))
			
			title_message.data[0] = 0x68
			title_message.data[1] = title_message.size()-2
			title_message.data[2] = 0x3B
			title_message.data[3] = 0x21
			title_message.data[4] = 0x62
			title_message.data[5] = 0x01
			title_message.data[6] = 0x40
			title_message.data[7:7+len(text)] = bytes(text, 'ascii')
			title_message.data[title_message.size()-1] = getChecksum(title_message)
			
			self.ibus_handler.writeIBusMessage(title_message)
			
			update_message = IBus.AIData(8)
			
			update_message.data[0] = 0x68
			update_message.data[1] = update_message.size()-2
			update_message.data[2] = 0x3B
			update_message.data[3] = 0xA5
			update_message.data[4] = 0x62
			update_message.data[5] = 0x01
			update_message.data[6] = 0x00
			update_message.data[7] = getChecksum(update_message)
			
			self.ibus_handler.writeIBusMessage(update_message)
		
	#Send the announcement message. May be redundant with the pong_looper object.
	def sendAnnouncement(self):
		announce_message = IBus.AIData(6)
		
		announce_message.data[0] = 0xED
		announce_message.data[1] = announce_message.size()-2
		announce_message.data[2] = 0xBF
		announce_message.data[3] = 0x02
		announce_message.data[4] = 0x01
		announce_message.data[5] = getChecksum(announce_message)
		
		self.ibus_handler.writeIBusMessage(announce_message)
	
	#Send the VM control message.
	def sendVMControl(self, power):
		vm_message = IBus.AIData(7)
		vm_message.data[0] = 0xED
		vm_message.data[1] = vm_message.size()-2
		vm_message.data[2] = 0xF0
		vm_message.data[3] = 0x4F
		if power:
			vm_message.data[4] = 0x11
			vm_message.data[5] = 0x11
		else:
			vm_message.data[4] = 0x12
			vm_message.data[5] = 0x11 #TODO: Double-check this byte.
		vm_message.data[6] = getChecksum(vm_message)
		
		self.ibus_handler.writeIBusMessage(vm_message)
	
	#Send the CD status message 0x39.
	def sendCDStatusMessage(self, status):
		cd_message = IBus.AIData(16)

		pseudo_status = 0x89
		if status == 0:
			pseudo_status = 0x82

		cd_message.data[0] = 0x18
		cd_message.data[1] = cd_message.size()-2
		cd_message.data[2] = 0x68
		cd_message.data[3] = 0x39
		cd_message.data[4] = status
		cd_message.data[5] = pseudo_status
		cd_message.data[6] = 0x00
		cd_message.data[7] = 0x20
		cd_message.data[8] = 0x00
		cd_message.data[9] = 0x01
		cd_message.data[10] = 0x01
		cd_message.data[11] = 0x00
		cd_message.data[12] = 0x01
		cd_message.data[13] = 0x01
		cd_message.data[14] = 0x01
		cd_message.data[15] = getChecksum(cd_message)
		
		self.ibus_handler.writeIBusMessage(cd_message)

class ColorGroup:
	br = (40, 32, 95)
	text_color = (191, 191, 239)
	header_color = (103, 95, 143)
	rect_color = (239, 96, 32)
	border_color = (215, 215, 239)
	border_outline = (0, 0, 0)
	
	main_font = None

if __name__ == "__main__":
	bmirror = BMirror()
	while bmirror.run:
		bmirror.loop()

	pg.quit()
