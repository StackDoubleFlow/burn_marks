use quest_hook::hook;
use quest_hook::libil2cpp::Il2CppObject;
use tracing_android::tracing::{info};

#[no_mangle]
pub extern "C" fn setup() {
    quest_hook::setup();
}

#[hook("", "MainSettingsModelSO", "Load")]
fn on_enable(this: &mut Il2CppObject, forced: bool) {
    on_enable.original(this, forced);

    let field = this.class().find_field_unchecked("burnMarkTrailsEnabled").unwrap();
    let val: &mut Il2CppObject = field.load(this);
    let _: () = val.invoke("set_value", true).unwrap();
}

#[no_mangle]
pub extern "C" fn load() {
    info!("Installing RustTest2 hooks!");

    on_enable.install();

    info!("Installed RustTest2 hooks!");
}
