import Mirror_Protocol
import ParameterList

'''A list of variables to be shared between the mirror handler and MKA object.'''
class CarLinkList:
	def __init__(self, parameters: ParameterList.ParameterList):
		self.parameters = parameters	#The shared parameter list.
		self.rx_cache = [Mirror_Protocol.Message()]*0	#The cache of messages received.