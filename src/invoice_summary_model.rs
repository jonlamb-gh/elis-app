use elis::steel_cent::formatting::us_style;
use elis::InvoiceSummary;
use gtk::prelude::*;
use gtk::{self, SelectionMode, Type};

use default_column::default_column;

#[derive(Clone)]
pub struct InvoiceSummaryModel {
    pub tree_view: gtk::TreeView,
    pub list_store: gtk::ListStore,
    pub columns: Vec<gtk::TreeViewColumn>,
}

impl InvoiceSummaryModel {
    pub fn new() -> Self {
        let tree_view = gtk::TreeView::new();
        let mut columns: Vec<gtk::TreeViewColumn> = Vec::new();

        let list_store = gtk::ListStore::new(&[
            Type::U32,    // total pieces
            Type::String, // estimated shipping
            Type::String, // sub total
            Type::String, // sales tax
            Type::String, // total cost
        ]);

        default_column("Total Pieces", &tree_view, &mut columns);
        default_column("Estimated Shipping", &tree_view, &mut columns);
        default_column("Sub Total", &tree_view, &mut columns);
        default_column("Sales Tax", &tree_view, &mut columns);
        default_column("Total Cost", &tree_view, &mut columns);

        tree_view.set_model(Some(&list_store));
        tree_view.set_headers_visible(true);
        tree_view.set_headers_clickable(false);
        tree_view.get_selection().set_mode(SelectionMode::None);

        InvoiceSummaryModel {
            tree_view,
            list_store,
            columns,
        }
    }

    pub fn update_values(&self, summary: &InvoiceSummary) {
        self.list_store.clear();
        self.list_store.insert_with_values(
            None,
            &[0, 1, 2, 3, 4],
            &[
                &(summary.total_pieces() as u32),
                &format!(
                    "{}",
                    us_style().display_for(summary.estimated_shipping_cost())
                ),
                &format!("{}", us_style().display_for(summary.sub_total_cost())),
                &format!("{}", us_style().display_for(summary.sales_tax_cost())),
                &format!("{}", us_style().display_for(summary.total_cost())),
            ],
        );
    }
}
