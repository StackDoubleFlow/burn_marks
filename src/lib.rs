use anyhow::{anyhow, Context, Result};
use quest_hook::hook;
use quest_hook::libil2cpp::Il2CppObject;
use tracing::info;

#[no_mangle]
pub extern "C" fn setup() {
    quest_hook::setup("burn_marks");
}

#[hook("", "MainSettingsModelSO", "Load")]
fn on_enable(this: &mut Il2CppObject, forced: bool) -> Result<()> {
    on_enable.original(this, forced)?;

    let burn_mark_trails_enabled = this
        .load::<Il2CppObject>("burnMarkTrailsEnabled")
        .context("Could not find field burnMarkTrailsEnabled")?;
    burn_mark_trails_enabled
        .invoke_void("set_value", true)
        .map_err(|err| anyhow!(err.to_string()))?;

    Ok(())
}

#[no_mangle]
pub extern "C" fn load() {
    info!("Installing burn_marks hooks!");

    on_enable.install().unwrap();

    info!("Installed burn_marks hooks!");
}
