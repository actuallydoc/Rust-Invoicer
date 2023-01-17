#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use crate::invoicer::Racun;
use eframe;
use eframe::egui;
use egui::{widgets, Color32, Image, TextureHandle, TextureOptions, Ui};
use egui::{RichText, Vec2};
use fs::File;
use std::env;
use std::fmt::{Display, Formatter};
use std::fs::{self, DirEntry};
use std::io::Read;
use std::path::{Path, PathBuf};
const PADDING: f32 = 5.0;
const WHITE: Color32 = Color32::WHITE;
const CYAN: Color32 = Color32::from_rgb(0, 255, 255);
const RED: Color32 = Color32::from_rgb(255, 0, 0);
const GREEN: Color32 = Color32::from_rgb(0, 255, 0);
const BLUE: Color32 = Color32::from_rgb(0, 0, 255);
const YELLOW: Color32 = Color32::from_rgb(255, 255, 0);

#[derive(Default)]
struct GuiApp {
    allowed_to_close: bool,
    show_confirmation_dialog: bool,
    show_image: bool,
    invoice_paths: Vec<DirEntry>,
    json_data: Vec<Racun>,
    texture: Option<egui::TextureHandle>,
    color_image: Option<egui::ColorImage>,
    delete_invoice: bool,
}

trait Data {
    fn get_invoices(&mut self) -> Vec<DirEntry>;
    fn parse_jsons(&mut self);
    fn new() -> Self;
    fn load_image_from_path(
        &mut self,
        path: &std::path::Path,
    ) -> Result<egui::ColorImage, image::ImageError>;
    fn delete_invoice(&mut self, racun: &Racun);
}

impl Data for GuiApp {
    fn new() -> Self {
        let mut this = Self {
            ..Default::default()
        }; // or Self::default()

        this.invoice_paths = this.get_invoices();
        this.parse_jsons();
        // this.json_data = this.parse_jsons();
        this
    }
    fn delete_invoice(&mut self, racun: &Racun) {
        println!("Deleting invoice: {:?}", racun);
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
        folders
    }
    fn parse_jsons(&mut self) {
        let paths = self.get_invoices();
        //Make a vector of invoices
        let mut json_data: Vec<Racun> = Vec::new();
        for path in paths {
            let mut file_path = path.path();
            file_path.push("output.json");
            let mut file_content = match File::open(file_path.to_str().unwrap().to_string()) {
                Ok(file) => file,
                Err(_) => panic!("Could not read the json file"),
            };
            let mut contents = String::new();
            match file_content.read_to_string(&mut contents) {
                Ok(_) => {
                    println!("File contents: {}", contents);
                    let invoice: Racun = match serde_json::from_str(&contents) {
                        Ok(invoice) => invoice,
                        Err(err) => panic!("Could not deserialize the file, error code: {}", err),
                    };
                    json_data.push(invoice);
                }
                Err(err) => panic!("Could not read the json file, error code: {}", err),
            };
        }
        self.json_data = json_data;
        println!("Json data: {:?}", self.json_data)
        // Open the json file
    }

    fn load_image_from_path(
        &mut self,
        path: &std::path::Path,
    ) -> Result<egui::ColorImage, image::ImageError> {
        let path = Path::new(path);
        let image = image::io::Reader::open(path)?.decode()?;
        let size = [image.width() as _, image.height() as _];
        let image_buffer = image.to_rgba8();
        let pixels = image_buffer.as_flat_samples();
        Ok(egui::ColorImage::from_rgba_unmultiplied(
            size,
            pixels.as_slice(),
        ))
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
                    if self.json_data.is_empty() {
                        ui.add(widgets::Spinner::new());
                    } else {
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
                        for invoice in self.json_data.iter_mut() {
                            ui.horizontal(|ui| {
                                ui.label(invoice.invoice.invoice_number.to_string())
                            });
                            ui.label(invoice.invoice.invoice_date.to_string());
                            ui.label(invoice.invoice.service_date.to_string());
                            ui.label(invoice.invoice.due_date.to_string());
                            ui.label(invoice.invoice.partner.partner_name.to_string());
                            ui.label(invoice.invoice.company.company_name.to_string());
                            ui.label(invoice.invoice.status.to_string());
                            for service in &invoice.invoice.services {
                                //Calculate the total price of the invoice
                                let mut total_price = 0.0;
                                total_price += service.service_price + service.service_tax;
                                ui.label(total_price.to_string());
                            }
                            ui.label(invoice.invoice.invoice_currency.to_string());

                            ui.horizontal(|ui| {
                                //When a button is clicked make some actions edit will open the invoice data in another window and u will be able to edit it there
                                //View will open the invoice in a pdf viewer
                                //Delete will delete the invoice
                                if ui.button("View").clicked() {
                                    self.show_image = true;
                                };
                                if ui.button("Edit").clicked() {
                                    //Open the invoice in a new window with its data and allow the user to edit it
                                    //TODO: Implement this
                                };
                                if ui.button("Delete").clicked() {
                                    self.delete_invoice = true;
                                    //Delete the invoice from the gui and from the json file
                                };
                            });

                            ui.end_row();
                        }

                        if self.show_image {
                            egui::Window::new("PDF WINDOW")
                                .collapsible(false)
                                .resizable(false)
                                .show(ctx, |ui| {
                                    ui.horizontal(|ui| {
                                        if ui.button("Close").clicked() {
                                            self.show_image = false;
                                        }
                                    });
                                });
                        } else {
                            self.show_image = false;
                        }
                    }
                });
            });
        });

        if self.show_confirmation_dialog {
            // Show confirmation dialog:
            egui::Window::new("Do you want to quit?")
                .collapsible(true)
                .resizable(true)
                .default_size(Vec2::new(600.0, 500.0))
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
fn show_image_window(ctx: &egui::Context, texture: &egui::TextureHandle) {
    let window = eframe::egui::Window::new("Image");
    window.collapsible(false).resizable(false).show(ctx, |ui| {
        ui.horizontal(|ui| {
            ui.add(egui::Image::new(texture, [1000.0, 800.0]));
        });
    });
}
