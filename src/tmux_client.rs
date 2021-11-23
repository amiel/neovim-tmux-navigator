use std::process::Command;

use clap::ArgMatches;
use neovim_lib::{Neovim, NeovimApi, Session};

use crate::tmux_util;

#[derive(Debug, PartialEq)]
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

        if self.tmux_socket.is_some() {
            if self.is_vim() {
                self.move_in_vim();
            } else {
                self.move_in_tmux();
            }
        } else {
            self.move_in_vim();
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

    fn move_in_tmux(&self) {
        // TODO: Zoom, move windows?
        let pane_info = tmux_util::get_pane_info().expect("Could not get pane info");

        if pane_info.is_top() && self.movement == Movement::Up {
            tmux_util::zoom();
        } else if pane_info.is_bottom() && self.movement == Movement::Down {
            tmux_util::zoom();
        } else if pane_info.is_left() && self.movement == Movement::Left {
            tmux_util::run(&["select-window", "-p"])
        } else if pane_info.is_right() && self.movement == Movement::Right {
            tmux_util::run(&["select-window", "-n"])
        } else {
            tmux_util::move_direction(self.tmux_movement());
        }
    }

    fn move_in_vim(&mut self) {
        let win_before = self.vim_window();

        self.nvim
            .command(format!("wincmd {}", self.vim_movement()).as_str())
            .unwrap();

        let win_after = self.vim_window();

        // If we did not end up moving, then we need to go back to tmux
        if win_after == win_before {
            if self.tmux_socket.is_some() {
                self.move_in_tmux();
            }
        }
    }

    fn vim_window(&mut self) -> neovim_lib::neovim_api::Window {
        self.nvim
            .get_current_win()
            .expect("could not get current window")
    }

    fn vim_movement(&self) -> &str {
        match self.movement {
            Movement::Up => "k",
            Movement::Down => "j",
            Movement::Left => "h",
            Movement::Right => "l",
            _ => panic!("Cannot move unknown direction"),
        }
    }

    fn tmux_movement(&self) -> &str {
        match self.movement {
            Movement::Up => "-U",
            Movement::Down => "-D",
            Movement::Left => "-L",
            Movement::Right => "-R",
            _ => panic!("Cannot move unknown direction"),
        }
    }
}
