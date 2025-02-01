use protobuf::{rt::*, Message};

pub const STREAM_TYPE_AUDIO: u32 = 1;
pub const STREAM_TYPE_VIDEO: u32 = 3;

pub const AUDIO_TYPE_MEDIA: u32 = 3;
pub const AUDIO_TYPE_VOICE: u32 = 1;

#[derive(Default, PartialEq, Clone)]
pub struct ChannelDescriptor {
	channel_id: u32,

	sensor_channel: Vec<SensorChannel>,
	
	//Input channel.
	input_event_channel: Vec<InputEventChannel>,

	//Output stream channel.
	output_stream_params: Vec<OutputStreamChannel>,
	input_stream_params: Vec<InputStreamChannel>,

	//Vendor extensions.
	//vendor_extension_params: Vec<VendorExtensionChannel>,
	
	//Empty channels.
	empty_params: Vec<u32>,

	navigation_status_params: Vec<NavigationStatusService>,

	special_fields: protobuf::SpecialFields,
}

impl ChannelDescriptor {
	pub fn get_channel_id(self) -> u32 {
		return self.channel_id;
	}

	pub fn set_channel_id(&mut self, channel_id: u32) {
		self.channel_id = channel_id;
	}

	pub fn new_with_id(channel_id: u32) -> Self {
		let mut new_self = Self::new();
		new_self.channel_id = channel_id;
		return new_self;
	}

	pub fn add_sensor_channel(&mut self) -> &mut SensorChannel {
		self.sensor_channel.push(SensorChannel::new());
		let index = self.sensor_channel.len() - 1;
		return &mut self.sensor_channel[index];
	}

	pub fn add_input_event(&mut self) -> &mut InputEventChannel {
		self.input_event_channel.push(InputEventChannel::new());
		let index = self.input_event_channel.len() - 1;
		return &mut self.input_event_channel[index];
	}

	pub fn add_output_stream(&mut self) -> &mut OutputStreamChannel {
		self.output_stream_params.push(OutputStreamChannel::new());
		let index = self.output_stream_params.len() - 1;
		return &mut self.output_stream_params[index];
	}

	pub fn add_input_stream(&mut self) -> &mut InputStreamChannel {
		self.input_stream_params.push(InputStreamChannel::new());
		let index = self.input_stream_params.len() - 1;
		return &mut self.input_stream_params[index];
	}

	pub fn add_navigation_service(&mut self) -> &mut NavigationStatusService {
		self.navigation_status_params.push(NavigationStatusService::new());
		let index = self.navigation_status_params.len() - 1;
		return &mut self.navigation_status_params[index];
	}

	/*pub fn add_vendor_extension(&mut self) -> &mut VendorExtensionChannel {
		self.vendor_extension_params.push(VendorExtensionChannel::new());
		let index = self.vendor_extension_params.len() - 1;
		return &mut self.vendor_extension_params[index];
	}*/
	
	pub fn add_empty_channel(&mut self, param: u32) {
		self.empty_params.push(param);
	}
}

impl Message for ChannelDescriptor {
	const NAME: &'static str = "Channel Descriptor";

	fn is_initialized(&self) -> bool {
		return true;
	}

	fn merge_from(&mut self, is: &mut protobuf::CodedInputStream) -> protobuf::Result<()> {
		todo!();
	}

	fn write_to_with_cached_sizes(&self, os: &mut protobuf::CodedOutputStream) -> protobuf::Result<()> {
		os.write_uint32(1, self.channel_id)?;

		for s in &self.sensor_channel {
			match s.write_to_bytes() {
				Ok(data) => {
					os.write_bytes(2, &data)?;
				}
				Err(e) => {
					println!("Error: {}", e);
				}
			}
		}

		for c in &self.output_stream_params {
			match c.write_to_bytes() {
				Ok(data) => {
					os.write_bytes(3, &data)?;
				}
				Err(e) => {
					println!("Error: {}", e);
				}
			}
		}

		for ev in &self.input_event_channel {
			match ev.write_to_bytes() {
				Ok(data) => {
					os.write_bytes(4, &data)?;
				}
				Err(e) => {
					println!("Error: {}", e);
				}
			}
		}

		for c in &self.input_stream_params {
			match c.write_to_bytes() {
				Ok(data) => {
					os.write_bytes(5, &data)?;
				}
				Err(e) => {
					println!("Error: {}", e);
				}
			}
		}

		for s in &self.navigation_status_params {
			match s.write_to_bytes() {
				Ok(data) => {
					os.write_bytes(8, &data)?;
				}
				Err(e) => {
					println!("Error: {}", e);
				}
			}
		}

		/*for v in &self.vendor_extension_params {
			match v.write_to_bytes() {
				Ok(data) => {
					os.write_bytes(12, &data)?;
				}
				Err(e) => {
					println!("Error: {}", e);
				}
			}
		}*/
		
		for e in &self.empty_params {
			os.write_bytes(*e, &[])?;
		}
		
		os.write_unknown_fields(self.special_fields.unknown_fields())?;
		return Ok(());
	}

	fn compute_size(&self) -> u64 {
		let mut total_size = 0;

		total_size += 1 + compute_raw_varint64_size(self.channel_id as u64);

		for s in &self.sensor_channel {
			total_size += s.compute_size();
		}

		for c in &self.output_stream_params {
			total_size += c.compute_size();
		}

		for ev in &self.input_event_channel {
			total_size += ev.compute_size();
		}

		for c in &self.input_stream_params {
			total_size += c.compute_size();
		}

		for s in &self.navigation_status_params {
			total_size += s.compute_size();
		}

		/*for v in &self.vendor_extension_params {
			total_size += v.compute_size();
		}*/
		
		for _e in &self.empty_params {
			total_size += 2;
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
			channel_id: 0,

			input_event_channel: Vec::new(),
			sensor_channel: Vec::new(),

			output_stream_params: Vec::new(),
			input_stream_params: Vec::new(),

			navigation_status_params: Vec::new(),

			//vendor_extension_params: Vec::new(),
			
			empty_params: Vec::new(),

			special_fields: protobuf::SpecialFields::default(),
		};
	}

	fn default_instance() -> &'static Self {
		static INSTANCE: Lazy<ChannelDescriptor> = Lazy::new();
		return INSTANCE.get(ChannelDescriptor::new);
	}
}

#[derive(Default, PartialEq, Clone)]
pub struct InputEventChannel {
	supported_keycodes: Vec<u32>,
	touch_config: Vec<TouchConfig>,

	special_fields: protobuf::SpecialFields,
}

impl InputEventChannel {
	pub fn add_keycode(&mut self, keycode: u32) {
		self.supported_keycodes.push(keycode);
	}

	pub fn add_touch_parameter(&mut self) -> &mut TouchConfig {
		self.touch_config.push(TouchConfig::new());
		let index = self.touch_config.len() - 1;
		return &mut self.touch_config[index];
	}
}

impl Message for InputEventChannel {
	const NAME: &'static str = "Input Event Channel";

	fn is_initialized(&self) -> bool {
		return true;
	}

	fn merge_from(&mut self, is: &mut protobuf::CodedInputStream) -> protobuf::Result<()> {
		while let Some(tag) = is.read_raw_tag_or_eof()? {
			match tag {
				8 => {
					self.supported_keycodes.push(is.read_uint32()?);
				}
				18 => {
					let mut touch_config = TouchConfig::new();
					let data = match is.read_bytes() {
						Ok(data) => data,
						Err(e) => {
							println!("Error: {}", e);
							continue;
						}
					};

					touch_config.merge_from_bytes(&data)?;
				}
				tag => {
					read_unknown_or_skip_group(tag, is, self.special_fields.mut_unknown_fields())?;
				}
			}
		}

		return Ok(());
	}

	fn write_to_with_cached_sizes(&self, os: &mut protobuf::CodedOutputStream) -> protobuf::Result<()> {
		for k in &self.supported_keycodes {
			os.write_uint32(1, *k)?;
		}

		for t in &self.touch_config {
			match t.write_to_bytes() {
				Ok(data) => {
					os.write_bytes(2, &data)?;
				}
				Err(e) => {
					println!("Error: {}", e);
				}
			}
		}
		
		os.write_unknown_fields(self.special_fields.unknown_fields())?;
		return Ok(());
	}

	fn compute_size(&self) -> u64 {
		let mut total_size = 0;

		for k in &self.supported_keycodes {
			total_size += 1 + compute_raw_varint64_size(*k as u64);
		}

		for t in &self.touch_config {
			total_size += t.compute_size();
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
			supported_keycodes: Vec::new(),
			touch_config: Vec::new(),

			special_fields: protobuf::SpecialFields::default(),
		}
	}

	fn default_instance() -> &'static Self {
		static INSTANCE: Lazy<InputEventChannel> = Lazy::new();
		return INSTANCE.get(InputEventChannel::new);
	}
}

#[derive(Default, PartialEq, Clone)]
pub struct TouchConfig {
	ts_width: u32,
	ts_height: u32,

	special_fields: protobuf::SpecialFields,
}

impl TouchConfig {
	pub fn set_dimensions(&mut self, ts_width: u32, ts_height: u32) {
		self.ts_height = ts_height;
		self.ts_width = ts_width;
	}
}

impl Message for TouchConfig {
	const NAME: &'static str = "Touch Input Configuration";

	fn is_initialized(&self) -> bool {
		return true;
	}

	fn merge_from(&mut self, is: &mut protobuf::CodedInputStream) -> protobuf::Result<()> {
		while let Some(tag) = is.read_raw_tag_or_eof()? {
			match tag {
				8 => {
					self.ts_width = is.read_uint32()?;
				}
				16 => {
					self.ts_height = is.read_uint32()?;
				}
				tag => {
					read_unknown_or_skip_group(tag, is, self.special_fields.mut_unknown_fields())?;
				}
			}
		}

		return Ok(());
	}

	fn write_to_with_cached_sizes(&self, os: &mut protobuf::CodedOutputStream) -> protobuf::Result<()> {
		os.write_uint32(1, self.ts_width)?;
		os.write_uint32(2, self.ts_height)?;
		
		os.write_unknown_fields(self.special_fields.unknown_fields())?;
		return Ok(());
	}

	fn compute_size(&self) -> u64 {
		let mut total_size = 0;

		total_size += 1 + compute_raw_varint64_size(self.ts_width as u64);
		total_size += 1 + compute_raw_varint64_size(self.ts_height as u64);
		
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
			ts_width: 800,
			ts_height: 480,

			special_fields: protobuf::SpecialFields::default(),
		}
	}

	fn default_instance() -> &'static Self {
		static INSTANCE: Lazy<TouchConfig> = Lazy::new();
		return INSTANCE.get(TouchConfig::new);
	}
}

#[derive(Default, PartialEq, Clone)]
pub struct SensorChannel {
	sensor_configs: Vec<SensorConfig>,

	special_fields: protobuf::SpecialFields,
}

impl SensorChannel {
	pub fn add_config(&mut self) -> &mut SensorConfig {
		self.sensor_configs.push(SensorConfig::new());
		let index = self.sensor_configs.len() - 1;
		return &mut self.sensor_configs[index];
	}
}

impl Message for SensorChannel {
	const NAME: &'static str = "Sensor Channel";

	fn is_initialized(&self) -> bool {
		return true;
	}

	fn merge_from(&mut self, is: &mut protobuf::CodedInputStream) -> protobuf::Result<()> {
		while let Some(tag) = is.read_raw_tag_or_eof()? {
			match tag {
				10 => {
					match is.read_bytes() {
						Ok(data) => {
							self.sensor_configs.push(SensorConfig::new());
							let index = self.sensor_configs.len() - 1;
							let sensor_config = &mut self.sensor_configs[index];

							sensor_config.merge_from_bytes(&data)?;
						}
						Err(e) => {
							println!("Error: {}", e);
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
		for s in &self.sensor_configs {
			match s.write_to_bytes() {
				Ok(data) => {
					os.write_bytes(1, &data)?;
				}
				Err(e) => {
					println!("Error: {}", e);
				}
			}
		}

		return Ok(());
	}

	fn compute_size(&self) -> u64 {
		let mut total_size = 0;

		for s in &self.sensor_configs {
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
			sensor_configs: Vec::new(),
			special_fields: protobuf::SpecialFields::default(),
		}
	}

	fn default_instance() -> &'static Self {
		static INSTANCE: Lazy<SensorChannel> = Lazy::new();
		return INSTANCE.get(SensorChannel::new);
	}
}

#[derive(Default, PartialEq, Clone)]
pub struct SensorConfig {
	pub sensor_type: u32,

	special_fields: protobuf::SpecialFields,
}

impl Message for SensorConfig {
	const NAME: &'static str = "Sensor";

	fn is_initialized(&self) -> bool {
		return true;
	}

	fn merge_from(&mut self, is: &mut protobuf::CodedInputStream) -> protobuf::Result<()> {
		while let Some(tag) = is.read_raw_tag_or_eof()? {
			match tag {
				8 => {
					self.sensor_type = is.read_uint32()?;
				}
				tag => {
					read_unknown_or_skip_group(tag, is, self.special_fields.mut_unknown_fields())?;
				}
			}
		}
		return Ok(());
	}

	fn write_to_with_cached_sizes(&self, os: &mut protobuf::CodedOutputStream) -> protobuf::Result<()> {
		os.write_uint32(1, self.sensor_type)?;
		os.write_unknown_fields(self.special_fields.unknown_fields())?;

		return Ok(());
	}

	fn compute_size(&self) -> u64 {
		let mut total_size = 1 + compute_raw_varint64_size(self.sensor_type as u64);

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
			sensor_type: 0,

			special_fields: protobuf::SpecialFields::default(),
		}
	}

	fn default_instance() -> &'static Self {
		static INSTANCE: Lazy<SensorConfig> = Lazy::new();
		return INSTANCE.get(SensorConfig::new);
	}
}

#[derive(Default, PartialEq, Clone)]
pub struct InputStreamChannel {
	pub stream_type: u32,

	audio_config: Vec<AudioConfig>,

	special_fields: protobuf::SpecialFields,
}

impl InputStreamChannel {
	pub fn add_audio_config(&mut self) -> &mut AudioConfig {
		self.audio_config.push(AudioConfig::new());
		let index = self.audio_config.len() - 1;
		return &mut self.audio_config[index];
	}
}

impl Message for InputStreamChannel {
	const NAME: &'static str = "Input Stream Channel";

	fn is_initialized(&self) -> bool {
		return true;
	}

	fn merge_from(&mut self, is: &mut protobuf::CodedInputStream) -> protobuf::Result<()> {
		while let Some(tag) = is.read_raw_tag_or_eof()? {
			match tag {
				8 => {
					self.stream_type = is.read_uint32()?;
				}
				18 => {
					//TODO: Read in audio config data.
				}
				tag => {
					read_unknown_or_skip_group(tag, is, self.special_fields.mut_unknown_fields())?;
				}
			}
		}

		return Ok(());
	}

	fn write_to_with_cached_sizes(&self, os: &mut protobuf::CodedOutputStream) -> protobuf::Result<()> {
		os.write_uint32(1, self.stream_type)?;

		for a in &self.audio_config {
			match a.write_to_bytes() {
				Ok(data) => {
					os.write_bytes(2, &data)?;
				}
				Err(e) => {
					println!("Error: {}", e);
				}
			}
		}

		os.write_unknown_fields(self.special_fields.unknown_fields())?;
		return Ok(());
	}

	fn compute_size(&self) -> u64 {
		let mut total_size = 0;

		total_size += 1 + compute_raw_varint64_size(self.stream_type as u64);

		for a in &self.audio_config {
			total_size += a.compute_size();
		}

		//Available in call bool.
		total_size += 2;

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
			stream_type: STREAM_TYPE_VIDEO,

			audio_config: Vec::new(),

			special_fields: protobuf::SpecialFields::default(),
		}
	}

	fn default_instance() -> &'static Self {
		static INSTANCE: Lazy<InputStreamChannel> = Lazy::new();
		return INSTANCE.get(InputStreamChannel::new);
	}
}

#[derive(Default, PartialEq, Clone)]
pub struct OutputStreamChannel {
	pub stream_type: u32,
	audio_type: u32,

	has_audio: bool,

	video_config: Vec<VideoConfig>,
	audio_config: Vec<AudioConfig>,

	available_in_call: bool,
	has_available_in_call: bool,

	special_fields: protobuf::SpecialFields,
}

impl OutputStreamChannel {
	pub fn set_audio_type(&mut self, audio_type: u32) {
		self.audio_type = audio_type;
		self.has_audio = true;
	}

	pub fn get_audio_type(self) -> u32 {
		return self.audio_type;
	}

	pub fn set_available_in_call(&mut self, available_in_call: bool) {
		self.available_in_call = available_in_call;
		self.has_available_in_call = true;
	}

	pub fn add_video_config(&mut self) -> &mut VideoConfig {
		self.video_config.push(VideoConfig::new());
		let index = self.video_config.len() - 1;
		return &mut self.video_config[index];
	}

	pub fn add_audio_config(&mut self) -> &mut AudioConfig {
		self.audio_config.push(AudioConfig::new());
		let index = self.audio_config.len() - 1;
		return &mut self.audio_config[index];
	}
}

impl Message for OutputStreamChannel {
	const NAME: &'static str = "Output Stream Channel";

	fn is_initialized(&self) -> bool {
		return true;
	}

	fn merge_from(&mut self, is: &mut protobuf::CodedInputStream) -> protobuf::Result<()> {
		while let Some(tag) = is.read_raw_tag_or_eof()? {
			match tag {
				8 => {
					self.stream_type = is.read_uint32()?;
				}
				16 => {
					self.audio_type = is.read_uint32()?;
					self.has_audio = true;
				}
				26 => {
					//TODO: Read in audio config data.
				}
				34 => {
					//TODO: Read in video config data.
				}
				40 => {
					self.available_in_call = is.read_bool()?;
				}
				tag => {
					read_unknown_or_skip_group(tag, is, self.special_fields.mut_unknown_fields())?;
				}
			}
		}

		return Ok(());
	}

	fn write_to_with_cached_sizes(&self, os: &mut protobuf::CodedOutputStream) -> protobuf::Result<()> {
		os.write_uint32(1, self.stream_type)?;

		if self.has_audio {
			os.write_uint32(2, self.audio_type)?;
		}

		for a in &self.audio_config {
			match a.write_to_bytes() {
				Ok(data) => {
					os.write_bytes(3, &data)?;
				}
				Err(e) => {
					println!("Error: {}", e);
				}
			}
		}
		
		for v in &self.video_config {
			match v.write_to_bytes() {
				Ok(data) => {
					os.write_bytes(4, &data)?;
				}
				Err(e) => {
					println!("Error: {}", e);
				}
			}
		}

		if self.has_available_in_call {
			os.write_bool(5, self.available_in_call)?;
		}

		os.write_unknown_fields(self.special_fields.unknown_fields())?;
		return Ok(());
	}

	fn compute_size(&self) -> u64 {
		let mut total_size = 0;

		total_size += 1 + compute_raw_varint64_size(self.stream_type as u64);

		if self.has_audio {
			total_size += 1 + compute_raw_varint64_size(self.audio_type as u64);
		}

		for a in &self.audio_config {
			total_size += a.compute_size();
		}

		for v in &self.video_config {
			total_size += v.compute_size();
		}

		//Available in call bool.
		if self.has_available_in_call {
			total_size += 2;
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
			stream_type: STREAM_TYPE_VIDEO,
			audio_type: 0,
			has_audio: false,

			video_config: Vec::new(),
			audio_config: Vec::new(),

			available_in_call: true,
			has_available_in_call: false,
			special_fields: protobuf::SpecialFields::default(),
		}
	}

	fn default_instance() -> &'static Self {
		static INSTANCE: Lazy<OutputStreamChannel> = Lazy::new();
		return INSTANCE.get(OutputStreamChannel::new);
	}
}

#[derive(Default, PartialEq, Clone)]
pub struct VideoConfig {
	pub video_resolution: u32,
	pub video_frame: u32,

	pub margin_width: u32,
	pub margin_height: u32,
	pub dpi: u32,
	pub additional_depth: u32,

	special_fields: protobuf::SpecialFields,
}

impl Message for VideoConfig {
	const NAME: &'static str = "Video Configuration";

	fn is_initialized(&self) -> bool {
		return true;
	}

	fn merge_from(&mut self, is: &mut protobuf::CodedInputStream) -> protobuf::Result<()> {
		while let Some(tag) = is.read_raw_tag_or_eof()? {
			match tag {
				8 => {
					self.video_resolution = is.read_uint32()?;
				}
				16 => {
					self.video_frame = is.read_uint32()?;
				}
				24 => {
					self.margin_width = is.read_uint32()?;
				}
				32 => {
					self.margin_height = is.read_uint32()?;
				}
				40 => {
					self.dpi = is.read_uint32()?;
				}
				48 => {
					self.additional_depth = is.read_uint32()?;
				}
				tag => {
					read_unknown_or_skip_group(tag, is, self.special_fields.mut_unknown_fields())?;
				}
			}
		}

		return Ok(());
	}

	fn write_to_with_cached_sizes(&self, os: &mut protobuf::CodedOutputStream) -> protobuf::Result<()> {
		os.write_uint32(1, self.video_resolution)?;
		os.write_uint32(2, self.video_frame)?;
		os.write_uint32(3, self.margin_width)?;
		os.write_uint32(4, self.margin_height)?;
		os.write_uint32(5, self.dpi)?;
		
		if self.additional_depth > 0 {
			os.write_uint32(6, self.additional_depth)?;
		}

		os.write_unknown_fields(self.special_fields.unknown_fields())?;
		return Ok(());
	}

	fn compute_size(&self) -> u64 {
		let mut total_size = 0;

		total_size += 1 + compute_raw_varint64_size(self.video_resolution as u64);
		total_size += 1 + compute_raw_varint64_size(self.video_frame as u64);
		total_size += 1 + compute_raw_varint64_size(self.margin_width as u64);
		total_size += 1 + compute_raw_varint64_size(self.margin_height as u64);
		total_size += 1 + compute_raw_varint64_size(self.dpi as u64);

		if self.additional_depth > 0 {
			total_size += 1 + compute_raw_varint64_size(self.additional_depth as u64);
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
			video_resolution: 1,
			video_frame: 1,
			margin_width: 0,
			margin_height: 0,
			dpi: 140,
			additional_depth: 0,
			special_fields: protobuf::SpecialFields::default(),
		}
	}

	fn default_instance() -> &'static Self {
		static INSTANCE: Lazy<VideoConfig> = Lazy::new();
		return INSTANCE.get(VideoConfig::new);
	}
}

#[derive(Default, PartialEq, Clone)]
pub struct AudioConfig {
	pub sample_rate: u32,
	pub bit_depth: u32,
	pub channel_count: u32,

	special_fields: protobuf::SpecialFields,
}

impl Message for AudioConfig {
	const NAME: &'static str = "Audio Configuration";

	fn is_initialized(&self) -> bool {
		return true;
	}

	fn merge_from(&mut self, is: &mut protobuf::CodedInputStream) -> protobuf::Result<()> {
		while let Some(tag) = is.read_raw_tag_or_eof()? {
			match tag {
				8 => {
					self.sample_rate = is.read_uint32()?;
				}
				16 => {
					self.bit_depth = is.read_uint32()?;
				}
				24 => {
					self.channel_count = is.read_uint32()?;
				}
				tag => {
					read_unknown_or_skip_group(tag, is, self.special_fields.mut_unknown_fields())?;
				}
			}
		}

		return Ok(());
	}

	fn write_to_with_cached_sizes(&self, os: &mut protobuf::CodedOutputStream) -> protobuf::Result<()> {
		os.write_uint32(1, self.sample_rate)?;
		os.write_uint32(2, self.bit_depth)?;
		os.write_uint32(3, self.channel_count)?;

		os.write_unknown_fields(self.special_fields.unknown_fields())?;

		return Ok(());
	}

	fn compute_size(&self) -> u64 {
		let mut total_size = 0;

		total_size += 1 + compute_raw_varint64_size(self.sample_rate as u64);
		total_size += 1 + compute_raw_varint64_size(self.bit_depth as u64);
		total_size += 1 + compute_raw_varint64_size(self.channel_count as u64);

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
			sample_rate: 48000,
			bit_depth: 16,
			channel_count: 2,

			special_fields: protobuf::SpecialFields::default(),
		}
	}

	fn default_instance() -> &'static Self {
		static INSTANCE: Lazy<AudioConfig> = Lazy::new();
		return INSTANCE.get(AudioConfig::new);
	}
}

#[derive(Default, PartialEq, Clone)]
pub struct NavigationStatusService {
	pub service_type: u8,
	pub minimum_interval: u32,

	special_fields: protobuf::SpecialFields,
}

impl Message for NavigationStatusService {
	const NAME: &'static str = "Navigation Status Service";

	fn is_initialized(&self) -> bool {
		return true;
	}

	fn merge_from(&mut self, is: &mut protobuf::CodedInputStream) -> protobuf::Result<()> {
		while let Some(tag) = is.read_raw_tag_or_eof()? {
			match tag {
				8 => {
					self.minimum_interval = is.read_uint32()?;
				}
				16 => {
					self.service_type = is.read_uint32()? as u8;
				}
				tag => {
					read_unknown_or_skip_group(tag, is, self.special_fields.mut_unknown_fields())?;
				}
			}
		}

		return Ok(());
	}

	fn write_to_with_cached_sizes(&self, os: &mut protobuf::CodedOutputStream) -> protobuf::Result<()> {
		os.write_uint32(1, self.minimum_interval)?;
		os.write_uint32(2, self.service_type as u32)?;
		os.write_unknown_fields(self.special_fields.unknown_fields())?;

		return Ok(());
	}

	fn compute_size(&self) -> u64 {
		let mut total_size = 0;

		total_size += 1 + compute_raw_varint64_size(self.minimum_interval as u64);
		total_size += 1 + compute_raw_varint64_size(self.service_type as u64);

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
			service_type: 2,
			minimum_interval: 1000,

			special_fields: protobuf::SpecialFields::default(),
		}
	}

	fn default_instance() -> &'static Self {
		static INSTANCE: Lazy<NavigationStatusService> = Lazy::new();
		return INSTANCE.get(NavigationStatusService::new);
	}
}

/*#[derive(Default, PartialEq, Clone)]
pub struct VendorExtensionChannel {
	pub name: String,
	pub data: Vec<u8>,

	special_fields: protobuf::SpecialFields,
}

impl Message for VendorExtensionChannel {
	const NAME: &'static str = "Vendor Extension Channel";

	fn is_initialized(&self) -> bool {
		return true;
	}

	fn merge_from(&mut self, is: &mut protobuf::CodedInputStream) -> protobuf::Result<()> {
		while let Some(tag) = is.read_raw_tag_or_eof()? {
			match tag {
				10 => {
					self.name = is.read_string()?;
				}
				26 => {
					self.data = is.read_bytes()?;
				}
				tag => {
					read_unknown_or_skip_group(tag, is, self.special_fields.mut_unknown_fields())?;
				}
			}
		}

		return Ok(());
	}

	fn write_to_with_cached_sizes(&self, os: &mut protobuf::CodedOutputStream) -> protobuf::Result<()> {
		os.write_string(1, &self.name)?;
		os.write_bytes(3, &self.data)?;

		os.write_unknown_fields(self.special_fields.unknown_fields())?;

		return Ok(());
	}

	fn compute_size(&self) -> u64 {
		let mut total_size = 0;

		total_size += string_size(1, &self.name);
		total_size += bytes_size(3, &self.data);
		
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
			name: "".to_string(),
			data: Vec::new(),
			special_fields: protobuf::SpecialFields::default(),
		}
	}

	fn default_instance() -> &'static Self {
		static INSTANCE: Lazy<VendorExtensionChannel> = Lazy::new();
		return INSTANCE.get(VendorExtensionChannel::new);
	}
}*/