use invoice::{Invoice, LineItem};
use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::path::Path;

#[derive(Serialize, Deserialize)]
#[allow(non_snake_case)]
struct OrderPDF {
    FileName: String,
    FileContent: String,
}

impl OrderPDF {
    fn from(path: &Path) -> Result<Self, Box<dyn Error>> {
        let mut file = File::open(path)?;
        let mut content = vec![];
        file.read_to_end(&mut content)?;
        let content = base64::encode(content);
        Ok(Self {
            FileName: path.file_name().expect("filename").to_str().expect("filename unicode").to_string(),
            FileContent: content,
        })
    }
}

#[derive(Serialize, Deserialize)]
#[allow(non_snake_case)]
struct OrderLine {
    Description: String,
    Quantity: f64,
    UnitPriceExcl: f64,
    VATPercentage: f64,
}

impl OrderLine {
    fn from(item: &LineItem) -> Self {
        Self {
            Description: item.description.clone(),
            Quantity: 1.,
            UnitPriceExcl: item.amount.float(),
            VATPercentage: 0.,
        }
    }
}

#[derive(Serialize, Deserialize)]
#[allow(non_snake_case)]
struct Order {
    OrderNumber: String,
    OrderTitle: String,
    OrderDate: String,
    ExpiryDate: String,
    OrderType: String,
    LastModified: String,
    Created: String,
    OrderDirection: String,
    CounterPartyID: u32,
    OrderPDF: OrderPDF,
    OrderLines: Vec<OrderLine>,
    VentilationCode: String,
    Paid: bool,
    IsSent: bool,
    Currency: String,
}

impl Order {
    fn from(invoice: &Invoice, path: &Path) -> Result<Self, Box<dyn Error>> {
        let index = invoice.index.to_string();
        let date = invoice.metadata.date.to_string();
        Ok(Order {
            OrderNumber: index.clone(),
            OrderTitle: index,
            OrderDate: date.clone(),
            ExpiryDate: invoice.metadata.date.add_days(30).to_string(),
            LastModified: date.clone(),
            Created: date,
            OrderDirection: "Income".to_string(),
            OrderType: "Invoice".to_string(),
            CounterPartyID: invoice.metadata.client.partyid,
            OrderPDF: OrderPDF::from(path)?,
            OrderLines: invoice.items.iter().map(OrderLine::from).collect(),
            VentilationCode: "55".to_string(),
            Paid: true,
            IsSent: true,
            Currency: "EUR".to_string()
        })
    }
}

pub fn generate_json(invoice: &Invoice, path: &Path) -> Result<String, Box<dyn Error>> {
    let order = Order::from(invoice, path)?;
    Ok(serde_json::to_string(&order)?)
}
