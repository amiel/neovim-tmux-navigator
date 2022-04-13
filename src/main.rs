mod cli;
mod neovim_server;
mod tmux_client;
mod tmux_util;

fn main() {
    let args = cli::build_cli().get_matches();

    match args.subcommand() {
        ("server", Some(sub_args)) => {
            simple_logging::log_to_file("neovim-tmux-navigator-server.log", log::LevelFilter::Info)
                .unwrap();
            log_panics::init();

            neovim_server::EventHandler::new(sub_args).recv()
        }
        ("client", Some(sub_args)) => {
            simple_logging::log_to_file("neovim-tmux-navigator-client.log", log::LevelFilter::Info)
                .unwrap();
            log_panics::init();

            tmux_client::Handler::new(sub_args).call()
        }
        _ => {}
    }
}
