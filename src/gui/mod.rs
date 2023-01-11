#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use eframe;
use eframe::egui;
use egui::style::Widgets;
use egui::RichText;
use egui::{widgets, Color32};
use std::env;
use std::fs;
use std::fs::DirEntry;
use std::path::PathBuf;
use std::sync::mpsc::channel;
use std::sync::mpsc::Receiver;
use std::sync::Arc;
use std::thread;
use std::time::Duration;
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
    invoices: Receiver<Vec<DirEntry>>,
}
//Add a table view with existing invoices

fn scan_invoices() -> Vec<DirEntry> {
    let mut path = env::current_dir().unwrap();
    let invoice_folder = PathBuf::from("invoices");
    path.push(invoice_folder);

    let folders: Vec<DirEntry> = fs::read_dir(path)
        .unwrap()
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.file_type().unwrap().is_dir())
        .collect();
    folders
}

impl GuiApp {
    fn new(scanned_dirs: Receiver<Vec<DirEntry>>) -> Self {
        //Init all the values that the gui will hold. probably the invoice data , statuses and other important data
        Self {
            allowed_to_close: false,
            show_confirmation_dialog: false,
            invoices: scanned_dirs,
        }
    }
}

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
        let mut scanned = self.invoices.try_recv();
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::new([false, false]).show(ui, |ui| {
                ui.label("Project repo:");
                ui.add(widgets::Hyperlink::new("https://github.com/actuallydoc"));

                ui.add_space(PADDING);
                ui.colored_label(
                    CYAN,
                    RichText::new(format!("This is a simple invoice manager written in Rust"))
                        .monospace(),
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
                    let scanned_dirs = self.invoices.try_recv();
                    println!("{:?}", scanned_dirs);
                    ui.add(widgets::Label::new("XD"));
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
    let (tx, rx) = channel();
    std::thread::spawn(move || loop {
        let scanned_dirs = scan_invoices();

        tx.send(scanned_dirs).unwrap();
        println!("Sending scanned dirs");
        thread::sleep(Duration::from_secs(5));
    });

    let mut options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(320.0, 240.0)),
        ..Default::default()
    };

    options.initial_window_size = Some(egui::vec2(900.0, 700.0));
    eframe::run_native(
        "Invoice GUI",
        options.clone(),
        Box::new(|_cc| Box::new(GuiApp::new(rx))),
    );
}
