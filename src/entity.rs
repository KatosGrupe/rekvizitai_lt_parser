use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Entity {
    pub account_number: String,
    pub address: String,
    pub average_wage: String,
    pub bank: String,
    pub business_hours: String,
    pub ceo: String,
    pub entity_type: String,
    pub mobile_phone: String,
    pub name: String,
    pub phone: String,
    pub registration_id: String,
    pub vat_id: String,
    pub website: String,
}

impl Entity {
    pub fn new() -> Entity {
        Entity {
            account_number: String::new(),
            address: String::new(),
            average_wage: String::new(),
            bank: String::new(),
            business_hours: String::new(),
            ceo: String::new(),
            entity_type: String::new(),
            mobile_phone: String::new(),
            name: String::new(),
            phone: String::new(),
            registration_id: String::new(),
            vat_id: String::new(),
            website: String::new(),
        }
    }
}
