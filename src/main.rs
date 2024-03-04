use bevy::prelude::*;

#[derive(Reflect, States, Clone, Copy, Eq, Hash, Debug, PartialEq, Default)]
enum GameState {
    #[default]
    Intro,
    Menu,
    // Game,
    // About,
    // Quit
}

#[derive(Clone, Copy, Eq, PartialEq, Hash, Debug)]
enum GameMode {
    PlayWithFriend,
    PlayForMoney
}

#[derive(Component)]
struct DisplayedText;

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, bevy::window::close_on_esc)
        .add_state::<GameState>()
        .add_plugins((Intro::IntroPlugin, MainMenu::MainMenuPlugin))
        .run();
}

mod Intro {
    use bevy::prelude::*;
    use crate::{DisplayedText, GameState};

    #[derive(Component)]
    pub struct IntroArtefact;
    #[derive(Component)]
    pub struct AnimatedLogoOverlay;

    pub struct IntroPlugin;

    impl Plugin for IntroPlugin {
        fn build(&self, app: &mut App) {
            app
                .add_systems(OnEnter(GameState::Intro), intro_setup)
                .add_systems(OnExit(GameState::Intro), cleanup)
                .add_systems(Update, (fadeout_logo, show_main_menu));
        }
    }

    fn intro_setup(mut commands: Commands, asset_server: Res<AssetServer>, time: Res<Time>) {
        let logo_image = asset_server.load("logo_gunman.png");


        commands.spawn(
            (NodeBundle {
                background_color: BackgroundColor(Color::BLACK),
                style: Style {
                    display: Display::Flex,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),

                    ..default()
                },
                transform: Transform::from_xyz(0.0, 0.0, -1.0),
                ..default()
            },
                IntroArtefact
            ))
            .with_children(|parent| {
                 parent.spawn(
                     (
                         ImageBundle {
                             image: UiImage { texture: logo_image, ..default() },
                             style: Style {
                                 width: Val::Px(400.0),
                                 height: Val::Px(400.0),
                                 ..default()
                             },
                             transform: Transform::from_xyz(0.0, 0.0, 0.0),
                             ..default()
                         },
                     ));

                parent.spawn((NodeBundle {
                    background_color: BackgroundColor(Color::Rgba {red: 0.0, green: 0.0, blue: 0.0, alpha: 0.0}),
                    style: Style {
                        position_type: PositionType::Absolute,
                        width: Val::Px(400.0),
                        height: Val::Px(400.0),
                        ..default()
                    },
                    ..default()
                },AnimatedLogoOverlay))
                ;
            })
        ;
    }

    fn show_main_menu(
        mut commands: Commands,
        time: Res<Time>,
        state: Res<State<GameState>>,
        mut next_state: ResMut<NextState<GameState>>,
        text_query: Query<Entity, With<DisplayedText>>,
    ) {
        match state.get() {
            GameState::Intro => {
                for text in text_query.iter() {
                    commands.entity(text).despawn();
                }

                commands.spawn((TextBundle{
                    text: Text::from_section(time.elapsed_seconds_wrapped().to_string(), TextStyle{..default()}),
                    ..default()
                }, DisplayedText, IntroArtefact));
                if time.elapsed_seconds_wrapped() > 5.0 {
                    next_state.set(GameState::Menu);
                }
            }
            _ => {}
        };
    }

    fn fadeout_logo(
        mut commands: Commands,
        mut query: Query<&mut BackgroundColor, With<AnimatedLogoOverlay>>,
        time: Res<Time>,
    ) {
        for mut bg_color in query.iter_mut() {
            let elapsed = time.elapsed_seconds_wrapped();
            let alpha = elapsed * 20.0/100.0;
            bg_color.0 = Color::Rgba { red: 0.0, green: 0.0, blue: 0.0, alpha: alpha };
        }
    }

    fn cleanup(mut commands: Commands, query: Query<Entity, With<IntroArtefact>>) {
        for entity in query.iter() {
            commands.entity(entity).despawn_recursive();
        }
    }
}

mod MainMenu {
    use bevy::prelude::*;
    use crate::{DisplayedText, GameState};

    pub struct MainMenuPlugin;

    impl Plugin for MainMenuPlugin {
        fn build(&self, app: &mut App) {
            app.add_systems(OnEnter(GameState::Menu), (render_main_menu));
        }
    }

    fn render_main_menu(mut commands: Commands) {
        commands.spawn((TextBundle{
            text: Text::from_section("Menu Will Be here", TextStyle{..default()}),
            ..default()
        }));

        commands.spawn(NodeBundle {
            style: Style {
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                ..default()
            },
            ..default()
        }).with_children(|parent| {
            parent.spawn(ButtonBundle {
                style: Style {
                    width: Val::Px(100.0),
                    ..default()
                },
                ..default()
            }).with_children(|parent| {
                parent.spawn(TextBundle {
                    style: Style {
                        justify_content: JustifyContent::Center,
                        ..default()
                    },
                    text: Text::from_section("Singleplayer", TextStyle {
                        color: Color::BLACK,
                        ..default()}
                    ),
                    ..default()
                });
            });

            parent.spawn(ButtonBundle {
                style: Style {width: Val::Px(100.0), ..default()},
                ..default()
            }).with_children(|parent| {
                parent.spawn(TextBundle {
                    text: Text::from_section("Play With Friend", TextStyle {..default()}),
                    ..default()
                });
            });

            parent.spawn(ButtonBundle {
                style: Style {width: Val::Px(100.0), ..default()},
                ..default()
            }).with_children(|parent| {
                parent.spawn(TextBundle {
                    text: Text::from_section("Play For Crypto", TextStyle {..default()}),
                    ..default()
                });
            });

            parent.spawn(ButtonBundle {
                style: Style {width: Val::Px(100.0), ..default()},
                ..default()
            }).with_children(|parent| {
                parent.spawn(TextBundle {
                    text: Text::from_section("Settings", TextStyle {..default()}),
                    ..default()
                });
            });

            parent.spawn(ButtonBundle {
                style: Style {width: Val::Px(100.0), ..default()},
                ..default()
            }).with_children(|parent| {
                parent.spawn(TextBundle {
                    text: Text::from_section("Quit", TextStyle {..default()}),
                    ..default()
                });
            });
        });

    }
}
