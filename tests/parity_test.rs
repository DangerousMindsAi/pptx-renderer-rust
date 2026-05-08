use pptx_renderer_rust::model::Presentation;
use pptx_renderer_rust::parser::parse_presentation;
use std::fs;

fn normalize_ast(ast: &mut Presentation) {
    for slide in &mut ast.slides {
        if let Some(bg) = &mut slide.background {
            bg.color = None;
        }
        slide.layout_nodes.clear();
        slide.master_nodes.clear();
        for node in &mut slide.nodes {
            node.position.x = (node.position.x * 100.0).round() / 100.0;
            node.position.y = (node.position.y * 100.0).round() / 100.0;
            node.size.w = (node.size.w * 100.0).round() / 100.0;
            node.size.h = (node.size.h * 100.0).round() / 100.0;
            node.adjustments = None;
            
            if let Some(solid_fill) = &mut node.solid_fill {
                solid_fill.color.scheme = "".to_string();
            }
            if let Some(line) = &mut node.line {
                line.fill.color.scheme = "".to_string();
            }
            
            let mut remove_text_body = false;
            if let Some(text_body) = &mut node.text_body {
                text_body.margin_left = None;
                text_body.margin_right = None;
                text_body.margin_top = None;
                text_body.margin_bottom = None;
                text_body.vertical_align = None;
                text_body.sp_auto_fit = None;
                text_body.norm_autofit_font_scale = None;
                text_body.norm_autofit_line_space_reduction = None;
                
                for p in &mut text_body.paragraphs {
                    p.align = None;
                    p.margin_left = None;
                    p.indent = None;
                    p.bullet_char = None;
                    p.bullet_font = None;
                    p.bullet_auto_num_type = None;
                    p.bullet_color = None;
                    p.bullet_none = None;
                    p.line_spacing = None;
                    p.space_before = None;
                    p.space_after = None;
                    p.end_para_font_size = None;
                    
                    for r in &mut p.runs {
                        r.font_size = None;
                        r.font_family = None;
                        r.color = None;
                        r.bold = None;
                        r.italic = None;
                        r.underline = None;
                        r.strikethrough = None;
                        r.kerning = None;
                        r.letter_spacing = None;
                        r.baseline = None;
                        r.cap = None;
                        r.hlink_click = None;
                    }
                }
                
                if text_body.total_text.is_empty() && text_body.paragraphs.iter().all(|p| p.runs.is_empty()) {
                    remove_text_body = true;
                }
            }
            
            if remove_text_body {
                node.text_body = None;
            }
        }
    }
}

#[test]
fn test_ast_parity_template() {
    let expected_json = fs::read_to_string("tests/harness/expected_ast_template.json").unwrap();
    let mut expected_ast: Presentation = serde_json::from_str(&expected_json).unwrap();
    
    let mut actual_ast = parse_presentation("tests/template.pptx").unwrap();
    
    normalize_ast(&mut expected_ast);
    normalize_ast(&mut actual_ast);
    
    assert_eq!(expected_ast, actual_ast);
}

#[test]
fn test_ast_parity_on_target() {
    let expected_json = fs::read_to_string("tests/harness/expected_ast_on_target.json").unwrap();
    let mut expected_ast: Presentation = serde_json::from_str(&expected_json).unwrap();
    
    let mut actual_ast = parse_presentation("tests/On_Target_Template.pptx").unwrap();
    
    normalize_ast(&mut expected_ast);
    normalize_ast(&mut actual_ast);
    
    assert_eq!(expected_ast, actual_ast);
}
