use bevy::app::Plugin;
use bevy::app::PostUpdate;
use bevy::log::info;
use bevy::math::vec2;
use bevy::math::Vec2;
use bevy::prelude::Bundle;
use bevy::prelude::Component;
use bevy::prelude::DetectChanges;
use bevy::prelude::IntoSystemConfigs;
use bevy::prelude::Query;
use bevy::prelude::Res;
use bevy::render::view::VisibilitySystems;
use bevy::text::update_text2d_layout;
use bevy::text::TextLayoutInfo;
use bevy::time::Time;
use std::f32::consts::PI;

#[derive(Bundle, Default)]
pub struct TextAnimationBundle {
    pub text_animation: TextAnimation,
    pub initial_positions_cache: InitialGyphPositionsCache,
}

impl<I> From<I> for TextAnimationBundle
where
    I: Into<TextAnimation>,
{
    fn from(text_animation: I) -> Self {
        Self {
            text_animation: text_animation.into(),
            ..Default::default()
        }
    }
}

pub struct AnimatedTextPlugin;

impl Plugin for AnimatedTextPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(
            PostUpdate,
            (
                #[cfg(not(feature = "bevy_ui"))]
                animate_text_system
                    .after(update_text2d_layout)
                    .before(VisibilitySystems::CalculateBounds),
                #[cfg(feature = "bevy_ui")]
                animate_text_system
                    .after(update_text2d_layout)
                    .after(bevy::ui::widget::text_system)
                    .before(VisibilitySystems::CalculateBounds),
            ),
        );
    }
}

#[derive(Component, Default)]
pub struct InitialGyphPositionsCache(Vec<Vec2>);

pub trait TextAnimator: 'static + Send + Sync {
    fn animate_glyph(
        &self,
        glyph_index: usize,
        len: usize,
        initial_position: Vec2,
        elapsed_seconds: f32,
    ) -> Vec2;
}

impl<F> TextAnimator for F
where
    F: Fn(usize, usize, Vec2, f32) -> Vec2 + 'static + Send + Sync,
{
    fn animate_glyph(
        &self,
        glyph_index: usize,
        len: usize,
        initial_position: Vec2,
        elapsed_seconds: f32,
    ) -> Vec2 {
        self(glyph_index, len, initial_position, elapsed_seconds)
    }
}

#[derive(Component)]
pub struct TextAnimation {
    pub animator: Box<dyn TextAnimator>,
}

impl Default for TextAnimation {
    fn default() -> Self {
        Self {
            animator: Box::new(|_, _, _, _| Vec2::ZERO),
        }
    }
}

impl TextAnimation {
    pub fn new(animator: impl TextAnimator) -> Self {
        Self {
            animator: Box::new(animator),
        }
    }

    pub fn sine_wave(amplitude: f32, frequency: f32, phase_shift: f32, stagger: f32) -> Self {
        Self::new(move |i: usize, _, p: Vec2, t: f32| {
            vec2(
                p.x,
                p.y + amplitude
                    * (2.0 * PI * frequency * t + phase_shift + i as f32 * stagger).sin(),
            )
        })
    }

    pub fn box_wave(amplitude: f32, frequency: f32, phase_shift: f32, stagger: f32) -> Self {
        Self::new(move |i: usize, _, p: Vec2, t: f32| {
            vec2(
                p.x,
                p.y + amplitude
                    * (2.0 * PI * frequency * t + phase_shift + i as f32 * stagger)
                        .sin()
                        .signum(),
            )
        })
    }

    pub fn bump(displacement: Vec2, start: usize, m: usize, frequency: f32) -> Self {
        Self::new(move |i: usize, len: usize, p: Vec2, t: f32| {
            let b = ((t * frequency) as usize + start) % len;
            if i % m == b % m {
                p + displacement
            } else {
                p
            }
        })
    }
}

impl<A> From<A> for TextAnimation
where
    A: 'static + Send + Sync + Fn(usize, usize, Vec2, f32) -> Vec2,
{
    fn from(animator: A) -> Self {
        TextAnimation::new(animator)
    }
}

pub fn animate_text_system(
    time: Res<Time>,
    mut animated_text_query: Query<(
        &mut TextLayoutInfo,
        &mut InitialGyphPositionsCache,
        &TextAnimation,
    )>,
) {
    let elapsed_seconds = time.elapsed_seconds();
    for (mut layout, mut initial_positions, animation) in animated_text_query.iter_mut() {
        if layout.is_changed() {
            info!("update initial positions");
            initial_positions.0.clear();
            initial_positions
                .0
                .extend(layout.glyphs.iter().map(|glyph| glyph.position));
        }

        let len = layout.glyphs.len();
        for (index, glyph) in layout.glyphs.iter_mut().enumerate() {
            glyph.position = animation.animator.animate_glyph(
                index,
                len,
                initial_positions.0[index],
                elapsed_seconds,
            );
        }
    }
}
