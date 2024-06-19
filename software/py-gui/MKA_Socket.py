import socket
import threading
from os import path, remove

import MKA

class SocketMessage:
    SOCKET_START = "MKASock"

    def __init__(self, opcode: int, length: int):
        self.opcode = opcode
        self.length = length

        self.data = bytes([0]*length)
        
    def refreshData(self, opcode: int, length: int):
        self.opcode = opcode
        self.length = length

        self.data = bytes([0]*length)

class SocketHandler:
    SOCKET_PATH = '/run/mka_to_backend.sock'

    def __init__(self, mka: MKA):
        #Socket Client:
        self.socket = socket.socket(socket.AF_UNIX, socket.SOCK_STREAM)
        self.socket.connect(self.SOCKET_PATH)
        
        self.mka = mka

        self.running = True
        self.rx_thread = threading.Thread(target = self.handleSocket)
        self.rx_thread.start()

    def handleSocket(self):
        '''Socket loop function.'''
        while self.running:
            msg = self.socket.recvfrom(1024)
            print(msg)
        self.socket.close()
        
    def readSocketBytes(self, length: int) -> bytes:
        '''Read and return raw bytes from a socket.'''
        read_msg = self.socket.recvfrom(length)
        return read_msg[0]
    
    def writeSocketBytes(self, data: bytes):
        '''Send raw bytes to a socket.'''
        self.socket.send(data)

    def writeSocketMessage(self, message: SocketMessage):
        '''Write a message to the socket.'''
        socket_start = bytes(SocketMessage.SOCKET_START, "utf-8")

        full_length = message.length + len(socket_start) + 3
        data = [0]*full_length

        for i in range(0, len(socket_start)):
            data[i] = socket_start[i]
        
        data[len(socket_start)] = message.opcode
        data[len(socket_start) + 1] = message.length + 1
        
        for i in range(0, message.length):
            data[len(socket_start) + 2 + i] = message.data[i]
        
        checksum = 0
        for i in range(0, full_length-1):
            checksum ^= data[i]

        data[full_length-1] = checksum
        data = bytes(data)
        self.writeSocketBytes(data)
        
    def readSocketMessage(self, message: SocketMessage, length: int) -> int:
        msg_data = self.readSocketBytes(length)
        socket_start = bytes(SocketMessage.SOCKET_START, "utf-8")
        
        if len(msg_data) <= 0:
            return len(msg_data)
            
        if len(msg_data <= len(socket_start)) + 3:
            return -1
            
        for i in range(0, len(socket_start)):
            if msg_data[i] != socket_start[i]:
                return -1
                
        opcode = msg_data[len(socket_start)]
        msg_length = msg_data[len(socket_start) + 1] - 1
        
        message.refreshData(opcode, msg_length)
        
        for i in range(0,msg_length):
            message.data[i] = msg_data[i + len(socket_start) + 2]
            
        return msg_length