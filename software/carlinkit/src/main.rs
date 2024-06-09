mod ipc;

use std::os::unix::net::UnixStream;

use ipc::*;

fn main() {
    let mut stream: UnixStream;
    match ipc::initSocket() {
        Ok(ret_stream) => {
            stream = ret_stream;
        }
        Err(_err) => {
            return;
        }
    }

    let send_msg = SocketMessage{
        opcode: 0x68,
        data: vec![0x68, 0x3, 0x18, 0x1, 0x87],
    };

    writeSocketMessage(&mut stream, send_msg);

    loop {
        let mut socket_msg = SocketMessage {
            opcode: 0,
            data: Vec::new(),
        };
        let l = readSocketMessage(&mut stream, &mut socket_msg);

        if l > 0 {
            println!("{:x?}", &socket_msg.data[0..l]);
        }
    }
}
