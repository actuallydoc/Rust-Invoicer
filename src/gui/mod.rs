#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use crate::invoicer::{Racun, init, Invoice, InvoiceStructure, FontSizes, Service, Company, Partner};
use eframe;
use eframe::egui;
use egui::{widgets, Color32, TextureHandle};
use egui::{RichText, Vec2};
use rand::Rng;
use fs::File;
use std::env;
use std::fmt::format;
use std::fs::{self, DirEntry};
use std::io::Read;
use std::path::PathBuf;
const PADDING: f32 = 5.0;
const WHITE: Color32 = Color32::WHITE;
const CYAN: Color32 = Color32::from_rgb(0, 255, 255);


#[derive(Default)]
struct GuiApp {
    allowed_to_close: bool,
    show_confirmation_dialog: bool,
    show_image: bool,
    invoice_paths: Vec<DirEntry>,
    json_data: Vec<Racun>,
    delete_invoice: bool,
    clicked_pdf_path: PathBuf,
    texture: Option<TextureHandle>,
    refresh: bool,
}

trait Data {
    fn get_invoices(&mut self) -> Vec<DirEntry>;
    fn parse_jsons(&mut self);
    fn new() -> Self;

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
        if self.refresh {
            self.invoice_paths = self.get_invoices();
            self.parse_jsons();
            self.refresh = false;
        }
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::new([false, false]).show(ui, |ui| {
                ui.label("Project repo:");
                ui.add(widgets::Hyperlink::new("https://github.com/actuallydoc"));
                ui.add_space(PADDING);
                ui.colored_label(
                    CYAN,
                    RichText::new(format!("This is a simple invoice manager written in Rust")),
                );
                if ui.button("Generate fake invoice").clicked() {
                    
                    let mut rng = rand::thread_rng();
                    let racun1 = Racun {
                        invoice: Invoice {
                            invoice_number: rng.gen_range(1..200),
                            invoice_date: format!("{}/{}/{}", rng.gen_range(1..31), rng.gen_range(1..12), rng.gen_range(2020..2021)),
                            due_date: format!("{}/{}/{}", rng.gen_range(1..31), rng.gen_range(1..12), rng.gen_range(2020..2021)),
                            service_date: format!("{}/{}/{}", rng.gen_range(1..31), rng.gen_range(1..12), rng.gen_range(2020..2021)),
                            invoice_currency: "EUR".to_string(),
                            company: Company {
                                company_address: "Company address".to_string(),
                                company_name: "Company name".to_string(),
                                company_bankname: "Company bank name".to_string(),
                                company_business_registered_at: "Company business registered at".to_string(),
                                company_currency: "EUR".to_string(),
                                company_iban: "Company iban".to_string(),
                                company_phone: "Company phone".to_string(),
                                company_postal_code: "Company postal code".to_string(),
                                company_registration_number: "Company registration number".to_string(),
                                company_vat_rate: 22.0,
                                company_signature: "Company signature".to_string(),
                                company_swift: "Company swift".to_string(),
                                company_vat_id: "Company vat id".to_string(),
                            },
                            invoice_location: "Slovenia".to_string(),
                            partner: Partner {
                                partner_address: "Partner address".to_string(),
                                partner_name: "Partner name".to_string(),
                                partner_postal_code: "Partner postal code".to_string(),
                                partner_vat_id: "Partner vat id".to_string(),

                            },
                            invoice_tax: 22.0,
                            invoice_reference: "123456789".to_string(),
                            created_by: "Invoice generator".to_string(),
                            services: vec![Service {
                                service_currency: "EUR".to_string(),
                                service_name: "Service name".to_string(),
                                service_price:rng.gen_range(1..1000) as f64,
                                service_quantity: rng.gen_range(1..5) as i32,
                                service_tax: 22.0,
   
                            }, Service {
                                service_currency: "EUR".to_string(),
                                service_name: "Service name".to_string(),
                                service_price: rng.gen_range(1..1000) as f64,
                                service_quantity: rng.gen_range(1..5) as i32,
                                service_tax: 22.0,
   
                            },Service {
                                service_currency: "EUR".to_string(),
                                service_name: "Service name".to_string(),
                                service_price: rng.gen_range(1..1000) as f64,
                                service_quantity: rng.gen_range(1..5) as i32,
                                service_tax: 22.0,
   
                            }],
                            status: "Paid".to_string(),
                            
                        },
                        config: InvoiceStructure {
                            font_sizes: FontSizes {
                                small:9.0,
                                medium:14.0,
                                large:16.0,
                            }
                        }
                    };
                    init(&racun1);
                    self.refresh = true;
                }
                //Debug purpose ui.colored_label(WHITE, self.clicked_pdf_path.to_string_lossy());
                ui.add_space(10.0);
                egui::Grid::new("invoice_grid").show(ui, |ui| {
                    if self.json_data.iter().count() == 0 {
                        ui.add(widgets::Spinner::new());
                        ui.label("No invoices found");
                        self.refresh = true;
                        ctx.request_repaint();
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
                                    for invoice_path in &self.invoice_paths {
                                        if invoice_path
                                            .path()
                                            .ends_with(&invoice.invoice.invoice_number.to_string())
                                        {
                                            self.clicked_pdf_path = invoice_path.path();
                                            //Get the JPG file from the clicked invoice and render it
                                            if let Some(value) = self.clicked_pdf_path.file_name() {
                                                println!("File name: {}", value.to_string_lossy());
                                                if let Ok(files) =
                                                    fs::read_dir(&self.clicked_pdf_path)
                                                {
                                                    for file in files {
                                                        if let Ok(file) = file {
                                                            if let Some(extension) =
                                                                file.path().extension()
                                                            {
                                                                if extension == "jpg" {
                                                                    println!(
                                                                        "File name: {}",
                                                                        file.path()
                                                                            .to_string_lossy()
                                                                    );
                                                                    //Get the image from the path and render it
                                                                    let image = image::io::Reader::open(file.path()).unwrap().decode().unwrap();
                                                                    let size = [image.width() as _, image.height() as _];
                                                                    let image_buffer = image.to_rgba8();
                                                                    let pixels = image_buffer.as_flat_samples();
                                                                    let color_img = egui::ColorImage::from_rgba_unmultiplied(
                                                                        size,
                                                                        pixels.as_slice());
                                                                        //self.image = Some(RetainedImage::from_color_image("Test image", color_img));
                                                                        self.texture = None;
                                                                        self.texture.get_or_insert_with(|| {
                                                                            // Load the texture only once.
                                                                            ui.ctx().load_texture(
                                                                                "image",
                                                                                color_img,
                                                                                Default::default()
                                                                            )
                                                                        });
                                                                       
                                                                
                                                                    // RetainedImage::from
                                                                    
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            } else {
                                                println!("Could not get the file name");
                                            }

                                            //Make the self.clicked_pdf_path -> Image on the GUI
                                        }
                                    }
                                    //Check which invoice is clicked and open the pdf in a new window

                                    self.show_image = true;
                                };
                                if ui.button("Edit").clicked() {
                                    //Open the invoice in a new window with its data and allow the user to edit it
                                    //TODO: Implement this
                                };
                                if ui.button("Delete").clicked() {
                                    self.delete_invoice = true;
                                    //Delete the invoice from the gui and from the json file
                                    //TODO: Implement this
                                };
                            });

                            ui.end_row();
                        }

                    }
                });
            });
        });
        
        if self.show_image {
                // println!("Show image is true");
            if self.texture.is_some() {
               
                // println!("Image is not none");
                if let Some(texture) = &mut self.texture {
                    egui::Window::new("Image").collapsible(true).resizable(true).default_size(Vec2::new(1000.0, 1000.0)).show(ctx, |ui| {
                        egui::ScrollArea::new([true, true]).show(ui, |ui| {
                        ui.add(egui::Image::new(texture.id(), [500.0, 700.0]));
                        // });
                        if ui.button("Close").clicked() {
                            self.show_image = false;
                        }
                    });
                    });
                
                }   
            }  else {
                // println!("Image is none");
                self.texture = None;
            }   
    
} else {
self.show_image = false;
}
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

