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

//Create a new, blank mirror message.
pub fn get_new_mirror_message() -> MirrorMessage {
    return MirrorMessage {
        message_type: 0,
        data: vec![0;0],
    }
}

//Get a mirror message from a USB header.
pub fn get_mirror_message_from_header(data: Vec<u8>) -> Option<MirrorMessage> {
    let mut mirror = get_new_mirror_message();

    if mirror.deserialize(data) {
        return Some(mirror);
    } else {
        return None;
    }
}

//Get a heartbeat mirror message.
pub fn get_heartbeat_message() -> MirrorMessage {
    return MirrorMessage {
        message_type: 170,
        data: vec![0;0],
    };
}

//Get an Open message with parameters defined.
pub fn get_open_message(width: u32, height: u32, video_frame_rate: u32, format: u32, packet_max: u32, ibox_version: u32, phone_work_mode: u32) -> MirrorMessage {
    let mut open_message = MirrorMessage {
        message_type: 1,
        data: vec![0;0],
    };

    let width_slice = width.to_le_bytes();
    for i in 0..width_slice.len() {
        open_message.data.push(width_slice[i]);
    }

    let height_slice = height.to_le_bytes();
    for i in 0..height_slice.len() {
        open_message.data.push(height_slice[i]);
    }

    let frame_slice = video_frame_rate.to_le_bytes();
    for i in 0..frame_slice.len() {
        open_message.data.push(frame_slice[i]);
    }

    let format_slice = format.to_le_bytes();
    for i in 0..format_slice.len() {
        open_message.data.push(format_slice[i]);
    }

    let packet_slice = packet_max.to_le_bytes();
    for i in 0..packet_slice.len() {
        open_message.data.push(packet_slice[i]);
    }

    let ibox_slice = ibox_version.to_le_bytes();
    for i in 0..ibox_slice.len() {
        open_message.data.push(ibox_slice[i]);
    }

    let phonework_slice = phone_work_mode.to_le_bytes();
    for i in 0..phonework_slice.len() {
        open_message.data.push(phonework_slice[i]);
    }

    return open_message;
}

//Get a Carplay/Android Auto command message.
pub fn get_carplay_command_message(command: u32) ->MirrorMessage {
    let mut command_message = MirrorMessage {
        message_type: 8,
        data: vec![0;0],
    };

    let command_slice = command.to_le_bytes();
    for i in 0..command_slice.len() {
        command_message.data.push(command_slice[i]);
    }

    return command_message;
}

//Get a manufacturer info message.
pub fn get_manufacturer_info(mn_a: u32, mn_b: u32) -> MirrorMessage {
    let mut manufacturer_message = MirrorMessage {
        message_type: 20,
        data: vec![0;0],
    };

    let slice_a = mn_a.to_le_bytes();
    let slice_b = mn_b.to_le_bytes();

    for i in 0..slice_a.len() {
        manufacturer_message.data.push(slice_a[i]);
    }

    for i in 0..slice_b.len() {
        manufacturer_message.data.push(slice_b[i]);
    }

    return manufacturer_message;
}

pub struct SendFileMessage {
    file_name: String,
    file_data: Vec<u8>,
}

impl SendFileMessage {
    //Get the MirrorMessage version of the file message.
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

//Get a Send File message with binary content.
pub fn get_sendfile_message(filename: String, filedata: Vec<u8>) -> SendFileMessage {
    return SendFileMessage { file_name: filename, file_data: filedata };
}

//Get a Send File message with string content.
pub fn get_sendstring_message(filename: String, filedata: String) -> SendFileMessage {
    return SendFileMessage { file_name: filename, file_data: filedata.as_bytes().to_vec() };
}

//Get a Send File message with integer content.
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
