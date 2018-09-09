use glib::object::Cast;
use gtk::prelude::*;
use gtk::{self, Widget};
use std::cell::RefCell;
use std::rc::Rc;

use elis::*;

use items_model::ItemsModel;
use notebook::NoteBook;

#[derive(Clone)]
pub struct NewInvoiceModel {
    pub vertical_layout: gtk::Box,
    pub new_item_button: gtk::Button,
    pub items_model: ItemsModel,
    pub invoice: Rc<RefCell<Invoice>>,
}

impl NewInvoiceModel {
    pub fn new(note: &mut NoteBook) -> Self {
        let new_item_button = gtk::Button::new_with_label("Add Item");
        let items_model = ItemsModel::new();
        let vertical_layout = gtk::Box::new(gtk::Orientation::Vertical, 0);
        let horizontal_layout = gtk::Grid::new();

        new_item_button.set_sensitive(true);

        vertical_layout.pack_start(&items_model.scrolled_win, true, true, 0);
        horizontal_layout.attach(&new_item_button, 0, 0, 2, 1);
        horizontal_layout.set_column_homogeneous(true);
        vertical_layout.pack_start(&horizontal_layout, false, true, 0);

        let vertical_layout: Widget = vertical_layout.upcast();
        note.create_tab("New Invoice", &vertical_layout);

        NewInvoiceModel {
            vertical_layout: vertical_layout
                .downcast::<gtk::Box>()
                .expect("Virtical layout downcast failed"),
            new_item_button,
            items_model,
            invoice: Rc::new(RefCell::new(Invoice::new())),
        }
    }
}
