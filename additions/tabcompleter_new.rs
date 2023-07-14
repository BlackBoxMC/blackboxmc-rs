// src/bukkit/command/mod.rs#TabCompleter
pub fn from_extendable(
    env: &crate::SharedJNIEnv<'mc>,
    lib_name: String,
    name: String,
) -> Result<Self, Box<dyn std::error::Error>> {
    let obj = jni::objects::JValueGen::Object(env.new_object(
        "net/ioixd/blackbox/extendables/ExtendableTabCompleter",
        "(Ljava/lang/String;Ljava/lang/String;)V",
        &[
            jni::objects::JValueGen::from(&jni::objects::JObject::from(
                env.new_string(name).unwrap(),
            )),
            jni::objects::JValueGen::from(&jni::objects::JObject::from(
                env.new_string(lib_name).unwrap(),
            )),
        ],
    )?);
    Ok(Self(env.clone(), unsafe {
        jni::objects::JObject::from_raw(obj.l()?.clone())
    }))
}
