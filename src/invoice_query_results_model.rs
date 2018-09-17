// TODO - make order info model capable of this, copy

use elis::chrono::{Local, TimeZone};
use elis::steel_cent::formatting::us_style;
use elis::{Invoice, LumberFobCostProvider, SiteSalesTaxProvider};
use gtk::prelude::*;
use gtk::{self, SelectionMode, Type};

use default_column::{default_center_column, default_column, default_right_column};

#[derive(Clone)]
pub struct InvoiceQueryResultsModel {
    scrolled_win: gtk::ScrolledWindow,
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
            Type::U32,    // [0] order number
            Type::String, // [1] customer
            Type::String, // [2] order date
            Type::String, // [3] shipment date
            Type::String, // [4] will call
            Type::U32,    // [5] total pieces
            Type::String, // [6] total cost
        ]);

        default_center_column("Order Number", &tree_view, &mut columns);
        default_right_column("Customer", &tree_view, &mut columns);
        default_center_column("Order Date (Local)", &tree_view, &mut columns);
        default_center_column("Shipment Date", &tree_view, &mut columns);
        default_center_column("Will Call", &tree_view, &mut columns);
        default_center_column("Total Pieces", &tree_view, &mut columns);
        default_column("Total Cost", &tree_view, &mut columns);

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

    pub fn get_widget(&self) -> &gtk::ScrolledWindow {
        &self.scrolled_win
    }

    pub fn clear(&self) {
        self.list_store.clear();
    }

    pub fn update_values<T>(&self, invoice: &Invoice, db_provider: &T)
    where
        T: LumberFobCostProvider + SiteSalesTaxProvider,
    {
        let order_info = invoice.order_info();
        let summary = invoice.summary(db_provider);
        let wc = if order_info.will_call() { "Yes" } else { "No" };
        let local_time = Local.from_utc_datetime(&order_info.order_date().naive_local());

        self.list_store.insert_with_values(
            None,
            &[0, 1, 2, 3, 4, 5, 6],
            &[
                &order_info.order_number(),
                &format!("{}", order_info.customer_name()),
                &format!("{}", local_time.format("%m/%d/%Y %H:%M:%S")),
                &format!("{}", order_info.shipment_date().format("%m/%d/%Y")),
                &format!("{}", wc),
                &(summary.total_pieces() as u32),
                &format!("{}", us_style().display_for(summary.total_cost())),
            ],
        );
    }
}
