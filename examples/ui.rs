use bevy::prelude::*;
use bevy_animated_text::*;

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..Default::default()
            },
            ..Default::default()
        })
        .with_children(|commands| {
            commands.spawn((
                TextBundle {
                    text: Text::from_section(
                        "Hello, World!",
                        TextStyle {
                            font_size: 50.,
                            ..Default::default()
                        },
                    ),
                    ..default()
                },
                TextAnimationBundle {
                    text_animation: TextAnimation::sine_wave(50., 0.25, 0., 1.),
                    ..default()
                },
            ));
        });
}

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, AnimatedTextPlugin))
        .add_systems(Startup, setup)
        .run();
}
