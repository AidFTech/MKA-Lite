// Socket implementation
use std::io::prelude::*;
use std::io::Error;
use std::os::unix::net::UnixStream;
use std::str;

use crate::getIBusMessage;
use crate::IBusMessage;
use crate::ParameterList;

const SOCKET_PATH: &str = "/run/mka_to_backend.sock";
const SOCKET_START: &str = "MKASock";

const OPCODE_PHONE_ACTIVE: u8 = 0x21;
const OPCODE_MKA_ACTIVE: u8 = 0x22;
const OPCODE_AUDIO_SELECTED: u8 = 0x23;
const OPCODE_PHONE_TYPE: u8 = 0x2B;
const OPCODE_PHONE_NAME: u8 = 0x2C;
const OPCODE_PLAYING: u8 = 0x39;
const OPCODE_BMBT_CONNECTED: u8 = 0xF0;

const OPCODE_IBUS_SEND: u8 = 0x18;
const OPCODE_IBUS_RECV: u8 = 0x68;

pub struct SocketMessage {
    pub opcode: u8,
    pub data: Vec<u8>,
}

//Get a UnixStream object.
pub fn initSocket() -> Result<UnixStream, Error> {
    let stream = UnixStream::connect(SOCKET_PATH);
    return stream;
}

//Read bytes from a socket.
fn readSocketBytes(stream: &mut UnixStream, data: &mut [u8]) -> usize {
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

//Write bytes to a socket.
fn writeSocketBytes(stream: &mut UnixStream, data: &mut Vec<u8>) -> usize {

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

//Read a full message from the socket.
pub fn readSocketMessage(stream: &mut UnixStream, message: &mut SocketMessage) -> usize {
    let mut data : [u8; 1024] = [0; 1024];
    let full_l = readSocketBytes(stream, &mut data);
    
    if full_l < SOCKET_START.len() {
        return 0;
    }

    let socket_start_msg = SOCKET_START.as_bytes();
    for i in 0..socket_start_msg.len() {
        if data[i] != socket_start_msg[i] {
            return 0;
        }
    }
    
    message.opcode = data[socket_start_msg.len()];
    let data_l: u8 = data[socket_start_msg.len() + 1]-1;
    let start = socket_start_msg.len() + 2;

    let mut checksum = 0;
    for i in 0..full_l - 1 {
        checksum ^= data[i];
    }

    if checksum != data[full_l-1] {
        return 0;
    }
    
    message.data = Vec::new();
    
    for i in start..full_l - 1 {
        message.data.push(data[i]);
    }

    return data_l as usize;
}

//Write a full message to the socket.
pub fn writeSocketMessage(stream: &mut UnixStream, message: SocketMessage) {
    let socket_start_msg = SOCKET_START.as_bytes();
    let mut data: Vec<u8> = vec![0; message.data.len() + socket_start_msg.len() + 3];

    for i in 0..socket_start_msg.len() {
        data[i] = socket_start_msg[i];
    }

    data[socket_start_msg.len()] = message.opcode;
    data[socket_start_msg.len() + 1] = (message.data.len() + 1) as u8;

    for i in 0..message.data.len() {
        data[i + socket_start_msg.len() + 2] = message.data[i];
    }
    
    let mut checksum: u8 = 0;
    for i in 0..data.len() - 1 {
        checksum ^= data[i];
    }
    
    let checksum_index = data.len() - 1;
    data[checksum_index] = checksum;

    let _ = writeSocketBytes(stream, &mut data);
}

pub fn writeIBusMessage(stream: &mut UnixStream, message: IBusMessage) {
    let bytes = message.getBytes();

    let mut socket_msg = SocketMessage {
        opcode: 0x68,
        data: vec![0;bytes.len()+3],
    };

    for i in 0..bytes.len() {
        socket_msg.data[i] = bytes[i];
    }

    writeSocketMessage(stream, socket_msg);
}

pub fn handleSocketMessage(parameter_list: &mut ParameterList, message: SocketMessage) {
    let opcode = message.opcode;
    let socket_bool: bool = message.data.len() > 0 && message.data[0] != 0;
    if opcode == OPCODE_PHONE_ACTIVE {
        parameter_list.phone_active = socket_bool;
    } else if opcode == OPCODE_MKA_ACTIVE {
        parameter_list.mka_active = socket_bool;
    } else if opcode == OPCODE_AUDIO_SELECTED {
        parameter_list.audio_selected = socket_bool;
    } else if opcode == OPCODE_PHONE_TYPE && message.data.len() >= 1 {
        parameter_list.phone_type = message.data[0];
    } else if opcode == OPCODE_PHONE_NAME {
        parameter_list.phone_name = String::from(str::from_utf8(&message.data).unwrap())
    } else if opcode == OPCODE_PLAYING {
        parameter_list.playing = socket_bool;
    } else if opcode == OPCODE_BMBT_CONNECTED {
        parameter_list.bmbt_connected = socket_bool;
    } else if opcode == OPCODE_IBUS_RECV {
        let ib_msg = getIBusMessage(message.data);

        if ib_msg.l() > 0 {
            parameter_list.ibus_cache = ib_msg;
            parameter_list.ibus_waiting = true;
        }
    }
}