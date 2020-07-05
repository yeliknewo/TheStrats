use legion::prelude::*;

use super::super::components::Resource;

pub fn make_resources(world: &mut World) {
    world.insert((), vec!(
        (Resource::new("Food".into()), ),
        (Resource::new("Horse".into()), ),
    ));
}