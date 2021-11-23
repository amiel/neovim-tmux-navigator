use std::process::Command;

use clap::ArgMatches;
use neovim_lib::{Neovim, NeovimApi, Session};

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
            // TODO: Use tmux_interface
            let output = Command::new("tmux")
                .args(&["show-option", "-w", "-v", "@nvim-listen-address"])
                .output()
                .unwrap();

            let value = std::str::from_utf8(&output.stdout)
                .unwrap()
                .strip_suffix("\n")
                .unwrap();

            build_session(value)
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
        let output = Command::new("tmux")
            .args(&["display-message", "-p", "#{pane_tty}"])
            .output()
            .unwrap();

        let value = std::str::from_utf8(&output.stdout)
            .unwrap()
            .strip_suffix("\n")
            .unwrap();

        // This command taken from https://github.com/christoomey/vim-tmux-navigator/commit/57701ac650990010ea97b1b4d64779d0b60c769b#diff-04c6e90faac2675aa89e2176d2eec7d8
        let c = &format!("ps -o state= -o comm= -t '{}' | grep -iqE '^[^TXZ ]+ +(\\S+\\/)?g?(view|n?vim?x?)(diff)?$'", value);

        let is_vim_status = Command::new("sh")
            .arg("-c")
            .arg(c)
            .status()
            .expect("Failed to check for vim");

        is_vim_status.success()
    }
}
