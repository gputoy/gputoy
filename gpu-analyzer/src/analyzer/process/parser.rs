use crate::diagnostics::DiagnosticSink;

use super::{LinkerProcess, Stage};

pub struct ParserProcess;

crate::store_for!(ParserStore => ParserProcess);

impl super::Process for ParserProcess {
    type Result = naga::Module;
    type Store = ParserStore;

    const PROCESS_NAME: &'static str = "parser";
    const STAGE_VARIANT: Stage = Stage::Parser;

    fn process(
        analyzer: &mut crate::Analyzer,
        key: crate::ModelKey,
        _sink: &mut DiagnosticSink,
    ) -> crate::Result<Self::Result> {
        let link_result = analyzer.store.get_result::<LinkerProcess>(key)?;
        let src = &link_result.processed_src;

        let module = analyzer
            .wgsl_parser
            .borrow_mut()
            .parse(src)
            .map_err(|err| crate::Error::Parse(key, Box::new(err)))?;

        Ok(module)
    }
}
