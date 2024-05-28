import socket
import threading
from os import path, remove

import MKA

class SocketHandler:
    SOCKET_PATH = '/run/mka_to_backend.sock'

    def __init__(self, mka: MKA):
        if path.exists(self.SOCKET_PATH):
            remove(self.SOCKET_PATH)

        #Socket:
        self.socket = socket.socket(socket.AF_UNIX, socket.SOCK_STREAM)
        self.socket.bind(self.SOCKET_PATH)
        self.socket.listen(5)

        self.client_socket = None
        
        self.mka = mka

        self.running = True
        self.rx_thread = threading.Thread(target = self.handleSocket)
        self.rx_thread.start()

    def handleSocket(self):
        self.client_socket, address = self.socket.accept()
        while self.running:
            msg = self.client_socket.recvfrom(1024)
            print(msg)
        self.client_socket.close()