// Socket implementation
use std::io::prelude::*;
use std::os::unix::net::UnixStream;
use std::ptr::null;

const SOCKET_PATH: &str = "/run/mka_to_backend.sock";

fn initSocket() -> Option<UnixStream> {
    let stream = UnixStream::connect(SOCKET_PATH);
    match stream {
        Ok(stream) => {
            return Some(stream);
        }
        Err(_err) => {
            return None;
        }
    }
}

fn readSocketMessage(stream: &mut UnixStream, data: &mut [u8]) -> usize {
    let l = stream.read(data);
    match l {
        Ok(l) => {
            return l;
        }
        Err(_err) => {
            return 0;
        }
    }
}

fn writeSocketMessage(stream: &mut UnixStream, data: &mut [u8]) -> usize {
    let bytes_written = stream.write(data);
    match bytes_written {
        Ok(bytes_written) => {
            return bytes_written;
        }
        Err(_err) => {
            return 0;
        }
    }
}
