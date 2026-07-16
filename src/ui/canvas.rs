use gtk4::gdk;
use gtk4::glib;
use gtk4::graphene;
use gtk4::pango;
use gtk4::prelude::*;
use gtk4::Widget;

use std::cell::RefCell;
use std::rc::Rc;

use crate::model::{CanvasItem, TextAlign, TextItem, TextStyle, ViewTransform};

mod imp {
    use gtk4::gdk;
    use gtk4::glib;
    use gtk4::glib::subclass::object::ObjectImpl;
    use gtk4::glib::subclass::types::{ObjectSubclass, ObjectSubclassExt};
    use gtk4::graphene;
    use gtk4::prelude::*;
    use gtk4::subclass::widget::WidgetImpl;
    use gtk4::Widget;

    use std::cell::RefCell;
    use std::rc::Rc;

    use crate::model::{CanvasItem, ViewTransform};

    pub struct CanvasState {
        pub items: Vec<CanvasItem>,
        pub view: ViewTransform,
        pub next_id: u64,
    }

    impl Default for CanvasState {
        fn default() -> Self {
            Self {
                items: Vec::new(),
                view: ViewTransform::default(),
                next_id: 1,
            }
        }
    }

    #[derive(Default)]
    pub struct Canvas {
        pub state: Rc<RefCell<CanvasState>>,
    }

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
            snapshot.append_color(&bg, &graphene::Rect::new(0.0, 0.0, width, height));

            let state = self.state.borrow();
            for item in state.items.iter() {
                if let CanvasItem::Text(t) = item {
                    super::draw_text(widget.upcast_ref::<gtk4::Widget>(), snapshot, t, &state.view);
                }
            }
        }
    }
}

glib::wrapper! {
    pub struct Canvas(ObjectSubclass<imp::Canvas>)
        @extends Widget;
}

fn draw_text(
    widget: &Widget,
    snap: &gtk4::Snapshot,
    item: &TextItem,
    view: &ViewTransform,
) {
    let (sx, sy) = view.world_to_screen(item.pos);
    let scale = view.scale;

    snap.save();
    snap.translate(&graphene::Point::new(sx as f32, sy as f32));
    snap.scale(scale as f32, scale as f32);

    let layout = widget.create_pango_layout(Some(&item.text));
    let desc = item.style.build_font_description();
    layout.set_font_description(Some(&desc));
    layout.set_width((item.width * pango::SCALE as f64) as i32);
    match item.align {
        TextAlign::Left => layout.set_alignment(pango::Alignment::Left),
        TextAlign::Center => layout.set_alignment(pango::Alignment::Center),
        TextAlign::Right => layout.set_alignment(pango::Alignment::Right),
    }

    let color = gdk::RGBA::new(
        item.style.color_rgba[0] as f32,
        item.style.color_rgba[1] as f32,
        item.style.color_rgba[2] as f32,
        item.style.color_rgba[3] as f32,
    );
    snap.append_layout(&layout, &color);

    snap.restore();
}

use gtk4::glib::subclass::types::ObjectSubclassIsExt;

impl Canvas {
    pub fn new() -> Self {
        let canvas: Self = glib::Object::new();

        canvas.setup_pan();
        canvas.setup_zoom();
        canvas.setup_click_to_add();

        canvas
    }

    pub fn widget(&self) -> Widget {
        self.clone().upcast()
    }

    pub fn state(&self) -> Rc<RefCell<imp::CanvasState>> {
        self.imp().state.clone()
    }

    fn setup_pan(&self) {
        let drag = gtk4::GestureDrag::new();
        drag.set_button(gdk::BUTTON_MIDDLE);

        let canvas = self.clone();
        drag.connect_drag_update(move |_, dx, dy| {
            let state_rc = canvas.state();
            let mut state = state_rc.borrow_mut();
            state.view.pan.x -= dx / state.view.scale;
            state.view.pan.y -= dy / state.view.scale;
            drop(state);
            canvas.queue_draw();
        });
        let canvas = self.clone();
        drag.connect_drag_end(move |_, _, _| {
            canvas.queue_draw();
        });

        self.add_controller(drag);
    }

    fn setup_zoom(&self) {
        let scroll = gtk4::EventControllerScroll::new(
            gtk4::EventControllerScrollFlags::VERTICAL | gtk4::EventControllerScrollFlags::KINETIC,
        );

        let canvas = self.clone();
        let scroll_ctrl = scroll.clone();
        scroll.connect_scroll(move |_, _dx, dy| {
            let modifiers = scroll_ctrl.current_event_state();
            if !modifiers.contains(gdk::ModifierType::CONTROL_MASK) {
                return glib::Propagation::Proceed;
            }

            let state_rc = canvas.state();
            let mut state = state_rc.borrow_mut();
            let factor = if dy < 0.0 { 1.1 } else { 0.9 };
            let new_scale = (state.view.scale * factor).clamp(0.2, 10.0);
            state.view.scale = new_scale;
            drop(state);
            canvas.queue_draw();
            glib::Propagation::Stop
        });

        self.add_controller(scroll);
    }

    fn setup_click_to_add(&self) {
        let click = gtk4::GestureClick::new();
        click.set_button(gdk::BUTTON_PRIMARY);

        let canvas = self.clone();
        click.connect_pressed(move |_, _n, x, y| {
            let world = {
                let state_rc = canvas.state();
                let state = state_rc.borrow();
                state.view.screen_to_world(x, y)
            };

            let id = {
                let state_rc = canvas.state();
                let mut state = state_rc.borrow_mut();
                let id = state.next_id;
                state.next_id += 1;
                state.items.push(CanvasItem::Text(TextItem {
                    id,
                    pos: world,
                    width: 320.0,
                    text: "Neue Notiz".to_string(),
                    style: TextStyle::default(),
                    align: crate::model::TextAlign::Left,
                    underlined: false,
                }));
                id
            };

            let _ = id;
            canvas.queue_draw();
        });

        self.add_controller(click);
    }
}
