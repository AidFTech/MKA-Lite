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

window_width = 720
window_height = 480

period = 0.1

class BMirror:
	def __init__(self):
		pg.init()

		pg.display.set_mode(size=(window_width, window_height), flags=pg.FULLSCREEN)
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

		self.audio_screen_open = False	#True if the MKIV audio screen is open.
		
		self.carplay_name = ""	#The name of the Carplay device.
		self.android_name = ""	#The name of the Android Auto device.

		self.app_name = "" #The name of the audio app currently running.
		self.song_name = "" #The name of the song.
		self.artist_name = "" #The name of the artist.
		self.album_name = "" #The name of the album.
		
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
		
		self.ibus_handler = IBusHandler.IBusHandler(self, self.mirror, 0xED)	#The IBus handler. Currently mostly used for sending messages.
		self.ibus_thread = threading.Thread(target=self.ibus_handler.loop)	#The IBus handler thread.
		self.ibus_thread.start()

		self.pong_looper = PongLoopHandler.PongLoopHandler(self.ibus_handler)	#A thread that sends the "pong" messages from the CD changer and VM.
		cd_pong_thread = threading.Thread(target=self.pong_looper.loopCD)
		cd_pong_thread.start()
		vm_pong_thread = threading.Thread(target=self.pong_looper.loopVM)
		vm_pong_thread.start()
	
	#Loop function, to run while the Pi is running.
	def loop(self):
		if self.active_menu is not None:
			self.active_menu.displayMenu(self.display_surface)
		
		self.run = self.handleEvents()
		
		time.sleep(1.0/60)
	
	#Interpret radio IBus messages.
	def handleRadioMessage(self, ib_data):
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
				
				self.sendHeaderText() #TODO: Send this only if the audio screen is active.
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
		elif (ib_data.data[3] == 0x23 or ib_data.data[3] == 0x21) and self.selected: #Headerbar text change message.
			if bytes("TR",'utf-8') in bytes(ib_data.data) and bytes("-",'utf-8') in bytes(ib_data.data):
				self.sendHeaderText()
		elif ib_data.data[3] == 0x46: #Send metadata.
			if (ib_data.data[4]&0x2) == 0 and self.selected:
				self.sendMetadata()
				self.audio_screen_open = True
			else:
				self.audio_screen_open = False


	def handleIKEMessage(self, ib_data):
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

	#Set the song and artist name.
	def setMetadata(self, cmd, text):
		last_app_name = self.app_name

		if cmd == IBusHandler.SONG_NAME:
			self.song_name = text
		elif cmd == IBusHandler.ARTIST_NAME:
			self.artist_name = text
		elif cmd == IBusHandler.ALBUM_NAME:
			self.album_name = text
		elif cmd == IBusHandler.APP_NAME:
			self.app_name = text
			if self.app_name != last_app_name:
				self.artist_name = ""
				self.song_name = ""
				self.album_name = ""

		if self.audio_screen_open:
			self.sendMetadata()

	#Send the text to be displayed to the header.
	def sendHeaderText(self):
		if self.android_connected:
			self.ibus_handler.sendGTIBusTitle("Android")
		elif self.carplay_connected:
			self.ibus_handler.sendGTIBusTitle("Carplay")
		else:
			self.ibus_handler.sendGTIBusTitle("MKA")

	def sendMetadata(self):
		last_data = 1
		if self.artist_name:
			last_data = 2
		if self.album_name:
			last_data = 3
		if self.app_name:
			last_data = 4
		
		if self.song_name:
			self.ibus_handler.sendRadioText(self.song_name, IBusHandler.SONG_NAME, last_data == 1)
		else:
			self.ibus_handler.sendRadioText(" ", IBusHandler.SONG_NAME, last_data == 1)

		if self.artist_name:
			self.ibus_handler.sendRadioText(self.artist_name, IBusHandler.ARTIST_NAME, last_data == 2)
		else:
			self.ibus_handler.sendRadioText(" ", IBusHandler.ARTIST_NAME, last_data == 2)

		if self.album_name:
			self.ibus_handler.sendRadioText(self.album_name, IBusHandler.ALBUM_NAME, last_data == 3)
		else:
			self.ibus_handler.sendRadioText(" ", IBusHandler.ALBUM_NAME, last_data == 3)
		
		if self.app_name:
			self.ibus_handler.sendRadioText(self.app_name, IBusHandler.APP_NAME, last_data == 4)
		else:
			self.ibus_handler.sendRadioText(" ", IBusHandler.APP_NAME, last_data == 4)
		

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

	def endVMControl(self):
		self.control = False

		menu_press = IBus.AIData(6)

		menu_press.data[0] = 0xF0
		menu_press.data[1] = menu_press.size() - 2
		menu_press.data[2] = 0xFF
		menu_press.data[3] = 0x48
		menu_press.data[4] = 0x34
		menu_press.data[5] = IBus.getChecksum(menu_press)

		self.ibus_handler.writeIBusMessage(menu_press)

		menu_release = IBus.AIData(6)

		menu_release.data[0] = 0xF0
		menu_release.data[1] = menu_release.size() - 2
		menu_release.data[2] = 0xFF
		menu_release.data[3] = 0x48
		menu_release.data[4] = 0xB4
		menu_release.data[5] = IBus.getChecksum(menu_release)

		self.ibus_handler.writeIBusMessage(menu_release)

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
