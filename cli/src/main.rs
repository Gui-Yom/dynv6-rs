use clap::Clap;
use dynv6_rs::DynV6;
use std::fs;

#[derive(Clap)]
#[clap(
    version = "1.0",
    author = "Guillaume A. <25181283+Gui-Yom@users.noreply.github.com>"
)]
enum Opts {
    Zones {
        #[clap(subcommand)]
        subcmd: ZonesSubCommand,
    },
    Records {
        zone_id: u64,
        #[clap(subcommand)]
        subcmd: RecordsSubCommand,
    },
}

#[derive(Clap)]
enum ZonesSubCommand {
    List,
}

#[derive(Clap)]
enum RecordsSubCommand {
    List,
}

fn main() {
    let opts: Opts = Opts::parse();
    let token = fs::read_to_string("token.txt").expect("Can't read file token.txt");
    let api = DynV6::new(&token);
    match opts {
        Opts::Zones { subcmd } => match subcmd {
            ZonesSubCommand::List => {
                api.list_zones().iter().for_each(|it| {
                    println!("{}: {} -> {}", it.id, it.name, it.ipv4address);
                });
            }
        },
        Opts::Records { zone_id, subcmd } => match subcmd {
            RecordsSubCommand::List => {
                api.list_records(zone_id)
                    .iter()
                    .for_each(|r| println!("record: {} {}: {}", r.record_type, r.name, r.data));
            }
        },
    }
}
