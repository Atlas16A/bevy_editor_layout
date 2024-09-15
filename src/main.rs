use std::process::id;

use bevy::{ecs::observer, input::mouse::MouseMotion, prelude::*};

mod pane;
use pane::{spawn_pane, Dragger, PaneArea, PaneHeader, PaneMenuButton, PaneRoot};
fn main() {
    App::new()
        // plugins are registered as part of the "app building" process
        .add_plugins((DefaultPlugins, TilingPlugin))
        .run();
}

pub struct TilingPlugin;

impl Plugin for TilingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
    }
}

#[derive(Component)]
struct Root;

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());

    let root = commands
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
        .insert(Root)
        .id();

    spawn_pane(&mut commands, root);

    /* let pane_root = commands
        .spawn(NodeBundle {
            //PANE ROOT
            style: Style {
                display: Display::Grid,
                grid_template_columns: vec![
                    GridTrack::px(10.0),
                    GridTrack::auto(),
                    GridTrack::px(10.0),
                ],
                grid_template_rows: vec![
                    GridTrack::px(10.0),
                    GridTrack::auto(),
                    GridTrack::px(10.0),
                ],
                align_content: AlignContent::Stretch,
                justify_content: JustifyContent::Stretch,
                ..Default::default()
            },

            ..Default::default()
        })
        .insert(PaneRoot)
        .id();

    let pane_area = commands
        .spawn(NodeBundle {
            style: Style {
                grid_row: GridPlacement::start(2),
                grid_column: GridPlacement::start(2),
                display: Display::Grid,
                grid_template_columns: vec![GridTrack::percent(100.0)],
                grid_template_rows: vec![GridTrack::px(25.0), GridTrack::auto()],

                ..Default::default()
            },
            background_color: BackgroundColor(PANE_COLOR),
            border_radius: ROUNDING,

            ..Default::default()
        })
        .insert(PaneArea)
        .id();

    let pane_header = commands
        .spawn(NodeBundle {
            style: Style {
                display: Display::Flex,
                flex_direction: FlexDirection::Row,
                height: Val::Px(25.0),
                align_items: AlignItems::Center,

                grid_row: GridPlacement::start(1),
                flex_grow: 1000.0,
                padding: UiRect {
                    left: Val::Px(10.0),
                    right: Val::Px(10.0),
                    top: Val::Px(0.0),
                    bottom: Val::Px(0.0),
                },
                ..Default::default()
            },
            background_color: BackgroundColor(HEADER_COLOR),
            border_radius: BorderRadius {
                top_left: Val::Px(10.0),
                top_right: Val::Px(10.0),
                bottom_left: Val::Px(0.0),
                bottom_right: Val::Px(0.0),
            },
            ..Default::default()
        })
        .insert(PaneHeader)
        .id();

    let pane_menu_button = commands
        .spawn(ButtonBundle {
            style: Style {
                width: Val::Px(31.0),
                height: Val::Px(19.0),

                margin: UiRect {
                    left: Val::Px(0.0),
                    right: Val::Px(10.0),
                    top: Val::Px(0.0),
                    bottom: Val::Px(0.0),
                },

                ..Default::default()
            },
            background_color: BackgroundColor(BUTTON_COLOR),
            border_radius: BorderRadius {
                top_left: Val::Px(3.0),
                top_right: Val::Px(3.0),
                bottom_left: Val::Px(3.0),
                bottom_right: Val::Px(3.0),
            },

            ..Default::default()
        })
        .insert(PaneMenuButton)
        .id();

    commands.entity(root).add_child(pane_root);
    commands.entity(pane_root).add_child(pane_area);
    commands.entity(pane_area).add_child(pane_header);
    commands.entity(pane_header).add_child(pane_menu_button);

    //Observers
    let top_left_dragger = commands
        .spawn(NodeBundle {
            style: Style {
                grid_row: GridPlacement::start(1),
                grid_column: GridPlacement::start(1),
                width: Val::Px(10.0),
                height: Val::Px(10.0),
                ..Default::default()
            },
            background_color: BackgroundColor(PANE_COLOR),
            ..Default::default()
        })
        .insert(Dragger)
        .observe(|_trigger: Trigger<Pointer<Click>>| {
            println!("Click detected 1x1");
        })
        .id();

    let top_right_dragger = commands
        .spawn(NodeBundle {
            style: Style {
                grid_row: GridPlacement::start(1),
                grid_column: GridPlacement::start(3),
                width: Val::Px(10.0),
                height: Val::Px(10.0),
                ..Default::default()
            },
            background_color: BackgroundColor(PANE_COLOR),
            ..Default::default()
        })
        .insert(Dragger)
        .observe(|_trigger: Trigger<Pointer<Click>>| {
            println!("Click detected 1x3");
        })
        .id();

    let bottom_left_dragger = commands
        .spawn(NodeBundle {
            style: Style {
                grid_row: GridPlacement::start(3),
                grid_column: GridPlacement::start(1),
                width: Val::Px(10.0),
                height: Val::Px(10.0),
                ..Default::default()
            },
            background_color: BackgroundColor(PANE_COLOR),
            ..Default::default()
        })
        .insert(Dragger)
        .observe(|_trigger: Trigger<Pointer<Click>>| {
            println!("Click detected 3x1");
        })
        .id();

    let bottom_right_dragger = commands
        .spawn(NodeBundle {
            style: Style {
                grid_row: GridPlacement::start(3),
                grid_column: GridPlacement::start(3),
                width: Val::Px(10.0),
                height: Val::Px(10.0),
                ..Default::default()
            },
            background_color: BackgroundColor(PANE_COLOR),
            ..Default::default()
        })
        .insert(Dragger)
        .observe(|_trigger: Trigger<Pointer<Click>>| {
            println!("Click detected 3x3");
        })
        .id();

    let top_dragger = commands
        .spawn(NodeBundle {
            style: Style {
                grid_row: GridPlacement::start(1),
                grid_column: GridPlacement::start(2),
                justify_items: JustifyItems::Stretch,
                align_items: AlignItems::Stretch,
                margin: UiRect {
                    left: Val::Px(25.0),
                    right: Val::Px(25.0),
                    top: Val::Px(1.0),
                    bottom: Val::Px(1.0),
                },
                ..Default::default()
            },
            background_color: BackgroundColor(PANE_COLOR),
            ..Default::default()
        })
        .insert(Dragger)
        .observe(|_trigger: Trigger<Pointer<Click>>| {
            println!("Click detected Resize Top");
        })
        .id();
    let left_dragger = commands
        .spawn(NodeBundle {
            style: Style {
                grid_row: GridPlacement::start(2),
                grid_column: GridPlacement::start(1),
                justify_items: JustifyItems::Stretch,
                align_items: AlignItems::Stretch,
                margin: UiRect {
                    left: Val::Px(1.0),
                    right: Val::Px(1.0),
                    top: Val::Px(25.0),
                    bottom: Val::Px(25.0),
                },
                ..Default::default()
            },
            background_color: BackgroundColor(PANE_COLOR),
            ..Default::default()
        })
        .insert(Dragger)
        .observe(|_trigger: Trigger<Pointer<Click>>| {
            println!("Click detected Resize Left");
        })
        .id();
    let right_dragger = commands
        .spawn(NodeBundle {
            style: Style {
                grid_row: GridPlacement::start(2),
                grid_column: GridPlacement::start(3),
                justify_items: JustifyItems::Stretch,
                align_items: AlignItems::Stretch,
                margin: UiRect {
                    left: Val::Px(1.0),
                    right: Val::Px(1.0),
                    top: Val::Px(25.0),
                    bottom: Val::Px(25.0),
                },
                ..Default::default()
            },
            background_color: BackgroundColor(PANE_COLOR),
            ..Default::default()
        })
        .insert(Dragger)
        .observe(|_trigger: Trigger<Pointer<Click>>| {
            println!("Click detected Resize Right");
        })
        .id();
    let bottom_dragger = commands
        .spawn(NodeBundle {
            style: Style {
                grid_row: GridPlacement::start(3),
                grid_column: GridPlacement::start(2),
                justify_items: JustifyItems::Stretch,
                align_items: AlignItems::Stretch,
                margin: UiRect {
                    left: Val::Px(25.0),
                    right: Val::Px(25.0),
                    top: Val::Px(1.0),
                    bottom: Val::Px(1.0),
                },
                ..Default::default()
            },
            background_color: BackgroundColor(PANE_COLOR),
            ..Default::default()
        })
        .insert(Dragger)
        .observe(|_trigger: Trigger<Pointer<Click>>| {
            println!("Click detected Resize Bottom");
        })
        .id();

    commands.entity(pane_root).add_child(top_left_dragger);
    commands.entity(pane_root).add_child(top_right_dragger);
    commands.entity(pane_root).add_child(bottom_left_dragger);
    commands.entity(pane_root).add_child(bottom_right_dragger);
    commands.entity(pane_root).add_child(top_dragger);
    commands.entity(pane_root).add_child(left_dragger);
    commands.entity(pane_root).add_child(right_dragger);
    commands.entity(pane_root).add_child(bottom_dragger); */

    /* commands.entity(pane_area).observe(
        move |trigger: Trigger<Pointer<DragOver>>,
              mut commands: Commands,
              mut root_style: Query<&mut Style, With<Root>>,
              mut mouse_movement: EventReader<MouseMotion>| {
            let mut style = root_style.single_mut();
            let mouse_pos = trigger.event().pointer_location.position;
            mouse_movement.read().for_each(|motion| {
                let delta = motion.delta;
                //normalize the vector to 4 cardinal directions
                let vector = delta.normalize();
                match vector {
                    Vec2 { x: 1.0, y: 0.0 } => {}
                    Vec2 { x: -1.0, y: 0.0 } => {}
                    Vec2 { x: 0.0, y: 1.0 } => {}
                    Vec2 { x: 0.0, y: -1.0 } => {}
                    _ => {}
                }
            });
        },
    ); */
}
