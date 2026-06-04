# User Guide & Code Examples

This guide walks you through using `pptx-renderer-rust` to parse, introspect, and render PowerPoint presentations.

---

## 1. Parsing a Presentation

To read a `.pptx` file, use the `parser::parse_presentation` function. This loads the ZIP archive, parses the slide tree, resolves placeholders, and returns a structured `Presentation` model.

```rust
use pptx_renderer_rust::parser::parse_presentation;

fn main() {
    let pptx_path = "tests/template.pptx";

    match parse_presentation(pptx_path) {
        Ok(presentation) => {
            println!("Presentation Dimensions: {}x{} px", presentation.width, presentation.height);
            println!("Total Slides Found: {}", presentation.slide_count);

            for slide in &presentation.slides {
                println!("Slide #{}", slide.index);
                if let Some(ref notes) = slide.notes {
                    println!("  Speaker Notes: {}", notes);
                }
                println!("  Direct slide nodes count: {}", slide.nodes.len());
                println!("  Inherited layout nodes: {}", slide.layout_nodes.len());
                println!("  Inherited master nodes: {}", slide.master_nodes.len());
            }
        }
        Err(e) => {
            eprintln!("Failed to parse presentation: {}", e);
        }
    }
}
```

---

## 2. Rendering a Slide to HTML

To render a slide to HTML, pass the parent `Presentation` (which contains layout dimensions) and the specific `Slide` to `renderer::render_slide`.

```rust
use std::fs;
use pptx_renderer_rust::parser::parse_presentation;
use pptx_renderer_rust::renderer::render_slide;

fn main() -> Result<(), String> {
    let presentation = parse_presentation("tests/template.pptx")?;
    
    // Get the first slide
    let slide = &presentation.slides[0];
    
    // Render the slide into styled HTML
    let slide_html = render_slide(&presentation, slide);
    
    // Wrap the fragment in a full HTML page
    let full_page_html = format!(r#"
<!DOCTYPE html>
<html>
<head>
    <meta charset="utf-8">
    <title>Slide {}</title>
    <style>
        * {{ margin: 0; padding: 0; box-sizing: border-box; }}
        body {{ margin: 0; padding: 20px; background-color: #f0f0f0; display: flex; justify-content: center; }}
    </style>
</head>
<body>
    {}
</body>
</html>
"#, slide.index, slide_html);

    // Save output
    fs::write("slide_output.html", full_page_html)
        .map_err(|e| e.to_string())?;
        
    println!("Successfully rendered slide to slide_output.html");
    Ok(())
}
```

---

## 3. Rasterizing Slides to PNG with `hyper-render`

`pptx-renderer-rust` is designed to be paired with `hyper-render` to convert HTML slides into high-fidelity PNG images. Because slides rely heavily on specific fonts (e.g. Arial, Calibri), you should configure the render engine with fallback fonts.

### Font Typeface Mapping
To maintain cross-platform visual consistency, the library automatically maps Microsoft Office default typefaces to free, metrically-compatible open-source fonts:
- **Calibri** and **Montserrat** are mapped to **Carlito**.
- **Arial** is mapped to **Arimo**.
- **Times New Roman** and **Georgia** are mapped to **Tinos**.

### Complete PNG Generation Code

Add both dependencies to your `Cargo.toml`:
```toml
[dependencies]
pptx-renderer-rust = { path = "path/to/pptx-renderer-rust" }
hyper-render = { git = "https://github.com/AGanguli/hyper-render.git", features = ["png"] }
```

Use the following Rust code to generate a PNG:

```rust
use std::fs;
use hyper_render::{render_to_png, Config};
use pptx_renderer_rust::parser::parse_presentation;
use pptx_renderer_rust::renderer::render_slide;

fn main() -> Result<(), String> {
    // 1. Parse presentation
    let presentation = parse_presentation("tests/template.pptx")?;
    let slide = &presentation.slides[0];
    
    // 2. Render HTML fragment
    let slide_html = render_slide(&presentation, slide);
    
    // 3. Wrap in clean body CSS
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

    // 4. Configure hyper-render with metrically compatible fonts
    let config = Config::new()
        .width(presentation.width)
        .height(presentation.height)
        // Load fonts from the pptx-renderer library fonts folder or path
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
        
    // 5. Convert to PNG bytes
    let png_bytes = render_to_png(&full_html, config)
        .map_err(|e| e.to_string())?;
        
    // 6. Write image output
    fs::write("slide_0.png", png_bytes).map_err(|e| e.to_string())?;
    println!("Successfully rasterized slide 0 to slide_0.png");

    Ok(())
}
```

---

## 4. Traversing Slide Nodes

If you need to inspect or manipulate slide contents programmatically, you can walk the `nodes` hierarchy. Slide groups contain nested child nodes that should be traversed recursively.

```rust
use pptx_renderer_rust::model::{Presentation, SlideNode};
use pptx_renderer_rust::parser::parse_presentation;

fn traverse_node(node: &SlideNode, depth: usize) {
    let indent = "  ".repeat(depth);
    println!("{}- [{}] Name: '{}' (Type: {}) at x: {}, y: {}", 
             indent, node.id, node.name, node.node_type, node.position.x, node.position.y);
             
    if let Some(ref text_body) = node.text_body {
        println!("{}  Text: {:?}", indent, text_body.total_text);
    }
    
    // Recursively walk groups
    if let Some(ref children) = node.children {
        for child in children {
            traverse_node(child, depth + 1);
        }
    }
}

fn main() -> Result<(), String> {
    let presentation = parse_presentation("tests/template.pptx")?;
    for slide in &presentation.slides {
        println!("\n--- Slide {} Node Tree ---", slide.index);
        for node in &slide.nodes {
            traverse_node(node, 0);
        }
    }
    Ok(())
}
```
