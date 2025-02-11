mod ibus;
mod mka;
mod context;
mod mirror;
mod aap;

use std::{env, thread};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use ibus::*;
use mka::MKAObj;
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
    let mutex_ibus_handler = Arc::new(Mutex::new(ibus_handler.unwrap()));
    let mutex_mirror_handler = Arc::new(Mutex::new(MirrorHandler::new(&mutex_context, &mutex_ibus_handler, 800, 480)));

    let thread_ibus_handler = Arc::clone(&mutex_ibus_handler);

    let mut mka_obj = MKAObj::new(&mutex_context, &mutex_ibus_handler, &mutex_mirror_handler);

    mka_obj.send_cd_ping();

    let _ibus_handle = thread::spawn( move || {
        loop {
            let mut ibus_handler = match thread_ibus_handler.try_lock() {
                Ok(ibus_handler) => ibus_handler,
                Err(_) => {
                    continue;
                }
            };

            if ibus_handler.bytes_available() >= 4 {
                let ib_msg = match ibus_handler.read_ibus_message() {
                    None => {
                        continue;
                    }
                    Some(ib_msg) => ib_msg,
                };
                ibus_handler.cache_message(ib_msg);
            }
        }
    });

    loop {
        mka_obj.check_ibus();

        // TODO: Return a Result() and act on errors (like run being false)
        match mutex_mirror_handler.try_lock() {
            Ok(mut mirror_handler) => {
                mirror_handler.process();
            }
            Err(_) => ()
        }

        if Instant::now() - ping_timer >= Duration::from_millis(20000) {
            ping_timer = Instant::now();
            mka_obj.send_cd_ping();
        }
    }
}