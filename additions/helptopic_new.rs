// src/bukkit/help/mod.rs#HelpTopic
pub fn from_extendable(
    env: &crate::SharedJNIEnv<'mc>,
    plugin: &'mc crate::bukkit::plugin::Plugin,
    address: i32,
    lib_name: String,
    name: String,
) -> Result<Self, Box<dyn std::error::Error>> {
    let obj = unsafe { plugin.new_extendable(address, "HelpTopic", name, lib_name) }?;
    Self::from_raw(env, obj)
}
