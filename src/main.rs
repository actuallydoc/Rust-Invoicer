use crate::invoicer::Racun;
use gui::entry;
use invoicer::init;

mod gui;
mod invoicer;
mod render;
mod rpc;
fn main() {
    let fresh_racun = Racun::parse_from_file();
    //Todo sign field and add a base64 for a sign.
    // init(&fresh_racun); This function makes the invoice
    //Gui entry
    entry();
    //2.Save the json file everytime you type something or change something. Maybe use json only for the current data that is inside the gui and database for saving it..
    //3.Discord RPC intergration
    //4.Database soon or later.
    //5. PDF viewer inside the gui.
}
