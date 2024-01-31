from MenuWindow import MenuWindow
from AttributeGroup import AttributeGroup
import pygame as pg

SETTINGS_MSG = "Settings"

NO_PHONE = 0
ANDROID = 1
CARPLAY = 2

'''The phone mirror menu.'''
class MirrorMenuWindow(MenuWindow):
	def __init__(self, attribute_group: AttributeGroup, file_path: str):
		super().__init__(attribute_group, file_path)
		self.MAX_SELECTED = 3
		self.mirror_msg = "Start Phone Mirroring"	#Generic placeholder message.
		self.phone_type = NO_PHONE	#The type of phone connected.

	def displayMenu(self, display: pg.surface):
		carplay_img = pg.image.load(self.file_path + 'Apple_CarPlay_Logo_100.png')
		android_img = pg.image.load(self.file_path + 'Android_Auto_icon_100.png')
		return_img = pg.image.load(self.file_path + 'return.png')
		
		WINDOW_WIDTH = self.attribute_group.w
		WINDOW_HEIGHT = self.attribute_group.h
		HEADER_HEIGHT = self.attribute_group.header_height
		RECT_WIDTH = self.attribute_group.rect_width
		OPTION_HEIGHT = self.attribute_group.option_height

		if self.phone_type == CARPLAY:
			display.blit(carplay_img, (RECT_WIDTH + 50, HEADER_HEIGHT + 50))
		elif self.phone_type == ANDROID:
			display.blit(android_img, (RECT_WIDTH + 50, HEADER_HEIGHT + 50))

		pg.draw.rect(display, self.attribute_group.header_color, pg.Rect(0, 0, WINDOW_WIDTH, HEADER_HEIGHT))
		pg.draw.rect(display, self.attribute_group.header_color, pg.Rect(0, WINDOW_HEIGHT-HEADER_HEIGHT, WINDOW_WIDTH, HEADER_HEIGHT))

		pg.draw.rect(display, self.attribute_group.border_color, pg.Rect(0, HEADER_HEIGHT, RECT_WIDTH, WINDOW_HEIGHT - 2*HEADER_HEIGHT - 2*OPTION_HEIGHT))
		pg.draw.rect(display, self.attribute_group.border_outline, pg.Rect(0, HEADER_HEIGHT, RECT_WIDTH, WINDOW_HEIGHT - 2*HEADER_HEIGHT - 2*OPTION_HEIGHT), 1)

		pg.draw.rect(display, self.attribute_group.border_color, pg.Rect(0, WINDOW_HEIGHT - HEADER_HEIGHT - OPTION_HEIGHT*2, RECT_WIDTH, OPTION_HEIGHT))
		pg.draw.rect(display, self.attribute_group.border_outline, pg.Rect(0, WINDOW_HEIGHT - HEADER_HEIGHT - OPTION_HEIGHT*2, RECT_WIDTH, OPTION_HEIGHT), 1)

		pg.draw.rect(display, self.attribute_group.border_color, pg.Rect(0, WINDOW_HEIGHT - HEADER_HEIGHT - OPTION_HEIGHT, RECT_WIDTH, OPTION_HEIGHT))
		pg.draw.rect(display, self.attribute_group.border_outline, pg.Rect(0, WINDOW_HEIGHT - HEADER_HEIGHT - OPTION_HEIGHT, RECT_WIDTH, OPTION_HEIGHT), 1)

		font = self.attribute_group.main_font
		text = font.render(self.mirror_msg, False, self.attribute_group.text_color)
		display.blit(text, (RECT_WIDTH + 10,HEADER_HEIGHT + 200))

		WINDOW_HEIGHT = self.attribute_group.h
		text = font.render(SETTINGS_MSG, False, self.attribute_group.text_color)
		display.blit(text, (RECT_WIDTH + 10,WINDOW_HEIGHT - HEADER_HEIGHT - OPTION_HEIGHT*2 + 16))

		display.blit(return_img, (RECT_WIDTH,WINDOW_HEIGHT - HEADER_HEIGHT - OPTION_HEIGHT))

		if self.selected >= 1 and self.selected <= self.MAX_SELECTED:
			r_y = HEADER_HEIGHT
			r_h = WINDOW_HEIGHT - 2*HEADER_HEIGHT - 2*OPTION_HEIGHT
			if self.selected > 1:
				r_h = OPTION_HEIGHT
			if self.selected == 2:
				r_y = WINDOW_HEIGHT - HEADER_HEIGHT - OPTION_HEIGHT*2
			elif self.selected == 3:
				r_y = WINDOW_HEIGHT - HEADER_HEIGHT - OPTION_HEIGHT
			
			pg.draw.rect(display, self.attribute_group.rect_color, pg.Rect(0, r_y, RECT_WIDTH, r_h))
			pg.draw.rect(display, self.attribute_group.rect_color, pg.Rect(0, r_y, WINDOW_WIDTH - RECT_WIDTH*2, r_h), 5)

	def setPhoneName(self, phone_name: str, phone_type: int):
		self.phone_type = phone_type
		if phone_type == NO_PHONE:
			self.mirror_msg = "Start Phone Mirroring"
		elif phone_type == ANDROID:
			self.mirror_msg = "Android Auto: " + phone_name
		elif phone_type == CARPLAY:
			self.mirror_msg = "Apple CarPlay: " + phone_name
