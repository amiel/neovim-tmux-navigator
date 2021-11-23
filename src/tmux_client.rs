use std::process::Command;

use clap::ArgMatches;
use neovim_lib::{Neovim, NeovimApi, Session};

use crate::tmux_util;

#[derive(Debug)]
enum Movement {
    Up,
    Down,
    Left,
    Right,
    Unknown,
}

pub struct Handler {
    nvim: Neovim,
    tmux_socket: Option<String>,
    movement: Movement,
}

fn build_session(nvim_socket: &str) -> Neovim {
    let session = Session::new_unix_socket(nvim_socket).unwrap();
    Neovim::new(session)
}

fn make_session(maybe_address: Option<&str>) -> Neovim {
    match maybe_address {
        Some(value) => build_session(value),
        None => {
            let value = tmux_util::get_option("@nvim-listen-address");

            build_session(value.as_str())
        }
    }
}

impl Handler {
    pub fn new(sub_args: &ArgMatches) -> Handler {
        let tmux_socket = sub_args.value_of("tmux-socket").map(|s| s.to_string());

        let nvim = make_session(sub_args.value_of("nvim-listen-address"));

        // TODO: Implement From?
        let movement = if sub_args.is_present("up") {
            Movement::Up
        } else if sub_args.is_present("down") {
            Movement::Down
        } else if sub_args.is_present("left") {
            Movement::Left
        } else if sub_args.is_present("right") {
            Movement::Right
        } else {
            Movement::Unknown
        };

        Handler {
            nvim,
            tmux_socket,
            movement,
        }
    }

    pub fn call(&mut self) {
        let _receiver = self.nvim.session.start_event_loop();

        if let Some(_) = self.tmux_socket {
            if self.is_vim() {
                self.nvim
                    .command("echo \"is vim - window: \" . winnr()")
                    .unwrap();
            } else {
                self.nvim
                    .command("echo \"not vim - window: \" . winnr()")
                    .unwrap();
            }
        } else {
            self.nvim
                .command("echo \"not tmux - window: \" . winnr()")
                .unwrap();
        }
    }

    fn is_vim(&self) -> bool {
        let tmux_pane_tty = tmux_util::get_value("pane_tty");

        // This command taken from https://github.com/christoomey/vim-tmux-navigator/commit/57701ac650990010ea97b1b4d64779d0b60c769b#diff-04c6e90faac2675aa89e2176d2eec7d8
        let c = &format!("ps -o state= -o comm= -t '{}' | grep -iqE '^[^TXZ ]+ +(\\S+\\/)?g?(view|n?vim?x?)(diff)?$'", tmux_pane_tty);

        let is_vim_status = Command::new("sh")
            .arg("-c")
            .arg(c)
            .status()
            .expect("Failed to check for vim");

        is_vim_status.success()
    }
}
