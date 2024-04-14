import Mirror_Protocol
import ParameterList
import AttributeGroup

class CarLinkList:
	"""A list of variables to be shared between the mirror handler and MKA object."""
	def __init__(self, parameters: ParameterList.ParameterList, attributes: AttributeGroup.AttributeGroup):
		self.parameters = parameters	#The shared parameter list.
		self.rx_cache = [Mirror_Protocol.Message()]*0	#The cache of messages received.
		self.attributes = attributes	#The shared attribute list.

		self.airplay_conf = bytes([0]*0)
		self.oem_logo = bytes([0]*0)
		self.icon_120 = bytes([0]*0)
		self.icon_180 = bytes([0]*0)
		self.icon_256 = bytes([0]*0)