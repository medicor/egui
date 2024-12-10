
use chrono::NaiveDate;
use eframe::egui;
use eframe::egui::Widget;

const DATEFORMAT: &str = "%Y-%m-%d";

pub struct Datefield<'a>
{
    proxy: &'a mut String,
    valid: bool
}

impl<'a> Datefield<'a>
{
    pub fn new (value: &'a mut String) -> Self {
        let date = NaiveDate::parse_from_str(value, DATEFORMAT);
        Self {
            proxy: value,
            valid: date.is_ok()
        }
    }

    fn reflect(&mut self, ui: &mut egui::Ui) {
        if !self.valid {
            let visuals = &mut ui.style_mut().visuals;
            visuals.widgets.hovered.bg_stroke = egui::Stroke::new(2.0, visuals.error_fg_color);
            visuals.selection.stroke = egui::Stroke::new(1.0, visuals.error_fg_color);
        }
    }
}

impl Widget for Datefield<'_>
{
    fn ui (mut self, ui: &mut egui::Ui) -> egui::Response {
        ui.vertical(|ui| {
            self.reflect(ui);
            let response = egui::TextEdit::singleline(self.proxy).ui(ui).highlight();
            if  response.lost_focus() || ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                self.valid = NaiveDate::parse_from_str(self.proxy, DATEFORMAT).is_ok();
            };
            response
        }).inner
    }
}
