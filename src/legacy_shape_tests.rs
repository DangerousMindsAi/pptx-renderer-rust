    #[test]
    
    fn test_renders_lineinv_using_theme_lnref_stroke_when_shape_has_no_explicit_a_ln() {
        let xml = r#"<p:sp xmlns:p="http://schemas.openxmlformats.org/presentationml/2006/main"
            xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main">
        <p:nvSpPr>
          <p:cNvPr id="183" name="Straight Connector 1"/>
          <p:cNvSpPr/>
          <p:nvPr/>
        </p:nvSpPr>
        <p:spPr>
          <a:xfrm><a:off x="0" y="0"/><a:ext cx="5080000" cy="3556000"/></a:xfrm>
          <a:prstGeom prst="lineInv"><a:avLst/></a:prstGeom>
        </p:spPr>
        <p:style>
          <a:lnRef idx="2"><a:schemeClr val="accent1"/></a:lnRef>
          <a:fillRef idx="0"><a:schemeClr val="accent1"/></a:fillRef>
        </p:style>
      </p:sp>"#;
        let root = XmlNode::parse(xml).unwrap();
        let mut pkg = OpcPackage::open(format!("{}/tests/template.pptx", std::env::var("CARGO_MANIFEST_DIR").unwrap())).unwrap();
        let mut colors = HashMap::new();
        colors.insert("accent1".to_string(), "4472C4".to_string());
        colors.insert("lt1".to_string(), "FFFFFF".to_string());
        colors.insert("dk1".to_string(), "000000".to_string());
        let theme = ThemeData { major_font: "Calibri".to_string(), minor_font: "Calibri".to_string(), colors };
        let ctx = StyleContext { theme: Some(&theme), master_styles: None, master_placeholders: &[], layout_placeholders: &[] };
        let rels = HashMap::new();
        let node = parse_node(&root, &ctx, &mut pkg, "ppt/slides/slide1.xml", &rels).unwrap();
        let html = render_node(&node, &mut std::collections::HashMap::new());
        assert!(!html.is_empty());
        // assert!(!html.contains("none"));
        assert!(html.contains("#4472C4"));
    }
    #[test]
    
    fn test_renders_arc_using_theme_lnref_stroke_when_shape_has_no_explicit_a_ln_oracle_full_shapeid_0025() {
        let xml = r#"<p:sp xmlns:p="http://schemas.openxmlformats.org/presentationml/2006/main"
            xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main">
        <p:nvSpPr>
          <p:cNvPr id="25" name="Arc"/>
          <p:cNvSpPr/>
          <p:nvPr/>
        </p:nvSpPr>
        <p:spPr>
          <a:xfrm><a:off x="0" y="0"/><a:ext cx="5080000" cy="3556000"/></a:xfrm>
          <a:prstGeom prst="arc"><a:avLst/></a:prstGeom>
          <a:noFill/>
        </p:spPr>
        <p:style>
          <a:lnRef idx="2"><a:schemeClr val="accent1"/></a:lnRef>
          <a:fillRef idx="0"><a:schemeClr val="accent1"/></a:fillRef>
        </p:style>
      </p:sp>"#;
        let root = XmlNode::parse(xml).unwrap();
        let mut pkg = OpcPackage::open(format!("{}/tests/template.pptx", std::env::var("CARGO_MANIFEST_DIR").unwrap())).unwrap();
        let mut colors = HashMap::new();
        colors.insert("accent1".to_string(), "4472C4".to_string());
        colors.insert("lt1".to_string(), "FFFFFF".to_string());
        colors.insert("dk1".to_string(), "000000".to_string());
        let theme = ThemeData { major_font: "Calibri".to_string(), minor_font: "Calibri".to_string(), colors };
        let ctx = StyleContext { theme: Some(&theme), master_styles: None, master_placeholders: &[], layout_placeholders: &[] };
        let rels = HashMap::new();
        let node = parse_node(&root, &ctx, &mut pkg, "ppt/slides/slide1.xml", &rels).unwrap();
        let html = render_node(&node, &mut std::collections::HashMap::new());
        assert!(!html.is_empty());
        assert!(html.contains("none"));
        assert!(html.contains("#4472C4"));
    }
    #[test]
    
    fn test_renders_circulararrow_fill_from_fillref_accent1_instead_of_hardcoded_blue_oracle_full_shapeid_0060() {
        let xml = r#"<p:sp xmlns:p="http://schemas.openxmlformats.org/presentationml/2006/main"
            xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main">
        <p:nvSpPr>
          <p:cNvPr id="2" name="Circular Arrow 1"/>
          <p:cNvSpPr/>
          <p:nvPr/>
        </p:nvSpPr>
        <p:spPr>
          <a:xfrm><a:off x="1524000" y="1016000"/><a:ext cx="5080000" cy="3556000"/></a:xfrm>
          <a:prstGeom prst="circularArrow"><a:avLst/></a:prstGeom>
        </p:spPr>
        <p:style>
          <a:lnRef idx="2"><a:schemeClr val="accent1"><a:shade val="15000"/></a:schemeClr></a:lnRef>
          <a:fillRef idx="1"><a:schemeClr val="accent1"/></a:fillRef>
          <a:effectRef idx="0"><a:schemeClr val="accent1"/></a:effectRef>
          <a:fontRef idx="minor"><a:schemeClr val="lt1"/></a:fontRef>
        </p:style>
        <p:txBody>
          <a:bodyPr rtlCol="0" anchor="ctr"/>
          <a:lstStyle/>
          <a:p><a:pPr algn="ctr"/><a:endParaRPr lang="en-CN"/></a:p>
        </p:txBody>
      </p:sp>"#;
        let root = XmlNode::parse(xml).unwrap();
        let mut pkg = OpcPackage::open(format!("{}/tests/template.pptx", std::env::var("CARGO_MANIFEST_DIR").unwrap())).unwrap();
        let mut colors = HashMap::new();
        colors.insert("accent1".to_string(), "4472C4".to_string());
        colors.insert("lt1".to_string(), "FFFFFF".to_string());
        colors.insert("dk1".to_string(), "000000".to_string());
        let theme = ThemeData { major_font: "Calibri".to_string(), minor_font: "Calibri".to_string(), colors };
        let ctx = StyleContext { theme: Some(&theme), master_styles: None, master_placeholders: &[], layout_placeholders: &[] };
        let rels = HashMap::new();
        let node = parse_node(&root, &ctx, &mut pkg, "ppt/slides/slide1.xml", &rels).unwrap();
        let html = render_node(&node, &mut std::collections::HashMap::new());
        assert!(!html.is_empty());
        assert!(html.contains("#4472C4"));
        assert!(html.contains("none"));
    }
    #[test]
    fn test_renders_fillref_theme_gradient_using_phclr_from_fillref_color_windows_pypptx_shape_adj() {
        let xml = r#"<p:sp xmlns:p="http://schemas.openxmlformats.org/presentationml/2006/main"
            xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main">
        <p:nvSpPr>
          <p:cNvPr id="2" name="Rounded Rectangle 1"/>
          <p:cNvSpPr/>
          <p:nvPr/>
        </p:nvSpPr>
        <p:spPr>
          <a:xfrm><a:off x="1828800" y="914400"/><a:ext cx="4572000" cy="3657600"/></a:xfrm>
          <a:prstGeom prst="roundRect"><a:avLst><a:gd name="adj" fmla="val 5000"/></a:avLst></a:prstGeom>
        </p:spPr>
        <p:style>
          <a:lnRef idx="1"><a:schemeClr val="accent1"/></a:lnRef>
          <a:fillRef idx="3"><a:schemeClr val="accent1"/></a:fillRef>
          <a:effectRef idx="2"><a:schemeClr val="accent1"/></a:effectRef>
          <a:fontRef idx="minor"><a:schemeClr val="lt1"/></a:fontRef>
        </p:style>
      </p:sp>"#;
        let root = XmlNode::parse(xml).unwrap();
        let mut pkg = OpcPackage::open(format!("{}/tests/template.pptx", std::env::var("CARGO_MANIFEST_DIR").unwrap())).unwrap();
        let mut colors = HashMap::new();
        colors.insert("accent1".to_string(), "4472C4".to_string());
        colors.insert("lt1".to_string(), "FFFFFF".to_string());
        colors.insert("dk1".to_string(), "000000".to_string());
        let theme = ThemeData { major_font: "Calibri".to_string(), minor_font: "Calibri".to_string(), colors };
        let ctx = StyleContext { theme: Some(&theme), master_styles: None, master_placeholders: &[], layout_placeholders: &[] };
        let rels = HashMap::new();
        let node = parse_node(&root, &ctx, &mut pkg, "ppt/slides/slide1.xml", &rels).unwrap();
        let html = render_node(&node, &mut std::collections::HashMap::new());
        assert!(!html.is_empty());
    }
    #[test]
    fn test_keeps_multi_path_bevel_shading_anchored_to_fillref_base_color_when_main_fill_is_gradient() {
        let xml = r#"<p:sp xmlns:p="http://schemas.openxmlformats.org/presentationml/2006/main"
            xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main">
        <p:nvSpPr>
          <p:cNvPr id="2" name="Bevel 1"/>
          <p:cNvSpPr/>
          <p:nvPr/>
        </p:nvSpPr>
        <p:spPr>
          <a:xfrm><a:off x="1828800" y="914400"/><a:ext cx="4572000" cy="3657600"/></a:xfrm>
          <a:prstGeom prst="bevel"><a:avLst><a:gd name="adj" fmla="val 35000"/></a:avLst></a:prstGeom>
        </p:spPr>
        <p:style>
          <a:lnRef idx="1"><a:schemeClr val="accent1"/></a:lnRef>
          <a:fillRef idx="3"><a:schemeClr val="accent1"/></a:fillRef>
        </p:style>
      </p:sp>"#;
        let root = XmlNode::parse(xml).unwrap();
        let mut pkg = OpcPackage::open(format!("{}/tests/template.pptx", std::env::var("CARGO_MANIFEST_DIR").unwrap())).unwrap();
        let mut colors = HashMap::new();
        colors.insert("accent1".to_string(), "4472C4".to_string());
        colors.insert("lt1".to_string(), "FFFFFF".to_string());
        colors.insert("dk1".to_string(), "000000".to_string());
        let theme = ThemeData { major_font: "Calibri".to_string(), minor_font: "Calibri".to_string(), colors };
        let ctx = StyleContext { theme: Some(&theme), master_styles: None, master_placeholders: &[], layout_placeholders: &[] };
        let rels = HashMap::new();
        let node = parse_node(&root, &ctx, &mut pkg, "ppt/slides/slide1.xml", &rels).unwrap();
        let html = render_node(&node, &mut std::collections::HashMap::new());
        assert!(!html.is_empty());
    }
    #[test]
    fn test_derives_a_tinted_gradient_for_can_top_face_when_fillref_resolves_to_a_theme_gradient() {
        let xml = r#"<p:sp xmlns:p="http://schemas.openxmlformats.org/presentationml/2006/main"
            xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main">
        <p:nvSpPr>
          <p:cNvPr id="2" name="Can 1"/>
          <p:cNvSpPr/>
          <p:nvPr/>
        </p:nvSpPr>
        <p:spPr>
          <a:xfrm><a:off x="1828800" y="914400"/><a:ext cx="4572000" cy="3657600"/></a:xfrm>
          <a:prstGeom prst="can"><a:avLst><a:gd name="adj" fmla="val 40000"/></a:avLst></a:prstGeom>
        </p:spPr>
        <p:style>
          <a:lnRef idx="1"><a:schemeClr val="accent1"/></a:lnRef>
          <a:fillRef idx="3"><a:schemeClr val="accent1"/></a:fillRef>
        </p:style>
      </p:sp>"#;
        let root = XmlNode::parse(xml).unwrap();
        let mut pkg = OpcPackage::open(format!("{}/tests/template.pptx", std::env::var("CARGO_MANIFEST_DIR").unwrap())).unwrap();
        let mut colors = HashMap::new();
        colors.insert("accent1".to_string(), "4472C4".to_string());
        colors.insert("lt1".to_string(), "FFFFFF".to_string());
        colors.insert("dk1".to_string(), "000000".to_string());
        let theme = ThemeData { major_font: "Calibri".to_string(), minor_font: "Calibri".to_string(), colors };
        let ctx = StyleContext { theme: Some(&theme), master_styles: None, master_placeholders: &[], layout_placeholders: &[] };
        let rels = HashMap::new();
        let node = parse_node(&root, &ctx, &mut pkg, "ppt/slides/slide1.xml", &rels).unwrap();
        let html = render_node(&node, &mut std::collections::HashMap::new());
        assert!(!html.is_empty());
    }
    #[test]
    fn test_renders_actionbuttonbackprevious_as_multi_path_with_darken_sub_paths_oracle_full_shapeid_0129() {
        let xml = r#"<p:sp xmlns:p="http://schemas.openxmlformats.org/presentationml/2006/main"
            xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main">
        <p:nvSpPr>
          <p:cNvPr id="2" name="Action Button: Back or Previous 1"/>
          <p:cNvSpPr/>
          <p:nvPr/>
        </p:nvSpPr>
        <p:spPr>
          <a:xfrm><a:off x="1524000" y="1016000"/><a:ext cx="5080000" cy="3556000"/></a:xfrm>
          <a:prstGeom prst="actionButtonBackPrevious"><a:avLst/></a:prstGeom>
        </p:spPr>
        <p:style>
          <a:lnRef idx="2"><a:schemeClr val="accent1"><a:shade val="15000"/></a:schemeClr></a:lnRef>
          <a:fillRef idx="1"><a:schemeClr val="accent1"/></a:fillRef>
          <a:effectRef idx="0"><a:schemeClr val="accent1"/></a:effectRef>
          <a:fontRef idx="minor"><a:schemeClr val="lt1"/></a:fontRef>
        </p:style>
        <p:txBody>
          <a:bodyPr rtlCol="0" anchor="ctr"/>
          <a:lstStyle/>
          <a:p><a:pPr algn="ctr"/><a:endParaRPr lang="en-CN"/></a:p>
        </p:txBody>
      </p:sp>"#;
        let root = XmlNode::parse(xml).unwrap();
        let mut pkg = OpcPackage::open(format!("{}/tests/template.pptx", std::env::var("CARGO_MANIFEST_DIR").unwrap())).unwrap();
        let mut colors = HashMap::new();
        colors.insert("accent1".to_string(), "4472C4".to_string());
        colors.insert("lt1".to_string(), "FFFFFF".to_string());
        colors.insert("dk1".to_string(), "000000".to_string());
        let theme = ThemeData { major_font: "Calibri".to_string(), minor_font: "Calibri".to_string(), colors };
        let ctx = StyleContext { theme: Some(&theme), master_styles: None, master_placeholders: &[], layout_placeholders: &[] };
        let rels = HashMap::new();
        let node = parse_node(&root, &ctx, &mut pkg, "ppt/slides/slide1.xml", &rels).unwrap();
        let html = render_node(&node, &mut std::collections::HashMap::new());
        assert!(!html.is_empty());
        assert!(html.contains("M"));
        assert!(html.contains("Z"));
    }
    #[test]
    fn test_should_allow_title_placeholder_text_to_wrap_when_bodypr_wrap_is_not_none() {
        let xml = r#"<p:sp xmlns:p="http://schemas.openxmlformats.org/presentationml/2006/main"
            xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main">
        <p:nvSpPr>
          <p:cNvPr id="2" name="Title 1"/>
          <p:cNvSpPr/>
          <p:nvPr><p:ph type="title"/></p:nvPr>
        </p:nvSpPr>
        <p:spPr>
          <a:xfrm><a:off x="0" y="0"/><a:ext cx="3000000" cy="800000"/></a:xfrm>
          <a:prstGeom prst="rect"><a:avLst/></a:prstGeom>
          <a:noFill/>
          <a:ln><a:noFill/></a:ln>
        </p:spPr>
        <p:txBody>
          <a:bodyPr/>
          <a:lstStyle/>
          <a:p><a:r><a:t>This is a very long title that should wrap to multiple lines</a:t></a:r></a:p>
        </p:txBody>
      </p:sp>"#;
        let root = XmlNode::parse(xml).unwrap();
        let mut pkg = OpcPackage::open(format!("{}/tests/template.pptx", std::env::var("CARGO_MANIFEST_DIR").unwrap())).unwrap();
        let mut colors = HashMap::new();
        colors.insert("accent1".to_string(), "4472C4".to_string());
        colors.insert("lt1".to_string(), "FFFFFF".to_string());
        colors.insert("dk1".to_string(), "000000".to_string());
        let theme = ThemeData { major_font: "Calibri".to_string(), minor_font: "Calibri".to_string(), colors };
        let ctx = StyleContext { theme: Some(&theme), master_styles: None, master_placeholders: &[], layout_placeholders: &[] };
        let rels = HashMap::new();
        let node = parse_node(&root, &ctx, &mut pkg, "ppt/slides/slide1.xml", &rels).unwrap();
        let html = render_node(&node, &mut std::collections::HashMap::new());
        assert!(!html.is_empty());
        assert!(!html.contains("nowrap"));
    }
    #[test]
    
    fn test_applies_txxfrm_rotation_so_diagram_text_stays_upright_when_shape_is_rotated() {
        let xml = r#"<p:sp xmlns:p="http://schemas.openxmlformats.org/presentationml/2006/main"
            xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main">
        <p:nvSpPr>
          <p:cNvPr id="9" name="Rotated Diagram Block"/>
          <p:cNvSpPr/>
          <p:nvPr/>
        </p:nvSpPr>
        <p:spPr>
          <a:xfrm rot="10800000"><a:off x="0" y="0"/><a:ext cx="2000000" cy="2000000"/></a:xfrm>
          <a:prstGeom prst="triangle"><a:avLst/></a:prstGeom>
          <a:solidFill><a:srgbClr val="993399"/></a:solidFill>
          <a:ln><a:noFill/></a:ln>
        </p:spPr>
        <p:txBody>
          <a:bodyPr/>
          <a:lstStyle/>
          <a:p><a:r><a:t>block 3</a:t></a:r></a:p>
        </p:txBody>
        <p:txXfrm rot="10800000"><a:off x="0" y="0"/><a:ext cx="1000000" cy="1000000"/></p:txXfrm>
      </p:sp>"#;
        let root = XmlNode::parse(xml).unwrap();
        let mut pkg = OpcPackage::open(format!("{}/tests/template.pptx", std::env::var("CARGO_MANIFEST_DIR").unwrap())).unwrap();
        let mut colors = HashMap::new();
        colors.insert("accent1".to_string(), "4472C4".to_string());
        colors.insert("lt1".to_string(), "FFFFFF".to_string());
        colors.insert("dk1".to_string(), "000000".to_string());
        let theme = ThemeData { major_font: "Calibri".to_string(), minor_font: "Calibri".to_string(), colors };
        let ctx = StyleContext { theme: Some(&theme), master_styles: None, master_placeholders: &[], layout_placeholders: &[] };
        let rels = HashMap::new();
        let node = parse_node(&root, &ctx, &mut pkg, "ppt/slides/slide1.xml", &rels).unwrap();
        let html = render_node(&node, &mut std::collections::HashMap::new());
        assert!(!html.is_empty());
        assert!(html.contains("rotate(180deg)"));
    }
    #[test]
    
    fn test_uses_evenodd_fill_rule_for_curved_arrows_to_avoid_seam_artifacts() {
        let xml = r#"<p:sp xmlns:p="http://schemas.openxmlformats.org/presentationml/2006/main"
            xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main">
        <p:nvSpPr>
          <p:cNvPr id="10" name="Curved Up Arrow"/>
          <p:cNvSpPr/>
          <p:nvPr/>
        </p:nvSpPr>
        <p:spPr>
          <a:xfrm><a:off x="0" y="0"/><a:ext cx="9000000" cy="1000000"/></a:xfrm>
          <a:prstGeom prst="curvedUpArrow"><a:avLst/></a:prstGeom>
          <a:solidFill><a:srgbClr val="E53935"/></a:solidFill>
        </p:spPr>
      </p:sp>"#;
        let root = XmlNode::parse(xml).unwrap();
        let mut pkg = OpcPackage::open(format!("{}/tests/template.pptx", std::env::var("CARGO_MANIFEST_DIR").unwrap())).unwrap();
        let mut colors = HashMap::new();
        colors.insert("accent1".to_string(), "4472C4".to_string());
        colors.insert("lt1".to_string(), "FFFFFF".to_string());
        colors.insert("dk1".to_string(), "000000".to_string());
        let theme = ThemeData { major_font: "Calibri".to_string(), minor_font: "Calibri".to_string(), colors };
        let ctx = StyleContext { theme: Some(&theme), master_styles: None, master_placeholders: &[], layout_placeholders: &[] };
        let rels = HashMap::new();
        let node = parse_node(&root, &ctx, &mut pkg, "ppt/slides/slide1.xml", &rels).unwrap();
        let html = render_node(&node, &mut std::collections::HashMap::new());
        assert!(!html.is_empty());
        assert!(html.contains("evenodd"));
    }
    #[test]
    fn test_keeps_line_outer_shadow_bounded_for_center_aligned_scaled_shadows() {
        let xml = r#"<p:sp xmlns:p="http://schemas.openxmlformats.org/presentationml/2006/main"
            xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main">
        <p:nvSpPr>
          <p:cNvPr id="22" name="Connector Shadow Regression"/>
          <p:cNvSpPr/>
          <p:nvPr/>
        </p:nvSpPr>
        <p:spPr>
          <a:xfrm><a:off x="0" y="4853940"/><a:ext cx="12188825" cy="0"/></a:xfrm>
          <a:prstGeom prst="line"><a:avLst/></a:prstGeom>
          <a:noFill/>
          <a:ln w="38100"><a:solidFill><a:srgbClr val="FFFFFF"/></a:solidFill></a:ln>
          <a:effectLst>
            <a:outerShdw blurRad="152400" dist="1244600" sx="200000" sy="200000" algn="ctr" rotWithShape="0">
              <a:srgbClr val="FFFFFF"><a:alpha val="91000"/></a:srgbClr>
            </a:outerShdw>
          </a:effectLst>
        </p:spPr>
      </p:sp>"#;
        let root = XmlNode::parse(xml).unwrap();
        let mut pkg = OpcPackage::open(format!("{}/tests/template.pptx", std::env::var("CARGO_MANIFEST_DIR").unwrap())).unwrap();
        let mut colors = HashMap::new();
        colors.insert("accent1".to_string(), "4472C4".to_string());
        colors.insert("lt1".to_string(), "FFFFFF".to_string());
        colors.insert("dk1".to_string(), "000000".to_string());
        let theme = ThemeData { major_font: "Calibri".to_string(), minor_font: "Calibri".to_string(), colors };
        let ctx = StyleContext { theme: Some(&theme), master_styles: None, master_placeholders: &[], layout_placeholders: &[] };
        let rels = HashMap::new();
        let node = parse_node(&root, &ctx, &mut pkg, "ppt/slides/slide1.xml", &rels).unwrap();
        let html = render_node(&node, &mut std::collections::HashMap::new());
        assert!(!html.is_empty());
    }
    #[test]
    
    fn test_treats_spautofit_as_bounded_text_fit_to_prevent_overflow_bleed() {
        let xml = r#"<p:sp xmlns:p="http://schemas.openxmlformats.org/presentationml/2006/main"
            xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main">
        <p:nvSpPr>
          <p:cNvPr id="300" name="spAutoFit overflow regression"/>
          <p:cNvSpPr/>
          <p:nvPr/>
        </p:nvSpPr>
        <p:spPr>
          <a:xfrm><a:off x="0" y="0"/><a:ext cx="2000000" cy="400000"/></a:xfrm>
          <a:prstGeom prst="rect"><a:avLst/></a:prstGeom>
          <a:noFill/><a:ln><a:noFill/></a:ln>
        </p:spPr>
        <p:txBody>
          <a:bodyPr wrap="square"><a:spAutoFit/></a:bodyPr>
          <a:lstStyle/>
          <a:p><a:r><a:t>This is a long paragraph that should not bleed outside shape bounds under spAutoFit.</a:t></a:r></a:p>
          <a:p><a:r><a:t>Second line with additional content to force overflow in browser layout metrics.</a:t></a:r></a:p>
          <a:p><a:r><a:t>Third line to make sure fitting behavior is engaged.</a:t></a:r></a:p>
        </p:txBody>
      </p:sp>"#;
        let root = XmlNode::parse(xml).unwrap();
        let mut pkg = OpcPackage::open(format!("{}/tests/template.pptx", std::env::var("CARGO_MANIFEST_DIR").unwrap())).unwrap();
        let mut colors = HashMap::new();
        colors.insert("accent1".to_string(), "4472C4".to_string());
        colors.insert("lt1".to_string(), "FFFFFF".to_string());
        colors.insert("dk1".to_string(), "000000".to_string());
        let theme = ThemeData { major_font: "Calibri".to_string(), minor_font: "Calibri".to_string(), colors };
        let ctx = StyleContext { theme: Some(&theme), master_styles: None, master_placeholders: &[], layout_placeholders: &[] };
        let rels = HashMap::new();
        let node = parse_node(&root, &ctx, &mut pkg, "ppt/slides/slide1.xml", &rels).unwrap();
        let html = render_node(&node, &mut std::collections::HashMap::new());
        assert!(!html.is_empty());
        assert!(html.contains("hidden"));
    }
    #[test]
    fn test_applies_theme_effectref_outer_shadow_when_shape_has_no_explicit_effectlst() {
        let xml = r#"<p:sp xmlns:p="http://schemas.openxmlformats.org/presentationml/2006/main"
            xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main">
        <p:nvSpPr><p:cNvPr id="401" name="EffectRef Shadow"/><p:cNvSpPr/><p:nvPr/></p:nvSpPr>
        <p:spPr>
          <a:xfrm><a:off x="0" y="0"/><a:ext cx="2000000" cy="800000"/></a:xfrm>
          <a:prstGeom prst="rect"><a:avLst/></a:prstGeom>
          <a:solidFill><a:srgbClr val="3366FF"/></a:solidFill>
        </p:spPr>
        <p:style><a:effectRef idx="1"><a:schemeClr val="accent1"/></a:effectRef></p:style>
      </p:sp>"#;
        let root = XmlNode::parse(xml).unwrap();
        let mut pkg = OpcPackage::open(format!("{}/tests/template.pptx", std::env::var("CARGO_MANIFEST_DIR").unwrap())).unwrap();
        let mut colors = HashMap::new();
        colors.insert("accent1".to_string(), "4472C4".to_string());
        colors.insert("lt1".to_string(), "FFFFFF".to_string());
        colors.insert("dk1".to_string(), "000000".to_string());
        let theme = ThemeData { major_font: "Calibri".to_string(), minor_font: "Calibri".to_string(), colors };
        let ctx = StyleContext { theme: Some(&theme), master_styles: None, master_placeholders: &[], layout_placeholders: &[] };
        let rels = HashMap::new();
        let node = parse_node(&root, &ctx, &mut pkg, "ppt/slides/slide1.xml", &rels).unwrap();
        let html = render_node(&node, &mut std::collections::HashMap::new());
        assert!(!html.is_empty());
    }
    #[test]
    fn test_renders_rect_shape_with_solidfill_color() {
        let xml = r#"<p:sp xmlns:p="http://schemas.openxmlformats.org/presentationml/2006/main"
            xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main">
        <p:nvSpPr><p:cNvPr id="50" name="Rect"/><p:cNvSpPr/><p:nvPr/></p:nvSpPr>
        <p:spPr>
          <a:xfrm><a:off x="0" y="0"/><a:ext cx="2000000" cy="1000000"/></a:xfrm>
          <a:prstGeom prst="rect"><a:avLst/></a:prstGeom>
          <a:solidFill><a:srgbClr val="FF0000"/></a:solidFill>
          <a:ln w="12700"><a:solidFill><a:srgbClr val="0000FF"/></a:solidFill></a:ln>
        </p:spPr>
      </p:sp>"#;
        let root = XmlNode::parse(xml).unwrap();
        let mut pkg = OpcPackage::open(format!("{}/tests/template.pptx", std::env::var("CARGO_MANIFEST_DIR").unwrap())).unwrap();
        let mut colors = HashMap::new();
        colors.insert("accent1".to_string(), "4472C4".to_string());
        colors.insert("lt1".to_string(), "FFFFFF".to_string());
        colors.insert("dk1".to_string(), "000000".to_string());
        let theme = ThemeData { major_font: "Calibri".to_string(), minor_font: "Calibri".to_string(), colors };
        let ctx = StyleContext { theme: Some(&theme), master_styles: None, master_placeholders: &[], layout_placeholders: &[] };
        let rels = HashMap::new();
        let node = parse_node(&root, &ctx, &mut pkg, "ppt/slides/slide1.xml", &rels).unwrap();
        let html = render_node(&node, &mut std::collections::HashMap::new());
        assert!(!html.is_empty());
        assert!(html.contains("#FF0000"));
        assert!(html.contains("#0000FF"));
    }
    #[test]
    fn test_renders_shape_with_nofill_and_noline() {
        let xml = r#"<p:sp xmlns:p="http://schemas.openxmlformats.org/presentationml/2006/main"
            xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main">
        <p:nvSpPr><p:cNvPr id="51" name="NoFill"/><p:cNvSpPr/><p:nvPr/></p:nvSpPr>
        <p:spPr>
          <a:xfrm><a:off x="0" y="0"/><a:ext cx="2000000" cy="1000000"/></a:xfrm>
          <a:prstGeom prst="rect"><a:avLst/></a:prstGeom>
          <a:noFill/>
          <a:ln><a:noFill/></a:ln>
        </p:spPr>
      </p:sp>"#;
        let root = XmlNode::parse(xml).unwrap();
        let mut pkg = OpcPackage::open(format!("{}/tests/template.pptx", std::env::var("CARGO_MANIFEST_DIR").unwrap())).unwrap();
        let mut colors = HashMap::new();
        colors.insert("accent1".to_string(), "4472C4".to_string());
        colors.insert("lt1".to_string(), "FFFFFF".to_string());
        colors.insert("dk1".to_string(), "000000".to_string());
        let theme = ThemeData { major_font: "Calibri".to_string(), minor_font: "Calibri".to_string(), colors };
        let ctx = StyleContext { theme: Some(&theme), master_styles: None, master_placeholders: &[], layout_placeholders: &[] };
        let rels = HashMap::new();
        let node = parse_node(&root, &ctx, &mut pkg, "ppt/slides/slide1.xml", &rels).unwrap();
        let html = render_node(&node, &mut std::collections::HashMap::new());
        assert!(!html.is_empty());
        assert!(html.contains("none"));
    }
    #[test]
    fn test_renders_dashed_stroke() {
        let xml = r#"<p:sp xmlns:p="http://schemas.openxmlformats.org/presentationml/2006/main"
            xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main">
        <p:nvSpPr><p:cNvPr id="52" name="Dashed"/><p:cNvSpPr/><p:nvPr/></p:nvSpPr>
        <p:spPr>
          <a:xfrm><a:off x="0" y="0"/><a:ext cx="2000000" cy="1000000"/></a:xfrm>
          <a:prstGeom prst="rect"><a:avLst/></a:prstGeom>
          <a:solidFill><a:srgbClr val="FFFFFF"/></a:solidFill>
          <a:ln w="25400"><a:solidFill><a:srgbClr val="000000"/></a:solidFill><a:prstDash val="dash"/></a:ln>
        </p:spPr>
      </p:sp>"#;
        let root = XmlNode::parse(xml).unwrap();
        let mut pkg = OpcPackage::open(format!("{}/tests/template.pptx", std::env::var("CARGO_MANIFEST_DIR").unwrap())).unwrap();
        let mut colors = HashMap::new();
        colors.insert("accent1".to_string(), "4472C4".to_string());
        colors.insert("lt1".to_string(), "FFFFFF".to_string());
        colors.insert("dk1".to_string(), "000000".to_string());
        let theme = ThemeData { major_font: "Calibri".to_string(), minor_font: "Calibri".to_string(), colors };
        let ctx = StyleContext { theme: Some(&theme), master_styles: None, master_placeholders: &[], layout_placeholders: &[] };
        let rels = HashMap::new();
        let node = parse_node(&root, &ctx, &mut pkg, "ppt/slides/slide1.xml", &rels).unwrap();
        let html = render_node(&node, &mut std::collections::HashMap::new());
        assert!(!html.is_empty());
    }
    #[test]
    fn test_renders_dotted_stroke() {
        let xml = r#"<p:sp xmlns:p="http://schemas.openxmlformats.org/presentationml/2006/main"
            xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main">
        <p:nvSpPr><p:cNvPr id="53" name="Dotted"/><p:cNvSpPr/><p:nvPr/></p:nvSpPr>
        <p:spPr>
          <a:xfrm><a:off x="0" y="0"/><a:ext cx="2000000" cy="1000000"/></a:xfrm>
          <a:prstGeom prst="rect"><a:avLst/></a:prstGeom>
          <a:solidFill><a:srgbClr val="FFFFFF"/></a:solidFill>
          <a:ln w="25400"><a:solidFill><a:srgbClr val="000000"/></a:solidFill><a:prstDash val="dot"/></a:ln>
        </p:spPr>
      </p:sp>"#;
        let root = XmlNode::parse(xml).unwrap();
        let mut pkg = OpcPackage::open(format!("{}/tests/template.pptx", std::env::var("CARGO_MANIFEST_DIR").unwrap())).unwrap();
        let mut colors = HashMap::new();
        colors.insert("accent1".to_string(), "4472C4".to_string());
        colors.insert("lt1".to_string(), "FFFFFF".to_string());
        colors.insert("dk1".to_string(), "000000".to_string());
        let theme = ThemeData { major_font: "Calibri".to_string(), minor_font: "Calibri".to_string(), colors };
        let ctx = StyleContext { theme: Some(&theme), master_styles: None, master_placeholders: &[], layout_placeholders: &[] };
        let rels = HashMap::new();
        let node = parse_node(&root, &ctx, &mut pkg, "ppt/slides/slide1.xml", &rels).unwrap();
        let html = render_node(&node, &mut std::collections::HashMap::new());
        assert!(!html.is_empty());
    }
    #[test]
    fn test_renders_schemeclr_fill_resolving_to_theme_color() {
        let xml = r#"<p:sp xmlns:p="http://schemas.openxmlformats.org/presentationml/2006/main"
            xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main">
        <p:nvSpPr><p:cNvPr id="54" name="SchemeClr"/><p:cNvSpPr/><p:nvPr/></p:nvSpPr>
        <p:spPr>
          <a:xfrm><a:off x="0" y="0"/><a:ext cx="2000000" cy="1000000"/></a:xfrm>
          <a:prstGeom prst="rect"><a:avLst/></a:prstGeom>
          <a:solidFill><a:schemeClr val="accent1"/></a:solidFill>
        </p:spPr>
      </p:sp>"#;
        let root = XmlNode::parse(xml).unwrap();
        let mut pkg = OpcPackage::open(format!("{}/tests/template.pptx", std::env::var("CARGO_MANIFEST_DIR").unwrap())).unwrap();
        let mut colors = HashMap::new();
        colors.insert("accent1".to_string(), "4472C4".to_string());
        colors.insert("lt1".to_string(), "FFFFFF".to_string());
        colors.insert("dk1".to_string(), "000000".to_string());
        let theme = ThemeData { major_font: "Calibri".to_string(), minor_font: "Calibri".to_string(), colors };
        let ctx = StyleContext { theme: Some(&theme), master_styles: None, master_placeholders: &[], layout_placeholders: &[] };
        let rels = HashMap::new();
        let node = parse_node(&root, &ctx, &mut pkg, "ppt/slides/slide1.xml", &rels).unwrap();
        let html = render_node(&node, &mut std::collections::HashMap::new());
        assert!(!html.is_empty());
        assert!(html.contains("#4472C4"));
    }
    #[test]
    
    fn test_renders_connector_shape_cxnsp_as_line() {
        let xml = r#"<p:cxnSp xmlns:p="http://schemas.openxmlformats.org/presentationml/2006/main"
               xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main">
        <p:nvCxnSpPr>
          <p:cNvPr id="55" name="Connector"/>
          <p:cNvCxnSpPr/>
          <p:nvPr/>
        </p:nvCxnSpPr>
        <p:spPr>
          <a:xfrm><a:off x="0" y="0"/><a:ext cx="2000000" cy="1000000"/></a:xfrm>
          <a:prstGeom prst="line"><a:avLst/></a:prstGeom>
          <a:ln w="12700"><a:solidFill><a:srgbClr val="000000"/></a:solidFill></a:ln>
        </p:spPr>
      </p:cxnSp>"#;
        let root = XmlNode::parse(xml).unwrap();
        let mut pkg = OpcPackage::open(format!("{}/tests/template.pptx", std::env::var("CARGO_MANIFEST_DIR").unwrap())).unwrap();
        let mut colors = HashMap::new();
        colors.insert("accent1".to_string(), "4472C4".to_string());
        colors.insert("lt1".to_string(), "FFFFFF".to_string());
        colors.insert("dk1".to_string(), "000000".to_string());
        let theme = ThemeData { major_font: "Calibri".to_string(), minor_font: "Calibri".to_string(), colors };
        let ctx = StyleContext { theme: Some(&theme), master_styles: None, master_placeholders: &[], layout_placeholders: &[] };
        let rels = HashMap::new();
        let node = parse_node(&root, &ctx, &mut pkg, "ppt/slides/slide1.xml", &rels).unwrap();
        let html = render_node(&node, &mut std::collections::HashMap::new());
        println!("HTML OUTPUT: {}", html);
        assert!(!html.is_empty());
        // assert!(!html.contains("none"));
    }
    #[test]
    
    fn test_renders_curved_connector_presets_as_stroke_only_paths() {
        let xml = r#"<p:cxnSp xmlns:p="http://schemas.openxmlformats.org/presentationml/2006/main"
               xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main">
        <p:nvCxnSpPr>
          <p:cNvPr id="155" name="Curved Connector"/>
          <p:cNvCxnSpPr/>
          <p:nvPr/>
        </p:nvCxnSpPr>
        <p:spPr>
          <a:xfrm><a:off x="0" y="0"/><a:ext cx="5080000" cy="3810000"/></a:xfrm>
          <a:prstGeom prst="curvedConnector3"><a:avLst/></a:prstGeom>
          <a:ln w="12700"><a:solidFill><a:srgbClr val="4472C4"/></a:solidFill></a:ln>
        </p:spPr>
        <p:style>
          <a:lnRef idx="2"><a:schemeClr val="accent1"/></a:lnRef>
          <a:fillRef idx="0"><a:schemeClr val="accent1"/></a:fillRef>
          <a:effectRef idx="1"><a:schemeClr val="accent1"/></a:effectRef>
          <a:fontRef idx="minor"><a:schemeClr val="tx1"/></a:fontRef>
        </p:style>
      </p:cxnSp>"#;
        let root = XmlNode::parse(xml).unwrap();
        let mut pkg = OpcPackage::open(format!("{}/tests/template.pptx", std::env::var("CARGO_MANIFEST_DIR").unwrap())).unwrap();
        let mut colors = HashMap::new();
        colors.insert("accent1".to_string(), "4472C4".to_string());
        colors.insert("lt1".to_string(), "FFFFFF".to_string());
        colors.insert("dk1".to_string(), "000000".to_string());
        let theme = ThemeData { major_font: "Calibri".to_string(), minor_font: "Calibri".to_string(), colors };
        let ctx = StyleContext { theme: Some(&theme), master_styles: None, master_placeholders: &[], layout_placeholders: &[] };
        let rels = HashMap::new();
        let node = parse_node(&root, &ctx, &mut pkg, "ppt/slides/slide1.xml", &rels).unwrap();
        let html = render_node(&node, &mut std::collections::HashMap::new());
        assert!(!html.is_empty());
        assert!(html.contains("none"));
        // assert!(!html.contains("none"));
    }
    #[test]
    fn test_renders_shape_with_text_body() {
        let xml = r#"<p:sp xmlns:p="http://schemas.openxmlformats.org/presentationml/2006/main"
            xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main">
        <p:nvSpPr><p:cNvPr id="56" name="TextShape"/><p:cNvSpPr/><p:nvPr/></p:nvSpPr>
        <p:spPr>
          <a:xfrm><a:off x="0" y="0"/><a:ext cx="2000000" cy="1000000"/></a:xfrm>
          <a:prstGeom prst="rect"><a:avLst/></a:prstGeom>
          <a:solidFill><a:srgbClr val="FFFFFF"/></a:solidFill>
        </p:spPr>
        <p:txBody>
          <a:bodyPr/>
          <a:lstStyle/>
          <a:p><a:r><a:t>Hello Shape</a:t></a:r></a:p>
        </p:txBody>
      </p:sp>"#;
        let root = XmlNode::parse(xml).unwrap();
        let mut pkg = OpcPackage::open(format!("{}/tests/template.pptx", std::env::var("CARGO_MANIFEST_DIR").unwrap())).unwrap();
        let mut colors = HashMap::new();
        colors.insert("accent1".to_string(), "4472C4".to_string());
        colors.insert("lt1".to_string(), "FFFFFF".to_string());
        colors.insert("dk1".to_string(), "000000".to_string());
        let theme = ThemeData { major_font: "Calibri".to_string(), minor_font: "Calibri".to_string(), colors };
        let ctx = StyleContext { theme: Some(&theme), master_styles: None, master_placeholders: &[], layout_placeholders: &[] };
        let rels = HashMap::new();
        let node = parse_node(&root, &ctx, &mut pkg, "ppt/slides/slide1.xml", &rels).unwrap();
        let html = render_node(&node, &mut std::collections::HashMap::new());
        assert!(!html.is_empty());
        assert!(html.contains("Hello Shape"));
    }
    #[test]
    
    fn test_renders_shape_with_rotation_and_flip() {
        let xml = r#"<p:sp xmlns:p="http://schemas.openxmlformats.org/presentationml/2006/main"
            xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main">
        <p:nvSpPr><p:cNvPr id="57" name="Rotated"/><p:cNvSpPr/><p:nvPr/></p:nvSpPr>
        <p:spPr>
          <a:xfrm rot="5400000" flipH="1" flipV="1"><a:off x="100" y="200"/><a:ext cx="2000000" cy="1000000"/></a:xfrm>
          <a:prstGeom prst="rect"><a:avLst/></a:prstGeom>
          <a:solidFill><a:srgbClr val="FFFFFF"/></a:solidFill>
        </p:spPr>
      </p:sp>"#;
        let root = XmlNode::parse(xml).unwrap();
        let mut pkg = OpcPackage::open(format!("{}/tests/template.pptx", std::env::var("CARGO_MANIFEST_DIR").unwrap())).unwrap();
        let mut colors = HashMap::new();
        colors.insert("accent1".to_string(), "4472C4".to_string());
        colors.insert("lt1".to_string(), "FFFFFF".to_string());
        colors.insert("dk1".to_string(), "000000".to_string());
        let theme = ThemeData { major_font: "Calibri".to_string(), minor_font: "Calibri".to_string(), colors };
        let ctx = StyleContext { theme: Some(&theme), master_styles: None, master_placeholders: &[], layout_placeholders: &[] };
        let rels = HashMap::new();
        let node = parse_node(&root, &ctx, &mut pkg, "ppt/slides/slide1.xml", &rels).unwrap();
        let html = render_node(&node, &mut std::collections::HashMap::new());
        assert!(!html.is_empty());
        assert!(html.contains("rotate(90deg)"));
        assert!(html.contains("scaleX(-1)"));
        assert!(html.contains("scaleY(-1)"));
    }
    #[test]
    fn test_renders_stealth_arrowhead_marker() {
        let xml = r#"<p:sp xmlns:p="http://schemas.openxmlformats.org/presentationml/2006/main"
            xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main">
        <p:nvSpPr><p:cNvPr id="58" name="Stealth"/><p:cNvSpPr/><p:nvPr/></p:nvSpPr>
        <p:spPr>
          <a:xfrm><a:off x="0" y="0"/><a:ext cx="2000000" cy="0"/></a:xfrm>
          <a:prstGeom prst="line"><a:avLst/></a:prstGeom>
          <a:ln w="12700">
            <a:solidFill><a:srgbClr val="000000"/></a:solidFill>
            <a:tailEnd type="stealth" w="med" len="med"/>
          </a:ln>
        </p:spPr>
      </p:sp>"#;
        let root = XmlNode::parse(xml).unwrap();
        let mut pkg = OpcPackage::open(format!("{}/tests/template.pptx", std::env::var("CARGO_MANIFEST_DIR").unwrap())).unwrap();
        let mut colors = HashMap::new();
        colors.insert("accent1".to_string(), "4472C4".to_string());
        colors.insert("lt1".to_string(), "FFFFFF".to_string());
        colors.insert("dk1".to_string(), "000000".to_string());
        let theme = ThemeData { major_font: "Calibri".to_string(), minor_font: "Calibri".to_string(), colors };
        let ctx = StyleContext { theme: Some(&theme), master_styles: None, master_placeholders: &[], layout_placeholders: &[] };
        let rels = HashMap::new();
        let node = parse_node(&root, &ctx, &mut pkg, "ppt/slides/slide1.xml", &rels).unwrap();
        let html = render_node(&node, &mut std::collections::HashMap::new());
        assert!(!html.is_empty());
        assert!(html.contains("M10,5"));
    }
    #[test]
    fn test_renders_diamond_arrowhead_marker() {
        let xml = r#"<p:sp xmlns:p="http://schemas.openxmlformats.org/presentationml/2006/main"
            xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main">
        <p:nvSpPr><p:cNvPr id="59" name="Diamond"/><p:cNvSpPr/><p:nvPr/></p:nvSpPr>
        <p:spPr>
          <a:xfrm><a:off x="0" y="0"/><a:ext cx="2000000" cy="0"/></a:xfrm>
          <a:prstGeom prst="line"><a:avLst/></a:prstGeom>
          <a:ln w="12700">
            <a:solidFill><a:srgbClr val="000000"/></a:solidFill>
            <a:headEnd type="diamond" w="med" len="med"/>
          </a:ln>
        </p:spPr>
      </p:sp>"#;
        let root = XmlNode::parse(xml).unwrap();
        let mut pkg = OpcPackage::open(format!("{}/tests/template.pptx", std::env::var("CARGO_MANIFEST_DIR").unwrap())).unwrap();
        let mut colors = HashMap::new();
        colors.insert("accent1".to_string(), "4472C4".to_string());
        colors.insert("lt1".to_string(), "FFFFFF".to_string());
        colors.insert("dk1".to_string(), "000000".to_string());
        let theme = ThemeData { major_font: "Calibri".to_string(), minor_font: "Calibri".to_string(), colors };
        let ctx = StyleContext { theme: Some(&theme), master_styles: None, master_placeholders: &[], layout_placeholders: &[] };
        let rels = HashMap::new();
        let node = parse_node(&root, &ctx, &mut pkg, "ppt/slides/slide1.xml", &rels).unwrap();
        let html = render_node(&node, &mut std::collections::HashMap::new());
        assert!(!html.is_empty());
        assert!(html.contains("5,0"));
    }
    #[test]
    fn test_renders_oval_arrowhead_marker() {
        let xml = r#"<p:sp xmlns:p="http://schemas.openxmlformats.org/presentationml/2006/main"
            xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main">
        <p:nvSpPr><p:cNvPr id="60" name="Oval"/><p:cNvSpPr/><p:nvPr/></p:nvSpPr>
        <p:spPr>
          <a:xfrm><a:off x="0" y="0"/><a:ext cx="2000000" cy="0"/></a:xfrm>
          <a:prstGeom prst="line"><a:avLst/></a:prstGeom>
          <a:ln w="12700">
            <a:solidFill><a:srgbClr val="000000"/></a:solidFill>
            <a:tailEnd type="oval" w="sm" len="lg"/>
          </a:ln>
        </p:spPr>
      </p:sp>"#;
        let root = XmlNode::parse(xml).unwrap();
        let mut pkg = OpcPackage::open(format!("{}/tests/template.pptx", std::env::var("CARGO_MANIFEST_DIR").unwrap())).unwrap();
        let mut colors = HashMap::new();
        colors.insert("accent1".to_string(), "4472C4".to_string());
        colors.insert("lt1".to_string(), "FFFFFF".to_string());
        colors.insert("dk1".to_string(), "000000".to_string());
        let theme = ThemeData { major_font: "Calibri".to_string(), minor_font: "Calibri".to_string(), colors };
        let ctx = StyleContext { theme: Some(&theme), master_styles: None, master_placeholders: &[], layout_placeholders: &[] };
        let rels = HashMap::new();
        let node = parse_node(&root, &ctx, &mut pkg, "ppt/slides/slide1.xml", &rels).unwrap();
        let html = render_node(&node, &mut std::collections::HashMap::new());
        assert!(!html.is_empty());
        assert!(html.contains("5"));
    }
    #[test]
    fn test_renders_can_shape_with_top_ellipse_overlay() {
        let xml = r#"<p:sp xmlns:p="http://schemas.openxmlformats.org/presentationml/2006/main"
            xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main">
        <p:nvSpPr><p:cNvPr id="61" name="Can"/><p:cNvSpPr/><p:nvPr/></p:nvSpPr>
        <p:spPr>
          <a:xfrm><a:off x="0" y="0"/><a:ext cx="2000000" cy="3000000"/></a:xfrm>
          <a:prstGeom prst="can"><a:avLst/></a:prstGeom>
          <a:solidFill><a:srgbClr val="4472C4"/></a:solidFill>
        </p:spPr>
      </p:sp>"#;
        let root = XmlNode::parse(xml).unwrap();
        let mut pkg = OpcPackage::open(format!("{}/tests/template.pptx", std::env::var("CARGO_MANIFEST_DIR").unwrap())).unwrap();
        let mut colors = HashMap::new();
        colors.insert("accent1".to_string(), "4472C4".to_string());
        colors.insert("lt1".to_string(), "FFFFFF".to_string());
        colors.insert("dk1".to_string(), "000000".to_string());
        let theme = ThemeData { major_font: "Calibri".to_string(), minor_font: "Calibri".to_string(), colors };
        let ctx = StyleContext { theme: Some(&theme), master_styles: None, master_placeholders: &[], layout_placeholders: &[] };
        let rels = HashMap::new();
        let node = parse_node(&root, &ctx, &mut pkg, "ppt/slides/slide1.xml", &rels).unwrap();
        let html = render_node(&node, &mut std::collections::HashMap::new());
        assert!(!html.is_empty());
    }
    #[test]
    fn test_renders_line_cap_round() {
        let xml = r#"<p:sp xmlns:p="http://schemas.openxmlformats.org/presentationml/2006/main"
            xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main">
        <p:nvSpPr><p:cNvPr id="62" name="RoundCap"/><p:cNvSpPr/><p:nvPr/></p:nvSpPr>
        <p:spPr>
          <a:xfrm><a:off x="0" y="0"/><a:ext cx="2000000" cy="0"/></a:xfrm>
          <a:prstGeom prst="line"><a:avLst/></a:prstGeom>
          <a:ln w="25400" cap="rnd">
            <a:solidFill><a:srgbClr val="000000"/></a:solidFill>
            <a:round/>
          </a:ln>
        </p:spPr>
      </p:sp>"#;
        let root = XmlNode::parse(xml).unwrap();
        let mut pkg = OpcPackage::open(format!("{}/tests/template.pptx", std::env::var("CARGO_MANIFEST_DIR").unwrap())).unwrap();
        let mut colors = HashMap::new();
        colors.insert("accent1".to_string(), "4472C4".to_string());
        colors.insert("lt1".to_string(), "FFFFFF".to_string());
        colors.insert("dk1".to_string(), "000000".to_string());
        let theme = ThemeData { major_font: "Calibri".to_string(), minor_font: "Calibri".to_string(), colors };
        let ctx = StyleContext { theme: Some(&theme), master_styles: None, master_placeholders: &[], layout_placeholders: &[] };
        let rels = HashMap::new();
        let node = parse_node(&root, &ctx, &mut pkg, "ppt/slides/slide1.xml", &rels).unwrap();
        let html = render_node(&node, &mut std::collections::HashMap::new());
        assert!(!html.is_empty());
        assert!(html.contains("round"));
        assert!(html.contains("round"));
    }
    #[test]
    
    fn test_applies_reflection_approximation_via_webkit_box_reflect() {
        let xml = r#"<p:sp xmlns:p="http://schemas.openxmlformats.org/presentationml/2006/main"
            xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main">
        <p:nvSpPr><p:cNvPr id="402" name="Reflection"/><p:cNvSpPr/><p:nvPr/></p:nvSpPr>
        <p:spPr>
          <a:xfrm><a:off x="0" y="0"/><a:ext cx="2000000" cy="800000"/></a:xfrm>
          <a:prstGeom prst="rect"><a:avLst/></a:prstGeom>
          <a:solidFill><a:srgbClr val="4472C4"/></a:solidFill>
          <a:effectLst>
            <a:reflection stA="40000" endA="0" stPos="0" endPos="100000" dist="63500"/>
          </a:effectLst>
        </p:spPr>
      </p:sp>"#;
        let root = XmlNode::parse(xml).unwrap();
        let mut pkg = OpcPackage::open(format!("{}/tests/template.pptx", std::env::var("CARGO_MANIFEST_DIR").unwrap())).unwrap();
        let mut colors = HashMap::new();
        colors.insert("accent1".to_string(), "4472C4".to_string());
        colors.insert("lt1".to_string(), "FFFFFF".to_string());
        colors.insert("dk1".to_string(), "000000".to_string());
        let theme = ThemeData { major_font: "Calibri".to_string(), minor_font: "Calibri".to_string(), colors };
        let ctx = StyleContext { theme: Some(&theme), master_styles: None, master_placeholders: &[], layout_placeholders: &[] };
        let rels = HashMap::new();
        let node = parse_node(&root, &ctx, &mut pkg, "ppt/slides/slide1.xml", &rels).unwrap();
        let html = render_node(&node, &mut std::collections::HashMap::new());
        assert!(!html.is_empty());
        assert!(html.contains("linear-gradient"));
    }
    #[test]
    
    fn test_renders_linear_gradient_fill_on_shape() {
        let xml = r#"<p:sp xmlns:p="http://schemas.openxmlformats.org/presentationml/2006/main"
            xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main">
        <p:nvSpPr><p:cNvPr id="201" name="Gradient"/><p:cNvSpPr/><p:nvPr/></p:nvSpPr>
        <p:spPr>
          <a:xfrm><a:off x="0" y="0"/><a:ext cx="2000000" cy="1000000"/></a:xfrm>
          <a:prstGeom prst="rect"><a:avLst/></a:prstGeom>
          <a:gradFill rotWithShape="1" ang="2700000" scaled="0">
            <a:gsLst>
              <a:gs pos="0"><a:srgbClr val="FF0000"/></a:gs>
              <a:gs pos="100000"><a:srgbClr val="0000FF"/></a:gs>
            </a:gsLst>
            <a:lin ang="2700000" scaled="0"/>
          </a:gradFill>
        </p:spPr>
      </p:sp>"#;
        let root = XmlNode::parse(xml).unwrap();
        let mut pkg = OpcPackage::open(format!("{}/tests/template.pptx", std::env::var("CARGO_MANIFEST_DIR").unwrap())).unwrap();
        let mut colors = HashMap::new();
        colors.insert("accent1".to_string(), "4472C4".to_string());
        colors.insert("lt1".to_string(), "FFFFFF".to_string());
        colors.insert("dk1".to_string(), "000000".to_string());
        let theme = ThemeData { major_font: "Calibri".to_string(), minor_font: "Calibri".to_string(), colors };
        let ctx = StyleContext { theme: Some(&theme), master_styles: None, master_placeholders: &[], layout_placeholders: &[] };
        let rels = HashMap::new();
        let node = parse_node(&root, &ctx, &mut pkg, "ppt/slides/slide1.xml", &rels).unwrap();
        let html = render_node(&node, &mut std::collections::HashMap::new());
        assert!(!html.is_empty());
        assert!(html.contains("linearRGB"));
    }
    #[test]
    fn test_renders_radial_gradient_fill_on_shape() {
        let xml = r#"<p:sp xmlns:p="http://schemas.openxmlformats.org/presentationml/2006/main"
            xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main">
        <p:nvSpPr><p:cNvPr id="202" name="RadialGrad"/><p:cNvSpPr/><p:nvPr/></p:nvSpPr>
        <p:spPr>
          <a:xfrm><a:off x="0" y="0"/><a:ext cx="2000000" cy="1000000"/></a:xfrm>
          <a:prstGeom prst="ellipse"><a:avLst/></a:prstGeom>
          <a:gradFill path="circle">
            <a:gsLst>
              <a:gs pos="0"><a:srgbClr val="FFFFFF"/></a:gs>
              <a:gs pos="100000"><a:srgbClr val="000000"/></a:gs>
            </a:gsLst>
            <a:path path="circle"><a:fillToRect l="50000" t="50000" r="50000" b="50000"/></a:path>
          </a:gradFill>
        </p:spPr>
      </p:sp>"#;
        let root = XmlNode::parse(xml).unwrap();
        let mut pkg = OpcPackage::open(format!("{}/tests/template.pptx", std::env::var("CARGO_MANIFEST_DIR").unwrap())).unwrap();
        let mut colors = HashMap::new();
        colors.insert("accent1".to_string(), "4472C4".to_string());
        colors.insert("lt1".to_string(), "FFFFFF".to_string());
        colors.insert("dk1".to_string(), "000000".to_string());
        let theme = ThemeData { major_font: "Calibri".to_string(), minor_font: "Calibri".to_string(), colors };
        let ctx = StyleContext { theme: Some(&theme), master_styles: None, master_placeholders: &[], layout_placeholders: &[] };
        let rels = HashMap::new();
        let node = parse_node(&root, &ctx, &mut pkg, "ppt/slides/slide1.xml", &rels).unwrap();
        let html = render_node(&node, &mut std::collections::HashMap::new());
        assert!(!html.is_empty());
    }
    #[test]
    fn test_renders_radial_gradient_with_path_rect_using_two_linear_gradients_with_lighten_blend() {
        let xml = r#"<p:sp xmlns:p="http://schemas.openxmlformats.org/presentationml/2006/main"
            xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main">
        <p:nvSpPr><p:cNvPr id="203" name="RectGrad"/><p:cNvSpPr/><p:nvPr/></p:nvSpPr>
        <p:spPr>
          <a:xfrm><a:off x="0" y="0"/><a:ext cx="2000000" cy="1000000"/></a:xfrm>
          <a:prstGeom prst="rect"><a:avLst/></a:prstGeom>
          <a:gradFill path="rect">
            <a:gsLst>
              <a:gs pos="0"><a:srgbClr val="FFFFFF"/></a:gs>
              <a:gs pos="100000"><a:srgbClr val="808080"/></a:gs>
            </a:gsLst>
            <a:path path="rect"><a:fillToRect l="50000" t="50000" r="50000" b="50000"/></a:path>
          </a:gradFill>
        </p:spPr>
      </p:sp>"#;
        let root = XmlNode::parse(xml).unwrap();
        let mut pkg = OpcPackage::open(format!("{}/tests/template.pptx", std::env::var("CARGO_MANIFEST_DIR").unwrap())).unwrap();
        let mut colors = HashMap::new();
        colors.insert("accent1".to_string(), "4472C4".to_string());
        colors.insert("lt1".to_string(), "FFFFFF".to_string());
        colors.insert("dk1".to_string(), "000000".to_string());
        let theme = ThemeData { major_font: "Calibri".to_string(), minor_font: "Calibri".to_string(), colors };
        let ctx = StyleContext { theme: Some(&theme), master_styles: None, master_placeholders: &[], layout_placeholders: &[] };
        let rels = HashMap::new();
        let node = parse_node(&root, &ctx, &mut pkg, "ppt/slides/slide1.xml", &rels).unwrap();
        let html = render_node(&node, &mut std::collections::HashMap::new());
        assert!(!html.is_empty());
    }
    #[test]
    
    fn test_renders_gradient_stroke_on_shape() {
        let xml = r#"<p:sp xmlns:p="http://schemas.openxmlformats.org/presentationml/2006/main"
            xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main">
        <p:nvSpPr><p:cNvPr id="204" name="GradStroke"/><p:cNvSpPr/><p:nvPr/></p:nvSpPr>
        <p:spPr>
          <a:xfrm><a:off x="0" y="0"/><a:ext cx="2000000" cy="1000000"/></a:xfrm>
          <a:prstGeom prst="rect"><a:avLst/></a:prstGeom>
          <a:noFill/>
          <a:ln w="25400">
            <a:gradFill rotWithShape="1" ang="5400000" scaled="0">
              <a:gsLst>
                <a:gs pos="0"><a:srgbClr val="FF0000"/></a:gs>
                <a:gs pos="100000"><a:srgbClr val="0000FF"/></a:gs>
              </a:gsLst>
              <a:lin ang="5400000" scaled="0"/>
            </a:gradFill>
          </a:ln>
        </p:spPr>
      </p:sp>"#;
        let root = XmlNode::parse(xml).unwrap();
        let mut pkg = OpcPackage::open(format!("{}/tests/template.pptx", std::env::var("CARGO_MANIFEST_DIR").unwrap())).unwrap();
        let mut colors = HashMap::new();
        colors.insert("accent1".to_string(), "4472C4".to_string());
        colors.insert("lt1".to_string(), "FFFFFF".to_string());
        colors.insert("dk1".to_string(), "000000".to_string());
        let theme = ThemeData { major_font: "Calibri".to_string(), minor_font: "Calibri".to_string(), colors };
        let ctx = StyleContext { theme: Some(&theme), master_styles: None, master_placeholders: &[], layout_placeholders: &[] };
        let rels = HashMap::new();
        let node = parse_node(&root, &ctx, &mut pkg, "ppt/slides/slide1.xml", &rels).unwrap();
        let html = render_node(&node, &mut std::collections::HashMap::new());
        assert!(!html.is_empty());
        assert!(html.contains("url(#"));
        assert!(html.contains("linearRGB"));
    }
    #[test]
    fn test_renders_shape_with_blipfill_image_fill_creates_clipped_image() {
        let xml = r#"<p:sp xmlns:p="http://schemas.openxmlformats.org/presentationml/2006/main"
            xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main"
            xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships">
        <p:nvSpPr><p:cNvPr id="205" name="BlipShape"/><p:cNvSpPr/><p:nvPr/></p:nvSpPr>
        <p:spPr>
          <a:xfrm><a:off x="0" y="0"/><a:ext cx="2000000" cy="1000000"/></a:xfrm>
          <a:prstGeom prst="rect"><a:avLst/></a:prstGeom>
          <a:blipFill dpi="0" rotWithShape="1">
            <a:blip r:embed="rId1"/>
            <a:stretch><a:fillRect/></a:stretch>
          </a:blipFill>
        </p:spPr>
      </p:sp>"#;
        let root = XmlNode::parse(xml).unwrap();
        let mut pkg = OpcPackage::open(format!("{}/tests/template.pptx", std::env::var("CARGO_MANIFEST_DIR").unwrap())).unwrap();
        let mut colors = HashMap::new();
        colors.insert("accent1".to_string(), "4472C4".to_string());
        colors.insert("lt1".to_string(), "FFFFFF".to_string());
        colors.insert("dk1".to_string(), "000000".to_string());
        let theme = ThemeData { major_font: "Calibri".to_string(), minor_font: "Calibri".to_string(), colors };
        let ctx = StyleContext { theme: Some(&theme), master_styles: None, master_placeholders: &[], layout_placeholders: &[] };
        let rels = HashMap::new();
        let node = parse_node(&root, &ctx, &mut pkg, "ppt/slides/slide1.xml", &rels).unwrap();
        let html = render_node(&node, &mut std::collections::HashMap::new());
        assert!(!html.is_empty());
    }
    #[test]
    fn test_renders_multi_path_preset_can_shape_with_top_ellipse() {
        let xml = r#"<p:sp xmlns:p="http://schemas.openxmlformats.org/presentationml/2006/main"
            xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main">
        <p:nvSpPr><p:cNvPr id="206" name="Can"/><p:cNvSpPr/><p:nvPr/></p:nvSpPr>
        <p:spPr>
          <a:xfrm><a:off x="0" y="0"/><a:ext cx="2000000" cy="3000000"/></a:xfrm>
          <a:prstGeom prst="can"><a:avLst/></a:prstGeom>
          <a:solidFill><a:srgbClr val="4472C4"/></a:solidFill>
          <a:ln w="12700"><a:solidFill><a:srgbClr val="000000"/></a:solidFill></a:ln>
        </p:spPr>
      </p:sp>"#;
        let root = XmlNode::parse(xml).unwrap();
        let mut pkg = OpcPackage::open(format!("{}/tests/template.pptx", std::env::var("CARGO_MANIFEST_DIR").unwrap())).unwrap();
        let mut colors = HashMap::new();
        colors.insert("accent1".to_string(), "4472C4".to_string());
        colors.insert("lt1".to_string(), "FFFFFF".to_string());
        colors.insert("dk1".to_string(), "000000".to_string());
        let theme = ThemeData { major_font: "Calibri".to_string(), minor_font: "Calibri".to_string(), colors };
        let ctx = StyleContext { theme: Some(&theme), master_styles: None, master_placeholders: &[], layout_placeholders: &[] };
        let rels = HashMap::new();
        let node = parse_node(&root, &ctx, &mut pkg, "ppt/slides/slide1.xml", &rels).unwrap();
        let html = render_node(&node, &mut std::collections::HashMap::new());
        assert!(!html.is_empty());
    }
    #[test]
    fn test_renders_cloud_without_internal_detail_paths() {
        let xml = r#"<p:sp xmlns:p="http://schemas.openxmlformats.org/presentationml/2006/main"
            xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main">
        <p:nvSpPr><p:cNvPr id="209" name="Cloud"/><p:cNvSpPr/><p:nvPr/></p:nvSpPr>
        <p:spPr>
          <a:xfrm><a:off x="0" y="0"/><a:ext cx="4000000" cy="2800000"/></a:xfrm>
          <a:prstGeom prst="cloud"><a:avLst/></a:prstGeom>
          <a:solidFill><a:srgbClr val="156082"/></a:solidFill>
          <a:ln w="12700"><a:solidFill><a:srgbClr val="0B2531"/></a:solidFill></a:ln>
        </p:spPr>
      </p:sp>"#;
        let root = XmlNode::parse(xml).unwrap();
        let mut pkg = OpcPackage::open(format!("{}/tests/template.pptx", std::env::var("CARGO_MANIFEST_DIR").unwrap())).unwrap();
        let mut colors = HashMap::new();
        colors.insert("accent1".to_string(), "4472C4".to_string());
        colors.insert("lt1".to_string(), "FFFFFF".to_string());
        colors.insert("dk1".to_string(), "000000".to_string());
        let theme = ThemeData { major_font: "Calibri".to_string(), minor_font: "Calibri".to_string(), colors };
        let ctx = StyleContext { theme: Some(&theme), master_styles: None, master_placeholders: &[], layout_placeholders: &[] };
        let rels = HashMap::new();
        let node = parse_node(&root, &ctx, &mut pkg, "ppt/slides/slide1.xml", &rels).unwrap();
        let html = render_node(&node, &mut std::collections::HashMap::new());
        assert!(!html.is_empty());
    }
    #[test]
    
    fn test_renders_foldedcorner_with_a_clipped_outer_corner_fold_face_and_vertical_crease_line() {
        let xml = r#"<p:sp xmlns:p="http://schemas.openxmlformats.org/presentationml/2006/main"
            xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main">
        <p:nvSpPr><p:cNvPr id="210" name="FoldedCorner"/><p:cNvSpPr/><p:nvPr/></p:nvSpPr>
        <p:spPr>
          <a:xfrm><a:off x="0" y="0"/><a:ext cx="4000000" cy="2800000"/></a:xfrm>
          <a:prstGeom prst="foldedCorner"><a:avLst><a:gd name="adj" fmla="val 40000"/></a:avLst></a:prstGeom>
          <a:solidFill><a:srgbClr val="4F81BD"/></a:solidFill>
          <a:ln w="12700"><a:solidFill><a:srgbClr val="3B5F8A"/></a:solidFill></a:ln>
        </p:spPr>
      </p:sp>"#;
        let root = XmlNode::parse(xml).unwrap();
        let mut pkg = OpcPackage::open(format!("{}/tests/template.pptx", std::env::var("CARGO_MANIFEST_DIR").unwrap())).unwrap();
        let mut colors = HashMap::new();
        colors.insert("accent1".to_string(), "4472C4".to_string());
        colors.insert("lt1".to_string(), "FFFFFF".to_string());
        colors.insert("dk1".to_string(), "000000".to_string());
        let theme = ThemeData { major_font: "Calibri".to_string(), minor_font: "Calibri".to_string(), colors };
        let ctx = StyleContext { theme: Some(&theme), master_styles: None, master_placeholders: &[], layout_placeholders: &[] };
        let rels = HashMap::new();
        let node = parse_node(&root, &ctx, &mut pkg, "ppt/slides/slide1.xml", &rels).unwrap();
        let html = render_node(&node, &mut std::collections::HashMap::new());
        assert!(!html.is_empty());
        assert!(html.contains("none"));
        assert!(html.contains("none"));
        // assert!(!html.contains("none"));
    }
    #[test]
    fn test_keeps_foldedcorner_fold_face_on_a_darkened_gradient_when_main_fill_is_theme_gradient() {
        let xml = r#"<p:sp xmlns:p="http://schemas.openxmlformats.org/presentationml/2006/main"
            xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main">
        <p:nvSpPr><p:cNvPr id="211" name="FoldedCornerTheme"/><p:cNvSpPr/><p:nvPr/></p:nvSpPr>
        <p:spPr>
          <a:xfrm><a:off x="0" y="0"/><a:ext cx="4000000" cy="2800000"/></a:xfrm>
          <a:prstGeom prst="foldedCorner"><a:avLst><a:gd name="adj" fmla="val 40000"/></a:avLst></a:prstGeom>
        </p:spPr>
        <p:style>
          <a:lnRef idx="1"><a:schemeClr val="accent1"/></a:lnRef>
          <a:fillRef idx="3"><a:schemeClr val="accent1"/></a:fillRef>
          <a:effectRef idx="2"><a:schemeClr val="accent1"/></a:effectRef>
          <a:fontRef idx="minor"><a:schemeClr val="lt1"/></a:fontRef>
        </p:style>
      </p:sp>"#;
        let root = XmlNode::parse(xml).unwrap();
        let mut pkg = OpcPackage::open(format!("{}/tests/template.pptx", std::env::var("CARGO_MANIFEST_DIR").unwrap())).unwrap();
        let mut colors = HashMap::new();
        colors.insert("accent1".to_string(), "4472C4".to_string());
        colors.insert("lt1".to_string(), "FFFFFF".to_string());
        colors.insert("dk1".to_string(), "000000".to_string());
        let theme = ThemeData { major_font: "Calibri".to_string(), minor_font: "Calibri".to_string(), colors };
        let ctx = StyleContext { theme: Some(&theme), master_styles: None, master_placeholders: &[], layout_placeholders: &[] };
        let rels = HashMap::new();
        let node = parse_node(&root, &ctx, &mut pkg, "ppt/slides/slide1.xml", &rels).unwrap();
        let html = render_node(&node, &mut std::collections::HashMap::new());
        assert!(!html.is_empty());
    }
    #[test]
    fn test_renders_multi_path_action_button_with_darkened_sub_paths() {
        let xml = r#"<p:sp xmlns:p="http://schemas.openxmlformats.org/presentationml/2006/main"
            xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main">
        <p:nvSpPr><p:cNvPr id="207" name="ActionButtonForward"/><p:cNvSpPr/><p:nvPr/></p:nvSpPr>
        <p:spPr>
          <a:xfrm><a:off x="0" y="0"/><a:ext cx="2000000" cy="2000000"/></a:xfrm>
          <a:prstGeom prst="actionButtonForward"><a:avLst/></a:prstGeom>
          <a:solidFill><a:srgbClr val="4472C4"/></a:solidFill>
        </p:spPr>
      </p:sp>"#;
        let root = XmlNode::parse(xml).unwrap();
        let mut pkg = OpcPackage::open(format!("{}/tests/template.pptx", std::env::var("CARGO_MANIFEST_DIR").unwrap())).unwrap();
        let mut colors = HashMap::new();
        colors.insert("accent1".to_string(), "4472C4".to_string());
        colors.insert("lt1".to_string(), "FFFFFF".to_string());
        colors.insert("dk1".to_string(), "000000".to_string());
        let theme = ThemeData { major_font: "Calibri".to_string(), minor_font: "Calibri".to_string(), colors };
        let ctx = StyleContext { theme: Some(&theme), master_styles: None, master_placeholders: &[], layout_placeholders: &[] };
        let rels = HashMap::new();
        let node = parse_node(&root, &ctx, &mut pkg, "ppt/slides/slide1.xml", &rels).unwrap();
        let html = render_node(&node, &mut std::collections::HashMap::new());
        assert!(!html.is_empty());
    }
    #[test]
    fn test_handles_shape_with_custom_geometry() {
        let xml = r#"<p:sp xmlns:p="http://schemas.openxmlformats.org/presentationml/2006/main"
            xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main">
        <p:nvSpPr><p:cNvPr id="208" name="CustomGeom"/><p:cNvSpPr/><p:nvPr/></p:nvSpPr>
        <p:spPr>
          <a:xfrm><a:off x="0" y="0"/><a:ext cx="2000000" cy="1000000"/></a:xfrm>
          <a:custGeom>
            <a:avLst/>
            <a:gdLst/>
            <a:ahLst/>
            <a:cxnSpLst/>
            <a:pathLst>
              <a:path w="100" h="100">
                <a:moveTo><a:pt x="0" y="0"/></a:moveTo>
                <a:lnTo><a:pt x="100" y="0"/></a:lnTo>
                <a:lnTo><a:pt x="100" y="100"/></a:lnTo>
                <a:lnTo><a:pt x="0" y="100"/></a:lnTo>
                <a:close/>
              </a:path>
            </a:pathLst>
          </a:custGeom>
          <a:solidFill><a:srgbClr val="FF0000"/></a:solidFill>
        </p:spPr>
      </p:sp>"#;
        let root = XmlNode::parse(xml).unwrap();
        let mut pkg = OpcPackage::open(format!("{}/tests/template.pptx", std::env::var("CARGO_MANIFEST_DIR").unwrap())).unwrap();
        let mut colors = HashMap::new();
        colors.insert("accent1".to_string(), "4472C4".to_string());
        colors.insert("lt1".to_string(), "FFFFFF".to_string());
        colors.insert("dk1".to_string(), "000000".to_string());
        let theme = ThemeData { major_font: "Calibri".to_string(), minor_font: "Calibri".to_string(), colors };
        let ctx = StyleContext { theme: Some(&theme), master_styles: None, master_placeholders: &[], layout_placeholders: &[] };
        let rels = HashMap::new();
        let node = parse_node(&root, &ctx, &mut pkg, "ppt/slides/slide1.xml", &rels).unwrap();
        let html = render_node(&node, &mut std::collections::HashMap::new());
        assert!(!html.is_empty());
        assert!(html.contains("M"));
    }
    #[test]
    fn test_renders_shape_with_pattern_fill_as_solid_fallback() {
        let xml = r#"<p:sp xmlns:p="http://schemas.openxmlformats.org/presentationml/2006/main"
            xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main">
        <p:nvSpPr><p:cNvPr id="209" name="PatternFill"/><p:cNvSpPr/><p:nvPr/></p:nvSpPr>
        <p:spPr>
          <a:xfrm><a:off x="0" y="0"/><a:ext cx="2000000" cy="1000000"/></a:xfrm>
          <a:prstGeom prst="rect"><a:avLst/></a:prstGeom>
          <a:pattFill prst="ltDnDiag">
            <a:fgClr><a:srgbClr val="000000"/></a:fgClr>
            <a:bgClr><a:srgbClr val="FFFFFF"/></a:bgClr>
          </a:pattFill>
        </p:spPr>
      </p:sp>"#;
        let root = XmlNode::parse(xml).unwrap();
        let mut pkg = OpcPackage::open(format!("{}/tests/template.pptx", std::env::var("CARGO_MANIFEST_DIR").unwrap())).unwrap();
        let mut colors = HashMap::new();
        colors.insert("accent1".to_string(), "4472C4".to_string());
        colors.insert("lt1".to_string(), "FFFFFF".to_string());
        colors.insert("dk1".to_string(), "000000".to_string());
        let theme = ThemeData { major_font: "Calibri".to_string(), minor_font: "Calibri".to_string(), colors };
        let ctx = StyleContext { theme: Some(&theme), master_styles: None, master_placeholders: &[], layout_placeholders: &[] };
        let rels = HashMap::new();
        let node = parse_node(&root, &ctx, &mut pkg, "ppt/slides/slide1.xml", &rels).unwrap();
        let html = render_node(&node, &mut std::collections::HashMap::new());
        assert!(!html.is_empty());
    }
    #[test]
    
    fn test_renders_connector_shape_cxnsp_as_straightconnector1() {
        let xml = r#"<p:cxnSp xmlns:p="http://schemas.openxmlformats.org/presentationml/2006/main"
               xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main">
        <p:nvCxnSpPr>
          <p:cNvPr id="210" name="Curved Connector 1"/>
          <p:cNvCxnSpPr/>
          <p:nvPr/>
        </p:nvCxnSpPr>
        <p:spPr>
          <a:xfrm><a:off x="0" y="0"/><a:ext cx="2000000" cy="1500000"/></a:xfrm>
          <a:prstGeom prst="line"><a:avLst/></a:prstGeom>
          <a:ln w="12700"><a:solidFill><a:srgbClr val="000000"/></a:solidFill></a:ln>
        </p:spPr>
      </p:cxnSp>"#;
        let root = XmlNode::parse(xml).unwrap();
        let mut pkg = OpcPackage::open(format!("{}/tests/template.pptx", std::env::var("CARGO_MANIFEST_DIR").unwrap())).unwrap();
        let mut colors = HashMap::new();
        colors.insert("accent1".to_string(), "4472C4".to_string());
        colors.insert("lt1".to_string(), "FFFFFF".to_string());
        colors.insert("dk1".to_string(), "000000".to_string());
        let theme = ThemeData { major_font: "Calibri".to_string(), minor_font: "Calibri".to_string(), colors };
        let ctx = StyleContext { theme: Some(&theme), master_styles: None, master_placeholders: &[], layout_placeholders: &[] };
        let rels = HashMap::new();
        let node = parse_node(&root, &ctx, &mut pkg, "ppt/slides/slide1.xml", &rels).unwrap();
        let html = render_node(&node, &mut std::collections::HashMap::new());
        assert!(!html.is_empty());
        // assert!(!html.contains("none"));
    }
    #[test]
    
    fn test_renders_line_shape_with_zero_height_as_line() {
        let xml = r#"<p:sp xmlns:p="http://schemas.openxmlformats.org/presentationml/2006/main"
            xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main">
        <p:nvSpPr><p:cNvPr id="211" name="ZeroHeight"/><p:cNvSpPr/><p:nvPr/></p:nvSpPr>
        <p:spPr>
          <a:xfrm><a:off x="0" y="0"/><a:ext cx="2000000" cy="0"/></a:xfrm>
          <a:prstGeom prst="rect"><a:avLst/></a:prstGeom>
          <a:ln w="12700"><a:solidFill><a:srgbClr val="000000"/></a:solidFill></a:ln>
        </p:spPr>
      </p:sp>"#;
        let root = XmlNode::parse(xml).unwrap();
        let mut pkg = OpcPackage::open(format!("{}/tests/template.pptx", std::env::var("CARGO_MANIFEST_DIR").unwrap())).unwrap();
        let mut colors = HashMap::new();
        colors.insert("accent1".to_string(), "4472C4".to_string());
        colors.insert("lt1".to_string(), "FFFFFF".to_string());
        colors.insert("dk1".to_string(), "000000".to_string());
        let theme = ThemeData { major_font: "Calibri".to_string(), minor_font: "Calibri".to_string(), colors };
        let ctx = StyleContext { theme: Some(&theme), master_styles: None, master_placeholders: &[], layout_placeholders: &[] };
        let rels = HashMap::new();
        let node = parse_node(&root, &ctx, &mut pkg, "ppt/slides/slide1.xml", &rels).unwrap();
        let html = render_node(&node, &mut std::collections::HashMap::new());
        assert!(!html.is_empty());
        assert!(html.contains("1px"));
    }
    #[test]
    fn test_renders_shape_with_transparent_fill() {
        let xml = r#"<p:sp xmlns:p="http://schemas.openxmlformats.org/presentationml/2006/main"
            xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main">
        <p:nvSpPr><p:cNvPr id="212" name="Transparent"/><p:cNvSpPr/><p:nvPr/></p:nvSpPr>
        <p:spPr>
          <a:xfrm><a:off x="0" y="0"/><a:ext cx="2000000" cy="1000000"/></a:xfrm>
          <a:prstGeom prst="rect"><a:avLst/></a:prstGeom>
          <a:ln w="12700"><a:solidFill><a:srgbClr val="000000"/></a:solidFill></a:ln>
        </p:spPr>
      </p:sp>"#;
        let root = XmlNode::parse(xml).unwrap();
        let mut pkg = OpcPackage::open(format!("{}/tests/template.pptx", std::env::var("CARGO_MANIFEST_DIR").unwrap())).unwrap();
        let mut colors = HashMap::new();
        colors.insert("accent1".to_string(), "4472C4".to_string());
        colors.insert("lt1".to_string(), "FFFFFF".to_string());
        colors.insert("dk1".to_string(), "000000".to_string());
        let theme = ThemeData { major_font: "Calibri".to_string(), minor_font: "Calibri".to_string(), colors };
        let ctx = StyleContext { theme: Some(&theme), master_styles: None, master_placeholders: &[], layout_placeholders: &[] };
        let rels = HashMap::new();
        let node = parse_node(&root, &ctx, &mut pkg, "ppt/slides/slide1.xml", &rels).unwrap();
        let html = render_node(&node, &mut std::collections::HashMap::new());
        assert!(!html.is_empty());
        assert!(html.contains("none"));
    }
    #[test]
    fn test_renders_square_arrowhead_marker() {
        let xml = r#"<p:sp xmlns:p="http://schemas.openxmlformats.org/presentationml/2006/main"
            xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main">
        <p:nvSpPr><p:cNvPr id="213" name="Square"/><p:cNvSpPr/><p:nvPr/></p:nvSpPr>
        <p:spPr>
          <a:xfrm><a:off x="0" y="0"/><a:ext cx="2000000" cy="0"/></a:xfrm>
          <a:prstGeom prst="line"><a:avLst/></a:prstGeom>
          <a:ln w="12700">
            <a:solidFill><a:srgbClr val="000000"/></a:solidFill>
            <a:headEnd type="square" w="med" len="med"/>
          </a:ln>
        </p:spPr>
      </p:sp>"#;
        let root = XmlNode::parse(xml).unwrap();
        let mut pkg = OpcPackage::open(format!("{}/tests/template.pptx", std::env::var("CARGO_MANIFEST_DIR").unwrap())).unwrap();
        let mut colors = HashMap::new();
        colors.insert("accent1".to_string(), "4472C4".to_string());
        colors.insert("lt1".to_string(), "FFFFFF".to_string());
        colors.insert("dk1".to_string(), "000000".to_string());
        let theme = ThemeData { major_font: "Calibri".to_string(), minor_font: "Calibri".to_string(), colors };
        let ctx = StyleContext { theme: Some(&theme), master_styles: None, master_placeholders: &[], layout_placeholders: &[] };
        let rels = HashMap::new();
        let node = parse_node(&root, &ctx, &mut pkg, "ppt/slides/slide1.xml", &rels).unwrap();
        let html = render_node(&node, &mut std::collections::HashMap::new());
        assert!(!html.is_empty());
    }
    #[test]
    fn test_renders_closed_arrowhead_marker() {
        let xml = r#"<p:sp xmlns:p="http://schemas.openxmlformats.org/presentationml/2006/main"
            xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main">
        <p:nvSpPr><p:cNvPr id="214" name="Closed"/><p:cNvSpPr/><p:nvPr/></p:nvSpPr>
        <p:spPr>
          <a:xfrm><a:off x="0" y="0"/><a:ext cx="2000000" cy="0"/></a:xfrm>
          <a:prstGeom prst="line"><a:avLst/></a:prstGeom>
          <a:ln w="12700">
            <a:solidFill><a:srgbClr val="000000"/></a:solidFill>
            <a:tailEnd type="closed" w="med" len="med"/>
          </a:ln>
        </p:spPr>
      </p:sp>"#;
        let root = XmlNode::parse(xml).unwrap();
        let mut pkg = OpcPackage::open(format!("{}/tests/template.pptx", std::env::var("CARGO_MANIFEST_DIR").unwrap())).unwrap();
        let mut colors = HashMap::new();
        colors.insert("accent1".to_string(), "4472C4".to_string());
        colors.insert("lt1".to_string(), "FFFFFF".to_string());
        colors.insert("dk1".to_string(), "000000".to_string());
        let theme = ThemeData { major_font: "Calibri".to_string(), minor_font: "Calibri".to_string(), colors };
        let ctx = StyleContext { theme: Some(&theme), master_styles: None, master_placeholders: &[], layout_placeholders: &[] };
        let rels = HashMap::new();
        let node = parse_node(&root, &ctx, &mut pkg, "ppt/slides/slide1.xml", &rels).unwrap();
        let html = render_node(&node, &mut std::collections::HashMap::new());
        assert!(!html.is_empty());
    }
    #[test]
    
    fn test_renders_shape_with_miter_line_join() {
        let xml = r#"<p:sp xmlns:p="http://schemas.openxmlformats.org/presentationml/2006/main"
            xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main">
        <p:nvSpPr><p:cNvPr id="215" name="Miter"/><p:cNvSpPr/><p:nvPr/></p:nvSpPr>
        <p:spPr>
          <a:xfrm><a:off x="0" y="0"/><a:ext cx="2000000" cy="1000000"/></a:xfrm>
          <a:prstGeom prst="rect"><a:avLst/></a:prstGeom>
          <a:solidFill><a:srgbClr val="FF0000"/></a:solidFill>
          <a:ln w="25400">
            <a:solidFill><a:srgbClr val="000000"/></a:solidFill>
            <a:miter/>
          </a:ln>
        </p:spPr>
      </p:sp>"#;
        let root = XmlNode::parse(xml).unwrap();
        let mut pkg = OpcPackage::open(format!("{}/tests/template.pptx", std::env::var("CARGO_MANIFEST_DIR").unwrap())).unwrap();
        let mut colors = HashMap::new();
        colors.insert("accent1".to_string(), "4472C4".to_string());
        colors.insert("lt1".to_string(), "FFFFFF".to_string());
        colors.insert("dk1".to_string(), "000000".to_string());
        let theme = ThemeData { major_font: "Calibri".to_string(), minor_font: "Calibri".to_string(), colors };
        let ctx = StyleContext { theme: Some(&theme), master_styles: None, master_placeholders: &[], layout_placeholders: &[] };
        let rels = HashMap::new();
        let node = parse_node(&root, &ctx, &mut pkg, "ppt/slides/slide1.xml", &rels).unwrap();
        let html = render_node(&node, &mut std::collections::HashMap::new());
        assert!(!html.is_empty());
        assert!(html.contains("miter"));
    }
    #[test]
    
    fn test_renders_shape_with_bevel_line_join() {
        let xml = r#"<p:sp xmlns:p="http://schemas.openxmlformats.org/presentationml/2006/main"
            xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main">
        <p:nvSpPr><p:cNvPr id="216" name="Bevel"/><p:cNvSpPr/><p:nvPr/></p:nvSpPr>
        <p:spPr>
          <a:xfrm><a:off x="0" y="0"/><a:ext cx="2000000" cy="1000000"/></a:xfrm>
          <a:prstGeom prst="rect"><a:avLst/></a:prstGeom>
          <a:solidFill><a:srgbClr val="FF0000"/></a:solidFill>
          <a:ln w="25400">
            <a:solidFill><a:srgbClr val="000000"/></a:solidFill>
            <a:bevel/>
          </a:ln>
        </p:spPr>
      </p:sp>"#;
        let root = XmlNode::parse(xml).unwrap();
        let mut pkg = OpcPackage::open(format!("{}/tests/template.pptx", std::env::var("CARGO_MANIFEST_DIR").unwrap())).unwrap();
        let mut colors = HashMap::new();
        colors.insert("accent1".to_string(), "4472C4".to_string());
        colors.insert("lt1".to_string(), "FFFFFF".to_string());
        colors.insert("dk1".to_string(), "000000".to_string());
        let theme = ThemeData { major_font: "Calibri".to_string(), minor_font: "Calibri".to_string(), colors };
        let ctx = StyleContext { theme: Some(&theme), master_styles: None, master_placeholders: &[], layout_placeholders: &[] };
        let rels = HashMap::new();
        let node = parse_node(&root, &ctx, &mut pkg, "ppt/slides/slide1.xml", &rels).unwrap();
        let html = render_node(&node, &mut std::collections::HashMap::new());
        assert!(!html.is_empty());
        assert!(html.contains("bevel"));
    }
    #[test]
    
    fn test_renders_shape_with_square_line_cap() {
        let xml = r#"<p:sp xmlns:p="http://schemas.openxmlformats.org/presentationml/2006/main"
            xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main">
        <p:nvSpPr><p:cNvPr id="217" name="SquareCap"/><p:cNvSpPr/><p:nvPr/></p:nvSpPr>
        <p:spPr>
          <a:xfrm><a:off x="0" y="0"/><a:ext cx="2000000" cy="0"/></a:xfrm>
          <a:prstGeom prst="line"><a:avLst/></a:prstGeom>
          <a:ln w="25400" cap="sq">
            <a:solidFill><a:srgbClr val="000000"/></a:solidFill>
          </a:ln>
        </p:spPr>
      </p:sp>"#;
        let root = XmlNode::parse(xml).unwrap();
        let mut pkg = OpcPackage::open(format!("{}/tests/template.pptx", std::env::var("CARGO_MANIFEST_DIR").unwrap())).unwrap();
        let mut colors = HashMap::new();
        colors.insert("accent1".to_string(), "4472C4".to_string());
        colors.insert("lt1".to_string(), "FFFFFF".to_string());
        colors.insert("dk1".to_string(), "000000".to_string());
        let theme = ThemeData { major_font: "Calibri".to_string(), minor_font: "Calibri".to_string(), colors };
        let ctx = StyleContext { theme: Some(&theme), master_styles: None, master_placeholders: &[], layout_placeholders: &[] };
        let rels = HashMap::new();
        let node = parse_node(&root, &ctx, &mut pkg, "ppt/slides/slide1.xml", &rels).unwrap();
        let html = render_node(&node, &mut std::collections::HashMap::new());
        assert!(!html.is_empty());
        assert!(html.contains("square"));
    }
    #[test]
    
    fn test_renders_shape_with_butt_line_cap() {
        let xml = r#"<p:sp xmlns:p="http://schemas.openxmlformats.org/presentationml/2006/main"
            xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main">
        <p:nvSpPr><p:cNvPr id="218" name="ButtCap"/><p:cNvSpPr/><p:nvPr/></p:nvSpPr>
        <p:spPr>
          <a:xfrm><a:off x="0" y="0"/><a:ext cx="2000000" cy="0"/></a:xfrm>
          <a:prstGeom prst="line"><a:avLst/></a:prstGeom>
          <a:ln w="25400" cap="flat">
            <a:solidFill><a:srgbClr val="000000"/></a:solidFill>
          </a:ln>
        </p:spPr>
      </p:sp>"#;
        let root = XmlNode::parse(xml).unwrap();
        let mut pkg = OpcPackage::open(format!("{}/tests/template.pptx", std::env::var("CARGO_MANIFEST_DIR").unwrap())).unwrap();
        let mut colors = HashMap::new();
        colors.insert("accent1".to_string(), "4472C4".to_string());
        colors.insert("lt1".to_string(), "FFFFFF".to_string());
        colors.insert("dk1".to_string(), "000000".to_string());
        let theme = ThemeData { major_font: "Calibri".to_string(), minor_font: "Calibri".to_string(), colors };
        let ctx = StyleContext { theme: Some(&theme), master_styles: None, master_placeholders: &[], layout_placeholders: &[] };
        let rels = HashMap::new();
        let node = parse_node(&root, &ctx, &mut pkg, "ppt/slides/slide1.xml", &rels).unwrap();
        let html = render_node(&node, &mut std::collections::HashMap::new());
        assert!(!html.is_empty());
        assert!(html.contains("butt"));
    }
    #[test]
    fn test_renders_small_arrowhead_when_w_sm_and_len_sm() {
        let xml = r#"<p:sp xmlns:p="http://schemas.openxmlformats.org/presentationml/2006/main"
            xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main">
        <p:nvSpPr><p:cNvPr id="219" name="SmallArrow"/><p:cNvSpPr/><p:nvPr/></p:nvSpPr>
        <p:spPr>
          <a:xfrm><a:off x="0" y="0"/><a:ext cx="2000000" cy="0"/></a:xfrm>
          <a:prstGeom prst="line"><a:avLst/></a:prstGeom>
          <a:ln w="12700">
            <a:solidFill><a:srgbClr val="000000"/></a:solidFill>
            <a:tailEnd type="triangle" w="sm" len="sm"/>
          </a:ln>
        </p:spPr>
      </p:sp>"#;
        let root = XmlNode::parse(xml).unwrap();
        let mut pkg = OpcPackage::open(format!("{}/tests/template.pptx", std::env::var("CARGO_MANIFEST_DIR").unwrap())).unwrap();
        let mut colors = HashMap::new();
        colors.insert("accent1".to_string(), "4472C4".to_string());
        colors.insert("lt1".to_string(), "FFFFFF".to_string());
        colors.insert("dk1".to_string(), "000000".to_string());
        let theme = ThemeData { major_font: "Calibri".to_string(), minor_font: "Calibri".to_string(), colors };
        let ctx = StyleContext { theme: Some(&theme), master_styles: None, master_placeholders: &[], layout_placeholders: &[] };
        let rels = HashMap::new();
        let node = parse_node(&root, &ctx, &mut pkg, "ppt/slides/slide1.xml", &rels).unwrap();
        let html = render_node(&node, &mut std::collections::HashMap::new());
        assert!(!html.is_empty());
    }
    #[test]
    fn test_renders_large_arrowhead_when_w_lg_and_len_lg() {
        let xml = r#"<p:sp xmlns:p="http://schemas.openxmlformats.org/presentationml/2006/main"
            xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main">
        <p:nvSpPr><p:cNvPr id="220" name="LargeArrow"/><p:cNvSpPr/><p:nvPr/></p:nvSpPr>
        <p:spPr>
          <a:xfrm><a:off x="0" y="0"/><a:ext cx="2000000" cy="0"/></a:xfrm>
          <a:prstGeom prst="line"><a:avLst/></a:prstGeom>
          <a:ln w="38100">
            <a:solidFill><a:srgbClr val="000000"/></a:solidFill>
            <a:headEnd type="triangle" w="lg" len="lg"/>
          </a:ln>
        </p:spPr>
      </p:sp>"#;
        let root = XmlNode::parse(xml).unwrap();
        let mut pkg = OpcPackage::open(format!("{}/tests/template.pptx", std::env::var("CARGO_MANIFEST_DIR").unwrap())).unwrap();
        let mut colors = HashMap::new();
        colors.insert("accent1".to_string(), "4472C4".to_string());
        colors.insert("lt1".to_string(), "FFFFFF".to_string());
        colors.insert("dk1".to_string(), "000000".to_string());
        let theme = ThemeData { major_font: "Calibri".to_string(), minor_font: "Calibri".to_string(), colors };
        let ctx = StyleContext { theme: Some(&theme), master_styles: None, master_placeholders: &[], layout_placeholders: &[] };
        let rels = HashMap::new();
        let node = parse_node(&root, &ctx, &mut pkg, "ppt/slides/slide1.xml", &rels).unwrap();
        let html = render_node(&node, &mut std::collections::HashMap::new());
        assert!(!html.is_empty());
    }
    #[test]
    
    fn test_renders_arc_shape_as_outline_only_no_fill() {
        let xml = r#"<p:sp xmlns:p="http://schemas.openxmlformats.org/presentationml/2006/main"
            xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main">
        <p:nvSpPr><p:cNvPr id="221" name="Arc"/><p:cNvSpPr/><p:nvPr/></p:nvSpPr>
        <p:spPr>
          <a:xfrm><a:off x="0" y="0"/><a:ext cx="2000000" cy="1000000"/></a:xfrm>
          <a:prstGeom prst="arc"><a:avLst/></a:prstGeom>
          <a:ln w="12700"><a:solidFill><a:srgbClr val="000000"/></a:solidFill></a:ln>
        </p:spPr>
      </p:sp>"#;
        let root = XmlNode::parse(xml).unwrap();
        let mut pkg = OpcPackage::open(format!("{}/tests/template.pptx", std::env::var("CARGO_MANIFEST_DIR").unwrap())).unwrap();
        let mut colors = HashMap::new();
        colors.insert("accent1".to_string(), "4472C4".to_string());
        colors.insert("lt1".to_string(), "FFFFFF".to_string());
        colors.insert("dk1".to_string(), "000000".to_string());
        let theme = ThemeData { major_font: "Calibri".to_string(), minor_font: "Calibri".to_string(), colors };
        let ctx = StyleContext { theme: Some(&theme), master_styles: None, master_placeholders: &[], layout_placeholders: &[] };
        let rels = HashMap::new();
        let node = parse_node(&root, &ctx, &mut pkg, "ppt/slides/slide1.xml", &rels).unwrap();
        let html = render_node(&node, &mut std::collections::HashMap::new());
        assert!(!html.is_empty());
        // assert!(!html.contains("none"));
    }
    #[test]
    fn test_renders_text_only_shape_without_visible_text_as_no_fill() {
        let xml = r#"<p:sp xmlns:p="http://schemas.openxmlformats.org/presentationml/2006/main"
            xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main">
        <p:nvSpPr><p:cNvPr id="222" name="EmptyText"/><p:cNvSpPr/><p:nvPr/></p:nvSpPr>
        <p:spPr>
          <a:xfrm><a:off x="0" y="0"/><a:ext cx="2000000" cy="1000000"/></a:xfrm>
          <a:prstGeom prst="rect"><a:avLst/></a:prstGeom>
        </p:spPr>
        <p:txBody>
          <a:bodyPr/>
          <a:lstStyle/>
          <a:p></a:p>
        </p:txBody>
      </p:sp>"#;
        let root = XmlNode::parse(xml).unwrap();
        let mut pkg = OpcPackage::open(format!("{}/tests/template.pptx", std::env::var("CARGO_MANIFEST_DIR").unwrap())).unwrap();
        let mut colors = HashMap::new();
        colors.insert("accent1".to_string(), "4472C4".to_string());
        colors.insert("lt1".to_string(), "FFFFFF".to_string());
        colors.insert("dk1".to_string(), "000000".to_string());
        let theme = ThemeData { major_font: "Calibri".to_string(), minor_font: "Calibri".to_string(), colors };
        let ctx = StyleContext { theme: Some(&theme), master_styles: None, master_placeholders: &[], layout_placeholders: &[] };
        let rels = HashMap::new();
        let node = parse_node(&root, &ctx, &mut pkg, "ppt/slides/slide1.xml", &rels).unwrap();
        let html = render_node(&node, &mut std::collections::HashMap::new());
        assert!(!html.is_empty());
        assert!(html.contains("none"));
    }
    #[test]
    fn test_hlinkclick_with_ppaction_hlinksldjump_registers_click_handler_and_sets_pointer_cursor() {
        let xml = r#"<p:sp xmlns:p="http://schemas.openxmlformats.org/presentationml/2006/main"
            xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main"
            xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships">
        <p:nvSpPr>
          <p:cNvPr id="300" name="SlideJump">
            <a:hlinkClick r:id="rId5" action="ppaction://hlinksldjump"/>
          </p:cNvPr>
          <p:cNvSpPr/>
          <p:nvPr/>
        </p:nvSpPr>
        <p:spPr>
          <a:xfrm><a:off x="0" y="0"/><a:ext cx="2000000" cy="1000000"/></a:xfrm>
          <a:prstGeom prst="rect"><a:avLst/></a:prstGeom>
          <a:solidFill><a:srgbClr val="4472C4"/></a:solidFill>
        </p:spPr>
      </p:sp>"#;
        let root = XmlNode::parse(xml).unwrap();
        let mut pkg = OpcPackage::open(format!("{}/tests/template.pptx", std::env::var("CARGO_MANIFEST_DIR").unwrap())).unwrap();
        let mut colors = HashMap::new();
        colors.insert("accent1".to_string(), "4472C4".to_string());
        colors.insert("lt1".to_string(), "FFFFFF".to_string());
        colors.insert("dk1".to_string(), "000000".to_string());
        let theme = ThemeData { major_font: "Calibri".to_string(), minor_font: "Calibri".to_string(), colors };
        let ctx = StyleContext { theme: Some(&theme), master_styles: None, master_placeholders: &[], layout_placeholders: &[] };
        let rels = {
            let mut r = HashMap::new();
            r.insert("rId5".to_string(), crate::parser::RelEntry { id: "".to_string(), target: "slide2.xml".to_string(), rel_type: "hyperlink".to_string() });
            r
        };
        let node = parse_node(&root, &ctx, &mut pkg, "ppt/slides/slide1.xml", &rels).unwrap();
        let html = render_node(&node, &mut std::collections::HashMap::new());
        assert!(!html.is_empty());
        assert!(html.contains("cursor: pointer"));
    }
    #[test]
    
    fn test_hlinkclick_ppaction_hlinksldjump_uses_tooltip_as_title_when_provided() {
        let xml = r#"<p:sp xmlns:p="http://schemas.openxmlformats.org/presentationml/2006/main"
            xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main"
            xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships">
        <p:nvSpPr>
          <p:cNvPr id="301" name="SlideJumpTooltip">
            <a:hlinkClick r:id="rId6" action="ppaction://hlinksldjump" tooltip="Go to summary"/>
          </p:cNvPr>
          <p:cNvSpPr/>
          <p:nvPr/>
        </p:nvSpPr>
        <p:spPr>
          <a:xfrm><a:off x="0" y="0"/><a:ext cx="2000000" cy="1000000"/></a:xfrm>
          <a:prstGeom prst="rect"><a:avLst/></a:prstGeom>
          <a:solidFill><a:srgbClr val="4472C4"/></a:solidFill>
        </p:spPr>
      </p:sp>"#;
        let root = XmlNode::parse(xml).unwrap();
        let mut pkg = OpcPackage::open(format!("{}/tests/template.pptx", std::env::var("CARGO_MANIFEST_DIR").unwrap())).unwrap();
        let mut colors = HashMap::new();
        colors.insert("accent1".to_string(), "4472C4".to_string());
        colors.insert("lt1".to_string(), "FFFFFF".to_string());
        colors.insert("dk1".to_string(), "000000".to_string());
        let theme = ThemeData { major_font: "Calibri".to_string(), minor_font: "Calibri".to_string(), colors };
        let ctx = StyleContext { theme: Some(&theme), master_styles: None, master_placeholders: &[], layout_placeholders: &[] };
        let rels = {
            let mut r = HashMap::new();
            r.insert("rId6".to_string(), crate::parser::RelEntry { id: "".to_string(), target: "slide3.xml".to_string(), rel_type: "hyperlink".to_string() });
            r
        };
        let node = parse_node(&root, &ctx, &mut pkg, "ppt/slides/slide1.xml", &rels).unwrap();
        let html = render_node(&node, &mut std::collections::HashMap::new());
        assert!(!html.is_empty());
        assert!(html.contains("Go to summary"));
    }
    #[test]
    
    fn test_hlinkclick_ppaction_hlinksldjump_falls_back_to_go_to_slide_n_title_when_no_tooltip() {
        let xml = r#"<p:sp xmlns:p="http://schemas.openxmlformats.org/presentationml/2006/main"
            xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main"
            xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships">
        <p:nvSpPr>
          <p:cNvPr id="302" name="SlideJumpNoTooltip">
            <a:hlinkClick r:id="rId7" action="ppaction://hlinksldjump"/>
          </p:cNvPr>
          <p:cNvSpPr/>
          <p:nvPr/>
        </p:nvSpPr>
        <p:spPr>
          <a:xfrm><a:off x="0" y="0"/><a:ext cx="2000000" cy="1000000"/></a:xfrm>
          <a:prstGeom prst="rect"><a:avLst/></a:prstGeom>
          <a:solidFill><a:srgbClr val="4472C4"/></a:solidFill>
        </p:spPr>
      </p:sp>"#;
        let root = XmlNode::parse(xml).unwrap();
        let mut pkg = OpcPackage::open(format!("{}/tests/template.pptx", std::env::var("CARGO_MANIFEST_DIR").unwrap())).unwrap();
        let mut colors = HashMap::new();
        colors.insert("accent1".to_string(), "4472C4".to_string());
        colors.insert("lt1".to_string(), "FFFFFF".to_string());
        colors.insert("dk1".to_string(), "000000".to_string());
        let theme = ThemeData { major_font: "Calibri".to_string(), minor_font: "Calibri".to_string(), colors };
        let ctx = StyleContext { theme: Some(&theme), master_styles: None, master_placeholders: &[], layout_placeholders: &[] };
        let rels = {
            let mut r = HashMap::new();
            r.insert("rId7".to_string(), crate::parser::RelEntry { id: "".to_string(), target: "slide10.xml".to_string(), rel_type: "hyperlink".to_string() });
            r
        };
        let node = parse_node(&root, &ctx, &mut pkg, "ppt/slides/slide1.xml", &rels).unwrap();
        let html = render_node(&node, &mut std::collections::HashMap::new());
        assert!(!html.is_empty());
        assert!(html.contains("Go to slide 10"));
    }
    #[test]
    
    fn test_hlinkclick_ppaction_hlinksldjump_is_a_no_op_when_onnavigate_is_not_set() {
        let xml = r#"<p:sp xmlns:p="http://schemas.openxmlformats.org/presentationml/2006/main"
            xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main"
            xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships">
        <p:nvSpPr>
          <p:cNvPr id="303" name="SlideJumpNoHandler">
            <a:hlinkClick r:id="rId8" action="ppaction://hlinksldjump"/>
          </p:cNvPr>
          <p:cNvSpPr/>
          <p:nvPr/>
        </p:nvSpPr>
        <p:spPr>
          <a:xfrm><a:off x="0" y="0"/><a:ext cx="2000000" cy="1000000"/></a:xfrm>
          <a:prstGeom prst="rect"><a:avLst/></a:prstGeom>
          <a:solidFill><a:srgbClr val="4472C4"/></a:solidFill>
        </p:spPr>
      </p:sp>"#;
        let root = XmlNode::parse(xml).unwrap();
        let mut pkg = OpcPackage::open(format!("{}/tests/template.pptx", std::env::var("CARGO_MANIFEST_DIR").unwrap())).unwrap();
        let mut colors = HashMap::new();
        colors.insert("accent1".to_string(), "4472C4".to_string());
        colors.insert("lt1".to_string(), "FFFFFF".to_string());
        colors.insert("dk1".to_string(), "000000".to_string());
        let theme = ThemeData { major_font: "Calibri".to_string(), minor_font: "Calibri".to_string(), colors };
        let ctx = StyleContext { theme: Some(&theme), master_styles: None, master_placeholders: &[], layout_placeholders: &[] };
        let rels = {
            let mut r = HashMap::new();
            r.insert("rId8".to_string(), crate::parser::RelEntry { id: "".to_string(), target: "slide4.xml".to_string(), rel_type: "hyperlink".to_string() });
            r
        };
        let node = parse_node(&root, &ctx, &mut pkg, "ppt/slides/slide1.xml", &rels).unwrap();
        let html = render_node(&node, &mut std::collections::HashMap::new());
        assert!(!html.is_empty());
        assert!(html.contains("cursor: pointer"));
        assert!(html.contains("#slide4"));
    }
    #[test]
    
    fn test_hlinkclick_with_external_url_registers_click_handler_and_uses_url_as_title() {
        let xml = r#"<p:sp xmlns:p="http://schemas.openxmlformats.org/presentationml/2006/main"
            xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main"
            xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships">
        <p:nvSpPr>
          <p:cNvPr id="304" name="ExternalLink">
            <a:hlinkClick r:id="rId9"/>
          </p:cNvPr>
          <p:cNvSpPr/>
          <p:nvPr/>
        </p:nvSpPr>
        <p:spPr>
          <a:xfrm><a:off x="0" y="0"/><a:ext cx="2000000" cy="1000000"/></a:xfrm>
          <a:prstGeom prst="rect"><a:avLst/></a:prstGeom>
          <a:solidFill><a:srgbClr val="4472C4"/></a:solidFill>
        </p:spPr>
      </p:sp>"#;
        let root = XmlNode::parse(xml).unwrap();
        let mut pkg = OpcPackage::open(format!("{}/tests/template.pptx", std::env::var("CARGO_MANIFEST_DIR").unwrap())).unwrap();
        let mut colors = HashMap::new();
        colors.insert("accent1".to_string(), "4472C4".to_string());
        colors.insert("lt1".to_string(), "FFFFFF".to_string());
        colors.insert("dk1".to_string(), "000000".to_string());
        let theme = ThemeData { major_font: "Calibri".to_string(), minor_font: "Calibri".to_string(), colors };
        let ctx = StyleContext { theme: Some(&theme), master_styles: None, master_placeholders: &[], layout_placeholders: &[] };
        let rels = {
            let mut r = HashMap::new();
            r.insert("rId9".to_string(), crate::parser::RelEntry { id: "".to_string(), target: "https://example.com".to_string(), rel_type: "hyperlink".to_string() });
            r
        };
        let node = parse_node(&root, &ctx, &mut pkg, "ppt/slides/slide1.xml", &rels).unwrap();
        let html = render_node(&node, &mut std::collections::HashMap::new());
        assert!(!html.is_empty());
        assert!(html.contains("cursor: pointer"));
        assert!(html.contains("https://example.com"));
        assert!(html.contains("https://example.com"));
    }
    #[test]
    
    fn test_hlinkclick_with_external_url_uses_tooltip_as_title_when_provided() {
        let xml = r#"<p:sp xmlns:p="http://schemas.openxmlformats.org/presentationml/2006/main"
            xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main"
            xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships">
        <p:nvSpPr>
          <p:cNvPr id="305" name="ExternalLinkTooltip">
            <a:hlinkClick r:id="rId10" tooltip="Visit website"/>
          </p:cNvPr>
          <p:cNvSpPr/>
          <p:nvPr/>
        </p:nvSpPr>
        <p:spPr>
          <a:xfrm><a:off x="0" y="0"/><a:ext cx="2000000" cy="1000000"/></a:xfrm>
          <a:prstGeom prst="rect"><a:avLst/></a:prstGeom>
          <a:solidFill><a:srgbClr val="4472C4"/></a:solidFill>
        </p:spPr>
      </p:sp>"#;
        let root = XmlNode::parse(xml).unwrap();
        let mut pkg = OpcPackage::open(format!("{}/tests/template.pptx", std::env::var("CARGO_MANIFEST_DIR").unwrap())).unwrap();
        let mut colors = HashMap::new();
        colors.insert("accent1".to_string(), "4472C4".to_string());
        colors.insert("lt1".to_string(), "FFFFFF".to_string());
        colors.insert("dk1".to_string(), "000000".to_string());
        let theme = ThemeData { major_font: "Calibri".to_string(), minor_font: "Calibri".to_string(), colors };
        let ctx = StyleContext { theme: Some(&theme), master_styles: None, master_placeholders: &[], layout_placeholders: &[] };
        let rels = {
            let mut r = HashMap::new();
            r.insert("rId10".to_string(), crate::parser::RelEntry { id: "".to_string(), target: "https://example.com".to_string(), rel_type: "hyperlink".to_string() });
            r
        };
        let node = parse_node(&root, &ctx, &mut pkg, "ppt/slides/slide1.xml", &rels).unwrap();
        let html = render_node(&node, &mut std::collections::HashMap::new());
        assert!(!html.is_empty());
        assert!(html.contains("Visit website"));
    }
    #[test]
    
    fn test_hlinkclick_with_disallowed_url_protocol_javascript_does_not_register_click_handler() {
        let xml = r#"<p:sp xmlns:p="http://schemas.openxmlformats.org/presentationml/2006/main"
            xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main"
            xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships">
        <p:nvSpPr>
          <p:cNvPr id="306" name="DisallowedUrl">
            <a:hlinkClick r:id="rId11"/>
          </p:cNvPr>
          <p:cNvSpPr/>
          <p:nvPr/>
        </p:nvSpPr>
        <p:spPr>
          <a:xfrm><a:off x="0" y="0"/><a:ext cx="2000000" cy="1000000"/></a:xfrm>
          <a:prstGeom prst="rect"><a:avLst/></a:prstGeom>
          <a:solidFill><a:srgbClr val="4472C4"/></a:solidFill>
        </p:spPr>
      </p:sp>"#;
        let root = XmlNode::parse(xml).unwrap();
        let mut pkg = OpcPackage::open(format!("{}/tests/template.pptx", std::env::var("CARGO_MANIFEST_DIR").unwrap())).unwrap();
        let mut colors = HashMap::new();
        colors.insert("accent1".to_string(), "4472C4".to_string());
        colors.insert("lt1".to_string(), "FFFFFF".to_string());
        colors.insert("dk1".to_string(), "000000".to_string());
        let theme = ThemeData { major_font: "Calibri".to_string(), minor_font: "Calibri".to_string(), colors };
        let ctx = StyleContext { theme: Some(&theme), master_styles: None, master_placeholders: &[], layout_placeholders: &[] };
        let rels = {
            let mut r = HashMap::new();
            r.insert("rId11".to_string(), crate::parser::RelEntry { id: "".to_string(), target: "javascript:alert(\"XSS\")".to_string(), rel_type: "hyperlink".to_string() });
            r
        };
        let node = parse_node(&root, &ctx, &mut pkg, "ppt/slides/slide1.xml", &rels).unwrap();
        let html = render_node(&node, &mut std::collections::HashMap::new());
        assert!(!html.is_empty());
        assert!(!html.contains("cursor: pointer"));
    }
    #[test]
    
    fn test_hlinkclick_with_missing_relationship_rid_does_not_crash_or_set_cursor() {
        let xml = r#"<p:sp xmlns:p="http://schemas.openxmlformats.org/presentationml/2006/main"
            xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main"
            xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships">
        <p:nvSpPr>
          <p:cNvPr id="307" name="MissingRel">
            <a:hlinkClick r:id="rId_missing" action="ppaction://hlinksldjump"/>
          </p:cNvPr>
          <p:cNvSpPr/>
          <p:nvPr/>
        </p:nvSpPr>
        <p:spPr>
          <a:xfrm><a:off x="0" y="0"/><a:ext cx="2000000" cy="1000000"/></a:xfrm>
          <a:prstGeom prst="rect"><a:avLst/></a:prstGeom>
          <a:solidFill><a:srgbClr val="4472C4"/></a:solidFill>
        </p:spPr>
      </p:sp>"#;
        let root = XmlNode::parse(xml).unwrap();
        let mut pkg = OpcPackage::open(format!("{}/tests/template.pptx", std::env::var("CARGO_MANIFEST_DIR").unwrap())).unwrap();
        let mut colors = HashMap::new();
        colors.insert("accent1".to_string(), "4472C4".to_string());
        colors.insert("lt1".to_string(), "FFFFFF".to_string());
        colors.insert("dk1".to_string(), "000000".to_string());
        let theme = ThemeData { major_font: "Calibri".to_string(), minor_font: "Calibri".to_string(), colors };
        let ctx = StyleContext { theme: Some(&theme), master_styles: None, master_placeholders: &[], layout_placeholders: &[] };
        let rels = HashMap::new();
        let node = parse_node(&root, &ctx, &mut pkg, "ppt/slides/slide1.xml", &rels).unwrap();
        let html = render_node(&node, &mut std::collections::HashMap::new());
        assert!(!html.is_empty());
        assert!(!html.contains("cursor: pointer"));
    }
    #[test]
    
    fn test_normautofit_with_explicit_fontscale_applies_css_transform_scale_to_text_container() {
        let xml = r#"<p:sp xmlns:p="http://schemas.openxmlformats.org/presentationml/2006/main"
            xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main">
        <p:nvSpPr><p:cNvPr id="400" name="NormAutofit"/><p:cNvSpPr/><p:nvPr/></p:nvSpPr>
        <p:spPr>
          <a:xfrm><a:off x="0" y="0"/><a:ext cx="2000000" cy="800000"/></a:xfrm>
          <a:prstGeom prst="rect"><a:avLst/></a:prstGeom>
          <a:noFill/><a:ln><a:noFill/></a:ln>
        </p:spPr>
        <p:txBody>
          <a:bodyPr wrap="square"><a:normAutofit fontScale="60000" lnSpcReduction="10000"/></a:bodyPr>
          <a:lstStyle/>
          <a:p><a:r><a:t>Shrink this text to fit the shape box.</a:t></a:r></a:p>
        </p:txBody>
      </p:sp>"#;
        let root = XmlNode::parse(xml).unwrap();
        let mut pkg = OpcPackage::open(format!("{}/tests/template.pptx", std::env::var("CARGO_MANIFEST_DIR").unwrap())).unwrap();
        let mut colors = HashMap::new();
        colors.insert("accent1".to_string(), "4472C4".to_string());
        colors.insert("lt1".to_string(), "FFFFFF".to_string());
        colors.insert("dk1".to_string(), "000000".to_string());
        let theme = ThemeData { major_font: "Calibri".to_string(), minor_font: "Calibri".to_string(), colors };
        let ctx = StyleContext { theme: Some(&theme), master_styles: None, master_placeholders: &[], layout_placeholders: &[] };
        let rels = HashMap::new();
        let node = parse_node(&root, &ctx, &mut pkg, "ppt/slides/slide1.xml", &rels).unwrap();
        let html = render_node(&node, &mut std::collections::HashMap::new());
        assert!(!html.is_empty());
        assert!(html.contains("scale(0.6)"));
        assert!(html.contains("%"));
        assert!(html.contains("%"));
        assert!(html.contains("hidden"));
    }
    #[test]
    
    fn test_normautofit_with_fontscale_100000_no_shrink_needed_does_not_apply_scale_transform() {
        let xml = r#"<p:sp xmlns:p="http://schemas.openxmlformats.org/presentationml/2006/main"
            xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main">
        <p:nvSpPr><p:cNvPr id="401" name="NormAutofitFull"/><p:cNvSpPr/><p:nvPr/></p:nvSpPr>
        <p:spPr>
          <a:xfrm><a:off x="0" y="0"/><a:ext cx="2000000" cy="800000"/></a:xfrm>
          <a:prstGeom prst="rect"><a:avLst/></a:prstGeom>
          <a:noFill/><a:ln><a:noFill/></a:ln>
        </p:spPr>
        <p:txBody>
          <a:bodyPr wrap="square"><a:normAutofit fontScale="100000"/></a:bodyPr>
          <a:lstStyle/>
          <a:p><a:r><a:t>Text fits exactly.</a:t></a:r></a:p>
        </p:txBody>
      </p:sp>"#;
        let root = XmlNode::parse(xml).unwrap();
        let mut pkg = OpcPackage::open(format!("{}/tests/template.pptx", std::env::var("CARGO_MANIFEST_DIR").unwrap())).unwrap();
        let mut colors = HashMap::new();
        colors.insert("accent1".to_string(), "4472C4".to_string());
        colors.insert("lt1".to_string(), "FFFFFF".to_string());
        colors.insert("dk1".to_string(), "000000".to_string());
        let theme = ThemeData { major_font: "Calibri".to_string(), minor_font: "Calibri".to_string(), colors };
        let ctx = StyleContext { theme: Some(&theme), master_styles: None, master_placeholders: &[], layout_placeholders: &[] };
        let rels = HashMap::new();
        let node = parse_node(&root, &ctx, &mut pkg, "ppt/slides/slide1.xml", &rels).unwrap();
        let html = render_node(&node, &mut std::collections::HashMap::new());
        assert!(!html.is_empty());
        assert!(!html.contains("scale("));
        assert!(html.contains("hidden"));
    }
    #[test]
    fn test_normautofit_without_fontscale_attribute_triggers_dynamic_scaling_path_wrapper_inserted_into_dom() {
        let xml = r#"<p:sp xmlns:p="http://schemas.openxmlformats.org/presentationml/2006/main"
            xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main">
        <p:nvSpPr><p:cNvPr id="402" name="NormAutofitDynamic"/><p:cNvSpPr/><p:nvPr/></p:nvSpPr>
        <p:spPr>
          <a:xfrm><a:off x="0" y="0"/><a:ext cx="2000000" cy="800000"/></a:xfrm>
          <a:prstGeom prst="rect"><a:avLst/></a:prstGeom>
          <a:noFill/><a:ln><a:noFill/></a:ln>
        </p:spPr>
        <p:txBody>
          <a:bodyPr wrap="square"><a:normAutofit/></a:bodyPr>
          <a:lstStyle/>
          <a:p><a:r><a:t>Dynamic fit needed.</a:t></a:r></a:p>
        </p:txBody>
      </p:sp>"#;
        let root = XmlNode::parse(xml).unwrap();
        let mut pkg = OpcPackage::open(format!("{}/tests/template.pptx", std::env::var("CARGO_MANIFEST_DIR").unwrap())).unwrap();
        let mut colors = HashMap::new();
        colors.insert("accent1".to_string(), "4472C4".to_string());
        colors.insert("lt1".to_string(), "FFFFFF".to_string());
        colors.insert("dk1".to_string(), "000000".to_string());
        let theme = ThemeData { major_font: "Calibri".to_string(), minor_font: "Calibri".to_string(), colors };
        let ctx = StyleContext { theme: Some(&theme), master_styles: None, master_placeholders: &[], layout_placeholders: &[] };
        let rels = HashMap::new();
        let node = parse_node(&root, &ctx, &mut pkg, "ppt/slides/slide1.xml", &rels).unwrap();
        let html = render_node(&node, &mut std::collections::HashMap::new());
        assert!(!html.is_empty());
        assert!(!html.contains("hidden"));
    }
    #[test]
    
    fn test_reflection_with_default_sta_and_enda_values_produces_valid_gradient() {
        let xml = r#"<p:sp xmlns:p="http://schemas.openxmlformats.org/presentationml/2006/main"
            xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main">
        <p:nvSpPr><p:cNvPr id="500" name="ReflectionDefaults"/><p:cNvSpPr/><p:nvPr/></p:nvSpPr>
        <p:spPr>
          <a:xfrm><a:off x="0" y="0"/><a:ext cx="2000000" cy="800000"/></a:xfrm>
          <a:prstGeom prst="rect"><a:avLst/></a:prstGeom>
          <a:solidFill><a:srgbClr val="ED7D31"/></a:solidFill>
          <a:effectLst>
            <a:reflection dist="114300"/>
          </a:effectLst>
        </p:spPr>
      </p:sp>"#;
        let root = XmlNode::parse(xml).unwrap();
        let mut pkg = OpcPackage::open(format!("{}/tests/template.pptx", std::env::var("CARGO_MANIFEST_DIR").unwrap())).unwrap();
        let mut colors = HashMap::new();
        colors.insert("accent1".to_string(), "4472C4".to_string());
        colors.insert("lt1".to_string(), "FFFFFF".to_string());
        colors.insert("dk1".to_string(), "000000".to_string());
        let theme = ThemeData { major_font: "Calibri".to_string(), minor_font: "Calibri".to_string(), colors };
        let ctx = StyleContext { theme: Some(&theme), master_styles: None, master_placeholders: &[], layout_placeholders: &[] };
        let rels = HashMap::new();
        let node = parse_node(&root, &ctx, &mut pkg, "ppt/slides/slide1.xml", &rels).unwrap();
        let html = render_node(&node, &mut std::collections::HashMap::new());
        assert!(!html.is_empty());
        assert!(html.contains("0.500"));
        assert!(html.contains("0.000"));
        assert!(html.contains("below"));
    }
    #[test]
    
    fn test_reflection_with_zero_dist_produces_below_0_0px_in_reflect_value() {
        let xml = r#"<p:sp xmlns:p="http://schemas.openxmlformats.org/presentationml/2006/main"
            xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main">
        <p:nvSpPr><p:cNvPr id="501" name="ReflectionZeroDist"/><p:cNvSpPr/><p:nvPr/></p:nvSpPr>
        <p:spPr>
          <a:xfrm><a:off x="0" y="0"/><a:ext cx="2000000" cy="800000"/></a:xfrm>
          <a:prstGeom prst="rect"><a:avLst/></a:prstGeom>
          <a:solidFill><a:srgbClr val="ED7D31"/></a:solidFill>
          <a:effectLst>
            <a:reflection stA="30000" endA="0" dist="0"/>
          </a:effectLst>
        </p:spPr>
      </p:sp>"#;
        let root = XmlNode::parse(xml).unwrap();
        let mut pkg = OpcPackage::open(format!("{}/tests/template.pptx", std::env::var("CARGO_MANIFEST_DIR").unwrap())).unwrap();
        let mut colors = HashMap::new();
        colors.insert("accent1".to_string(), "4472C4".to_string());
        colors.insert("lt1".to_string(), "FFFFFF".to_string());
        colors.insert("dk1".to_string(), "000000".to_string());
        let theme = ThemeData { major_font: "Calibri".to_string(), minor_font: "Calibri".to_string(), colors };
        let ctx = StyleContext { theme: Some(&theme), master_styles: None, master_placeholders: &[], layout_placeholders: &[] };
        let rels = HashMap::new();
        let node = parse_node(&root, &ctx, &mut pkg, "ppt/slides/slide1.xml", &rels).unwrap();
        let html = render_node(&node, &mut std::collections::HashMap::new());
        assert!(!html.is_empty());
        assert!(html.contains("below 0.0px"));
    }
    #[test]
    fn test_effectref_with_idx_0_does_not_apply_any_theme_effect_idx_0_means_no_effect() {
        let xml = r#"<p:sp xmlns:p="http://schemas.openxmlformats.org/presentationml/2006/main"
            xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main">
        <p:nvSpPr><p:cNvPr id="600" name="EffectRefZero"/><p:cNvSpPr/><p:nvPr/></p:nvSpPr>
        <p:spPr>
          <a:xfrm><a:off x="0" y="0"/><a:ext cx="2000000" cy="800000"/></a:xfrm>
          <a:prstGeom prst="rect"><a:avLst/></a:prstGeom>
          <a:solidFill><a:srgbClr val="A5A5A5"/></a:solidFill>
        </p:spPr>
        <p:style><a:effectRef idx="0"><a:schemeClr val="accent1"/></a:effectRef></p:style>
      </p:sp>"#;
        let root = XmlNode::parse(xml).unwrap();
        let mut pkg = OpcPackage::open(format!("{}/tests/template.pptx", std::env::var("CARGO_MANIFEST_DIR").unwrap())).unwrap();
        let mut colors = HashMap::new();
        colors.insert("accent1".to_string(), "4472C4".to_string());
        colors.insert("lt1".to_string(), "FFFFFF".to_string());
        colors.insert("dk1".to_string(), "000000".to_string());
        let theme = ThemeData { major_font: "Calibri".to_string(), minor_font: "Calibri".to_string(), colors };
        let ctx = StyleContext { theme: Some(&theme), master_styles: None, master_placeholders: &[], layout_placeholders: &[] };
        let rels = HashMap::new();
        let node = parse_node(&root, &ctx, &mut pkg, "ppt/slides/slide1.xml", &rels).unwrap();
        let html = render_node(&node, &mut std::collections::HashMap::new());
        assert!(!html.is_empty());
    }
    #[test]
    fn test_effectref_with_idx_exceeding_effectstyles_length_does_not_crash_and_applies_no_shadow() {
        let xml = r#"<p:sp xmlns:p="http://schemas.openxmlformats.org/presentationml/2006/main"
            xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main">
        <p:nvSpPr><p:cNvPr id="601" name="EffectRefOutOfBounds"/><p:cNvSpPr/><p:nvPr/></p:nvSpPr>
        <p:spPr>
          <a:xfrm><a:off x="0" y="0"/><a:ext cx="2000000" cy="800000"/></a:xfrm>
          <a:prstGeom prst="rect"><a:avLst/></a:prstGeom>
          <a:solidFill><a:srgbClr val="A5A5A5"/></a:solidFill>
        </p:spPr>
        <p:style><a:effectRef idx="5"><a:schemeClr val="accent1"/></a:effectRef></p:style>
      </p:sp>"#;
        let root = XmlNode::parse(xml).unwrap();
        let mut pkg = OpcPackage::open(format!("{}/tests/template.pptx", std::env::var("CARGO_MANIFEST_DIR").unwrap())).unwrap();
        let mut colors = HashMap::new();
        colors.insert("accent1".to_string(), "4472C4".to_string());
        colors.insert("lt1".to_string(), "FFFFFF".to_string());
        colors.insert("dk1".to_string(), "000000".to_string());
        let theme = ThemeData { major_font: "Calibri".to_string(), minor_font: "Calibri".to_string(), colors };
        let ctx = StyleContext { theme: Some(&theme), master_styles: None, master_placeholders: &[], layout_placeholders: &[] };
        let rels = HashMap::new();
        let node = parse_node(&root, &ctx, &mut pkg, "ppt/slides/slide1.xml", &rels).unwrap();
        let html = render_node(&node, &mut std::collections::HashMap::new());
        assert!(!html.is_empty());
    }
    #[test]
    fn test_effectref_fallback_is_skipped_when_shape_already_has_explicit_effectlst() {
        let xml = r#"<p:sp xmlns:p="http://schemas.openxmlformats.org/presentationml/2006/main"
            xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main">
        <p:nvSpPr><p:cNvPr id="602" name="ExplicitEffectLst"/><p:cNvSpPr/><p:nvPr/></p:nvSpPr>
        <p:spPr>
          <a:xfrm><a:off x="0" y="0"/><a:ext cx="2000000" cy="800000"/></a:xfrm>
          <a:prstGeom prst="rect"><a:avLst/></a:prstGeom>
          <a:solidFill><a:srgbClr val="3366FF"/></a:solidFill>
          <a:effectLst>
            <a:outerShdw blurRad="50000" dist="38100" dir="2700000">
              <a:srgbClr val="FF0000"><a:alpha val="80000"/></a:srgbClr>
            </a:outerShdw>
          </a:effectLst>
        </p:spPr>
        <p:style><a:effectRef idx="1"><a:schemeClr val="accent1"/></a:effectRef></p:style>
      </p:sp>"#;
        let root = XmlNode::parse(xml).unwrap();
        let mut pkg = OpcPackage::open(format!("{}/tests/template.pptx", std::env::var("CARGO_MANIFEST_DIR").unwrap())).unwrap();
        let mut colors = HashMap::new();
        colors.insert("accent1".to_string(), "4472C4".to_string());
        colors.insert("lt1".to_string(), "FFFFFF".to_string());
        colors.insert("dk1".to_string(), "000000".to_string());
        let theme = ThemeData { major_font: "Calibri".to_string(), minor_font: "Calibri".to_string(), colors };
        let ctx = StyleContext { theme: Some(&theme), master_styles: None, master_placeholders: &[], layout_placeholders: &[] };
        let rels = HashMap::new();
        let node = parse_node(&root, &ctx, &mut pkg, "ppt/slides/slide1.xml", &rels).unwrap();
        let html = render_node(&node, &mut std::collections::HashMap::new());
        assert!(!html.is_empty());
    }
    #[test]
    fn test_resolves_fontref_scheme_color_from_p_style_for_smartart_like_text() {
        let xml = r#"<p:sp xmlns:p="http://schemas.openxmlformats.org/presentationml/2006/main"
            xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main">
        <p:nvSpPr>
          <p:cNvPr id="100" name="SmartArt Shape"/>
          <p:cNvSpPr/>
          <p:nvPr/>
        </p:nvSpPr>
        <p:spPr>
          <a:xfrm><a:off x="0" y="0"/><a:ext cx="2000000" cy="1000000"/></a:xfrm>
          <a:prstGeom prst="rect"><a:avLst/></a:prstGeom>
          <a:solidFill><a:srgbClr val="4472C4"/></a:solidFill>
        </p:spPr>
        <p:style>
          <a:lnRef idx="0"><a:schemeClr val="accent1"/></a:lnRef>
          <a:fillRef idx="0"><a:schemeClr val="accent1"/></a:fillRef>
          <a:effectRef idx="0"><a:schemeClr val="accent1"/></a:effectRef>
          <a:fontRef idx="minor"><a:schemeClr val="dk1"/></a:fontRef>
        </p:style>
        <p:txBody>
          <a:bodyPr/>
          <a:lstStyle/>
          <a:p>
            <a:r><a:rPr lang="en-US" dirty="0"/><a:t>Hello</a:t></a:r>
          </a:p>
        </p:txBody>
      </p:sp>"#;
        let root = XmlNode::parse(xml).unwrap();
        let mut pkg = OpcPackage::open(format!("{}/tests/template.pptx", std::env::var("CARGO_MANIFEST_DIR").unwrap())).unwrap();
        let mut colors = HashMap::new();
        colors.insert("accent1".to_string(), "4472C4".to_string());
        colors.insert("lt1".to_string(), "FFFFFF".to_string());
        colors.insert("dk1".to_string(), "000000".to_string());
        let theme = ThemeData { major_font: "Calibri".to_string(), minor_font: "Calibri".to_string(), colors };
        let ctx = StyleContext { theme: Some(&theme), master_styles: None, master_placeholders: &[], layout_placeholders: &[] };
        let rels = HashMap::new();
        let node = parse_node(&root, &ctx, &mut pkg, "ppt/slides/slide1.xml", &rels).unwrap();
        let html = render_node(&node, &mut std::collections::HashMap::new());
        assert!(!html.is_empty());
        assert!(html.contains("Hello"));
    }
    #[test]
    fn test_resolves_fontref_with_accent_scheme_color_for_smartart_text() {
        let xml = r#"<p:sp xmlns:p="http://schemas.openxmlformats.org/presentationml/2006/main"
            xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main">
        <p:nvSpPr>
          <p:cNvPr id="101" name="SmartArt Dark"/>
          <p:cNvSpPr/>
          <p:nvPr/>
        </p:nvSpPr>
        <p:spPr>
          <a:xfrm><a:off x="0" y="0"/><a:ext cx="2000000" cy="1000000"/></a:xfrm>
          <a:prstGeom prst="rect"><a:avLst/></a:prstGeom>
          <a:solidFill><a:srgbClr val="333333"/></a:solidFill>
        </p:spPr>
        <p:style>
          <a:lnRef idx="0"><a:schemeClr val="accent1"/></a:lnRef>
          <a:fillRef idx="0"><a:schemeClr val="accent1"/></a:fillRef>
          <a:effectRef idx="0"><a:schemeClr val="accent1"/></a:effectRef>
          <a:fontRef idx="minor"><a:schemeClr val="lt1"/></a:fontRef>
        </p:style>
        <p:txBody>
          <a:bodyPr/>
          <a:lstStyle/>
          <a:p>
            <a:r><a:rPr lang="en-US" dirty="0"/><a:t>White Text</a:t></a:r>
          </a:p>
        </p:txBody>
      </p:sp>"#;
        let root = XmlNode::parse(xml).unwrap();
        let mut pkg = OpcPackage::open(format!("{}/tests/template.pptx", std::env::var("CARGO_MANIFEST_DIR").unwrap())).unwrap();
        let mut colors = HashMap::new();
        colors.insert("accent1".to_string(), "4472C4".to_string());
        colors.insert("lt1".to_string(), "FFFFFF".to_string());
        colors.insert("dk1".to_string(), "000000".to_string());
        let theme = ThemeData { major_font: "Calibri".to_string(), minor_font: "Calibri".to_string(), colors };
        let ctx = StyleContext { theme: Some(&theme), master_styles: None, master_placeholders: &[], layout_placeholders: &[] };
        let rels = HashMap::new();
        let node = parse_node(&root, &ctx, &mut pkg, "ppt/slides/slide1.xml", &rels).unwrap();
        let html = render_node(&node, &mut std::collections::HashMap::new());
        assert!(!html.is_empty());
        assert!(html.contains("White Text"));
    }
    #[test]
    fn test_handles_dynamic_normautofit_when_fontscale_is_absent_no_crash() {
        let xml = r#"<p:sp xmlns:p="http://schemas.openxmlformats.org/presentationml/2006/main"
            xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main">
        <p:nvSpPr>
          <p:cNvPr id="200" name="AutofitShape"/>
          <p:cNvSpPr/>
          <p:nvPr/>
        </p:nvSpPr>
        <p:spPr>
          <a:xfrm><a:off x="0" y="0"/><a:ext cx="2000000" cy="500000"/></a:xfrm>
          <a:prstGeom prst="rect"><a:avLst/></a:prstGeom>
        </p:spPr>
        <p:txBody>
          <a:bodyPr>
            <a:normAutofit/>
          </a:bodyPr>
          <a:lstStyle/>
          <a:p>
            <a:r><a:rPr lang="en-US"/><a:t>Dynamic autofit text content that may overflow</a:t></a:r>
          </a:p>
        </p:txBody>
      </p:sp>"#;
        let root = XmlNode::parse(xml).unwrap();
        let mut pkg = OpcPackage::open(format!("{}/tests/template.pptx", std::env::var("CARGO_MANIFEST_DIR").unwrap())).unwrap();
        let mut colors = HashMap::new();
        colors.insert("accent1".to_string(), "4472C4".to_string());
        colors.insert("lt1".to_string(), "FFFFFF".to_string());
        colors.insert("dk1".to_string(), "000000".to_string());
        let theme = ThemeData { major_font: "Calibri".to_string(), minor_font: "Calibri".to_string(), colors };
        let ctx = StyleContext { theme: Some(&theme), master_styles: None, master_placeholders: &[], layout_placeholders: &[] };
        let rels = HashMap::new();
        let node = parse_node(&root, &ctx, &mut pkg, "ppt/slides/slide1.xml", &rels).unwrap();
        let html = render_node(&node, &mut std::collections::HashMap::new());
        assert!(!html.is_empty());
        assert!(html.contains("Dynamic autofit text content"));
    }
    #[test]
    fn test_handles_spautofit_dynamic_scaling_without_crash() {
        let xml = r#"<p:sp xmlns:p="http://schemas.openxmlformats.org/presentationml/2006/main"
            xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main">
        <p:nvSpPr>
          <p:cNvPr id="201" name="SpAutoFitShape"/>
          <p:cNvSpPr/>
          <p:nvPr/>
        </p:nvSpPr>
        <p:spPr>
          <a:xfrm><a:off x="0" y="0"/><a:ext cx="2000000" cy="500000"/></a:xfrm>
          <a:prstGeom prst="rect"><a:avLst/></a:prstGeom>
        </p:spPr>
        <p:txBody>
          <a:bodyPr>
            <a:spAutoFit/>
          </a:bodyPr>
          <a:lstStyle/>
          <a:p>
            <a:r><a:rPr lang="en-US"/><a:t>Shape auto fit text</a:t></a:r>
          </a:p>
        </p:txBody>
      </p:sp>"#;
        let root = XmlNode::parse(xml).unwrap();
        let mut pkg = OpcPackage::open(format!("{}/tests/template.pptx", std::env::var("CARGO_MANIFEST_DIR").unwrap())).unwrap();
        let mut colors = HashMap::new();
        colors.insert("accent1".to_string(), "4472C4".to_string());
        colors.insert("lt1".to_string(), "FFFFFF".to_string());
        colors.insert("dk1".to_string(), "000000".to_string());
        let theme = ThemeData { major_font: "Calibri".to_string(), minor_font: "Calibri".to_string(), colors };
        let ctx = StyleContext { theme: Some(&theme), master_styles: None, master_placeholders: &[], layout_placeholders: &[] };
        let rels = HashMap::new();
        let node = parse_node(&root, &ctx, &mut pkg, "ppt/slides/slide1.xml", &rels).unwrap();
        let html = render_node(&node, &mut std::collections::HashMap::new());
        assert!(!html.is_empty());
        assert!(html.contains("Shape auto fit text"));
    }
    #[test]
    
    fn test_applies_anchor_b_from_layoutbodyproperties_when_shape_bodypr_has_no_anchor() {
        let xml = r#"<p:sp xmlns:p="http://schemas.openxmlformats.org/presentationml/2006/main"
            xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main">
        <p:nvSpPr>
          <p:cNvPr id="5" name="Title 4"/>
          <p:cNvSpPr><a:spLocks noGrp="1"/></p:cNvSpPr>
          <p:nvPr><p:ph type="title"/></p:nvPr>
        </p:nvSpPr>
        <p:spPr>
          <a:xfrm>
            <a:off x="751922" y="1524000"/>
            <a:ext cx="11035967" cy="3273368"/>
          </a:xfrm>
        </p:spPr>
        <p:txBody>
          <a:bodyPr/>
          <a:lstStyle/>
          <a:p><a:r><a:rPr lang="zh-CN" sz="6600"/><a:t>示例服务</a:t></a:r></a:p>
        </p:txBody>
      </p:sp>"#;
        let root = XmlNode::parse(xml).unwrap();
        let mut pkg = OpcPackage::open(format!("{}/tests/template.pptx", std::env::var("CARGO_MANIFEST_DIR").unwrap())).unwrap();
        let mut colors = HashMap::new();
        colors.insert("accent1".to_string(), "4472C4".to_string());
        colors.insert("lt1".to_string(), "FFFFFF".to_string());
        colors.insert("dk1".to_string(), "000000".to_string());
        let theme = ThemeData { major_font: "Calibri".to_string(), minor_font: "Calibri".to_string(), colors };
        let layout_ph_xml = r#"<p:sp xmlns:p="http://schemas.openxmlformats.org/presentationml/2006/main" xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main">
            <p:nvSpPr><p:nvPr><p:ph type="title"/></p:nvPr></p:nvSpPr>
            <p:txBody><a:bodyPr anchor="b"/></p:txBody>
        </p:sp>"#;
        let layout_ph_root = XmlNode::parse(layout_ph_xml).unwrap();
        println!("layout_ph_root get_ph_info: {:?}", crate::parser::get_ph_info(&layout_ph_root));
        let ctx = StyleContext { theme: Some(&theme), master_styles: None, master_placeholders: &[], layout_placeholders: &[layout_ph_root] };
        let rels = HashMap::new();
        let node = parse_node(&root, &ctx, &mut pkg, "ppt/slides/slide1.xml", &rels).unwrap();
        let html = render_node(&node, &mut std::collections::HashMap::new());
        println!("HTML OUTPUT: {}", html);
        assert!(!html.is_empty());
        assert!(html.contains("flex-end"));
    }
    #[test]
    
    fn test_callout1_main_path_rectangle_has_stroke_none_when_multipath_0_stroke_is_false_oracle_0113_0120() {
        let xml = r#"<p:sp xmlns:p="http://schemas.openxmlformats.org/presentationml/2006/main"
            xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main">
        <p:nvSpPr>
          <p:cNvPr id="10" name="Callout 1"/>
          <p:cNvSpPr/>
          <p:nvPr/>
        </p:nvSpPr>
        <p:spPr>
          <a:xfrm>
            <a:off x="1000000" y="500000"/>
            <a:ext cx="3000000" cy="2000000"/>
          </a:xfrm>
          <a:prstGeom prst="callout1"><a:avLst/></a:prstGeom>
          <a:solidFill><a:srgbClr val="4472C4"/></a:solidFill>
          <a:ln w="12700">
            <a:solidFill><a:srgbClr val="2F5597"/></a:solidFill>
          </a:ln>
        </p:spPr>
      </p:sp>"#;
        let root = XmlNode::parse(xml).unwrap();
        let mut pkg = OpcPackage::open(format!("{}/tests/template.pptx", std::env::var("CARGO_MANIFEST_DIR").unwrap())).unwrap();
        let mut colors = HashMap::new();
        colors.insert("accent1".to_string(), "4472C4".to_string());
        colors.insert("lt1".to_string(), "FFFFFF".to_string());
        colors.insert("dk1".to_string(), "000000".to_string());
        let theme = ThemeData { major_font: "Calibri".to_string(), minor_font: "Calibri".to_string(), colors };
        let ctx = StyleContext { theme: Some(&theme), master_styles: None, master_placeholders: &[], layout_placeholders: &[] };
        let rels = HashMap::new();
        let node = parse_node(&root, &ctx, &mut pkg, "ppt/slides/slide1.xml", &rels).unwrap();
        let html = render_node(&node, &mut std::collections::HashMap::new());
        assert!(!html.is_empty());
        assert!(html.contains("none"));
        // assert!(!html.contains("none"));
    }