use std::sync::{Arc, Mutex};

use crate::connect_usb_dongle;
use crate::get_usb_connection;
use crate::Context;
use crate::USBConnection;

pub struct MirrorHandler<'a> {
    context: &'a Arc<Mutex<Context>>,
    usb_link: USBConnection<'a>,
    run: bool,
    startup: bool,
}

impl<'a> MirrorHandler<'a> {
    pub fn full_loop(&mut self) {
        if !self.usb_link.get_running() {
            let run = connect_usb_dongle(&mut self.usb_link);

            if !run {
                return; //TODO: Should we still run full_loop even if no dongle is connected?
            } else {
                self.run = true;
            }
        } else if !self.startup {

        }

        self.usb_link.full_loop();

        match self.context.try_lock() {
            Ok(mut ctx) => {
                if ctx.rx_cache.len() > 0 {
                    for m in 0..ctx.rx_cache.len() {
                        let message = &ctx.rx_cache[m];
                        println!("T: {} L: {}", message.message_type, message.data.len());
                        //TODO: Handle the message.
                    }
                    ctx.rx_cache = Vec::new();
                }
            }
            Err(_) => {
                println!("Mirror: Parameter list is locked.");
            }
        };

    }

    pub fn get_run(&mut self) -> bool {
        return self.run;
    }
}

pub fn get_mirror_handler<'a> (context: &'a Arc<Mutex<Context>>) -> MirrorHandler<'a> {
    let new_usb_link = get_usb_connection(&context);
    return MirrorHandler {
        context,
        usb_link: new_usb_link,
        run: true,
        startup: false,
    };
}

