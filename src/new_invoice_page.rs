use elis::lumber::{FobCostReader, Lumber};
use elis::steel_cent::{currency::USD, Money};
use elis::{BillableItem, Database, Invoice, OrderNumber};
use glib::object::Cast;
use gtk::prelude::*;
use gtk::{self, Widget};
use std::cell::{Cell, RefCell};
use std::rc::Rc;

use fob_reader::FobReader;
use invoice_summary_model::InvoiceSummaryModel;
use items_model::{ItemId, ItemsModel};
use notebook::NoteBook;
use order_info_model::OrderInfoModel;

#[derive(Clone)]
pub struct NewInvoicePage {
    vertical_layout: gtk::Box,
    new_item_button: gtk::Button,
    delete_item_button: gtk::Button,
    clear_invoice_button: gtk::Button,
    pub save_invoice_button: gtk::Button,
    order_info_model: OrderInfoModel,
    items_model: ItemsModel,
    summary_model: InvoiceSummaryModel,
    pub invoice: Rc<RefCell<Invoice>>,
    selected_item_id: Rc<Cell<Option<ItemId>>>,
    // TODO - make this better
    default_item_lumber_type: String,
    fob_reader: FobReader,
}

impl NewInvoicePage {
    pub fn new(note: &mut NoteBook, db: Rc<RefCell<Database>>) -> Self {
        let fob_reader = FobReader { db };
        let order_info_model = OrderInfoModel::new();
        let items_model = ItemsModel::new();
        let summary_model = InvoiceSummaryModel::new();
        let new_item_button = gtk::Button::new_with_label("Add");
        let delete_item_button = gtk::Button::new_with_label("Delete");
        let clear_invoice_button = gtk::Button::new_with_label("Clear Invoice");
        let save_invoice_button = gtk::Button::new_with_label("Save Invoice");
        let vertical_layout = gtk::Box::new(gtk::Orientation::Vertical, 0);
        let horizontal_layout = gtk::Grid::new();
        let selected_item_id = Rc::new(Cell::new(None));
        let invoice = Rc::new(RefCell::new(Invoice::new(1)));

        let mut first_lumber_data = Lumber::new(String::new(), Money::zero(USD));
        fob_reader
            .db
            .borrow()
            .read(|db| {
                let data = db
                    .lumber_types
                    .values()
                    .next()
                    .expect("Failed to get lumber type");
                first_lumber_data = data.clone();
            }).expect("Failed to read from database");
        let default_item_lumber_type = String::from(first_lumber_data.type_name());

        order_info_model.update_model(invoice.borrow().order_info());
        summary_model.update_model(&invoice.borrow().summary(&fob_reader));

        new_item_button.set_sensitive(true);
        delete_item_button.set_sensitive(false);
        clear_invoice_button.set_sensitive(true);
        save_invoice_button.set_sensitive(false);

        // TODO - refactor a global refresh routine
        new_item_button.connect_clicked(
            clone!(fob_reader, invoice, items_model, summary_model, default_item_lumber_type, save_invoice_button => move |_| {
            let item = BillableItem::new(default_item_lumber_type.clone());
            invoice.borrow_mut().add_billable_item(item);
            refresh_items_model(&invoice.borrow(), &items_model, &fob_reader);
            summary_model.update_model(&invoice.borrow().summary(&fob_reader));
            save_invoice_button.set_sensitive(true);
        }),
        );

        delete_item_button.connect_clicked(
            clone!(fob_reader, invoice, selected_item_id, items_model,summary_model, save_invoice_button => move |_| {
            if let Some(item_id) = selected_item_id.get() {
                invoice.borrow_mut().remove_billable_item(item_id);
                refresh_items_model(&invoice.borrow(), &items_model, &fob_reader);
                summary_model.update_model(&invoice.borrow().summary(&fob_reader));
            }

            if invoice.borrow().billable_items().len() == 0 {
                save_invoice_button.set_sensitive(false);
            }
        }),
        );

        clear_invoice_button.connect_clicked(
            clone!(fob_reader, invoice, items_model, summary_model, save_invoice_button => move |_| {
            save_invoice_button.set_sensitive(false);
            invoice.borrow_mut().clear_billable_items();
            refresh_items_model(&invoice.borrow(), &items_model, &fob_reader);
            summary_model.update_model(&invoice.borrow().summary(&fob_reader));
        }),
        );

        items_model.tree_view.connect_cursor_changed(
            clone!(delete_item_button, selected_item_id => move |tree_view| {
            let selection = tree_view.get_selection();
            let (id, selected) = if let Some((model, iter)) = selection.get_selected() {
                if let Some(x) = model.get_value(&iter, 7).get::<u32>().map(|x| x as ItemId) {
                    (Some(x), true)
                } else {
                    (None, false)
                }
            } else {
                (None, false)
            };

            selected_item_id.set(id);
            delete_item_button.set_sensitive(selected);
        }),
        );

        vertical_layout.pack_start(&order_info_model.tree_view, false, false, 0);
        vertical_layout.pack_start(&items_model.scrolled_win, true, true, 0);
        horizontal_layout.attach(&new_item_button, 0, 0, 1, 1);
        horizontal_layout.attach(&delete_item_button, 1, 0, 1, 1);
        horizontal_layout.attach(&clear_invoice_button, 2, 0, 1, 1);
        horizontal_layout.attach(&save_invoice_button, 3, 0, 1, 1);
        horizontal_layout.set_column_homogeneous(false);
        vertical_layout.pack_start(&horizontal_layout, false, true, 0);
        vertical_layout.pack_start(&summary_model.tree_view, false, false, 0);

        let vertical_layout: Widget = vertical_layout.upcast();
        note.create_tab("New Invoice", &vertical_layout);

        NewInvoicePage {
            vertical_layout: vertical_layout
                .downcast::<gtk::Box>()
                .expect("Virtical layout downcast failed"),
            new_item_button,
            delete_item_button,
            clear_invoice_button,
            save_invoice_button,
            order_info_model,
            items_model,
            summary_model,
            invoice,
            selected_item_id,
            default_item_lumber_type,
            fob_reader,
        }
    }

    pub fn replace_invoice(&self, new_order_number: OrderNumber) -> Invoice {
        let new_invoice = Invoice::new(new_order_number);

        self.save_invoice_button.set_sensitive(false);
        self.order_info_model.update_model(new_invoice.order_info());
        self.summary_model
            .update_model(&new_invoice.summary(&self.fob_reader));
        self.items_model.clear_model();

        self.invoice.replace(Invoice::new(new_order_number))
    }
}

fn refresh_items_model<T: FobCostReader>(invoice: &Invoice, model: &ItemsModel, fob_reader: &T) {
    model.clear_model();
    for (id, item) in invoice.billable_items().iter().enumerate() {
        model.update_model(item, id as ItemId, fob_reader);
    }
}
