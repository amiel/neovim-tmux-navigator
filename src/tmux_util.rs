use std::process::Command;

use regex::Regex;

pub fn get_value(name: &str) -> String {
    let message = format!("#{{{}}}", name);

    get_command_result(&["display-message", "-p", &message]).unwrap_or("".to_string())
}

// TODO: Resolve every single unwrap
pub fn get_option(name: &str) -> Option<String> {
    get_command_result(&["show-option", "-w", "-v", name])
}

// TODO: Resolve unwraps and reurn a result
pub fn set_option(name: &str, value: &str) {
    run(&["set-option", "-w", name, value])
}

pub fn move_direction(direction: &str) {
    run(&["select-pane", direction])
}

pub fn zoom() {
    run(&["resize-pane", "-Z"])
}

pub fn run(args: &[&str]) {
    Command::new("tmux").args(args).spawn().unwrap();
}

// TODO: Resolve every single unwrap
pub fn get_command_result(args: &[&str]) -> Option<String> {
    let output = Command::new("tmux")
        .args(args)
        .output()
        .expect("Could not run tmux command");

    let mut value = String::from_utf8_lossy(&output.stdout);
    let value = value.to_mut();

    if value.ends_with('\n') {
        value.pop();
    }

    Some(value.to_string())
}

#[derive(Debug)]
struct Window {
    width: u16,
    height: u16,
    layout: String,
    zoomed: bool,
}

#[derive(Debug)]
pub struct PaneInfo {
    id: String,
    width: u16,
    height: u16,
    x: u16,
    y: u16,
    window: Window,
}

pub fn get_pane_info() -> Result<PaneInfo, Box<dyn std::error::Error>> {
    let id = get_value("pane_id").replace("%", "");
    let width: u16 = get_value("pane_width").parse()?;
    let height: u16 = get_value("pane_height").parse()?;

    let window = get_window_info()?;

    let re = Regex::new(&format!(r"\d+x\d+,(\d+),(\d+),{}\b", id))?;

    let caps = re.captures(&window.layout).unwrap();

    let x: u16 = caps.get(1).unwrap().as_str().to_string().parse::<u16>()?;
    let y: u16 = caps.get(2).unwrap().as_str().to_string().parse::<u16>()?;

    Ok(PaneInfo {
        id,
        width,
        height,
        x,
        y,
        window,
    })
}

fn get_window_info() -> Result<Window, Box<dyn std::error::Error>> {
    let layout = get_value("window_layout");
    let width: u16 = get_value("window_width").parse()?;
    let height: u16 = get_value("window_height").parse()?;
    let zoomed = get_value("window_zoomed_flag") == "1";

    Ok(Window {
        layout,
        width,
        height,
        zoomed,
    })
}

impl PaneInfo {
    pub fn is_top(&self) -> bool {
        self.y == 0
    }

    pub fn is_left(&self) -> bool {
        self.x == 0
    }

    pub fn is_bottom(&self) -> bool {
        self.y + self.height == self.window.height
    }

    pub fn is_right(&self) -> bool {
        self.x + self.width == self.window.width
    }
}
