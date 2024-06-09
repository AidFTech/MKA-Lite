import subprocess

class AudioDecoder(object):

    def __init__(self):
        self.decoder = subprocess.Popen(
            [
                "play",
                "-t",
                "raw",
                "-b",
                "16",
                "-c",
                "2",
                "--endian",
                "little",
                "-e",
                "signed",
                "-r",
                "44100",
                "-q",
                "-"
            ],
            stdin=subprocess.PIPE,
        )

    def start(self):
        pass

    def stop(self):
        if self.decoder is not None:
            self.decoder.terminate()

    def sendAudioData(self, data: bytes):
        if self.decoder is not None:
            self.decoder.stdin.write(data)

    def running(self) -> bool:
        return self.decoder is not None
