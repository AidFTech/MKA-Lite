from MenuWindow import MenuWindow
from AttributeGroup import AttributeGroup
import ParameterList
import pygame as pg

'''The settings menu.'''
class SettingsMenuWindow(MenuWindow):
	def __init__(self, attribute_group: AttributeGroup, parameter_group: ParameterList.ParameterList, file_path: str):
		super().__init__(attribute_group, parameter_group, file_path)
		self.MAX_SELECTED = 6

	def displayMenu(self, display: pg.surface):
		WINDOW_WIDTH = self.attribute_group.w
		WINDOW_HEIGHT = self.attribute_group.h
		HEADER_HEIGHT = self.attribute_group.header_height
		RECT_WIDTH = self.attribute_group.rect_width
		OPTION_HEIGHT = self.attribute_group.option_height

		pg.draw.rect(display, self.attribute_group.header_color, pg.Rect(0, 0, WINDOW_WIDTH, HEADER_HEIGHT))
		pg.draw.rect(display, self.attribute_group.header_color, pg.Rect(0, WINDOW_HEIGHT-HEADER_HEIGHT, WINDOW_WIDTH, HEADER_HEIGHT))