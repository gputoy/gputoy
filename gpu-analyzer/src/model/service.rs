use super::{Model, ModelCache, ModelKey};
use gpu_common::FilePath;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Default, Ord, PartialEq, PartialOrd, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ModelVersion {
    pub current: u32,
    pub patched: u32,
}

impl ModelVersion {
    /// Increase the patched version, indicating that there is a
    /// new change not yet applied to the model.
    pub fn new_patch_version(self, patched: u32) -> Self {
        Self {
            current: self.current,
            patched: self.patched.max(patched),
        }
    }

    /// Move the current version up to the patched version,
    /// indicating all changes have been applied to the model
    /// and the model is up to date.
    pub fn current_patched(self) -> Self {
        Self {
            current: self.patched,
            patched: self.patched,
        }
    }
}
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub struct ModelChangeText {
    pub range_offset: u32,
    pub range_length: u32,
    pub text: String,
}

#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ModelChangeChar {
    val: char,
    offset: u32,
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "dev", derive(Serialize, Deserialize))]
pub enum ModelChange {
    Char(ModelChangeChar),
    Text(ModelChangeText),
}

impl ModelChange {
    pub fn char(val: char, offset: u32) -> Self {
        Self::Char(ModelChangeChar { val, offset })
    }

    pub fn text(range_offset: u32, range_length: u32, text: String) -> Self {
        Self::Text(ModelChangeText {
            range_offset,
            range_length,
            text,
        })
    }
}

impl Model {
    pub fn apply_change(&mut self, change: ModelChange) {
        match change {
            ModelChange::Char(change) => self.data.insert(change.offset as usize, change.val),
            ModelChange::Text(change) => {
                self.data
                    .insert_str(change.range_offset as usize, &change.text);
            }
        }
    }
}

/// ModelCache service methods
///
/// These methods allow for runtime changes to text models. Used within
/// various service methods for the `Analyzer`, but primarily `model_sync`.
///
/// Enabled with the "service" feature
impl ModelCache {
    /// Push model change to analyzer.
    ///
    /// Does not apply the change to the text immediately, the changes are queued and
    /// must be commited with `apply_changes`.
    ///
    /// Returns new versioning information of affected model.
    pub fn push_change(
        &mut self,
        key: ModelKey,
        change: ModelChange,
        version_id: u32,
    ) -> crate::Result<ModelVersion> {
        match self.changes.get_mut(key) {
            Some(changes) => changes.push(change),
            None => {
                let new_changes = vec![change];
                self.changes.insert(key, new_changes);
            }
        }
        let version = self
            .versions
            .get(key)
            .copied()
            .unwrap_or_default()
            .new_patch_version(version_id);
        self.versions.insert(key, version);
        Ok(version)
    }

    /// Apply queued changes to model at path.
    pub fn apply_changes(&mut self, path: &FilePath) -> crate::Result<()> {
        let key = self.get_key(path)?;
        if self.is_dirty(key) {
            self._apply_changes(key)
        } else {
            Ok(())
        }
    }

    /// Apply queued changes to all models.
    pub fn apply_all_changes(&mut self) -> crate::Result<()> {
        let dirty_keys = self
            .changes
            .keys()
            .filter(|&k| self.is_dirty(k))
            .collect::<Vec<_>>();

        for key in dirty_keys {
            self._apply_changes(key)?;
        }
        Ok(())
    }

    /// Consume all queued changes for this model and apply in order.
    ///
    /// Only call on dirty files.
    fn _apply_changes(&mut self, key: ModelKey) -> crate::Result<()> {
        debug_assert!(self.is_dirty(key));

        if let Some(changes) = self.changes.insert(key, Vec::new()) {
            let model = self.get_model_mut(key)?;
            for change in changes {
                model.apply_change(change);
            }
        }
        let version = self.versions.get(key).copied().unwrap();
        self.versions.insert(key, version.current_patched());

        Ok(())
    }

    fn is_dirty(&self, key: ModelKey) -> bool {
        match self.versions.get(key) {
            Some(version) => version.current != version.patched,
            None => false,
        }
    }
}
