mod ipc;
mod ibus;
mod context;
mod mirror_usblink;
mod mirror_mirrorhandler;
mod mirror_messages;

use std::os::unix::net::UnixStream;

use ipc::*;
use ibus::*;
use context::Context;
use mirror_usblink::*;
use mirror_mirrorhandler::*;
use mirror_messages::*;

use std::sync::Arc;
use std::sync::Mutex;

fn main() {
    let mut stream: UnixStream = init_default_socket().unwrap();
    let context: Context = Context::new();
    let mutex_context: Arc<Mutex<Context>> = Arc::new(Mutex::new(context));
    let mut usb_conn = USBConnection::new(&mutex_context);
    let mut mirror_handler = MirrorHandler::new(&mutex_context, &mut usb_conn);

    loop {
		let mut new_context = match mutex_context.try_lock() {
			Ok(new_context) => new_context,
			Err(_) => {
				println!("Main: Parameter list is locked.");
				continue;
			}
		};

        let mut socket_msg = SocketMessage {
            opcode: 0,
            data: Vec::new(),
        };
        let l = read_socket_message(&mut stream, &mut socket_msg);

        if l > 0 {
            handle_socket_message(&mut new_context, socket_msg);
        }

        if new_context.ibus_waiting {
            new_context.ibus_waiting = false;
            println!("{:X?}", new_context.ibus_cache.get_bytes());
            //TODO: Interpret the IBus message.
        }
        std::mem::drop(new_context);
        mirror_handler.process();
    }
}
