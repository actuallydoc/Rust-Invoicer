use crate::invoicer::Racun;
use printpdf::*;
use std::fs::File;
use std::io::BufWriter;
mod invoicer;

fn main() {
    let mut fresh_racun = Racun::parse_from_file();

    //Document entry
    let (doc, page1, layer1) = PdfDocument::new(
        fresh_racun.invoice.invoice_number.to_string(),
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

    render_invoice_header(&current_layer, &fresh_racun, &standard_font);
    render_company_header(&current_layer, &fresh_racun, &standard_font, &bold_font);
    render_partner_header(&current_layer, &fresh_racun, &standard_font);
    let (x, y) = render_table_header(&current_layer, &fresh_racun, &bold_font);
    render_table_contents(&current_layer, &fresh_racun, &standard_font, x, y);
    //Save pdf entry
    doc.save(&mut BufWriter::new(
        File::create(format!("račun {}.pdf", fresh_racun.invoice.invoice_number)).unwrap(),
    ))
    .unwrap();
}

fn render_table_contents(
    layer: &PdfLayerReference,
    racun: &Racun,
    bold: &IndirectFontRef,
    mut x: Mm,
    mut y: Mm,
) {
    //TODO
    todo!()
}

fn render_table_header(
    layer: &PdfLayerReference,
    racun: &Racun,
    bold: &IndirectFontRef,
) -> (Mm, Mm) {
    //Opis
    let mut y = Mm(193.0);
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
    x += Mm(18.0);
    layer.use_text("Znesek", racun.config.font_sizes.small, x, y, &bold);

    (x, y)
}

fn render_partner_header(
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

fn render_company_header(
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

fn render_invoice_header(
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
