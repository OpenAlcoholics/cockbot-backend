use crate::database::AccessoryCategory;

#[derive(Debug, GraphQLInputObject)]
pub struct AccessoryCategoryInput {
    pub name: String,
    pub description: Option<String>,
    pub image_link: Option<String>,
}

impl Into<AccessoryCategory> for AccessoryCategoryInput {
    fn into(self) -> AccessoryCategory {
        AccessoryCategory {
            id: -1,
            name: self.name,
            description: self.description,
            image_link: self.image_link,
        }
    }
}
