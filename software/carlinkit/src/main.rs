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
    let mut stream: UnixStream = init_default_socket().unwrap();

    let parameter_list: ParameterList = get_parameter_list();

    let mutex_parameter_list: Arc<Mutex<ParameterList>> = Arc::new(Mutex::new(parameter_list));
    let mut mirror_handler = get_mirror_handler(&mutex_parameter_list);

    while mirror_handler.get_run() {
        let mut socket_msg = SocketMessage {
            opcode: 0,
            data: Vec::new(),
        };
        let l = read_socket_message(&mut stream, &mut socket_msg);

        let mut new_parameter_list = mutex_parameter_list.lock().unwrap();

        if l > 0 {
            handle_socket_message(&mut new_parameter_list, socket_msg);
        }

        if new_parameter_list.ibus_waiting {
            new_parameter_list.ibus_waiting = false;

            println!("{:X?}", new_parameter_list.ibus_cache.get_bytes());
            //TODO: Interpret the IBus message.
        }

        mirror_handler.full_loop();
    }
}
