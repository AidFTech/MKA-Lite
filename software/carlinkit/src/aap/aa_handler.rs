use std::sync::Arc;
use std::sync::Mutex;
use std::time::Duration;
use std::ffi::c_void;
use std::time::Instant;

use protobuf::CodedOutputStream;
use protobuf::Message;
use openssl_sys::*;
use protobuf::MessageField;

use crate::mirror::mpv::MpvVideo;
use crate::mirror::mpv::RdAudio;
use crate::Context;

use crate::aap::aap_services::ServiceChannels;
use crate::aap::protos::protos::key_event::Key;

use super::aap_usb::AndroidUSBConnection;
use super::media_messages::*;

use super::protos::protos::*;

const AAP_FRAME_FIRST_FRAME: u8 = 1;
const AAP_FRAME_LAST_FRAME: u8 = 2;
const AAP_FRAME_CONTROL_MESSAGE: u8 = 4;
const AAP_FRAME_ENCRYPTED: u8 = 8;

const MAX_FRAME_PAYLOAD_SIZE: usize = 0x4000;

const CERT_BUF: &[u8] = b"-----BEGIN CERTIFICATE-----\n\
MIIDKjCCAhICARswDQYJKoZIhvcNAQELBQAwWzELMAkGA1UEBhMCVVMxEzARBgNV\n\
BAgMCkNhbGlmb3JuaWExFjAUBgNVBAcMDU1vdW50YWluIFZpZXcxHzAdBgNVBAoM\n\
Fkdvb2dsZSBBdXRvbW90aXZlIExpbmswJhcRMTQwNzA0MDAwMDAwLTA3MDAXETQ1\n\
MDQyOTE0MjgzOC0wNzAwMFMxCzAJBgNVBAYTAkpQMQ4wDAYDVQQIDAVUb2t5bzER\n\
MA8GA1UEBwwISGFjaGlvamkxFDASBgNVBAoMC0pWQyBLZW53b29kMQswCQYDVQQL\n\
DAIwMTCCASIwDQYJKoZIhvcNAQEBBQADggEPADCCAQoCggEBAM911mNnUfx+WJtx\n\
uk06GO7kXRW/gXUVNQBkbAFZmVdVNvLoEQNthi2X8WCOwX6n6oMPxU2MGJnvicP3\n\
6kBqfHhfQ2Fvqlf7YjjhgBHh0lqKShVPxIvdatBjVQ76aym5H3GpkigLGkmeyiVo\n\
VO8oc3cJ1bO96wFRmk7kJbYcEjQyakODPDu4QgWUTwp1Z8Dn41ARMG5OFh6otITL\n\
XBzj9REkUPkxfS03dBXGr5/LIqvSsnxib1hJ47xnYJXROUsBy3e6T+fYZEEzZa7y\n\
7tFioHIQ8G/TziPmvFzmQpaWMGiYfoIgX8WoR3GD1diYW+wBaZTW+4SFUZJmRKgq\n\
TbMNFkMCAwEAATANBgkqhkiG9w0BAQsFAAOCAQEAsGdH5VFn78WsBElMXaMziqFC\n\
zmilkvr85/QpGCIztI0FdF6xyMBJk/gYs2thwvF+tCCpXoO8mjgJuvJZlwr6fHzK\n\
Ox5hNUb06AeMtsUzUfFjSZXKrSR+XmclVd+Z6/ie33VhGePOPTKYmJ/PPfTT9wvT\n\
93qswcxhA+oX5yqLbU3uDPF1ZnJaEeD/YN45K/4eEA4/0SDXaWW14OScdS2LV0Bc\n\
YmsbkPVNYZn37FlY7e2Z4FUphh0A7yME2Eh/e57QxWrJ1wubdzGnX8mrABc67ADU\n\
U5r9tlTRqMs7FGOk6QS2Cxp4pqeVQsrPts4OEwyPUyb3LfFNo3+sP111D9zEow==\n\
-----END CERTIFICATE-----\n";

const PRIV_BUF: &[u8] = b"-----BEGIN RSA PRIVATE KEY-----\n\
MIIEowIBAAKCAQEAz3XWY2dR/H5Ym3G6TToY7uRdFb+BdRU1AGRsAVmZV1U28ugR\n\
A22GLZfxYI7Bfqfqgw/FTYwYme+Jw/fqQGp8eF9DYW+qV/tiOOGAEeHSWopKFU/E\n\
i91q0GNVDvprKbkfcamSKAsaSZ7KJWhU7yhzdwnVs73rAVGaTuQlthwSNDJqQ4M8\n\
O7hCBZRPCnVnwOfjUBEwbk4WHqi0hMtcHOP1ESRQ+TF9LTd0Fcavn8siq9KyfGJv\n\
WEnjvGdgldE5SwHLd7pP59hkQTNlrvLu0WKgchDwb9POI+a8XOZClpYwaJh+giBf\n\
xahHcYPV2Jhb7AFplNb7hIVRkmZEqCpNsw0WQwIDAQABAoIBAB2u7ZLheKCY71Km\n\
bhKYqnKb6BmxgfNfqmq4858p07/kKG2O+Mg1xooFgHrhUhwuKGbCPee/kNGNrXeF\n\
pFW9JrwOXVS2pnfaNw6ObUWhuvhLaxgrhqLAdoUEgWoYOHcKzs3zhj8Gf6di+edq\n\
SyTA8+xnUtVZ6iMRKvP4vtCUqaIgBnXdmQbGINP+/4Qhb5R7XzMt/xPe6uMyAIyC\n\
y5Fm9HnvekaepaeFEf3bh4NV1iN/R8px6cFc6ELYxIZc/4Xbm91WGqSdB0iSriaZ\n\
TjgrmaFjSO40tkCaxI9N6DGzJpmpnMn07ifhl2VjnGOYwtyuh6MKEnyLqTrTg9x0\n\
i3mMwskCgYEA9IyljPRerXxHUAJt+cKOayuXyNt80q9PIcGbyRNvn7qIY6tr5ut+\n\
ZbaFgfgHdSJ/4nICRq02HpeDJ8oj9BmhTAhcX6c1irH5ICjRlt40qbPwemIcpybt\n\
mb+DoNYbI8O4dUNGH9IPfGK8dRpOok2m+ftfk94GmykWbZF5CnOKIp8CgYEA2Syc\n\
5xlKB5Qk2ZkwXIzxbzozSfunHhWWdg4lAbyInwa6Y5GB35UNdNWI8TAKZsN2fKvX\n\
RFgCjbPreUbREJaM3oZ92o5X4nFxgjvAE1tyRqcPVbdKbYZgtcqqJX06sW/g3r/3\n\
RH0XPj2SgJIHew9sMzjGWDViMHXLmntI8rVA7d0CgYBOr36JFwvrqERN0ypNpbMr\n\
epBRGYZVSAEfLGuSzEUrUNqXr019tKIr2gmlIwhLQTmCxApFcXArcbbKs7jTzvde\n\
PoZyZJvOr6soFNozP/YT8Ijc5/quMdFbmgqhUqLS5CPS3z2N+YnwDNj0mO1aPcAP\n\
STmcm2DmxdaolJksqrZ0owKBgQCD0KJDWoQmaXKcaHCEHEAGhMrQot/iULQMX7Vy\n\
gl5iN5E2EgFEFZIfUeRWkBQgH49xSFPWdZzHKWdJKwSGDvrdrcABwdfx520/4MhK\n\
d3y7CXczTZbtN1zHuoTfUE0pmYBhcx7AATT0YCblxrynosrHpDQvIefBBh5YW3AB\n\
cKZCOQKBgEM/ixzI/OVSZ0Py2g+XV8+uGQyC5XjQ6cxkVTX3Gs0ZXbemgUOnX8co\n\
eCXS4VrhEf4/HYMWP7GB5MFUOEVtlLiLM05ruUL7CrphdfgayDXVcTPfk75lLhmu\n\
KAwp3tIHPoJOQiKNQ3/qks5km/9dujUGU2ARiU3qmxLMdgegFz8e\n\
-----END RSA PRIVATE KEY-----\n";

const CHANNEL_COUNT: usize = ServiceChannels::MaximumChannel as usize;
const EMPTY_DATA_ARRAY: Vec<u8> = Vec::new();

pub struct AapHandler <'a> {
    usb_handler: AndroidUSBConnection,
    context: &'a Arc<Mutex<Context>>,
    
    mpv_video: &'a Arc<Mutex<MpvVideo>>,
    rd_audio: &'a Arc<Mutex<RdAudio>>,
    nav_audio: &'a Arc<Mutex<RdAudio>>,

    w: u16,
    h: u16,

    current_data: [Vec<u8>; CHANNEL_COUNT],
    total_size: [u32; CHANNEL_COUNT],
    data_complete: [bool; CHANNEL_COUNT],
    channel_session: [i32; CHANNEL_COUNT],

    bio_write: *mut BIO,
    bio_read: *mut BIO,

    ssl: *mut SSL,
    ssl_method: *const SSL_METHOD,
    ssl_context: *mut SSL_CTX,

    had_sdr: bool,
    first_video: bool,

    enter_hold: bool,
    home_hold: bool,

    connection_start: Instant,
    ping_timer: Instant,

    android_start: i64,
}

impl<'a> AapHandler <'a> {
    pub fn new(context: &'a Arc<Mutex<Context>>, mpv_video: &'a Arc<Mutex<MpvVideo>>, rd_audio: &'a Arc<Mutex<RdAudio>>, nav_audio: &'a Arc<Mutex<RdAudio>>, w: u16, h: u16) -> Self {
        return Self {
            usb_handler: AndroidUSBConnection::new(),
            context,

            mpv_video,
            rd_audio,
            nav_audio,

            w,
            h,

            current_data: [EMPTY_DATA_ARRAY; CHANNEL_COUNT],
            total_size: [0; CHANNEL_COUNT],
            data_complete: [false; CHANNEL_COUNT],
            channel_session: [0; CHANNEL_COUNT],

            bio_write: std::ptr::null_mut(),
            bio_read: std::ptr::null_mut(),

            ssl: std::ptr::null_mut(),
            ssl_method: std::ptr::null_mut(),
            ssl_context: std::ptr::null_mut(),

            had_sdr: false,
            first_video: false,

            enter_hold: false,
            home_hold: false,

            connection_start: Instant::now(),

            android_start: 0,
            ping_timer: Instant::now(),
        }
    }

    pub fn process(&mut self) {
        let context = match self.context.try_lock() {
            Ok(context) => context,
            Err(_) => {
                println!("AA Handler process: Context locked.");
                return;
            }
        };

        let phone_type = context.phone_type;
        std::mem::drop(context);

        if phone_type == 3 { //Carplay active.
            return;
        }

        //Android Auto stuff.
        let data;
        if !self.usb_handler.get_connected() {
            let connected = self.usb_handler.connect();

            if connected {
                self.write_block(true, 0, [0x0, 0x1, 0x0, 0x1].to_vec(), ControlMessageType::MESSAGE_VERSION_REQUEST as u16, Duration::from_millis(2000), false);

                if phone_type != 5 {
                    self.start_connection();
                }
            } else {
                self.stop_connection();
            }

            data = [].to_vec();
        } else {
            data = self.usb_handler.read_bytes();
        }

        if data.len() > 0 {
            self.process_bytes(data);
            for i in 0..CHANNEL_COUNT {
                self.process_message(i);
            }
        }

        if phone_type == 5 && Instant::now() - self.ping_timer > Duration::from_millis(2000) {
            self.ping_timer = Instant::now();
            
            let ping_msg = PingRequest::new();
            self.write_message(true, ServiceChannels::ControlChannel as u8, ping_msg, ControlMessageType::MESSAGE_PING_REQUEST as u16, Duration::from_millis(5000), true);
        }
    }

    //Start a connection to the phone.
    fn start_connection(&mut self) {
        self.connection_start = Instant::now();
        match self.context.try_lock() {
            Ok(mut context) => {
                context.phone_type = 5;
            }
            Err(_) => {
                println!("AA Handler start connection: Context locked.")
            }
        }

        //self.refresh_media_audio();
        //self.refresh_nav_audio();
    }

    //End the connection to the phone.
    fn stop_connection(&mut self) {
        self.had_sdr = false;
        self.first_video = false;

        self.bio_write = std::ptr::null_mut();
        self.bio_read = std::ptr::null_mut();
        self.ssl = std::ptr::null_mut();
        self.ssl_method = std::ptr::null_mut();
        self.ssl_context = std::ptr::null_mut();

        match self.context.try_lock() {
            Ok(mut context) => {
                context.phone_type = 0;
            }
            Err(_) => {
                println!("AA Handler stop connection: Context locked.")
            }
        }
    }

    //Clear the message currently being read.
    fn clear_data(&mut self, chan: usize) {
        self.current_data[chan] = Vec::new();
        self.data_complete[chan] = false;
    }

    //Write a protobuf message.
    fn write_message(&mut self, retry: bool, channel: u8, message: impl Message, message_code: u16, timeout: Duration, encrypt: bool) -> bool {
        let data = match message.write_to_bytes() {
            Ok(data) => data,
            Err(e) => {
                println!("Error: {}", e);
                return false;
            }
        };

        let mut buffer = Vec::new();

        let message_code_buffer = u16::to_be_bytes(message_code);
        for b in message_code_buffer {
            buffer.push(b);
        }

        for b in data {
            buffer.push(b);
        }

        if !encrypt {
            return self.write_unencoded(retry, channel, buffer, timeout);
        } else {
            return self.write_encoded(retry, channel, buffer, timeout);
        }
    }

    //Write a byte block.
    fn write_block(&mut self, retry: bool, channel: u8, data: Vec<u8>, message_code: u16, timeout: Duration, encrypt: bool) -> bool {
        let mut buffer = Vec::new();

        let message_code_buffer = u16::to_be_bytes(message_code);
        for b in message_code_buffer {
            buffer.push(b);
        }

        for b in data {
            buffer.push(b);
        }

        if !encrypt {
            return self.write_unencoded(retry, channel, buffer, timeout);
        } else {
            return self.write_encoded(retry, channel, buffer, timeout);
        }
    }

    //Write plain bytes to the USB device.
    fn write_unencoded(&mut self, retry: bool, channel: u8, data: Vec<u8>, timeout: Duration) -> bool {
        if data.len() < 2 {
            return false;
        }
        
        let mut init_flags = 0;
        let message_type = u16::from_be_bytes([data[0], data[1]]);
    
        if channel != ServiceChannels::ControlChannel as u8 && message_type >= 2 && message_type < 0x8000 {
            //Control type message.
            init_flags |= AAP_FRAME_CONTROL_MESSAGE;
        }
    
        let len = data.len()/MAX_FRAME_PAYLOAD_SIZE;
        let full_len = data.len();
        for frame in 0..=len {
            let mut flags = init_flags;
    
            if frame == 0 {
                flags |= AAP_FRAME_FIRST_FRAME as u8;
            }
    
            let mut current_length = MAX_FRAME_PAYLOAD_SIZE;
            if frame + 1 > len {
                flags |= AAP_FRAME_LAST_FRAME as u8;
                current_length = full_len - frame*MAX_FRAME_PAYLOAD_SIZE;
            }
    
            let mut buffer = Vec::new();
            buffer.push(channel);
            buffer.push(flags);
            
            let current_len_bytes = u16::to_be_bytes(current_length as u16);
            for b in current_len_bytes {
                buffer.push(b);
            }
    
            if (flags&AAP_FRAME_FIRST_FRAME) != 0 && (flags&AAP_FRAME_LAST_FRAME) == 0 {
                //Write total length.
                let length_bytes = u32::to_be_bytes(full_len as u32);
                for b in length_bytes {
                    buffer.push(b);
                }
            }
    
            let buffer_data = &data[frame*MAX_FRAME_PAYLOAD_SIZE..current_length];
            for b in buffer_data {
                buffer.push(*b);
            }
    
            if !self.usb_handler.write_bytes(&buffer, retry, timeout) {
                println!("Error. Couldn't write unencoded bytes.");
                return false;
            }
        }
    
        return true;
    }

    //Write encrypted bytes to the USB device.
    fn write_encoded(&mut self, retry: bool, channel: u8, data: Vec<u8>, timeout: Duration) -> bool {
        if data.len() < 2 {
            return false;
        }
        
        let mut init_flags = AAP_FRAME_ENCRYPTED;
        let message_type = u16::from_be_bytes([data[0], data[1]]);
    
        if channel != ServiceChannels::ControlChannel as u8 && message_type >= 2 && message_type < 0x8000 {
            //Control type message.
            init_flags |= AAP_FRAME_CONTROL_MESSAGE;
        }
    
        let len = data.len()/MAX_FRAME_PAYLOAD_SIZE;
        let full_len = data.len();
        for frame in 0..=len {
            let mut flags = init_flags;
    
            if frame == 0 {
                flags |= AAP_FRAME_FIRST_FRAME as u8;
            }
    
            let mut current_length = MAX_FRAME_PAYLOAD_SIZE;
            if frame + 1 > len {
                flags |= AAP_FRAME_LAST_FRAME as u8;
                current_length = full_len - frame*MAX_FRAME_PAYLOAD_SIZE;
            }
    
            let buffer_data = &data[frame*MAX_FRAME_PAYLOAD_SIZE..current_length];
            let ssl = self.ssl;
            if ssl == std::ptr::null_mut() {
                println!("Error: SSL not defined.");
                return false;
            }

            let const_data = buffer_data.as_ptr() as *const c_void;
            unsafe { 
                SSL_write(ssl, const_data, buffer_data.len() as i32);
            }

            let bio_read = self.bio_read;
            if bio_read == std::ptr::null_mut() {
                println!("Error: Read BIO not defined.");
                return false;
            }

            let mut encrypted_data = vec![0;0x10000];
            let encrypted_data_ptr = encrypted_data.as_mut_ptr() as *mut c_void;
            let encrypted_data_len;
            
            unsafe {
                encrypted_data_len = BIO_read(bio_read, encrypted_data_ptr, encrypted_data.len() as i32);
            }
            
            let mut buffer = Vec::new();
            buffer.push(channel);
            buffer.push(flags);
            
            let current_len_bytes = u16::to_be_bytes(encrypted_data_len as u16);
            for b in current_len_bytes {
                buffer.push(b);
            }
    
            if (flags&AAP_FRAME_FIRST_FRAME) != 0 && (flags&AAP_FRAME_LAST_FRAME) == 0 {
                //Write total length.
                let length_bytes = u32::to_be_bytes(full_len as u32);
                for b in length_bytes {
                    buffer.push(b);
                }
            }

            let mut push_len = encrypted_data_len;
            if push_len > 0xFFFF || push_len < 0 {
                push_len = 0xFFFF;
            }

            for i in 0..push_len as usize {
                buffer.push(encrypted_data[i]);
            }
                
            if !self.usb_handler.write_bytes(&buffer, retry, timeout) {
                println!("Error.");
                return false;
            }
        }
    
        return true;
    }

    //Process read bytes.
    fn process_bytes(&mut self, full_data: Vec<u8>) {
        if full_data.len() < 4 {
            return;
        }

        let mut data = full_data.clone();

        let current_channel = data[0] as usize;
        let flags = data[1];

        let len_bytes = [data[2], data[3]];
        let len = u16::from_be_bytes(len_bytes);

        if len as usize > 0xFFFF {// MAX_FRAME_PAYLOAD_SIZE {
            println!("Error: Message is too big.");
            return;
        }

        if (flags&AAP_FRAME_FIRST_FRAME) != 0 {
            self.current_data[current_channel] = Vec::new();
            self.data_complete[current_channel] = false;
        }

        let mut large_message = false;

        let mut start = 4;
        if (flags&AAP_FRAME_FIRST_FRAME) != 0 && (flags&AAP_FRAME_LAST_FRAME) == 0 {
            start += 4;

            if full_data.len() < 6 {
                return;
            }
            
            self.total_size[current_channel] = u32::from_be_bytes([data[2], data[3], data[4], data[5]]);
            large_message = true;
        }

        if (flags&AAP_FRAME_ENCRYPTED) != 0 { //Encrypted data.
            let bio_write = self.bio_write;
            if bio_write == std::ptr::null_mut() {
                println!("Error: Write BIO not defined.");
                self.clear_data(current_channel);
                return;
            }

            let ssl = self.ssl;

            if ssl == std::ptr::null_mut() {
                println!("Error: SSL not defined.");
                self.clear_data(current_channel);
                return;
            }

            let bytes_written: i32;

            unsafe { 
                let const_data = data[start..].as_mut_ptr() as *const c_void;
                bytes_written = BIO_write(bio_write, const_data, (data.len() - start) as i32);
            }

            if bytes_written <= 0 {
                println!("Error: Invalid bytes written.");
                self.clear_data(current_channel);
                return;
            }

            let mut decoded_len = data.len() - start;
            if large_message {
                if self.total_size[current_channel] as usize > decoded_len {
                    decoded_len = self.total_size[current_channel] as usize;
                }
            }

            let decoded_data: &mut [u8] = &mut vec![0;decoded_len];
            let bytes_read;

            unsafe {
                let decoded_data_mut = decoded_data.as_mut_ptr() as *mut c_void;
                bytes_read = SSL_read(ssl, decoded_data_mut, (decoded_len) as i32);
            }

            if bytes_read <= 0 || bytes_read > (decoded_len) as i32 {
                println!("Error: Invalid bytes read.");
                self.clear_data(current_channel);
                return;
            }

            for i in 0..bytes_read as usize {
                self.current_data[current_channel].push(decoded_data[i]);
            }
        } else {
            for i in start..data.len() {
                self.current_data[current_channel].push(data[i]);
            }
        }

        if (flags&AAP_FRAME_LAST_FRAME) != 0 { //Last frame.
            self.data_complete[current_channel] = true;
        }
    }

    //Process a read message.
    fn process_message(&mut self, chan: usize) {
        if !self.data_complete[chan] {
            return;
        }

        let full_msg_data = self.current_data[chan].clone();
        if full_msg_data.len() < 2 {
            return;
        }

        let msg_type = u16::from_be_bytes([full_msg_data[0], full_msg_data[1]]);
        let msg_data = &full_msg_data[2..];

        /*if chan != VideoChannel as usize && chan != MediaAudioChannel as usize && full_msg_data.len() < 100 {
            println!("Received message type {}, on channel {}, length {}.", msg_type, chan as u8, msg_data.len() - 2);
            //println!("Message: {:X?}", msg_data);
        }*/

        if msg_type == ControlMessageType::MESSAGE_VERSION_RESPONSE as u16 { //Version response.
            self.begin_ssl_handshake();
        } else if msg_type == ControlMessageType::MESSAGE_ENCAPSULATED_SSL as u16 { //Handshake response.
            self.handle_ssl_handshake(msg_data.to_vec());
        } else if msg_type == ControlMessageType::MESSAGE_SERVICE_DISCOVERY_REQUEST as u16 { //Service request.
            self.handle_service_discovery_request(chan);
        } else if msg_type == ControlMessageType::MESSAGE_AUDIO_FOCUS_REQUEST as u16 { //Audio focus request.
            let mut request = AudioFocusRequestNotification::new();
            let request_data = msg_data;
            match request.merge_from_bytes(request_data) {
                Ok(_) => {
                    println!("Audio focus request on ch {}", chan);
                    self.handle_audio_focus_request(chan as u8, request);
                }
                Err(e) => {
                    println!("Error: {}", e);
                }
            }
        } else if msg_type == ControlMessageType::MESSAGE_CHANNEL_OPEN_REQUEST as u16 { //Channel open request.
            let mut request = ChannelOpenRequest::new();
            let request_data = msg_data;
            match request.merge_from_bytes(request_data) {
                Ok(_) => {
                    self.handle_channel_open_request(chan as u8, request.service_id() as u32);
                }
                Err(e) => {
                    println!("Error: {}", e);
                }
            }
        } else if msg_type == MediaMessageId::MEDIA_MESSAGE_CODEC_CONFIG as u16 || msg_type == MediaMessageId::MEDIA_MESSAGE_DATA as u16 {
            //TODO: Project media.
            self.send_media_ack(chan as u8);
            if chan == ServiceChannels::VideoChannel as usize {
                let mut video_data = VideoMsg::new();
                video_data.set_data(&full_msg_data);
                //self.handle_video_message(video_data);
            } else if chan == ServiceChannels::Audio1Channel as usize || chan == ServiceChannels::MediaAudioChannel as usize {
                let mut audio_data = AudioMsg::new();
                audio_data.set_data(&full_msg_data);
                audio_data.set_channel(chan as u8);

                //self.handle_audio_message(audio_data);
            }
        } else if msg_type == ControlMessageType::MESSAGE_PING_REQUEST as u16 {
            let mut ping = PingResponse::new();
            let mut timestamp = self.get_timestamp();

            match ping.merge_from_bytes(msg_data) {
                Ok(_) => {
                    timestamp = ping.timestamp() as u64;
                }
                Err(_) => {

                }
            }

            let mut response = PingResponse::new();
            response.set_timestamp(timestamp as i64);

            self.write_message(true, chan as u8, response, ControlMessageType::MESSAGE_PING_RESPONSE as u16, Duration::from_millis(5000), true);
        } else if msg_type == ControlMessageType::MESSAGE_NAV_FOCUS_REQUEST as u16 {
            let mut response = NavFocusNotification::new();
            response.set_focus_type(NavFocusType::NAV_FOCUS_PROJECTED);

            self.write_message(true, chan as u8, response, ControlMessageType::MESSAGE_NAV_FOCUS_NOTIFICATION as u16, Duration::from_millis(5000), true);
        } else if chan == ServiceChannels::PhoneStatusChannel as usize {
            if msg_type == MediaPlaybackStatusMessageId::MEDIA_PLAYBACK_STATUS as u16 {
                let mut playback_msg = MediaPlaybackStatus::new();
                let mut msg_read = false;
                match playback_msg.merge_from_bytes(msg_data) {
                    Ok(_) => {
                        msg_read = true;
                    }
                    Err(e) => {
                        println!("Error: {}", e);
                    }
                }

                if msg_read {
                    let mut context = match self.context.try_lock() {
                        Ok(context) => context,
                        Err(_) => {
                            println!("AAP message processing: Context Locked.");
                            return;
                        }
                    };

                    let playing = playback_msg.state() == media_playback_status::State::PLAYING;
                    let loading = playback_msg.state() == media_playback_status::State::STOPPED;

                    if context.audio_selected {
                        context.track_time = playback_msg.playback_seconds() as i32;

                        context.app = playback_msg.media_source().to_string();
                        context.playing = playing;
                    } else if playing || loading {
                        self.send_button_message(KeyCode::KEYCODE_MEDIA_PAUSE as u32, 0x0);
                        self.send_button_message(KeyCode::KEYCODE_MEDIA_PAUSE as u32, 0x2);
                    }
                }
            } else if msg_type == MediaPlaybackStatusMessageId::MEDIA_PLAYBACK_METADATA as u16 {
                let mut meta_msg = MediaPlaybackMetadata::new();
                let mut msg_read = false;

                match meta_msg.merge_from_bytes(msg_data) {
                    Ok(_) => {
                        msg_read = true;
                    }
                    Err(e) => {
                        println!("Error: {}", e);
                    }
                }

                if msg_read {
                    let mut context = match self.context.try_lock() {
                        Ok(context) => context,
                        Err(_) => {
                            println!("AAP message processing: Context Locked.");
                            return;
                        }
                    };

                    context.song_title = meta_msg.song().to_string();
                    context.artist = meta_msg.artist().to_string();
                    context.album = meta_msg.album().to_string();
                }
                
            }
        } else if chan == ServiceChannels::TouchChannel as usize {
            if msg_type == InputMessageId::INPUT_MESSAGE_KEY_BINDING_REQUEST as u16 {
                self.handle_binding_request(chan as u8);
            }
        } else if chan == ServiceChannels::SensorChannel as usize {
            if msg_type == SensorMessageId::SENSOR_MESSAGE_REQUEST as u16 {
                self.handle_sensor_start_request(chan as u8);
            }
        } else if chan == ServiceChannels::MediaAudioChannel as usize || chan == ServiceChannels::Audio1Channel as usize ||
            chan == ServiceChannels::Audio2Channel as usize || chan == ServiceChannels::VideoChannel as usize || chan == ServiceChannels::MicrophoneChannel as usize {
            //self.send_media_ack(chan as u8);
            if msg_type == MediaMessageId::MEDIA_MESSAGE_SETUP as u16 {
                self.handle_media_setup_request(chan as u8);
            } else if msg_type == MediaMessageId::MEDIA_MESSAGE_START as u16 {
                let mut request = MediaStartRequest::new();
                match request.merge_from_bytes(msg_data) {
                    Ok(_) => {
                        self.channel_session[chan] = request.session;
                    }
                    Err(e) => {
                        println!("Error: {}", e);
                    }
                }
            } else if msg_type == MediaMessageId::MEDIA_MESSAGE_STOP as u16 {
                self.channel_session[chan] = 0;
            }
        }

        self.clear_data(chan);
    }

    //Set night mode.
    pub fn set_night_mode(&mut self, night: bool) {
        if !self.usb_handler.get_connected() {
            return;
        }

        let mut night_sensor_msg = NightModeData::new();
        night_sensor_msg.set_night_mode(night);

        let mut sensor_msg = SensorBatch::new();
        sensor_msg.night_mode_data.push(night_sensor_msg);

        self.write_message(true, ServiceChannels::SensorChannel as u8, sensor_msg, SensorMessageId::SENSOR_MESSAGE_BATCH as u16, Duration::from_millis(5000), true);
    }

    //Start playing audio if start is true.
    pub fn start_stop_audio(&mut self, start: bool) {
        if !self.usb_handler.get_connected() {
            return;
        }

        if start {
            self.send_button_message(KeyCode::KEYCODE_MEDIA_PLAY as u32, 0x0);
            self.send_button_message(KeyCode::KEYCODE_MEDIA_PLAY as u32, 0x2);
        } else {
            self.send_button_message(KeyCode::KEYCODE_MEDIA_PAUSE as u32, 0x0);
            self.send_button_message(KeyCode::KEYCODE_MEDIA_PAUSE as u32, 0x2);
        }
    }

    //Show the audio source window.
    pub fn show_audio_window(&mut self) {
        self.send_button_message(KeyCode::KEYCODE_MUSIC as u32, 0x0);
        self.send_button_message(KeyCode::KEYCODE_MUSIC as u32, 0x2);
    }

    //Internal message handles:
    //Send the initial handshake.
    fn begin_ssl_handshake(&mut self) {
        openssl_sys::init();
        
        unsafe {
            OPENSSL_init_ssl(0, std::ptr::null());
            OPENSSL_init_ssl(OPENSSL_INIT_LOAD_SSL_STRINGS | 0x2, std::ptr::null());

            if RAND_status() != 1 {
                return;
            }

            let cert_buf = CERT_BUF.as_ptr() as *const c_void;
            let certificate_bio = BIO_new_mem_buf(cert_buf, CERT_BUF.len() as i32);

            let u1 = std::ptr::null_mut();
            let x509= std::ptr::null_mut();
            let pccb1 = None;

            let x509_cert = PEM_read_bio_X509(certificate_bio, x509, pccb1, u1);
            if x509_cert == std::ptr::null_mut() {
                println!("BIO Certification Error.");
                return;
            }

            let priv_buf = PRIV_BUF.as_ptr() as *const c_void;
            let private_bio = BIO_new_mem_buf(priv_buf, PRIV_BUF.len() as i32);

            let u2 = std::ptr::null_mut();
            let pccb2 = None;
            let pkey = std::ptr::null_mut();

            let pkey_cert = PEM_read_bio_PrivateKey(private_bio, pkey, pccb2, u2);
            if pkey_cert == std::ptr::null_mut() {
                println!("BIO PKEY Certification Error.");
                return;
            }

            self.ssl_method = TLS_client_method();
            self.ssl_context = SSL_CTX_new(self.ssl_method);
            
            SSL_CTX_use_certificate(self.ssl_context, x509_cert);
            SSL_CTX_use_PrivateKey(self.ssl_context, pkey_cert);

            let ssl = SSL_new(self.ssl_context);
            if ssl == std::ptr::null_mut() {
                println!("Error: SSL returned null pointer.");
                return;
            } else {
                self.ssl = ssl;
            }

            //TODO: Check private key?
            let bio_write = BIO_new(BIO_s_mem());
            if bio_write == std::ptr::null_mut() {
                println!("Error: Write BIO returned null pointer.");
                return;
            } else {
                self.bio_write = bio_write;
            }

            let bio_read = BIO_new(BIO_s_mem());
            if bio_read == std::ptr::null_mut() {
                println!("Error: Read BIO returned null pointer.");
                return;
            } else {
                self.bio_read = bio_read;
            }

            SSL_set_bio(self.ssl, self.bio_write, self.bio_read);
            //TODO: Read/write sizes?

            SSL_set_connect_state(self.ssl);
            SSL_set_verify(self.ssl, SSL_VERIFY_NONE, None);

            ERR_clear_error();

            let ret = SSL_do_handshake(self.ssl);
            if SSL_get_error(self.ssl, ret) == SSL_ERROR_WANT_READ {
                let mut handshake_data = vec![0; MAX_FRAME_PAYLOAD_SIZE];
                
                let c_handshake_data = handshake_data.as_mut_ptr() as *mut c_void;
                let len = BIO_read(self.bio_read, c_handshake_data, handshake_data.len() as i32);

                let mut set_handshake_data = Vec::new();
                for i in 0..len {
                    set_handshake_data.push(handshake_data[i as usize]);
                }

                self.write_block(false, ServiceChannels::ControlChannel as u8, set_handshake_data, ControlMessageType::MESSAGE_ENCAPSULATED_SSL as u16, Duration::from_millis(5000), false);
            } else {
                println!("Error code {}", SSL_get_error(self.ssl, ret));
                return;
            }
        }

        println!("Handshake successful!");
    }

    //Handle a handshake response.
    fn handle_ssl_handshake(&mut self, data: Vec<u8>) {

        let c_data = data.as_ptr() as *const c_void;
        unsafe { 
            BIO_write(self.bio_write, c_data, data.len() as i32);

            let ret = SSL_do_handshake(self.ssl);

            if SSL_get_error(self.ssl, ret) == SSL_ERROR_WANT_READ {
                let mut handshake_data = vec![0; MAX_FRAME_PAYLOAD_SIZE];
                
                let c_handshake_data = handshake_data.as_mut_ptr() as *mut c_void;
                let len = BIO_read(self.bio_read, c_handshake_data, handshake_data.len() as i32);

                let mut set_handshake_data = Vec::new();
                for i in 0..len {
                    set_handshake_data.push(handshake_data[i as usize]);
                }

                self.write_block(false, ServiceChannels::ControlChannel as u8, set_handshake_data, ControlMessageType::MESSAGE_ENCAPSULATED_SSL as u16, Duration::from_millis(5000), false);
            } else if SSL_get_error(self.ssl, ret) != 0 {
                return;
            }

            let auth_message = AuthResponse::new();
            self.write_message(false, ServiceChannels::ControlChannel as u8, auth_message, ControlMessageType::MESSAGE_AUTH_COMPLETE as u16, Duration::from_millis(2000), false);
        }
    }

    //Send a button message.
    fn send_button_message(&mut self, button: u32, state: u8) {
        if state == 0x1 {
            return;
        }

        let mut press_key = Key::new();
        press_key.set_keycode(button);
        press_key.set_down(state != 0x2);
        press_key.set_metastate(button);
        press_key.set_longpress(false);

        let mut press_event = KeyEvent::new();
        press_event.keys.push(press_key);

        let mut press_msg = InputReport::new();
        press_msg.key_event = MessageField::some(press_event);
        press_msg.set_timestamp(self.get_timestamp());

        println!("Sent: {:X?}", press_msg.write_to_bytes());
        self.write_message(true, ServiceChannels::TouchChannel as u8, press_msg, InputMessageId::INPUT_MESSAGE_INPUT_REPORT as u16, Duration::from_millis(5000), true);
    }

    //Send a scroll wheel message- clockwise if cw is true.
    fn send_scroll_message(&mut self, cw: bool) {
        let mut scroll_data = Vec::new();

        let mut os = CodedOutputStream::vec(&mut scroll_data);
        let _ = os.write_uint32(1, KeyCode::KEYCODE_ROTARY_CONTROLLER as u32);

        if cw {
            let _ = os.write_int32(2, 1);
        } else {
            let _ = os.write_int32(2, -1);
        }

        std::mem::drop(os);

        let scroll_data_l = (scroll_data.len()&0xFF) as u8;

        scroll_data.insert(0, 0xA);
        scroll_data.insert(1, scroll_data_l);

        let mut send_data = Vec::new();

        let mut os = CodedOutputStream::vec(&mut send_data);
        let _ = os.write_uint64(1, self.get_timestamp());
        let _ = os.write_bytes(6, &scroll_data);

        std::mem::drop(os);

        println!("Sent: {:X?}", send_data);
        self.write_block(true, ServiceChannels::TouchChannel as u8, send_data, InputMessageId::INPUT_MESSAGE_INPUT_REPORT as u16, Duration::from_millis(5000), true);
    }

    //Get the current timestamp.
    fn get_timestamp(&self) -> u64 {
        let time = (Instant::now() - self.connection_start).as_nanos();
        return (time&0xFFFFFFFFFFFFFF) as u64;
    }

    //Handle a service discovery request.
    fn handle_service_discovery_request(&mut self, chan: usize) {
        if self.had_sdr {
            return;
        }

        let mut request = ServiceDiscoveryRequest::new();
        let request_data = &self.current_data[chan][2..];

        match request.merge_from_bytes(request_data) {
            Ok(_) => {
                let phone_name = request.device_name();
                match self.context.try_lock() {
                    Ok(mut context) => {
                        context.phone_name = phone_name.to_string();
                    }
                    Err(_) => {
                        println!("Service discovery request: Context locked.");
                    }
                }
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }

        let context = match self.context.try_lock() {
            Ok(context) => context,
            Err(_) => {
                println!("Service discovery request: Context locked.");
                return;
            }
        };
        
        let mut response = ServiceDiscoveryResponse::new();
        //TODO: Configure the response based on the context settings.

        //Input:
        let mut input_service = InputSourceService::new();

        input_service.keycodes_supported.push(KeyCode::KEYCODE_HOME as i32);
        input_service.keycodes_supported.push(KeyCode::KEYCODE_BACK as i32);

        input_service.keycodes_supported.push(KeyCode::KEYCODE_CALL as i32);

        input_service.keycodes_supported.push(KeyCode::KEYCODE_ENDCALL as i32);
        input_service.keycodes_supported.push(KeyCode::KEYCODE_DPAD_UP as i32);
        input_service.keycodes_supported.push(KeyCode::KEYCODE_DPAD_DOWN as i32);
        input_service.keycodes_supported.push(KeyCode::KEYCODE_DPAD_LEFT as i32);
        input_service.keycodes_supported.push(KeyCode::KEYCODE_DPAD_RIGHT as i32);
        input_service.keycodes_supported.push(KeyCode::KEYCODE_DPAD_CENTER as i32);
        //input_service.keycodes_supported.push(InputButton::ButtonMic as i32);
        input_service.keycodes_supported.push(KeyCode::KEYCODE_MEDIA_PLAY_PAUSE as i32);
        input_service.keycodes_supported.push(KeyCode::KEYCODE_MEDIA_NEXT as i32);
        input_service.keycodes_supported.push(KeyCode::KEYCODE_MEDIA_PREVIOUS as i32);
        
        input_service.keycodes_supported.push(KeyCode::KEYCODE_MUSIC as i32);

        input_service.keycodes_supported.push(KeyCode::KEYCODE_ROTARY_CONTROLLER as i32);

        //input_service.keycodes_supported.push(InputButton::ButtonTel as i32);

        input_service.keycodes_supported.push(KeyCode::KEYCODE_MEDIA as i32);
        input_service.keycodes_supported.push(KeyCode::KEYCODE_RADIO as i32);
        //input_service.keycodes_supported.push(InputButton::Button1 as i32);
        //input_service.keycodes_supported.push(InputButton::Button2 as i32);
        //input_service.keycodes_supported.push(InputButton::Button3 as i32);
        input_service.keycodes_supported.push(KeyCode::KEYCODE_MEDIA_PLAY as i32);
        input_service.keycodes_supported.push(KeyCode::KEYCODE_MEDIA_PAUSE as i32);
        
        let mut input_service_wrapper = Service::new();
        input_service_wrapper.set_id(ServiceChannels::TouchChannel as i32);
        input_service_wrapper.input_source_service = MessageField::some(input_service);
        response.services.push(input_service_wrapper);

        //Sensors:
        let mut sensor_service = SensorSourceService::new();

        let mut sensor_driving_status = sensor_source_service::Sensor::new();
        sensor_driving_status.set_sensor_type(SensorType::SENSOR_DRIVING_STATUS_DATA);

        let mut sensor_night = sensor_source_service::Sensor::new();
        sensor_night.set_sensor_type(SensorType::SENSOR_NIGHT_MODE);

        let mut sensor_location = sensor_source_service::Sensor::new();
        sensor_location.set_sensor_type(SensorType::SENSOR_LOCATION);

        sensor_service.sensors.push(sensor_driving_status);
        sensor_service.sensors.push(sensor_night);
        sensor_service.sensors.push(sensor_location);

        let mut sensor_service_wrapper = Service::new();
        sensor_service_wrapper.set_id(ServiceChannels::SensorChannel as i32);
        sensor_service_wrapper.sensor_source_service = MessageField::some(sensor_service);
        response.services.push(sensor_service_wrapper);

        //Video:
        let mut video_sink = MediaSinkService::new();
        let mut video_config = VideoConfiguration::new();

        let mut margin_w = 0;
        let mut margin_h = 0;

        if self.w >= 1920 || self.h >= 1080 {
            video_config.set_codec_resolution(VideoCodecResolutionType::VIDEO_1920x1080);

            if self.h < 1080 {
                margin_h = 1080 - self.h as u32;
            }
            if self.w < 1920 {
                margin_w = 1920 - self.w as u32;
            }
        } else if self.w >= 1280 || self.h >= 720 {
            video_config.set_codec_resolution(VideoCodecResolutionType::VIDEO_1280x720);
            
            if self.h < 720 {
                margin_h = 720 - self.h as u32;
            }
            if self.w < 1280 {
                margin_w = 1280 - self.w as u32;
            }
        } else {
            if self.w <= 800 && self.h <= 480 {
                video_config.set_codec_resolution(VideoCodecResolutionType::VIDEO_800x480);
                
                if self.h < 480 {
                    margin_h = 480 - self.h as u32;
                }
                if self.w < 800 {
                    margin_w = 800 - self.w as u32;
                }
            } else {
                video_config.set_codec_resolution(VideoCodecResolutionType::VIDEO_1280x720);
            
                if self.h < 720 {
                    margin_h = 720 - self.h as u32;
                }
                if self.w < 1280 {
                    margin_w = 1280 - self.w as u32;
                }
            }
        }
        
        video_config.set_frame_rate(VideoFrameRateType::VIDEO_FPS_60);
        video_config.set_width_margin(margin_w);
        video_config.set_height_margin(margin_h);
        video_config.set_density(160);
        video_config.set_decoder_additional_depth(0);

        video_sink.video_configs.push(video_config);

        video_sink.set_available_type(MediaCodecType::MEDIA_CODEC_VIDEO_H264_BP);
        video_sink.set_available_while_in_call(true);
    
        let mut video_sink_wrapper = Service::new();
        video_sink_wrapper.set_id(ServiceChannels::VideoChannel as i32);
        video_sink_wrapper.media_sink_service = MessageField::some(video_sink);
        response.services.push(video_sink_wrapper);

        //Main audio:
        let mut audio_sink = MediaSinkService::new();
        let mut audio_config = AudioConfiguration::new();

        audio_config.set_sampling_rate(48000);
        audio_config.set_number_of_bits(16);
        audio_config.set_number_of_channels(2);
        
        audio_sink.set_available_type(MediaCodecType::MEDIA_CODEC_AUDIO_PCM);
        audio_sink.set_available_while_in_call(false);
        audio_sink.set_audio_type(AudioStreamType::AUDIO_STREAM_MEDIA);
        audio_sink.audio_configs.push(audio_config);

        let mut audio_sink_wrapper = Service::new();
        audio_sink_wrapper.set_id(ServiceChannels::MediaAudioChannel as i32);
        audio_sink_wrapper.media_sink_service = MessageField::some(audio_sink);
        response.services.push(audio_sink_wrapper);

        //Voice audio:
        let mut voice_sink = MediaSinkService::new();
        let mut voice_config = AudioConfiguration::new();
        
        voice_config.set_sampling_rate(16000);
        voice_config.set_number_of_bits(16);
        voice_config.set_number_of_channels(1);
        
        voice_sink.set_available_type(MediaCodecType::MEDIA_CODEC_AUDIO_PCM);
        voice_sink.set_audio_type(AudioStreamType::AUDIO_STREAM_GUIDANCE);
        voice_sink.set_available_while_in_call(true);
        voice_sink.audio_configs.push(voice_config);

        let mut voice_sink_wrapper = Service::new();
        voice_sink_wrapper.set_id(ServiceChannels::Audio1Channel as i32);
        voice_sink_wrapper.media_sink_service = MessageField::some(voice_sink);
        response.services.push(voice_sink_wrapper);

        //Mic audio:
        let mut mic_source = MediaSourceService::new();
        let mut mic_config = AudioConfiguration::new();

        mic_config.set_sampling_rate(16000);
        mic_config.set_number_of_bits(16);
        mic_config.set_number_of_channels(1);

        mic_source.set_available_type(MediaCodecType::MEDIA_CODEC_AUDIO_PCM);
        mic_source.audio_config = MessageField::some(mic_config);

        let mut mic_source_wrapper = Service::new();
        mic_source_wrapper.set_id(ServiceChannels::MicrophoneChannel as i32);
        mic_source_wrapper.media_source_service = MessageField::some(mic_source);
        response.services.push(mic_source_wrapper);

        //Media playback:
        let media_playback_status_service = MediaPlaybackStatusService::new();
        let mut media_status_service_wrapper = Service::new();
        media_status_service_wrapper.set_id(ServiceChannels::MediaStatusChannel as i32);
        media_status_service_wrapper.media_playback_service = MessageField::some(media_playback_status_service);
        response.services.push(media_status_service_wrapper);

        //Notifications:
        let notification_service = GenericNotificationService::new();
        let mut notification_service_wrapper = Service::new();
        notification_service_wrapper.set_id(ServiceChannels::NotificationChannel as i32);
        notification_service_wrapper.generic_notification_service = MessageField::some(notification_service);
        response.services.push(notification_service_wrapper);

        //Navigation:
        let mut navigation_service = NavigationStatusService::new();
        navigation_service.set_minimum_interval_ms(750);
        navigation_service.set_type(navigation_status_service::InstrumentClusterType::ENUM);

        let mut navigation_service_wrapper = Service::new();
        navigation_service_wrapper.set_id(ServiceChannels::NavigationChannel as i32);
        navigation_service_wrapper.navigation_status_service = MessageField::some(navigation_service);
        response.services.push(navigation_service_wrapper);

        //response.set_make("AidF".to_string());
        response.set_head_unit_make("AidF".to_string());
        response.set_head_unit_model("AIA-RPI100".to_string());
        response.set_can_play_native_media_during_vr(true);

        response.set_display_name("AidF".to_string());

        println!("Response: {:X?}", response.write_to_bytes());
        self.write_message(true, chan as u8, response, ControlMessageType::MESSAGE_SERVICE_DISCOVERY_RESPONSE as u16, Duration::from_millis(2000), false);

        self.had_sdr = true;
    }

    //Handle an audio focus request.
    fn handle_audio_focus_request(&mut self, channel: u8, req: AudioFocusRequestNotification) {
        let mut response = AudioFocusNotification::new();

        if req.request() == AudioFocusRequestType::AUDIO_FOCUS_RELEASE { //Release.
            response.set_focus_state(AudioFocusStateType::AUDIO_FOCUS_STATE_LOSS);
        } else {
            response.set_focus_state(AudioFocusStateType::AUDIO_FOCUS_STATE_GAIN);
        }

        self.write_message(true, channel, response, ControlMessageType::MESSAGE_AUDIO_FOCUS_NOTIFICATION as u16, Duration::from_millis(5000), true);
    }

    //Handle a channel open request.
    fn handle_channel_open_request(&mut self, channel: u8, channel_to_open: u32) {
        let mut response = ChannelOpenResponse::new();
        response.set_status(MessageStatus::STATUS_SUCCESS);

        self.write_message(true, channel, response.clone(), ControlMessageType::MESSAGE_CHANNEL_OPEN_RESPONSE as u16, Duration::from_millis(5000), true);
        if channel as u32 != channel_to_open {
            self.write_message(true, channel_to_open as u8, response, ControlMessageType::MESSAGE_CHANNEL_OPEN_RESPONSE as u16, Duration::from_millis(5000), true);
        }

        if channel == ServiceChannels::SensorChannel as u8 || channel_to_open == ServiceChannels::SensorChannel as u32 {
            let mut night_sensor_msg = NightModeData::new();

            match self.context.try_lock() {
                Ok(context) => {
                    night_sensor_msg.set_night_mode(context.headlights_on); 
                }
                Err(_) => {
                    println!("Channel Open Request: Context Locked.")
                }
            }

            let mut status_msg = DrivingStatusData::new();
            status_msg.set_status(DrivingStatus::DRIVE_STATUS_UNRESTRICTED as i32);

            let mut sensor_msg = SensorBatch::new();
            sensor_msg.night_mode_data.push(night_sensor_msg);
            sensor_msg.driving_status_data.push(status_msg);
            
            self.write_message(true, ServiceChannels::SensorChannel as u8, sensor_msg, SensorMessageId::SENSOR_MESSAGE_BATCH as u16, Duration::from_millis(5000), true);
        }
    }

    //Handle a binding request.
    fn handle_binding_request(&mut self, channel: u8) {
        let mut response = KeyBindingResponse::new();
        response.set_status(0);

        self.write_message(true, channel, response, InputMessageId::INPUT_MESSAGE_KEY_BINDING_RESPONSE as u16, Duration::from_millis(5000), true);
    }

    //Handle a sensor start request.
    fn handle_sensor_start_request(&mut self, channel: u8) {
        let mut response = SensorResponse::new();
        response.set_status(MessageStatus::STATUS_SUCCESS);

        self.write_message(true, channel, response, SensorMessageId::SENSOR_MESSAGE_RESPONSE as u16, Duration::from_millis(5000), true);
    }

    //Handle a media setup request.
    fn handle_media_setup_request(&mut self, channel: u8) {
        let mut response = Config::new();
        response.set_status(config::Status::STATUS_READY);
        response.set_max_unacked(1);
        response.configuration_indices.push(0);

        self.write_message(true, channel, response, MediaMessageId::MEDIA_MESSAGE_CONFIG as u16, Duration::from_millis(5000), true);

        if !self.first_video && channel == ServiceChannels::VideoChannel as u8 {
            self.first_video = true;

            let mut video_focus_gained = VideoFocusNotification::new();
            video_focus_gained.set_focus(VideoFocusMode::VIDEO_FOCUS_PROJECTED);
            video_focus_gained.set_unsolicited(true);

            self.write_message(true, ServiceChannels::VideoChannel as u8, video_focus_gained, MediaMessageId::MEDIA_MESSAGE_VIDEO_FOCUS_NOTIFICATION as u16, Duration::from_millis(5000), true);

            let mut sensor_message = SensorBatch::new();

            let mut location_message = LocationData::new();
            location_message.set_speed_e3(0);

            sensor_message.location_data.push(location_message);

            self.write_message(true, ServiceChannels::SensorChannel as u8, sensor_message, SensorMessageId::SENSOR_MESSAGE_BATCH as u16, Duration::from_millis(5000), true);
        }
    }

    //Send a media acknowledgement.
    fn send_media_ack(&mut self, channel: u8) {
        let mut ack_msg = Start::new();
        ack_msg.set_session_id(self.channel_session[channel as usize]);
        ack_msg.set_configuration_index(1);

        self.write_message(true, channel, ack_msg, MediaMessageId::MEDIA_MESSAGE_ACK as u16, Duration::from_millis(5000), true);
    }
}