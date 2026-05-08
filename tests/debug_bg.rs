use pptx_renderer_rust::parser::parse_presentation;

#[test]
fn debug_bg() {
    let pres = parse_presentation("tests/On_Target_Template.pptx").unwrap();
    for slide in pres.slides {
        println!("Slide {}: bg={:?}, master_nodes={}, layout_nodes={}, slide_nodes={}", 
                 slide.index, slide.background, slide.master_nodes.len(), slide.layout_nodes.len(), slide.nodes.len());
        
        let mut count = 0;
        for node in slide.master_nodes.iter().chain(slide.layout_nodes.iter()).chain(slide.nodes.iter()) {
            if node.blip_embed.is_some() {
                count += 1;
            }
        }
        println!("  - nodes with blip_embed: {}", count);
    }
}
