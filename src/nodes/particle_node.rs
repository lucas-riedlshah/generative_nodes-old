use std::{cell::RefCell, rc::Rc};

use druid::{
    widget::{Container, CrossAxisAlignment, Flex, Label},
    Color, Widget, WidgetExt,
};
use nalgebra::Vector2;

use super::common::Particle;
use crate::{core::{App, Cache, CacheIndex, Direction, Node, Port}, gui::{graph_widget::PortDirection, node_widget::NodeWidget, port_widget::PortWidget}};

// Inputs
const MASS: usize = 0;
const FORCE: usize = 1;
const SET_ACCELERATION: usize = 2;
const SET_VELOCITY: usize = 3;
const SET_POSITION: usize = 4;
// Outputs
const PARTICLE: usize = 5;
const ACCELERATION: usize = 6;
const VELOCITY: usize = 7;
const POSITION: usize = 8;
// Other
const USE_ACCELERATION: usize = 9;
const USE_VELOCITY: usize = 10;
const USE_POSITION: usize = 11;

pub fn node_factory(cache: &mut Cache) -> Node {
    let mass = cache.insert(1.);
    let force = cache.insert(Vector2::new(0., 0.));
    let set_acceleration = cache.insert(Vector2::new(0., 0.));
    let set_velocity = cache.insert(Vector2::new(0., 0.));
    let set_position = cache.insert(Vector2::new(0., 0.));
    let use_acceleration = cache.insert(false);
    let use_velocity = cache.insert(false);
    let use_position = cache.insert(false);

    let particle = cache.insert(Particle::new());
    let acceleration = cache.insert(Vector2::new(0., 0.));
    let velocity = cache.insert(Vector2::new(0., 0.));
    let position = cache.insert(Vector2::new(0., 0.));

    let mut ports = Vec::new();
    ports.push(Port::new(mass, Direction::Input));
    ports.push(Port::new(force, Direction::Input));
    ports.push(Port::new(set_acceleration, Direction::Input));
    ports.push(Port::new(set_velocity, Direction::Input));
    ports.push(Port::new(set_position, Direction::Input));
    ports.push(Port::new(particle, Direction::Output));
    ports.push(Port::new(acceleration, Direction::Output));
    ports.push(Port::new(velocity, Direction::Output));
    ports.push(Port::new(position, Direction::Output));
    // TODO: these 3 "inputs" could probably just be replaced by the is_connected parameter of the [Port].
    ports.push(Port::new(use_acceleration, Direction::Input));
    ports.push(Port::new(use_velocity, Direction::Input));
    ports.push(Port::new(use_position, Direction::Input));


    Node::new(ports, remove_all_cache)
        .with_compute(compute)
        .with_create_remove_input_cache(disconnect, connect)
}

fn compute(ports: &Vec<Port>, cache: &mut Cache) {
    *cache
        .get_mut::<Particle>(&ports[PARTICLE].get_cache_index())
        .unwrap()
        .get_mut_mass() = cache.get::<f64>(&ports[MASS].get_cache_index()).unwrap().clone();

    if *cache.get::<bool>(&ports[USE_POSITION].get_cache_index()).unwrap() {
        let set_position = cache.get::<Vector2<f64>>(&ports[SET_POSITION].get_cache_index()).unwrap();
        *cache
            .get_mut::<Particle>(&ports[PARTICLE].get_cache_index())
            .unwrap()
            .get_mut_position() = set_position.clone();
    } else if *cache.get::<bool>(&ports[USE_VELOCITY].get_cache_index()).unwrap() {
        let set_velocity = cache.get::<Vector2<f64>>(&ports[SET_VELOCITY].get_cache_index()).unwrap();
        *cache
            .get_mut::<Particle>(&ports[PARTICLE].get_cache_index())
            .unwrap()
            .get_mut_velocity() = set_velocity.clone();
    } else if *cache.get::<bool>(&ports[USE_ACCELERATION].get_cache_index()).unwrap() {
        let set_acceleration = cache
            .get::<Vector2<f64>>(&ports[SET_ACCELERATION].get_cache_index())
            .unwrap();
        *cache
            .get_mut::<Particle>(&ports[PARTICLE].get_cache_index())
            .unwrap()
            .get_mut_acceleration() = set_acceleration.clone();
    } else {
        let force = cache.get::<Vector2<f64>>(&ports[FORCE].get_cache_index()).unwrap().clone();
        cache
            .get_mut::<Particle>(&ports[PARTICLE].get_cache_index())
            .unwrap()
            .apply_force(force);
    }

    cache
        .get_mut::<Particle>(&ports[PARTICLE].get_cache_index())
        .unwrap()
        .update();

    let particle = cache.get::<Particle>(&ports[PARTICLE].get_cache_index()).unwrap();
    let acceleration = *particle.get_acceleration();
    let velocity = *particle.get_velocity();
    let position = *particle.get_position();

    *cache
        .get_mut::<Vector2<f64>>(&ports[ACCELERATION].get_cache_index())
        .unwrap() = acceleration;
    *cache.get_mut::<Vector2<f64>>(&ports[VELOCITY].get_cache_index()).unwrap() = velocity;
    *cache.get_mut::<Vector2<f64>>(&ports[POSITION].get_cache_index()).unwrap() = position;
}

fn connect(node: &Node, port: usize, cache: &mut Cache) {
    match port {
        MASS => cache.remove::<f64>(&node.get_ports()[MASS].get_cache_index()),
        FORCE => cache.remove::<Vector2<f64>>(&node.get_ports()[FORCE].get_cache_index()),
        SET_ACCELERATION => {
            *cache
                .get_mut::<bool>(&node.get_ports()[USE_ACCELERATION].get_cache_index())
                .unwrap() = true;
            cache.remove::<Vector2<f64>>(&node.get_ports()[SET_ACCELERATION].get_cache_index())
        }
        SET_VELOCITY => {
            *cache
                .get_mut::<bool>(&node.get_ports()[USE_VELOCITY].get_cache_index())
                .unwrap() = true;
            cache.remove::<Vector2<f64>>(&node.get_ports()[SET_VELOCITY].get_cache_index())
        }
        SET_POSITION => {
            *cache
                .get_mut::<bool>(&node.get_ports()[USE_POSITION].get_cache_index())
                .unwrap() = true;
            cache.remove::<Vector2<f64>>(&node.get_ports()[SET_POSITION].get_cache_index())
        }
        _ => (),
    }
}

fn disconnect(node: &Node, port: usize, cache: &mut Cache) -> Option<CacheIndex> {
    match port {
        MASS => Some(cache.insert(1.)),
        FORCE => Some(cache.insert(Vector2::new(0., 0.))),
        SET_ACCELERATION => {
            *cache
                .get_mut::<bool>(&node.get_ports()[USE_ACCELERATION].get_cache_index())
                .unwrap() = false;
            Some(cache.insert(Vector2::new(0., 0.)))
        }
        SET_VELOCITY => {
            *cache
                .get_mut::<bool>(&node.get_ports()[USE_VELOCITY].get_cache_index())
                .unwrap() = false;
            Some(cache.insert(Vector2::new(0., 0.)))
        }
        SET_POSITION => {
            *cache
                .get_mut::<bool>(&node.get_ports()[USE_POSITION].get_cache_index())
                .unwrap() = false;
            Some(cache.insert(Vector2::new(0., 0.)))
        }
        _ => None,
    }
}

fn remove_all_cache(ports: &Vec<Port>, cache: &mut Cache) {
    cache.remove::<f64>(&ports[MASS].get_cache_index());
    cache.remove::<Vector2<f64>>(&ports[FORCE].get_cache_index());
    cache.remove::<Vector2<f64>>(&ports[SET_ACCELERATION].get_cache_index());
    cache.remove::<Vector2<f64>>(&ports[SET_VELOCITY].get_cache_index());
    cache.remove::<Vector2<f64>>(&ports[SET_POSITION].get_cache_index());
    cache.remove::<bool>(&ports[USE_ACCELERATION].get_cache_index());
    cache.remove::<bool>(&ports[USE_POSITION].get_cache_index());
    cache.remove::<bool>(&ports[USE_VELOCITY].get_cache_index());
    cache.remove::<Particle>(&ports[PARTICLE].get_cache_index());
    cache.remove::<Vector2<f64>>(&ports[ACCELERATION].get_cache_index());
    cache.remove::<Vector2<f64>>(&ports[VELOCITY].get_cache_index());
    cache.remove::<Vector2<f64>>(&ports[POSITION].get_cache_index());
}

pub fn widget_factory(index: usize) -> Box<dyn Widget<Rc<RefCell<App>>>> {
    Box::new(NodeWidget::new(
        Container::new(
            Flex::column()
                .with_child(Label::new("Particle"))
                .with_spacer(5.)
                .with_child(
                    // Inputs
                    Flex::column()
                        .cross_axis_alignment(CrossAxisAlignment::Start)
                        .with_child(
                            Flex::row()
                                .with_child(PortWidget::new(
                                    index,
                                    MASS,
                                    PortDirection::Input,
                                    PortWidget::F64,
                                ))
                                .with_spacer(5.)
                                // .with_child(TextBox::new().lens(StringInputLens("string"))),
                                .with_child(Label::new("Mass")),
                        )
                        .with_spacer(5.)
                        .with_child(
                            Flex::row()
                                .with_child(PortWidget::new(
                                    index,
                                    FORCE,
                                    PortDirection::Input,
                                    PortWidget::VECTOR2F64,
                                ))
                                .with_spacer(5.)
                                // .with_child(TextBox::new().lens(StringInputLens("string"))),
                                .with_child(Label::new("Force")),
                        )
                        .with_spacer(5.)
                        .with_child(
                            Flex::row()
                                .with_child(PortWidget::new(
                                    index,
                                    SET_POSITION,
                                    PortDirection::Input,
                                    PortWidget::VECTOR2F64,
                                ))
                                .with_spacer(5.)
                                // .with_child(TextBox::new().lens(StringInputLens("string"))),
                                .with_child(Label::new("Set Position")),
                        )
                        .with_spacer(5.)
                        .with_child(
                            Flex::row()
                                .with_child(PortWidget::new(
                                    index,
                                    SET_VELOCITY,
                                    PortDirection::Input,
                                    PortWidget::VECTOR2F64,
                                ))
                                .with_spacer(5.)
                                // .with_child(TextBox::new().lens(StringInputLens("string"))),
                                .with_child(Label::new("Set Velocity")),
                        )
                        .with_spacer(5.)
                        .with_child(
                            Flex::row()
                                .with_child(PortWidget::new(
                                    index,
                                    SET_ACCELERATION,
                                    PortDirection::Input,
                                    PortWidget::VECTOR2F64,
                                ))
                                .with_spacer(5.)
                                // .with_child(TextBox::new().lens(StringInputLens("string"))),
                                .with_child(Label::new("Set Acceleration")),
                        )
                        .expand_width(),
                )
                .with_spacer(5.)
                .with_child(
                    // Outputs
                    Flex::column()
                        .cross_axis_alignment(CrossAxisAlignment::End)
                        .with_child(
                            Flex::row()
                                .with_child(Label::new("Particle"))
                                .with_spacer(5.)
                                .with_child(PortWidget::new(
                                    index,
                                    PARTICLE,
                                    PortDirection::Output,
                                    PortWidget::PARTICLE,
                                )),
                        )
                        .with_spacer(5.)
                        .with_child(
                            Flex::row()
                                .with_child(Label::new("Acceleration"))
                                .with_spacer(5.)
                                .with_child(PortWidget::new(
                                    index,
                                    ACCELERATION,
                                    PortDirection::Output,
                                    PortWidget::VECTOR2F64,
                                )),
                        )
                        .with_spacer(5.)
                        .with_child(
                            Flex::row()
                                .with_child(Label::new("Velocity"))
                                .with_spacer(5.)
                                .with_child(PortWidget::new(
                                    index,
                                    VELOCITY,
                                    PortDirection::Output,
                                    PortWidget::VECTOR2F64,
                                )),
                        )
                        .with_spacer(5.)
                        .with_child(
                            Flex::row()
                                .with_child(Label::new("Position"))
                                .with_spacer(5.)
                                .with_child(PortWidget::new(
                                    index,
                                    POSITION,
                                    PortDirection::Output,
                                    PortWidget::VECTOR2F64,
                                )),
                        )
                        .expand_width(),
                )
                .fix_width(200.)
                .padding(5.),
        )
        .rounded(10.)
        .background(Color::rgba8(50, 50, 50, 230))
        .border(Color::rgb8(25, 25, 25), 1.),
    ))
}
