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
    let mut usb_conn = USBConnection::new(&mutex_context);
    let mut mirror_handler = MirrorHandler::new(&mutex_context, &mut usb_conn, &mutex_stream);

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

        if new_context.ibus_waiting {
            new_context.ibus_waiting = false;
            println!("{:X?}", new_context.ibus_cache.get_bytes());
            
            let ibus_msg = &new_context.ibus_cache;
            if ibus_msg.sender == 0xF0 { //From BMBT.
                if ibus_msg.l() >= 2 && ibus_msg.data[0] == 0x49 {
                    let clockwise = ibus_msg.data[1]&0x8 != 0;
                    let steps = ibus_msg.data[1]&0x7F;

                    let mut cmd: u32 = 100;
                    if clockwise {
                        cmd = 101;
                    }

                    for i in 0..steps {
                        mirror_handler.send_carplay_command(cmd);
                    }
                    println!("Turned!");
                }
            }
        }
        std::mem::drop(new_context);
        // TODO: Return a Result() and act on errors (like run being false)
        mirror_handler.process();
    }
}
