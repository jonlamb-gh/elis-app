use elis::Invoice;
use gtk::prelude::*;
use gtk::{self, Widget};

use invoice_query_results_model::InvoiceQueryResultsModel;
use notebook::NoteBook;

#[derive(Clone)]
pub struct InvoiceSearchPage {
    vertical_layout: gtk::Box,
    pub page_index: u32,
    results_model: InvoiceQueryResultsModel,
}

impl InvoiceSearchPage {
    pub fn new(note: &mut NoteBook) -> Self {
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
        }
    }

    //pub fn set_results(&self, invoices: &[Invoice]) {
    pub fn set_results<'a, I>(&self, invoices: I)
    where
        I: Iterator<Item = &'a Invoice>,
    {
        self.results_model.clear_model();

        for inv in invoices {
            self.results_model.update_model(inv.order_info());
        }
    }
}
