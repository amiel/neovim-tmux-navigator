use std::process::Command;

use clap::ArgMatches;
use neovim_lib::{Neovim, NeovimApi, Session};

enum Message {
    Up,
    Down,
    Left,
    Right,
    Unknown(String),
}

impl From<String> for Message {
    fn from(event: String) -> Self {
        match &event[..] {
            "up" => Message::Up,
            "down" => Message::Down,
            "left" => Message::Left,
            "right" => Message::Right,
            _ => Message::Unknown(event),
        }
    }
}

pub struct EventHandler {
    nvim: Neovim,
    nvim_socket: String,
    tmux_socket: Option<String>,
}

impl EventHandler {
    pub fn new(sub_args: &ArgMatches) -> EventHandler {
        let session = Session::new_parent().unwrap();
        let nvim = Neovim::new(session);

        let nvim_socket = sub_args
            .value_of("nvim-listen-address")
            .unwrap()
            .to_string();
        let tmux_socket = sub_args.value_of("tmux-socket").map(|s| s.to_string());

        EventHandler {
            nvim,
            nvim_socket,
            tmux_socket,
        }
    }

    fn setup(&mut self) {
        self.nvim
            .command("echo \"neovim-tmux-navigator EventHandler\"")
            .unwrap();

        if let Some(value) = self.tmux_socket.clone() {
            self.nvim
                .command(&format!("echo \"in tmux: {}\"", value))
                .unwrap();

            Command::new("tmux")
                .args(&[
                    "set-option",
                    "-w",
                    "@nvim-listen-address",
                    &self.nvim_socket,
                ])
                .spawn()
                .unwrap();
        }
    }

    pub fn recv(&mut self) {
        let receiver = self.nvim.session.start_event_loop_channel();

        self.setup();

        for (event, _values) in receiver {
            match Message::from(event) {
                Message::Up => {}
                Message::Down => {}
                Message::Left => {}
                Message::Right => {}

                // Handle anything else
                Message::Unknown(event) => {
                    self.nvim
                        .command(&format!("echo \"Unknown command: {}\"", event))
                        .unwrap();
                }
            }
        }
    }
}
