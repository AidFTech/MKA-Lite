use std::sync::{Arc, Mutex};

use crate::getUSBConnection;
use crate::ParameterList;
use crate::USBConnection;

pub struct MirrorHandler<'a> {
    parameter_list: &'a Arc<Mutex<ParameterList>>,
    usb_link: USBConnection<'a>,
    run: bool,
}

impl<'a> MirrorHandler<'a> {
    pub fn connect_dongle(&mut self) {
        while !self.usb_link.running && self.run {
            // Send messages.
        }
    }

    pub fn getRun(&mut self) -> bool {
        return self.run;
    }
}

pub fn getMirrorHandler<'a> (parameter_list: &'a Arc<Mutex<ParameterList>>) -> MirrorHandler<'a> {
    let new_usb_link = getUSBConnection(&parameter_list);

    let mut handler = MirrorHandler {
        parameter_list: parameter_list,
        usb_link: new_usb_link,
        run: true,
    };

    return handler;
}

