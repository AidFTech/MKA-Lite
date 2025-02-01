use protobuf::{rt::*, Message};

pub struct VideoMsg {
	data: Vec<u8>,
	msg_type: u16,
}

impl VideoMsg {
	pub fn get_data(self) -> Vec<u8> {
		return self.data;
	}

	pub fn get_type(self) -> u16 {
		return self.msg_type;
	}

	pub fn set_data(&mut self, data: &[u8]) {
		if data.len() < 2 {
			return;
		}

		self.msg_type = u16::from_be_bytes([data[0], data[1]]);
		let mut start = 2;

		if self.msg_type == 0 {
			start = 10;
		}

		self.data = data[start..].to_vec();
	}

	pub fn new() -> Self {
		return Self {
			data: Vec::new(),
			msg_type: 1,
		}
	}
}

pub struct AudioMsg {
	data: Vec<u8>,
	msg_type: u16,
	channel: u8,
}

impl AudioMsg {
	pub fn get_data(&self) -> Vec<u8> {
		return self.data.clone();
	}

	pub fn get_type(&self) -> u16 {
		return self.msg_type;
	}

	pub fn get_channel(&self) -> u8 {
		return self.channel;
	}
	
	pub fn set_data(&mut self, data: &[u8]) {
		if data.len() < 2 {
			return;
		}

		self.msg_type = u16::from_be_bytes([data[0], data[1]]);
		let mut start = 2;

		if self.msg_type == 0 { //Contains timestamp.
			start = 10;
		}

		self.data = data[start..].to_vec();
	}

	pub fn set_channel(&mut self, channel: u8) {
		self.channel = channel;
	}

	pub fn new() -> Self {
		return Self {
			data: Vec::new(),
			msg_type: 1,
			channel: 4,
		}
	}
}

#[derive(Default, PartialEq, Clone)]
pub struct MediaStartRequest {
	pub session: i32,
	pub config: u32, 

	special_fields: protobuf::SpecialFields,
}

impl Message for MediaStartRequest {
	const NAME: &'static str = "Media Start Request";

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
					self.config = is.read_uint32()?;
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
		os.write_uint32(2, self.config)?;
		
		os.write_unknown_fields(self.special_fields.unknown_fields())?;
		return Ok(());
	}

	fn compute_size(&self) -> u64 {
		let mut total_size = 1;
		if self.session >= 0 {
			total_size += compute_raw_varint64_size(self.session as u64);
		} else {
			total_size += 10;
		}

		total_size += 1 + compute_raw_varint64_size(self.config as u64);
		
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
			config: 0,
			special_fields: protobuf::SpecialFields::default(),
		}
	}

	fn default_instance() -> &'static Self {
		static INSTANCE: Lazy<MediaStartRequest> = Lazy::new();
		return INSTANCE.get(MediaStartRequest::new);
	}
}

#[derive(Default, PartialEq, Clone)]
pub struct MediaPlaybackMessage {
	pub playback_state: u8,
	pub media_app: String,
	pub track_progress: i32,

	u1: i32,
	u2: i32,
	u3: i32,

	special_fields: protobuf::SpecialFields,
}

impl Message for MediaPlaybackMessage {
	const NAME: &'static str = "Media Playback Message";

	fn is_initialized(&self) -> bool {
		return true;
	}

	fn merge_from(&mut self, is: &mut protobuf::CodedInputStream) -> protobuf::Result<()> {
		while let Some(tag) = is.read_raw_tag_or_eof()? {
			match tag {
				8 => {
					self.playback_state = is.read_uint32()? as u8;
				}
				18 => {
					self.media_app = is.read_string()?;
				}
				24 => {
					self.track_progress = is.read_int32()?;
				}
				32 => {
					self.u1 = is.read_int32()?;
				}
				40 => {
					self.u2 = is.read_int32()?;
				}
				48 => {
					self.u3 = is.read_int32()?;
				}
				tag => {
					read_unknown_or_skip_group(tag, is, self.special_fields.mut_unknown_fields())?;
				}
			}
		}

		return Ok(());
	}

	fn write_to_with_cached_sizes(&self, os: &mut protobuf::CodedOutputStream) -> protobuf::Result<()> {
		os.write_uint32(1, self.playback_state as u32)?;
		os.write_string(2, &self.media_app)?;
		os.write_int32(3, self.track_progress)?;
		os.write_int32(4, self.u1)?;
		os.write_int32(5, self.u2)?;
		os.write_int32(6, self.u3)?;
		
		os.write_unknown_fields(self.special_fields.unknown_fields())?;
		return Ok(());
	}

	fn compute_size(&self) -> u64 {
		let mut total_size = 0;

		total_size += 1 + compute_raw_varint64_size(self.playback_state as u64);
		total_size += string_size(2, &self.media_app);
		
		total_size += 1;
		if self.track_progress >= 0 {
			total_size += compute_raw_varint64_size(self.track_progress as u64);
		} else {
			total_size += 10;
		}

		total_size += 1;
		if self.u1 >= 0 {
			total_size += compute_raw_varint64_size(self.u1 as u64);
		} else {
			total_size += 10;
		}

		total_size += 1;
		if self.u2 >= 0 {
			total_size += compute_raw_varint64_size(self.u2 as u64);
		} else {
			total_size += 10;
		}

		total_size += 1;
		if self.u3 >= 0 {
			total_size += compute_raw_varint64_size(self.u3 as u64);
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
			playback_state: 0,
			media_app: "".to_string(),
			track_progress: 0,

			u1: 0,
			u2: 0,
			u3: 0,

			special_fields: protobuf::SpecialFields::default(),
		}
	}

	fn default_instance() -> &'static Self {
		static INSTANCE: Lazy<MediaPlaybackMessage> = Lazy::new();
		return INSTANCE.get(MediaPlaybackMessage::new);
	}
}

#[derive(Default, PartialEq, Clone)]
pub struct MediaMetaMessage {
	pub track_name: String,
	pub artist_name: String,
	pub album_name: String,
	pub album_art: Vec<u8>,
	pub track_length: i32,
	u1: i32,

	special_fields: protobuf::SpecialFields,
}

impl Message for MediaMetaMessage {
	const NAME: &'static str = "Media Metadata Message";

	fn is_initialized(&self) -> bool {
		return true;
	}

	fn merge_from(&mut self, is: &mut protobuf::CodedInputStream) -> protobuf::Result<()> {
		while let Some(tag) = is.read_raw_tag_or_eof()? {
			match tag {
				10 => {
					self.track_name = is.read_string()?;
				}
				18 => {
					self.artist_name = is.read_string()?;
				}
				26 => {
					self.album_name = is.read_string()?;
				}
				34 => {
					self.album_art = is.read_bytes()?
				}
				48 => {
					self.track_length = is.read_int32()?;
				}
				56 => {
					self.u1 = is.read_int32()?;
				}
				tag => {
					read_unknown_or_skip_group(tag, is, self.special_fields.mut_unknown_fields())?;
				}
			}
		}

		return Ok(());
	}

	fn write_to_with_cached_sizes(&self, os: &mut protobuf::CodedOutputStream) -> protobuf::Result<()> {
		os.write_string(1, &self.track_name)?;
		os.write_string(2, &self.artist_name)?;
		os.write_string(3, &self.album_name)?;

		if self.album_art.len() > 0 {
			os.write_bytes(4, &self.album_art)?;
		}

		os.write_int32(6, self.track_length)?;
		os.write_int32(7, self.u1)?;
		
		os.write_unknown_fields(self.special_fields.unknown_fields())?;
		return Ok(());
	}

	fn compute_size(&self) -> u64 {
		let mut total_size = 0;

		total_size += string_size(1, &self.track_name);
		total_size += string_size(2, &self.artist_name);
		total_size += string_size(3, &self.album_name);

		if self.album_art.len() > 0 {
			total_size += bytes_size(4, &self.album_art);
		}

		total_size += 1;
		if self.track_length >= 0 {
			total_size += compute_raw_varint64_size(self.track_length as u64);
		} else {
			total_size += 10;
		}

		total_size += 1;
		if self.u1 >= 0 {
			total_size += compute_raw_varint64_size(self.u1 as u64);
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
			track_name: "".to_string(),
			artist_name: "".to_string(),
			album_name: "".to_string(),
			album_art: Vec::new(),
			track_length: 0,
			u1: 0,

			special_fields: protobuf::SpecialFields::default(),
		}
	}

	fn default_instance() -> &'static Self {
		static INSTANCE: Lazy<MediaMetaMessage> = Lazy::new();
		return INSTANCE.get(MediaMetaMessage::new);
	}
}