use elis::lumber::{DryingMethod, Grade, Specification};
use elis::steel_cent::formatting::us_style;
use elis::{BillableItem, Database, LumberFobCostProvider, SiteSalesTaxProvider};
use gtk::prelude::*;
use gtk::{self, Type};
use pango::WrapMode;
use std::cell::RefCell;
use std::rc::Rc;

use default_column::{default_column, default_combo_column};

pub type ItemId = usize;

// TODO - only providing editables?
#[derive(Clone)]
pub struct CellRenderers {
    pub lumber_type: gtk::CellRendererCombo,
    pub drying_method: gtk::CellRendererCombo,
    pub grade: gtk::CellRendererCombo,
    pub spec: gtk::CellRendererCombo,
    pub description: gtk::CellRendererText,
    pub board_dimensions: gtk::CellRendererText,
    pub quantity: gtk::CellRendererText,
}

#[derive(Clone)]
pub struct ItemsModel {
    scrolled_win: gtk::ScrolledWindow,
    pub tree_view: gtk::TreeView,
    list_store: gtk::ListStore,
    columns: Vec<gtk::TreeViewColumn>,
    pub cell_renderers: CellRenderers,
    db: Rc<RefCell<Database>>,
}

impl ItemsModel {
    pub fn new(db: Rc<RefCell<Database>>) -> Self {
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

        let combo_model = gtk::ListStore::new(&[Type::String]);
        db.borrow()
            .read(|db| {
                for lt_name in db.lumber_types.keys() {
                    combo_model.insert_with_values(None, &[0], &[lt_name]);
                }
            }).expect("Failed to read from database");

        let rend_lumber_type =
            default_combo_column("Lumber Type", &combo_model, &tree_view, &mut columns);

        let combo_model = gtk::ListStore::new(&[Type::String]);
        for dm in DryingMethod::enumerate() {
            combo_model.insert_with_values(None, &[0], &[&dm.to_str()]);
        }
        let rend_drying_method =
            default_combo_column("Drying Method", &combo_model, &tree_view, &mut columns);

        let combo_model = gtk::ListStore::new(&[Type::String]);
        for g in Grade::enumerate() {
            combo_model.insert_with_values(None, &[0], &[&g.to_str()]);
        }
        let rend_grade = default_combo_column("Grade", &combo_model, &tree_view, &mut columns);

        let combo_model = gtk::ListStore::new(&[Type::String]);
        for s in Specification::enumerate() {
            combo_model.insert_with_values(None, &[0], &[&s.to_str()]);
        }
        let rend_spec = default_combo_column("Spec", &combo_model, &tree_view, &mut columns);

        let rend_description = default_column("Description", &tree_view, &mut columns);
        rend_description.set_property_editable(true);
        rend_description.set_property_wrap_mode(WrapMode::WordChar);
        rend_description.set_property_wrap_width(200);

        let rend_board_dimensions =
            default_column("Dimensions (T x W x L)", &tree_view, &mut columns);
        rend_board_dimensions.set_property_editable(true);

        let rend_quantity = default_column("Quantity", &tree_view, &mut columns);
        rend_quantity.set_property_editable(true);

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
            cell_renderers: CellRenderers {
                lumber_type: rend_lumber_type,
                drying_method: rend_drying_method,
                grade: rend_grade,
                spec: rend_spec,
                description: rend_description,
                board_dimensions: rend_board_dimensions,
                quantity: rend_quantity,
            },
            db,
        }
    }

    pub fn get_widget(&self) -> &gtk::ScrolledWindow {
        &self.scrolled_win
    }

    pub fn clear(&self) {
        self.list_store.clear();
    }

    pub fn update_values<T>(&self, item: &BillableItem, item_id: ItemId, db_provider: &T)
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
        let item_id_column: i32 = 10;

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
