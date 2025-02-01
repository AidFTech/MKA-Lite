use protobuf::rt::*;
use protobuf::Message;

use super::aap_services::SensorType;

#[derive(Default, PartialEq, Clone)]
pub struct SensorMessage {
	event_location: Vec<SensorEventLocation>,
	event_night: Vec<SensorEventNightMode>,
	event_status: Vec<SensorEventDrivingStatus>,

	special_fields: protobuf::SpecialFields,
}

impl SensorMessage {
	pub fn add_event_location(&mut self) -> &mut SensorEventLocation {
		if self.event_location.len() < 1 {
			self.event_location.push(SensorEventLocation::new());
		}

		let index = self.event_location.len() - 1;
		return &mut self.event_location[index];
	}
	
	pub fn add_event_night(&mut self) -> &mut SensorEventNightMode {
		if self.event_night.len() < 1 {
			self.event_night.push(SensorEventNightMode::new());
		}

		let index = self.event_night.len() - 1;
		return &mut self.event_night[index];
	}

	pub fn add_event_status(&mut self) -> &mut SensorEventDrivingStatus {
		if self.event_status.len() < 1 {
			self.event_status.push(SensorEventDrivingStatus::new());
		}

		let index = self.event_status.len() - 1;
		return &mut self.event_status[index];
	}
}

impl Message for SensorMessage {
	const NAME: &'static str = "Sensor Message";

	fn is_initialized(&self) -> bool {
		return true;
	}

	fn merge_from(&mut self, is: &mut protobuf::CodedInputStream) -> protobuf::Result<()> {
		while let Some(tag) = is.read_raw_tag_or_eof()? {
			match tag {
				10 => {
					self.add_event_location().merge_from_bytes(&is.read_bytes()?)?;
				}
				82 => {
					self.add_event_night().merge_from_bytes(&is.read_bytes()?)?;	
				}
				114 => {
					self.add_event_status().merge_from_bytes(&is.read_bytes()?)?;
				}
				tag => {
					read_unknown_or_skip_group(tag, is, self.special_fields.mut_unknown_fields())?;
				}
			}
		}

		return Ok(());
	}

	fn write_to_with_cached_sizes(&self, os: &mut protobuf::CodedOutputStream) -> protobuf::Result<()> {
		for l in &self.event_location {
			let ev_data = l.write_to_bytes()?;
			os.write_bytes(SensorType::SensorTypeLocation as u32, &ev_data)?;
		}
		
		for n in &self.event_night {
			let ev_data = n.write_to_bytes()?;
			os.write_bytes(SensorType::SensorTypeNightData as u32, &ev_data)?;
		}

		for s in &self.event_status {
			let ev_data = s.write_to_bytes()?;
			os.write_bytes(SensorType::SensorTypeDrivingStatus as u32, &ev_data)?;
		}
		
		os.write_unknown_fields(self.special_fields.unknown_fields())?;
		return Ok(());
	}

	fn compute_size(&self) -> u64 {
		let mut total_size =  0;

		for l in &self.event_location {
			total_size += l.compute_size();
		}

		for n in &self.event_night {
			total_size += n.compute_size();
		}

		for s in &self.event_status {
			total_size += s.compute_size();
		}
		
		total_size += unknown_fields_size(self.special_fields.unknown_fields());
		self.special_fields.cached_size().set(total_size as u32);

		return total_size;
	}

	fn special_fields(&self) -> &protobuf::SpecialFields {
		return &self.special_fields;
	}

	fn mut_special_fields(&mut self) -> &mut protobuf::SpecialFields {
		return &mut self.special_fields;
	}

	fn new() -> Self {
		return Self {
			event_location: Vec::new(),
			event_night: Vec::new(),
			event_status: Vec::new(),

			special_fields: protobuf::SpecialFields::default(),
		}
	}

	fn default_instance() -> &'static Self {
		static INSTANCE: Lazy<SensorMessage> = Lazy::new();
		return INSTANCE.get(SensorMessage::new);
	}
}

#[derive(Default, PartialEq, Clone)]
pub struct SensorEventLocation {
	has_timestamp: bool,
	has_latitude: bool,
	has_longitude: bool,
	has_accuracy: bool,
	has_altitude: bool,
	has_speed: bool,
	has_bearing: bool,

	timestamp: u64,
	latitude: i32,
	longitude: i32,
	accuracy: u32,
	altitude: i32,
	speed: i32,
	bearing: i32,

	special_fields: protobuf::SpecialFields,
}

impl SensorEventLocation {
	pub fn set_timestamp(&mut self, timestamp: u64) {
		self.timestamp = timestamp;
		self.has_timestamp = true;
	}

	pub fn set_latitude(&mut self, latitude: i32) {
		self.latitude = latitude;
		self.has_latitude = true;
	}

	pub fn set_longitude(&mut self, longitude: i32) {
		self.longitude = longitude;
		self.has_longitude = true;
	}

	pub fn set_accuracy(&mut self, accuracy: u32) {
		self.accuracy = accuracy;
		self.has_accuracy = true;
	}

	pub fn set_altitude(&mut self, altitude: i32) {
		self.altitude = altitude;
		self.has_altitude = true;
	}

	pub fn set_speed(&mut self, speed: i32) {
		self.speed = speed;
		self.has_speed = true;
	}

	pub fn set_bearing(&mut self, bearing: i32) {
		self.bearing = bearing;
		self.has_bearing = true;
	}
}

impl Message for SensorEventLocation {
	const NAME: &'static str = "Location Sensor Event";

	fn is_initialized(&self) -> bool {
		return true;
	}

	fn merge_from(&mut self, is: &mut protobuf::CodedInputStream) -> protobuf::Result<()> {
		while let Some(tag) = is.read_raw_tag_or_eof()? {
			match tag {
				8 => {
					self.timestamp = is.read_uint64()?;
					self.has_timestamp = true;
				}
				16 => {
					self.latitude = is.read_int32()?;
					self.has_latitude = true;
				}
				24 => {
					self.longitude = is.read_int32()?;
					self.has_longitude = true;
				}
				32 => {
					self.accuracy = is.read_uint32()?;
					self.has_accuracy = true;
				}
				40 => {
					self.altitude = is.read_int32()?;
					self.has_altitude = true;
				}
				48 => {
					self.speed = is.read_int32()?;
					self.has_speed = true;
				}
				56 => {
					self.bearing = is.read_int32()?;
					self.has_bearing = true;
				}
				tag => {
					read_unknown_or_skip_group(tag, is, self.special_fields.mut_unknown_fields())?;
				}
			}
		}

		return Ok(());
	}

	fn write_to_with_cached_sizes(&self, os: &mut protobuf::CodedOutputStream) -> protobuf::Result<()> {
		if self.has_timestamp {
			os.write_uint64(1, self.timestamp)?;
		}

		if self.has_latitude {
			os.write_int32(2, self.latitude)?;
		}

		if self.has_longitude {
			os.write_int32(3, self.longitude)?;
		}

		if self.has_accuracy {
			os.write_uint32(4, self.accuracy)?;
		}

		if self.has_altitude {
			os.write_int32(5, self.altitude)?;
		}

		if self.has_speed {
			os.write_int32(6, self.speed)?;
		}

		if self.has_bearing {
			os.write_int32(7, self.bearing)?;
		}
		
		os.write_unknown_fields(self.special_fields.unknown_fields())?;
		return Ok(());
	}

	fn compute_size(&self) -> u64 {
		let mut total_size = 0;

		if self.has_timestamp {
			total_size += 1 + compute_raw_varint64_size(self.timestamp);
		}

		if self.has_accuracy {
			total_size += 1 + compute_raw_varint64_size(self.accuracy as u64);
		}

		if self.has_latitude {
			total_size += 1;
			if self.latitude >= 0 {
				total_size += compute_raw_varint64_size(self.latitude as u64);
			} else {
				total_size += 10;
			}
		}

		if self.has_longitude {
			total_size += 1;
			if self.longitude >= 0 {
				total_size += compute_raw_varint64_size(self.longitude as u64);
			} else {
				total_size += 10;
			}
		}

		if self.has_altitude {
			total_size += 1;
			if self.altitude >= 0 {
				total_size += compute_raw_varint64_size(self.altitude as u64);
			} else {
				total_size += 10;
			}
		}

		if self.has_speed {
			total_size += 1;
			if self.speed >= 0 {
				total_size += compute_raw_varint64_size(self.speed as u64);
			} else {
				total_size += 10;
			}
		}

		if self.has_bearing {
			total_size += 1;
			if self.bearing >= 0 {
				total_size += compute_raw_varint64_size(self.bearing as u64);
			} else {
				total_size += 10;
			}
		}
		
		total_size += unknown_fields_size(self.special_fields.unknown_fields());
		self.special_fields.cached_size().set(total_size as u32);

		return total_size;
	}

	fn special_fields(&self) -> &protobuf::SpecialFields {
		return &self.special_fields;
	}

	fn mut_special_fields(&mut self) -> &mut protobuf::SpecialFields {
		return &mut self.special_fields;
	}

	fn new() -> Self {
		return Self {
			has_timestamp: false,
			has_latitude: false,
			has_longitude: false,
			has_accuracy: false,
			has_altitude: false,
			has_speed: false,
			has_bearing: false,
			
			timestamp: 0,
			latitude: 0,
			longitude: 0,
			accuracy: 0,
			altitude: 0,
			speed: 0,
			bearing: 0,

			special_fields: protobuf::SpecialFields::default(),
		}
	}

	fn default_instance() -> &'static Self {
		static INSTANCE: Lazy<SensorEventLocation> = Lazy::new();
		return INSTANCE.get(SensorEventLocation::new);
	}
}

#[derive(Default, PartialEq, Clone)]
pub struct SensorEventDrivingStatus {
	pub status: u32,
	special_fields: protobuf::SpecialFields,
}

impl Message for SensorEventDrivingStatus {
	const NAME: &'static str = "Driving Status Sensor Event";

	fn is_initialized(&self) -> bool {
		return true;
	}

	fn merge_from(&mut self, is: &mut protobuf::CodedInputStream) -> protobuf::Result<()> {
		while let Some(tag) = is.read_raw_tag_or_eof()? {
			match tag {
				8 => {
					self.status = is.read_uint32()?;
				}
				tag => {
					read_unknown_or_skip_group(tag, is, self.special_fields.mut_unknown_fields())?;
				}
			}
		}

		return Ok(());
	}

	fn write_to_with_cached_sizes(&self, os: &mut protobuf::CodedOutputStream) -> protobuf::Result<()> {
		os.write_uint32(1, self.status)?;
		
		os.write_unknown_fields(self.special_fields.unknown_fields())?;
		return Ok(());
	}

	fn compute_size(&self) -> u64 {
		let mut total_size =  1 + compute_raw_varint64_size(self.status as u64);
		
		total_size += unknown_fields_size(self.special_fields.unknown_fields());
		self.special_fields.cached_size().set(total_size as u32);

		return total_size;
	}

	fn special_fields(&self) -> &protobuf::SpecialFields {
		return &self.special_fields;
	}

	fn mut_special_fields(&mut self) -> &mut protobuf::SpecialFields {
		return &mut self.special_fields;
	}

	fn new() -> Self {
		return Self {
			status: 0,
			special_fields: protobuf::SpecialFields::default(),
		}
	}

	fn default_instance() -> &'static Self {
		static INSTANCE: Lazy<SensorEventDrivingStatus> = Lazy::new();
		return INSTANCE.get(SensorEventDrivingStatus::new);
	}
}

#[derive(Default, PartialEq, Clone)]
pub struct SensorEventNightMode {
	pub night_mode: bool,
	special_fields: protobuf::SpecialFields,
}

impl Message for SensorEventNightMode {
	const NAME: &'static str = "Night Mode Sensor Event";

	fn is_initialized(&self) -> bool {
		return true;
	}

	fn merge_from(&mut self, is: &mut protobuf::CodedInputStream) -> protobuf::Result<()> {
		while let Some(tag) = is.read_raw_tag_or_eof()? {
			match tag {
				8 => {
					self.night_mode = is.read_bool()?;
				}
				tag => {
					read_unknown_or_skip_group(tag, is, self.special_fields.mut_unknown_fields())?;
				}
			}
		}

		return Ok(());
	}

	fn write_to_with_cached_sizes(&self, os: &mut protobuf::CodedOutputStream) -> protobuf::Result<()> {
		os.write_bool(1, self.night_mode)?;
		
		os.write_unknown_fields(self.special_fields.unknown_fields())?;
		return Ok(());
	}

	fn compute_size(&self) -> u64 {
		let mut total_size = 2;
		
		total_size += unknown_fields_size(self.special_fields.unknown_fields());
		self.special_fields.cached_size().set(total_size as u32);

		return total_size;
	}

	fn special_fields(&self) -> &protobuf::SpecialFields {
		return &self.special_fields;
	}

	fn mut_special_fields(&mut self) -> &mut protobuf::SpecialFields {
		return &mut self.special_fields;
	}

	fn new() -> Self {
		return Self {
			night_mode: false,
			special_fields: protobuf::SpecialFields::default(),
		}
	}

	fn default_instance() -> &'static Self {
		static INSTANCE: Lazy<SensorEventNightMode> = Lazy::new();
		return INSTANCE.get(SensorEventNightMode::new);
	}
}