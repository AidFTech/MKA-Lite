//IBus implementation

use std::time::{Duration, Instant};

use serialport::{Parity, SerialPort};

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

// CDC Commands
pub const IBUS_CDC_CMD_GET_STATUS: u8 = 0x00;
pub const IBUS_CDC_CMD_STOP_PLAYING: u8 = 0x01;
pub const IBUS_CDC_CMD_PAUSE_PLAYING: u8 = 0x02;
pub const IBUS_CDC_CMD_START_PLAYING: u8 = 0x03;
pub const IBUS_CDC_CMD_CHANGE_TRACK: u8 = 0x0A;
pub const IBUS_CDC_CMD_SEEK: u8 = 0x04;
pub const IBUS_CDC_CMD_CHANGE_TRACK_BLAUPUNKT: u8 = 0x05;
pub const IBUS_CDC_CMD_CD_CHANGE: u8 = 0x06;
pub const IBUS_CDC_CMD_SCAN: u8 = 0x07;
pub const IBUS_CDC_CMD_RANDOM_MODE: u8 = 0x08;
// CDC Status
pub const IBUS_CDC_STAT_STOP: u8 = 0x00;
pub const IBUS_CDC_STAT_PAUSE: u8 = 0x01;
pub const IBUS_CDC_STAT_PLAYING: u8 = 0x02;
pub const IBUS_CDC_STAT_FAST_FWD: u8 = 0x03;
pub const IBUS_CDC_STAT_FAST_REV: u8 = 0x04;
pub const IBUS_CDC_STAT_END: u8 = 0x07;
pub const IBUS_CDC_STAT_LOADING: u8 = 0x08;

//Radio Commands
pub const IBUS_CMD_RAD_SCREEN_MODE_UPDATE: u8 = 0x46;
pub const IBUS_CMD_RAD_UPDATE_MAIN_AREA: u8 = 0x23;
pub const IBUS_CMD_RAD_C43_SCREEN_UPDATE: u8 = 0x21;
pub const IBUS_CMD_RAD_C43_SET_MENU_MODE: u8 = 0xC0;
pub const IBUS_CMD_RAD_WRITE_MID_DISPLAY: u8 = 0x23;
pub const IBUS_CMD_RAD_WRITE_MID_MENU: u8 = 0x21;

//GT Commands
pub const IBUS_CMD_GT_WRITE_NO_CURSOR: u8 = 0x21;

pub const IBUS_CMD_GT_CHANGE_UI_REQ: u8 = 0x20;
pub const IBUS_CMD_GT_CHANGE_UI_RESP: u8 = 0x21;
pub const IBUS_CMD_GT_WRITE_RESPONSE: u8 = 0x22;
pub const IBUS_CMD_GT_WRITE_TITLE: u8 = 0x23;
pub const IBUS_CMD_GT_MENU_SELECT: u8 = 0x31;
pub const IBUS_CMD_GT_DISPLAY_RADIO_MENU: u8 = 0x37;
pub const IBUS_CMD_GT_SCREEN_MODE_SET: u8 = 0x45;
pub const IBUS_CMD_GT_RAD_TV_STATUS: u8 = 0x4E;
pub const IBUS_CMD_GT_MONITOR_CONTROL: u8 = 0x4F;
pub const IBUS_CMD_GT_WRITE_INDEX: u8 = 0x60;
pub const IBUS_CMD_GT_WRITE_INDEX_TMC: u8 = 0x61;
pub const IBUS_CMD_GT_WRITE_ZONE: u8 = 0x62;
pub const IBUS_CMD_GT_WRITE_STATIC: u8 = 0x63;
pub const IBUS_CMD_GT_TELEMATICS_COORDINATES: u8 = 0xA2;
pub const IBUS_CMD_GT_TELEMATICS_LOCATION: u8 = 0xA4;
pub const IBUS_CMD_GT_WRITE_WITH_CURSOR: u8 = 0xA5;

pub const IBUS_CMD_RAD_LED_TAPE_CTRL: u8 = 0x4A;

const IBUS_WAIT: u64 = 5;

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
    rx_cache: Vec<IBusMessage>,
    tx_cache: Vec<IBusMessage>,
}

impl IBusHandler {
    ///Get an IBus handler from a port name.
    pub fn new(port_str: String) -> Option<IBusHandler> {
        let port_builder = serialport::new(port_str, 9600).parity(Parity::Even);
        let new_port = match port_builder.open() {
            Ok(new_port) => new_port,
            Err(err) => {
                println!("Error: {}", err);
                return None;
            }
        };
        return Some(IBusHandler {
            port: new_port,
            rx_cache: Vec::new(),
            tx_cache: Vec::new(),
        });
    }

    ///Send an IBus message.
    pub fn write_ibus_message(&mut self, message: IBusMessage) {
        let data = message.get_bytes();

        //let _ = self.port.set_timeout(Duration::from_millis(0));

        //Write the data.
        match self.port.write_all(&data) {
            Ok(_) => {
            }
            Err(err) => {
                println!("IBus write error: {}", err);
                //let _ = self.port.set_timeout(Duration::from_millis(IBUS_WAIT));
                return;
            }
        };
        
        match self.port.flush() {
            Ok(_) => {
            }
            Err(err) => {
                println!("IBus write error: {}", err);
                //let _ = self.port.set_timeout(Duration::from_millis(IBUS_WAIT));
                return;
            }
        };

        //let _ = self.port.set_timeout(Duration::from_millis(IBUS_WAIT));
    }

    ///Read the IBus port.
    pub fn read_ibus_message(&mut self) -> Option<IBusMessage> {
        let mut byte_count = match self.port.bytes_to_read() {
            Ok(l) => l,
            Err(_) => {
                return None;
            }
        };

        if byte_count < 2 {
            return None;
        }

        let ib_wait = Duration::from_millis(IBUS_WAIT);
        let mut start;

        /*while Instant::now() - start < ib_wait {
            let new_byte_count = match self.port.bytes_to_read() {
                Ok(new_byte_count) => new_byte_count,
                Err(_) => {
                    break;
                }
            };
            if new_byte_count > byte_count {
                byte_count = new_byte_count;
                start = Instant::now();
            }
        }*/

        if byte_count >= 2 {
            let mut init_stream = vec![0;2];
            match self.port.read_exact(&mut init_stream) {
                Ok(_) => {

                } Err(err) => {
                    println!("IBus read error: {}", err);
                    return None;
                }
            }

            let l = init_stream[1] as usize;
            start = Instant::now();
            byte_count = match self.port.bytes_to_read() {
                Ok(l) => l,
                Err(_) => {
                    return None;
                }
            };

            while byte_count < l as u32 {
                let new_byte_count = match self.port.bytes_to_read() {
                    Ok(new_byte_count) => new_byte_count,
                    Err(_) => {
                        break;
                    }
                };
                if new_byte_count > byte_count {
                    byte_count = new_byte_count;
                    start = Instant::now();
                }

                if Instant::now() - start > ib_wait {
                    let mut db = vec![0;byte_count as usize];
                    let _ = self.port.read_exact(&mut db);
                    return None;
                }
            }

            if byte_count < l as u32 {
                let mut db = vec![0;byte_count as usize];
                let _ = self.port.read_exact(&mut db);
                return None;
            }

            let mut byte_stream = vec![0;l];
            match self.port.read_exact(&mut byte_stream) {
                Ok(l) => l,
                Err(err) => {
                    println!("IBus read error: {}", err);
                    return None;
                }
            };

            byte_stream.insert(0, init_stream[1]);
            byte_stream.insert(0, init_stream[0]);

            let ibus_return = get_ibus_message(byte_stream);

            if ibus_return.l() > 0 {
                return Some(ibus_return);
            } else {
                byte_count = match self.port.bytes_to_read() {
                    Ok(l) => l,
                    Err(_) => {
                        return None;
                    }
                };
                let mut db = vec![0;byte_count as usize];
                let _ = self.port.read_exact(&mut db);
                return None;
            }
        } else {
            return None;
        }
    }

    ///Add a message to the RX cache.
    pub fn rx_cache_message(&mut self, message: IBusMessage) {
        self.rx_cache.push(message);
    }

    ///Get the TX message cache.
    pub fn get_tx_cache(&mut self) -> &mut Vec<IBusMessage> {
        return &mut self.tx_cache;
    }

    ///Get the RX message cache.
    pub fn get_rx_cache(&mut self) -> &mut Vec<IBusMessage> {
        return &mut self.rx_cache;
    }

    ///Get the number of bytes available.
    pub fn bytes_available(&mut self) -> u32 {
        let l = match self.port.bytes_to_read() {
            Ok(l) => l,
            Err(_) => 0,
        };
        return l;
    }
}
