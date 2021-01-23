mod app;
mod grid;
mod location;

use app::App;

fn main() {
    let mut app = App::new();
    app.run();
}
