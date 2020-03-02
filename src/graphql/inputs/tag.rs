use crate::database::Tag;

#[derive(Debug, GraphQLInputObject)]
pub struct TagInput {
    name: String,
    description: Option<String>,
}

impl Into<Tag> for TagInput {
    fn into(self) -> Tag {
        Tag {
            id: String::new(),
            name: self.name,
            description: self.description,
        }
    }
}
