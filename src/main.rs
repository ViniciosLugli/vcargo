mod cargo;
mod custom_commands;

use std::env;

use ansi_term::Colour;
use cargo::{Cargo, CargoCommand};
use custom_commands::*;

// CHANGE FOR YOU SPECS ----------------------------------------------------------------
const _CUSTOM_FILE_INIT: &'static str = "C:\\Users\\vinic\\Documents\\scripts\\rust\\_DEFAULT_";
// -------------------------------------------------------------------------------------

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let cargo_child: CargoCommand = CargoCommand {
        default_command: "cargo",
    };

    println!(
        "{}{:?}",
        Colour::Yellow.paint("Running vcargo with args: "),
        args
    );

    check_and_run_command(cargo_child, args);
}

fn check_and_run_command(cargo: CargoCommand, args: Vec<String>) -> () {
    if cargo.run(args.clone()) {
        println!(
            "{}",
            Colour::Green.bold().paint("Cargo finished successfully!")
        );
    } else {
        println!(
            "{}",
            Colour::Red.bold().paint("Cargo finished with errors!")
        );
    };

    //ADD MODULES:
    if args.len() > 0 {
        if args[0] == "init" {
            println!(
                "{}",
                Colour::Purple.paint("Copying default dirs in this folder...")
            );
            let pathbuf = std::env::current_dir().unwrap();
            let folder_name = pathbuf.as_path().to_str().unwrap();
            if copy_folder(_CUSTOM_FILE_INIT, folder_name) {
                println!("{}", Colour::Green.bold().paint("Copy successfully!"));
            } else {
                println!("{}", Colour::Red.bold().paint("Failed to copy dirs! Check that the path does not already contain the files."));
            }
        }
    }
}
