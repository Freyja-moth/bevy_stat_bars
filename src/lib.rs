pub mod prelude;
pub(crate) mod stat_bar;

use crate::prelude::*;

pub struct StatBarPlugin;

impl Plugin for StatBarPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (update_vertical_bar_position, update_horizontal_bar_position),
        );
    }
}

fn update_vertical_bar_position(mut stat_bars: Query<(&mut Style, &StatBar)>) {
    stat_bars
        .iter_mut()
        .filter(|(_, bar)| bar.is_active())
        .filter(|(_, bar)| bar.direction().is_vertical())
        .for_each(|(mut style, bar)| {
            let speed = bar.speed().clamp(0., 1.);
            let value = bar.value() * 100.;

            if let Val::Percent(percent) = style.height {
                if let Val::Percent(max_percent) = style.max_height {
                    let clamped_val = percent.lerp(value.clamp(0., max_percent), speed);

                    style.height = Val::Percent(clamped_val);
                } else if let Val::Auto = style.max_height {
                    style.max_height = Val::Percent(100.);
                }
            } else if let Val::Auto = style.height {
                style.height = Val::Percent(value);
            }
        });
}

fn update_horizontal_bar_position(mut stat_bars: Query<(&mut Style, &StatBar)>) {
    stat_bars
        .iter_mut()
        .filter(|(_, bar)| bar.is_active())
        .filter(|(_, bar)| bar.direction().is_horizontal())
        .for_each(|(mut style, bar)| {
            let speed = bar.speed().clamp(0., 1.);
            let value = bar.value() * 100.;

            if let Val::Percent(percent) = style.width {
                if let Val::Percent(max_percent) = style.max_width {
                    let clamped_val = percent.lerp(value.clamp(0., max_percent), speed);

                    style.width = Val::Percent(clamped_val);
                } else if let Val::Auto = style.max_width {
                    style.max_width = Val::Percent(100.);
                }
            } else if let Val::Auto = style.width {
                style.width = Val::Percent(value);
            }
        });
}
