import BMirror

import pygame as pg
from pygame import Rect

window_width = 800
window_height = 480

margin_width = 60
small_rect_size = 70
header_height = 40
rect_w = 20

class MainMenu:
	options = ["Apple CarPlay", "Android Auto", "Setup", " "]
	selected = 1
	
	def __init__(self, color_group, parent):
		self.color_group = color_group
		self.parent = parent
	
	def displayMenu(self, display):
		display.fill(self.color_group.br)
		
		carplay_img = pg.image.load('Apple_CarPlay_Logo_100.png')
		android_img = pg.image.load('Android_Auto_icon_100.png')
		return_img = pg.image.load('return.png')
		
		display.blit(carplay_img, (150, 120))
		display.blit(android_img, (575, 120))
		
		font = self.color_group.main_font
		for i in range(0,len(self.options)):
			text = font.render(self.options[i], False, self.color_group.text_color)
			t_x = margin_width
			t_y = margin_width
			r_x = 0
			r_y = header_height
			r_h = int(window_height/2)

			if i%2 == 1:
				t_x = window_width - margin_width - text.get_width()
				r_x = window_width - rect_w

			if i >= 2:
				t_y = window_height - margin_width - text.get_height()
				r_y = window_height - small_rect_size - header_height
				r_h = small_rect_size

			display.blit(text, (t_x,t_y))
			pg.draw.rect(display, self.color_group.border_color, Rect(r_x, r_y, rect_w, r_h))
			pg.draw.rect(display, self.color_group.border_outline, Rect(r_x, r_y, rect_w, r_h), 1)
		
		if hasattr(self.parent, "carplay_name") and self.parent.carplay_name != "":
			carplay_text = font.render(self.parent.carplay_name, False, self.color_group.text_color)
			t_x = 200 - carplay_text.get_width()/2
			t_y = 240
			
			display.blit(carplay_text, (t_x, t_y))
		
		if hasattr(self.parent, "android_name") and self.parent.android_name != "":
			android_text = font.render(self.parent.android_name, False, self.color_group.text_color)
			t_x = 625 - android_text.get_width()/2
			t_y = 240
			
			display.blit(android_text, (t_x, t_y))
		
		display.blit(return_img, (window_width - rect_w*2 - return_img.get_width(), window_height - small_rect_size - header_height + small_rect_size/2 - return_img.get_height()/2))
		
		pg.draw.rect(display, self.color_group.header_color, Rect(0, 0, window_width, header_height))
		pg.draw.rect(display, self.color_group.header_color, Rect(0, window_height-header_height, window_width, header_height))
		
		title_text = font.render("MKA Lite Main Menu", False, self.color_group.text_color)
		display.blit(title_text, (4,-3))
		
		if hasattr(self.parent, "time_clock") and self.parent.time_clock != "":
			time_text = font.render(self.parent.time_clock, False, self.color_group.text_color)
			display.blit(time_text, (4, window_height-header_height-3))
		
		if hasattr(self.parent, "date") and self.parent.date != "":
			date_text = font.render(self.parent.date, False, self.color_group.text_color)
			display.blit(date_text, (window_width - date_text.get_width() - 20, window_height-header_height-3))
		
		r_x = 0
		r_y = header_height
		r_w = int(window_width/2)
		r_h = int(window_height/2)

		if self.selected == 2 or self.selected == 3:
			r_y = window_height - small_rect_size - header_height
			r_h = small_rect_size

		if self.selected == 3 or self.selected == 4:
			r_x = window_width/2

		pg.draw.rect(display, self.color_group.rect_color, Rect(r_x, r_y, r_w, r_h), 5)
		
		if self.selected == 3 or self.selected == 4:
			r_x = window_width - rect_w
		pg.draw.rect(display, self.color_group.rect_color, Rect(r_x, r_y, rect_w, r_h))
		
		pg.display.update()
		
	def makeSelection(self, item):
		if item == 1: #Start CarPlay
			if hasattr(self.parent, "carplay_connected") and self.parent.carplay_connected and hasattr(self.parent, "mirror"):
				self.parent.openPhoneConnectScreen(self.parent.carplay_name)
				self.parent.mirror.decoder.setWindow(True)
		elif item == 4: #Start Android
			if hasattr(self.parent, "android_connected") and self.parent.android_connected and hasattr(self.parent, "mirror"):
				self.parent.openPhoneConnectScreen(self.parent.android_name)
				self.parent.mirror.decoder.setWindow(True)
		elif item == 2: #Setup
			self.parent.openSettingsMenu()
		elif item == 3: #Return to MKIV
			self.parent.endVMControl()
