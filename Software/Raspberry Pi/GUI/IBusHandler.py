import serial
import IBus

IB_RX = 4

ISerial = serial.Serial(port="/dev/ttyAMA0", baudrate=9600, parity=serial.PARITY_EVEN)

class IBusHandler:
	ib_data = IBus.AIData(0)
	running = True
	parent = None
	parent_id = 0x01
	
	def __init__(self, parent, parent_id):
		self.parent = parent
		self.parent_id = parent_id
	
	def loop(self):
		ib_data = self.ib_data
		while self.running:
			msg_received = False
			msg_received = IBus.readAIData(ISerial, ib_data)
			
			if msg_received:
				if IBus.checkValidity(ib_data):
					if self.parent is not None:
						self.parent.handleIBusMessage(ib_data)
	
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
