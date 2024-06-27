use rusb::*;
use std::time::Duration;
use std::time::SystemTime;

use crate::ParameterList;

const VENDOR_ID: u16 = 0x1314;
const DEVICE_ID_WIRED: u16 = 0x1520;
const DEVICE_ID_WIRELESS: u16 = 0x1521;

//Verbatim from rusb example.
struct Endpoint {
    config: u8,
    iface: u8,
    setting: u8,
    tx_address: u8,
    rx_address: u8,
}

pub struct USBConnection {
    pub running: bool,
    
    pub device: Option<Device<Context>>,
    pub device_handle: Option<DeviceHandle<Context>>,

    pub rx: u8,
    pub tx: u8,

    pub parameters: *mut ParameterList,
}

pub fn getUSBConnection(parameters: *mut ParameterList) -> USBConnection {
    let mut the_return = USBConnection {
        running: false,

        device: None,
        device_handle: None,

        rx: 0,
        tx: 0,

        parameters: parameters,
    };

    while !the_return.connectDongle() {

    }

    return the_return;
}

impl USBConnection {
    fn connectDongle(&mut self) -> bool {
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
            let endpoint = match getUSBEndpoint(device, descriptor) {
                Some(T) => T,
                None => {
                    return false;
                }
            };
            self.tx = endpoint.tx_address;
            self.rx = endpoint.rx_address;
            self.running = true;

            println!("Found and connected!");
            return true;
        }

        return false;
    }

    //Message read thread loop.
    pub fn readThread(&mut self) {
        let handle = self.device_handle.as_mut().unwrap();

        while self.running {
            let mut buffer: [u8;65536] = [0; 65536];
            match handle.read_bulk(self.rx, &mut buffer, Duration::from_secs(1)) {
                Ok(len) => {
                    println!(" - read: {:?}", &buffer[..len]);
                }
                Err(_) => {
                    continue;
                }
            }
        }
    }

    //Heartbeat thread loop.
    fn heartbeatThread(&mut self) {
        while self.running {

        }
    }
}

fn getUSBEndpoint<T: UsbContext>(device: &mut Device<T>, device_descriptor: DeviceDescriptor) -> Option<Endpoint> {
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