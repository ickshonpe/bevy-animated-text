use bevy::prelude::*;
use bevy_animated_text::*;

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn((
        Text2dBundle {
            text: Text::from_section(
                "Hello, World!",
                TextStyle {
                    font_size: 50.,
                    ..Default::default()
                },
            ),
            ..default()
        },
        TextAnimationBundle::from(TextAnimation::sine_wave(50., 0.25, 0., 1.)),
    ));
}

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, AnimatedTextPlugin))
        .add_systems(Startup, setup)
        .run();
}
