#![allow(deprecated)]
use blackboxmc_general::JNIInstantiatable;
use blackboxmc_general::JNIRaw;
use color_eyre::eyre::Result;
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
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Ljava/util/Map;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "serialize", sig.as_str(), args);
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
        let sig = String::from("()LString;");
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
        val_args: impl Into<blackboxmc_java::util::JavaMap<'mc>>,
    ) -> Result<
        Option<crate::configuration::serialization::ConfigurationSerializable<'mc>>,
        Box<dyn std::error::Error>,
    > {
        let sig = String::from(
            "(Ljava/util/Map;)Lcrate::configuration::serialization::ConfigurationSerializable;",
        );
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(val_args.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "deserialize",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {
            return Ok(None);
        }
        Ok(Some(
            crate::configuration::serialization::ConfigurationSerializable::from_raw(
                &self.jni_ref(),
                unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) },
            )?,
        ))
    }
    pub fn deserialize_object_with_args(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        val_args: impl Into<blackboxmc_java::util::JavaMap<'mc>>,
        clazz: std::option::Option<jni::objects::JClass<'mc>>,
    ) -> Result<
        Option<crate::configuration::serialization::ConfigurationSerializable<'mc>>,
        Box<dyn std::error::Error>,
    > {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Ljava/util/Map;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(val_args.into().jni_object().clone())
        });
        args.push(val_1);
        if let Some(a) = clazz {
            sig += "Ljava/lang/Class;";
            let val_2 = jni::objects::JValueGen::Object(a.into());
            args.push(val_2);
        }
        sig += ")Lorg/bukkit/configuration/serialization/ConfigurationSerializable;";
        let cls =
            jni.find_class("org/bukkit/configuration/serialization/ConfigurationSerialization");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "deserializeObject", sig.as_str(), args);
        let res = jni.translate_error(res)?;
        if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {
            return Ok(None);
        }
        let obj = res.l()?;
        Ok(Some(
            crate::configuration::serialization::ConfigurationSerializable::from_raw(&jni, obj)?,
        ))
    }
    pub fn register_class_with_clazz(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        clazz: jni::objects::JClass<'mc>,
        alias: std::option::Option<impl Into<String>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Ljava/lang/Class;";
        let val_1 = jni::objects::JValueGen::Object(clazz.into());
        args.push(val_1);
        if let Some(a) = alias {
            sig += "Ljava/lang/String;";
            let val_2 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
                jni.new_string(a.into())?,
            ));
            args.push(val_2);
        }
        sig += ")V";
        let cls =
            jni.find_class("org/bukkit/configuration/serialization/ConfigurationSerialization");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "registerClass", sig.as_str(), args);
        jni.translate_error(res)?;
        Ok(())
    }
    pub fn unregister_class_with_clazz(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        clazz: jni::objects::JClass<'mc>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Ljava/lang/Class;";
        let val_1 = jni::objects::JValueGen::Object(clazz.into());
        args.push(val_1);
        sig += ")V";
        let cls =
            jni.find_class("org/bukkit/configuration/serialization/ConfigurationSerialization");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "unregisterClass", sig.as_str(), args);
        jni.translate_error(res)?;
        Ok(())
    }
    pub fn get_class_by_alias(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        alias: impl Into<String>,
    ) -> Result<Option<jni::objects::JClass<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/String;)Ljni::objects::JClass;");
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            jni.new_string(alias.into())?,
        ));
        let cls =
            jni.find_class("org/bukkit/configuration/serialization/ConfigurationSerialization");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(
            cls,
            "getClassByAlias",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = jni.translate_error(res)?;
        if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {
            return Ok(None);
        }
        Ok(Some(unsafe {
            jni::objects::JClass::from_raw(res.as_jni().l)
        }))
    }
    pub fn get_alias(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        clazz: jni::objects::JClass<'mc>,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/Class;)LString;");
        let val_1 = jni::objects::JValueGen::Object(clazz.into());
        let cls =
            jni.find_class("org/bukkit/configuration/serialization/ConfigurationSerialization");
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
    // SUPER CLASS: Object

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
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
        let sig = String::from("()Ljni::objects::JClass;");
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
