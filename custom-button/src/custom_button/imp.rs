use glib::Object;
use gtk::glib;
use gtk::subclass::prelude::*;

// 通过 glib::wrapper 包裹 CustomButton 声明
glib::wrapper! {
    pub struct CustomButton(ObjectSubclass<CustomButtonClass>)
        @extends gtk::Button, gtk::Widget,
        @implements gtk::Accessible, gtk::Actionable, gtk::Buildable, gtk::ConstraintTarget;
}

// CustomButton 实现 ObjectSubclass
impl CustomButton {
    pub fn new() -> Self {
        Object::builder().build()
    }

    pub fn with_label(label: &str) -> Self {
        Object::builder().property("label", label).build()
    }
}

impl Default for CustomButton {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Default)]
pub struct CustomButtonClass;
#[glib::object_subclass]
impl ObjectSubclass for CustomButtonClass {
    const NAME: &'static str = "CustomButtonClass";
    type Type = CustomButton; // 之后将被创建的实际GObject
    type ParentType = gtk::Button;
}

// Trait shared by all GObjects
impl ObjectImpl for CustomButtonClass {}

// Trait shared by all widgets
impl WidgetImpl for CustomButtonClass {}

// Trait shared by all buttons
impl ButtonImpl for CustomButtonClass {}
