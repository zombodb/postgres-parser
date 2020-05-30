use postgres_parser::*;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let parse_list = parse_query(args.get(1).expect("no arguments")).unwrap();
    let as_json = serde_json::to_string_pretty(&parse_list).expect("failed to convert to json");

    println!("{}", as_json);
}
