use crate::{
    analyzer::{
        ExportProcess, ImportProcess, LinkerProcess, ParserProcess, Process, ProcessProgress,
    },
    model::ModelKey,
};

dispatch_store! {
    linker_results: LinkerProcess,
    export_results: ExportProcess,
    import_results: ImportProcess,
    parser_results: ParserProcess,
}

impl DispatchStorage {
    /// Get reference to Proc's store
    pub fn get<Proc: Process>(&self) -> &Proc::Store
    where
        Self: StoreProvider<Proc>,
    {
        <Self as StoreProvider<Proc>>::get_store(self)
    }

    /// Get mutable reference to Proc's store
    pub fn get_mut<Proc: Process>(&mut self) -> &mut Proc::Store
    where
        Self: StoreProvider<Proc>,
    {
        <Self as StoreProvider<Proc>>::get_store_mut(self)
    }

    /// Get value in Proc's store at key
    pub fn get_result<'a, Proc: Process>(&'a self, key: ModelKey) -> crate::Result<&'a Proc::Result>
    where
        Self: StoreProvider<Proc>,
        Proc::Store: 'a,
    {
        let store = self.get();
        store.get(key)
    }

    /// Set value in Proc's store at key
    pub fn set_result<Proc: Process>(
        &'_ mut self,
        key: ModelKey,
        value: Proc::Result,
    ) -> Option<Proc::Result>
    where
        Self: StoreProvider<Proc>,
    {
        // bump the version up to indicate a stage was processed
        // so this model may be selected for the next stage
        let prev_version = self.version.remove(key).unwrap_or_default();
        self.version.insert(
            key,
            ProcessProgress {
                stage: Proc::STAGE_VARIANT,
                ..prev_version
            },
        );

        // store result
        let store = &mut self.get_mut();
        store.store(key, value)
    }

    pub fn process_progress_for(&self, key: ModelKey) -> ProcessProgress {
        self.version.get(key).copied().unwrap_or_default()
    }
}

pub trait ProcessStore<Proc: Process> {
    /// Create new store with capacity
    fn with_capacity(capacity: usize) -> Self;
    fn get(&self, key: ModelKey) -> crate::Result<&Proc::Result>;
    fn store(&mut self, key: ModelKey, value: Proc::Result) -> Option<Proc::Result>;
}

pub trait StoreProvider<Proc: Process> {
    fn get_store(&self) -> &Proc::Store;
    fn get_store_mut(&mut self) -> &mut Proc::Store;
}
