use gtk::prelude::*;
use gtk::{self, Widget};

use notebook::NoteBook;

#[derive(Clone)]
pub struct SiteInfoPage {
    pub vertical_layout: gtk::Box,
    /*pub tree_view: gtk::TreeView,
     *pub list_store: gtk::ListStore,
     *pub columns: Vec<gtk::TreeViewColumn>, */
}

impl SiteInfoPage {
    pub fn new(note: &mut NoteBook) -> Self {
        let vertical_layout = gtk::Box::new(gtk::Orientation::Vertical, 0);

        let vertical_layout: Widget = vertical_layout.upcast();
        note.create_tab("Site Info", &vertical_layout);

        SiteInfoPage {
            vertical_layout: vertical_layout
                .downcast::<gtk::Box>()
                .expect("Virtical layout downcast failed"),
        }
    }
}
