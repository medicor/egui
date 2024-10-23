
#![windows_subsystem = "windows"]
#![allow(non_snake_case)]

use chrono::NaiveDate;
use eframe::egui;
use eframe:: { 
    App, 
    Frame
};

mod switch;
use switch::toggle;


#[derive(serde::Deserialize, serde::Serialize)]
struct Compounder 
{
    start_date: NaiveDate,
    final_date: NaiveDate,
    mode: bool
}

impl Default for Compounder {
    fn default() -> Self {
        Self {
            start_date: NaiveDate::from_ymd_opt(2024,  8, 31).unwrap(),
            final_date: NaiveDate::from_ymd_opt(2024, 10, 14).unwrap(),
            mode: false
        }
    }
}

impl Compounder {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
//      cc.egui_ctx.set_theme(egui::Theme::Dark);
        //eframe::WindowAttributes::with_window_icon(eframe::icon_data::from_png_bytes(&include_bytes!("../assets/face-stylized.png")[..]));
        let mut fd = egui::FontDefinitions::default();
        fd.font_data.insert("Inter Medium".to_owned(), egui::FontData::from_static(include_bytes!("../assets/Inter-Medium.ttf")));
        fd.families.get_mut(&egui::FontFamily::Proportional).unwrap().insert(0, "Inter Medium".to_owned());
//      fd.families.get_mut(&FontFamily::Monospace).unwrap().push("Inter Medium".to_owned());
        cc.egui_ctx.set_fonts(fd);
        cc.egui_ctx.set_zoom_factor(1.2);
        egui_extras::install_image_loaders(&cc.egui_ctx);
        if let Some(ps) = cc.storage {
            return eframe::get_value(ps, eframe::APP_KEY).unwrap_or_default();
        }
        Default::default()
    }
}

impl App for Compounder {
    fn update(&mut self, context: &egui::Context, _frame: &mut Frame) {
        egui::CentralPanel::default().show (context, |ui| {
            let mut yc: u8 = 0;
            let mut mc: u8 = 0;
            let mut wc: u8 = 0;
            let mut dc: u8 = 0;

            // egui::Image::new (egui::include_image!("../assets/Panel-Background.svg")).paint_at(ui, ui.ctx().screen_rect());
            // egui::widgets::global_theme_preference_switch(ui);
            ui.style_mut().spacing.slider_width = 640.0;

            ui.label(format!("Number of days (final_date-start_date) = {}", (self.final_date-self.start_date).num_days()));
            ui.label(format!("Year = {}", yc));
            ui.add(toggle(&mut self.mode));
            ui.add(egui::Slider::new(&mut yc, 0..=25).text("years"));
            ui.add(egui::Slider::new(&mut mc, 0..=11).text("months"));
            ui.add(egui::Slider::new(&mut wc, 0..=51).text("weeks"));
            ui.add(egui::Slider::new(&mut dc, 0..=30).text("days"));
        });
    }
}

fn main() -> eframe::Result {
    //eframe::set_app_icon_windows();
    eframe::run_native (
        "Compounder", 
        eframe::NativeOptions {
            viewport: eframe::egui::ViewportBuilder::default()
                .with_inner_size([640.0, 360.0])
                .with_min_inner_size([320.0, 180.0])
                .with_max_inner_size([1280.0, 720.0])
                .with_icon(eframe::icon_data::from_png_bytes(&include_bytes!("../assets/Compounder.png")[..]).unwrap_or_default()),
            ..Default::default()
        },
        Box::new(|cc| Ok(Box::new(Compounder::new(cc))))
    )
}
