use crate::model::{ModelChange, ModelChangeText, ModelVersion};
use gpu_common::FilePath;

impl crate::Analyzer {
    pub fn handle_file_delta_char(
        &mut self,
        path: &gpu_common::FilePath,
        ch: char,
        offset: u32,
        version_id: u32,
    ) -> crate::Result<ModelVersion> {
        let key = self.model_cache.get_key(path)?;
        self.model_cache
            .push_change(key, ModelChange::char(ch, offset), version_id)
    }

    pub fn handle_file_delta(
        &mut self,
        path: &gpu_common::FilePath,
        changes: Vec<ModelChangeText>,
        version_id: u32,
        _is_flush: bool,
    ) -> crate::Result<ModelVersion> {
        let key = self.model_cache.get_key(path)?;
        for change in changes {
            self.model_cache
                .push_change(key, ModelChange::Text(change), version_id)?;
        }

        Ok(*self.model_cache.versions.get(key).unwrap())
    }

    pub fn commit_changes(&mut self, file_path: Option<&FilePath>) -> crate::Result<()> {
        match file_path {
            Some(path) => self.model_cache.apply_changes(path),
            None => self.model_cache.apply_all_changes(),
        }
    }
}
