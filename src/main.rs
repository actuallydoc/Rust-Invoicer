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

    current_layer.use_text(
        "Hello",
        fresh_racun.config.font_sizes.small,
        Mm(20.0),
        Mm(20.0),
        &bold_font,
    );
    current_layer.use_text(
        "Hello",
        fresh_racun.config.font_sizes.small,
        Mm(10.0),
        Mm(10.0),
        &bold_font,
    );
    //Save pdf entry
    doc.save(&mut BufWriter::new(
        File::create(format!("račun {}.pdf", fresh_racun.invoice.invoice_number)).unwrap(),
    ))
    .unwrap();
}
