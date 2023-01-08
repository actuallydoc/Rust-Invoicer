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
    //Save pdf entry
    doc.save(&mut BufWriter::new(
        File::create(format!("račun {}.pdf", fresh_racun.invoice.invoice_number)).unwrap(),
    ))
    .unwrap();
}

fn render_company_header(
    layer: &PdfLayerReference,
    racun: &Racun,
    standard_font: &IndirectFontRef,
    bold_font: &IndirectFontRef,
) {
    layer.use_text(
        format!("{}", racun.invoice.company.company_name),
        racun.config.font_sizes.small,
        Mm(132.0),
        Mm(276.0),
        &bold_font,
    );
    layer.use_text(
        format!("{}", racun.invoice.company.company_address),
        racun.config.font_sizes.small,
        Mm(132.0),
        Mm(271.0),
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
