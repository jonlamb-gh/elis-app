use elis::steel_cent::formatting::us_style;
use elis::InvoiceSummary;
use gtk::prelude::*;
use gtk::{self, SelectionMode, Type};

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

        append_column("Total Pieces", &mut columns, &tree_view, None);
        append_column("Estimated Shipping", &mut columns, &tree_view, None);
        append_column("Sub Total", &mut columns, &tree_view, None);
        append_column("Sales Tax", &mut columns, &tree_view, None);
        append_column("Total Cost", &mut columns, &tree_view, None);

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

    pub fn update_model(&self, summary: &InvoiceSummary) {
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
    column.set_clickable(false);
    column.set_sort_column_id(id);
    tree_view.append_column(&column);
    v.push(column);
}
