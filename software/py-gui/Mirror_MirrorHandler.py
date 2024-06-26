import ParameterList
import CarLinkList
import threading
import struct
import time

import pygame as pg

import Mirror_USBLink
import Mirror_Protocol
import Mirror_Decoder
import Mirror_AudioDecoder

AUDIO_WAIT = 0.2 #The amount of time to wait before changing the audio symbol to a ||.

class MirrorHandler:
    """Phone mirror handler object."""
    def __init__(self, link_list: CarLinkList.CarLinkList, file_path: str):
        self.link_list = link_list
        self.parameters = link_list.parameters
        self.usb_link = Mirror_USBLink.USB_Connection(self.link_list, self)

        self.file_path = file_path #Filepath for the decoder overlay font.

        self.run = True

        self.startup_thread = threading.Thread(target = self.connectDongleThread)
        self.startup_thread.start()

        self.decoder = None
        self.audio_decoder = Mirror_AudioDecoder.AudioDecoder()
        self.audio_timer = time.perf_counter()

        self.videomem_data = bytes([0]*0)

    def loop(self):
        """Mirror loop function."""
        cache_len = len(self.link_list.rx_cache)
        if cache_len == 0:
            return
        for i in range(0, cache_len):
            msg = self.link_list.rx_cache[i]
            self.interpretMessage(msg)
        self.link_list.rx_cache.clear()
        if time.perf_counter() - self.audio_timer > AUDIO_WAIT:
            self.parameters.playing = False

    def interpretMessage(self, msg: Mirror_Protocol.Message):
        """Interpret a message from the dongle."""
        if isinstance(msg, Mirror_Protocol.Open):	#Startup message.
            self.usb_link.sendMultiple(Mirror_Protocol.opened_info)
            self.usb_link.sendMessage(Mirror_Protocol.MetaData(mediaDelay = 300, androidAutoSizeW = 800, AndroidAutoSizeH = 480))

            msg_91 = Mirror_Protocol.Message(9)
            msg_91._setdata(bytes([1]))
            self.usb_link.sendMessage(msg_91)

            msg_88 = Mirror_Protocol.Message(0x88)
            msg_88._setdata(bytes([1]))
            self.usb_link.sendMessage(msg_88)

            self.usb_link.startDongle()
        elif isinstance(msg, Mirror_Protocol.Plugged):	#Phone connected.
            self.parameters.phone_type = int(msg.phone_type)
            self.startPhoneConnection()
        elif isinstance(msg, Mirror_Protocol.Unplugged):	#Phone disconnected.
            self.parameters.phone_type = 0
            self.parameters.phone_name = ""
            self.stopPhoneConnection()
        elif isinstance(msg, Mirror_Protocol.VideoData):	#Video data. Cache.
            self.videomem_data = msg.data
            if self.decoder is not None:
                self.decoder.send(msg.data)
        elif isinstance(msg, Mirror_Protocol.MetaData):	#Metadata. Interpret.
            self.handleMetaData(msg)
        elif isinstance(msg, Mirror_Protocol.AudioData):	#Audio data. Handle appropriately.
            self.sendAudio(msg)

    def connectDongleThread(self):
        """Dongle connection loop."""
        while not self.usb_link.running and self.run:
            connected = self.usb_link.connectDongle()
            if connected:
                while self.usb_link.running and not self.usb_link.startup and self.run:
                    self.usb_link.sendMultiple(Mirror_Protocol.startup_info)
                    self.usb_link.sendMessage(Mirror_Protocol.SendFile("/etc/airplay.conf", self.link_list.airplay_conf))
                    self.usb_link.sendMessage(Mirror_Protocol.SendFile("/etc/oem_icon.png", self.link_list.oem_logo))
                    self.usb_link.sendMessage(Mirror_Protocol.SendFile("/etc/icon_120x120.png", self.link_list.icon_120))
                    self.usb_link.sendMessage(Mirror_Protocol.SendFile("/etc/icon_180x180.png", self.link_list.icon_180))
                    self.usb_link.sendMessage(Mirror_Protocol.SendFile("/etc/icon_256x256.png", self.link_list.icon_256))

                    time.sleep(1)

                    if (not self.usb_link.running) and (not self.usb_link.startup):
                        self.usb_link.stop()

    def startPhoneConnection(self):
        """Start a phone connection."""
        if self.decoder is None:
            self.decoder = Mirror_Decoder.Decoder(
                self.parameters.fullscreen,
                self.link_list,
                self.file_path,
                pg.display.get_surface().get_width(),
                pg.display.get_surface().get_height()
            )

            if not self.audio_decoder.running():
                self.audio_decoder.start()

            self.decoder.setWindow(self.parameters.autoconnect)

    def stopPhoneConnection(self):
        """End a phone connection."""
        self.parameters.next_menu = ParameterList.NEXTMENU_MIRROR_MENU

        self.parameters.song_title = ""
        self.parameters.artist = ""
        self.parameters.album = ""
        self.parameters.app = ""

        if self.decoder is not None:
            self.decoder.send(self.videomem_data)
            self.decoder.stop()
            self.decoder = None

            self.audio_decoder.stop()

    def getWindow(self) -> bool:
        """Return whether the window is minimized or full."""
        if self.decoder is not None:
            return self.decoder.getWindow()
        else:
            return False

    def sendMirrorCommand(self, command: int):
        """Send a CarPlay command to the phone."""
        command_msg = Mirror_Protocol.CarPlay()
        command_data = struct.pack("<L", command)
        command_msg._setdata(command_data)
        self.usb_link.sendMessage(command_msg)

    def handleMetaData(self, msg: Mirror_Protocol.MetaData):
        """Handle a metadata message, e.g. song title."""
        if hasattr(msg, "MDModel"):
            self.parameters.phone_name = str(msg.MDModel)

        if hasattr(msg, "MediaSongName"):
            self.parameters.song_title = str(msg.MediaSongName)

        if hasattr(msg, "MediaArtistName"):
            self.parameters.artist = str(msg.MediaArtistName)

        if hasattr(msg, "MediaAlbumName"):
            self.parameters.album = str(msg.MediaAlbumName)

        if hasattr(msg, "MediaAPPName"):
            self.parameters.app = str(msg.MediaAPPName)
            if not hasattr(msg, "MediaSongName"):
                self.parameters.song_title = ""
                if not hasattr(msg, "MediaArtistName"):
                    self.parameters.artist = ""
                if not hasattr(msg, "MediaAlbumName"):
                    self.parameters.album = ""

    def sendVideo(self, msg: Mirror_Protocol.VideoData):
        """Send a video message."""
        if self.decoder is not None:
            self.videomem_data = msg.data
            if self.decoder is not None:
                self.decoder.send(msg.data)

    def sendAudio(self, msg: Mirror_Protocol.AudioData):
        """Send or handle an audio message."""
        self.audio_timer = time.perf_counter()
        self.parameters.playing = True

        if hasattr(msg, "audioType") and msg.audioType == 1 and not self.parameters.audio_selected:
            pass #TODO: Force a switch to the MKA. This could be coming from Siri or Google Assistant.

            if hasattr(msg, "data"):
                try:
                    self.audio_decoder.sendAudioData(msg.data)
                except Exception as e:
                    print(f"exception: {e}")


    def stopAll(self):
        """Stop everything, e.g. if the car is turned off."""
        self.run = False
        self.stopPhoneConnection()
        self.usb_link.stop()
        try:
            if self.startup_thread.is_alive():
                self.startup_thread.join()
        except AttributeError:
            pass
