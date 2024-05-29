import socket
import threading
from os import path, remove

import MKA

class SocketHandler:
    SOCKET_PATH = '/run/mka_to_backend.sock'

    def __init__(self, mka: MKA):
        #if path.exists(self.SOCKET_PATH):
        #    remove(self.SOCKET_PATH)

        #Socket:
        self.socket = socket.socket(socket.AF_UNIX, socket.SOCK_STREAM)
        self.socket.connect(self.SOCKET_PATH)
        
        self.mka = mka

        self.running = True
        self.rx_thread = threading.Thread(target = self.handleSocket)
        self.rx_thread.start()

    def handleSocket(self):
        while self.running:
            msg = self.socket.recvfrom(1024)
            print(msg)
        self.socket.close()