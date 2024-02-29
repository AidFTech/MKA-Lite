import CarLinkList
import threading
import time

import Mirror_USBLink
import Mirror_Protocol

class MirrorHandler:
	def __init__(self, link_list: CarLinkList.CarLinkList):
		self.link_list = link_list
		self.usb_link = Mirror_USBLink.USB_Connection(self.link_list)

	def __del__(self):
		self.usb_link.stop()

	def connectDongle(self, manufacturer_id: int, device_id: int):
		if (not hasattr(self, "startup_thread")) or (not self.startup_thread.is_alive()):
			self.startup_thread = threading.Thread(target = self.connectDongleThread, args=(manufacturer_id, device_id))
			self.startup_thread.start()

	def loop(self):
		if not self.usb_link.running:
			self.connectDongle(0x1314, 0x1520)
		if len(self.link_list.rx_cache) > 0: #Message waiting.
			for i in range(0,len(self.link_list.rx_cache)):
				msg = self.link_list.rx_cache[i]
				if isinstance(msg, Mirror_Protocol.Open): #Startup message.
					self.usb_link.sendMultiple(Mirror_Protocol.opened_info)
					self.usb_link.sendMessage(Mirror_Protocol.MetaData(mediaDelay = 300, androidAutoSizeW = 800, AndroidAutoSizeH = 480))

					self.usb_link.startDongle()
			
			self.link_list.rx_cache.clear()

	def connectDongleThread(self, manufacturer_id: int, device_id: int):
		while not self.usb_link.running:
			connected = self.usb_link.connectDongle(manufacturer_id, device_id)

		if connected:
			while self.usb_link.running and not self.usb_link.startup:
				self.usb_link.sendMultiple(Mirror_Protocol.startup_info)

				#TODO: Send pictures for icons.

				time.sleep(1)

		if (not self.usb_link.running) or (not self.usb_link.startup):
			self.usb_link.stop()