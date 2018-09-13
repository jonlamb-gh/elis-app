use elis::steel_cent::formatting::us_style;
use elis::{BillableItem, LumberFobCostProvider, SiteSalesTaxProvider};
use gtk::prelude::*;
use gtk::{self, Type};

use default_column::default_column;

pub type ItemId = usize;

#[derive(Clone)]
pub struct ItemsModel {
    pub scrolled_win: gtk::ScrolledWindow,
    pub tree_view: gtk::TreeView,
    list_store: gtk::ListStore,
    columns: Vec<gtk::TreeViewColumn>,
}

impl ItemsModel {
    pub fn new() -> Self {
        let scrolled_win = gtk::ScrolledWindow::new(None, None);
        let tree_view = gtk::TreeView::new();
        let mut columns: Vec<gtk::TreeViewColumn> = Vec::new();

        let list_store = gtk::ListStore::new(&[
            // these are the visible columns
            Type::String, // lumber type
            Type::String, // drying method
            Type::String, // grade
            Type::String, // spec
            Type::String, // description
            Type::String, // dimensions
            Type::U32,    // quantity
            Type::String, // board feet
            Type::String, // fob <location>
            Type::String, // cost
            // last column is hidden
            // it contains the item ID (usually vector index)
            Type::U32, // item_id
        ]);

        let renderer = default_column("Lumber Type", &tree_view, &mut columns);
        //renderer.set_property_editable(true);

        default_column("Drying Method", &tree_view, &mut columns);
        default_column("Grade", &tree_view, &mut columns);
        default_column("Spec", &tree_view, &mut columns);
        default_column("Description", &tree_view, &mut columns);
        default_column("Dimensions (T x W x L)", &tree_view, &mut columns);
        default_column("Quantity", &tree_view, &mut columns);
        default_column("BF", &tree_view, &mut columns);
        default_column("FOB", &tree_view, &mut columns);
        default_column("Cost", &tree_view, &mut columns);

        tree_view.set_model(Some(&list_store));
        tree_view.set_headers_visible(true);
        scrolled_win.add(&tree_view);

        ItemsModel {
            scrolled_win,
            tree_view,
            list_store,
            columns,
        }
    }

    pub fn clear_model(&self) {
        self.list_store.clear();
    }

    pub fn update_model<T>(&self, item: &BillableItem, item_id: ItemId, db_provider: &T)
    where
        T: LumberFobCostProvider + SiteSalesTaxProvider,
    {
        let fob_cost = db_provider.fob_cost(&item.lumber_type());
        let lumber_props = item.lumber_props();

        self.list_store.insert_with_values(
            None,
            &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10],
            &[
                &item.lumber_type(),
                &lumber_props.drying_method.to_str(),
                &lumber_props.grade.to_str(),
                &lumber_props.spec.to_str(),
                &item.description(),
                &format!("{}", item.board_dimensions()),
                &(item.quantity() as u32),
                // TODO - config
                &format!("{:.3}", item.board_dimensions().board_feet()),
                &format!("{}", us_style().display_for(&fob_cost)),
                &format!("{}", us_style().display_for(&item.cost(db_provider))),
                &(item_id as u32),
            ],
        );
    }
}
