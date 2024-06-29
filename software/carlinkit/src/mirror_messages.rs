const HEADERSIZE: u8 = 4*4;
const MAGIC: u32 = 0x55aa55aa;

struct MirrorMessage {
    pub message_type: u8,
    pub data: Vec<u8>,
}
