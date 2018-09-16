// TODO - need to update sytle/consistency/etc
// results/error-handling
//
// model level modules:
// update_values()
// get_widget()
// clear()?
//
// page level modules:
// page_index()
// update_models()?
// load_initial_values()?
// connect_handlers()?

extern crate elis_lib as elis;
extern crate gio;
extern crate glib;
extern crate gtk;

use gio::prelude::*;
use gtk::prelude::*;
use std::cell::RefCell;
use std::env::args;
use std::rc::Rc;

#[macro_use]
mod macros;
mod customer_query_results_model;
mod customer_search_page;
mod db_provider;
mod default_column;
mod invoice_query_results_model;
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
use elis::lumber::Lumber;
use elis::steel_cent::currency::USD;
use elis::steel_cent::Money;
use elis::{database_from_path, CustomerInfo, Database, SiteInfo};
use invoice_search_page::InvoiceSearchPage;
use new_customer_page::NewCustomerPage;
use new_invoice_page::NewInvoicePage;
use notebook::NoteBook;
use site_info_page::SiteInfoPage;

// TODO - which structs need to be refcell/etc?
pub fn build_ui(application: &gtk::Application) {
    let db: Rc<RefCell<Database>> = Rc::new(RefCell::new(
        database_from_path("elis.db").expect("Failed to open database"),
    ));

    // TODO - error handling
    let _ = db.borrow().load();

    // TESTING - populate the lumber type db with some samples
    db.borrow()
        .write(|db| {
            let lt = Lumber::new("Douglas Fir".to_string(), Money::of_major_minor(USD, 2, 60));
            db.lumber_types.insert(lt.type_name().to_string(), lt);
            let lt = Lumber::new("Red Pine".to_string(), Money::of_major_minor(USD, 1, 73));
            db.lumber_types.insert(lt.type_name().to_string(), lt);
            let lt = Lumber::new("Pine".to_string(), Money::of_major_minor(USD, 1, 17));
            db.lumber_types.insert(lt.type_name().to_string(), lt);
        }).expect("Failed to write to database");

    // construct a sample site
    db.borrow()
        .write(|db| {
            db.site_info = SiteInfo::new(
                "WoodSpot".to_string(),
                "PO Box 1234 Somewher, AB 92992".to_string(),
                "345-290-2343".to_string(),
                "".to_string(),
                0.088,
            );
        }).expect("Failed to write to database");

    // sample customers
    db.borrow()
        .write(|db| {
            let customer = CustomerInfo::new(
                "Jon".to_string(),
                "123 Somewhere".to_string(),
                "123-203-2343".to_string(),
                String::new(),
            );
            db.customers.insert(customer.name().to_string(), customer);
            let customer = CustomerInfo::new(
                "Steve".to_string(),
                "429 Somewhere".to_string(),
                "123-203-2343".to_string(),
                String::new(),
            );
            db.customers.insert(customer.name().to_string(), customer);
        }).expect("Failed to write to database");

    // save it
    db.borrow().save().expect("Failed to save database");

    let window = gtk::ApplicationWindow::new(application);
    let mut note = NoteBook::new();
    let new_invoice_page = NewInvoicePage::new(&mut note, db.clone());
    let invoice_search_page = InvoiceSearchPage::new(&mut note, db.clone());
    let _new_customer_page = NewCustomerPage::new(&mut note);
    let customer_search_page = CustomerSearchPage::new(&mut note, db.clone());
    let site_info_page = SiteInfoPage::new(&mut note, db.clone());

    // TODO -  read notes on rustbreak panics in closures, corrupts db
    //  move this down into the page logic?
    new_invoice_page
        .save_invoice_button
        .connect_clicked(clone!(db, new_invoice_page => move |_| {

        let invoice = new_invoice_page.replace_invoice();

        println!("Adding invoice {} to database", invoice.order_info().order_number());

        db.borrow().write(|db| {
            // TODO - check for existing key/orderNumber somewhere
            if ! db.invoices.contains_key(&invoice.order_info().order_number()) {
                db.invoices.insert(
                    invoice.order_info().order_number(),
                    invoice.clone(),
                );
            } else {
                println!("ignoring existent invoice");
            }
        }).expect("Failed to write to database");

        db.borrow().save().expect("Failed to save database");
    }));

    // TODO - hacky
    note.notebook.connect_switch_page(
        clone!(db, invoice_search_page => move |_nb, _page, page_index| {
            if page_index == invoice_search_page.index() {
                db.borrow().read(|db| {
                    invoice_search_page.set_results(db.invoices.values());
                }).expect("Failed to read from database");
            } else if page_index == site_info_page.index() {
                site_info_page.update_models();
            } else if page_index == customer_search_page.index() {
                db.borrow().read(|db| {
                    customer_search_page.set_results(db.customers.values());
                }).expect("Failed to read from database");
            }
        }),
    );

    window.set_title("ELIS 0.0.1");
    window.set_border_width(10);
    window.set_position(gtk::WindowPosition::Center);
    window.set_default_size(1024, 768);

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
    let application = gtk::Application::new("com.github.elis-app", gio::ApplicationFlags::empty())
        .expect("Initialization failed");

    application.connect_startup(|app| {
        build_ui(app);
    });

    application.connect_activate(|_| {});

    application.run(&args().collect::<Vec<_>>());
}
