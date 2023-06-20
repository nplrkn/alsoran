//! ue - a simulated UE that talks Nas and implements a pre-canned setup sequence

use hex;
use mocks::{DuUeContext, MockDu};
use slog::{debug, Logger};
use std::{
    io::{Read, Write},
    process::{ChildStdin, ChildStdout, Command, Stdio},
};

pub struct Ue {
    pub id: u32,
    stdin: ChildStdin,
    stdout: ChildStdout,
    pub du_context: DuUeContext,
}

impl Ue {
    pub async fn new(id: u32, du: &MockDu) -> Self {
        // Spawn the ue-sim process.
        let mut child = Command::new("ue-sim")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .expect("Failed to run 'ue-sim' - is it in the path?");
        let stdout = child.stdout.take().expect("Couldn't take stdout");
        let stdin = child.stdin.take().expect("Couldn't take stdin");

        Ue {
            id,
            stdout,
            stdin,
            du_context: du.new_ue_context(id, "127.0.0.1").await.unwrap(),
        }
    }

    pub fn build_next_nas_message(&mut self) -> Vec<u8> {
        let mut s = String::new();
        let mut buf = [0];
        loop {
            let _bytes_read = self.stdout.read(&mut buf);
            if buf[0] == b'\n' {
                break;
            }
            s.push(buf[0] as char);
        }
        hex::decode(s.clone()).expect(&format!("String '{}' didn't decode to hex", s))
    }

    pub fn handle_nas(&mut self, nas_bytes: Vec<u8>, logger: &Logger) {
        let hex_string = hex::encode(nas_bytes) + "\n";
        debug!(&logger, "Send to ue-sim {}", hex_string);
        self.stdin
            .write_all(hex_string.as_bytes())
            .expect("Lost connection to UE SIM")
    }
}
