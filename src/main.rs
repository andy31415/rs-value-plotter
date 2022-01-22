mod measurements;

use crate::measurements::MeasurementWindow;
use eframe::{egui, epi};

use std::io::BufRead;
use std::sync::*;
use std::thread;
use tracing::{error, info, warn};

pub struct MonitorApp {
    measurements: Arc<Mutex<MeasurementWindow>>,
}

impl MonitorApp {
    fn new(look_behind: usize) -> Self {
        Self {
            measurements: Arc::new(Mutex::new(MeasurementWindow::new_with_look_behind(
                look_behind,
            ))),
        }
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
                plot_ui.line(egui::plot::Line::new(
                    self.measurements.lock().unwrap().into_plot_values(),
                ));
            });
        });
        // make it always repaint. TODO: can we slow down here?
        ctx.request_repaint();
    }
}

use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[clap(author, version, about)]
struct Args {
    /// Name of the person to greet
    #[clap(short, long, default_value_t = 1000)]
    window_size: usize,
}

fn main() {
    let args = Args::parse();

    let subscriber = tracing_subscriber::FmtSubscriber::builder()
        .with_max_level(tracing::Level::TRACE)
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    let app = MonitorApp::new(args.window_size);
    let native_options = eframe::NativeOptions::default();

    let monitor_ref = app.measurements.clone();

    thread::spawn(move || {
        let stdin = std::io::stdin();
        for line in stdin.lock().lines() {
            match line {
                Ok(s) => {
                    let parts = s.split(' ').collect::<Vec<&str>>();
                    if parts.len() != 2 {
                        warn!("Need exactly two parts: {}", s);
                        continue;
                    }

                    let x = parts.first().expect("Have 2 parts");
                    let y = parts.last().expect("Have 2 parts");

                    let x = match x.parse::<f64>() {
                        Ok(value) => value,
                        _ => {
                            warn!("Failed to parse {}", x);
                            continue;
                        }
                    };

                    let y = match y.parse::<f64>() {
                        Ok(value) => value,
                        _ => {
                            warn!("Failed to parse {}", x);
                            continue;
                        }
                    };

                    monitor_ref
                        .lock()
                        .unwrap()
                        .add(measurements::Measurement::new(x, y));
                }
                _ => {
                    error!("Failed to read line");
                    break;
                }
            }
        }
    });

    info!("Main thread started");
    eframe::run_native(Box::new(app), native_options);
}
