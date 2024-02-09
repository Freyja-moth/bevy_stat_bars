use bevy::prelude::*;
use bevy_health_bar::prelude::*;

#[derive(Component)]
pub struct Bar1;

#[derive(Component)]
pub struct Bar2;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, StatBarPlugin))
        .add_systems(Startup, spawn_bar)
        .add_systems(Update, (update_bar1, update_bar2))
        .run();
}

fn spawn_bar(mut commands: Commands) {
    commands.spawn(Camera3dBundle::default());

    commands.spawn(FRAMING()).with_children(|parent| {
        parent.spawn(BAR1_FRAMING()).with_children(|parent| {
            parent
                .spawn(BAR1_INSIDE())
                .insert(StatBar::new())
                .insert(Bar1);
        });
        parent.spawn(BAR2_FRAMING()).with_children(|parent| {
            parent
                .spawn(BAR2_INSIDE())
                .insert(StatBar::new().change_vertical())
                .insert(Bar2);
        });
    });
}

fn update_bar1(mut bar_query: Query<&mut StatBar, With<Bar1>>, input: Res<Input<KeyCode>>) {
    let mut bar = bar_query.single_mut();
    if input.just_pressed(KeyCode::Right) {
        bar.set_value(1.);
    } else if input.just_pressed(KeyCode::Left) {
        bar.set_value(0.);
    }
}

fn update_bar2(mut bar_query: Query<&mut StatBar, With<Bar2>>, input: Res<Input<KeyCode>>) {
    let mut bar = bar_query.single_mut();

    if input.just_pressed(KeyCode::Up) {
        bar.set_value(1.);
    } else if input.just_pressed(KeyCode::Down) {
        bar.set_value(0.);
    }
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
        height: Val::Percent(0.),
        width: Val::Percent(100.),
        ..Default::default()
    },
    background_color: INSIDE_COLOR,
    ..Default::default()
};
