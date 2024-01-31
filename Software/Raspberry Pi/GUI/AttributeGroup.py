import pygame as pg

'''Attribute group.'''
class AttributeGroup:
	pg.font.init()

	br = (40, 32, 95)
	text_color = (191, 191, 239)
	header_color = (103, 95, 143)
	rect_color = (239, 96, 32)
	border_color = (215, 215, 239)
	border_outline = (0, 0, 0)
	
	main_font = pg.font.Font
	w = 0
	h = 0
	header_height = 0
	rect_width = 0
	option_height = 0