use std::collections::HashMap;

/// Helper: get adjustment value or default, converting from 100000ths to fraction.
fn adj(adjustments: Option<&HashMap<String, f64>>, name: &str, default_val: f64) -> f64 {
    let raw = adjustments.and_then(|a| a.get(name)).copied().unwrap_or(default_val);
    raw / 100000.0
}

/// Raw adj helper: get adjustment value without dividing by 100000.
fn adj_raw(adjustments: Option<&HashMap<String, f64>>, name: &str, default_val: f64) -> f64 {
    adjustments.and_then(|a| a.get(name)).copied().unwrap_or(default_val)
}

/// Helper: generate a star polygon.
fn star_shape(w: f64, h: f64, points: usize, inner_ratio: f64) -> String {
    let cx = w / 2.0;
    let cy = h / 2.0;
    let outer_rx = w / 2.0;
    let outer_ry = h / 2.0;
    let inner_rx = outer_rx * inner_ratio;
    let inner_ry = outer_ry * inner_ratio;
    let total_points = points * 2;
    let mut parts = Vec::new();

    for i in 0..total_points {
        let angle = (2.0 * std::f64::consts::PI * (i as f64)) / (total_points as f64) - std::f64::consts::PI / 2.0;
        let is_outer = i % 2 == 0;
        let rx = if is_outer { outer_rx } else { inner_rx };
        let ry = if is_outer { outer_ry } else { inner_ry };
        let x = cx + rx * angle.cos();
        let y = cy + ry * angle.sin();
        if i == 0 {
            parts.push(format!("M{},{}", x, y));
        } else {
            parts.push(format!("L{},{}", x, y));
        }
    }
    parts.push("Z".to_string());
    parts.join(" ")
}

pub fn get_preset_shape_path(
    preset_geometry: &str,
    w: f64,
    h: f64,
    adjustments: Option<&HashMap<String, f64>>,
) -> String {
    let safe_w = if w == 0.0 { 1.0 } else { w };
    let safe_h = if h == 0.0 { 1.0 } else { h };

    match preset_geometry.to_lowercase().as_str() {
        // Basic Rectangles
        "rect" => format!("M0,0 L{},0 L{},{} L0,{} Z", w, w, h, h),
        "roundrect" => {
            let a = adj(adjustments, "adj", 16667.0);
            let r = w.min(h) * a;
            format!(
                "M{r},0 L{w_r},0 A{r},{r} 0 0,1 {w},{r} L{w},{h_r} A{r},{r} 0 0,1 {w_r},{h} L{r},{h} A{r},{r} 0 0,1 0,{h_r} L0,{r} A{r},{r} 0 0,1 {r},0 Z",
                r = r, w = w, h = h, w_r = w - r, h_r = h - r
            )
        }
        
        // Ellipses & Circles
        "ellipse" => {
            let rx = w / 2.0;
            let ry = h / 2.0;
            format!("M{},{} A{},{} 0 1,1 0,{} A{},{} 0 1,1 {},{} Z", w, ry, rx, ry, ry, rx, ry, w, ry)
        }
        
        // Triangles
        "triangle" | "isostriangle" => {
            let a = adj(adjustments, "adj", 50000.0);
            let top_x = w * a;
            format!("M{},0 L{},{} L0,{} Z", top_x, w, h, h)
        }
        "rttriangle" => format!("M0,0 L{},{} L0,{} Z", w, h, h),
        
        // Diamonds
        "diamond" => {
            let cx = w / 2.0;
            let cy = h / 2.0;
            format!("M{},0 L{},{} L{},{} L0,{} Z", cx, w, cy, cx, h, cy)
        }
        
        // Lines and Connectors
        "line" | "straightconnector1" => {
            if w == 0.0 {
                format!("M0.5,0 L0.5,{}", safe_h)
            } else if h == 0.0 {
                format!("M0,0.5 L{},0.5", safe_w)
            } else {
                format!("M0,0 L{},{}", w, h)
            }
        }
        "lineinv" => {
            if w == 0.0 {
                format!("M0.5,0 L0.5,{}", safe_h)
            } else if h == 0.0 {
                format!("M0,0.5 L{},0.5", safe_w)
            } else {
                format!("M{},0 L0,{}", w, h)
            }
        }
        "bentconnector2" => format!("M0,0 L{},0 L{},{}", w, w, h),
        "bentconnector3" => {
            let a = adj(adjustments, "adj1", 50000.0);
            let mid_x = w * a;
            format!("M0,0 L{},0 L{},{} L{},{}", mid_x, mid_x, h, w, h)
        }
        "curvedconnector2" => format!("M0,0 C{},0 0,{} {},{}", w, h, w, h),
        
        // Arrows
        "rightarrow" => {
            let a1 = adj(adjustments, "adj1", 50000.0); // shaft width ratio
            let a2 = adj(adjustments, "adj2", 50000.0); // head length ratio
            let ss = w.min(h);
            let shaft_half_h = (h * a1) / 2.0;
            let head_len = ss * a2;
            let cy = h / 2.0;
            let shaft_end = w - head_len;
            format!(
                "M0,{} L{},{} L{},0 L{},{} L{},{} L{},{} L0,{} Z",
                cy - shaft_half_h, shaft_end, cy - shaft_half_h, shaft_end, w, cy, shaft_end, h, shaft_end, cy + shaft_half_h, cy + shaft_half_h
            )
        }
        "leftarrow" => {
            let a1 = adj(adjustments, "adj1", 50000.0);
            let a2 = adj(adjustments, "adj2", 50000.0);
            let ss = w.min(h);
            let shaft_half_h = (h * a1) / 2.0;
            let head_len = ss * a2;
            let cy = h / 2.0;
            format!(
                "M{},{} L{},{} L{},0 L0,{} L{},{} L{},{} L{},{} Z",
                w, cy - shaft_half_h, head_len, cy - shaft_half_h, head_len, cy, head_len, h, head_len, cy + shaft_half_h, w, cy + shaft_half_h
            )
        }
        "uparrow" => {
            let a1 = adj(adjustments, "adj1", 50000.0);
            let a2 = adj(adjustments, "adj2", 50000.0);
            let shaft_half_w = (w * a1) / 2.0;
            let head_len = h * a2;
            let cx = w / 2.0;
            format!(
                "M{},{} L{},{} L0,{} L{},0 L{},{} L{},{} L{},{} Z",
                cx - shaft_half_w, h, cx - shaft_half_w, head_len, head_len, cx, w, head_len, cx + shaft_half_w, head_len, cx + shaft_half_w, h
            )
        }
        "downarrow" => {
            let a1 = adj(adjustments, "adj1", 50000.0);
            let a2 = adj(adjustments, "adj2", 50000.0);
            let shaft_half_w = (w * a1) / 2.0;
            let head_len = h * a2;
            let cx = w / 2.0;
            let shaft_end = h - head_len;
            format!(
                "M{},0 L{},0 L{},{} L{},{} L{},{} L0,{} L{},{} Z",
                cx - shaft_half_w, cx + shaft_half_w, cx + shaft_half_w, shaft_end, w, shaft_end, cx, h, shaft_end, cx - shaft_half_w, shaft_end
            )
        }
        
        // Stars
        "star4" => {
            let a = adj(adjustments, "adj", 12500.0) * 2.0;
            star_shape(w, h, 4, a.max(0.0).min(1.0))
        }
        "star5" => {
            let a_raw = adj_raw(adjustments, "adj", 19098.0);
            let a = a_raw.max(0.0).min(50000.0);
            let hf = 105146.0;
            let vf = 110557.0;
            let swd2 = ((w / 2.0) * hf) / 100000.0;
            let shd2 = ((h / 2.0) * vf) / 100000.0;
            let svc = ((h / 2.0) * vf) / 100000.0;
            let iwd2 = (swd2 * a) / 50000.0;
            let ihd2 = (shd2 * a) / 50000.0;
            let cx = w / 2.0;
            let step = (2.0 * std::f64::consts::PI) / 5.0;
            let half_step = step / 2.0;
            let start_angle = -std::f64::consts::PI / 2.0;
            let mut parts = Vec::new();
            for i in 0..5 {
                let outer_angle = start_angle + step * (i as f64);
                let inner_angle = outer_angle + half_step;
                let ox = cx + swd2 * outer_angle.cos();
                let oy = svc + shd2 * outer_angle.sin();
                let ix = cx + iwd2 * inner_angle.cos();
                let iy = svc + ihd2 * inner_angle.sin();
                if i == 0 {
                    parts.push(format!("M{},{}", ox, oy));
                } else {
                    parts.push(format!("L{},{}", ox, oy));
                }
                parts.push(format!("L{},{}", ix, iy));
            }
            parts.push("Z".to_string());
            parts.join(" ")
        }
        "star6" => {
            let a_raw = adj_raw(adjustments, "adj", 28868.0);
            let a = a_raw.max(0.0).min(50000.0);
            let hf = 115470.0;
            let swd2 = ((w / 2.0) * hf) / 100000.0;
            let shd2 = h / 2.0;
            let iwd2 = (swd2 * a) / 50000.0;
            let ihd2 = (shd2 * a) / 50000.0;
            let cx = w / 2.0;
            let cy = h / 2.0;
            let step = (2.0 * std::f64::consts::PI) / 6.0;
            let half_step = step / 2.0;
            let start_angle = -std::f64::consts::PI / 2.0;
            let mut parts = Vec::new();
            for i in 0..6 {
                let outer_angle = start_angle + step * (i as f64);
                let inner_angle = outer_angle + half_step;
                let ox = cx + swd2 * outer_angle.cos();
                let oy = cy + shd2 * outer_angle.sin();
                let ix = cx + iwd2 * inner_angle.cos();
                let iy = cy + ihd2 * inner_angle.sin();
                if i == 0 {
                    parts.push(format!("M{},{}", ox, oy));
                } else {
                    parts.push(format!("L{},{}", ox, oy));
                }
                parts.push(format!("L{},{}", ix, iy));
            }
            parts.push("Z".to_string());
            parts.join(" ")
        }
        "star8" => {
            let a = adj(adjustments, "adj", 37500.0) * 2.0;
            star_shape(w, h, 8, a.max(0.0).min(1.0))
        }
        "star10" => {
            let a = adj(adjustments, "adj", 37500.0) * 2.0;
            star_shape(w, h, 10, a.max(0.0).min(1.0))
        }
        
        // Default to Rectangle bounding box for unimplemented shapes
        _ => format!("M0,0 L{},0 L{},{} L0,{} Z", w, w, h, h),
    }
}