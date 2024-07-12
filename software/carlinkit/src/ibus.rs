//IBus implementation
pub struct IBusMessage {
    pub sender: u8,
    pub receiver: u8,
    pub data: Vec<u8>
}

impl Clone for IBusMessage {
    fn clone(&self) -> Self {
        let mut new_data: Vec<u8> = vec![0;self.data.len()];
        for i in 0..self.data.len() {
            new_data[i] = self.data[i];
        }

        return IBusMessage {
            sender: self.sender,
            receiver: self.receiver,
            data: new_data,
        }
    }
}

impl IBusMessage {
    pub fn l(&self) -> usize {
        return self.data.len();
    }
    
    pub fn get_bytes(&self) -> Vec<u8> {
        let mut data: Vec<u8> = vec![0; self.data.len() + 4];
        
        data[0] = self.sender;
        data[1] = (self.data.len() + 2) as u8;
        data[2] = self.receiver;
        
        for i in 0..self.data.len() {
            data[i+3] = self.data[i];
        }
        
        let mut checksum: u8 = 0;
        
        for i in 0..data.len() - 1 {
            checksum ^= data[i];
        }
        
        let checksum_index = data.len() - 1;
        data[checksum_index] = checksum;
        
        return data;
    }
}

pub fn get_ibus_message(data: Vec<u8>) -> IBusMessage {
    let mut checksum = 0;
    for i in 0..data.len() - 1 {
        checksum ^= data[i];
    }

    if checksum != data[data.len()-1] {
        return IBusMessage {
            sender: 0,
            receiver: 0,
            data: vec![0;0],
        };
    }
    
    if data.len() < 4 {
        return IBusMessage {
            sender: 0,
            receiver: 0,
            data: vec![0;0],
        };
    }

    if data[1] != (data.len() - 2) as u8 {
        return IBusMessage {
            sender: 0,
            receiver: 0,
            data: vec![0;0],
        };
    }

    let mut the_return = IBusMessage {
        sender: data[0],
        receiver: data[2],
        data: vec![0;data.len() - 4],
    };

    for i in 0..the_return.l() {
        the_return.data[i] = data[i+3];
    }

    return the_return;
}