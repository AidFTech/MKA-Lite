import socket
import threading
from os import path, remove
from subprocess import Popen
import subprocess

class AudioDecoder(object):

    SOCKET_PATH = '/run/mka_audio.sock'

    data = []
    client_threads = []

    def __init__(self):
        if path.exists(self.SOCKET_PATH):
            remove(self.SOCKET_PATH)
        self.socket = socket.socket(socket.AF_UNIX, socket.SOCK_STREAM)
        self.socket.bind(self.SOCKET_PATH)
        self.accept_thread = threading.Thread(target=self.acceptClient)
        self.accept_thread.start()
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
            stdout=subprocess.DEVNULL,
            stderr=subprocess.DEVNULL,
        )

    def start(self):
        pass

    def acceptClient(self):
        self.socket.listen(1)
        while True:
            conn, _ = self.socket.accept()
            client_thread = threading.Thread(
                target=self.handleClient,
                args=(conn,)
            )
            client_thread.start()
            self.client_threads.append(client_thread)

    def handleClient(self, conn):
        is_active = True
        while is_active:
            while len(self.data):
                try:
                    conn.send(self.data.pop(0))
                except ConnectionResetError:
                    conn.close()
                    is_active = False


    def stop(self):
        if self.decoder is not None:
            self.decoder.terminate()
            self.socket.close()
            remove(self.SOCKET_PATH)
            self.decoder = None
        for c in self.client_threads:
            if not c.is_alive:
                continue
            c.join()

    def sendAudioData(self, data: bytes):
        if self.decoder is not None:
            self.data.append(data)

    def running(self) -> bool:
        return self.decoder is not None
