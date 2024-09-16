use pandora::app::App;

fn main() {
    let mut app = App::default();

    app.create_window("Simple Window 1", 800, 600)
        .expect("Failed to create window 1.");
    app.create_window("Simple Window 2", 800, 600)
        .expect("Failed to create window 2.");

    app.run().expect("Failed to run app.");
}
