use crate::random::*;
use bevy::prelude::*;
use rand::RngCore;
use std::collections::HashSet;

#[derive(Resource, Deref, DerefMut, Default)]
struct Items(HashSet<u32>);

fn spawn_entities(mut items: ResMut<Items>) {
    for i in 0..1000 {
        items.insert(i);
    }
}

fn update_entities(items: Res<Items>, mut random: ResMut<Random>) {
    for _ in 0..1000 {
        if items.contains(&random.next_u32()) {}
    }
}

pub struct HashMapPlugin;

impl Plugin for HashMapPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Items::default());
        app.add_systems(Startup, spawn_entities);
        app.add_systems(Update, update_entities);
    }
}
