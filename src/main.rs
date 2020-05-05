use druid::widget::prelude::*;
use druid::widget::{Align, Flex, Label, Padding};
use druid::{AppLauncher, Data, Lens, PlatformError, Widget, WidgetExt, WindowDesc};
use log::{info};

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

fn build_ui() -> impl Widget<Reps> {
    Padding::new(
        8.0,
        Flex::column()
            .with_child(
                // TODO: see calc example `op_button_label` for an example of a label with a custom background (round for example)
                // TODO: see invalidations example `CircleView` for drawing a circle background/button
                Label::new(|reps: &Reps, _env: &_| String::from(format!("{}", reps.reps.unwrap_or(reps.target_reps))))
                .with_text_size(48.0)
                .center()
                .on_click( |_ctx, reps, _env| { reps.toggle();}),
                /*
                Label::new(|reps: &Option<u8>, _env: &_| String::from(format!("{}", reps.unwrap_or(0))))
                    .with_text_size(48.0)
                    .center()
                    .lens(Reps::reps)
                    .on_click( |_ctx, reps, _env| { reps.toggle();}),
                */
            )
    )
}

fn main() -> Result<(), PlatformError> {

    let reps = Reps {
        target_reps: 8,
        reps: None,
    };

    let main_window = WindowDesc::new(build_ui).show_titlebar(false);

    AppLauncher::with_window(main_window)
        .use_simple_logger()
        .launch(reps)?;

    Ok(())
}
