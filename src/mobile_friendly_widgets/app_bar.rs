// Copyright 2020 Thomas Markiewicz.

//! An app bar widget loosely based on Flutter's AppBar
//! https://api.flutter.dev/flutter/material/AppBar-class.html

use druid::kurbo::BezPath;
use druid::piet::{FontBuilder, ImageFormat, InterpolationMode, Text, TextLayoutBuilder};
use druid::widget::prelude::*;
use druid::{Affine, AppLauncher, Color, Data, LocalizedString, Point, Rect, Size, WindowDesc};

pub struct AppBar {
    title: String,
    height: f64,
}

impl AppBar {
    pub fn new(title: String) -> AppBar {
        AppBar { title, height: 54. }
    }
}

impl<T: Data> Widget<T> for AppBar {
    fn event(&mut self, _ctx: &mut EventCtx, _event: &Event, _data: &mut T, _env: &Env) {}

    fn lifecycle(&mut self, _ctx: &mut LifeCycleCtx, _event: &LifeCycle, _data: &T, _env: &Env) {}

    fn update(&mut self, _ctx: &mut UpdateCtx, _old_data: &T, _data: &T, _env: &Env) {}

    fn layout(
        &mut self,
        _layout_ctx: &mut LayoutCtx,
        bc: &BoxConstraints,
        _data: &T,
        _env: &Env,
    ) -> Size {
        // BoxConstraints are passed by the parent widget.
        // This method can return any Size within those constraints:
        // bc.constrain(my_size)
        //
        // To check if a dimension is infinite or not (e.g. scrolling):
        // bc.is_width_bounded() / bc.is_height_bounded()

        // return max width and fixed height
        Size::new(bc.max().width, self.height)
    }

    // The paint method gets called last, after an event flow.
    // It goes event -> update -> layout -> paint, and each method can influence the next.
    // Basically, anything that changes the appearance of a widget causes a paint.
    fn paint(&mut self, ctx: &mut PaintCtx, _data: &T, _env: &Env) {
        // Let's draw a picture with Piet!

        // Clear the whole widget with the color of your choice
        // (ctx.size() returns the size of the layout rect we're painting in)
        let size = ctx.size();
        let rect = Rect::from_origin_size(Point::ORIGIN, size);
        ctx.fill(rect, &Color::WHITE);

        // Note: ctx also has a `clear` method, but that clears the whole context,
        // and we only want to clear this widget's area.

        // Create an arbitrary bezier path
        /*
        let mut path = BezPath::new();
        path.move_to(Point::ORIGIN);
        path.quad_to((80.0, 90.0), (size.width, size.height));
        // Create a color
        let stroke_color = Color::rgb8(0, 128, 0);
        // Stroke the path with thickness 1.0
        ctx.stroke(path, &stroke_color, 1.0);
        */

        // Rectangles: the path for practical people
        let rect = Rect::from_origin_size(Point::ORIGIN, (size.width, self.height));
        // Note the Color:rgba8 which includes an alpha channel (7F in this case)
        let fill_color = Color::rgba8(0xff, 0x52, 0x52, 0xFF);
        ctx.fill(rect, &fill_color);

        // Text is easy, if you ignore all these unwraps. Just pick a font and a size.
        let font = ctx
            .text()
            .new_font_by_name("Segoe UI", 24.0)
            .build()
            .unwrap();
        // Here's where we actually use the UI state
        let layout = ctx
            .text()
            .new_text_layout(&font, &self.title, std::f64::INFINITY)
            .build()
            .unwrap();

        let title_color = Color::rgb8(0xff, 0xff, 0xff);
        ctx.draw_text(&layout, (16.0, 35.0), &title_color);

        /*
        // Let's rotate our text slightly. First we save our current (default) context:
        ctx.with_save(|ctx| {
            // Now we can rotate the context (or set a clip path, for instance):
            ctx.transform(Affine::rotate(0.1));
            ctx.draw_text(&layout, (80.0, 40.0), &fill_color);
        });
        // When we exit with_save, the original context's rotation is restored
        */

        /*
        // Let's burn some CPU to make a (partially transparent) image buffer
        let image_data = make_image_data(256, 256);
        let image = ctx
            .make_image(256, 256, &image_data, ImageFormat::RgbaSeparate)
            .unwrap();
        // The image is automatically scaled to fit the rect you pass to draw_image
        ctx.draw_image(
            &image,
            Rect::from_origin_size(Point::ORIGIN, size),
            InterpolationMode::Bilinear,
        );
        */
    }
}

fn make_image_data(width: usize, height: usize) -> Vec<u8> {
    let mut result = vec![0; width * height * 4];
    for y in 0..height {
        for x in 0..width {
            let ix = (y * width + x) * 4;
            result[ix] = x as u8;
            result[ix + 1] = y as u8;
            result[ix + 2] = !(x as u8);
            result[ix + 3] = 127;
        }
    }
    result
}
