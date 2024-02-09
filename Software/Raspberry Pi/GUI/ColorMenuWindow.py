from MenuWindow import MenuWindow
from AttributeGroup import AttributeGroup
import ParameterList
import pygame as pg
import ColorHandler

COLOR_MSG_CUSTOM = "Custom"
COLOR_MSG_BACK = "Back"

'''The color settings menu.'''
class ColorMenuWindow(MenuWindow):
	color_presets = ["OE Color 1", "OE Color 2", "M Colors", "Night"] #TODO: Make this a bit more flexible.

	def __init__(self, attribute_group: AttributeGroup, parameter_group: ParameterList.ParameterList, file_path: str):
		super().__init__(attribute_group, parameter_group, file_path)
		self.MAX_SELECTED = len(self.color_presets) + 2
		self.selected = 1

	def displayMenu(self, display: pg.surface):
		WINDOW_WIDTH = self.attribute_group.w
		WINDOW_HEIGHT = self.attribute_group.h
		HEADER_HEIGHT = self.attribute_group.header_height
		RECT_WIDTH = self.attribute_group.rect_width
		OPTION_HEIGHT = 50

		return_img = pg.image.load(self.file_path + 'return.png')

		font = self.attribute_group.main_font

		pg.draw.rect(display, self.attribute_group.header_color, pg.Rect(0, 0, WINDOW_WIDTH, HEADER_HEIGHT))
		pg.draw.rect(display, self.attribute_group.header_color, pg.Rect(0, WINDOW_HEIGHT-HEADER_HEIGHT, WINDOW_WIDTH, HEADER_HEIGHT))
		self.drawClock(display)

		title_text = font.render("Color Settings", False, self.attribute_group.text_color)
		display.blit(title_text, (4,-3))

		for i in range(0,self.MAX_SELECTED):
			pg.draw.rect(display, self.attribute_group.border_color, pg.Rect(0, HEADER_HEIGHT + i*OPTION_HEIGHT, RECT_WIDTH, OPTION_HEIGHT))
			pg.draw.rect(display, self.attribute_group.border_outline, pg.Rect(0, HEADER_HEIGHT + i*OPTION_HEIGHT, RECT_WIDTH, OPTION_HEIGHT), 1)
			
			if i < len(self.color_presets):
				text = font.render(self.color_presets[i], False, self.attribute_group.text_color)
			elif i == self.MAX_SELECTED - 2:
				text = font.render(COLOR_MSG_CUSTOM, False, self.attribute_group.text_color)
			else:
				text = font.render(COLOR_MSG_BACK, False, self.attribute_group.text_color)
			t_x = RECT_WIDTH + 10
			t_y = HEADER_HEIGHT + OPTION_HEIGHT*i

			display.blit(text, (t_x,t_y))

			if i == self.MAX_SELECTED - 1:
				display.blit(return_img, (WINDOW_WIDTH - 170, t_y + OPTION_HEIGHT/2 - return_img.get_height()/2))

			if i == self.selected - 1:
				pg.draw.rect(display, self.attribute_group.rect_color, pg.Rect(0, HEADER_HEIGHT + i*OPTION_HEIGHT, RECT_WIDTH, OPTION_HEIGHT))
				pg.draw.rect(display, self.attribute_group.rect_color, pg.Rect(0, HEADER_HEIGHT + i*OPTION_HEIGHT, WINDOW_WIDTH - RECT_WIDTH*2, OPTION_HEIGHT), 5)

	def makeSelection(self):
		selected = self.selected - 1
		if selected < len(self.color_presets):
			if "OE Color 1" in self.color_presets[selected]:
				ColorHandler.setColors(self.attribute_group, ColorHandler.OEMColor1)
			elif "OE Color 2" in self.color_presets[selected]:
				ColorHandler.setColors(self.attribute_group, ColorHandler.OEMColor2)
			elif "M Colors" in self.color_presets[selected]:
				ColorHandler.setColors(self.attribute_group, ColorHandler.MColor)
			elif "Night" in self.color_presets[selected]:
				ColorHandler.setColors(self.attribute_group, ColorHandler.NightColor)
		elif selected == self.MAX_SELECTED - 1:
			self.parameter_group.next_menu = ParameterList.NEXTMENU_SETTINGS_MENU