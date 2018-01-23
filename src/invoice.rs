use std::fmt;
use std::iter::Sum;

#[derive(Debug)]
pub struct Invoice {
    pub index: InvoiceIndex,
    pub metadata: InvoiceData,
    pub items: Vec<LineItem>,
}

#[derive(Debug)]
pub struct InvoiceIndex {
    pub year: u16,
    pub index: u8,
}

impl InvoiceIndex {
    pub fn filename(&self) -> String {
        format!("{:04}/{:02}.html", self.year, self.index)
    }
}

impl fmt::Display for InvoiceIndex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:04}-{:02}", self.year, self.index)
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Year(pub u16);

#[derive(Copy, Clone, Debug)]
pub struct Month(pub u8);

impl Month {
    pub fn name(&self) -> &'static str {
        static MONTHS: [&'static str; 12] = [
            "January",
            "February",
            "March",
            "April",
            "May",
            "June",
            "July",
            "August",
            "September",
            "October",
            "November",
            "December",
        ];
        MONTHS[self.0 as usize - 1]
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Day(pub u8);

#[derive(Copy, Clone, Debug)]
pub struct Date {
    pub year: Year,
    pub month: Month,
    pub day: Day,
}

impl Date {
    pub fn new(year: u16, month: u8, day: u8) -> Self {
        Self {
            year: Year(year),
            month: Month(month),
            day: Day(day),
        }
    }
}

impl fmt::Display for Date {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:04}-{:02}-{:02}", self.year.0, self.month.0, self.day.0)
    }
}

#[derive(Clone, Debug)]
pub struct Business {
    pub name: String,
    pub street: String,
    pub city: String,
    pub country: String,
    pub vat: String,
    pub bank: String,
    pub iban: String,
    pub bic: String,
}

#[derive(Clone, Debug)]
pub struct Client {
    pub name: String,
    pub street: String,
    pub city: String,
    pub country: String,
    pub vat: String,
    pub vatpolicy: String,
}

#[derive(Clone, Debug)]
pub struct InvoiceData {
    pub business: Business,
    pub client: Client,
    pub date: Date,
}

#[derive(Copy, Clone, Debug)]
pub struct Money(pub i32);

impl Sum for Money {
    fn sum<I: Iterator<Item = Self>>(iterator: I) -> Self {
        Money(iterator.map(|Money(m)| m).sum())
    }
}

impl fmt::Display for Money {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let sign = if self.0 < 0 {
            "\u{2212}"
        } else {
            ""
        };
        write!(f, "{}\u{20ac}{}.{:02}", sign, (self.0 / 100).abs(), self.0 % 100)
    }
}

#[derive(Clone, Debug)]
pub struct LineItem {
    pub description: String,
    pub amount: Money,
    pub attachment: Option<String>,
}
