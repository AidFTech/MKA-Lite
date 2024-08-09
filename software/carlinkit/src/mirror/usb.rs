use rusb::{Context as USBContext, Device, DeviceDescriptor, DeviceHandle, Direction, TransferType, UsbContext};
use std::time::{Duration, SystemTime};

use crate::mirror::messages::{
    HEADERSIZE,
    MirrorMessage,
    mirror_message_from_header,
};

const VENDOR_ID: u16 = 0x1314;
const DEVICE_ID_WIRED: u16 = 0x1520;
const DEVICE_ID_WIRELESS: u16 = 0x1521;

struct Endpoint {
    config: u8,
    iface: u8,
    setting: u8,
    tx_address: u8,
    rx_address: u8,
}

pub struct USBConnection {
    pub connected: bool,

    device: Option<Device<USBContext>>,
    device_handle: Option<DeviceHandle<USBContext>>,

    rx: u8,
    tx: u8,
}

impl USBConnection {

    pub fn new() -> USBConnection {
        return USBConnection {
            connected: false,

            device: None,
            device_handle: None,

            rx: 0,
            tx: 0,
        }
    }

    pub fn connect(&mut self) -> bool {
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

        let device_handle = self.device_handle.as_mut().unwrap();
        let device = self.device.as_mut().unwrap();

        match device_handle.reset() {
            Ok(_) => {
                println!("USB Device Reset!");
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
        self.connected = true;

        return true;
    }

    pub fn read(&mut self) -> Option<MirrorMessage> {
        if !self.connected {
            return None;
        }

        let handle = self.device_handle.as_mut().unwrap();

        let mut buffer: [u8;HEADERSIZE] = [0;HEADERSIZE];
        let len = match handle.read_bulk(self.rx, &mut buffer, Duration::from_millis(100)) {
            //TODO: This sends an empty USB message to the dongle every 100ms according to Wireshark.
            // Is it possible to check the size of what can be read first?
            Ok(len) => len,
            Err(err) => {
                match err {
                    rusb::Error::Timeout => {
                        return None;
                    }
                    _ => {
                        self.connected = false;
                        return None;
                    }
                }
            }
        };

        if len == HEADERSIZE {
            let mut message = mirror_message_from_header(buffer.to_vec());
            let valid = message.deserialize(buffer.to_vec());

            if !valid {
                return None;
            }
            let data_len = message.data.len();

            if data_len <= 0 {
                return Some(message);
            }

            let mut data_buffer: Vec<u8> = vec![0;data_len];
            let buf_len = match handle.read_bulk(self.rx, &mut data_buffer, Duration::from_millis(100)) {
                Ok(len) => len,
                Err(err) => {
                    match err {
                        rusb::Error::Timeout => {
                            return None;
                        }
                        _ => {
                            self.connected = false;
                            return None;
                        }
                    }
                }
            };

            if buf_len == data_len {
                for i in 0..data_len {
                    message.data[i] = data_buffer[i];
                }
                return Some(message);
            }
        }
        None
    }

    // Write a message to the socket.
    pub fn write_message(&mut self, message: MirrorMessage) {
        let data = message.serialize();
        let handle = self.device_handle.as_mut().unwrap();

        let header = &data[0..HEADERSIZE];
        let usb_data = &data[HEADERSIZE..data.len()];

        match handle.write_bulk(self.tx, header, Duration::from_millis(1000)) {
            Ok(_) => {

            }
            Err(err) => {
                println!("{}", err.to_string());
                return; //TODO: Stop running?
            }
        }

        if usb_data.len() > 0 {
           match handle.write_bulk(self.tx, usb_data, Duration::from_millis(1000)) {
                Ok(_) => {}
                Err(err) => {
                    println!("{}", err.to_string());
                    return; //TODO: Stop running?
                }
            }
        }
    }
}

fn get_usb_endpoint<T: UsbContext>(device: &mut Device<T>, device_descriptor: DeviceDescriptor) -> Option<Endpoint> {
    for config in 0..device_descriptor.num_configurations() {
        let config_descriptor = match device.config_descriptor(config) {
            Ok(descriptor) => descriptor,
            Err(_) => continue,
        };

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
                    if endpoint_descriptor.transfer_type() != TransferType::Bulk {
                        continue;
                    }
                    if new_endpoint.config == 0 {
                        new_endpoint.config = config_descriptor.number();
                        new_endpoint.iface = interface_descriptor.interface_number();
                        new_endpoint.setting = interface_descriptor.setting_number();
                    }
                    if endpoint_descriptor.direction() == Direction::In {
                        new_endpoint.rx_address = endpoint_descriptor.address();
                    } else if endpoint_descriptor.direction() == Direction::Out {
                        new_endpoint.tx_address = endpoint_descriptor.address();
                    }
                    if new_endpoint.tx_address != 0 && new_endpoint.rx_address != 0 {
                        return Some(new_endpoint);
                    }
                }
            }
        }
    }
    return None;
}
