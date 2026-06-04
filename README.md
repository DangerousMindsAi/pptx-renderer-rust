# pptx-renderer-rust

A high-performance Rust library for parsing Office Open XML (.pptx) presentation files into a structured data model and rendering slides to HTML and PNG formats.

## Key Features

- **Robust Parsing**: Extracts slides, layouts, master slides, shapes, pictures, tables, groups, and text bodies (including rich formatting like margins, alignment, bullets, line spacing, etc.).
- **Theme-aware Resolution**: Correctly resolves properties (fonts, colors, placeholders) by traversing the slide layout and slide master hierarchy.
- **HTML Slide Rendering**: Generates clean, CSS-styled HTML representations of slides with full support for vector shapes (SVG), text positioning, table structures, and groups.
- **PNG Rasterization**: Integrates seamlessly with `hyper-render` to rasterize rendered slides into PNG images.
- **Standalone XML Utilities**: Utilizes a fast, stack-based `XmlNode` parser built on top of `quick-xml`.

## Documentation

For comprehensive guides and references, explore the following documentation:

- [**Architectural Overview**](doc/architecture.md): Deep-dive into the parsing pipeline, rendering architecture, and style inheritance model.
- [**API Reference**](doc/api.md): Complete module and type documentation, including `Presentation`, `Slide`, `SlideNode`, and core functions.
- [**User Guide & Examples**](doc/user_guide.md): Practical code examples demonstrating parsing, rendering slides to HTML, and configuring custom fonts for PNG rasterization.

## Quick Start

Add `pptx-renderer-rust` to your `Cargo.toml`:

```toml
[dependencies]
pptx-renderer-rust = { path = "path/to/pptx-renderer-rust" }
```

### Example: Parse and Render a Slide

Here is a quick example of parsing a PPTX file and rendering its first slide to HTML:

```rust
use pptx_renderer_rust::parser::parse_presentation;
use pptx_renderer_rust::renderer::render_slide;

fn main() -> Result<(), String> {
    // 1. Parse the presentation
    let presentation = parse_presentation("sample.pptx")?;
    println!("Parsed presentation with {} slides.", presentation.slides.len());

    // 2. Render the first slide to HTML
    if let Some(first_slide) = presentation.slides.first() {
        let html_content = render_slide(&presentation, first_slide);
        println!("Generated HTML:\n{}", html_content);
    }

    Ok(())
}
```

## License

This project is proprietary and confidential. All rights reserved.
