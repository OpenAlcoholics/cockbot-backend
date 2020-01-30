use crate::database::Glass;

#[derive(Debug, GraphQLInputObject)]
pub struct GlassInput {
    name: String,
    estimated_size: Option<i32>,
    image_link: Option<String>,
}

impl Into<Glass> for GlassInput {
    fn into(self) -> Glass {
        Glass {
            id: -1,
            name: self.name,
            estimated_size: self.estimated_size,
            image_link: self.image_link,
        }
    }
}
