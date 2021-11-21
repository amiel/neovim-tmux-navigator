use clap::{app_from_crate, crate_authors, crate_description, crate_name, crate_version};
use clap::{App, AppSettings, Arg, SubCommand};

pub fn build_cli() -> App<'static, 'static> {
    app_from_crate!()
        .setting(AppSettings::InferSubcommands)
        .setting(AppSettings::SubcommandRequired)
        .subcommands(vec![
            SubCommand::with_name("client")
                .about("Run from tmux to control neovim")
                .args(&[
                    Arg::with_name("up")
                        .short("U")
                        .help("Move up (implies client mode)")
                        .conflicts_with_all(&["down", "left", "right"]),
                    Arg::with_name("down")
                        .short("D")
                        .help("Move down (implies client mode)")
                        .conflicts_with_all(&["up", "left", "right"]),
                    Arg::with_name("left")
                        .short("L")
                        .help("Move left (implies client mode)")
                        .conflicts_with_all(&["up", "down", "right"]),
                    Arg::with_name("right")
                        .short("R")
                        .help("Move right (implies client mode)")
                        .conflicts_with_all(&["up", "down", "left"]),
                    Arg::with_name("tmux-socket")
                        .required(true)
                        .long("tmux-socket")
                        .env("TMUX")
                        .help("TMUX socket"),
                    Arg::with_name("nvim-listen-address")
                        .long("nvim-listen-address")
                        .env("NVIM_LISTEN_ADDRESS")
                        .help("Unix socket to connect to neovim (implies server mode)"),
                ]),
            SubCommand::with_name("server")
                .about("Run as a subprocess of neovim")
                .args(&[
                    Arg::with_name("nvim-listen-address")
                        .required(true)
                        .long("nvim-listen-address")
                        .env("NVIM_LISTEN_ADDRESS")
                        .help("Unix socket to connect to neovim (implies server mode)"),
                    Arg::with_name("tmux-socket")
                        .long("tmux-socket")
                        .env("TMUX")
                        .help("TMUX socket"),
                ]),
        ])
}
