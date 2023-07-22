// src/bukkit/generator/mod.rs#BlockPopulator
pub fn from_extendable(
    env: &crate::SharedJNIEnv<'mc>,
    plugin: &'mc crate::bukkit::plugin::Plugin,
    lib_name: String,
    name: String,
) -> Result<Self, Box<dyn std::error::Error>> {
    let obj = unsafe { plugin.new_extendable("BlockPopulator", name, lib_name) }?;
    Self::from_raw(env, obj)
}
