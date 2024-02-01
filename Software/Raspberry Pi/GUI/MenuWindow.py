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
