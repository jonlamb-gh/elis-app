// TODO - make order info model capable of this, copy

use elis::*;
use gtk::prelude::*;
use gtk::{self, SelectionMode, Type};

#[derive(Clone)]
pub struct InvoiceQueryResultsModel {
    pub scrolled_win: gtk::ScrolledWindow,
    tree_view: gtk::TreeView,
    list_store: gtk::ListStore,
    columns: Vec<gtk::TreeViewColumn>,
}

impl InvoiceQueryResultsModel {
    pub fn new() -> Self {
        let scrolled_win = gtk::ScrolledWindow::new(None, None);
        let tree_view = gtk::TreeView::new();
        let mut columns: Vec<gtk::TreeViewColumn> = Vec::new();

        let list_store = gtk::ListStore::new(&[
            Type::String, // customer
            Type::String, // confirms with
            Type::U32,    // order number
            Type::String, // est weight
            Type::String, // order date
            Type::String, // shipment date
            Type::Bool,   // will call
        ]);

        append_column("Customer", &mut columns, &tree_view, None);
        append_column("Confirms with", &mut columns, &tree_view, None);
        append_column("Order Number", &mut columns, &tree_view, None);
        append_column("Est Weight", &mut columns, &tree_view, None);
        append_column("Order Date", &mut columns, &tree_view, None);
        append_column("Shipment Date", &mut columns, &tree_view, None);
        append_column("Will Call", &mut columns, &tree_view, None);

        tree_view.set_model(Some(&list_store));
        tree_view.set_headers_visible(true);
        tree_view.set_headers_clickable(false);
        tree_view.get_selection().set_mode(SelectionMode::None);
        scrolled_win.add(&tree_view);

        InvoiceQueryResultsModel {
            scrolled_win,
            tree_view,
            list_store,
            columns,
        }
    }

    pub fn clear_model(&self) {
        self.list_store.clear();
    }

    pub fn update_model(&self, order_info: &OrderInfo) {
        self.list_store.insert_with_values(
            None,
            &[0, 1, 2, 3, 4, 5, 6],
            &[
                &format!("{}", order_info.customer()),
                &format!("{}", order_info.confirms_with()),
                &order_info.order_number(),
                &format!("{}", order_info.weight_estimate()),
                &format!("{}", order_info.order_date().format("%m/%d/%Y")),
                &format!("{}", order_info.shipment_date().format("%m/%d/%Y")),
                &order_info.will_call(),
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
    renderer.set_property_xalign(0.5);

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
    column.set_clickable(false);
    column.set_sort_column_id(id);
    tree_view.append_column(&column);
    v.push(column);
}
