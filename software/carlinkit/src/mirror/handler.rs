use std::sync::{Arc, Mutex};

use crate::Context;
use crate::USBConnection;

use super::messages::get_carplay_command_message;
use super::messages::get_manufacturer_info;
use super::messages::get_open_message;
use super::messages::get_sendint_message;
use super::messages::get_sendstring_message;
use super::messages::MirrorMessage;

pub struct MirrorHandler<'a> {
    context: &'a Arc<Mutex<Context>>,
    usb_conn: &'a mut USBConnection<'a>,
    run: bool,
    startup: bool,
}

impl<'a> MirrorHandler<'a> {
    pub fn new(context: &'a Arc<Mutex<Context>>, usb_conn: &'a mut USBConnection <'a>) -> MirrorHandler <'a> {
        return MirrorHandler {
            context,
            usb_conn,
            run: true,
            startup: false,
        };
    }

    pub fn process(&mut self) {
        if !self.run {
            return;
        }
        if !self.usb_conn.get_running() {
            let run = self.usb_conn.connect_dongle();

            if !run {
                return; //TODO: Should we still run full_loop even if no dongle is connected?
            } else {
                self.run = true;
            }
        } else if !self.startup {
            self.send_dongle_startup();
        }
        self.usb_conn.full_loop();

        match self.context.try_lock() {
            Ok(mut ctx) => {
                if ctx.rx_cache.len() > 0 {
                    for m in 0..ctx.rx_cache.len() {
                        let message = &ctx.rx_cache[m];
                        println!("L: {} T: {}", message.data.len(), message.message_type);
                        self.interpret_message(message);
                    }
                    ctx.rx_cache = Vec::new();
                }
            }
            Err(_) => {
                println!("Mirror: Parameter list is locked.");
            }
        };
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
        }
    }
}
