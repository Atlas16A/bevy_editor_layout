use bevy::prelude::*;

use crate::column_width_percentage_from_mouse;

#[derive(Component)]
pub struct RootPaneLayoutNode;

#[derive(Component)]
pub struct PaneRootNode;

#[derive(Component)]
pub struct PaneAreaNode;

#[derive(Component)]
pub struct PaneHeaderNode;

#[derive(Component)]
pub struct PaneMenuButtonNode;

#[derive(Component)]
pub struct PaneSplitterNode;

#[derive(Component)]
pub struct TopResizeBarNode;

#[derive(Component)]
pub struct BottomResizeBarNode;

#[derive(Component)]
pub struct LeftResizeBarNode;

#[derive(Component)]
pub struct RightResizeBarNode;

pub const SCREEN_COLOR: Color = Color::hsla(0.0, 0.0, 0.09, 1.0);
pub const PANE_COLOR: Color = Color::hsla(0.0, 0.0, 0.25, 1.0);
pub const HEADER_COLOR: Color = Color::hsla(0.0, 0.0, 0.22, 1.0);
pub const BUTTON_COLOR: Color = Color::hsla(0.0, 0.0, 0.16, 1.0);
pub const ROUNDING: BorderRadius = BorderRadius {
    top_left: Val::Px(10.0),
    top_right: Val::Px(10.0),
    bottom_left: Val::Px(10.0),
    bottom_right: Val::Px(10.0),
};

pub fn spawn_pane(
    commands: &mut Commands,
    root: Entity,
    placement_row: i16,
    placement_col: i16,
    col_span: u16,
    row_span: u16,
) {
    commands.entity(root).with_children(|parent| {
        parent
            .spawn(NodeBundle {
                //PANE ROOT
                style: Style {
                    display: Display::Grid,
                    grid_template_columns: vec![
                        GridTrack::px(15.0),
                        GridTrack::auto(),
                        GridTrack::px(15.0),
                    ],
                    grid_template_rows: vec![
                        GridTrack::px(15.0),
                        GridTrack::auto(),
                        GridTrack::px(15.0),
                    ],
                    align_content: AlignContent::Stretch,
                    justify_content: JustifyContent::Stretch,

                    overflow: Overflow::clip(),

                    grid_column: GridPlacement::start_span(placement_col, col_span),
                    grid_row: GridPlacement::start_span(placement_row, row_span),
                    ..Default::default()
                },

                ..Default::default()
            })
            .insert(PaneRootNode)
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
                    .insert(PaneAreaNode)
                    .observe(
                        |trigger: Trigger<Pointer<Over>>,
                         root: Query<Entity, With<RootPaneLayoutNode>>,
                         styles: Query<&mut Style>,
                         parents: Query<&Parent>| {
                            let pane_root = parents.get(trigger.entity()).unwrap().get();
                            let style = styles.get(pane_root).unwrap();
                            let grid_row = style.grid_row.get_start().unwrap();
                            let grid_col = style.grid_column.get_start().unwrap();
                            println!(
                                "Focused Pane at grid row: {}, grid col: {}",
                                grid_row, grid_col
                            );
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
                            .insert(PaneHeaderNode)
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
                                    .insert(PaneMenuButtonNode);
                            });
                    });
                //Dragging areas
                parent
                    .spawn(NodeBundle {
                        style: Style {
                            grid_row: GridPlacement::start(1),
                            grid_column: GridPlacement::start(1),
                            justify_items: JustifyItems::Stretch,
                            align_items: AlignItems::Stretch,
                            ..Default::default()
                        },
                        background_color: BackgroundColor(PANE_COLOR),
                        ..Default::default()
                    })
                    .insert(PaneSplitterNode)
                    .observe(|_trigger: Trigger<Pointer<Click>>| {
                        println!("Click detected 1x1");
                    });
                parent
                    .spawn(NodeBundle {
                        style: Style {
                            grid_row: GridPlacement::start(1),
                            grid_column: GridPlacement::start(3),
                            justify_items: JustifyItems::Stretch,
                            align_items: AlignItems::Stretch,
                            ..Default::default()
                        },
                        background_color: BackgroundColor(PANE_COLOR),
                        ..Default::default()
                    })
                    .insert(PaneSplitterNode)
                    .observe(|_trigger: Trigger<Pointer<Click>>| {
                        println!("Click detected 1x3");
                    });
                parent
                    .spawn(NodeBundle {
                        style: Style {
                            grid_row: GridPlacement::start(3),
                            grid_column: GridPlacement::start(1),
                            justify_items: JustifyItems::Stretch,
                            align_items: AlignItems::Stretch,
                            ..Default::default()
                        },
                        background_color: BackgroundColor(PANE_COLOR),
                        ..Default::default()
                    })
                    .insert(PaneSplitterNode)
                    .observe(|_trigger: Trigger<Pointer<Click>>| {
                        println!("Click detected 3x1");
                    });
                parent
                    .spawn(NodeBundle {
                        style: Style {
                            grid_row: GridPlacement::start(3),
                            grid_column: GridPlacement::start(3),

                            justify_items: JustifyItems::Stretch,
                            align_items: AlignItems::Stretch,
                            ..Default::default()
                        },
                        border_radius: BorderRadius {
                            top_left: Val::Px(-5.0),
                            top_right: Val::Px(-5.0),
                            bottom_left: Val::Px(-5.0),
                            bottom_right: Val::Px(-5.0),
                        },
                        background_color: BackgroundColor(PANE_COLOR),
                        ..Default::default()
                    })
                    .insert(PaneSplitterNode)
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
                                top: Val::Px(0.0),
                                bottom: Val::Px(2.0),
                            },
                            ..Default::default()
                        },
                        border_radius: BorderRadius {
                            top_left: Val::Px(0.0),
                            top_right: Val::Px(0.0),
                            bottom_left: Val::Px(5.0),
                            bottom_right: Val::Px(5.0),
                        },
                        background_color: BackgroundColor(PANE_COLOR),
                        ..Default::default()
                    })
                    .insert(TopResizeBarNode)
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
                                left: Val::Px(0.0),
                                right: Val::Px(2.0),
                                top: Val::Px(25.0),
                                bottom: Val::Px(25.0),
                            },
                            ..Default::default()
                        },
                        border_radius: BorderRadius {
                            top_left: Val::Px(0.0),
                            top_right: Val::Px(5.0),
                            bottom_left: Val::Px(0.0),
                            bottom_right: Val::Px(5.0),
                        },
                        background_color: BackgroundColor(PANE_COLOR),
                        ..Default::default()
                    })
                    .insert(LeftResizeBarNode)
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
                                left: Val::Px(2.0),
                                right: Val::Px(0.0),
                                top: Val::Px(25.0),
                                bottom: Val::Px(25.0),
                            },
                            ..Default::default()
                        },
                        border_radius: BorderRadius {
                            top_left: Val::Px(5.0),
                            top_right: Val::Px(0.0),
                            bottom_left: Val::Px(5.0),
                            bottom_right: Val::Px(0.0),
                        },
                        background_color: BackgroundColor(PANE_COLOR),
                        ..Default::default()
                    })
                    .insert(RightResizeBarNode)
                    .observe(
                        |trigger: Trigger<Pointer<Drag>>,
                         root: Query<Entity, With<RootPaneLayoutNode>>,
                         parents: Query<&Parent>,
                         mut styles: Query<&mut Style>,
                         window: Query<&Window>,
                         panes: Query<Entity, With<PaneRootNode>>| {
                            panes.iter().for_each(|pane| {
                                let trigger_pane = parents.get(trigger.entity()).unwrap().get();
                                let trigger_pane_style = styles.get(trigger_pane).unwrap();

                                //Check if the pane is not the pane holding the resize bar
                                if pane != trigger_pane {
                                    let root_style = styles.get(root.single()).unwrap();
                                    if trigger_pane_style.grid_column.get_end().unwrap() - 1
                                        != root_style.grid_template_columns.len() as i16
                                    {
                                    }
                                }

                                /* if styles
                                    .get(parents.get(trigger.entity()).unwrap().get())
                                    .unwrap()
                                    .grid_column
                                    .get_end()
                                    .unwrap()
                                    == styles
                                        .get(root.single())
                                        .unwrap()
                                        .grid_template_columns
                                        .len() as i16
                                    && styles
                                        .get(trigger.entity())
                                        .unwrap()
                                        .grid_column
                                        .get_start()
                                        .unwrap()
                                        != 3
                                {
                                    //Pane is at the right edge of the screen and the dragger is the left dragger
                                    if pane != parents.get(trigger.entity()).unwrap().get()
                                        && styles
                                            .get(pane)
                                            .unwrap()
                                            .grid_column
                                            .get_start()
                                            .unwrap()
                                            == (styles
                                                .get(parents.get(trigger.entity()).unwrap().get())
                                                .unwrap()
                                                .grid_column
                                                .get_start()
                                                .unwrap()
                                                + 1)
                                        && styles
                                            .get(parents.get(trigger.entity()).unwrap().get())
                                            .unwrap()
                                            .grid_column
                                            .get_start()
                                            .unwrap()
                                            == 0
                                    {
                                        //grow and shrink the columns corrosponding to the dragger and the mouse direction
                                        let new_width = column_width_percentage_from_mouse(
                                            trigger.event().pointer_location.position.x,
                                            window.single().width(),
                                        );
                                        let pane_start = styles
                                            .get(parents.get(trigger.entity()).unwrap().get())
                                            .unwrap()
                                            .left;
                                        println!("Pane Start: {:#?}", pane_start);
                                    }
                                } */
                            });
                        },
                    );
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
                                top: Val::Px(2.0),
                                bottom: Val::Px(0.0),
                            },

                            ..Default::default()
                        },
                        border_radius: BorderRadius {
                            top_left: Val::Px(5.0),
                            top_right: Val::Px(5.0),
                            bottom_left: Val::Px(0.0),
                            bottom_right: Val::Px(0.0),
                        },
                        background_color: BackgroundColor(PANE_COLOR),
                        ..Default::default()
                    })
                    .insert(BottomResizeBarNode)
                    .observe(|_trigger: Trigger<Pointer<Drag>>| {});
            });
    });
}
