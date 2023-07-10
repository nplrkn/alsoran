//! signal - a way to exit from main() when Ctrl-C is pressed (or some other signal is received).

use anyhow::Result;
use async_std::channel::Sender;
use async_std::prelude::*;
use signal_hook::consts::signal::*;
use signal_hook_async_std::Signals;

pub async fn wait_for_signal() -> Result<i32> {
    let signals = Signals::new([SIGHUP, SIGTERM, SIGINT, SIGQUIT])?;
    let handle = signals.handle();
    let (sig_sender, sig_receiver) = async_channel::unbounded();
    let signals_task = async_std::task::spawn(handle_signals(signals, sig_sender));
    let signal = sig_receiver.recv().await;
    handle.close();
    signals_task.await;
    Ok(signal?)
}

async fn handle_signals(signals: Signals, sig_sender: Sender<i32>) {
    let mut signals = signals.fuse();
    while let Some(signal) = signals.next().await {
        match signal {
            SIGHUP => {
                // Reload configuration
                // Reopen the log file
            }
            SIGTERM | SIGINT | SIGQUIT => {
                // Shutdown the system;
                let _ = sig_sender.send(signal).await;
            }
            _ => unreachable!(),
        }
    }
}
