use elis::{Database, OrderInfo};
use gtk::prelude::*;
use gtk::{self, SelectionMode, Type};
use pango::WrapMode;
use std::cell::RefCell;
use std::rc::Rc;

use default_column::{default_center_column, default_combo_column, default_toggle_column};
#[derive(Clone)]
pub struct CellRenderers {
    pub customer: gtk::CellRendererCombo,
    pub confirms_with: gtk::CellRendererText,
    pub will_call: gtk::CellRendererToggle,
}

#[derive(Clone)]
pub struct OrderInfoModel {
    tree_view: gtk::TreeView,
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
            Type::String, // [0] customer name
            Type::String, // [1] confirms with
            Type::U32,    // [2] order number
            Type::String, // [3] est weight
            Type::String, // [4] order date
            Type::String, // [5] shipment date
            Type::String, // [6] site name
            Type::Bool,   // [7] will call
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

        let rend_confirms_with = default_center_column("Confirms with", &tree_view, &mut columns);
        rend_confirms_with.set_property_editable(true);
        rend_confirms_with.set_property_wrap_mode(WrapMode::WordChar);
        rend_confirms_with.set_property_wrap_width(200);

        default_center_column("Order Number", &tree_view, &mut columns);

        default_center_column("Est Weight", &tree_view, &mut columns);

        default_center_column("Order Date", &tree_view, &mut columns);

        default_center_column("Shipment Date", &tree_view, &mut columns);

        default_center_column("Site", &tree_view, &mut columns);

        let rend_will_call = default_toggle_column("Will Call", &tree_view, &mut columns);

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
                confirms_with: rend_confirms_with,
                will_call: rend_will_call,
            },
            db,
        }
    }

    pub fn get_widget(&self) -> &gtk::TreeView {
        &self.tree_view
    }

    pub fn update_values(&self, order_info: &OrderInfo) {
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

    pub fn unselect(&self) {
        self.tree_view.get_selection().unselect_all();
    }
}
