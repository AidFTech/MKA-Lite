from AttributeGroup import AttributeGroup
import pygame as pg

'''Generic PyGame menu window.'''
class MenuWindow:
	def __init__(self, attribute_group: AttributeGroup, file_path: str):
		self.attribute_group = attribute_group	#The color group to be used.
		self.file_path = file_path	#The file path for loading images, fonts, etc.

		self.selected = 0	#The selected option.

	'''Generic window display message.'''
	def displayMenu(self, display: pg.surface):
		display.fill(self.attribute_group.br)