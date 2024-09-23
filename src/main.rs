use bevy::prelude::*;

mod pane;
use pane::{spawn_pane, RootPaneLayoutNode};
fn main() {
    App::new()
        // plugins are registered as part of the "app building" process
        .add_plugins((DefaultPlugins, TilingPlugin))
        .run();
}

pub struct TilingPlugin;

impl Plugin for TilingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (setup_pane.after(setup), setup));
    }
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());

    commands
        .spawn(NodeBundle {
            style: Style {
                display: Display::Grid,
                grid_template_columns: vec![GridTrack::percent(100.0)],
                grid_template_rows: vec![GridTrack::percent(100.0)],
                width: Val::Vw(100.0),
                height: Val::Vh(100.0),
                ..Default::default()
            },

            ..Default::default()
        })
        .insert(RootPaneLayoutNode);
}

fn setup_pane(
    mut commands: Commands,
    root: Query<Entity, With<RootPaneLayoutNode>>,
    mut style: Query<&mut Style>,
) {
    style.get_mut(root.single()).unwrap().grid_template_columns =
        vec![GridTrack::percent(50.0), GridTrack::percent(50.0)];
    style.get_mut(root.single()).unwrap().grid_template_rows =
        vec![GridTrack::percent(50.0), GridTrack::percent(50.0)];
    spawn_pane(&mut commands, root.single(), 1, 1, 1, 1);
    spawn_pane(&mut commands, root.single(), 1, 2, 1, 1);
    spawn_pane(&mut commands, root.single(), 2, 1, 2, 1);
}

fn px_to_percent(px: f32, total: f32) -> f32 {
    (px / total) * 100.0
}
//Use the mouse positing to adjust the column width of two columns so the border is on the mouse position
fn column_width_percentage_from_mouse(mouse_pos: f32, total_width: f32) -> f32 {
    px_to_percent(mouse_pos, total_width)
}
