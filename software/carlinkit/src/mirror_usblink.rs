use std::sync::{Arc, Mutex};
use rusb::{Context, Device, DeviceDescriptor, DeviceHandle, Direction, TransferType, UsbContext};
use std::time::Duration;
use std::time::SystemTime;

use crate::get_heartbeat_message;
use crate::get_mirror_message_from_header;
use crate::MirrorMessage;
use crate::ParameterList;
use crate::mirror_messages;
use crate::HEADERSIZE;

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
    pub running: bool,

    pub device: Option<Device<Context>>,
    pub device_handle: Option<DeviceHandle<Context>>,

    pub rx: u8,
    pub tx: u8,

    pub parameters: &'a Arc<Mutex<ParameterList>>,

    heartbeat_time: SystemTime,
}

pub fn get_usb_connection<'a>(parameters: &'a Arc<Mutex<ParameterList>>) -> USBConnection {
    let the_return = USBConnection {
        running: false,

        device: None,
        device_handle: None,

        rx: 0,
        tx: 0,

        parameters: parameters,

        heartbeat_time: SystemTime::now(),
    };

    return the_return;
}

impl <'a> USBConnection <'a> {
    fn connect_dongle(&mut self) -> bool {
        let start_time = SystemTime::now();
        let mut has_new_device = false;
        let mut device_id: u16;

        while !has_new_device {
            let context = match Context::new() {
                Ok(context) => context,
                Err(_e) => continue,
            };

            let device_list = match context.devices() {
                Ok(device_list) => device_list,
                Err(_e) => continue,
            };

            for id in 0..2 {
                if id%2 == 1 {
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

            if start_time.elapsed().unwrap().as_millis() > 20 && !has_new_device {
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
    pub fn full_loop(&mut self) {
        self.heartbeat_loop();
        self.read_loop();
    }

    //Message read thread loop.
    fn read_loop(&mut self) {
        if !self.running {
            return;
        }
        
        let handle = self.device_handle.as_mut().unwrap();

        let mut buffer: [u8;mirror_messages::HEADERSIZE] = [0;mirror_messages::HEADERSIZE];
        let len = match handle.read_bulk(self.rx, &mut buffer, Duration::from_millis(200)) {
            Ok(len) => len,
            Err(_) => {
                //TODO: In the original Python code, an errno other than 110 would stop the USB handler from running.
                return;
            }
        };

        if len == mirror_messages::HEADERSIZE {
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
                let n = header.data.len();
                let mut data_buffer: Vec<u8> = vec![0;n];
                let n_comp = match handle.read_bulk(self.rx, &mut data_buffer, Duration::from_secs(1)) {
                    Ok(len) => len,
                    Err(_) => {
                        return;
                    }
                };

                if n_comp == n {
                    msg_read = true;

                    for i in 0..n {
                        header.data[i] = data_buffer[i];
                    }
                }
            }

            if msg_read { 
                //TODO: Socket video and audio.
                let mut parameters = self.parameters.lock().unwrap();
                parameters.rx_cache.push(header);
            }
        }
    }

    //Heartbeat thread loop.
    fn heartbeat_loop(&mut self) {
        if !self.running {
            return;
        }

        println!("{}", self.heartbeat_time.elapsed().unwrap().as_millis());
        if self.heartbeat_time.elapsed().unwrap().as_millis() > 2000 {
            self.heartbeat_time = SystemTime::now();
            self.write_message(get_heartbeat_message());
        }
    }

    //Write a message to the socket.
    pub fn write_message(&mut self, message: MirrorMessage) {
        let data = message.serialize();
        let handle = self.device_handle.as_mut().unwrap();
        
        let header = &data[0..HEADERSIZE];
        let usb_data = &data[HEADERSIZE..data.len()];

        match handle.write_bulk(self.tx, header, Duration::from_millis(200)) {
            Ok(_) => {

            }
            Err(_) => {
                return; //TODO: Stop running?
            }
        }

        if usb_data.len() > 0 {
           match handle.write_bulk(self.tx, usb_data, Duration::from_millis(200)) {
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

pub fn connect_usb_dongle(usb_link: &mut USBConnection) -> bool {
    return usb_link.connect_dongle();
}
