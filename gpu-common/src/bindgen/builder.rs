use std::{
    cell::RefCell,
    collections::HashMap,
    path::{Path, PathBuf},
    rc::Rc,
};

use super::{json_to_ts, Config, Error, Result};

#[derive(Clone)]
pub struct Builder {
    path_buf: PathBuf,
    inner: Rc<RefCell<Inner>>,
}

#[derive(Clone)]
struct Inner {
    config: Config,
    files: HashMap<PathBuf, String>,
    ts: Vec<String>,
}

impl Inner {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            files: Default::default(),
            ts: Default::default(),
        }
    }

    pub fn append_ts<S: ToString>(&mut self, val: S) {
        self.ts.push(val.to_string());
    }

    pub fn finish(mut self) -> Result<()> {
        self.files
            .insert(self.config.ts_index_path(), self.ts.join("\n\n"));
        // Write all files excluding ts, as the root json shcema will need to be written
        // first as a dependency for `json_to_ts::ts_from_json_schema`
        for (path, file) in self.files {
            std::fs::write(path.clone(), file)
                .map_err(|err| Error::CreateFile(path.clone(), err))?;
            log::info!("Generated {}", path.display());
        }

        let ts_content = json_to_ts::ts_from_json_schema(&self.config)?;
        self.ts.push(ts_content);
        std::fs::write(self.config.ts_index_path(), self.ts.join("\n\n"))
            .map_err(|err| Error::CreateFile(self.config.ts_index_path(), err))?;
        Ok(())
    }
}

impl Builder {
    pub fn new(config: Config) -> Self {
        Self {
            path_buf: config.root_path.clone(),
            inner: Rc::new(RefCell::new(Inner::new(config))),
        }
    }

    pub fn append_ts<S: ToString>(&self, val: S) {
        self.inner.borrow_mut().append_ts(val)
    }

    pub fn file<P: AsRef<Path>>(&self, file_name: P) -> Writer {
        let mut file_path = self.path_buf.clone();
        file_path.push(file_name);
        Writer {
            file_path,
            contents: Vec::new(),
            inner: self.inner.clone(),
        }
    }
    pub fn dir<P: AsRef<Path>>(&self, dir_name: P) -> Result<Self> {
        let mut path_buf = self.path_buf.clone();
        path_buf.push(dir_name);
        std::fs::create_dir_all(path_buf.clone())
            .map_err(|err| Error::CreateDirectory(path_buf.clone(), err))?;
        Ok(Self {
            path_buf,
            inner: self.inner.clone(),
        })
    }

    pub fn finish(self) -> Result<()> {
        self.inner.borrow().clone().finish()
    }
}

pub struct Writer {
    file_path: PathBuf,
    contents: Vec<String>,
    inner: Rc<RefCell<Inner>>,
}

impl Writer {
    pub fn containing<S: ToString>(self, val: S) {
        self.append(val.to_string()).finish()
    }
    pub fn append<S: ToString>(mut self, val: S) -> Self {
        self.contents.push(val.to_string());
        self
    }
    pub fn containing_default<D>(self) -> Result<()>
    where
        D: std::fmt::Debug + Default + serde::Serialize,
    {
        self.default::<D>()?.finish();
        Ok(())
    }
    pub fn default<D>(self) -> Result<Self>
    where
        D: std::fmt::Debug + Default + serde::Serialize,
    {
        let value = serde_json::to_string_pretty(&D::default()).map_err(Error::SerializeDefault)?;
        Ok(self.append(value))
    }

    pub fn containing_value<S>(self, val: &S) -> Result<()>
    where
        S: std::fmt::Debug + serde::Serialize,
    {
        self.value::<S>(val)?.finish();
        Ok(())
    }
    pub fn value<S>(self, val: &S) -> Result<Self>
    where
        S: std::fmt::Debug + serde::Serialize,
    {
        let value = serde_json::to_string_pretty(val).map_err(Error::SerializeValue)?;
        Ok(self.append(value))
    }

    pub fn containing_schema<S: schemars::JsonSchema>(self) -> Result<()> {
        self.schema::<S>()?.finish();
        Ok(())
    }
    pub fn schema<S: schemars::JsonSchema>(self) -> Result<Self> {
        let gen = schemars::gen::SchemaSettings::draft07().into_generator();
        let schema = gen.into_root_schema_for::<S>();

        let value = serde_json::to_string_pretty(&schema).map_err(Error::SerializeSchema)?;
        Ok(self.append(value))
    }
    pub fn finish(self) {
        let contents = self.contents.join("\n");
        self.inner
            .borrow_mut()
            .files
            .insert(self.file_path, contents);
    }
}
