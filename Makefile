.default: target/debug/neovim-tmux-navigator


target/debug/neovim-tmux-navigator: src/main.rs
	cargo build
