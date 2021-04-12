use nalgebra::Vector2;

use super::common::Particle;
use crate::core::{Cache, CacheIndex, Node};

// Input ports
const MASS: usize = 0;
const FORCE: usize = 1;
const ACCELERATION: usize = 2;
const VELOCITY: usize = 3;
const POSITION: usize = 4;
const USE_ACCELERATION: usize = 5;
const USE_VELOCITY: usize = 6;
const USE_POSITION: usize = 7;

// Output ports
const PARTICLE: usize = 0;

pub fn particle_node_factory(cache: &mut Cache) -> Node {
    let mass = cache.insert(1.);
    let force = cache.insert(Vector2::new(0., 0.));
    let acceleration = cache.insert(Vector2::new(0., 0.));
    let velocity = cache.insert(Vector2::new(0., 0.));
    let position = cache.insert(Vector2::new(0., 0.));
    let use_acceleration = cache.insert(true);
    let use_velocity = cache.insert(false);
    let use_position = cache.insert(false);
    let particle = cache.insert(Particle::new());

    let mut inputs = Vec::new();
    inputs.push(mass);
    inputs.push(force);
    inputs.push(acceleration);
    inputs.push(velocity);
    inputs.push(position);
    inputs.push(use_acceleration);
    inputs.push(use_velocity);
    inputs.push(use_position);

    let mut outputs = Vec::new();
    outputs.push(particle);

    Node::new(inputs, outputs, remove_all_cache)
        .with_compute(compute)
        .with_create_remove_input_cache(create_input_cache, remove_input_cache)
}

fn compute(inputs: &Vec<CacheIndex>, outputs: &Vec<CacheIndex>, cache: &mut Cache) {
    *cache
        .get_mut::<Particle>(&outputs[PARTICLE])
        .unwrap()
        .get_mut_mass() = cache.get::<f64>(&inputs[MASS]).unwrap().clone();

    if *cache.get::<bool>(&inputs[USE_POSITION]).unwrap() {
        println!("using pos");
        let set_position = cache.get::<Vector2<f64>>(&inputs[POSITION]).unwrap();
        *cache
            .get_mut::<Particle>(&outputs[PARTICLE])
            .unwrap()
            .get_mut_position() = set_position.clone();
    } else if *cache.get::<bool>(&inputs[USE_VELOCITY]).unwrap() {
        println!("using vel");
        let set_velocity = cache.get::<Vector2<f64>>(&inputs[VELOCITY]).unwrap();
        *cache
            .get_mut::<Particle>(&outputs[PARTICLE])
            .unwrap()
            .get_mut_velocity() = set_velocity.clone();
    } else if *cache.get::<bool>(&inputs[USE_ACCELERATION]).unwrap() {
        println!("using acc");
        let set_acceleration = cache.get::<Vector2<f64>>(&inputs[ACCELERATION]).unwrap();
        *cache
            .get_mut::<Particle>(&outputs[PARTICLE])
            .unwrap()
            .get_mut_acceleration() = set_acceleration.clone();
    } else {
        println!("using force");
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

    println!(
        "{}",
        cache
            .get::<Particle>(&outputs[PARTICLE])
            .unwrap()
            .get_position()
    );
}

fn remove_input_cache(node: &Node, port: usize, cache: &mut Cache) {
    match port {
        MASS => cache.remove::<f64>(&node.get_inputs()[MASS]),
        FORCE => cache.remove::<Vector2<f64>>(&node.get_inputs()[FORCE]),
        ACCELERATION => {
            *cache
                .get_mut::<bool>(&node.get_inputs()[USE_ACCELERATION])
                .unwrap() = true;
            cache.remove::<Vector2<f64>>(&node.get_inputs()[ACCELERATION])
        }
        VELOCITY => {
            *cache
                .get_mut::<bool>(&node.get_inputs()[USE_VELOCITY])
                .unwrap() = true;
            cache.remove::<Vector2<f64>>(&node.get_inputs()[VELOCITY])
        }
        POSITION => {
            *cache
                .get_mut::<bool>(&node.get_inputs()[USE_POSITION])
                .unwrap() = true;
            cache.remove::<Vector2<f64>>(&node.get_inputs()[POSITION])
        }
        _ => (),
    }
}

fn create_input_cache(node: &Node, port: usize, cache: &mut Cache) -> Option<CacheIndex> {
    match port {
        MASS => Some(cache.insert(1.)),
        FORCE => Some(cache.insert(Vector2::new(0., 0.))),
        ACCELERATION => {
            *cache
                .get_mut::<bool>(&node.get_inputs()[USE_ACCELERATION])
                .unwrap() = false;
            Some(cache.insert(Vector2::new(0., 0.)))
        }
        VELOCITY => {
            *cache
                .get_mut::<bool>(&node.get_inputs()[USE_VELOCITY])
                .unwrap() = false;
            Some(cache.insert(Vector2::new(0., 0.)))
        }
        POSITION => {
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
    cache.remove::<Vector2<f64>>(&inputs[ACCELERATION]);
    cache.remove::<Vector2<f64>>(&inputs[VELOCITY]);
    cache.remove::<Vector2<f64>>(&inputs[POSITION]);
    cache.remove::<bool>(&inputs[USE_ACCELERATION]);
    cache.remove::<bool>(&inputs[USE_POSITION]);
    cache.remove::<bool>(&inputs[USE_VELOCITY]);
    cache.remove::<Particle>(&outputs[PARTICLE]);
}
