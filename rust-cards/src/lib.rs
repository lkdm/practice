//! Entity Component System (ECS)
//!
//! ECS is composed of three core concepts:
//! 1. **Entities** – Unique IDs representing game objects.
//! 2. **Components** – Plain data attached to entities to define their properties.
//! 3. **Systems** – Logic that runs on entities with specific components.
//!
//! # Systems
//! Systems can be created from functions whose parameters implement [`SystemParam`].
//! - [`Query<&Component>`]: Gives access to the internals of a component

use bevy_ecs::prelude::*;
use bevy_ecs::system::*;

#[derive(Component)]
pub struct Cost;

#[derive(Component)]
pub struct Card {
    pub name: String,
    pub cost: Cost,
}

#[derive(Component)]
pub enum CardKind {
    Resource,
}

pub trait CardBehavior: Send + Sync {
    fn on_play(&self, entity: Entity, world: &mut World);
    fn on_turn_start(&self, entity: Entity, world: &mut World);
}

#[derive(Component)]
pub struct Behaviours(pub Vec<Box<dyn CardBehavior>>);

#[derive(Component)]
pub struct Health {
    current: u32,
}

#[derive(Component)]
pub struct Flammable;

pub fn fire_damage_system(mut query: Query<&mut Health, With<Flammable>>) {
    query.iter_mut().for_each(|mut health| health.current -= 5)
}
