use bevy::prelude::*;
mod camera;
mod hash_map;
mod profiler;
mod query;
mod random;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Query vs Hash Map".into(),
                position: WindowPosition::Centered(MonitorSelection::Primary),
                ..default()
            }),
            ..default()
        }))
        .add_plugins((
            camera::CameraPlugin,
            random::RandomPlugin,
            profiler::ProfilerPlugin,
            // query::QueryPlugin,
            hash_map::HashMapPlugin,
        ))
        .run();
}
