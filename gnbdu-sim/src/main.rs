mod ue;
use anyhow::Result;
use hex;
use mocks::MockDu;
use slog::info;
use std::{
    io::{Read, Write},
    process::{ChildStdin, ChildStdout, Command, Stdio},
};
use ue::Ue;

#[async_std::main]
async fn main() -> Result<()> {
    let logger = common::logging::init_terminal_logging();
    let (du, stop_source) = MockDu::new(&logger).await;

    du.establish_connection("127.0.0.1:38472".to_string())
        .await?;

    let mut child = Command::new("ue-sim")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to run 'ue-sim' - is it in the path?");
    let mut stdout = child.stdout.take().unwrap();
    let mut stdin = child.stdin.take().unwrap();

    info!(&logger, "Get NAS registration from UE");
    let nas_message = recv_nas(&mut stdout);
    info!(&logger, "Yay");

    du.perform_rrc_setup(nas_message, &logger).await?;

    let nas_authentication_request = du.receive_nas().await?;
    send_nas(&mut stdin, nas_authentication_request);

    drop(stop_source);

    Ok(())
}

fn recv_nas<T: Read>(reader: &mut T) -> Vec<u8> {
    let mut s = String::new();
    let mut buf = [0];
    loop {
        let _bytes_read = reader.read(&mut buf);
        if buf[0] == b'\n' {
            break;
        }
        s.push(buf[0] as char);
    }
    hex::decode(s.clone()).expect(&format!("String {} didn't decode to hex", s))
}

fn send_nas<T: Write>(writer: &mut T, nas_bytes: Vec<u8>) {
    let hex_string = hex::encode(nas_bytes);
    writer
        .write_all(hex_string.as_bytes())
        .expect("Lost connection to UE SIM")
}
