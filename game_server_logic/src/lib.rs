use bevy::prelude::*;

pub struct GameServerLogic;

impl Plugin for GameServerLogic {
    fn build(&self, app: &mut App) {
        app
            .add_event::<Clicked>()
            .add_systems(Startup, spawn_map)
            .add_systems(Update, process_click);
    }
}

#[derive(Component, Clone, Debug, Reflect)]
pub struct SelectedUnit;

#[derive(Component, Clone, Debug, Reflect)]
pub enum Tile {
    FlatGround,
    Rocks,
    Forest
}

#[derive(Component, Clone, Debug, Reflect)]
pub enum Unit {
    Blue,
    Red
}

#[derive(Event, Clone, Debug, Reflect)]
pub enum Clicked {
    Unit { x: u32, y: u32, entity: Entity },
    Tile { x: u32, y: u32, x_offset: u8, y_offset: u8 }
}

#[derive(Component, Clone, Debug, Reflect)]
pub struct GridPosition { pub x: u32, pub y: u32 }

const LEVEL_LAYOUT : [[Tile;3];3] = [
    [ Tile::Forest, Tile::Forest, Tile::Rocks],
    [ Tile::FlatGround, Tile::Forest, Tile::FlatGround],
    [Tile::FlatGround, Tile::FlatGround, Tile::FlatGround]
];

const UNITS : [(u32, u32, Unit);2] = [(0,1, Unit::Blue), (2,1, Unit::Red)];

fn spawn_map(mut commands: Commands) {
    for (y, row) in LEVEL_LAYOUT.iter().enumerate() {
        for (x, tile) in row.iter().enumerate() {
            commands.spawn((GridPosition { x: x as u32,y: y as u32}, tile.clone()));
        }
    }

    for (x,y,unit) in UNITS {
        commands.spawn((GridPosition { x, y, }, unit.clone()));
    }
}

fn process_click(mut clicks: EventReader<Clicked>, mut commands: Commands, units: Query<(Entity, &Unit, &GridPosition)>, tiles: Query<(&Tile, &GridPosition)>, selected_units: Query<Entity, With<SelectedUnit>>) {
    for click in clicks.read() {
        match click{
            Clicked::Unit { x, y, entity } => {
                for entity in &selected_units {
                    commands.entity(entity).remove::<SelectedUnit>();
                }
                if units.get(*entity).is_ok() {
                    commands.entity(*entity).insert(SelectedUnit);
                }
            },
            Clicked::Tile { x, y, x_offset, y_offset } => {
                for unit in &selected_units {
                    commands.entity(unit).insert(GridPosition { x: *x, y: *y});
                }
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
