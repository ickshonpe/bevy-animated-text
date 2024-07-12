use bevy::color::palettes;
use bevy::prelude::*;
use bevy::text::Text2dBounds;
use bevy_animated_text::*;

fn get_animation(index: usize) -> TextAnimation {
    match index {
        0 => TextAnimation::sine_wave(5., 1., 0., 0.5),
        1 => TextAnimation::box_wave(5., 0.5, 0., 0.25),
        2 => TextAnimation::bump(-5. * Vec2::Y, 0, 10, 10.),
        _ => TextAnimation::new(|_, _, mut p: Vec2, t: f32| {
            let u = 0.4 * (p.y * t.sin() + p.x * (1. + t.sin()));
            p.x += u;
            p.y += 0.1 * u;
            p
        }),
    }
}

fn switch_animation(
    mut index: Local<usize>,
    keys: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut TextAnimation>,
) {
    if keys.just_pressed(KeyCode::Space) {
        *index = (*index + 1) % 4;
        for mut animation in query.iter_mut() {
            *animation = get_animation(*index);
        }
    }
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn((
        Text2dBundle {
            text: Text::from_section(
                "Press space to switch animation style",
                TextStyle {
                    color: palettes::css::GREEN.into(),
                    ..Default::default()
                },
            ),
            transform: Transform::from_xyz(0., 300., 0.),
            ..default()
        },
        TextAnimationBundle::from(get_animation(0)),
    ));
    commands.spawn((
        Text2dBundle {
            text: Text::from_section("Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.", TextStyle::default()),
            text_2d_bounds: Text2dBounds {
                size: Vec2::new(300., f32::INFINITY),
            },
            ..default()
        },
        TextAnimationBundle::from(get_animation(0)),
    ));
}

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, AnimatedTextPlugin))
        .add_systems(Startup, setup)
        .add_systems(Update, switch_animation)
        .run();
}
