pub struct Context {
	pub bmbt_connected: bool,
	pub audio_selected: bool,
	pub phone_active: bool,
	pub fullscreen: bool,
	pub playing: bool,

	pub phone_type: u8,
	pub phone_name: String,

    pub song_title: String,
    pub artist: String,
    pub album: String,
    pub app: String,

    pub version: i8,

    pub headlights_on: bool,
}

impl Context {
    pub fn new() -> Self {

        return Self {
            bmbt_connected: false,
            audio_selected: false,
            phone_active: true,
            fullscreen: false,
            playing: false,

            phone_type: 0,
            phone_name: "".to_string(),

            song_title: "".to_string(),
            artist: "".to_string(),
            album: "".to_string(),
            app: "".to_string(),

            version: -1,

            headlights_on: false,
        };
    }
}
