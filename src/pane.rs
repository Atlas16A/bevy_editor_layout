use bevy::{ecs::entity, input::mouse::MouseMotion, prelude::*};

#[derive(Component)]
pub struct Root;

#[derive(Component)]
pub struct PaneRoot;

#[derive(Component)]
pub struct PaneArea;

#[derive(Component)]
pub struct PaneHeader;

#[derive(Component)]
pub struct PaneMenuButton;

#[derive(Component)]
pub struct Dragger;

const SCREEN_COLOR: Color = Color::hsla(0.0, 0.0, 0.09, 1.0);
const PANE_COLOR: Color = Color::hsla(0.0, 0.0, 0.25, 1.0);
const HEADER_COLOR: Color = Color::hsla(0.0, 0.0, 0.22, 1.0);
const BUTTON_COLOR: Color = Color::hsla(0.0, 0.0, 0.16, 1.0);
const ROUNDING: BorderRadius = BorderRadius {
    top_left: Val::Px(10.0),
    top_right: Val::Px(10.0),
    bottom_left: Val::Px(10.0),
    bottom_right: Val::Px(10.0),
};

pub fn spawn_pane(commands: &mut Commands, parent: Entity) {
    commands.entity(parent).with_children(|parent| {
        parent
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
            .with_children(|parent| {
                parent
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
                    .observe(
                        /* |trigger: Trigger<Pointer<DragOver>>,
                         drag_points: Query<(), With<Dragger>>| {
                            if drag_points.contains(trigger.event().dragged) {
                                println!("DragOver detected");
                            }
                        } */
                        |trigger: Trigger<Pointer<DragOver>>,
                         mut commands: Commands,
                         mut mouse_movement: EventReader<MouseMotion>,
                         mut root: Query<&mut Style, With<Root>>| {
                            let mouse_pos = trigger.event().pointer_location.position;
                            mouse_movement.read().for_each(|motion| {
                                let delta = motion.delta;
                                //normalize the vector to 4 cardinal directions
                                let vector = delta.normalize();
                                match vector {
                                    //Left
                                    Vec2 { x: 1.0, y: 0.0 } => {
                                        //Get the column/rows of the pane
                                        //Get the Style of the Root node and update the grid_template_columns
                                        let root_style = root.single_mut(); //This crashes? Why? It should have spawned the root node by now?
                                    }
                                    //Right
                                    Vec2 { x: -1.0, y: 0.0 } => {}
                                    //Up
                                    Vec2 { x: 0.0, y: 1.0 } => {}
                                    //Down
                                    Vec2 { x: 0.0, y: -1.0 } => {}
                                    _ => {}
                                }
                            });
                        },
                    )
                    .with_children(|parent| {
                        parent
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
                            .with_children(|parent| {
                                parent
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
                                    .insert(PaneMenuButton);
                            });
                    });
                //Dragging areas
                parent
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
                    });
                parent
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
                    });
                parent
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
                    });
                parent
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
                    });
                parent
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
                    });
                parent
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
                    });
                parent
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
                    });
                parent
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
                    });
            });
    });
}
