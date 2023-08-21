#![allow(deprecated)]
use blackboxmc_general::JNIInstantiatable;
use blackboxmc_general::JNIInstantiatableEnum;
use blackboxmc_general::JNIRaw;
use color_eyre::eyre::Result;

///
/// This is a representation of an abstract class.
#[repr(C)]
pub struct MetadataStore<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for MetadataStore<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}

impl<'mc> JNIInstantiatable<'mc> for MetadataStore<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate MetadataStore from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/metadata/MetadataStore")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a MetadataStore object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> MetadataStore<'mc> {
    pub fn set_metadata(
        &self,
        arg0: jni::objects::JObject<'mc>,
        arg1: impl Into<String>,
        arg2: impl Into<crate::metadata::MetadataValue<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from(
            "(Ljava/lang/Object;Ljava/lang/String;Lorg/bukkit/metadata/MetadataValue;)V",
        );
        let val_1 = jni::objects::JValueGen::Object(arg0);
        let val_2 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(arg1.into())?,
        ));
        let val_3 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg2.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setMetadata",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
                jni::objects::JValueGen::from(val_3),
            ],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn get_metadata(
        &self,
        arg0: jni::objects::JObject<'mc>,
        arg1: impl Into<String>,
    ) -> Result<Vec<crate::metadata::MetadataValue<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/Object;Ljava/lang/String;)Ljava/util/List;");
        let val_1 = jni::objects::JValueGen::Object(arg0);
        let val_2 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(arg1.into())?,
        ));
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getMetadata",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let list = blackboxmc_java::util::JavaList::from_raw(&self.jni_ref(), res.l()?)?;
        let iter = list.iterator()?;
        while iter.has_next()? {
            let obj = iter.next()?;
            new_vec.push(crate::metadata::MetadataValue::from_raw(
                &self.jni_ref(),
                obj,
            )?);
        }
        Ok(new_vec)
    }

    pub fn has_metadata(
        &self,
        arg0: jni::objects::JObject<'mc>,
        arg1: impl Into<String>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/Object;Ljava/lang/String;)Z");
        let val_1 = jni::objects::JValueGen::Object(arg0);
        let val_2 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(arg1.into())?,
        ));
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "hasMetadata",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn remove_metadata(
        &self,
        arg0: jni::objects::JObject<'mc>,
        arg1: impl Into<String>,
        arg2: impl Into<crate::plugin::Plugin<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/Object;Ljava/lang/String;Lorg/bukkit/plugin/Plugin;)V");
        let val_1 = jni::objects::JValueGen::Object(arg0);
        let val_2 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(arg1.into())?,
        ));
        let val_3 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg2.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeMetadata",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
                jni::objects::JValueGen::from(val_3),
            ],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn invalidate_all(
        &self,
        arg0: impl Into<crate::plugin::Plugin<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/plugin/Plugin;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "invalidateAll",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn instance_of<A>(&self, other: A) -> bool
    where
        A: blackboxmc_general::JNIProvidesClassName,
    {
        let cls = &self.jni_ref().find_class(other.class_name()).unwrap();
        self.jni_ref()
            .is_instance_of(&self.jni_object(), cls)
            .unwrap()
    }
}

///
/// This is a representation of an abstract class.
#[repr(C)]
pub struct MetadataValue<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for MetadataValue<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}

impl<'mc> JNIInstantiatable<'mc> for MetadataValue<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate MetadataValue from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/metadata/MetadataValue")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a MetadataValue object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> MetadataValue<'mc> {
    pub fn from_extendable(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        plugin: &'mc crate::plugin::Plugin,
        address: i32,
        lib_name: String,
        name: String,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let obj = unsafe { plugin.new_extendable(address, "MetadataValue", name, lib_name) }?;
        Self::from_raw(env, obj)
    }

    pub fn value(&self) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/Object;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "value", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l()?)
    }

    pub fn as_string(&self) -> Result<String, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/String;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "asString", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }

    pub fn as_int(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "asInt", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }

    pub fn owning_plugin(
        &self,
    ) -> Result<Option<crate::plugin::Plugin<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/plugin/Plugin;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getOwningPlugin", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {
            return Ok(None);
        }
        Ok(Some(crate::plugin::Plugin::from_raw(
            &self.jni_ref(),
            unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) },
        )?))
    }

    pub fn invalidate(&self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "invalidate", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn as_float(&self) -> Result<f32, Box<dyn std::error::Error>> {
        let sig = String::from("()F");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "asFloat", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.f()?)
    }

    pub fn as_double(&self) -> Result<f64, Box<dyn std::error::Error>> {
        let sig = String::from("()D");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "asDouble", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.d()?)
    }

    pub fn as_long(&self) -> Result<i64, Box<dyn std::error::Error>> {
        let sig = String::from("()J");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "asLong", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.j()?)
    }

    pub fn as_short(&self) -> Result<i16, Box<dyn std::error::Error>> {
        let sig = String::from("()S");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "asShort", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.s()?)
    }

    pub fn as_byte(&self) -> Result<i8, Box<dyn std::error::Error>> {
        let sig = String::from("()B");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "asByte", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.b()?)
    }

    pub fn as_boolean(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "asBoolean", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn instance_of<A>(&self, other: A) -> bool
    where
        A: blackboxmc_general::JNIProvidesClassName,
    {
        let cls = &self.jni_ref().find_class(other.class_name()).unwrap();
        self.jni_ref()
            .is_instance_of(&self.jni_object(), cls)
            .unwrap()
    }
}
/// This interface is implemented by all objects that can provide metadata about themselves.
///
/// This is a representation of an abstract class.
#[repr(C)]
pub struct Metadatable<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for Metadatable<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}

impl<'mc> JNIInstantiatable<'mc> for Metadatable<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Metadatable from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/metadata/Metadatable")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Metadatable object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> Metadatable<'mc> {
    pub fn set_metadata(
        &self,
        arg0: impl Into<String>,
        arg1: impl Into<crate::metadata::MetadataValue<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/String;Lorg/bukkit/metadata/MetadataValue;)V");
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(arg0.into())?,
        ));
        let val_2 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg1.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setMetadata",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
            ],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn get_metadata(
        &self,
        arg0: impl Into<String>,
    ) -> Result<Vec<crate::metadata::MetadataValue<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/String;)Ljava/util/List;");
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(arg0.into())?,
        ));
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getMetadata",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let list = blackboxmc_java::util::JavaList::from_raw(&self.jni_ref(), res.l()?)?;
        let iter = list.iterator()?;
        while iter.has_next()? {
            let obj = iter.next()?;
            new_vec.push(crate::metadata::MetadataValue::from_raw(
                &self.jni_ref(),
                obj,
            )?);
        }
        Ok(new_vec)
    }

    pub fn has_metadata(
        &self,
        arg0: impl Into<String>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/String;)Z");
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(arg0.into())?,
        ));
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "hasMetadata",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn remove_metadata(
        &self,
        arg0: impl Into<String>,
        arg1: impl Into<crate::plugin::Plugin<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/String;Lorg/bukkit/plugin/Plugin;)V");
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(arg0.into())?,
        ));
        let val_2 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg1.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeMetadata",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
            ],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn instance_of<A>(&self, other: A) -> bool
    where
        A: blackboxmc_general::JNIProvidesClassName,
    {
        let cls = &self.jni_ref().find_class(other.class_name()).unwrap();
        self.jni_ref()
            .is_instance_of(&self.jni_object(), cls)
            .unwrap()
    }
}

#[repr(C)]
pub struct MetadataStoreBase<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for MetadataStoreBase<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}

impl<'mc> JNIInstantiatable<'mc> for MetadataStoreBase<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate MetadataStoreBase from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/metadata/MetadataStoreBase")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a MetadataStoreBase object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> MetadataStoreBase<'mc> {
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<crate::metadata::MetadataStoreBase<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let cls = jni.find_class("org/bukkit/metadata/MetadataStoreBase");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(cls, sig.as_str(), vec![]);
        let res = jni.translate_error_no_gen(res)?;
        crate::metadata::MetadataStoreBase::from_raw(&jni, res)
    }

    pub fn set_metadata(
        &self,
        arg0: jni::objects::JObject<'mc>,
        arg1: impl Into<String>,
        arg2: impl Into<crate::metadata::MetadataValue<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from(
            "(Ljava/lang/Object;Ljava/lang/String;Lorg/bukkit/metadata/MetadataValue;)V",
        );
        let val_1 = jni::objects::JValueGen::Object(arg0);
        let val_2 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(arg1.into())?,
        ));
        let val_3 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg2.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setMetadata",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
                jni::objects::JValueGen::from(val_3),
            ],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn get_metadata(
        &self,
        arg0: jni::objects::JObject<'mc>,
        arg1: impl Into<String>,
    ) -> Result<Vec<crate::metadata::MetadataValue<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/Object;Ljava/lang/String;)Ljava/util/List;");
        let val_1 = jni::objects::JValueGen::Object(arg0);
        let val_2 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(arg1.into())?,
        ));
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getMetadata",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let list = blackboxmc_java::util::JavaList::from_raw(&self.0, res.l()?)?;
        let iter = list.iterator()?;
        while iter.has_next()? {
            let obj = iter.next()?;
            new_vec.push(crate::metadata::MetadataValue::from_raw(&self.0, obj)?);
        }
        Ok(new_vec)
    }

    pub fn has_metadata(
        &self,
        arg0: jni::objects::JObject<'mc>,
        arg1: impl Into<String>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/Object;Ljava/lang/String;)Z");
        let val_1 = jni::objects::JValueGen::Object(arg0);
        let val_2 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(arg1.into())?,
        ));
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "hasMetadata",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn remove_metadata(
        &self,
        arg0: jni::objects::JObject<'mc>,
        arg1: impl Into<String>,
        arg2: impl Into<crate::plugin::Plugin<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/Object;Ljava/lang/String;Lorg/bukkit/plugin/Plugin;)V");
        let val_1 = jni::objects::JValueGen::Object(arg0);
        let val_2 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(arg1.into())?,
        ));
        let val_3 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg2.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeMetadata",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
                jni::objects::JValueGen::from(val_3),
            ],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn invalidate_all(
        &self,
        arg0: impl Into<crate::plugin::Plugin<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/plugin/Plugin;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "invalidateAll",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn wait_with_long(
        &self,
        arg0: std::option::Option<i64>,
        arg1: std::option::Option<i32>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "J";
            let val_1 = jni::objects::JValueGen::Long(a.into());
            args.push(val_1);
        }
        if let Some(a) = arg1 {
            sig += "I";
            let val_2 = jni::objects::JValueGen::Int(a.into());
            args.push(val_2);
        }
        sig += ")V";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "wait", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn equals(
        &self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/Object;)Z");
        let val_1 = jni::objects::JValueGen::Object(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "equals",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    #[doc(hidden)]
    pub fn internal_to_string(&self) -> Result<String, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/String;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "toString", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }

    pub fn hash_code(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hashCode", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }

    pub fn class(&self) -> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/Class;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getClass", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(unsafe { jni::objects::JClass::from_raw(res.as_jni().l) })
    }

    pub fn notify(&self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notify", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn notify_all(&self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notifyAll", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn instance_of<A>(&self, other: A) -> bool
    where
        A: blackboxmc_general::JNIProvidesClassName,
    {
        let cls = &self.jni_ref().find_class(other.class_name()).unwrap();
        self.jni_ref()
            .is_instance_of(&self.jni_object(), cls)
            .unwrap()
    }
}

impl<'mc> std::string::ToString for MetadataStoreBase<'mc> {
    fn to_string(&self) -> String {
        match &self.internal_to_string() {
            Ok(a) => a.clone(),
            Err(err) => format!("Error calling MetadataStoreBase.toString: {}", err),
        }
    }
}

/// The LazyMetadataValue class implements a type of metadata that is not computed until another plugin asks for it.
/// <p>By making metadata values lazy, no computation is done by the providing plugin until absolutely necessary (if ever). Additionally, LazyMetadataValue objects cache their values internally unless overridden by a <a href="LazyMetadataValue.CacheStrategy.html" title="enum in org.bukkit.metadata"><code>LazyMetadataValue.CacheStrategy</code></a> or invalidated at the individual or plugin level. Once invalidated, the LazyMetadataValue will recompute its value when asked.</p>
#[repr(C)]
pub struct LazyMetadataValue<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
#[derive(PartialEq, Eq)]
pub enum LazyMetadataValueCacheStrategyEnum {
    CacheAfterFirstEval,
    NeverCache,
    CacheEternally,
}
impl std::fmt::Display for LazyMetadataValueCacheStrategyEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LazyMetadataValueCacheStrategyEnum::CacheAfterFirstEval => {
                f.write_str("CACHE_AFTER_FIRST_EVAL")
            }
            LazyMetadataValueCacheStrategyEnum::NeverCache => f.write_str("NEVER_CACHE"),
            LazyMetadataValueCacheStrategyEnum::CacheEternally => f.write_str("CACHE_ETERNALLY"),
        }
    }
}
impl<'mc> std::fmt::Display for LazyMetadataValueCacheStrategy<'mc> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.2.fmt(f)
    }
}
#[repr(C)]
pub struct LazyMetadataValueCacheStrategy<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
    pub LazyMetadataValueCacheStrategyEnum,
);
impl<'mc> std::ops::Deref for LazyMetadataValueCacheStrategy<'mc> {
    type Target = LazyMetadataValueCacheStrategyEnum;
    fn deref(&self) -> &Self::Target {
        return &self.2;
    }
}

impl<'mc> JNIRaw<'mc> for LazyMetadataValueCacheStrategy<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}

impl<'mc> JNIInstantiatableEnum<'mc> for LazyMetadataValueCacheStrategy<'mc> {
    type Enum = LazyMetadataValueCacheStrategyEnum;

    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,

        e: Self::Enum,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate LazyMetadataValueCacheStrategy from null object."
            )
            .into());
        }
        let (valid, name) =
            env.validate_name(&obj, "org/bukkit/metadata/LazyMetadataValue$CacheStrategy")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a LazyMetadataValueCacheStrategy object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj, e))
        }
    }
}

impl<'mc> LazyMetadataValueCacheStrategy<'mc> {
    pub const CACHE_AFTER_FIRST_EVAL: LazyMetadataValueCacheStrategyEnum =
        LazyMetadataValueCacheStrategyEnum::CacheAfterFirstEval;
    pub const NEVER_CACHE: LazyMetadataValueCacheStrategyEnum =
        LazyMetadataValueCacheStrategyEnum::NeverCache;
    pub const CACHE_ETERNALLY: LazyMetadataValueCacheStrategyEnum =
        LazyMetadataValueCacheStrategyEnum::CacheEternally;
    pub fn from_string(str: String) -> std::option::Option<LazyMetadataValueCacheStrategyEnum> {
        match str.as_str() {
            "CACHE_AFTER_FIRST_EVAL" => {
                Some(LazyMetadataValueCacheStrategyEnum::CacheAfterFirstEval)
            }
            "NEVER_CACHE" => Some(LazyMetadataValueCacheStrategyEnum::NeverCache),
            "CACHE_ETERNALLY" => Some(LazyMetadataValueCacheStrategyEnum::CacheEternally),
            _ => None,
        }
    }

    pub fn value_of(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<String>,
    ) -> Result<LazyMetadataValueCacheStrategy<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(jni.new_string(arg0.into())?);
        let cls = jni.find_class("org/bukkit/metadata/LazyMetadataValue$CacheStrategy");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(
            cls,
            "valueOf",
            "(Ljava/lang/String;)Lorg/bukkit/metadata/LazyMetadataValue$CacheStrategy;",
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        let raw_obj = obj;
        let variant = jni.call_method(&raw_obj, "toString", "()Ljava/lang/String;", vec![]);
        let variant = jni.translate_error(variant)?;
        let variant_str = jni
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        LazyMetadataValueCacheStrategy::from_raw(
            &jni,
            raw_obj,
            LazyMetadataValueCacheStrategy::from_string(variant_str)
                .ok_or(eyre::eyre!("String gaven for variant was invalid"))?,
        )
    }

    pub fn instance_of<A>(&self, other: A) -> bool
    where
        A: blackboxmc_general::JNIProvidesClassName,
    {
        let cls = &self.jni_ref().find_class(other.class_name()).unwrap();
        self.jni_ref()
            .is_instance_of(&self.jni_object(), cls)
            .unwrap()
    }
}

impl<'mc> JNIRaw<'mc> for LazyMetadataValue<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}

impl<'mc> JNIInstantiatable<'mc> for LazyMetadataValue<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate LazyMetadataValue from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/metadata/LazyMetadataValue")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a LazyMetadataValue object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> LazyMetadataValue<'mc> {
    pub fn value(&self) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/Object;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "value", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l()?)
    }

    pub fn invalidate(&self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "invalidate", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn as_string(&self) -> Result<String, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/String;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "asString", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }

    pub fn as_int(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "asInt", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }

    pub fn owning_plugin(
        &self,
    ) -> Result<Option<crate::plugin::Plugin<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/plugin/Plugin;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getOwningPlugin", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {
            return Ok(None);
        }
        Ok(Some(crate::plugin::Plugin::from_raw(
            &self.jni_ref(),
            unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) },
        )?))
    }

    pub fn as_float(&self) -> Result<f32, Box<dyn std::error::Error>> {
        let sig = String::from("()F");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "asFloat", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.f()?)
    }

    pub fn as_double(&self) -> Result<f64, Box<dyn std::error::Error>> {
        let sig = String::from("()D");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "asDouble", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.d()?)
    }

    pub fn as_long(&self) -> Result<i64, Box<dyn std::error::Error>> {
        let sig = String::from("()J");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "asLong", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.j()?)
    }

    pub fn as_short(&self) -> Result<i16, Box<dyn std::error::Error>> {
        let sig = String::from("()S");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "asShort", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.s()?)
    }

    pub fn as_byte(&self) -> Result<i8, Box<dyn std::error::Error>> {
        let sig = String::from("()B");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "asByte", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.b()?)
    }

    pub fn as_boolean(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "asBoolean", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn wait_with_long(
        &self,
        arg0: std::option::Option<i64>,
        arg1: std::option::Option<i32>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "J";
            let val_1 = jni::objects::JValueGen::Long(a.into());
            args.push(val_1);
        }
        if let Some(a) = arg1 {
            sig += "I";
            let val_2 = jni::objects::JValueGen::Int(a.into());
            args.push(val_2);
        }
        sig += ")V";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "wait", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn equals(
        &self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/Object;)Z");
        let val_1 = jni::objects::JValueGen::Object(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "equals",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    #[doc(hidden)]
    pub fn internal_to_string(&self) -> Result<String, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/String;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "toString", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }

    pub fn hash_code(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hashCode", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }

    pub fn class(&self) -> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/Class;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getClass", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(unsafe { jni::objects::JClass::from_raw(res.as_jni().l) })
    }

    pub fn notify(&self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notify", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn notify_all(&self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notifyAll", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn instance_of<A>(&self, other: A) -> bool
    where
        A: blackboxmc_general::JNIProvidesClassName,
    {
        let cls = &self.jni_ref().find_class(other.class_name()).unwrap();
        self.jni_ref()
            .is_instance_of(&self.jni_object(), cls)
            .unwrap()
    }
}

impl<'mc> std::string::ToString for LazyMetadataValue<'mc> {
    fn to_string(&self) -> String {
        match &self.internal_to_string() {
            Ok(a) => a.clone(),
            Err(err) => format!("Error calling LazyMetadataValue.toString: {}", err),
        }
    }
}

impl<'mc> Into<crate::metadata::MetadataValueAdapter<'mc>> for LazyMetadataValue<'mc> {
    fn into(self) -> crate::metadata::MetadataValueAdapter<'mc> {
        crate::metadata::MetadataValueAdapter::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting LazyMetadataValue into crate::metadata::MetadataValueAdapter")
    }
}
/// A FixedMetadataValue is a special case metadata item that contains the same value forever after initialization. Invalidating a FixedMetadataValue has no effect.
/// <p>This class extends LazyMetadataValue for historical reasons, even though it overrides all the implementation methods. it is possible that in the future that the inheritance hierarchy may change.</p>
#[repr(C)]
pub struct FixedMetadataValue<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for FixedMetadataValue<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}

impl<'mc> JNIInstantiatable<'mc> for FixedMetadataValue<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate FixedMetadataValue from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/metadata/FixedMetadataValue")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a FixedMetadataValue object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> FixedMetadataValue<'mc> {
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<crate::plugin::Plugin<'mc>>,
        arg1: jni::objects::JObject<'mc>,
    ) -> Result<crate::metadata::FixedMetadataValue<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/plugin/Plugin;Ljava/lang/Object;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let val_2 = jni::objects::JValueGen::Object(arg1);
        let cls = jni.find_class("org/bukkit/metadata/FixedMetadataValue");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(
            cls,
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
            ],
        );
        let res = jni.translate_error_no_gen(res)?;
        crate::metadata::FixedMetadataValue::from_raw(&jni, res)
    }

    pub fn value(&self) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/Object;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "value", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l()?)
    }

    pub fn invalidate(&self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "invalidate", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn as_string(&self) -> Result<String, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/String;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "asString", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }

    pub fn as_int(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "asInt", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }

    pub fn owning_plugin(
        &self,
    ) -> Result<Option<crate::plugin::Plugin<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/plugin/Plugin;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getOwningPlugin", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {
            return Ok(None);
        }
        Ok(Some(crate::plugin::Plugin::from_raw(
            &self.jni_ref(),
            unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) },
        )?))
    }

    pub fn as_float(&self) -> Result<f32, Box<dyn std::error::Error>> {
        let sig = String::from("()F");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "asFloat", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.f()?)
    }

    pub fn as_double(&self) -> Result<f64, Box<dyn std::error::Error>> {
        let sig = String::from("()D");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "asDouble", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.d()?)
    }

    pub fn as_long(&self) -> Result<i64, Box<dyn std::error::Error>> {
        let sig = String::from("()J");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "asLong", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.j()?)
    }

    pub fn as_short(&self) -> Result<i16, Box<dyn std::error::Error>> {
        let sig = String::from("()S");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "asShort", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.s()?)
    }

    pub fn as_byte(&self) -> Result<i8, Box<dyn std::error::Error>> {
        let sig = String::from("()B");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "asByte", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.b()?)
    }

    pub fn as_boolean(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "asBoolean", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn wait_with_long(
        &self,
        arg0: std::option::Option<i64>,
        arg1: std::option::Option<i32>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "J";
            let val_1 = jni::objects::JValueGen::Long(a.into());
            args.push(val_1);
        }
        if let Some(a) = arg1 {
            sig += "I";
            let val_2 = jni::objects::JValueGen::Int(a.into());
            args.push(val_2);
        }
        sig += ")V";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "wait", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn equals(
        &self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/Object;)Z");
        let val_1 = jni::objects::JValueGen::Object(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "equals",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    #[doc(hidden)]
    pub fn internal_to_string(&self) -> Result<String, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/String;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "toString", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }

    pub fn hash_code(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hashCode", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }

    pub fn class(&self) -> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/Class;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getClass", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(unsafe { jni::objects::JClass::from_raw(res.as_jni().l) })
    }

    pub fn notify(&self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notify", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn notify_all(&self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notifyAll", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn instance_of<A>(&self, other: A) -> bool
    where
        A: blackboxmc_general::JNIProvidesClassName,
    {
        let cls = &self.jni_ref().find_class(other.class_name()).unwrap();
        self.jni_ref()
            .is_instance_of(&self.jni_object(), cls)
            .unwrap()
    }
}

impl<'mc> std::string::ToString for FixedMetadataValue<'mc> {
    fn to_string(&self) -> String {
        match &self.internal_to_string() {
            Ok(a) => a.clone(),
            Err(err) => format!("Error calling FixedMetadataValue.toString: {}", err),
        }
    }
}

impl<'mc> Into<crate::metadata::LazyMetadataValue<'mc>> for FixedMetadataValue<'mc> {
    fn into(self) -> crate::metadata::LazyMetadataValue<'mc> {
        crate::metadata::LazyMetadataValue::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting FixedMetadataValue into crate::metadata::LazyMetadataValue")
    }
}
/// Optional base class for facilitating MetadataValue implementations.
/// <p>This provides all the conversion functions for MetadataValue so that writing an implementation of MetadataValue is as simple as implementing value() and invalidate().</p>
#[repr(C)]
pub struct MetadataValueAdapter<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for MetadataValueAdapter<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}

impl<'mc> JNIInstantiatable<'mc> for MetadataValueAdapter<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate MetadataValueAdapter from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/metadata/MetadataValueAdapter")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a MetadataValueAdapter object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> MetadataValueAdapter<'mc> {
    pub fn as_string(&self) -> Result<String, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/String;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "asString", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }

    pub fn as_int(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "asInt", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }

    pub fn owning_plugin(
        &self,
    ) -> Result<Option<crate::plugin::Plugin<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/plugin/Plugin;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getOwningPlugin", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {
            return Ok(None);
        }
        Ok(Some(crate::plugin::Plugin::from_raw(
            &self.jni_ref(),
            unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) },
        )?))
    }

    pub fn as_float(&self) -> Result<f32, Box<dyn std::error::Error>> {
        let sig = String::from("()F");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "asFloat", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.f()?)
    }

    pub fn as_double(&self) -> Result<f64, Box<dyn std::error::Error>> {
        let sig = String::from("()D");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "asDouble", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.d()?)
    }

    pub fn as_long(&self) -> Result<i64, Box<dyn std::error::Error>> {
        let sig = String::from("()J");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "asLong", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.j()?)
    }

    pub fn as_short(&self) -> Result<i16, Box<dyn std::error::Error>> {
        let sig = String::from("()S");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "asShort", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.s()?)
    }

    pub fn as_byte(&self) -> Result<i8, Box<dyn std::error::Error>> {
        let sig = String::from("()B");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "asByte", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.b()?)
    }

    pub fn as_boolean(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "asBoolean", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn wait_with_long(
        &self,
        arg0: std::option::Option<i64>,
        arg1: std::option::Option<i32>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "J";
            let val_1 = jni::objects::JValueGen::Long(a.into());
            args.push(val_1);
        }
        if let Some(a) = arg1 {
            sig += "I";
            let val_2 = jni::objects::JValueGen::Int(a.into());
            args.push(val_2);
        }
        sig += ")V";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "wait", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn equals(
        &self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/Object;)Z");
        let val_1 = jni::objects::JValueGen::Object(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "equals",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    #[doc(hidden)]
    pub fn internal_to_string(&self) -> Result<String, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/String;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "toString", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }

    pub fn hash_code(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hashCode", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }

    pub fn class(&self) -> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/Class;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getClass", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(unsafe { jni::objects::JClass::from_raw(res.as_jni().l) })
    }

    pub fn notify(&self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notify", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn notify_all(&self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notifyAll", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn value(&self) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/Object;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "value", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l()?)
    }

    pub fn invalidate(&self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "invalidate", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn instance_of<A>(&self, other: A) -> bool
    where
        A: blackboxmc_general::JNIProvidesClassName,
    {
        let cls = &self.jni_ref().find_class(other.class_name()).unwrap();
        self.jni_ref()
            .is_instance_of(&self.jni_object(), cls)
            .unwrap()
    }
}

impl<'mc> std::string::ToString for MetadataValueAdapter<'mc> {
    fn to_string(&self) -> String {
        match &self.internal_to_string() {
            Ok(a) => a.clone(),
            Err(err) => format!("Error calling MetadataValueAdapter.toString: {}", err),
        }
    }
}

impl<'mc> Into<crate::metadata::MetadataValue<'mc>> for MetadataValueAdapter<'mc> {
    fn into(self) -> crate::metadata::MetadataValue<'mc> {
        crate::metadata::MetadataValue::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting MetadataValueAdapter into crate::metadata::MetadataValue")
    }
}
#[derive(PartialEq, Eq)]
pub enum CacheStrategyEnum {
    CacheAfterFirstEval,
    NeverCache,
    CacheEternally,
}
impl std::fmt::Display for CacheStrategyEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CacheStrategyEnum::CacheAfterFirstEval => f.write_str("CACHE_AFTER_FIRST_EVAL"),
            CacheStrategyEnum::NeverCache => f.write_str("NEVER_CACHE"),
            CacheStrategyEnum::CacheEternally => f.write_str("CACHE_ETERNALLY"),
        }
    }
}
impl<'mc> std::fmt::Display for CacheStrategy<'mc> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.2.fmt(f)
    }
}
#[repr(C)]
pub struct CacheStrategy<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
    pub CacheStrategyEnum,
);
impl<'mc> std::ops::Deref for CacheStrategy<'mc> {
    type Target = CacheStrategyEnum;
    fn deref(&self) -> &Self::Target {
        return &self.2;
    }
}

impl<'mc> JNIRaw<'mc> for CacheStrategy<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}

impl<'mc> JNIInstantiatableEnum<'mc> for CacheStrategy<'mc> {
    type Enum = CacheStrategyEnum;

    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,

        e: Self::Enum,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate CacheStrategy from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/metadata/CacheStrategy")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a CacheStrategy object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj, e))
        }
    }
}

impl<'mc> CacheStrategy<'mc> {
    pub const CACHE_AFTER_FIRST_EVAL: CacheStrategyEnum = CacheStrategyEnum::CacheAfterFirstEval;
    pub const NEVER_CACHE: CacheStrategyEnum = CacheStrategyEnum::NeverCache;
    pub const CACHE_ETERNALLY: CacheStrategyEnum = CacheStrategyEnum::CacheEternally;
    pub fn from_string(str: String) -> std::option::Option<CacheStrategyEnum> {
        match str.as_str() {
            "CACHE_AFTER_FIRST_EVAL" => Some(CacheStrategyEnum::CacheAfterFirstEval),
            "NEVER_CACHE" => Some(CacheStrategyEnum::NeverCache),
            "CACHE_ETERNALLY" => Some(CacheStrategyEnum::CacheEternally),
            _ => None,
        }
    }

    pub fn value_of(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<String>,
    ) -> Result<CacheStrategy<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(jni.new_string(arg0.into())?);
        let cls = jni.find_class("org/bukkit/metadata/CacheStrategy");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(
            cls,
            "valueOf",
            "(Ljava/lang/String;)Lorg/bukkit/metadata/CacheStrategy;",
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        let raw_obj = obj;
        let variant = jni.call_method(&raw_obj, "toString", "()Ljava/lang/String;", vec![]);
        let variant = jni.translate_error(variant)?;
        let variant_str = jni
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        CacheStrategy::from_raw(
            &jni,
            raw_obj,
            CacheStrategy::from_string(variant_str)
                .ok_or(eyre::eyre!("String gaven for variant was invalid"))?,
        )
    }

    pub fn instance_of<A>(&self, other: A) -> bool
    where
        A: blackboxmc_general::JNIProvidesClassName,
    {
        let cls = &self.jni_ref().find_class(other.class_name()).unwrap();
        self.jni_ref()
            .is_instance_of(&self.jni_object(), cls)
            .unwrap()
    }
}
