use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::sync::{Arc, Mutex};
use std::time::SystemTime;

use crate::{Context, IBusMessage};
use crate::USBConnection;

use crate::ibus::*;

use super::messages::{get_carplay_command_message, get_sendfile_message};
use super::messages::get_manufacturer_info;
use super::messages::get_open_message;
use super::messages::get_sendint_message;
use super::messages::get_sendstring_message;
use super::messages::get_heartbeat_message;
use super::messages::MirrorMessage;
use super::messages::MetaDataMessage;
use super::mpv::{get_decode_type, MpvVideo};
use super::mpv::RdAudio;

pub const SONG_NAME: u8 = 1;
pub const ARTIST: u8 = 2;
pub const ALBUM: u8 = 3;
pub const APP: u8 = 4;

//const PHONE_LED_OFF: u8 = 0;
const PHONE_LED_GREEN: u8 = 1;
const PHONE_LED_RED: u8 = 2;

pub struct MirrorHandler<'a> {
    context: &'a Arc<Mutex<Context>>,
    usb_conn: USBConnection,
    run: bool,
    startup: bool,
    ibus_handler: &'a Arc<Mutex<IBusHandler>>,
    mpv_video: MpvVideo,
    rd_audio: RdAudio,
    nav_audio: RdAudio,
    heartbeat_time: SystemTime,
}

impl<'a> MirrorHandler<'a> {
    pub fn new(context: &'a Arc<Mutex<Context>>, ibus_handler: &'a Arc<Mutex<IBusHandler>>, w: u16, h: u16) -> MirrorHandler <'a> {
        let mut mpv_found = 0;
        let mut mpv_video: Option<MpvVideo> = None;
        let mut rd_audio: Option<RdAudio> = None;
        let mut nav_audio: Option<RdAudio> = None;

        while mpv_found < 3 {
            match MpvVideo::new(w, h) {
                Err(e) => println!("Failed to Start Mpv: {}", e.to_string()),
                Ok(mpv) => {
                    mpv_video = Some(mpv);
                    mpv_found += 1;
                }
            };

            match RdAudio::new() {
                Err(e) => println!("Failed to Start Rodio: {}", e.to_string()),
                Ok(rodio) => {
                    rd_audio = Some(rodio);
                    mpv_found += 1;
                }
            }
            
            match RdAudio::new() {
                Err(e) => println!("Failed to Start Rodio: {}", e.to_string()),
                Ok(rodio) => {
                    nav_audio = Some(rodio);
                    mpv_found += 1;
                }
            }
        }

        return MirrorHandler {
            context,
            usb_conn: USBConnection::new(),
            run: true,
            startup: false,
            ibus_handler,
            mpv_video: mpv_video.unwrap(),
            rd_audio: rd_audio.unwrap(),
            nav_audio: nav_audio.unwrap(),
            heartbeat_time: SystemTime::now(),
        };
    }

    pub fn process(&mut self) {
        let phone_type = match self.context.try_lock() {
            Ok(context) => {
                context.phone_type
            }
            Err(_) => {
                return;
            }
        };

        if !self.run {
            return;
        }
        if !self.usb_conn.connected {
            if phone_type == 3 {
                match self.context.try_lock() {
					Ok(mut context) => {
						context.phone_type = 0;
					}
					Err(_) => {
						println!("Carplay Handler stop connection: Context locked.");
						return;
					}
				};
            }

            self.startup = false;
            let run = self.usb_conn.connect();

            if !run {
                return; //TODO: Should we still run full_loop even if no dongle is connected?
            } else {
                self.run = true;
            }
        } else if !self.startup {
            self.send_dongle_startup();
        }
        let mirror_message = self.usb_conn.read();

        match mirror_message {
            Some(mirror_message) => self.interpret_message(&mirror_message),
            None => ()
        }
        self.heartbeat();
    }

    fn heartbeat(&mut self) {
        if self.heartbeat_time.elapsed().unwrap().as_millis() > 2000 {
            self.heartbeat_time = SystemTime::now();
            self.usb_conn.write_message(get_heartbeat_message());
        }
    }

    pub fn send_carplay_command(&mut self, command: u32) {
        let msg = get_carplay_command_message(command);
        self.usb_conn.write_message(msg);
    }

    pub fn handle_ibus_message(&mut self, ibus_msg: IBusMessage) {
        let mut context = match self.context.try_lock() {
            Ok(context) => context,
            Err(_) => {
                println!("IBus: Context locked.");
                return;
            }
        };
        
        if ibus_msg.sender == IBUS_DEVICE_BMBT { //From BMBT.
            if ibus_msg.l() >= 2 && ibus_msg.data[0] == 0x49 && context.phone_active {
                let clockwise = ibus_msg.data[1]&0x80 != 0;
                let steps = ibus_msg.data[1]&0x7F;

                let mut cmd: u32 = 100;
                if clockwise {
                    cmd = 101;
                }

                for _i in 0..steps {
                    self.send_carplay_command(cmd);
                }
            } else if ibus_msg.l() >= 2 && ibus_msg.data[0] == 0x48 && context.phone_active {
                let command = ibus_msg.data[1]&0x3F;
                let state = (ibus_msg.data[1]&0xC0) >> 6;
                if command == 0x5 && state == 0x2 { //Enter button.
                    self.send_carplay_command(104);
                    self.send_carplay_command(105);
                } else if command == 0x8 && state == 0x2 { //Phone button released.
                    self.send_carplay_command(106);
                } else if command == 0x8 && state == 0x1 { //Phone button held.
                    self.send_carplay_command(200);
                } else if command == 0x14 && state == 0x2 { //Direction button released.
                    self.send_carplay_command(203);
                }
            }
        } else if ibus_msg.sender == 0xD0 && ibus_msg.l() >= 2 && ibus_msg.data[0] == 0x5B {
            //TODO: Do not run this command if the RLS is connected.
            let last_headlights_on = context.headlights_on;
            if ((ibus_msg.data[1]&0x01) != 0) != last_headlights_on {
                let headlights_on = (ibus_msg.data[1]&0x01) != 0;
                context.headlights_on = headlights_on;
                if headlights_on {
                    self.send_carplay_command(16);
                } else {
                    self.send_carplay_command(17);
                }
            }
        }
    }

    fn send_dongle_startup(&mut self) {
        let mut dongle_message_dpi = get_sendint_message(String::from("/tmp/screen_dpi"), 160);
        let mut dongle_message_android = get_sendint_message(String::from("/etc/android_work_mode"), 1);
        let dongle_message_open = get_open_message(800, 480, 30, 5, 49152, 2, 2);

        self.usb_conn.write_message(dongle_message_dpi.get_mirror_message());
        self.usb_conn.write_message(dongle_message_android.get_mirror_message());
        self.usb_conn.write_message(dongle_message_open);
        
        //Send airplay.conf...
        let mut config_file = match File::open(Path::new("airplay.conf")) {
            Ok(file) => file,
            Err(err) => {
                println!("Error opening file: {}", err);
                return;
            }
        };
        
        let mut config_data = Vec::new();
        match config_file.read_to_end(&mut config_data) {
            Ok(_) => {

            }
            Err(err) => {
                println!("Error reading file: {}", err);
                return;
            }
        };
        
        let mut config_msg = get_sendfile_message("/etc/airplay.conf".to_string(), config_data);
        self.usb_conn.write_message(config_msg.get_mirror_message());
        
        //Send BMW.png...
        let mut android_icon_file = match File::open(Path::new("BMW.png")) {
            Ok(file) => file,
            Err(err) => {
                println!("Error opening file: {}", err);
                return;
            }
        };

        let mut android_icon_data: Vec<u8> = Vec::new();
        match android_icon_file.read_to_end(&mut android_icon_data) {
            Ok(_) => {

            }
            Err(err) => {
                println!("Error reading file: {}", err);
                return;
            }
        };

        let mut android_icon_msg = get_sendfile_message("/etc/oem_icon.png".to_string(), android_icon_data);
        self.usb_conn.write_message(android_icon_msg.get_mirror_message());
        
        //Send BMW_icon.png...
        let mut carplay_icon_file = match File::open(Path::new("BMW_icon.png")) {
            Ok(file) => file,
            Err(err) => {
                println!("Error opening file: {}", err);
                return;
            }
        };
        
        let mut carplay_icon_data: Vec<u8> = Vec::new();
        match carplay_icon_file.read_to_end(&mut carplay_icon_data) {
            Ok(_) => {

            }
            Err(err) => {
                println!("Error reading file: {}", err);
                return;
            }
        };
        
        let mut carplay_icon_msg = get_sendfile_message("/etc/icon_120x120.png".to_string(), carplay_icon_data);
        self.usb_conn.write_message(carplay_icon_msg.get_mirror_message());
        
        //carplay_icon_msg = get_sendfile_message("/etc/icon_180x180.png".to_string(), carplay_icon_data);
        //self.usb_conn.write_message(carplay_icon_msg.get_mirror_message());
        
        //carplay_icon_msg = get_sendfile_message("/etc/icon_256x256.png".to_string(), carplay_icon_data);
        //self.usb_conn.write_message(carplay_icon_msg.get_mirror_message());
    }

    fn interpret_message(&mut self, message: &MirrorMessage) {
        if message.message_type == 0x1 {
            // Open message.
            self.startup = true;
            println!("Starting Carlinkit...");

            let startup_msg_manufacturer = get_manufacturer_info(0, 0);
            let mut startup_msg_night = get_sendint_message(String::from("/tmp/night_mode"), 0);
            let mut startup_msg_hand_drive = get_sendint_message(String::from("/tmp/hand_drive_mode"), 0); //0=left, 1=right
            let mut startup_msg_charge = get_sendint_message(String::from("/tmp/charge_mode"), 0);
            let mut startup_msg_name = get_sendstring_message(String::from("/etc/box_name"), String::from("MKA"));
            let startup_msg_carplay = get_carplay_command_message(101);

            self.usb_conn.write_message(startup_msg_manufacturer);
            self.usb_conn.write_message(startup_msg_night.get_mirror_message());
            self.usb_conn.write_message(startup_msg_hand_drive.get_mirror_message());
            self.usb_conn.write_message(startup_msg_charge.get_mirror_message());
            self.usb_conn.write_message(startup_msg_name.get_mirror_message());
            self.usb_conn.write_message(startup_msg_carplay);

            let mut startup_msg_meta = MetaDataMessage::new(25);
            startup_msg_meta.add_int(String::from("mediaDelay"), 300);
            startup_msg_meta.add_int(String::from("androidAutoSizeW"), 800);
            startup_msg_meta.add_int(String::from("androidAutoSizeH"), 480);
            self.usb_conn.write_message(startup_msg_meta.get_mirror_message());

            let mut msg_91 = MirrorMessage::new(9);
            msg_91.push_int(1);
            self.usb_conn.write_message(msg_91);

            let mut msg_88 = MirrorMessage::new(0x88);
            msg_88.push_int(1);
            self.usb_conn.write_message(msg_88);
            self.heartbeat_time = SystemTime::now();
        } else if message.message_type == 2 {
            println!("Phone trying to connect...");
            //Phone connected.
            let data = message.clone().decode();
            if data.len() <= 0 {
                return;
            }
            println!("Phone Connected!");
            let phone_type = data[0];
            let mut selected = false;
            match self.context.try_lock() {
                Ok(mut context) => {
                    context.phone_type = phone_type as u8;
                    selected = context.audio_selected;
                }
                Err(_) => {
                }
            }
            
            self.mpv_video.start();
            self.set_phone_light(PHONE_LED_GREEN);

            if selected {
                self.send_radio_screen_update();
            }

        } else if message.message_type == 4 {
            // Phone disconnected.
            self.mpv_video.stop();
            self.set_phone_light(PHONE_LED_RED);

            let mut selected = false;
            match self.context.try_lock() {
                Ok(mut context) => {
                    context.phone_type = 0;
                    context.phone_name = "".to_string();
                    context.song_title = "".to_string();
                    context.artist = "".to_string();
                    context.album = "".to_string();
                    context.app = "".to_string();
                    selected = context.audio_selected;
                }
                Err(_) => {
                }
            }

            if selected {
                self.send_radio_screen_update();
            }

        } else if message.message_type == 6 { //Video.
            let mut data = vec![0;0];
            for i in 20..message.data.len() {
                data.push(message.data[i]);
            }
            self.mpv_video.send_video(&data);
        } else if message.message_type == 7 { //Audio.
            if message.data.len() > 16 {
                let (current_sample, current_bits, current_channel) = self.rd_audio.get_audio_profile();

                let decode_num = u32::from_le_bytes([message.data[0], message.data[1], message.data[2], message.data[3]]);
                let (new_sample, new_bits, new_channel) = get_decode_type(decode_num);

                let audio_type = u32::from_le_bytes([message.data[8], message.data[9], message.data[10], message.data[11]]);

                if new_sample != current_sample || new_bits != current_bits || new_channel != current_channel {
                    if audio_type == 1 {
                        self.rd_audio.set_audio_profile(new_sample, new_bits, new_channel);
                    } else if audio_type == 2 {
                        self.nav_audio.set_audio_profile(new_sample, new_bits, new_channel);
                    }
                }

                let mut data = Vec::new();
                for i in 12..message.data.len() {
                    data.push(message.data[i]);
                }

                if audio_type == 1 {
                    self.rd_audio.send_audio(&data);
                } else if audio_type == 2 {
                    self.nav_audio.send_audio(&data);
                }
            }
        } else if message.message_type == 25 || message.message_type == 42 {
            // Handle metadata.
            let meta_message = MetaDataMessage::from(message.clone());
            self.handle_metadata(meta_message);
        }
    }

    fn handle_metadata(&mut self, meta_message: MetaDataMessage) {
        let mut context = match self.context.try_lock() {
             Ok(context) =>{
                context
            }
            Err(_) => {
                println!("Metadata: Context locked.");
                return;
            }
        };

        let mut phone_name_changed = false;
        let mut song_title_changed = false;
        let mut artist_changed = false;
        let mut album_changed = false;
        let mut app_changed = false;

        for string_var in meta_message.string_vars {
            if string_var.variable == "MDModel" {
                context.phone_name = string_var.value;
                phone_name_changed = true;
            } else if string_var.variable == "MediaSongName" {
                if context.song_title != string_var.value {
                    context.song_title = string_var.value;
                    song_title_changed = true;
                }
            } else if string_var.variable == "MediaArtistName" {
                if context.artist != string_var.value {
                    context.artist = string_var.value;
                    artist_changed = true;
                }
            } else if string_var.variable == "MediaAlbumName" {
                if context.album != string_var.value {
                    context.album = string_var.value;
                    album_changed = true;
                }
            } else if string_var.variable == "MediaAPPName" {
                if context.app != string_var.value {
                    context.app = string_var.value;
                    app_changed = true;

                    if !song_title_changed {
                        context.song_title = "".to_string();
                        song_title_changed = true;
                    }
                    if !artist_changed {
                        context.artist = "".to_string();
                        artist_changed = true;
                    }
                    if !album_changed {
                        context.album = "".to_string();
                        album_changed = true;
                    }
                }
            }
        }

        if context.audio_selected && phone_name_changed {
            std::mem::drop(context);
            self.send_radio_screen_update();
        } else if context.audio_selected && (song_title_changed || artist_changed || album_changed || app_changed) {
            self.send_all_radio_center_text(context.version, true, context.song_title.clone(), context.artist.clone(), context.album.clone(), context.app.clone());
        }
    }

    fn send_radio_screen_update(&mut self) {
        let context = match self.context.try_lock() {
            Ok(context) => context,
            Err(_) => {
                println!("Screen Update: Context locked.");
                return;
            }
        };

        let version = context.version;

        self.send_all_radio_center_text(version, true, context.song_title.clone(), context.artist.clone(), context.album.clone(), context.app.clone());
        let phone_type = context.phone_type;
        if phone_type == 3 {
            self.send_radio_main_text("CarPlay".to_string());
        } else if phone_type == 5 {
            self.send_radio_main_text("Android".to_string());
        } else {
            self.send_radio_main_text("MKA".to_string());
        }

        self.send_radio_subtitle_text(" ".to_string(), 1, false);
        
        if context.playing {
            self.send_radio_subtitle_text(">".to_string(), 2, false);
        } else {
            self.send_radio_subtitle_text("||".to_string(), 2, false);
        }

        let phone_name = context.phone_name.clone();
        self.send_radio_subtitle_text(phone_name, 6, true);
    }

//Send a radio header change message.
    fn send_radio_main_text(&mut self, text: String) {
        let mut ibus_handler = match self.ibus_handler.try_lock() {
            Ok(ibus_handler) => ibus_handler,
            Err(_) => {
                println!("Radio Main Text: IBus handler locked.");
                return;
            }
        };

        let mut text_data = Vec::new();
        text_data.push(IBUS_CMD_GT_WRITE_TITLE);
        text_data.push(0x62);
        text_data.push(0x30);

        let text_bytes = text.as_bytes();

        if text.len() >= 1 {
            for i in 0..text.len() {
                text_data.push(text_bytes[i]);
            }
        } else {
            text_data.push(0x20);
        }

        text_data.push(0x8E);

        let text_msg = IBusMessage {
            sender: IBUS_DEVICE_RAD,
            receiver: IBUS_DEVICE_GT,
            data: text_data,
        };
        ibus_handler.write_ibus_message(text_msg);
    }

    //Send a radio subtitle change message. 
    fn send_radio_subtitle_text(&mut self, text: String, zone: u8, refresh: bool) {
        let mut ibus_handler = match self.ibus_handler.try_lock() {
            Ok(ibus_handler) => ibus_handler,
            Err(_) => {
                println!("Radio Subtitle Text: IBus handler locked.");
                return;
            }
        };

        let mut text_data = Vec::new();
        text_data.push(IBUS_CMD_GT_WRITE_WITH_CURSOR);
        text_data.push(0x62);
        text_data.push(0x1);
        text_data.push(0x40 | (zone&0xF));

        let text_bytes = text.as_bytes();
        if text_bytes.len() >= 1 {
            for i in 0..text_bytes.len() {
                text_data.push(text_bytes[i]);
            }
        } else {
            text_data.push(0x20);
        }

        let text_msg = IBusMessage {
            sender: IBUS_DEVICE_RAD,
            receiver: IBUS_DEVICE_GT,
            data: text_data,
        };

        ibus_handler.write_ibus_message(text_msg);
        std::mem::drop(ibus_handler);

        if refresh {
            self.send_refresh(0x62);
        }
    }

    //Send a radio text change message.
    fn send_radio_center_text(&mut self, text: String, position: u8, version: i8) {
        let mut ibus_handler = match self.ibus_handler.try_lock() {
            Ok(ibus_handler) => ibus_handler,
            Err(_) => {
                println!("Radio Center Text: IBus handler locked.");
                return;
            }
        };
        
        let index: u8;
        if position == SONG_NAME {
            index = 0x41;
        } else if position == ARTIST {
            index = 0x42;
        } else if position == ALBUM {
            index = 0x43;
        } else if position == APP {
            index = 0x44;
        } else {
            return;
        }

        let mut text_data = Vec::new();
        text_data.push(IBUS_CMD_GT_WRITE_WITH_CURSOR);
        
        if version >= 5 {
            text_data.push(0x63);
        } else {
            text_data.push(0x60);
        }

        text_data.push(0x1);
        text_data.push(index);

        let text_bytes = text.as_bytes();
        for i in 0..text_bytes.len() {
            text_data.push(text_bytes[i]);
        }

        let text_change_message = IBusMessage {
            sender: IBUS_DEVICE_RAD,
            receiver: IBUS_DEVICE_GT,
            data: text_data,
        };

        ibus_handler.write_ibus_message(text_change_message);
    }

    //Send multiple radio text change messages.
    fn send_all_radio_center_text(&mut self, version: i8, refresh: bool, song_title: String, artist: String, album: String, app: String) {
        self.send_radio_center_text(song_title, SONG_NAME, version);
        self.send_radio_center_text(artist, ARTIST, version);
        self.send_radio_center_text(album, ALBUM, version);
        self.send_radio_center_text(app, APP, version);

        if refresh {
            let mut index = 0x63;
            if version < 5 {
                index = 0x60;
            }
            self.send_refresh(index)
        }
    }

    //Send a refresh message.
    fn send_refresh(&mut self, index: u8) {
        let mut ibus_handler = match self.ibus_handler.try_lock() {
            Ok(ibus_handler) => ibus_handler,
            Err(_) => {
                println!("Refresh: IBus handler locked.");
                return;
            }
        };
        
        let refresh_data = [IBUS_CMD_GT_WRITE_WITH_CURSOR, index, 0x1, 0x0].to_vec();
        let refresh_msg = IBusMessage {
            sender: IBUS_DEVICE_RAD,
            receiver: IBUS_DEVICE_GT,
            data: refresh_data,
        };

        ibus_handler.write_ibus_message(refresh_msg);
    }

    //Set the state of the phone LEDs on the BMBT.
    fn set_phone_light(&mut self, state: u8) {
        let mut ibus_handler = match self.ibus_handler.try_lock() {
            Ok(ibus_handler) => ibus_handler,
            Err(_) => {
                println!("Phone Light: IBus handler locked.");
                return;
            }
        };
        
        let mut phone_data = [0x2B, 0x00];
        if state == PHONE_LED_GREEN {
            phone_data[1] = 0x10;
        } else if state == PHONE_LED_RED {
            phone_data[1] = 0x1;
        }

        ibus_handler.write_ibus_message(IBusMessage {
            sender: IBUS_DEVICE_TEL,
            receiver: IBUS_DEVICE_ANZV,
            data: phone_data.to_vec(),
        });
    }
}
