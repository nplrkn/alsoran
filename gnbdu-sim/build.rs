//! This build script builds and installs the NAS processing helper executable.  This is in Go,
//! scavenged from the Free5GC project - because no open source Rust library apparently exists
//! and this seemed to be the quickest and easiest kludge to get the call flow we need.
use std::process::Command;

fn main() {
    let output = Command::new("sh")
        .arg("-c")
        .arg("go install")
        .current_dir("./ue-sim")
        .output()
        .unwrap();

    if output.stderr.len() != 0 {
        println!("{}", String::from_utf8_lossy(&output.stderr));
        std::process::exit(1);
    }

    println!("cargo:rerun-if-changed=ue-sim");
}
