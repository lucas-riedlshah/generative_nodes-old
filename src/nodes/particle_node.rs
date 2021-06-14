use std::{cell::RefCell, rc::Rc};

use druid::{
    widget::{Container, CrossAxisAlignment, Flex, Label},
    Color, Widget, WidgetExt,
};
use nalgebra::Vector2;

use super::common::Particle;
use crate::{
    core::{App, Cache, CacheIndex, Node},
    gui::{graph_widget::PortDirection, node_widget::NodeWidget, port_widget::PortWidget},
};

// Input ports
const MASS: usize = 0;
const FORCE: usize = 1;
const SET_ACCELERATION: usize = 2;
const SET_VELOCITY: usize = 3;
const SET_POSITION: usize = 4;
const USE_ACCELERATION: usize = 5;
const USE_VELOCITY: usize = 6;
const USE_POSITION: usize = 7;

// Output ports
const PARTICLE: usize = 0;
const ACCELERATION: usize = 1;
const VELOCITY: usize = 2;
const POSITION: usize = 3;

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

    let mut inputs = Vec::new();
    inputs.push(mass);
    inputs.push(force);
    inputs.push(set_acceleration);
    inputs.push(set_velocity);
    inputs.push(set_position);
    inputs.push(use_acceleration);
    inputs.push(use_velocity);
    inputs.push(use_position);

    let mut outputs = Vec::new();
    outputs.push(particle);
    outputs.push(acceleration);
    outputs.push(velocity);
    outputs.push(position);

    Node::new(inputs, outputs, remove_all_cache)
        .with_compute(compute)
        .with_create_remove_input_cache(disconnect, connect)
}

fn compute(inputs: &Vec<CacheIndex>, outputs: &Vec<CacheIndex>, cache: &mut Cache) {
    *cache
        .get_mut::<Particle>(&outputs[PARTICLE])
        .unwrap()
        .get_mut_mass() = cache.get::<f64>(&inputs[MASS]).unwrap().clone();

    if *cache.get::<bool>(&inputs[USE_POSITION]).unwrap() {
        let set_position = cache.get::<Vector2<f64>>(&inputs[SET_POSITION]).unwrap();
        *cache
            .get_mut::<Particle>(&outputs[PARTICLE])
            .unwrap()
            .get_mut_position() = set_position.clone();
    } else if *cache.get::<bool>(&inputs[USE_VELOCITY]).unwrap() {
        let set_velocity = cache.get::<Vector2<f64>>(&inputs[SET_VELOCITY]).unwrap();
        *cache
            .get_mut::<Particle>(&outputs[PARTICLE])
            .unwrap()
            .get_mut_velocity() = set_velocity.clone();
    } else if *cache.get::<bool>(&inputs[USE_ACCELERATION]).unwrap() {
        let set_acceleration = cache
            .get::<Vector2<f64>>(&inputs[SET_ACCELERATION])
            .unwrap();
        *cache
            .get_mut::<Particle>(&outputs[PARTICLE])
            .unwrap()
            .get_mut_acceleration() = set_acceleration.clone();
    } else {
        let force = cache.get::<Vector2<f64>>(&inputs[FORCE]).unwrap().clone();
        cache
            .get_mut::<Particle>(&outputs[PARTICLE])
            .unwrap()
            .apply_force(force);
    }

    cache
        .get_mut::<Particle>(&outputs[PARTICLE])
        .unwrap()
        .update();

    let particle = cache.get::<Particle>(&outputs[PARTICLE]).unwrap();
    let acceleration = *particle.get_acceleration();
    let velocity = *particle.get_velocity();
    let position = *particle.get_position();

    *cache
        .get_mut::<Vector2<f64>>(&outputs[ACCELERATION])
        .unwrap() = acceleration;
    *cache.get_mut::<Vector2<f64>>(&outputs[VELOCITY]).unwrap() = velocity;
    *cache.get_mut::<Vector2<f64>>(&outputs[POSITION]).unwrap() = position;
}

fn connect(node: &Node, port: usize, cache: &mut Cache) {
    match port {
        MASS => cache.remove::<f64>(&node.get_inputs()[MASS]),
        FORCE => cache.remove::<Vector2<f64>>(&node.get_inputs()[FORCE]),
        SET_ACCELERATION => {
            *cache
                .get_mut::<bool>(&node.get_inputs()[USE_ACCELERATION])
                .unwrap() = true;
            cache.remove::<Vector2<f64>>(&node.get_inputs()[SET_ACCELERATION])
        }
        SET_VELOCITY => {
            *cache
                .get_mut::<bool>(&node.get_inputs()[USE_VELOCITY])
                .unwrap() = true;
            cache.remove::<Vector2<f64>>(&node.get_inputs()[SET_VELOCITY])
        }
        SET_POSITION => {
            *cache
                .get_mut::<bool>(&node.get_inputs()[USE_POSITION])
                .unwrap() = true;
            cache.remove::<Vector2<f64>>(&node.get_inputs()[SET_POSITION])
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
                .get_mut::<bool>(&node.get_inputs()[USE_ACCELERATION])
                .unwrap() = false;
            Some(cache.insert(Vector2::new(0., 0.)))
        }
        SET_VELOCITY => {
            *cache
                .get_mut::<bool>(&node.get_inputs()[USE_VELOCITY])
                .unwrap() = false;
            Some(cache.insert(Vector2::new(0., 0.)))
        }
        SET_POSITION => {
            *cache
                .get_mut::<bool>(&node.get_inputs()[USE_POSITION])
                .unwrap() = false;
            Some(cache.insert(Vector2::new(0., 0.)))
        }
        _ => None,
    }
}

fn remove_all_cache(inputs: &Vec<CacheIndex>, outputs: &Vec<CacheIndex>, cache: &mut Cache) {
    cache.remove::<f64>(&inputs[MASS]);
    cache.remove::<Vector2<f64>>(&inputs[FORCE]);
    cache.remove::<Vector2<f64>>(&inputs[SET_ACCELERATION]);
    cache.remove::<Vector2<f64>>(&inputs[SET_VELOCITY]);
    cache.remove::<Vector2<f64>>(&inputs[SET_POSITION]);
    cache.remove::<bool>(&inputs[USE_ACCELERATION]);
    cache.remove::<bool>(&inputs[USE_POSITION]);
    cache.remove::<bool>(&inputs[USE_VELOCITY]);
    cache.remove::<Particle>(&outputs[PARTICLE]);
    cache.remove::<Vector2<f64>>(&outputs[ACCELERATION]);
    cache.remove::<Vector2<f64>>(&outputs[VELOCITY]);
    cache.remove::<Vector2<f64>>(&outputs[POSITION]);
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
                .fix_width(150.)
                .padding(5.),
        )
        .rounded(10.)
        .background(Color::rgba8(50, 50, 50, 230))
        .border(Color::rgb8(25, 25, 25), 1.),
    ))
}
