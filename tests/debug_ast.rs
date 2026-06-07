use pptx_renderer_rust::parser::parse_presentation;

#[test]
fn debug_ast() {
    let pres = parse_presentation("../doc-scribe/tests/v2_test_output.pptx").unwrap();
    for (idx, slide) in pres.slides.iter().enumerate() {
        for node in &slide.nodes {
            if let Some(tb) = &node.text_body {
                let has_inline_code = tb.paragraphs.iter().any(|p| {
                    p.runs.iter().any(|r| r.text.contains("inline code"))
                });
                if has_inline_code {
                    println!("FOUND shape on slide index {}, xml index: {}", idx, slide.index);
                    for (p_idx, p) in tb.paragraphs.iter().enumerate() {
                        println!("  Paragraph {}: line_spacing = {:?}", p_idx, p.line_spacing);
                    }
                }
            }
        }
    }
}
