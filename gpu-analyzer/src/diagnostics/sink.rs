use crate::{analyzer::Stage, ModelKey};

use super::IntoDiagnostic;

#[derive(Debug)]
pub struct DiagnosticSink {
    stage: Stage,
    inner: Vec<Box<dyn IntoDiagnostic>>,
    dispatch_count: usize,
}

impl DiagnosticSink {
    pub fn new(stage: Stage) -> Self {
        Self {
            stage,
            inner: Vec::new(),
            dispatch_count: 0,
        }
    }
    pub fn push(&mut self, diagnostic: Box<dyn IntoDiagnostic>) {
        self.dispatch_count += 1;
        self.inner.push(diagnostic)
    }

    pub fn prep_dispatch(&mut self) {
        self.dispatch_count = 0;
    }

    pub fn num_diagnostics_for_dispatch(&self) -> usize {
        self.dispatch_count
    }

    pub fn into_inner(self) -> Vec<Box<dyn IntoDiagnostic>> {
        self.inner
    }

    pub fn throw_on_fail(&self, key: ModelKey) -> crate::Result<()> {
        let err_count = self.inner.iter().filter(|e| !e.recoverable()).count();
        if self.inner.is_empty() {
            Ok(())
        } else {
            Err(crate::Error::DispatchFailure(self.stage, key, err_count))
        }
    }

    pub fn should_fail(&self) -> bool {
        self.inner.iter().any(|e| !e.recoverable())
    }

    pub fn should_fail_dispatch(&self) -> bool {
        self.inner
            .iter()
            .rev()
            .take(self.dispatch_count)
            .any(|e| !e.recoverable())
    }
}
