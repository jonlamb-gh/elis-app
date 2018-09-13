use elis::CustomerInfo;
use gtk::prelude::*;
use gtk::{self, SelectionMode, Type};

#[derive(Clone)]
pub struct CustomerQueryResultsModel {
    pub scrolled_win: gtk::ScrolledWindow,
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

        append_column("Name", &mut columns, &tree_view, None);
        append_column("Address", &mut columns, &tree_view, None);
        append_column("Phone Number", &mut columns, &tree_view, None);
        append_column("Notes", &mut columns, &tree_view, None);

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

    pub fn clear_model(&self) {
        self.list_store.clear();
    }

    pub fn update_model(&self, customer_info: &CustomerInfo) {
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

// TODO - min/max width pattern
fn append_column(
    title: &str,
    v: &mut Vec<gtk::TreeViewColumn>,
    tree_view: &gtk::TreeView,
    max_width: Option<i32>,
) {
    let id = v.len() as i32;
    let renderer = gtk::CellRendererText::new();

    let column = gtk::TreeViewColumn::new();
    column.set_title(title);
    column.set_resizable(true);
    if let Some(max_width) = max_width {
        column.set_max_width(max_width);
        column.set_expand(true);
    }
    column.set_min_width(50);
    column.pack_start(&renderer, true);
    column.add_attribute(&renderer, "text", id);
    column.set_clickable(true);
    column.set_sort_column_id(id);
    tree_view.append_column(&column);
    v.push(column);
}
