mod app;
mod grid;

use app::App;

fn main() {
    let mut app = App::new();
    app.run();
}
