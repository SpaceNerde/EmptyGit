use anathema::prelude::*;
use anathema::component::Component;
use anathema::component::Value;
use anathema::component::List;
use anathema::component::State;

use git2::Repository;
use git2::BranchType;

#[derive(State)]
struct EmptyUiState {
    branches: Value<List<String>>,
}

struct EmptyUi {}

impl Component for EmptyUi {
    type State = EmptyUiState;
    type Message = ();
}


fn main() {

    let repo = match Repository::open(".") {
        Ok(repo) => repo,
        Err(e) => panic!("failed to open: {}", e),
    };
    
    let mut empty_ui_state = EmptyUiState {
        branches: List::empty(),
    };

    let branches = repo.branches(Some(BranchType::Remote)).unwrap();


    for branch in branches {
        let (branch, _branch_type) = branch.unwrap();

        if let Some(name) = branch.name().unwrap() {
            println!("{}", name.to_string());
            empty_ui_state.branches.push(name.to_string());
        }
    }
    
    let mut backend = TuiBackend::builder()
        .hide_cursor()
        .finish()
        .unwrap();

    let mut runtime = Runtime::builder(Document::new("@main"), backend);

    runtime
        .register_component(
            "main",
            "templates/main.aml",
            EmptyUi{},
            empty_ui_state
        )
        .unwrap();

    runtime.finish().unwrap().run(); 
}
