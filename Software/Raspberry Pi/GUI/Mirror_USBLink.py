#Acknowledgement to the works of Colin Munro and Sebastian Gotte.

import usb.core
import usb.util
import threading

import Mirror_Protocol
import CarLinkList

from time import time, sleep

# Map of vendor IDs and product IDs
CARLINK_DEVICES = {
    0x1314: [0x1520, 0x1521]
}

MAX_WAIT = 20

class USB_Connection:
    """USB connection object. All USB functions are handled through this."""
    def __init__(self, carlink_list: CarLinkList.CarLinkList, parent):
        self.device = None
        self.rx = None
        self.tx = None
        self.running = False
        self.startup = False
        self.carlink_list = carlink_list

        self.parent = parent #TODO: Is there a way to type this variable without creating a circular import?

    def connectDongle(self) -> bool:
        """Connect the dongle, return whether successful."""
        start_time = time()
        has_new_device = False
        while self.device == None and not has_new_device:
            for vendor_id, product_ids in CARLINK_DEVICES.items():
                for product_id in product_ids:
                    self.device = usb.core.find(
                        idVendor = vendor_id,
                        idProduct = product_id
                    )
                    if self.device:
                        has_new_device = True
                        break
                if time() - start_time >= MAX_WAIT:
                    return False
        if has_new_device:
            for interface_num in range(0, len(self.device[0].interfaces())):
                if self.device.is_kernel_driver_active(interface_num):
                    self.device.detach_kernel_driver(interface_num)
            try:
                self.device.get_active_configuration()
            except usb.core.USBError:
                self.device.set_configuration()
            interface = self.device.get_active_configuration()[(0,0)]

            self.rx = usb.util.find_descriptor(
                interface,
                custom_match = lambda e: usb.util.endpoint_direction(e.bEndpointAddress) == usb.util.ENDPOINT_IN
            )
            if self.rx is None:
                return False
            self.rx.clear_halt()

            self.tx = usb.util.find_descriptor(
                interface,
                custom_match = lambda e: usb.util.endpoint_direction(e.bEndpointAddress) == usb.util.ENDPOINT_OUT
            )
            if self.tx is None:
                return False
            self.tx.clear_halt()

            self.out_locker = threading.Lock()
            self.running = True
            self.run_thread = threading.Thread(target=self.readThread)
            self.run_thread.start()
            return True
        return False

    def startDongle(self):
        """Start the connected dongle."""
        if not self.running:
            return
        self.startup = True
        self.heartbeat_thread = threading.Thread(target=self.heartbeatThread)
        self.heartbeat_thread.start()

    def readThread(self):
        """The message read thread loop."""
        while self.running == True:
            msg_read = False

            if not self.running:
                break

            try:
                data = self.rx.read(Mirror_Protocol.Message.headersize)
            except usb.core.USBError as e:
                if e.errno != 110:
                    self.running = False
                    break
                    #TODO: Send message to parent.
                else:
                    continue
            if len(data) == Mirror_Protocol.Message.headersize:
                header = Mirror_Protocol.Message()
                try:
                    header.deserialise(data)
                except ValueError as e:
                    pass #TODO: Send message to parent?

                n = len(header._data())
                if n > 0:
                    try:
                        msg = header.upgrade(self.rx.read(n))
                        msg_read = True
                    except usb.core.USBError as e:
                        msg_read = False #TODO: Send message to parent? Something is wrong here..?
                else:
                    msg = header.upgrade(bytes([0]*0))
                    msg_read = True

                if msg_read:
                    if hasattr(msg, "msgtype") and msg.msgtype == 6: #Video data.
                        self.parent.sendVideo(msg)
                    elif hasattr(msg, "msgtype") and msg.msgtype == 7: #Audio data.
                        self.parent.sendAudio(msg)
                    else:
                        self.carlink_list.rx_cache.append(msg)


    def heartbeatThread(self):
        """The dongle heartbeat loop. A heartbeat message must be sent regularly."""
        while self.running and self.startup:
            try:
                self.sendMessage(Mirror_Protocol.Heartbeat())
            except usb.core.USBError:
                self.running = False
                self.startup = False
                break
            except:
                pass
            sleep(Mirror_Protocol.Heartbeat.lifecycle)
        if not self.running or not self.startup:
            self.running = False
            self.startup = False


    def sendMessage(self, message: Mirror_Protocol.Message):
        """Send a message to the dongle."""
        if self.tx is not None:
            data = message.serialise()
            while not self.out_locker.acquire():
                pass
            try:
                self.tx.write(data[:message.headersize])
                self.tx.write(data[message.headersize:])
            except usb.core.USBError:
                pass #TODO: Something went very wrong here.
            finally:
                self.out_locker.release()

    def sendMultiple(self, messages: list[Mirror_Protocol.Message]):
        """Send multiple messages to the dongle."""
        for m in messages:
            self.sendMessage(m)

    def stop(self):
        """End the dongle connection."""
        if self.device:
            self.device.reset()
            usb.util.dispose_resources(self.device)
        self.running = False
        self.startup = False
        self.device = None
        self.rx = None
        self.tx = None
        try:
            self.run_thread.join()
        except (RuntimeError, AttributeError):
            pass

        try:
            self.heartbeat_thread.join()
        except (RuntimeError, AttributeError):
            pass
