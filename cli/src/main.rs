use clap::Clap;
use dynv6_rs::DynV6;
use publicsuffix::Domain;
use std::{fs, str::FromStr};

#[derive(Clap)]
#[clap(
    version = "1.0.0",
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
    Get { zone: ZoneId },
    Update { id: u64, address: String },
    Delete { id: u64 },
}

enum ZoneId {
    Id(u64),
    Name(String),
}

impl FromStr for ZoneId {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let as_id = s.parse().map(ZoneId::Id).ok();
        if as_id.is_some() {
            return Ok(as_id.unwrap());
        }
        // quick and easy way to filter
        if s.contains(".") {
            return Ok(ZoneId::Name(s.to_string()));
        } else {
            return Err(format!("Can't parse {} as a zone id nor a domain name.", s));
        }
    }
}

#[derive(Clap)]
enum RecordsSubCommand {
    List,
    Get { id: u64 },
    Delete { id: u64 },
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
            ZonesSubCommand::Get { zone } => {
                let it = match zone {
                    ZoneId::Id(id) => api.get_zone(id),
                    ZoneId::Name(name) => api.get_zone_by_name(&name),
                };
                println!(
                    "{}: {} -> {} (created : {}, updated : {})",
                    it.id, it.name, it.ipv4address, it.created_at, it.updated_at
                );
            }
            ZonesSubCommand::Update { id, address } => api
                .update_zone(id, &address, None)
                .expect("Error when trying to update zone."),
            ZonesSubCommand::Delete { id } => {
                if api.delete_zone(id) {
                    println!("Deleted 1 zone.")
                } else {
                    println!("Error ! No zone were deleted.")
                }
            }
        },
        Opts::Records { zone_id, subcmd } => match subcmd {
            RecordsSubCommand::List => {
                api.list_records(zone_id)
                    .iter()
                    .for_each(|r| println!("{}: {} {}: {}", r.id, r.record_type, r.name, r.data));
            }
            RecordsSubCommand::Get { id } => {
                let it = api.get_record(zone_id, id);
                println!("{} : {} {} -> {}", it.id, it.record_type, it.name, it.data)
            }
            RecordsSubCommand::Delete { id } => {
                if api.delete_record(zone_id, id) {
                    println!("Deleted 1 zone.")
                } else {
                    println!("Error ! No zone were deleted.")
                }
            }
        },
    }
}
