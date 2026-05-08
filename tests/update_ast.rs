use pptx_renderer_rust::parser::parse_presentation;
use std::fs;

fn main() {
    let actual_ast1 = parse_presentation("tests/template.pptx").unwrap();
    let json1 = serde_json::to_string_pretty(&actual_ast1).unwrap();
    fs::write("tests/harness/expected_ast_template.json", json1).unwrap();

    let actual_ast2 = parse_presentation("tests/On_Target_Template.pptx").unwrap();
    let json2 = serde_json::to_string_pretty(&actual_ast2).unwrap();
    fs::write("tests/harness/expected_ast_on_target.json", json2).unwrap();
    println!("Updated ASTs!");
}
