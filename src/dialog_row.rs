use grammers_client::types::Dialog;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::glib;
use std::sync::Arc;

mod imp {
    use super::*;
    use gtk::CompositeTemplate;
    use std::cell::RefCell;

    #[derive(Default, CompositeTemplate)]
    #[template(resource = "/com/github/melix99/telegrand/dialog_row.ui")]
    pub struct DialogRow {
        #[template_child]
        pub chat_name_label: TemplateChild<gtk::Label>,
        #[template_child]
        pub last_message_label: TemplateChild<gtk::Label>,
        pub dialog: RefCell<Option<Arc<Dialog>>>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for DialogRow {
        const NAME: &'static str = "DialogRow";
        type Type = super::DialogRow;
        type ParentType = gtk::ListBoxRow;

        fn class_init(klass: &mut Self::Class) {
            Self::bind_template(klass);
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for DialogRow {
        fn constructed(&self, obj: &Self::Type) {
            self.parent_constructed(obj);
        }
    }

    impl WidgetImpl for DialogRow {}
    impl ListBoxRowImpl for DialogRow {}
}

glib::wrapper! {
    pub struct DialogRow(ObjectSubclass<imp::DialogRow>)
        @extends gtk::Widget, gtk::ListBoxRow;
}

impl DialogRow {
    pub fn new(dialog: Dialog) -> Self {
        let dialog_row = glib::Object::new(&[])
            .expect("Failed to create DialogRow");

        let self_ = imp::DialogRow::from_instance(&dialog_row);
        self_.dialog.replace(Some(Arc::new(dialog)));

        let dialog = self_.dialog.borrow().as_ref().unwrap().clone();
        let chat = dialog.chat();
        let chat_name = chat.name();
        self_.chat_name_label.set_text(chat_name);

        let last_message = dialog.last_message.as_ref().unwrap().text();
        self_.last_message_label.set_text(last_message);

        dialog_row
    }

    pub fn get_dialog(&self) -> Arc<Dialog> {
        let self_ = imp::DialogRow::from_instance(self);
        self_.dialog.borrow().clone().unwrap()
    }
}
