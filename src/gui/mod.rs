#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use crate::invoicer::{Racun, init, Invoice, InvoiceStructure, FontSizes, Service, Company, Partner};
use eframe;
use eframe::egui;
use egui::{widgets, Color32, TextureHandle, Align};
use egui::{RichText, Vec2};
use rand::Rng;
use fs::File;
use std::{env, thread};
use std::fs::{self, DirEntry};
use std::io::Read;
use std::path::PathBuf;
//Consts
const PADDING: f32 = 5.0;
const WHITE: Color32 = Color32::WHITE;
const CYAN: Color32 = Color32::from_rgb(0, 255, 255);



struct GuiApp {
    allowed_to_close: bool,
    show_confirmation_dialog: bool,
    show_image: bool,
    invoice_paths: Vec<DirEntry>,
    json_data: Vec<Racun>,
    delete_invoice: bool,
    clicked_pdf_path: PathBuf,
    // delete_invoice_path: PathBuf,
    texture: Option<TextureHandle>,
    refresh: bool, 
    create: bool,
    edit: bool,
    latest_invoice: Racun,
    add_service: bool
}

trait Data {
    fn get_invoices(&mut self) -> Vec<DirEntry>;
    fn parse_jsons(&mut self);
    
    fn new() -> Self;
}

impl Data for GuiApp {
    fn new() -> Self {
        let mut this = Self {
            allowed_to_close: false,
            clicked_pdf_path: PathBuf::new(),
            show_confirmation_dialog: false,
            show_image: false,
            invoice_paths: Vec::new(),
            json_data: Vec::new(),
            delete_invoice: false,
            // delete_invoice_path: PathBuf::new(),
            texture: None,
            refresh: false,    
            create: false,
            edit: false,
            latest_invoice: Racun::new(),
            add_service: false
        };
        this.invoice_paths = this.get_invoices();
        this.parse_jsons();
        this
    }
   

    fn get_invoices(&mut self) -> Vec<DirEntry> {
        let mut path = env::current_dir().unwrap();
        let invoice_folder = PathBuf::from("invoices");
        path.push(&invoice_folder);
    
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
                Err(_) => {
                    // println!("Could not parse the json file.There could be a problem with the json file or the file could be missing.");
                    continue;
            
                }, //*!TODO This panics alot if the user clicks refresh too fast or if the dir doesnt have the json (idk how tho) *//
            };
            let mut contents = String::new();
            match file_content.read_to_string(&mut contents) {
                Ok(_) => {
                    //println!("File contents: {}", contents);
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
            self.refresh = true;
            egui::ScrollArea::new([false, false]).show(ui, |ui| {
                ui.label("Project repo:");
                ui.add(widgets::Hyperlink::new("https://github.com/actuallydoc"));
                ui.add_space(PADDING);
                ui.colored_label(
                    CYAN,
                    RichText::new(format!("This is a simple invoice manager written in Rust")),
                );
                if ui.button(RichText::new("Create").color(Color32::GREEN)).clicked() {
                    self.create = true;
                }
                //*!Only for debug purposes  *//
                // if ui.button("Generate fake invoice").clicked() {
                   
                //     let racun = make_fake_invoice();
                //     //Spawn a new thread to generate the invoice and not freeze the ui
                //     thread::spawn(move || {
                //         match init(&racun.clone()) {
                //             Ok(_) => {
                //                 println!("Invoice generated");
                                
                    
                //             }
                //             Err(err) => {
                //                 println!("Error: {}", err);
                //             }
                //         };
                //     });
                   

                //     //*!TODO Refresh doesnt work so it doesnnt. *//
                   
                
                // }
                ui.add_space(PADDING);
                ui.add_space(PADDING);
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
                                            if let Some(_) = self.clicked_pdf_path.file_name() {
                                                
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
                                    if self.delete_invoice {
                                        for invoice_path in &self.invoice_paths {
                                            if invoice_path
                                                .path()
                                                .ends_with(&invoice.invoice.invoice_number.to_string())
                                            {
                                             //Delete the invoice from the json file
                                                match fs::remove_dir_all(invoice_path.path()) {
                                                    Ok(_) => self.refresh = true ,
                                                    Err(_) => (),
                                            
                                                }
                                               
                                            }

                                        }
                                    }
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
        if self.create {
            egui::Window::new("Create invoice!").resizable(true).show(ctx,|ui|{
                ui.horizontal(|ui|{
                    ui.with_layout(egui::Layout::left_to_right(Align::Center), |ui| {
                        ui.vertical(|ui| {
                            ui.heading("Company data");
                            ui.add(egui::TextEdit::singleline(&mut self.latest_invoice.invoice.company.company_name))
                                .on_hover_text("Company name");
                            ui.add(egui::TextEdit::singleline(&mut self.latest_invoice.invoice.company.company_iban))
                                .on_hover_text("Company IBAN");
                            ui.add(egui::TextEdit::singleline(&mut self.latest_invoice.invoice.company.company_swift))
                                .on_hover_text("Company SWIFT");
                            ui.add(egui::TextEdit::singleline(&mut self.latest_invoice.invoice.company.company_bankname))
                            .on_hover_text("Company Bank Name");
                            ui.add(egui::TextEdit::singleline(&mut self.latest_invoice.invoice.company.company_address))
                                .on_hover_text("Company address");
                            ui.add(egui::TextEdit::singleline(&mut self.latest_invoice.invoice.company.company_postal_code))
                                .on_hover_text("Company postal code");
                            ui.add(egui::TextEdit::singleline(&mut self.latest_invoice.invoice.company.company_vat_id))
                                .on_hover_text("Company VAT");
                            ui.add(egui::TextEdit::singleline(&mut self.latest_invoice.invoice.company.company_phone))
                                .on_hover_text("Company PHONE");
                            ui.add(egui::TextEdit::singleline(&mut self.latest_invoice.invoice.company.company_registration_number))
                                .on_hover_text("Company Registeration number");
                            ui.add(egui::TextEdit::singleline(&mut self.latest_invoice.invoice.company.company_business_registered_at))
                                .on_hover_text("Company Registered at");
                            ui.add(egui::TextEdit::singleline(&mut self.latest_invoice.invoice.company.company_currency))
                                .on_hover_text("Company currency");

                            //*!TODO Add a file picker for the company logo and convert it to base64 when creating the invoice*/
                            ui.add(egui::TextEdit::singleline(&mut self.latest_invoice.invoice.company.company_signature))
                                .on_hover_text("Company signature");
                        });
                        ui.vertical(|ui| {
                            ui.heading("Partner data");
                            ui.add(egui::TextEdit::singleline(&mut self.latest_invoice.invoice.partner.partner_name))
                                .on_hover_text("Partner name");
                            ui.add(egui::TextEdit::singleline(&mut self.latest_invoice.invoice.partner.partner_address))
                                .on_hover_text("Partner Address");
                            ui.add(egui::TextEdit::singleline(&mut self.latest_invoice.invoice.partner.partner_postal_code))
                                .on_hover_text("Partner postal code");
                            ui.add(egui::TextEdit::singleline(&mut self.latest_invoice.invoice.partner.partner_vat_id))
                                .on_hover_text("Partner VAT");
                        });
                        ui.vertical(|ui| {
                            ui.heading("Invoice data");
                            ui.add(egui::TextEdit::singleline(&mut self.latest_invoice.invoice.invoice_number))
                                .on_hover_text("Invoice number");
                            ui.add(egui::TextEdit::singleline(&mut self.latest_invoice.invoice.invoice_date))
                                .on_hover_text("Invoice date");
                            ui.add(egui::TextEdit::singleline(&mut self.latest_invoice.invoice.service_date))
                                .on_hover_text("Invoice service date");
                            ui.add(egui::TextEdit::singleline(&mut self.latest_invoice.invoice.due_date))
                                .on_hover_text("Invoice due date");
                            ui.add(egui::TextEdit::singleline(&mut self.latest_invoice.invoice.invoice_location))
                                .on_hover_text("Invoice location");
                            ui.add(egui::TextEdit::singleline(&mut self.latest_invoice.invoice.invoice_currency))
                                .on_hover_text("Invoice currency (SYMBOL)");
                            ui.add(egui::TextEdit::singleline(&mut self.latest_invoice.invoice.invoice_reference))
                                .on_hover_text("Invoice reference");
                            ui.add(egui::TextEdit::singleline(&mut self.latest_invoice.invoice.created_by))
                                .on_hover_text("Created by");
                            ui.add(
                                egui::Slider::new(&mut self.latest_invoice.invoice.invoice_tax, 0.0..=100.0)
                                    .text("Tax rate")
                                    .max_decimals(3),
                            )
                            .on_hover_text(
                                "Payment amount without vat. Vat is calculated on the end",
                            );
                        });
                    }); 
                    ui.vertical(|ui| {
                        let mut delete = None;
                        egui::ScrollArea::vertical().show(ui, |ui| {
                            ui.vertical_centered(|ui| {
                                ui.heading("Services");
                                for (pos, service) in self.latest_invoice.invoice.services.iter_mut().enumerate() {
                                    ui.horizontal(|ui| {
                                        ui.add(egui::TextEdit::multiline(&mut service.service_name))
                                            .on_hover_text("Service description");
                                        ui.add(
                                            egui::Slider::new(&mut service.service_price, 0.0..=10000.0)
                                                .text("Amount to pay")
                                                .max_decimals(3),
                                        )
                                        .on_hover_text(
                                            "Payment amount without vat. Vat is calculated on the end",
                                        );
                                        ui.add(
                                            egui::Slider::new(&mut service.service_quantity, 0..=100)
                                                .text("Quantity")
                                                .max_decimals(3),
                                        )
                                        .on_hover_text(
                                            "Payment amount without vat. Vat is calculated on the end",
                                        );
                                        if ui
                                        .add(egui::Button::new("Delete"))
                                        .on_hover_text("Delete a service. THIS CANNOT BE UNDONE!")
                                        .clicked()
                                             {
                                        delete = Some(pos);
                                            };
                                     });
                                     
                                }
                                if let Some(pos) = delete {
                                    
                                    self.latest_invoice.invoice.services.remove(pos);
                                }
                                if ui.button("Add service").clicked() {
                                    self.add_service = true;
                                }
                            });
                        });
                     
                    });
                    if self.add_service {
                        egui::Window::new("Add a service")
                            .collapsible(false)
                            .resizable(false)
                            .min_width(500.0)
                            .min_height(500.0)
                            .show(ctx, |ui| {
                                ui.horizontal(|ui| {
                                    ui.label("What kind of service do you want to add?");
                                    if ui
                                        .button("Create a blank service!")
                                        .on_hover_text("Create a blank service with no data")
                                        .clicked()
                                    {
                                        let new_service = Service::default();
                                        self.latest_invoice.invoice.services.push(new_service);
                                        // self.service_count += 1;
                                        self.add_service = false;
                                    }
                                })
                            });
                    }
                    
                });
                if ui.button("Create").clicked(){
                        let invoice = self.latest_invoice.clone();
                        thread::spawn(move || {
                            match init(&invoice) {
                                Ok(_) => {
                                    // println!("Invoice generated");
                                    // println!("Invoice: {:#?}", invoice)
                                    
                                }
                                Err(err) => {
                                    println!("Error: {}", err);
                                }
                            };
                        });
                    }
                if ui.button("Close").clicked(){
                    self.create = false
                }
            
            }); 
        }
       
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


//Only for testing purposes
fn make_fake_invoice()-> Racun {
    let mut rng = rand::thread_rng();
    let racun1 = Racun {
        invoice: Invoice {
            invoice_number: rng.gen_range(1..200).to_string(),
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
                service_price: 15.30,
                service_quantity: 1,
                service_tax: 22.0,

            }, Service {
                service_currency: "EUR".to_string(),
                service_name: "Service name".to_string(),
                service_price: 15.30,
                service_quantity: 1,
                service_tax: 22.0,

            },Service {
                service_currency: "EUR".to_string(),
                service_name: "Service name".to_string(),
                service_price: 15.30,
                service_quantity: 1,
                service_tax: 22.0,

            }],
            status: crate::invoicer::PaymentStatus::UNPAID,
            
        },
        config: InvoiceStructure {
            font_sizes: FontSizes {
                small:9.0,
                medium:14.0,
                large:16.0,
            }
        }
    };
    racun1
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
    ).unwrap();
}
