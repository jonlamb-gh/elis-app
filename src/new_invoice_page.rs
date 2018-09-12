use glib::object::Cast;
use gtk::prelude::*;
use gtk::{self, Widget};
use std::cell::{Cell, RefCell};
use std::rc::Rc;

use elis::*;

use invoice_summary_model::InvoiceSummaryModel;
use items_model::{add_item_to_model, ItemId, ItemsModel};
use notebook::NoteBook;
use order_info_model::OrderInfoModel;

#[derive(Clone)]
pub struct NewInvoicePage {
    vertical_layout: gtk::Box,
    new_item_button: gtk::Button,
    delete_item_button: gtk::Button,
    clear_invoice_button: gtk::Button,
    save_pdf_button: gtk::Button,
    pub review_submit_button: gtk::Button,
    order_info_model: OrderInfoModel,
    items_model: ItemsModel,
    summary_model: InvoiceSummaryModel,
    pub invoice: Rc<RefCell<Invoice>>,
    selected_item_id: Rc<Cell<Option<ItemId>>>,
}

impl NewInvoicePage {
    pub fn new(note: &mut NoteBook) -> Self {
        let order_info_model = OrderInfoModel::new();
        let items_model = ItemsModel::new();
        let summary_model = InvoiceSummaryModel::new();
        let new_item_button = gtk::Button::new_with_label("Add");
        let delete_item_button = gtk::Button::new_with_label("Delete");
        let clear_invoice_button = gtk::Button::new_with_label("Clear Invoice");
        let save_pdf_button = gtk::Button::new_with_label("Save PDF");
        let review_submit_button = gtk::Button::new_with_label("Review and Submit");
        let vertical_layout = gtk::Box::new(gtk::Orientation::Vertical, 0);
        let horizontal_layout = gtk::Grid::new();
        let selected_item_id = Rc::new(Cell::new(None));
        let invoice = Rc::new(RefCell::new(Invoice::new(1)));

        order_info_model.update_model(invoice.borrow().order_info());
        summary_model.update_model(&invoice.borrow().summary());

        new_item_button.set_sensitive(true);
        delete_item_button.set_sensitive(false);
        clear_invoice_button.set_sensitive(true);
        save_pdf_button.set_sensitive(false);
        review_submit_button.set_sensitive(false);

        // TODO - refactor a global refresh routine
        let list_store = items_model.list_store.clone();
        new_item_button.connect_clicked(
            clone!(invoice, summary_model, review_submit_button => move |_| {
            let item = BillableItem::new();
            invoice.borrow_mut().add_billable_item(item);
            refresh_items_model(&invoice.borrow(), &list_store);
            summary_model.update_model(&invoice.borrow().summary());
            review_submit_button.set_sensitive(true);
        }),
        );

        let list_store = items_model.list_store.clone();
        delete_item_button.connect_clicked(
            clone!(invoice, selected_item_id, summary_model, review_submit_button => move |_| {
            if let Some(item_id) = selected_item_id.get() {
                invoice.borrow_mut().remove_billable_item(item_id);
                refresh_items_model(&invoice.borrow(), &list_store);
                summary_model.update_model(&invoice.borrow().summary());
            }

            if invoice.borrow().items().len() == 0 {
                review_submit_button.set_sensitive(false);
            }
        }),
        );

        let list_store = items_model.list_store.clone();
        clear_invoice_button.connect_clicked(
            clone!(invoice, summary_model, review_submit_button => move |_| {
            review_submit_button.set_sensitive(false);
            invoice.borrow_mut().remove_billable_items();
            refresh_items_model(&invoice.borrow(), &list_store);
            summary_model.update_model(&invoice.borrow().summary());
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
        horizontal_layout.attach(&save_pdf_button, 3, 0, 1, 1);
        horizontal_layout.attach(&review_submit_button, 4, 0, 1, 1);
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
            save_pdf_button,
            review_submit_button,
            order_info_model,
            items_model,
            summary_model,
            invoice,
            selected_item_id,
        }
    }

    pub fn replace_invoice(&self, new_order_number: OrderNumber) -> Invoice {
        let new_invoice = Invoice::new(new_order_number);

        self.review_submit_button.set_sensitive(false);
        self.order_info_model.update_model(new_invoice.order_info());
        self.summary_model.update_model(&new_invoice.summary());
        self.items_model.list_store.clear();

        self.invoice.replace(Invoice::new(new_order_number))
    }
}

fn refresh_items_model(invoice: &Invoice, list: &gtk::ListStore) {
    list.clear();
    for (id, item) in invoice.items().iter().enumerate() {
        add_item_to_model(item, id as ItemId, list);
    }
}
