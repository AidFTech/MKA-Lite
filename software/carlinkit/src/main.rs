mod ipc;
mod ibus;
mod context;
mod mirror;

use std::sync::{Arc, Mutex};

use ipc::*;
use ibus::*;
use context::Context;
use mirror::handler::MirrorHandler;
use mirror::usb::USBConnection;

fn main() {
    let mutex_stream = Arc::new(Mutex::new(init_default_socket().unwrap()));
    let context: Context = Context::new();
    let mutex_context: Arc<Mutex<Context>> = Arc::new(Mutex::new(context));
    let mut mirror_handler = MirrorHandler::new(&mutex_context, &mutex_stream, 800, 480);

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

        let mut l = 0;
        match mutex_stream.try_lock() {
            Ok(mut stream) => {
                l = read_socket_message(&mut stream, &mut socket_msg);
            }
            Err(_) => {

             }
        }

        if l > 0 {
            handle_socket_message(&mut new_context, socket_msg);
        }

        let ibus_waiting = new_context.ibus_waiting;
        let ibus_msg = new_context.ibus_cache.clone();

        if ibus_waiting {
            new_context.ibus_waiting = false;
        }

        std::mem::drop(new_context);

        if ibus_waiting {
            println!("{:X?}", ibus_msg.get_bytes());
            mirror_handler.handle_ibus_message(ibus_msg);
        }

        // TODO: Return a Result() and act on errors (like run being false)
        mirror_handler.process();
    }
}
