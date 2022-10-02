mod measurements;

use crate::measurements::MeasurementWindow;
use eframe::egui;

use std::io::BufRead;
use std::sync::*;
use std::thread;
use tracing::{error, info, warn};

pub struct MonitorApp {
    include_y: Vec<f64>,
    measurements: Arc<Mutex<MeasurementWindow>>,
}

impl MonitorApp {
    fn new(look_behind: usize) -> Self {
        Self {
            measurements: Arc::new(Mutex::new(MeasurementWindow::new_with_look_behind(
                look_behind,
            ))),
            include_y: Vec::new(),
        }
    }
}

impl eframe::App for MonitorApp {
    /// Called by the frame work to save state before shutdown.
    /// Note that you must enable the `persistence` feature for this to work.
    #[cfg(feature = "persistence")]
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let mut plot = egui::plot::Plot::new("measurements");
            for y in self.include_y.iter() {
                plot = plot.include_y(*y);
            }

            plot.show(ui, |plot_ui| {
                plot_ui.line(egui::plot::Line::new(
                    self.measurements.lock().unwrap().plot_values(),
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

    #[clap(short, long)]
    include_y: Vec<f64>,
}

fn main() {
    let args = Args::parse();

    let subscriber = tracing_subscriber::FmtSubscriber::builder()
        .with_max_level(tracing::Level::TRACE)
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    let mut app = MonitorApp::new(args.window_size);

    app.include_y = args.include_y;

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
    eframe::run_native("Monitor app", native_options, Box::new(|_| Box::new(app)));
}
