use druid::kurbo::{Circle, Point, Size};
use druid::widget::prelude::*;
use druid::widget::{Align, Flex, Label, Padding, Painter, SizedBox};
use druid::{theme, AppLauncher, Color, Data, Lens, PlatformError, Widget, WidgetExt, WindowDesc};
use log::info;

#[derive(Clone, Data, Lens)]
struct Reps {
    target_reps: u8,
    reps: Option<u8>,
}

impl Reps {
    fn toggle(&mut self) {
        self.reps = match self.reps {
            None => Some(self.target_reps),
            Some(current) => {
                if current > 0 {
                    Some(current - 1)
                } else {
                    None
                }
            }
        };

        info!("reps: {:?}", self.reps);
    }
}

/*
fn build_ui() -> impl Widget<()> {
    Padding::new(
        8.0,
        Flex::row()
            .with_flex_child(
                Flex::column()
                    .with_child(Align::left(Label::new("Top Left")))
                    .with_flex_spacer(1.0)
                    .with_child(Align::left(Label::new("Bottom Left"))),
                1.0,
            )
            .with_flex_child(
                Flex::column()
                    .with_child(Align::right(Label::new("Top Right")))
                    .with_flex_spacer(1.0)
                    .with_child(Align::right(Label::new("Bottom Right"))),
                1.0,
            ),
    )
}
*/

fn rep_button() -> impl Widget<Reps> {
    let painter = Painter::new(|ctx, _, env| {
        let bounds = ctx.size().to_rect();

        //ctx.fill(bounds, &Color::rgb8(0xff, 0x52, 0x52));

        let radius = (bounds.x1 - bounds.x0) / 2.0;
        let center = Point::from((bounds.x0 + radius, bounds.y0 + radius));

        ctx.fill(Circle::new(center, radius), &Color::rgb8(0xff, 0x52, 0x52));

        if ctx.is_hot() {
            ctx.stroke(Circle::new(center, radius - 1.0), &Color::WHITE, 1.0);
        }

        if ctx.is_active() {
            ctx.fill(Circle::new(center, radius), &env.get(theme::PRIMARY_LIGHT));
        }
    });

    let label = Label::new(|reps: &Reps, _env: &_| {
        String::from(format!("{}", reps.reps.unwrap_or(reps.target_reps)))
    })
    .with_text_size(36.0)
    .center()
    .on_click(|_ctx, reps, _env| {
        reps.toggle();
    });

    let sized_box = SizedBox::new(label)
        .width(64.)
        .height(64.)
        .background(painter);

    Padding::new(8.0, sized_box)
}

fn build_ui() -> impl Widget<Reps> {
    Padding::new(8.0, 
        Flex::row().must_fill_main_axis(true)
            .with_flex_child(rep_button(), 1.0)
            .with_flex_child(rep_button(), 1.0)
            .with_flex_child(rep_button(), 1.0)
            .with_flex_child(rep_button(), 1.0)
            .with_flex_child(rep_button(), 1.0)
    )
}

fn main() -> Result<(), PlatformError> {
    let reps = Reps {
        target_reps: 8,
        reps: None,
    };

    let main_window = WindowDesc::new(build_ui)
        .show_titlebar(false)
        .window_size(Size::new(720.0, 1320.0));

    AppLauncher::with_window(main_window)
        .use_simple_logger()
        .launch(reps)?;

    Ok(())
}
