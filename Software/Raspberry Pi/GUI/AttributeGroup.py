import pygame as pg

'''Attribute group.'''
class AttributeGroup:
	pg.font.init()

	br = (40, 32, 95)	#Background color.
	text_color = (191, 191, 239)	#Text color.
	header_color = (103, 95, 143)	#Header color.
	rect_color = (239, 96, 32)	#Selecting rectangle color.
	border_color = (215, 215, 239)	#Unselected option border color.
	border_outline = (0, 0, 0)	#Unselected option border outline color.
	
	main_font = pg.font.Font	#Font.
	w = 0	#Display width.
	h = 0	#Display height.
	header_height = 0	#Header height.
	rect_width = 0	#Unselected option rectangle width.
	option_height = 0	#Option height.