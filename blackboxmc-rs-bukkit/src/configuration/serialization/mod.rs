#![allow(deprecated)]
use blackboxmc_general::JNIInstantiatable;
use blackboxmc_general::JNIRaw;
use color_eyre::eyre::Result;
/// Utility class for storing and retrieving classes for <a href="../Configuration.html" title="interface in org.bukkit.configuration"><code>Configuration</code></a>.
#[repr(C)]
pub struct ConfigurationSerialization<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for ConfigurationSerialization<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for ConfigurationSerialization<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate ConfigurationSerialization from null object."
            )
            .into());
        }
        let (valid, name) = env.validate_name(
            &obj,
            "org/bukkit/configuration/serialization/ConfigurationSerialization",
        )?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a ConfigurationSerialization object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> ConfigurationSerialization<'mc> {
    pub fn deserialize(
        &self,
        arg0: impl Into<blackboxmc_java::util::JavaMap<'mc>>,
    ) -> Result<
        crate::configuration::serialization::ConfigurationSerializable<'mc>,
        Box<dyn std::error::Error>,
    > {
        let sig = String::from(
            "(Ljava/util/Map;)Lorg/bukkit/configuration/serialization/ConfigurationSerializable;",
        );
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "deserialize",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::configuration::serialization::ConfigurationSerializable::from_raw(
            &self.jni_ref(),
            unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) },
        )
    }

    pub fn get_alias(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: jni::objects::JClass<'mc>,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/Class;)Ljava/lang/String;");
        let val_1 = jni::objects::JValueGen::Object(arg0.into());
        let cls = jni.find_class("java/lang/String");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(
            cls,
            "getAlias",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = jni.translate_error(res)?;
        Ok(jni
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }

    pub fn register_class_with_class(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: jni::objects::JClass<'mc>,
        arg1: std::option::Option<impl Into<String>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Ljava/lang/Class;";
        let val_1 = jni::objects::JValueGen::Object(arg0.into());
        args.push(val_1);
        if let Some(a) = arg1 {
            sig += "Ljava/lang/String;";
            let val_2 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
                jni.new_string(a.into())?,
            ));
            args.push(val_2);
        }
        sig += ")V";
        let cls = jni.find_class("void");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "registerClass", sig.as_str(), args);
        jni.translate_error(res)?;
        Ok(())
    }

    pub fn unregister_class_with_string(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Ljava/lang/String;";
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            jni.new_string(arg0.into())?,
        ));
        args.push(val_1);
        sig += ")V";
        let cls = jni.find_class("void");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "unregisterClass", sig.as_str(), args);
        jni.translate_error(res)?;
        Ok(())
    }

    pub fn deserialize_object_with_map(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<blackboxmc_java::util::JavaMap<'mc>>,
        arg1: std::option::Option<jni::objects::JClass<'mc>>,
    ) -> Result<
        crate::configuration::serialization::ConfigurationSerializable<'mc>,
        Box<dyn std::error::Error>,
    > {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Ljava/util/Map;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        if let Some(a) = arg1 {
            sig += "Ljava/lang/Class;";
            let val_2 = jni::objects::JValueGen::Object(a.into());
            args.push(val_2);
        }
        sig += ")Lorg/bukkit/configuration/serialization/ConfigurationSerializable;";
        let cls =
            jni.find_class("org/bukkit/configuration/serialization/ConfigurationSerializable");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "deserializeObject", sig.as_str(), args);
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        crate::configuration::serialization::ConfigurationSerializable::from_raw(&jni, obj)
    }

    pub fn get_class_by_alias(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<String>,
    ) -> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/String;)Ljava/lang/Class;");
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            jni.new_string(arg0.into())?,
        ));
        let cls = jni.find_class("java/lang/Class");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(
            cls,
            "getClassByAlias",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = jni.translate_error(res)?;
        Ok(unsafe { jni::objects::JClass::from_raw(res.as_jni().l) })
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
/// Represents an object that may be serialized.
/// <p>These objects MUST implement one of the following, in addition to the methods as defined by this interface:</p>
/// <ul>
/// <li>A static method "deserialize" that accepts a single <a class="external-link" title="class or interface in java.util" href="https://docs.oracle.com/javase/8/docs/api/java/util/Map.html"><code>Map</code></a>&lt; <a title="class or interface in java.lang" class="external-link" href="https://docs.oracle.com/javase/8/docs/api/java/lang/String.html"><code>String</code></a>, <a class="external-link" href="https://docs.oracle.com/javase/8/docs/api/java/lang/Object.html" title="class or interface in java.lang"><code>Object</code></a>&gt; and returns the class.</li>
/// <li>A static method "valueOf" that accepts a single <a title="class or interface in java.util" class="external-link" href="https://docs.oracle.com/javase/8/docs/api/java/util/Map.html"><code>Map</code></a>&lt;<a class="external-link" href="https://docs.oracle.com/javase/8/docs/api/java/lang/String.html" title="class or interface in java.lang"><code>String</code></a>, <a class="external-link" href="https://docs.oracle.com/javase/8/docs/api/java/lang/Object.html" title="class or interface in java.lang"><code>Object</code></a>&gt; and returns the class.</li>
/// <li>A constructor that accepts a single <a title="class or interface in java.util" class="external-link" href="https://docs.oracle.com/javase/8/docs/api/java/util/Map.html"><code>Map</code></a>&lt;<a href="https://docs.oracle.com/javase/8/docs/api/java/lang/String.html" title="class or interface in java.lang" class="external-link"><code>String</code></a>, <a class="external-link" href="https://docs.oracle.com/javase/8/docs/api/java/lang/Object.html" title="class or interface in java.lang"><code>Object</code></a>&gt;.</li>
/// </ul> In addition to implementing this interface, you must register the class with <a href="ConfigurationSerialization.html#registerClass(java.lang.Class)"><code>ConfigurationSerialization.registerClass(Class)</code></a>.
///
/// This is a representation of an abstract class.
#[repr(C)]
pub struct ConfigurationSerializable<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for ConfigurationSerializable<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for ConfigurationSerializable<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate ConfigurationSerializable from null object."
            )
            .into());
        }
        let (valid, name) = env.validate_name(
            &obj,
            "org/bukkit/configuration/serialization/ConfigurationSerializable",
        )?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a ConfigurationSerializable object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> ConfigurationSerializable<'mc> {
    pub fn from_extendable(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        plugin: &'mc crate::plugin::Plugin,
        address: i32,
        lib_name: String,
        name: String,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let obj = unsafe { plugin.new_extendable(address, "ConfigSerializable", name, lib_name) }?;
        Self::from_raw(env, obj)
    }

    pub fn serialize(
        &self,
    ) -> Result<blackboxmc_java::util::JavaMap<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/Map;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "serialize", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::util::JavaMap::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
/// Represents an "alias" that a <a href="ConfigurationSerializable.html" title="interface in org.bukkit.configuration.serialization"><code>ConfigurationSerializable</code></a> may be stored as. If this is not present on a <a title="interface in org.bukkit.configuration.serialization" href="ConfigurationSerializable.html"><code>ConfigurationSerializable</code></a> class, it will use the fully qualified name of the class.
/// <p>This value will be stored in the configuration so that the configuration deserialization can determine what type it is.</p>
/// <p>Using this annotation on any other class than a <a title="interface in org.bukkit.configuration.serialization" href="ConfigurationSerializable.html"><code>ConfigurationSerializable</code></a> will have no effect.</p>
///
/// This is a representation of an abstract class.
#[repr(C)]
pub struct SerializableAs<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for SerializableAs<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for SerializableAs<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate SerializableAs from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(
            &obj,
            "org/bukkit/configuration/serialization/SerializableAs",
        )?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a SerializableAs object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> SerializableAs<'mc> {
    pub fn value(&self) -> Result<String, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/String;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "value", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
/// Applies to a <a href="ConfigurationSerializable.html" title="interface in org.bukkit.configuration.serialization"><code>ConfigurationSerializable</code></a> that will delegate all deserialization to another <a href="ConfigurationSerializable.html" title="interface in org.bukkit.configuration.serialization"><code>ConfigurationSerializable</code></a>.
///
/// This is a representation of an abstract class.
#[repr(C)]
pub struct DelegateDeserialization<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for DelegateDeserialization<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for DelegateDeserialization<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate DelegateDeserialization from null object."
            )
            .into());
        }
        let (valid, name) = env.validate_name(
            &obj,
            "org/bukkit/configuration/serialization/DelegateDeserialization",
        )?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a DelegateDeserialization object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> DelegateDeserialization<'mc> {
    pub fn value(&self) -> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/Class;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "value", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(unsafe { jni::objects::JClass::from_raw(res.as_jni().l) })
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
