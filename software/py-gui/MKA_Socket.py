import socket
import threading
from os import path, remove

import MenuWindow
from MKA_IBus import IBus_Message, getIBus

OPCODE_PHONE_ACTIVE = 0x21
OPCODE_MKA_ACTIVE = 0x22
OPCODE_AUDIO_SELECTED = 0x23
OPCODE_PHONE_TYPE = 0x2B
OPCODE_PHONE_NAME = 0x2C
OPCODE_PLAYING = 0x39
OPCODE_IBUS_SEND = 0x18
OPCODE_IBUS_RECV = 0x68
OPCODE_BMBT_CONNECTED = 0xF0

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

    def __init__(self, menu_window: MenuWindow.MenuWindow):
        #Socket Client:
        self.socket = socket.socket(socket.AF_UNIX, socket.SOCK_STREAM)
        self.socket.connect(self.SOCKET_PATH)
        
        self.menu_window = menu_window
        self.parameter_list = menu_window.parameter_group

        self.running = True
        self.rx_thread = threading.Thread(target = self.handleSocket)
        self.rx_thread.start()

    def handleSocket(self):
        '''Socket loop function.'''
        msg = SocketMessage(0x68, 0)
        while self.running:
            msg_len = self.readSocketMessage(msg, 1024)
            if msg_len > 0:
                self.interpretSocketMessage(msg)
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
        '''Read a message from the socket.'''
        msg_data = self.readSocketBytes(length)
        socket_start = bytes(SocketMessage.SOCKET_START, "utf-8")
        
        if len(msg_data) <= 0:
            return len(msg_data)
            
        if len(msg_data) <= len(socket_start) + 3:
            return -1
            
        for i in range(0, len(socket_start)):
            if msg_data[i] != socket_start[i]:
                return -1
                
        opcode = msg_data[len(socket_start)]
        msg_length = msg_data[len(socket_start) + 1] - 1
        
        message.refreshData(opcode, msg_length)
        new_data = [0]*msg_length
        
        for i in range(0,msg_length):
            new_data[i] = msg_data[i + len(socket_start) + 2]
        
        message.data = bytes(new_data)

        return msg_length
    
    def interpretSocketMessage(self, message: SocketMessage):
        '''Interpret a message from the socket.'''
        socket_bool = False
        if message.length > 0 and message.data[0] != 0:
            socket_bool = True

        if message.opcode == OPCODE_PHONE_ACTIVE:
            self.parameter_list.phone_active = socket_bool
        elif message.opcode == OPCODE_MKA_ACTIVE:
            self.parameter_list.mka_active = socket_bool
        elif message.opcode == OPCODE_AUDIO_SELECTED:
            self.parameter_list.audio_selected = socket_bool
        elif message.opcode == OPCODE_PLAYING:
            self.parameter_list.playing = socket_bool
        elif message.opcode == OPCODE_PHONE_TYPE:
            self.parameter_list.phone_type = message.data[0]
        elif message.opcode == OPCODE_PHONE_NAME:
            self.parameter_list.phone_name = message.data.decode("utf-8")
        elif message.opcode == OPCODE_IBUS_RECV:
            ib_data = getIBus(message.data)
            self.interpretIBus(ib_data)

    def interpretIBus(self, ib_data: IBus_Message):
        if ib_data.sender == 0xF0 and ib_data.l() >= 2 and self.parameter_list.mka_active: #From BMBT.
            if not self.parameter_list.phone_active: #If not the case, this message needs to be interpreted by Rust.
                if ib_data.data[0] == 0x49 and ib_data.receiver == 0x3B: #Knob turn.
                    steps = ib_data.data[1]&0x7F
                    clockwise = (ib_data.data[1]&0x80) != 0

                    if clockwise:
                        for i in range(0, steps):
                            self.menu_window.decrementSelected()
                    else:
                        for i in range(0,steps):
                            self.menu_window.incrementSelected()
                elif ib_data.data[0] == 0x48: #Button.
                    button = ib_data.data[1]&0x3F
                    state = (ib_data.data[1]&0xC0)>>6

                    if button == 0x5 and state == 0x2: #Enter button.
                        self.menu_window.makeSelection()
