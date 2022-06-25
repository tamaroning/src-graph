use std::{env, path::PathBuf, process};

const USAGE: &str = r#"Usage: cargo src-graph"#;

fn driver_path() -> PathBuf {
    let mut path = env::current_exe()
        .expect("current executable path invalid")
        .with_file_name("src-graph-driver");

    if cfg!(windows) {
        path.set_extension("exe");
    }

    path
}

fn main() {
    if env::args().len() > 2 {
        println!("{USAGE}");
        return;
    }

    let mut cmd = process::Command::new("cargo");
    let cmd = cmd
        .env("RUSTC_WORKSPACE_WRAPPER", driver_path())
        .arg("check");

    let exit_status = cmd
        .spawn()
        .expect("could not run cargo")
        .wait()
        .expect("failed to wait for cargo?");

    process::exit(exit_status.code().unwrap_or(-1))
}
