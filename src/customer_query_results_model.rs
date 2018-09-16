use elis::CustomerInfo;
use gtk::prelude::*;
use gtk::{self, SelectionMode, Type};

use default_column::default_column;

#[derive(Clone)]
pub struct CustomerQueryResultsModel {
    scrolled_win: gtk::ScrolledWindow,
    tree_view: gtk::TreeView,
    list_store: gtk::ListStore,
    columns: Vec<gtk::TreeViewColumn>,
}

impl CustomerQueryResultsModel {
    pub fn new() -> Self {
        let scrolled_win = gtk::ScrolledWindow::new(None, None);
        let tree_view = gtk::TreeView::new();
        let mut columns: Vec<gtk::TreeViewColumn> = Vec::new();

        let list_store = gtk::ListStore::new(&[
            Type::String, // name
            Type::String, // address
            Type::String, // phone number
            Type::String, // notes
        ]);

        default_column("Name", &tree_view, &mut columns);
        default_column("Address", &tree_view, &mut columns);
        default_column("Phone Number", &tree_view, &mut columns);
        default_column("Notes", &tree_view, &mut columns);

        tree_view.set_model(Some(&list_store));
        tree_view.set_headers_visible(true);
        tree_view.set_headers_clickable(true);
        tree_view.get_selection().set_mode(SelectionMode::Single);
        scrolled_win.add(&tree_view);

        CustomerQueryResultsModel {
            scrolled_win,
            tree_view,
            list_store,
            columns,
        }
    }

    pub fn get_widget(&self) -> &gtk::ScrolledWindow {
        &self.scrolled_win
    }

    pub fn clear(&self) {
        self.list_store.clear();
    }

    pub fn update_values(&self, customer_info: &CustomerInfo) {
        self.list_store.insert_with_values(
            None,
            &[0, 1, 2, 3],
            &[
                &customer_info.name(),
                &customer_info.address(),
                &customer_info.phone_number(),
                &customer_info.notes(),
            ],
        );
    }
}
