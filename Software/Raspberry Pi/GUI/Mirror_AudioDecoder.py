import socket
from os import path, remove
from subprocess import Popen

class AudioDecoder (object):

    SOCKET_PATH = '/run/mka_audio.sock'

    def __init__(self):
        self.decoder = None
        if path.exists(self.SOCKET_PATH):
            remove(self.SOCKET_PATH)
        self.socket = socket.socket(socket.AF_UNIX, socket.SOCK_STREAM)
        self.socket.bind(self.SOCKET_PATH)
        self.socket.listen(1)

    def start(self):
        self.decoder = Popen(
            [
                "ffplay", 
                "-f",
                "s16le",
                "-ac",
                "2",
                "-ar",
                "44100",
                "-nodisp",
                'unix:%s' % self.SOCKET_PATH
            ],
        )
        not_bound = True
        while not_bound:
            self.socket.accept()
            not_bound = False

    def stop(self):
        if self.decoder is not None:
            self.decoder.terminate()
            self.socket.close()
            remove(self.SOCKET_PATH)

            self.decoder = None

    def sendAudioData(self, data: bytes):
        if self.decoder is not None:
            self.socket.send(data)

    def running(self) -> bool:
        return self.decoder is not None
