extern crate csv as csv_parser;

extern crate chrono;
extern crate kuchiki;
#[macro_use]
extern crate log;
#[macro_use]
extern crate maplit;
#[macro_use]
extern crate markup5ever;
#[macro_use]
extern crate serde_derive;

use invoice::Invoice;
use std::error::Error;
use std::fmt;
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, ExitStatus};

pub use csv::read_invoice;

pub mod args;
mod csv;
mod html;
pub mod invoice;
mod json;

#[derive(Debug)]
struct PdfCreationError(ExitStatus);

impl fmt::Display for PdfCreationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl Error for PdfCreationError {
    fn description(&self) -> &str {
        "failed to create PDF"
    }
}

fn generate_pdf(path: &Path) -> Result<PathBuf, Box<dyn Error>> {
    trace!("generate_pdf");

    let output = path.with_extension("pdf");

    let mut child = Command::new("prince").arg(path).arg("-o").arg(&output).spawn()?;

    let ecode = child.wait()?;

    if !ecode.success() {
        return Err(Box::new(PdfCreationError(ecode)));
    }

    Ok(output)
}

pub fn generate_invoice(path: &Path, invoice: &Invoice) -> Result<(), Box<dyn Error>> {
    trace!("do_generate_invoice");

    let result = invoice.generate_invoice()?;

    let path = path.join(invoice.index.filename());
    result.serialize_to_file(&path)?;

    let pdf = generate_pdf(&path)?;

    let json = json::generate_json(&invoice, &pdf)?;
    let jsonpath = path.with_extension("json");
    let mut jsonfile = File::create(jsonpath)?;
    jsonfile.write_all(json.as_ref())?;

    Ok(())
}
