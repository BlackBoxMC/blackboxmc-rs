#![allow(deprecated)]
use blackboxmc_general::JNIInstantiatable;
use blackboxmc_general::JNIRaw;
use color_eyre::eyre::Result;
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
    /// Initializes a FixedMetadataValue with an Object
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        owning_plugin: impl Into<crate::plugin::Plugin<'mc>>,
        value: jni::objects::JObject<'mc>,
    ) -> Result<crate::metadata::FixedMetadataValue<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/plugin/Plugin;Ljava/lang/Object;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(owning_plugin.into().jni_object().clone())
        });
        let val_2 = jni::objects::JValueGen::Object(value);
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

    pub fn invalidate(&self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "invalidate", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn value(&self) -> Result<Option<jni::objects::JObject<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/Object;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "value", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {
            return Ok(None);
        }
        Ok(Some(res.l()?))
    }
    // SUPER CLASS: org.bukkit.metadata.LazyMetadataValue ( ['invalidate', 'value'])

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::metadata::LazyMetadataValue<'mc>> for FixedMetadataValue<'mc> {
    fn into(self) -> crate::metadata::LazyMetadataValue<'mc> {
        crate::metadata::LazyMetadataValue::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting FixedMetadataValue into crate::metadata::LazyMetadataValue")
    }
}
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

    pub fn as_int(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "asInt", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
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
    /// Fetches the value of this metadata item.
    pub fn value(&self) -> Result<Option<jni::objects::JObject<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/Object;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "value", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {
            return Ok(None);
        }
        Ok(Some(res.l()?))
    }
    /// Invalidates this metadata item, forcing it to recompute when next
    /// accessed.
    pub fn invalidate(&self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "invalidate", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::metadata::MetadataValue<'mc>> for MetadataValueAdapter<'mc> {
    fn into(self) -> crate::metadata::MetadataValue<'mc> {
        crate::metadata::MetadataValue::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting MetadataValueAdapter into crate::metadata::MetadataValue")
    }
}
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
    /// Sets a metadata value in the implementing object's metadata store.
    pub fn set_metadata(
        &self,
        metadata_key: impl Into<String>,
        new_metadata_value: impl Into<crate::metadata::MetadataValue<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/String;Lorg/bukkit/metadata/MetadataValue;)V");
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(metadata_key.into())?,
        ));
        let val_2 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(new_metadata_value.into().jni_object().clone())
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
    /// Returns a list of previously set metadata values from the implementing
    /// object's metadata store.
    pub fn get_metadata(
        &self,
        metadata_key: impl Into<String>,
    ) -> Result<Vec<crate::metadata::MetadataValue<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/String;)Ljava/util/List;");
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(metadata_key.into())?,
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
    /// Tests to see whether the implementing object contains the given
    /// metadata value in its metadata store.
    pub fn has_metadata(
        &self,
        metadata_key: impl Into<String>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/String;)Z");
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(metadata_key.into())?,
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
    /// Removes the given metadata value from the implementing object's
    /// metadata store.
    pub fn remove_metadata(
        &self,
        metadata_key: impl Into<String>,
        owning_plugin: impl Into<crate::plugin::Plugin<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/String;Lorg/bukkit/plugin/Plugin;)V");
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(metadata_key.into())?,
        ));
        let val_2 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(owning_plugin.into().jni_object().clone())
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

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
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
    /// Adds a metadata value to an object. Each metadata value is owned by a
    /// specific {@link Plugin}. If a plugin has already added a metadata value
    /// to an object, that value will be replaced with the value of {@code
    /// newMetadataValue}. Multiple plugins can set independent values for the
    /// same {@code metadataKey} without conflict.
    ///
    /// Implementation note: I considered using a {@link
    /// java.util.concurrent.locks.ReadWriteLock} for controlling access to
    /// {@code metadataMap}, but decided that the added overhead wasn't worth
    /// the finer grained access control.
    ///
    /// Bukkit is almost entirely single threaded so locking overhead shouldn't
    /// pose a problem.
    pub fn set_metadata(
        &self,
        subject: jni::objects::JObject<'mc>,
        metadata_key: impl Into<String>,
        new_metadata_value: impl Into<crate::metadata::MetadataValue<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(LT;Ljava/lang/String;Lorg/bukkit/metadata/MetadataValue;)V");
        let val_1 = jni::objects::JValueGen::Object(subject);
        let val_2 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(metadata_key.into())?,
        ));
        let val_3 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(new_metadata_value.into().jni_object().clone())
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
    /// Returns all metadata values attached to an object. If multiple
    /// have attached metadata, each will value will be included.
    pub fn get_metadata(
        &self,
        subject: jni::objects::JObject<'mc>,
        metadata_key: impl Into<String>,
    ) -> Result<Vec<crate::metadata::MetadataValue<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("(LT;Ljava/lang/String;)Ljava/util/List;");
        let val_1 = jni::objects::JValueGen::Object(subject);
        let val_2 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(metadata_key.into())?,
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
    /// Tests to see if a metadata attribute has been set on an object.
    pub fn has_metadata(
        &self,
        subject: jni::objects::JObject<'mc>,
        metadata_key: impl Into<String>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(LT;Ljava/lang/String;)Z");
        let val_1 = jni::objects::JValueGen::Object(subject);
        let val_2 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(metadata_key.into())?,
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
    /// Removes a metadata item owned by a plugin from a subject.
    pub fn remove_metadata(
        &self,
        subject: jni::objects::JObject<'mc>,
        metadata_key: impl Into<String>,
        owning_plugin: impl Into<crate::plugin::Plugin<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(LT;Ljava/lang/String;Lorg/bukkit/plugin/Plugin;)V");
        let val_1 = jni::objects::JValueGen::Object(subject);
        let val_2 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(metadata_key.into())?,
        ));
        let val_3 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(owning_plugin.into().jni_object().clone())
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
    /// Invalidates all metadata in the metadata store that originates from the
    /// given plugin. Doing this will force each invalidated metadata item to
    /// be recalculated the next time it is accessed.
    pub fn invalidate_all(
        &self,
        owning_plugin: impl Into<crate::plugin::Plugin<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/plugin/Plugin;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(owning_plugin.into().jni_object().clone())
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

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
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
    /// Adds a metadata value to an object.
    pub fn set_metadata(
        &self,
        subject: jni::objects::JObject<'mc>,
        metadata_key: impl Into<String>,
        new_metadata_value: impl Into<crate::metadata::MetadataValue<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(LT;Ljava/lang/String;Lorg/bukkit/metadata/MetadataValue;)V");
        let val_1 = jni::objects::JValueGen::Object(subject);
        let val_2 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(metadata_key.into())?,
        ));
        let val_3 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(new_metadata_value.into().jni_object().clone())
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
    /// Returns all metadata values attached to an object. If multiple plugins
    /// have attached metadata, each will value will be included.
    pub fn get_metadata(
        &self,
        subject: jni::objects::JObject<'mc>,
        metadata_key: impl Into<String>,
    ) -> Result<Vec<crate::metadata::MetadataValue<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("(LT;Ljava/lang/String;)Ljava/util/List;");
        let val_1 = jni::objects::JValueGen::Object(subject);
        let val_2 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(metadata_key.into())?,
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
    /// Tests to see if a metadata attribute has been set on an object.
    pub fn has_metadata(
        &self,
        subject: jni::objects::JObject<'mc>,
        metadata_key: impl Into<String>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(LT;Ljava/lang/String;)Z");
        let val_1 = jni::objects::JValueGen::Object(subject);
        let val_2 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(metadata_key.into())?,
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
    /// Removes a metadata item owned by a plugin from a subject.
    pub fn remove_metadata(
        &self,
        subject: jni::objects::JObject<'mc>,
        metadata_key: impl Into<String>,
        owning_plugin: impl Into<crate::plugin::Plugin<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(LT;Ljava/lang/String;Lorg/bukkit/plugin/Plugin;)V");
        let val_1 = jni::objects::JValueGen::Object(subject);
        let val_2 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(metadata_key.into())?,
        ));
        let val_3 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(owning_plugin.into().jni_object().clone())
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
    /// Invalidates all metadata in the metadata store that originates from the
    /// given plugin. Doing this will force each invalidated metadata item to
    /// be recalculated the next time it is accessed.
    pub fn invalidate_all(
        &self,
        owning_plugin: impl Into<crate::plugin::Plugin<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/plugin/Plugin;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(owning_plugin.into().jni_object().clone())
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

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
#[repr(C)]
pub struct LazyMetadataValue<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

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
    pub fn value(&self) -> Result<Option<jni::objects::JObject<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/Object;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "value", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {
            return Ok(None);
        }
        Ok(Some(res.l()?))
    }

    pub fn invalidate(&self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "invalidate", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    // SUPER CLASS: org.bukkit.metadata.MetadataValueAdapter ( ['value', 'invalidate'])

    pub fn owning_plugin(
        &self,
    ) -> Result<Option<crate::plugin::Plugin<'mc>>, Box<dyn std::error::Error>> {
        let temp_clone = crate::metadata::MetadataValueAdapter::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::metadata::MetadataValueAdapter = temp_clone.into();
        real.owning_plugin()
    }

    pub fn as_int(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let temp_clone = crate::metadata::MetadataValueAdapter::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::metadata::MetadataValueAdapter = temp_clone.into();
        real.as_int()
    }

    pub fn as_float(&self) -> Result<f32, Box<dyn std::error::Error>> {
        let temp_clone = crate::metadata::MetadataValueAdapter::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::metadata::MetadataValueAdapter = temp_clone.into();
        real.as_float()
    }

    pub fn as_double(&self) -> Result<f64, Box<dyn std::error::Error>> {
        let temp_clone = crate::metadata::MetadataValueAdapter::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::metadata::MetadataValueAdapter = temp_clone.into();
        real.as_double()
    }

    pub fn as_long(&self) -> Result<i64, Box<dyn std::error::Error>> {
        let temp_clone = crate::metadata::MetadataValueAdapter::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::metadata::MetadataValueAdapter = temp_clone.into();
        real.as_long()
    }

    pub fn as_short(&self) -> Result<i16, Box<dyn std::error::Error>> {
        let temp_clone = crate::metadata::MetadataValueAdapter::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::metadata::MetadataValueAdapter = temp_clone.into();
        real.as_short()
    }

    pub fn as_byte(&self) -> Result<i8, Box<dyn std::error::Error>> {
        let temp_clone = crate::metadata::MetadataValueAdapter::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::metadata::MetadataValueAdapter = temp_clone.into();
        real.as_byte()
    }

    pub fn as_boolean(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::metadata::MetadataValueAdapter::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::metadata::MetadataValueAdapter = temp_clone.into();
        real.as_boolean()
    }

    pub fn as_string(&self) -> Result<String, Box<dyn std::error::Error>> {
        let temp_clone = crate::metadata::MetadataValueAdapter::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::metadata::MetadataValueAdapter = temp_clone.into();
        real.as_string()
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::metadata::MetadataValueAdapter<'mc>> for LazyMetadataValue<'mc> {
    fn into(self) -> crate::metadata::MetadataValueAdapter<'mc> {
        crate::metadata::MetadataValueAdapter::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting LazyMetadataValue into crate::metadata::MetadataValueAdapter")
    }
}
#[repr(C)]
pub struct MetadataEvaluationException<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for MetadataEvaluationException<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for MetadataEvaluationException<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate MetadataEvaluationException from null object."
            )
            .into());
        }
        let (valid, name) =
            env.validate_name(&obj, "org/bukkit/metadata/MetadataEvaluationException")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a MetadataEvaluationException object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> MetadataEvaluationException<'mc> {
    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
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
    /// Fetches the value of this metadata item.
    pub fn value(&self) -> Result<Option<jni::objects::JObject<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/Object;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "value", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {
            return Ok(None);
        }
        Ok(Some(res.l()?))
    }
    /// Attempts to convert the value of this metadata item into an int.
    pub fn as_int(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "asInt", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    /// Attempts to convert the value of this metadata item into a float.
    pub fn as_float(&self) -> Result<f32, Box<dyn std::error::Error>> {
        let sig = String::from("()F");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "asFloat", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.f()?)
    }
    /// Attempts to convert the value of this metadata item into a double.
    pub fn as_double(&self) -> Result<f64, Box<dyn std::error::Error>> {
        let sig = String::from("()D");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "asDouble", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.d()?)
    }
    /// Attempts to convert the value of this metadata item into a long.
    pub fn as_long(&self) -> Result<i64, Box<dyn std::error::Error>> {
        let sig = String::from("()J");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "asLong", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.j()?)
    }
    /// Attempts to convert the value of this metadata item into a short.
    pub fn as_short(&self) -> Result<i16, Box<dyn std::error::Error>> {
        let sig = String::from("()S");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "asShort", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.s()?)
    }
    /// Attempts to convert the value of this metadata item into a byte.
    pub fn as_byte(&self) -> Result<i8, Box<dyn std::error::Error>> {
        let sig = String::from("()B");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "asByte", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.b()?)
    }
    /// Attempts to convert the value of this metadata item into a boolean.
    pub fn as_boolean(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "asBoolean", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Attempts to convert the value of this metadata item into a string.
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
    /// Returns the {@link Plugin} that created this metadata item.
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
    /// Invalidates this metadata item, forcing it to recompute when next
    /// accessed.
    pub fn invalidate(&self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "invalidate", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
pub enum LazyMetadataValueCacheStrategy<'mc> {
    CacheAfterFirstEval {
        inner: LazyMetadataValueCacheStrategyStruct<'mc>,
    },
    NeverCache {
        inner: LazyMetadataValueCacheStrategyStruct<'mc>,
    },
    CacheEternally {
        inner: LazyMetadataValueCacheStrategyStruct<'mc>,
    },
}
impl<'mc> std::fmt::Display for LazyMetadataValueCacheStrategy<'mc> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LazyMetadataValueCacheStrategy::CacheAfterFirstEval { .. } => {
                f.write_str("CACHE_AFTER_FIRST_EVAL")
            }
            LazyMetadataValueCacheStrategy::NeverCache { .. } => f.write_str("NEVER_CACHE"),
            LazyMetadataValueCacheStrategy::CacheEternally { .. } => f.write_str("CACHE_ETERNALLY"),
        }
    }
}

impl<'mc> LazyMetadataValueCacheStrategy<'mc> {
    pub fn value_of(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<String>,
    ) -> Result<LazyMetadataValueCacheStrategy<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(env.new_string(arg0.into())?);
        let cls = env.find_class("org/bukkit/metadata/LazyMetadataValue/CacheStrategy");
        let cls = env.translate_error_with_class(cls)?;
        let res = env.call_static_method(
            cls,
            "valueOf",
            "(Ljava/lang/String;)Lorg/bukkit/metadata/LazyMetadataValue/CacheStrategy;",
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = env.translate_error(res)?;
        let obj = res.l()?;
        let variant = env.call_method(&obj, "toString", "()Ljava/lang/String;", vec![]);
        let variant = env.translate_error(variant)?;
        let variant_str = env
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        match variant_str.as_str() {
            "CACHE_AFTER_FIRST_EVAL" => Ok(LazyMetadataValueCacheStrategy::CacheAfterFirstEval {
                inner: LazyMetadataValueCacheStrategyStruct::from_raw(env, obj)?,
            }),
            "NEVER_CACHE" => Ok(LazyMetadataValueCacheStrategy::NeverCache {
                inner: LazyMetadataValueCacheStrategyStruct::from_raw(env, obj)?,
            }),
            "CACHE_ETERNALLY" => Ok(LazyMetadataValueCacheStrategy::CacheEternally {
                inner: LazyMetadataValueCacheStrategyStruct::from_raw(env, obj)?,
            }),

            _ => Err(eyre::eyre!("String gaven for variant was invalid").into()),
        }
    }
}

#[repr(C)]
pub struct LazyMetadataValueCacheStrategyStruct<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for LazyMetadataValueCacheStrategy<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        match self {
            Self::CacheAfterFirstEval { inner } => inner.0.clone(),
            Self::NeverCache { inner } => inner.0.clone(),
            Self::CacheEternally { inner } => inner.0.clone(),
        }
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        match self {
            Self::CacheAfterFirstEval { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::NeverCache { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::CacheEternally { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
        }
    }
}
impl<'mc> JNIInstantiatable<'mc> for LazyMetadataValueCacheStrategy<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate LazyMetadataValueCacheStrategy from null object."
            )
            .into());
        }
        let (valid, name) =
            env.validate_name(&obj, "org/bukkit/metadata/LazyMetadataValue/CacheStrategy")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a LazyMetadataValueCacheStrategy object, got {}",
                name
            )
            .into())
        } else {
            let variant = env.call_method(&obj, "toString", "()Ljava/lang/String;", vec![]);
            let variant = env.translate_error(variant)?;
            let variant_str = env
                .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
                .to_string_lossy()
                .to_string();
            match variant_str.as_str() {
                "CACHE_AFTER_FIRST_EVAL" => {
                    Ok(LazyMetadataValueCacheStrategy::CacheAfterFirstEval {
                        inner: LazyMetadataValueCacheStrategyStruct::from_raw(env, obj)?,
                    })
                }
                "NEVER_CACHE" => Ok(LazyMetadataValueCacheStrategy::NeverCache {
                    inner: LazyMetadataValueCacheStrategyStruct::from_raw(env, obj)?,
                }),
                "CACHE_ETERNALLY" => Ok(LazyMetadataValueCacheStrategy::CacheEternally {
                    inner: LazyMetadataValueCacheStrategyStruct::from_raw(env, obj)?,
                }),
                _ => Err(eyre::eyre!("String gaven for variant was invalid").into()),
            }
        }
    }
}

impl<'mc> JNIRaw<'mc> for LazyMetadataValueCacheStrategyStruct<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for LazyMetadataValueCacheStrategyStruct<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate LazyMetadataValueCacheStrategyStruct from null object."
            )
            .into());
        }
        let (valid, name) =
            env.validate_name(&obj, "org/bukkit/metadata/LazyMetadataValue/CacheStrategy")?;
        if !valid {
            Err(eyre::eyre!(
                    "Invalid argument passed. Expected a LazyMetadataValueCacheStrategyStruct object, got {}",
                    name
                )
                .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> LazyMetadataValueCacheStrategyStruct<'mc> {
    pub fn values(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<crate::metadata::LazyMetadataValueCacheStrategy<'mc>, Box<dyn std::error::Error>>
    {
        let sig = String::from("()Lorg/bukkit/metadata/LazyMetadataValue/CacheStrategy;");
        let cls = jni.find_class("org/bukkit/metadata/LazyMetadataValue/CacheStrategy");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "values", sig.as_str(), vec![]);
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        crate::metadata::LazyMetadataValueCacheStrategy::from_raw(&jni, obj)
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
#[repr(C)]
pub struct MetadataConversionException<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for MetadataConversionException<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for MetadataConversionException<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate MetadataConversionException from null object."
            )
            .into());
        }
        let (valid, name) =
            env.validate_name(&obj, "org/bukkit/metadata/MetadataConversionException")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a MetadataConversionException object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> MetadataConversionException<'mc> {
    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
