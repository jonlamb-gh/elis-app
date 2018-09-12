extern crate elis_lib as elis;
extern crate gio;
extern crate glib;
extern crate gtk;

use gio::prelude::*;
use gtk::prelude::*;
use std::cell::{Cell, RefCell};
use std::env::args;
use std::rc::Rc;

#[macro_use]
mod macros;
mod customer_search_page;
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
use elis::from_path as db_from_path;
use elis::{Database, OrderNumber};
use invoice_search_page::InvoiceSearchPage;
use new_customer_page::NewCustomerPage;
use new_invoice_page::NewInvoicePage;
use notebook::NoteBook;
use site_info_page::SiteInfoPage;

// TODO - which structs need to be refcell/etc?
pub fn build_ui(application: &gtk::Application) {
    let window = gtk::ApplicationWindow::new(application);
    let mut note = NoteBook::new();
    let new_invoice_page = NewInvoicePage::new(&mut note);
    let invoice_search_page = InvoiceSearchPage::new(&mut note);
    let _new_customer_page = NewCustomerPage::new(&mut note);
    let _customer_search_page = CustomerSearchPage::new(&mut note);
    let _site_info_page = SiteInfoPage::new(&mut note);
    // TODO - this should be provided by the db
    let next_order_number: Rc<Cell<OrderNumber>> = Rc::new(Cell::new(1));
    let db: Rc<RefCell<Database>> = Rc::new(RefCell::new(
        db_from_path("elis.db").expect("Failed to open database"),
    ));

    // hacky way to pick a starting number until db is usable
    if db.borrow().load().is_ok() {
        let mut next_order_num: OrderNumber = 0;
        db.borrow()
            .read(|db| {
                for order_num in db.invoices.keys() {
                    if *order_num > next_order_num {
                        next_order_num = *order_num;
                    }
                }
            }).expect("Failed to read from database");
        next_order_num += 1;
        println!("next order number = {}", next_order_num);
        next_order_number.set(next_order_num);
        let _ = new_invoice_page.replace_invoice(next_order_number.get());
        next_order_number.set(next_order_num + 1);
    }

    // TODO -  read notes on rustbreak panics in closures, corrupts db
    new_invoice_page.save_invoice_button.connect_clicked(
        clone!(db, new_invoice_page, next_order_number => move |_| {

        let next_order_num = next_order_number.get();
        let invoice = new_invoice_page.replace_invoice(next_order_num);
        next_order_number.set(next_order_num + 1);

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
    }),
    );

    note.notebook.connect_switch_page(
        clone!(invoice_search_page => move |_nb, _page, page_index| {
            println!("switch-page : page = {}", page_index);

            if page_index == invoice_search_page.page_index {
                db.borrow().read(|db| {
                    invoice_search_page.set_results(db.invoices.values());
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
    let application = gtk::Application::new("ELIS", gio::ApplicationFlags::empty())
        .expect("Initialization failed");

    application.connect_startup(|app| {
        build_ui(app);
    });

    application.connect_activate(|_| {});

    application.run(&args().collect::<Vec<_>>());
}
