use elis::lumber::{FobCostReader, LumberType};
use elis::steel_cent::{currency, Money};
use elis::Database;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Clone)]
pub struct FobReader {
    pub db: Rc<RefCell<Database>>,
}

// TODO - needs to be a result
impl FobCostReader for FobReader {
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
