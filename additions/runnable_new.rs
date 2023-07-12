// src/bukkit/scheduler/mod.rs#BukkitRunnable
pub fn from_extendable<T>(
    plugin: &super::plugin::Plugin,
    event: &'mc T,
    lib_name: String,
    name: String,
) -> Result<Self, Box<dyn std::error::Error>>
where
    T: crate::JNIRaw<'mc>,
{
    let obj = jni::objects::JValueGen::Object(event.jni_ref().new_object(
        "net/ioixd/blackbox/extendables/ExtendableBukkitRunnable",
        "(Lorg/bukkit/plugin/Plugin;Ljava/lang/String;Ljava/lang/String;)V",
        &[
            jni::objects::JValueGen::from(&plugin.1),
            jni::objects::JValueGen::from(&jni::objects::JObject::from(
                event.jni_ref().new_string(name).unwrap(),
            )),
            jni::objects::JValueGen::from(&jni::objects::JObject::from(
                event.jni_ref().new_string(lib_name).unwrap(),
            )),
        ],
    )?);
    Ok(Self(event.jni_ref(), unsafe {
        jni::objects::JObject::from_raw(obj.l()?.clone())
    }))
}
