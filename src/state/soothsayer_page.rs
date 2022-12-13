use crate::data::prelude::{Soothsayer, StarSign};

#[derive(PartialEq, Eq, Clone, serde:: Serialize, serde::Deserialize, Default)]
pub struct SoothsayerPage {
    pub star_sign: StarSign,
    pub soothsayer: Soothsayer,
}