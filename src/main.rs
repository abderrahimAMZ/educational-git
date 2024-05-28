use flate2::read;
use flate2::read::ZlibDecoder;
#[allow(unused_imports)]
use std::env;
#[allow(unused_imports)]
use std::fs;
use std::io::Read;

use clap::{arg, command, value_parser, Arg, ArgAction, Command};

fn main() {
    let matches = Command::new("educational-git")
        .version("1.0")
        .about("it does some of what git does, poorly. don't use it for your projects!")
        .propagate_version(true)
        .subcommand_required(true)
        .subcommand(Command::new("init").about("initilize your git repo"))
        .subcommand(
            Command::new("cat-file")
                .arg_required_else_help(true)
                .about("plumbing command to see your file type and content")
                .arg(
                    Arg::new("p")
                        .short('p')
                        .long("p")
                        .action(ArgAction::Set)
                        .value_name("FILE"),
                ),
        )
        .get_matches();

    match matches.subcommand() {
        Some(("init", _)) => {
            let args: Vec<String> = env::args().collect();
            if args[1] == "init" {
                fs::create_dir(".git").unwrap();
                fs::create_dir(".git/objects").unwrap();
                fs::create_dir(".git/refs").unwrap();
                fs::write(".git/HEAD", "ref: refs/heads/main\n").unwrap();
                println!("Initialized git directory")
            } else {
                println!("unknown command: {}", args[1])
            }
        }

        Some(("cat-file", submatches)) => match submatches.get_one::<String>("p") {
            Some(file_name) => {
                //
                let file_name: Vec<char> = file_name.chars().collect();
                //TODO : take first two letter and open the folder in the objects folder.
                let first_two_letters: String = file_name.iter().take(2).collect();
                let whats_left_from_file_name: String = file_name.iter().skip(2).collect();

                let file_path = format!(
                    "./.git/objects/{}/{}",
                    first_two_letters, whats_left_from_file_name
                );

                let file_content =
                    fs::read_to_string(file_path.clone()).expect("error in the specified path");
                /*
                let mut decoder = ZlibDecoder::new(file_content);
                let mut content: Vec<u8> = vec![];
                decoder.read_to_end(&mut content).unwrap();
                */
                print!("{:?}", file_content);
            }
            None => panic!("provide file name"),
        },
        _ => (),
    }
}
