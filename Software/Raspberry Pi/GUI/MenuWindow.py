from AttributeGroup import AttributeGroup
from ParameterList import ParameterList
import pygame as pg

'''Generic PyGame menu window.'''
class MenuWindow:
	def __init__(self, attribute_group: AttributeGroup, parameter_group: ParameterList, file_path: str):
		self.attribute_group = attribute_group	#The color group to be used.
		self.parameter_group = parameter_group	#The parameter group to be used.
		self.file_path = file_path	#The file path for loading images, fonts, etc.

		self.selected = 0	#The selected option.
		self.MAX_SELECTED = 1

	'''Generic window display message.'''
	def displayMenu(self, display: pg.surface):
		display.fill(self.attribute_group.br)

	'''Draw the clock.'''
	def drawClock(self, display: pg.surface):
		time_str = "--:--"
		if self.parameter_group.ike_hour >= 0 and self.parameter_group.ike_minute >= 0:
			if self.parameter_group.ike_24h or (self.parameter_group.ike_hour >=1 and self.parameter_group.ike_hour <= 12):
				time_str = str(self.parameter_group.ike_hour)
				time_str += ":"
			elif self.parameter_group.ike_hour == 0:
				time_str = "12:"
			elif self.parameter_group.ike_hour >= 13:
				time_str = str(self.parameter_group.ike_hour-12)
				time_str += ":"
				
			if self.parameter_group.ike_minute < 10:
				time_str += "0" + str(self.parameter_group.ike_minute)
			else:
				time_str += str(self.parameter_group.ike_minute)

			if not self.parameter_group.ike_24h:
				if self.parameter_group.ike_hour < 12:
					time_str += " AM"
				else:
					time_str += " PM"
					
		date_str = self.parameter_group.ike_datestring
		
		font = self.attribute_group.main_font

		time_text = font.render(time_str, False, self.attribute_group.text_color)
		display.blit(time_text, (4, self.attribute_group.h-self.attribute_group.header_height-3))

		date_text = font.render(date_str, False, self.attribute_group.text_color)
		display.blit(date_text, (self.attribute_group.w - date_text.get_width() - 20, self.attribute_group.h-self.attribute_group.header_height-3))

	'''Choose the selected option.'''
	def setSelected(self, new_selected: int):
		if new_selected < 0:
			new_selected = 0
		elif new_selected > self.MAX_SELECTED:
			new_selected = self.MAX_SELECTED

		self.selected = new_selected

	'''Get the selected option.'''
	def getSelected(self) -> int:
		return self.selected

	'''Increment the selected option.'''
	def incrementSelected(self):
		new_selected = self.selected + 1
		if new_selected > self.MAX_SELECTED:
			new_selected = 1
		#TODO: Figure out whether to skip this option.

		self.selected = new_selected
	
	'''Decrement the selected option.'''
	def decrementSelected(self):
		new_selected = self.selected - 1
		if new_selected < 1:
			new_selected = self.MAX_SELECTED
		#TODO: Figure out whether to skip this option.

		self.selected = new_selected

	'''Perform the selected option.'''
	def makeSelection(self):
		pass
