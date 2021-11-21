mod cli;
mod neovim_server;
mod tmux_client;

fn main() {
    let args = cli::build_cli().get_matches();

    match args.subcommand() {
        ("server", Some(sub_args)) => {
            let nvim = sub_args
                .value_of("nvim-listen-address")
                .unwrap()
                .to_string();
            let tmux = sub_args.value_of("tmux-socket").map(|s| s.to_string());

            neovim_server::EventHandler::new(nvim, tmux).recv();
        }

        ("client", Some(_sub_args)) => {
            tmux_client::Handler::new().call();
        }
        _ => {}
    }
}
