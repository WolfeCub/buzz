use super::*;
use proptest::prelude::*;

proptest! {
    #[test]
    fn parses_valid_http_methods(test_method in "GET|PUT|POST|DELETE|PATCH|OPTIONS") {
        let format = format!("{} / HTTP/1.1", test_method.to_string());

        let parser = Parser::new(format.as_bytes());
        let result = parse_http_method(&parser);

        assert!(result.is_ok());

        assert_eq!(result.unwrap().to_string(), test_method);
        assert_eq!(parser.offset() - 1, test_method.to_string().len());
    }

    #[test]
    fn fails_parses_random_http_methods(method: String) {
        let format = format!("{method} / HTTP/1.1");
        let parser = Parser::new(format.as_bytes());
        let result = parse_http_method(&parser);

        assert!(result.is_err());
    }

    #[test]
    fn parses_valid_paths(test_path in "/[A-Za-z0-9-._~!$&'()*+,;=:@%?]+") {
        let format = format!("{test_path} HTTP/1.1");

        let parser = Parser::new(format.as_bytes());
        let result = parse_http_path(&parser);

        assert!(result.is_ok());

        assert_eq!(result.unwrap(), test_path);
        assert_eq!(parser.offset() - 1, test_path.len());
    }

    #[test]
    fn fails_parses_invalid_paths(test_path in "[A-Z]+[ ]+[A-Z]+") {
        let format = format!("{test_path} HTTP/1.1");

        let parser = Parser::new(format.as_bytes());
        let result = parse_http_path(&parser);

        assert!(result.is_ok());

        assert_ne!(result.unwrap(), test_path);
        assert_ne!(parser.offset() - 1, test_path.len());
    }

    #[test]
    fn parses_valid_versions(test_version in "[0-9]{1,10}\\.[0-9]{1,10}") {
        let format = format!("HTTP/{test_version}\r\n");

        let parser = Parser::new(format.as_bytes());
        let result = parse_http_version(&parser);

        assert!(result.is_ok());

        let version = result.unwrap();
        assert_eq!(version, test_version.parse::<f64>().unwrap());
        assert_eq!(parser.offset(), parser.data.len());
    }

    #[test]
    fn fails_parses_invalid_versions(test_version: String) {
        let parser = Parser::new(test_version.as_bytes());
        let result = parse_http_version(&parser);

        assert!(result.is_err());
    }

    #[test]
    fn parses_valid_headers(test_key in "[A-Za-z-_]+", test_value in r#"[A-Za-z-_:;.,\\/"'?!(){}\[\]@<>=-\\+*#$&`|~\\^%]+"#) {
        let format = format!("{test_key}: {test_value}\r\n");

        let parser = Parser::new(format.as_bytes());
        let result = parse_http_header(&parser);

        assert!(result.is_ok());

        let option = result.unwrap();

        assert!(option.is_some());

        let (key, value) = option.unwrap();
        assert_eq!(key, test_key);
        assert_eq!(value, test_value);
    }

    #[test]
    fn fails_parses_invalid_headers(header in "[^:^\r^\n]+") {
        let parser = Parser::new(header.as_bytes());
        let result = parse_http_header(&parser);

        assert!(result.is_err());
    }
}
