use std::io::Cursor;
use std::io::Write;
use std::process::Command;
use std::process::Stdio;
use std::process::Child;

use rodio::Decoder;
use rodio::OutputStream;
use rodio::Source;

pub struct MpvVideo {
    process: Child
}

impl MpvVideo {
    pub fn new(h: u16, w: u16) -> Result<MpvVideo, String> {
        let mut mpv_cmd = Command::new("mpv");
        mpv_cmd.arg(format!("--geometry={}x{}", h, w));
        mpv_cmd.arg("--hwdec=rpi");
        mpv_cmd.arg("--demuxer-rawvideo-fps=60");
        mpv_cmd.arg("--untimed");
        mpv_cmd.arg("--osc=no");
        mpv_cmd.arg("--fps=60");
        mpv_cmd.arg("-");
        match mpv_cmd.stdin(Stdio::piped()).spawn() {
            Err(e) => return Err(format!("Could not start video Mpv: {} ", e)),
            Ok(process) => return Ok(MpvVideo { process }),
        }
    }

    pub fn send_video(&mut self, data: &[u8]) {
        let mut child_stdin = self.process.stdin.as_ref().unwrap();
        let _ = child_stdin.write(&data);
    }
}

pub struct RdAudio {
    data: Vec<u8>,
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

        let data = Vec::new();
        return Ok(RdAudio{data});
    }

    pub fn send_audio(&mut self, data: &[u8]) {
        /*let mut child_stdin = self.process.stdin.as_ref().unwrap();
        let _ = child_stdin.write_all(&data);*/
        for i in 0..data.len() {
            self.data.push(data[i]);
        }

        let mut new_data = get_wav_header(self.data.len(), 48000, 2, 16);

        for i in 0..self.data.len() {
            new_data.push(self.data[i]);
        }
        self.data.clear();

        let (_stream, handler) = OutputStream::try_default().unwrap();

        let cursor = Cursor::new(new_data);
        let source = match Decoder::new_wav(cursor) {
            Ok(source) => source,
            Err(err) => {
                println!("Decoder Error: {}", err);
                return;
            }
        };
        match handler.play_raw(source.convert_samples()) {
            Ok(_) => {

            }
            Err(err) => {
                println!("Play Error: {}", err);
            }
        }
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