use neovim_lib::{Neovim, NeovimApi, Session};
use std::process::Command;

pub struct Handler {
    nvim: Neovim,
}

impl Handler {
    pub fn new() -> Handler {
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

                Handler { nvim }
            }
            None => panic!("Could not parse listen address"),
        }
    }

    pub fn call(&mut self) {
        let _receiver = self.nvim.session.start_event_loop();
        self.nvim.command("echo winnr()").unwrap();
    }
}
