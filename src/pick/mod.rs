use ashpd::desktop::screenshot::ScreenshotProxy;
use ashpd::zbus;
use ashpd::WindowIdentifier;
use gtk4::prelude::EditableExt;
use gtk4::subclass::prelude::*;
use gtk4::{gio, glib};

mod imp {
    use gtk4::prelude::*;
    use gtk4::subclass::prelude::*;
    use gtk4::{glib, CompositeTemplate};
    #[derive(Debug, Default, CompositeTemplate)]
    #[template(file = "pickwindow.ui")]
    pub struct PickWindow {
        #[template_child]
        pub line: TemplateChild<gtk4::Entry>,
        #[template_child]
        pub button: TemplateChild<gtk4::Button>,
    }
    #[glib::object_subclass]
    impl ObjectSubclass for PickWindow {
        const NAME: &'static str = "PickWindow";
        type Type = super::PickWindow;
        type ParentType = gtk4::Box;
        fn class_init(klass: &mut Self::Class) {
            Self::bind_template(klass);
            klass.install_action("start", None, move |page, _, _| {
                let ctx = glib::MainContext::default();
                ctx.spawn_local(glib::clone!(@weak page => async move {
                    page.pick_color().await.unwrap();
                }));
            });
        }

        // You must call `Widget`'s `init_template()` within `instance_init()`.
        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }
    impl ObjectImpl for PickWindow {
        fn constructed(&self, obj: &Self::Type) {
            obj.init_label();
            self.parent_constructed(obj);
        }
    }
    impl WidgetImpl for PickWindow {}
    impl BoxImpl for PickWindow {}
}
glib::wrapper! {
    pub struct PickWindow(ObjectSubclass<imp::PickWindow>)
        @extends gtk4::Widget, gtk4::Box, @implements gio::ActionMap, gio::ActionGroup;
}

impl PickWindow {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        glib::Object::new(&[]).expect("Failed to create a ScreenCastPage")
    }
    pub fn init_label(&self) {
        // To access fields such as template children, you must get
        // the private struct.
        let self_ = imp::PickWindow::from_instance(self);
        self_.line.set_text("Here is the color");
    }
    async fn pick_color(&self) -> ashpd::Result<()> {
        let connection = zbus::Connection::session().await?;
        let proxy = ScreenshotProxy::new(&connection).await?;
        let color = proxy.pick_color(&WindowIdentifier::default()).await?;
        let self_ = imp::PickWindow::from_instance(self);
        self_.line.set_text(&format!(
            "0x{:02X}{:02X}{:02X}",
            (color.red() * 255.0) as i32,
            (color.green() * 255.0) as i32,
            (color.blue() * 255.0) as i32
        ));
        #[cfg(debug_assertions)]
        println!(
            "({:02X}, {:02X}, {:02X})",
            (color.red() * 255.0) as i32,
            (color.green() * 255.0) as i32,
            (color.blue() * 255.0) as i32
        );
        Ok(())
    }
}
