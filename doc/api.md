# API Reference

This document provides a comprehensive reference of all public modules, entrypoints, and data structures exposed by the `pptx-renderer-rust` crate.

---

## Crate Entrypoints

### `get_default_fonts`
```rust
pub fn get_default_fonts() -> Vec<Vec<u8>>
```
Returns a list of default TrueType fonts embedded in the library. These fonts are used as system-independent fallbacks:
- Carlito (Regular, Italic, Bold, BoldItalic) - Fallback for Calibri and Montserrat.
- Arimo (Regular, Italic, Bold, BoldItalic) - Fallback for Arial.
- Tinos (Regular, Italic, Bold, BoldItalic) - Fallback for Times New Roman and Georgia.

---

## 1. Module: `parser`

Core module responsible for loading and parsing `.pptx` documents.

### `parse_presentation`
```rust
pub fn parse_presentation(path: &str) -> Result<Presentation, String>
```
Parses a PowerPoint slide presentation from a given file path.
- **Parameters**: `path` - The absolute or relative file path to the `.pptx` file.
- **Returns**: `Ok(Presentation)` on success, or `Err(String)` describing the parsing error.

---

## 2. Module: `renderer`

Core module responsible for generating CSS-styled HTML fragments of presentation slides.

### `render_slide`
```rust
pub fn render_slide(presentation: &Presentation, slide: &Slide) -> String
```
Renders a given slide to a self-contained HTML `<div>` fragment, using absolute CSS styling matching the presentation dimensions.
- **Parameters**:
  - `presentation`: Reference to the parsed `Presentation` AST containing presentation-wide metadata (width/height).
  - `slide`: Reference to the specific `Slide` to render.
- **Returns**: A `String` containing the HTML fragment.

### `render_background`
```rust
pub fn render_background(presentation: &Presentation, slide: &Slide) -> String
```
Renders the CSS background properties for a given slide. Handles solid colors, alphas, linear gradients, tiled textures, and stretch images.
- **Returns**: A `String` containing the CSS styles (e.g. `background-color: ...` or `background: linear-gradient(...)`).

### `render_node`
```rust
pub fn render_node(node: &SlideNode, list_counters: &mut std::collections::HashMap<u32, usize>) -> String
```
Recursively renders a single `SlideNode` (Shape, Picture, Table, or Group) into an HTML fragment.
- **Parameters**:
  - `node`: The node to render.
  - `list_counters`: A mutable map tracking numbered lists (e.g., `<ol>` level counters).

---

## 3. Module: `model`

Contains the strongly-typed AST representing presentations, slides, and layout nodes.

### `Presentation`
Represents the root of a parsed PowerPoint file.
```rust
pub struct Presentation {
    pub width: u32,             // Presentation width in pixels (converted from EMUs)
    pub height: u32,            // Presentation height in pixels (converted from EMUs)
    pub slide_count: u32,       // Number of slides in the presentation
    pub slides: Vec<Slide>,     // Slide elements in order of appearance
}
```

### `Slide`
Represents a single slide container, including nodes inherited from layouts and master slides.
```rust
pub struct Slide {
    pub index: usize,                 // 1-based index of the slide
    pub notes: Option<String>,        // Speaker notes associated with the slide
    pub background: Option<Background>, // Slide-specific background settings
    pub nodes: Vec<SlideNode>,        // Content nodes defined on this slide
    pub layout_nodes: Vec<SlideNode>, // Placeholder nodes inherited from Slide Layout
    pub master_nodes: Vec<SlideNode>, // Placeholder/decoration nodes inherited from Slide Master
}
```

### `Background`
Defines the visual background of a slide.
```rust
pub struct Background {
    pub color: Option<String>,       // Hex color (e.g. "#FF0000")
    pub alpha: Option<f64>,          // Opacity value (0.0 to 1.0)
    pub blip_embed: Option<String>,  // Base64 image data URI for background image
    pub is_tile: Option<bool>,       // True if background image should tile
    pub is_cover: Option<bool>,      // True if background image should scale as cover
    pub grad_fill: Option<GradientFill>, // Optional linear gradient fill
}
```

### `SlideNode`
Represent a layout item on a slide. Can act as a shape, picture, table, or group of sub-nodes.
```rust
pub struct SlideNode {
    pub id: String,                     // Unique identifier (from OpenXML cNvPr)
    pub name: String,                   // Name of the node
    pub alt_text: Option<String>,       // Optional alt text
    pub node_type: String,              // Node type: "shape", "picture", "table", or "group"
    pub position: Position,             // X and Y coordinates (in px)
    pub size: Size,                     // Width and Height dimensions (in px)
    pub rotation: f64,                  // Rotation angle in degrees
    pub flip_h: bool,                   // True if flipped horizontally
    pub flip_v: bool,                   // True if flipped vertically
    pub text_body: Option<TextBody>,    // Text content overlay inside the node
    
    // Group fields
    pub children: Option<Vec<SlideNode>>, // Child nodes inside a group container
    pub child_offset: Option<Position>,  // Internal coordinate offset for groups
    pub child_extent: Option<Size>,      // Internal scale extent for groups
    
    // Shape fields
    pub preset_geometry: Option<String>, // Shape type preset (e.g., "rect", "ellipse")
    pub adjustments: Option<HashMap<String, f64>>, // Shape adjustment overrides (e.g. arrow heads)
    pub solid_fill: Option<SolidFill>,   // Shape background color
    pub grad_fill: Option<GradientFill>, // Shape background gradient
    pub line: Option<BorderLine>,        // Shape border outline properties
    pub hlink_click: Option<String>,     // Action hyperlink URL (e.g. "http://..." or "#slide2")
    pub hlink_tooltip: Option<String>,   // Optional link tooltip
    pub reflection: Option<Reflection>,   // Reflection style settings
    
    // Picture fields
    pub blip_embed: Option<String>,      // Base64 Data URI for picture contents
    pub crop: Option<CropRect>,          // Image crop percentages
    
    // Table fields
    pub columns: Option<Vec<f64>>,       // Column widths (in px)
    pub rows: Option<Vec<TableRow>>,     // Rows containing table cells
    pub table_style_id: Option<String>,  // Guid of applied table style
    pub table_properties: Option<TableProperties>, // Table layout properties (e.g., band_row, first_row)
}
```

### `TextBody`
Holds structural paragraphs and alignment properties for text boxes.
```rust
pub struct TextBody {
    pub paragraphs: Vec<TextParagraph>,
    pub total_text: String,
    pub margin_left: Option<f64>,
    pub margin_right: Option<f64>,
    pub margin_top: Option<f64>,
    pub margin_bottom: Option<f64>,
    pub vertical_align: Option<String>, // "t" (top), "ctr" (center), "b" (bottom)
    pub sp_auto_fit: Option<bool>,
    pub norm_autofit_font_scale: Option<f64>,
    pub norm_autofit_line_space_reduction: Option<f64>,
}
```

### `TextParagraph`
```rust
pub struct TextParagraph {
    pub level: u32,                  // List level indentation index (0 to 8)
    pub text: String,                // Unformatted paragraph text
    pub runs: Vec<TextRun>,          // Inline text runs making up the paragraph
    pub align: Option<String>,       // "ctr", "r", "just", "l"
    pub margin_left: Option<f64>,    // Left indentation margin
    pub indent: Option<f64>,         // Bullet indentation offset
    pub bullet_char: Option<String>, // String representation of bullet character
    pub bullet_font: Option<String>, // Bullet typeface
    pub bullet_auto_num_type: Option<String>, // e.g. "arabicPeriod"
    pub bullet_color: Option<String>,
    pub bullet_none: Option<bool>,
}
```

### `TextRun`
A segment of inline text with uniform character formatting.
```rust
pub struct TextRun {
    pub text: String,
    pub font_size: Option<f64>,      // Font size in points
    pub font_family: Option<String>,  // Font typeface name
    pub color: Option<String>,        // Hex color value
    pub bold: Option<bool>,
    pub italic: Option<bool>,
    pub underline: Option<bool>,
    pub strikethrough: Option<bool>,
    pub highlight: Option<String>,    // Background highlight color
    pub hlink_click: Option<String>,  // Inline link target
    pub is_break: Option<bool>,       // True if this run is a line break (<br/>)
}
```

---

## 4. Module: `xml`

Exposes utility structures for reading openxml nodes.

### `XmlNode`
A simplified XML representation suited for traversing OpenXML node hierarchies.
```rust
pub struct XmlNode {
    pub tag: String,
    pub attributes: HashMap<String, String>,
    pub children: Vec<XmlNode>,
    pub text: Option<String>,
}

impl XmlNode {
    // Parse raw XML string into an XmlNode tree.
    pub fn parse(xml: &str) -> Result<Self, String>;
    
    // Find the first child node matching the tag name (ignoring namespaces).
    pub fn child(&self, tag: &str) -> Option<&XmlNode>;
    
    // Iterate over children matching the tag name (ignoring namespaces).
    pub fn children(&self, tag: &str) -> impl Iterator<Item = &XmlNode>;
    
    // Retrieve value of a node attribute.
    pub fn attr(&self, key: &str) -> Option<&String>;
    
    // Parse an attribute value into a floating-point number.
    pub fn num_attr(&self, key: &str) -> Option<f64>;
}
```

---

## 5. Helper Modules

### `shapes_presets`
```rust
pub fn get_preset_shape_path(
    geom: &str, 
    w: f64, 
    h: f64, 
    adjustments: Option<&HashMap<String, f64>>
) -> String
```
Computes and returns the SVG path command `d` attribute for a given shape geometry (e.g. `rect`, `ellipse`, `diamond`, `rightArrow`) of dimensions `w` and `h` with optional shape parameters.

### `table_styles`
```rust
pub fn get_predefined_table_style(style_id: &str) -> Option<TableStyle>
```
Retrieves a matching predefined table style containing colors, borders, and fonts based on its Guid.
