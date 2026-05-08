use pptx_renderer_rust::parser::parse_presentation;

#[test]
fn debug_ast() {
    let pres = parse_presentation("tests/On_Target_Template.pptx").unwrap();
    let slide = &pres.slides[2];
    for node in &slide.layout_nodes {
        if node.blip_embed.is_some() {
            println!("Node with blip_embed: {:#?}", node);
        }
    }
}
