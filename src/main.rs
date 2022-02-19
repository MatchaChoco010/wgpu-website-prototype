mod app;
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

    #[cfg(target_arch = "wasm32")]
    wasm_bindgen_futures::spawn_local(app.run());

    #[cfg(not(target_arch = "wasm32"))]
    pollster::block_on(app.run());
}
