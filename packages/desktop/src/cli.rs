use std::{io, process};

use anyhow::Result;
use clap::{Parser, Subcommand};
use windows::Win32::UI::Input::KeyboardAndMouse::VK_RETURN;
use windows::Win32::{
  Foundation::{HWND, LPARAM, WPARAM},
  System::Console::{FreeConsole, GetConsoleWindow},
};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None, arg_required_else_help = true)]
pub struct Cli {
  #[command(subcommand)]
  pub command: CliCommand,
}

#[derive(Subcommand, Debug)]
pub enum CliCommand {
  /// Open a window by its ID (eg. 'zebar open bar').
  Open {
    /// ID of the window to open (eg. 'bar').
    window_id: String,

    /// Arguments to pass to the window.
    ///
    /// These become available via the 'self' provider.
    #[clap(short, long, num_args = 1.., value_parser=parse_open_args)]
    args: Option<Vec<(String, String)>>,
  },
  /// Output available monitors.
  Monitors {
    /// Use ASCII NUL character (character code 0) instead of newlines
    /// for delimiting monitors.
    ///
    /// Useful for piping to 'xargs -0'.
    #[clap(short, long)]
    print0: bool,
  },
}

/// Print to `stdout`/`stderror` and exit the process.
pub fn print_and_exit(output: Result<String>) {
  use windows::Win32::UI::WindowsAndMessaging::*;
  match output {
    Ok(output) => {
      print!("{}", output);
      // unsafe {
      //   let cw: HWND = GetConsoleWindow();
      //   println!("cw: {:?}", cw);
      //   // SendMessageW(cw, WM_KEYDOWN, 0x0D, 0);
      //   // SendMessageW(cw, WM_KEYUP, 0x0D, 0);
      //   // let x = VK_ENTER;
      //   let xx =
      //     SendMessageW(cw, WM_CHAR, WPARAM(0x0D as _), LPARAM(0 as _));
      //   println!("xx: {:?}", xx);
      //   println!("xx: {:?}", xx.0);
      //   // SendMessageW(cw, WM_KEYUP, WPARAM(0x0D as _), LPARAM(0 as _));

      //   // let _ = SendMessageW(cw, 0x0102, 0x0D.into(), 0x0.into());
      //   let _ = FreeConsole();
      // };
      process::exit(0);
    }
    Err(err) => {
      eprintln!("Error: {}", err);
      // unsafe {
      //   let cw = GetConsoleWindow();
      //   SendMessageW(cw, WM_CHAR, WPARAM(0x0D as _), LPARAM(0 as _));
      //   // SendMessageW(cw, WM_KEYUP, WPARAM(0x0D as _), LPARAM(0 as _));
      //   let _ = FreeConsole();
      // };
      process::exit(1);
    }
  }
}

/// Parses arguments passed to the `open` CLI command into a string tuple.
fn parse_open_args(input: &str) -> Result<(String, String), String> {
  let mut parts = input.split('=');

  match (parts.next(), parts.next()) {
    (Some(key), Some(value)) => Ok((key.into(), value.into())),
    _ => Err("Arguments must be of format KEY1=VAL1".into()),
  }
}
