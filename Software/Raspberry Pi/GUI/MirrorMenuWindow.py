from MenuWindow import MenuWindow
from AttributeGroup import AttributeGroup
import pygame as pg

'''The phone mirror menu.'''
class MirrorMenuWindow(MenuWindow):
	#def __init__(self, attribute_group: AttributeGroup, file_path: str):
	#	super().__init__(self, attribute_group, file_path)

	def displayMenu(self, display: pg.surface):
		carplay_img = pg.image.load(self.file_path + 'Apple_CarPlay_Logo_100.png')
		android_img = pg.image.load(self.file_path + 'Android_Auto_icon_100.png')
		return_img = pg.image.load(self.file_path + 'return.png')
		
		window_width = self.attribute_group.w
		window_height = self.attribute_group.h
		header_height = self.attribute_group.header_height

		display.blit(carplay_img, (int(window_width*3/16), int(window_height/4)))
		display.blit(android_img, (int(window_width*23/32), int(window_height/4)))

		pg.draw.rect(display, self.attribute_group.header_color, pg.Rect(0, 0, window_width, header_height))
		pg.draw.rect(display, self.attribute_group.header_color, pg.Rect(0, window_height-header_height, window_width, header_height))