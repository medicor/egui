#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
// #![windows_subsystem = "windows"] // Causes stdout to disappear.

#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![deny(clippy::panic)]
#![deny(unused_must_use)]

const GUI_SIZE: egui::Vec2 = egui::Vec2::new(400.0, 390.0);
const ACCENT_COLOR: egui::Color32 = egui::Color32::from_rgb(170, 0, 204);

use chrono::NaiveDate;
use eframe::egui;
use eframe:: { 
    App, 
    Frame
};

mod switch;
mod datefield;

use switch::Switch;
use datefield::Datefield;

#[derive(serde::Deserialize, serde::Serialize, PartialEq, Copy, Clone)]
enum InterfaceSize 
{
    Small,
    Medium,
    Large
}

#[derive(serde::Deserialize, serde::Serialize, PartialEq, Copy, Clone)]
enum InterfaceMode
{
    Dark,
    Light
}

#[derive(serde::Deserialize, serde::Serialize)]
struct Compounder 
{
    start_date: String,
    final_date: String,
    ui_size: InterfaceSize,
    ui_mode: InterfaceMode
}

impl Compounder 
{
    fn new (context: &eframe::CreationContext<'_>) -> Self {
        let cc: Compounder = if let Some(ps) = context.storage { eframe::get_value(ps, eframe::APP_KEY).unwrap_or_default() } else { Default::default() };
        // egui_extras::install_image_loaders(&cc.egui_ctx);
        set_fonts(&context.egui_ctx);
        set_style(&context.egui_ctx, cc.ui_mode);
        cc
    }

    fn resize (&mut self, context: &egui::Context, size: InterfaceSize) {
        if  self.ui_size == size {
            return;
        }
        self.ui_size = size;
        let zf = match size {
            InterfaceSize::Small  => 1.0,
            InterfaceSize::Medium => 1.3,
            InterfaceSize::Large  => 1.7
        };
        // context.set_zoom_factor(zf); // Strange things happen when zoom is set through method.
        context.options_mut(|writer| writer.zoom_factor = zf);
        context.send_viewport_cmd(egui::ViewportCommand::InnerSize(GUI_SIZE)); // Hack to make gui resize.
    }

    fn remode (&mut self, context: &egui::Context, mode: InterfaceMode) {
        if  self.ui_mode == mode {
            return;
        }
        self.ui_mode = mode;
        set_style(context, mode);
    }

    fn get_frame (&mut self) -> egui::Frame {
        let cb = match self.ui_mode {
            InterfaceMode::Dark  => egui::Color32::from_rgb( 20,  20,  20),
            InterfaceMode::Light => egui::Color32::from_rgb(250, 250, 250)
        };
        egui::Frame {
            inner_margin: egui::Margin::same(24.0),
            fill: cb,
            ..Default::default()
        }
    }

}

impl Default for Compounder 
{
    fn default() -> Self {
        let dt = chrono::Local::now().date_naive();
        Self {
            start_date: dt.to_string(),
            final_date: dt.checked_add_months(chrono::Months::new(12)).unwrap_or_default().to_string(),
            ui_size: InterfaceSize::Small,
            ui_mode: InterfaceMode::Dark
        }
    }
}

impl App for Compounder 
{
    fn save (&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    fn update (&mut self, context: &egui::Context, _frame: &mut Frame) {
        egui::CentralPanel::default().frame(self.get_frame()).show(context, |ui| {
            // egui::Image::new (egui::include_image!("../assets/Panel-Background.svg")).paint_at(ui, ui.ctx().screen_rect());
            ui.style_mut().spacing.item_spacing = egui::Vec2::new(16.0, 8.0);
            ui.style_mut().spacing.text_edit_width = 75.0;
            ui.horizontal(|ui| {
                ui.vertical(|ui| {
                    ui.label(egui::RichText::new("START DATE").small().weak());
                    ui.add(Datefield::new(&mut self.start_date));
                    // // if response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                    // // }
                    // if ui.text_edit_singleline(&mut self.editor).highlight().lost_focus() {
                    //     if let Ok(date) = NaiveDate::parse_from_str(&self.editor, "%Y-%m-%d") {
                    //         self.start_date = date;
                    //         self.editor = String::from("");
                    //         // println!("Assigning new date {date}");
                    //     }
                    // };
                    ui.add_space(12.0);
                    ui.label(egui::RichText::new("FINAL DATE").small().weak());
                    let mut ss = self.final_date.to_string();
                    if ui.text_edit_singleline(&mut ss).highlight().lost_focus() {
                        if let Ok(date) = NaiveDate::parse_from_str(&ss, "%Y-%m-%d") {
                            self.final_date = date.to_string();
                        }
                    };
                });
                ui.add_space(36.0);
                ui.vertical(|ui| {
                    let mut yc: u8 = 0;
                    let mut mc: u8 = 0;
                    let mut wc: u8 = 0;
                    let mut dc: u8 = 0;
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
                    let mut iv: String = String::from("0");
                    ui.label(egui::RichText::new("START AMOUNT").small().weak());
                    if ui.text_edit_singleline(&mut iv).highlight().changed() {
                        println!("{iv}")
                    };
                    ui.add_space(12.0);
                    ui.horizontal(|ui| {
                        ui.vertical(|ui| {
                            ui.label(egui::RichText::new("FINAL AMOUNT").small().weak());
                            if ui.text_edit_singleline(&mut iv).highlight().changed() {
                                println!("{iv}")
                            };
                        });
                        ui.label(egui::RichText::new("\n  =  ").strong());
                        ui.vertical(|ui| {
                            ui.label(egui::RichText::new("CAGR").small().weak());
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
                    ui.label(egui::RichText::new("DARK MODE").small().weak());
                    if ui.add(Switch::new(InterfaceMode::Dark == self.ui_mode)).clicked() {
                        match self.ui_mode {
                            InterfaceMode::Dark  => self.remode(ui.ctx(), InterfaceMode::Light),
                            InterfaceMode::Light => self.remode(ui.ctx(), InterfaceMode::Dark)
                        }
                    };
                });
                ui.add_space(12.0);
                ui.vertical(|ui| {
                    ui.label(egui::RichText::new("TEXT SIZE").small().weak());
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

fn set_fonts (context: &egui::Context) {
    let fontname = "Sans Font";
    let mut font = egui::FontDefinitions::default();
    font.font_data.insert(fontname.to_string(), egui::FontData::from_static(include_bytes!("../assets/Inter-Regular.ttf")));
    if let Some(p) = font.families.get_mut(&egui::FontFamily::Proportional) {
        p.insert(0, fontname.to_string());
        context.set_fonts(font);
    };
}

fn set_style (context: &egui::Context, mode: InterfaceMode) {
    let mut vs: egui::Visuals;
    match mode {
        InterfaceMode::Dark  => {
            context.set_theme(egui::Theme::Dark);
            vs = egui::Visuals::dark();
            vs.override_text_color = Option::Some(egui::Color32::from_gray(255));
        },
        InterfaceMode::Light => {
            context.set_theme(egui::Theme::Light);
            vs = egui::Visuals::light();
            vs.override_text_color = Option::Some(egui::Color32::from_gray(0));
        }
    }
    vs.widgets.active.bg_fill = ACCENT_COLOR;
    vs.widgets.noninteractive.bg_fill = ACCENT_COLOR;
    vs.selection.bg_fill = ACCENT_COLOR.gamma_multiply(0.6);
    vs.widgets.hovered.bg_fill = ACCENT_COLOR;//.gamma_multiply(2.0).to_opaque();
    vs.widgets.hovered.weak_bg_fill = ACCENT_COLOR.gamma_multiply(0.1);
    vs.slider_trailing_fill = true;
    context.set_visuals(vs);

}

fn main() -> eframe::Result {
    // let factorial = | n | (1..=n).product::<i32>(); // Nice!
    // println!("{}", factorial(5));
    eframe::run_native(
        "Compounder", 
        eframe::NativeOptions {
            viewport: eframe::egui::ViewportBuilder::default()
                .with_resizable(false)
                .with_maximize_button(false)
                .with_inner_size(GUI_SIZE)
                .with_icon(eframe::icon_data::from_png_bytes(&include_bytes!("../assets/Compounder.png")[..]).unwrap_or_default()),
            ..Default::default()
        },
        Box::new(|context| {
            Ok(Box::new(Compounder::new(context)))
        })
    )
}

/*
function run() {
    // Output
    var daysInAYear = 365.25;
    var oneDay = 24 * 60 * 60 * 1000; // hours*minutes*seconds*milliseconds
    var totalYield = parseInputValue(document.getElementById('totalYield').value / 100);
    var firstDate = new Date(document.getElementById('startDate').value);
    var secondDate = new Date(document.getElementById('endDate').value);
    var diffDays = Math.round(Math.abs((firstDate.getTime() - secondDate.getTime()) / (oneDay)));
    var calcYears = diffDays / daysInAYear;
    var diffYears = Math.floor(diffDays / daysInAYear);
    var remainingMonth = Math.floor(((diffDays / daysInAYear) - diffYears) * 12);
    var monthText = '';
    var aCAGR = Math.pow(1 + (totalYield), 1 / calcYears) - 1;
    var heltal = Math.floor(aCAGR * 100);
    var decimal = -(heltal - (aCAGR * 100));
}*/