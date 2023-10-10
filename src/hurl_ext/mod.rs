pub mod ext {
    use hurl_core4::parser::ParseError;

    pub fn parse_err_to_pos_err(error: &HurlParseErrorEnum, pos: HurlPos) -> String {
        let message = parse_err_to_message(error);
        format!("{} at Ln {}, Col {}", message, pos.line, pos.column)
    }

    pub fn parse_err_to_message(error: &HurlParseErrorEnum) -> String {
        match error {
            HurlParseErrorEnum::Expecting { value } => format!("expecting: {value}"),
            HurlParseErrorEnum::Method { name } => format!("method: {}", http_error(name)),
            HurlParseErrorEnum::Version {} => "version".to_string(),
            HurlParseErrorEnum::Status {} => "status".to_string(),
            HurlParseErrorEnum::Filename {} => "filename".to_string(),
            HurlParseErrorEnum::FileContentType {} => "file type".to_string(),
            HurlParseErrorEnum::Space {} => "space".to_string(),
            HurlParseErrorEnum::RequestSectionName { name } => {
                format!("RequestSectionName: {name}")
            }
            HurlParseErrorEnum::ResponseSectionName { name } => {
                format!("ResponseSectionName: {name}")
            }
            HurlParseErrorEnum::JsonpathExpr {} => "JsonpathExpr".to_string(),
            HurlParseErrorEnum::XPathExpr {} => "XPathExpr".to_string(),
            HurlParseErrorEnum::TemplateVariable {} => "TemplateVariable".to_string(),
            HurlParseErrorEnum::Json {} => "json".to_string(),
            HurlParseErrorEnum::Xml {} => "xml".to_string(),
            HurlParseErrorEnum::Predicate => "Predicate".to_string(),
            HurlParseErrorEnum::PredicateValue => "PredicateValue".to_string(),
            HurlParseErrorEnum::RegexExpr { message } => format!("regex: {message}"),
            HurlParseErrorEnum::Unexpected { character } => format!("Unexpected char: {character}"),
            HurlParseErrorEnum::Eof {} => "Eof".to_string(),
            HurlParseErrorEnum::Url {} => "Url".to_string(),
            HurlParseErrorEnum::DuplicateSection => "duplicate section".to_string(),
            HurlParseErrorEnum::RequestSection => "request section".to_string(),
            HurlParseErrorEnum::ResponseSection => "response section".to_string(),
            HurlParseErrorEnum::HexDigit => "HexDigit".to_string(),
            HurlParseErrorEnum::Unicode => "Unicode".to_string(),
            HurlParseErrorEnum::EscapeChar => "EscapeChar".to_string(),
            HurlParseErrorEnum::InvalidCookieAttribute => "Invalid cookie attribute".to_string(),
            HurlParseErrorEnum::OddNumberOfHexDigits => "Odd number of hex digits".to_string(),
            HurlParseErrorEnum::UrlIllegalCharacter(char) => format!("illegal char in url: {char}"),
            HurlParseErrorEnum::InvalidOption => "Invalid option".to_string(),
            HurlParseErrorEnum::Multiline => "Multiline".to_string(),
            HurlParseErrorEnum::GraphQlVariables => "GraphQlVariables".to_string(),
            HurlParseErrorEnum::UrlInvalidStart => "Url start is invalid".to_string(),
        }
    }

    fn http_error(err: &str) -> &str {
        if matches!(err.to_lowercase().as_str(), "ttp" | "tp" | "htp") {
            return "you might want HTTP";
        }
        err
    }

    pub fn parse(s: &str) -> Result<String, HurlParseError> {
        match hurl_core4::parser::parse_hurl_file(s) {
            Ok(_) => Ok("".into()),
            Err(err) => Err(err.into()),
        }
    }

    impl From<hurl_core4::parser::Error> for HurlParseError {
        fn from(value: hurl_core4::parser::Error) -> Self {
            Self {
                pos: value.pos.into(),
                inner: value.inner.into(),
            }
        }
    }

    #[derive(Clone, Debug, PartialEq, Eq)]
    pub struct HurlParseError {
        pub pos: HurlPos,
        pub inner: HurlParseErrorEnum,
    }

    impl From<hurl_core4::ast::Pos> for HurlPos {
        fn from(value: hurl_core4::ast::Pos) -> Self {
            Self {
                line: value.line,
                column: value.column,
            }
        }
    }

    #[derive(Clone, Debug, PartialEq, Eq)]
    pub struct HurlPos {
        pub line: usize,
        pub column: usize,
    }

    impl From<ParseError> for HurlParseErrorEnum {
        fn from(value: ParseError) -> Self {
            match value {
                ParseError::Expecting { value } => HurlParseErrorEnum::Expecting { value },
                ParseError::Method { name } => HurlParseErrorEnum::Method { name },
                ParseError::Version {} => HurlParseErrorEnum::Version {},
                ParseError::Status {} => HurlParseErrorEnum::Status {},
                ParseError::Filename {} => HurlParseErrorEnum::Filename {},
                ParseError::FileContentType {} => HurlParseErrorEnum::FileContentType {},
                ParseError::Space {} => HurlParseErrorEnum::Space {},
                ParseError::RequestSectionName { name } => {
                    HurlParseErrorEnum::RequestSectionName { name }
                }
                ParseError::ResponseSectionName { name } => {
                    HurlParseErrorEnum::ResponseSectionName { name }
                }
                ParseError::JsonPathExpr {} => HurlParseErrorEnum::JsonpathExpr {},
                ParseError::XPathExpr {} => HurlParseErrorEnum::XPathExpr {},
                ParseError::TemplateVariable {} => HurlParseErrorEnum::TemplateVariable {},
                ParseError::Json {} => HurlParseErrorEnum::Json {},
                ParseError::Xml {} => HurlParseErrorEnum::Xml {},
                ParseError::Predicate => HurlParseErrorEnum::Predicate,
                ParseError::PredicateValue => HurlParseErrorEnum::PredicateValue,
                ParseError::RegexExpr { message } => HurlParseErrorEnum::RegexExpr { message },
                ParseError::Unexpected { character } => {
                    HurlParseErrorEnum::Unexpected { character }
                }
                ParseError::Eof {} => HurlParseErrorEnum::Eof {},
                ParseError::DuplicateSection => HurlParseErrorEnum::DuplicateSection,
                ParseError::RequestSection => HurlParseErrorEnum::RequestSection,
                ParseError::ResponseSection => HurlParseErrorEnum::ResponseSection,
                ParseError::HexDigit => HurlParseErrorEnum::HexDigit,
                ParseError::Unicode => HurlParseErrorEnum::Unicode,
                ParseError::EscapeChar => HurlParseErrorEnum::EscapeChar,
                ParseError::InvalidCookieAttribute => HurlParseErrorEnum::InvalidCookieAttribute,
                ParseError::OddNumberOfHexDigits => HurlParseErrorEnum::OddNumberOfHexDigits,
                ParseError::UrlIllegalCharacter(c) => HurlParseErrorEnum::UrlIllegalCharacter(c),
                ParseError::InvalidOption => HurlParseErrorEnum::InvalidOption,
                ParseError::Multiline => HurlParseErrorEnum::Multiline,
                ParseError::GraphQlVariables => HurlParseErrorEnum::GraphQlVariables,
                ParseError::Url {} => HurlParseErrorEnum::Url {},
                ParseError::UrlInvalidStart => HurlParseErrorEnum::UrlInvalidStart,
            }
        }
    }

    #[derive(Clone, Debug, PartialEq, Eq)]
    pub enum HurlParseErrorEnum {
        UrlInvalidStart,
        Expecting { value: String },
        Method { name: String },
        Version {},
        Status {},
        Filename {},
        FileContentType {},
        Space {},
        RequestSectionName { name: String },
        ResponseSectionName { name: String },
        JsonpathExpr {},
        XPathExpr {},
        TemplateVariable {},
        Json {},
        Xml {},
        Predicate,
        PredicateValue,
        RegexExpr { message: String },

        Unexpected { character: String },
        Eof {},
        Url {},

        DuplicateSection,
        RequestSection,
        ResponseSection,

        HexDigit,
        Unicode,
        EscapeChar,

        InvalidCookieAttribute,
        OddNumberOfHexDigits,
        UrlIllegalCharacter(char),
        InvalidOption,
        Multiline,
        GraphQlVariables,
    }
}
