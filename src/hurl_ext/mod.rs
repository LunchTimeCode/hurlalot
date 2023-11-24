pub mod ext {

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

    impl From<hurl_core4::parser::ParseError> for HurlParseErrorEnum {
        fn from(value: hurl_core4::parser::ParseError) -> Self {
            match value {
                hurl_core4::parser::ParseError::Expecting { value } => {
                    HurlParseErrorEnum::Expecting { value }
                }
                hurl_core4::parser::ParseError::Method { name } => {
                    HurlParseErrorEnum::Method { name }
                }
                hurl_core4::parser::ParseError::Version {} => HurlParseErrorEnum::Version {},
                hurl_core4::parser::ParseError::Status {} => HurlParseErrorEnum::Status {},
                hurl_core4::parser::ParseError::Filename {} => HurlParseErrorEnum::Filename {},
                hurl_core4::parser::ParseError::FileContentType {} => {
                    HurlParseErrorEnum::FileContentType {}
                }
                hurl_core4::parser::ParseError::Space {} => HurlParseErrorEnum::Space {},
                hurl_core4::parser::ParseError::RequestSectionName { name } => {
                    HurlParseErrorEnum::RequestSectionName { name }
                }
                hurl_core4::parser::ParseError::ResponseSectionName { name } => {
                    HurlParseErrorEnum::ResponseSectionName { name }
                }
                hurl_core4::parser::ParseError::JsonpathExpr {} => {
                    HurlParseErrorEnum::JsonpathExpr {}
                }
                hurl_core4::parser::ParseError::XPathExpr {} => HurlParseErrorEnum::XPathExpr {},
                hurl_core4::parser::ParseError::TemplateVariable {} => {
                    HurlParseErrorEnum::TemplateVariable {}
                }
                hurl_core4::parser::ParseError::Json {} => HurlParseErrorEnum::Json {},
                hurl_core4::parser::ParseError::Xml {} => HurlParseErrorEnum::Xml {},
                hurl_core4::parser::ParseError::Predicate => HurlParseErrorEnum::Predicate,
                hurl_core4::parser::ParseError::PredicateValue => {
                    HurlParseErrorEnum::PredicateValue
                }
                hurl_core4::parser::ParseError::RegexExpr { message } => {
                    HurlParseErrorEnum::RegexExpr { message }
                }
                hurl_core4::parser::ParseError::Unexpected { character } => {
                    HurlParseErrorEnum::Unexpected { character }
                }
                hurl_core4::parser::ParseError::Eof {} => HurlParseErrorEnum::Eof {},
                hurl_core4::parser::ParseError::DuplicateSection => {
                    HurlParseErrorEnum::DuplicateSection
                }
                hurl_core4::parser::ParseError::RequestSection => {
                    HurlParseErrorEnum::RequestSection
                }
                hurl_core4::parser::ParseError::ResponseSection => {
                    HurlParseErrorEnum::ResponseSection
                }
                hurl_core4::parser::ParseError::HexDigit => HurlParseErrorEnum::HexDigit,
                hurl_core4::parser::ParseError::Unicode => HurlParseErrorEnum::Unicode,
                hurl_core4::parser::ParseError::EscapeChar => HurlParseErrorEnum::EscapeChar,
                hurl_core4::parser::ParseError::InvalidCookieAttribute => {
                    HurlParseErrorEnum::InvalidCookieAttribute
                }
                hurl_core4::parser::ParseError::OddNumberOfHexDigits => {
                    HurlParseErrorEnum::OddNumberOfHexDigits
                }
                hurl_core4::parser::ParseError::UrlIllegalCharacter(c) => {
                    HurlParseErrorEnum::UrlIllegalCharacter(c)
                }
                hurl_core4::parser::ParseError::InvalidOption => HurlParseErrorEnum::InvalidOption,
                hurl_core4::parser::ParseError::Multiline => HurlParseErrorEnum::Multiline,
                hurl_core4::parser::ParseError::GraphQlVariables => {
                    HurlParseErrorEnum::GraphQlVariables
                }
                hurl_core4::parser::ParseError::Url {} => HurlParseErrorEnum::Url {},
            }
        }
    }

    #[derive(Clone, Debug, PartialEq, Eq)]
    pub enum HurlParseErrorEnum {
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
