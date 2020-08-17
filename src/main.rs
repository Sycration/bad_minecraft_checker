mod file_opener;
mod test;
mod acct_checker;
mod auth_structs;
use file_opener::file_opener::check_list;
use std::env::args;
extern crate clap;
use clap::{Arg, App, SubCommand};
use std::fs;
fn main() {
    let matches = App::new("Minecraft Account Checker")
        .version("0.1-Alpha")
        .author("Sycration V. Xyrozalda <sycration@gmail.com>")
        .about("Checks a list of Minecraft accounts in format username or email:password to see if they are valid ψ(｀∇´)ψ")
        .arg(Arg::with_name("list")
            .short("l")
            .long("list_of_accounts")
            .value_name("LIST")
            .help("File containing a list of Minecraft accounts")
            .takes_value(true))
        .get_matches();
    let pre_path: String = match matches.value_of("list") {
        Some(t) => t,
        _ => {
            println!("No arguments provided! Use the -h flag for help.");
            std::process::exit(1);
        }
    }.parse().unwrap();
    let pre_path = pre_path.replace(r"\", r"\\");
    pub fn path_exists(path: String) -> bool {
        fs::metadata(path).is_ok()
    }
    if !path_exists((&pre_path).parse().unwrap()) {
        println!("The file at {} does not exist", pre_path);
        std::process::exit(1);
    }
    check_list(pre_path);
}
