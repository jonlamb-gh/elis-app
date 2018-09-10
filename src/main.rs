// TODO - break apart new invoice logic from main page

extern crate elis_lib as elis;
extern crate gio;
extern crate glib;
extern crate gtk;

use gio::prelude::*;
use gtk::prelude::*;

use std::env::args;

#[macro_use]
mod macros;
mod invoice_summary_model;
mod items_model;
mod new_invoice_model;
mod notebook;

use new_invoice_model::NewInvoiceModel;
use notebook::NoteBook;

fn build_ui(application: &gtk::Application) {
    let window = gtk::ApplicationWindow::new(application);
    let mut note = NoteBook::new();
    let _new_invoice_model = NewInvoiceModel::new(&mut note);

    window.set_title("Electronic Lumber Invoice System (ELIS V0.0.1)");
    window.set_border_width(10);
    window.set_position(gtk::WindowPosition::Center);
    window.set_default_size(768, 432);

    window.connect_delete_event(clone!(window => move |_, _| {
        window.destroy();
        Inhibit(false)
    }));

    let v_box = gtk::Box::new(gtk::Orientation::Vertical, 0);
    v_box.pack_start(&note.notebook, true, true, 0);
    window.add(&v_box);

    window.show_all();
    window.activate();
}

fn main() {
    let application = gtk::Application::new("com.github.basic", gio::ApplicationFlags::empty())
        .expect("Initialization failed...");

    application.connect_startup(|app| {
        build_ui(app);
    });
    application.connect_activate(|_| {});

    application.run(&args().collect::<Vec<_>>());
}
