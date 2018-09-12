use elis::{Database, SiteInfo};
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
    pub page_index: u32,
}

impl SiteInfoPage {
    pub fn new(note: &mut NoteBook, db: Rc<RefCell<Database>>) -> Self {
        let site_info_model = SiteInfoModel::new();
        let lumber_type_model = LumberTypeModel::new(db);
        let vertical_layout = gtk::Box::new(gtk::Orientation::Vertical, 0);

        site_info_model.update_model(&SiteInfo::new());
        lumber_type_model.update_model();

        vertical_layout.pack_start(&site_info_model.tree_view, false, false, 0);
        vertical_layout.pack_start(&lumber_type_model.scrolled_win, true, true, 0);

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

    pub fn update_models(&self) {
        self.lumber_type_model.update_model();
    }
}
