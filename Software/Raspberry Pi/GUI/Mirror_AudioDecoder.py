import subprocess

class AudioDecoder:
	def __init__(self):
		self.decoder = None
		
	def start(self):
		self.decoder = subprocess.Popen(["ffplay", "-f", "s16le", "-ac", "2", "-ar", "44100", "-nodisp", "-"], stdin = subprocess.PIPE, stdout = subprocess.DEVNULL, stderr=subprocess.DEVNULL, bufsize = 1)
	
	def stop(self):
		if self.decoder is not None:
			self.decoder.terminate()

	def sendAudioData(self, data: bytes):
		if self.decoder is not None:
			self.decoder.stdin.write(data)
			self.decoder.stdin.flush()

	def running(self) -> bool:
		return self.decoder is not None