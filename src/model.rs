use std::sync::Arc;

use crate::{material::Material, mesh::Mesh};

#[derive(Clone)]
pub struct Model {
    pub mesh: Arc<Mesh>,
    pub material: Arc<Material>,
}
