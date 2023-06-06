use hurl_core::{ast::Pos, parser::ParseError};

pub fn parse_err_to_pos_err(error: &ParseError, pos: Pos) -> String {
    let message = parse_err_to_message(error);
    format!("{} at Ln {}, Col {}", message, pos.line, pos.column)
}

pub fn parse_err_to_message(error: &ParseError) -> String {
    match error {
        ParseError::Expecting { value } => format!("expecting: {value}"),
        ParseError::Method { name } => format!("method: {name}"),
        ParseError::Version {} => "version".to_string(),
        ParseError::Status {} => "status".to_string(),
        ParseError::Filename {} => "filename".to_string(),
        ParseError::FileContentType {} => "file type".to_string(),
        ParseError::Space {} => "space".to_string(),
        ParseError::RequestSectionName { name } => format!("RequestSectionName: {name}"),
        ParseError::ResponseSectionName { name } => format!("ResponseSectionName: {name}"),
        ParseError::JsonpathExpr {} => "JsonpathExpr".to_string(),
        ParseError::XPathExpr {} => "XPathExpr".to_string(),
        ParseError::TemplateVariable {} => "TemplateVariable".to_string(),
        ParseError::Json {} => "json".to_string(),
        ParseError::Xml {} => "xml".to_string(),
        ParseError::Predicate => "Predicate".to_string(),
        ParseError::PredicateValue => "PredicateValue".to_string(),
        ParseError::RegexExpr { message } => format!("regex: {message}"),
        ParseError::Unexpected { character } => format!("Unexpected char: {character}"),
        ParseError::Eof {} => "Eof".to_string(),
        ParseError::Url {} => "Url".to_string(),
        ParseError::DuplicateSection => "duplicate section".to_string(),
        ParseError::RequestSection => "request section".to_string(),
        ParseError::ResponseSection => "response section".to_string(),
        ParseError::HexDigit => "HexDigit".to_string(),
        ParseError::Unicode => "Unicode".to_string(),
        ParseError::EscapeChar => "EscapeChar".to_string(),
        ParseError::InvalidCookieAttribute => "Invalid cookie attribute".to_string(),
        ParseError::OddNumberOfHexDigits => "Odd number of hex digits".to_string(),
        ParseError::UrlIllegalCharacter(char) => format!("illegal char in url: {char}"),
        ParseError::InvalidOption => "Invalid option".to_string(),
        ParseError::Multiline => "Multiline".to_string(),
        ParseError::GraphQlVariables => "GraphQlVariables".to_string(),
    }
}
