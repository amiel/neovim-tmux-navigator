use clap::ArgMatches;
use neovim_lib::{Neovim, NeovimApi, Session};

use crate::tmux_util;

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

        // nvim.set_client_info("neovim-tmux-navigator", vec![], "plugin", vec![], vec![])
        //     .expect("Could not set client info");

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
        if self.tmux_socket.is_some() {
            tmux_util::set_option("@nvim-listen-address", &self.nvim_socket);
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
                        .expect("Unknown command from neovim");
                }
            }
        }
    }
}
