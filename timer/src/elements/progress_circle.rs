use gpui::{Canvas, Half, PathBuilder, canvas, point, prelude::*, px, rgb};
use std::f32::consts::*;

pub fn progress_circle_element(mut progress: f32) -> Canvas<()> {
    if progress <= 0. {
        progress = 1.
    }
    canvas(
        // FnOnce(Bounds<Pixels>, &mut Window, &mut App)
        |_, _, _| {},
        // FnOnce(Bounds<Pixels>, T, &mut Window, &mut App)
        move |bounds, _, window, _app| {
            let center = bounds.center();
            let radius = bounds.size.width.half().half();
            let line_width = px(5.);
            let angle = -FRAC_PI_2 + 2.0 * PI * progress;
            let progress_point = point(
                center.x + radius * angle.cos(),
                center.y + radius * angle.sin(),
            );
            let start_point = point(center.x, center.y - radius);

            // 進捗度合いを表す濃い線

            // 残りを表す薄い線
            let mut builder = PathBuilder::stroke(line_width);
            builder.move_to(start_point);
            if progress >= 1.0 {
                builder.arc_to(
                    point(radius, radius),
                    px(0.),
                    false,
                    true,
                    point(center.x, center.y + radius),
                );
                builder.arc_to(
                    point(radius, radius),
                    px(0.),
                    false,
                    true,
                    point(center.x, center.y - radius),
                );
                builder.close();
            } else if progress > 0.0 {
                builder.arc_to(
                    point(radius, radius),
                    px(0.),
                    (progress > 0.5),
                    true,
                    progress_point,
                );
            }
            if let Ok(path) = builder.build() {
                window.paint_path(path, rgb(0xb5c4b5));
            }
        },
    )
    .size_full()
}
