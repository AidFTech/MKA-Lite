from MenuWindow import MenuWindow
from AttributeGroup import AttributeGroup
import ParameterList
import pygame as pg

class NightModeMenuWindow(MenuWindow):
	def __init__(self, attribute_group: AttributeGroup, parameter_group: ParameterList.ParameterList, file_path: str):
		super().__init__(attribute_group, parameter_group, file_path)
		self.MAX_SELECTED = 7
		self.options = [""]*self.MAX_SELECTED

		for i in range(0,len(self.options)-1):
			self.options[i] = str(i+1)

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

		title_text = font.render("Night Mode Sensitivity", False, self.attribute_group.text_color)
		display.blit(title_text, (4,-3))

		for i in range(0,self.MAX_SELECTED):
			pg.draw.rect(display, self.attribute_group.border_color, pg.Rect(0, HEADER_HEIGHT + i*OPTION_HEIGHT, RECT_WIDTH, OPTION_HEIGHT))
			pg.draw.rect(display, self.attribute_group.border_outline, pg.Rect(0, HEADER_HEIGHT + i*OPTION_HEIGHT, RECT_WIDTH, OPTION_HEIGHT), 1)
			
			text = font.render(self.options[i], False, self.attribute_group.text_color)
			t_x = RECT_WIDTH + 10
			t_y = HEADER_HEIGHT + OPTION_HEIGHT*i

			display.blit(text, (t_x,t_y))

			if i == self.MAX_SELECTED - 2:	#Back button.
				display.blit(return_img, (WINDOW_WIDTH - 170, t_y + OPTION_HEIGHT/2 - return_img.get_height()/2))

			if i == self.selected - 1:
				pg.draw.rect(display, self.attribute_group.rect_color, pg.Rect(0, HEADER_HEIGHT + i*OPTION_HEIGHT, RECT_WIDTH, OPTION_HEIGHT))
				pg.draw.rect(display, self.attribute_group.rect_color, pg.Rect(0, HEADER_HEIGHT + i*OPTION_HEIGHT, WINDOW_WIDTH - RECT_WIDTH*2, OPTION_HEIGHT), 5)

	def goBack(self):
		self.parameter_group.next_menu = ParameterList.NEXTMENU_SETTINGS_MENU