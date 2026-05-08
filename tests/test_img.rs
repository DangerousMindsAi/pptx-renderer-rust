use hyper_render::{render_to_png, Config};

#[test]
fn test_img_renders() {
    let img_base64 = "iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAYAAAAfFcSJAAAADUlEQVR4nGNgYPj/HwADAgH/5ncLrgAAAABJRU5ErkJggg==";
    let html = format!(r#"
        <html>
            <body>
                <div style="background-color: red; width: 100px; height: 100px; position: relative;">
                    <img src="data:image/png;base64,{}" style="position: absolute; left: 10px; top: 10px; width: 50px; height: 50px; display: block;" />
                </div>
            </body>
        </html>
    "#, img_base64);

    let config = Config::new().width(200).height(200);
    let png = render_to_png(&html, config).unwrap();
    std::fs::write("scratch/test_img.png", png).unwrap();
    println!("PNG generated");
}

#[test]
fn test_blitz_svg() {
    let svg = r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 100 100"><rect width="100" height="100" fill="red"/></svg>"#;
    use base64::{Engine as _, engine::general_purpose};
    let encoded = general_purpose::STANDARD.encode(svg);
    let html = format!(r#"
    <div style="width: 200px; height: 200px; background-color: blue;">
        <img src="data:image/svg+xml;base64,{}" style="width: 100px; height: 100px;" />
    </div>
    "#, encoded);
    
    let png = hyper_render::render_to_png(&html, hyper_render::Config::default()).unwrap();
    std::fs::write("scratch/test_blitz_svg.png", png).unwrap();
}
