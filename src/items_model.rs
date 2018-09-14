use elis::steel_cent::formatting::us_style;
use elis::{BillableItem, LumberFobCostProvider, SiteSalesTaxProvider};
use gtk::prelude::*;
use gtk::{self, Type};
use std::ops;

use default_column::default_column;

pub type ItemId = usize;

// only provides the editable columns?
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ItemColumn {
    LumberType,
    Description,
    Quantity,
    HiddenItemId,
}

impl ItemColumn {
    pub fn column_index(&self) -> u32 {
        match *self {
            ItemColumn::LumberType => 0,
            ItemColumn::Description => 4,
            ItemColumn::Quantity => 6,
            ItemColumn::HiddenItemId => 10,
        }
    }
}

impl ops::Index<ItemColumn> for [gtk::CellRendererText] {
    type Output = gtk::CellRendererText;

    fn index(&self, i: ItemColumn) -> &gtk::CellRendererText {
        &(self[i as usize])
    }
}

#[derive(Clone)]
pub struct ItemsModel {
    pub scrolled_win: gtk::ScrolledWindow,
    pub tree_view: gtk::TreeView,
    list_store: gtk::ListStore,
    columns: Vec<gtk::TreeViewColumn>,
    pub editable_renderers: [gtk::CellRendererText; 3],
}

impl ItemsModel {
    pub fn new() -> Self {
        let scrolled_win = gtk::ScrolledWindow::new(None, None);
        let tree_view = gtk::TreeView::new();
        let mut columns: Vec<gtk::TreeViewColumn> = Vec::new();

        let list_store = gtk::ListStore::new(&[
            // these are the visible columns
            Type::String, // [0] lumber type
            Type::String, // [1] drying method
            Type::String, // [2] grade
            Type::String, // [3] spec
            Type::String, // [4] description
            Type::String, // [5] dimensions
            Type::U32,    // [6] quantity
            Type::String, // [7] board feet
            Type::String, // [8] fob <location>
            Type::String, // [9] cost
            // last column is hidden
            // it contains the item ID (usually vector index)
            Type::U32, // [10] item_id
        ]);

        let renderer_lum_type = default_column("Lumber Type", &tree_view, &mut columns);
        default_column("Drying Method", &tree_view, &mut columns);
        default_column("Grade", &tree_view, &mut columns);
        default_column("Spec", &tree_view, &mut columns);
        let renderer_desc = default_column("Description", &tree_view, &mut columns);
        default_column("Dimensions (T x W x L)", &tree_view, &mut columns);
        let renderer_quant = default_column("Quantity", &tree_view, &mut columns);
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
            editable_renderers: [renderer_lum_type, renderer_desc, renderer_quant],
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

    pub fn get_selected(&self) -> (Option<ItemId>, bool) {
        let selection = self.tree_view.get_selection();
        let item_id_column = ItemColumn::HiddenItemId.column_index() as i32;

        let (id, selected) = if let Some((model, iter)) = selection.get_selected() {
            let value = model
                .get_value(&iter, item_id_column)
                .get::<u32>()
                .map(|x| x as ItemId);

            if let Some(x) = value {
                (Some(x), true)
            } else {
                (None, false)
            }
        } else {
            (None, false)
        };

        (id, selected)
    }
}
