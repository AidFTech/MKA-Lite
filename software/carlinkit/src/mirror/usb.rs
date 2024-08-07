use std::io::Write;
use std::os::unix::net::UnixStream;
use std::sync::{Arc, Mutex};
use rusb::{Context as USBContext, Device, DeviceDescriptor, DeviceHandle, Direction, TransferType, UsbContext};
use std::time::Duration;
use std::time::SystemTime;

use crate::mirror::messages::{
    HEADERSIZE,
    MirrorMessage,
    get_heartbeat_message,
    get_mirror_message_from_header,

};
use crate::{init_socket, Context};

const VENDOR_ID: u16 = 0x1314;
const DEVICE_ID_WIRED: u16 = 0x1520;
const DEVICE_ID_WIRELESS: u16 = 0x1521;

// Verbatim from rusb example.
struct Endpoint {
    config: u8,
    iface: u8,
    setting: u8,
    tx_address: u8,
    rx_address: u8,
}

pub struct USBConnection<'a> {
    running: bool,

    device: Option<Device<USBContext>>,
    device_handle: Option<DeviceHandle<USBContext>>,

    rx: u8,
    tx: u8,

    context: &'a Arc<Mutex<Context>>,

    heartbeat_time: SystemTime,

    video_socket: Option<UnixStream>,
}

impl <'a> USBConnection <'a> {

    pub fn new(context: &'a Arc<Mutex<Context>>) -> USBConnection <'a> {
        return USBConnection {
            running: false,

            device: None,
            device_handle: None,

            rx: 0,
            tx: 0,

            context,

            heartbeat_time: SystemTime::now(),

            video_socket: None,
        }
    }

    pub fn connect_dongle(&mut self) -> bool {
        let start_time = SystemTime::now();
        let mut has_new_device = false;
        let mut device_id: u16;

        while !has_new_device {
            let context = match USBContext::new() {
                Ok(context) => context,
                Err(_e) => continue,
            };

            let device_list = match context.devices() {
                Ok(device_list) => device_list,
                Err(_e) => continue,
            };

            for id in 0..2 {
                if id % 2 == 1 {
                    device_id = DEVICE_ID_WIRELESS;
                } else {
                    device_id = DEVICE_ID_WIRED;
                }

                for device in device_list.iter() {
                    let device_descriptor = match device.device_descriptor() {
                        Ok(d) => d,
                        Err(_e) => continue,
                    };

                    if device_descriptor.vendor_id() == VENDOR_ID && device_descriptor.product_id() == device_id {
                        match device.open() {
                            Ok(handle) => {
                                self.device_handle = Some(handle);
                                self.device = Some(device);
                                has_new_device = true;
                                break;
                            }
                            Err(_e) => continue,
                        }
                    }
                }
            }

            if start_time.elapsed().unwrap().as_millis() > 50 && !has_new_device {
                return false;
            }
        }

        if has_new_device {
            let device_handle = self.device_handle.as_mut().unwrap();
            let device = self.device.as_mut().unwrap();

            match device_handle.reset() {
                Ok(_) => {

                }
                Err(_e) => {
                    return false;
                }
            };

            let descriptor = device.device_descriptor().unwrap();
            let endpoint = match get_usb_endpoint(device, descriptor) {
                Some(t) => t,
                None => {
                    return false;
                }
            };

            match device_handle.kernel_driver_active(endpoint.iface) {
                Ok(true) => {
                    device_handle.detach_kernel_driver(endpoint.iface).ok();
                }
                _ => {

                }
            };

            //Done in read_device.rs under configure_endpoint.
            match device_handle.set_active_configuration(endpoint.config) {
                Ok(_) => {
                }
                Err(_) => {
                    return false;
                }
            }

            match device_handle.claim_interface(endpoint.iface) {
                Ok(_) => {
                }
                Err(_) => {
                    return false;
                }
            }

            match device_handle.set_alternate_setting(endpoint.iface, endpoint.setting) {
                Ok(_) => {
                }
                Err(_) => {
                    return false;
                }
            }

            self.tx = endpoint.tx_address;
            self.rx = endpoint.rx_address;
            self.running = true;

            return true;
        }

        return false;
    }

    //Public full loop function.
    pub fn full_loop(&mut self, heartbeat: bool) {
        if heartbeat {
            self.heartbeat_loop();
        }
        self.read_loop();
    }

    //Reset the heartbeat timer.
    pub fn reset_heartbeat(&mut self) {
        self.heartbeat_time = SystemTime::now();
    }

    //Message read thread loop.
    fn read_loop(&mut self) {
        if !self.running {
            return;
        }

        let handle = self.device_handle.as_mut().unwrap();

        let mut buffer: [u8;HEADERSIZE] = [0;HEADERSIZE];
        let len = match handle.read_bulk(self.rx, &mut buffer, Duration::from_millis(100)) { //TODO: This sends an empty USB message to the dongle every 100ms according to Wireshark. Possible to check the size of what can be read first?
            Ok(len) => len,
            Err(err) => {
                match err {
                    rusb::Error::Timeout => {
                        return;
                    }
                    _ => {
                        self.running = false;
                        return;
                    }
                } 
            }
        };

        if len == HEADERSIZE {
            let mut msg_read = false;
            let mut header = match get_mirror_message_from_header(buffer.to_vec()){
                Some(msg) => {
                    msg_read = true;
                    msg
                }
                None =>
                    MirrorMessage {
                        message_type: 0,
                        data: Vec::new(),
                    },
            };
            let valid = header.deserialize(buffer.to_vec());

            if valid {
                let data_len = header.data.len();
                let mut data_buffer: Vec<u8> = vec![0;data_len];
                let n_comp = match handle.read_bulk(self.rx, &mut data_buffer, Duration::from_millis(100)) {
                    Ok(len) => len,
                    Err(err) => {
                        match err {
                            rusb::Error::Timeout => {
                                return;
                            }
                            _ => {
                                self.running = false;
                                return;
                            }
                        } 
                    }
                };

                if n_comp == data_len {
                    msg_read = true;

                    for i in 0..data_len {
                        header.data[i] = data_buffer[i];
                    }
                }
            }

            if msg_read {
                //TODO: Socket video and audio.
                match self.context.try_lock() {
                    Ok(mut context) => {
                        if header.message_type == 6 {
                            match self.video_socket.as_mut() {
                                Some(video_socket) => {
                                    if header.data.len() > 20 {
                                        let mut video_data: Vec<u8> = Vec::new();
                                        for i in 20..header.data.len() {
                                            video_data.push(header.data[i]);
                                        }
                                        let _ = video_socket.write(&video_data);
                                    }
                                }
                                None => {
                                    
                                }
                            }
                        } else if header.message_type == 7 {
                            //Push to audio socket.
                        } else {
                            context.rx_cache.push(header);
                        }
                    }
                    Err(_) => {
                        println!("USB: Parameter list is locked.");
                    }
                }
            }
        }
    }

    //Heartbeat thread loop.
    fn heartbeat_loop(&mut self) {
        if !self.running {
            return;
        }

        if self.heartbeat_time.elapsed().unwrap().as_millis() > 2000 {
            self.heartbeat_time = SystemTime::now();
            self.write_message(get_heartbeat_message());
        }
    }

    //Start the video socket.
    pub fn start_video(&mut self) {
        match self.video_socket {
            Some(_) => {
                return;
            }
            _ => {

            }
        }

        self.video_socket = init_socket(String::from("/run/mka_video.sock"));
        match self.video_socket {
            Some(_) => {
                println!("Successfully opened!")
            }
            None => {
                return;
            }
        }

        println!("USB: Video started!");
    }

    //Write a message to the socket.
    pub fn write_message(&mut self, message: MirrorMessage) {
        let data = message.serialize();
        let handle = self.device_handle.as_mut().unwrap();

        let header = &data[0..HEADERSIZE];
        let usb_data = &data[HEADERSIZE..data.len()];

        match handle.write_bulk(self.tx, header, Duration::from_millis(1000)) {
            Ok(_) => {

            }
            Err(_) => {
                return; //TODO: Stop running?
            }
        }

        if usb_data.len() > 0 {
           match handle.write_bulk(self.tx, usb_data, Duration::from_millis(1000)) {
                Ok(_) => {

                }
                Err(_) => {
                    return; //TODO: Stop running?
                }
            }
        }
    }

    pub fn get_running(&mut self) -> bool {
        return self.running;
    }
}

fn get_usb_endpoint<T: UsbContext>(device: &mut Device<T>, device_descriptor: DeviceDescriptor) -> Option<Endpoint> {
    for config in 0..device_descriptor.num_configurations() {
        let config_descriptor = match device.config_descriptor(config) {
            Ok(descriptor) => descriptor,
            Err(_) => continue,
        };

        let mut found_tx = false;
        let mut found_rx = false;

        let mut new_endpoint = Endpoint {
            config: 0,
            iface: 0,
            setting: 0,
            tx_address: 0,
            rx_address: 0,
        };

        for interface in config_descriptor.interfaces() {
            for interface_descriptor in interface.descriptors() {
                for endpoint_descriptor in interface_descriptor.endpoint_descriptors() {
                    if endpoint_descriptor.direction() == Direction::In && endpoint_descriptor.transfer_type() == TransferType::Bulk {
                        new_endpoint.config = config_descriptor.number();
                        new_endpoint.iface = interface_descriptor.interface_number();
                        new_endpoint.setting = interface_descriptor.setting_number();
                        new_endpoint.rx_address = endpoint_descriptor.address();

                        found_rx = true;
                    } else if endpoint_descriptor.direction() == Direction::Out && endpoint_descriptor.transfer_type() == TransferType::Bulk {
                        new_endpoint.config = config_descriptor.number();
                        new_endpoint.iface = interface_descriptor.interface_number();
                        new_endpoint.setting = interface_descriptor.setting_number();
                        new_endpoint.tx_address = endpoint_descriptor.address();

                        found_tx = true;
                    }

                    if found_tx && found_rx {
                        return Some(new_endpoint);
                    }
                }
            }
        }
    }

    return None;
}
