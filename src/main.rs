extern crate invoices;

use std::env;
use std::error::Error;
use std::path::Path;

pub fn generate_invoice() -> Result<(), Box<Error>> {
    let path = Path::new(".");
    let index = invoices::args::parse(env::args().skip(1))?;
    let invoice = invoices::read_invoice(path, index)?;
    invoices::generate_invoice(path, &invoice)?;
    Ok(())
}

fn main() {
    generate_invoice().unwrap();
}
