use druid::widget::{Align, Flex, Label, Padding};
use druid::{AppLauncher, PlatformError, Widget, WindowDesc};

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

fn main() -> Result<(), PlatformError> {
    AppLauncher::with_window(WindowDesc::new(build_ui).show_titlebar(false)).launch(())?;
    Ok(())
}
