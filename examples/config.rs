use anyhow::Result;
use xdiff::DiffConfig;

fn main() -> Result<()> {
    let content = include_str!("../fixtures/test.yaml");
    let config = DiffConfig::from_yaml(content)?;
    println!("{:#?}", config);

    let rust_profile = config.get_profile("rust");
    println!("{:#?}", rust_profile);
    Ok(())
}
