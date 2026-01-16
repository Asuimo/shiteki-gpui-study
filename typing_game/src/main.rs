use gpui::{App, AppContext, Application, Bounds, WindowBounds, WindowOptions, point, px, size};

mod elements;
mod models;
mod views;
use views::game::GameView;

fn main() {
    let app = Application::new();
    app.run(|app_cx: &mut App| {
        app_cx
            .open_window(
                WindowOptions {
                    window_bounds: Some(WindowBounds::Windowed(Bounds::new(
                        point(px(400.0), px(300.0)),
                        size(px(800.0), px(600.0)),
                    ))),
                    ..Default::default()
                },
                |_window, app_cx_2| app_cx_2.new(|view_cx| GameView::new(view_cx)),
            )
            .unwrap();
    });
}
