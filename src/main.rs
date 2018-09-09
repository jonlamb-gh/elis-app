// TODO - break apart new invoice logic from main page

extern crate elis_lib as elis;
extern crate gio;
extern crate glib;
extern crate gtk;

use gio::prelude::*;
use gtk::prelude::*;

use std::env::args;

mod items_model;
mod notebook;

use std::cell::RefCell;
use std::rc::Rc;
use elis::*;
use items_model::{ItemsModel, add_item_to_model};
use notebook::NoteBook;

// make moving clones into closures more convenient
macro_rules! clone {
    (@param _) => ( _ );
    (@param $x:ident) => ( $x );
    ($($n:ident),+ => move || $body:expr) => (
        {
            $( let $n = $n.clone(); )+
            move || $body
        }
    );
    ($($n:ident),+ => move |$($p:tt),+| $body:expr) => (
        {
            $( let $n = $n.clone(); )+
            move |$(clone!(@param $p),)+| $body
        }
    );
}

fn update_window(invoice: &Invoice, list: &gtk::ListStore) {
    list.clear();
    for item in invoice.items() {
        add_item_to_model(item, list);
    }
}

fn build_ui(application: &gtk::Application) {
    let window = gtk::ApplicationWindow::new(application);
    let mut note = NoteBook::new();

    // TODO - move to top level app struct
    let invoice: Rc<RefCell<Invoice>> = Rc::new(RefCell::new(Invoice::new()));

    let items_model = ItemsModel::new(&[], &mut note);
    let new_item_button = items_model.new_item_button.clone();

    window.set_title("ELIS");
    window.set_border_width(10);
    window.set_position(gtk::WindowPosition::Center);
    window.set_default_size(640, 480);

    window.connect_delete_event(clone!(window => move |_, _| {
        window.destroy();
        Inhibit(false)
    }));

    let v_box = gtk::Box::new(gtk::Orientation::Vertical, 0);
    v_box.pack_start(&note.notebook, true, true, 0);
    window.add(&v_box);

    window.show_all();
    window.activate();

    let list_store = items_model.list_store.clone();
    new_item_button.connect_clicked(clone!(invoice => move |_| {
        let item = BillableItem::new();
        invoice.borrow_mut().add_billable_item(item);
        update_window(&invoice.borrow(), &list_store);
    }));
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
