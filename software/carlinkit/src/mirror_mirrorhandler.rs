use std::thread;
use std::thread::JoinHandle;

use crate::getUSBConnection;
use crate::ParameterList;
use crate::USBConnection;

pub struct MirrorHandler {
    parameter_list: *mut ParameterList,
    usb_link: USBConnection,
    run: bool,

    startup_join_handle: JoinHandle<()>,
}

pub fn getMirrorHandler(parameter_list: *mut ParameterList) -> MirrorHandler {
    let new_usb_link = getUSBConnection(parameter_list);

    let mut the_return = MirrorHandler {
        parameter_list: parameter_list,
        usb_link: new_usb_link,
        run: true,

        startup_join_handle: thread::spawn(move || {
            
        }),
    };

	the_return.startup_join_handle = thread::spawn(move || {
		the_return.connectDongleThread();
	});

    return the_return;
}

impl MirrorHandler {
    fn connectDongleThread(&mut self) {
        while !self.usb_link.running && self.run {
            //Send messages.
        }
    }
}
