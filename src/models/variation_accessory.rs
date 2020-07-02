use crate::models::{Accessory, Variation};

#[derive(Debug, GraphQLObject)]
pub struct VariationAccessory {
    pub variation: Variation,
    pub accessory: Accessory,
}
