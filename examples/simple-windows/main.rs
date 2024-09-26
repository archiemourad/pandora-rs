use pandora::app::App;
use winit::event_loop::ControlFlow;

fn main() {
    let mut app = App::new(ControlFlow::Poll).expect("Failed to create app.");

    app.create_window("Simple Window 1", 800, 600)
        .expect("Failed to create window 1.");
    app.create_window("Simple Window 2", 800, 600)
        .expect("Failed to create window 2.");

    app.run().expect("Failed to run app.");
}
