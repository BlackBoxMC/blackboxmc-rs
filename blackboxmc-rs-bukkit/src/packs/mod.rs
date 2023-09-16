#![allow(deprecated)]
use blackboxmc_general::JNIInstantiatable;
use blackboxmc_general::JNIRaw;
use color_eyre::eyre::Result;
/// Show the compatibility of the data pack with the server.
pub enum DataPackCompatibility<'mc> {
    New {
        inner: DataPackCompatibilityStruct<'mc>,
    },
    Old {
        inner: DataPackCompatibilityStruct<'mc>,
    },
    Compatible {
        inner: DataPackCompatibilityStruct<'mc>,
    },
}
impl<'mc> std::fmt::Display for DataPackCompatibility<'mc> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DataPackCompatibility::New { .. } => f.write_str("NEW"),
            DataPackCompatibility::Old { .. } => f.write_str("OLD"),
            DataPackCompatibility::Compatible { .. } => f.write_str("COMPATIBLE"),
        }
    }
}

impl<'mc> DataPackCompatibility<'mc> {
    pub fn value_of(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<String>,
    ) -> Result<DataPackCompatibility<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(env.new_string(arg0.into())?);
        let cls = env.find_class("org/bukkit/packs/DataPack$Compatibility");
        let cls = env.translate_error_with_class(cls)?;
        let res = env.call_static_method(
            cls,
            "valueOf",
            "(Ljava/lang/String;)Lorg/bukkit/packs/DataPack$Compatibility;",
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
            "NEW" => Ok(DataPackCompatibility::New {
                inner: DataPackCompatibilityStruct::from_raw(env, obj)?,
            }),
            "OLD" => Ok(DataPackCompatibility::Old {
                inner: DataPackCompatibilityStruct::from_raw(env, obj)?,
            }),
            "COMPATIBLE" => Ok(DataPackCompatibility::Compatible {
                inner: DataPackCompatibilityStruct::from_raw(env, obj)?,
            }),

            _ => Err(eyre::eyre!("String gaven for variant was invalid").into()),
        }
    }
}

#[repr(C)]
pub struct DataPackCompatibilityStruct<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for DataPackCompatibility<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        match self {
            Self::New { inner } => inner.0.clone(),
            Self::Old { inner } => inner.0.clone(),
            Self::Compatible { inner } => inner.0.clone(),
        }
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        match self {
            Self::New { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Old { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Compatible { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
        }
    }
}
impl<'mc> JNIInstantiatable<'mc> for DataPackCompatibility<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate DataPackCompatibility from null object."
            )
            .into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/packs/DataPack$Compatibility")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a DataPackCompatibility object, got {}",
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
                "NEW" => Ok(DataPackCompatibility::New {
                    inner: DataPackCompatibilityStruct::from_raw(env, obj)?,
                }),
                "OLD" => Ok(DataPackCompatibility::Old {
                    inner: DataPackCompatibilityStruct::from_raw(env, obj)?,
                }),
                "COMPATIBLE" => Ok(DataPackCompatibility::Compatible {
                    inner: DataPackCompatibilityStruct::from_raw(env, obj)?,
                }),
                _ => Err(eyre::eyre!("String gaven for variant was invalid").into()),
            }
        }
    }
}

impl<'mc> JNIRaw<'mc> for DataPackCompatibilityStruct<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for DataPackCompatibilityStruct<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate DataPackCompatibilityStruct from null object."
            )
            .into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/packs/DataPack$Compatibility")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a DataPackCompatibilityStruct object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> DataPackCompatibilityStruct<'mc> {
    pub fn values(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<Vec<crate::packs::DataPackCompatibility<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()[Lorg/bukkit/packs/DataPack$Compatibility;");
        let cls = jni.find_class("org/bukkit/packs/DataPack$Compatibility");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "values", sig.as_str(), vec![]);
        let res = jni.translate_error(res)?;
        let arr = Into::<jni::objects::JObjectArray>::into(res.l()?);
        let len = jni.get_array_length(&arr)?;
        let mut vec = Vec::new();
        for i in 0..len {
            let res = jni.get_object_array_element(&arr, i)?;
            vec.push({ crate::packs::DataPackCompatibility::from_raw(&jni, res)? });
        }
        Ok(vec)
    }
    // SUPER CLASS: Enum

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
/// Represents a data pack.
///
/// This is a representation of an abstract class.
#[repr(C)]
pub struct DataPack<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for DataPack<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for DataPack<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate DataPack from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/packs/DataPack")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a DataPack object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> DataPack<'mc> {
    pub fn source(&self) -> Result<crate::packs::DataPackSource<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/packs/DataPack$Source;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getSource", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::packs::DataPackSource::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn description(&self) -> Result<String, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/String;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getDescription", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }

    pub fn requested_features(
        &self,
    ) -> Result<blackboxmc_java::util::JavaSet<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/Set;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getRequestedFeatures",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::util::JavaSet::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn title(&self) -> Result<String, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/String;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getTitle", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }

    pub fn pack_format(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getPackFormat", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }

    pub fn is_enabled(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isEnabled", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn is_required(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isRequired", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn compatibility(
        &self,
    ) -> Result<crate::packs::DataPackCompatibility<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/packs/DataPack$Compatibility;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getCompatibility",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::packs::DataPackCompatibility::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    pub fn key(&self) -> Result<crate::NamespacedKey<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = DataPack::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::Keyed = temp_clone.into();
        real.key()
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::Keyed<'mc>> for DataPack<'mc> {
    fn into(self) -> crate::Keyed<'mc> {
        crate::Keyed::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting DataPack into crate::Keyed")
    }
}
/// Manager of data packs.
///
/// This is a representation of an abstract class.
#[repr(C)]
pub struct DataPackManager<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for DataPackManager<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for DataPackManager<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate DataPackManager from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/packs/DataPackManager")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a DataPackManager object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> DataPackManager<'mc> {
    pub fn get_enabled_data_packs(
        &self,
        arg0: impl Into<crate::World<'mc>>,
    ) -> Result<Vec<crate::packs::DataPack<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/World;)Ljava/util/Collection;");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getEnabledDataPacks",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let col = blackboxmc_java::util::JavaCollection::from_raw(&self.jni_ref(), res.l()?)?;
        let iter = col.iterator()?;
        while iter.has_next()? {
            let obj = iter.next()?;
            new_vec.push(crate::packs::DataPack::from_raw(&self.jni_ref(), obj)?);
        }
        Ok(new_vec)
    }

    pub fn data_packs(
        &self,
    ) -> Result<Vec<crate::packs::DataPack<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/Collection;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getDataPacks", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let col = blackboxmc_java::util::JavaCollection::from_raw(&self.jni_ref(), res.l()?)?;
        let iter = col.iterator()?;
        while iter.has_next()? {
            let obj = iter.next()?;
            new_vec.push(crate::packs::DataPack::from_raw(&self.jni_ref(), obj)?);
        }
        Ok(new_vec)
    }

    pub fn get_data_pack(
        &self,
        arg0: impl Into<crate::NamespacedKey<'mc>>,
    ) -> Result<crate::packs::DataPack<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/NamespacedKey;)Lorg/bukkit/packs/DataPack;");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getDataPack",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::packs::DataPack::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn get_disabled_data_packs(
        &self,
        arg0: impl Into<crate::World<'mc>>,
    ) -> Result<Vec<crate::packs::DataPack<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/World;)Ljava/util/Collection;");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getDisabledDataPacks",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let col = blackboxmc_java::util::JavaCollection::from_raw(&self.jni_ref(), res.l()?)?;
        let iter = col.iterator()?;
        while iter.has_next()? {
            let obj = iter.next()?;
            new_vec.push(crate::packs::DataPack::from_raw(&self.jni_ref(), obj)?);
        }
        Ok(new_vec)
    }

    pub fn is_enabled_by_feature_with_entity_type(
        &self,
        arg0: impl Into<crate::entity::EntityType<'mc>>,
        arg1: impl Into<crate::World<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/entity/EntityType;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        sig += "Lorg/bukkit/World;";
        let val_2 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg1.into().jni_object().clone())
        });
        args.push(val_2);
        sig += ")Z";
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "isEnabledByFeature",
            sig.as_str(),
            args,
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
/// Represent the source of a data pack.
pub enum DataPackSource<'mc> {
    Default { inner: DataPackSourceStruct<'mc> },
    BuiltIn { inner: DataPackSourceStruct<'mc> },
    Feature { inner: DataPackSourceStruct<'mc> },
    World { inner: DataPackSourceStruct<'mc> },
    Server { inner: DataPackSourceStruct<'mc> },
}
impl<'mc> std::fmt::Display for DataPackSource<'mc> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DataPackSource::Default { .. } => f.write_str("DEFAULT"),
            DataPackSource::BuiltIn { .. } => f.write_str("BUILT_IN"),
            DataPackSource::Feature { .. } => f.write_str("FEATURE"),
            DataPackSource::World { .. } => f.write_str("WORLD"),
            DataPackSource::Server { .. } => f.write_str("SERVER"),
        }
    }
}

impl<'mc> DataPackSource<'mc> {
    pub fn value_of(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<String>,
    ) -> Result<DataPackSource<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(env.new_string(arg0.into())?);
        let cls = env.find_class("org/bukkit/packs/DataPack$Source");
        let cls = env.translate_error_with_class(cls)?;
        let res = env.call_static_method(
            cls,
            "valueOf",
            "(Ljava/lang/String;)Lorg/bukkit/packs/DataPack$Source;",
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
            "DEFAULT" => Ok(DataPackSource::Default {
                inner: DataPackSourceStruct::from_raw(env, obj)?,
            }),
            "BUILT_IN" => Ok(DataPackSource::BuiltIn {
                inner: DataPackSourceStruct::from_raw(env, obj)?,
            }),
            "FEATURE" => Ok(DataPackSource::Feature {
                inner: DataPackSourceStruct::from_raw(env, obj)?,
            }),
            "WORLD" => Ok(DataPackSource::World {
                inner: DataPackSourceStruct::from_raw(env, obj)?,
            }),
            "SERVER" => Ok(DataPackSource::Server {
                inner: DataPackSourceStruct::from_raw(env, obj)?,
            }),

            _ => Err(eyre::eyre!("String gaven for variant was invalid").into()),
        }
    }
}

#[repr(C)]
pub struct DataPackSourceStruct<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for DataPackSource<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        match self {
            Self::Default { inner } => inner.0.clone(),
            Self::BuiltIn { inner } => inner.0.clone(),
            Self::Feature { inner } => inner.0.clone(),
            Self::World { inner } => inner.0.clone(),
            Self::Server { inner } => inner.0.clone(),
        }
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        match self {
            Self::Default { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::BuiltIn { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Feature { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::World { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Server { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
        }
    }
}
impl<'mc> JNIInstantiatable<'mc> for DataPackSource<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate DataPackSource from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/packs/DataPack$Source")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a DataPackSource object, got {}",
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
                "DEFAULT" => Ok(DataPackSource::Default {
                    inner: DataPackSourceStruct::from_raw(env, obj)?,
                }),
                "BUILT_IN" => Ok(DataPackSource::BuiltIn {
                    inner: DataPackSourceStruct::from_raw(env, obj)?,
                }),
                "FEATURE" => Ok(DataPackSource::Feature {
                    inner: DataPackSourceStruct::from_raw(env, obj)?,
                }),
                "WORLD" => Ok(DataPackSource::World {
                    inner: DataPackSourceStruct::from_raw(env, obj)?,
                }),
                "SERVER" => Ok(DataPackSource::Server {
                    inner: DataPackSourceStruct::from_raw(env, obj)?,
                }),
                _ => Err(eyre::eyre!("String gaven for variant was invalid").into()),
            }
        }
    }
}

impl<'mc> JNIRaw<'mc> for DataPackSourceStruct<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for DataPackSourceStruct<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate DataPackSourceStruct from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/packs/DataPack$Source")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a DataPackSourceStruct object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> DataPackSourceStruct<'mc> {
    pub fn values(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<Vec<crate::packs::DataPackSource<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()[Lorg/bukkit/packs/DataPack$Source;");
        let cls = jni.find_class("org/bukkit/packs/DataPack$Source");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "values", sig.as_str(), vec![]);
        let res = jni.translate_error(res)?;
        let arr = Into::<jni::objects::JObjectArray>::into(res.l()?);
        let len = jni.get_array_length(&arr)?;
        let mut vec = Vec::new();
        for i in 0..len {
            let res = jni.get_object_array_element(&arr, i)?;
            vec.push({ crate::packs::DataPackSource::from_raw(&jni, res)? });
        }
        Ok(vec)
    }
    // SUPER CLASS: Enum

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
pub enum Compatibility<'mc> {
    New { inner: CompatibilityStruct<'mc> },
    Old { inner: CompatibilityStruct<'mc> },
    Compatible { inner: CompatibilityStruct<'mc> },
}
impl<'mc> std::fmt::Display for Compatibility<'mc> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Compatibility::New { .. } => f.write_str("NEW"),
            Compatibility::Old { .. } => f.write_str("OLD"),
            Compatibility::Compatible { .. } => f.write_str("COMPATIBLE"),
        }
    }
}

impl<'mc> Compatibility<'mc> {
    pub fn value_of(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<String>,
    ) -> Result<Compatibility<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(env.new_string(arg0.into())?);
        let cls = env.find_class("org/bukkit/packs/Compatibility");
        let cls = env.translate_error_with_class(cls)?;
        let res = env.call_static_method(
            cls,
            "valueOf",
            "(Ljava/lang/String;)Lorg/bukkit/packs/Compatibility;",
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
            "NEW" => Ok(Compatibility::New {
                inner: CompatibilityStruct::from_raw(env, obj)?,
            }),
            "OLD" => Ok(Compatibility::Old {
                inner: CompatibilityStruct::from_raw(env, obj)?,
            }),
            "COMPATIBLE" => Ok(Compatibility::Compatible {
                inner: CompatibilityStruct::from_raw(env, obj)?,
            }),

            _ => Err(eyre::eyre!("String gaven for variant was invalid").into()),
        }
    }
}

#[repr(C)]
pub struct CompatibilityStruct<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for Compatibility<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        match self {
            Self::New { inner } => inner.0.clone(),
            Self::Old { inner } => inner.0.clone(),
            Self::Compatible { inner } => inner.0.clone(),
        }
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        match self {
            Self::New { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Old { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Compatible { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
        }
    }
}
impl<'mc> JNIInstantiatable<'mc> for Compatibility<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Compatibility from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/packs/Compatibility")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Compatibility object, got {}",
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
                "NEW" => Ok(Compatibility::New {
                    inner: CompatibilityStruct::from_raw(env, obj)?,
                }),
                "OLD" => Ok(Compatibility::Old {
                    inner: CompatibilityStruct::from_raw(env, obj)?,
                }),
                "COMPATIBLE" => Ok(Compatibility::Compatible {
                    inner: CompatibilityStruct::from_raw(env, obj)?,
                }),
                _ => Err(eyre::eyre!("String gaven for variant was invalid").into()),
            }
        }
    }
}

impl<'mc> JNIRaw<'mc> for CompatibilityStruct<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for CompatibilityStruct<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate CompatibilityStruct from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/packs/Compatibility")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a CompatibilityStruct object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> CompatibilityStruct<'mc> {
    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
pub enum Source<'mc> {
    Default { inner: SourceStruct<'mc> },
    BuiltIn { inner: SourceStruct<'mc> },
    Feature { inner: SourceStruct<'mc> },
    World { inner: SourceStruct<'mc> },
    Server { inner: SourceStruct<'mc> },
}
impl<'mc> std::fmt::Display for Source<'mc> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Source::Default { .. } => f.write_str("DEFAULT"),
            Source::BuiltIn { .. } => f.write_str("BUILT_IN"),
            Source::Feature { .. } => f.write_str("FEATURE"),
            Source::World { .. } => f.write_str("WORLD"),
            Source::Server { .. } => f.write_str("SERVER"),
        }
    }
}

impl<'mc> Source<'mc> {
    pub fn value_of(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<String>,
    ) -> Result<Source<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(env.new_string(arg0.into())?);
        let cls = env.find_class("org/bukkit/packs/Source");
        let cls = env.translate_error_with_class(cls)?;
        let res = env.call_static_method(
            cls,
            "valueOf",
            "(Ljava/lang/String;)Lorg/bukkit/packs/Source;",
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
            "DEFAULT" => Ok(Source::Default {
                inner: SourceStruct::from_raw(env, obj)?,
            }),
            "BUILT_IN" => Ok(Source::BuiltIn {
                inner: SourceStruct::from_raw(env, obj)?,
            }),
            "FEATURE" => Ok(Source::Feature {
                inner: SourceStruct::from_raw(env, obj)?,
            }),
            "WORLD" => Ok(Source::World {
                inner: SourceStruct::from_raw(env, obj)?,
            }),
            "SERVER" => Ok(Source::Server {
                inner: SourceStruct::from_raw(env, obj)?,
            }),

            _ => Err(eyre::eyre!("String gaven for variant was invalid").into()),
        }
    }
}

#[repr(C)]
pub struct SourceStruct<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for Source<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        match self {
            Self::Default { inner } => inner.0.clone(),
            Self::BuiltIn { inner } => inner.0.clone(),
            Self::Feature { inner } => inner.0.clone(),
            Self::World { inner } => inner.0.clone(),
            Self::Server { inner } => inner.0.clone(),
        }
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        match self {
            Self::Default { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::BuiltIn { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Feature { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::World { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Server { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
        }
    }
}
impl<'mc> JNIInstantiatable<'mc> for Source<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Source from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/packs/Source")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Source object, got {}",
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
                "DEFAULT" => Ok(Source::Default {
                    inner: SourceStruct::from_raw(env, obj)?,
                }),
                "BUILT_IN" => Ok(Source::BuiltIn {
                    inner: SourceStruct::from_raw(env, obj)?,
                }),
                "FEATURE" => Ok(Source::Feature {
                    inner: SourceStruct::from_raw(env, obj)?,
                }),
                "WORLD" => Ok(Source::World {
                    inner: SourceStruct::from_raw(env, obj)?,
                }),
                "SERVER" => Ok(Source::Server {
                    inner: SourceStruct::from_raw(env, obj)?,
                }),
                _ => Err(eyre::eyre!("String gaven for variant was invalid").into()),
            }
        }
    }
}

impl<'mc> JNIRaw<'mc> for SourceStruct<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for SourceStruct<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate SourceStruct from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/packs/Source")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a SourceStruct object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> SourceStruct<'mc> {
    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
