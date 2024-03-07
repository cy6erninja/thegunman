use bevy::prelude::*;
use bevy_rapier2d::plugin::{NoUserData, RapierPhysicsPlugin};
use bevy_rapier2d::render::RapierDebugRenderPlugin;

const DEFAULT_TILE_SIZE: f32 = 10.0;

#[derive(Reflect, States, Clone, Copy, Eq, Hash, Debug, PartialEq, Default)]
enum GameState {
    #[default]
    Intro,
    Menu,
    SingleplayerGame,
    GameWithFriend,
    GameForMoney,
    Settings,
    Quit
}

#[derive(Component, PartialEq)]
enum MenuButtonAction {
    StartSinglePlayerGame,
    StartPlayingWithFriend,
    StartPlayingForMoney,
    OpenSettings,
    Quit
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
        .add_plugins(
            (
                RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0),
                RapierDebugRenderPlugin::default()
            )
        )
        .add_plugins((Intro::IntroPlugin, MainMenu::MainMenuPlugin, SingleplayerGame::SingleplayerGamePlugin))
        .add_systems(Startup, setup)
        .add_systems(Update, bevy::window::close_on_esc)
        .add_state::<GameState>()
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
                .add_systems(Update, (logo_fadein, show_main_menu));
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
                if time.elapsed_seconds_wrapped() > 2.0 {
                    next_state.set(GameState::Menu);
                }
            }
            _ => {}
        };
    }

    fn logo_fadein(
        mut commands: Commands,
        mut query: Query<&mut BackgroundColor, With<AnimatedLogoOverlay>>,
        time: Res<Time>,
    ) {
        for mut bg_color in query.iter_mut() {
            let elapsed = time.elapsed_seconds_wrapped();
            let alpha = 1.0 - elapsed * 2.0 / 100.0;
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
    use std::process;
    use bevy::prelude::*;
    use crate::{GameState, MenuButtonAction};
    use crate::Intro::IntroArtefact;

    pub struct MainMenuPlugin;

    impl Plugin for MainMenuPlugin {
        fn build(&self, app: &mut App) {
            app.add_systems(OnEnter(GameState::Menu), (render_main_menu))
                .add_systems(OnExit(GameState::Menu), (cleanup))
                .add_systems(Update, (handle_menu_button_actions));
        }
    }

    fn render_main_menu(mut commands: Commands, mouse: ResMut<Input<MouseButton>>) {
        fn render_button(parent: &mut ChildBuilder, name: &str, menu_button_action: MenuButtonAction) {
            parent.spawn((ButtonBundle {
                style: Style {
                    padding: UiRect::all(Val::Px(5.0)),
                    ..default()
                },
                background_color: BackgroundColor(Color::RED),
                ..default()
            }, menu_button_action)).with_children(|parent| {
                parent.spawn(TextBundle {
                    style: Style {
                        justify_content: JustifyContent::Center,
                        ..default()
                    },
                    text: Text::from_section(name, TextStyle {
                        color: Color::BLACK,
                        font_size: 20.0,
                        ..default()}
                    ),
                    ..default()
                });
            });
        }

        commands.spawn(NodeBundle {
            style: Style {
                display: Display::Flex,
                row_gap: Val::Px(20.0),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                ..default()
            },
            ..default()
        }).with_children(|mut parent| {
            render_button(&mut parent, "Singleplayer", MenuButtonAction::StartSinglePlayerGame);
            render_button(&mut parent, "Play With A Friend", MenuButtonAction::StartPlayingWithFriend);
            render_button(&mut parent, "Play For Crypto", MenuButtonAction::StartPlayingForMoney);
            render_button(&mut parent, "Settings", MenuButtonAction::OpenSettings);
            render_button(&mut parent, "Quit", MenuButtonAction::Quit);
        });
    }

    fn handle_menu_button_actions(
        query: Query<(&MenuButtonAction, &Interaction), Changed<Interaction>>,
        mut next_state: ResMut<NextState<GameState>>
    ) {
        for (mba, inter) in query.iter() {
           if *inter == Interaction::Pressed {
               match *mba {
                   MenuButtonAction::StartSinglePlayerGame => {
                       next_state.set(GameState::SingleplayerGame);
                   },
                   MenuButtonAction::StartPlayingWithFriend => {
                       next_state.set(GameState::GameWithFriend);
                   },
                   MenuButtonAction::StartPlayingForMoney => {
                       next_state.set(GameState::GameForMoney);
                   },
                   MenuButtonAction::OpenSettings => {
                       next_state.set(GameState::Settings);
                   },
                   MenuButtonAction::Quit => {
                       process::exit(0x0);
                   }
               };
           }
        }
    }

    fn cleanup(mut commands: Commands, query: Query<Entity, With<MenuButtonAction>>) {
        for entity in query.iter() {
            commands.entity(entity).despawn_recursive();
        }
    }
}

mod SingleplayerGame {
    use bevy::prelude::*;
    use bevy_rapier2d::na::abs;
    use bevy_rapier2d::parry::math::Rotation;
    use bevy_rapier2d::prelude::{Collider, Friction, GravityScale, LockedAxes, RigidBody, Velocity};
    use bevy_rapier2d::rapier::prelude::RigidBodyVelocity;
    use rand::Rng;
    use crate::{DEFAULT_TILE_SIZE, GameState};

    #[derive(Component)]
    struct GameEntity;

    #[derive(Component)]
    struct Character;

    pub struct SingleplayerGamePlugin;

    impl Plugin for SingleplayerGamePlugin{
        fn build(&self, app: &mut App) {
            app.add_systems(OnEnter(GameState::SingleplayerGame), (init_singleplayer_game, generate_map))
                .add_systems(Update, move_character)
                .add_systems(OnExit(GameState::SingleplayerGame), cleanup)
            ;
        }
    }

    fn init_singleplayer_game(mut commands: Commands, asset_server: Res<AssetServer>) {
        let main_character = asset_server.load("platformer/Tiles/Characters/tile_0010.png");

        commands.spawn(
            (
                SpriteBundle {
                    texture: main_character,
                    visibility: Visibility::Visible,
                    ..default()
                },
                Character,
                RigidBody::Dynamic,
                Collider::cuboid(DEFAULT_TILE_SIZE, DEFAULT_TILE_SIZE),
                GravityScale(2.0),
                Velocity::zero(),
                LockedAxes::ROTATION_LOCKED,
                Friction::new(2.0)
            )
        );
    }

    pub fn generate_map(mut commands: Commands, asset_server: Res<AssetServer>) {
        const MAP_START_X: f32 = -1000.0;

        let ground_texture = asset_server.load("platformer/Tiles/tile_0041.png");
        let bg_texture = asset_server.load("platformer/Tiles/Backgrounds/tile_0023.png");

        for i in 0..(MAP_START_X / DEFAULT_TILE_SIZE).abs() as i32 {
            commands.spawn(
                (
                    SpriteBundle {
                        texture: ground_texture.clone(),
                        transform: Transform::from_xyz(MAP_START_X + i as f32 * DEFAULT_TILE_SIZE * 2.0, -200.0, 1.0),
                        ..default()
                    },
                    RigidBody::Fixed,
                    Collider::cuboid(DEFAULT_TILE_SIZE, DEFAULT_TILE_SIZE),
                )
            );
        }

        for  i in 0..(MAP_START_X / DEFAULT_TILE_SIZE).abs() as i32 {
            for j in 0..20 {
                commands.spawn(
                    (
                        SpriteBundle {
                            texture: bg_texture.clone(),
                            transform: Transform::from_xyz(MAP_START_X + i as f32 * DEFAULT_TILE_SIZE * 2.0, 190.0 - j as f32 * DEFAULT_TILE_SIZE * 2.0, -1.0),
                            ..default()
                        },
                        RigidBody::Fixed,
                        // Collider::cuboid(DEFAULT_TILE_SIZE, DEFAULT_TILE_SIZE),
                    )
                );

            }
        }

        //Level 2 platforms

        let mut rng = rand::thread_rng();
        let mut consequtive_gaps: u32 = 0;
        let mut consequtive_tiles: u32 = 0;

        const MIN_CONSEQUTIVE_GAPS: u32 = 2;
        const MAX_CONSEQUTIVE_GAPS: u32 = 3;
        const MIN_CONSEQUTIVE_TILES:u32 = 2;

        for i in 0..(MAP_START_X / DEFAULT_TILE_SIZE).abs() as i32 {
            let is_empty_tile = rng.gen::<bool>() || MIN_CONSEQUTIVE_GAPS - consequtive_gaps == 1;

            if consequtive_tiles >= MIN_CONSEQUTIVE_TILES && is_empty_tile && consequtive_gaps < MAX_CONSEQUTIVE_GAPS {
                consequtive_tiles = 0;
                consequtive_gaps += 1;

                continue
            };

            consequtive_tiles += 1;
            consequtive_gaps = 0;

            commands.spawn(
                (
                    SpriteBundle {
                        texture: ground_texture.clone(),
                        transform: Transform::from_xyz(MAP_START_X + i as f32 * DEFAULT_TILE_SIZE * 2.0, -200.0 + 4.0 * DEFAULT_TILE_SIZE * 2.0, 1.0),
                        ..default()
                    },
                    RigidBody::Fixed,
                    Collider::cuboid(DEFAULT_TILE_SIZE, DEFAULT_TILE_SIZE),
                )
            );
        }
    }

    fn move_character(
        keyboard_input: Res<Input<KeyCode>>,
        mut query: Query<&mut Velocity, With<Character>>
    ) {
        const CHARACTER_SPEED: f32 = 100.0;
        for (mut velocity) in query.iter_mut() {
            if keyboard_input.pressed(KeyCode::Right) {
                velocity.linvel = Vec2::new(CHARACTER_SPEED, velocity.linvel.y).into();
            }

            if keyboard_input.pressed(KeyCode::Left) {
                velocity.linvel = Vec2::new(-CHARACTER_SPEED, velocity.linvel.y).into();
            }
        }
    }

    fn cleanup(mut commands: Commands, query: Query<Entity, With<GameEntity>>) {
        for entity in query.iter() {
            commands.entity(entity).despawn_recursive();
        }
    }
}
