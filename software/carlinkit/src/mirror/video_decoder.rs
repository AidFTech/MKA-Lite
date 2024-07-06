use mpv::*;

pub fn get_video_decoder(w: u32, h: u32, fullscreen: bool, socket_address: String) -> Option<MpvHandler> {
	let mut mpv_builder = match mpv::MpvHandlerBuilder::new() {
		Ok(mpv_builder) => mpv_builder,
		Err(_) => {
			return None;
		}
	};

	match mpv_builder.set_option("input-default-bindings", false) {
		Ok(_) => {

		}
		Err(_) => {
			println!("Could not set input-default-bindings.");
			return None;
		}
	}

	match mpv_builder.set_option("untimed", true) {
		Ok(_) => {

		}
		Err(_) => {
			println!("Could not set untimed.");
			return None;
		}
	}

	match mpv_builder.set_option("opengl-glfinish", true) {
		Ok(_) => {

		}
		Err(_) => {
			println!("Could not set opengl-glfinish.");
			return None;
		}
	}

	match mpv_builder.set_option("hwdec-extra-frames", 75) {
		Ok(_) => {

		}
		Err(_) => {
			println!("Could not set hwdec-extra-frames.");
			return None;
		}
	}

	match mpv_builder.set_option("audio", false) {
		Ok(_) => {

		}
		Err(_) => {
			println!("Could not set audio.");
			return None;
		}
	}

	match mpv_builder.set_option("demuxer-rawvideo-fps", 30) {
		Ok(_) => {

		}
		Err(_) => {
			println!("Could not set demuxer-rawvideo-fps.");
			return None;
		}
	}

	match mpv_builder.set_option("fps", 30) {
		Ok(_) => {

		}
		Err(_) => {
			println!("Could not set fps.");
			return None;
		}
	}

	match mpv_builder.set_option("video-aspect-override", (h/w) as i64) {
		Ok(_) => {

		}
		Err(_) => {
			println!("Could not set video-aspect-override.");
			return None;
		}
	}

	match mpv_builder.set_option("fs", fullscreen) {
		Ok(_) => {

		}
		Err(_) => {
			println!("Could not set fs.");
			return None;
		}
	}

	match mpv_builder.set_option("keep-open", true) {
		Ok(_) => {

		}
		Err(_) => {
			println!("Could not set fs.");
			return None;
		}
	}

	match mpv_builder.set_option("input-ipc-server", socket_address.as_str()) {
		Ok(_) => {

		}
		Err(_) => {
			println!("Could not set input-ipc-server.");
			return None;
		}
	}

	match mpv_builder.build() {
		Ok(player) => {
			return Some(player);
		}
		Err(_) => {
			return None;
		}
	}
}