#[test]
fn simple_dependency_chain() {
    // let files = files_from_dir_str("/dependencies/simple").unwrap();
    // let models = ModelCache::from_files(files);
    // let main_key = models
    //     .get_key(&FilePath::try_from("/main.wgsl").unwrap())
    //     .unwrap();
    // let types_key = models
    //     .get_key(&FilePath::try_from("/types.wgsl").unwrap())
    //     .unwrap();

    // let mut dependencies = Dependencies::new();
    // assert!(dependencies.cache_deps(&models).is_ok());

    // assert_eq!(
    //     dependencies.get_imports(main_key).unwrap(),
    //     &vec![ModelImport {
    //         from: types_key,
    //         idx: 0,
    //         location: Location::new(types_key, 8..102),
    //     }]
    // );

    // let export = &dependencies.get_exports(types_key).unwrap()[0];
    // assert_eq!(
    //     export,
    //     &ModelExport {
    //         ident: "FragIn".into(),
    //         location: Location::new(types_key, 8..102),
    //     }
    // );
}
