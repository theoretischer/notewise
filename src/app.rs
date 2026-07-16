use gtk4::glib::ExitCode;
use gtk4::prelude::*;
use libadwaita::Application as AdwApplication;

use crate::ui::Window;

const APP_ID: &str = "org.gnome.Notewise";

pub struct App {
    app: AdwApplication,
}

impl App {
    pub fn new() -> Self {
        let app = AdwApplication::builder()
            .application_id(APP_ID)
            .build();

        app.connect_startup(|_| {
            // Ressourcen / Settings später hier laden
        });

        app.connect_activate(build_ui);

        Self { app }
    }

    pub fn run(&self) -> ExitCode {
        gtk4::Application::run(self.app.upcast_ref::<gtk4::Application>())
    }
}

fn build_ui(app: &AdwApplication) {
    let window = Window::new(app);
    window.present();
}
