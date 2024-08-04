use std::os::unix::net::UnixStream;
use std::sync::{Arc, Mutex};
use std::time::SystemTime;

use crate::{Context, IBusMessage, SocketMessage};
use crate::USBConnection;

use crate::ipc::*;

use super::messages::get_carplay_command_message;
use super::messages::get_manufacturer_info;
use super::messages::get_open_message;
use super::messages::get_sendint_message;
use super::messages::get_sendstring_message;
use super::messages::get_heartbeat_message;
use super::messages::MirrorMessage;
use super::messages::MetaDataMessage;
use super::mpv::{get_decode_type, MpvVideo};
use super::mpv::RdAudio;

pub struct MirrorHandler<'a> {
    context: &'a Arc<Mutex<Context>>,
    usb_conn: USBConnection,
    run: bool,
    startup: bool,
    stream: &'a Arc<Mutex<UnixStream>>,
    mpv_video: MpvVideo,
    rd_audio: RdAudio,
    nav_audio: RdAudio,
    heartbeat_time: SystemTime,
}

impl<'a> MirrorHandler<'a> {
    pub fn new(context: &'a Arc<Mutex<Context>>, stream: &'a Arc<Mutex<UnixStream>>) -> MirrorHandler <'a> {
        let mut mpv_found = 0;
        let mut mpv_video: Option<MpvVideo> = None;
        let mut rd_audio: Option<RdAudio> = None;
        let mut nav_audio: Option<RdAudio> = None;

        while mpv_found < 3 {
            match MpvVideo::new(720, 480) {
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
            
            //TODO: Different hardware output?
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
            stream,
            mpv_video: mpv_video.unwrap(),
            rd_audio: rd_audio.unwrap(),
            nav_audio: nav_audio.unwrap(),
            heartbeat_time: SystemTime::now()
        };
    }

    pub fn process(&mut self) {
        if !self.run {
            return;
        }
        if !self.usb_conn.connected {
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
        let context = match self.context.try_lock() {
            Ok(context) => context,
            Err(_) => {
                println!("IBus: Context locked.");
                return;
            }
        };

        if ibus_msg.sender == 0xF0 { //From BMBT.
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
                }
            }
        } else if ibus_msg.l() >= 3 && ibus_msg.data[0] == 0x48 && ibus_msg.data[1] == 0x8E {
            //TODO: This is an AIBus command for testing. We need to figure out the equivalent MKA command.
            if ibus_msg.data[2] == 0x0 {
                self.mpv_video.set_minimize(true);
            } else {
                self.mpv_video.set_minimize(false);
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
        //TODO: Send icon messages.
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
            //Phone connected.
            let data = message.clone().decode();
            if data.len() <= 0 {
                return;
            }
            println!("Phone Connected!");
            let phone_type = data[0];
            match self.context.try_lock() {
                Ok(mut context) => {
                    context.phone_type = phone_type as u8;
                }
                Err(_) => {
                }
            }

            let mut socket_message = SocketMessage {
                opcode: OPCODE_PHONE_TYPE,
                data: vec![0;0],
            };
            socket_message.data.push(phone_type as u8);

            match self.stream.try_lock() {
                Ok(mut stream) => {
                    write_socket_message(&mut stream, socket_message);
                }
                Err(_) => {

                }
            }
            
            self.mpv_video.start();

            //TODO: Start the decoders.
        } else if message.message_type == 4 {
            // Phone disconnected.
            let mut socket_message = SocketMessage {
                opcode: OPCODE_PHONE_TYPE,
                data: vec![0;0],
            };
            socket_message.data.push(0);

            match self.stream.try_lock() {
                Ok(mut stream) => {
                    write_socket_message(&mut stream, socket_message);
                }
                Err(_) => {

                }
            }
            self.mpv_video.stop();

            //TODO: Stop the decoders.
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

        for string_var in meta_message.string_vars {
            if string_var.variable == "MDModel" {
                context.phone_name = string_var.value;

                match self.stream.try_lock() {
                    Ok(mut stream) => {
                        let socket_message = SocketMessage{opcode: OPCODE_PHONE_NAME, data: context.phone_name.as_bytes().to_vec()};
                        write_socket_message(&mut stream, socket_message);
                    }
                    Err(_) => {

                    }
                }
            }
        }
    }
}
