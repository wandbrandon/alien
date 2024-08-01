pub mod pixel_camera;
pub mod character_controller;

use avian2d::{ math::{ Scalar, Vector }, prelude::* };
use character_controller::{ CharacterControllerBundle, CharacterControllerPlugin };
use pixel_camera::{ PixelCameraPlugin, PIXEL_PERFECT_LAYERS };
use bevy::{
    prelude::*,
    render::{ mesh::PrimitiveTopology, render_asset::RenderAssetUsages },
    sprite::MaterialMesh2dBundle,
};
use leafwing_input_manager::{ axislike::AxisType, prelude::* };

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug, Reflect)]
enum Action {
    Move,
    Jump,
    Rudder,
}

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(ImagePlugin::default_nearest()),
            PixelCameraPlugin,
            InputManagerPlugin::<Action>::default(),
            PhysicsPlugins::default().with_length_unit(70.0),
            PhysicsDebugPlugin::default(),
            CharacterControllerPlugin,
        ))
        .insert_resource(Msaa::Off)
        .insert_resource(ClearColor(Color::srgb(0.05, 0.05, 0.1)))
        .insert_resource(Gravity(Vector::NEG_Y * 1000.0))
        .add_systems(Startup, setup)
        //.add_systems(Update, player_control_system)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>
) {
    // Player
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes.add(Capsule2d::new(12.5, 20.0)).into(),
            material: materials.add(Color::srgb(0.2, 0.7, 0.9)),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        },
        CharacterControllerBundle::new(Collider::capsule(12.5, 20.0)).with_movement(
            1250.0,
            0.92,
            400.0,
            (30.0 as Scalar).to_radians()
        ),
        Friction::ZERO.with_combine_rule(CoefficientCombine::Min),
        Restitution::ZERO.with_combine_rule(CoefficientCombine::Min),
        ColliderDensity(2.0),
        GravityScale(1.5),
    ));

    // A cube to move around
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::srgb(0.0, 0.4, 0.7),
                custom_size: Some(Vec2::new(30.0, 30.0)),
                ..default()
            },
            transform: Transform::from_xyz(50.0, 0.0, 0.0),
            ..default()
        },
        RigidBody::Dynamic,
        Collider::rectangle(30.0, 30.0),
    ));

    // Platforms
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::srgb(0.7, 0.7, 0.8),
                custom_size: Some(Vec2::new(100.0, 50.0)),
                ..default()
            },
            transform: Transform::from_xyz(0.0, -100.0, 0.0),
            ..default()
        },
        RigidBody::Static,
        Collider::rectangle(1100.0, 50.0),
    ));
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::srgb(0.7, 0.7, 0.8),
                custom_size: Some(Vec2::new(200.0, 25.0)),
                ..default()
            },
            transform: Transform::from_xyz(100.0, -35.0, 0.0),
            ..default()
        },
        RigidBody::Static,
        Collider::rectangle(200.0, 25.0),
    ));
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::srgb(0.7, 0.7, 0.8),
                custom_size: Some(Vec2::new(200.0, 25.0)),
                ..default()
            },
            transform: Transform::from_xyz(-100.0, 0.0, 0.0),
            ..default()
        },
        RigidBody::Static,
        Collider::rectangle(200.0, 25.0),
    ));
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::srgb(0.7, 0.7, 0.8),
                custom_size: Some(Vec2::new(80.0, 80.0)),
                ..default()
            },
            transform: Transform::from_xyz(300.0, -110.0, 0.0),
            ..default()
        },
        RigidBody::Static,
        Collider::rectangle(80.0, 80.0),
    ));
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::srgb(0.7, 0.7, 0.8),
                custom_size: Some(Vec2::new(80.0, 80.0)),
                ..default()
            },
            transform: Transform::from_xyz(-300.0, -110.0, 0.0),
            ..default()
        },
        RigidBody::Static,
        Collider::rectangle(150.0, 80.0),
    ));

    // Ramps

    let mut ramp_mesh = Mesh::new(PrimitiveTopology::TriangleList, RenderAssetUsages::default());

    ramp_mesh.insert_attribute(
        Mesh::ATTRIBUTE_POSITION,
        vec![[-125.0, 80.0, 0.0], [-125.0, 0.0, 0.0], [125.0, 0.0, 0.0]]
    );

    let ramp_collider = Collider::triangle(
        Vector::new(-125.0, 80.0),
        Vector::NEG_X * 125.0,
        Vector::X * 125.0
    );

    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes.add(ramp_mesh).into(),
            material: materials.add(Color::srgb(0.4, 0.4, 0.5)),
            transform: Transform::from_xyz(-275.0, -150.0, 0.0),
            ..default()
        },
        RigidBody::Static,
        ramp_collider,
    ));

    let mut ramp_mesh = Mesh::new(PrimitiveTopology::TriangleList, RenderAssetUsages::default());

    ramp_mesh.insert_attribute(
        Mesh::ATTRIBUTE_POSITION,
        vec![[20.0, -40.0, 0.0], [20.0, 40.0, 0.0], [-20.0, -40.0, 0.0]]
    );

    let ramp_collider = Collider::triangle(
        Vector::new(20.0, -40.0),
        Vector::new(20.0, 40.0),
        Vector::new(-20.0, -40.0)
    );

    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes.add(ramp_mesh).into(),
            material: materials.add(Color::srgb(0.4, 0.4, 0.5)),
            transform: Transform::from_xyz(200.0, -110.0, 0.0),
            ..default()
        },
        RigidBody::Static,
        ramp_collider,
    ));
}

// fn player_control_system(
//     mut query: Query<(&mut TnuaController, &ActionState<Action>), With<CharacterController>>
// ) {
//     for (mut controller, action_state) in query.iter_mut() {
//         if action_state.pressed(&Action::Move) {
//             println!("Moving in direction {:?}", action_state.action_data(&Action::Move));
//             controller.basis(TnuaBuiltinWalk {
//                 // Move in the direction the player entered, at a speed of 10.0:
//                 desired_velocity: Vec3 {
//                     x: action_state.value(&Action::Move) * 20.0,
//                     y: 0.0,
//                     z: 0.0,
//                 },

//                 // Turn the character in the movement direction:
//                 desired_forward: Vec3 {
//                     x: action_state.value(&Action::Move),
//                     y: 0.0,
//                     z: 0.0,
//                 },

//                 // Must be larger than the height of the entity's center from the bottom of its
//                 // collider, or else the character will not float and Tnua will not work properly:
//                 float_height: 64.0,

//                 // TnuaBuiltinWalk has many other fields that can be configured:
//                 ..Default::default()
//             });
//         }

//         if action_state.just_pressed(&Action::Jump) {
//             println!("Jumped");
//             // The jump action must be fed as long as the player holds the button.
//             controller.action(TnuaBuiltinJump {
//                 // The full height of the jump, if the player does not release the button:
//                 height: 4.0,

//                 // TnuaBuiltinJump too has other fields that can be configured:
//                 ..Default::default()
//             });
//         }
//     }
// }

// fn spawn_character_controller(mut commands: Commands, asset_server: Res<AssetServer>) {
//     // Describes how to convert from player inputs into those actions
//     let mut input_map = InputMap::new([(Action::Jump, GamepadButtonType::South)]);

//     input_map.insert(Action::Move, SingleAxis::symmetric(GamepadAxisType::LeftStickX, default()));
//     input_map.insert(Action::Rudder, DualAxis::right_stick());

//     commands.spawn((
//         RigidBody::Static,
//         Collider::rectangle(320.0, 10.0),
//         TransformBundle::from_transform(Transform::from_xyz(0.0, -50.0, 0.0)),
//         PIXEL_PERFECT_LAYERS,
//     ));

//     commands
//         .spawn(InputManagerBundle::with_map(input_map))
//         .insert(CharacterController)
//         .insert((RigidBody::Dynamic, Collider::capsule(16.0, 16.0)))
//         .insert(TnuaControllerBundle::default())
//         .insert(SpriteBundle {
//             texture: asset_server.load("samus.png"),
//             ..default()
//         })
//         .insert(PIXEL_PERFECT_LAYERS);
// }
