use elis::{Database, OrderInfo};
use gtk::prelude::*;
use gtk::{self, SelectionMode, Type};
use std::cell::RefCell;
use std::rc::Rc;

use default_column::{default_column, default_combo_column};
#[derive(Clone)]
pub struct CellRenderers {
    pub customer: gtk::CellRendererCombo,
}

#[derive(Clone)]
pub struct OrderInfoModel {
    pub tree_view: gtk::TreeView,
    list_store: gtk::ListStore,
    columns: Vec<gtk::TreeViewColumn>,
    pub cell_renderers: CellRenderers,
    db: Rc<RefCell<Database>>,
}

impl OrderInfoModel {
    pub fn new(db: Rc<RefCell<Database>>) -> Self {
        let tree_view = gtk::TreeView::new();
        let mut columns: Vec<gtk::TreeViewColumn> = Vec::new();

        let list_store = gtk::ListStore::new(&[
            Type::String, // customer name
            Type::String, // confirms with
            Type::U32,    // order number
            Type::String, // est weight
            Type::String, // order date
            Type::String, // shipment date
            Type::String, // site name
            Type::Bool,   // will call
        ]);

        let combo_model = gtk::ListStore::new(&[Type::String]);
        db.borrow()
            .read(|db| {
                for name in db.customers.keys() {
                    combo_model.insert_with_values(None, &[0], &[name]);
                }
            }).expect("Failed to read from database");

        let rend_customer =
            default_combo_column("Customer", &combo_model, &tree_view, &mut columns);

        let renderer = default_column("Confirms with", &tree_view, &mut columns);
        renderer.set_property_xalign(0.5);

        let renderer = default_column("Order Number", &tree_view, &mut columns);
        renderer.set_property_xalign(0.5);

        let renderer = default_column("Est Weight", &tree_view, &mut columns);
        renderer.set_property_xalign(0.5);

        let renderer = default_column("Order Date", &tree_view, &mut columns);
        renderer.set_property_xalign(0.5);

        let renderer = default_column("Shipment Date", &tree_view, &mut columns);
        renderer.set_property_xalign(0.5);

        let renderer = default_column("Site", &tree_view, &mut columns);
        renderer.set_property_xalign(0.5);

        default_column("Will Call", &tree_view, &mut columns);

        tree_view.set_model(Some(&list_store));
        tree_view.set_headers_visible(true);
        tree_view.set_headers_clickable(false);
        tree_view.get_selection().set_mode(SelectionMode::Single);

        OrderInfoModel {
            tree_view,
            list_store,
            columns,
            cell_renderers: CellRenderers {
                customer: rend_customer,
            },
            db,
        }
    }

    pub fn update_model(&self, order_info: &OrderInfo) {
        self.list_store.clear();
        self.list_store.insert_with_values(
            None,
            &[0, 1, 2, 3, 4, 5, 6, 7],
            &[
                &format!("{}", order_info.customer_name()),
                &format!("{}", order_info.confirms_with()),
                &order_info.order_number(),
                &format!("{}", order_info.weight_estimate()),
                &format!("{}", order_info.order_date().format("%m/%d/%Y")),
                &format!("{}", order_info.shipment_date().format("%m/%d/%Y")),
                &format!("{}", order_info.site_name()),
                &order_info.will_call(),
            ],
        );
    }
}
