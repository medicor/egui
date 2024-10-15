
#![windows_subsystem = "windows"]
#![allow(non_snake_case)]

use chrono::NaiveDate;
use eframe::egui;
use eframe::egui::CentralPanel;
use eframe:: { 
    App, 
    Frame
};

#[derive(serde::Deserialize, serde::Serialize)]
struct Compounder 
{
    start_date: NaiveDate,
    final_date: NaiveDate
}

impl Default for Compounder {
    fn default() -> Self {
        Self {
            start_date: NaiveDate::from_ymd_opt(2024,  8, 31).unwrap(),
            final_date: NaiveDate::from_ymd_opt(2024, 10, 14).unwrap()
        }
    }
}

impl Compounder {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // cc.egui_ctx.set_visuals(egui::Visuals { panel_fill: egui::Color32::RED, ..Default::default() });
        cc.egui_ctx.set_visuals(egui::Visuals::dark()); //TODO: Not working!!!!
        cc.egui_ctx.set_zoom_factor(1.2);
        if let Some(ps) = cc.storage {
            return eframe::get_value(ps, eframe::APP_KEY).unwrap_or_default();
        }
        Default::default()
    }
}

impl App for Compounder {
    fn update(&mut self, context: &egui::Context, _frame: &mut Frame) {
        CentralPanel::default().show (context, |ui| {
            ui.label(format!("Number of days = {}", (self.final_date-self.start_date).num_days()));
        });
    }
}

fn main() -> eframe::Result {
    eframe::run_native (
        "Compounder", 
        eframe::NativeOptions {
            viewport: eframe::egui::ViewportBuilder::default()
                .with_inner_size([400.0, 300.0])
                .with_min_inner_size([300.0, 220.0]),
                ..Default::default()
        },
        Box::new(|cc| Ok(Box::new(Compounder::new(cc))))
    )
}
