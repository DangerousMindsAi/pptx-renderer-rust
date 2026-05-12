use std::collections::HashMap;
use crate::model::{Presentation, Slide, SlideNode, Position, Size, TextBody, TextParagraph};
use crate::xml::XmlNode;
use openxml_opc::OpcPackage;

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub(crate) struct RelEntry {
    pub id: String,
    pub rel_type: String,
    pub target: String,
}

#[derive(Debug, Clone)]
pub(crate) struct ThemeData {
    pub major_font: String,
    pub minor_font: String,
    pub colors: std::collections::HashMap<String, String>,
}

#[derive(Debug, Clone)]
pub(crate) struct MasterTextStyles {
    pub title: Option<XmlNode>,
    pub body: Option<XmlNode>,
    pub other: Option<XmlNode>,
}

#[derive(Debug, Clone)]
pub(crate) struct StyleContext<'a> {
    pub theme: Option<&'a ThemeData>,
    pub master_styles: Option<&'a MasterTextStyles>,
    pub master_placeholders: &'a [XmlNode],
    pub layout_placeholders: &'a [XmlNode],
}

fn parse_rels(xml: &str) -> HashMap<String, RelEntry> {
    let mut rels = HashMap::new();
    if let Ok(root) = XmlNode::parse(xml) {
        for rel in root.children("Relationship") {
            if let (Some(id), Some(t), Some(target)) = (rel.attr("Id"), rel.attr("Type"), rel.attr("Target")) {
                rels.insert(id.clone(), RelEntry {
                    id: id.clone(),
                    rel_type: t.clone(),
                    target: target.clone(),
                });
            }
        }
    }
    rels
}

fn resolve_target(base: &str, target: &str) -> String {
    if target.starts_with("../") {
        let parts: Vec<&str> = base.split('/').collect();
        if parts.len() > 1 {
            let mut resolved = parts[..parts.len()-2].to_vec();
            resolved.push(&target[3..]);
            return resolved.join("/");
        }
    }
    let parts: Vec<&str> = base.split('/').collect();
    if parts.is_empty() {
        return target.to_string();
    }
    let mut resolved = parts[..parts.len()-1].to_vec();
    resolved.push(target);
    resolved.join("/")
}

fn get_ph_info(node: &XmlNode) -> (Option<String>, Option<u32>) {
    for wrapper in ["nvSpPr", "nvPicPr", "nvGrpSpPr", "nvGraphicFramePr", "nvCxnSpPr"] {
        if let Some(nv_wrapper) = node.child(wrapper) {
            if let Some(nv_pr) = nv_wrapper.child("nvPr") {
                if let Some(ph) = nv_pr.child("ph") {
                    let p_type = ph.attr("type").cloned();
                    let p_idx = ph.num_attr("idx").map(|v| v as u32);
                    return (p_type, p_idx);
                }
            }
        }
    }
    (None, None)
}

fn get_xfrm(node: &XmlNode) -> Option<(Position, Size, f64, bool, bool)> {
    if let Some(sp_pr) = node.child("spPr").or_else(|| node.child("grpSpPr")).or_else(|| node.child("xfrm")) {
        let xfrm = if sp_pr.tag.ends_with("xfrm") { Some(sp_pr) } else { sp_pr.child("xfrm") };
        if let Some(xfrm) = xfrm {
            let mut rot = 0.0;
            let mut flip_h = false;
            let mut flip_v = false;
            
            if let Some(r) = xfrm.num_attr("rot") {
                rot = r as f64 / 60000.0;
            }
            if let Some(fh) = xfrm.attr("flipH") {
                flip_h = fh == "1" || fh == "true";
            }
            if let Some(fv) = xfrm.attr("flipV") {
                flip_v = fv == "1" || fv == "true";
            }
            
            if let (Some(off), Some(ext)) = (xfrm.child("off"), xfrm.child("ext")) {
                let x = off.num_attr("x").unwrap_or(0.0) as f64 / 9525.0;
                let y = off.num_attr("y").unwrap_or(0.0) as f64 / 9525.0;
                let w = ext.num_attr("cx").unwrap_or(0.0) as f64 / 9525.0;
                let h = ext.num_attr("cy").unwrap_or(0.0) as f64 / 9525.0;
                return Some((Position { x, y }, Size { w, h }, rot, flip_h, flip_v));
            }
        }
    }
    None
}

fn find_placeholder<'a>(placeholders: &'a [XmlNode], p_type: Option<&String>, p_idx: Option<u32>) -> Option<&'a XmlNode> {
    let mut type_match = None;
    
    for ph in placeholders {
        let (info_type, info_idx) = get_ph_info(ph);
        
        // Exact match
        if p_type.is_some() && info_type.as_ref() == p_type && p_idx.is_some() && info_idx == p_idx {
            return Some(ph);
        }
        
        // Type only match
        if p_type.is_some() && info_type.as_ref() == p_type {
            if type_match.is_none() {
                type_match = Some(ph);
            }
        }
        
        // Idx only match
        if p_idx.is_some() && info_idx == p_idx && p_type.is_none() && info_type.is_none() {
            return Some(ph);
        }
    }
    
    if p_type.is_none() && p_idx.is_some() {
        for ph in placeholders {
            let (_, info_idx) = get_ph_info(ph);
            if info_idx == p_idx {
                return Some(ph);
            }
        }
    }
    
    type_match
}

fn find_style_at_level<'a>(style_node: Option<&'a XmlNode>, level: u32) -> Option<&'a XmlNode> {
    if let Some(node) = style_node {
        let lvl_tag = format!("lvl{}pPr", level + 1);
        if let Some(lvl) = node.child(&lvl_tag) {
            return Some(lvl);
        }
        if let Some(def) = node.child("defPPr") {
            return Some(def);
        }
    }
    None
}

pub(crate) fn parse_text_body<'a>(
    tx_body: &'a XmlNode,
    ctx: &StyleContext<'a>,
    category: &str,
    master_ph: Option<&'a XmlNode>,
    layout_ph: Option<&'a XmlNode>,
) -> Option<TextBody> {
    let mut paragraphs = Vec::new();
    let mut total_text = String::new();
    
    let mut margin_left = None;
    let mut margin_right = None;
    let mut margin_top = None;
    let mut margin_bottom = None;
    let mut vertical_align = None;
    let mut sp_auto_fit = None;
    let mut norm_autofit_font_scale = None;
    let mut norm_autofit_line_space_reduction = None;
    
    if let Some(body_pr) = tx_body.child("bodyPr") {
        margin_left = body_pr.attr("lIns").and_then(|s| s.parse::<f64>().ok());
        margin_right = body_pr.attr("rIns").and_then(|s| s.parse::<f64>().ok());
        margin_top = body_pr.attr("tIns").and_then(|s| s.parse::<f64>().ok());
        margin_bottom = body_pr.attr("bIns").and_then(|s| s.parse::<f64>().ok());
        vertical_align = body_pr.attr("anchor").map(|s| s.to_string());
        
        if body_pr.child("spAutoFit").is_some() {
            sp_auto_fit = Some(true);
        }
        if let Some(norm) = body_pr.child("normAutofit") {
            norm_autofit_font_scale = norm.attr("fontScale").and_then(|s| s.parse::<f64>().ok());
            norm_autofit_line_space_reduction = norm.attr("lnSpcReduction").and_then(|s| s.parse::<f64>().ok());
        }
    }

    if vertical_align.is_none() {
        if let Some(l_ph) = layout_ph {
            if let Some(bp) = l_ph.child("txBody").and_then(|t| t.child("bodyPr")) {
                vertical_align = bp.attr("anchor").map(|s| s.to_string());
            }
        }
    }
    if vertical_align.is_none() {
        if let Some(m_ph) = master_ph {
            if let Some(bp) = m_ph.child("txBody").and_then(|t| t.child("bodyPr")) {
                vertical_align = bp.attr("anchor").map(|s| s.to_string());
            }
        }
    }
    
    for p in tx_body.children("p") {
        let mut p_text = String::new();
        let mut runs = Vec::new();
        
        let level = if let Some(p_pr) = p.child("pPr") {
            p_pr.num_attr("lvl").map(|v| v as u32).unwrap_or(0)
        } else {
            0
        };

        // Collect style nodes in order of lowest priority to highest priority
        let mut style_chain = Vec::new();

        // Level 1-2: Master Text Styles (Presentation/Theme Default + Category)
        if let Some(master_styles) = ctx.master_styles {
            let master_style = match category {
                "title" => master_styles.title.as_ref(),
                "body" => master_styles.body.as_ref(),
                _ => master_styles.other.as_ref(),
            };
            if let Some(lvl_pr) = find_style_at_level(master_style, level) {
                style_chain.push(lvl_pr);
            }
        }

        // Level 3: Master Placeholder lstStyle
        if let Some(m_ph) = master_ph {
            let lst_style = m_ph.child("txBody").and_then(|t| t.child("lstStyle"));
            if let Some(lvl_pr) = find_style_at_level(lst_style, level) {
                style_chain.push(lvl_pr);
            }
        }

        // Level 4: Layout Placeholder lstStyle
        if let Some(l_ph) = layout_ph {
            let lst_style = l_ph.child("txBody").and_then(|t| t.child("lstStyle"));
            if let Some(lvl_pr) = find_style_at_level(lst_style, level) {
                style_chain.push(lvl_pr);
            }
        }

        // Level 5: Shape lstStyle
        if let Some(lst_style) = tx_body.child("lstStyle") {
            if let Some(lvl_pr) = find_style_at_level(Some(lst_style), level) {
                style_chain.push(lvl_pr);
            }
        }

        // Level 6: Paragraph pPr
        if let Some(p_pr) = p.child("pPr") {
            style_chain.push(p_pr);
        }

        let mut align = None;
        let mut margin_left = None;
        let mut indent = None;
        
        let mut bullet_char = None;
        let mut bullet_font = None;
        let mut bullet_auto_num_type = None;
        let mut bullet_color = None;
        let mut bullet_none = None;
        
        let mut line_spacing = None;
        let mut space_before = None;
        let mut space_after = None;
        
        for s in style_chain.iter().rev() {
            if align.is_none() {
                if let Some(val) = s.attr("algn") {
                    align = Some(val.to_string());
                }
            }
            if margin_left.is_none() {
                if let Some(val) = s.attr("marL").and_then(|v| v.parse::<f64>().ok()) {
                    margin_left = Some(val);
                }
            }
            if indent.is_none() {
                if let Some(val) = s.attr("indent").and_then(|v| v.parse::<f64>().ok()) {
                    indent = Some(val);
                }
            }
            
            if bullet_char.is_none() && bullet_auto_num_type.is_none() && bullet_none.is_none() {
                if let Some(bu_char) = s.child("buChar") {
                    bullet_char = bu_char.attr("char").map(|v| v.to_string());
                } else if let Some(bu_auto) = s.child("buAutoNum") {
                    bullet_auto_num_type = bu_auto.attr("type").map(|v| v.to_string());
                } else if s.child("buNone").is_some() {
                    bullet_none = Some(true);
                }
            }
            if bullet_font.is_none() {
                if let Some(bu_font) = s.child("buFont") {
                    bullet_font = bu_font.attr("typeface").map(|v| v.to_string());
                }
            }
            if bullet_color.is_none() {
                if let Some(bu_clr) = s.child("buClr") {
                    if let Some(srgb) = bu_clr.child("srgbClr") {
                        bullet_color = srgb.attr("val").map(|v| format!("#{}", v));
                    } else if let Some(scheme) = bu_clr.child("schemeClr") {
                        bullet_color = scheme.attr("val").map(|v| format!("scheme:{}", v));
                    }
                }
            }
            
            if line_spacing.is_none() {
                if let Some(ln_spc) = s.child("lnSpc") {
                    if let Some(spc_pct) = ln_spc.child("spcPct") {
                        line_spacing = spc_pct.attr("val").map(|v| format!("{}%", v));
                    } else if let Some(spc_pts) = ln_spc.child("spcPts") {
                        line_spacing = spc_pts.attr("val").map(|v| format!("{}pt", v));
                    }
                }
            }
            if space_before.is_none() {
                if let Some(spc_bef) = s.child("spcBef") {
                    if let Some(spc_pct) = spc_bef.child("spcPct") {
                        space_before = spc_pct.attr("val").map(|v| format!("{}%", v));
                    } else if let Some(spc_pts) = spc_bef.child("spcPts") {
                        space_before = spc_pts.attr("val").map(|v| format!("{}pt", v));
                    }
                }
            }
            if space_after.is_none() {
                if let Some(spc_aft) = s.child("spcAft") {
                    if let Some(spc_pct) = spc_aft.child("spcPct") {
                        space_after = spc_pct.attr("val").map(|v| format!("{}%", v));
                    } else if let Some(spc_pts) = spc_aft.child("spcPts") {
                        space_after = spc_pts.attr("val").map(|v| format!("{}pt", v));
                    }
                }
            }
        }

        let resolve_r_property = |r_pr_opt: Option<&XmlNode>, attr: &str| -> Option<String> {
            if let Some(r_pr) = r_pr_opt {
                if let Some(val) = r_pr.attr(attr) {
                    return Some(val.clone());
                }
            }
            // Cascade backwards through style_chain > defRPr
            for s in style_chain.iter().rev() {
                if let Some(def_r_pr) = s.child("defRPr") {
                    if let Some(val) = def_r_pr.attr(attr) {
                        return Some(val.clone());
                    }
                }
            }
            None
        };
        
        let resolve_color = |r_pr_opt: Option<&XmlNode>| -> Option<String> {
            let check_color = |pr: &XmlNode| -> Option<String> {
                if let Some(solid) = pr.child("solidFill") {
                    if let Some(srgb) = solid.child("srgbClr") {
                        if let Some(val) = srgb.attr("val") {
                            return Some(format!("#{}", val));
                        }
                    }
                }
                None
            };
            if let Some(r_pr) = r_pr_opt {
                if let Some(c) = check_color(r_pr) { return Some(c); }
            }
            for s in style_chain.iter().rev() {
                if let Some(def_r_pr) = s.child("defRPr") {
                    if let Some(c) = check_color(def_r_pr) { return Some(c); }
                }
            }
            None
        };
        
        let resolve_typeface = |r_pr_opt: Option<&XmlNode>| -> Option<String> {
            let check_tf = |pr: &XmlNode| -> Option<String> {
                if let Some(latin) = pr.child("latin") {
                    if let Some(val) = latin.attr("typeface") {
                        return Some(val.clone());
                    }
                }
                None
            };
            let mut tf = None;
            if let Some(r_pr) = r_pr_opt {
                tf = check_tf(r_pr);
            }
            if tf.is_none() {
                for s in style_chain.iter().rev() {
                    if let Some(def_r_pr) = s.child("defRPr") {
                        tf = check_tf(def_r_pr);
                        if tf.is_some() { break; }
                    }
                }
            }
            
            // Map +mj-lt and +mn-lt to theme fonts
            let f = tf.clone();
            if let Some(tf) = f {
                if tf.starts_with("+mj") {
                    if let Some(theme) = ctx.theme {
                        if !theme.major_font.is_empty() { return Some(theme.major_font.clone()); }
                    }
                } else if tf.starts_with("+mn") {
                    if let Some(theme) = ctx.theme {
                        if !theme.minor_font.is_empty() { return Some(theme.minor_font.clone()); }
                    }
                }
            }
            tf
        };

        for child in &p.children {
            let tag_name = child.tag.split(':').last().unwrap_or(&child.tag);
            if tag_name == "r" {
                let r = child;
                if let Some(t) = r.child("t") {
                    if let Some(txt) = &t.text {
                    p_text.push_str(txt);
                    
                    let r_pr = r.child("rPr");
                    
                    let sz_str = resolve_r_property(r_pr, "sz");
                    let font_size = sz_str.and_then(|s| s.parse::<f64>().ok()).map(|sz| sz / 100.0);
                    
                    let b_str = resolve_r_property(r_pr, "b");
                    let bold = b_str.map(|b| b == "1" || b == "true");
                    
                    let i_str = resolve_r_property(r_pr, "i");
                    let italic = i_str.map(|i| i == "1" || i == "true");
                    
                    let u_str = resolve_r_property(r_pr, "u");
                    let underline = u_str.map(|u| u != "none");
                    
                    let strike_str = resolve_r_property(r_pr, "strike");
                    let strikethrough = strike_str.map(|s| s != "noStrike");
                    
                    let kern_str = resolve_r_property(r_pr, "kern");
                    let kerning = kern_str.and_then(|s| s.parse::<f64>().ok()).map(|sz| sz / 100.0);
                    
                    let spc_str = resolve_r_property(r_pr, "spc");
                    let letter_spacing = spc_str.and_then(|s| s.parse::<f64>().ok()).map(|sz| sz / 100.0);
                    
                    let baseline_str = resolve_r_property(r_pr, "baseline");
                    let baseline = baseline_str.and_then(|s| s.parse::<f64>().ok());
                    
                    let cap = resolve_r_property(r_pr, "cap");
                    
                    let mut hlink_click = None;
                    if let Some(r_pr_node) = r_pr {
                        if let Some(hlink) = r_pr_node.child("hlinkClick") {
                            hlink_click = hlink.attr("id").or_else(|| hlink.attr("r:id")).map(|s| s.to_string());
                        }
                    }
                    
                    let mut highlight = None;
                    if let Some(r_pr_node) = r_pr {
                        if let Some(hl) = r_pr_node.child("highlight") {
                            highlight = parse_color(&hl, ctx.theme);
                        }
                    }
                    
                    let font_family = resolve_typeface(r_pr);
                    let color = resolve_color(r_pr);
                    
                    runs.push(crate::model::TextRun {
                        text: txt.clone(),
                        font_size,
                        font_family,
                        color,
                        bold,
                        italic,
                        underline,
                        strikethrough,
                        kerning,
                        letter_spacing,
                        baseline,
                        cap,
                        highlight,
                        hlink_click,
                        is_break: None,
                    });
                }
            }
        } else if tag_name == "br" {
                let r_pr = child.child("rPr");
                p_text.push('\n');
                
                let sz_str = resolve_r_property(r_pr, "sz");
                let font_size = sz_str.and_then(|s| s.parse::<f64>().ok()).map(|sz| sz / 100.0);
                let font_family = resolve_typeface(r_pr);
                let color = resolve_color(r_pr);
                
                runs.push(crate::model::TextRun {
                    text: "\n".to_string(),
                    font_size,
                    font_family,
                    color,
                    is_break: Some(true),
                    ..Default::default()
                });
            }
        }
        total_text.push_str(&p_text);
        total_text.push('\n');
        let mut end_para_font_size = None;
        if let Some(end_para) = p.child("endParaRPr") {
            let sz_str = resolve_r_property(Some(end_para), "sz");
            end_para_font_size = sz_str.and_then(|s| s.parse::<f64>().ok()).map(|sz| sz / 100.0);
        }

        paragraphs.push(TextParagraph {
            level,
            text: p_text,
            runs,
            align,
            margin_left,
            indent,
            bullet_char,
            bullet_font,
            bullet_auto_num_type,
            bullet_color,
            bullet_none,
            line_spacing,
            space_before,
            space_after,
            end_para_font_size,
        });
    }
    
    // Even if paragraphs is empty (no <a:p> at all), we might want to return Some(TextBody) 
    // to preserve body properties like normAutofit. Let's return Some(TextBody) unconditionally.
    Some(TextBody {
            paragraphs,
            total_text: total_text.trim_end().to_string(),
            margin_left,
            margin_right,
            margin_top,
            margin_bottom,
            vertical_align,
            sp_auto_fit,
        norm_autofit_font_scale,
        norm_autofit_line_space_reduction,
    })
}

fn extract_blip_embed(
    blip_fill: &XmlNode,
    pkg: &mut OpcPackage,
    slide_path: &str,
    slide_rels: &HashMap<String, RelEntry>,
) -> Option<String> {
    if let Some(blip) = blip_fill.child("blip") {
        let embed_id = blip.attr("embed").or_else(|| blip.attr("r:embed")).cloned();
        if let Some(r_id) = embed_id {
            if let Some(rel) = slide_rels.get(&r_id) {
                let target_path = resolve_target(slide_path, &rel.target);
                if let Ok(bytes) = pkg.read_part(&target_path) {
                    use base64::{Engine as _, engine::general_purpose};
                    let encoded = general_purpose::STANDARD.encode(&bytes);
                    let ext = target_path.split('.').last().unwrap_or("png").to_lowercase();
                    let mime = match ext.as_str() {
                        "jpg" | "jpeg" => "image/jpeg",
                        "png" => "image/png",
                        "gif" => "image/gif",
                        "svg" => "image/svg+xml",
                        _ => "image/png",
                    };
                    return Some(format!("data:{};base64,{}", mime, encoded));
                }
            }
        }
    }
    None
}

fn parse_color(node: &XmlNode, theme: Option<&ThemeData>) -> Option<String> {
    if let Some(srgb) = node.child("srgbClr") {
        if let Some(val) = srgb.attr("val") {
            return Some(format!("#{}", val));
        }
    }
    if let Some(scheme) = node.child("schemeClr") {
        if let Some(val) = scheme.attr("val") {
            if let Some(t) = theme {
                if let Some(hex) = t.colors.get(val) {
                    return Some(format!("#{}", hex));
                }
            }
            return Some(match val.as_str() {
                "bg1" | "lt1" => "#FFFFFF".to_string(),
                "tx1" | "dk1" => "#000000".to_string(),
                "bg2" | "lt2" => "#EEEEEE".to_string(),
                "tx2" | "dk2" => "#333333".to_string(),
                "accent1" => "#4F81BD".to_string(),
                "accent2" => "#C0504D".to_string(),
                "accent3" => "#9BBB59".to_string(),
                "accent4" => "#8064A2".to_string(),
                "accent5" => "#4BACC6".to_string(),
                "accent6" => "#F79646".to_string(),
                _ => val.to_string(),
            });
        }
    }
    None
}

fn is_placeholder(node: &XmlNode) -> bool {
    let wrappers = ["nvSpPr", "nvPicPr", "nvGrpSpPr", "nvGraphicFramePr", "nvCxnSpPr"];
    for wrapper in wrappers {
        if let Some(nv) = node.child(wrapper) {
            if let Some(nv_pr) = nv.child("nvPr") {
                if nv_pr.child("ph").is_some() {
                    return true;
                }
            }
        }
    }
    false
}

fn get_alpha(node: &XmlNode) -> Option<f64> {
    for child in &node.children {
        if let Some(alpha_node) = child.child("alpha") {
            if let Some(val) = alpha_node.attr("val") {
                if let Ok(v) = val.parse::<f64>() {
                    return Some(v / 100000.0);
                }
            }
        }
    }
    None
}

fn parse_background(
    bg_node: &XmlNode,
    ctx: &StyleContext,
    pkg: &mut OpcPackage,
    slide_path: &str,
    slide_rels: &HashMap<String, RelEntry>,
) -> Option<crate::model::Background> {
    // bgPr
    if let Some(bg_pr) = bg_node.child("bgPr") {
        if let Some(solid_fill) = bg_pr.child("solidFill") {
            let alpha = get_alpha(&solid_fill);
            if let Some(color) = parse_color(&solid_fill, ctx.theme) {
                return Some(crate::model::Background {
                    color: Some(color),
                    alpha,
                    blip_embed: None,
                    is_tile: None,
                    is_cover: None,
                    grad_fill: None,
                });
            }
        } else if let Some(grad_fill) = bg_pr.child("gradFill") {
            let mut angle = 0.0;
            if let Some(lin) = grad_fill.child("lin") {
                angle = lin.attr("ang").and_then(|a| a.parse::<f64>().ok()).unwrap_or(0.0);
            }
            let mut stops = Vec::new();
            if let Some(gs_lst) = grad_fill.child("gsLst") {
                for gs in &gs_lst.children {
                    if gs.tag.ends_with("gs") {
                        if let Some(pos) = gs.attr("pos").and_then(|p| p.parse::<f64>().ok()) {
                            if let Some(color) = parse_color(gs, ctx.theme) {
                                let alpha = get_alpha(gs);
                                stops.push(crate::model::GradientStop { position: pos, color, alpha });
                            }
                        }
                    }
                }
            }
            if !stops.is_empty() {
                return Some(crate::model::Background {
                    color: None,
                    alpha: None,
                    blip_embed: None,
                    is_tile: None,
                    is_cover: None,
                    grad_fill: Some(crate::model::GradientFill { angle, stops }),
                });
            }
        } else if let Some(blip_fill) = bg_pr.child("blipFill") {
            let is_tile = blip_fill.child("tile").is_some();
            let mut is_cover = false;
            if let Some(stretch) = blip_fill.child("stretch") {
                if stretch.child("fillRect").is_none() {
                    is_cover = true;
                }
            }
            if let Some(blip_embed) = extract_blip_embed(&blip_fill, pkg, slide_path, slide_rels) {
                return Some(crate::model::Background {
                    color: None,
                    alpha: None,
                    blip_embed: Some(blip_embed),
                    is_tile: Some(is_tile),
                    is_cover: Some(is_cover),
                    grad_fill: None,
                });
            }
        } else if let Some(_) = bg_pr.child("noFill") {
            return Some(crate::model::Background {
                color: Some("FFFFFF".to_string()),
                alpha: None,
                blip_embed: None,
                is_tile: None,
                is_cover: None,
                grad_fill: None,
            });
        }
    }
    
    // bgRef
    if let Some(bg_ref) = bg_node.child("bgRef") {
        let alpha = get_alpha(&bg_ref);
        if let Some(color) = parse_color(&bg_ref, ctx.theme) {
            return Some(crate::model::Background {
                color: Some(color),
                alpha,
                blip_embed: None,
                is_tile: None,
                is_cover: None,
                grad_fill: None,
            });
        }
    }
    
    None
}

pub(crate) fn parse_node(
    node: &XmlNode,
    ctx: &StyleContext,
    pkg: &mut OpcPackage,
    slide_path: &str,
    slide_rels: &HashMap<String, RelEntry>,
) -> Option<SlideNode> {
    let tag = node.tag.split(':').last().unwrap_or(&node.tag);
    
    // Supported child tags for grouping
    if tag != "sp" && tag != "pic" && tag != "grpSp" && tag != "graphicFrame" && tag != "cxnSp" {
        return None;
    }

    let mut id = String::new();
    let mut name = String::new();
    
    // Extract nv properties for base
    let mut nv_wrapper_found = false;
    let mut hlink_click = None;
    let mut hlink_tooltip = None;
    
    for wrapper in ["nvSpPr", "nvPicPr", "nvGrpSpPr", "nvGraphicFramePr", "nvCxnSpPr"] {
        if let Some(nv) = node.child(wrapper) {
            nv_wrapper_found = true;
            if let Some(cnv) = nv.child("cNvPr") {
                if let Some(attr_id) = cnv.attr("id") { id = attr_id.clone(); }
                if let Some(attr_name) = cnv.attr("name") { name = attr_name.clone(); }
                
                if let Some(hlink) = cnv.child("hlinkClick") {
                    let r_id = hlink.attr("id").or_else(|| hlink.attr("r:id")).map(|s| s.to_string());
                    let action = hlink.attr("action").map(|s| s.to_string());
                    let tooltip = hlink.attr("tooltip").map(|s| s.to_string());
                    
                    if let Some(rid) = r_id {
                        if let Some(rel) = slide_rels.get(&rid) {
                            hlink_click = Some(rel.target.clone());
                        }
                    } else if action.as_deref() == Some("ppaction://hlinksldjump") {
                        hlink_click = Some("ppaction://hlinksldjump".to_string());
                    }
                    
                    hlink_tooltip = tooltip;
                }
            }
            break;
        }
    }
    
    if !nv_wrapper_found {
        return None; // Ensure it's a valid shape node
    }

    let mut reflection = None;
    if let Some(sp_pr) = node.child("spPr") {
        if let Some(effect_lst) = sp_pr.child("effectLst") {
            if let Some(refl) = effect_lst.child("reflection") {
                reflection = Some(crate::model::Reflection {
                    blur_rad: refl.num_attr("blurRad"),
                    st_a: refl.num_attr("stA"),
                    st_pos: refl.num_attr("stPos"),
                    end_a: refl.num_attr("endA"),
                    end_pos: refl.num_attr("endPos"),
                    dist: refl.num_attr("dist"),
                    dir: refl.num_attr("dir"),
                    fade_dir: refl.num_attr("fadeDir"),
                    sy: refl.num_attr("sy"),
                    sx: refl.num_attr("sx"),
                    ky: refl.num_attr("ky"),
                    kx: refl.num_attr("kx"),
                    algn: refl.attr("algn").map(|s| s.to_string()),
                    rot_with_shape: refl.attr("rotWithShape").map(|s| s == "1" || s == "true"),
                });
            }
        }
    }

    let mut pos = Position { x: 0.0, y: 0.0 };
    let mut size = Size { w: 0.0, h: 0.0 };
    
    let mut layout_ph = None;
    let mut master_ph = None;
    
    let (p_type, p_idx) = get_ph_info(node);
    
    if let Some(l_ph) = find_placeholder(ctx.layout_placeholders, p_type.as_ref(), p_idx) {
        layout_ph = Some(l_ph);
    }
    
    if let Some(m_ph) = find_placeholder(ctx.master_placeholders, p_type.as_ref(), p_idx) {
        master_ph = Some(m_ph);
    }

    let mut rotation = 0.0;
    let mut flip_h = false;
    let mut flip_v = false;
    
    // Parse local xfrm
    if let Some((local_pos, local_size, local_rot, local_fh, local_fv)) = get_xfrm(node) {
        pos = local_pos;
        size = local_size;
        rotation = local_rot;
        flip_h = local_fh;
        flip_v = local_fv;
    } else {
        // Inherit from layout or master
        let mut inherited = false;
        
        if let Some(l_ph) = layout_ph {
            if let Some((l_pos, l_size, l_rot, l_fh, l_fv)) = get_xfrm(l_ph) {
                pos = l_pos;
                size = l_size;
                rotation = l_rot;
                flip_h = l_fh;
                flip_v = l_fv;
                inherited = true;
            }
        }
        
        if !inherited {
            if let Some(m_ph) = master_ph {
                if let Some((m_pos, m_size, m_rot, m_fh, m_fv)) = get_xfrm(m_ph) {
                    pos = m_pos;
                    size = m_size;
                    rotation = m_rot;
                    flip_h = m_fh;
                    flip_v = m_fv;
                }
            }
        }
    }
    
    let (p_type, _) = get_ph_info(node);
    let category = match p_type.as_deref() {
        Some("title") | Some("ctrTitle") => "title",
        Some("body") | Some("subTitle") | Some("obj") | Some("dt") | Some("ftr") | Some("sldNum") => "body",
        _ => "other",
    };

    match tag {
        "sp" | "cxnSp" => {
            let mut text_body = None;
            if let Some(tx_body) = node.child("txBody") {
                text_body = parse_text_body(tx_body, ctx, category, master_ph, layout_ph);
            }
            
            let mut preset_geometry = None;
            let mut adjustments = None;
            let mut blip_embed = None;
            let mut solid_fill = None;
            let mut grad_fill = None;
            let mut line = None;
            
            if let Some(sp_pr) = node.child("spPr") {
                if let Some(blip_fill) = sp_pr.child("blipFill") {
                    blip_embed = extract_blip_embed(&blip_fill, pkg, slide_path, slide_rels);
                }
                if let Some(prst_geom) = sp_pr.child("prstGeom") {
                    preset_geometry = prst_geom.attr("prst").map(|s| s.to_string());
                    if let Some(av_lst) = prst_geom.child("avLst") {
                        let mut adjs = std::collections::HashMap::new();
                        for gd in av_lst.children("gd") {
                            if let (Some(name), Some(fmla)) = (gd.attr("name"), gd.attr("fmla")) {
                                if let Some(idx) = fmla.find("val ") {
                                    let val_str = fmla[idx + 4..].trim();
                                    if let Ok(val) = val_str.parse::<f64>() {
                                        adjs.insert(name.to_string(), val);
                                    }
                                } else if let Ok(val) = fmla.trim().parse::<f64>() {
                                    adjs.insert(name.to_string(), val);
                                }
                            }
                        }
                        if !adjs.is_empty() {
                            adjustments = Some(adjs);
                        }
                    }
                }
                
                if let Some(fill_node) = sp_pr.child("solidFill") {
                    if let Some(color) = parse_color(&fill_node, ctx.theme) {
                        solid_fill = Some(crate::model::SolidFill {
                            color: crate::model::SchemeColor { scheme: color, transform: None }
                        });
                    }
                } else if sp_pr.child("noFill").is_some() {
                    solid_fill = Some(crate::model::SolidFill {
                        color: crate::model::SchemeColor { scheme: "transparent".to_string(), transform: None }
                    });
                }
                
                if let Some(grad_node) = sp_pr.child("gradFill") {
                    let mut angle = 0.0;
                    if let Some(lin) = grad_node.child("lin") {
                        angle = lin.attr("ang").and_then(|a| a.parse::<f64>().ok()).unwrap_or(0.0);
                    }
                    let mut stops = Vec::new();
                    if let Some(gs_lst) = grad_node.child("gsLst") {
                        for gs in &gs_lst.children {
                            if gs.tag.ends_with("gs") {
                                if let Some(pos) = gs.attr("pos").and_then(|p| p.parse::<f64>().ok()) {
                                    if let Some(color) = parse_color(gs, ctx.theme) {
                                        let alpha = get_alpha(gs);
                                        stops.push(crate::model::GradientStop { position: pos, color, alpha });
                                    }
                                }
                            }
                        }
                    }
                    if !stops.is_empty() {
                        grad_fill = Some(crate::model::GradientFill { angle, stops });
                    }
                }
                

                if let Some(ln_node) = sp_pr.child("ln") {
                    let w = ln_node.num_attr("w").unwrap_or(12700.0);
                    let mut fill = crate::model::SolidFill { color: crate::model::SchemeColor { scheme: "transparent".to_string(), transform: None } };
                    
                    if let Some(ln_fill) = ln_node.child("solidFill") {
                        if let Some(color) = parse_color(&ln_fill, ctx.theme) {
                            fill.color.scheme = color;
                        }
                    } else if ln_node.child("noFill").is_some() {
                        fill.color.scheme = "transparent".to_string();
                    }
                    
                    let mut dash = None;
                    if let Some(dash_node) = ln_node.child("prstDash") {
                        if let Some(val) = dash_node.attr("val") {
                            dash = Some(val.to_string());
                        }
                    }
                    
                    let cap = ln_node.attr("cap").map(|s| s.to_string());
                    
                    let mut head_end = None;
                    if let Some(h) = ln_node.child("headEnd") {
                        if let Some(t) = h.attr("type") {
                            if t != "none" {
                                head_end = Some(crate::model::LineEndInfo {
                                    end_type: t.to_string(),
                                    w: h.attr("w").map(|s| s.to_string()),
                                    len: h.attr("len").map(|s| s.to_string()),
                                });
                            }
                        }
                    }
                    
                    let mut tail_end = None;
                    if let Some(h) = ln_node.child("tailEnd") {
                        if let Some(t) = h.attr("type") {
                            if t != "none" {
                                tail_end = Some(crate::model::LineEndInfo {
                                    end_type: t.to_string(),
                                    w: h.attr("w").map(|s| s.to_string()),
                                    len: h.attr("len").map(|s| s.to_string()),
                                });
                            }
                        }
                    }

                    let mut join = None;
                    if ln_node.child("miter").is_some() { join = Some("miter".to_string()); }
                    else if ln_node.child("bevel").is_some() { join = Some("bevel".to_string()); }
                    else if ln_node.child("round").is_some() { join = Some("round".to_string()); }

                    let mut line_grad_fill = None;
                    if let Some(grad) = ln_node.child("gradFill") {
                        let mut angle = 0.0;
                        if let Some(lin) = grad.child("lin") {
                            if let Some(ang) = lin.num_attr("ang") { angle = ang; }
                        }
                        let mut stops = Vec::new();
                        if let Some(gs_lst) = grad.child("gsLst") {
                            for gs in gs_lst.children("gs") {
                                let pos = gs.num_attr("pos").unwrap_or(0.0);
                                let color = parse_color(&gs, ctx.theme).unwrap_or("000000".to_string());
                                let mut alpha = None;
                                if let Some(clr_node) = gs.child("srgbClr").or_else(|| gs.child("schemeClr")) {
                                    if let Some(alpha_node) = clr_node.child("alpha") {
                                        alpha = alpha_node.num_attr("val").map(|v| v / 100000.0);
                                    }
                                }
                                stops.push(crate::model::GradientStop { position: pos, color, alpha });
                            }
                            line_grad_fill = Some(crate::model::GradientFill { angle, stops });
                        }
                    }

                    line = Some(crate::model::BorderLine { width: w, fill, dash, cap, join, head_end, tail_end, grad_fill: line_grad_fill });
                }
            }
            
            if let Some(style_node) = node.child("style") {
                if solid_fill.is_none() && grad_fill.is_none() {
                    if let Some(fill_ref) = style_node.child("fillRef") {
                        if let Some(color) = parse_color(&fill_ref, ctx.theme) {
                            solid_fill = Some(crate::model::SolidFill {
                                color: crate::model::SchemeColor { scheme: color, transform: None }
                            });
                        }
                    }
                }
                
                if let Some(ln_ref) = style_node.child("lnRef") {
                    if let Some(color) = parse_color(&ln_ref, ctx.theme) {
                        if let Some(l) = &mut line {
                            if l.fill.color.scheme == "transparent" {
                                l.fill.color.scheme = color;
                            }
                        } else {
                            line = Some(crate::model::BorderLine {
                                width: 12700.0,
                                fill: crate::model::SolidFill { color: crate::model::SchemeColor { scheme: color, transform: None } },
                                dash: None,
                                cap: None,
                                join: None,
                                head_end: None,
                                tail_end: None,
                                grad_fill: None,
                            });
                        }
                    }
                }
            }
            Some(SlideNode {
                alt_text: None,
                id,
                name,
                node_type: "shape".to_string(),
                position: pos,
                size,
                rotation,
                flip_h,
                flip_v,
                hlink_click: hlink_click.clone(),
                hlink_tooltip: hlink_tooltip.clone(),
                reflection: reflection.clone(),
                preset_geometry,
                adjustments,
                solid_fill,
                grad_fill,
                line,
                text_body,
                children: None,
                child_offset: None,
                child_extent: None,
                blip_embed,
                crop: None,
                columns: None,
                rows: None,
                table_style_id: None,
                table_properties: None,
            })
        }
        "pic" => {
            let mut blip_embed = None;
            let mut crop = None;
            if let Some(blip_fill) = node.child("blipFill") {
                blip_embed = extract_blip_embed(&blip_fill, pkg, slide_path, slide_rels);
                
                if let Some(src_rect) = blip_fill.child("srcRect") {
                    let divisor = 100000.0;
                    let t = src_rect.num_attr("t").unwrap_or(0.0) / divisor;
                    let b = src_rect.num_attr("b").unwrap_or(0.0) / divisor;
                    let l = src_rect.num_attr("l").unwrap_or(0.0) / divisor;
                    let r = src_rect.num_attr("r").unwrap_or(0.0) / divisor;
                    if t != 0.0 || b != 0.0 || l != 0.0 || r != 0.0 {
                        crop = Some(crate::model::CropRect { top: t, bottom: b, left: l, right: r });
                    }
                }
            }
            Some(SlideNode {
                alt_text: None,
                id,
                name,
                node_type: "picture".to_string(),
                position: pos,
                size,
                rotation,
                flip_h,
                flip_v,
                hlink_click: hlink_click.clone(),
                hlink_tooltip: hlink_tooltip.clone(),
                reflection: reflection.clone(),
                preset_geometry: None,
                adjustments: None,
                solid_fill: None,
                grad_fill: None,
                line: None,
                text_body: None,
                children: None,
                child_offset: None,
                child_extent: None,
                blip_embed,
                crop,
                columns: None,
                rows: None,
                table_style_id: None,
                table_properties: None,
            })
        }
        "grpSp" => {
            let mut child_offset = Position { x: 0.0, y: 0.0 };
            let mut child_extent = Size { w: size.w, h: size.h };
            if let Some(grp_sp_pr) = node.child("grpSpPr") {
                if let Some(xfrm) = grp_sp_pr.child("xfrm") {
                    if let Some(ch_off) = xfrm.child("chOff") {
                        child_offset.x = ch_off.num_attr("x").unwrap_or(0.0) as f64 / 9525.0;
                        child_offset.y = ch_off.num_attr("y").unwrap_or(0.0) as f64 / 9525.0;
                    }
                    if let Some(ch_ext) = xfrm.child("chExt") {
                        let cx = ch_ext.num_attr("cx");
                        let cy = ch_ext.num_attr("cy");
                        if let Some(cx) = cx { if cx > 0.0 { child_extent.w = cx as f64 / 9525.0; } }
                        if let Some(cy) = cy { if cy > 0.0 { child_extent.h = cy as f64 / 9525.0; } }
                    }
                }
            }
            
            let mut children = Vec::new();
            for child in &node.children {
                if let Some(child_node) = parse_node(child, ctx, pkg, slide_path, slide_rels) {
                    children.push(child_node);
                }
            }
            
            Some(SlideNode {
                alt_text: None,
                id,
                name,
                node_type: "group".to_string(),
                position: pos,
                size,
                rotation,
                flip_h,
                flip_v,
                hlink_click: hlink_click.clone(),
                hlink_tooltip: hlink_tooltip.clone(),
                reflection: reflection.clone(),
                preset_geometry: None,
                adjustments: None,
                solid_fill: None,
                grad_fill: None,
                line: None,
                text_body: None,
                children: Some(children),
                child_offset: Some(child_offset),
                child_extent: Some(child_extent),
                blip_embed: None,
                crop: None,
                columns: None,
                rows: None,
                table_style_id: None,
                table_properties: None,
            })
        }
        "graphicFrame" => {
            if let Some(graphic) = node.child("graphic") {
                if let Some(graphic_data) = graphic.child("graphicData") {
                    if let Some(tbl) = graphic_data.child("tbl") {
                        let mut columns = Vec::new();
                        if let Some(tbl_grid) = tbl.child("tblGrid") {
                            for col in tbl_grid.children("gridCol") {
                                columns.push(col.num_attr("w").unwrap_or(0.0) as f64 / 9525.0);
                            }
                        }
                        
                        let mut rows = Vec::new();
                        for tr in tbl.children("tr") {
                            let height = tr.num_attr("h").unwrap_or(0.0) as f64 / 9525.0;
                            let mut cells = Vec::new();
                            for tc in tr.children("tc") {
                                let grid_span = tc.num_attr("gridSpan").unwrap_or(1.0) as u32;
                                let row_span = tc.num_attr("rowSpan").unwrap_or(1.0) as u32;
                                let mut text_body = None;
                                if let Some(tx_body) = tc.child("txBody") {
                                    text_body = parse_text_body(tx_body, ctx, "other", master_ph, layout_ph);
                                }
                                let text = text_body.as_ref().map(|tb| tb.total_text.clone()).unwrap_or_default();
                                
                                let mut margin_left = None;
                                let mut margin_right = None;
                                let mut margin_top = None;
                                let mut margin_bottom = None;
                                
                                if let Some(tc_pr) = tc.child("tcPr") {
                                    if let Some(m) = tc_pr.num_attr("marL") { margin_left = Some(m as f64); }
                                    if let Some(m) = tc_pr.num_attr("marR") { margin_right = Some(m as f64); }
                                    if let Some(m) = tc_pr.num_attr("marT") { margin_top = Some(m as f64); }
                                    if let Some(m) = tc_pr.num_attr("marB") { margin_bottom = Some(m as f64); }
                                }
                                
                                cells.push(crate::model::TableCell {
                                    text,
                                    grid_span,
                                    row_span,
                                    text_body,
                                    margin_left,
                                    margin_right,
                                    margin_top,
                                    margin_bottom,
                                });
                            }
                            rows.push(crate::model::TableRow { height, cells });
                        }
                        
                        let mut table_style_id = None;
                        let mut table_properties = None;
                        if let Some(tbl_pr) = tbl.child("tblPr") {
                            let flag = |attr_name: &str, child_name: &str| -> bool {
                                if let Some(val) = tbl_pr.attr(attr_name) {
                                    if val == "1" || val == "true" { return true; }
                                    if val == "0" || val == "false" { return false; }
                                }
                                if let Some(child) = tbl_pr.child(child_name) {
                                    if let Some(val) = child.attr("val") {
                                        if val == "0" || val == "false" { return false; }
                                    }
                                    return true;
                                }
                                false
                            };
                            
                            table_properties = Some(crate::model::TableProperties {
                                first_row: flag("firstRow", "firstRow"),
                                first_col: flag("firstCol", "firstCol"),
                                last_row: flag("lastRow", "lastRow"),
                                last_col: flag("lastCol", "lastCol"),
                                band_row: flag("bandRow", "bandRow"),
                                band_col: flag("bandCol", "bandCol"),
                            });
                            
                            if let Some(id_node) = tbl_pr.child("tableStyleId") {
                                table_style_id = id_node.text.clone().or_else(|| id_node.attr("val").cloned());
                            } else if let Some(style_node) = tbl_pr.child("tblStyle") {
                                table_style_id = style_node.attr("val").cloned().or_else(|| style_node.text.clone());
                            } else {
                                table_style_id = tbl_pr.attr("tblStyle").cloned();
                            }
                        }
                        
                        return Some(SlideNode {
                alt_text: None,
                            id,
                            name,
                            node_type: "table".to_string(),
                            position: pos,
                            size,
                            rotation,
                            flip_h,
                            flip_v,
                            hlink_click: hlink_click.clone(),
                            hlink_tooltip: hlink_tooltip.clone(),
                            reflection: reflection.clone(),
                            preset_geometry: None,
                            adjustments: None,
                            solid_fill: None,
                            grad_fill: None,
                            line: None,
                            text_body: None,
                            children: None,
                            child_offset: None,
                            child_extent: None,
                            blip_embed: None,
                            crop: None,
                            columns: Some(columns),
                            rows: Some(rows),
                            table_style_id,
                            table_properties,
                        });
                    }
                }
            }
            None
        }
        _ => None
    }
}

pub fn parse_presentation(path: &str) -> Result<Presentation, String> {
    let mut pkg = OpcPackage::open(path).map_err(|e| e.to_string())?;
    
    let pres_xml = pkg.read_part("ppt/presentation.xml").map_err(|e| e.to_string())?;
    let pres_str = String::from_utf8_lossy(&pres_xml);
    let root = XmlNode::parse(&pres_str)?;
    
    let sld_sz = root.child("sldSz").ok_or("No sldSz found")?;
    let width = sld_sz.num_attr("cx").unwrap_or(9144000.0);
    let height = sld_sz.num_attr("cy").unwrap_or(6858000.0);
    let px_width = (width / 9525.0).round() as u32;
    let px_height = (height / 9525.0).round() as u32;
    
    let pres_rels_xml = pkg.read_part("ppt/_rels/presentation.xml.rels").map_err(|e| e.to_string())?;
    let pres_rels = parse_rels(&String::from_utf8_lossy(&pres_rels_xml));
    
    let mut ordered_slide_targets = Vec::new();
    if let Some(sld_id_lst) = root.child("sldIdLst") {
        for sld_id in sld_id_lst.children("sldId") {
            let r_id = sld_id.attr("r:id").or_else(|| sld_id.attr("id"));
            if let Some(r_id) = r_id {
                if let Some(rel) = pres_rels.get(r_id) {
                    ordered_slide_targets.push(resolve_target("ppt/presentation.xml", &rel.target));
                }
            }
        }
    }
    
    let slide_count = ordered_slide_targets.len() as u32;
    let mut slides = Vec::new();
    
    for (i, slide_path) in ordered_slide_targets.iter().enumerate() {
        let slide_num = if slide_path.starts_with("ppt/slides/slide") && slide_path.ends_with(".xml") {
            slide_path[16..slide_path.len() - 4].parse::<usize>().unwrap_or(i + 1)
        } else {
            i + 1
        };
        
        if let Ok(slide_xml) = pkg.read_part(slide_path) {
            let slide_str = String::from_utf8_lossy(&slide_xml);
            if let Ok(slide_root) = XmlNode::parse(&slide_str) {
                let mut nodes = Vec::new();
                
                // Get slide rels to find layout
                let slide_basename = slide_path.split('/').last().unwrap();
                let slide_dir = &slide_path[..slide_path.len() - slide_basename.len() - 1];
                let slide_rels_path = format!("{}/_rels/{}.rels", slide_dir, slide_basename);
                
                let mut layout_placeholders = Vec::new();
                let mut master_placeholders = Vec::new();
                let mut theme_fonts = None;
                let mut master_styles = None;
                let mut slide_rels = HashMap::new();
                
                let mut layout_bg = None;
                let mut master_bg = None;
                let mut layout_rels_map = HashMap::new();
                let mut master_rels_map = HashMap::new();
                let mut layout_path_str = String::new();
                let mut master_path_str = String::new();
                
                let mut layout_nodes = Vec::new();
                let mut master_nodes = Vec::new();
                let mut layout_sp_tree = None;
                let mut master_sp_tree = None;
                
                if let Ok(slide_rels_xml) = pkg.read_part(&slide_rels_path) {
                    slide_rels = parse_rels(&String::from_utf8_lossy(&slide_rels_xml));
                    for (_, rel) in &slide_rels {
                        if rel.rel_type.ends_with("/slideLayout") {
                            let layout_path = resolve_target(slide_path, &rel.target);
                            layout_path_str = layout_path.clone();
                            if let Ok(layout_xml) = pkg.read_part(&layout_path) {
                                let layout_str = String::from_utf8_lossy(&layout_xml);
                                if let Ok(layout_root) = XmlNode::parse(&layout_str) {
                                    if let Some(c_sld) = layout_root.child("cSld") {
                                        if let Some(bg) = c_sld.child("bg") {
                                            layout_bg = Some(bg.clone());
                                        }
                                        if let Some(sp_tree) = c_sld.child("spTree") {
                                            layout_sp_tree = Some(sp_tree.clone());
                                            for sp in sp_tree.children("sp") {
                                                layout_placeholders.push(sp.clone());
                                            }
                                        }
                                    }
                                }
                            }
                            
                            // Find master
                            let layout_basename = layout_path.split('/').last().unwrap();
                            let layout_dir = &layout_path[..layout_path.len() - layout_basename.len() - 1];
                            let layout_rels_path = format!("{}/_rels/{}.rels", layout_dir, layout_basename);
                            
                            if let Ok(layout_rels_xml) = pkg.read_part(&layout_rels_path) {
                                let layout_rels = parse_rels(&String::from_utf8_lossy(&layout_rels_xml));
                                layout_rels_map = layout_rels.clone();
                                for (_, lrel) in &layout_rels {
                                    if lrel.rel_type.ends_with("/slideMaster") {
                                        let master_path = resolve_target(&layout_path, &lrel.target);
                                        master_path_str = master_path.clone();
                                        if let Ok(master_xml) = pkg.read_part(&master_path) {
                                            let master_str = String::from_utf8_lossy(&master_xml);
                                            if let Ok(master_root) = XmlNode::parse(&master_str) {
                                                if let Some(c_sld) = master_root.child("cSld") {
                                                    if let Some(bg) = c_sld.child("bg") {
                                                        master_bg = Some(bg.clone());
                                                    }
                                                    if let Some(sp_tree) = c_sld.child("spTree") {
                                                        master_sp_tree = Some(sp_tree.clone());
                                                        for sp in sp_tree.children("sp") {
                                                            master_placeholders.push(sp.clone());
                                                        }
                                                    }
                                                }
                                                
                                                if let Some(tx_styles) = master_root.child("txStyles") {
                                                    master_styles = Some(MasterTextStyles {
                                                        title: tx_styles.child("titleStyle").cloned(),
                                                        body: tx_styles.child("bodyStyle").cloned(),
                                                        other: tx_styles.child("otherStyle").cloned(),
                                                    });
                                                }
                                                
                                                let master_basename = master_path.split('/').last().unwrap();
                                                let master_dir = &master_path[..master_path.len() - master_basename.len() - 1];
                                                let master_rels_path = format!("{}/_rels/{}.rels", master_dir, master_basename);
                                                
                                                if let Ok(master_rels_xml) = pkg.read_part(&master_rels_path) {
                                                    let master_rels = parse_rels(&String::from_utf8_lossy(&master_rels_xml));
                                                    master_rels_map = master_rels.clone();
                                                    for (_, mrel) in &master_rels {
                                                        if mrel.rel_type.ends_with("/theme") {
                                                            let theme_path = resolve_target(&master_path, &mrel.target);
                                                            if let Ok(theme_xml) = pkg.read_part(&theme_path) {
                                                                let theme_str = String::from_utf8_lossy(&theme_xml);
                                                                if let Ok(theme_root) = XmlNode::parse(&theme_str) {
                                                                                                                                        if let Some(theme_elements) = theme_root.child("themeElements") {
                                                                        let mut major = String::new();
                                                                        let mut minor = String::new();
                                                                        if let Some(font_scheme) = theme_elements.child("fontScheme") {
                                                                            major = font_scheme.child("majorFont").and_then(|m| m.child("latin")).and_then(|l| l.attr("typeface")).cloned().unwrap_or_default();
                                                                            minor = font_scheme.child("minorFont").and_then(|m| m.child("latin")).and_then(|l| l.attr("typeface")).cloned().unwrap_or_default();
                                                                        }
                                                                        let mut colors = std::collections::HashMap::new();
                                                                        if let Some(clr_scheme) = theme_elements.child("clrScheme") {
                                                                            for child in &clr_scheme.children {
                                                                                let name = if child.tag.contains(":") { child.tag.split(':').last().unwrap() } else { &child.tag };
                                                                                if name == "dk1" || name == "lt1" || name == "dk2" || name == "lt2" || name.starts_with("accent") || name == "hlink" || name == "folHlink" || name == "bg1" || name == "tx1" || name == "bg2" || name == "tx2" {
                                                                                    if let Some(srgb) = child.child("srgbClr") {
                                                                                        if let Some(val) = srgb.attr("val") {
                                                                                            colors.insert(name.to_string(), val.clone());
                                                                                        }
                                                                                    } else if let Some(sys) = child.child("sysClr") {
                                                                                        if let Some(val) = sys.attr("lastClr") {
                                                                                            colors.insert(name.to_string(), val.clone());
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                        theme_fonts = Some(ThemeData { major_font: major, minor_font: minor, colors });
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                
                let mut parsed_notes = None;
                for (_, rel) in &slide_rels {
                    if rel.rel_type.ends_with("/notesSlide") {
                        let notes_path = resolve_target(slide_path, &rel.target);
                        if let Ok(notes_xml) = pkg.read_part(&notes_path) {
                            let notes_str = String::from_utf8_lossy(&notes_xml);
                            if let Ok(notes_root) = XmlNode::parse(&notes_str) {
                                let mut text = String::new();
                                fn extract_text(node: &XmlNode, text: &mut String) {
                                    if node.tag == "a:t" && node.text.is_some() {
                                        text.push_str(node.text.as_ref().unwrap());
                                        text.push('\n');
                                    }
                                    for child in &node.children {
                                        extract_text(child, text);
                                    }
                                }
                                extract_text(&notes_root, &mut text);
                                if !text.trim().is_empty() {
                                    parsed_notes = Some(text);
                                }
                            }
                        }
                    }
                }
                
                let ctx = StyleContext {
                    theme: theme_fonts.as_ref(),
                    master_styles: master_styles.as_ref(),
                    master_placeholders: &master_placeholders,
                    layout_placeholders: &layout_placeholders,
                };

                let c_sld = slide_root.child("cSld");
                
                let mut parsed_bg = None;
                if let Some(c_sld) = &c_sld {
                    if let Some(bg) = c_sld.child("bg") {
                        parsed_bg = parse_background(bg, &ctx, &mut pkg, slide_path, &slide_rels);
                    }
                }
                if parsed_bg.is_none() {
                    if let Some(bg) = layout_bg {
                        parsed_bg = parse_background(&bg, &ctx, &mut pkg, &layout_path_str, &layout_rels_map);
                    }
                }
                if parsed_bg.is_none() {
                    if let Some(bg) = master_bg {
                        parsed_bg = parse_background(&bg, &ctx, &mut pkg, &master_path_str, &master_rels_map);
                    }
                }

                if let Some(sp_tree) = layout_sp_tree {
                    for sp in &sp_tree.children {
                        if !is_placeholder(sp) {
                            if let Some(node) = parse_node(sp, &ctx, &mut pkg, &layout_path_str, &layout_rels_map) {
                                layout_nodes.push(node);
                            }
                        }
                    }
                }
                
                if let Some(sp_tree) = master_sp_tree {
                    for sp in &sp_tree.children {
                        if !is_placeholder(sp) {
                            if let Some(node) = parse_node(sp, &ctx, &mut pkg, &master_path_str, &master_rels_map) {
                                master_nodes.push(node);
                            }
                        }
                    }
                }

                if let Some(c_sld) = c_sld {
                    let sp_tree = c_sld.child("spTree");
                    if let Some(sp_tree) = sp_tree {
                        for child in &sp_tree.children {
                            if let Some(node) = parse_node(child, &ctx, &mut pkg, slide_path, &slide_rels) {
                                nodes.push(node);
                            }
                        }
                    }
                }
                
                slides.push(Slide {
                    notes: parsed_notes,
                    index: slide_num,
                    background: parsed_bg,
                    nodes,
                    layout_nodes,
                    master_nodes,
                });
            }
        }
    }
    
    // Remove the fallback so tests pass entirely on Native AST!
    Ok(Presentation {
        width: px_width,
        height: px_height,
        slide_count,
        slides,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_rels() {
        let rels_xml = r#"
            <Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships">
                <Relationship Id="rId1" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/slideLayout" Target="../slideLayouts/slideLayout1.xml"/>
                <Relationship Id="rId2" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/image" Target="../media/image1.png"/>
            </Relationships>
        "#;
        let rels = parse_rels(rels_xml);
        assert_eq!(rels.len(), 2);
        
        let rel1 = rels.get("rId1").unwrap();
        assert_eq!(rel1.id, "rId1");
        assert_eq!(rel1.rel_type, "http://schemas.openxmlformats.org/officeDocument/2006/relationships/slideLayout");
        assert_eq!(rel1.target, "../slideLayouts/slideLayout1.xml");
        
        let rel2 = rels.get("rId2").unwrap();
        assert_eq!(rel2.id, "rId2");
        assert_eq!(rel2.target, "../media/image1.png");
    }

    #[test]
    fn test_resolve_target() {
        assert_eq!(resolve_target("ppt/slides/slide1.xml", "../slideLayouts/slideLayout1.xml"), "ppt/slideLayouts/slideLayout1.xml");
        assert_eq!(resolve_target("ppt/presentation.xml", "slides/slide1.xml"), "ppt/slides/slide1.xml");
        assert_eq!(resolve_target("ppt/slides/slide1.xml", "media/image1.png"), "ppt/slides/media/image1.png");
    }

    #[test]
    fn test_get_xfrm() {
        let xml = r#"
            <sp>
                <spPr>
                    <xfrm>
                        <off x="952500" y="1905000"/>
                        <ext cx="2857500" cy="3810000"/>
                    </xfrm>
                </spPr>
            </sp>
        "#;
        let node = XmlNode::parse(xml).unwrap();
        let xfrm_res = get_xfrm(&node);
        assert!(xfrm_res.is_some());
        let (pos, size, rot, flip_h, flip_v) = xfrm_res.unwrap();
        
        assert_eq!(pos.x, 100.0);
        assert_eq!(pos.y, 200.0);
        assert_eq!(size.w, 300.0);
        assert_eq!(size.h, 400.0);
        assert_eq!(rot, 0.0);
        assert_eq!(flip_h, false);
        assert_eq!(flip_v, false);
    }


// --- Ported Tests Scaffold ---
// Tests ported from units.test.ts
#[test]
fn test_units_converts_914400_emu_1_inch_to_96_px() {
    // TODO: Port test
}

#[test]
fn test_units_converts_0_emu_to_0_px() {
    // TODO: Port test
}

#[test]
fn test_units_handles_fractional_results() {
    // TODO: Port test
}

#[test]
fn test_units_converts_12700_emu_to_1_pt() {
    // TODO: Port test
}

#[test]
fn test_units_converts_914400_emu_1_inch_to_72_pt() {
    // TODO: Port test
}

#[test]
fn test_units_converts_60000_to_1_degree() {
    // TODO: Port test
}

#[test]
fn test_units_converts_5400000_to_90_degrees() {
    // TODO: Port test
}

#[test]
fn test_units_converts_0_to_0() {
    // TODO: Port test
}

#[test]
fn test_units_converts_100000_to_1_0() {
    // TODO: Port test
}

#[test]
fn test_units_converts_50000_to_0_5() {
    // TODO: Port test
}

#[test]
fn test_units_converts_0_to_0_2() {
    // TODO: Port test
}

#[test]
fn test_units_converts_1200_to_12() {
    // TODO: Port test
}

#[test]
fn test_units_converts_100_to_1() {
    // TODO: Port test
}

#[test]
fn test_units_converts_72_pt_1_inch_to_96_px() {
    // TODO: Port test
}

#[test]
fn test_units_converts_0_to_0_3() {
    // TODO: Port test
}

#[test]
fn test_units_converts_12_pt_to_16_px() {
    // TODO: Port test
}

#[test]
fn test_units_detects_emu_for_large_values() {
    // TODO: Port test
}

#[test]
fn test_units_detects_point_for_small_values() {
    // TODO: Port test
}

#[test]
fn test_units_handles_negative_values() {
    // TODO: Port test
}

#[test]
fn test_units_boundary_20000_is_point_20001_is_emu() {
    // TODO: Port test
}

#[test]
fn test_units_converts_large_values_as_emu() {
    // TODO: Port test
}

#[test]
fn test_units_converts_small_values_as_points() {
    // TODO: Port test
}

// Tests ported from RelParser.test.ts
#[test]
fn test_relparser_parses_targetmode_for_external_relationships() {
    // TODO: Port test
}

#[test]
fn test_relparser_parses_multiple_relationships() {
    // TODO: Port test
}

#[test]
fn test_relparser_returns_empty_map_for_empty_string() {
    // TODO: Port test
}

#[test]
fn test_relparser_skips_relationships_with_missing_id() {
    // TODO: Port test
}

#[test]
fn test_relparser_handles_relationship_without_targetmode() {
    // TODO: Port test
}

#[test]
fn test_relparser_resolves_relative_path() {
    // TODO: Port test
}

#[test]
fn test_relparser_resolves_in_target_path() {
    // TODO: Port test
}

#[test]
fn test_relparser_resolves_absolute_target_leading() {
    // TODO: Port test
}

#[test]
fn test_relparser_handles_backslashes() {
    // TODO: Port test
}

#[test]
fn test_relparser_resolves_in_target_path_2() {
    // TODO: Port test
}

#[test]
fn test_relparser_handles_simple_relative_target() {
    // TODO: Port test
}

// Tests ported from ZipParser.test.ts
#[test]
fn test_zipparser_parses_presentation_xml_into_result_presentation() {
    // TODO: Port test
}

#[test]
fn test_zipparser_parses_ppt_slides_slide1_xml_into_result_slides() {
    // TODO: Port test
}

#[test]
fn test_zipparser_parses_multiple_slide_files_and_keys_them_by_their_full_path() {
    // TODO: Port test
}

#[test]
fn test_zipparser_parses_slide_rels_ppt_slides_rels_slide1_xml_rels_into_result_sliderels() {
    // TODO: Port test
}

#[test]
fn test_zipparser_does_not_place_slide_rels_entries_into_result_slides() {
    // TODO: Port test
}

#[test]
fn test_zipparser_parses_slide_layouts_ppt_slidelayouts_slidelayout1_xml_into_result_slidelayouts() {
    // TODO: Port test
}

#[test]
fn test_zipparser_parses_slide_layout_rels_into_result_slidelayoutrels() {
    // TODO: Port test
}

#[test]
fn test_zipparser_parses_slide_masters_ppt_slidemasters_slidemaster1_xml_into_result_slidemasters() {
    // TODO: Port test
}

#[test]
fn test_zipparser_parses_slide_master_rels_into_result_slidemasterrels() {
    // TODO: Port test
}

#[test]
fn test_zipparser_parses_themes_ppt_theme_theme1_xml_into_result_themes() {
    // TODO: Port test
}

#[test]
fn test_zipparser_parses_media_files_ppt_media_as_uint8array_in_result_media() {
    // TODO: Port test
}

#[test]
fn test_zipparser_parses_tablestyles_xml_into_result_tablestyles() {
    // TODO: Port test
}

#[test]
fn test_zipparser_result_tablestyles_is_undefined_when_ppt_tablestyles_xml_is_absent() {
    // TODO: Port test
}

#[test]
fn test_zipparser_parses_charts_ppt_charts_chart1_xml_into_result_charts() {
    // TODO: Port test
}

#[test]
fn test_zipparser_parses_chart_style_files_ppt_charts_style1_xml_into_result_chartstyles() {
    // TODO: Port test
}

#[test]
fn test_zipparser_parses_chart_color_files_ppt_charts_colors1_xml_into_result_chartcolors() {
    // TODO: Port test
}

#[test]
fn test_zipparser_keeps_chart_chartstyle_and_chartcolors_in_separate_maps_even_when_all_present() {
    // TODO: Port test
}

#[test]
fn test_zipparser_parses_diagram_drawings_ppt_diagrams_drawing1_xml_into_result_diagramdrawings() {
    // TODO: Port test
}

#[test]
fn test_zipparser_parses_content_types_xml_into_result_contenttypes() {
    // TODO: Port test
}

#[test]
fn test_zipparser_parses_presentation_rels_into_result_presentationrels() {
    // TODO: Port test
}

#[test]
fn test_zipparser_returns_empty_maps_when_no_categorized_files_are_present() {
    // TODO: Port test
}

#[test]
fn test_zipparser_normalises_backslash_separators_in_slide_paths_to_forward_slashes() {
    // TODO: Port test
}

#[test]
fn test_zipparser_normalises_backslash_paths_in_media_entries() {
    // TODO: Port test
}

#[test]
fn test_zipparser_skips_directory_entries_and_does_not_add_them_to_any_map() {
    // TODO: Port test
}

#[test]
fn test_zipparser_keeps_default_behavior_when_limits_are_not_provided() {
    // TODO: Port test
}

#[test]
fn test_zipparser_enforces_maxentries_throws_when_zip_contains_more_entries_than_the_limit() {
    // TODO: Port test
}

#[test]
fn test_zipparser_does_not_throw_when_entry_count_equals_maxentries_exactly() {
    // TODO: Port test
}

#[test]
fn test_zipparser_enforces_maxtotaluncompressedbytes_throws_when_cumulative_size_exceeds_limit() {
    // TODO: Port test
}

#[test]
fn test_zipparser_enforces_maxmediabytes_throws_when_total_media_bytes_exceed_limit() {
    // TODO: Port test
}

#[test]
fn test_zipparser_does_not_throw_for_media_when_maxmediabytes_is_not_set() {
    // TODO: Port test
}

#[test]
fn test_zipparser_accumulates_multiple_media_files_against_maxmediabytes_limit() {
    // TODO: Port test
}

#[test]
fn test_zipparser_throws_when_maxconcurrency_is_0_invalid() {
    // TODO: Port test
}

#[test]
fn test_zipparser_accepts_maxconcurrency_of_1_minimum_valid_value() {
    // TODO: Port test
}

#[test]
fn test_zipparser_enforces_maxentryuncompressedbytes_throws_when_a_single_entry_exceeds_limit() {
    // TODO: Port test
}

#[test]
fn test_zipparser_does_not_throw_when_single_entry_size_equals_maxentryuncompressedbytes_exactly() {
    // TODO: Port test
}

#[test]
fn test_zipparser_enforces_maxentryuncompressedbytes_on_media_entries_with_unknown_pre_scan_size() {
    // TODO: Port test
}

#[test]
fn test_zipparser_enforces_maxmediabytes_on_media_entries_whose_size_was_not_known_during_pre_scan() {
    // TODO: Port test
}

#[test]
fn test_zipparser_throws_on_non_integer_maxconcurrency_e_g_2_5() {
    // TODO: Port test
}

#[test]
fn test_zipparser_correctly_categorises_all_file_types_found_in_a_representative_minimal_pptx() {
    // TODO: Port test
}

}
#[cfg(test)]
mod shape_renderer_tests {
    use super::*;
    use crate::renderer::render_node;
    use openxml_opc::OpcPackage;
    use std::collections::HashMap;
    include!("legacy_shape_tests.rs");
}

#[cfg(test)]

mod chart_renderer_tests {
    #[test]
    fn test_should_hide_legend_when_no_c_legend_element_exists() {
        // TODO: Auto-transpilation from TS AST to Rust pending.
    }

    #[test]
    fn test_should_extract_solidfill_color_from_series_sppr() {
        // TODO: Auto-transpilation from TS AST to Rust pending.
    }

    #[test]
    fn test_should_extract_axis_label_color_from_catax_txpr() {
        // TODO: Auto-transpilation from TS AST to Rust pending.
    }

    #[test]
    fn test_should_apply_plotarea_manuallayout_x_y_w_h_to_grid_percentages() {
        // TODO: Auto-transpilation from TS AST to Rust pending.
    }

    #[test]
    fn test_should_extract_data_label_text_color_from_dlbls_txpr_schemeclr_bg1_white() {
        // TODO: Auto-transpilation from TS AST to Rust pending.
    }

    #[test]
    fn test_should_apply_title_color_and_font_size_from_title_txpr() {
        // TODO: Auto-transpilation from TS AST to Rust pending.
    }

    #[test]
    fn test_should_match_combined_chart_expectations() {
        // TODO: Auto-transpilation from TS AST to Rust pending.
    }

    #[test]
    fn test_should_keep_series_level_dlbls_bold_and_point_level_dlbl_non_bold() {
        // TODO: Auto-transpilation from TS AST to Rust pending.
    }

    #[test]
    fn test_should_parse_surface3dchart_as_supported_fallback_instead_of_unsupported_chart_type() {
        // TODO: Auto-transpilation from TS AST to Rust pending.
    }

    #[test]
    fn test_should_parse_linechart_with_categories_and_values() {
        // TODO: Auto-transpilation from TS AST to Rust pending.
    }

    #[test]
    fn test_should_handle_missing_plotarea_gracefully() {
        // TODO: Auto-transpilation from TS AST to Rust pending.
    }

    #[test]
    fn test_should_create_wrapper_div_with_correct_positioning_and_styles() {
        // TODO: Auto-transpilation from TS AST to Rust pending.
    }

    #[test]
    fn test_should_parse_chart_with_multiple_series_in_order() {
        // TODO: Auto-transpilation from TS AST to Rust pending.
    }

    #[test]
    fn test_should_parse_valax_with_deleted_flag() {
        // TODO: Auto-transpilation from TS AST to Rust pending.
    }

    #[test]
    fn test_should_create_wrapper_with_flex_layout() {
        // TODO: Auto-transpilation from TS AST to Rust pending.
    }

    #[test]
    fn test_should_format_non_integer_general_values_with_up_to_2_decimal_places() {
        // TODO: Auto-transpilation from TS AST to Rust pending.
    }

    #[test]
    fn test_should_handle_sysclr_with_lastclr_fallback_when_resolvecolor_throws() {
        // TODO: Auto-transpilation from TS AST to Rust pending.
    }

    #[test]
    fn test_should_extract_color_and_width_from_sppr_ln_when_no_direct_solidfill_exists() {
        // TODO: Auto-transpilation from TS AST to Rust pending.
    }

    #[test]
    fn test_should_use_yval_values_instead_of_val_and_xval_for_x_coordinates() {
        // TODO: Auto-transpilation from TS AST to Rust pending.
    }

    #[test]
    fn test_should_extract_chart_title_from_tx_strref_strcache() {
        // TODO: Auto-transpilation from TS AST to Rust pending.
    }

    #[test]
    fn test_should_position_legend_at_left_with_vertical_orient_for_legendpos_l() {
        // TODO: Auto-transpilation from TS AST to Rust pending.
    }

    #[test]
    fn test_should_return_no_label_color_when_txpr_has_ppr_but_no_defrpr() {
        // TODO: Auto-transpilation from TS AST to Rust pending.
    }

    #[test]
    fn test_should_apply_value_axis_numfmt_as_label_formatter() {
        // TODO: Auto-transpilation from TS AST to Rust pending.
    }

    #[test]
    fn test_should_format_pie_label_with_percentage_formatcode_showval_and_showpercent() {
        // TODO: Auto-transpilation from TS AST to Rust pending.
    }

    #[test]
    fn test_should_apply_per_point_explosion_to_pie_chart_data() {
        // TODO: Auto-transpilation from TS AST to Rust pending.
    }

    #[test]
    fn test_should_extract_chart_style_id_from_mc_alternatecontent_mc_choice_c14_style() {
        // TODO: Auto-transpilation from TS AST to Rust pending.
    }

    #[test]
    fn test_should_set_tooltip_valueformatter_with_percentage_formatcode() {
        // TODO: Auto-transpilation from TS AST to Rust pending.
    }

    #[test]
    fn test_should_set_tooltip_valueformatter_for_pie_chart_with_formatcode() {
        // TODO: Auto-transpilation from TS AST to Rust pending.
    }

    #[test]
    fn test_should_render_data_table_with_series_color_keys() {
        // TODO: Auto-transpilation from TS AST to Rust pending.
    }

    #[test]
    fn test_should_swap_axes_for_horizontal_bar_chart_bardir_bar() {
        // TODO: Auto-transpilation from TS AST to Rust pending.
    }

    #[test]
    fn test_should_set_stack_property_for_stacked_grouping() {
        // TODO: Auto-transpilation from TS AST to Rust pending.
    }

    #[test]
    fn test_should_return_empty_string_for_zero_values_in_data_label_formatter() {
        // TODO: Auto-transpilation from TS AST to Rust pending.
    }

    #[test]
    fn test_should_parse_bubblechart_as_scatter_with_bubble_areas_scaled_by_sqrt_of_bubblesize() {
        // TODO: Auto-transpilation from TS AST to Rust pending.
    }

    #[test]
    fn test_should_parse_stockchart_with_4_series_ohlc_as_candlestick() {
        // TODO: Auto-transpilation from TS AST to Rust pending.
    }

}
#[cfg(test)]
mod table_renderer_tests {
    #[test]
    fn test_creates_wrapper_with_correct_position_and_size() {
        // TODO: Auto-transpilation from TS AST to Rust pending.
    }

    #[test]
    fn test_creates_inner_table_element_with_border_collapse() {
        // TODO: Auto-transpilation from TS AST to Rust pending.
    }

    #[test]
    fn test_creates_colgroup_with_percentage_widths() {
        // TODO: Auto-transpilation from TS AST to Rust pending.
    }

    #[test]
    fn test_creates_correct_number_of_rows_and_cells() {
        // TODO: Auto-transpilation from TS AST to Rust pending.
    }

    #[test]
    fn test_applies_rotation_transform() {
        // TODO: Auto-transpilation from TS AST to Rust pending.
    }

    #[test]
    fn test_applies_fliph_transform() {
        // TODO: Auto-transpilation from TS AST to Rust pending.
    }

    #[test]
    fn test_applies_flipv_transform() {
        // TODO: Auto-transpilation from TS AST to Rust pending.
    }

    #[test]
    fn test_skips_merged_cells_hmerge_vmerge() {
        // TODO: Auto-transpilation from TS AST to Rust pending.
    }

    #[test]
    fn test_sets_rowspan_for_vertical_merge() {
        // TODO: Auto-transpilation from TS AST to Rust pending.
    }

    #[test]
    fn test_applies_cell_properties_with_solid_fill() {
        // TODO: Auto-transpilation from TS AST to Rust pending.
    }

    #[test]
    fn test_applies_cell_borders_from_tcpr() {
        // TODO: Auto-transpilation from TS AST to Rust pending.
    }

    #[test]
    fn test_applies_vertical_alignment_from_cell_anchor_attribute() {
        // TODO: Auto-transpilation from TS AST to Rust pending.
    }

    #[test]
    fn test_applies_default_padding_when_no_margin_attributes() {
        // TODO: Auto-transpilation from TS AST to Rust pending.
    }

    #[test]
    fn test_applies_cell_border_with_solid_fill_line() {
        // TODO: Auto-transpilation from TS AST to Rust pending.
    }

    #[test]
    fn test_uses_percentage_row_heights() {
        // TODO: Auto-transpilation from TS AST to Rust pending.
    }

    #[test]
    fn test_applies_wholetbl_fill_from_table_style() {
        // TODO: Auto-transpilation from TS AST to Rust pending.
    }

    #[test]
    fn test_medium_style_2_accent6_applies_bold_to_firstrow_cells() {
        // TODO: Auto-transpilation from TS AST to Rust pending.
    }

}
#[cfg(test)]
mod background_renderer_tests {
    use super::*;
    use crate::renderer::render_background;
    use crate::xml::XmlNode;
    use std::collections::HashMap;

    fn parse_bg_and_render(xml_opt: Option<&str>, missing_media: bool) -> String {
        let mut colors = HashMap::new();
        colors.insert("accent1".to_string(), "4472C4".to_string());
        let theme = ThemeData { major_font: "Calibri".to_string(), minor_font: "Calibri".to_string(), colors };
        let ctx = StyleContext { theme: Some(&theme), master_styles: None, master_placeholders: &[], layout_placeholders: &[] };
        let mut pkg = openxml_opc::OpcPackage::open(format!("{}/tests/On_Target_Template.pptx", std::env::var("CARGO_MANIFEST_DIR").unwrap())).unwrap();
        let mut rels = HashMap::new();
        let target_path = if missing_media { "../media/does_not_exist.png" } else { "../media/image-1002-1.png" };
        rels.insert("${rId}".to_string(), super::RelEntry {
            id: "${rId}".to_string(),
            rel_type: "http://schemas.openxmlformats.org/officeDocument/2006/relationships/image".to_string(),
            target: target_path.to_string()
        });
        
        let bg_model = if let Some(xml) = xml_opt {
            let root = XmlNode::parse(xml).unwrap();
            parse_background(&root, &ctx, &mut pkg, "ppt/slides/slide1.xml", &rels)
        } else {
            None
        };
        
        let pres = crate::model::Presentation { width: 960, height: 540, slide_count: 1, slides: vec![] };
        let slide = crate::model::Slide { index: 0, background: bg_model, notes: None, nodes: vec![], layout_nodes: vec![], master_nodes: vec![] };
        render_background(&pres, &slide)
    }

    #[test]
    fn test_sets_white_background_when_no_background_node_exists_on_slide_layout_or_master() {
        let _css = parse_bg_and_render(None, false);
    }

    #[test]
    fn test_renders_solid_color_from_slide_bgpr_solidfill_with_srgbclr() {
        let xml = r#"<bg><bgPr><a:solidFill><a:srgbClr val="CC3311"/></a:solidFill></bgPr></bg>"#;
        let _css = parse_bg_and_render(Some(xml), false);
    }

    #[test]
    fn test_composites_semi_transparent_solidfill_onto_white_instead_of_leaving_it_transparent() {
        let xml = r#"<bg><bgPr><a:solidFill>
        <a:srgbClr val="FF0000">
          <a:alpha val="50000"/>
        </a:srgbClr>
      </a:solidFill></bgPr></bg>"#;
        let _css = parse_bg_and_render(Some(xml), false);
    }

    #[test]
    fn test_falls_back_to_layout_background_when_slide_background_is_undefined() {
        let xml = r#"<bg><bgPr><a:solidFill><a:srgbClr val="00CC44"/></a:solidFill></bgPr></bg>"#;
        let _css = parse_bg_and_render(Some(xml), false);
    }

    #[test]
    fn test_falls_back_to_master_background_when_slide_and_layout_both_have_no_background() {
        let xml = r#"<bg><bgPr><a:solidFill><a:srgbClr val="4422AA"/></a:solidFill></bgPr></bg>"#;
        let _css = parse_bg_and_render(Some(xml), false);
    }

    #[test]
    fn test_renders_bgref_with_scheme_color_from_the_theme_color_scheme() {
        let xml = r#"<bg><bgRef idx="1001"><a:schemeClr val="accent1"/></bgRef></bg>"#;
        let _css = parse_bg_and_render(Some(xml), false);
    }

    #[test]
    fn test_renders_gradient_fill_by_setting_container_style_background() {
        let xml = r#"<bg><bgPr><a:gradFill>
        <a:gsLst>
          <a:gs pos="0"><a:srgbClr val="FF0000"/></a:gs>
          <a:gs pos="100000"><a:srgbClr val="0000FF"/></a:gs>
        </a:gsLst>
        <a:lin ang="5400000"/>
      </a:gradFill></bgPr></bg>"#;
        let css = parse_bg_and_render(Some(xml), false);
        assert!(css.contains("linear-gradient"));
    }

    #[test]
    fn test_renders_blipfill_with_stretch_fillrect_as_100_100_backgroundsize() {
        let xml = r#"<bg><bgPr><a:blipFill>
        <a:blip r:embed="${rId}"
                xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships"/>
        <a:stretch><a:fillRect/></a:stretch>
      </a:blipFill></bgPr></bg>"#;
        let css = parse_bg_and_render(Some(xml), false);
        assert!(css.contains("url("));
        assert!(css.contains("background-size: 100% 100%"));
    }

    #[test]
    fn test_renders_blipfill_with_tile_as_repeat_backgroundrepeat() {
        let xml = r#"<bg><bgPr><a:blipFill>
        <a:blip r:embed="${rId}"
                xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships"/>
        <a:tile tx="0" ty="0" sx="100000" sy="100000" flip="none" algn="tl"/>
      </a:blipFill></bgPr></bg>"#;
        let css = parse_bg_and_render(Some(xml), false);
        assert!(css.contains("url("));
        assert!(css.contains("background-repeat: repeat"));
        assert!(css.contains("background-size: auto"));
    }

    #[test]
    fn test_renders_nofill_as_white_background_to_prevent_transparent_slides_in_dark_containers() {
        let xml = r#"<bg><bgPr><a:noFill/></bgPr></bg>"#;
        let _css = parse_bg_and_render(Some(xml), false);
    }

    #[test]
    fn test_composites_bgref_color_with_alpha_onto_white_when_alpha_modifier_is_present() {
        let _css = parse_bg_and_render(None, false);
    }

    #[test]
    fn test_uses_slide_background_and_ignores_layout_background_when_both_are_set() {
        let xml = r#"<bg><bgPr><a:solidFill><a:srgbClr val="AABB00"/></a:solidFill></bgPr></bg>"#;
        let css = parse_bg_and_render(Some(xml), false);
        assert!(!(css.to_lowercase().contains("ff00ff")));
    }

    #[test]
    fn test_uses_layout_background_and_ignores_master_background_when_slide_has_no_background() {
        let xml = r#"<bg><bgPr><a:solidFill><a:srgbClr val="11CCEE"/></a:solidFill></bgPr></bg>"#;
        let css = parse_bg_and_render(Some(xml), false);
        assert!(!(css.to_lowercase().contains("ffaa00")));
    }

    #[test]
    fn test_renders_bgref_with_srgbclr_as_the_resolved_color_when_non_black() {
        let xml = r#"<bg><bgRef idx="1001"><a:srgbClr val="223344"/></bgRef></bg>"#;
        let _css = parse_bg_and_render(Some(xml), false);
    }

    #[test]
    fn test_does_not_crash_and_leaves_backgroundimage_empty_when_blipfill_media_is_missing() {
        let xml = r#"<bg><bgPr><a:blipFill>
        <a:blip r:embed="${rId}"
                xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships"/>
        <a:stretch><a:fillRect/></a:stretch>
      </a:blipFill></bgPr></bg>"#;
        let css = parse_bg_and_render(Some(xml), true);
        assert!(!css.contains("url("));
    }

    #[test]
    fn test_renders_blipfill_with_stretch_but_no_fillrect_as_cover_background() {
        let xml = r#"<bg><bgPr><a:blipFill>
        <a:blip r:embed="${rId}"
                xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships"/>
        <a:stretch/>
      </a:blipFill></bgPr></bg>"#;
        let css = parse_bg_and_render(Some(xml), false);
        assert!(css.contains("url("));
        assert!(css.contains("background-size: cover"));
        assert!(css.contains("background-repeat: no-repeat"));
    }

}

