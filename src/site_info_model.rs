use elis::Database;
use gtk::prelude::*;
use gtk::{self, SelectionMode, Type};
use std::cell::RefCell;
use std::rc::Rc;

use default_column::default_column;

#[derive(Clone)]
pub struct SiteInfoModel {
    pub tree_view: gtk::TreeView,
    list_store: gtk::ListStore,
    columns: Vec<gtk::TreeViewColumn>,
    db: Rc<RefCell<Database>>,
}

impl SiteInfoModel {
    pub fn new(db: Rc<RefCell<Database>>) -> Self {
        let tree_view = gtk::TreeView::new();
        let mut columns: Vec<gtk::TreeViewColumn> = Vec::new();

        let list_store = gtk::ListStore::new(&[
            Type::String, // site name
            Type::String, // address
            Type::String, // phone number
            Type::String, // fax number
            Type::String, // sales tax
        ]);

        let renderer = default_column("Site Name", &tree_view, &mut columns);
        renderer.set_property_xalign(0.5);

        let renderer = default_column("Address", &tree_view, &mut columns);
        renderer.set_property_xalign(0.5);

        let renderer = default_column("Phone Number", &tree_view, &mut columns);
        renderer.set_property_xalign(0.5);

        let renderer = default_column("Fax Number", &tree_view, &mut columns);
        renderer.set_property_xalign(0.5);

        default_column("Sales Tax", &tree_view, &mut columns);

        db.borrow()
            .read(|db| {
                list_store.insert_with_values(
                    None,
                    &[0, 1, 2, 3, 4],
                    &[
                        &db.site_info.site_name(),
                        &db.site_info.address(),
                        &db.site_info.phone_number(),
                        &db.site_info.fax_number(),
                        &format!("{:.3} %", db.site_info.sales_tax() * 100.0),
                    ],
                );
            }).expect("Failed to read from database");

        tree_view.set_model(Some(&list_store));
        tree_view.set_headers_visible(true);
        tree_view.set_headers_clickable(false);
        tree_view.get_selection().set_mode(SelectionMode::None);

        SiteInfoModel {
            tree_view,
            list_store,
            columns,
            db,
        }
    }

    // TODO - move out of db closure
    // TODO - need to fix this, row disappears when updated, but is there if resized
    pub fn update_model(&self) {
        /*
        self.list_store.clear();

        self.db
            .borrow()
            .read(|db| {
                self.list_store.insert_with_values(
                    None,
                    &[0, 1, 2, 3, 4],
                    &[
                        &db.site_info.site_name(),
                        &db.site_info.address(),
                        &db.site_info.phone_number(),
                        &db.site_info.fax_number(),
                        &format!("{:.3} %", db.site_info.sales_tax() * 100.0),
                    ],
                );
            }).expect("Failed to read from database");
        */
    }
}
