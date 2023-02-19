use super::{Error, Location, Model, ModelKey};

#[cfg(feature = "service")]
use super::service::{ModelChange, ModelVersion};
use gpu_common::FilePath;

#[derive(Debug)]
pub struct ModelCache {
    pub(super) models: slotmap::HopSlotMap<ModelKey, Model>,
    pub(super) idents: naga::FastHashMap<FilePath, ModelKey>,
    #[cfg(feature = "service")]
    pub(crate) changes: slotmap::SparseSecondaryMap<ModelKey, Vec<ModelChange>>,
    #[cfg(feature = "service")]
    pub(crate) versions: slotmap::SecondaryMap<ModelKey, ModelVersion>,
}

impl ModelCache {
    pub fn from_files(files: gpu_common::Files) -> Self {
        // Allocate more space if the analyzer is running as a service
        // to account for new models
        #[cfg(not(feature = "service"))]
        let capacity = files.map.len();
        #[cfg(feature = "service")]
        let capacity = files.map.len() * 2;

        let mut models = slotmap::HopSlotMap::with_capacity_and_key(capacity);
        let mut idents = naga::FastHashMap::with_capacity_and_hasher(capacity, Default::default());
        let mut paths = slotmap::SecondaryMap::with_capacity(capacity);

        for (path, file) in files.map.into_iter() {
            let model = Model::from((path.clone(), file));
            let key: ModelKey = models.insert(model);
            idents.insert(path.clone(), key);
            paths.insert(key, path);
        }

        Self {
            models,
            idents,
            #[cfg(feature = "service")]
            changes: slotmap::SparseSecondaryMap::with_capacity(capacity),
            #[cfg(feature = "service")]
            versions: slotmap::SecondaryMap::with_capacity(capacity),
        }
    }

    pub fn capacity(&self) -> usize {
        self.models.capacity()
    }

    pub fn iter(&self) -> impl Iterator<Item = (ModelKey, &Model)> {
        self.models.iter()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = (ModelKey, &mut Model)> {
        self.models.iter_mut()
    }

    pub fn iter_idents(&self) -> impl Iterator<Item = (&FilePath, &ModelKey)> {
        self.idents.iter()
    }

    pub fn shaders(&self) -> impl Iterator<Item = (ModelKey, &Model)> {
        self.iter().filter(|(_, model)| model.extension.is_shader())
    }

    pub fn get_location(&self, location: &Location) -> Result<&str, Error> {
        Ok(self
            .get_model(location.model)?
            .value_in_range(location.range.clone()))
    }

    pub fn get_key(&self, path: &FilePath) -> Result<ModelKey, Error> {
        self.idents
            .get(path)
            .copied()
            .ok_or(Error::ModelKeyNotFound(path.clone()))
    }

    pub fn get_model(&self, key: ModelKey) -> Result<&Model, Error> {
        self.models.get(key).ok_or(Error::StaleKey(key))
    }

    pub fn get_model_mut(&mut self, key: ModelKey) -> Result<&mut Model, Error> {
        self.models.get_mut(key).ok_or(Error::StaleKey(key))
    }

    pub fn get_model_by_path(&self, path: &FilePath) -> Result<&Model, Error> {
        self.get_key(path).and_then(|key| self.get_model(key))
    }

    pub fn get_model_by_path_mut(&mut self, path: &FilePath) -> Result<&mut Model, Error> {
        self.get_key(path).and_then(|key| self.get_model_mut(key))
    }

    pub(crate) fn path(&self, key: ModelKey) -> Result<&FilePath, Error> {
        Ok(&self.get_model(key)?.path)
    }
}
