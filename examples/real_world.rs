use bevy::prelude::*;
use bevy_stat_bars::prelude::*;

#[derive(Resource, Debug)]
pub struct Health(u8);

impl Default for Health {
    fn default() -> Self {
        Self(100)
    }
}

impl Health {
    fn add(&mut self, value: u8) {
        self.0 = self.0.checked_add(value).unwrap_or(100).min(100);
    }
    fn sub(&mut self, value: u8) {
        self.0 = self.0.checked_sub(value).unwrap_or_default();
    }

    fn raise(&mut self) {
        self.0 = 100
    }
    fn lower(&mut self) {
        self.0 = 0
    }

    fn value(&self) -> f32 {
        self.0 as f32 / 100.
    }
}

#[derive(Component)]
pub struct HealthBar;

#[derive(Resource, Debug)]
pub struct Stamina(u8);

impl Default for Stamina {
    fn default() -> Self {
        Self(100)
    }
}

impl Stamina {
    fn add(&mut self, value: u8) {
        self.0 = self.0.checked_add(value).unwrap_or(100).min(100);
    }
    fn sub(&mut self, value: u8) {
        self.0 = self.0.checked_sub(value).unwrap_or_default();
    }

    fn raise(&mut self) {
        self.0 = 100
    }
    fn lower(&mut self) {
        self.0 = 0
    }

    fn value(&self) -> f32 {
        self.0 as f32 / 100.
    }
}

#[derive(Component)]
pub struct StaminaBar;

#[derive(Resource, Debug)]
pub struct Mana(u8);

impl Default for Mana {
    fn default() -> Self {
        Self(100)
    }
}

impl Mana {
    fn add(&mut self, value: u8) {
        self.0 = self.0.checked_add(value).unwrap_or(100).min(100);
    }
    fn sub(&mut self, value: u8) {
        self.0 = self.0.checked_sub(value).unwrap_or_default();
    }

    fn raise(&mut self) {
        self.0 = 100
    }

    fn lower(&mut self) {
        self.0 = 0
    }

    fn value(&self) -> f32 {
        self.0 as f32 / 100.
    }
}

#[derive(Component)]
pub struct ManaBar;

fn main() {
    App::new()
        .init_resource::<Health>()
        .init_resource::<Mana>()
        .init_resource::<Stamina>()
        .add_plugins((DefaultPlugins, StatBarPlugin))
        .add_systems(Startup, spawn_bar)
        .add_systems(
            Update,
            (
                update_stats,
                update_health_bar.run_if(resource_changed::<Health>()),
                update_mana_bar.run_if(resource_changed::<Mana>()),
                update_stamina_bar.run_if(resource_changed::<Stamina>()),
            ),
        )
        .run();
}

fn spawn_bar(mut commands: Commands) {
    commands.spawn(Camera3dBundle::default());

    commands.spawn(FRAMING()).with_children(|parent| {
        parent
            .spawn(HEALTH_BAR())
            .insert(StatBar::new().change_value(1.))
            .insert(HealthBar);
        parent
            .spawn(MANA_BAR())
            .insert(StatBar::new().change_value(1.).change_speed(0.3))
            .insert(ManaBar);
        parent
            .spawn(STAMINA_BAR())
            .insert(StatBar::new().change_value(1.).change_speed(0.45))
            .insert(StaminaBar);
    });

    commands.spawn(CONTROLS());
}

fn update_stats(
    mut health: ResMut<Health>,
    mut mana: ResMut<Mana>,
    mut stamina: ResMut<Stamina>,
    input: Res<Input<KeyCode>>,
) {
    input.get_pressed().for_each(|&key| match key {
        KeyCode::Q => health.sub(1),
        KeyCode::W => health.add(1),
        KeyCode::A => mana.sub(1),
        KeyCode::S => mana.add(1),
        KeyCode::Z => stamina.sub(1),
        KeyCode::X => stamina.add(1),
        _ => {}
    });
    input.get_just_pressed().for_each(|&key| match key {
        KeyCode::E => health.lower(),
        KeyCode::R => health.raise(),
        KeyCode::D => mana.lower(),
        KeyCode::F => mana.raise(),
        KeyCode::C => stamina.lower(),
        KeyCode::V => stamina.raise(),
        _ => {}
    });
}

fn update_health_bar(mut bar_query: Query<&mut StatBar, With<HealthBar>>, health: Res<Health>) {
    let mut bar = bar_query.single_mut();

    bar.set_value(health.value());
}

fn update_mana_bar(mut bar_query: Query<&mut StatBar, With<ManaBar>>, mana: Res<Mana>) {
    let mut bar = bar_query.single_mut();

    bar.set_value(mana.value())
}

fn update_stamina_bar(mut bar_query: Query<&mut StatBar, With<StaminaBar>>, stamina: Res<Stamina>) {
    let mut bar = bar_query.single_mut();

    bar.set_value(stamina.value());
}

const HEALTH_COLOR: BackgroundColor =
    BackgroundColor(Color::rgb(175. / 255., 22. / 255., 56. / 255.));
const MANA_COLOR: BackgroundColor =
    BackgroundColor(Color::rgb(41. / 255., 32. / 255., 124. / 255.));
const STAMINA_COLOR: BackgroundColor =
    BackgroundColor(Color::rgb(12. / 255., 102. / 255., 3. / 255.));

const FRAMING: fn() -> NodeBundle = || NodeBundle {
    style: Style {
        display: Display::Grid,
        margin: UiRect::all(Val::Percent(2.5)),
        width: Val::Percent(25.),
        height: Val::Percent(7.5),
        align_items: AlignItems::Center,
        grid_template_rows: RepeatedGridTrack::flex(3, 1.0),
        row_gap: Val::Percent(25.),
        ..Default::default()
    },
    ..Default::default()
};

const HEALTH_BAR: fn() -> NodeBundle = || NodeBundle {
    style: Style {
        width: Val::Percent(100.),
        height: Val::Percent(100.),
        ..Default::default()
    },
    background_color: HEALTH_COLOR,
    ..Default::default()
};

const MANA_BAR: fn() -> NodeBundle = || NodeBundle {
    style: Style {
        max_width: Val::Percent(80.),
        height: Val::Percent(100.),
        ..Default::default()
    },
    background_color: MANA_COLOR,
    ..Default::default()
};

const STAMINA_BAR: fn() -> NodeBundle = || NodeBundle {
    style: Style {
        max_width: Val::Percent(75.),
        height: Val::Percent(100.),
        ..Default::default()
    },
    background_color: STAMINA_COLOR,
    ..Default::default()
};

const CONTROLS: fn() -> TextBundle = || TextBundle {
    text: Text {
        sections: vec![
            TextSection {
                value: "Health\nQ: Lower\nW: Raise\nE: Set to Zero\nR: Set to full\n".into(),
                style: TextStyle {
                    font_size: 32.,
                    color: Color::GRAY,
                    ..Default::default()
                },
            },
            TextSection {
                value: "Mana\nA: Lower\nS: Raise\nD: Set to Zero\nF: Set to full\n".into(),
                style: TextStyle {
                    font_size: 32.,
                    color: Color::GRAY,
                    ..Default::default()
                },
            },
            TextSection {
                value: "Stamina\nZ: Lower\nX: Raise\nC: Set to Zero\nV: Set to full\n".into(),
                style: TextStyle {
                    font_size: 32.,
                    color: Color::GRAY,
                    ..Default::default()
                },
            },
        ],
        ..Default::default()
    },
    style: Style {
        align_self: AlignSelf::End,
        justify_content: JustifyContent::SpaceAround,
        ..Default::default()
    },
    ..Default::default()
};
