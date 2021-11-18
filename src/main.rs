use std::env;
use std::process::Command;

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

struct EventHandler {
    nvim: Neovim,
}

impl EventHandler {
    fn new() -> EventHandler {
        let session = Session::new_parent().unwrap();
        let nvim = Neovim::new(session);

        EventHandler { nvim }
    }

    fn recv(&mut self) {
        let receiver = self.nvim.session.start_event_loop_channel();

        self.nvim
            .command("echo \"neovim-tmux-navigator EventHandler\"")
            .unwrap();

        match env::var("TMUX") {
            Ok(value) => {
                self.nvim
                    .command(&format!("echo \"in tmux: {}\"", value))
                    .unwrap();

                let nvim_listen_address = env::var("NVIM_LISTEN_ADDRESS").unwrap();

                Command::new("tmux")
                    .args(&[
                        "set-option",
                        "-w",
                        "@nvim-listen-address",
                        &nvim_listen_address,
                    ])
                    .spawn()
                    .unwrap();
            }
            Err(_) => {
                println!("Not in tmux");
                self.nvim.command("echo \"not in tmux\"").unwrap();
            }
        }

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

struct TmuxEventHandler {
    nvim: Neovim,
}

impl TmuxEventHandler {
    fn new() -> TmuxEventHandler {
        // TODO: Use tmux_interface
        let output = Command::new("tmux")
            .args(&["show-option", "-w", "-v", "@nvim-listen-address"])
            .output()
            .unwrap();

        match std::str::from_utf8(&output.stdout)
            .unwrap()
            .strip_suffix("\n")
        {
            Some(socket_path) => {
                let session = Session::new_unix_socket(socket_path).unwrap();
                let nvim = Neovim::new(session);

                TmuxEventHandler { nvim }
            }
            None => panic!("Could not parse listen address"),
        }
    }

    fn recv(&mut self) {
        let _receiver = self.nvim.session.start_event_loop();
        self.nvim.command("echo winnr()").unwrap();

        // for (event, _values) in receiver {
        //     match Message::from(event) {
        //         Message::Up => {}
        //         Message::Down => {}
        //         Message::Left => {}
        //         Message::Right => {}

        //         // Handle anything else
        //         Message::Unknown(event) => {
        //             self.nvim
        //                 .command(&format!("echo \"Unknown command: {}\"", event))
        //                 .unwrap();
        //         }
        //     }
        // }
    }
}

fn main() {
    match env::var("NVIM_LISTEN_ADDRESS") {
        Ok(_) => {
            let mut event_handler = EventHandler::new();

            event_handler.recv();
        }
        Err(_) => {
            if let Ok(_) = env::var("TMUX") {
                let mut tmux_event_handler = TmuxEventHandler::new();
                tmux_event_handler.recv();
            }
        }
    }
}
