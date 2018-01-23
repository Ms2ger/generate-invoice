use csv_parser::{self, Reader};
use invoice::{Business, Client, Date, Invoice, InvoiceData, InvoiceIndex, LineItem, Money};
use std::collections::HashMap;
use std::error::Error;
use std::fmt;
use std::fs::File;
use std::io::Read;
use std::path::Path;

#[derive(Debug)]
struct MissingData(&'static str);

impl fmt::Display for MissingData {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl Error for MissingData {
    fn description(&self) -> &str {
        self.0
    }
}

#[derive(Debug, Deserialize)]
struct SerializedInvoiceData {
    business: String,
    client: String,
    year: u16,
    month: u8,
    day: u8,
}

struct Invoices {
    // XXX Don't use HashMap
    items: HashMap<u8, Vec<LineItem>>,
    metadata: HashMap<u8, SerializedInvoiceData>,
}

impl Invoices {
    fn read_costs<R: Read>(reader: R) -> Result<HashMap<u8, Vec<LineItem>>, Box<Error>> {
        #[derive(Debug, Deserialize)]
        struct SerializedLineItem {
            index: u8,
            amount: i32,
            services: String,
            attachment: String,
        }

        impl Into<(u8, LineItem)> for SerializedLineItem {
            fn into(self) -> (u8, LineItem) {
                let Self {
                    index,
                    amount,
                    services,
                    attachment,
                } = self;

                let attachment = if attachment.is_empty() {
                    None
                } else {
                    Some(attachment)
                };

                let item = LineItem {
                    amount: Money(amount),
                    description: services,
                    attachment,
                };

                (index, item)
            }
        }

        trace!("read_monthly_costs");

        let mut reader = Reader::from_reader(reader);

        let mut items = HashMap::new();
        for item in reader.deserialize::<SerializedLineItem>() {
            let item = item?;
            let (index, item) = item.into();
            items.entry(index).or_insert_with(Vec::new).push(item);
        }

        Ok(items)
    }

    fn read_invoice_data<R: Read>(
        reader: R,
    ) -> Result<HashMap<u8, SerializedInvoiceData>, Box<Error>> {
        #[derive(Debug, Deserialize)]
        struct FullSerializedInvoiceData {
            index: u8,
            business: String,
            client: String,
            year: u16,
            month: u8,
            day: u8,
        }

        impl Into<(u8, SerializedInvoiceData)> for FullSerializedInvoiceData {
            fn into(self) -> (u8, SerializedInvoiceData) {
                let Self {
                    index,
                    business,
                    client,
                    year,
                    month,
                    day,
                } = self;

                let item = SerializedInvoiceData {
                    business,
                    client,
                    year,
                    month,
                    day,
                };

                (index, item)
            }
        }

        trace!("read_invoice_data");

        let mut reader = Reader::from_reader(reader);

        let items = reader
            .deserialize::<FullSerializedInvoiceData>()
            .map(|data| Ok(data?.into()))
            .collect::<Result<_, Box<Error>>>()?;

        Ok(items)
    }

    fn read(path: &Path, year: u16) -> Result<Self, Box<Error>> {
        let folder = path.join(year.to_string());

        let filename = folder.join("data.csv");
        let items = Self::read_costs(File::open(filename)?)?;

        let filename = folder.join("invoices.csv");
        let metadata = Self::read_invoice_data(File::open(filename)?)?;

        Ok(Self {
            items,
            metadata,
        })
    }

    fn get(&self, index: u8) -> (&[LineItem], &SerializedInvoiceData) {
        let items = self.items.get(&index).expect("Missing entry in data.csv");
        let metadata = self.metadata.get(&index).expect("Missing entry in invoices.csv");
        (items, metadata)
    }
}

struct Clients {
    clients: HashMap<String, Client>,
}

impl Clients {
    fn from_reader<R: Read>(reader: R) -> Result<Self, Box<Error>> {
        #[derive(Debug, Deserialize)]
        struct SerializedClient {
            id: String,
            name: String,
            street: String,
            city: String,
            country: String,
            vat: String,
            vatpolicy: String,
        }

        impl Into<(String, Client)> for SerializedClient {
            fn into(self) -> (String, Client) {
                let Self {
                    id,
                    name,
                    street,
                    city,
                    country,
                    vat,
                    vatpolicy,
                } = self;
                let client = Client {
                    name,
                    street,
                    city,
                    country,
                    vat,
                    vatpolicy,
                };
                (id, client)
            }
        }

        trace!("Clients::from_reader");

        let mut reader = Reader::from_reader(reader);
        let clients = reader
            .deserialize::<SerializedClient>()
            .map(|result| result.map(SerializedClient::into))
            .collect::<csv_parser::Result<_>>()?;
        Ok(Self {
            clients,
        })
    }

    fn get(&self, key: &str) -> Option<&Client> {
        self.clients.get(key)
    }
}

struct Businesses {
    businesses: HashMap<String, Business>,
}

impl Businesses {
    fn from_reader<R: Read>(reader: R) -> Result<Self, Box<Error>> {
        #[derive(Debug, Deserialize)]
        struct SerializedBusiness {
            id: String,
            name: String,
            street: String,
            city: String,
            country: String,
            vat: String,
            bank: String,
            iban: String,
            bic: String,
        }

        impl Into<(String, Business)> for SerializedBusiness {
            fn into(self) -> (String, Business) {
                let Self {
                    id,
                    name,
                    street,
                    city,
                    country,
                    vat,
                    bank,
                    iban,
                    bic,
                } = self;
                let business = Business {
                    name,
                    street,
                    city,
                    country,
                    vat,
                    bank,
                    iban,
                    bic,
                };
                (id, business)
            }
        }

        trace!("Clients::from_reader");

        let mut reader = Reader::from_reader(reader);
        let businesses = reader
            .deserialize::<SerializedBusiness>()
            .map(|result| result.map(SerializedBusiness::into))
            .collect::<csv_parser::Result<_>>()?;
        Ok(Self {
            businesses,
        })
    }

    fn get(&self, key: &str) -> Option<&Business> {
        self.businesses.get(key)
    }
}

pub fn read_invoice(path: &Path, index: InvoiceIndex) -> Result<Invoice, Box<Error>> {
    let clients = Clients::from_reader(File::open(path.join("clients.csv"))?)?;
    let businesses = Businesses::from_reader(File::open(path.join("businesses.csv"))?)?;

    let data = Invoices::read(path, index.year)?;
    let (costs, metadata) = data.get(index.index);

    let &SerializedInvoiceData {
        ref business,
        ref client,
        year,
        month,
        day,
    } = metadata;
    let metadata = InvoiceData {
        business: businesses.get(business).ok_or(MissingData("Missing business"))?.clone(),
        client: clients.get(client).ok_or(MissingData("Missing client"))?.clone(),
        date: Date::new(year, month, day),
    };

    Ok(Invoice {
        index,
        metadata: metadata,
        items: costs.to_vec(),
    })
}
