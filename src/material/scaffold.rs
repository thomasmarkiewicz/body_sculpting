// Copyright 2020 Thomas Markiewicz.

//! An Scaffold widget inspired by Flutter's Scaffold

use druid::kurbo::BezPath;
use druid::piet::{FontBuilder, ImageFormat, InterpolationMode, Text, TextLayoutBuilder};
use druid::widget::prelude::*;
use druid::widget::Container;
use druid::{
    Affine, AppLauncher, Color, Data, LocalizedString, Point, Rect, Size, WidgetPod, WindowDesc,
};
use log::info;

use super::app_bar::AppBar;

pub struct Scaffold<T> {
    app_bar: WidgetPod<T, Box<dyn Widget<T>>>,
    body: WidgetPod<T, Box<dyn Widget<T>>>,
    fab: WidgetPod<T, Box<dyn Widget<T>>>,
}

impl<T> Scaffold<T> {
    pub fn new(
        app_bar: impl Widget<T> + 'static,
        body: impl Widget<T> + 'static,
        fab: impl Widget<T> + 'static,
    ) -> Self {
        Scaffold {
            app_bar: WidgetPod::new(app_bar).boxed(),
            body: WidgetPod::new(body).boxed(),
            fab: WidgetPod::new(fab).boxed(),
        }
    }
}

impl<T: Data> Widget<T> for Scaffold<T> {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut T, env: &Env) {
        self.app_bar.event(ctx, event, data, env);
        self.body.event(ctx, event, data, env);
        self.fab.event(ctx, event, data, env);
    }

    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &T, env: &Env) {
        self.app_bar.lifecycle(ctx, event, data, env);
        self.body.lifecycle(ctx, event, data, env);
        self.fab.lifecycle(ctx, event, data, env);
    }

    fn update(&mut self, ctx: &mut UpdateCtx, _old_data: &T, data: &T, env: &Env) {
        self.app_bar.update(ctx, data, env);
        self.body.update(ctx, data, env);
        self.fab.update(ctx, data, env);
    }

    fn layout(
        &mut self,
        layout_ctx: &mut LayoutCtx,
        bc: &BoxConstraints,
        data: &T,
        env: &Env,
    ) -> Size {
        bc.debug_check("Scaffold");
        // loosen constraints when passing to children. (TODO: why? what does that do?)
        let loosened_bc = bc; // bc.loosen();

        // layout appBar
        let app_bar_size = self.app_bar.layout(layout_ctx, &loosened_bc, data, env);
        let rect = Rect::from_origin_size(Point::ORIGIN, app_bar_size);
        self.app_bar.set_layout_rect(layout_ctx, data, env, rect);

        // layout body
        let loosened_bc = loosened_bc.shrink((0., app_bar_size.height));
        let body_size = self.body.layout(layout_ctx, &loosened_bc, data, env);
        let rect = Rect::from_origin_size((0., app_bar_size.height), body_size);
        self.body.set_layout_rect(layout_ctx, data, env, rect);

        // layout fab
        const FAB_MARGIN: f64 = 8.;
        let fab_size = self.fab.layout(layout_ctx, &loosened_bc, data, env);
        let rect = Rect::from_origin_size(
            (
                bc.max().width - fab_size.width - FAB_MARGIN,
                bc.max().height - fab_size.height - FAB_MARGIN,
            ),
            fab_size,
        );
        self.fab.set_layout_rect(layout_ctx, data, env, rect);

        // BoxConstraints are passed by the parent widget.
        // This method can return any Size within those constraints:
        // bc.constrain(my_size)
        //
        // To check if a dimension is infinite or not (e.g. scrolling):
        // bc.is_width_bounded() / bc.is_height_bounded()
        bc.max()
    }

    // The paint method gets called last, after an event flow.
    // It goes event -> update -> layout -> paint, and each method can influence the next.
    // Basically, anything that changes the appearance of a widget causes a paint.
    fn paint(&mut self, ctx: &mut PaintCtx, data: &T, env: &Env) {
        self.app_bar.paint(ctx, data, env);
        self.body.paint(ctx, data, env);
        self.fab.paint(ctx, data, env);
    }
}
