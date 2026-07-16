mod app;
mod model;
mod ui;

use gtk4::glib::ExitCode;

fn main() -> ExitCode {
    let app = app::App::new();
    app.run()
}
