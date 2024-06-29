//Parameter list.

use crate::IBusMessage;

pub struct ParameterList {
	pub bmbt_connected: bool,
	pub audio_selected: bool,
	pub mka_active: bool,
	pub phone_active: bool,
	pub fullscreen: bool,
	pub playing: bool,

	pub phone_type: u8,
	pub phone_name: String,

	pub ibus_waiting: bool,
	pub ibus_cache: IBusMessage,
}

pub fn get_parameter_list() -> ParameterList {
	return ParameterList {
		bmbt_connected: false,
		audio_selected: false,
		mka_active: false,
		phone_active: false,
		fullscreen: false,
		playing: false,
		
		phone_type: 0,
		phone_name: "".to_string(),

		ibus_waiting: false,
		ibus_cache: IBusMessage {
			sender: 0,
			receiver: 0,
			data: Vec::new(),
		}
	};
}