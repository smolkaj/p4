//! End-to-end tests using grpcurl and trycmd.
//!
//! Tests execute grpcurl commands from markdown files and verify responses.

use std::process::{Child, Command};
use std::time::Duration;

fn start_server() -> Child {
    // Kill any existing server instances.
    Command::new("pkill")
        .args(&["-f", "p4runtime-server"])
        .output()
        .expect("Failed to kill existing server instances via `pkill -f p4runtime-server`");

    // Build the server first so that running it will be predictably fast.
    Command::new("cargo")
        .args(&["build", "--quiet", "--bin", "p4runtime-server"])
        .output()
        .expect("Failed to build server via `cargo build --bin p4runtime-server`");

    // Run the server in the background.
    let server = Command::new("cargo")
        .args(&["run", "--bin", "p4runtime-server", "--quiet"])
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .spawn()
        .expect("Failed to start server via `cargo run --bin p4runtime-server`");

    // Wait for server to be ready - trycmd needs time for server to bind
    std::thread::sleep(Duration::from_millis(1000));
    server
}

fn find_grpcurl() -> std::path::PathBuf {
    which::which("grpcurl").expect(
        "grpcurl not found in PATH. You can install it as follows:
- macOS: brew install grpcurl
- Ubuntu/Linux: snap install grpcurl
- Using Go: go install github.com/fullstorydev/grpcurl/cmd/grpcurl@latest
- Or download from: https://github.com/fullstorydev/grpcurl/releases
",
    )
}

#[test]
fn test_markdown_tests() {
    let grpcurl = find_grpcurl();
    for test_case in ["tests/p4runtime_test.md"] {
        let _server = start_server();
        trycmd::TestCases::new()
            .register_bin("grpcurl", grpcurl.clone())
            .case(test_case)
            .run();
    }
}
