use legion::prelude::*;

use super::super::components::Demographic;

pub fn make_demographics(world: &mut World) {
    world.insert((), vec!(
        (Demographic::new(), ),
        (Demographic::new(), ),
    ));
}