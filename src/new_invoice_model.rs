use glib::object::Cast;
use gtk::prelude::*;
use gtk::{self, Widget};
use std::cell::{Cell, RefCell};
use std::rc::Rc;

use elis::*;

use items_model::{add_item_to_model, ItemId, ItemsModel};
use notebook::NoteBook;

#[derive(Clone)]
pub struct NewInvoiceModel {
    pub vertical_layout: gtk::Box,
    pub new_item_button: gtk::Button,
    pub edit_item_button: gtk::Button,
    pub delete_item_button: gtk::Button,
    pub items_model: ItemsModel,
    pub invoice: Rc<RefCell<Invoice>>,
    pub selected_item_id: Rc<Cell<Option<ItemId>>>,
}

impl NewInvoiceModel {
    pub fn new(note: &mut NoteBook) -> Self {
        let invoice = Rc::new(RefCell::new(Invoice::new()));
        let new_item_button = gtk::Button::new_with_label("Add");
        let edit_item_button = gtk::Button::new_with_label("Edit");
        let delete_item_button = gtk::Button::new_with_label("Delete");
        let items_model = ItemsModel::new();
        let vertical_layout = gtk::Box::new(gtk::Orientation::Vertical, 0);
        let horizontal_layout = gtk::Grid::new();
        let selected_item_id = Rc::new(Cell::new(None));

        new_item_button.set_sensitive(true);
        edit_item_button.set_sensitive(false);
        delete_item_button.set_sensitive(false);

        let list_store = items_model.list_store.clone();
        new_item_button.connect_clicked(clone!(invoice => move |_| {
            let item = BillableItem::new();
            invoice.borrow_mut().add_billable_item(item);
            refresh_items_model(&invoice.borrow(), &list_store);
        }));

        let list_store = items_model.list_store.clone();
        delete_item_button.connect_clicked(clone!(invoice, selected_item_id => move |_| {
            if let Some(item_id) = selected_item_id.get() {
                invoice.borrow_mut().remove_billable_item(item_id);
                refresh_items_model(&invoice.borrow(), &list_store);
            }
        }));

        items_model.tree_view.connect_cursor_changed(
            clone!(edit_item_button, delete_item_button, selected_item_id => move |tree_view| {
            let selection = tree_view.get_selection();
            let (id, selected) = if let Some((model, iter)) = selection.get_selected() {
                if let Some(x) = model.get_value(&iter, 7).get::<u32>().map(|x| x as ItemId) {
                    (Some(x), true)
                } else {
                    (None, false)
                }
            } else {
                (None, false)
            };

            selected_item_id.set(id);
            edit_item_button.set_sensitive(selected);
            delete_item_button.set_sensitive(selected);
        }),
        );

        vertical_layout.pack_start(&items_model.scrolled_win, true, true, 0);
        horizontal_layout.attach(&new_item_button, 0, 0, 1, 1);
        horizontal_layout.attach(&edit_item_button, 1, 0, 1, 1);
        horizontal_layout.attach(&delete_item_button, 2, 0, 1, 1);
        horizontal_layout.set_column_homogeneous(false);
        vertical_layout.pack_start(&horizontal_layout, false, true, 0);

        let vertical_layout: Widget = vertical_layout.upcast();
        note.create_tab("New Invoice", &vertical_layout);

        NewInvoiceModel {
            vertical_layout: vertical_layout
                .downcast::<gtk::Box>()
                .expect("Virtical layout downcast failed"),
            new_item_button,
            edit_item_button,
            delete_item_button,
            items_model,
            invoice,
            selected_item_id,
        }
    }
}

fn refresh_items_model(invoice: &Invoice, list: &gtk::ListStore) {
    list.clear();
    for (id, item) in invoice.items().iter().enumerate() {
        add_item_to_model(item, id as ItemId, list);
    }
}
