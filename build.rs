// Necessary because of this issue: https://github.com/rust-lang/cargo/issues/9641
// Basically it's an option which should have "the explicit opt-in"
fn main() -> anyhow::Result<()> {
    embuild::build::CfgArgs::output_propagated("ESP_IDF")?;
    embuild::build::LinkArgs::output_propagated("ESP_IDF")
}
