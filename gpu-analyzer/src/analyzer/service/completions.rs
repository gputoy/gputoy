impl crate::Analyzer {
    pub fn get_completions_at_position(
        &mut self,
        path: gpu_common::FilePath,
        _prev_word: &str,
        _curr_word: &str,
        delim: &str,
        _position: super::types::Position,
    ) -> crate::Result<Vec<super::types::CompletionItem>> {
        // log::info!("get_completions: prev_word: {prev_word}");
        let key = self.model_cache.get_key(&path)?;
        let _model = self.model_cache.get_model(key)?;

        let ret = Vec::new();
        match delim {
            "." => {}
            "@" => {}
            "" => {}
            _ => {}
        }

        Ok(ret)
    }
}
