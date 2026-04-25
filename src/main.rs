fn main() {
    wasm_logger::init(wasm_logger::Config::new(log::Level::Warn));
    yew::Renderer::<messaging_app::ui::app::App>::new().render();
}
