#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use crate::invoicer::Invoice;
use eframe;
use eframe::egui;
use egui::RichText;
use egui::{widgets, Color32};
use std::env;
use std::fmt::Debug;
use std::fs::{self, DirEntry};

use std::path::PathBuf;
//Create a gui struct

const PADDING: f32 = 5.0;
const WHITE: Color32 = Color32::WHITE;
const CYAN: Color32 = Color32::from_rgb(0, 255, 255);
const RED: Color32 = Color32::from_rgb(255, 0, 0);
const GREEN: Color32 = Color32::from_rgb(0, 255, 0);
const BLUE: Color32 = Color32::from_rgb(0, 0, 255);
const YELLOW: Color32 = Color32::from_rgb(255, 255, 0);

#[derive(Debug, Default)]
struct GuiApp {
    allowed_to_close: bool,
    show_confirmation_dialog: bool,
    invoice_paths: Vec<DirEntry>,
    json_data: Vec<Invoice>,
}

trait Data {
    fn get_invoices(&mut self) -> Vec<DirEntry>;
    fn parse_jsons(&mut self) -> Vec<Invoice>;
    fn new() -> Self;
}

impl Data for GuiApp {
    fn new() -> Self {
        let mut this = Self {
            ..Default::default()
        }; // or Self::default()
        this.invoice_paths = this.get_invoices();
        this.json_data = this.parse_jsons();
        this
    }

    fn get_invoices(&mut self) -> Vec<DirEntry> {
        let mut path = env::current_dir().unwrap();
        let invoice_folder = PathBuf::from("invoices");
        path.push(&invoice_folder);
        println!("Path: {:?}", path);
        print!("{}", invoice_folder.display());

        let folders: Vec<DirEntry> = fs::read_dir(path)
            .unwrap()
            .filter_map(|entry| entry.ok())
            .filter(|entry| entry.file_type().unwrap().is_dir())
            .collect();
        //println!("Data sent: {:?}", folders);
        folders
    }
    #[allow(unused_mut)]
    fn parse_jsons(&mut self) -> Vec<Invoice> {
        let paths = self.get_invoices();
        //Make a vector of invoices
        let mut json_data: Vec<Invoice> = Vec::new();
        for path in paths {
            let mut _path = path.path();
            _path.push("output.json");
            println!("Path: {:?}", _path);

            let readen = fs::read_to_string(_path).expect("Unable to read file");
            println!("Readen: {:?}", readen);
            // let data = fs::read_to_string(cwd).expect("Unable to read file");
            // let parsed: Invoice =
            //     serde_json::from_str(&data).expect("JSON does not have correct format.");

            // println!("Parsed data: {:?}", parsed);
            // json_data.push(parsed);
        }
        println!("Parsed json data: {:?}", json_data);
        json_data
        //Vec::new()
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
                egui::Grid::new("invoice_grid").show(ui, |ui| {
                    //fetch the invoices and display them
                    ui.horizontal(|ui| ui.colored_label(WHITE, "Invoice number"));
                    ui.colored_label(WHITE, "Invoice Date");
                    ui.colored_label(WHITE, "Service Date");
                    ui.colored_label(WHITE, "Due Date");
                    ui.colored_label(WHITE, "Partner");
                    ui.colored_label(WHITE, "Provider");
                    ui.colored_label(WHITE, "Status");
                    ui.colored_label(WHITE, "Amount");
                    ui.colored_label(WHITE, "Currency");
                    ui.colored_label(WHITE, "Actions");
                    ui.end_row();

                    for invoice in self.json_data.clone() {
                        ui.colored_label(WHITE, format!("{}", &invoice.invoice_number));
                        ui.colored_label(WHITE, &invoice.invoice_date);
                        ui.colored_label(WHITE, &invoice.service_date);
                        ui.colored_label(WHITE, &invoice.due_date);
                        ui.colored_label(WHITE, &invoice.partner.partner_name);
                        ui.colored_label(WHITE, &invoice.company.company_name);
                        ui.colored_label(WHITE, &invoice.invoice_currency);
                        for service in invoice.services {
                            let amount = service.service_price;
                            ui.colored_label(WHITE, format!("{}", amount));
                        }
                        ui.colored_label(WHITE, invoice.invoice_currency);
                        ui.colored_label(WHITE, "Actions");
                        ui.end_row();
                    }
                    //ui.add_space(10.0);
                    ui.colored_label(WHITE, format!("{:?}", self.invoice_paths));
                    ui.colored_label(WHITE, format!("{:?}", self.json_data));
                });
            });
            //Loading animation spinner very useful.
            //ui.add(widgets::Spinner::new());
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
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(900.0, 700.0)),
        ..Default::default()
    };
    let app = GuiApp::new();

    eframe::run_native(
        "Invoice GUI",
        options.clone(),
        Box::new(|_cc| Box::new(app)),
    );
}
