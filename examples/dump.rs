use postgres_parser::*;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let parse_list = parse_query(args.get(1).expect("no arguments")).unwrap();

    println!("{:#?}", parse_list);
}
