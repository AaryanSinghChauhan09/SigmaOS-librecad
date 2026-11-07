//! librecad model — 2D CAD.

#[derive(Debug, Clone)]
pub struct Entity {
    pub name: String,
    pub type_: String,
    pub layer: String,
}

#[derive(Debug, Clone)]
pub struct Layer {
    pub name: String,
    pub visible: bool,
    pub locked: bool,
}

#[derive(Debug, Clone)]
pub struct Model {
    pub layers: Vec<Layer>,
    pub entities: Vec<Entity>,
}

impl Default for Model {
    fn default() -> Self { Self {
        layers: vec![
            Layer { name: "0".into(), visible: true, locked: false },
        ],
        entities: vec![],
    } }
}

impl Model {
    pub fn new() -> Self { Self::default() }

    pub fn add_layer(&mut self, layer: Layer) { self.layers.push(layer); }

    pub fn add_entity(&mut self, entity: Entity) -> bool {
        if self.layers.iter().any(|l| l.name == entity.layer) {
            self.entities.push(entity); true
        } else { false }
    }

    pub fn list_entities(&self) -> &[Entity] { &self.entities }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn layer_entity() {
        let mut l = Model::new();
        assert!(l.add_entity(Entity { name: "line1".into(), type_: "line".into(), layer: "0".into() }));
        assert_eq!(l.list_entities().len(), 1);
    }
}
