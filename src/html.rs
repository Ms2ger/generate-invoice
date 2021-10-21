use invoice::{Invoice, LineItem, Money};

use kuchiki::{self, Attribute, ExpandedName, NodeRef};
use kuchiki::traits::TendrilSink;
use markup5ever::{LocalName, QualName};
use std::collections::HashMap;
use std::error::Error;

fn create_element<I>(local: LocalName, attributes: I) -> NodeRef
where
    I: IntoIterator<Item = (ExpandedName, Attribute)>,
{
    let name = QualName {
        prefix: None,
        ns: ns!(html),
        local,
    };
    NodeRef::new_element(name, attributes)
}

fn insert_output(document: &NodeRef, data: &HashMap<&str, &str>) {
    trace!("insert_output");

    for output in document.select("output").expect("hard-coded selector") {
        let attributes = output.attributes.borrow();
        let field =
            attributes.get("data-field").expect("output element without data-field attribute.");
        let content = *data.get(field).expect("output element with unknown data-field attribute.");
        output.as_node().append(NodeRef::new_text(content));
    }
}

fn insert_items(document: &NodeRef, items: &[LineItem]) -> Result<(), Box<dyn Error>> {
    trace!("insert_items");

    let mut items_bodies = document.select(".items").expect("hard-coded selector");
    let items_body = items_bodies.next().expect("missing .items element");
    for item in items {
        let row = create_element(local_name!("tr"), None);
        let description_cell = create_element(local_name!("td"), None);
        description_cell.append(NodeRef::new_text(item.description.clone()));
        row.append(description_cell);

        let class_name = ExpandedName {
            ns: ns!(),
            local: local_name!("class"),
        };
        let class_value = Attribute {
            prefix: None,
            value: "num".to_string(),
        };
        let amount_cell = create_element(local_name!("td"), Some((class_name, class_value)));
        amount_cell.append(NodeRef::new_text(item.amount.to_string()));
        row.append(amount_cell);

        items_body.as_node().append(row);
    }

    assert!(items_bodies.next().is_none());

    Ok(())
}

fn substitute_template(
    mut template: &[u8],
    data: &HashMap<&str, &str>,
    items: &[LineItem],
) -> Result<NodeRef, Box<dyn Error>> {
    trace!("substitute_template");

    let document = kuchiki::parse_html().from_utf8().read_from(&mut template)?;
    insert_output(&document, data);
    insert_items(&document, items)?;
    Ok(document)
}

impl Invoice {
    pub fn generate_invoice(&self) -> Result<NodeRef, Box<dyn Error>> {
        trace!("Invoice::generate");

        let total = self.items.iter().map(|item| item.amount).sum::<Money>().to_string();
        let invoice_date = self.metadata.date.to_string();
        let invoice_index = self.index.to_string();

        let substitutions = hashmap!{
            "total" => &*total,
            "invoice-date" => &*invoice_date,
            "invoice-index" => &*invoice_index,
            "client-name" => &*self.metadata.client.name,
            "client-street" => &*self.metadata.client.street,
            "client-city" => &*self.metadata.client.city,
            "client-country" => &*self.metadata.client.country,
            "client-vat" => &*self.metadata.client.vat,
            "client-vat-policy" => &*self.metadata.client.vatpolicy,
            "business-name" => &*self.metadata.business.name,
            "business-street" => &*self.metadata.business.street,
            "business-city" => &*self.metadata.business.city,
            "business-country" => &*self.metadata.business.country,
            "business-vat" => &*self.metadata.business.vat,
            "business-bank" => &*self.metadata.business.bank,
            "business-iban" => &*self.metadata.business.iban,
            "business-bic" => &*self.metadata.business.bic,
        };

        Ok(substitute_template(
            include_bytes!("../etc/template.html"),
            &substitutions,
            &self.items,
        )?)
    }
}
