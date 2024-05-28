mod blob;

use flate2::read;
use flate2::read::ZlibDecoder;
#[allow(unused_imports)]
use std::env;
#[allow(unused_imports)]
use std::fs;
use std::fs::File;
use std::io::BufReader;
use std::io::Read;
use blob::Blob;
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
        .subcommand(
            Command::new("hash-object")
                .arg_required_else_help(true)
                .about("hashes your object and stores it in the objects folder")
                .arg(
                    Arg::new("p")
                        .required(true)
                        .action(ArgAction::Set)
                )
                .arg(
                    Arg::new("w")
                        .short('w')
                        .action(ArgAction::SetTrue)
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

                let mut out = String::new();
                let mut decoder = ZlibDecoder::new(BufReader::new(
                    File::open(file_path.clone()).expect("error opening file "),
                ));
                decoder.read_to_string(&mut out);
                let mut index =  out.find("\0").unwrap_or_else(|| out.len());
                if index  != out.len() {
                    index = index +1;
                }
                let (blog, output) = out.split_at(index);
                print!("{}", output);
            }
            None => panic!("provide file name"),
        },
        Some(("hash-object", submatches)) => {


            let blob;

            match submatches.get_one::<String>("p") {
                Some(file_name) => {
                    blob = Blob::new(file_name);
                    blob.print_hash();

                }
                None => panic!("provide file name"),
            }
            match submatches.get_flag("w") {
                true => blob.write_to_objects(),
                false => (),
            }
        },
        _ => (),
    }
}
