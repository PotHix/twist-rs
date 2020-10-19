use std::{env, process};
use structopt::StructOpt;

use twist_rs;

struct Twist {
    token: String,
}

#[derive(Debug, StructOpt)]
#[structopt(name = "twist", about = "The CLI for the Twist API")]
struct Opt {
    #[structopt(short = "d", long = "debug")]
    debug: bool,

    #[structopt(subcommand)]
    cmd: Command
}

#[derive(Debug, StructOpt)]
enum Command {
    Search {
        #[structopt(short = "q")]
        query: String,
    },

    Thread {
        id: u64,
    },

    Comment {
        id: u64,
    },
}

fn main() {
    if let Ok(t) = env::var("auth") {
        let config = Twist { token: t };
    } else {
        println!("Token not found. Please define it as the $auth environment variable");
        process::exit(0);
    }

    let opt = Opt::from_args();
    println!("{:?}", opt);

    match opt.cmd {
        Command::Search { query } => println!("thread {}", query), //twist_rs::search::search(query),
        Command::Thread { id } => println!("thread {}", id),
        Command::Comment { id } => println!("comment {}", id),
    }
}
