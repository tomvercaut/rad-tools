slint::include_modules!();

fn main() -> Result<(), slint::PlatformError>{
    let ui = AppWindow::new()?;
    // ui.on_request_increase_value({
    //    let handle = ui.as_weak();
    //     move || {
    //         let ui = handle.unwrap();
    //         ui.set_counter(ui.get_counter() + 1);
    //     }
    // });
    
    // ui.on_search_modified(move |s: String| {
    //     let handle = ui.as_weak();
    //     move || {
    //         let _ui = handle.unwrap();
    //         println!("Modified string: {s}");
    //     }
    // } );
    ui.on_search_modified({
        move |text| {
            let s = text.to_string();
            println!("Modified text: {s}");
        }
    });
    
    ui.run()
}
