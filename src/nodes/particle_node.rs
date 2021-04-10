use std::any::Any;

use nalgebra::Vector2;

use crate::core::{Cache, CacheIndex, Node};
use super::common::Particle;

pub fn particle_node_factory(cache: &mut Cache) -> Node {
    let mass = cache.insert(1.);
    let force = cache.insert(Vector2::new(1., 1.));
    let acceleration = cache.insert(None::<Vector2<f64>>);
    let velocity = cache.insert(None::<Vector2<f64>>);
    let position = cache.insert(None::<Vector2<f64>>);
    let particle = cache.insert(Particle::new());

    let mut inputs = Vec::new();
    inputs.push(mass);
    inputs.push(force);
    inputs.push(acceleration);
    inputs.push(velocity);
    inputs.push(position);

    let mut outputs = Vec::new();
    outputs.push(particle);

    Node::new(inputs, outputs, remove_all_cache)
        .with_compute(compute)
        .with_create_remove_input_cache(create_input_cache, remove_input_cache)
}

fn compute(inputs: &Vec<CacheIndex>, outputs: &Vec<CacheIndex>, cache: &mut Cache) {
    *cache.get_mut::<Particle>(&outputs[0]).unwrap().get_mut_mass() = cache.get::<f64>(&inputs[0]).unwrap().clone();

    loop {
        if let Some(set_position) = cache.get::<Option<Vector2<f64>>>(&inputs[4]).unwrap() {
            *cache.get_mut::<Particle>(&outputs[0]).unwrap().get_mut_position() = set_position.clone();
            break;
        }

        if let Some(set_velocity) = cache.get::<Option<Vector2<f64>>>(&inputs[3]).unwrap() {
            *cache.get_mut::<Particle>(&outputs[0]).unwrap().get_mut_velocity() = set_velocity.clone();
            break;
        }
        
        if let Some(set_acceleration) = cache.get::<Option<Vector2<f64>>>(&inputs[2]).unwrap() {
            *cache.get_mut::<Particle>(&outputs[0]).unwrap().get_mut_acceleration() = set_acceleration.clone();
            break;
        }

        let force = cache.get::<Vector2<f64>>(&inputs[1]).unwrap().clone();
        cache.get_mut::<Particle>(&outputs[0]).unwrap().apply_force(force);
        break;
    }

    cache.get_mut::<Particle>(&outputs[0]).unwrap().update();

    println!("{}", cache.get::<Particle>(&outputs[0]).unwrap().get_position());
}

fn remove_input_cache(port: usize, cache_index: CacheIndex, cache: &mut Cache) {
    match port {
        0 => cache.remove::<Vector2<f64>>(&cache_index),
        1 => cache.remove::<f64>(&cache_index),
        _ => (),
    }
}

fn create_input_cache(port: usize, cache: &mut Cache) -> Option<CacheIndex> {
    match port {
        0 => Some(cache.insert(Vector2::new(0., 0.))),
        1 => Some(cache.insert(1.)),
        _ => None,
    }
}

fn remove_all_cache(inputs: &Vec<CacheIndex>, outputs: &Vec<CacheIndex>, cache: &mut Cache) {
    cache.remove::<Vector2<f64>>(&inputs[0]);
    cache.remove::<f64>(&inputs[1]);
    cache.remove::<Vector2<f64>>(&outputs[0]);
    cache.remove::<Vector2<f64>>(&outputs[1]);
    cache.remove::<Vector2<f64>>(&outputs[2]);
}
