pub struct Context {
	pub audio_on: bool, //True if the BMW audio system is on.
    pub audio_open: bool, //True if the audio window is open.
	pub audio_selected: bool, //True if the MKA is selected as the active audio device.
	pub phone_active: bool, //True if phone mirroring is active.
	pub fullscreen: bool, //True on the final Raspberry Pi, false for testing.
	pub playing: bool, //True if the phone is playing music.

	pub phone_type: u8, //The phone type, as defined by the dongle.
	pub phone_name: String, //The name of the phone.

    pub song_title: String, //The song title.
    pub artist: String, //The artist name.
    pub album: String, // The album name.
    pub app: String, //The app name.

    pub version: i8, //The nav computer version.

    pub headlights_on: bool, //True if headlights are turned on.
}

impl Context {
    pub fn new() -> Self {

        return Self {
            audio_on: false,
            audio_open: false,
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

            version: 5,

            headlights_on: false,
        };
    }
}
