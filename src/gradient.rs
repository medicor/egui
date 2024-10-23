
use eframe::{
    epaint,
    egui::{
        self, 
        ahash::HashMap, 
        pos2, 
        vec2, 
        Color32, 
        Image, 
        Rect, 
        TextureHandle, 
        TextureOptions, 
        Ui
    }
};

#[derive(Clone, Hash, PartialEq, Eq)]
pub struct Gradient(pub Vec<Color32>);

impl Gradient 
{
    pub fn ground_truth_gamma_gradient(left: Color32, right: Color32) -> Self {
        let n = 255;
        Self(
            (0..=n)
                .map(|i| {
                    let t = i as f32 / n as f32;
                    left.lerp_to_gamma(right, t)
                })
                .collect(),
        )
    }

    pub fn to_pixel_row(&self) -> Vec<Color32> {
        self.0.clone()
    }
}

#[derive(Default)]
struct TextureManager(HashMap<Gradient, TextureHandle>);

impl TextureManager 
{
    fn get(&mut self, ctx: &egui::Context, gradient: &Gradient) -> &TextureHandle {
        self.0.entry(gradient.clone()).or_insert_with(|| {
            let pixels = gradient.to_pixel_row();
            let width = pixels.len();
            let height = 1;
            ctx.load_texture(
                "color_test_gradient",
                epaint::ColorImage {
                    size: [width, height],
                    pixels,
                },
                TextureOptions::LINEAR,
            )
        })
    }
}

pub struct ColorTest 
{
    tex_mngr: TextureManager,
    texture_gradients: bool,
}

impl Default for ColorTest 
{
    fn default() -> Self {
        Self {
            tex_mngr: Default::default(),
            texture_gradients: true,
        }
    }
}

impl ColorTest 
{
    pub fn tex_gradient(&mut self, ui: &mut Ui, bg_fill: Color32, gradient: &Gradient) {
        if !self.texture_gradients {
            return;
        }
        ui.horizontal(|ui| {
            let tex = self.tex_mngr.get(ui.ctx(), gradient);
            let texel_offset = 0.0; //0.5 / (gradient.0.len() as f32);
            let uv = Rect::from_min_max(pos2(texel_offset, 0.0), pos2(1.0 - texel_offset, 1.0));
            ui.add(
                Image::from_texture(
                    (tex.id(), vec2(1024.0, 64.0)))
                    .bg_fill(bg_fill)
                    .uv(uv))
        });
    }
}


/*
// Use inside `run_native` with:
mod gradient;
use gradient::ColorTest;
ui.scope(|ui| {
    ColorTest::default().tex_gradient(
        ui, 
        egui::Color32::from_rgb(255, 255, 200), 
        &gradient::Gradient::ground_truth_gamma_gradient(
            egui::Color32::from_rgb(250, 250, 230), 
            egui::Color32::from_rgb(150, 200, 130)
        )
    );
});
*/
