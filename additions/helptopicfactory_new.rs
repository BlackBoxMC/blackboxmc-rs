// blackboxmc-rs-bukkit/src/help/mod.rs#HelpTopicFactory
pub fn from_extendable(
    env: &blackboxmc_general::SharedJNIEnv<'mc>,
    plugin: &'mc crate::plugin::Plugin,
    address: i32,
    lib_name: String,
    name: String,
) -> Result<Self, Box<dyn std::error::Error>> {
    let obj = unsafe { plugin.new_extendable(address, "HelpTopicFactory", name, lib_name) }?;
    Self::from_raw(env, obj)
}
