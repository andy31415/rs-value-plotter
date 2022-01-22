#![forbid(unsafe_code)]
#![warn(clippy::all, rust_2018_idioms)]

mod measurements;

use eframe::{egui, epi};

pub struct MonitorApp {}

impl Default for MonitorApp {
    fn default() -> Self {
        Self {}
    }
}

impl epi::App for MonitorApp {
    fn name(&self) -> &str {
        "Monitor App"
    }

    /// Called once before the first frame.
    fn setup(
        &mut self,
        _ctx: &egui::CtxRef,
        _frame: &epi::Frame,
        _storage: Option<&dyn epi::Storage>,
    ) {
    }

    /// Called by the frame work to save state before shutdown.
    /// Note that you must enable the `persistence` feature for this to work.
    #[cfg(feature = "persistence")]
    fn save(&mut self, storage: &mut dyn epi::Storage) {
        epi::set_value(storage, epi::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::CtxRef, _frame: &epi::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::plot::Plot::new("measurements").show(ui, |plot_ui| {
                let values_iter = (-100..100).map(|x| {
                    let x = (x as f32) / 10.0;
                    egui::plot::Value::new(x, 2. * x * x - 3. * x + 2.)
                });

                plot_ui.line(egui::plot::Line::new(egui::plot::Values::from_values_iter(
                    values_iter,
                )));
            });
        });
    }
}

fn main() {
    let app = MonitorApp::default();
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(Box::new(app), native_options);
}
