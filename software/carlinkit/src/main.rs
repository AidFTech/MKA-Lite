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

use std::thread;

fn main() {
    let mut stream: UnixStream;
    match ipc::initSocket() {
        Ok(ret_stream) => {
            let _ = ret_stream.set_nonblocking(true);
            stream = ret_stream;
        }
        Err(_err) => {
            return;
        }
    }

    let mut parameter_list: ParameterList = getParameterList();

    let mutex_parameter_list: Arc<Mutex<ParameterList>> = Arc::new(Mutex::new(parameter_list));
    let mut mirror_handler = getMirrorHandler(&mutex_parameter_list);

    //let socket_thread = thread::spawn(move || {
        while mirror_handler.getRun() {
            let mut socket_msg = SocketMessage {
                opcode: 0,
                data: Vec::new(),
            };
            let l = readSocketMessage(&mut stream, &mut socket_msg);

            let mut new_parameter_list = mutex_parameter_list.lock().unwrap();

            if l > 0 {
                handleSocketMessage(&mut new_parameter_list, socket_msg);
            }

            if new_parameter_list.ibus_waiting {
                new_parameter_list.ibus_waiting = false;

                println!("{:X?}", new_parameter_list.ibus_cache.getBytes());
                //TODO: Interpret the IBus message.
            }
        }
    //});

    //socket_thread.join().unwrap();
}
