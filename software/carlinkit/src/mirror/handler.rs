use std::os::unix::net::UnixStream;
use std::sync::{Arc, Mutex};

use crate::{Context, SocketMessage};
use crate::USBConnection;

use crate::ipc::*;

use super::messages::get_carplay_command_message;
use super::messages::get_manufacturer_info;
use super::messages::get_open_message;
use super::messages::get_sendint_message;
use super::messages::get_sendstring_message;
use super::messages::MirrorMessage;
use super::messages::MetaDataMessage;

pub struct MirrorHandler<'a> {
    context: &'a Arc<Mutex<Context>>,
    usb_conn: &'a mut USBConnection<'a>,
    stream: &'a Arc<Mutex<UnixStream>>,
    run: bool,
    startup: bool,
}

impl<'a> MirrorHandler<'a> {
    pub fn new(context: &'a Arc<Mutex<Context>>, usb_conn: &'a mut USBConnection <'a>, stream: &'a Arc<Mutex<UnixStream>>) -> MirrorHandler <'a> {
        return MirrorHandler {
            context,
            usb_conn,
            stream,
            run: true,
            startup: false,
        };
    }

    pub fn process(&mut self) {
        if !self.run {
            return;
        }
        if !self.usb_conn.get_running() {
            self.startup = false;
            let run = self.usb_conn.connect_dongle();

            if !run {
                return; //TODO: Should we still run full_loop even if no dongle is connected?
            } else {
                self.run = true;
            }
        } else if !self.startup {
            self.send_dongle_startup();
        }
        self.usb_conn.full_loop(self.startup);

        let mut rx_cache: Vec<MirrorMessage> = Vec::new();

        match self.context.try_lock() {
            Ok(mut ctx) => {
                if ctx.rx_cache.len() > 0 {
                    for m in 0..ctx.rx_cache.len() {
                        rx_cache.push(ctx.rx_cache[m].clone());
                    }
                    ctx.rx_cache = Vec::new();
                }
            }
            Err(_) => {
                println!("Mirror: Parameter list is locked.");
            }
        };

        for message in rx_cache {
            self.interpret_message(&message);
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
        if message.message_type == 0x1 { //Open message.
            self.startup = true;

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
            msg_91.data.push(1);
            self.usb_conn.write_message(msg_91);

            let mut msg_88 = MirrorMessage::new(0x88);
            msg_88.data.push(1);
            self.usb_conn.write_message(msg_88);

            self.usb_conn.reset_heartbeat();
        } else if message.message_type == 2 { //Plugged message.
            //Phone connected.
            let data = message.clone().decode();
            if data.len() <= 0 {
                return;
            }

            let phone_type = data[0];
            match self.context.try_lock() {
                Ok(mut context) => {
                    context.phone_type = phone_type as u8;
                }
                Err(_) => {
                    return;
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
            //TODO: Start the decoders.
        } else if message.message_type == 4 { //Unplugged message.
            //Phone disconnected.
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
            //TODO: Stop the decoders.
        } else if message.message_type == 25 || message.message_type == 42 { //Metadata message.
            //Handle metadata.
            let meta_message = MetaDataMessage::from(message.clone());

            /*match String::from_utf8(meta_message.get_mirror_message().data) {
                Ok(meta) => {
                    println!("{}", meta);
                }
                Err(_) => {
    
                }
            };*/

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
