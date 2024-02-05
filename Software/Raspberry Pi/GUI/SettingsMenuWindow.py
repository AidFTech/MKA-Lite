from MenuWindow import MenuWindow
from AttributeGroup import AttributeGroup
import ParameterList
import pygame as pg

SETTINGS_MSG_NIGHTSENS = "Night Mode Sensitivity: "
SETTINGS_MSG_AUTOCONNECT = "Auto Connect"
SETTINGS_MSG_SOURCE = "Audio Source: "
SETTINGS_MSG_HUD = "Audio HUD"
SETTINGS_MSG_BACK = "Back"

DEFAULT_MAX = 4

'''The settings menu.'''
class SettingsMenuWindow(MenuWindow):
	def __init__(self, attribute_group: AttributeGroup, parameter_group: ParameterList.ParameterList, file_path: str):
		super().__init__(attribute_group, parameter_group, file_path)
		self.MAX_SELECTED = DEFAULT_MAX
		self.options = [""]*self.MAX_SELECTED

		self.loadDefaultOptions()
		self.selected = 1

	def displayMenu(self, display: pg.surface):
		WINDOW_WIDTH = self.attribute_group.w
		WINDOW_HEIGHT = self.attribute_group.h
		HEADER_HEIGHT = self.attribute_group.header_height
		RECT_WIDTH = self.attribute_group.rect_width
		OPTION_HEIGHT = 50

		check_unchecked = pg.image.load(self.file_path + 'box_unchecked.png')
		check_checked = pg.image.load(self.file_path + 'box_checked.png')
		return_img = pg.image.load(self.file_path + 'return.png')

		font = self.attribute_group.main_font

		pg.draw.rect(display, self.attribute_group.header_color, pg.Rect(0, 0, WINDOW_WIDTH, HEADER_HEIGHT))
		pg.draw.rect(display, self.attribute_group.header_color, pg.Rect(0, WINDOW_HEIGHT-HEADER_HEIGHT, WINDOW_WIDTH, HEADER_HEIGHT))
		self.drawClock(display)

		title_text = font.render("MKA Settings", False, self.attribute_group.text_color)
		display.blit(title_text, (4,-3))

		for i in range(0,self.MAX_SELECTED):
			pg.draw.rect(display, self.attribute_group.border_color, pg.Rect(0, HEADER_HEIGHT + i*OPTION_HEIGHT, RECT_WIDTH, OPTION_HEIGHT))
			pg.draw.rect(display, self.attribute_group.border_outline, pg.Rect(0, HEADER_HEIGHT + i*OPTION_HEIGHT, RECT_WIDTH, OPTION_HEIGHT), 1)
			
			text = font.render(self.options[i], False, self.attribute_group.text_color)
			t_x = RECT_WIDTH + 10
			t_y = HEADER_HEIGHT + OPTION_HEIGHT*i

			display.blit(text, (t_x,t_y))

			if SETTINGS_MSG_BACK in self.options[i]:
				display.blit(return_img, (WINDOW_WIDTH - 170, t_y + OPTION_HEIGHT/2 - return_img.get_height()/2))
			elif SETTINGS_MSG_AUTOCONNECT in self.options[i]:
				if self.parameter_group.autoconnect:
					display.blit(check_checked, (WINDOW_WIDTH - 170, t_y + OPTION_HEIGHT/2 - check_checked.get_height()/2))
				else:
					display.blit(check_unchecked, (WINDOW_WIDTH - 170, t_y + OPTION_HEIGHT/2 - check_unchecked.get_height()/2))
			elif SETTINGS_MSG_HUD in self.options[i]:
				if self.parameter_group.audio_hud:
					display.blit(check_checked, (WINDOW_WIDTH - 170, t_y + OPTION_HEIGHT/2 - check_checked.get_height()/2))
				else:
					display.blit(check_unchecked, (WINDOW_WIDTH - 170, t_y + OPTION_HEIGHT/2 - check_unchecked.get_height()/2))

			if i == self.selected - 1:
				pg.draw.rect(display, self.attribute_group.rect_color, pg.Rect(0, HEADER_HEIGHT + i*OPTION_HEIGHT, RECT_WIDTH, OPTION_HEIGHT))
				pg.draw.rect(display, self.attribute_group.rect_color, pg.Rect(0, HEADER_HEIGHT + i*OPTION_HEIGHT, WINDOW_WIDTH - RECT_WIDTH*2, OPTION_HEIGHT), 5)

	def loadDefaultOptions(self):
		self.MAX_SELECTED = DEFAULT_MAX
		self.options = [""]*self.MAX_SELECTED
		add = 0
		if self.parameter_group.light_level > 0:
			self.MAX_SELECTED += 1
			self.options = [""]*self.MAX_SELECTED
			self.options[0] = SETTINGS_MSG_NIGHTSENS + str(self.parameter_group.night_level)
			add = 1
		self.options[add] = SETTINGS_MSG_AUTOCONNECT
		self.options[add + 1] = SETTINGS_MSG_SOURCE + "MKA" #TODO: Add source name.
		self.options[add + 2] = SETTINGS_MSG_HUD
		self.options[add + 3] = SETTINGS_MSG_BACK

	def makeSelection(self):
		selected = self.selected - 1
		if SETTINGS_MSG_BACK in self.options[selected]:
			self.parameter_group.next_menu = ParameterList.NEXTMENU_MIRROR_MENU
		elif SETTINGS_MSG_AUTOCONNECT in self.options[selected]:
			self.parameter_group.autoconnect = not self.parameter_group.autoconnect
		elif SETTINGS_MSG_HUD in self.options[selected]:
			self.parameter_group.audio_hud = not self.parameter_group.audio_hud