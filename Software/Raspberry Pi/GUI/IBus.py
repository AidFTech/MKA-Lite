import serial
import time
import RPi.GPIO as GPIO

class AIData:
	def __init__(self, newl):
		self.data = [0]*newl
	
	def size(self):
		return len(self.data)
	
	def sender(self):
		return self.data[0]
	
	def recipient(self):
		return self.data[2]
	
	def refresh(self, newl):
		self.data = [0]*newl

def getChecksum(ai_b):
	checksum = 0
	
	for i in range(0, ai_b.size() - 1):
		checksum = checksum ^ ai_b.data[i]
	
	return checksum

def checkValidity(ai_b):
	l = ai_b.size()
	checksum = getChecksum(ai_b)

	if checksum == ai_b.data[l - 1]:
		return True
	else:
		return False

def checkDestination(ai_b, dest_id):
	if not checkValidity(ai_b):
		return False
	
	if ai_b.size() < 3:
		return False
	
	if ai_b.data[2] == dest_id or ai_b.data[2] == 0xFF:
		return True
	else:
		return False

def readAIData(ai_port, ai_b):
	received = False
	
	if ai_port.in_waiting > 0:
		data_rec = list(ai_port.read(2))

		try:
			sender = data_rec[0]
			l = data_rec[1]
		except IndexError:
			return received

		data = list(ai_port.read(l))
		
		try:
			ai_b.refresh(l+2)
			ai_b.data[0] = sender
			ai_b.data[1] = l
			
			for i in range(0, l):
				ai_b.data[i+2] = data[i]
		except IndexError:
			return received
			
		received = True
	
	return received

def writeAIBusMessage(ai_port, ai_b, rx_pin):
	GPIO.setmode(GPIO.BCM)
	GPIO.setup(rx_pin, GPIO.IN)
	first_low = int(time.perf_counter()*1000)
	last_low = first_low
	
	while last_low - first_low < 20:
		last_low = int(time.perf_counter()*1000)
		if GPIO.input(rx_pin) == 0:
			last_low = int(time.perf_counter()*1000)
	
	ai_port.write(ai_b.data)
