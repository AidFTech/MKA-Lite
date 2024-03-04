#Acknowledgement to the works of Colin Munro and Sebastian Gotte.

import usb.core
import usb.util
import threading
import struct
import time

import Mirror_Protocol
import CarLinkList

DEFAULT_VENDOR = 0x1314
DEFAULT_PRODUCT = 0x1520

MAX_WAIT = 20

class USB_Connection:
	def __init__(self, carlink_list: CarLinkList.CarLinkList):
		self.device = None
		self.rx = None
		self.tx = None
		self.running = False
		self.startup = False
		self.carlink_list = carlink_list

	def __del__(self):
		self.stop()

	def connectDongle(self, vendor_id = DEFAULT_VENDOR, product_id = DEFAULT_PRODUCT) -> bool:
		start_time = time.time()
		while self.device == None:
			self.device = usb.core.find(idVendor = vendor_id, idProduct = product_id)
			if time.time() - start_time >= MAX_WAIT:
				return False

		device = self.device

		try:
			device.reset()
			device.set_configuration()
			interface = device.get_active_configuration()[(0,0)]

			self.rx = usb.util.find_descriptor(interface, custom_match = lambda e: usb.util.endpoint_direction(e.bEndpointAddress) == usb.util.ENDPOINT_IN)
			if self.rx is None:
				return False
			self.rx.clear_halt()

			self.tx = usb.util.find_descriptor(interface, custom_match = lambda e: usb.util.endpoint_direction(e.bEndpointAddress) == usb.util.ENDPOINT_OUT)
			if self.tx is None:
				return False
			self.tx.clear_halt()

			self.out_locker = threading.Lock()
			self.running = True
			self.run_thread = threading.Thread(target=self.readThread)
			self.run_thread.start()
		except usb.core.USBError:
			return False
		return True
	
	def startDongle(self):
		if not self.running:
			return
		
		self.startup = True
		self.heartbeat_thread = threading.Thread(target=self.heartbeatThread)
		self.heartbeat_thread.start()

	def readThread(self):
		while self.running == True:
			msg_read = False

			try:
				data = self.rx.read(Mirror_Protocol.Message.headersize)
			except usb.core.USBError as e:
				if e.errno != 110:
					self.running = False
					break
					#TODO: Send message to parent.
				else:
					continue
			if len(data) == Mirror_Protocol.Message.headersize:
				header = Mirror_Protocol.Message()
				try:
					header.deserialise(data)
				except ValueError as e:
					pass #TODO: Send message to parent?

				n = len(header._data())
				if n > 0:
					try:
						msg = header.upgrade(self.rx.read(n))
						msg_read = True
					except usb.core.USBError as e:
						msg_read = False #TODO: Send message to parent? Something is wrong here..?
				else:
					msg = header.upgrade(bytes([0]*0))
					msg_read = True

				if msg_read:
					self.carlink_list.rx_cache.append(msg)

		#if not self.running:
		#	self.stop()

	def heartbeatThread(self):
		while self.running and self.startup:
			try:
				self.sendMessage(Mirror_Protocol.Heartbeat())
			except usb.core.USBError as e:
				self.running = False
				self.startup = False
				break
			except:
				pass
			time.sleep(Mirror_Protocol.Heartbeat.lifecycle)
		
		if not self.running or not self.startup:
			self.running = False
			self.startup = False


	def sendMessage(self, message: Mirror_Protocol.Message):
		if self.tx is not None:
			data = message.serialise()
			while not self.out_locker.acquire():
				pass
			try:
				self.tx.write(data[:message.headersize])
				self.tx.write(data[message.headersize:])
			except usb.core.USBError:
				pass #TODO: Something went very wrong here.
			finally:
				self.out_locker.release()

	def sendMultiple(self, messages: list[Mirror_Protocol.Message]):
		for m in messages:
			self.sendMessage(m)
	
	def stop(self):
		self.running = False
		self.startup = False
		self.device = None
		self.rx = None
		self.tx = None
		try:
			self.run_thread.join()
		except (RuntimeError, AttributeError):
			pass

		try:
			self.heartbeat_thread.join()
		except (RuntimeError, AttributeError):
			pass

Error = usb.core.USBError