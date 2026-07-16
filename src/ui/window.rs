use gtk4::prelude::*;
use libadwaita::prelude::*;
use libadwaita::Application as AdwApplication;
use libadwaita::ApplicationWindow;
use libadwaita::HeaderBar;

use crate::ui::Canvas;

pub struct Window {
    window: ApplicationWindow,
}

impl Window {
    pub fn new(app: &AdwApplication) -> Self {
        let window = ApplicationWindow::builder()
            .application(app)
            .title("Notewise")
            .default_width(1100)
            .default_height(750)
            .build();

        let header = HeaderBar::builder().build();

        let new_button = gtk4::Button::with_label("Neue Notiz");
        new_button.add_css_class("suggested-action");
        header.pack_end(&new_button);

        let canvas = Canvas::new();

        let content = gtk4::Box::builder()
            .orientation(gtk4::Orientation::Vertical)
            .build();
        content.append(&header);
        content.append(&canvas.widget());

        window.set_content(Some(&content));

        Self { window }
    }

    pub fn present(&self) {
        self.window.present();
    }
}