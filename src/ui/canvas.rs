use gtk4::prelude::*;
use gtk4::glib;
use gtk4::Widget;

mod imp {
    use gtk4::gdk;
    use gtk4::graphene;
    use gtk4::glib;
    use gtk4::glib::subclass::types::{ObjectSubclass, ObjectSubclassExt};
    use gtk4::prelude::*;
    use gtk4::subclass::widget::WidgetImpl;
    use gtk4::glib::subclass::object::ObjectImpl;
    use gtk4::Widget;

    #[derive(Default)]
    pub struct Canvas {}

    #[glib::object_subclass]
    impl ObjectSubclass for Canvas {
        const NAME: &'static str = "NotewiseCanvas";
        type Type = super::Canvas;
        type ParentType = Widget;
    }

    impl ObjectImpl for Canvas {}

    impl WidgetImpl for Canvas {
        fn snapshot(&self, snapshot: &gtk4::Snapshot) {
            let widget = self.obj();
            let width = widget.width() as f32;
            let height = widget.height() as f32;

            let bg = gdk::RGBA::new(1.0, 1.0, 1.0, 1.0);
            snapshot.append_color(
                &bg,
                &graphene::Rect::new(0.0, 0.0, width, height),
            );
        }
    }
}

glib::wrapper! {
    pub struct Canvas(ObjectSubclass<imp::Canvas>)
        @extends Widget;
}

impl Canvas {
    pub fn new() -> Self {
        glib::Object::new()
    }

    pub fn widget(&self) -> Widget {
        self.clone().upcast()
    }
}
