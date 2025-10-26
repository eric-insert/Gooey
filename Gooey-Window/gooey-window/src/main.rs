use gooey_window::platform::agnostic::{events::{Event, EventDispatcher, WindowEvent}, window::{IWindow, Window, WindowConfig}};


fn main() {
    let mut dispatcher = EventDispatcher::new(None);
    let mut window_config = WindowConfig::default();
    window_config.title = "My First Gooey Window";

    let window = Window::create(&window_config);
    let windo = Window::create(&window_config);
    let window_id = window.id;
    dispatcher.register_window(window, None);
    dispatcher.register_window(windo, None);

    dispatcher.run(|event| match event {
        Event::WindowEvent { window_id: id, event } => {
            println!("Window {:?} -> {:?}", id, event);
            if id == window_id && matches!(event, WindowEvent::CloseRequested) {
                std::process::exit(0); // exit app cleanly
            }
        }
        _ => {
            println!("Event: {:?}", event);
        }
    });
}
