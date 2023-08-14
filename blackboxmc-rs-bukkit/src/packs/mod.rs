#![allow(deprecated)]
use blackboxmc_general::JNIRaw;
use color_eyre::eyre::Result;
/// Show the compatibility of the data pack with the server.
#[derive(PartialEq, Eq)]
pub enum DataPackCompatibilityEnum {
    New,
    Old,
    Compatible,
}
impl std::fmt::Display for DataPackCompatibilityEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DataPackCompatibilityEnum::New => f.write_str("NEW"),
            DataPackCompatibilityEnum::Old => f.write_str("OLD"),
            DataPackCompatibilityEnum::Compatible => f.write_str("COMPATIBLE"),
        }
    }
}
pub struct DataPackCompatibility<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
    pub DataPackCompatibilityEnum,
);
impl<'mc> std::ops::Deref for DataPackCompatibility<'mc> {
    type Target = DataPackCompatibilityEnum;
    fn deref(&self) -> &Self::Target {
        return &self.2;
    }
}
impl<'mc> JNIRaw<'mc> for DataPackCompatibility<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> DataPackCompatibility<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
        e: DataPackCompatibilityEnum,
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
            Ok(Self(env.clone(), obj, e))
        }
    }
    pub const NEW: DataPackCompatibilityEnum = DataPackCompatibilityEnum::New;
    pub const OLD: DataPackCompatibilityEnum = DataPackCompatibilityEnum::Old;
    pub const COMPATIBLE: DataPackCompatibilityEnum = DataPackCompatibilityEnum::Compatible;
    pub fn from_string(str: String) -> std::option::Option<DataPackCompatibilityEnum> {
        match str.as_str() {
            "NEW" => Some(DataPackCompatibilityEnum::New),
            "OLD" => Some(DataPackCompatibilityEnum::Old),
            "COMPATIBLE" => Some(DataPackCompatibilityEnum::Compatible),
            _ => None,
        }
    }

    pub fn value_of(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<String>,
    ) -> Result<DataPackCompatibility<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(jni.new_string(arg0.into())?);
        let cls = jni.find_class("org/bukkit/packs/DataPack$Compatibility");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(
            cls,
            "valueOf",
            "(Ljava/lang/String;)Lorg/bukkit/packs/DataPack$Compatibility;",
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
        DataPackCompatibility::from_raw(
            &jni,
            raw_obj,
            DataPackCompatibility::from_string(variant_str)
                .ok_or(eyre::eyre!("String gaven for variant was invalid"))?,
        )
    }

    //
}
/// Represents a data pack.
///
/// This is a representation of an abstract class.
pub struct DataPack<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> DataPack<'mc> {
    pub fn from_raw(
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
    //

    pub fn source(
        &mut self,
    ) -> Result<crate::packs::DataPackSource<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/packs/DataPack$Source;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getSource", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant =
            self.jni_ref()
                .call_method(&raw_obj, "toString", "()Ljava/lang/String;", vec![]);
        let variant = self.jni_ref().translate_error(variant)?;
        let variant_str = self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::packs::DataPackSource::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::packs::DataPackSource::from_string(variant_str)
                .ok_or(eyre::eyre!("String gaven for variant was invalid"))?,
        )
    }
    //

    pub fn description(&mut self) -> Result<String, Box<dyn std::error::Error>> {
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
    //

    pub fn requested_features(
        &mut self,
    ) -> Result<blackboxmc_java::JavaSet<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/Set;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getRequestedFeatures",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::JavaSet::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn title(&mut self) -> Result<String, Box<dyn std::error::Error>> {
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
    //

    pub fn pack_format(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getPackFormat", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    //

    pub fn is_enabled(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isEnabled", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn is_required(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isRequired", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn compatibility(
        &mut self,
    ) -> Result<crate::packs::DataPackCompatibility<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/packs/DataPack$Compatibility;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getCompatibility",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant =
            self.jni_ref()
                .call_method(&raw_obj, "toString", "()Ljava/lang/String;", vec![]);
        let variant = self.jni_ref().translate_error(variant)?;
        let variant_str = self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::packs::DataPackCompatibility::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::packs::DataPackCompatibility::from_string(variant_str)
                .ok_or(eyre::eyre!("String gaven for variant was invalid"))?,
        )
    }
    //

    pub fn key(&mut self) -> Result<crate::NamespacedKey<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/NamespacedKey;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getKey", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::NamespacedKey::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
}
impl<'mc> JNIRaw<'mc> for DataPack<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
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
pub struct DataPackManager<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> DataPackManager<'mc> {
    pub fn from_raw(
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
    //

    pub fn get_enabled_data_packs(
        &mut self,
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
        let mut col = blackboxmc_java::JavaCollection::from_raw(&self.jni_ref(), res.l()?)?;
        let mut iter = blackboxmc_java::JavaIterator::from_raw(&self.jni_ref(), col.iterator()?)?;
        while iter.has_next()? {
            let obj = iter.next()?;
            new_vec.push(crate::packs::DataPack::from_raw(&self.jni_ref(), obj)?);
        }
        Ok(new_vec)
    }
    //

    pub fn data_packs(
        &mut self,
    ) -> Result<Vec<crate::packs::DataPack<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/Collection;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getDataPacks", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let mut col = blackboxmc_java::JavaCollection::from_raw(&self.jni_ref(), res.l()?)?;
        let mut iter = blackboxmc_java::JavaIterator::from_raw(&self.jni_ref(), col.iterator()?)?;
        while iter.has_next()? {
            let obj = iter.next()?;
            new_vec.push(crate::packs::DataPack::from_raw(&self.jni_ref(), obj)?);
        }
        Ok(new_vec)
    }
    //

    pub fn get_data_pack(
        &mut self,
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
    //

    pub fn get_disabled_data_packs(
        &mut self,
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
        let mut col = blackboxmc_java::JavaCollection::from_raw(&self.jni_ref(), res.l()?)?;
        let mut iter = blackboxmc_java::JavaIterator::from_raw(&self.jni_ref(), col.iterator()?)?;
        while iter.has_next()? {
            let obj = iter.next()?;
            new_vec.push(crate::packs::DataPack::from_raw(&self.jni_ref(), obj)?);
        }
        Ok(new_vec)
    }
    //

    pub fn is_enabled_by_feature_with_material(
        &mut self,
        arg0: impl Into<crate::entity::EntityType<'mc>>,
        arg1: std::option::Option<impl Into<crate::World<'mc>>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/entity/EntityType;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        if let Some(a) = arg1 {
            sig += "Lorg/bukkit/World;";
            let val_2 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(a.into().jni_object().clone())
            });
            args.push(val_2);
        }
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
}
impl<'mc> JNIRaw<'mc> for DataPackManager<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
/// Represent the source of a data pack.
#[derive(PartialEq, Eq)]
pub enum DataPackSourceEnum {
    Default,
    BuiltIn,
    Feature,
    World,
    Server,
}
impl std::fmt::Display for DataPackSourceEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DataPackSourceEnum::Default => f.write_str("DEFAULT"),
            DataPackSourceEnum::BuiltIn => f.write_str("BUILT_IN"),
            DataPackSourceEnum::Feature => f.write_str("FEATURE"),
            DataPackSourceEnum::World => f.write_str("WORLD"),
            DataPackSourceEnum::Server => f.write_str("SERVER"),
        }
    }
}
pub struct DataPackSource<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
    pub DataPackSourceEnum,
);
impl<'mc> std::ops::Deref for DataPackSource<'mc> {
    type Target = DataPackSourceEnum;
    fn deref(&self) -> &Self::Target {
        return &self.2;
    }
}
impl<'mc> JNIRaw<'mc> for DataPackSource<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> DataPackSource<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
        e: DataPackSourceEnum,
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
            Ok(Self(env.clone(), obj, e))
        }
    }
    pub const DEFAULT: DataPackSourceEnum = DataPackSourceEnum::Default;
    pub const BUILT_IN: DataPackSourceEnum = DataPackSourceEnum::BuiltIn;
    pub const FEATURE: DataPackSourceEnum = DataPackSourceEnum::Feature;
    pub const WORLD: DataPackSourceEnum = DataPackSourceEnum::World;
    pub const SERVER: DataPackSourceEnum = DataPackSourceEnum::Server;
    pub fn from_string(str: String) -> std::option::Option<DataPackSourceEnum> {
        match str.as_str() {
            "DEFAULT" => Some(DataPackSourceEnum::Default),
            "BUILT_IN" => Some(DataPackSourceEnum::BuiltIn),
            "FEATURE" => Some(DataPackSourceEnum::Feature),
            "WORLD" => Some(DataPackSourceEnum::World),
            "SERVER" => Some(DataPackSourceEnum::Server),
            _ => None,
        }
    }

    pub fn value_of(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<String>,
    ) -> Result<DataPackSource<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(jni.new_string(arg0.into())?);
        let cls = jni.find_class("org/bukkit/packs/DataPack$Source");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(
            cls,
            "valueOf",
            "(Ljava/lang/String;)Lorg/bukkit/packs/DataPack$Source;",
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
        DataPackSource::from_raw(
            &jni,
            raw_obj,
            DataPackSource::from_string(variant_str)
                .ok_or(eyre::eyre!("String gaven for variant was invalid"))?,
        )
    }

    //
}
#[derive(PartialEq, Eq)]
pub enum CompatibilityEnum {
    New,
    Old,
    Compatible,
}
impl std::fmt::Display for CompatibilityEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CompatibilityEnum::New => f.write_str("NEW"),
            CompatibilityEnum::Old => f.write_str("OLD"),
            CompatibilityEnum::Compatible => f.write_str("COMPATIBLE"),
        }
    }
}
pub struct Compatibility<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
    pub CompatibilityEnum,
);
impl<'mc> std::ops::Deref for Compatibility<'mc> {
    type Target = CompatibilityEnum;
    fn deref(&self) -> &Self::Target {
        return &self.2;
    }
}
impl<'mc> JNIRaw<'mc> for Compatibility<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Compatibility<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
        e: CompatibilityEnum,
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
            Ok(Self(env.clone(), obj, e))
        }
    }
    pub const NEW: CompatibilityEnum = CompatibilityEnum::New;
    pub const OLD: CompatibilityEnum = CompatibilityEnum::Old;
    pub const COMPATIBLE: CompatibilityEnum = CompatibilityEnum::Compatible;
    pub fn from_string(str: String) -> std::option::Option<CompatibilityEnum> {
        match str.as_str() {
            "NEW" => Some(CompatibilityEnum::New),
            "OLD" => Some(CompatibilityEnum::Old),
            "COMPATIBLE" => Some(CompatibilityEnum::Compatible),
            _ => None,
        }
    }

    pub fn value_of(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<String>,
    ) -> Result<Compatibility<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(jni.new_string(arg0.into())?);
        let cls = jni.find_class("org/bukkit/packs/Compatibility");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(
            cls,
            "valueOf",
            "(Ljava/lang/String;)Lorg/bukkit/packs/Compatibility;",
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
        Compatibility::from_raw(
            &jni,
            raw_obj,
            Compatibility::from_string(variant_str)
                .ok_or(eyre::eyre!("String gaven for variant was invalid"))?,
        )
    }
}
#[derive(PartialEq, Eq)]
pub enum SourceEnum {
    Default,
    BuiltIn,
    Feature,
    World,
    Server,
}
impl std::fmt::Display for SourceEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SourceEnum::Default => f.write_str("DEFAULT"),
            SourceEnum::BuiltIn => f.write_str("BUILT_IN"),
            SourceEnum::Feature => f.write_str("FEATURE"),
            SourceEnum::World => f.write_str("WORLD"),
            SourceEnum::Server => f.write_str("SERVER"),
        }
    }
}
pub struct Source<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
    pub SourceEnum,
);
impl<'mc> std::ops::Deref for Source<'mc> {
    type Target = SourceEnum;
    fn deref(&self) -> &Self::Target {
        return &self.2;
    }
}
impl<'mc> JNIRaw<'mc> for Source<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Source<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
        e: SourceEnum,
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
            Ok(Self(env.clone(), obj, e))
        }
    }
    pub const DEFAULT: SourceEnum = SourceEnum::Default;
    pub const BUILT_IN: SourceEnum = SourceEnum::BuiltIn;
    pub const FEATURE: SourceEnum = SourceEnum::Feature;
    pub const WORLD: SourceEnum = SourceEnum::World;
    pub const SERVER: SourceEnum = SourceEnum::Server;
    pub fn from_string(str: String) -> std::option::Option<SourceEnum> {
        match str.as_str() {
            "DEFAULT" => Some(SourceEnum::Default),
            "BUILT_IN" => Some(SourceEnum::BuiltIn),
            "FEATURE" => Some(SourceEnum::Feature),
            "WORLD" => Some(SourceEnum::World),
            "SERVER" => Some(SourceEnum::Server),
            _ => None,
        }
    }

    pub fn value_of(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<String>,
    ) -> Result<Source<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(jni.new_string(arg0.into())?);
        let cls = jni.find_class("org/bukkit/packs/Source");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(
            cls,
            "valueOf",
            "(Ljava/lang/String;)Lorg/bukkit/packs/Source;",
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
        Source::from_raw(
            &jni,
            raw_obj,
            Source::from_string(variant_str)
                .ok_or(eyre::eyre!("String gaven for variant was invalid"))?,
        )
    }
}
