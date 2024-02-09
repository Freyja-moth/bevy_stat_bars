use bevy::prelude::*;
use bevy_stat_bars::prelude::*;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, StatBarPlugin))
        .add_systems(Startup, spawn_bar)
        .add_systems(Update, update_bar)
        .run();
}

fn spawn_bar(mut commands: Commands) {
    commands.spawn(Camera3dBundle::default());

    commands.spawn(FRAMING()).with_children(|parent| {
        parent.spawn(BAR1_FRAMING()).with_children(|parent| {
            parent.spawn(BAR1_INSIDE()).insert(StatBar::new());
        });
        parent.spawn(BAR2_FRAMING()).with_children(|parent| {
            parent
                .spawn(BAR2_INSIDE())
                .insert(StatBar::new().change_direction(StatBarDirection::Vertical));
        });
    });
}

fn update_bar(mut bar_query: Query<&mut StatBar>, mut flip: Local<bool>, time: Res<Time>) {
    bar_query.iter_mut().for_each(|mut bar| {
        if *flip {
            bar.add_value(-time.delta_seconds());
            if bar.value() <= 0. {
                *flip = false;
            }
        } else {
            bar.add_value(time.delta_seconds());
            if bar.value() >= 1. {
                *flip = true;
            }
        }
    });
}

const FRAMING_COLOR: BackgroundColor =
    BackgroundColor(Color::rgb(18. / 255., 55. / 255., 42. / 255.));
const INSIDE_COLOR: BackgroundColor =
    BackgroundColor(Color::rgb(67. / 255., 104. / 255., 80. / 255.));

const FRAMING: fn() -> NodeBundle = || NodeBundle {
    style: Style {
        display: Display::Grid,
        padding: UiRect::all(Val::Percent(5.)),
        width: Val::Percent(100.),
        height: Val::Percent(100.),
        align_items: AlignItems::Center,
        justify_items: JustifyItems::Center,
        grid_template_columns: RepeatedGridTrack::flex(2, 1.0),
        ..Default::default()
    },
    ..Default::default()
};

const BAR1_FRAMING: fn() -> NodeBundle = || NodeBundle {
    style: Style {
        width: Val::Percent(45.),
        height: Val::Percent(10.),
        padding: UiRect::all(Val::Percent(0.75)),
        ..Default::default()
    },
    background_color: FRAMING_COLOR,
    ..Default::default()
};

const BAR2_FRAMING: fn() -> NodeBundle = || NodeBundle {
    style: Style {
        width: Val::Percent(10.),
        height: Val::Percent(45.),
        padding: UiRect::all(Val::Percent(0.75)),
        flex_direction: FlexDirection::ColumnReverse,
        ..Default::default()
    },
    background_color: FRAMING_COLOR,
    ..Default::default()
};

const BAR1_INSIDE: fn() -> NodeBundle = || NodeBundle {
    style: Style {
        height: Val::Percent(100.),
        width: Val::Percent(0.),
        ..Default::default()
    },
    background_color: INSIDE_COLOR,
    ..Default::default()
};

const BAR2_INSIDE: fn() -> NodeBundle = || NodeBundle {
    style: Style {
        height: Val::Percent(50.),
        width: Val::Percent(100.),
        ..Default::default()
    },
    background_color: INSIDE_COLOR,
    ..Default::default()
};
