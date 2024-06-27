use std::sync::{Arc, Mutex};
use std::thread;
use std::thread::JoinHandle;

use crate::getUSBConnection;
use crate::ParameterList;
use crate::USBConnection;

pub struct MirrorHandler <'a> {
    parameter_list: &'a Arc<Mutex<ParameterList>>,
    usb_link: USBConnection<'a>,
    run: bool,

    startup_join_handle: JoinHandle<()>,
}

impl <'a> MirrorHandler<'static> {
    fn connect_dongle_thread(&mut self) {
        while !self.usb_link.running && self.run {
            // Send messages.
        }
    }
}

pub fn getMirrorHandler<'a>(parameter_list: &Arc<Mutex<ParameterList>>) -> MirrorHandler <'static> {
    let new_usb_link = getUSBConnection(parameter_list);

    let mut handler = MirrorHandler {
        parameter_list: parameter_list,
        usb_link: new_usb_link,
        run: true,

        startup_join_handle: thread::spawn(move || {

        }),
    };

	handler.startup_join_handle = thread::spawn(move || {
		//handler.connect_dongle_thread();
	});

    return handler;
}

