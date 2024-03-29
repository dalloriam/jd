use clap::Parser;

mod cli;

use anyhow::Result;

/// This should be called before calling any cli method or printing any output.
/// See: https://github.com/rust-lang/rust/issues/46016
pub fn reset_signal_pipe_handler() -> Result<()> {
    #[cfg(target_family = "unix")]
    {
        use nix::sys::signal;

        unsafe {
            signal::signal(signal::Signal::SIGPIPE, signal::SigHandler::SigDfl)?;
        }
    }

    Ok(())
}

fn main() {
    reset_signal_pipe_handler().unwrap();

    if let Err(e) = cli::Root::parse().run() {
        eprintln!("{}", e);
    }
}
