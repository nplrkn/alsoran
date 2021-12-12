use async_std::channel::Sender;
use async_std::prelude::*;
use signal_hook::consts::signal::*;
use signal_hook_async_std::Signals;
use slog::info;
use std::io::Error;
use worker::logging;

#[async_std::main]
async fn main() -> Result<(), Error> {
    let root_logger = logging::init();

    // Set up signal catching task
    // TODO factor out to separate fn module?
    let signals = Signals::new(&[SIGHUP, SIGTERM, SIGINT, SIGQUIT]).unwrap();
    let handle = signals.handle();
    let (sig_sender, sig_receiver) = async_channel::unbounded();
    let signals_task = async_std::task::spawn(handle_signals(signals, sig_sender));

    worker::run(root_logger.clone()).await;

    // Block until we receive a signal.
    let signal = sig_receiver.recv().await.unwrap();
    info!(root_logger, "Caught signal {} - terminate", signal);

    // Terminate the signal stream.
    handle.close();
    signals_task.await;
    Ok(())
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
