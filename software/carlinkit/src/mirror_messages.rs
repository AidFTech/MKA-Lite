pub const HEADERSIZE: usize = 4*4;
const MAGIC: u32 = 0x55aa55aa;

pub struct MirrorMessage {
    pub message_type: u32,
    pub data: Vec<u8>,
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
}

pub fn get_new_mirror_message() -> MirrorMessage {
    return MirrorMessage {
        message_type: 0,
        data: vec![0;0],
    }
}

pub fn get_heartbeat_message() -> MirrorMessage {
    return MirrorMessage {
        message_type: 170,
        data: vec![0;0],
    };
}