import BMirror

import pygame as pg
from pygame import Rect

window_width = 800
window_height = 480

margin_width = 60
small_rect_size = 70
header_height = 40
rect_w = 20

class PhoneScreen:
	selected = 1
	
	def __init__(self, color_group, parent, phone_name):
		self.color_group = color_group
		self.parent = parent
		self.phone_name = phone_name
	
	def displayMenu(self, display):
		display.fill(self.color_group.br)
		
		return_img = pg.image.load('return.png')
		
		font = self.color_group.main_font
		t_x = margin_width
		t_y = margin_width
			
		if self.phone_name != "":
			phone_message = self.phone_name + " is connecting."
		else:
			phone_message = "Phone is connecting."
			
		text = font.render(phone_message, False, self.color_group.text_color)
		display.blit(text, (t_x,t_y))
		
		r_x = window_width - rect_w
		r_y = window_height - small_rect_size - header_height 
		r_h = small_rect_size
		
		pg.draw.rect(display, self.color_group.border_color, Rect(r_x, r_y, rect_w, r_h))
		pg.draw.rect(display, self.color_group.border_outline, Rect(r_x, r_y, rect_w, r_h), 1)
		
		display.blit(return_img, (window_width - rect_w*2 - return_img.get_width(), window_height - small_rect_size - header_height + small_rect_size/2 - return_img.get_height()/2))
		
		pg.draw.rect(display, self.color_group.header_color, Rect(0, 0, window_width, header_height))
		pg.draw.rect(display, self.color_group.header_color, Rect(0, window_height-header_height, window_width, header_height))
		
		if self.selected == 1:
			pg.draw.rect(display, self.color_group.rect_color, Rect(r_x, r_y, rect_w, r_h))
			r_x = int(window_width/2)
			r_w = int(window_width/2)
			pg.draw.rect(display, self.color_group.rect_color, Rect(r_x, r_y, r_w, r_h), 5)
		
		pg.display.update()
		
	def makeSelection(self, item):
		if item == 1: 
			self.parent.mirror.decoder.setWindow(False)
			self.parent.openMainMenu()
