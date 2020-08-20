pub mod mobile_friendly_widgets;

use druid::kurbo::{Circle, Point, Size};
use druid::widget::prelude::*;
use druid::widget::{Align, Flex, Label, Padding, Painter, SizedBox};
use druid::{
    theme, AppLauncher, Color, Data, Lens, LensWrap, PlatformError, Widget, WidgetExt, WindowDesc,
};
use log::info;

use mobile_friendly_widgets::{AppBar, FloatingActionButton, Scaffold};

#[derive(Clone, Data, Lens)]
struct State {
    title: String,
}

impl State {
    fn set_title(&mut self, title: String) {
        self.title = title;
        info!("title: {:?}", self.title);
    }
}

fn header() -> impl Widget<State> {
    let painter = Painter::new(|ctx, _, env| {
        let bounds = ctx.size().to_rect();
        ctx.fill(bounds, &Color::rgb8(0xff, 0x52, 0x52));
        /*
                let radius = (bounds.x1 - bounds.x0) / 2.0;
                let center = Point::from((bounds.x0 + radius, bounds.y0 + radius));

                ctx.fill(Circle::new(center, radius), &Color::rgb8(0xff, 0x52, 0x52));

                if ctx.is_hot() {
                    ctx.stroke(Circle::new(center, radius - 1.0), &Color::WHITE, 1.0);
                }

                if ctx.is_active() {
                    ctx.fill(Circle::new(center, radius), &env.get(theme::PRIMARY_LIGHT));
                }
        */
    });

    let label = Label::new(|state: &State, _env: &_| String::from(format!("{}", state.title)))
        .with_text_size(36.0)
        .center();

    let sized_box = SizedBox::new(label)
        .width(320.) // TODO: how do I make it full window width?
        .height(64.)
        .background(painter);

    sized_box

    //Padding::new(0.0, sized_box)
}

fn build_ui() -> impl Widget<State> {
    //let appBar = LensWrap::new(AppBar {}, lenses: state::title);
    //Padding::new(0.0, Flex::column().with_child(AppBar {}))

    // AppBar
    let app_bar = AppBar::new(String::from("Hello World"));

    // Body
    let label = Label::new("How are you?").with_text_size(18.0).center();
    let body = Padding::new(8.0, label);

    // FloatingActionButton
    let fab = FloatingActionButton::new("+");

    // Scaffold
    let scaffold = Scaffold::new(app_bar, body, fab);
    scaffold
}

fn main() -> Result<(), PlatformError> {
    let state = State {
        title: "Workouts".to_string(),
    };

    let main_window = WindowDesc::new(build_ui)
        .show_titlebar(false)
        .window_size(Size::new(320.0, 480.0));

    AppLauncher::with_window(main_window)
        .use_simple_logger()
        .launch(state)?;

    Ok(())
}
