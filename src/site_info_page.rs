use elis::Database;
use gtk::prelude::*;
use gtk::{self, Widget};
use std::cell::RefCell;
use std::rc::Rc;

use lumber_type_model::LumberTypeModel;
use notebook::NoteBook;
use site_info_model::SiteInfoModel;

#[derive(Clone)]
pub struct SiteInfoPage {
    vertical_layout: gtk::Box,
    site_info_model: SiteInfoModel,
    lumber_type_model: LumberTypeModel,
    page_index: u32,
}

impl SiteInfoPage {
    pub fn new(note: &mut NoteBook, db: Rc<RefCell<Database>>) -> Self {
        let site_info_model = SiteInfoModel::new(db.clone());
        let lumber_type_model = LumberTypeModel::new(db.clone());
        let vertical_layout = gtk::Box::new(gtk::Orientation::Vertical, 0);

        site_info_model.update_values();
        lumber_type_model.update_values();

        vertical_layout.pack_start(site_info_model.get_widget(), false, false, 0);
        vertical_layout.pack_start(lumber_type_model.get_widget(), true, true, 0);

        let vertical_layout: Widget = vertical_layout.upcast();
        let page_index = note.create_tab("Site Info", &vertical_layout).unwrap();

        SiteInfoPage {
            vertical_layout: vertical_layout
                .downcast::<gtk::Box>()
                .expect("Virtical layout downcast failed"),
            site_info_model,
            lumber_type_model,
            page_index,
        }
    }

    pub fn index(&self) -> u32 {
        self.page_index
    }

    pub fn update_models(&self) {
        // TODO - need to fix this, row disappears when updated, but is there if resized
        self.site_info_model.update_values();
        self.lumber_type_model.update_values();
    }
}
