//IBus implementation

use serialport::{new, Parity, SerialPort};

//From BlueBus:
// Devices
pub const IBUS_DEVICE_GM: u8 = 0x00; /* Body module */
pub const IBUS_DEVICE_CDC: u8 = 0x18; /* CD Changer */
pub const IBUS_DEVICE_FUH: u8 = 0x28; /* Radio controlled clock */
pub const IBUS_DEVICE_CCM: u8 = 0x30; /* Check control module */
pub const IBUS_DEVICE_GT: u8 = 0x3B; /* Graphics driver (in navigation system) */
pub const IBUS_DEVICE_DIA: u8 = 0x3F; /* Diagnostic */
pub const IBUS_DEVICE_GTF: u8 = 0x43; /* Graphics driver for rear screen (in navigation system) */
pub const IBUS_DEVICE_EWS: u8 = 0x44; /* EWS (Immobileiser) */
pub const IBUS_DEVICE_CID: u8 = 0x46; /* Central information display (flip-up LCD screen) */
pub const IBUS_DEVICE_MFL: u8 = 0x50; /* Multi function steering wheel */
pub const IBUS_DEVICE_IHK: u8 = 0x5B; /* HVAC */
pub const IBUS_DEVICE_PDC: u8 = 0x60; /* Park Distance Control */
pub const IBUS_DEVICE_RAD: u8 = 0x68; /* Radio */
pub const IBUS_DEVICE_DSP: u8 = 0x6A; /* DSP */
pub const IBUS_DEVICE_SM0: u8 = 0x72; /* Seat memory - 0 */
pub const IBUS_DEVICE_SDRS: u8 = 0x73; /* Sirius Radio */
pub const IBUS_DEVICE_CDCD: u8 = 0x76; /* CD changer, DIN size. */
pub const IBUS_DEVICE_NAVE: u8 = 0x7F; /* Navigation (Europe) */
pub const IBUS_DEVICE_IKE: u8 = 0x80; /* Instrument cluster electronics */
pub const IBUS_DEVICE_GLO: u8 = 0xBF; /* Global, broadcast address */
pub const IBUS_DEVICE_MID: u8 = 0xC0; /* Multi-info display */
pub const IBUS_DEVICE_TEL: u8 = 0xC8; /* Telephone */
pub const IBUS_DEVICE_TCU: u8 = 0xCA; /* BMW Assist */
pub const IBUS_DEVICE_LCM: u8 = 0xD0; /* Light control module */
pub const IBUS_DEVICE_IRIS: u8 = 0xE0; /* Integrated radio information system */
pub const IBUS_DEVICE_ANZV: u8 = 0xE7; /* Front display */
pub const IBUS_DEVICE_RLS: u8 = 0xE8; /* Rain Light Sensor */
pub const IBUS_DEVICE_VM: u8 = 0xED; /* Video Module */
pub const IBUS_DEVICE_BMBT: u8 = 0xF0; /* On-board monitor */
pub const IBUS_DEVICE_LOC: u8 = 0xFF; /* Local */

pub struct IBusMessage {
    pub sender: u8,
    pub receiver: u8,
    pub data: Vec<u8>
}

impl Clone for IBusMessage {
    fn clone(&self) -> Self {
        let mut new_data: Vec<u8> = vec![0;self.data.len()];
        for i in 0..self.data.len() {
            new_data[i] = self.data[i];
        }

        return IBusMessage {
            sender: self.sender,
            receiver: self.receiver,
            data: new_data,
        }
    }
}

impl IBusMessage {
    //Message length.
    pub fn l(&self) -> usize {
        return self.data.len();
    }
    
    //Get bytes from an IBus message.
    pub fn get_bytes(&self) -> Vec<u8> {
        let mut data: Vec<u8> = vec![0; self.data.len() + 4];
        
        data[0] = self.sender;
        data[1] = (self.data.len() + 2) as u8;
        data[2] = self.receiver;
        
        for i in 0..self.data.len() {
            data[i+3] = self.data[i];
        }
        
        let mut checksum: u8 = 0;
        
        for i in 0..data.len() - 1 {
            checksum ^= data[i];
        }
        
        let checksum_index = data.len() - 1;
        data[checksum_index] = checksum;
        
        return data;
    }
}

//Get an IBus message from a vector of bytes.
pub fn get_ibus_message(data: Vec<u8>) -> IBusMessage {
    if data.len() < 4 {
        return IBusMessage {
            sender: 0,
            receiver: 0,
            data: Vec::new(),
        };
    }

    let mut checksum = 0;
    for i in 0..data.len() - 1 {
        checksum ^= data[i];
    }

    if checksum != data[data.len()-1] {
        return IBusMessage {
            sender: 0,
            receiver: 0,
            data: Vec::new(),
        };
    }
    
    if data[1] != (data.len() - 2) as u8 {
        return IBusMessage {
            sender: 0,
            receiver: 0,
            data: Vec::new(),
        };
    }

    let mut the_return = IBusMessage {
        sender: data[0],
        receiver: data[2],
        data: vec![0;data.len() - 4],
    };

    for i in 0..the_return.l() {
        the_return.data[i] = data[i+3];
    }

    return the_return;
}

pub struct IBusHandler {
    port: Box<dyn SerialPort>,
    byte_cache: Vec<u8>
}

impl IBusHandler {
    //Get an IBus handler from a port name.
    pub fn new(port_str: String) -> Option<IBusHandler> {
        let port_builder = serialport::new(port_str, 9600).parity(Parity::Even);
        match port_builder.open() {
            Ok(new_port) => {
                return Some(IBusHandler {port: new_port, byte_cache: Vec::new()});
            }
            Err(err) => {
                println!("Error: {}", err);
                return None;
            }
        };
    }

    //Send an IBus message.
    pub fn write_ibus_message(&mut self, message: IBusMessage) {
        let data = message.get_bytes();

        //Make sure there is nothing waiting to be read.
        let pre_byte_count = match self.port.bytes_to_read() {
            Ok(l) => l,
            Err(_) => {
                return;
            }
        };
        if pre_byte_count > 0 {
            let mut buf = vec![0;pre_byte_count as usize];
            let _ = self.port.read(&mut buf);

            for d in buf {
                self.byte_cache.push(d);
            }
        }
        
        //Write the data.
        match self.port.write(&data) {
            Ok(_) => {

            }
            Err(err) => {
                println!("IBus write error: {}", err);
                return;
            }
        };
    }

    //Read the IBus port.
    pub fn read_ibus_message(&mut self) -> Option<IBusMessage> {
        if self.byte_cache.len() >= 2 {
            if self.byte_cache.len() - 2 >= self.byte_cache[1] as usize {
                let mut l = self.byte_cache[1] as usize + 4;
                
                if l <= self.byte_cache.len() {

                    let mut cached_data = Vec::new();
                    for i in 0..l {
                        cached_data.push(self.byte_cache[i]);
                    }

                    let cached_msg = get_ibus_message(cached_data);
                    if cached_msg.l() > 0 {
                        return Some(cached_msg);
                    }
                    //If message was invalid, we just see what is waiting.   
                } else {
                    l = self.byte_cache.len();
                }
                
                for _i in 0..l {
                    self.byte_cache.remove(0);
                }
            } else {
                self.byte_cache.clear();
            }
        }

        let byte_count = match self.port.bytes_to_read() {
            Ok(l) => l,
            Err(_) => {
                return None;
            }
        };

        if byte_count < 4 {
            return None;
        }

        let mut byte_buf = vec![0;byte_count as usize];
        match self.port.read(&mut byte_buf) {
            Ok(_) => {

            }
            Err(err) => {
                println!("Read error: {}", err);
                return None;
            }
        }

        let mut msg_buf = Vec::new();
        for i in 0..(byte_buf[1] as usize + 4) {
            msg_buf.push(byte_buf[i]);
        }

        let start = byte_buf[1] as usize + 4;
        for _i in 0..start {
            msg_buf.remove(0);
        }

        if msg_buf.len() > 0 {
            for i in 0..msg_buf.len() {
                self.byte_cache.push(msg_buf[i]);
            }
        }

        let new_message = get_ibus_message(msg_buf);
        if new_message.l() > 0 {
            return Some(new_message);
        } else {
            return None;
        }
    }
}