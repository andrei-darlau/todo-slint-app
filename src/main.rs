use slint::VecModel;
use std::error::Error;
use std::rc::Rc;
slint::include_modules!();

fn main() -> Result<(), Box<dyn Error>> {
    let ui = AppWindow::new()?;

    let todo_model = Rc::new(VecModel::from(vec![]));
    ui.set_todo_model(todo_model.clone().into());

    let todo_model_adding = todo_model.clone();

    ui.on_add_item(move |s| {
        todo_model_adding.push(TodoItem {title: s, checked: false});
    });

    let todo_model_clearing = todo_model.clone();
    
    ui.on_clear_all(move || {
        todo_model_clearing.clear();
    });


    ui.run()?;

    Ok(())
}
