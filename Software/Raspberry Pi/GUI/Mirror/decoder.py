
# "Autobox" dongle driver for HTML 'streaming'
# Created by Colin Munro, December 2019
# See README.md for more information

"""Simple utility code to decode an h264 stream to a series of PNGs."""

from enum import Enum, IntEnum
from os import truncate
from usb_link import Error
import threading, os
import mpv

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
class Decoder:
	class _Thread(threading.Thread):
		def __init__(self, owner):
			super().__init__()
			self.owner = owner
			self.running = threading.Event()
			self.shutdown = False
			
			player = mpv.MPV(log_handler=self.owner.log, input_default_bindings=False, input_vo_keyboard=True, hwdec="rpi", hwdec_codecs="h264", untimed=True, opengl_glfinish=True, hwdec_extra_frames = 75, audio=False, demuxer_rawvideo_fps=30, fps=30)
			self.owner.player = player
			player.fullscreen = True
			
		def run(self):
			player = self.owner.player
		
			@player.python_stream('mirror_video')
			def reader():
				while not self.shutdown:
					try:
						# yield os.read(self.owner.readPipe, 4012)
						#yield os.read(self.owner.readPipe, 2048)
						yield os.read(self.owner.readPipe, 1000000)
					except Error:
						print("something broke on reading the MPV input pipe")
			
			@player.on_key_press('LEFT') 
			def left():
				self.owner.on_key_event(KeyEvent.BUTTON_LEFT) 
			@player.on_key_press('RIGHT')
			def right():
				self.owner.on_key_event(KeyEvent.BUTTON_RIGHT)
			@player.on_key_press('ENTER')
			def enter():
				self.owner.on_key_event(KeyEvent.BUTTON_SELECT_DOWN)
			@player.on_key_press('ESC') #back
			def esc():
				self.owner.on_key_event(KeyEvent.BUTTON_BACK)
			@player.on_key_press('SPACE') #play
			def space():
				self.owner.on_key_event(KeyEvent.BUTTON_PLAY)
			@player.on_key_press('s') #siri
			def s():
				self.owner.on_key_event(KeyEvent.BUTTON_SIRI)
			@player.on_key_press('p') #pause
			def p(): 
				self.owner.on_key_event(KeyEvent.BUTTON_PAUSE)
			@player.on_key_press('e')
			def fforward():
				self.owner.on_key_event(KeyEvent.BUTTON_NEXT_TRACK)
			@player.on_key_press('r')
			def rewind():
				self.owner.on_key_event(KeyEvent.BUTTON_PREV_TRACK)
			@player.on_key_press('h')
			def home():
				self.owner.on_key_event(KeyEvent.BUTTON_HOME)
			@player.on_key_press('f')
			def fullscreen():
				player.fullscreen = not player.fullscreen
	
	def __init__(self):	
		# self.child = subprocess.Popen(["mpv", "--hwdec=rpi", "--demuxer-rawvideo-fps=60", "--fps=60", "-"], stdin=subprocess.PIPE, stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL, bufsize=1)	
		
		self.readPipe, self.writePipe = os.pipe()
		
		self.playing = False
		self.thread = self._Thread(self)
		self.thread.start()

	def stop(self):
		# self.child.terminate()
		self.playing = False
		self.thread.shutdown = True
		self.thread.join()
		self.player.terminate()

	def send(self, data):
		os.write(self.writePipe, data)
		
		if not self.playing:
			self.player.play('python://mirror_video')
			self.playing = True
		# self.child.stdin.write(data)
		# self.child.stdin.flush()
		
	def setWindow(self, window_status):
		self.player.window_minimized=not window_status

	def on_key_event(self,event):
		"""Callback for when a key event is received from the video player [called from a worker thread]."""

	def log(self, loglevel, component, message):
		print('[{}] {}: {}'.format(loglevel, component, message))
