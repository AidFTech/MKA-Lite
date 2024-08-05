use core::str;
use std::io::Cursor;
use std::io::Write;
use std::process::Command;
use std::process::Stdio;
use std::process::Child;

use rodio::Decoder as AudioDecoder;
use rodio::OutputStream;
use rodio::OutputStreamHandle;
use rodio::Sink;

pub struct MpvVideo {
    process: Child,
}

impl MpvVideo {
    pub fn new(width: u16, height: u16) -> Result<MpvVideo, String> {
        let mut mpv_cmd = Command::new("mpv");
        //mpv_cmd.arg(format!("--geometry={}x{}", width, height));
        mpv_cmd.arg("--hwdec=rpi");
        mpv_cmd.arg("--demuxer-rawvideo-fps=60");
        mpv_cmd.arg("--untimed");
        mpv_cmd.arg("--osc=no");
        mpv_cmd.arg("--fps=60");
        mpv_cmd.arg(format!("--video-aspect-override={}/{}", width, height));
        mpv_cmd.arg("--input-ipc-server=/tmp/mka_cmd.sock");
        mpv_cmd.arg("-");
        match mpv_cmd.stdin(Stdio::piped()).spawn() {
            Err(e) => return Err(format!("Could not start video Mpv: {} ", e)),
            Ok(process) => {
                return Ok(MpvVideo { process: process });
            }
        }
    }

    pub fn send_video(&mut self, data: &[u8]) {
        let mut child_stdin = self.process.stdin.as_ref().unwrap();
        let _ = child_stdin.write(&data);
    }
    
    pub fn start(&mut self) {
        //Start video playback.
    }
    
    pub fn stop(&mut self) {
        //Stop video playback.
        self.set_minimize(true);
    }
    
    pub fn set_minimize(&mut self, minimize: bool) {
        let pid = self.process.id();
        let wid_cmd = Command::new("xdotool").arg("search").arg("--pid").arg(format!("{}", pid)).output();

        let wid_vec = match wid_cmd {
            Ok(wid) => wid.stdout,
            Err(_) => {
                return;
            }
        };

        let wid_str = match str::from_utf8(&wid_vec) {
            Ok(wid_str) => wid_str,
            Err(_) => {
                return;
            }
        };

        if minimize {
            let minimize_cmd = Command::new("xdotool").arg("windowminimize").arg(wid_str).output();
            match minimize_cmd {
                Ok(_) => {

                }
                Err(_) => {
                    return;
                }
            }
        } else {
            let minimize_cmd = Command::new("xdotool").arg("windowactivate").arg(wid_str).output();
            match minimize_cmd {
                Ok(_) => {

                }
                Err(_) => {
                    return;
                }
            }
        }
    }
}

pub struct RdAudio {
    _stream: OutputStream,
    _handler: OutputStreamHandle,
    sink: Sink,
    data: Vec<u8>,
    sample: u32,
    bits: u16,
    channels: u16,
}

impl RdAudio {
    pub fn new() -> Result<RdAudio, String> {
        /*let mut ff_cmd = Command::new("mpv");
        /*ff_cmd.arg("-f");
        ff_cmd.arg("s16le");
        ff_cmd.arg("-ac");
        ff_cmd.arg("2");
        ff_cmd.arg("-ar");
        ff_cmd.arg("48000");
        ff_cmd.arg("-nodisp");
        ff_cmd.arg("-");*/
        ff_cmd.arg("--video=no");
        ff_cmd.arg("--untimed");
        ff_cmd.arg("--demuxer-rawaudio-channels=2");
        ff_cmd.arg("--demuxer-rawaudio-format=s16le");
        ff_cmd.arg("--demuxer-rawaudio-rate=48000");
        ff_cmd.arg("-");
        match ff_cmd.stdin(Stdio::piped()).spawn() {
            Err(e) => return Err(format!("Could not start audio FFPlay: {} ", e)),
            Ok(process) => return Ok(FfAudio { process }),
        }*/

        let (stream, handler) = OutputStream::try_default().unwrap();
        let sink = Sink::try_new(&handler).unwrap();

        let data = Vec::new();
        return Ok(RdAudio{_stream: stream, _handler: handler, sink, data, sample: 48000, bits: 16, channels: 2});
    }

    pub fn send_audio(&mut self, data: &[u8]) {
        /*let mut child_stdin = self.process.stdin.as_ref().unwrap();
        let _ = child_stdin.write_all(&data);*/
        for i in 0..data.len() {
            self.data.push(data[i]);
        }

        let mut new_data = get_wav_header(self.data.len(), self.sample, self.channels, self.bits);

        for i in 0..self.data.len() {
            new_data.push(self.data[i]);
        }
        self.data.clear(); 

        let cursor = Cursor::new(new_data);
        let source = match AudioDecoder::new_wav(cursor) {
            Ok(source) => source,
            Err(err) => {
                println!("Decoder Error: {}", err);
                return;
            }
        };
        /*match self.handler.play_raw(source.convert_samples()) {
            Ok(_) => {

            }
            Err(err) => {
                println!("Play Error: {}", err);
            }
        }*/
        self.sink.append(source);
    }
    
    pub fn set_audio_profile(&mut self, sample: u32, bits: u16, channels: u16) {
        self.sample = sample;
        self.bits = bits;
        self.channels = channels;
    }
    
    pub fn get_audio_profile(&mut self) -> (u32, u16, u16) {
        return (self.sample, self.bits, self.channels);
    }
}

fn get_wav_header(len: usize, sample: u32, channels: u16, bits: u16) -> Vec<u8> {
    let mut wav_header = Vec::new();

    let riff_str = "RIFF".as_bytes();
    for i in 0..riff_str.len() {
        wav_header.push(riff_str[i]);
    }

    let full_size = ((len + 36) as u32).to_le_bytes();
    for d in full_size {
        wav_header.push(d);
    }

    let wave_str = "WAVEfmt ".as_bytes();
    for i in 0..wave_str.len() {
        wav_header.push(wave_str[i]);
    }

    let meta_size = (16 as u32).to_le_bytes();
    for d in meta_size {
        wav_header.push(d);
    }

    let pcm = (1 as u16).to_le_bytes();
    for d in pcm {
        wav_header.push(d);
    }

    let channels_le = channels.to_le_bytes();
    for d in channels_le {
        wav_header.push(d);
    }

    let sample_le = sample.to_le_bytes();
    for d in sample_le {
        wav_header.push(d);
    }

    let check1 = ((bits as u32)*(channels as u32)*sample/8).to_le_bytes();
    for d in check1 {
        wav_header.push(d);
    }

    let check2 = (bits*channels/8).to_le_bytes();
    for d in check2 {
        wav_header.push(d);
    }

    let bits_le = bits.to_le_bytes();
    for d in bits_le {
        wav_header.push(d);
    }

    let data_str = "data".as_bytes();
    for i in 0..data_str.len() {
        wav_header.push(data_str[i]);
    }

    let len_le = (len as u32).to_le_bytes();
    for d in len_le {
        wav_header.push(d);
    }

    return wav_header;
}

pub fn get_decode_type(decode_num: u32) -> (u32, u16, u16) {
    if decode_num == 1 || decode_num == 2 {
        return (44100, 16, 2);
    } else if decode_num == 3 {
        return(8000, 16, 1);
    } else if decode_num == 4 {
        return (48000, 16, 2);
    } else if decode_num == 5 {
        return(16000, 16, 1);
    } else if decode_num == 6 {
        return(24000, 16, 1);
    } else if decode_num == 7 {
        return(16000, 16, 2);
    } else {
        return (44100, 16, 2);
    }
}
