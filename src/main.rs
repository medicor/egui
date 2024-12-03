#![windows_subsystem = "windows"]

const GUI_SIZE: egui::Vec2 = egui::Vec2::new(400.0, 400.0);
const ACCENT_COLOR: egui::Color32 = egui::Color32::from_rgb(170, 0, 204);

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

#[derive(serde::Deserialize, serde::Serialize, PartialEq, Copy, Clone)]
enum InterfaceMode
{
    Dark,
    Light
}

#[derive(serde::Deserialize, serde::Serialize)]
struct Compounder 
{
    start_date: NaiveDate,
    final_date: NaiveDate,
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
            InterfaceSize::Small  => 1.1,
            InterfaceSize::Medium => 1.3,
            InterfaceSize::Large  => 1.6
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
            start_date: dt, //NaiveDate::from_ymd_opt(2024,  8, 31).unwrap(),
            final_date: dt,
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
        dbg!("update");
        egui::CentralPanel::default().frame(self.get_frame()).show(context, |ui| {
            // egui::Image::new (egui::include_image!("../assets/Panel-Background.svg")).paint_at(ui, ui.ctx().screen_rect());
            // egui::widgets::global_theme_preference_buttons(ui);
            ui.style_mut().spacing.item_spacing = egui::Vec2::new(16.0, 8.0);
            ui.style_mut().spacing.text_edit_width = 75.0;
            ui.horizontal(|ui| {
                ui.vertical(|ui| {
                    let mut ss: String = self.start_date.to_string();
                    ui.label(egui::RichText::new("START DATE").small().weak());
                    if ui.text_edit_singleline(&mut ss).highlight().changed() {
                        self.start_date = NaiveDate::from_str(&ss).unwrap();
                        println!("{ss}")
                    };
                    ui.add_space(12.0);
                    ui.label(egui::RichText::new("FINAL DATE").small().weak());
                    if ui.text_edit_singleline(&mut ss).highlight().changed() {
                        self.final_date = NaiveDate::from_str(&ss).unwrap();
                        println!("{ss}")
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
                    let mut im: bool = InterfaceMode::Dark == self.ui_mode;
                    if ui.add(toggle(&mut im)).clicked() {
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
    let mut fd = egui::FontDefinitions::default();
    fd.font_data.insert("Sans Font".to_owned(), egui::FontData::from_static(include_bytes!("../assets/Inter-Regular.ttf")));
    fd.families.get_mut(&egui::FontFamily::Proportional).unwrap().insert(0, "Sans Font".to_owned());
    context.set_fonts(fd);
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
    vs.widgets.hovered.bg_fill = ACCENT_COLOR.gamma_multiply(0.2);
    vs.widgets.hovered.weak_bg_fill = ACCENT_COLOR.gamma_multiply(0.1);
    context.set_visuals(vs);

}

fn main() -> eframe::Result {
    let factorial = | n | (1..=n).product::<i32>(); // Nice!
    println!("{}", factorial(5));
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
