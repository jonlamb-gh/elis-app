// TODO - break apart new invoice logic from main page
// docs on objects:
// https://developer.gnome.org/gtk3/stable/TreeWidgetObjects.html

extern crate elis_lib as elis;
extern crate gio;
extern crate glib;
extern crate gtk;

use gio::prelude::*;
use gtk::prelude::*;

use std::env::args;

#[macro_use]
mod macros;
mod customer_search_page;
mod invoice_search_page;
mod invoice_summary_model;
mod items_model;
mod lumber_type_model;
mod new_customer_page;
mod new_invoice_page;
mod notebook;
mod order_info_model;
mod site_info_model;
mod site_info_page;

use customer_search_page::CustomerSearchPage;
use invoice_search_page::InvoiceSearchPage;
use new_customer_page::NewCustomerPage;
use new_invoice_page::NewInvoicePage;
use notebook::NoteBook;
use site_info_page::SiteInfoPage;

fn build_ui(application: &gtk::Application) {
    let window = gtk::ApplicationWindow::new(application);
    let mut note = NoteBook::new();
    let _new_invoice_page = NewInvoicePage::new(&mut note);
    let _invoice_search_page = InvoiceSearchPage::new(&mut note);
    let _new_customer_page = NewCustomerPage::new(&mut note);
    let _customer_search_page = CustomerSearchPage::new(&mut note);
    let _site_info_page = SiteInfoPage::new(&mut note);

    window.set_title("ELIS 0.0.1");
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
