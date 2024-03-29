#![deny(warnings)]

use std::process::{Stdio, Command, exit};
use std::str::{self};
use std::path::{PathBuf, Path};
use std::thread::{sleep};
use std::time::{Duration};
use std::env::{args_os};

fn main() {
    if let Err(e) = run() {
        eprintln!("{}", e);
        exit(1);
    }
}

fn execute<N: AsRef<str>>(max_retry_count: u32, name: impl FnOnce() -> N, command: &mut Command) -> Result<(), String> {
    let mut retry_count = 0;
    let status = loop {
        let status = command.stdin(Stdio::null()).status().map_err(|e| e.to_string())?;
        if status.success() { return Ok(()); }
        if status.code() != Some(1) || retry_count == max_retry_count { break status; }
        retry_count += 1;
        sleep(Duration::from_millis(1));
    };
    Err(format!("{} finished with non-zero {}\n.", name().as_ref(), status))
}

fn run() -> Result<(), String> {
    let mut args = args_os();
    args.next().expect("arg0");
    let config = args.next();
    if config.is_some() {
        if let Some(extra_arg) = args.next() {
            return Err(format!("Extra argument `{}`. Usage: wi-fi [WPA_SUPPLICANT_CONF]", extra_arg.to_string_lossy()));
        }
    }
    execute(0, || "ifconfig wlan0 down", Command::new("/sbin/ifconfig").arg("wlan0").arg("down"))?;
    while Path::new("/var/run/wpa_supplicant/wlan0").exists() {
        execute(
            0,
            || "wpa_cli -i wlan0 terminate",
            Command::new("/usr/sbin/wpa_cli").arg("-i").arg("wlan0").arg("terminate")
        )?;
        sleep(Duration::from_millis(100));
    }
    if let Some(config) = config {
        let config = PathBuf::from(config);
        execute(
            0, 
            || format!("wpa_supplicant -B -C /var/run/wpa_supplicant -i wlan0 -c {}", config.display()),
            Command::new("/usr/sbin/wpa_supplicant")
                .arg("-B")
                .arg("-C")
                .arg("/var/run/wpa_supplicant")
                .arg("-i")
                .arg("wlan0")
                .arg("-c")
                .arg(&config)
        )?;
        execute(10, || "dhclient -b wlan0", Command::new("/sbin/dhclient").arg("-b").arg("wlan0"))?;
    }
    Ok(())
}
