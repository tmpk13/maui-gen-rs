
mod tui;
use tui::*;

mod project;
use project::*;




// PROJECT_NAME

// view_dir_path
// view_model_dir_path

// NEW_VIEW
// NEW_VIEW_MODEL

fn main() -> std::io::Result<()> {
    let project = ProjectBuilder::new("Proj").view_dir_path("Proj2").build();
    let project2 = ProjectBuilder::new("Proj").new_view("Search").build();

    println!();
    println!("\x1B[0;33m{:#?}\x1B[0m", project);
    println!();
    println!("\x1B[0;33m{:#?}\x1B[0m", project2);

    ratatui::run(|terminal| App::default().run(terminal))
}
