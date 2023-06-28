#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use crate::invoicer::{init, Company, Partner, Racun, Service};

use eframe::egui;
use eframe::{self, IconData};

use egui::{widgets, Align, Color32, Layout, TextureHandle};
use egui::{RichText, Vec2};
use fs::File;

use native_dialog::{self, FileDialog, MessageDialog, MessageType};

use std::fs::{self, DirEntry, OpenOptions};
use std::io::{Read, Write};
use std::path::PathBuf;
use std::{env, thread};
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
    texture: Option<TextureHandle>,
    refresh: bool,
    create: bool,
    edit: bool,
    selected_signature: Option<String>,
    signature_path: Option<PathBuf>,
    clicked_invoice: Racun,
    latest_invoice: Racun,
    clicked_company: Company,
    clicked_partner: Partner,
    clicked_service: Service,
    add_service: bool,
    manage_partners: bool,
    edit_partner: bool,
    manage_companies: bool,
    create_company: bool,
    create_partner: bool,
    create_service: bool,
    edit_company: bool,
    manage_services: bool,
    edit_service: bool,
    empty_company: Company,
    empty_partner: Partner,
    empty_service: Service,
    temp_company: Company,
    temp_partner: Partner,
    temp_service: Service,
    companies: Vec<Company>,
    partners: Vec<Partner>,
    services: Vec<Service>,
    change_company: bool,
    change_partner: bool,
}
trait Data {
    fn new() -> Self;
    //Pdf functions
    fn generate_pdf(&mut self, invoice: Racun) -> bool;
    //Json functions
    fn get_invoices(&mut self) -> Option<Vec<DirEntry>>;
    fn parse_jsons(&mut self);
    fn get_partners(&mut self);
    fn get_services(&mut self);
    fn get_companies(&mut self);
    fn save_partner(&mut self, partner: Partner);
    fn save_service(&mut self, service: Service);
    fn save_company(&mut self, company: Company);
    //Ui functions
    fn render_header(&mut self, ui: &mut egui::Ui, ctx: &egui::Context);
    fn render_main(&mut self, ui: &mut egui::Ui, ctx: &egui::Context);
    fn render_footer(&mut self, ui: &mut egui::Ui);
    fn render_image_window(&mut self, ctx: &egui::Context);
    fn render_confirmation_dialog(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame);
    //Edit , Create Ui functions
    fn render_edit_invoice(&mut self, ctx: &egui::Context);
    fn render_create_company(&mut self, ctx: &egui::Context);
    fn render_create_invoice(&mut self, ctx: &egui::Context);
    fn render_create_service(&mut self, ctx: &egui::Context);
    fn render_create_partner(&mut self, ctx: &egui::Context);
    fn render_add_service(&mut self, ctx: &egui::Context);
    fn render_change_partner(&mut self, ctx: &egui::Context);
    fn render_change_company(&mut self, ctx: &egui::Context);

    fn render_manage_companies(&mut self, ctx: &egui::Context);
    fn render_manage_partners(&mut self, ctx: &egui::Context);
    fn render_manage_services(&mut self, ctx: &egui::Context);

    fn render_edit_company(&mut self, ctx: &egui::Context);
    fn render_edit_partner(&mut self, ctx: &egui::Context);
    fn render_edit_service(&mut self, ctx: &egui::Context);
    fn delete_invoice(&mut self, invoice: Racun);
}
impl Data for GuiApp {
    fn new() -> Self {
        let mut this = Self {
            clicked_company: Company::default(),
            clicked_partner: Partner::default(),
            clicked_service: Service::default(),
            allowed_to_close: false,
            clicked_pdf_path: PathBuf::new(),
            show_confirmation_dialog: false,
            show_image: false,
            invoice_paths: Vec::new(),
            json_data: Vec::new(),
            companies: Vec::new(),
            partners: Vec::new(),
            services: Vec::new(),
            delete_invoice: false,
            texture: None,
            refresh: false,
            create: false,
            edit: false,
            create_company: false,
            create_partner: false,
            create_service: false,
            selected_signature: None,
            signature_path: None,
            latest_invoice: Racun::default(),
            add_service: false,
            clicked_invoice: Racun::default(),
            manage_companies: false,
            edit_company: false,
            manage_partners: false,
            edit_partner: false,
            manage_services: false,
            edit_service: false,
            empty_company: Company::default(),
            empty_partner: Partner::default(),
            empty_service: Service::default(),
            temp_company: Company::default(),
            temp_partner: Partner::default(),
            temp_service: Service::default(),
            change_company: false,
            change_partner: false,
        };
        if let Some(invoices) = this.get_invoices() {
            this.invoice_paths = invoices;
        } else {
            this.invoice_paths = Vec::new();
        }
        this.get_companies();
        this.get_partners();
        this.get_services();
        this.parse_jsons();
        this
    }
    fn get_invoices(&mut self) -> Option<Vec<DirEntry>> {
        let mut path = env::current_dir().unwrap();
        let invoice_folder = PathBuf::from("invoices");
        path.push(&invoice_folder);
        match fs::read_dir(path) {
            Ok(folders) => {
                let folders: Vec<DirEntry> = folders
                    .filter_map(|entry| entry.ok())
                    .filter(|entry| entry.file_type().unwrap().is_dir())
                    .collect();
                Some(folders)
            }
            Err(_) => {
                fs::create_dir(invoice_folder).unwrap();
                None
            }
        }
    }
    //*!TODO if the json file is empty dont parse it cause its gonna panic or i can just use a print statement */
    fn get_companies(&mut self) {
        //Open the json file
        let mut file_content = match File::open("companies.json") {
            Ok(file) => file,
            Err(_) => {
                let file = File::create("companies.json").unwrap();
                file
            }
        };
        let mut contents = String::new();
        match file_content.read_to_string(&mut contents) {
            Ok(_) => {
                let companies: Vec<Company> = match serde_json::from_str(&contents) {
                    Ok(invoice) => invoice,
                    Err(err) => panic!("Could not deserialize the file, error code: {}", err),
                };

                for company in companies {
                    self.companies.push(company);
                }
            }
            Err(err) => panic!("Could not read the json file, error code: {}", err),
        };
    }
    fn get_partners(&mut self) {
        let mut file_content = match File::open("partners.json") {
            Ok(file) => file,
            Err(_) => {
                let file = File::create("partners.json").unwrap();
                file
            }
        };
        let mut contents = String::new();
        match file_content.read_to_string(&mut contents) {
            Ok(_) => {
                let partners: Vec<Partner> = match serde_json::from_str(&contents) {
                    Ok(invoice) => invoice,
                    Err(err) => panic!("Could not deserialize the file, error code: {}", err),
                };

                for partner in partners {
                    self.partners.push(partner);
                }
            }
            Err(err) => panic!("Could not read the json file, error code: {}", err),
        };
    }
    fn get_services(&mut self) {
        let mut file_content = match File::open("services.json") {
            Ok(file) => file,
            Err(_) => {
                let file = File::create("services.json").unwrap();
                file
            }
        };
        let mut contents = String::new();
        match file_content.read_to_string(&mut contents) {
            Ok(_) => {
                let services: Vec<Service> = match serde_json::from_str(&contents) {
                    Ok(invoice) => invoice,
                    Err(err) => panic!("Could not deserialize the file, error code: {}", err),
                };

                for service in services {
                    self.services.push(service);
                }
            }
            Err(err) => panic!("Could not read the json file, error code: {}", err),
        };
    }
    fn save_company(&mut self, company: Company) {
        self.companies.push(company);
        let file_name = "companies.json";
        match OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .truncate(true)
            .open(file_name)
        {
            Ok(mut file) => {
                let serialized = serde_json::to_string(&self.companies).unwrap();
                file.write_all(serialized.as_bytes()).unwrap();
            }
            Err(err) => panic!("Could not open the file, error code: {}", err),
        };
    }
    fn save_partner(&mut self, partner: Partner) {
        self.partners.push(partner);
        let file_name = "partners.json";
        match OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .truncate(true)
            .open(file_name)
        {
            Ok(mut file) => {
                let serialized = serde_json::to_string(&self.partners).unwrap();
                file.write_all(serialized.as_bytes()).unwrap();
            }
            Err(err) => panic!("Could not open the file, error code: {}", err),
        };
    }
    fn save_service(&mut self, service: Service) {
        self.services.push(service);
        let file_name = "services.json";
        match OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .truncate(true)
            .open(file_name)
        {
            Ok(mut file) => {
                let serialized = serde_json::to_string(&self.services).unwrap();
                file.write_all(serialized.as_bytes()).unwrap();
            }
            Err(err) => panic!("Could not open the file, error code: {}", err),
        };
    }
    fn generate_pdf(&mut self, invoice: Racun) -> bool {
        print!("Generating pdf...");
        let temp_path = self.signature_path.clone();
        let handle = thread::spawn(move || {
            let mut state = false;
            if temp_path.is_some() {
                match init(&invoice, Some(&temp_path.unwrap())) {
                    Ok(_) => {
                        state = true;
                    }
                    Err(_err) => {
                        state = false;
                    }
                }
            } else {
                match init(&invoice, None) {
                    Ok(_) => {
                        state = true;
                    }
                    Err(_err) => {
                        state = false;
                    }
                }
            }

            state
        });
        let result = handle.join().unwrap();
        result
    }
    fn parse_jsons(&mut self) {
        let paths = self.get_invoices();
        //Make a vector of invoices
        let mut json_data: Vec<Racun> = Vec::new();
        for path in paths.unwrap() {
            let mut file_path = path.path();
            file_path.push("output.json");
            let mut file_content = match File::open(file_path.to_str().unwrap()) {
                Ok(file) => file,
                Err(_) => {
                    continue;
                } //*!TODO This panics alot if the user clicks refresh too fast or if the dir doesnt have the json (idk how tho) *//
            };
            let mut contents = String::new();
            match file_content.read_to_string(&mut contents) {
                Ok(_) => {
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
    fn render_image_window(&mut self, ctx: &egui::Context) {
        egui::Window::new("Pdf viewer")
            .collapsible(true)
            .resizable(true)
            .default_size(Vec2::new(1000.0, 1000.0))
            .show(ctx, |ui| {
                egui::ScrollArea::new([true, true]).show(ui, |ui| {
                    ui.add(egui::Image::new(
                        self.texture.as_ref().unwrap().id(),
                        [500.0, 700.0],
                    ));
                    if ui.button("Close").clicked() {
                        self.show_image = false;
                    }
                })
            });
    }
    fn render_confirmation_dialog(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::Window::new("Do you want to quit?")
            .collapsible(true)
            .resizable(false)
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
    fn render_manage_companies(&mut self, ctx: &egui::Context) {
        egui::Window::new("Company dashboard")
            .resizable(true)
            .collapsible(true)
            .movable(true)
            .show(ctx, |ui| {
                egui::Grid::new("company_grid").show(ui, |ui| {
                    let mut edit = None;
                    if self.companies.is_empty() {
                        if ui.button("Create company!").clicked() {
                            self.create_company = true;
                        }
                    } else {
                        ui.colored_label(CYAN, "Company name");
                        ui.colored_label(CYAN, "Company address");
                        ui.colored_label(CYAN, "Company postal code");
                        ui.colored_label(CYAN, "Company VAT ID");
                        ui.colored_label(CYAN, "Company Phone");
                        ui.colored_label(CYAN, "Actions");
                        ui.end_row();
                        for company in self.companies.clone() {
                            ui.label(company.company_name.clone());
                            ui.label(company.company_address.clone());
                            ui.label(company.company_postal_code.clone());
                            ui.label(company.company_vat_id.clone());
                            ui.label(company.company_phone.clone());
                            if ui.button("Edit").clicked() {
                                edit = Some(company.clone());
                            }
                            if ui.button("Delete").clicked() {
                                self.companies.remove(
                                    self.companies.iter().position(|x| x == &company).unwrap(),
                                );
                            }
                            ui.end_row();
                        }

                        if edit.is_some() {
                            self.clicked_company = edit.unwrap();
                            self.edit_company = true;
                        }
                        ui.vertical(|ui| {
                            ui.add_space(15.0);
                            if ui.button("Create").clicked() {
                                self.create_company = true;
                            }
                            ui.add_space(15.0);
                            if ui.button("Close").clicked() {
                                self.manage_companies = false;
                            }
                        });
                    }
                })
            });
    }
    fn render_manage_partners(&mut self, ctx: &egui::Context) {
        egui::Window::new("Partner Dashboard").show(ctx, |ui| {
            ui.vertical(|ui| {
                egui::Grid::new("company_grid").show(ui, |ui| {
                    if self.partners.is_empty() {
                        if ui.button("Create partner!").clicked() {
                            self.create_partner = true;
                        }
                    } else {
                        ui.colored_label(CYAN, "Partner name");
                        ui.colored_label(CYAN, "Partner address");
                        ui.colored_label(CYAN, "Partner postal code");
                        ui.colored_label(CYAN, "Partner VAT ID");
                        ui.colored_label(CYAN, "Actions");
                        ui.end_row();
                        for partner in self.partners.clone() {
                            ui.label(partner.partner_name.clone());
                            ui.label(partner.partner_address.clone());
                            ui.label(partner.partner_postal_code.clone());
                            ui.label(partner.partner_vat_id.clone());
                            if ui.button("Edit").clicked() {
                                todo!("Edit partner window")
                            }
                            if ui.button("Delete").clicked() {
                                self.partners.remove(
                                    self.partners.iter().position(|x| x == &partner).unwrap(),
                                );
                            }
                            ui.end_row();
                        }
                        ui.vertical(|ui| {
                            ui.add_space(15.0);
                            if ui.button("Create").clicked() {
                                self.create_partner = true;
                            }

                            ui.add_space(15.0);
                            if ui.button("Close").clicked() {
                                self.manage_partners = false;
                            }
                        });
                    }
                });
            });
        });
    }

    fn render_manage_services(&mut self, ctx: &egui::Context) {
        egui::Window::new("Services Dashboard").show(ctx, |ui| {
            ui.vertical(|ui| {
                if self.services.is_empty() {
                    if ui.button("No services found create one!").clicked() {
                        self.create_service = true;
                    }
                } else {
                    egui::Grid::new("service_grid").show(ui, |ui| {
                        ui.colored_label(CYAN, "Service name");
                        ui.colored_label(CYAN, "Service price");
                        ui.colored_label(CYAN, "Actions");
                        ui.end_row();
                        for services in self.services.clone() {
                            ui.label(services.service_name.clone());
                            ui.label(services.service_price.to_string());
                            if ui.button("Edit").clicked() {
                                todo!("Edit service window")
                            }
                            if ui.button("Delete").clicked() {
                                self.services.remove(
                                    self.services.iter().position(|x| x == &services).unwrap(),
                                );
                            }
                            ui.end_row();
                        }
                        ui.vertical(|ui| {
                            ui.add_space(15.0);
                            if ui.button("Create").clicked() {
                                self.create_service = true;
                            }

                            ui.add_space(15.0);
                            if ui.button("Close").clicked() {
                                self.manage_services = false;
                            }
                        });
                    });
                }
            });
        });
    }
    fn render_change_company(&mut self, ctx: &egui::Context) {
        egui::Window::new("Change company").show(ctx, |ui| {
            ui.vertical(|ui| {
                if self.companies == Vec::new() {
                    ui.label("There are no companies, please create one");
                } else {
                    egui::ComboBox::from_label("Select premade companies: ")
                        .selected_text(format!("{}", self.temp_company.company_name.to_string()))
                        .show_ui(ui, |ui| {
                            for company in self.companies.iter() {
                                if ui
                                    .selectable_value(
                                        &mut self.temp_company,
                                        company.clone(),
                                        format!("{}", company.company_name.to_string()),
                                    )
                                    .clicked()
                                {
                                    self.latest_invoice.invoice.company = self.temp_company.clone();
                                    self.change_company = false;
                                };
                            }
                        });
                }
                ui.add_space(20.0);
                if ui.button("Create a new company").clicked() {
                    self.create_company = false;
                    self.create_company = true;
                }
                ui.add_space(10.00);
                if ui.button("Exit").clicked() {
                    self.change_company = false;
                }
            })
        });
    }

    fn render_change_partner(&mut self, ctx: &egui::Context) {
        egui::Window::new("Change partner").show(ctx, |ui| {
            ui.vertical(|ui| {
                if self.partners == Vec::new() {
                    ui.label("There are no partners in the database, please create one");
                } else {
                    egui::ComboBox::from_label("Select premade partners: ")
                        .selected_text(format!("{}", self.temp_partner.partner_name.to_string()))
                        .show_ui(ui, |ui| {
                            for partner in self.partners.iter() {
                                if ui
                                    .selectable_value(
                                        &mut self.temp_partner,
                                        partner.clone(),
                                        format!("{}", partner.partner_name.to_string()),
                                    )
                                    .clicked()
                                {
                                    self.latest_invoice.invoice.partner = self.temp_partner.clone();
                                    self.change_partner = false;
                                };
                            }
                        });
                }
                ui.add_space(20.0);
                if ui.button("Create a new partner").clicked() {
                    self.change_partner = false;
                    self.create_partner = true;
                }
                ui.add_space(20.0);
                if ui.button("Exit").clicked() {
                    self.change_partner = false;
                }
            })
        });
    }

    fn render_add_service(&mut self, ctx: &egui::Context) {
        egui::Window::new("Add a service")
        .collapsible(false)
        .resizable(false)
        .min_width(500.0)
        .min_height(500.0)
        .show(ctx, |ui| {
            ui.vertical(|ui| {
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
                ui.add_space(30.0);
                //Loop through the serivces and make a dropdown
                if self.services == Vec::new() {
                    ui.label("No premade services found in the database, please create one or choose blank!");
                } else {
                egui::ComboBox::from_label("Select premade services: ")
                    .selected_text(format!("{}", self.temp_service.service_name.to_string()))
                    .show_ui(ui, |ui| {
                        for service in self.services.iter() {
                            if ui.selectable_value(&mut self.temp_service , service.clone(), format!("{}", service.service_name.to_string())).clicked() {
                                self.latest_invoice.invoice.services.push(service.clone());
                                self.add_service = false;
                            };
                        }
                    }
                );
            }
            ui.add_space(30.0);
            if ui.button("Create a new service").clicked() {
                self.add_service = false;
                self.create_service = true;
            }
            ui.add_space(30.0);
            if ui.button("Cancel").clicked() {
                self.add_service = false;
            }
            })
        });
    }
    fn render_header(&mut self, ui: &mut egui::Ui, _ctx: &egui::Context) {
        ui.label("Project repo:");
        ui.add(widgets::Hyperlink::new("https://github.com/actuallydoc"));
        ui.add_space(PADDING);
        ui.horizontal(|ui| {
            ui.colored_label(
                CYAN,
                RichText::new("This is a simple invoice manager written in Rust".to_string()),
            );
            ui.add_space(PADDING);
            if ui
                .button(RichText::new("Create invoice").color(Color32::GREEN))
                .clicked()
            {
                self.create = true;
            }
            ui.add_space(PADDING);
            if ui
                .button(RichText::new("Manage Company").color(Color32::GRAY))
                .clicked()
            {
                self.manage_companies = true;
            }
            ui.add_space(PADDING);
            if ui
                .button(RichText::new("Manage Partners").color(Color32::GRAY))
                .clicked()
            {
                self.manage_partners = true;
            }
            ui.add_space(PADDING);
            if ui
                .button(RichText::new("Manage Services").color(Color32::GRAY))
                .clicked()
            {
                self.manage_services = true;
            }
        });
        ui.add_space(PADDING);
        ui.add_space(PADDING);
        ui.add_space(10.0);
    }
    fn render_main(&mut self, ui: &mut egui::Ui, ctx: &egui::Context) {
        egui::Grid::new("invoice_grid").show(ui, |ui| {
            if self.json_data.iter().len() == 0 {
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
                        total_price += service.service_price * (100.0 + invoice.invoice.invoice_tax) / 100.0;
                        ui.label(format!("{:.2} {}", total_price, invoice.invoice.invoice_currency));
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
                                    .ends_with(&invoice.invoice.invoice_number)
                                {
                                    self.clicked_pdf_path = invoice_path.path();
                                    //Get the JPG file from the clicked invoice and render it
                                    if self.clicked_pdf_path.file_name().is_some() {
                                        if let Ok(files) =
                                            fs::read_dir(&self.clicked_pdf_path)
                                        {
                                            for file in files {
                                                if let Ok(file) = file {
                                                    if let Some(extension) =
                                                        file.path().extension()
                                                    {
                                                        if extension == "jpg" {
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
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    } else {
                                    }
                                    //Make the self.clicked_pdf_path -> Image on the GUI
                                }
                            }
                            //Check which invoice is clicked and open the pdf in a new window
                            self.show_image = true;
                        };
                        if ui.button("Edit").clicked() {
                            self.clicked_invoice = invoice.clone();
                            self.edit = true;
                        };
                        if ui.button("Delete").clicked() {
                            self.delete_invoice = true;
                            if self.delete_invoice {
                                for invoice_path in &self.invoice_paths {
                                    if invoice_path
                                        .path()
                                        .ends_with(&invoice.invoice.invoice_number)
                                    {
                                     //Delete the invoice from the json file
                                     if fs::remove_dir_all(invoice_path.path()).is_ok() {
                                        self.refresh = true;
                                     }
                                    }
                                }
                            }
                        };
                    });
                    ui.end_row();
                }
            }
        });
    }
    fn render_footer(&mut self, ui: &mut egui::Ui) {
        ui.vertical(|ui| {
            ui.add_space(150.0);
            ui.with_layout(Layout::left_to_right(egui::Align::BOTTOM), |ui| {
                ui.horizontal(|ui| {
                    ui.label("Total invoices:");
                    ui.label(self.json_data.iter().len().to_string());
                    ui.add_space(50.00);
                    ui.label(format!("Companies: {}", self.companies.len().to_string()));
                    ui.add_space(50.00);
                    ui.label(format!("Partners: {}", self.partners.len().to_string()));
                    ui.add_space(50.00);
                    ui.label(format!("Services: {}", self.services.len().to_string()));
                    // ui.label(format!("Discord: {}",self.presence_data.state()));
                });
            });
        });
    }
    fn render_edit_invoice(&mut self, ctx: &egui::Context) {
        egui::Window::new("Edit invoice!")
            .resizable(true)
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.with_layout(egui::Layout::left_to_right(Align::Center), |ui| {
                        ui.vertical(|ui| {
                            ui.heading("Company data");
                            if self.companies.len() > 0 {
                                if ui
                                    .button(RichText::new("Change company").color(Color32::RED))
                                    .clicked()
                                {
                                    self.change_company = true;
                                }
                            }
                            ui.add(egui::TextEdit::singleline(
                                &mut self.clicked_invoice.invoice.company.company_name,
                            ))
                            .on_hover_text("Company name");
                            ui.add(egui::TextEdit::singleline(
                                &mut self.clicked_invoice.invoice.company.company_iban,
                            ))
                            .on_hover_text("Company IBAN");
                            ui.add(egui::TextEdit::singleline(
                                &mut self.clicked_invoice.invoice.company.company_swift,
                            ))
                            .on_hover_text("Company SWIFT");
                            ui.add(egui::TextEdit::singleline(
                                &mut self.clicked_invoice.invoice.company.company_bankname,
                            ))
                            .on_hover_text("Company Bank Name");
                            ui.add(egui::TextEdit::singleline(
                                &mut self.clicked_invoice.invoice.company.company_address,
                            ))
                            .on_hover_text("Company address");
                            ui.add(egui::TextEdit::singleline(
                                &mut self.clicked_invoice.invoice.company.company_postal_code,
                            ))
                            .on_hover_text("Company postal code");
                            ui.add(egui::TextEdit::singleline(
                                &mut self.clicked_invoice.invoice.company.company_vat_id,
                            ))
                            .on_hover_text("Company VAT");
                            ui.add(egui::TextEdit::singleline(
                                &mut self.clicked_invoice.invoice.company.company_phone,
                            ))
                            .on_hover_text("Company PHONE");
                            ui.add(egui::TextEdit::singleline(
                                &mut self
                                    .clicked_invoice
                                    .invoice
                                    .company
                                    .company_registration_number,
                            ))
                            .on_hover_text("Company Registeration number");
                            ui.add(egui::TextEdit::singleline(
                                &mut self
                                    .clicked_invoice
                                    .invoice
                                    .company
                                    .company_business_registered_at,
                            ))
                            .on_hover_text("Company Registered at");
                            ui.add(egui::TextEdit::singleline(
                                &mut self.clicked_invoice.invoice.company.company_currency,
                            ))
                            .on_hover_text("Company currency");
                            ui.add_space(10.0);
                            if self
                                .clicked_invoice
                                .invoice
                                .company
                                .company_signature_path
                                .is_some()
                            {
                                ui.label(format!(
                                    "Signature: {}",
                                    self.clicked_invoice
                                        .invoice
                                        .company
                                        .company_signature_path
                                        .clone()
                                        .unwrap()
                                        .as_path()
                                        .display()
                                ));
                            } else {
                                ui.label("Signature is not set!");
                            }

                            if ui.button("Change signature").clicked() {
                                let path = FileDialog::new()
                                    .set_location("~/Desktop")
                                    .add_filter("PNG Image", &["png"])
                                    .add_filter("JPEG Image", &["jpg", "jpeg"])
                                    .show_open_single_file()
                                    .unwrap();
                                let path = match path {
                                    Some(path) => path,
                                    None => return,
                                };

                                let yes = MessageDialog::new()
                                    .set_type(MessageType::Info)
                                    .set_title("Do you want to open the file?")
                                    .set_text(&format!("{:#?}", path))
                                    .show_confirm()
                                    .unwrap();

                                if yes {
                                    println!("Opening file...");
                                    println!("{}", format!("{:#?}", path));
                                    let base_string =
                                        image_base64::to_base64(path.as_os_str().to_str().unwrap());
                                    self.selected_signature = Some(base_string);
                                    self.signature_path = Some(path.clone());
                                    self.clicked_invoice.invoice.company.company_signature_path =
                                        Some(path);
                                }
                            }
                        });
                        ui.vertical(|ui| {
                            ui.heading("Partner data");
                            if self.partners.len() > 0 {
                                if ui
                                    .button(RichText::new("Change partner").color(Color32::RED))
                                    .clicked()
                                {
                                    self.change_partner = true;
                                }
                            }
                            ui.add(egui::TextEdit::singleline(
                                &mut self.clicked_invoice.invoice.partner.partner_name,
                            ))
                            .on_hover_text("Partner name");
                            ui.add(egui::TextEdit::singleline(
                                &mut self.clicked_invoice.invoice.partner.partner_address,
                            ))
                            .on_hover_text("Partner Address");
                            ui.add(egui::TextEdit::singleline(
                                &mut self.clicked_invoice.invoice.partner.partner_postal_code,
                            ))
                            .on_hover_text("Partner postal code");
                            ui.add(egui::TextEdit::singleline(
                                &mut self.clicked_invoice.invoice.partner.partner_vat_id,
                            ))
                            .on_hover_text("Partner VAT");
                            ui.add(egui::Checkbox::new(
                                &mut self.clicked_invoice.invoice.partner.is_vat_payer,
                                "Partner is VAT payer",
                            ));
                        });
                        ui.vertical(|ui| {
                            ui.heading("Invoice data");
                            ui.add(egui::TextEdit::singleline(
                                &mut self.clicked_invoice.invoice.invoice_number,
                            ))
                            .on_hover_text("Invoice number");
                            ui.add(egui::TextEdit::singleline(
                                &mut self.clicked_invoice.invoice.invoice_date,
                            ))
                            .on_hover_text("Invoice date");
                            ui.add(egui::TextEdit::singleline(
                                &mut self.clicked_invoice.invoice.service_date,
                            ))
                            .on_hover_text("Invoice service date");
                            ui.add(egui::TextEdit::singleline(
                                &mut self.clicked_invoice.invoice.due_date,
                            ))
                            .on_hover_text("Invoice due date");
                            ui.add(egui::TextEdit::singleline(
                                &mut self.clicked_invoice.invoice.invoice_location,
                            ))
                            .on_hover_text("Invoice location");
                            ui.add(egui::TextEdit::singleline(
                                &mut self.clicked_invoice.invoice.invoice_currency,
                            ))
                            .on_hover_text("Invoice currency (SYMBOL)");
                            ui.add(egui::TextEdit::singleline(
                                &mut self.clicked_invoice.invoice.invoice_reference,
                            ))
                            .on_hover_text("Invoice reference");
                            ui.add(egui::TextEdit::singleline(
                                &mut self.clicked_invoice.invoice.created_by,
                            ))
                            .on_hover_text("Created by");
                            ui.add(
                                egui::Slider::new(
                                    &mut self.clicked_invoice.invoice.invoice_tax,
                                    0.0..=100.0,
                                )
                                .text("Tax rate")
                                .max_decimals(3),
                            )
                            .on_hover_text(
                                "Payment amount without vat. Vat is calculated on the end11",
                            );
                        });
                    });

                    if self.add_service {
                        self.render_add_service(ctx);
                    }
                });
                ui.vertical(|ui| {
                    let mut delete = None;
                    egui::ScrollArea::vertical().show(ui, |ui| {
                        ui.vertical_centered(|ui| {
                            ui.heading("Services");
                            for (pos, service) in
                                self.clicked_invoice.invoice.services.iter_mut().enumerate()
                            {
                                ui.horizontal(|ui| {
                                    ui.add(egui::TextEdit::multiline(&mut service.service_name))
                                        .on_hover_text("Service description");
                                    ui.add(
                                        egui::Slider::new(
                                            &mut service.service_price,
                                            0.0..=10000.0,
                                        )
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
                                self.clicked_invoice.invoice.services.remove(pos);
                            }
                            if ui.button("Add service").clicked() {
                                self.add_service = true;
                            }
                        });
                        if ui.button("Edit").clicked() {
                            //Locate the index
                            self.delete_invoice(self.clicked_invoice.clone());
                            self.generate_pdf(self.clicked_invoice.clone());
                            self.edit = false;
                        }
                        ui.add_space(20.0);
                        if ui.button("Exit").clicked() {
                            self.edit = false;
                        }
                    });
                });
            });
    }
    fn render_create_invoice(&mut self, ctx: &egui::Context) {
        egui::Window::new("Create invoice!")
            .resizable(true)
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.with_layout(egui::Layout::left_to_right(Align::Center), |ui| {
                        ui.vertical(|ui| {
                            ui.heading("Company data");
                            if self.companies.len() > 0 {
                                if ui
                                    .button(RichText::new("Change company").color(Color32::RED))
                                    .clicked()
                                {
                                    self.change_company = true;
                                }
                            }
                            ui.add(egui::TextEdit::singleline(
                                &mut self.latest_invoice.invoice.company.company_name,
                            ))
                            .on_hover_text("Company name");
                            ui.add(egui::TextEdit::singleline(
                                &mut self.latest_invoice.invoice.company.company_iban,
                            ))
                            .on_hover_text("Company IBAN");
                            ui.add(egui::TextEdit::singleline(
                                &mut self.latest_invoice.invoice.company.company_swift,
                            ))
                            .on_hover_text("Company SWIFT");
                            ui.add(egui::TextEdit::singleline(
                                &mut self.latest_invoice.invoice.company.company_bankname,
                            ))
                            .on_hover_text("Company Bank Name");
                            ui.add(egui::TextEdit::singleline(
                                &mut self.latest_invoice.invoice.company.company_address,
                            ))
                            .on_hover_text("Company address");
                            ui.add(egui::TextEdit::singleline(
                                &mut self.latest_invoice.invoice.company.company_postal_code,
                            ))
                            .on_hover_text("Company postal code");
                            ui.add(egui::TextEdit::singleline(
                                &mut self.latest_invoice.invoice.company.company_vat_id,
                            ))
                            .on_hover_text("Company VAT");
                            ui.add(egui::TextEdit::singleline(
                                &mut self.latest_invoice.invoice.company.company_phone,
                            ))
                            .on_hover_text("Company PHONE");
                            ui.add(egui::TextEdit::singleline(
                                &mut self
                                    .latest_invoice
                                    .invoice
                                    .company
                                    .company_registration_number,
                            ))
                            .on_hover_text("Company Registeration number");
                            ui.add(egui::TextEdit::singleline(
                                &mut self
                                    .latest_invoice
                                    .invoice
                                    .company
                                    .company_business_registered_at,
                            ))
                            .on_hover_text("Company Registered at");
                            ui.add(egui::TextEdit::singleline(
                                &mut self.latest_invoice.invoice.company.company_currency,
                            ))
                            .on_hover_text("Company currency");

                            if self.selected_signature.is_some() {
                                ui.label(format!(
                                    "Selected signature: {:#?}",
                                    self.signature_path.clone().unwrap()
                                ));
                            } else {
                                ui.label("No signature selected!");
                            }
                            if ui.button("Change signature").clicked() {
                                let path = FileDialog::new()
                                    .set_location("~/Desktop")
                                    .add_filter("PNG Image", &["png"])
                                    .add_filter("JPEG Image", &["jpg", "jpeg"])
                                    .show_open_single_file()
                                    .unwrap();
                                let path = match path {
                                    Some(path) => path,
                                    None => return,
                                };

                                let yes = MessageDialog::new()
                                    .set_type(MessageType::Info)
                                    .set_title("Do you want to open the file?")
                                    .set_text(&format!("{:#?}", path))
                                    .show_confirm()
                                    .unwrap();

                                if yes {
                                    println!("Opening file...");
                                    println!("{}", format!("{:#?}", path));
                                    let base_string =
                                        image_base64::to_base64(path.as_os_str().to_str().unwrap());
                                    self.selected_signature = Some(base_string);
                                    self.signature_path = Some(path.clone());
                                    self.latest_invoice.invoice.company.company_signature_path =
                                        Some(path);
                                }
                            }
                        });
                        ui.vertical(|ui| {
                            ui.heading("Partner data");
                            if self.partners.len() > 0 {
                                if ui
                                    .button(RichText::new("Change partner").color(Color32::RED))
                                    .clicked()
                                {
                                    self.change_partner = true;
                                }
                            }
                            ui.add(egui::TextEdit::singleline(
                                &mut self.latest_invoice.invoice.partner.partner_name,
                            ))
                            .on_hover_text("Partner name");
                            ui.add(egui::TextEdit::singleline(
                                &mut self.latest_invoice.invoice.partner.partner_address,
                            ))
                            .on_hover_text("Partner Address");
                            ui.add(egui::TextEdit::singleline(
                                &mut self.latest_invoice.invoice.partner.partner_postal_code,
                            ))
                            .on_hover_text("Partner postal code");
                            ui.add(egui::TextEdit::singleline(
                                &mut self.latest_invoice.invoice.partner.partner_vat_id,
                            ))
                            .on_hover_text("Partner VAT");
                            ui.add(egui::Checkbox::new(
                                &mut self.latest_invoice.invoice.partner.is_vat_payer,
                                "Partner is VAT payer",
                            ));
                        });
                        ui.vertical(|ui| {
                            ui.heading("Invoice data");
                            ui.add(egui::TextEdit::singleline(
                                &mut self.latest_invoice.invoice.invoice_number,
                            ))
                            .on_hover_text("Invoice number");
                            ui.add(egui::TextEdit::singleline(
                                &mut self.latest_invoice.invoice.invoice_date,
                            ))
                            .on_hover_text("Invoice date");
                            ui.add(egui::TextEdit::singleline(
                                &mut self.latest_invoice.invoice.service_date,
                            ))
                            .on_hover_text("Invoice service date");
                            ui.add(egui::TextEdit::singleline(
                                &mut self.latest_invoice.invoice.due_date,
                            ))
                            .on_hover_text("Invoice due date");
                            ui.add(egui::TextEdit::singleline(
                                &mut self.latest_invoice.invoice.invoice_location,
                            ))
                            .on_hover_text("Invoice location");
                            ui.add(egui::TextEdit::singleline(
                                &mut self.latest_invoice.invoice.invoice_currency,
                            ))
                            .on_hover_text("Invoice currency (SYMBOL)");
                            ui.add(egui::TextEdit::singleline(
                                &mut self.latest_invoice.invoice.invoice_reference,
                            ))
                            .on_hover_text("Invoice reference");
                            ui.add(egui::TextEdit::singleline(
                                &mut self.latest_invoice.invoice.created_by,
                            ))
                            .on_hover_text("Created by");
                            ui.add(
                                egui::Slider::new(
                                    &mut self.latest_invoice.invoice.invoice_tax,
                                    0.0..=100.0,
                                )
                                .text("Tax rate")
                                .max_decimals(3),
                            )
                            .on_hover_text(
                                "Payment amount without vat. Vat is calculated on the end",
                            );
                        });
                    });
                    if self.add_service {
                        self.render_add_service(ctx);
                    }
                });
                ui.vertical(|ui| {
                    let mut delete = None;
                    egui::ScrollArea::vertical().show(ui, |ui| {
                        ui.vertical_centered(|ui| {
                            ui.heading("Services");
                            for (pos, service) in
                                self.latest_invoice.invoice.services.iter_mut().enumerate()
                            {
                                ui.horizontal(|ui| {
                                    ui.add(egui::TextEdit::multiline(&mut service.service_name))
                                        .on_hover_text("Service description");
                                    ui.add(
                                        egui::Slider::new(
                                            &mut service.service_price,
                                            0.0..=10000.0,
                                        )
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
                if ui.button("Create").clicked() {
                    let result = self.generate_pdf(self.latest_invoice.clone());
                    if result {
                        self.create = false;
                    }
                }
                if ui.button("Close").clicked() {
                    self.create = false
                }
            });
    }
    fn render_create_company(&mut self, ctx: &egui::Context) {
        egui::Window::new("Create Company")
            .resizable(true)
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.with_layout(egui::Layout::left_to_right(Align::Center), |ui| {
                        ui.vertical(|ui| {
                            ui.heading("Company data");
                            ui.add(egui::TextEdit::singleline(
                                &mut self.empty_company.company_name,
                            ))
                            .on_hover_text("Company name");
                            ui.add(egui::TextEdit::singleline(
                                &mut self.empty_company.company_iban,
                            ))
                            .on_hover_text("Company IBAN");
                            ui.add(egui::TextEdit::singleline(
                                &mut self.empty_company.company_swift,
                            ))
                            .on_hover_text("Company SWIFT");
                            ui.add(egui::TextEdit::singleline(
                                &mut self.empty_company.company_bankname,
                            ))
                            .on_hover_text("Company Bank Name");
                            ui.add(egui::TextEdit::singleline(
                                &mut self.empty_company.company_address,
                            ))
                            .on_hover_text("Company address");
                            ui.add(egui::TextEdit::singleline(
                                &mut self.empty_company.company_postal_code,
                            ))
                            .on_hover_text("Company postal code");
                            ui.add(egui::TextEdit::singleline(
                                &mut self.empty_company.company_vat_id,
                            ))
                            .on_hover_text("Company VAT");
                            ui.add(egui::TextEdit::singleline(
                                &mut self.empty_company.company_phone,
                            ))
                            .on_hover_text("Company PHONE");
                            ui.add(egui::TextEdit::singleline(
                                &mut self.empty_company.company_registration_number,
                            ))
                            .on_hover_text("Company Registeration number");
                            ui.add(egui::TextEdit::singleline(
                                &mut self.empty_company.company_business_registered_at,
                            ))
                            .on_hover_text("Company Registered at");
                            ui.add(egui::TextEdit::singleline(
                                &mut self.empty_company.company_currency,
                            ))
                            .on_hover_text("Company currency");

                            if ui.button("Change signature").clicked() {
                                let path = FileDialog::new()
                                    .set_location("~/Desktop")
                                    .add_filter("PNG Image", &["png"])
                                    .add_filter("JPEG Image", &["jpg", "jpeg"])
                                    .show_open_single_file()
                                    .unwrap();
                                let path = match path {
                                    Some(path) => path,
                                    None => PathBuf::new(),
                                };

                                let yes = MessageDialog::new()
                                    .set_type(MessageType::Info)
                                    .set_title("Do you want to open the file?")
                                    .set_text(&format!("{:#?}", path))
                                    .show_confirm()
                                    .unwrap();

                                if yes {
                                    println!("Opening file...");
                                    println!("{}", format!("{:#?}", path));
                                    let base_string =
                                        image_base64::to_base64(path.as_os_str().to_str().unwrap());
                                    self.selected_signature = Some(base_string);
                                    self.signature_path = Some(path.clone());
                                    self.empty_company.company_signature_path = Some(path);
                                }
                            }
                            ui.horizontal(|ui| {
                                if ui.button("Create").clicked() {
                                    self.create_company = false;
                                    self.save_company(self.empty_company.clone());
                                    //Reset the value
                                    self.empty_company = Company::default();
                                }
                                ui.add_space(10.0);
                                if ui.button("Cancel").clicked() {
                                    self.create_company = false;
                                    //Reset the value
                                    self.empty_company = Company::default();
                                }
                            })
                        })
                    });
                })
            });
    }
    fn render_create_partner(&mut self, ctx: &egui::Context) {
        egui::Window::new("Create Partner")
            .resizable(true)
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.with_layout(egui::Layout::left_to_right(Align::Center), |ui| {
                        ui.vertical(|ui| {
                            ui.heading("Partner data");
                            ui.add(egui::TextEdit::singleline(
                                &mut self.empty_partner.partner_name,
                            ))
                            .on_hover_text("Partner name");
                            ui.add(egui::TextEdit::singleline(
                                &mut self.empty_partner.partner_address,
                            ))
                            .on_hover_text("Partner address");
                            ui.add(egui::TextEdit::singleline(
                                &mut self.empty_partner.partner_postal_code,
                            ))
                            .on_hover_text("Partner postal code");
                            ui.add(egui::TextEdit::singleline(
                                &mut self.empty_partner.partner_vat_id,
                            ))
                            .on_hover_text("Partner VAT");

                            ui.horizontal(|ui| {
                                if ui.button("Create").clicked() {
                                    self.create_partner = false;
                                    self.save_partner(self.empty_partner.clone());
                                    //Reset the value
                                    self.empty_partner = Partner::default();
                                }
                                ui.add_space(10.0);
                                if ui.button("Cancel").clicked() {
                                    self.create_partner = false;
                                    //Reset the value
                                    self.empty_partner = Partner::default();
                                }
                            })
                        })
                    });
                })
            });
    }
    fn render_create_service(&mut self, ctx: &egui::Context) {
        egui::Window::new("Create Service")
            .resizable(true)
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.with_layout(egui::Layout::left_to_right(Align::Center), |ui| {
                        ui.vertical(|ui| {
                            ui.add(egui::TextEdit::multiline(
                                &mut self.empty_service.service_name,
                            ))
                            .on_hover_text("Service Name");
                            ui.add(widgets::Slider::new(
                                &mut self.empty_service.service_price,
                                0.0..=1000.0,
                            ))
                            .on_hover_text("Service Price");
                            ui.horizontal(|ui| {
                                if ui.button("Create").clicked() {
                                    self.create_service = false;
                                    self.save_service(self.empty_service.clone());
                                    //Reset the value
                                    self.empty_service = Service::default();
                                }
                                ui.add_space(10.0);
                                if ui.button("Cancel").clicked() {
                                    self.create_service = false;
                                    //Reset the value
                                    self.empty_service = Service::default();
                                }
                            })
                        })
                    });
                })
            });
    }

    fn render_edit_company(&mut self, ctx: &egui::Context) {
        egui::Window::new("Edit company")
            .resizable(true)
            .show(ctx, |ui| {
                ui.vertical(|ui| {
                    ui.with_layout(egui::Layout::left_to_right(Align::Center), |ui| {
                        ui.vertical(|ui| {
                            ui.add(egui::TextEdit::singleline(
                                &mut self.clicked_company.company_name,
                            ))
                            .on_hover_text("Company name");
                            ui.add(egui::TextEdit::singleline(
                                &mut self.clicked_company.company_iban,
                            ))
                            .on_hover_text("Company IBAN");
                            ui.add(egui::TextEdit::singleline(
                                &mut self.clicked_company.company_swift,
                            ))
                            .on_hover_text("Company SWIFT");
                            ui.add(egui::TextEdit::singleline(
                                &mut self.clicked_company.company_bankname,
                            ))
                            .on_hover_text("Company Bank Name");
                            ui.add(egui::TextEdit::singleline(
                                &mut self.clicked_company.company_address,
                            ))
                            .on_hover_text("Company address");
                            ui.add(egui::TextEdit::singleline(
                                &mut self.clicked_company.company_postal_code,
                            ))
                            .on_hover_text("Company postal code");
                            ui.add(egui::TextEdit::singleline(
                                &mut self.clicked_company.company_vat_id,
                            ))
                            .on_hover_text("Company VAT");
                            ui.add(egui::TextEdit::singleline(
                                &mut self.clicked_company.company_phone,
                            ))
                            .on_hover_text("Company PHONE");
                            ui.add(egui::TextEdit::singleline(
                                &mut self.clicked_company.company_registration_number,
                            ))
                            .on_hover_text("Company Registeration number");
                            ui.add(egui::TextEdit::singleline(
                                &mut self.clicked_company.company_business_registered_at,
                            ))
                            .on_hover_text("Company Registered at");
                            ui.add(egui::TextEdit::singleline(
                                &mut self.clicked_company.company_currency,
                            ))
                            .on_hover_text("Company currency");
                            ui.add_space(10.0);
                            if ui.button("Change signature").clicked() {
                                let path = FileDialog::new()
                                    .set_location("~/Desktop")
                                    .add_filter("PNG Image", &["png"])
                                    .add_filter("JPEG Image", &["jpg", "jpeg"])
                                    .show_open_single_file()
                                    .unwrap();
                                let path = match path {
                                    Some(path) => path,
                                    None => PathBuf::new(),
                                };

                                let yes = MessageDialog::new()
                                    .set_type(MessageType::Info)
                                    .set_title("Do you want to open the file?")
                                    .set_text(&format!("{:#?}", path))
                                    .show_confirm()
                                    .unwrap();

                                if yes {
                                    println!("Opening file...");
                                    println!("{}", format!("{:#?}", path));
                                    let base_string =
                                        image_base64::to_base64(path.as_os_str().to_str().unwrap());
                                    self.selected_signature = Some(base_string);
                                    self.signature_path = Some(path.clone());
                                    self.clicked_company.company_signature_path = Some(path);
                                }
                            }
                            ui.horizontal(|ui| {
                                if ui.button("Edit").clicked() {
                                    self.edit_company = false;
                                    //Edit the invoice in the self.invoices vector
                                    let index = self
                                        .companies
                                        .iter()
                                        .position(|x| x.id == self.clicked_company.id)
                                        .unwrap();
                                    self.companies[index] = self.clicked_company.clone();
                                    self.clicked_company = Company::default();
                                }
                                ui.add_space(10.0);
                                if ui.button("Cancel").clicked() {
                                    self.edit_company = false;
                                    //Reset the value
                                    self.clicked_company = Company::default();
                                }
                            })
                        });
                    });
                });
            });
    }

    fn render_edit_partner(&mut self, _ctx: &egui::Context) {
        todo!()
    }

    fn render_edit_service(&mut self, _ctx: &egui::Context) {
        todo!()
    }

    fn delete_invoice(&mut self, _invoice: Racun) {
        for _invoice in self.invoice_paths.iter_mut() {
            todo!();
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
        if self.refresh {
            if self.get_invoices().is_some() {
                self.invoice_paths = self.get_invoices().unwrap();
                self.parse_jsons();
                self.refresh = false;
            } else {
                self.invoice_paths = Vec::new();
                self.parse_jsons();
                self.refresh = false;
            }
        }
        egui::CentralPanel::default().show(ctx, |ui| {
            self.refresh = true;
            egui::ScrollArea::new([false, false]).show(ui, |ui| {
                self.render_header(ui, ctx);
                self.render_main(ui, ctx);
                self.render_footer(ui);
            });
        });
        if self.edit {
            self.render_edit_invoice(ctx);
        }
        if self.manage_companies {
            self.render_manage_companies(ctx);
        }
        if self.create_company {
            self.render_create_company(ctx);
        }
        if self.create_partner {
            self.render_create_partner(ctx);
        }
        if self.manage_partners {
            self.render_manage_partners(ctx);
        }
        if self.manage_services {
            self.render_manage_services(ctx);
        }
        if self.create_service {
            self.render_create_service(ctx);
        }
        if self.change_company {
            self.render_change_company(ctx);
        }
        if self.change_partner {
            self.render_change_partner(ctx);
        }
        if self.create {
            self.render_create_invoice(ctx);
        }
        if self.edit_company {
            self.render_edit_company(ctx)
        }
        if self.edit_partner {
            self.render_edit_partner(ctx);
        }
        if self.edit_service {
            self.render_edit_service(ctx);
        }

        if self.show_image {
            if self.texture.is_some() {
                if let Some(_texture) = &mut self.texture {
                    self.render_image_window(ctx);
                }
            } else {
                self.texture = None;
            }
        } else {
            self.show_image = false;
        }
        if self.show_confirmation_dialog {
            // Show confirmation dialog:
            self.render_confirmation_dialog(ctx, frame);
        }
    }
}

pub fn entry() {
    //*!TODO This doesnt work yet the icon gets loaded but not shown also only works on windows */
    let icon = IconData {
        rgba: include_bytes!("../../assets/logo.jpg").to_vec(),
        width: 512,
        height: 512,
    };
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(900.0, 700.0)),
        icon_data: Some(icon),
        ..Default::default()
    };

    let app = GuiApp::new();

    eframe::run_native("Invoice GUI", options, Box::new(|_cc| Box::new(app))).unwrap();
}
