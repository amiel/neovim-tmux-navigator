use std::process::Command;

use clap::ArgMatches;
use neovim_lib::{Neovim, NeovimApi, Session};

pub struct Handler {
    nvim: Neovim,
    tmux_socket: Option<String>,
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

        // TODO: Parse direction args

        Handler { nvim, tmux_socket }
    }

    pub fn call(&mut self) {
        let _receiver = self.nvim.session.start_event_loop();

        if let Some(_) = self.tmux_socket {
            self.nvim.command("echo \"in tmux\"").unwrap();
        }

        self.nvim.command("echo winnr()").unwrap();
    }
}
