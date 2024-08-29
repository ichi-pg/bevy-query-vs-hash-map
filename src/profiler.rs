use bevy::diagnostic::*;
use bevy::prelude::*;
use iyes_perf_ui::entries::*;
use iyes_perf_ui::*;

fn spawn_profiler(mut commands: Commands) {
    commands.spawn(PerfUiBundle::default());
}

pub struct ProfilerPlugin;

impl Plugin for ProfilerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            FrameTimeDiagnosticsPlugin,
            EntityCountDiagnosticsPlugin,
            SystemInformationDiagnosticsPlugin,
            PerfUiPlugin,
        ));
        app.add_systems(Startup, spawn_profiler);
    }
}
