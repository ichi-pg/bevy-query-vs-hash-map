use crate::random::*;
use bevy::prelude::*;
use rand::RngCore;

#[derive(Component)]
struct ItemID(u32);

fn spawn_entities(mut commands: Commands) {
    for i in 0..1000 {
        commands.spawn((NodeBundle::default(), ItemID(i)));
    }
}

fn update_entities(query: Query<&ItemID>, mut random: ResMut<Random>) {
    for _ in 0..1000 {
        let i = random.next_u32();
        for item_id in &query {
            if item_id.0 == i {}
        }
    }
}

pub struct QueryPlugin;

impl Plugin for QueryPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_entities);
        app.add_systems(Update, update_entities);
    }
}
