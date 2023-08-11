use std::{env, path::Path, process::Command};

// TODO: switch to cargo binary dependency when available
// see https://rust-lang.github.io/rfcs/3028-cargo-binary-dependencies.html
const CIRCOM_PATH:&str = "language/circom";
const SNARKJS_PATH:&str = "language/snarkjs";
const RAPIDSNARK_PATH:&str = "language/rapidsnark";
const PREFIX_PATH:&str = "../";
const GIT:&str = "/.git";
pub fn main() {
    println!("cargo:rerun-if-changed=../language/circom/");
    println!("cargo:rerun-if-changed=../language/snarkjs/build/");
    println!("cargo:rerun-if-changed=../language/rapidsnark/build/");
    let cargo = env::var("CARGO").unwrap();

    // initialize and update git submodules
    if !(Path::new(format!("{}, {}", CIRCOM_PATH, GIT).as_str()).exists() &&
        Path::new(format!("{}, {}", SNARKJS_PATH, GIT).as_str()).exists() &&
        Path::new(format!("{}, {}", RAPIDSNARK_PATH, GIT).as_str()).exists()) {
        assert!(
            Command::new("git")
                .arg("submodule")
                .arg("update")
                .arg("--init")
                .arg("--recursive")
                .status()
                .unwrap()
                .success(),
            "Git submodule initialization failed."
        );
    }

    // build circom
    assert!(
        Command::new(&cargo)
            .arg("build")
            .arg("--release")
            .current_dir(Path::new(PREFIX_PATH).join(CIRCOM_PATH))
            .status()
            .unwrap()
            .success(),
        "Circom build failed."
    );

    // npm clean install
    assert!(
        Command::new("npm")
            .arg("ci")
            .current_dir(Path::new(PREFIX_PATH).join(SNARKJS_PATH))
            .status()
            .unwrap()
            .success(),
        "Npm SnarkJS clean install failed."
    );

    // build rapidsnark
    assert!(
        Command::new("npm")
            .arg("ci")
            .current_dir(Path::new(PREFIX_PATH).join(RAPIDSNARK_PATH))
            .status()
            .unwrap()
            .success(),
        "Npm RapidSnark clean install failed."
    );
    assert!(
        Command::new("npx")
            .arg("task")
            .arg("createFieldSources")
            .current_dir(Path::new(PREFIX_PATH).join(RAPIDSNARK_PATH))
            .status()
            .unwrap()
            .success(),
        "Npx RapidSnark createFieldSources failed."
    );
    assert!(
        Command::new("npx")
            .arg("task")
            .arg("buildProver")
            .current_dir(Path::new(PREFIX_PATH).join(RAPIDSNARK_PATH))
            .status()
            .unwrap()
            .success(),
        "Npx RapidSnark buildProver failed."
    );
}
