use neovim_lib::{Neovim, NeovimApi, Session};

enum Message {
    Up,
    Down,
    Left,
    Right,
    Unknown(String),
}

impl From<String> for Message {
    fn from(event: String) -> Self {
        match &event[..] {
            "up" => Message::Up,
            "down" => Message::Down,
            "left" => Message::Left,
            "right" => Message::Right,
            _ => Message::Unknown(event),
        }
    }
}

struct EventHandler {
    nvim: Neovim,
}

impl EventHandler {
    fn new() -> EventHandler {
        let session = Session::new_parent().unwrap();
        let nvim = Neovim::new(session);

        EventHandler { nvim }
    }

    fn recv(&mut self) {
        let receiver = self.nvim.session.start_event_loop_channel();

        for (event, _values) in receiver {
            match Message::from(event) {
                Message::Up => {
                    // let nums = values
                    //     .iter()
                    //     .map(|v| v.as_i64().unwrap())
                    //     .collect::<Vec<i64>>();

                    // let sum = self.calculator.add(nums);
                    // self.nvim
                    //     .command(&format!("echo \"Sum: {}\"", sum.to_string()))
                    //     .unwrap();
                }

                Message::Down => {
                    self.nvim
                        .command(&format!("echo \"Product: {}\"", "Down"))
                        .unwrap();
                }
                Message::Left => {}
                Message::Right => {}

                // Handle anything else
                Message::Unknown(event) => {
                    self.nvim
                        .command(&format!("echo \"Unknown command: {}\"", event))
                        .unwrap();
                }
            }
        }
    }
}

fn main() {
    let mut event_handler = EventHandler::new();

    event_handler.recv();
}
