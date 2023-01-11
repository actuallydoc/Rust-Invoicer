use crate::invoicer::Racun;
use gui::entry;
use invoicer::init;
mod gui;
mod invoicer;
fn main() {
    let fresh_racun = Racun::parse_from_file();
    //Document generation with the json data
    //Todo sign field and add a base64 for a sign.
    init(&fresh_racun);

    entry();
    //1.Create a gui
    //GUI FUNCTIONS
    //1.Have all the fields that are needed to create an invoice.
    //2.Save the json file everytime you type something or change something. Maybe use json only for the current data that is inside the gui and database for saving it..
    //3.Discord RPC intergration
    //4.Database soon or later.
    //5.?
    //6.?
    //7.?
}
