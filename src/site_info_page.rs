use gtk::prelude::*;
use gtk::{self, Widget};

use elis::SiteInfo;
use notebook::NoteBook;
use site_info_model::SiteInfoModel;

#[derive(Clone)]
pub struct SiteInfoPage {
    vertical_layout: gtk::Box,
    site_info_model: SiteInfoModel,
}

impl SiteInfoPage {
    pub fn new(note: &mut NoteBook) -> Self {
        let site_info_model = SiteInfoModel::new();
        let vertical_layout = gtk::Box::new(gtk::Orientation::Vertical, 0);

        site_info_model.update_model(&SiteInfo::default());

        vertical_layout.pack_start(&site_info_model.tree_view, true, true, 0);

        let vertical_layout: Widget = vertical_layout.upcast();
        note.create_tab("Site Info", &vertical_layout);

        SiteInfoPage {
            vertical_layout: vertical_layout
                .downcast::<gtk::Box>()
                .expect("Virtical layout downcast failed"),
            site_info_model,
        }
    }
}
