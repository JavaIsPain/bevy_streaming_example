//! A simple 3D scene with light shining over a cube sitting on a plane.

use bevy::prelude::*;
use bevy_mod_picking::prelude::*;
use game_server_logic::{Clicked, GameServerLogic, GridPosition, SelectedUnit, Tile, Unit};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(DefaultPickingPlugins)
        .add_plugins(GameServerLogic)
        .add_systems(Startup, setup)
        .add_systems(PostUpdate, (draw_tiles,draw_units, move_units, display_selection))
        .add_systems(Last, make_pickable)
        .run();
}

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
) {
    // light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });
    // camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-2.5, 4.5, 9.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}

fn draw_tiles(
    mut commands: Commands,
    tiles: Query<(Entity, &Tile, &GridPosition), Without<Handle<Scene>>>,
    asset_server: Res<AssetServer>,
) {
    for (entity, tile, transform) in &tiles {
        let asset = match tile {
            Tile::FlatGround => "tile_dirt",
            Tile::Rocks => "tile_rock",
            Tile::Forest => "tile_tree",
        };
        let current_transform = transform.clone();
        commands.entity(entity).insert((
            SceneBundle {
                transform: Transform::from_xyz(transform.x as f32, 0.0, transform.y as f32),
                scene: asset_server.load(format!(
                    "client_assets/kenney-tower-defense/Models/GLTF format/{asset}.glb#Scene0"
                )),
                ..Default::default()
            },
            PickableBundle::default(),
            On::<Pointer<Click>>::run(move |event: Listener<Pointer<Click>>, mut writer: EventWriter<Clicked>| {
                let position_normalized = event.hit.position.unwrap_or(Vec3::splat(-0.5)) + 0.5;
                let position_normalized = position_normalized - position_normalized.floor();
                let x_offset = (position_normalized.x * u8::MAX as f32) as u8;
                let y_offset = (position_normalized.z * u8::MAX as f32) as u8;
                writer.send(Clicked::Tile { x: current_transform.x, y: current_transform.y, x_offset, y_offset });
            }),
        ));
    }
}

fn draw_units(
    mut commands: Commands,
    units: Query<(Entity, &Unit, &GridPosition), Without<Handle<Mesh>>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    if units.is_empty() {
        return;
    }
    let red = materials.add(Color::RED);
    let blue = materials.add(Color::BLUE);
    let mesh = meshes.add(Cuboid::new(0.5, 1.0, 0.5));
    for (entity, unit, transform) in &units {
        let material = match unit {
            Unit::Blue => blue.clone(),
            Unit::Red => red.clone(),
        };
        let current = entity;
        commands.entity(entity).insert((
            PbrBundle {
                mesh: mesh.clone(),
                material,
                transform: Transform::from_xyz(transform.x as f32, 0.6, transform.y as f32),
                ..default()
            },
            PickableBundle::default(),
            On::<Pointer<Click>>::run(move |mut writer: EventWriter<Clicked>, query: Query<&GridPosition>| {
                let Ok(transform) = query.get(current) else {
                    return;
                };
                writer.send(Clicked::Unit { x: transform.x, y: transform.y, entity });
            }),
        ));
    }
}

fn move_units(mut commands: Commands,
    units: Query<(Entity, &GridPosition), Changed<GridPosition>>) {
    for (entity, position) in &units {
        commands.entity(entity).insert(Transform::from_xyz(position.x as f32, 0.6, position.y as f32));
    }
}

#[derive(Component)]
struct SelectionMarker(Entity);

fn display_selection(mut commands: Commands, 
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    selected: Query<Entity, Added<SelectedUnit>>, existing_selections: Query<&SelectionMarker>, mut removed: RemovedComponents<SelectedUnit>) {
    for selected in &selected {
        let mut child = None;
        commands.entity(selected).with_children(|p| {
            let material = materials.add(Color::GREEN);
            let mesh: Handle<Mesh> = meshes.add(Cuboid::new(0.2, 0.2, 0.2));
   
            let id = p.spawn(PbrBundle {
                transform: Transform::from_xyz(0., 1.0, 0.),
                mesh,
                material,
                ..Default::default()
            }).id();
            child = Some(id);
        });
        if let Some(child) = child {
            commands.entity(selected).insert(SelectionMarker(child));
        }
    }

    for removed in removed.read() {
        if let Ok(selection) = existing_selections.get(removed) {
            commands.entity(selection.0).despawn_recursive();
        }
    }
}

fn make_pickable(
    mut commands: Commands,
    meshes: Query<Entity, (With<Handle<Mesh>>, Without<Pickable>)>,
) {
    for entity in meshes.iter() {
        commands
            .entity(entity)
            .insert(PickableBundle::default());
    }
}
