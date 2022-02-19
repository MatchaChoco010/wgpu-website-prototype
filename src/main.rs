mod app;
mod egui_state;
mod pass;
mod utils;
use app::App;

fn main() {
    #[cfg(target_arch = "wasm32")]
    {
        wasm_logger::init(wasm_logger::Config::default());
        std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        std::env::set_var("RUST_LOG", "info");
        env_logger::init();
    }

    let app = App::new();
    app.run();
}
