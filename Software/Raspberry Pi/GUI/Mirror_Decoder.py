
# "Autobox" dongle driver for HTML 'streaming'
# Created by Colin Munro, December 2019
# See README.md for more information

"""Simple utility code to decode an h264 stream to a series of PNGs."""

from enum import Enum, IntEnum
from os import truncate
from Mirror_USBLink import Error

import CarLinkList

import threading, os
import mpv
from PIL import Image, ImageDraw, ImageFont

class KeyEvent(IntEnum):
	BUTTON_SIRI = 5
	BUTTON_LEFT = 100
	BUTTON_RIGHT = 101
	BUTTON_ANDROID_UP = 102
	BUTTON_ANDROID_DOWN = 103
	BUTTON_SELECT_DOWN = 104
	BUTTON_SELECT_UP = 105 
	BUTTON_BACK = 106
	BUTTON_DOWN = 114
	BUTTON_HOME = 200
	BUTTON_PLAY = 201
	BUTTON_PAUSE = 202
	BUTTON_TOGGLE = 203
	BUTTON_NEXT_TRACK = 204
	BUTTON_PREV_TRACK = 205

	NIGHT_MODE = 16
	DAY_MODE = 17
class Decoder:
	"""Video decoder object."""
	class _Thread(threading.Thread):
		"""Video decoder thread."""
		def __init__(self, owner, full: bool, w=720, h=480):
			super().__init__()
			self.owner = owner
			self.running = threading.Event()
			self.shutdown = False
			
			player = mpv.MPV(log_handler=self.owner.log, input_default_bindings=False, input_vo_keyboard=True, hwdec="rpi", hwdec_codecs="h264", untimed=True, opengl_glfinish=True, hwdec_extra_frames = 75, audio=False, demuxer_rawvideo_fps=30, fps=30, video_aspect_override = w/h)
			self.player = player
			player.fullscreen = full
			
		def run(self):
			"""Thread loop function."""
			player = self.player
		
			@player.python_stream('mirror_video')
			def reader():
				while not self.shutdown:
					try:
						yield os.read(self.owner.readPipe, 2048)
					except Error:
						print("something broke on reading the MPV input pipe")
	
	def __init__(self, full: bool, link_list: CarLinkList.CarLinkList, w=720, h=480):	
		# self.child = subprocess.Popen(["mpv", "--hwdec=rpi", "--demuxer-rawvideo-fps=60", "--fps=60", "-"], stdin=subprocess.PIPE, stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL, bufsize=1)	
		
		self.readPipe, self.writePipe = os.pipe()
		
		self.playing = False
		self.thread = self._Thread(self,full,w,h)
		self.player = self.thread.player
		self.thread.start()

		self.link_list = link_list

		self.overlay_font = ImageFont.truetype('arial.ttf', 32)
		self.overlay = self.player.create_image_overlay()
		#self.overlay_image = Image.new('RGBA', (self.link_list.attributes.w, self.link_list.attributes.header_height), self.link_list.attributes.header_color)
		#self.overlay_image_draw = ImageDraw.Draw(self.overlay_image)

		if not self.playing:
			self.player.play('python://mirror_video')
			self.playing = True

	def stop(self):
		"""Stop playing video."""
		# self.child.terminate()
		self.playing = False
		self.thread.shutdown = True
		self.thread.join()
		self.player.terminate()

	def send(self, data: bytes):
		"""Send video data."""
		os.write(self.writePipe, data)
		
	#	if not self.playing:
	#		self.player.play('python://mirror_video')
	#		self.playing = True
		# self.child.stdin.write(data)
		# self.child.stdin.flush()

	def setWindow(self, window_status: bool):
		"""Set whether the window is minimized, i.e. the phone screen is active."""
		self.player.window_minimized=not window_status

	def setOverlayText(self, text: str):
		"""Set overlay text on the window."""
		if len(text) > 0:
			overlay_image = Image.new('RGBA', (self.link_list.attributes.w, self.link_list.attributes.header_height), self.link_list.attributes.header_color)
			overlay_image_draw = ImageDraw.Draw(overlay_image)
			overlay_image_draw.text((0,0), text, font= self.overlay_font, fill=self.link_list.attributes.text_color)
			self.overlay.update(overlay_image, (0,0))
		else:
			self.overlay.remove()

	def getWindow(self) -> bool:
		"""Returns whether the window is minimized."""
		return not self.player.window_minimized
	
	def getWritePipe(self) -> int:
		"""Return the write pipe- this may be leftover..."""
		return self.writePipe

	def log(self, loglevel, component, message):
		print('[{}] {}: {}'.format(loglevel, component, message))
