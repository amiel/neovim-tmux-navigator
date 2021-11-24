# neovim-tmux-navigator

## Usage

Use `C-<hjkl>` to navigate left, down, up, right, respectively. neovim-tmux-navigator will switch between vim splits and tmux panes seamlessly. If already at the top or bottom, continuing will zoom in tmux. If already at the left or right, continuing will switch tmux windows.

## Installation

### Nvim plugin

Add `amiel/neovim-tmux-navigator` to your vim plugins, however you like.

### Building tool

For now, a rust toolchain is required to build.

```
cargo install --path /path/to/neovim-tmux-navigator
```

And make sure `$HOME/.cargo/bin` is in your PATH.

### Tmux

Add the following to your .tmux.conf

```
# Smart pane switching with awareness of vim splits
bind -n C-k run-shell 'neovim-tmux-navigator client -U'
bind -n C-j run-shell 'neovim-tmux-navigator client -D'
bind -n C-h run-shell 'neovim-tmux-navigator client -L'
bind -n C-l run-shell 'neovim-tmux-navigator client -R'
```


## TODO

* Improve installation instructions
* Improve installation simplicity (provide pre-built binaries)
* Allow configuring off-edge behavior (zoom vs next/previous tmux window)
* Improved error handling (using unwrap way to much)
* Finish support for C-\ (previous pane)
* Allow configuring keymaps
* tmux plugin with tpm
* nvim without tmux?
