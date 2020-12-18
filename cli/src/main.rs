use clap::Clap;
use dynv6_rs::DynV6;
use std::fs;

#[derive(Clap)]
#[clap(
    version = "1.0",
    author = "Guillaume A. <25181283+Gui-Yom@users.noreply.github.com>"
)]
struct Opts {
    #[clap(subcommand)]
    subcmd: SubCommand,
}

#[derive(Clap)]
enum SubCommand {
    Test,
    Print,
}

fn main() {
    let opts: Opts = Opts::parse();
    match opts.subcmd {
        SubCommand::Test => {
            println!("Test !")
        }
        SubCommand::Print => {
            println!("Print ! ")
        }
    }

    let token = fs::read_to_string("token.txt").expect("Can't read file token.txt");
    let api = DynV6::new(&token);
    api.list_zones().iter().for_each(|it| {
        println!("{}: {} -> {}", it.id, it.name, it.ipv4address);
        api.list_records(it.id)
            .iter()
            .for_each(|r| println!("record: {} {}: {}", r.record_type, r.name, r.data))
    });
}
