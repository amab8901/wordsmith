use gtk4::prelude::*;
use gtk4::{Application, ApplicationWindow, Button, Frame};
const APP_ID: &str = "wordsmith";

fn main() {
    // Create a new application
    let app = Application::builder().application_id(APP_ID).build();

    // Connect to "activate" signal of `app`
    app.connect_activate(build_ui);

    // Run the application
    app.run();
}

fn build_ui(app: &Application) {
    // Create a button with label and margins
    let button = Button::builder()
        .label("Press me!")
        .margin_top(120)
        .margin_bottom(120)
        .margin_start(120)
        .margin_end(120)
        .build();
        
    // Create a frame in which to show/edit text.
    let frame = Frame::builder()
        .label("Frame lol")
        .margin_top(70)
        .margin_bottom(7)
        .margin_start(7)
        .margin_end(7)
        .height_request(500)
        .width_request(500)
        .focusable(true)
        .focus_on_click(true)
        .build();
        

    
    // Connect to "clicked" signal of `button`
    button.connect_clicked(move |button| {
        // Set the label to "Hello World!" after the button has been clicked on
        button.set_label("Hello World!");
    });

    // Create a window
    let window = ApplicationWindow::builder()
        .application(app)
        .title("My GTK App")
        .child(&button)
        .child(&frame)
        .build();

    // Present window
    window.present();
}