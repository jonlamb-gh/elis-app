use elis::Database;
use gtk::prelude::*;
use gtk::{self, SelectionMode, Type};
use std::cell::RefCell;
use std::rc::Rc;

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

        append_column("Site Name", &mut columns, &tree_view, None);
        append_column("Address", &mut columns, &tree_view, None);
        append_column("Phone Number", &mut columns, &tree_view, None);
        append_column("Fax Number", &mut columns, &tree_view, None);
        append_column("Sales Tax", &mut columns, &tree_view, None);

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

    if title != "Sales Tax" {
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
