use gtk::prelude::*;
use gtk::{self, Widget};

use notebook::NoteBook;

#[derive(Clone)]
pub struct NewCustomerPage {
    pub vertical_layout: gtk::Box,
}

impl NewCustomerPage {
    pub fn new(note: &mut NoteBook) -> Self {
        let vertical_layout = gtk::Box::new(gtk::Orientation::Vertical, 0);

        let vertical_layout: Widget = vertical_layout.upcast();
        note.create_tab("New Customer", &vertical_layout);

        NewCustomerPage {
            vertical_layout: vertical_layout
                .downcast::<gtk::Box>()
                .expect("Virtical layout downcast failed"),
        }
    }
}
