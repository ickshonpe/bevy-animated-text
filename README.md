# bevy-animated-text

Basic animated text plugin for Bevy.

### Usage
Add the `AnimatedTextPlugin` to your app and spawn an entity with a `TextAnimationBundle` together with either a `TextBundle` or `Text2dBundle`:
```
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
        TextAnimationBundle::from(|i: usize, _, p: Vec2, t: f32| p + 50. * (t + i as f32).sin() * Vec2::Y),
    ));
}

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, AnimatedTextPlugin))
        .add_systems(Startup, setup)
        .run();
}
```

Unfortunately in order to work nicely with Bevy's existing text implementation it's limited to only animating glyph positions.

### Examples
* ```cargo run --example hello_world```
* ```cargo run --example text2d```
* ```cargo run --example ui```
The `text2d` example is the most complete.
