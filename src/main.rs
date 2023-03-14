mod modules;

use modules::clipboard;
use modules::window;

unsafe fn display_clipboard_image() -> Result<(), anyhow::Error> {
    let image = clipboard::get_image()?;

    Ok(window::open_rgba(image)?)
}

fn main() {
    unsafe {
        match display_clipboard_image() {
            Err(e) => {
                println!("{e}");
            }
            Ok(_) => return,
        }
    };
}
