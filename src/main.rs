use crate::invoicer::Racun;
use invoicer::init;
mod invoicer;

fn main() {
    let fresh_racun = Racun::parse_from_file();
    //Document generation
    init(&fresh_racun);
}
