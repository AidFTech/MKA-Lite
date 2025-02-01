use std::sync::Arc;
use std::sync::Mutex;
use std::time::Duration;
use std::ffi::c_void;
use std::time::Instant;

use protobuf::CodedOutputStream;
use protobuf::Message;
use openssl_sys::*;

use crate::Context;

use crate::aap::aap_services::*;
use crate::aap::aap_services::ServiceChannels::*;
use crate::aap::aap_services::InputChannelMessage::*;
use crate::aap::aap_services::SensorChannelMessage::*;
use crate::aap::aap_services::MediaChannelMessage::*;

use crate::aap::aap_channel_descriptor::*;

use super::aap_messages::*;
use super::aap_usb::AndroidUSBConnection;
use super::media_messages::*;
use super::sensor_messages::SensorMessage;

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

const CHANNEL_COUNT: usize = MaximumChannel as usize;
const EMPTY_DATA_ARRAY: Vec<u8> = Vec::new();

pub struct AapHandler <'a> {
    usb_handler: AndroidUSBConnection,
    context: &'a Arc<Mutex<Context>>,

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
}

impl<'a> AapHandler <'a> {
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
                self.write_block(true, 0, [0x0, 0x1, 0x0, 0x1].to_vec(), ControlMessage::ControlMessageVersionRequest as u16, Duration::from_millis(2000), false);

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
            
            let ping_msg = PingMessage::new();
            self.write_message(true, ControlChannel as u8, ping_msg, ProtocolMessage::ProtocolMessagePingRequest as u16, Duration::from_millis(5000), true);
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

        if msg_type == ControlMessage::ControlMessageVersionResponse as u16 { //Version response.
            self.begin_ssl_handshake();
        } else if msg_type == ControlMessage::ControlMessageSSLHandshake as u16 { //Handshake response.
            self.handle_ssl_handshake(msg_data.to_vec());
        } else if msg_type == ControlMessage::ControlMessageServiceDiscoveryRequest as u16 { //Service request.
            self.handle_service_discovery_request(chan);
        } else if msg_type == ProtocolMessage::ProtocolMessageAudioFocusRequest as u16 { //Audio focus request.
            let mut request = AudioFocusRequest::new();
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
        } else if msg_type == ProtocolMessage::ProtocolMessageChannelOpenRequest as u16 { //Channel open request.
            let mut request = ChannelOpenRequest::new();
            let request_data = msg_data;
            match request.merge_from_bytes(request_data) {
                Ok(_) => {
                    self.handle_channel_open_request(chan as u8, request.channel_id);
                }
                Err(e) => {
                    println!("Error: {}", e);
                }
            }
        } else if msg_type == ProtocolMessage::ProtocolMessageMediaData as u16 || msg_type == ProtocolMessage::ProtocolMessageMediaDataTime as u16 {
            //TODO: Project media.
            self.send_media_ack(chan as u8);
            if chan == VideoChannel as usize {
                let mut video_data = VideoMsg::new();
                video_data.set_data(&full_msg_data);
                //self.handle_video_message(video_data);
            } else if chan == Audio1Channel as usize || chan == MediaAudioChannel as usize {
                let mut audio_data = AudioMsg::new();
                audio_data.set_data(&full_msg_data);
                audio_data.set_channel(chan as u8);

                //self.handle_audio_message(audio_data);
            }
        } else if msg_type == ProtocolMessage::ProtocolMessagePingRequest as u16 {
            let mut ping = PingMessage::new();
            let mut timestamp = self.get_timestamp();

            match ping.merge_from_bytes(msg_data) {
                Ok(_) => {
                    timestamp = ping.timestamp as u64;
                }
                Err(_) => {

                }
            }

            let mut response = PingMessage::new();
            response.timestamp = timestamp as i64;

            self.write_message(true, chan as u8, response, ProtocolMessage::ProtocolMessagePingResponse as u16, Duration::from_millis(5000), true);
        } else if msg_type == ProtocolMessage::ProtocolMessageNavigationFocusRequest as u16 {
            let mut response = NavigationFocusMessage::new();
            response.focus_type = 2;

            self.write_message(true, chan as u8, response, ProtocolMessage::ProtocolMessageNavigationFocusResponse as u16, Duration::from_millis(5000), true);
        } else if chan == PhoneStatusChannel as usize {
            if msg_type == MediaInfoMessage::MediaInfoMessagePlayback as u16 {
                let mut playback_msg = MediaPlaybackMessage::new();
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

                    let playing = playback_msg.playback_state == 2;
                    let loading = playback_msg.playback_state == 1;

                    if context.audio_selected {
                        context.track_time = playback_msg.track_progress;

                        context.app = playback_msg.media_app;
                        context.playing = playing;
                    } else if playing || loading {
                        self.send_button_message(InputButton::ButtonStop as u32);
                    }
                }
            } else if msg_type == MediaInfoMessage::MediaInfoMessageMeta as u16 {
                let mut meta_msg = MediaMetaMessage::new();
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

                    context.song_title = meta_msg.track_name;
                    context.artist = meta_msg.artist_name;
                    context.album = meta_msg.album_name;
                }
                
            }
        } else if chan == TouchChannel as usize {
            if msg_type == InputChannelMessageBindingRequest as u16 {
                self.handle_binding_request(chan as u8);
            }
        } else if chan == SensorChannel as usize {
            if msg_type == SensorChannelMessageStartRequest as u16 {
                self.handle_sensor_start_request(chan as u8);
            }
        } else if chan == MediaAudioChannel as usize || chan == Audio1Channel as usize ||
            chan == Audio2Channel as usize || chan == VideoChannel as usize || chan == MicrophoneChannel as usize {
            //self.send_media_ack(chan as u8);
            if msg_type == MediaChannelMessageSetupRequest as u16 {
                self.handle_media_setup_request(chan as u8);
            } else if msg_type == MediaChannelMessageStartRequest as u16 {
                let mut request = MediaStartRequest::new();
                match request.merge_from_bytes(msg_data) {
                    Ok(_) => {
                        self.channel_session[chan] = request.session;
                    }
                    Err(e) => {
                        println!("Error: {}", e);
                    }
                }
            } else if msg_type == MediaChannelMessageStopRequest as u16 {
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

        let mut sensor_msg = SensorMessage::new();
        let night_sensor = sensor_msg.add_event_night();

        night_sensor.night_mode = night;

        self.write_message(true, SensorChannel as u8, sensor_msg, SensorChannelMessageEvent as u16, Duration::from_millis(5000), true);
    }

    //Start playing audio if start is true.
    pub fn start_stop_audio(&mut self, start: bool) {
        if !self.usb_handler.get_connected() {
            return;
        }

        if start {
            self.send_button_message(InputButton::ButtonStart as u32);
        } else {
            self.send_button_message(InputButton::ButtonStop as u32);
        }
    }

    //Show the audio source window.
    pub fn show_audio_window(&mut self) {
        self.send_button_message(InputButton::ButtonMusic as u32);
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

                self.write_block(false, ServiceChannels::ControlChannel as u8, set_handshake_data, ControlMessage::ControlMessageSSLHandshake as u16, Duration::from_millis(5000), false);
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

                self.write_block(false, ServiceChannels::ControlChannel as u8, set_handshake_data, ControlMessage::ControlMessageSSLHandshake as u16, Duration::from_millis(5000), false);
            } else if SSL_get_error(self.ssl, ret) != 0 {
                return;
            }

            let auth_message = AuthCompleteResponse::new();
            self.write_message(false, ServiceChannels::ControlChannel as u8, auth_message, ControlMessage::ControlMessageAuthComplete as u16, Duration::from_millis(2000), false);
        }
    }

    //Send a button message.
    fn send_button_message(&mut self, button: u32) {
        let press_msg = ButtonPressMessage::get_button_press(button, true);
        let release_msg = ButtonPressMessage::get_button_press(button, false);

        let mut press_data = press_msg.write_to_bytes().unwrap();
        let mut release_data = release_msg.write_to_bytes().unwrap();

        let press_len = (press_data.len()&0xFF) as u8;
        let release_len = (release_data.len()&0xFF) as u8;

        press_data.insert(0, 0xA);
        press_data.insert(1, press_len);

        release_data.insert(0, 0xA);
        release_data.insert(1, release_len);

        let mut send_data = Vec::new();

        let mut os = CodedOutputStream::vec(&mut send_data);
        let _ = os.write_uint64(1, self.get_timestamp());
        let _ = os.write_bytes(4, &press_data);

        std::mem::drop(os);

        self.write_block(true, TouchChannel as u8, send_data, InputChannelMessageInputEvent as u16, Duration::from_millis(5000), true);

        let mut send_rel_data = Vec::new();

        let mut os = CodedOutputStream::vec(&mut send_rel_data);
        let _ = os.write_uint64(1, self.get_timestamp());
        let _ = os.write_bytes(4, &release_data);

        std::mem::drop(os);

        self.write_block(true, TouchChannel as u8, send_rel_data, InputChannelMessageInputEvent as u16, Duration::from_millis(5000), true);
    }

    //Send a scroll wheel message- clockwise if cw is true.
    fn send_scroll_message(&mut self, cw: bool) {
        let mut scroll_data = Vec::new();

        let mut os = CodedOutputStream::vec(&mut scroll_data);
        let _ = os.write_uint32(1, InputButton::ButtonScroll as u32);

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

        self.write_block(true, TouchChannel as u8, send_data, InputChannelMessageInputEvent as u16, Duration::from_millis(5000), true);
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
                let phone_name = request.get_phone_name();
                match self.context.try_lock() {
                    Ok(mut context) => {
                        context.phone_name = phone_name;
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
        
        let mut response = ServiceDiscoveryResponse::new();
        //TODO: Configure the response based on the context settings.

        let input_channel = response.add_channel(TouchChannel as u32);
        let input_touch = input_channel.add_input_event();

        //let touch_config = input_touch.add_touch_parameter();
        //touch_config.set_dimensions(800, 480);

        input_touch.add_keycode(InputButton::ButtonMenu as u32);
        input_touch.add_keycode(InputButton::ButtonMic1 as u32);
        input_touch.add_keycode(InputButton::ButtonHome as u32);
        input_touch.add_keycode(InputButton::ButtonBack as u32);
        input_touch.add_keycode(InputButton::ButtonPhone as u32);
        input_touch.add_keycode(InputButton::ButtonCallend as u32);
        input_touch.add_keycode(InputButton::ButtonUp as u32);
        input_touch.add_keycode(InputButton::ButtonDown as u32);
        input_touch.add_keycode(InputButton::ButtonLeft as u32);
        input_touch.add_keycode(InputButton::ButtonRight as u32);
        input_touch.add_keycode(InputButton::ButtonEnter as u32);
        input_touch.add_keycode(InputButton::ButtonMic as u32);
        input_touch.add_keycode(InputButton::ButtonPlayPause as u32);
        input_touch.add_keycode(InputButton::ButtonNext as u32);
        input_touch.add_keycode(InputButton::ButtonPrev as u32);
        input_touch.add_keycode(InputButton::ButtonMusic as u32);
        input_touch.add_keycode(InputButton::ButtonScroll as u32);
        input_touch.add_keycode(InputButton::ButtonTel as u32);
        input_touch.add_keycode(InputButton::ButtonNavigation as u32);
        input_touch.add_keycode(InputButton::ButtonMedia as u32);
        input_touch.add_keycode(InputButton::ButtonRadio as u32);
        input_touch.add_keycode(InputButton::Button1 as u32);
        input_touch.add_keycode(InputButton::Button2 as u32);
        input_touch.add_keycode(InputButton::Button3 as u32);
        input_touch.add_keycode(InputButton::ButtonStart as u32);
        input_touch.add_keycode(InputButton::ButtonStop as u32);

        let sensor_channel = response.add_channel(SensorChannel as u32);
        let sensor_list = sensor_channel.add_sensor_channel();

        sensor_list.add_config().sensor_type = SensorType::SensorTypeDrivingStatus as u32;
        sensor_list.add_config().sensor_type = SensorType::SensorTypeNightData as u32;
        sensor_list.add_config().sensor_type = SensorType::SensorTypeLocation as u32;
        //sensor_list.add_config().sensor_type = SensorType::SensorTypeSpeed as u32;
        //sensor_list.add_config().sensor_type = SensorType::SensorTypeGear as u32;

        let video_channel = response.add_channel(VideoChannel as u32);
        let video_stream = video_channel.add_output_stream();

        video_stream.stream_type = STREAM_TYPE_VIDEO;
        video_stream.set_available_in_call(true);

        let video_config = video_stream.add_video_config();

        if self.w >= 1920 && self.h >= 1080 {
            video_config.video_resolution = 3;
        } else if self.w >= 1280 && self.h >= 720 {
            video_config.video_resolution = 2;
        } else {
            video_config.video_resolution = 1;
        }
        
        video_config.video_frame = 1;
        video_config.margin_width = 0;
        video_config.margin_height = 0;
        video_config.dpi = 160;
        video_config.additional_depth = 0;

        let audio_channel = response.add_channel(MediaAudioChannel as u32);
        let audio_stream = audio_channel.add_output_stream();

        audio_stream.stream_type = STREAM_TYPE_AUDIO;
        audio_stream.set_audio_type(AUDIO_TYPE_MEDIA);
        audio_stream.set_available_in_call(true);

        let audio_config = audio_stream.add_audio_config();
        audio_config.sample_rate = 48000;
        
        let speech_channel = response.add_channel(Audio1Channel as u32);
        let speech_stream = speech_channel.add_output_stream();
        
        speech_stream.stream_type = STREAM_TYPE_AUDIO;
        speech_stream.set_audio_type(AUDIO_TYPE_VOICE);
        
        let speech_config = speech_stream.add_audio_config();
        speech_config.sample_rate = 16000;
        speech_config.channel_count = 1;

        let mic_channel = response.add_channel(MicrophoneChannel as u32);
        let mic_stream = mic_channel.add_input_stream();

        mic_stream.stream_type = STREAM_TYPE_AUDIO;
        let mic_config = mic_stream.add_audio_config();

        mic_config.sample_rate = 16000;
        mic_config.bit_depth = 16;
        mic_config.channel_count = 1;

        let media_notification = response.add_channel(PhoneStatusChannel as u32);
        media_notification.add_empty_channel(9);

        response.add_channel(NotificationChannel as u32);

        let nav_channel = response.add_channel(NavigationChannel as u32);
        let nav_service = nav_channel.add_navigation_service();
        
        nav_service.minimum_interval = 750;
        
        //println!("Message: {:X?}", self.current_data);
        println!("Response: {:X?}", response.write_to_bytes());
        self.write_message(true, chan as u8, response, ControlMessage::ControlMessageServiceDiscoveryResponse as u16, Duration::from_millis(2000), false);

        self.had_sdr = true;
    }

    //Handle an audio focus request.
    fn handle_audio_focus_request(&mut self, channel: u8, req: AudioFocusRequest) {
        let mut response = AudioFocusResponse::new();

        if req.focus_type == 4 { //Release.
            response.focus_type = 3;
        } else {
            response.focus_type = 1;
        }

        self.write_message(true, channel, response, ProtocolMessage::ProtocolMessageAudioFocusResponse as u16, Duration::from_millis(5000), true);
    }

    //Handle a channel open request.
    fn handle_channel_open_request(&mut self, channel: u8, channel_to_open: u32) {
        let mut response = ChannelOpenResponse::new();
        response.status = 0;

        self.write_message(true, channel, response.clone(), ProtocolMessage::ProtocolMessageChannelOpenResponse as u16, Duration::from_millis(5000), true);
        if channel as u32 != channel_to_open {
            self.write_message(true, channel_to_open as u8, response, ProtocolMessage::ProtocolMessageChannelOpenResponse as u16, Duration::from_millis(5000), true);
        }

        if channel == ServiceChannels::SensorChannel as u8 || channel_to_open == ServiceChannels::SensorChannel as u32 {
            let mut sensor_msg = SensorMessage::new();
            let night_sensor = sensor_msg.add_event_night();

            night_sensor.night_mode = false;
            match self.context.try_lock() {
                Ok(context) => {
                    night_sensor.night_mode = context.headlights_on;
                }
                Err(_) => {
                    println!("Channel Open Request: Context Locked.")
                }
            }

            let status_msg = sensor_msg.add_event_status();
            status_msg.status = 0;

            self.write_message(true, SensorChannel as u8, sensor_msg, 0x8003, Duration::from_millis(5000), true);
        }
    }

    //Handle a binding request.
    fn handle_binding_request(&mut self, channel: u8) {
        let mut response = BindingResponse::new();
        response.status = 0;

        self.write_message(true, channel, response, InputChannelMessageBindingResponse as u16, Duration::from_millis(5000), true);
    }

    //Handle a sensor start request.
    fn handle_sensor_start_request(&mut self, channel: u8) {
        let mut response = SensorStartResponse::new();
        response.status = 0;

        self.write_message(true, channel, response, SensorChannelMessageStartResponse as u16, Duration::from_millis(5000), true);
    }

    //Handle a media setup request.
    fn handle_media_setup_request(&mut self, channel: u8) {
        let mut response = MediaSetupResponse::new();
        response.status = 2;
        response.max_unacked = 1;
        response.add_config(0);

        self.write_message(true, channel, response, MediaChannelMessageSetupResponse as u16, Duration::from_millis(5000), true);

        if !self.first_video && channel == VideoChannel as u8 {
            self.first_video = true;

            let mut video_focus_gained = VideoFocus::new();
            video_focus_gained.mode = true;
            video_focus_gained.unrequested = true;

            self.write_message(true, VideoChannel as u8, video_focus_gained, MediaChannelMessageVideoFocus as u16, Duration::from_millis(5000), true);

            let mut sensor_message = SensorMessage::new();
            let location_message = sensor_message.add_event_location();

            location_message.set_speed(0);

            self.write_message(true, SensorChannel as u8, sensor_message, SensorChannelMessageEvent as u16, Duration::from_millis(5000), true);
        }
    }

    //Send a media acknowledgement.
    fn send_media_ack(&mut self, channel: u8) {
        let mut ack_msg = MediaAck::new();
        ack_msg.session = self.channel_session[channel as usize];
        ack_msg.value = 1;

        self.write_message(true, channel, ack_msg, MediaChannelMessage::MediaChannelMessageAck as u16, Duration::from_millis(5000), true);
    }
}