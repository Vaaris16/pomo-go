use std::sync::Arc;

use egui::{Context, FontData, FontDefinitions, FontFamily};

pub fn setup_fonts(ctx: &Context) {
    let mut fonts = FontDefinitions::default();

    fonts.font_data.insert(
        "inter-bold".to_owned(),
        Arc::new(FontData::from_static(include_bytes!(
            "../../../assets/fonts/inter/static/Inter_28pt-Bold.ttf"
        ))),
    );

    fonts.families.insert(
        FontFamily::Name("Inter-Bold".into()),
        vec!["inter-bold".to_owned()],
    );
    ctx.set_fonts(fonts);
}
