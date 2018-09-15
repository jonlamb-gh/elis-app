use elis::steel_cent::formatting::us_style;
use elis::Database;
use gtk::prelude::*;
use gtk::{self, SelectionMode, Type};
use std::cell::RefCell;
use std::rc::Rc;

use default_column::default_column;

#[derive(Clone)]
pub struct LumberTypeModel {
    pub scrolled_win: gtk::ScrolledWindow,
    tree_view: gtk::TreeView,
    list_store: gtk::ListStore,
    columns: Vec<gtk::TreeViewColumn>,
    db: Rc<RefCell<Database>>,
}

impl LumberTypeModel {
    pub fn new(db: Rc<RefCell<Database>>) -> Self {
        let scrolled_win = gtk::ScrolledWindow::new(None, None);
        let tree_view = gtk::TreeView::new();
        let mut columns: Vec<gtk::TreeViewColumn> = Vec::new();

        let list_store = gtk::ListStore::new(&[
            Type::String, // type name
            Type::String, // FOB price
        ]);

        default_column("Lumber Type", &tree_view, &mut columns);
        default_column("FOB Price", &tree_view, &mut columns);

        tree_view.set_model(Some(&list_store));
        tree_view.set_headers_visible(true);
        tree_view.set_headers_clickable(true);
        tree_view.get_selection().set_mode(SelectionMode::Single);
        scrolled_win.add(&tree_view);

        LumberTypeModel {
            scrolled_win,
            tree_view,
            list_store,
            columns,
            db,
        }
    }

    pub fn update_model(&self) {
        self.list_store.clear();

        self.db
            .borrow()
            .read(|db| {
                for lt in db.lumber_types.values() {
                    self.list_store.insert_with_values(
                        None,
                        &[0, 1],
                        &[
                            &lt.type_name(),
                            &format!("{}", us_style().display_for(lt.fob_cost())),
                        ],
                    );
                }
            }).expect("Failed to read from database");
    }
}
