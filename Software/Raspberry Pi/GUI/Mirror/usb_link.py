#Acknowledgement to the works of Colin Munro and Sebastian Gotte.

import usb.core
import usb.util
import threading
import struct
import protocol
import time

default_vendor = 0x1314
default_product = 0x1520

class Connection:
	def __init__(self, parent):
		self.device = None
		self.rx = None
		self.tx = None
		self.parent = parent
		self.running = False
	
	def connectDongle(self, vendor_id = default_vendor, product_id = default_product):
		
		while self.device == None: #TODO: Timeout?
			self.device = usb.core.find(idVendor = vendor_id, idProduct = product_id)

		device = self.device

		device.reset()
		device.set_configuration()
		interface = device.get_active_configuration()[(0,0)]

		self.rx = usb.util.find_descriptor(interface, custom_match = lambda e: usb.util.endpoint_direction(e.bEndpointAddress) == usb.util.ENDPOINT_IN)
		if self.rx is None:
			pass #TODO: Something!
		self.rx.clear_halt()

		self.tx = usb.util.find_descriptor(interface, custom_match = lambda e: usb.util.endpoint_direction(e.bEndpointAddress) == usb.util.ENDPOINT_OUT)
		if self.tx is None:
			pass #TODO: Something!
		self.tx.clear_halt()

		self.out_locker = threading.Lock()
		self.running = True
		self.thread = threading.Thread(target=self.readThread)
		self.thread.start()


	def readThread(self):
		while self.running == True:
			try:
				data = self.rx.read(protocol.Message.headersize)
			except usb.core.USBError as e:
				if e.errno != 110:
					self.running = False
					#TOOD: Send message to parent.
				continue
			if len(data) == protocol.Message.headersize:
				header = protocol.Message()
				try:
					header.deserialise(data)
				except ValueError as e:
					pass #TODO: Send message to parent?
				
				n = len(header._data())
				if n > 0:
					try:
						msg = header.upgrade(self.rx.read(n))
					except usb.core.USBError as e:
						#TODO: Send message to parent?
						continue
				else:
					msg = header.upgrade(bytes([0]*0))
					#msg = header
				
				try:
					self.interpretMessage(msg)
				except Exception as e:
					#TODO: Send message to parent?
					continue
	
	def interpretMessage(self, message):
		self.parent.interpretMessage(message)
	
	def sendMessage(self, message):
		if self.tx is not None:
			data = message.serialise()
			while not self.out_locker.acquire():
				pass
			try:
				self.tx.write(data[:message.headersize])
				self.tx.write(data[message.headersize:])
			except usb.core.USBError:
				self.parent.reconnect(default_vendor, default_product)
			finally:
				self.out_locker.release()
			
	def sendMultiple(self, messages):
		for m in messages:
			self.sendMessage(m)
	
	def stop(self):
		self.running = False
		self.thread.join()
		self.device = None
		self.rx = None
		self.tx = None

Error = usb.core.USBError
