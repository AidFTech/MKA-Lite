import serial
import IBus
import time

IB_RX = 4

SONG_NAME = 1
ARTIST_NAME = 2
ALBUM_NAME = 3
APP_NAME = 4

WAIT_TIME = 100

ISerial = serial.Serial(port="/dev/ttyAMA0", baudrate=9600, parity=serial.PARITY_EVEN, timeout=0.1)

class IBusHandler:
	ib_data = IBus.AIData(0)
	running = True
	parent = None
	parent_id = 0x01
	mirror = None
	
	def __init__(self, parent, mirror, parent_id):
		self.parent = parent
		self.parent_id = parent_id
		self.mirror = mirror
	
	def loop(self):
		ib_data = self.ib_data
		while self.running:
			msg_received = False
			msg_received = IBus.readAIData(ISerial, ib_data)
			
			if msg_received:
				if IBus.checkValidity(ib_data):
					self.readIBusMessage(ib_data)
	
	def readIBusMessage(self, ib_data):
		cmd_pass = False
		if hasattr(self.mirror, "decoder"):
			if self.mirror.decoder is not None and not self.mirror.decoder.player.window_minimized:
				cmd_pass = True
			
		if ib_data.size() < 4:
			return
		
		if ib_data.data[3] == 0x1 and (ib_data.data[2] == 0xED or ib_data.data[2] == 0xBF or ib_data.data[2] == 0xFF): #Ping
			self.sendPong(ib_data.data[0], 0xED)
		elif ib_data.data[3] == 0x1 and ib_data.data[2] == 0x18: #Ping to CD changer.
			self.sendPong(ib_data.data[0], 0x18)
		elif ib_data.data[3] == 0x4F: #Change in screen control.
			if(ib_data.data[4]&0x01) != 0:
				self.parent.control = True
		elif ib_data.data[0] == 0xF0 and self.parent.control: #From BMBT.
			if ib_data.data[3] == 0x49: #Selection knob turn.
				num_turns = ib_data.data[4]&0xF
				for i in range(0,num_turns):
					if (ib_data.data[4]&0x80) == 0:
						if not cmd_pass:
							if self.parent.active_menu.selected < len(self.parent.active_menu.options):
								self.parent.active_menu.selected += 1
							else:
								self.parent.active_menu.selected = 1
						else:
							self.mirror.sendCommand(100)
					else:
						if not cmd_pass:
							if self.parent.active_menu.selected > 1:
								self.parent.active_menu.selected -= 1
							else:
								self.parent.active_menu.selected = len(self.parent.active_menu.options)
						else:
							self.mirror.sendCommand(101)
			elif ib_data.data[3] == 0x48: #Button press.
				button = ib_data.data[4]&0x3F
				state = (ib_data.data[4]&0xC0) >> 6
				
				if button == 0x5 and state == 0x0: #Selection knob.
					if not cmd_pass:
						self.parent.active_menu.makeSelection(self.parent.active_menu.selected)
					else:
						self.mirror.sendCommand(104)
						self.mirror.sendCommand(105)
				elif button == 0x34 and state == 0x2: #Menu button released.
					self.parent.control = False
				elif cmd_pass:
					if self.parent.selected and button == 0x14 and state == 0: #Direction/pause button.
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
		elif ib_data.data[0] == 0x68: #From radio.
			self.parent.handleRadioMessage(ib_data)
		elif ib_data.data[0] == 0xD0: #From LCM.
			if ib_data.data[3] == 0x5B and not self.parent.RLS_connected:
				last_night = self.parent.night
				self.parent.night = (ib_data.data[4]&0x01) != 0
				if self.parent.night != last_night:
					self.mirror.setDayNight(self.parent.night)
		elif ib_data.data[0] == 0x80: #From IKE.
			self.parent.handleIKEMessage(ib_data)
		elif ib_data.data[0] == 0xE8: #From RLS.
			if not self.parent.RLS_connected:
				self.parent.RLS_connected = True
			if ib_data.data[3] == 0x59:
				last_night = self.parent.night
				self.parent.night = ((ib_data.data[4]&0x01) != 0) and ((ib_data.data[4]>>4) < self.parent.light_thresh)
				if self.parent.night != last_night:
					self.mirror.setDayNight(self.parent.night)
		elif ib_data.data[0] == 0x3B: #From navigation computer.
			if ib_data.data[3] == 0xA0: #Version message.
				version_data0 = ib_data.data[15]
				version_data1 = ib_data.data[16]

				version = bytearray([version_data0, version_data1])
				self.parent.gt_version = int(version.decode())
			elif ib_data.data[3] == 0x4E: #Ensure the radio is enabled.
				if (ib_data.data[4]&0x1) != 0x0:
					self.activateRadio()
			
			if self.parent.gt_version <= 0:
				self.sendVersionQuery(0x3B)

	def writeIBusMessage(self, ib_msg):
		IBus.writeAIBusMessage(ISerial, ib_msg, IB_RX)
		
	def sendPong(self, receiver, my_id):
		pong = IBus.AIData(6)
		
		pong.data[0] = my_id
		pong.data[1] = pong.size()-2
		pong.data[2] = receiver
		pong.data[3] = 0x2
		pong.data[4] = 0x0
		pong.data[5] = IBus.getChecksum(pong)
		
		self.writeIBusMessage(pong)

	def sendVersionQuery(self, receiver):
		query = IBus.AIData(5)

		query.data[0] = 0x3F
		query.data[1] = query.size()-2
		query.data[2] = receiver
		query.data[3] = 0x0
		query.data[4] = IBus.getChecksum(query)

		self.writeIBusMessage(query)

	def activateRadio(self):
		activator = IBus.AIData(7)

		activator.data[0] = 0x3B
		activator.data[1] = activator.size()-2
		activator.data[2] = 0x68
		activator.data[3] = 0x4E
		activator.data[4] = 0x0
		activator.data[5] = 0x0
		activator.data[6] = IBus.getChecksum(activator)

		self.writeIBusMessage(activator)
	
	def deactivateRadioMenu(self):
		deactivator = IBus.AIData(6)

		deactivator.data[0] = 0x68
		deactivator.data[1] = deactivator.size()-2
		deactivator.data[2] = 0x3B
		deactivator.data[3] = 0x46
		deactivator.data[4] = 0x4
		deactivator.data[5] = IBus.getChecksum(deactivator)

		self.writeIBusMessage(deactivator)

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
		vm_message.data[6] = IBus.getChecksum(vm_message)
		
		self.writeIBusMessage(vm_message)

	#Send radio metadata text.
	def sendRadioText(self, text, position, refresh):
		if self.parent.gt_version >= 5:
			index = 0x40
			if position == SONG_NAME:
				index = 0x41
			elif position == ARTIST_NAME:
				index = 0x42
			elif position == ALBUM_NAME:
				index = 0x43
			elif position == APP_NAME:
				index = 0x44
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

			text_message.data[text_message.size()-1] = IBus.getChecksum(text_message)

			self.writeIBusMessage(text_message)
			
			if refresh:
				update_message = IBus.AIData(8)
				
				update_message.data[0] = 0x68
				update_message.data[1] = update_message.size()-2
				update_message.data[2] = 0x3B
				update_message.data[3] = 0xA5
				update_message.data[4] = 0x63
				update_message.data[5] = 0x01
				update_message.data[6] = 0x00
				update_message.data[7] = IBus.getChecksum(update_message)
				
				self.writeIBusMessage(update_message)
		else:
			index = -1
			if position == SONG_NAME:
				index = 0
			elif position == ARTIST_NAME:
				index = 1
			elif position == ALBUM_NAME:
				index = 2
			elif position == APP_NAME:
				index = 3
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

			text_message.data[text_message.size()-1] = IBus.getChecksum(text_message)
			
			self.writeIBusMessage(text_message)
			
			if refresh:
				update_message = IBus.AIData(8)
				
				update_message.data[0] = 0x68
				update_message.data[1] = update_message.size()-2
				update_message.data[2] = 0x3B
				update_message.data[3] = 0xA5
				update_message.data[4] = 0x60
				update_message.data[5] = 0x01
				update_message.data[6] = 0x00
				update_message.data[7] = IBus.getChecksum(update_message)
				
				self.writeIBusMessage(update_message)

	#Set the text in the title header in the audio screen.
	def sendGTIBusTitle(self, text):
		if self.parent.gt_version < 4:
			title_message = IBus.AIData(8+len(text))
			
			title_message.data[0] = 0x68
			title_message.data[1] = title_message.size()-2
			title_message.data[2] = 0x3B
			title_message.data[3] = 0x23
			title_message.data[4] = 0x62
			title_message.data[5] = 0x30
			title_message.data[6:6+len(text)] = bytes(text, 'ascii')
			title_message.data[title_message.size()-2] = 0x8E #Flag for telling the system that it sent the message.
			title_message.data[title_message.size()-1] = IBus.getChecksum(title_message)
			
			self.writeIBusMessage(title_message)

			update_message = IBus.AIData(9)
			
			update_message.data[0] = 0x68
			update_message.data[1] = update_message.size()-2
			update_message.data[2] = 0x3B
			update_message.data[3] = 0xA5
			update_message.data[4] = 0x62
			update_message.data[5] = 0x01
			update_message.data[6] = 0x00
			update_message.data[7] = 0x8E
			update_message.data[8] = IBus.getChecksum(update_message)
			
			self.writeIBusMessage(update_message)
		else:
			title_message = IBus.AIData(9+len(text))
			
			title_message.data[0] = 0x68
			title_message.data[1] = title_message.size()-2
			title_message.data[2] = 0x3B
			title_message.data[3] = 0x21
			title_message.data[4] = 0x62
			title_message.data[5] = 0x01
			title_message.data[6] = 0x40
			title_message.data[7:7+len(text)] = bytes(text, 'ascii')
			title_message.data[title_message.size()-2] = 0x8E #Flag for telling the system that it sent the message.
			title_message.data[title_message.size()-1] = IBus.getChecksum(title_message)
			
			self.writeIBusMessage(title_message)
			
			update_message = IBus.AIData(9)
			
			update_message.data[0] = 0x68
			update_message.data[1] = update_message.size()-2
			update_message.data[2] = 0x3B
			update_message.data[3] = 0xA5
			update_message.data[4] = 0x62
			update_message.data[5] = 0x01
			update_message.data[6] = 0x00
			update_message.data[7] = 0x8E
			update_message.data[8] = IBus.getChecksum(update_message)
			
			self.writeIBusMessage(update_message)

	##Set the "subtitles" in the top of the headerbar.
	def sendGTIBusSubtitle(self, text, position, refresh):
		if position > 6:
			return
		
		subtitle_message = IBus.AIData(9+len(text))

		subtitle_message.data[0] = 0x68
		subtitle_message.data[1] = subtitle_message.size()-2
		subtitle_message.data[2] = 0x3B
		subtitle_message.data[3] = 0xA5
		subtitle_message.data[4] = 0x62
		subtitle_message.data[5] = 0x01
		subtitle_message.data[6] = position|0x40
		subtitle_message.data[7:7+len(text)] = bytes(text, 'ascii')
		subtitle_message.data[subtitle_message.size()-2] = 0x8E #Flag for telling the system that it sent the message.
		subtitle_message.data[subtitle_message.size()-1] = IBus.getChecksum(subtitle_message)

		self.writeIBusMessage(subtitle_message)

		if refresh:
			update_message = IBus.AIData(9)
		
			update_message.data[0] = 0x68
			update_message.data[1] = update_message.size()-2
			update_message.data[2] = 0x3B
			update_message.data[3] = 0xA5
			update_message.data[4] = 0x62
			update_message.data[5] = 0x01
			update_message.data[6] = 0x00
			update_message.data[7] = 0x8E
			update_message.data[8] = IBus.getChecksum(update_message)