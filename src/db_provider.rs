use elis::{lumber::LumberType, LumberFobCostProvider, SiteSalesTaxProvider};

use elis::steel_cent::{currency, Money};
use elis::Database;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Clone)]
pub struct DbProvider {
    pub db: Rc<RefCell<Database>>,
}

// TODO - needs to be a result
// TODO - must not panic inside db closures
impl LumberFobCostProvider for DbProvider {
    fn fob_cost(&self, lumber_type: &LumberType) -> Money {
        let mut cost = Money::zero(currency::USD);
        self.db
            .borrow()
            .read(|db| {
                let lt = db
                    .lumber_types
                    .get(lumber_type)
                    .expect(&format!("Failed to get lumber type '{}'", lumber_type));
                cost = lt.fob_cost().clone();
            }).expect("Failed to read from database");
        cost
    }
}

// TODO - result, check site name
impl SiteSalesTaxProvider for DbProvider {
    fn sales_tax(&self, _site_name: &str) -> f64 {
        let mut sales_tax: f64 = 0.0;
        self.db
            .borrow()
            .read(|db| {
                sales_tax = db.site_info.sales_tax();
            }).expect("Failed to read from database");
        sales_tax
    }
}
