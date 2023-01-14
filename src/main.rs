use crate::invoicer::Racun;
use gui::entry;
use invoicer::init;
use render::export_pdf_to_jpegs;
use std::env;
mod gui;
mod invoicer;
mod render;
fn main() {
    let fresh_racun = Racun::parse_from_file();
    //Todo sign field and add a base64 for a sign.
    init(&fresh_racun);

    //Gui entry
    // entry();

    //1.Have all the fields that are needed to create an invoice.
    //2.Save the json file everytime you type something or change something. Maybe use json only for the current data that is inside the gui and database for saving it..
    //3.Discord RPC intergration
    //4.Database soon or later.
    //5. PDF viewer inside the gui.
    let mut path = env::current_dir().unwrap();
    let invoice_folder = "invoices";

    path.push(invoice_folder);
    path.push("1\\račun 1.pdf");
    println!("Path: {:?}", path);

    // export_pdf_to_jpegs(
    //     "C://Users//Maj//Desktop//rust_pdf//invoices//1//račun 1.pdf",
    //     None,
    //     125,
    // )
    // .unwrap();
    //6.?
    //7.?
}
