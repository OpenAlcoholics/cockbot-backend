#[derive(Debug, GraphQLObject)]
pub struct AccessoryCategoryQuery {
    pub id: Option<i32>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub image_link: Option<String>,
}
