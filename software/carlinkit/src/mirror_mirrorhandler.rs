use std::sync::{Arc, Mutex};

use crate::connect_usb_dongle;
use crate::get_usb_connection;
use crate::ParameterList;
use crate::USBConnection;

pub struct MirrorHandler<'a> {
    parameter_list: &'a Arc<Mutex<ParameterList>>,
    usb_link: USBConnection<'a>,
    run: bool,
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
        }

        self.usb_link.full_loop();

        /*let mut parameters = self.parameter_list.lock().unwrap();
        if parameters.rx_cache.len() > 0 {
            for m in 0..parameters.rx_cache.len() {
                let message = &parameters.rx_cache[m];
                println!("T: {} L: {}", message.message_type, message.data.len());
                //TODO: Handle the message.
            }

            parameters.rx_cache = Vec::new();
        }*/
    }

    pub fn get_run(&mut self) -> bool {
        return self.run;
    }
}

pub fn get_mirror_handler<'a> (parameter_list: &'a Arc<Mutex<ParameterList>>) -> MirrorHandler<'a> {
    let new_usb_link = get_usb_connection(&parameter_list);

    let handler = MirrorHandler {
        parameter_list: parameter_list,
        usb_link: new_usb_link,
        run: true,
    };

    return handler;
}

