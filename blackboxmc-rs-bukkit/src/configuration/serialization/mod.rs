#![allow(deprecated)]
use blackboxmc_general::JNIRaw;
use color_eyre::eyre::Result;
/// Utility class for storing and retrieving classes for <a href="../Configuration.html" title="interface in org.bukkit.configuration"><code>Configuration</code></a>.
pub struct ConfigurationSerialization<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for ConfigurationSerialization<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> ConfigurationSerialization<'mc> {
    pub fn from_raw(
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
    /// Attempts to get a registered <a href="ConfigurationSerializable.html" title="interface in org.bukkit.configuration.serialization"><code>ConfigurationSerializable</code></a> class by its alias
    pub fn get_class_by_alias(
        jni: blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<&'mc String>,
    ) -> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(jni.new_string(arg0.into()).unwrap());
        let cls = &jni.find_class("java/lang/Class")?;
        let res = jni.call_static_method(
            cls,
            "getClassByAlias",
            "(Ljava/lang/String;)Ljava/lang/Class;",
            &[jni::objects::JValueGen::from(&val_1)],
        )?;
        Ok(unsafe { jni::objects::JClass::from_raw(res.as_jni().l) })
    }
    /// Attempts to deserialize the given arguments into a new instance of the given class.
    /// <p>The class must implement <a href="ConfigurationSerializable.html" title="interface in org.bukkit.configuration.serialization"><code>ConfigurationSerializable</code></a>, including the extra methods as specified in the javadoc of ConfigurationSerializable.</p>
    /// <p>If a new instance could not be made, an example being the class not fully implementing the interface, null will be returned.</p>
    /// Attempts to deserialize the given arguments into a new instance of the given class.
    /// <p>The class must implement <a title="interface in org.bukkit.configuration.serialization" href="ConfigurationSerializable.html"><code>ConfigurationSerializable</code></a>, including the extra methods as specified in the javadoc of ConfigurationSerializable.</p>
    /// <p>If a new instance could not be made, an example being the class not fully implementing the interface, null will be returned.</p>
    pub fn deserialize(
        &mut self,
        arg0: impl Into<&'mc blackboxmc_java::JavaMap<'mc>>,
    ) -> Result<
        crate::configuration::serialization::ConfigurationSerializable<'mc>,
        Box<dyn std::error::Error>,
    > {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "deserialize",
            "(Ljava/util/Map;)Lorg/bukkit/configuration/serialization/ConfigurationSerializable;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::configuration::serialization::ConfigurationSerializable::from_raw(
            &self.jni_ref(),
            unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) },
        )
    }
    /// Gets the correct alias for the given <a title="interface in org.bukkit.configuration.serialization" href="ConfigurationSerializable.html"><code>ConfigurationSerializable</code></a> class
    pub fn get_alias(
        jni: blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: jni::objects::JClass<'mc>,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let cls = &jni.find_class("java/lang/String")?;
        let res = jni.call_static_method(
            cls,
            "getAlias",
            "(Ljava/lang/Class;)Ljava/lang/String;",
            &[jni::objects::JValueGen::from(&val_1)],
        )?;
        Ok(jni
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    /// Registers the given <a href="ConfigurationSerializable.html" title="interface in org.bukkit.configuration.serialization"><code>ConfigurationSerializable</code></a> class by its alias
    /// Registers the given alias to the specified <a href="ConfigurationSerializable.html" title="interface in org.bukkit.configuration.serialization"><code>ConfigurationSerializable</code></a> class
    pub fn register_class_with_class(
        jni: blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: std::option::Option<jni::objects::JClass<'mc>>,
        arg1: std::option::Option<impl Into<&'mc String>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = arg0.unwrap();
        let val_2 = jni::objects::JObject::from(jni.new_string(arg1.unwrap().into()).unwrap());
        let cls = &jni.find_class("void")?;
        let res = jni.call_static_method(
            cls,
            "registerClass",
            "(Ljava/lang/Class;Ljava/lang/String;)V",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        )?;
        Ok(())
    }
    /// Unregisters the specified alias to a <a href="ConfigurationSerializable.html" title="interface in org.bukkit.configuration.serialization"><code>ConfigurationSerializable</code></a>
    /// Unregisters any aliases for the specified <a href="ConfigurationSerializable.html" title="interface in org.bukkit.configuration.serialization"><code>ConfigurationSerializable</code></a> class
    pub fn unregister_class_with_class(
        jni: blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: std::option::Option<impl Into<&'mc String>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(jni.new_string(arg0.unwrap().into()).unwrap());
        let cls = &jni.find_class("void")?;
        let res = jni.call_static_method(
            cls,
            "unregisterClass",
            "(Ljava/lang/String;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        )?;
        Ok(())
    }
    /// Attempts to deserialize the given arguments into a new instance of the given class.
    /// <p>The class must implement <a href="ConfigurationSerializable.html" title="interface in org.bukkit.configuration.serialization"><code>ConfigurationSerializable</code></a>, including the extra methods as specified in the javadoc of ConfigurationSerializable.</p>
    /// <p>If a new instance could not be made, an example being the class not fully implementing the interface, null will be returned.</p>
    /// Attempts to deserialize the given arguments into a new instance of the given class.
    /// <p>The class must implement <a href="ConfigurationSerializable.html" title="interface in org.bukkit.configuration.serialization"><code>ConfigurationSerializable</code></a>, including the extra methods as specified in the javadoc of ConfigurationSerializable.</p>
    /// <p>If a new instance could not be made, an example being the class not fully implementing the interface, null will be returned.</p>
    pub fn deserialize_object_with_map(
        jni: blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: std::option::Option<impl Into<&'mc blackboxmc_java::JavaMap<'mc>>>,
        arg1: std::option::Option<jni::objects::JClass<'mc>>,
    ) -> Result<
        crate::configuration::serialization::ConfigurationSerializable<'mc>,
        Box<dyn std::error::Error>,
    > {
        let val_1 =
            unsafe { jni::objects::JObject::from_raw(arg0.unwrap().into().jni_object().clone()) };
        let val_2 = arg1.unwrap();
        let cls =
            &jni.find_class("org/bukkit/configuration/serialization/ConfigurationSerializable")?;
        let res = jni.call_static_method(cls,"deserializeObject",
"(Ljava/util/Map;Ljava/lang/Class;)Lorg/bukkit/configuration/serialization/ConfigurationSerializable;",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)])?;
        let obj = res.l()?;
        crate::configuration::serialization::ConfigurationSerializable::from_raw(&jni, obj)
    }

    pub fn wait(
        &mut self,
        arg0: std::option::Option<i64>,
        arg1: std::option::Option<i32>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Long(arg0.unwrap().into());
        let val_2 = jni::objects::JValueGen::Int(arg1.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "wait",
            "(JI)V",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn equals(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "equals",
            "(Ljava/lang/Object;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn to_string(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "toString", "()Ljava/lang/String;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }

    pub fn hash_code(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hashCode", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }
    /// Attempts to get a registered <a href="ConfigurationSerializable.html" title="interface in org.bukkit.configuration.serialization"><code>ConfigurationSerializable</code></a> class by its alias
    pub fn class(&mut self) -> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getClass", "()Ljava/lang/Class;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(unsafe { jni::objects::JClass::from_raw(res.as_jni().l) })
    }

    pub fn notify(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notify", "()V", &[]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn notify_all(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notifyAll", "()V", &[]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
}
/// Represents an object that may be serialized.
/// <p>These objects MUST implement one of the following, in addition to the methods as defined by this interface:</p>
/// <ul>
/// <li>A static method "deserialize" that accepts a single <a title="class or interface in java.util" href="https://docs.oracle.com/javase/8/docs/api/java/util/Map.html" class="external-link"><code>Map</code></a>&lt; <a title="class or interface in java.lang" href="https://docs.oracle.com/javase/8/docs/api/java/lang/String.html" class="external-link"><code>String</code></a>, <a class="external-link" href="https://docs.oracle.com/javase/8/docs/api/java/lang/Object.html" title="class or interface in java.lang"><code>Object</code></a>&gt; and returns the class.</li>
/// <li>A static method "valueOf" that accepts a single <a href="https://docs.oracle.com/javase/8/docs/api/java/util/Map.html" title="class or interface in java.util" class="external-link"><code>Map</code></a>&lt;<a title="class or interface in java.lang" href="https://docs.oracle.com/javase/8/docs/api/java/lang/String.html" class="external-link"><code>String</code></a>, <a href="https://docs.oracle.com/javase/8/docs/api/java/lang/Object.html" class="external-link" title="class or interface in java.lang"><code>Object</code></a>&gt; and returns the class.</li>
/// <li>A constructor that accepts a single <a class="external-link" href="https://docs.oracle.com/javase/8/docs/api/java/util/Map.html" title="class or interface in java.util"><code>Map</code></a>&lt;<a href="https://docs.oracle.com/javase/8/docs/api/java/lang/String.html" class="external-link" title="class or interface in java.lang"><code>String</code></a>, <a title="class or interface in java.lang" href="https://docs.oracle.com/javase/8/docs/api/java/lang/Object.html" class="external-link"><code>Object</code></a>&gt;.</li>
/// </ul> In addition to implementing this interface, you must register the class with <a href="ConfigurationSerialization.html#registerClass(java.lang.Class)"><code>ConfigurationSerialization.registerClass(Class)</code></a>.
///
/// This is a representation of an abstract class.
pub struct ConfigurationSerializable<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
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
    pub fn from_raw(
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
    /// Creates a Map representation of this class.
    /// <p>This class must provide a method to restore this class, as defined in the <a href="ConfigurationSerializable.html" title="interface in org.bukkit.configuration.serialization"><code>ConfigurationSerializable</code></a> interface javadocs.</p>
    pub fn serialize(
        &mut self,
    ) -> Result<blackboxmc_java::JavaMap<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "serialize", "()Ljava/util/Map;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::JavaMap::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
}
impl<'mc> JNIRaw<'mc> for ConfigurationSerializable<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
/// Represents an "alias" that a <a title="interface in org.bukkit.configuration.serialization" href="ConfigurationSerializable.html"><code>ConfigurationSerializable</code></a> may be stored as. If this is not present on a <a title="interface in org.bukkit.configuration.serialization" href="ConfigurationSerializable.html"><code>ConfigurationSerializable</code></a> class, it will use the fully qualified name of the class.
/// <p>This value will be stored in the configuration so that the configuration deserialization can determine what type it is.</p>
/// <p>Using this annotation on any other class than a <a title="interface in org.bukkit.configuration.serialization" href="ConfigurationSerializable.html"><code>ConfigurationSerializable</code></a> will have no effect.</p>
///
/// This is a representation of an abstract class.
pub struct SerializableAs<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> SerializableAs<'mc> {
    pub fn from_raw(
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
    /// This is the name your class will be stored and retrieved as.
    /// <p>This name MUST be unique. We recommend using names such as "MyPluginThing" instead of "Thing".</p>
    pub fn value(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "value", "()Ljava/lang/String;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }

    pub fn equals(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "equals",
            "(Ljava/lang/Object;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn to_string(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "toString", "()Ljava/lang/String;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }

    pub fn hash_code(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hashCode", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }

    pub fn annotation_type(
        &mut self,
    ) -> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "annotationType",
            "()Ljava/lang/Class;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(unsafe { jni::objects::JClass::from_raw(res.as_jni().l) })
    }
}
impl<'mc> JNIRaw<'mc> for SerializableAs<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
/// Applies to a <a href="ConfigurationSerializable.html" title="interface in org.bukkit.configuration.serialization"><code>ConfigurationSerializable</code></a> that will delegate all deserialization to another <a href="ConfigurationSerializable.html" title="interface in org.bukkit.configuration.serialization"><code>ConfigurationSerializable</code></a>.
///
/// This is a representation of an abstract class.
pub struct DelegateDeserialization<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> DelegateDeserialization<'mc> {
    pub fn from_raw(
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
    /// Which class should be used as a delegate for this classes deserialization
    pub fn value(&mut self) -> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "value", "()Ljava/lang/Class;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(unsafe { jni::objects::JClass::from_raw(res.as_jni().l) })
    }

    pub fn equals(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "equals",
            "(Ljava/lang/Object;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn to_string(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "toString", "()Ljava/lang/String;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }

    pub fn hash_code(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hashCode", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }

    pub fn annotation_type(
        &mut self,
    ) -> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "annotationType",
            "()Ljava/lang/Class;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(unsafe { jni::objects::JClass::from_raw(res.as_jni().l) })
    }
}
impl<'mc> JNIRaw<'mc> for DelegateDeserialization<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
