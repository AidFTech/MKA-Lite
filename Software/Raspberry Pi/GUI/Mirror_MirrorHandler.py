import ParameterList
import CarLinkList
import threading
import struct
import time

import pygame as pg

import Mirror_USBLink
import Mirror_Protocol
import Mirror_Decoder

class MirrorHandler:
	def __init__(self, link_list: CarLinkList.CarLinkList):
		self.link_list = link_list
		self.parameters = link_list.parameters
		self.usb_link = Mirror_USBLink.USB_Connection(self.link_list)
		self.startup_thread = threading.Thread(target = self.connectDongleThread, args=(Mirror_USBLink.DEFAULT_VENDOR, Mirror_USBLink.DEFAULT_PRODUCT))
		self.startup_thread.start()

		self.decoder = None

		self.videomem_data = bytes([0]*0)

	def __del__(self):
		self.stopAll()
		try:
			self.startup_thread.join()
		except AttributeError:
			pass
			
	def loop(self):
		if len(self.link_list.rx_cache) > 0:	#Message waiting.
			for i in range(0,len(self.link_list.rx_cache)):
				msg = self.link_list.rx_cache[i]
				if isinstance(msg, Mirror_Protocol.Open):	#Startup message.
					self.usb_link.sendMultiple(Mirror_Protocol.opened_info)
					self.usb_link.sendMessage(Mirror_Protocol.MetaData(mediaDelay = 300, androidAutoSizeW = 800, AndroidAutoSizeH = 480))

					self.usb_link.startDongle()
				elif isinstance(msg, Mirror_Protocol.Plugged):	#Phone connected.
					self.parameters.phone_type = int(msg.phone_type)
					self.startPhoneConnection()
				elif isinstance(msg, Mirror_Protocol.Unplugged):
					self.parameters.phone_type = 0
					self.stopPhoneConnection()
				elif isinstance(msg, Mirror_Protocol.VideoData):
					self.videomem_data = msg.data
					if self.decoder is not None:
						self.decoder.send(msg.data)

			self.link_list.rx_cache.clear()

	def connectDongleThread(self, manufacturer_id: int, device_id: int):
		while not self.usb_link.running:
			connected = self.usb_link.connectDongle(manufacturer_id, device_id)

			if connected:
				while self.usb_link.running and not self.usb_link.startup:
					self.usb_link.sendMultiple(Mirror_Protocol.startup_info)

					self.usb_link.sendMessage(Mirror_Protocol.SendFile("/etc/airplay.conf", self.link_list.airplay_conf))
					self.usb_link.sendMessage(Mirror_Protocol.SendFile("/etc/oem_icon.png", self.link_list.oem_logo))
					self.usb_link.sendMessage(Mirror_Protocol.SendFile("/etc/icon_120x120.png", self.link_list.icon_120))
					self.usb_link.sendMessage(Mirror_Protocol.SendFile("/etc/icon_180x180.png", self.link_list.icon_180))
					self.usb_link.sendMessage(Mirror_Protocol.SendFile("/etc/icon_256x256.png", self.link_list.icon_256))

					time.sleep(1)

				if (not self.usb_link.running) and (not self.usb_link.startup):
					self.usb_link.stop()

	def startPhoneConnection(self):
		if self.decoder is None:
			self.decoder = Mirror_Decoder.Decoder(self.parameters.fullscreen, pg.display.get_surface().get_width(), pg.display.get_surface().get_height())

		self.decoder.setWindow(self.parameters.autoconnect)

	def stopPhoneConnection(self):
		self.parameters.next_menu = ParameterList.NEXTMENU_MIRROR_MENU

		if self.decoder is not None:
			self.decoder.send(self.videomem_data)
			self.decoder.stop()
			self.decoder = None

	def getWindow(self) -> bool:
		if self.decoder is not None:
			return self.decoder.getWindow()
		else:
			return False
		
	def sendMirrorCommand(self, command: int):
		command_msg = Mirror_Protocol.CarPlay()
		command_data = struct.pack("<L", command)
		command_msg._setdata(command_data)
		self.usblink.sendMessage(command_msg)

	def stopAll(self):
		self.stopPhoneConnection()
		self.usb_link.stop()