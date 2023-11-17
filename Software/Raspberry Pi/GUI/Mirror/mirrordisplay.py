#Acknowledgement to the works of Colin Munro and Sebastian Gotte.

import decoder
import audiodecoder
import protocol
import usb_link

import struct
import time
import queue
import threading

class MirrorDisplay:
	
	def __init__(self, amirror):
		self.amirror = amirror
		self.usblink = usb_link.Connection(self)
		self.decoder = None
		self.audio_decoder = None
		self.running = False
		self.startup = False
		self.phone_type = 0
		self.videomem = None
		
		self.autostart = True
		
	def __del__(self):
		self.usblink.running = False
		self.running = False
		self.startup = False
		self.heartbeat.join()
		self.decoder.send(self.videomem.data)
		self.decoder.stop()
		self.usblink.stop()
		self.audio_decoder.stop()
	
	def sendCommand(self, command):
		command_msg = protocol.CarPlay()
		command_data = struct.pack("<L", command)
		command_msg._setdata(command_data)
		self.usblink.sendMessage(command_msg)
	
	def startDongle(self, manufacturer_id, product_id):
		#TODO: Notify AMirror that the dongle is starting up.
		self.running = True
		self.usblink.connectDongle(manufacturer_id, product_id)
		#TOOD: Throw an error if there is a timeout.
		self.startup = False
		
		while self.startup == False: #TODO: And device is still connected.
			self.usblink.sendMultiple(protocol.startup_info)
			time.sleep(1)
	
	def heartbeatThread(self):
		while self.running and self.startup:
			try:
				self.usblink.sendMessage(protocol.Heartbeat())
			except:
				pass
			time.sleep(protocol.Heartbeat.lifecycle)
	
	def interpretMessage(self, message):
		if isinstance(message, protocol.Open):
			if not self.startup:
				self.startup = True
				self.usblink.sendMultiple(protocol.opened_info)

				#TODO: Interpret IDs 9 and 88, figure out what needs to send when.
				#TODO: Message to disable touchscreen?
				self.usblink.sendMessage(protocol.MetaData(mediaDelay=300, androidAutoSizeW=800, androidAutoSizeH=480))

				self.heartbeat = threading.Thread(target=self.heartbeatThread)
				self.heartbeat.start()
		
		elif isinstance(message, protocol.VideoData):
			self.videomem = message
			if self.decoder is not None:
				self.decoder.send(message.data)
		
		elif isinstance(message, protocol.Plugged):
			self.phone_type = message.phone_type
			self.sendPhoneType(self.phone_type)
			
			self.startPhoneConnection()
			if not self.autostart:
				self.decoder.setWindow(False)
			else:
				self.decoder.setWindow(True)

			self.sendPhoneType(self.phone_type)
		
		elif isinstance(message, protocol.Unplugged):
			self.phone_type = 0
			self.stopPhoneConnection()
			self.sendPhoneType(0)
		
		elif isinstance(message, protocol.MetaData):
			if hasattr(message, "MDModel"):
				self.amirror.setPhoneName(message.MDModel)
			
			if hasattr(message, "MediaAPPName"):
				self.amirror.sendAIBusText(0x60, message.MediaAPPName)
			
			if hasattr(message, "MediaSongName"):
				self.amirror.sendAIBusText(0x61, message.MediaSongName)
			
			if hasattr(message, "MediaArtistName"):
				self.amirror.sendAIBusText(0x62, message.MediaArtistName)
			
			if hasattr(message, "MediaAlbumName"):
				self.amirror.sendAIBusText(0x63, message.MediaAlbumName)
	
	def startPhoneConnection(self):
		if self.decoder is None:
			self.decoder = decoder.Decoder()
		
		if self.audio_decoder is None:
			self.audio_decoder = audiodecoder.AudioDecoder()
			
	def stopPhoneConnection(self):
		self.amirror.openMainMenu()
		
		if self.decoder is not None:
			self.decoder.send(self.videomem.data)
			self.decoder.stop()
			self.decoder = None
		
		if self.audio_decoder is not None:
			self.audio_decoder.stop()
			self.audio_decoder = None
		
	def sendPhoneType(self, phone_type):
		self.amirror.setPhoneType(phone_type)
	
	def setDayNight(self, night):
		if night:
			self.sendCommand(16)
		else:
			self.sendCommand(17)
	
	def reconnect(self, manufacturer_id, product_id): #Attempt to reconnect the dongle.
		self.phone_type = 0
		self.decoder.send(self.videomem.data)
		self.decoder.stop()
		self.audio_decoder.stop()
		self.decoder = None
		self.audio_decoder = None
		
		self.startup = False
		self.heartbeat.join()
		
		self.startDongle(manufacturer_id, product_id)
	
	def throwCriticalError(self, error_msg):
		pass
		#TODO: Notify AMirror of the error.
