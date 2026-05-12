pub mod model;
pub mod parser;
pub mod xml;
pub mod renderer;
pub mod shapes_presets;
pub mod table_styles;

pub fn get_default_fonts() -> Vec<Vec<u8>> {
    vec![
        include_bytes!("fonts/Carlito-Regular.ttf").to_vec(),
        include_bytes!("fonts/Carlito-Italic.ttf").to_vec(),
        include_bytes!("fonts/Carlito-Bold.ttf").to_vec(),
        include_bytes!("fonts/Carlito-BoldItalic.ttf").to_vec(),
        include_bytes!("fonts/Arimo-Regular.ttf").to_vec(),
        include_bytes!("fonts/Arimo-Italic.ttf").to_vec(),
        include_bytes!("fonts/Arimo-Bold.ttf").to_vec(),
        include_bytes!("fonts/Arimo-BoldItalic.ttf").to_vec(),
        include_bytes!("fonts/Tinos-Regular.ttf").to_vec(),
        include_bytes!("fonts/Tinos-Italic.ttf").to_vec(),
        include_bytes!("fonts/Tinos-Bold.ttf").to_vec(),
        include_bytes!("fonts/Tinos-BoldItalic.ttf").to_vec(),
    ]
}
