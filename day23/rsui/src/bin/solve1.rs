use day23ui::Part1;

use gloo_worker::Registrable;

fn main() {
    console_error_panic_hook::set_once();

    rsui::Solve::<Part1>::registrar().register();
}
