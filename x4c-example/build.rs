use std::process::Command;

fn main() {
    println!("cargo:rerun-if-changed=../p4/examples/codegen/router.p4");
    let output = Command::new("../target/debug/x4c")
        .arg("../p4/examples/codegen/router.p4")
        .arg("-o")
        .arg("src/router.rs")
        .output()
        .expect("Failed to execute x4c");

    if !output.status.success() {
        panic!("x4c failed: {:?}", output);
    }
}
