use eframe::egui;


pub fn setup_custom_fonts(ctx: &egui::Context) {
    // Start with the default fonts (we will be adding to them rather than replacing them).
    let mut fonts = egui::FontDefinitions::default();

    // Install my own font (maybe supporting non-latin characters).
    // .ttf and .otf files supported.
    fonts.font_data.insert(
        "LilitaOne-Regular".to_owned(),
        egui::FontData::from_static(include_bytes!(
            "../../assets/fonts/Lilita_One/LilitaOne-Regular.ttf"
        )),
    );

    // Put my font first (highest priority) for proportional text:

    // Put my font as last fallback for monospace:
    fonts
        .families
        .entry(egui::FontFamily::Monospace)
        .or_default()
        .insert(0, "LilitaOne-Regular".to_owned());

    // Tell egui to use these fonts:
    ctx.set_fonts(fonts);
}