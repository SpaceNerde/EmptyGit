use anathema::prelude::*;
use anathema::component::Component;

struct EmptyUi;

impl Component for EmptyUi {
    type State = ();
    type Message = ();
}

fn main() {
   let mut backend = TuiBackend::builder()
        .enable_alt_screen()
        .enable_raw_mode()
        .hide_cursor()
        .finish()
        .unwrap();

    let mut runtime = Runtime::builder(Document::new("@main"), backend);

    runtime
        .register_component(
            "main",
            "templates/main.aml",
            EmptyUi{},
            ()
        )
        .unwrap();

    runtime.finish().unwrap().run(); 
}
