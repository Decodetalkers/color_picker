
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
//use ashpd::desktop::camera::CameraProxy;
//
//pub async fn run2() -> ashpd::Result<()> {
//    let connection = zbus::Connection::session().await?;
//    let proxy = CameraProxy::new(&connection).await?;
//
//    if proxy.is_camera_present().await? {
//        proxy.access_camera().await?;
//        proxy.open_pipe_wire_remote().await?;
//        // pass the remote fd to GStreamer for example
//    }
//    Ok(())
//}
////use ashpd::desktop::screenshot;
////
//////use ashpd::desktop::camera::pipewire_node_id;
////async fn run3() -> ashpd::Result<()> {
////    let connection = zbus::Connection::session().await?;
////    let proxy = screenshot::ScreenshotProxy::new(&connection).await?;
////    Ok(())
////}
//use ashpd::desktop::screencast::{CursorMode, ScreenCastProxy, SourceType};
//use ashpd::enumflags2::BitFlags;
//
//async fn run3() -> ashpd::Result<()> {
//    let connection = zbus::Connection::session().await?;
//    let proxy = ScreenCastProxy::new(&connection).await?;
//
//    let session = proxy.create_session().await?;
//
//    proxy
//        .select_sources(
//            &session,
//            BitFlags::from(CursorMode::Metadata),
//            SourceType::Monitor | SourceType::Window,
//            true,
//        )
//        .await?;
//
//    let streams = proxy.start(&session, &WindowIdentifier::default()).await?;
//
//    streams.iter().for_each(|stream| {
//        println!("node id: {}", stream.pipe_wire_node_id());
//        println!("size: {:?}", stream.size());
//        println!("position: {:?}", stream.position());
//    });
//    Ok(())
//}
//async fn run4() -> ashpd::Result<()> {
//    let connection = zbus::Connection::session().await?;
//    let proxy = ScreenshotProxy::new(&connection).await?;
//
//    let uri = proxy.screenshot(&WindowIdentifier::default(), false, false).await?;
//    println!("URI: {}", uri);
//    Ok(())
//}
//use ashpd::desktop::screenshot;
//#[tokio::main]
//async fn main() -> ashpd::Result<()> {
//    run().await?;
//    run2().await?;
//    run3().await?;
//    run4().await?;
//    let uri = screenshot::take(&WindowIdentifier::default(), false, false).await?;
//    println!("URI: {}", uri);
//    Ok(())
//}
