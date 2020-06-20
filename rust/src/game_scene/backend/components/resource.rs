pub struct Resource {
    name: String
}

impl Resource {
    pub fn new(
        name: String
    ) -> Resource {
        Resource {
            name: name,
        }
    }
}