use day23ui::*;

fn main() {
    console_error_panic_hook::set_once();

    let model_props = rsui::ModelProps::new(rs::INPUT.to_string());

    yew::Renderer::<rsui::Model<Part1, Part2>>::with_props(model_props).render();
}
