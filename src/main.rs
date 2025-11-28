use std::path::PathBuf;
use std::process::{Command, Stdio};

use clap::Parser;
use rpassword::prompt_password;

#[cfg(not(target_os = "macos"))]
fn main() {
    eprintln!("This tool only works on macOS because it shells out to hdiutil.");
    std::process::exit(1);
}

#[cfg(target_os = "macos")]
fn main() {
    if let Err(err) = run() {
        eprintln!("Error: {err}");
        std::process::exit(1);
    }
}

#[cfg(target_os = "macos")]
#[derive(Parser, Debug)]
#[command(
    name = "dmg-util",
    about = "Create an encrypted APFS DMG image by wrapping macOS hdiutil."
)]
struct DmgArgs {
    /// Size of the DMG (e.g. 100m, 1g)
    #[arg(short, long, default_value = "100m")]
    size: String,

    /// Volume name inside the DMG
    #[arg(short = 'n', long, default_value = "MyVolume")]
    volume_name: String,

    /// Output DMG path
    #[arg(short, long, default_value = "myvolume.dmg")]
    output: PathBuf,

    /// Encryption algorithm to use with hdiutil
    #[arg(short = 'e', long, default_value = "AES-256")]
    encryption: String,

    /// File system to format the DMG with
    #[arg(short = 'f', long, default_value = "APFS")]
    filesystem: String,

    /// Disk image type, defaults to UDIF
    #[arg(short = 't', long, default_value = "UDIF")]
    image_type: String,
}

#[cfg(target_os = "macos")]
fn run() -> Result<(), Box<dyn std::error::Error>> {
    let args = DmgArgs::parse();

    let passphrase = prompt_password("Enter passphrase: ")?;

    let mut command = Command::new("hdiutil");
    let mut cmd_args: Vec<String> = vec![
        "create".into(),
        "-size".into(),
        args.size.clone(),
        "-fs".into(),
        args.filesystem.clone(),
        "-type".into(),
        args.image_type.clone(),
        "-encryption".into(),
        args.encryption.clone(),
        "-volname".into(),
        args.volume_name.clone(),
    ];

    // Always pass via argv per request (note: visible to other processes).
    cmd_args.push("-passphrase".into());
    cmd_args.push(passphrase.clone());

    cmd_args.push(args.output.display().to_string());

    // Log the command without exposing the passphrase contents.
    let rendered = cmd_args
        .iter()
        .map(|s| redact_arg(s, &passphrase))
        .collect::<Vec<_>>()
        .join(" ");
    println!("Running: hdiutil {rendered}");

    command.args(&cmd_args);

    // Let hdiutil stream output directly to the terminal for progress/errors.
    command.stdout(Stdio::inherit());
    command.stderr(Stdio::inherit());

    let mut child = command.spawn()?;

    let status = child.wait()?;
    if !status.success() {
        return Err(format!("hdiutil exited with status {status}").into());
    }

    Ok(())
}

#[cfg(target_os = "macos")]
fn redact_arg(arg: &str, passphrase: &str) -> String {
    let mut display = arg.to_string();
    if arg == passphrase {
        display = "******".into();
    }
    if display.contains([' ', '"', '\'']) {
        format!("{display:?}")
    } else {
        display
    }
}
