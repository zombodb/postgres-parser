use postgres_parser::*;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let filename = args.get(1).expect("no filename");

    let contents = std::fs::read_to_string(filename).expect("failed to read file");

    let parse_list = match parse_query(&contents) {
        Ok(query) => query,
        Err(e) => {
            eprintln!("{:?}", e);
            return;
        }
    };
    let as_json = serde_json::to_string_pretty(&parse_list).expect("failed to convert to json");

    println!("{}", as_json);
}
