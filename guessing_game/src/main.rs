use gpui::*;
use views::game::GameView;

mod elements;
mod models;
mod views;

fn main() {
    Application::new().run(|app_cx: &mut App| {
        app_cx
            .open_window(
                // crate::WindowOptions
                WindowOptions {
                    window_bounds: Some(WindowBounds::Windowed(Bounds::new(
                        point(px(400.0), px(300.0)),
                        size(px(800.0), px(600.0)),
                    ))),
                    ..Default::default()
                },
                // impl FnOnce(&mut Window, &mut App) -> Entity<V>
                |_window: &mut Window, app_cx_2: &mut App| {
                    // fn new<T>(impl FnOnce(&mut Context<T>) -> T) -> Entity<T>
                    app_cx_2.new(
                        // impl FnOnce(&mut Context<T>) -> T
                        |view_cx| {
                            // T
                            GameView::new(view_cx)
                        },
                    )
                },
            )
            .unwrap();
    });
}
