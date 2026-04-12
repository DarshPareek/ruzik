use raylib::prelude::*;

// #[no_mangle] is required so the hot-reloader can find this exact function
#[unsafe(no_mangle)]
pub fn get_box_color() -> Color {
    Color::PINK
}
