use core::time::Duration;
use std::{process::Command, thread};

const CHIP: &str = "LPC845M301JBD48";
const TARGET: &str = "1fc9:0132:0F00A019";
const ASSISTANT: &str = "1fc9:0132:0F01502C";

#[test]
fn gpio() {
    build();

    // TODO refactor + add synchronization
    // run target
    let mut c = Command::new("probe-run")
        .args(&[
            "--chip",
            CHIP,
            "--probe",
            TARGET,
            "--defmt",
            "target/thumbv6m-none-eabi/debug/output",
        ])
        .current_dir("../firmware")
        .spawn()
        .unwrap();

    // let it flash
    thread::sleep(Duration::from_secs(5));

    // run assistant
    assert!(Command::new("probe-run")
        .args(&[
            "--chip",
            CHIP,
            "--probe",
            ASSISTANT,
            "--defmt",
            "target/thumbv6m-none-eabi/debug/input",
        ])
        .current_dir("../firmware")
        .status()
        .unwrap()
        .success());

    // terminate target process
    c.kill().unwrap();
}

// TODO run only once
fn build() {
    // build firmware
    assert!(Command::new("cargo")
        .arg("build")
        .current_dir("../firmware")
        .status()
        .unwrap()
        .success());
}
