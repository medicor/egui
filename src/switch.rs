
use eframe::egui;

pub fn switch (ui: &mut egui::Ui, state: &mut bool) -> egui::Response {
    let size = ui.spacing().interact_size.y * egui::vec2(2.5, 1.0);
    let (area, mut response) = ui.allocate_exact_size(size, egui::Sense::click());
    if response.clicked() {
        *state = !*state;
        response.mark_changed(); // report back that the value changed
    }
    if ui.is_rect_visible(area) {
        let easing  = ui.ctx().animate_bool_responsive(response.id, *state);
        let area    = area.expand(ui.style().interact_selectable(&response, *state).expansion);
        let radius  = 0.5 * area.height();
        let visuals = &ui.visuals().widgets;
        let widgets = ui.style().interact(&response);
        ui.painter().rect(area, radius, widgets.bg_fill, widgets.bg_stroke); // Paint "slider" beneath.
        let circle  = egui::lerp((area.left() + radius - 2.0)..=(area.right() - radius), easing);
        let center  = egui::pos2(circle, area.center().y);
        ui.painter().circle(center, 0.8 * radius, widgets.bg_fill, widgets.fg_stroke); // Paint "knob" above.
    }
    response
}

pub fn toggle (state: &mut bool) -> impl egui::Widget + '_ {
    move |ui: &mut egui::Ui| switch(ui, state)
}
