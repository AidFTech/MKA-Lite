use std::io::Write;
use std::process::Command;
use std::process::Stdio;
use std::process::Child;

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

pub struct FfAudio {
    process: Child
}

impl FfAudio {
    pub fn new() -> Result<FfAudio, String> {
        let mut ff_cmd = Command::new("ffplay");
        ff_cmd.arg("-f");
        ff_cmd.arg("s16le");
        ff_cmd.arg("-ac");
        ff_cmd.arg("2");
        ff_cmd.arg("-ar");
        ff_cmd.arg("44100");
        ff_cmd.arg("-nodisp");
        ff_cmd.arg("--enable-libpulse");
        ff_cmd.arg("-");
        match ff_cmd.stdin(Stdio::piped()).spawn() {
            Err(e) => return Err(format!("Could not start audio FFPlay: {} ", e)),
            Ok(process) => return Ok(FfAudio { process }),
        }
    }

    pub fn send_audio(&mut self, data: &[u8]) {
        let mut child_stdin = self.process.stdin.as_ref().unwrap();
        let _ = child_stdin.write(&data);
    }
}