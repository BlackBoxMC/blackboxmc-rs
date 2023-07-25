// blackboxmc-rs-bukkit/src/plugin/mod.rs#Plugin
/// Return one of the extendable classes that BlackBox supports, based on the value given.
///
/// ## Safety
/// - It returns a Java Object that you must then cast into the proper object via JNI. You are responsible for the checks yourself.
/// - This function is specific to the BlackboxPlugin class supplied within the plugin, and will error out if you pass a regular JavaPlugin.
pub unsafe fn new_extendable(
    &self,
    address: i32,
    class_name: impl Into<String>
        + std::convert::AsRef<str>
        + std::convert::AsRef<str>
        + std::convert::AsRef<str>,
    name: impl Into<String> + std::convert::AsRef<str> + std::convert::AsRef<str>,
    lib_name: impl Into<String> + std::convert::AsRef<str> + std::convert::AsRef<str>,
) -> Result<jni::objects::JObject, Box<dyn std::error::Error>> {
    let obj = self.jni_ref().call_method(
        &self.1,
        "newExtendable",
        "(ILjava/lang/String;Ljava/lang/String;Ljava/lang/String;)Ljava/lang/Object;",
        &[
            jni::objects::JValueGen::Int(address),
            jni::objects::JValueGen::from(&jni::objects::JObject::from(
                self.jni_ref().new_string(class_name).unwrap(),
            )),
            jni::objects::JValueGen::from(&jni::objects::JObject::from(
                self.jni_ref().new_string(name).unwrap(),
            )),
            jni::objects::JValueGen::from(&jni::objects::JObject::from(
                self.jni_ref().new_string(lib_name).unwrap(),
            )),
        ],
    );
    let obj = self.jni_ref().translate_error(obj)?;
    Ok(jni::objects::JObject::from_raw(obj.l()?))
}
