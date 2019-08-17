use std::collections::HashMap;
use std::env::current_dir;
use std::fs::File;
use std::io::{Error, Write};
use std::process::Command;
use std::thread::sleep;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

use chrono::Local;

trait IntoString {
    fn into_string(self) -> String;
}

impl IntoString for Vec<u8> {
    fn into_string(self) -> String {
        String::from_utf8_lossy(&self).trim().to_string()
    }
}

fn main() {
    println!("Starting loop!");

    let mut table: HashMap<String, u32> = HashMap::new();
    let mut filename = format!("{}.json", Local::now().to_rfc3339());
    let mut second_count = 0;

    loop {
        if let Ok(path) = get_active_window() {
            match table.get_mut(&path) {
                Some(count) => {
                    *count += 1;
                }
                None => {
                    table.insert(path, 1);
                }
            }
        }

        if second_count % 10 == 0 {
            save_to_logfile(&filename, &table);
        }

        second_count += 1;
        sleep(Duration::from_secs(1));
    }
}

fn save_to_logfile(filename: &str, table: &HashMap<String, u32>) {
    if let Ok(mut file) = File::create(filename) {
        let result = file.write_all(format!("{:?}", table).as_bytes());
        if let Err(e) = result {
            eprintln!("{}", e);
        }
    } else {
        eprintln!("Unable to create file!");
    }
}

fn get_active_window() -> Result<String, Error> {
    let id = Command::new("xdotool")
        .arg("getwindowfocus")
        .output()?
        .stdout
        .into_string();

    let pid = Command::new("xdotool")
        .arg("getwindowpid")
        .arg(id)
        .output()?
        .stdout
        .into_string();

    let command = Command::new("readlink")
        .arg("-f")
        .arg(format!("/proc/{}/exe", pid))
        .output()?
        .stdout
        .into_string();

    let command = if command.ends_with("java") {
        let jcmd = Command::new("jcmd")
            .output()?
            .stdout
            .into_string();

        let mut map = HashMap::new();
        let lines: Vec<String> = jcmd
            .split("\n")
            .map(|i| i.to_string())
            .collect();

        for line in lines {
            let parts: Vec<_> = line.split(" ").collect();
            map.insert(parts[0].to_string(), parts[1].to_string());
        }

        match map.get(&pid) {
            Some(it) => format!("java {}", it),
            None => command
        }
    } else {
        command
    };

    Ok(command)
}