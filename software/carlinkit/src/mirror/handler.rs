use std::sync::{Arc, Mutex};

use crate::Context;
use crate::USBConnection;

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

        }
        self.usb_conn.full_loop();

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
}
