use dialoguer::{theme::ColorfulTheme, History, Input, Select};
use std::{collections::VecDeque, process};

fn main() {
    let selections = &[
        "Long persistent (multiple commands)",
        "Per-command (one command)",
    ];

    match Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Choose history type")
        .default(0)
        .items(&selections[..])
        .interact()
        .unwrap()
    {
        0 => {
            println!("Use 'exit' to quit the prompt");
            println!("Use the Up/Down arrows to scroll through history");
            let mut history = MyHistory::default();
            loop {
                if let Ok(cmd) = Input::<String>::with_theme(&ColorfulTheme::default())
                    .with_prompt("dialoguer")
                    .history_with(&mut history)
                    .interact_text()
                {
                    if cmd == "exit" {
                        process::exit(0);
                    }
                    println!("Entered {}", cmd);
                }
            }
        }
        1 => {
            println!("Type a valid e-mail");
            println!("Use the Up/Down arrows if your first attempt was unsuccesseful");
            let email = Input::with_theme(&ColorfulTheme::default())
                .with_prompt("E-mail")
                .validate_with(|email: &String| -> Result<(), String> {
                    if !email.contains('@') {
                        return Err(format!("{}: '{}'", "Invalid email address", email));
                    }

                    Ok(())
                })
                .history_infinite()
                .interact()
                .expect("Unable to read email");

            println!("Welcome, {email}!");
        }
        _ => unreachable!(),
    };
}

struct MyHistory {
    max: usize,
    history: VecDeque<String>,
}

impl Default for MyHistory {
    fn default() -> Self {
        MyHistory {
            max: 4,
            history: VecDeque::new(),
        }
    }
}

impl History for MyHistory {
    fn read(&self, pos: usize) -> Option<String> {
        self.history.get(pos).cloned()
    }

    fn write(&mut self, val: String) {
        if self.history.len() == self.max {
            self.history.pop_back();
        }
        self.history.push_front(val.to_string());
    }
}
