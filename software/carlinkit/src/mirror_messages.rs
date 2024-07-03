pub const HEADERSIZE: usize = 4*4;
const MAGIC: u32 = 0x55aa55aa;

pub struct MirrorMessage {
    pub message_type: u32,
    pub data: Vec<u8>,
}

impl Clone for MirrorMessage {
    fn clone(&self) -> Self {
        let mut new_data: Vec<u8> = vec![0;self.data.len()];
        for i in 0..new_data.len() {
            new_data[i] = self.data[i];
        }

        return MirrorMessage {
            message_type: self.message_type,
            data: new_data,
        }
    }
}

impl MirrorMessage {
    //Read message header data and return whether successful.
    pub fn deserialize(&mut self, data: Vec<u8>) -> bool{
        if data.len() != HEADERSIZE {
            return false;
        }

        let mut slice:[u8;4] = [0;4];

        //Magic number check.
        for i in 0..4 {
            slice[i] = data[i];
        }

        let magic = u32::from_le_bytes(slice);
        if magic != MAGIC {
            return false;
        }

        //USB data length.
        for i in 0..4 {
            slice[i] = data[i+4];
        }

        let data_len = u32::from_le_bytes(slice);
        
        //Message type.
        for i in 0..4 {
            slice[i] = data[i+8];
        }

        let message_type = u32::from_le_bytes(slice);

        //Message type check.
        for i in 0..4 {
            slice[i] = data[i+12];
        }

        let message_type_check = u32::from_le_bytes(slice);
        if message_type_check != message_type ^ 0xFFFFFFFF {
            return false;
        }

        self.message_type = message_type;
        self.data = vec![0;data_len as usize];
        return true;
    }

    //Return all USB bytes from the message.
    pub fn serialize(self) -> Vec<u8> {
        let mut msg_data = Vec::<u8>::new();

        let magic_bytes = MAGIC.to_le_bytes();
        for i in 0..magic_bytes.len() {
            msg_data.push(magic_bytes[i]);
        }

        let len_bytes = (self.data.len() as u32).to_le_bytes();
        for i in 0..len_bytes.len() {
            msg_data.push(len_bytes[i]);
        }

        let type_bytes = self.message_type.to_le_bytes();
        for i in 0..type_bytes.len() {
            msg_data.push(type_bytes[i]);
        }

        let xor_type_bytes = (self.message_type^0xFFFFFFFF).to_le_bytes();
        for i in 0..xor_type_bytes.len() {
            msg_data.push(xor_type_bytes[i]);
        }

        for i in 0..self.data.len() {
            msg_data.push(self.data[i]);
        }

        return msg_data;
    }
}

pub fn get_new_mirror_message() -> MirrorMessage {
    return MirrorMessage {
        message_type: 0,
        data: vec![0;0],
    }
}

pub fn get_mirror_message_from_header(data: Vec<u8>) -> Option<MirrorMessage> {
    let mut mirror = get_new_mirror_message();

    if mirror.deserialize(data) {
        return Some(mirror);
    } else {
        return None;
    }
}

pub fn get_heartbeat_message() -> MirrorMessage {
    return MirrorMessage {
        message_type: 170,
        data: vec![0;0],
    };
}

pub struct SendFileMessage {
    file_name: String,
    file_data: Vec<u8>,
}

impl SendFileMessage {
    pub fn get_mirror_message(&mut self) -> MirrorMessage {
        let mut send_file_message = MirrorMessage {
            message_type: 153,
            data: vec![0;0],
        };

        let full_filename = self.file_name.as_bytes();
        let filename_len = (full_filename.len() as u32 + 1).to_le_bytes();

        for i in 0..filename_len.len() {
            send_file_message.data.push(filename_len[i]);
        }

        for i in 0..full_filename.len() {
            send_file_message.data.push(full_filename[i]);
        }

        send_file_message.data.push(0);

        let data_len = (self.file_data.len() as u32).to_le_bytes();

        for i in 0..data_len.len() {
            send_file_message.data.push(data_len[i]);
        }

        for i in 0..self.file_data.len() {
            send_file_message.data.push(self.file_data[i]);
        }

        return send_file_message;
    }
}

pub fn get_sendfile_message(filename: String, filedata: Vec<u8>) -> SendFileMessage {
    return SendFileMessage { file_name: filename, file_data: filedata };
}

pub fn get_sendstring_message(filename: String, filedata: String) -> SendFileMessage {
    return SendFileMessage { file_name: filename, file_data: filedata.as_bytes().to_vec() };
}

pub fn get_sendint_message(filename: String, filedata: u32) -> SendFileMessage {
    let mut new_int_message = SendFileMessage {
        file_name: filename,
        file_data: vec![0;0],
    };

    let new_int_bytes = filedata.to_le_bytes();
    for i in 0..new_int_bytes.len() {
        new_int_message.file_data.push(new_int_bytes[i]);
    }

    return new_int_message;
}
