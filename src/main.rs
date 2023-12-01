use bevy::{prelude::*, render::camera::ScalingMode};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins
            // default_nearest() suggests that it sets the default image scaling algorithm to "nearest neighbor". 
            // This algorithm is often used for pixel art or low-resolution images because it keeps the pixels sharp and doesn't blur them when scaling.
            .set(ImagePlugin::default_nearest())
            .set(WindowPlugin{
                primary_window: Some(Window {
                    title: "My Farm".into(), 
                    resolution: (640.0, 480.0).into(),
                    resizable: false,
                    ..default()
                }),
                ..default()
            })
            .build(),
        )
        .insert_resource(ClearColor(Color::rgb(0.9, 0.9, 0.9)))
        .insert_resource(Money(100.0))
        .add_systems(Startup, (setup, spawn_game_ui))
        .add_systems(Update, (character_movement, spawn_pig, pig_lifetime, update_money_ui))
        .run();
}

#[derive(Component)]
pub struct Player{
    pub speed: f32,
}

#[derive(Resource)]
pub struct Money(pub f32);

#[derive(Component)]
pub struct Pig {
    pub lifetime: Timer,
}

#[derive(Component)]
pub struct MoneyText;

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>
){
    // commands.spawn(Camera2dBundle::default());
    let mut camera = Camera2dBundle::default();
    camera.projection.scaling_mode = ScalingMode::AutoMin {
        min_width: 256.0,
        min_height: 144.0,
    };

    commands.spawn(camera);

    let texture = asset_server.load("character.png");

    commands.spawn((
        SpriteBundle{
            texture,
            ..default()
        },
        Player{speed: 100.0}
    ));
    // .insert(Player{speed:100.0});
}

fn character_movement(
    mut characters: Query<(&mut Transform, &Player)>,
    input: Res<Input<KeyCode>>,
    time: Res<Time>,
){
    for (mut transform, player) in &mut characters{
        let movement_amount = player.speed * time.delta_seconds();
        if input.pressed(KeyCode::W){
            transform.translation.y += movement_amount;
        }
        if input.pressed(KeyCode::S){
            transform.translation.y -= movement_amount;
        }
        if input.pressed(KeyCode::D){
            transform.translation.x += movement_amount;
        }
        if input.pressed(KeyCode::A){
            transform.translation.x -= movement_amount;
        }
    }
}

fn spawn_pig(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    input: Res<Input<KeyCode>>,
    mut money: ResMut<Money>,
    player: Query<&Transform, With<Player>>,
){
    if !input.just_pressed(KeyCode::Space) {
        return;
    }

    let player_transform = player.single();
    if money.0 >= 10.0{
        money.0 -= 10.0;
        info!("Spent RM 10 on a pig, remaining money: {}", money.0);

        let texture = asset_server.load("pig.png");

        commands.spawn((
            SpriteBundle{
                texture,
                // player_transform is a reference to a Transform struct 
                // When you pass this Transform to the SpriteBundle, it expects an actual Transform value, not a reference to one.
                // By using *player_transform, you are dereferencing player_transform to get the Transform value it refers to. 
                transform: *player_transform,
                ..default()
            },
            Pig{
                // 2.0 is the duration of the timer in seconds. So, this timer will count for 2 seconds.
                // TimerMode::Once is the mode of the timer. It tells the timer to only count down once and then stop. It won't repeat.
                lifetime:Timer::from_seconds(2.0, TimerMode::Once),
            }
        ));
    }
    
}

fn pig_lifetime(
    mut commands: Commands,
    time: Res<Time>,
    mut pigs: Query<(Entity, &mut Pig)>,
    mut money: ResMut<Money>,
){
    for(pig_entity, mut pig) in &mut pigs {
        pig.lifetime.tick(time.delta());

        // This condition checks if the pig's lifetime timer has finished (reached 0).
        if pig.lifetime.finished() {
            money.0 += 15.0;
            commands.entity(pig_entity).despawn();
            info!("Pig sold for RM 15 ! Current Money: RM {}", money.0);
        }
    }
}

fn spawn_game_ui(
    mut commands: Commands
) {
    commands.spawn((
        // nodes are the basic building blocks for creating user interfaces.
        NodeBundle {
            style: Style {
                // Sets the width of the node to 100% of its parent's width. 
                width: Val::Percent(100.0),
                // Sets the height of the node to 10% of its parent's height.
                height: Val::Percent(10.0),
                // Aligns the child elements of the node to the center.
                align_items: AlignItems::Center,
                // Adds padding of 10 pixels on all sides (top, right, bottom, left) of the node.
                padding:UiRect::all(Val::Px(10.0)),
                ..default()
            },
            background_color: Color::BLUE.into(),
            ..default()
        },
        // Adds a Name component to the entity
        Name::new("UI Root"),
    ))
    // with_children method is used to attach child elements to the root node.
    .with_children(|commands|{
        commands.spawn((
            // This struct is used to create a text element in the UI. 
            TextBundle{
                text:Text::from_section(
                    "Money!",
                    TextStyle{
                        font_size: 32.0,
                        ..default()
                    },
                ),
                ..default()
            },
            MoneyText,
        ));
    });
}

fn update_money_ui(
    mut texts:Query<&mut Text, With<MoneyText>>, 
    money: Res<Money>
){
    for mut text in &mut texts{
        text.sections[0].value = format!("Money: RM {}", money.0);
    }
}
