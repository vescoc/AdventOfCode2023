use day25ui::Part2;

use gloo_worker::Registrable;

fn main() {
    console_error_panic_hook::set_once();

    rsui::Solve::<Part2>::registrar().register();
}
