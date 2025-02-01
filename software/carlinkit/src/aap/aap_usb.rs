use std::time::{Duration, Instant};

use rusb::{Context as USBContext, Device, DeviceDescriptor, DeviceHandle, Direction, TransferType, UsbContext};

struct Endpoint {
	config: u8,
	iface: u8,
	setting: u8,
	tx_address: u8,
	rx_address: u8,
}

pub struct AndroidUSBConnection {
	aa_activated: bool,

	device: Option<Device<USBContext>>,
	device_handle: Option<DeviceHandle<USBContext>>,

	last_check: Instant,
}

impl AndroidUSBConnection {
	pub fn new() -> Self {
		return Self {
			aa_activated: false,

			device: None,
			device_handle: None,

			last_check: Instant::now(),
		}
	}

	pub fn get_connected(&mut self) -> bool {
		return self.aa_activated;
	}

	pub fn connect(&mut self) -> bool {
		if Instant::now() - self.last_check < Duration::from_millis(100) {
			return false;
		}

		self.last_check = Instant::now();

		let context = match USBContext::new() {
			Ok(context) => context,
			Err(_e) => return false,
		};

		let device_list = match context.devices() {
			Ok(device_list) => device_list,
			Err(_e) => return false,
		};

		for d in device_list.iter() {
			let mut handle;
			let mut device;

			match d.open() {
				Ok(d_handle) => {
					device = d;
					handle = d_handle;
				}

				Err(_) => {
					continue;
				}
			}

			let device_descriptor = match device.device_descriptor() {
				Ok(d) => d,
				Err(_e) => continue,
			};

			if device_descriptor.vendor_id() == 0x1314 {
				continue;
			}

			if device_descriptor.vendor_id() == 0x18D1 && (device_descriptor.product_id()&0xFFF0) == 0x2D00 { //Google accessory.
				match handle.claim_interface(0) {
					Ok(_) => {
						println!("Interface claim successful!");
					}
					Err(_) => {
						continue;
					}
				}

				self.device = Some(device);
				self.device_handle = Some(handle);
				self.aa_activated = true;

				return true;
			} else {
				let endpoint = match get_usb_endpoint(&mut device, device_descriptor) {
					Some(t) => t,
					None => {
						continue;
					}
				};

				match handle.claim_interface(endpoint.iface) {
					Ok(_) => {

					}
					Err(_) => {
						continue;
					}
				}

				if !self.aa_activated {
					let protocol = get_protocol(&mut handle);
					if protocol <= 0 { //Device does not support AA.
						continue;
					}

					send_string(&mut handle, 0, "Android");
					send_string(&mut handle, 1, "Android Auto");
					send_string(&mut handle, 2, "Android Auto");
					send_string(&mut handle, 3, "2.0.1");
					send_string(&mut handle, 4, "https://www.flashyconfidence.com");
					send_string(&mut handle, 5, "FC-AAAAAA001");

					start_accessory(&mut handle);
				}
			}
		}

		return false;
	}

	pub fn read_bytes(&mut self) -> Vec<u8> {
		if !self.aa_activated {
			return [].to_vec();
		}

		let handle = self.device_handle.as_mut().unwrap();

		let mut data = vec![0;131800];
		match handle.read_bulk(0x81, &mut data, Duration::from_millis(100)) {
			Ok(len) => {
				return data[0..len].to_vec();
			}
			Err(e) => {
				match e {
					rusb::Error::Timeout => {
						//Do nothing.
						return [].to_vec();
					}
					_=> {
						println!("Error: {}", e);
						self.aa_activated = false;
						return [].to_vec();
					}
				}
			}
		}
	}

	pub fn write_bytes(&mut self, buffer: &Vec<u8>, retry: bool, timeout: Duration) -> bool {
		if !self.aa_activated {
			return false;
		}

		let handle = match &self.device_handle {
			Some(handle) => handle,
			None => {
				return false;
			}
		};

		let mut sent = false;
		let start_time = Instant::now();

		while !sent && Instant::now() - start_time < timeout {
			match handle.write_bulk(1, &buffer, timeout) {
				Ok(l) => {
					if l != buffer.len() {
						if !retry {
							return false;
						} else {
							continue;
						}
					} else {
						sent = true;
					}
				}
				Err(e) => {
					println!("Error: {}", e);

					match e {
						rusb::Error::Timeout => {
							//Do nothing.
						}
						_=> {
							println!("Error: {}", e);
							self.aa_activated = false;
							return false;
						}
					}
					
					if !retry {
						return false;
					} else {
						continue;
					}
				}
			}
		}

		return sent;
	}
}

fn get_protocol(handle: &mut DeviceHandle<USBContext>) -> u16 {
	let mut vec = vec![0; 2];
	//request_type(Direction::In, RequestType::Standard, Recipient::Other)
	match handle.read_control(0xC0, 0x33, 0, 0, &mut vec, Duration::from_millis(100)) {
		Ok(d_len) => {
			if d_len%2 != 0 || d_len == 0 {
				println!("Protocol Length: {}", d_len);
				return 0;
			}

			let version = u16::from_be_bytes([vec[0], vec[1]]);

			println!("Protocol Version: {}", version);
			return version;
		}
		Err(e) => {
			println!("Error: {}", e.to_string());
			return 0;
		}
	}
}

fn send_string(handle: &mut DeviceHandle<USBContext>, index: u16, string: &str) {
	let data = string.as_bytes().to_vec();
	match handle.write_control(0x40, 0x34, 0, index, &data, Duration::from_millis(100)) {
		Ok(_) => {
			println!("Sent: {}", string);
		}
		Err(e) => {
			println!("Error: {}", e.to_string());
			return;
		}
	}
}

fn start_accessory(handle: &mut DeviceHandle<USBContext>) {
	match handle.write_control(0x40, 0x35, 0,0, &[0;0], Duration::from_millis(100)) {
		Ok(_) => {
		}
		Err(e) => {
			println!("Error: {}", e.to_string());
			return;
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