use embuild::{
    build::{CfgArgs, LinkArgs},
};

// Necessary because of this issue: https://github.com/rust-lang/cargo/issues/9641
fn main() -> anyhow::Result<()> {
    LinkArgs::output_propagated("ESP_IDF")?;
    CfgArgs::try_from_env("ESP_IDF")
}
