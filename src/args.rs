use invoice::InvoiceIndex;
use std::error::Error;
use std::fmt;

#[derive(Debug)]
struct WrongArguments(&'static str);

impl fmt::Display for WrongArguments {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl Error for WrongArguments {
    fn description(&self) -> &str {
        self.0
    }
}

pub fn parse<I>(args: I) -> Result<InvoiceIndex, Box<dyn Error>>
where
    I: IntoIterator<Item = String>,
{
    let mut year = None;
    let mut index = None;

    for (i, arg) in args.into_iter().enumerate() {
        match i {
            0 => year = Some(arg.parse()?),
            1 => index = Some(arg.parse()?),
            _ => Err(WrongArguments("too many arguments"))?,
        }
    }

    Ok(InvoiceIndex {
        year: year.ok_or(WrongArguments("missing year"))?,
        index: index.ok_or(WrongArguments("missing index"))?,
    })
}
