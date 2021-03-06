use crate::database::AccessoryCategory;

#[derive(Debug, GraphQLObject)]
pub struct Accessory {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub image_link: Option<String>,
    pub category: AccessoryCategory,
}
