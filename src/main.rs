mod cli;
mod neovim_server;
mod tmux_client;

fn main() {
    let args = cli::build_cli().get_matches();

    match args.subcommand() {
        ("server", Some(sub_args)) => neovim_server::EventHandler::new(sub_args).recv(),
        ("client", Some(sub_args)) => tmux_client::Handler::new(sub_args).call(),
        _ => {}
    }
}
