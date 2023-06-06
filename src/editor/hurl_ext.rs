use hurl_core::{ast::Pos, parser::ParseError};

pub fn parse_err_to_string(error: ParseError, pos: Pos) -> String {
    let message = match error.clone() {
        ParseError::Expecting { value } => format!("expecting: {value}"),
        ParseError::Method { name } => format!("method: {name}"),
        ParseError::Version {} => format!("version"),
        ParseError::Status {} => format!("status"),
        ParseError::Filename {} => format!("filename"),
        ParseError::FileContentType {} => format!("file type"),
        ParseError::Space {} => format!("space"),
        ParseError::RequestSectionName { name } => format!("RequestSectionName: {name}"),
        ParseError::ResponseSectionName { name } => format!("ResponseSectionName: {name}"),
        ParseError::JsonpathExpr {} => format!("JsonpathExpr"),
        ParseError::XPathExpr {} => format!("XPathExpr"),
        ParseError::TemplateVariable {} => format!("TemplateVariable"),
        ParseError::Json {} => format!("json"),
        ParseError::Xml {} => format!("xml"),
        ParseError::Predicate => format!("Predicate"),
        ParseError::PredicateValue => format!("PredicateValue"),
        ParseError::RegexExpr { message } => format!("regex: {message}"),
        ParseError::Unexpected { character } => format!("Unexpected char: {character}"),
        ParseError::Eof {} => format!("Eof"),
        ParseError::Url {} => format!("Url"),
        ParseError::DuplicateSection => format!("duplicate section"),
        ParseError::RequestSection => format!("request section"),
        ParseError::ResponseSection => format!("response section"),
        ParseError::HexDigit => format!("HexDigit"),
        ParseError::Unicode => format!("Unicode"),
        ParseError::EscapeChar => format!("EscapeChar"),
        ParseError::InvalidCookieAttribute => format!("Invalid cookie attribute"),
        ParseError::OddNumberOfHexDigits => format!("Odd number of hex digits"),
        ParseError::UrlIllegalCharacter(char) => format!("illegal char in url: {char}"),
        ParseError::InvalidOption => format!("Invalid option"),
        ParseError::Multiline => format!("Multiline"),
        ParseError::GraphQlVariables => format!("GraphQlVariables"),
    };
    format!("{} at Ln {}, Col {}", message, pos.line, pos.column)
}
