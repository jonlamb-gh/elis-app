use gtk::prelude::*;
use gtk::{CellRendererText, TreeView, TreeViewColumn};

pub fn default_column(
    title: &str,
    tree_view: &TreeView,
    columns: &mut Vec<TreeViewColumn>,
) -> CellRendererText {
    let id = columns.len() as i32;
    let renderer = CellRendererText::new();
    let column = TreeViewColumn::new();

    column.set_title(title);
    column.set_resizable(true);
    column.set_min_width(25);
    column.pack_start(&renderer, true);
    column.add_attribute(&renderer, "text", id);
    column.set_clickable(true);
    column.set_sort_column_id(id);

    tree_view.append_column(&column);
    columns.push(column);

    renderer
}
