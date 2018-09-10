use gtk::prelude::*;
use gtk::{self, Widget};

use notebook::NoteBook;

#[derive(Clone)]
pub struct InvoiceSearchPage {
    pub vertical_layout: gtk::Box,
}

impl InvoiceSearchPage {
    pub fn new(note: &mut NoteBook) -> Self {
        let vertical_layout = gtk::Box::new(gtk::Orientation::Vertical, 0);

        let vertical_layout: Widget = vertical_layout.upcast();
        note.create_tab("Invoice Search", &vertical_layout);

        InvoiceSearchPage {
            vertical_layout: vertical_layout
                .downcast::<gtk::Box>()
                .expect("Virtical layout downcast failed"),
        }
    }
}
