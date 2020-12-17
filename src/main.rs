use std::fs;

use dynv6_rs::DynV6;

fn main() {
    let token = fs::read_to_string("token.txt").expect("Can't read file token.txt");
    let api = DynV6::new(&token);
    api.list_zones()
        .iter()
        .for_each(|it| {
            println!("{}: {} -> {}", it.id, it.name, it.ipv4address);
            api.list_records(it.id).iter().for_each(|r| {
                println!("record: {} {}: {}", r.record_type, r.name, r.data)
            })
        })
}
