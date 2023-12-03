import IBus
import time

CD_PERIOD = 3
VM_PERIOD = 3

class PongLoopHandler:
    def __init__(self, bushandler):
        self.ibus_handler = bushandler
        self.running = True
    
    def loopCD(self):
        while self.running:
            cd_announce_message = IBus.AIData(6)
		
            cd_announce_message.data[0] = 0x18
            cd_announce_message.data[1] = cd_announce_message.size()-2
            cd_announce_message.data[2] = 0xBF
            cd_announce_message.data[3] = 0x02
            cd_announce_message.data[4] = 0x00
            cd_announce_message.data[5] = IBus.getChecksum(cd_announce_message)
            
            self.ibus_handler.writeIBusMessage(cd_announce_message)
            time.sleep(CD_PERIOD)

    def loopVM(self):
        while self.running:
            vm_announce_message = IBus.AIData(6)
		
            vm_announce_message.data[0] = 0xED
            vm_announce_message.data[1] = vm_announce_message.size()-2
            vm_announce_message.data[2] = 0xBF
            vm_announce_message.data[3] = 0x02
            vm_announce_message.data[4] = 0x00
            vm_announce_message.data[5] = IBus.getChecksum(vm_announce_message)
            
            self.ibus_handler.writeIBusMessage(vm_announce_message)
            time.sleep(VM_PERIOD)