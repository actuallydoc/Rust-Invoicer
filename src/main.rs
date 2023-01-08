use printpdf::*;
use serde::{Deserialize, Serialize};
use std::fs::read_to_string;
use std::fs::File;
use std::io::BufWriter;
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FontSizes {
    pub small: f64,
    pub medium: f64,
    pub large: f64,
}

impl FontSizes {
    //Pull the sizes from the JSON
    pub fn new() -> Self {
        let data = read_to_string("structure.json").expect("Unable to read file");
        let parsed: Self = serde_json::from_str(&data).expect("JSON does not have correct format.");
        //println!("{:?}", parsed);
        FontSizes {
            small: parsed.small,
            medium: parsed.medium,
            large: parsed.large,
        }
    }
}
#[derive(Serialize, Deserialize, Debug)]
struct InvoiceStructure {}
#[derive(Serialize, Deserialize, Debug)]

struct Partner {
    partnerName: String,
    partnerAddress: String,
    partnerPostal_code: String,
    partnerVat_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Services {
    services: Vec<Service>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Service {
    serviceName: String,
    serviceQuantity: f64,
    servicePrice: f64,
    serviceTax: f64,
    servicePayment: f64,
}
impl Services {
    fn new() -> Self {
        let data = read_to_string("structure.json").expect("Unable to read file");
        let parsed: Self = serde_json::from_str(&data).expect("JSON does not have correct format.");
        // println!("{:#?}", parsed);
        Services {
            services: parsed.services,
        }
    }
}
#[derive(Serialize, Deserialize, Debug)]
struct Company {
    companyCurrency: String,
    companyName: String,
    companyAddress: String,
    companyPostal_code: String,
    companyVat_id: String,
    companyIban: String,
    companySwift: String,
    companyRegistration_number: String,
    companyPhone: String,
    companySignature: String, //Base64 string
    companyVat_rate: f64,
}
impl Company {
    fn new() -> Self {
        let data = read_to_string("structure.json").expect("Unable to read file");
        let parsed: Self = serde_json::from_str(&data).expect("JSON does not have correct format.");

        Company {
            companyCurrency: parsed.companyCurrency,
            companyName: parsed.companyName,
            companyAddress: parsed.companyAddress,
            companyPostal_code: parsed.companyPostal_code,
            companyVat_id: parsed.companyVat_id,
            companyIban: parsed.companyIban,
            companySwift: parsed.companySwift,
            companyRegistration_number: parsed.companyRegistration_number,
            companyPhone: parsed.companyPhone,
            companySignature: parsed.companySignature,
            companyVat_rate: parsed.companyVat_rate,
        }
    }
}
#[derive(Serialize, Deserialize, Debug)]
struct Racun {
    sizes: FontSizes,
    invoice_number: i32,
    invoice_date: String,
    service_date: String,
    due_date: String,
    partner: Partner,
    company: Company,
    services: Services,
}

impl Partner {
    fn new() -> Self {
        let data = read_to_string("structure.json").expect("Unable to read file");
        let parsed: Self = serde_json::from_str(&data).expect("JSON does not have correct format.");

        Partner {
            partnerName: parsed.partnerName,
            partnerAddress: parsed.partnerAddress,
            partnerPostal_code: parsed.partnerPostal_code,
            partnerVat_id: parsed.partnerVat_id,
        }
    }
}

impl Racun {
    fn new() -> Self {
        Racun {
            sizes: FontSizes::new(),
            invoice_number: 31,
            invoice_date: "123".to_string(),
            service_date: "123".to_string(),
            due_date: "123".to_string(),
            partner: Partner::new(),
            company: Company::new(),
            services: Services::new(),
        }
    }
}
fn main() {
    let mut fresh_racun = Racun::new();

    println!("{:#?}", fresh_racun);

    //Document entry
    let (doc, page1, layer1) = PdfDocument::new(
        fresh_racun.invoice_number.to_string(),
        Mm(210.0),
        Mm(297.0),
        "Layer 1",
    );
    //Font entry
    let bold_font = doc
        .add_external_font(File::open("fonts/DejaVuSans-Bold.ttf").unwrap())
        .unwrap();
    let standard_font = doc.add_external_font(File::open("fonts/DejaVuSans.ttf").unwrap());

    let current_layer = doc.get_page(page1).get_layer(layer1);
    //Start of text
    current_layer.begin_text_section();

    //current_layer.use_text("Hello", small_fize_font, Mm(10.0), Mm(10.0), &bold_font);

    //Save pdf entry
    doc.save(&mut BufWriter::new(
        File::create(format!("račun {}.pdf", fresh_racun.invoice_number)).unwrap(),
    ))
    .unwrap();
}
