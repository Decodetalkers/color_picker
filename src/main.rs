mod pick;

use gtk4::prelude::*;
fn main() {
    let application =
        gtk4::Application::new(Some("com.gitlab.screencast.client"), Default::default());
    application.connect_activate(|app| {
        let window = gtk4::ApplicationWindow::new(app);
        window.set_title(Some("First GTK Program"));
        window.set_default_size(350, 70);

        let win = pick::PickWindow::new();
        window.set_child(Some(&win));
        window.show();
    });
    application.run();
}
