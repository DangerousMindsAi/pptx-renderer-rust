use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Presentation {
    pub width: u32,
    pub height: u32,
    pub slide_count: u32,
    pub slides: Vec<Slide>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Slide {
    pub index: usize,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub background: Option<Background>,
    pub nodes: Vec<SlideNode>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub layout_nodes: Vec<SlideNode>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub master_nodes: Vec<SlideNode>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Background {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alpha: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blip_embed: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_tile: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_cover: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grad_fill: Option<GradientFill>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct SlideNode {
    pub id: String,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alt_text: Option<String>,
    pub node_type: String,
    pub position: Position,
    pub size: Size,
    pub rotation: f64,
    pub flip_h: bool,
    pub flip_v: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_body: Option<TextBody>,
    
    // Group fields
    #[serde(skip_serializing_if = "Option::is_none")]
    pub children: Option<Vec<SlideNode>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub child_offset: Option<Position>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub child_extent: Option<Size>,
    
    // Shape fields
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preset_geometry: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub adjustments: Option<std::collections::HashMap<String, f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub solid_fill: Option<SolidFill>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grad_fill: Option<GradientFill>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line: Option<BorderLine>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hlink_click: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hlink_tooltip: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reflection: Option<Reflection>,
    
    // Picture fields
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blip_embed: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub crop: Option<CropRect>,
    
    // Table fields
    #[serde(skip_serializing_if = "Option::is_none")]
    pub columns: Option<Vec<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rows: Option<Vec<TableRow>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_style_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_properties: Option<TableProperties>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TableProperties {
    pub first_row: bool,
    pub first_col: bool,
    pub last_row: bool,
    pub last_col: bool,
    pub band_row: bool,
    pub band_col: bool,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ColorTransform {
    pub kind: String, // "tint", "shade", "alpha"
    pub val: f64, // e.g. 20000 -> 0.2
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct SchemeColor {
    pub scheme: String, // "dk1", "accent1", etc.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transform: Option<ColorTransform>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GradientStop {
    pub position: f64,
    pub color: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alpha: Option<f64>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GradientFill {
    pub angle: f64,
    pub stops: Vec<GradientStop>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct SolidFill {
    pub color: SchemeColor,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LineEndInfo {
    #[serde(rename = "type")]
    pub end_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub w: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub len: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct Reflection {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blur_rad: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub st_a: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub st_pos: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_a: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_pos: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dist: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dir: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fade_dir: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sy: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sx: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kx: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ky: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub algn: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rot_with_shape: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct BorderLine {
    pub width: f64,
    pub fill: SolidFill,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dash: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cap: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub join: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub head_end: Option<LineEndInfo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tail_end: Option<LineEndInfo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grad_fill: Option<GradientFill>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct TableBorders {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub left: Option<BorderLine>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub right: Option<BorderLine>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top: Option<BorderLine>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bottom: Option<BorderLine>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inside_h: Option<BorderLine>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inside_v: Option<BorderLine>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct TableStylePart {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bold: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fill: Option<SolidFill>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub borders: Option<TableBorders>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct TableStyle {
    pub style_id: String,
    pub style_name: String,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tbl_bg: Option<SolidFill>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub whole_tbl: Option<TableStylePart>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub band1_h: Option<TableStylePart>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub band1_v: Option<TableStylePart>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub band2_h: Option<TableStylePart>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub band2_v: Option<TableStylePart>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_row: Option<TableStylePart>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_col: Option<TableStylePart>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_row: Option<TableStylePart>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_col: Option<TableStylePart>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CropRect {
    pub top: f64,
    pub bottom: f64,
    pub left: f64,
    pub right: f64,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TableCell {
    pub text: String,
    pub grid_span: u32,
    pub row_span: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_body: Option<TextBody>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub margin_left: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub margin_right: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub margin_top: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub margin_bottom: Option<f64>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TableRow {
    pub height: f64,
    pub cells: Vec<TableCell>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct Position {
    pub x: f64,
    pub y: f64,
}

impl PartialEq for Position {
    fn eq(&self, other: &Self) -> bool {
        (self.x - other.x).abs() < 1e-4 && (self.y - other.y).abs() < 1e-4
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct Size {
    pub w: f64,
    pub h: f64,
}

impl PartialEq for Size {
    fn eq(&self, other: &Self) -> bool {
        (self.w - other.w).abs() < 1e-4 && (self.h - other.h).abs() < 1e-4
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct TextBody {
    pub paragraphs: Vec<TextParagraph>,
    pub total_text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub margin_left: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub margin_right: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub margin_top: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub margin_bottom: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vertical_align: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sp_auto_fit: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub norm_autofit_font_scale: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub norm_autofit_line_space_reduction: Option<f64>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct TextRun {
    pub text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub font_size: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub font_family: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bold: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub italic: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub underline: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strikethrough: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kerning: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub letter_spacing: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub baseline: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cap: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub highlight: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hlink_click: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_break: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct TextParagraph {
    pub level: u32,
    pub text: String, // Kept for backwards parity with expected_ast.json test harness logic for now
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub runs: Vec<TextRun>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub align: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub margin_left: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub indent: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bullet_char: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bullet_font: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bullet_auto_num_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bullet_color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bullet_none: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_spacing: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub space_before: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub space_after: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_para_font_size: Option<f64>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_slidenode_serialization() {
        let node = SlideNode {
            id: "1".to_string(),
            name: "Title 1".to_string(),
            alt_text: None,
            node_type: "shape".to_string(),
            position: Position { x: 10.0, y: 20.0 },
            size: Size { w: 100.0, h: 50.0 },
            rotation: 0.0,
            flip_h: false,
            flip_v: false,
            preset_geometry: None,
            adjustments: None,
            solid_fill: None,
            grad_fill: None,
            line: None,
            text_body: None,
            children: None,
            hlink_click: None,
            hlink_tooltip: None,
            reflection: None,
            child_offset: None,
            child_extent: None,
            blip_embed: None,
            crop: None,
            columns: None,
            rows: None,
            table_style_id: None,
table_properties: None,
        };

        let serialized = serde_json::to_string(&node).unwrap();
        
        let expected = json!({
            "id": "1",
            "name": "Title 1",
            "nodeType": "shape",
            "position": {"x": 10.0, "y": 20.0},
            "size": {"w": 100.0, "h": 50.0},
            "rotation": 0.0,
            "flipH": false,
            "flipV": false
        });
        
        let parsed: serde_json::Value = serde_json::from_str(&serialized).unwrap();
        assert_eq!(parsed, expected);
        assert!(!parsed.as_object().unwrap().contains_key("textBody"));
        assert!(!parsed.as_object().unwrap().contains_key("children"));
    }

    #[test]
    fn test_textrun_serialization_with_options() {
        let run = TextRun {
            text: "Hello".to_string(),
            font_size: Some(24.0),
            font_family: Some("Arial".to_string()),
            color: Some("#FF0000".to_string()),
            bold: Some(true),
            italic: None, underline: None, strikethrough: None, kerning: None, letter_spacing: None, baseline: None, cap: None, highlight: None, hlink_click: None, is_break: None,
        };
        
        let serialized = serde_json::to_string(&run).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&serialized).unwrap();
        
        assert_eq!(parsed["text"], "Hello");
        assert_eq!(parsed["fontSize"], 24.0);
        assert_eq!(parsed["fontFamily"], "Arial");
        assert_eq!(parsed["color"], "#FF0000");
        assert_eq!(parsed["bold"], true);
        assert!(!parsed.as_object().unwrap().contains_key("italic"));
    }
}
