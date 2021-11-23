use std::process::Command;

pub fn get_value(name: &str) -> String {
    let message = format!("#{{{}}}", name);

    get_command_result(&["display-message", "-p", &message])
}

// TODO: Resolve every single unwrap
pub fn get_option(name: &str) -> String {
    get_command_result(&["show-option", "-w", "-v", name])
}

// TODO: Resolve unwraps and reurn a result
pub fn set_option(name: &str, value: &str) {
    Command::new("tmux")
        .args(&["set-option", "-w", name, value])
        .spawn()
        .unwrap();
}

// TODO: Resolve every single unwrap
pub fn get_command_result(args: &[&str]) -> String {
    let output = Command::new("tmux").args(args).output().unwrap();

    let value = std::str::from_utf8(&output.stdout)
        .unwrap()
        .strip_suffix("\n")
        .unwrap();

    value.to_string()
}
