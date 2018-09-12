use elis::{Database, Invoice};
use gtk::prelude::*;
use gtk::{self, Widget};
use std::cell::RefCell;
use std::rc::Rc;

use fob_reader::FobReader;
use invoice_query_results_model::InvoiceQueryResultsModel;
use notebook::NoteBook;

#[derive(Clone)]
pub struct InvoiceSearchPage {
    vertical_layout: gtk::Box,
    pub page_index: u32,
    results_model: InvoiceQueryResultsModel,
    fob_reader: FobReader,
}

impl InvoiceSearchPage {
    pub fn new(note: &mut NoteBook, db: Rc<RefCell<Database>>) -> Self {
        let fob_reader = FobReader { db };
        let results_model = InvoiceQueryResultsModel::new();
        let vertical_layout = gtk::Box::new(gtk::Orientation::Vertical, 0);

        vertical_layout.pack_start(&results_model.scrolled_win, true, true, 0);

        let vertical_layout: Widget = vertical_layout.upcast();
        let page_index = note.create_tab("Invoice Search", &vertical_layout).unwrap();

        InvoiceSearchPage {
            vertical_layout: vertical_layout
                .downcast::<gtk::Box>()
                .expect("Virtical layout downcast failed"),
            page_index,
            results_model,
            fob_reader,
        }
    }

    pub fn set_results<'a, I>(&self, invoices: I)
    where
        I: Iterator<Item = &'a Invoice>,
    {
        self.results_model.clear_model();

        for inv in invoices {
            self.results_model.update_model(inv, &self.fob_reader);
        }
    }
}
