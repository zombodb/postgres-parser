use postgres_parser::{parse_query, quote_identifier, PgParserError};

#[test]
#[ignore]
fn test_utf8_parsing() {
    assert!(parse_query("SELECT 'aⓐ' ~ U&'a\\24D0' AS t;").is_ok())
}

#[test]
fn test_invalid_query() {
    let result = parse_query("invalid query");
    assert!(result.is_err());
    let err = result.err().unwrap();
    assert_eq!(
        err,
        PgParserError::ParseError {
            message: "syntax error at or near \"invalid\"".to_string(),
            cursor_pos: 1
        }
    )
}

#[test]
fn test_quote_identifier_quotes_necessary() {
    assert_eq!("\"table\"", quote_identifier(&Some("table".into())));
    assert_eq!("\"foo\"\"bar\"", quote_identifier(&Some("foo\"bar".into())));
}

#[test]
fn test_quote_identifier_quotes_unnecessary() {
    assert_eq!("foo", quote_identifier(&Some("foo".into())));
}

#[test]
fn test_quote_identifier_empty_strings() {
    assert_eq!("", quote_identifier(&Some("".into())));
    assert_eq!("", quote_identifier(&None));
}
