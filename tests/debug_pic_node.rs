use pptx_renderer_rust::parser::parse_presentation;

#[test]
fn debug_pic_node() {
    let pres = parse_presentation("tests/On_Target_Template.pptx").unwrap();
    let slide = &pres.slides[0];
    for (i, node) in slide.layout_nodes.iter().enumerate() {
        println!("Layout Node {}: type={}, pos={:?}, size={:?}", i, node.node_type, node.position, node.size);
    }
    for (i, node) in slide.nodes.iter().enumerate() {
        println!("Slide Node {}: type={}, pos={:?}, size={:?}", i, node.node_type, node.position, node.size);
    }
}
