use std::io::Write;
use std::process::Command;
use std::process::Stdio;
use std::process::Child;

pub struct Mpv {
    process: Child
}

impl Mpv {
    pub fn new(h: u16, w: u16) -> Result<Mpv, String> {
        let mut mpv_cmd = Command::new("mpv");
        mpv_cmd.arg(format!("--geometry={}x{}", h, w));
        mpv_cmd.arg("--hwdec=rpi");
        mpv_cmd.arg("--demuxer-rawvideo-fps=60");
        //mpv_cmd.arg("--untimed");
        mpv_cmd.arg("--fps=60");
        mpv_cmd.arg("-");
        match mpv_cmd.stdin(Stdio::piped()).spawn() {
            Err(e) => return Err(format!("Could not start Mpv: {} ", e)),
            Ok(process) => return Ok(Mpv { process }),
        }
    }

    pub fn send_video(&mut self, data: &[u8]) {
        let mut child_stdin = self.process.stdin.as_ref().unwrap();
        let _ = child_stdin.write(&data);
    }
}
