class IBus_Message:
    def __init__(self, l: int, sender: int, receiver: int):
        self.data = bytes([0]*l)
        self.sender = sender
        self.receiver = receiver

    def l(self) -> int:
        return len(self.data)
    
    def getBytes(self) -> bytes:
        data = [0]*(len(self.data) + 4)
        data[0] = self.sender
        data[1] = len(self.data) + 2
        data[2] = self.receiver

        data[3:3+len(self.data)] = self.data

        checksum = 0
        for i in range(0,len(data)-1):
            checksum ^= data[i]

        data[len(data)-1] = checksum
        return bytes(data)
    
def getIBus(data: bytes) -> IBus_Message:
    checksum = 0
    for i in range(0,len(data)-1):
        checksum ^= data[i]

    if checksum != data[len(data)-1]:
        return IBus_Message(0,0,0)
    
    if len(data) < 4:
        return IBus_Message(0,0,0)
    
    if data[1] != len(data)-2:
        return IBus_Message(0,0,0)

    the_return = IBus_Message(len(data)-4, data[0], data[2])
    new_data = [0]*the_return.l()
    
    new_data[0:the_return.l()] = data[3:len(data)-1]
    the_return.data = bytes(new_data)
    
    return the_return
