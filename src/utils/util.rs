use std::{path::Path, num::NonZeroU32};

use softbuffer::Surface;
use tiny_skia::{Transform, FillRule, PathBuilder, Rect, Paint, Pixmap, Color, Stroke};
use winit::window::Icon;

pub fn load_icon(path: &Path) -> Icon {
    let (icon_rgba, icon_width, icon_height) = {
        let image = image::open(path)
            .expect("Failed to open icon path")
            .into_rgba8();
        let (width, height) = image.dimensions();
        let rgba = image.into_raw();
        (rgba, width, height)
    };
    Icon::from_rgba(icon_rgba, icon_width, icon_height).expect("Failed to open icon")
}

pub fn draw_surface(window : &winit::window::Window, surface : &mut Surface, ) -> Pixmap{
    let (width, height) = {
        let size = window.inner_size();
        (size.width, size.height)
    };

    surface
        .resize(
            NonZeroU32::new(width).unwrap(),
            NonZeroU32::new(height).unwrap(),
        )
        .unwrap();

    let mut stroke = Stroke::default();
    stroke.width = 10.0;
    let mut pixmap = Pixmap::new(width, height).unwrap();
    pixmap.fill(Color::WHITE);
    let path = PathBuilder::from_circle(
        (width / 2) as f32,
        (height / 2) as f32,
        (width.min(height) / 20) as f32,
    )
    .unwrap();
    let mut paint = Paint::default();
    paint.set_color_rgba8(0, 0, 0, 255);
    pixmap.fill_path(
        &path,
        &paint,
        FillRule::EvenOdd,
        Transform::identity(),
        None,
    );
    paint.set_color_rgba8(255, 255, 255, 255);



    paint.set_color_rgba8(255, 255, 255, 255);
    let mut l1 = PathBuilder::new();
    l1.move_to((width / 2) as f32 + (width / 8) as f32, (height / 2) as f32);
    l1.line_to(
        (width / 2) as f32 + 2. * (width / 8) as f32,
        (height / 2) as f32,
    );
    l1.line_to(
        (width / 2) as f32 + 2. * (width / 8) as f32 - (width / 16) as f32,
        (height / 2) as f32 - 20.,
    );
    l1.move_to(
        (width / 2) as f32 + 2. * (width / 8) as f32,
        (height / 2) as f32,
    );
    l1.line_to(
        (width / 2) as f32 + 2. * (width / 8) as f32 - (width / 16) as f32,
        (height / 2) as f32 + 20.,
    );

    let l1 = l1.finish().unwrap();
    pixmap.fill_path(&l1, &paint, FillRule::Winding, Transform::identity(), None);

    let mut l2 = PathBuilder::new();
    l2.move_to((width / 2) as f32 - (width / 8) as f32, (height / 2) as f32);
    l2.line_to(
        (width / 2) as f32 - 2. * (width / 8) as f32,
        (height / 2) as f32,
    );
    l2.line_to(
        (width / 2) as f32 - 2. * (width / 8) as f32 + (width / 16) as f32,
        (height / 2) as f32 - 20.,
    );
    l2.move_to(
        (width / 2) as f32 - 2. * (width / 8) as f32,
        (height / 2) as f32,
    );
    l2.line_to(
        (width / 2) as f32 - 2. * (width / 8) as f32 + (width / 16) as f32,
        (height / 2) as f32 + 20.,
    );
    let l2 = l2.finish().unwrap();
    pixmap.fill_path(&l2, &paint, FillRule::Winding, Transform::identity(), None);
    let rectangle = Rect::from_xywh(
        (width / 2) as f32 - (width / 16) as f32,
        (height / 2) as f32 - (width / 16) as f32,
        (width / 8) as f32,
        (width / 8) as f32,
    );
    let rectangle = PathBuilder::from_rect(rectangle.unwrap());
    pixmap.fill_path(
        &rectangle,
        &paint,
        FillRule::Winding,
        Transform::identity(),
        None,
    );

    paint.set_color_rgba8(0, 0, 0, 255);

    pixmap.stroke_path(&path, &paint, &stroke, Transform::identity(), None);
    pixmap.stroke_path(&l1, &paint, &stroke, Transform::identity(), None);
    pixmap.stroke_path(&l2, &paint, &stroke, Transform::identity(), None);
    pixmap.stroke_path(&rectangle, &paint, &stroke, Transform::identity(), None);

    pixmap
}