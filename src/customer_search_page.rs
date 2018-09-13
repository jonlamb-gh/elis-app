use elis::{CustomerInfo, Database};
use gtk::prelude::*;
use gtk::{self, Widget};
use std::cell::RefCell;
use std::rc::Rc;

use customer_query_results_model::CustomerQueryResultsModel;
use db_provider::DbProvider;
use notebook::NoteBook;

#[derive(Clone)]
pub struct CustomerSearchPage {
    pub vertical_layout: gtk::Box,
    pub page_index: u32,
    results_model: CustomerQueryResultsModel,
    db_provider: DbProvider,
}

impl CustomerSearchPage {
    pub fn new(note: &mut NoteBook, db: Rc<RefCell<Database>>) -> Self {
        let db_provider = DbProvider { db };
        let results_model = CustomerQueryResultsModel::new();
        let vertical_layout = gtk::Box::new(gtk::Orientation::Vertical, 0);

        vertical_layout.pack_start(&results_model.scrolled_win, true, true, 0);

        let vertical_layout: Widget = vertical_layout.upcast();
        let page_index = note
            .create_tab("Customer Search", &vertical_layout)
            .unwrap();

        CustomerSearchPage {
            vertical_layout: vertical_layout
                .downcast::<gtk::Box>()
                .expect("Virtical layout downcast failed"),
            page_index,
            results_model,
            db_provider,
        }
    }

    pub fn set_results<'a, I>(&self, customers: I)
    where
        I: Iterator<Item = &'a CustomerInfo>,
    {
        self.results_model.clear_model();

        for customer in customers {
            self.results_model.update_model(customer);
        }
    }
}
