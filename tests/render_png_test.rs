use hyper_render::{render_to_png, Config};
use pptx_renderer_rust::parser::parse_presentation;
use pptx_renderer_rust::renderer::render_slide;
use std::fs;

#[test]
fn test_rasterize_template() {
    let actual_ast = parse_presentation("tests/template.pptx").unwrap();
    let slide_html = render_slide(&actual_ast, &actual_ast.slides[0]);
    
    // Wrap in full HTML doc
    let full_html = format!(r#"
        <!DOCTYPE html>
        <html>
            <head>
                <style>
                    * {{ margin: 0; padding: 0; box-sizing: border-box; }}
                    body {{ margin: 0 !important; padding: 0 !important; overflow: hidden !important; }}
                </style>
            </head>
            <body style="margin: 0 !important; padding: 0 !important; overflow: hidden !important;">{}</body>
        </html>
    "#, slide_html);

    let config = Config::new()
        .width(actual_ast.width)
        .height(actual_ast.height)
        .font(include_bytes!("../src/fonts/Carlito-Regular.ttf").to_vec())
        .font(include_bytes!("../src/fonts/Carlito-Italic.ttf").to_vec())
        .font(include_bytes!("../src/fonts/Carlito-Bold.ttf").to_vec())
        .font(include_bytes!("../src/fonts/Carlito-BoldItalic.ttf").to_vec())
        .font(include_bytes!("../src/fonts/Arimo-Regular.ttf").to_vec())
        .font(include_bytes!("../src/fonts/Arimo-Italic.ttf").to_vec())
        .font(include_bytes!("../src/fonts/Arimo-Bold.ttf").to_vec())
        .font(include_bytes!("../src/fonts/Arimo-BoldItalic.ttf").to_vec())
        .font(include_bytes!("../src/fonts/Tinos-Regular.ttf").to_vec())
        .font(include_bytes!("../src/fonts/Tinos-Italic.ttf").to_vec())
        .font(include_bytes!("../src/fonts/Tinos-Bold.ttf").to_vec())
        .font(include_bytes!("../src/fonts/Tinos-BoldItalic.ttf").to_vec());
        
    let png_bytes = render_to_png(&full_html, config).unwrap();
    
    fs::write("tests/harness/actual_template_slide_0.png", png_bytes).unwrap();
    assert!(true); // We just want to generate the PNG for now to verify hyper-render works
}

#[test]
fn test_rasterize_on_target() {
    let actual_ast = parse_presentation("tests/On_Target_Template.pptx").unwrap();
    
    for slide in &actual_ast.slides {
        let slide_html = render_slide(&actual_ast, slide);
        
        // Wrap in full HTML doc
        let full_html = format!(r#"
            <!DOCTYPE html>
            <html>
                <head>
                    <style>
                        * {{ margin: 0; padding: 0; box-sizing: border-box; }}
                        body {{ margin: 0 !important; padding: 0 !important; overflow: hidden !important; }}
                    </style>
                </head>
                <body style="margin: 0 !important; padding: 0 !important; overflow: hidden !important;">
                    {}
                </body>
            </html>
        "#, slide_html);
        
        
        let full_html = full_html.replace("Georgia", "Tinos");
        let full_html = full_html.replace("Montserrat ExtraBold", "Carlito");
        let full_html = full_html.replace("Montserrat", "Carlito");
        fs::write(format!("tests/harness/actual_on_target_slide_{}.html", slide.index), &full_html).unwrap();

        let config = Config::new()
            .width(actual_ast.width)
            .height(actual_ast.height)
            .font(include_bytes!("../src/fonts/Carlito-Regular.ttf").to_vec())
            .font(include_bytes!("../src/fonts/Carlito-Italic.ttf").to_vec())
            .font(include_bytes!("../src/fonts/Carlito-Bold.ttf").to_vec())
            .font(include_bytes!("../src/fonts/Carlito-BoldItalic.ttf").to_vec())
            .font(include_bytes!("../src/fonts/Arimo-Regular.ttf").to_vec())
            .font(include_bytes!("../src/fonts/Arimo-Italic.ttf").to_vec())
            .font(include_bytes!("../src/fonts/Arimo-Bold.ttf").to_vec())
            .font(include_bytes!("../src/fonts/Arimo-BoldItalic.ttf").to_vec())
            .font(include_bytes!("../src/fonts/Tinos-Regular.ttf").to_vec())
            .font(include_bytes!("../src/fonts/Tinos-Italic.ttf").to_vec())
            .font(include_bytes!("../src/fonts/Tinos-Bold.ttf").to_vec())
            .font(include_bytes!("../src/fonts/Tinos-BoldItalic.ttf").to_vec());
            
        let png_bytes = render_to_png(&full_html, config).unwrap();
        
        fs::write(format!("tests/harness/actual_on_target_slide_{}.png", slide.index), png_bytes).unwrap();
    }
}
