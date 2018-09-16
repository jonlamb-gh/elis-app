use gtk;
use gtk::prelude::*;

const DEFAULT_ROW_HEIGHT: i32 = 35;

pub fn default_column(
    title: &str,
    tree_view: &gtk::TreeView,
    columns: &mut Vec<gtk::TreeViewColumn>,
) -> gtk::CellRendererText {
    let id = columns.len() as i32;
    let renderer = gtk::CellRendererText::new();
    renderer.set_property_xalign(0.0);
    renderer.set_fixed_size(-1, DEFAULT_ROW_HEIGHT);

    let column = gtk::TreeViewColumn::new();
    column.set_title(title);
    column.set_resizable(true);
    column.set_min_width(50);
    column.pack_start(&renderer, true);
    column.add_attribute(&renderer, "text", id);
    column.set_clickable(true);
    column.set_sort_column_id(id);

    tree_view.append_column(&column);
    columns.push(column);

    renderer
}

pub fn default_center_column(
    title: &str,
    tree_view: &gtk::TreeView,
    columns: &mut Vec<gtk::TreeViewColumn>,
) -> gtk::CellRendererText {
    let renderer = default_column(title, tree_view, columns);
    renderer.set_property_xalign(0.5);
    renderer
}

pub fn default_right_column(
    title: &str,
    tree_view: &gtk::TreeView,
    columns: &mut Vec<gtk::TreeViewColumn>,
) -> gtk::CellRendererText {
    let renderer = default_column(title, tree_view, columns);
    renderer.set_property_xalign(1.0);
    renderer
}

pub fn default_combo_column(
    title: &str,
    combo_model: &gtk::ListStore,
    tree_view: &gtk::TreeView,
    columns: &mut Vec<gtk::TreeViewColumn>,
) -> gtk::CellRendererCombo {
    let id = columns.len() as i32;

    let renderer = gtk::CellRendererCombo::new();
    renderer.set_fixed_size(-1, DEFAULT_ROW_HEIGHT);
    renderer.set_visible(true);
    renderer.set_property_editable(true);
    renderer.set_sensitive(true);
    renderer.set_property_model(Some(combo_model));
    renderer.set_property_text_column(0);

    let column = gtk::TreeViewColumn::new();
    column.set_visible(true);
    column.set_title(title);
    column.set_resizable(true);
    column.set_expand(false);
    column.set_min_width(50);
    column.pack_start(&renderer, true);
    column.add_attribute(&renderer, "text", id);
    column.set_clickable(true);
    column.set_sort_column_id(id);

    tree_view.append_column(&column);
    columns.push(column);

    renderer
}

pub fn default_toggle_column(
    title: &str,
    tree_view: &gtk::TreeView,
    columns: &mut Vec<gtk::TreeViewColumn>,
) -> gtk::CellRendererToggle {
    let id = columns.len() as i32;

    let renderer = gtk::CellRendererToggle::new();
    renderer.set_fixed_size(-1, DEFAULT_ROW_HEIGHT);
    renderer.set_visible(true);
    renderer.set_activatable(true);
    renderer.set_sensitive(true);
    renderer.set_property_xalign(0.0);

    let column = gtk::TreeViewColumn::new();
    column.set_visible(true);
    column.set_title(title);
    column.set_resizable(true);
    column.set_expand(false);
    column.set_min_width(50);
    column.pack_start(&renderer, true);
    column.set_clickable(true);
    column.set_sort_column_id(id);

    tree_view.append_column(&column);
    columns.push(column);

    renderer
}
