use elis::steel_cent::formatting::us_style;
use elis::{BillableItem, LumberFobCostProvider, SiteSalesTaxProvider};
use gtk::prelude::*;
use gtk::{self, Type};

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

        append_column("Lumber Type", &mut columns, &tree_view, None);
        append_column("Drying Method", &mut columns, &tree_view, None);
        append_column("Grade", &mut columns, &tree_view, None);
        append_column("Spec", &mut columns, &tree_view, None);
        append_column("Description", &mut columns, &tree_view, None);
        append_column("Dimensions (T x W x L)", &mut columns, &tree_view, None);
        append_column("Quantity", &mut columns, &tree_view, None);
        append_column("BF", &mut columns, &tree_view, None);
        append_column("FOB", &mut columns, &tree_view, None);
        append_column("Cost", &mut columns, &tree_view, None);

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

// TODO - min/max width pattern
fn append_column(
    title: &str,
    v: &mut Vec<gtk::TreeViewColumn>,
    tree_view: &gtk::TreeView,
    max_width: Option<i32>,
) {
    let id = v.len() as i32;

    let renderer = gtk::CellRendererText::new();
    //renderer.set_property_editable(true);

    let column = gtk::TreeViewColumn::new();
    column.set_title(title);
    column.set_resizable(true);
    if let Some(max_width) = max_width {
        column.set_max_width(max_width);
        column.set_expand(true);
    }
    column.set_min_width(25);
    column.pack_start(&renderer, true);
    column.add_attribute(&renderer, "text", id);
    column.set_clickable(true);
    column.set_sort_column_id(id);
    tree_view.append_column(&column);
    v.push(column);
}
