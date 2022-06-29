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
