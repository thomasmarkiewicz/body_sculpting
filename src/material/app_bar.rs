// Copyright 2020 Thomas Markiewicz.

//! An app bar widget loosely based on Flutter's AppBar
//! https://api.flutter.dev/flutter/material/AppBar-class.html

use druid::piet::{FontBuilder, Text, TextLayoutBuilder};
use druid::widget::prelude::*;
use druid::{Color, Data, Point, Rect, Size};

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
        // Fill
        let size = ctx.size();
        let rect = Rect::from_origin_size(Point::ORIGIN, size);
        let fill_color = Color::rgb8(0xff, 0x52, 0x52);
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
    }
}
