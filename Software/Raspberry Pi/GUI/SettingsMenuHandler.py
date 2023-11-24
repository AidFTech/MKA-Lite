import sys
import os

import pygame as pg
from pygame import Rect

window_width = 800
window_height = 480

margin_width = 60
right_margin_width = 200
header_height = 40
rect_w = 20

opt_pairapple = "Pair Apple Device"
opt_pairandroid = "Pair Android Device"

opt_listdevices = "Device List"
opt_connect = "Connect Device"

opt_lightsens = "Night Mode Sensitivity"

opt_autoconnect = "Auto Connect"
opt_back = "Back"

class SettingsMenu:
	options = [opt_pairapple, opt_pairandroid, opt_listdevices, opt_connect, opt_autoconnect, opt_back]
	selected = 1
	
	def __init__(self, color_group, parent):
		self.options = [opt_pairapple, opt_pairandroid, opt_listdevices, opt_connect, opt_autoconnect, opt_back]
		self.color_group = color_group
		self.parent = parent

		if hasattr(self.parent, "light_thresh") and hasattr(self.parent, "RLS_connected") and self.parent.RLS_connected:
			light_str = opt_lightsens + ": " + str(self.parent.light_thresh)
			self.options.insert(len(self.options)-2, light_str)
	
	def displayMenu(self, display):
		display.fill(self.color_group.br)
		
		font = self.color_group.main_font
		
		check_unchecked = pg.image.load('box_unchecked.png')
		check_checked = pg.image.load('box_checked.png')
		return_img = pg.image.load('return.png')
		
		pg.draw.rect(display, self.color_group.header_color, Rect(0, 0, window_width, header_height))
		pg.draw.rect(display, self.color_group.header_color, Rect(0, window_height-header_height, window_width, header_height))
		
		title_text = font.render("AMirror Settings", False, self.color_group.text_color)
		display.blit(title_text, (4,-3))
		
		text_h = font.render(self.options[0], False, self.color_group.text_color).get_height()
		for i in range(0,len(self.options)):
			text = font.render(self.options[i], False, self.color_group.text_color)
			t_x = margin_width
			t_y = header_height + i*text.get_height()
			
			display.blit(text, (t_x,t_y))
			
			pg.draw.rect(display, self.color_group.border_color, Rect(0, t_y, rect_w, text_h))
			pg.draw.rect(display, self.color_group.border_outline, Rect(0, t_y, rect_w, text_h),1)
			
			if self.options[i] == opt_autoconnect and hasattr(self.parent, "mirror"):
				if self.parent.mirror.autostart:
					display.blit(check_checked, (window_width - right_margin_width + 30, t_y + text_h/2 - check_checked.get_height()/2))
				else:
					display.blit(check_unchecked, (window_width - right_margin_width + 30, t_y + text_h/2 - check_unchecked.get_height()/2))
			elif self.options[i] == opt_back:
				display.blit(return_img, (window_width - right_margin_width + 30, t_y + text_h/2 - return_img.get_height()/2))
			
		r_x = 0
		r_y = header_height + (self.selected-1)*text_h
		r_w = int(window_width - right_margin_width)
		r_h = text_h
		
		pg.draw.rect(display, self.color_group.rect_color, Rect(r_x, r_y, r_w, r_h), 5)
		pg.draw.rect(display, self.color_group.rect_color, Rect(r_x, r_y, rect_w, r_h))
		
		pg.display.update()
		
	def makeSelection(self, item):
		if self.options[item-1] == opt_back:
			self.parent.openMainMenu()
		elif self.options[item - 1] == opt_autoconnect:
			if hasattr(self.parent, "mirror"):
				self.parent.mirror.autostart = not self.parent.mirror.autostart
		elif opt_lightsens in self.options[item - 1] and hasattr(self.parent, "light_thresh"):
			if self.parent.light_thresh < 6:
				self.parent.light_thresh += 1
			else:
				self.parent.light_thresh = 1
			light_str = opt_lightsens + ": " + str(self.parent.light_thresh)
			self.options[item - 1] = light_str
