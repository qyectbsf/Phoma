use std::error::Error;

use std::thread;
//use std::time;

slint::include_modules!();

fn main() -> Result<(), Box<dyn Error>> {
    let ui = AppWindow::new()?;

    ui.on_request_increase_value({
        let ui_handle = ui.as_weak();
        move || {
            let ui = ui_handle.unwrap();
            ui.set_counter(ui.get_counter() + 1);
            ui.hide().unwrap();

        }
    });

    ui.run()?;

    Ok(())
}
