use std::{env, process};
use structopt::StructOpt;

use twist_rs;

#[derive(Debug, StructOpt)]
#[structopt(name = "twist", about = "The CLI for the Twist API")]
struct Opt {
    #[structopt(short = "d", long = "debug")]
    debug: bool,

    #[structopt(subcommand)]
    cmd: Command,
}

#[derive(Debug, StructOpt)]
enum Command {
    Search {
        #[structopt(short = "q")]
        query: String,
    },
}

fn main() {
    let token;
    match env::var("auth") {
        Ok(val) => token = val,
        Err(_e) => {
            println!("Token not found. Please define it as the $auth environment variable");
            process::exit(0);
        }
    }

    let opt = Opt::from_args();
    println!("{:?}", opt);

    match opt.cmd {
        Command::Search { query } => {
            if let Ok(res) = twist_rs::endpoints::search::search(token, query) {
                println!("{}", res);
            }
        }
    };
}
