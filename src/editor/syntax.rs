use egui_code_editor::Syntax;
use std::collections::BTreeSet;

pub fn hurl() -> Syntax {
    Syntax {
        language: "Hurl",
        case_sensitive: true,
        comment: "#",
        comment_multiline: ["#", "#"],
        hyperlinks: BTreeSet::from([]),
        keywords: BTreeSet::from([
            "jsonpath",
            "count",
            "==",
            ">=",
            "<=",
            "<",
            ">",
            "!=",
            "not",
            "isString",
            "isCollection",
            "isDate",
            "isBoolean",
            "isFloat",
            "isInteger",
            "includes",
            "isEmpty",
            "exists",
            "matches",
            "contains",
            "endsWith",
            "startsWith",
            "",
        ]),
        types: BTreeSet::from([
            "[Captures]",
            "[Asserts]",
            "[FormParams]",
            "[Captures]",
            "[Asserts]",
            "[Asserts]",
            "HTTP",
        ]),
        special: BTreeSet::from([
            "GET", "POST", "HTTP", "HEAD", "PUT", "DELETE", "OPTIONS", "TRACE", "PATCH",
        ]),
    }
}
