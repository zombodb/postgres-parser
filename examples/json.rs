use postgres_parser::*;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let query_string = args.get(1).expect("no arguments");
    let parse_list = match parse_query(query_string) {
        Ok(query) => query,
        Err(e) => {
            eprintln!("{:?}", e);
            return;
        }
    };
    let as_json = serde_json::to_string_pretty(&parse_list).expect("failed to convert to json");

    println!("{}", as_json);
}
