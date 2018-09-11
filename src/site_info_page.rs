use gtk::prelude::*;
use gtk::{self, Widget};

use elis::SiteInfo;
use lumber_type_model::LumberTypeModel;
use notebook::NoteBook;
use site_info_model::SiteInfoModel;

#[derive(Clone)]
pub struct SiteInfoPage {
    vertical_layout: gtk::Box,
    site_info_model: SiteInfoModel,
    lumber_type_model: LumberTypeModel,
}

impl SiteInfoPage {
    pub fn new(note: &mut NoteBook) -> Self {
        let site_info_model = SiteInfoModel::new();
        let lumber_type_model = LumberTypeModel::new();
        let vertical_layout = gtk::Box::new(gtk::Orientation::Vertical, 0);

        site_info_model.update_model(&SiteInfo::default());
        lumber_type_model.update_model();

        vertical_layout.pack_start(&site_info_model.tree_view, false, false, 0);
        vertical_layout.pack_start(&lumber_type_model.scrolled_win, true, true, 0);

        let vertical_layout: Widget = vertical_layout.upcast();
        note.create_tab("Site Info", &vertical_layout);

        SiteInfoPage {
            vertical_layout: vertical_layout
                .downcast::<gtk::Box>()
                .expect("Virtical layout downcast failed"),
            site_info_model,
            lumber_type_model,
        }
    }
}
