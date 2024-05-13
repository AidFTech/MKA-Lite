import sys
import os

if os.path.exists("./GUI") and not os.path.exists("./MKA_Defaults.py"):
    sys.path.append("./GUI")

import pygame as pg
import time
import MKA_Defaults as defaults

import MenuWindow
import MirrorMenuWindow
import SettingsMenuWindow
import ColorMenuWindow
import NightModeMenuWindow

from AttributeGroup import AttributeGroup
import ParameterList
import CarLinkList

import Mirror_MirrorHandler
import Mirror_Decoder

class MKA:
    def __init__(self, fullscreen: bool, full_interface: bool):
        """Fullscreen is defined as true if running on the Pi (defined in C). Full_interface is true if the full interface is required (e.g. for non-nav vehicles)."""
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

        self.file_path = __file__.replace(os.path.basename(__file__), '')

        self.attribute_group = AttributeGroup()	#The assigned color/attribute group.
        self.attribute_group.main_font = pg.font.Font(self.file_path + 'ariblk.ttf', 32)	#The color group font.
        self.attribute_group.w = defaults.WINDOW_WIDTH
        self.attribute_group.h = defaults.WINDOW_HEIGHT
        self.attribute_group.header_height = defaults.HEADER_HEIGHT
        self.attribute_group.rect_width = defaults.RECT_WIDTH
        self.attribute_group.option_height = defaults.OPTION_HEIGHT

        self.parameter_list = ParameterList.ParameterList()	#The assigned parameter group.
        self.carlink_list = CarLinkList.CarLinkList(self.parameter_list, self.attribute_group)	#The assigned CarLinkList

        self.last_radio_title = self.parameter_list.main_radio_title

        self.parameter_list.fullscreen = fullscreen

        airplay_conf = open(self.file_path + 'airplay.conf','rb').read()	#A configuration file to be sent to the dongle.
        oem_logo = open(self.file_path + 'BMW.png', 'rb').read()	#The Android Auto icon to be sent to the dongle.
        icon_120 = open(self.file_path + 'BMW_icon.png', 'rb').read()	#A Carplay icon to be sent to the dongle.
        icon_180 = open(self.file_path + 'BMW_icon.png', 'rb').read()	#A Carplay icon to be sent to the dongle.
        icon_256 = open(self.file_path + 'BMW_icon.png', 'rb').read()	#A Carplay icon to be sent to the dongle.

        self.carlink_list.oem_logo = oem_logo
        self.carlink_list.airplay_conf = airplay_conf
        self.carlink_list.icon_120 = icon_120
        self.carlink_list.icon_180 = icon_180
        self.carlink_list.icon_256 = icon_256

        self.active_menu = MenuWindow.MenuWindow	#The active menu window.
        self.active_menu = None
        if not full_interface:
            self.active_menu = MirrorMenuWindow.MirrorMenuWindow(self.attribute_group, self.parameter_list, self.file_path)
            self.active_menu.setSelected(1)

        self.mirror = Mirror_MirrorHandler.MirrorHandler(self.carlink_list, self.file_path)

        self.run = True	#True if the program is running.
        self.overlay_timer_run = False
        self.overlay_timer = time.perf_counter()

    def loop(self):
        """Loop function, to run while the Pi is running."""
        self.display_surface.fill(self.attribute_group.br)

        self.mirror.loop()
        self.parameter_list.dongle_connected = self.mirror.usb_link.startup

        if self.active_menu is not None:
            self.active_menu.displayMenu(self.display_surface)

            scaled_win = pg.transform.smoothscale(self.display_surface, (pg.display.get_surface().get_width(), pg.display.get_surface().get_height()))
            pg.display.get_surface().blit(scaled_win, (0,0))

            pg.display.update()

            if self.last_radio_title != self.parameter_list.main_radio_title:
                self.last_radio_title = self.parameter_list.main_radio_title
                self.setOverlayText(self.parameter_list.main_radio_title, True)

            if self.overlay_timer_run and time.perf_counter() - self.overlay_timer >= 5:
                self.overlay_timer_run = False
                self.setOverlayText("", False)

            self.checkNextWindow()
            self.run = self.handleEvents() and self.run

            if not self.run:
                self.mirror.stopAll()


    def handleEvents(self) -> bool:
        """Look for the <Escape> key or Close button."""
        events = pg.event.get()
        for e in events:
            if e.type == pg.QUIT:
                return False
            elif e.type == pg.KEYDOWN:
                if e.key == pg.K_ESCAPE:
                    return False
        return True

    def checkNextWindow(self):
        """Open a queued window."""
        next_menu = self.parameter_list.next_menu
        if next_menu > 0:
            if next_menu == ParameterList.NEXTMENU_MIRROR_MENU:
                self.active_menu = MirrorMenuWindow.MirrorMenuWindow(self.attribute_group, self.parameter_list, self.file_path)
            elif next_menu == ParameterList.NEXTMENU_SETTINGS_MENU:
                self.active_menu = SettingsMenuWindow.SettingsMenuWindow(self.attribute_group, self.parameter_list, self.file_path)
            elif next_menu == ParameterList.NEXTMENU_COLOR_MENU:
                self.active_menu = ColorMenuWindow.ColorMenuWindow(self.attribute_group, self.parameter_list, self.file_path)
            elif next_menu == ParameterList.NEXTMENU_NIGHT_SENS_MENU:
                self.active_menu = NightModeMenuWindow.NightModeMenuWindow(self.attribute_group, self.parameter_list, self.file_path)

                self.parameter_list.next_menu = ParameterList.NEXTMENU_NO_MENU

    def setVersion(self, data0: int, data1: int):
        """Set the GT version."""
        version = bytearray([data0, data1])
        self.parameter_list.version = int(version.decode())

    def knobTurn(self, clockwise: bool, count: int):
        """IBus knob turn. "Clockwise" is true if the knob is turned clockwise."""
        if self.active_menu is None:
            return

        if not self.mirror.getWindow():
            if not clockwise:
                for i in range(count):
                    self.active_menu.incrementSelected()
                else:
                    for i in range(count):
                        self.active_menu.decrementSelected()
            else:
                if clockwise:
                    for i in range(count):
                        self.mirror.sendMirrorCommand(Mirror_Decoder.KeyEvent.BUTTON_RIGHT)
                    else:
                        for i in range(count):
                            self.mirror.sendMirrorCommand(Mirror_Decoder.KeyEvent.BUTTON_LEFT)

    def handleEnterButton(self):
        """Enter button pressed. Normally this will call a function in the active menu."""
        if not self.mirror.getWindow():
            if self.active_menu is None:
                return
            else:
                self.active_menu.makeSelection()
        else:
            self.mirror.sendMirrorCommand(Mirror_Decoder.KeyEvent.BUTTON_SELECT_DOWN)
            self.mirror.sendMirrorCommand(Mirror_Decoder.KeyEvent.BUTTON_SELECT_UP)

    def handleBackButton(self):
        """Back button pressed. Normally this will call a function in the active menu."""
        if not self.mirror.getWindow():
            if self.active_menu is None:
                return
            else:
                self.active_menu.goBack()
        else:
            self.mirror.sendMirrorCommand(Mirror_Decoder.KeyEvent.BUTTON_BACK)

    def handleHomeButton(self):
        """Home button pressed. Normally this will call a function in the active menu."""
        if not self.mirror.getWindow():
            if self.active_menu is not MirrorMenuWindow.MirrorMenuWindow:
                self.parameter_list.next_menu = ParameterList.NEXTMENU_MIRROR_MENU
            else:
                self.mirror.sendMirrorCommand(Mirror_Decoder.KeyEvent.BUTTON_HOME)

    def handleSeekButton(self, right: bool):
        """One of the seek buttons pressed. "Right" is true if it was the right button that was pressed."""
        if self.parameter_list.audio_selected:
            if right:
                self.mirror.sendMirrorCommand(Mirror_Decoder.KeyEvent.BUTTON_NEXT_TRACK)
            else:
                self.mirror.sendMirrorCommand(Mirror_Decoder.KeyEvent.BUTTON_PREV_TRACK)

    def handleDirectionButton(self):
        """Direction button pressed. Play/pause."""
        if self.parameter_list.audio_selected:
            self.parameter_list.playing = not self.parameter_list.playing
            self.mirror.sendMirrorCommand(Mirror_Decoder.KeyEvent.BUTTON_TOGGLE)

    def setNightMode(self):
        """Set whether night mode is active."""
        if (self.parameter_list.headlights_on and self.parameter_list.light_level <= 0) or (self.parameter_list.light_level <= self.parameter_list.night_level and self.parameter_list.light_level > 0):
            self.mirror.sendMirrorCommand(Mirror_Decoder.KeyEvent.NIGHT_MODE)
        else:
            self.mirror.sendMirrorCommand(Mirror_Decoder.KeyEvent.DAY_MODE)

    def setSelected(self):
        """Send a message to start/stop music if the source is changed."""
        if self.parameter_list.audio_selected:
            self.mirror.sendMirrorCommand(Mirror_Decoder.KeyEvent.BUTTON_PLAY)
        else:
            self.mirror.sendMirrorCommand(Mirror_Decoder.KeyEvent.BUTTON_PAUSE)

    def setOverlayText(self, text: str, clear: bool):
        """Set header overlay text. If clear is True, clear the message after a few seconds."""
        if self.mirror.decoder is not None and self.parameter_list.audio_hud:
            self.mirror.decoder.setOverlayText(text)

            if clear:
                self.overlay_timer_run = True
                self.overlay_timer = time.perf_counter()

if __name__ == '__main__':
    mka = MKA(True, True)
    mka.loop()
