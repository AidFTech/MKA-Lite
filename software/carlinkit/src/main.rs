mod ipc;
mod ibus;
mod parameter_list;
mod Mirror_USBLink;
mod mirror_mirrorhandler;

use std::os::unix::net::UnixStream;

use ipc::*;
use ibus::*;
use parameter_list::*;
use Mirror_USBLink::*;
use mirror_mirrorhandler::*;

use std::sync::Arc;
use std::sync::Mutex;

fn main() {
    /*let mut stream: UnixStream;
    match ipc::initSocket() {
        Ok(ret_stream) => {
            stream = ret_stream;
        }
        Err(_err) => {
            return;
        }
    }*/

    let mut parameter_list: ParameterList = getParameterList();

    /*loop {
        let mut socket_msg = SocketMessage {
            opcode: 0,
            data: Vec::new(),
        };
        let l = readSocketMessage(&mut stream, &mut socket_msg);

        if l > 0 {
            handleSocketMessage(&mut parameter_list, socket_msg);
        }

        if parameter_list.ibus_waiting {
            parameter_list.ibus_waiting = false;

            println!("{:X?}", parameter_list.ibus_cache.getBytes());
            //TODO: Interpret the IBus message.
        }
    }*/
    let mutex_parameter_list: Arc<Mutex<ParameterList>> = Arc::new(Mutex::new(parameter_list));
    let mut mirror_link = getUSBConnection(&mutex_parameter_list);
    mirror_link.readThread();
}
