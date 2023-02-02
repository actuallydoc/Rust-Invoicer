use chrono::Datelike;
use printpdf::*;
use serde::{Deserialize, Serialize};
use std::{
    env,
    fmt::Display,
    fs::{self, read_to_string, File},
    io::{BufWriter, Write},
    path::PathBuf,
};

use crate::render::export_pdf_to_jpegs;
#[derive(Default, Serialize, Deserialize, Debug, Clone)]
pub enum PaymentStatus {
    PAID,
    #[default]
    UNPAID,
}
impl Display for PaymentStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PaymentStatus::PAID => write!(f, "PAID"),
            PaymentStatus::UNPAID => write!(f, "UNPAID"),
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, Copy)]
#[serde(rename_all = "camelCase")]
pub struct FontSizes {
    pub small: f64,
    pub medium: f64,
    pub large: f64,
}

#[derive(Default, Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct InvoiceStructure {
    pub font_sizes: FontSizes,
}
#[derive(Default, Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Partner {
    pub partner_name: String,
    pub partner_address: String,
    pub partner_postal_code: String,
    pub partner_vat_id: String,
}

#[derive(Default, Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Service {
    pub service_name: String,
    pub service_quantity: i32,
    pub service_price: f64,
    pub service_tax: f64,
    pub service_currency: String,
}

#[derive(Default, Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Company {
    pub company_currency: String,
    pub company_name: String,
    pub company_address: String,
    pub company_postal_code: String,
    pub company_bankname: String,
    pub company_vat_id: String,
    pub company_iban: String,
    pub company_swift: String,
    pub company_registration_number: String,
    pub company_phone: String,
    pub company_signature: String, //Base64 string
    pub company_vat_rate: f64,
    pub company_business_registered_at: String,
}
impl Company {}
#[derive(Default, Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Racun {
    pub invoice: Invoice,
    pub config: InvoiceStructure,
}
#[derive(Default, Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Invoice {
    pub invoice_number: i32,
    pub invoice_date: String,
    pub invoice_location: String,
    pub service_date: String,
    pub invoice_currency: String,
    pub due_date: String,
    pub partner: Partner,
    pub company: Company,
    pub invoice_tax: f64,
    pub invoice_reference: String,
    pub services: Vec<Service>,
    pub created_by: String,
    pub status: PaymentStatus,
}

impl Racun {
    pub fn parse_from_file() -> Self {
        let data = read_to_string("data.json").expect("Cannot read file");
        let parsed: Self = serde_json::from_str(&data).expect("JSON does not have correct format.");
        parsed
    }
}
//Helper functions
fn make_line(layer: &PdfLayerReference, x1: Mm, y1: Mm, x2: Mm, y2: Mm) {
    let line_points = vec![(Point::new(x1, y1), false), (Point::new(x2, y2), false)];
    let line = Line {
        points: line_points,
        is_closed: true,
        has_fill: true,
        has_stroke: false,
        is_clipping_path: false,
    };
    layer.add_shape(line);
}

pub fn render_footer(
    layer: &PdfLayerReference,
    racun: &Racun,
    standard_font: &IndirectFontRef,
    y: Mm,
) {
    let y = y - Mm(97.0);

    make_line(&layer, Mm(13.0), y + Mm(2.0), Mm(197.0), y + Mm(2.0));

    let y = y - Mm(1.0);
    layer.use_text(
        format!(
            "Vpis v poslovni register pri {}. Matična št: {}",
            racun.invoice.company.company_business_registered_at,
            racun.invoice.company.company_registration_number
        ),
        9.0,
        Mm(65.0),
        y,
        standard_font,
    )
}
pub fn render_payment_footer(
    layer: &PdfLayerReference,
    racun: &Racun,
    standard_font: &IndirectFontRef,
    y: Mm,
) -> Mm {
    let current_date = chrono::Utc::now().year();
    let mut y = y - Mm(10.0);
    let base_x = Mm(15.0);
    layer.use_text(
        format!(
            "Sklic za številko: {}00 {:04}-{}",
            racun.invoice.invoice_reference, racun.invoice.invoice_number, current_date
        ),
        9.0,
        base_x,
        y,
        standard_font,
    );
    y = y - Mm(3.0);
    layer.use_text(
        format!("Sestavil: {}", racun.invoice.created_by),
        9.0,
        base_x,
        y,
        standard_font,
    );
    y = y - Mm(4.0);

    //Payment info /method
    layer.use_text(
        format!(
            "Plačilo na TRR: {} {} ., SWIFT: {}",
            racun.invoice.company.company_iban,
            racun.invoice.company.company_bankname,
            racun.invoice.company.company_swift
        ),
        9.0,
        base_x,
        y,
        standard_font,
    );

    y
}

pub fn render_summary_table(
    layer: &PdfLayerReference,
    racun: &Racun,
    standard_font: &IndirectFontRef,
    bold_font: &IndirectFontRef,
    y: Mm,
    total_price: f64,
    calculated_tax_difference: f64,
    total_price_with_tax: f64,
) -> Mm {
    let y = y - Mm(15.0);
    make_line(&layer, Mm(13.0), y, Mm(197.0), y);

    //Adding text "Davčna stopnja", "Osnova za DDV", "DDV", "Znesek z DDV"
    let mut y = y - Mm(3.0);
    let tax_x = Mm(14.0);
    let base_tax_x = Mm(70.0);
    let tax_difference_x = Mm(125.0);
    let total_price_x = Mm(150.0);
    layer.use_text("Davčna stopnja", 9.0, tax_x, y, bold_font);

    layer.use_text("Osnova za DDV", 9.0, base_tax_x, y, bold_font);

    layer.use_text("DDV", 9.0, tax_difference_x, y, bold_font);

    layer.use_text("Znesek z DDV", 9.0, total_price_x, y, bold_font);
    y = y - Mm(4.0);
    layer.use_text(
        format!("DDV {}%", racun.invoice.invoice_tax),
        9.0,
        tax_x,
        y,
        standard_font,
    );
    layer.use_text(
        format!("{:.2}{}", total_price, racun.invoice.invoice_currency),
        9.0,
        base_tax_x,
        y,
        standard_font,
    );

    layer.use_text(
        format!(
            "{:.2?}{}",
            total_price_with_tax, racun.invoice.invoice_currency
        ),
        9.0,
        total_price_x,
        y,
        standard_font,
    );
    layer.use_text(
        format!(
            "{:.2}{}",
            calculated_tax_difference, racun.invoice.invoice_currency
        ),
        9.0,
        tax_difference_x,
        y,
        standard_font,
    );

    y
}

pub fn render_service(
    x: Mm,
    mut y: Mm,
    layer: &PdfLayerReference,
    font: &IndirectFontRef,
    service: &Service,
) -> (Mm, Mm) {
    //Converting it to float and getting total price of services multiplied by quantity
    let service_by_quantity_price = service.service_price * (service.service_quantity as f64);
    //Adding a vat percentage price to the service price
    let new_value = add_percent(service_by_quantity_price, 0.22);
    //Render service with a price and ddv percentage
    //Always a constant
    let service_x = Mm(180.0);
    //Rendering price that has to be paid included with tax
    layer.use_text(
        format!("{:.2}{}", new_value, service.service_currency),
        9.0,
        service_x,
        y,
        font,
    );
    //Render service DDV percentage
    //Always a constant
    let ddv_x = Mm(165.0);
    //Formated text add a percentage sign to the service_tax string
    let formated_vat = format!("{}%", service.service_tax);

    layer.use_text(formated_vat, 9.0, ddv_x, y, font);
    //Render service price
    //Always a constant
    let price_x = Mm(145.0);
    //Convert a float to an int

    let formated_price = format!(
        "{:.2}{}",
        service_by_quantity_price, service.service_currency
    );
    layer.use_text(formated_price, 9.0, price_x, y, font);

    //Render service quantity
    //Always a constant
    let quantity_x = Mm(125.0);
    layer.use_text(
        &service.service_quantity.to_string(),
        9.0,
        quantity_x,
        y,
        font,
    );
    for line in service.service_name.lines() {
        layer.use_text(line, 9.0, x, y, font);
        y -= Mm(4.0);
    }
    //Render a line under the service
    make_line(layer, Mm(13.0), y, Mm(197.0), y);
    (x, y - Mm(4.0))
}

pub fn render_table_contents(
    layer: &PdfLayerReference,
    racun: &Racun,
    standard_font: &IndirectFontRef,
) -> (Mm, f64, f64, f64) {
    let mut x = Mm(15.0);
    let mut y = Mm(185.0);
    let mut total_price = 0.0;
    //Render services with the lines above
    for service in racun.invoice.services.iter() {
        total_price += service.service_price * (service.service_quantity as f64);
        let (new_x, new_y) = render_service(x, y, layer, standard_font, service);
        x = new_x;
        y = new_y;
    }
    let final_table_y = render_table_end(y, layer, racun, standard_font, total_price);
    //Render Total price , Tax price and Total price with tax
    final_table_y //Updated y and prices to put them into the summary table
}

pub fn render_table_end(
    y: Mm,
    layer: &PdfLayerReference,
    racun: &Racun,
    font: &IndirectFontRef,
    total_price: f64,
) -> (Mm, f64, f64, f64) {
    //Constant location of the Field
    let x = Mm(165.0);
    //Render total price without tax
    layer.use_text(
        format!(
            "Skupaj: {:.2}{}",
            total_price, racun.invoice.invoice_currency
        ),
        9.0,
        x,
        y,
        font,
    );

    make_line(layer, Mm(165.0), y - Mm(1.0), Mm(195.0), y - Mm(1.0));
    ///////////////////////////////////////////
    //Decrease the Y by a couple of Mm
    let y = y - Mm(4.0);
    //Render tax
    //Always a constant
    let tax_x = Mm(165.0);
    //Calculate how much is the tax difference

    let total_price_with_tax = total_price * (1.00 + (racun.invoice.invoice_tax / 100.00));
    let calculated_tax_difference = total_price_with_tax - total_price;
    layer.use_text(
        format!(
            "DDV: {:.2}{}",
            calculated_tax_difference, racun.invoice.invoice_currency
        ),
        9.0,
        tax_x,
        y,
        font,
    );

    make_line(layer, Mm(165.0), y - Mm(1.0), Mm(195.0), y - Mm(1.0));

    //To pay field
    let y = y - Mm(4.0);
    let to_pay_x = Mm(165.0);
    layer.use_text(
        format!(
            "Za plačilo: {:.2}{}",
            total_price_with_tax, racun.invoice.invoice_currency
        ),
        9.0,
        to_pay_x,
        y,
        font,
    );
    //Decrease the Y by a couple of Mm
    let y = y - Mm(1.0);
    make_line(layer, Mm(165.0), y, Mm(195.0), y);
    return (
        y,
        total_price,
        calculated_tax_difference,
        total_price_with_tax,
    );
}

pub fn render_table_header(layer: &PdfLayerReference, racun: &Racun, bold: &IndirectFontRef) {
    //Opis
    let y = Mm(193.0);
    let mut x = Mm(15.0);
    layer.use_text("Opis", racun.config.font_sizes.small, x, y, &bold);

    //Količina
    x += Mm(105.0);
    layer.use_text("Količina", racun.config.font_sizes.small, x, y, &bold);

    //Cena
    x += Mm(22.0);
    layer.use_text("Cena", racun.config.font_sizes.small, x, y, &bold);

    //DDV
    x += Mm(22.0);
    layer.use_text("DDV", racun.config.font_sizes.small, x, y, &bold);

    //Znesek
    x += Mm(15.0);
    layer.use_text("Znesek", racun.config.font_sizes.small, x, y, &bold);

    make_line(layer, Mm(13.0), Mm(190.0), Mm(197.0), Mm(190.0));
}

pub fn render_partner_header(
    layer: &PdfLayerReference,
    racun: &Racun,
    standard_font: &IndirectFontRef,
) {
    //Partner name
    layer.use_text(
        format!("{}", racun.invoice.company.company_name),
        racun.config.font_sizes.small,
        Mm(15.0),
        Mm(233.0),
        &standard_font,
    );
    //Partner address
    layer.use_text(
        format!("{}", racun.invoice.partner.partner_address),
        racun.config.font_sizes.small,
        Mm(15.0),
        Mm(228.0),
        &standard_font,
    );
    //Partner postal code with city
    layer.use_text(
        format!("{}", racun.invoice.partner.partner_postal_code),
        racun.config.font_sizes.small,
        Mm(15.0),
        Mm(223.0),
        &standard_font,
    );

    //Partner tax number
    layer.use_text(
        format!(
            "ID za DDV kupca: SI {}",
            racun.invoice.partner.partner_vat_id
        ),
        racun.config.font_sizes.small,
        Mm(15.0),
        Mm(202.0),
        &standard_font,
    );
}

pub fn render_company_header(
    layer: &PdfLayerReference,
    racun: &Racun,
    standard_font: &IndirectFontRef,
    bold_font: &IndirectFontRef,
) {
    //Company name
    layer.use_text(
        format!("{}", racun.invoice.company.company_name),
        racun.config.font_sizes.small,
        Mm(132.0),
        Mm(276.0),
        &bold_font,
    );
    //Company address
    layer.use_text(
        format!("{}", racun.invoice.company.company_address),
        racun.config.font_sizes.small,
        Mm(132.0),
        Mm(271.0),
        &standard_font,
    );
    //Company postal code with address
    layer.use_text(
        format!("{}", racun.invoice.company.company_postal_code),
        racun.config.font_sizes.small,
        Mm(132.0),
        Mm(267.0),
        &standard_font,
    );

    //Company tax number
    layer.use_text(
        format!("ID za DDV: SI{}", racun.invoice.company.company_vat_id),
        racun.config.font_sizes.small,
        Mm(132.0),
        Mm(263.0),
        &standard_font,
    );

    //Company bank account
    layer.use_text(
        format!("BAN št: {}", racun.invoice.company.company_iban),
        racun.config.font_sizes.small,
        Mm(132.0),
        Mm(259.0),
        &standard_font,
    );
    //Company swift
    layer.use_text(
        format!("SWIFT: {}", racun.invoice.company.company_swift),
        racun.config.font_sizes.small,
        Mm(132.0),
        Mm(255.0),
        &standard_font,
    );
    //Company registration number
    layer.use_text(
        format!(
            "Matična št: {}",
            racun.invoice.company.company_registration_number
        ),
        racun.config.font_sizes.small,
        Mm(132.0),
        Mm(251.0),
        &standard_font,
    );
    //Company phone
    layer.use_text(
        format!("Tel: {}", racun.invoice.company.company_phone),
        racun.config.font_sizes.small,
        Mm(132.0),
        Mm(247.0),
        &standard_font,
    );
    //Invoice number
    layer.use_text(
        format!("Račun št: {}", racun.invoice.invoice_number),
        racun.config.font_sizes.small,
        Mm(132.0),
        Mm(243.0),
        &standard_font,
    );
}

fn add_percent(original_value: f64, percent: f64) -> f64 {
    let percent_value = original_value * percent;
    original_value + percent_value
}

pub fn render_invoice_header(
    layer: &PdfLayerReference,
    racun: &Racun,
    standard_font: &IndirectFontRef,
) {
    //Datum izdaje računa
    layer.use_text(
        format!(
            "Datum izdaje: {}, {}",
            racun.invoice.invoice_location, racun.invoice.invoice_date
        ),
        racun.config.font_sizes.small,
        Mm(15.0),
        Mm(274.0),
        &standard_font,
    );
    //Datum opravljene storitve
    layer.use_text(
        format!("Datum opr. storitve: {}", racun.invoice.service_date),
        racun.config.font_sizes.small,
        Mm(15.0),
        Mm(270.0),
        &standard_font,
    );
    //Rok plačila
    layer.use_text(
        format!("Rok plačila: {}", racun.invoice.due_date),
        racun.config.font_sizes.small,
        Mm(15.0),
        Mm(266.0),
        &standard_font,
    );
}
pub fn init(racun: Racun) {
    let (doc, page1, layer1) = PdfDocument::new(
        racun.invoice.invoice_number.to_string(),
        Mm(210.0), //Page size A4
        Mm(297.0), //Page size A4
        "Layer 1",
    );
    //Font entry
    let bold_font = doc
        .add_external_font(File::open("fonts/DejaVuSans-Bold.ttf").expect("Could't open font file"))
        .unwrap();
    let standard_font = doc
        .add_external_font(File::open("fonts/DejaVuSans.ttf").expect("Could't open font file"))
        .unwrap();
    let current_layer = doc.get_page(page1).get_layer(layer1);
    //Start of text
    current_layer.begin_text_section();
    render_invoice_header(&current_layer, &racun, &standard_font);
    render_company_header(&current_layer, &racun, &standard_font, &bold_font);
    render_partner_header(&current_layer, &racun, &standard_font);
    render_table_header(&current_layer, &racun, &bold_font);
    let (y, total_price, calculated_tax_difference, total_price_with_tax) =
        render_table_contents(&current_layer, &racun, &standard_font);
    let y = render_summary_table(
        &current_layer,
        &racun,
        &standard_font,
        &bold_font,
        y,
        total_price,
        calculated_tax_difference,
        total_price_with_tax,
    );

    //Make payment footer
    let y = render_payment_footer(&current_layer, &racun, &standard_font, y);
    render_footer(&current_layer, &racun, &standard_font, y);
    //Save pdf entry and return the path to the pdf file
    let path = save_invoice(doc, &racun);

    match path {
        Some((a, jpg_file_path)) => {
            match export_pdf_to_jpegs(
                a.to_str().expect("Coulnd't convert to &str"),
                jpg_file_path.to_str().expect("Coulnd't convert to &str"),
                None,
                racun.invoice.invoice_number,
            ) {
                Ok(_) => {
                    println!("Invoice image saved ✔");
                    println!("Invoice saved {}", jpg_file_path.to_str().unwrap());
                    save_to_json(&racun);
                    println!("Invoice json saved ✔");
                }
                Err(e) => println!("Error saving invoice image: {}", e),
            }
        }
        None => println!("Error Already exists or couldn't save pdf"),
    }
    //Save the json data to output.json
    save_to_json(&racun);
}

pub fn save_to_json(racun: &Racun) {
    let cwd = env::current_dir().expect("Couldn't get current directory");
    let invoice_dir = cwd.join("invoices");
    let invoice_number_dir = invoice_dir.join(racun.invoice.invoice_number.to_string());

    if !invoice_number_dir.exists() {
        fs::create_dir(&invoice_number_dir).expect("Couldn't create invoice directory");
    }
    let invoice_json = invoice_number_dir.join("output.json");
    if invoice_number_dir.exists() {
        //Convert the struct into json string
        let json = serde_json::to_string(&racun).expect("Couldn't convert to json");
        //Write the json string to the file
        let mut file = File::create(invoice_json).unwrap();
        file.write_all(json.as_bytes()).unwrap();
        println!("✔");
    } else {
        panic!("The invoice number directory doesn't exist");
    }
}

pub fn save_invoice(doc: PdfDocumentReference, racun: &Racun) -> Option<(PathBuf, PathBuf)> {
    //Firstly make a new directory in the invoice directory and the name is the invoice number
    //Then save the invoice in that directory
    let cwd = env::current_dir().expect("Couldn't get current directory");
    let invoice_dir = cwd.join("invoices");
    let invoice_number_dir = invoice_dir.join(racun.invoice.invoice_number.to_string());

    if !invoice_dir.exists() {
        // println!("Creating invoice directory");
        fs::create_dir(invoice_dir).expect("❌");
        print!("Created invoice directory ✔");
    }
    if invoice_number_dir.exists() {
        None
    } else {
        fs::create_dir(&invoice_number_dir).expect("The invoice number directory already exists");
        let pdf_path =
            invoice_number_dir.join(format!("racun {}.pdf", racun.invoice.invoice_number));
        doc.save(&mut BufWriter::new(File::create(&pdf_path).unwrap()))
            .expect("Couldn't save pdf file");
        println!("✔");
        Some((pdf_path, invoice_number_dir))
    }
}
