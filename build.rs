use std::process::Command;

fn main() {
    build_ui_files();
}

fn build_ui_files() {
    if std::env::var("BUILD_UI").is_err() {
        return;
    }

    let ui_dir = match std::env::current_dir() {
        Ok(mut d) => {
            d.push("ui");
            d
        },
        Err(e) => panic!("Could not get current directory: {}", e),
    };

    let yarn_command = match std::env::var("YARN_COMMAND") {
        Ok(c) => c,
        _ => "yarn".to_string(),
    };

    let ui_build_status = Command::new(yarn_command)
        .arg("build")
        .current_dir(ui_dir)
        .status()
        .expect("Failed to build UI");
    if !ui_build_status.success() {
        panic!("Could not build UI");
    }
}
