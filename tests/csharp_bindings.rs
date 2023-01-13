use interoptopus::{Error, Interop};

#[test]
#[cfg_attr(miri, ignore)]
fn bindings_c() -> Result<(), Error> {
    use interoptopus_backend_csharp::{Config, Generator};

    Generator::new(
        Config {
            dll_name: "akatsuki_pp".to_string(),
            ..Config::default()
        },
        akatsuki_pp_ffi::my_inventory(),
    )
    .write_file("bindings/AkatsukiPPFFI.cs")?;

    Ok(())
}
