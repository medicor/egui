
#![windows_subsystem = "windows"]
#![allow(non_snake_case)]

use std::str::FromStr;
use chrono::NaiveDate;
use eframe::egui;
use eframe:: { 
    App, 
    Frame
};

mod switch;
use switch::toggle;

#[derive(serde::Deserialize, serde::Serialize, PartialEq, Copy, Clone)]
enum InterfaceSize 
{
    Small,
    Medium,
    Large
}

impl InterfaceSize 
{
}

#[derive(serde::Deserialize, serde::Serialize)]
struct Compounder 
{
    start_date: NaiveDate,
    final_date: NaiveDate,
    ui_size: InterfaceSize,
    ui_mode: bool
}

impl Compounder 
{
    fn new (cc: &eframe::CreationContext<'_>) -> Self {
        let mut fd = egui::FontDefinitions::default();
        fd.font_data.insert("Inter Medium".to_owned(), egui::FontData::from_static(include_bytes!("../assets/Inter-Medium.ttf")));
        fd.families.get_mut(&egui::FontFamily::Proportional).unwrap().insert(0, "Inter Medium".to_owned());
        cc.egui_ctx.set_fonts(fd);
        // egui_extras::install_image_loaders(&cc.egui_ctx);
        if let Some(ps) = cc.storage {
            return eframe::get_value(ps, eframe::APP_KEY).unwrap_or_default();
        }
        Default::default()
    }

    fn resize (&mut self, context: &egui::Context, size: InterfaceSize) {
        if  self.ui_size == size {
            return;
        }
        self.ui_size = size;
        let (zf, ws) = match size {
            InterfaceSize::Small  => (1.0, 390.0),
            InterfaceSize::Medium => (1.2, 395.0),
            InterfaceSize::Large  => (1.5, 400.0)
        };
        // context.set_zoom_factor(zf); // Strange things happen when zoom is set through method.
        context.options_mut(|writer| writer.zoom_factor = zf);
        context.send_viewport_cmd(egui::ViewportCommand::InnerSize(egui::Vec2::new(ws, ws)));
    }

}

impl Default for Compounder 
{
    fn default() -> Self {
        let today = chrono::Local::now().date_naive();
        Self {
            start_date: today, //NaiveDate::from_ymd_opt(2024,  8, 31).unwrap(),
            final_date: today,
            ui_size: InterfaceSize::Small,
            ui_mode: false
        }
    }
}

impl App for Compounder 
{
    fn save (&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    fn update (&mut self, context: &egui::Context, _frame: &mut Frame) {
        egui::CentralPanel::default().frame(egui::Frame::default().inner_margin(24.0)).show (context, |ui| {
            let mut yc: u8 = 0;
            let mut mc: u8 = 0;
            let mut wc: u8 = 0;
            let mut dc: u8 = 0;
            let mut ss: String = self.start_date.to_string();
            let mut iv: String = String::from("0");
            
            // egui::Image::new (egui::include_image!("../assets/Panel-Background.svg")).paint_at(ui, ui.ctx().screen_rect());
            ui.style_mut().spacing.item_spacing = egui::Vec2::new(16.0, 8.0);
            ui.style_mut().spacing.text_edit_width = 75.0;
            ui.horizontal(|ui| {
                ui.vertical(|ui| {
                    ui.label("Start date:");
                    if ui.text_edit_singleline(&mut ss).highlight().changed() {
                        self.start_date = NaiveDate::from_str(&ss).unwrap();
                        println!("{ss}")
                    };
                    ui.add_space(12.0);
                    ui.label("Final date:");
                    if ui.text_edit_singleline(&mut ss).highlight().changed() {
                        self.final_date = NaiveDate::from_str(&ss).unwrap();
                        println!("{ss}")
                    };
                });
                ui.add_space(36.0);
                ui.vertical(|ui| {
                    ui.add_space(12.0);
                    ui.add(egui::Slider::new(&mut yc, 0..=25).text("years"));
                    ui.add(egui::Slider::new(&mut mc, 0..=11).text("months"));
                    ui.add(egui::Slider::new(&mut wc, 0..=51).text("weeks"));
                    ui.add(egui::Slider::new(&mut dc, 0..=30).text("days"));
                });
            });
            ui.add_space(12.0);
            ui.separator();
            ui.add_space(12.0);
            ui.horizontal(|ui| {
                ui.vertical(|ui| {
                    ui.label("Start amount:");
                    if ui.text_edit_singleline(&mut iv).highlight().changed() {
                        println!("{iv}")
                    };
                    ui.add_space(12.0);
                    ui.horizontal(|ui| {
                        ui.vertical(|ui| {
                            ui.label("Final amount:");
                            if ui.text_edit_singleline(&mut iv).highlight().changed() {
                                println!("{iv}")
                            };
                        });
                        // ui.add_space(12.0);
                        ui.label("\n  =  ");
                        ui.vertical(|ui| {
                            ui.label("CAGR:");
                            if ui.text_edit_singleline(&mut iv).highlight().changed() {
                                println!("{iv}")
                            };
                        });
                    });
                });
            });
            ui.add_space(12.0);
            ui.separator();
            ui.add_space(12.0);
            ui.horizontal(|ui| {
                ui.vertical(|ui| {
                    ui.label("Light mode:");
                    if ui.add(toggle(&mut self.ui_mode)).changed() {

                    };
                });
                ui.add_space(12.0);
                ui.vertical(|ui| {
                    ui.label("Text size:");
                    ui.horizontal(|ui| {
                        if ui.selectable_label(self.ui_size == InterfaceSize::Small,  "small" ).highlight().clicked() {
                            self.resize(ui.ctx(), InterfaceSize::Small);
                        };
                        if ui.selectable_label(self.ui_size == InterfaceSize::Medium, "medium").highlight().clicked() {
                            self.resize(ui.ctx(), InterfaceSize::Medium);
                        };
                        if ui.selectable_label(self.ui_size == InterfaceSize::Large,  "large" ).highlight().clicked() {
                            self.resize(ui.ctx(), InterfaceSize::Large);
                        };
                    });
                });
            });
        });
    }
}

fn main() -> eframe::Result {
    eframe::run_native (
        "Compounder", 
        eframe::NativeOptions {
            viewport: eframe::egui::ViewportBuilder::default()
                .with_resizable(false)
                .with_maximize_button(false)
                .with_inner_size([390.0, 390.0])
                .with_icon(eframe::icon_data::from_png_bytes(&include_bytes!("../assets/Compounder.png")[..]).unwrap_or_default()),
            ..Default::default()
        },
        Box::new(|cc| Ok(Box::new(Compounder::new(cc))))
    )
}
