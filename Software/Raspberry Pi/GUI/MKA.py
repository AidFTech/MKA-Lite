import sys
import os

if os.path.exists("./GUI") and not os.path.exists("./MKA_Defaults.py"):
	sys.path.append("./GUI")

import pygame as pg
import MKA_Defaults as defaults

import MenuWindow
import MirrorMenuWindow
import SettingsMenuWindow
import ColorMenuWindow

from AttributeGroup import AttributeGroup
import ParameterList
import CarLinkList

class MKA:
	'''Fullscreen is defined as true if running on the Pi (defined in C). Full_interface is true if the full interface is required (e.g. for non-nav vehicles).'''
	def __init__(self, fullscreen: bool, full_interface: bool, file_path: str):
		pg.init()
		if fullscreen:
			pg.display.set_mode(flags=pg.FULLSCREEN)
			pg.mouse.set_visible(False)
		else:
			pg.display.set_mode(size=(defaults.WINDOW_WIDTH,defaults.WINDOW_HEIGHT))

		if not full_interface:
			pg.display.set_caption("MKA-Lite")
		else:
			pg.display.set_caption("MKA")

		self.full_interface = full_interface	#True if the full interface is required (e.g. for non-nav vehicles).
		self.fullscreen = fullscreen	#True if the program is running in fullscreen mode, i.e. on the Pi.
		self.display_surface = pg.Surface((defaults.WINDOW_WIDTH, defaults.WINDOW_HEIGHT))	#Render surface for the potentially scaled window.

		self.file_path = getFileRoot(file_path)	#The file path the script is being called from.

		path_str = self.file_path

		self.attribute_group = AttributeGroup()	#The assigned color/attribute group.
		self.attribute_group.main_font = pg.font.Font(path_str + 'ariblk.ttf', 32)	#The color group font.
		self.attribute_group.w = defaults.WINDOW_WIDTH
		self.attribute_group.h = defaults.WINDOW_HEIGHT
		self.attribute_group.header_height = defaults.HEADER_HEIGHT
		self.attribute_group.rect_width = defaults.RECT_WIDTH
		self.attribute_group.option_height = defaults.OPTION_HEIGHT
		
		self.parameter_list = ParameterList.ParameterList()	#The assigned parameter group.
		self.carlink_list = CarLinkList.CarLinkList(self.parameter_list)	#The assigned CarLinkList

		self.airplay_conf = open(path_str + 'airplay.conf','rb').read()	#A configuration file to be sent to the dongle.
		self.oem_logo = open(path_str + 'BMW.png', 'rb').read()	#The Android Auto icon to be sent to the dongle.
		self.icon_120 = open(path_str + 'BMW_icon.png', 'rb').read()	#A Carplay icon to be sent to the dongle.
		self.icon_180 = open(path_str + 'BMW_icon.png', 'rb').read()	#A Carplay icon to be sent to the dongle.
		self.icon_256 = open(path_str + 'BMW_icon.png', 'rb').read()	#A Carplay icon to be sent to the dongle.

		self.active_menu = MenuWindow.MenuWindow	#The active menu window.
		self.active_menu = None
		if not full_interface:
			self.active_menu = MirrorMenuWindow.MirrorMenuWindow(self.attribute_group, self.parameter_list, self.file_path)
			self.active_menu.setSelected(1)

		self.run = True	#True if the program is running.

	'''Loop function, to run while the Pi is running.'''
	def loop(self):
		self.display_surface.fill(self.attribute_group.br)

		if self.active_menu is not None:
			self.active_menu.displayMenu(self.display_surface)

		scaled_win = pg.transform.smoothscale(self.display_surface, (pg.display.get_surface().get_width(), pg.display.get_surface().get_height()))
		pg.display.get_surface().blit(scaled_win, (0,0))
		
		pg.display.update()

		self.checkNextWindow()
		self.run = self.handleEvents()

	'''Look for the <Escape> key or Close button.'''
	def handleEvents(self) -> bool:
		events = pg.event.get()
		for e in events:
			if e.type == pg.QUIT:
				return False
			elif e.type == pg.KEYDOWN:
				if e.key == pg.K_ESCAPE:
					return False
		
		return True
	
	'''Open a queued window.'''
	def checkNextWindow(self):
		next_menu = self.parameter_list.next_menu
		if next_menu > 0:
			if next_menu == ParameterList.NEXTMENU_MIRROR_MENU:
				self.active_menu = MirrorMenuWindow.MirrorMenuWindow(self.attribute_group, self.parameter_list, self.file_path)
			elif next_menu == ParameterList.NEXTMENU_SETTINGS_MENU:
				self.active_menu = SettingsMenuWindow.SettingsMenuWindow(self.attribute_group, self.parameter_list, self.file_path)
			elif next_menu == ParameterList.NEXTMENU_COLOR_MENU:
				self.active_menu = ColorMenuWindow.ColorMenuWindow(self.attribute_group, self.parameter_list, self.file_path)
			
			self.parameter_list.next_menu = ParameterList.NEXTMENU_NO_MENU

	'''IBus knob turn. "Clockwise" is true if the knob is turned clockwise.'''
	def knobTurn(self, clockwise: bool, count: int):
		if self.active_menu is None:
			return

		#TODO: Determine whether the phone mirror is active.
		if not clockwise:
			for i in range(count):
				self.active_menu.incrementSelected()
		else:
			for i in range(count):
				self.active_menu.decrementSelected()

	'''Enter button pressed. Normally this will call a function in the active menu.'''
	def handleEnterButton(self):
		#TODO: Determine whether the phone mirror is active.
		if self.active_menu is None:
			return
		else:
			self.active_menu.makeSelection()

	def setNightMode(self):
		if (self.parameter_list.headlights_on and self.parameter_list.light_level <= 0) or (self.parameter_list.light_level <= self.parameter_list.night_level and self.parameter_list.light_level > 0):
			print("Night Mode On") #TODO: Send the night mode message.
		else:
			print("Night Mode Off") #TODO: Send the message to cancel night mode.
	
def getFileRoot(fname: str) -> str:
	MYNAME = "MKA.py"
	if fname.find(MYNAME) < 0: #File is being called from the same directory as the C file.
		return ""

	the_return = fname.replace(MYNAME, "")
	return the_return
