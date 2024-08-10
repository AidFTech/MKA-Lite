mod ibus;
mod context;
mod mirror;

use std::env;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use ibus::*;
use context::Context;
use mirror::handler::MirrorHandler;
use mirror::usb::USBConnection;

fn main() {
    let args = env::args();
    if args.len() < 2 {
        println!("A serial port argument is required.");
        return;
    }

    let mut ibus_handler = None;
    for arg in args {
        if arg.contains("./carlinkit") { //TODO: Is there another way to check? The ./carlinkit is counted as an argument.
            continue;
        }

        ibus_handler = IBusHandler::new(arg.clone());

        match ibus_handler {
            Some(ref _ibus_handler) => {
                println!("Successfully opened {}!", arg.clone());
                break;
            }
            None => {
                println!("Could not connect to {}.", arg);
                //Continue the loop.
            }
        }
    }

    match ibus_handler {
        Some(ref _ibus_handler) => {
            //Continue the program.
        }
        None => {
            println!("Could not open the IBus handler.");
            return;
        }
    }

    let mut ping_timer = Instant::now();

    let context: Context = Context::new();
    let mutex_context: Arc<Mutex<Context>> = Arc::new(Mutex::new(context));
    let mut mirror_handler = MirrorHandler::new(&mutex_context, ibus_handler.unwrap(), 800, 480);

    mirror_handler.send_cd_ping();

    loop {
        mirror_handler.check_ibus();

        // TODO: Return a Result() and act on errors (like run being false)
        mirror_handler.process();

        if Instant::now() - ping_timer >= Duration::from_millis(20000) {
            ping_timer = Instant::now();
            mirror_handler.send_cd_ping();
        }
    }
}
