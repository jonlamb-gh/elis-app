use elis::steel_cent::formatting::us_style;
use elis::LumberType;
use gtk::prelude::*;
use gtk::{self, SelectionMode, Type};

#[derive(Clone)]
pub struct LumberTypeModel {
    pub scrolled_win: gtk::ScrolledWindow,
    tree_view: gtk::TreeView,
    list_store: gtk::ListStore,
    columns: Vec<gtk::TreeViewColumn>,
}

impl LumberTypeModel {
    pub fn new() -> Self {
        let scrolled_win = gtk::ScrolledWindow::new(None, None);
        let tree_view = gtk::TreeView::new();
        let mut columns: Vec<gtk::TreeViewColumn> = Vec::new();

        let list_store = gtk::ListStore::new(&[
            Type::String, // type name
            Type::String, // FOB price
        ]);

        append_column("Lumber Type", &mut columns, &tree_view, None);
        append_column("FOB Price", &mut columns, &tree_view, None);

        tree_view.set_model(Some(&list_store));
        tree_view.set_headers_visible(true);
        tree_view.set_headers_clickable(false);
        tree_view.get_selection().set_mode(SelectionMode::None);
        scrolled_win.add(&tree_view);

        LumberTypeModel {
            scrolled_win,
            tree_view,
            list_store,
            columns,
        }
    }

    pub fn update_model(&self) {
        self.list_store.clear();

        for lt in LumberType::enumerate() {
            self.list_store.insert_with_values(
                None,
                &[0, 1],
                &[
                    &lt.to_str(),
                    &format!("{}", us_style().display_for(&lt.fob_price())),
                ],
            );
        }
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
