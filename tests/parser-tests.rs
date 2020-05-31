use postgres_parser::{parse_query, PgParserError};

#[test]
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
