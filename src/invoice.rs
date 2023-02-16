use chrono::Datelike;
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
pub struct Date(chrono::NaiveDate);

impl Date {
    pub fn new(year: u16, month: u8, day: u8) -> Option<Self> {
        chrono::NaiveDate::from_ymd_opt(year.into(), month.into(), day.into()).map(Self)
    }

    pub fn add_days(&self, days: i64) -> Self {
        Self(self.0 + chrono::Duration::days(days))
    }

    pub fn month_name(&self) -> &'static str {
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
        &MONTHS[self.0.month0() as usize]
    }
}

impl fmt::Display for Date {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0.format("%Y-%m-%d"))
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
    pub partyid: u32,
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

impl Money {
    pub fn float(&self) -> f64 {
        f64::from(self.0) / 100.0
    }
}

impl Invoice {
    pub fn total(&self) -> Money {
        self.items.iter().map(|item| item.amount).sum()
    }
}

#[derive(Clone, Debug)]
pub struct LineItem {
    pub description: String,
    pub amount: Money,
    pub attachment: Option<String>,
}
