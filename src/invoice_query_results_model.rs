// TODO - make order info model capable of this, copy

use elis::lumber::FobCostReader;
use elis::steel_cent::formatting::us_style;
use elis::Invoice;
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
            Type::U32,    // order number
            Type::String, // customer
            Type::String, // order date
            Type::String, // shipment date
            Type::Bool,   // will call
            Type::U32,    // total pieces
            Type::String, // total cost
        ]);

        append_column("Order Number", &mut columns, &tree_view, None);
        append_column("Customer", &mut columns, &tree_view, None);
        append_column("Order Date", &mut columns, &tree_view, None);
        append_column("Shipment Date", &mut columns, &tree_view, None);
        append_column("Will Call", &mut columns, &tree_view, None);
        append_column("Total Pieces", &mut columns, &tree_view, None);
        append_column("Total Cost", &mut columns, &tree_view, None);

        tree_view.set_model(Some(&list_store));
        tree_view.set_headers_visible(true);
        tree_view.set_headers_clickable(true);
        tree_view.get_selection().set_mode(SelectionMode::Single);
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

    pub fn update_model<T: FobCostReader>(&self, invoice: &Invoice, fob_reader: &T) {
        let order_info = invoice.order_info();
        let summary = invoice.summary(fob_reader);

        self.list_store.insert_with_values(
            None,
            &[0, 1, 2, 3, 4, 5, 6],
            &[
                &order_info.order_number(),
                &format!("{}", order_info.customer_name()),
                &format!("{}", order_info.order_date().format("%m/%d/%Y")),
                &format!("{}", order_info.shipment_date().format("%m/%d/%Y")),
                &order_info.will_call(),
                &(summary.total_pieces() as u32),
                &format!("{}", us_style().display_for(summary.total_cost())),
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

    if title == "Order Number" {
        renderer.set_property_xalign(1.0);
    } else if title == "Total Cost" {
        renderer.set_property_xalign(0.0);
    } else {
        renderer.set_property_xalign(0.5);
    }

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
