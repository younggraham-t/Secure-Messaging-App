fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<messaging_app::ui::app::App>::new().render();
}
