use crate::database::{Accessory, AccessoryCategory};

#[derive(Debug, GraphQLInputObject)]
pub struct AccessoryInput {
    pub name: String,
    pub description: Option<String>,
    pub image_link: Option<String>,
    pub category_id: i32,
}

impl Into<Accessory> for AccessoryInput {
    fn into(self) -> Accessory {
        Accessory {
            id: -1,
            name: self.name,
            description: self.description,
            image_link: self.image_link,
            category_id: self.category_id,
        }
    }
}
