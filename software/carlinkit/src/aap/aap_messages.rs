use protobuf::Message;
use protobuf::rt::*;

use super::aap_channel_descriptor::ChannelDescriptor;

#[derive(Default, PartialEq, Clone)]
pub struct AuthCompleteResponse {
	status: i32,
	special_fields: protobuf::SpecialFields,
}

impl Message for AuthCompleteResponse {
	const NAME: &'static str = "Authentication Complete";
	
	fn is_initialized(&self) -> bool {
		return true;
	}
	
	fn merge_from(&mut self, is: &mut protobuf::CodedInputStream) -> protobuf::Result<()> {
		while let Some(tag) = is.read_raw_tag_or_eof()? {
			match tag {
				8 => {
					match is.read_int32() {
						Ok(status) => {
							self.status = status;
						}
						Err(e) => {
							return Err(e);
						}
					}
				}
				tag => {
					read_unknown_or_skip_group(tag, is, self.special_fields.mut_unknown_fields())?;
				}
			}
		}

		return Ok(());
	}
	
	fn write_to_with_cached_sizes(&self, os: &mut protobuf::CodedOutputStream) -> protobuf::Result<()> {
		match os.write_int32(1, self.status) {
			Ok(_) => {

			}
			Err(e) => {
				return Err(e);
			}
		}

		os.write_unknown_fields(self.special_fields.unknown_fields())?;
		return Ok(());
	}
	
	fn compute_size(&self) -> u64 {
		let mut total_size = 0;

		total_size += 1 + compute_raw_varint64_size(self.status as u64);

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
		static INSTANCE: Lazy<AuthCompleteResponse> = Lazy::new();
		return INSTANCE.get(AuthCompleteResponse::new);
	}
	
}

#[derive(Default, PartialEq, Clone)]
pub struct ServiceDiscoveryRequest {
	phone_name: String,
	special_fields: protobuf::SpecialFields,
}

impl ServiceDiscoveryRequest {
	pub fn get_phone_name(self) -> String {
		return self.phone_name;
	}
}

impl Message for ServiceDiscoveryRequest {
	const NAME: &'static str = "Service Discovery Request";

	fn is_initialized(&self) -> bool {
		return true;
	}

	fn merge_from(&mut self, is: &mut protobuf::CodedInputStream) -> protobuf::Result<()> {
		while let Some(tag) = is.read_raw_tag_or_eof()? {
			match tag {
				42 => {
					match is.read_string() {
						Ok(phone_name) => {
							self.phone_name = phone_name;
						}
						Err(e) => {
							return Err(e);
						}
					}
				}
				tag => {
					read_unknown_or_skip_group(tag, is, self.special_fields.mut_unknown_fields())?;
				}
			}
		}

		return Ok(());
	}

	fn write_to_with_cached_sizes(&self, os: &mut protobuf::CodedOutputStream) -> protobuf::Result<()> {
		match os.write_string(5, &self.phone_name) {
			Ok(_) => {

			}
			Err(e) => {
				return Err(e);
			}
		}
		
		os.write_unknown_fields(self.special_fields.unknown_fields())?;
		return Ok(());
	}

	fn compute_size(&self) -> u64 {
		let mut total_size = 0;

		total_size += string_size(5, &self.phone_name);

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
			phone_name: "".to_string(),
			special_fields: protobuf::SpecialFields::default(),
		}
	}

	fn default_instance() -> &'static Self {
		static INSTANCE: Lazy<ServiceDiscoveryRequest> = Lazy::new();
		return INSTANCE.get(ServiceDiscoveryRequest::new);
	}
}

#[derive(Default, PartialEq, Clone)]
pub struct ServiceDiscoveryResponse {
	pub head_unit_name: String,
	pub car_model: String,
	pub car_year: String,
	pub car_serial: String,
	pub driver_pos: bool,
	pub headunit_make: String,
	pub headunit_model: String,
	pub sw_build: String,
	pub sw_version: String,
	pub native_media: bool,
	pub hide_clock: bool,

	additional_descriptors: Vec<ChannelDescriptor>,
	
	special_fields: protobuf::SpecialFields,
}

impl Message for ServiceDiscoveryResponse {
	const NAME: &'static str = "Service Discovery Response";

	fn is_initialized(&self) -> bool {
		return true;
	}

	fn merge_from(&mut self, is: &mut protobuf::CodedInputStream) -> protobuf::Result<()> {
		while let Some(tag) = is.read_raw_tag_or_eof()? {
			match tag {
				10 => {
					//TODO: Add any extra tags.	
				}
				18 => { //Headunit name.
					is.read_string_into(&mut self.head_unit_name)?;
				}
				26 => { //Car model.
					is.read_string_into(&mut self.car_model)?;
				}
				34 => { //Car year.
					is.read_string_into(&mut self.car_year)?;
				}
				42 => { //Car serial.
					is.read_string_into(&mut self.car_serial)?;
				}
				48 => { //LHD/RHD.
					match is.read_bool() {
						Ok(driver_pos) => {
							self.driver_pos = driver_pos;
						}
						Err(_) => {
							continue;
						}
					}
				}
				58 => { //Headunit make.
					is.read_string_into(&mut self.headunit_make)?;
				}
				66 => { //Headunit model.
					is.read_string_into(&mut self.headunit_model)?;
				}
				74 => { //Software build.
					is.read_string_into(&mut self.sw_build)?;
				}
				82 => { //Software version.
					is.read_string_into(&mut self.sw_version)?;
				}
				88 => { //Native media.
					match is.read_bool() {
						Ok(native_media) => {
							self.native_media = native_media;
						}
						Err(_) => {
							continue;
						}
					}
				}
				96 => { //Hide clock.
					match is.read_bool() {
						Ok(hide_clock) => {
							self.hide_clock = hide_clock;
						}
						Err(_) => {
							continue;
						}
					}
				}
				tag => {
					read_unknown_or_skip_group(tag, is, self.special_fields.mut_unknown_fields())?;
				}
			}
		}

		return Ok(());
	}

	fn write_to_with_cached_sizes(&self, os: &mut protobuf::CodedOutputStream) -> protobuf::Result<()> {
		for desc in &self.additional_descriptors {
			match desc.write_to_bytes() {
				Ok(data) => {
					os.write_bytes(1, &data)?;
				}
				Err(e) => {
					println!("Error: {}", e);
				}
			}
		}

		if self.head_unit_name.len() > 0 {
			os.write_string(2, &self.head_unit_name)?;
		}
		
		if self.car_model.len() > 0 {
			os.write_string(3, &self.car_model)?;
		}
		
		if self.car_year.len() > 0 {
			os.write_string(4, &self.car_year)?;
		}
		
		if self.car_serial.len() > 0 {
			os.write_string(5, &self.car_serial)?;
		}

		os.write_bool(6, self.driver_pos)?;
		
		if self.headunit_make.len() > 0 {
			os.write_string(7, &self.headunit_make)?;
		}
		
		if self.headunit_model.len() > 0 {
			os.write_string(8, &self.headunit_model)?;
		}
		
		if self.sw_build.len() > 0 {
			os.write_string(9, &self.sw_build)?;
		}
		
		if self.sw_version.len() > 0 {
			os.write_string(10, &self.sw_version)?;
		}
		
		os.write_bool(11, self.native_media)?;
		
		os.write_bool(12, self.hide_clock)?;
		
		os.write_unknown_fields(self.special_fields.unknown_fields())?;
		return Ok(());
	}

	fn compute_size(&self) -> u64 {
		let mut total_size = 0;

		total_size += 2 + compute_raw_varint64_size(self.driver_pos as u64);
		total_size += 2 + compute_raw_varint64_size(self.native_media as u64);
		total_size += 2 + compute_raw_varint64_size(self.hide_clock as u64);

		for desc in &self.additional_descriptors {
			total_size += desc.compute_size();
		}

		if self.head_unit_name.len() > 0 {
			total_size += string_size(18, &self.head_unit_name);
		}
		
		if self.car_model.len() > 0 {
			total_size += string_size(26, &self.car_model);
		}
		
		if self.car_year.len() > 0 {
			total_size += string_size(34, &self.car_year);
		}
		
		if self.car_serial.len() > 0 {
			total_size += string_size(42, &self.car_serial);
		}
		
		if self.headunit_make.len() > 0 {
			total_size += string_size(58, &self.headunit_make);
		}
		
		if self.headunit_model.len() > 0 {
			total_size += string_size(66, &self.headunit_model);
		}
		
		if self.sw_build.len() > 0 {
			total_size += string_size(74, &self.sw_build);
		}
		
		if self.sw_version.len() > 0 {
			total_size += string_size(82, &self.sw_version);
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
			head_unit_name: "AidF".to_string(),
			car_model: "Civic".to_string(),
			car_year: "2014".to_string(),
			car_serial: "ABC123".to_string(),
			driver_pos: false,
			headunit_make: "AidF".to_string(),
			headunit_model: "AIAHU-100".to_string(),
			sw_build: "SWB1".to_string(),
			sw_version: "SWV1".to_string(),
			native_media: true,
			hide_clock: false,

			additional_descriptors: Vec::new(),
			
			special_fields: protobuf::SpecialFields::default(),
		}
	}

	fn default_instance() -> &'static Self {
		static INSTANCE: Lazy<ServiceDiscoveryResponse> = Lazy::new();
		return INSTANCE.get(ServiceDiscoveryResponse::new);
	}
}

impl ServiceDiscoveryResponse {
	pub fn add_channel(&mut self, channel_id: u32) -> &mut ChannelDescriptor {
		self.additional_descriptors.push(ChannelDescriptor::new_with_id(channel_id));
		let index = self.additional_descriptors.len() - 1;
		return &mut self.additional_descriptors[index];
	}

	pub fn get_channel(&mut self, index: usize) -> &mut ChannelDescriptor {
		return &mut self.additional_descriptors[index];
	}

	pub fn get_channel_count(self) -> usize {
		return self.additional_descriptors.len();
	}
}

#[derive(Default, PartialEq, Clone)]
pub struct AudioFocusRequest {
	pub focus_type: u32,

	special_fields: protobuf::SpecialFields,
}

impl Message for AudioFocusRequest {
	const NAME: &'static str = "Audio Focus Request";

	fn is_initialized(&self) -> bool {
		return true;
	}

	fn merge_from(&mut self, is: &mut protobuf::CodedInputStream) -> protobuf::Result<()> {
		while let Some(tag) = is.read_raw_tag_or_eof()? {
			match tag {
				8 => {
					self.focus_type = is.read_uint32()?;
				}
				tag => {
					read_unknown_or_skip_group(tag, is, self.special_fields.mut_unknown_fields())?;
				}
			}
		}

		return Ok(());
	}

	fn write_to_with_cached_sizes(&self, os: &mut protobuf::CodedOutputStream) -> protobuf::Result<()> {
		os.write_uint32(1, self.focus_type)?;
		
		os.write_unknown_fields(self.special_fields.unknown_fields())?;
		return Ok(());
	}

	fn compute_size(&self) -> u64 {
		let mut total_size = 1 + compute_raw_varint64_size(self.focus_type as u64);
		
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
			focus_type: 0,
			special_fields: protobuf::SpecialFields::default(),
		}
	}

	fn default_instance() -> &'static Self {
		static INSTANCE: Lazy<AudioFocusRequest> = Lazy::new();
		return INSTANCE.get(AudioFocusRequest::new);
	}
}

#[derive(Default, PartialEq, Clone)]
pub struct AudioFocusResponse {
	pub focus_type: u32,

	special_fields: protobuf::SpecialFields,
}

impl Message for AudioFocusResponse {
	const NAME: &'static str = "Audio Focus Response";

	fn is_initialized(&self) -> bool {
		return true;
	}

	fn merge_from(&mut self, is: &mut protobuf::CodedInputStream) -> protobuf::Result<()> {
		while let Some(tag) = is.read_raw_tag_or_eof()? {
			match tag {
				8 => {
					self.focus_type = is.read_uint32()?;
				}
				tag => {
					read_unknown_or_skip_group(tag, is, self.special_fields.mut_unknown_fields())?;
				}
			}
		}

		return Ok(());
	}

	fn write_to_with_cached_sizes(&self, os: &mut protobuf::CodedOutputStream) -> protobuf::Result<()> {
		os.write_uint32(1, self.focus_type)?;
		
		os.write_unknown_fields(self.special_fields.unknown_fields())?;
		return Ok(());
	}

	fn compute_size(&self) -> u64 {
		let mut total_size = 1 + compute_raw_varint64_size(self.focus_type as u64);
		
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
			focus_type: 0,
			special_fields: protobuf::SpecialFields::default(),
		}
	}

	fn default_instance() -> &'static Self {
		static INSTANCE: Lazy<AudioFocusResponse> = Lazy::new();
		return INSTANCE.get(AudioFocusResponse::new);
	}
}

#[derive(Default, PartialEq, Clone)]
pub struct ChannelOpenRequest {
	pub priority: u32,
	pub channel_id: u32,

	special_fields: protobuf::SpecialFields,
}

impl Message for ChannelOpenRequest {
	const NAME: &'static str = "Channel Open Request";

	fn is_initialized(&self) -> bool {
		return true;
	}

	fn merge_from(&mut self, is: &mut protobuf::CodedInputStream) -> protobuf::Result<()> {
		while let Some(tag) = is.read_raw_tag_or_eof()? {
			match tag {
				8 => {
					self.priority = is.read_uint32()?;
				}
				16 => {
					self.channel_id = is.read_uint32()?;
				}
				tag => {
					read_unknown_or_skip_group(tag, is, self.special_fields.mut_unknown_fields())?;
				}
			}
		}

		return Ok(());
	}

	fn write_to_with_cached_sizes(&self, os: &mut protobuf::CodedOutputStream) -> protobuf::Result<()> {
		os.write_uint32(1, self.priority)?;
		os.write_uint32(2, self.channel_id)?;
		
		os.write_unknown_fields(self.special_fields.unknown_fields())?;
		return Ok(());
	}

	fn compute_size(&self) -> u64 {
		let mut total_size = 1 + compute_raw_varint64_size(self.priority as u64);
		total_size += 1 + compute_raw_varint64_size(self.channel_id as u64);
		
		total_size += unknown_fields_size(self.special_fields.unknown_fields());
		self.special_fields.cached_size().set(total_size as u32);

		return total_size;
	}

	fn special_fields(&self) -> &protobuf::SpecialFields {
		return &self.special_fields
	}

	fn mut_special_fields(&mut self) -> &mut protobuf::SpecialFields {
		return &mut self.special_fields;
	}

	fn new() -> Self {
		return Self {
			priority: 0,
			channel_id: 0,

			special_fields: protobuf::SpecialFields::default(),
		}
	}

	fn default_instance() -> &'static Self {
		static INSTANCE: Lazy<ChannelOpenRequest> = Lazy::new();
		return INSTANCE.get(ChannelOpenRequest::new);
	}
}

#[derive(Default, PartialEq, Clone)]
pub struct ChannelOpenResponse {
	pub status: u32,
	special_fields: protobuf::SpecialFields,
}

impl Message for ChannelOpenResponse {
	const NAME: &'static str = "Channel Open Response";

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
		let mut total_size = 1 + compute_raw_varint64_size(self.status as u64);
		
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
		static INSTANCE: Lazy<ChannelOpenResponse> = Lazy::new();
		return INSTANCE.get(ChannelOpenResponse::new);
	}
}

#[derive(Default, PartialEq, Clone)]
pub struct BindingResponse {
	pub status: u32,
	special_fields: protobuf::SpecialFields,
}

impl Message for BindingResponse {
	const NAME: &'static str = "Binding Response";

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
		let mut total_size = 1 + compute_raw_varint64_size(self.status as u64);
		
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
		static INSTANCE: Lazy<BindingResponse> = Lazy::new();
		return INSTANCE.get(BindingResponse::new);
	}
}

#[derive(Default, PartialEq, Clone)]
pub struct SensorStartResponse {
	pub status: u32,
	special_fields: protobuf::SpecialFields,
}

impl Message for SensorStartResponse {
	const NAME: &'static str = "Sensor Start Response";

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
		let mut total_size = 1 + compute_raw_varint64_size(self.status as u64);
		
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
		static INSTANCE: Lazy<SensorStartResponse> = Lazy::new();
		return INSTANCE.get(SensorStartResponse::new);
	}
}

#[derive(Default, PartialEq, Clone)]
pub struct MediaSetupResponse {
	pub status: u32,
	pub max_unacked: u32,
	configs: Vec<u32>,

	special_fields: protobuf::SpecialFields,
}

impl MediaSetupResponse {
	pub fn add_config(&mut self, config: u32) {
		self.configs.push(config);
	}
}

impl Message for MediaSetupResponse {
	const NAME: &'static str = "Media Setup Response";

	fn is_initialized(&self) -> bool {
		return true;
	}

	fn merge_from(&mut self, is: &mut protobuf::CodedInputStream) -> protobuf::Result<()> {
		while let Some(tag) = is.read_raw_tag_or_eof()? {
			match tag {
				8 => {
					self.status = is.read_uint32()?;
				}
				16 => {
					self.max_unacked = is.read_uint32()?;
				}
				24 => {
					self.configs.push(is.read_uint32()?);
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
		os.write_uint32(2, self.max_unacked)?;

		for c in &self.configs {
			os.write_uint32(3, *c)?;
		}
		
		os.write_unknown_fields(self.special_fields.unknown_fields())?;
		return Ok(());
	}

	fn compute_size(&self) -> u64 {
		let mut total_size = 1 + compute_raw_varint64_size(self.status as u64);
		total_size += 1 + compute_raw_varint64_size(self.max_unacked as u64);

		for c in &self.configs {
			total_size += 1 + compute_raw_varint64_size(*c as u64);
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
			status: 0,
			max_unacked: 0,
			configs: Vec::new(),

			special_fields: protobuf::SpecialFields::default(),
		}
	}

	fn default_instance() -> &'static Self {
		static INSTANCE: Lazy<MediaSetupResponse> = Lazy::new();
		return INSTANCE.get(MediaSetupResponse::new);
	}
}

#[derive(Default, PartialEq, Clone)]
pub struct MediaAck {
	pub session: i32,
	pub value: u32,

	special_fields: protobuf::SpecialFields,
}

impl Message for MediaAck {
	const NAME: &'static str = "Media Acknowledgement";

	fn is_initialized(&self) -> bool {
		return true;
	}

	fn merge_from(&mut self, is: &mut protobuf::CodedInputStream) -> protobuf::Result<()> {
		while let Some(tag) = is.read_raw_tag_or_eof()? {
			match tag {
				8 => {
					self.session = is.read_int32()?;
				}
				16 => {
					self.value = is.read_uint32()?;
				}
				tag => {
					read_unknown_or_skip_group(tag, is, self.special_fields.mut_unknown_fields())?;
				}
			}
		}

		return Ok(());
	}

	fn write_to_with_cached_sizes(&self, os: &mut protobuf::CodedOutputStream) -> protobuf::Result<()> {
		os.write_int32(1, self.session)?;
		os.write_uint32(2, self.value)?;
		
		os.write_unknown_fields(self.special_fields.unknown_fields())?;
		return Ok(());
	}

	fn compute_size(&self) -> u64 {
		let mut total_size = 1 + compute_raw_varint64_size(self.value as u64);
		
		if self.session >= 0 {
			total_size += 1 + compute_raw_varint64_size(self.session as u64);
		} else {
			total_size += 1 + 10;
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
			session: 0,
			value: 0,

			special_fields: protobuf::SpecialFields::default(),
		}
	}

	fn default_instance() -> &'static Self {
		static INSTANCE: Lazy<MediaAck> = Lazy::new();
		return INSTANCE.get(MediaAck::new);
	}
}

#[derive(Default, PartialEq, Clone)]
pub struct PingMessage {
	pub timestamp: i64,

	special_fields: protobuf::SpecialFields,
}

impl Message for PingMessage {
	const NAME: &'static str = "Ping Message";

	fn is_initialized(&self) -> bool {
		return true;
	}

	fn merge_from(&mut self, is: &mut protobuf::CodedInputStream) -> protobuf::Result<()> {
		while let Some(tag) = is.read_raw_tag_or_eof()? {
			match tag {
				8 => {
					self.timestamp = is.read_int64()?;
				}
				tag => {
					read_unknown_or_skip_group(tag, is, self.special_fields.mut_unknown_fields())?;
				}
			}
		}

		return Ok(());
	}

	fn write_to_with_cached_sizes(&self, os: &mut protobuf::CodedOutputStream) -> protobuf::Result<()> {
		os.write_int64(1, self.timestamp)?;
		
		os.write_unknown_fields(self.special_fields.unknown_fields())?;
		return Ok(());
	}

	fn compute_size(&self) -> u64 {
		let mut total_size = 1;
		if self.timestamp >= 0 {
			total_size += compute_raw_varint64_size(self.timestamp as u64);
		} else {
			total_size += 10;
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
			timestamp: 0,

			special_fields: protobuf::SpecialFields::default(),
		}
	}

	fn default_instance() -> &'static Self {
		static INSTANCE: Lazy<PingMessage> = Lazy::new();
		return INSTANCE.get(PingMessage::new);
	}
}

#[derive(Default, PartialEq, Clone)]
pub struct NavigationFocusMessage {
	pub focus_type: u32,

	special_fields: protobuf::SpecialFields,
}

impl Message for NavigationFocusMessage {
	const NAME: &'static str = "Navigation Focus Message";

	fn is_initialized(&self) -> bool {
		return true;
	}

	fn merge_from(&mut self, is: &mut protobuf::CodedInputStream) -> protobuf::Result<()> {
		while let Some(tag) = is.read_raw_tag_or_eof()? {
			match tag {
				8 => {
					self.focus_type = is.read_uint32()?;
				}
				tag => {
					read_unknown_or_skip_group(tag, is, self.special_fields.mut_unknown_fields())?;
				}
			}
		}

		return Ok(());
	}

	fn write_to_with_cached_sizes(&self, os: &mut protobuf::CodedOutputStream) -> protobuf::Result<()> {
		os.write_uint32(1, self.focus_type)?;
		
		os.write_unknown_fields(self.special_fields.unknown_fields())?;
		return Ok(());
	}

	fn compute_size(&self) -> u64 {
		let mut total_size = 1 + compute_raw_varint64_size(self.focus_type as u64);
		
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
			focus_type: 0,

			special_fields: protobuf::SpecialFields::default(),
		}
	}

	fn default_instance() -> &'static Self {
		static INSTANCE: Lazy<NavigationFocusMessage> = Lazy::new();
		return INSTANCE.get(NavigationFocusMessage::new);
	}
}

#[derive(Default, PartialEq, Clone)]
pub struct VideoFocus {
	pub mode: bool,
	pub unrequested: bool,
	special_fields: protobuf::SpecialFields,
}

impl Message for VideoFocus {
	const NAME: &'static str = "Video Focus";

	fn is_initialized(&self) -> bool {
		return true;
	}

	fn merge_from(&mut self, is: &mut protobuf::CodedInputStream) -> protobuf::Result<()> {
		while let Some(tag) = is.read_raw_tag_or_eof()? {
			match tag {
				8 => {
					self.mode = is.read_bool()?;
				}
				16 => {
					self.unrequested = is.read_bool()?;
				}
				tag => {
					read_unknown_or_skip_group(tag, is, self.special_fields.mut_unknown_fields())?;
				}
			}
		}

		return Ok(());
	}

	fn write_to_with_cached_sizes(&self, os: &mut protobuf::CodedOutputStream) -> protobuf::Result<()> {
		os.write_bool(1, self.mode)?;
		os.write_bool(2, self.unrequested)?;
		
		os.write_unknown_fields(self.special_fields.unknown_fields())?;
		return Ok(());
	}

	fn compute_size(&self) -> u64 {
		let mut total_size = 4;
		
		total_size += unknown_fields_size(self.special_fields.unknown_fields());
		self.special_fields.cached_size().set(total_size as u32);

		return total_size;
	}

	fn special_fields(&self) -> &protobuf::SpecialFields {
		return &self.special_fields
	}

	fn mut_special_fields(&mut self) -> &mut protobuf::SpecialFields {
		return &mut self.special_fields;
	}

	fn new() -> Self {
		return Self {
			mode: true,
			unrequested: true,

			special_fields: protobuf::SpecialFields::default(),
		}
	}

	fn default_instance() -> &'static Self {
		static INSTANCE: Lazy<VideoFocus> = Lazy::new();
		return INSTANCE.get(VideoFocus::new);
	}
}

#[derive(Default, PartialEq, Clone)]
pub struct ButtonPressMessage {
	button_code: u32,
	is_pressed: bool,
	long_press: bool,

	special_fields: protobuf::SpecialFields,
}

impl ButtonPressMessage {
	pub fn get_button_press(button: u32, pressed: bool) -> Self {
		return Self {
			button_code: button,
			is_pressed: pressed,
			long_press: false,

			special_fields: protobuf::SpecialFields::default(),
		}
	}

	pub fn get_long_button_press(button: u32, pressed: bool, long_press: bool) -> Self {
		return Self {
			button_code: button,
			is_pressed: pressed,
			long_press,

			special_fields: protobuf::SpecialFields::default(),
		}
	}
}

impl Message for ButtonPressMessage {
	const NAME: &'static str = "Button Press";

	fn is_initialized(&self) -> bool {
		return true;
	}

	fn merge_from(&mut self, is: &mut protobuf::CodedInputStream) -> protobuf::Result<()> {
		while let Some(tag) = is.read_raw_tag_or_eof()? {
			match tag {
				8 => {
					self.button_code = is.read_uint32()?;
				}
				16 => {
					self.is_pressed = is.read_bool()?;
				}
				32 => {
					self.long_press = is.read_bool()?;
				}
				tag => {
					read_unknown_or_skip_group(tag, is, self.special_fields.mut_unknown_fields())?;
				}
			}
		}

		return Ok(());
	}

	fn write_to_with_cached_sizes(&self, os: &mut protobuf::CodedOutputStream) -> protobuf::Result<()> {
		os.write_uint32(1, self.button_code)?;
		os.write_bool(2, self.is_pressed)?;
		os.write_uint32(3, self.button_code)?;
		os.write_bool(4, self.long_press)?;
		
		os.write_unknown_fields(self.special_fields.unknown_fields())?;
		return Ok(());
	}

	fn compute_size(&self) -> u64 {
		let mut total_size = 4 + 2 + 2*compute_raw_varint64_size(self.button_code as u64);
		
		total_size += unknown_fields_size(self.special_fields.unknown_fields());
		self.special_fields.cached_size().set(total_size as u32);

		return total_size;
	}

	fn special_fields(&self) -> &protobuf::SpecialFields {
		return &self.special_fields;
	}

	fn mut_special_fields(&mut self) -> &mut protobuf::SpecialFields {
		return &mut self.special_fields
	}

	fn new() -> Self {
		return Self {
			button_code: 0,
			is_pressed: false,
			long_press: false,

			special_fields: protobuf::SpecialFields::default(),
		};
	}

	fn default_instance() -> &'static Self {
		static INSTANCE: Lazy<ButtonPressMessage> = Lazy::new();
		return INSTANCE.get(ButtonPressMessage::new);
	}
}