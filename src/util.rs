//! librecad model.

#[derive(Debug, Clone)]
pub struct Model3D {
    pub id: usize,
    pub name: String,
    pub path: String,
    pub objects: Vec<String>,
    pub modified: bool,
}

#[derive(Debug, Clone)]
pub struct Model {
    pub models: Vec<Model3D>,
    next_id: usize,
}

impl Default for Model {
    fn default() -> Self { Self { models: vec![], next_id: 1 } }
}

impl Model {
    pub fn new() -> Self { Self::default() }

    pub fn create(&mut self, name: &str, path: &str) -> usize {
        let id = self.next_id;
        self.next_id += 1;
        self.models.push(Model3D { id, name: name.into(), path: path.into(), objects: vec![], modified: false });
        id
    }

    pub fn add_object(&mut self, id: usize, obj: &str) -> bool {
        if let Some(m) = self.models.iter_mut().find(|m| m.id == id) {
            m.objects.push(obj.into());
            m.modified = true;
            true
        } else { false }
    }

    pub fn save(&mut self, id: usize) -> bool {
        if let Some(m) = self.models.iter_mut().find(|m| m.id == id) {
            m.modified = false;
            true
        } else { false }
    }

    pub fn export(&self, id: usize, format: &str) -> Option<String> {
        self.models.iter().find(|m| m.id == id).map(|m| format!("{}.{}", m.path, format))
    }

    pub fn close(&mut self, id: usize) -> bool {
        let before = self.models.len();
        self.models.retain(|m| m.id != id);
        self.models.len() < before
    }

    pub fn list(&self) -> &[Model3D] { &self.models }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn create_object_save_export() {
        let mut m = Model::new();
        let id = m.create("part1", "/tmp/part1");
        assert!(m.add_object(id, "cube"));
        assert!(m.save(id));
        assert!(m.export(id, "stl").unwrap().ends_with(".stl"));
        assert!(m.close(id));
    }
}
