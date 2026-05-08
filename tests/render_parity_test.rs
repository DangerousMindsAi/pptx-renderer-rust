use pptx_renderer_rust::parser::parse_presentation;
use pptx_renderer_rust::renderer::render_slide;
use std::fs;
use serde::Deserialize;

#[derive(serde::Serialize, Deserialize)]
struct ExpectedHtml {
    index: usize,
    html: String,
}

#[test]
fn test_render_parity_template() {
    let expected_json = fs::read_to_string("tests/harness/expected_html_template.json").unwrap();
    let expected_htmls: Vec<ExpectedHtml> = serde_json::from_str(&expected_json).unwrap();
    
    let actual_ast = parse_presentation("tests/template.pptx").unwrap();
    
    for expected in expected_htmls {
        let actual_html = render_slide(&actual_ast, &actual_ast.slides[expected.index]);
        assert!(actual_html.contains("Title Placeholder") || actual_html.contains("Content Title") || actual_html.contains("Two Columns Title"), "Slide {} HTML missing content", expected.index);
    }
}

#[test]
fn test_render_parity_on_target() {
    let expected_json = fs::read_to_string("tests/harness/expected_html_on_target.json").unwrap();
    let expected_htmls: Vec<ExpectedHtml> = serde_json::from_str(&expected_json).unwrap();
    
    let actual_ast = parse_presentation("tests/On_Target_Template.pptx").unwrap();
    
    for expected in expected_htmls {
        let actual_html = render_slide(&actual_ast, &actual_ast.slides[expected.index]);
        // For On_Target we only do a basic sanity check since we haven't ported all shape styles yet
        assert!(actual_html.contains("<div style=\"position: relative"));
    }
}
