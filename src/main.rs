use std::{error::Error, io::Write, process::Command};

use rayon::prelude::*;

fn main() {
    match run() {
        Ok(_) => {}
        Err(err) => {
            eprintln!("{}", err);
        }
    }
}

fn sys_and_commands() -> Vec<(&'static str, Vec<Vec<&'static str>>)> {
    vec![
        (
            "inanna",
            vec![
                vec!["sudo", "inxi", "-v", "7", "-c", "0", "-z"],
                vec!["sudo", "lscpu"],
                vec!["sudo", "lspci", "-v", "-v", "-v"],
                vec!["sudo", "lsipc"],
                vec!["sudo", "lsmem"],
                vec!["sudo", "lsusb", "-v", "-v"],
                vec!["sudo", "lsmod"],
            ],
        ),
        (
            "ki",
            vec![
                vec![
                    "ssh",
                    "root@ki",
                    "perl",
                    "/root/inxi",
                    "-v",
                    "7",
                    "-c",
                    "0",
                    "-z",
                ],
                vec!["ssh", "root@ki", "vnstat"],
                vec!["ssh", "root@ki", "dmesg"],
            ],
        ),
        (
            "ur",
            vec![
                vec![
                    "ssh",
                    "ur",
                    "sudo",
                    "/usr/local/bin/inxi",
                    "-v",
                    "7",
                    "-c",
                    "0",
                    "-z",
                ],
                vec![
                    "ssh",
                    "ur",
                    "sudo",
                    "/usr/local/sbin/smartctl",
                    "-a",
                    "/dev/ada0",
                ],
                vec![
                    "ssh",
                    "ur",
                    "sudo",
                    "/usr/local/sbin/smartctl",
                    "-a",
                    "/dev/ada1",
                ],
                vec![
                    "ssh",
                    "ur",
                    "sudo",
                    "/usr/local/sbin/smartctl",
                    "-a",
                    "/dev/ada2",
                ],
                vec![
                    "ssh",
                    "ur",
                    "sudo",
                    "/usr/local/sbin/smartctl",
                    "-a",
                    "/dev/ada3",
                ],
                vec![
                    "ssh",
                    "ur",
                    "sudo",
                    "lsusb",
                    "-v",
                ],
                vec![
                    "ssh",
                    "ur",
                    "sudo",
                    "dmesg",
                ],
            ],
        ),
        (
            "media-odroid",
            vec![
                vec!["ssh", "root@media-odroid", "cat", "/etc/motd"],
                vec!["ssh", "root@media-odroid", "uname", "-a"],
                vec!["ssh", "root@media-odroid", "free", "-m"],
                vec!["ssh", "root@media-odroid", "aplay", "-L"],
            ],
        ),
        (
            "vps",
            vec![vec!["ssh", "vps", "inxi", "-v", "7", "-c", "0", "-z"]],
        ),
    ]
}

fn run() -> Result<(), Box<dyn Error>> {
    let cwd = std::env::current_dir()?;

    let scs = sys_and_commands();
    scs.par_iter().for_each(|sc| {
        let mut sys_cwd = cwd.clone();
        let ts = chrono::offset::Utc::now();
        let ts_str = ts.format("%Y-%m-%d").to_string();

        sys_cwd.extend(["..", "..", "content", "systems", sc.0]);

        let _ = std::fs::create_dir(sys_cwd.clone());

        sys_cwd.extend(["index.md"]);
        let mut f = std::fs::File::create(sys_cwd).unwrap();

        let hdr = format!(
            r#"+++
title = "{}"
description = "{} System Information"
date = {}

[taxonomies]
categories = ["systems"]
+++"#,
            sc.0, sc.0, ts_str
        );

        f.write_all(&hdr.into_bytes()).unwrap();

        let now_str = chrono::offset::Utc::now()
            .format("\n\ngenerated %Y-%m-%d %H:%M")
            .to_string();
        f.write_all(&now_str.into_bytes()).unwrap();

        for c in &sc.1 {
            let mut command = Command::new(c[0]);
            for arg in &c[1..] {
                command.arg(arg);
            }

            match command.output() {
                Ok(out) => {
                    let command_str = c.join(" ");
                    let cmd_hdr = format!("\n## Command '{}'", command_str);
                    f.write_all(&cmd_hdr.into_bytes()).unwrap();

                    f.write_all("\n```\n".as_bytes()).unwrap();
                    f.write_all(&out.stdout).unwrap();
                    f.write_all("```\n".as_bytes()).unwrap();

                    f.flush().unwrap();
                }
                Err(err) => eprintln!("{}", err),
            }
        }
    });

    Ok(())
}
