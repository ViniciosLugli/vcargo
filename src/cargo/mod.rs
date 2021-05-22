use ansi_term::Colour;
use std::process::Command;
pub struct CargoCommand {
    pub default_command: &'static str,
}

pub trait Cargo {
    fn run(&self, args: Vec<String>) -> bool;
}

impl Cargo for CargoCommand {
    fn run(&self, args: Vec<String>) -> bool {
        let mut init_cmd: Command = Command::new(self.default_command);
        for arg in args {
            init_cmd.arg(arg);
        }
        println!("{}", Colour::Purple.paint("Calling cargo process..."));
        let mut child = init_cmd.spawn().unwrap();

        match child.try_wait() {
            Ok(Some(_)) => {
                return true;
            }
            Ok(None) => {
                child
                    .wait()
                    .expect("Unexpected error on command! Check the args for cargo package...");
                return true;
            }
            Err(_) => return false,
        };
    }
}
