use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "twist", about = "The CLI for the Twist API")]
struct Opt {
    #[structopt(short = "d", long = "debug")]
    debug: bool,

    #[structopt(subcommand)]  // Note that we mark a field as a subcommand
    cmd: Command
}

#[derive(Debug, StructOpt)]
enum Command {
    #[structopt(name = "search")]
    Search {
        #[structopt(short = "q")]
        query: Option<String>,
    },
}

fn main() {
    let opt = Opt::from_args();
    println!("{:?}", opt);
}
