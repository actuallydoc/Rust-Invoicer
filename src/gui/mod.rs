#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use eframe;
use eframe::egui;
use egui::style::Widgets;
use egui::{
    lerp, vec2, widgets, Align, Color32, Layout, Pos2, Response, Sense, Shape, Stroke, TextBuffer,
    Ui,
};
use egui::{RichText, Widget};
use std::fs;
use std::ops::RangeInclusive;
use std::path::PathBuf;
//Create a gui struct

const PADDING: f32 = 5.0;
const WHITE: Color32 = Color32::WHITE;
const CYAN: Color32 = Color32::from_rgb(0, 255, 255);
const RED: Color32 = Color32::from_rgb(255, 0, 0);
const GREEN: Color32 = Color32::from_rgb(0, 255, 0);
const BLUE: Color32 = Color32::from_rgb(0, 0, 255);
const YELLOW: Color32 = Color32::from_rgb(255, 255, 0);

struct GuiApp {
    allowed_to_close: bool,
    show_confirmation_dialog: bool,
    invoice_number: String,
}
impl GuiApp {
    fn new() -> Self {
        //Init all the values that the gui will hold. probably the invoice data , statuses and other important data
        Self {
            allowed_to_close: false,
            show_confirmation_dialog: false,
            invoice_number: String::new(),
        }
    }
}
// #[derive(Default)]
// struct CustomWidget {
//     size: Option<f32>,
//     color: Option<Color32>,
// }

// impl CustomWidget {
//     pub fn new() -> Self {
//         Self::default()
//     }
//     pub fn color(mut self, color: impl Into<Color32>) -> Self {
//         self.color = Some(color.into());
//         self
//     }
// }
//Custom
//impl Widget for CustomWidget {
// fn ui(self, ui: &mut Ui) -> Response {
//     let size = self
//         .size
//         .unwrap_or_else(|| ui.style().spacing.interact_size.y);
//     let color = self
//         .color
//         .unwrap_or_else(|| ui.visuals().strong_text_color());
//     let (rect, response) = ui.allocate_exact_size(vec2(size, size), Sense::hover());

//     if ui.is_rect_visible(rect) {
//         ui.ctx().request_repaint();

//         let radius = (rect.height() / 1.0) - 2.0;
//         let n_points = 20;
//         let start_angle = ui.input().time * std::f64::consts::TAU;
//         let end_angle = start_angle + 240f64.to_radians() * ui.input().time.sin();
//         let points: Vec<Pos2> = (0..n_points)
//             .map(|i| {
//                 let angle = lerp(start_angle..=end_angle, i as f64 / n_points as f64);
//                 let (sin, cos) = angle.sin_cos();
//                 rect.center() + radius * vec2(cos as f32, sin as f32)
//             })
//             .collect();
//         ui.painter()
//             .add(Shape::line(points, Stroke::new(3.0, color)));
//     }

//     response
// }

impl eframe::App for GuiApp {
    fn on_close_event(&mut self) -> bool {
        self.show_confirmation_dialog = true;
        self.allowed_to_close
    }
    fn on_exit(&mut self, _gl: Option<&eframe::glow::Context>) {
        // Save settings here
    }

    fn save(&mut self, _storage: &mut dyn eframe::Storage) {}
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::new([false, false]).show(ui, |ui| {
                ui.label("Project repo:");
                ui.add(widgets::Hyperlink::new("https://github.com/actuallydoc"));

                ui.add_space(PADDING);
                ui.colored_label(
                    CYAN,
                    RichText::new(format!("This is a simple invoice manager written in Rust")),
                );
                ui.add_space(10.0);
                ui.horizontal(|ui| {
                    ui.colored_label(WHITE, "Invoice Number");
                    ui.add_space(10.0);

                    ui.colored_label(WHITE, "Invoice Date");
                    ui.add_space(10.0);
                    ui.colored_label(WHITE, "Service Date");
                    ui.add_space(10.0);
                    ui.colored_label(WHITE, "Due Date");
                    ui.add_space(10.0);
                    ui.colored_label(WHITE, "Partner");
                    ui.add_space(10.0);
                    ui.colored_label(WHITE, "Provider");
                    ui.add_space(10.0);
                    ui.colored_label(WHITE, "Status");
                    ui.add_space(10.0);
                    ui.colored_label(WHITE, "Amount");
                    ui.add_space(10.0);
                    ui.colored_label(WHITE, "Currency");
                });
                ui.add(widgets::Separator::default());
                ui.add_space(PADDING);

                ui.horizontal(|ui| {
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        ui.label("world!");
                        ui.label("Hello");
                    });
                    ui.colored_label(WHITE, "21");
                    ui.add_space(PADDING);
                    ui.colored_label(WHITE, "08.2.2022");
                    ui.add_space(10.0);
                    ui.colored_label(WHITE, "08.2.2022");
                    ui.add_space(10.0);
                    ui.colored_label(WHITE, "Due Date");
                    ui.add_space(10.0);
                    ui.colored_label(WHITE, "Partner");
                    ui.add_space(10.0);
                    ui.colored_label(WHITE, "Provider");
                    ui.add_space(10.0);
                    ui.colored_label(WHITE, "Status");
                    ui.add_space(10.0);
                    ui.colored_label(WHITE, "Amount");
                    ui.add_space(10.0);
                    ui.colored_label(WHITE, "Currency");

                    //Loading animation spinner very useful.
                    //ui.add(widgets::Spinner::new());
                });
            })
        });

        if self.show_confirmation_dialog {
            // Show confirmation dialog:
            egui::Window::new("Do you want to quit?")
                .collapsible(false)
                .resizable(false)
                .show(ctx, |ui| {
                    ui.horizontal(|ui| {
                        if ui.button("Cancel").clicked() {
                            self.show_confirmation_dialog = false;
                        }

                        if ui.button("Yes!").clicked() {
                            self.allowed_to_close = true;
                            frame.close();
                        }
                    });
                });
        }
    }
}

pub fn entry() {
    let mut options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(320.0, 240.0)),
        ..Default::default()
    };

    options.initial_window_size = Some(egui::vec2(900.0, 700.0));
    eframe::run_native(
        "Invoice GUI",
        options.clone(),
        Box::new(|_cc| Box::new(GuiApp::new())),
    );
}

// fn scan_thread(tx: Sender<Vec<DirEntry>>) {
//     println!("Thread started");
//     thread::spawn(move || {
//         let mut path = env::current_dir().unwrap();
//         let invoice_folder = PathBuf::from("invoices");
//         path.push(invoice_folder);

//         let folders: Vec<DirEntry> = fs::read_dir(path)
//             .unwrap()
//             .filter_map(|entry| entry.ok())
//             .filter(|entry| entry.file_type().unwrap().is_dir())
//             .collect();
//         println!("Data sent: {:?}", folders);
//         tx.send(folders).unwrap();
//     });
// }
