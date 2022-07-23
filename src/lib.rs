use std::process::Command;

pub fn command_description(command_name: &str) -> String {
    // gets a one-line description from the manpages about selected command

    // capture the output from whatis
    let output = Command::new("whatis")
        .arg(format!("{command_name}"))
        .output()
        .expect("whatis command failed to start");
    String::from_utf8_lossy(&output.stdout).to_string()
}