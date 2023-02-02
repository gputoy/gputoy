pub struct Client {
    context: crate::Context,
    #[cfg(feature = "sdk")]
    analyzer: gpu_compiler::Compiler,
}

impl Client {
    pub async fn new() -> Result<Self, crate::Error> {
        Ok(Self {
            context: crate::Context::new().await?,
            #[cfg(feature = "sdk")]
            analyzer: gpu_compiler::Compiler::new(),
        })
    }

    // pub async fn init_from_build(build: ProjectBuild) -> Result<(), crate::Error> {

    // }

    // pub fn dispatch(state: DispatchState) -> Result<(), crate::Error> {
    //     Ok(())
    // }

    // pub fn handle
}

// #[cfg(feature = "sdk")]
// impl Client {

//     pub fn client_event(ev: &Event) ->  {

//     }

//     pub fn incremental_build() {

//     }
// }
