pub mod bukkit;
pub mod bungee;

// hand-written type
pub trait JNIRaw<'mc> {
    fn jni_ref(&self) -> jni::JNIEnv<'mc>;
    fn jni_object(&self) -> jni::objects::JObject<'mc>;
}
