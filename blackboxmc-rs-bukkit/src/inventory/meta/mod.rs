#![allow(deprecated)]
use blackboxmc_general::JNIInstantiatable;
use blackboxmc_general::JNIRaw;
use color_eyre::eyre::Result;
/// Represents the generation (or level of copying) of a written book
pub enum BookMetaGeneration<'mc> {
    Original {
        inner: BookMetaGenerationStruct<'mc>,
    },
    CopyOfOriginal {
        inner: BookMetaGenerationStruct<'mc>,
    },
    CopyOfCopy {
        inner: BookMetaGenerationStruct<'mc>,
    },
    Tattered {
        inner: BookMetaGenerationStruct<'mc>,
    },
}
impl<'mc> std::fmt::Display for BookMetaGeneration<'mc> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BookMetaGeneration::Original { .. } => f.write_str("ORIGINAL"),
            BookMetaGeneration::CopyOfOriginal { .. } => f.write_str("COPY_OF_ORIGINAL"),
            BookMetaGeneration::CopyOfCopy { .. } => f.write_str("COPY_OF_COPY"),
            BookMetaGeneration::Tattered { .. } => f.write_str("TATTERED"),
        }
    }
}

impl<'mc> BookMetaGeneration<'mc> {
    pub fn value_of(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<String>,
    ) -> Result<BookMetaGeneration<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(env.new_string(arg0.into())?);
        let cls = env.find_class("org/bukkit/inventory/meta/BookMeta$Generation");
        let cls = env.translate_error_with_class(cls)?;
        let res = env.call_static_method(
            cls,
            "valueOf",
            "(Ljava/lang/String;)Lorg/bukkit/inventory/meta/BookMeta$Generation;",
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
            "ORIGINAL" => Ok(BookMetaGeneration::Original {
                inner: BookMetaGenerationStruct::from_raw(env, obj)?,
            }),
            "COPY_OF_ORIGINAL" => Ok(BookMetaGeneration::CopyOfOriginal {
                inner: BookMetaGenerationStruct::from_raw(env, obj)?,
            }),
            "COPY_OF_COPY" => Ok(BookMetaGeneration::CopyOfCopy {
                inner: BookMetaGenerationStruct::from_raw(env, obj)?,
            }),
            "TATTERED" => Ok(BookMetaGeneration::Tattered {
                inner: BookMetaGenerationStruct::from_raw(env, obj)?,
            }),

            _ => Err(eyre::eyre!("String gaven for variant was invalid").into()),
        }
    }
}

#[repr(C)]
pub struct BookMetaGenerationStruct<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for BookMetaGeneration<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        match self {
            Self::Original { inner } => inner.0.clone(),
            Self::CopyOfOriginal { inner } => inner.0.clone(),
            Self::CopyOfCopy { inner } => inner.0.clone(),
            Self::Tattered { inner } => inner.0.clone(),
        }
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        match self {
            Self::Original { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::CopyOfOriginal { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::CopyOfCopy { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::Tattered { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
        }
    }
}
impl<'mc> JNIInstantiatable<'mc> for BookMetaGeneration<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate BookMetaGeneration from null object.").into(),
            );
        }
        let (valid, name) =
            env.validate_name(&obj, "org/bukkit/inventory/meta/BookMeta$Generation")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a BookMetaGeneration object, got {}",
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
                "ORIGINAL" => Ok(BookMetaGeneration::Original {
                    inner: BookMetaGenerationStruct::from_raw(env, obj)?,
                }),
                "COPY_OF_ORIGINAL" => Ok(BookMetaGeneration::CopyOfOriginal {
                    inner: BookMetaGenerationStruct::from_raw(env, obj)?,
                }),
                "COPY_OF_COPY" => Ok(BookMetaGeneration::CopyOfCopy {
                    inner: BookMetaGenerationStruct::from_raw(env, obj)?,
                }),
                "TATTERED" => Ok(BookMetaGeneration::Tattered {
                    inner: BookMetaGenerationStruct::from_raw(env, obj)?,
                }),
                _ => Err(eyre::eyre!("String gaven for variant was invalid").into()),
            }
        }
    }
}

impl<'mc> JNIRaw<'mc> for BookMetaGenerationStruct<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for BookMetaGenerationStruct<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate BookMetaGenerationStruct from null object."
            )
            .into());
        }
        let (valid, name) =
            env.validate_name(&obj, "org/bukkit/inventory/meta/BookMeta$Generation")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a BookMetaGenerationStruct object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> BookMetaGenerationStruct<'mc> {
    pub fn values(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<Vec<crate::inventory::meta::BookMetaGeneration<'mc>>, Box<dyn std::error::Error>>
    {
        let sig = String::from("()[Lorg/bukkit/inventory/meta/BookMeta$Generation;");
        let cls = jni.find_class("org/bukkit/inventory/meta/BookMeta$Generation");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "values", sig.as_str(), vec![]);
        let res = jni.translate_error(res)?;
        let arr = Into::<jni::objects::JObjectArray>::into(res.l()?);
        let len = jni.get_array_length(&arr)?;
        let mut vec = Vec::new();
        for i in 0..len {
            let res = jni.get_object_array_element(&arr, i)?;
            vec.push({ crate::inventory::meta::BookMetaGeneration::from_raw(&jni, res)? });
        }
        Ok(vec)
    }
    // SUPER CLASS: Enum

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}

///
/// This is a representation of an abstract class.
#[repr(C)]
pub struct BlockDataMeta<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for BlockDataMeta<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for BlockDataMeta<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate BlockDataMeta from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/inventory/meta/BlockDataMeta")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a BlockDataMeta object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> BlockDataMeta<'mc> {
    pub fn get_block_data(
        &self,
        arg0: impl Into<crate::Material<'mc>>,
    ) -> Result<crate::block::data::BlockData<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/Material;)Lorg/bukkit/block/data/BlockData;");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getBlockData",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::block::data::BlockData::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn set_block_data(
        &self,
        arg0: impl Into<crate::block::data::BlockData<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/data/BlockData;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setBlockData",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn has_block_data(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "hasBlockData", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    pub fn display_name(&self) -> Result<String, Box<dyn std::error::Error>> {
        let temp_clone = BlockDataMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.display_name()
    }
    pub fn as_string(&self) -> Result<String, Box<dyn std::error::Error>> {
        let temp_clone = BlockDataMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.as_string()
    }
    pub fn has_display_name(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = BlockDataMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.has_display_name()
    }
    pub fn set_display_name(
        &self,
        arg0: impl Into<String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = BlockDataMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.set_display_name(arg0)
    }
    pub fn has_localized_name(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = BlockDataMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.has_localized_name()
    }
    pub fn localized_name(&self) -> Result<String, Box<dyn std::error::Error>> {
        let temp_clone = BlockDataMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.localized_name()
    }
    pub fn set_localized_name(
        &self,
        arg0: impl Into<String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = BlockDataMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.set_localized_name(arg0)
    }
    pub fn has_lore(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = BlockDataMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.has_lore()
    }
    pub fn lore(&self) -> Result<Option<Vec<String>>, Box<dyn std::error::Error>> {
        let temp_clone = BlockDataMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.lore()
    }
    pub fn set_lore(&self, arg0: Vec<impl Into<String>>) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = BlockDataMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.set_lore(arg0)
    }
    pub fn has_custom_model_data(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = BlockDataMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.has_custom_model_data()
    }
    pub fn custom_model_data(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let temp_clone = BlockDataMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.custom_model_data()
    }
    pub fn set_custom_model_data(&self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = BlockDataMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.set_custom_model_data(arg0)
    }
    pub fn has_enchants(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "hasEnchants", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    pub fn has_enchant(
        &self,
        arg0: impl Into<crate::enchantments::Enchantment<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = BlockDataMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.has_enchant(arg0)
    }
    pub fn get_enchant_level(
        &self,
        arg0: impl Into<crate::enchantments::Enchantment<'mc>>,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        let temp_clone = BlockDataMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.get_enchant_level(arg0)
    }
    pub fn enchants(
        &self,
    ) -> Result<blackboxmc_java::util::JavaMap<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/Map;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getEnchants", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::util::JavaMap::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    pub fn add_enchant(
        &self,
        arg0: impl Into<crate::enchantments::Enchantment<'mc>>,
        arg1: i32,
        arg2: bool,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = BlockDataMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.add_enchant(arg0, arg1, arg2)
    }
    pub fn remove_enchant(
        &self,
        arg0: impl Into<crate::enchantments::Enchantment<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = BlockDataMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.remove_enchant(arg0)
    }
    pub fn has_conflicting_enchant(
        &self,
        arg0: impl Into<crate::enchantments::Enchantment<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = BlockDataMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.has_conflicting_enchant(arg0)
    }
    pub fn add_item_flags(
        &self,
        arg0: Vec<crate::inventory::ItemFlag<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("([Lorg/bukkit/inventory/ItemFlag;)V");
        let arr = self.jni_ref().new_object_array(
            arg0.len() as i32,
            "org/bukkit/inventory/ItemFlag",
            jni::objects::JObject::null(),
        );
        let arr = self.jni_ref().translate_error_no_gen(arr)?;
        for i in 0..arg0.len() {
            let val_1 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(arg0.get(i).unwrap().jni_object().clone())
            });
            self.jni_ref()
                .set_object_array_element(&arr, i as i32, val_1.l()?)?;
        }
        let val_1 = jni::objects::JValueGen::Object(arr);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "addItemFlags",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1.l()?)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub fn remove_item_flags(
        &self,
        arg0: Vec<crate::inventory::ItemFlag<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("([Lorg/bukkit/inventory/ItemFlag;)V");
        let arr = self.jni_ref().new_object_array(
            arg0.len() as i32,
            "org/bukkit/inventory/ItemFlag",
            jni::objects::JObject::null(),
        );
        let arr = self.jni_ref().translate_error_no_gen(arr)?;
        for i in 0..arg0.len() {
            let val_1 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(arg0.get(i).unwrap().jni_object().clone())
            });
            self.jni_ref()
                .set_object_array_element(&arr, i as i32, val_1.l()?)?;
        }
        let val_1 = jni::objects::JValueGen::Object(arr);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeItemFlags",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1.l()?)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub fn item_flags(
        &self,
    ) -> Result<blackboxmc_java::util::JavaSet<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/Set;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getItemFlags", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::util::JavaSet::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    pub fn has_item_flag(
        &self,
        arg0: impl Into<crate::inventory::ItemFlag<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = BlockDataMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.has_item_flag(arg0)
    }
    pub fn is_unbreakable(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = BlockDataMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.is_unbreakable()
    }
    pub fn set_unbreakable(&self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = BlockDataMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.set_unbreakable(arg0)
    }
    pub fn has_attribute_modifiers(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "hasAttributeModifiers",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    pub fn attribute_modifiers(
        &self,
    ) -> Result<Option<jni::objects::JObject<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lcom/google/common/collect/Multimap;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getAttributeModifiers",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {
            return Ok(None);
        }
        Ok(Some(res.l()?))
    }
    pub fn add_attribute_modifier(
        &self,
        arg0: impl Into<crate::attribute::Attribute<'mc>>,
        arg1: impl Into<crate::attribute::AttributeModifier<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = BlockDataMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.add_attribute_modifier(arg0, arg1)
    }
    pub fn set_attribute_modifiers(
        &self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lcom/google/common/collect/Multimap;)V");
        let val_1 = jni::objects::JValueGen::Object(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setAttributeModifiers",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub fn remove_attribute_modifier_with_equipment_slot(
        &self,
        arg0: impl Into<crate::inventory::EquipmentSlot<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/inventory/EquipmentSlot;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        sig += ")Z";
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeAttributeModifier",
            sig.as_str(),
            args,
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    pub fn remove_attribute_modifier_with_attribute(
        &self,
        arg0: impl Into<crate::attribute::Attribute<'mc>>,
        arg1: std::option::Option<impl Into<crate::attribute::AttributeModifier<'mc>>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/attribute/Attribute;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        if let Some(a) = arg1 {
            sig += "Lorg/bukkit/attribute/AttributeModifier;";
            let val_2 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(a.into().jni_object().clone())
            });
            args.push(val_2);
        }
        sig += ")Z";
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeAttributeModifier",
            sig.as_str(),
            args,
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn custom_tag_container(
        &self,
    ) -> Result<crate::inventory::meta::tags::CustomItemTagContainer<'mc>, Box<dyn std::error::Error>>
    {
        let temp_clone = BlockDataMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.custom_tag_container()
    }

    pub fn set_version(&self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = BlockDataMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.set_version(arg0)
    }
    pub fn clone(&self) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = BlockDataMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.clone()
    }
    // SUPER CLASS: Cloneable
    // SUPER CLASS: ConfigurationSerializable
    pub fn serialize(
        &self,
    ) -> Result<blackboxmc_java::util::JavaMap<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::configuration::serialization::ConfigurationSerializable::from_raw(
            &self.0,
            unsafe { jni::objects::JObject::from_raw(self.1.clone()) },
        )?;
        let real: crate::configuration::serialization::ConfigurationSerializable =
            temp_clone.into();
        real.serialize()
    }
    // SUPER CLASS: PersistentDataHolder
    pub fn persistent_data_container(
        &self,
    ) -> Result<crate::persistence::PersistentDataContainer<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::persistence::PersistentDataHolder::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::persistence::PersistentDataHolder = temp_clone.into();
        real.persistent_data_container()
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::inventory::meta::ItemMeta<'mc>> for BlockDataMeta<'mc> {
    fn into(self) -> crate::inventory::meta::ItemMeta<'mc> {
        crate::inventory::meta::ItemMeta::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting BlockDataMeta into crate::inventory::meta::ItemMeta")
    }
}
/// Represents armor that an entity can equip.
/// <p><strong>Note: Armor trims are part of an experimental feature of Minecraft and hence subject to change.</strong></p>
///
/// This is a representation of an abstract class.
#[repr(C)]
pub struct ArmorMeta<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for ArmorMeta<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for ArmorMeta<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate ArmorMeta from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/inventory/meta/ArmorMeta")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a ArmorMeta object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> ArmorMeta<'mc> {
    pub fn has_trim(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasTrim", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn set_trim(
        &self,
        arg0: impl Into<crate::inventory::meta::trim::ArmorTrim<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/inventory/meta/trim/ArmorTrim;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setTrim",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn trim(
        &self,
    ) -> Result<Option<crate::inventory::meta::trim::ArmorTrim<'mc>>, Box<dyn std::error::Error>>
    {
        let sig = String::from("()Lorg/bukkit/inventory/meta/trim/ArmorTrim;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getTrim", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {
            return Ok(None);
        }
        Ok(Some(crate::inventory::meta::trim::ArmorTrim::from_raw(
            &self.jni_ref(),
            unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) },
        )?))
    }

    pub fn clone(
        &self,
    ) -> Result<crate::inventory::meta::ArmorMeta<'mc>, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Lorg/bukkit/inventory/meta/ArmorMeta;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "clone", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::meta::ArmorMeta::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    pub fn display_name(&self) -> Result<String, Box<dyn std::error::Error>> {
        let temp_clone = ArmorMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.display_name()
    }
    pub fn as_string(&self) -> Result<String, Box<dyn std::error::Error>> {
        let temp_clone = ArmorMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.as_string()
    }
    pub fn has_display_name(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = ArmorMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.has_display_name()
    }
    pub fn set_display_name(
        &self,
        arg0: impl Into<String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = ArmorMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.set_display_name(arg0)
    }
    pub fn has_localized_name(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = ArmorMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.has_localized_name()
    }
    pub fn localized_name(&self) -> Result<String, Box<dyn std::error::Error>> {
        let temp_clone = ArmorMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.localized_name()
    }
    pub fn set_localized_name(
        &self,
        arg0: impl Into<String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = ArmorMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.set_localized_name(arg0)
    }
    pub fn has_lore(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = ArmorMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.has_lore()
    }
    pub fn lore(&self) -> Result<Option<Vec<String>>, Box<dyn std::error::Error>> {
        let temp_clone = ArmorMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.lore()
    }
    pub fn set_lore(&self, arg0: Vec<impl Into<String>>) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = ArmorMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.set_lore(arg0)
    }
    pub fn has_custom_model_data(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = ArmorMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.has_custom_model_data()
    }
    pub fn custom_model_data(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let temp_clone = ArmorMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.custom_model_data()
    }
    pub fn set_custom_model_data(&self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = ArmorMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.set_custom_model_data(arg0)
    }
    pub fn has_enchants(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "hasEnchants", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    pub fn has_enchant(
        &self,
        arg0: impl Into<crate::enchantments::Enchantment<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = ArmorMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.has_enchant(arg0)
    }
    pub fn get_enchant_level(
        &self,
        arg0: impl Into<crate::enchantments::Enchantment<'mc>>,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        let temp_clone = ArmorMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.get_enchant_level(arg0)
    }
    pub fn enchants(
        &self,
    ) -> Result<blackboxmc_java::util::JavaMap<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/Map;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getEnchants", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::util::JavaMap::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    pub fn add_enchant(
        &self,
        arg0: impl Into<crate::enchantments::Enchantment<'mc>>,
        arg1: i32,
        arg2: bool,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = ArmorMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.add_enchant(arg0, arg1, arg2)
    }
    pub fn remove_enchant(
        &self,
        arg0: impl Into<crate::enchantments::Enchantment<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = ArmorMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.remove_enchant(arg0)
    }
    pub fn has_conflicting_enchant(
        &self,
        arg0: impl Into<crate::enchantments::Enchantment<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = ArmorMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.has_conflicting_enchant(arg0)
    }
    pub fn add_item_flags(
        &self,
        arg0: Vec<crate::inventory::ItemFlag<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("([Lorg/bukkit/inventory/ItemFlag;)V");
        let arr = self.jni_ref().new_object_array(
            arg0.len() as i32,
            "org/bukkit/inventory/ItemFlag",
            jni::objects::JObject::null(),
        );
        let arr = self.jni_ref().translate_error_no_gen(arr)?;
        for i in 0..arg0.len() {
            let val_1 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(arg0.get(i).unwrap().jni_object().clone())
            });
            self.jni_ref()
                .set_object_array_element(&arr, i as i32, val_1.l()?)?;
        }
        let val_1 = jni::objects::JValueGen::Object(arr);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "addItemFlags",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1.l()?)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub fn remove_item_flags(
        &self,
        arg0: Vec<crate::inventory::ItemFlag<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("([Lorg/bukkit/inventory/ItemFlag;)V");
        let arr = self.jni_ref().new_object_array(
            arg0.len() as i32,
            "org/bukkit/inventory/ItemFlag",
            jni::objects::JObject::null(),
        );
        let arr = self.jni_ref().translate_error_no_gen(arr)?;
        for i in 0..arg0.len() {
            let val_1 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(arg0.get(i).unwrap().jni_object().clone())
            });
            self.jni_ref()
                .set_object_array_element(&arr, i as i32, val_1.l()?)?;
        }
        let val_1 = jni::objects::JValueGen::Object(arr);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeItemFlags",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1.l()?)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub fn item_flags(
        &self,
    ) -> Result<blackboxmc_java::util::JavaSet<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/Set;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getItemFlags", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::util::JavaSet::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    pub fn has_item_flag(
        &self,
        arg0: impl Into<crate::inventory::ItemFlag<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = ArmorMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.has_item_flag(arg0)
    }
    pub fn is_unbreakable(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = ArmorMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.is_unbreakable()
    }
    pub fn set_unbreakable(&self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = ArmorMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.set_unbreakable(arg0)
    }
    pub fn has_attribute_modifiers(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "hasAttributeModifiers",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    pub fn attribute_modifiers(
        &self,
    ) -> Result<Option<jni::objects::JObject<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lcom/google/common/collect/Multimap;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getAttributeModifiers",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {
            return Ok(None);
        }
        Ok(Some(res.l()?))
    }
    pub fn add_attribute_modifier(
        &self,
        arg0: impl Into<crate::attribute::Attribute<'mc>>,
        arg1: impl Into<crate::attribute::AttributeModifier<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = ArmorMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.add_attribute_modifier(arg0, arg1)
    }
    pub fn set_attribute_modifiers(
        &self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lcom/google/common/collect/Multimap;)V");
        let val_1 = jni::objects::JValueGen::Object(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setAttributeModifiers",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub fn remove_attribute_modifier_with_equipment_slot(
        &self,
        arg0: impl Into<crate::inventory::EquipmentSlot<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/inventory/EquipmentSlot;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        sig += ")Z";
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeAttributeModifier",
            sig.as_str(),
            args,
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    pub fn remove_attribute_modifier_with_attribute(
        &self,
        arg0: impl Into<crate::attribute::Attribute<'mc>>,
        arg1: std::option::Option<impl Into<crate::attribute::AttributeModifier<'mc>>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/attribute/Attribute;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        if let Some(a) = arg1 {
            sig += "Lorg/bukkit/attribute/AttributeModifier;";
            let val_2 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(a.into().jni_object().clone())
            });
            args.push(val_2);
        }
        sig += ")Z";
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeAttributeModifier",
            sig.as_str(),
            args,
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn custom_tag_container(
        &self,
    ) -> Result<crate::inventory::meta::tags::CustomItemTagContainer<'mc>, Box<dyn std::error::Error>>
    {
        let temp_clone = ArmorMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.custom_tag_container()
    }

    pub fn set_version(&self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = ArmorMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.set_version(arg0)
    }
    // SUPER CLASS: Cloneable
    // SUPER CLASS: ConfigurationSerializable
    pub fn serialize(
        &self,
    ) -> Result<blackboxmc_java::util::JavaMap<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::configuration::serialization::ConfigurationSerializable::from_raw(
            &self.0,
            unsafe { jni::objects::JObject::from_raw(self.1.clone()) },
        )?;
        let real: crate::configuration::serialization::ConfigurationSerializable =
            temp_clone.into();
        real.serialize()
    }
    // SUPER CLASS: PersistentDataHolder
    pub fn persistent_data_container(
        &self,
    ) -> Result<crate::persistence::PersistentDataContainer<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::persistence::PersistentDataHolder::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::persistence::PersistentDataHolder = temp_clone.into();
        real.persistent_data_container()
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::inventory::meta::ItemMeta<'mc>> for ArmorMeta<'mc> {
    fn into(self) -> crate::inventory::meta::ItemMeta<'mc> {
        crate::inventory::meta::ItemMeta::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting ArmorMeta into crate::inventory::meta::ItemMeta")
    }
}
/// Represents a bucket of axolotl.
///
/// This is a representation of an abstract class.
#[repr(C)]
pub struct AxolotlBucketMeta<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for AxolotlBucketMeta<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for AxolotlBucketMeta<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate AxolotlBucketMeta from null object.").into(),
            );
        }
        let (valid, name) =
            env.validate_name(&obj, "org/bukkit/inventory/meta/AxolotlBucketMeta")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a AxolotlBucketMeta object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> AxolotlBucketMeta<'mc> {
    pub fn variant(
        &self,
    ) -> Result<crate::entity::AxolotlVariant<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/entity/Axolotl$Variant;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getVariant", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::entity::AxolotlVariant::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn set_variant(
        &self,
        arg0: impl Into<crate::entity::AxolotlVariant<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/entity/Axolotl$Variant;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setVariant",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn has_variant(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "hasVariant", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn clone(
        &self,
    ) -> Result<crate::inventory::meta::AxolotlBucketMeta<'mc>, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Lorg/bukkit/inventory/meta/AxolotlBucketMeta;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "clone", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::meta::AxolotlBucketMeta::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    pub fn display_name(&self) -> Result<String, Box<dyn std::error::Error>> {
        let temp_clone = AxolotlBucketMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.display_name()
    }
    pub fn as_string(&self) -> Result<String, Box<dyn std::error::Error>> {
        let temp_clone = AxolotlBucketMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.as_string()
    }
    pub fn has_display_name(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = AxolotlBucketMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.has_display_name()
    }
    pub fn set_display_name(
        &self,
        arg0: impl Into<String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = AxolotlBucketMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.set_display_name(arg0)
    }
    pub fn has_localized_name(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = AxolotlBucketMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.has_localized_name()
    }
    pub fn localized_name(&self) -> Result<String, Box<dyn std::error::Error>> {
        let temp_clone = AxolotlBucketMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.localized_name()
    }
    pub fn set_localized_name(
        &self,
        arg0: impl Into<String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = AxolotlBucketMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.set_localized_name(arg0)
    }
    pub fn has_lore(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = AxolotlBucketMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.has_lore()
    }
    pub fn lore(&self) -> Result<Option<Vec<String>>, Box<dyn std::error::Error>> {
        let temp_clone = AxolotlBucketMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.lore()
    }
    pub fn set_lore(&self, arg0: Vec<impl Into<String>>) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = AxolotlBucketMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.set_lore(arg0)
    }
    pub fn has_custom_model_data(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = AxolotlBucketMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.has_custom_model_data()
    }
    pub fn custom_model_data(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let temp_clone = AxolotlBucketMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.custom_model_data()
    }
    pub fn set_custom_model_data(&self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = AxolotlBucketMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.set_custom_model_data(arg0)
    }
    pub fn has_enchants(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "hasEnchants", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    pub fn has_enchant(
        &self,
        arg0: impl Into<crate::enchantments::Enchantment<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = AxolotlBucketMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.has_enchant(arg0)
    }
    pub fn get_enchant_level(
        &self,
        arg0: impl Into<crate::enchantments::Enchantment<'mc>>,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        let temp_clone = AxolotlBucketMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.get_enchant_level(arg0)
    }
    pub fn enchants(
        &self,
    ) -> Result<blackboxmc_java::util::JavaMap<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/Map;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getEnchants", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::util::JavaMap::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    pub fn add_enchant(
        &self,
        arg0: impl Into<crate::enchantments::Enchantment<'mc>>,
        arg1: i32,
        arg2: bool,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = AxolotlBucketMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.add_enchant(arg0, arg1, arg2)
    }
    pub fn remove_enchant(
        &self,
        arg0: impl Into<crate::enchantments::Enchantment<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = AxolotlBucketMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.remove_enchant(arg0)
    }
    pub fn has_conflicting_enchant(
        &self,
        arg0: impl Into<crate::enchantments::Enchantment<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = AxolotlBucketMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.has_conflicting_enchant(arg0)
    }
    pub fn add_item_flags(
        &self,
        arg0: Vec<crate::inventory::ItemFlag<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("([Lorg/bukkit/inventory/ItemFlag;)V");
        let arr = self.jni_ref().new_object_array(
            arg0.len() as i32,
            "org/bukkit/inventory/ItemFlag",
            jni::objects::JObject::null(),
        );
        let arr = self.jni_ref().translate_error_no_gen(arr)?;
        for i in 0..arg0.len() {
            let val_1 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(arg0.get(i).unwrap().jni_object().clone())
            });
            self.jni_ref()
                .set_object_array_element(&arr, i as i32, val_1.l()?)?;
        }
        let val_1 = jni::objects::JValueGen::Object(arr);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "addItemFlags",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1.l()?)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub fn remove_item_flags(
        &self,
        arg0: Vec<crate::inventory::ItemFlag<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("([Lorg/bukkit/inventory/ItemFlag;)V");
        let arr = self.jni_ref().new_object_array(
            arg0.len() as i32,
            "org/bukkit/inventory/ItemFlag",
            jni::objects::JObject::null(),
        );
        let arr = self.jni_ref().translate_error_no_gen(arr)?;
        for i in 0..arg0.len() {
            let val_1 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(arg0.get(i).unwrap().jni_object().clone())
            });
            self.jni_ref()
                .set_object_array_element(&arr, i as i32, val_1.l()?)?;
        }
        let val_1 = jni::objects::JValueGen::Object(arr);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeItemFlags",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1.l()?)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub fn item_flags(
        &self,
    ) -> Result<blackboxmc_java::util::JavaSet<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/Set;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getItemFlags", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::util::JavaSet::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    pub fn has_item_flag(
        &self,
        arg0: impl Into<crate::inventory::ItemFlag<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = AxolotlBucketMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.has_item_flag(arg0)
    }
    pub fn is_unbreakable(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = AxolotlBucketMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.is_unbreakable()
    }
    pub fn set_unbreakable(&self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = AxolotlBucketMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.set_unbreakable(arg0)
    }
    pub fn has_attribute_modifiers(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "hasAttributeModifiers",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    pub fn attribute_modifiers(
        &self,
    ) -> Result<Option<jni::objects::JObject<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lcom/google/common/collect/Multimap;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getAttributeModifiers",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {
            return Ok(None);
        }
        Ok(Some(res.l()?))
    }
    pub fn add_attribute_modifier(
        &self,
        arg0: impl Into<crate::attribute::Attribute<'mc>>,
        arg1: impl Into<crate::attribute::AttributeModifier<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = AxolotlBucketMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.add_attribute_modifier(arg0, arg1)
    }
    pub fn set_attribute_modifiers(
        &self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lcom/google/common/collect/Multimap;)V");
        let val_1 = jni::objects::JValueGen::Object(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setAttributeModifiers",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub fn remove_attribute_modifier_with_equipment_slot(
        &self,
        arg0: impl Into<crate::inventory::EquipmentSlot<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/inventory/EquipmentSlot;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        sig += ")Z";
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeAttributeModifier",
            sig.as_str(),
            args,
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    pub fn remove_attribute_modifier_with_attribute(
        &self,
        arg0: impl Into<crate::attribute::Attribute<'mc>>,
        arg1: std::option::Option<impl Into<crate::attribute::AttributeModifier<'mc>>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/attribute/Attribute;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        if let Some(a) = arg1 {
            sig += "Lorg/bukkit/attribute/AttributeModifier;";
            let val_2 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(a.into().jni_object().clone())
            });
            args.push(val_2);
        }
        sig += ")Z";
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeAttributeModifier",
            sig.as_str(),
            args,
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn custom_tag_container(
        &self,
    ) -> Result<crate::inventory::meta::tags::CustomItemTagContainer<'mc>, Box<dyn std::error::Error>>
    {
        let temp_clone = AxolotlBucketMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.custom_tag_container()
    }

    pub fn set_version(&self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = AxolotlBucketMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.set_version(arg0)
    }
    // SUPER CLASS: Cloneable
    // SUPER CLASS: ConfigurationSerializable
    pub fn serialize(
        &self,
    ) -> Result<blackboxmc_java::util::JavaMap<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::configuration::serialization::ConfigurationSerializable::from_raw(
            &self.0,
            unsafe { jni::objects::JObject::from_raw(self.1.clone()) },
        )?;
        let real: crate::configuration::serialization::ConfigurationSerializable =
            temp_clone.into();
        real.serialize()
    }
    // SUPER CLASS: PersistentDataHolder
    pub fn persistent_data_container(
        &self,
    ) -> Result<crate::persistence::PersistentDataContainer<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::persistence::PersistentDataHolder::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::persistence::PersistentDataHolder = temp_clone.into();
        real.persistent_data_container()
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::inventory::meta::ItemMeta<'mc>> for AxolotlBucketMeta<'mc> {
    fn into(self) -> crate::inventory::meta::ItemMeta<'mc> {
        crate::inventory::meta::ItemMeta::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting AxolotlBucketMeta into crate::inventory::meta::ItemMeta")
    }
}
/// Represents a skull that can have an owner.
///
/// This is a representation of an abstract class.
#[repr(C)]
pub struct SkullMeta<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for SkullMeta<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for SkullMeta<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate SkullMeta from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/inventory/meta/SkullMeta")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a SkullMeta object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> SkullMeta<'mc> {
    pub fn owner(&self) -> Result<Option<String>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/String;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getOwner", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {
            return Ok(None);
        }
        Ok(Some(
            self.jni_ref()
                .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
                .to_string_lossy()
                .to_string(),
        ))
    }

    pub fn set_owner(&self, arg0: impl Into<String>) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/String;)Z");
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(arg0.into())?,
        ));
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setOwner",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn has_owner(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasOwner", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn owning_player(
        &self,
    ) -> Result<Option<crate::OfflinePlayer<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/OfflinePlayer;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getOwningPlayer", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {
            return Ok(None);
        }
        Ok(Some(crate::OfflinePlayer::from_raw(
            &self.jni_ref(),
            unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) },
        )?))
    }

    pub fn set_owning_player(
        &self,
        arg0: impl Into<crate::OfflinePlayer<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/OfflinePlayer;)Z");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setOwningPlayer",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn owner_profile(
        &self,
    ) -> Result<Option<crate::profile::PlayerProfile<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/profile/PlayerProfile;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getOwnerProfile", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {
            return Ok(None);
        }
        Ok(Some(crate::profile::PlayerProfile::from_raw(
            &self.jni_ref(),
            unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) },
        )?))
    }

    pub fn set_owner_profile(
        &self,
        arg0: impl Into<crate::profile::PlayerProfile<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/profile/PlayerProfile;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setOwnerProfile",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn note_block_sound(
        &self,
    ) -> Result<Option<crate::NamespacedKey<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/NamespacedKey;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getNoteBlockSound",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {
            return Ok(None);
        }
        Ok(Some(crate::NamespacedKey::from_raw(
            &self.jni_ref(),
            unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) },
        )?))
    }

    pub fn set_note_block_sound(
        &self,
        arg0: impl Into<crate::NamespacedKey<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/NamespacedKey;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setNoteBlockSound",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn clone(
        &self,
    ) -> Result<crate::inventory::meta::SkullMeta<'mc>, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Lorg/bukkit/inventory/meta/SkullMeta;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "clone", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::meta::SkullMeta::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    pub fn display_name(&self) -> Result<String, Box<dyn std::error::Error>> {
        let temp_clone = SkullMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.display_name()
    }
    pub fn as_string(&self) -> Result<String, Box<dyn std::error::Error>> {
        let temp_clone = SkullMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.as_string()
    }
    pub fn has_display_name(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = SkullMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.has_display_name()
    }
    pub fn set_display_name(
        &self,
        arg0: impl Into<String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = SkullMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.set_display_name(arg0)
    }
    pub fn has_localized_name(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = SkullMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.has_localized_name()
    }
    pub fn localized_name(&self) -> Result<String, Box<dyn std::error::Error>> {
        let temp_clone = SkullMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.localized_name()
    }
    pub fn set_localized_name(
        &self,
        arg0: impl Into<String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = SkullMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.set_localized_name(arg0)
    }
    pub fn has_lore(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = SkullMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.has_lore()
    }
    pub fn lore(&self) -> Result<Option<Vec<String>>, Box<dyn std::error::Error>> {
        let temp_clone = SkullMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.lore()
    }
    pub fn set_lore(&self, arg0: Vec<impl Into<String>>) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = SkullMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.set_lore(arg0)
    }
    pub fn has_custom_model_data(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = SkullMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.has_custom_model_data()
    }
    pub fn custom_model_data(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let temp_clone = SkullMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.custom_model_data()
    }
    pub fn set_custom_model_data(&self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = SkullMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.set_custom_model_data(arg0)
    }
    pub fn has_enchants(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "hasEnchants", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    pub fn has_enchant(
        &self,
        arg0: impl Into<crate::enchantments::Enchantment<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = SkullMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.has_enchant(arg0)
    }
    pub fn get_enchant_level(
        &self,
        arg0: impl Into<crate::enchantments::Enchantment<'mc>>,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        let temp_clone = SkullMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.get_enchant_level(arg0)
    }
    pub fn enchants(
        &self,
    ) -> Result<blackboxmc_java::util::JavaMap<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/Map;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getEnchants", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::util::JavaMap::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    pub fn add_enchant(
        &self,
        arg0: impl Into<crate::enchantments::Enchantment<'mc>>,
        arg1: i32,
        arg2: bool,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = SkullMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.add_enchant(arg0, arg1, arg2)
    }
    pub fn remove_enchant(
        &self,
        arg0: impl Into<crate::enchantments::Enchantment<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = SkullMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.remove_enchant(arg0)
    }
    pub fn has_conflicting_enchant(
        &self,
        arg0: impl Into<crate::enchantments::Enchantment<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = SkullMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.has_conflicting_enchant(arg0)
    }
    pub fn add_item_flags(
        &self,
        arg0: Vec<crate::inventory::ItemFlag<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("([Lorg/bukkit/inventory/ItemFlag;)V");
        let arr = self.jni_ref().new_object_array(
            arg0.len() as i32,
            "org/bukkit/inventory/ItemFlag",
            jni::objects::JObject::null(),
        );
        let arr = self.jni_ref().translate_error_no_gen(arr)?;
        for i in 0..arg0.len() {
            let val_1 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(arg0.get(i).unwrap().jni_object().clone())
            });
            self.jni_ref()
                .set_object_array_element(&arr, i as i32, val_1.l()?)?;
        }
        let val_1 = jni::objects::JValueGen::Object(arr);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "addItemFlags",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1.l()?)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub fn remove_item_flags(
        &self,
        arg0: Vec<crate::inventory::ItemFlag<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("([Lorg/bukkit/inventory/ItemFlag;)V");
        let arr = self.jni_ref().new_object_array(
            arg0.len() as i32,
            "org/bukkit/inventory/ItemFlag",
            jni::objects::JObject::null(),
        );
        let arr = self.jni_ref().translate_error_no_gen(arr)?;
        for i in 0..arg0.len() {
            let val_1 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(arg0.get(i).unwrap().jni_object().clone())
            });
            self.jni_ref()
                .set_object_array_element(&arr, i as i32, val_1.l()?)?;
        }
        let val_1 = jni::objects::JValueGen::Object(arr);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeItemFlags",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1.l()?)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub fn item_flags(
        &self,
    ) -> Result<blackboxmc_java::util::JavaSet<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/Set;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getItemFlags", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::util::JavaSet::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    pub fn has_item_flag(
        &self,
        arg0: impl Into<crate::inventory::ItemFlag<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = SkullMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.has_item_flag(arg0)
    }
    pub fn is_unbreakable(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = SkullMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.is_unbreakable()
    }
    pub fn set_unbreakable(&self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = SkullMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.set_unbreakable(arg0)
    }
    pub fn has_attribute_modifiers(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "hasAttributeModifiers",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    pub fn attribute_modifiers(
        &self,
    ) -> Result<Option<jni::objects::JObject<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lcom/google/common/collect/Multimap;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getAttributeModifiers",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {
            return Ok(None);
        }
        Ok(Some(res.l()?))
    }
    pub fn add_attribute_modifier(
        &self,
        arg0: impl Into<crate::attribute::Attribute<'mc>>,
        arg1: impl Into<crate::attribute::AttributeModifier<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = SkullMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.add_attribute_modifier(arg0, arg1)
    }
    pub fn set_attribute_modifiers(
        &self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lcom/google/common/collect/Multimap;)V");
        let val_1 = jni::objects::JValueGen::Object(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setAttributeModifiers",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub fn remove_attribute_modifier_with_equipment_slot(
        &self,
        arg0: impl Into<crate::inventory::EquipmentSlot<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/inventory/EquipmentSlot;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        sig += ")Z";
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeAttributeModifier",
            sig.as_str(),
            args,
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    pub fn remove_attribute_modifier_with_attribute(
        &self,
        arg0: impl Into<crate::attribute::Attribute<'mc>>,
        arg1: std::option::Option<impl Into<crate::attribute::AttributeModifier<'mc>>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/attribute/Attribute;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        if let Some(a) = arg1 {
            sig += "Lorg/bukkit/attribute/AttributeModifier;";
            let val_2 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(a.into().jni_object().clone())
            });
            args.push(val_2);
        }
        sig += ")Z";
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeAttributeModifier",
            sig.as_str(),
            args,
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn custom_tag_container(
        &self,
    ) -> Result<crate::inventory::meta::tags::CustomItemTagContainer<'mc>, Box<dyn std::error::Error>>
    {
        let temp_clone = SkullMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.custom_tag_container()
    }

    pub fn set_version(&self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = SkullMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.set_version(arg0)
    }
    // SUPER CLASS: Cloneable
    // SUPER CLASS: ConfigurationSerializable
    pub fn serialize(
        &self,
    ) -> Result<blackboxmc_java::util::JavaMap<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::configuration::serialization::ConfigurationSerializable::from_raw(
            &self.0,
            unsafe { jni::objects::JObject::from_raw(self.1.clone()) },
        )?;
        let real: crate::configuration::serialization::ConfigurationSerializable =
            temp_clone.into();
        real.serialize()
    }
    // SUPER CLASS: PersistentDataHolder
    pub fn persistent_data_container(
        &self,
    ) -> Result<crate::persistence::PersistentDataContainer<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::persistence::PersistentDataHolder::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::persistence::PersistentDataHolder = temp_clone.into();
        real.persistent_data_container()
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::inventory::meta::ItemMeta<'mc>> for SkullMeta<'mc> {
    fn into(self) -> crate::inventory::meta::ItemMeta<'mc> {
        crate::inventory::meta::ItemMeta::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting SkullMeta into crate::inventory::meta::ItemMeta")
    }
}
/// Represents a compass that can track a specific location.
///
/// This is a representation of an abstract class.
#[repr(C)]
pub struct CompassMeta<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for CompassMeta<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for CompassMeta<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate CompassMeta from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/inventory/meta/CompassMeta")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a CompassMeta object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> CompassMeta<'mc> {
    pub fn has_lodestone(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "hasLodestone", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn lodestone(&self) -> Result<Option<crate::Location<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/Location;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getLodestone", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {
            return Ok(None);
        }
        Ok(Some(crate::Location::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })?))
    }

    pub fn set_lodestone(
        &self,
        arg0: impl Into<crate::Location<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/Location;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setLodestone",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn is_lodestone_tracked(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "isLodestoneTracked",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Sets if this compass is tracking a specific lodestone. If true the compass will only work if there is a lodestone at the tracked location.
    pub fn set_lodestone_tracked(&self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        let val_1 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setLodestoneTracked",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn clone(&self) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Ljava/lang/Object;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "clone", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l()?)
    }
    pub fn display_name(&self) -> Result<String, Box<dyn std::error::Error>> {
        let temp_clone = CompassMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.display_name()
    }
    pub fn as_string(&self) -> Result<String, Box<dyn std::error::Error>> {
        let temp_clone = CompassMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.as_string()
    }
    pub fn has_display_name(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = CompassMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.has_display_name()
    }
    pub fn set_display_name(
        &self,
        arg0: impl Into<String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = CompassMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.set_display_name(arg0)
    }
    pub fn has_localized_name(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = CompassMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.has_localized_name()
    }
    pub fn localized_name(&self) -> Result<String, Box<dyn std::error::Error>> {
        let temp_clone = CompassMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.localized_name()
    }
    pub fn set_localized_name(
        &self,
        arg0: impl Into<String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = CompassMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.set_localized_name(arg0)
    }
    pub fn has_lore(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = CompassMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.has_lore()
    }
    pub fn lore(&self) -> Result<Option<Vec<String>>, Box<dyn std::error::Error>> {
        let temp_clone = CompassMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.lore()
    }
    pub fn set_lore(&self, arg0: Vec<impl Into<String>>) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = CompassMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.set_lore(arg0)
    }
    pub fn has_custom_model_data(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = CompassMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.has_custom_model_data()
    }
    pub fn custom_model_data(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let temp_clone = CompassMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.custom_model_data()
    }
    pub fn set_custom_model_data(&self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = CompassMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.set_custom_model_data(arg0)
    }
    pub fn has_enchants(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "hasEnchants", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    pub fn has_enchant(
        &self,
        arg0: impl Into<crate::enchantments::Enchantment<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = CompassMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.has_enchant(arg0)
    }
    pub fn get_enchant_level(
        &self,
        arg0: impl Into<crate::enchantments::Enchantment<'mc>>,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        let temp_clone = CompassMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.get_enchant_level(arg0)
    }
    pub fn enchants(
        &self,
    ) -> Result<blackboxmc_java::util::JavaMap<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/Map;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getEnchants", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::util::JavaMap::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    pub fn add_enchant(
        &self,
        arg0: impl Into<crate::enchantments::Enchantment<'mc>>,
        arg1: i32,
        arg2: bool,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = CompassMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.add_enchant(arg0, arg1, arg2)
    }
    pub fn remove_enchant(
        &self,
        arg0: impl Into<crate::enchantments::Enchantment<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = CompassMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.remove_enchant(arg0)
    }
    pub fn has_conflicting_enchant(
        &self,
        arg0: impl Into<crate::enchantments::Enchantment<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = CompassMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.has_conflicting_enchant(arg0)
    }
    pub fn add_item_flags(
        &self,
        arg0: Vec<crate::inventory::ItemFlag<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("([Lorg/bukkit/inventory/ItemFlag;)V");
        let arr = self.jni_ref().new_object_array(
            arg0.len() as i32,
            "org/bukkit/inventory/ItemFlag",
            jni::objects::JObject::null(),
        );
        let arr = self.jni_ref().translate_error_no_gen(arr)?;
        for i in 0..arg0.len() {
            let val_1 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(arg0.get(i).unwrap().jni_object().clone())
            });
            self.jni_ref()
                .set_object_array_element(&arr, i as i32, val_1.l()?)?;
        }
        let val_1 = jni::objects::JValueGen::Object(arr);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "addItemFlags",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1.l()?)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub fn remove_item_flags(
        &self,
        arg0: Vec<crate::inventory::ItemFlag<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("([Lorg/bukkit/inventory/ItemFlag;)V");
        let arr = self.jni_ref().new_object_array(
            arg0.len() as i32,
            "org/bukkit/inventory/ItemFlag",
            jni::objects::JObject::null(),
        );
        let arr = self.jni_ref().translate_error_no_gen(arr)?;
        for i in 0..arg0.len() {
            let val_1 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(arg0.get(i).unwrap().jni_object().clone())
            });
            self.jni_ref()
                .set_object_array_element(&arr, i as i32, val_1.l()?)?;
        }
        let val_1 = jni::objects::JValueGen::Object(arr);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeItemFlags",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1.l()?)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub fn item_flags(
        &self,
    ) -> Result<blackboxmc_java::util::JavaSet<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/Set;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getItemFlags", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::util::JavaSet::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    pub fn has_item_flag(
        &self,
        arg0: impl Into<crate::inventory::ItemFlag<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = CompassMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.has_item_flag(arg0)
    }
    pub fn is_unbreakable(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = CompassMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.is_unbreakable()
    }
    pub fn set_unbreakable(&self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = CompassMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.set_unbreakable(arg0)
    }
    pub fn has_attribute_modifiers(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "hasAttributeModifiers",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    pub fn attribute_modifiers(
        &self,
    ) -> Result<Option<jni::objects::JObject<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lcom/google/common/collect/Multimap;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getAttributeModifiers",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {
            return Ok(None);
        }
        Ok(Some(res.l()?))
    }
    pub fn add_attribute_modifier(
        &self,
        arg0: impl Into<crate::attribute::Attribute<'mc>>,
        arg1: impl Into<crate::attribute::AttributeModifier<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = CompassMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.add_attribute_modifier(arg0, arg1)
    }
    pub fn set_attribute_modifiers(
        &self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lcom/google/common/collect/Multimap;)V");
        let val_1 = jni::objects::JValueGen::Object(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setAttributeModifiers",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub fn remove_attribute_modifier_with_equipment_slot(
        &self,
        arg0: impl Into<crate::inventory::EquipmentSlot<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/inventory/EquipmentSlot;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        sig += ")Z";
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeAttributeModifier",
            sig.as_str(),
            args,
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    pub fn remove_attribute_modifier_with_attribute(
        &self,
        arg0: impl Into<crate::attribute::Attribute<'mc>>,
        arg1: std::option::Option<impl Into<crate::attribute::AttributeModifier<'mc>>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/attribute/Attribute;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        if let Some(a) = arg1 {
            sig += "Lorg/bukkit/attribute/AttributeModifier;";
            let val_2 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(a.into().jni_object().clone())
            });
            args.push(val_2);
        }
        sig += ")Z";
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeAttributeModifier",
            sig.as_str(),
            args,
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn custom_tag_container(
        &self,
    ) -> Result<crate::inventory::meta::tags::CustomItemTagContainer<'mc>, Box<dyn std::error::Error>>
    {
        let temp_clone = CompassMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.custom_tag_container()
    }

    pub fn set_version(&self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = CompassMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.set_version(arg0)
    }
    // SUPER CLASS: Cloneable
    // SUPER CLASS: ConfigurationSerializable
    pub fn serialize(
        &self,
    ) -> Result<blackboxmc_java::util::JavaMap<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::configuration::serialization::ConfigurationSerializable::from_raw(
            &self.0,
            unsafe { jni::objects::JObject::from_raw(self.1.clone()) },
        )?;
        let real: crate::configuration::serialization::ConfigurationSerializable =
            temp_clone.into();
        real.serialize()
    }
    // SUPER CLASS: PersistentDataHolder
    pub fn persistent_data_container(
        &self,
    ) -> Result<crate::persistence::PersistentDataContainer<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::persistence::PersistentDataHolder::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::persistence::PersistentDataHolder = temp_clone.into();
        real.persistent_data_container()
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::inventory::meta::ItemMeta<'mc>> for CompassMeta<'mc> {
    fn into(self) -> crate::inventory::meta::ItemMeta<'mc> {
        crate::inventory::meta::ItemMeta::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting CompassMeta into crate::inventory::meta::ItemMeta")
    }
}
/// Represents a suspicious stew that can have custom effects.
///
/// This is a representation of an abstract class.
#[repr(C)]
pub struct SuspiciousStewMeta<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for SuspiciousStewMeta<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for SuspiciousStewMeta<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate SuspiciousStewMeta from null object.").into(),
            );
        }
        let (valid, name) =
            env.validate_name(&obj, "org/bukkit/inventory/meta/SuspiciousStewMeta")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a SuspiciousStewMeta object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> SuspiciousStewMeta<'mc> {
    pub fn has_custom_effects(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "hasCustomEffects",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn custom_effects(
        &self,
    ) -> Result<Vec<crate::potion::PotionEffect<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/List;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getCustomEffects",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let list = blackboxmc_java::util::JavaList::from_raw(&self.jni_ref(), res.l()?)?;
        let iter = list.iterator()?;
        while iter.has_next()? {
            let obj = iter.next()?;
            new_vec.push(crate::potion::PotionEffect::from_raw(&self.jni_ref(), obj)?);
        }
        Ok(new_vec)
    }

    pub fn add_custom_effect(
        &self,
        arg0: impl Into<crate::potion::PotionEffect<'mc>>,
        arg1: bool,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/potion/PotionEffect;Z)Z");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let val_2 = jni::objects::JValueGen::Bool(arg1.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "addCustomEffect",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn remove_custom_effect(
        &self,
        arg0: impl Into<crate::potion::PotionEffectType<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/potion/PotionEffectType;)Z");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeCustomEffect",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn has_custom_effect(
        &self,
        arg0: impl Into<crate::potion::PotionEffectType<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/potion/PotionEffectType;)Z");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "hasCustomEffect",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn clear_custom_effects(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "clearCustomEffects",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn clone(
        &self,
    ) -> Result<crate::inventory::meta::SuspiciousStewMeta<'mc>, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Lorg/bukkit/inventory/meta/SuspiciousStewMeta;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "clone", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::meta::SuspiciousStewMeta::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    pub fn display_name(&self) -> Result<String, Box<dyn std::error::Error>> {
        let temp_clone = SuspiciousStewMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.display_name()
    }
    pub fn as_string(&self) -> Result<String, Box<dyn std::error::Error>> {
        let temp_clone = SuspiciousStewMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.as_string()
    }
    pub fn has_display_name(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = SuspiciousStewMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.has_display_name()
    }
    pub fn set_display_name(
        &self,
        arg0: impl Into<String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = SuspiciousStewMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.set_display_name(arg0)
    }
    pub fn has_localized_name(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = SuspiciousStewMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.has_localized_name()
    }
    pub fn localized_name(&self) -> Result<String, Box<dyn std::error::Error>> {
        let temp_clone = SuspiciousStewMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.localized_name()
    }
    pub fn set_localized_name(
        &self,
        arg0: impl Into<String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = SuspiciousStewMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.set_localized_name(arg0)
    }
    pub fn has_lore(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = SuspiciousStewMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.has_lore()
    }
    pub fn lore(&self) -> Result<Option<Vec<String>>, Box<dyn std::error::Error>> {
        let temp_clone = SuspiciousStewMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.lore()
    }
    pub fn set_lore(&self, arg0: Vec<impl Into<String>>) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = SuspiciousStewMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.set_lore(arg0)
    }
    pub fn has_custom_model_data(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = SuspiciousStewMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.has_custom_model_data()
    }
    pub fn custom_model_data(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let temp_clone = SuspiciousStewMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.custom_model_data()
    }
    pub fn set_custom_model_data(&self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = SuspiciousStewMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.set_custom_model_data(arg0)
    }
    pub fn has_enchants(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "hasEnchants", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    pub fn has_enchant(
        &self,
        arg0: impl Into<crate::enchantments::Enchantment<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = SuspiciousStewMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.has_enchant(arg0)
    }
    pub fn get_enchant_level(
        &self,
        arg0: impl Into<crate::enchantments::Enchantment<'mc>>,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        let temp_clone = SuspiciousStewMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.get_enchant_level(arg0)
    }
    pub fn enchants(
        &self,
    ) -> Result<blackboxmc_java::util::JavaMap<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/Map;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getEnchants", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::util::JavaMap::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    pub fn add_enchant(
        &self,
        arg0: impl Into<crate::enchantments::Enchantment<'mc>>,
        arg1: i32,
        arg2: bool,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = SuspiciousStewMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.add_enchant(arg0, arg1, arg2)
    }
    pub fn remove_enchant(
        &self,
        arg0: impl Into<crate::enchantments::Enchantment<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = SuspiciousStewMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.remove_enchant(arg0)
    }
    pub fn has_conflicting_enchant(
        &self,
        arg0: impl Into<crate::enchantments::Enchantment<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = SuspiciousStewMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.has_conflicting_enchant(arg0)
    }
    pub fn add_item_flags(
        &self,
        arg0: Vec<crate::inventory::ItemFlag<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("([Lorg/bukkit/inventory/ItemFlag;)V");
        let arr = self.jni_ref().new_object_array(
            arg0.len() as i32,
            "org/bukkit/inventory/ItemFlag",
            jni::objects::JObject::null(),
        );
        let arr = self.jni_ref().translate_error_no_gen(arr)?;
        for i in 0..arg0.len() {
            let val_1 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(arg0.get(i).unwrap().jni_object().clone())
            });
            self.jni_ref()
                .set_object_array_element(&arr, i as i32, val_1.l()?)?;
        }
        let val_1 = jni::objects::JValueGen::Object(arr);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "addItemFlags",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1.l()?)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub fn remove_item_flags(
        &self,
        arg0: Vec<crate::inventory::ItemFlag<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("([Lorg/bukkit/inventory/ItemFlag;)V");
        let arr = self.jni_ref().new_object_array(
            arg0.len() as i32,
            "org/bukkit/inventory/ItemFlag",
            jni::objects::JObject::null(),
        );
        let arr = self.jni_ref().translate_error_no_gen(arr)?;
        for i in 0..arg0.len() {
            let val_1 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(arg0.get(i).unwrap().jni_object().clone())
            });
            self.jni_ref()
                .set_object_array_element(&arr, i as i32, val_1.l()?)?;
        }
        let val_1 = jni::objects::JValueGen::Object(arr);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeItemFlags",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1.l()?)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub fn item_flags(
        &self,
    ) -> Result<blackboxmc_java::util::JavaSet<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/Set;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getItemFlags", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::util::JavaSet::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    pub fn has_item_flag(
        &self,
        arg0: impl Into<crate::inventory::ItemFlag<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = SuspiciousStewMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.has_item_flag(arg0)
    }
    pub fn is_unbreakable(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = SuspiciousStewMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.is_unbreakable()
    }
    pub fn set_unbreakable(&self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = SuspiciousStewMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.set_unbreakable(arg0)
    }
    pub fn has_attribute_modifiers(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "hasAttributeModifiers",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    pub fn attribute_modifiers(
        &self,
    ) -> Result<Option<jni::objects::JObject<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lcom/google/common/collect/Multimap;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getAttributeModifiers",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {
            return Ok(None);
        }
        Ok(Some(res.l()?))
    }
    pub fn add_attribute_modifier(
        &self,
        arg0: impl Into<crate::attribute::Attribute<'mc>>,
        arg1: impl Into<crate::attribute::AttributeModifier<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = SuspiciousStewMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.add_attribute_modifier(arg0, arg1)
    }
    pub fn set_attribute_modifiers(
        &self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lcom/google/common/collect/Multimap;)V");
        let val_1 = jni::objects::JValueGen::Object(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setAttributeModifiers",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub fn remove_attribute_modifier_with_equipment_slot(
        &self,
        arg0: impl Into<crate::inventory::EquipmentSlot<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/inventory/EquipmentSlot;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        sig += ")Z";
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeAttributeModifier",
            sig.as_str(),
            args,
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    pub fn remove_attribute_modifier_with_attribute(
        &self,
        arg0: impl Into<crate::attribute::Attribute<'mc>>,
        arg1: std::option::Option<impl Into<crate::attribute::AttributeModifier<'mc>>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/attribute/Attribute;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        if let Some(a) = arg1 {
            sig += "Lorg/bukkit/attribute/AttributeModifier;";
            let val_2 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(a.into().jni_object().clone())
            });
            args.push(val_2);
        }
        sig += ")Z";
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeAttributeModifier",
            sig.as_str(),
            args,
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn custom_tag_container(
        &self,
    ) -> Result<crate::inventory::meta::tags::CustomItemTagContainer<'mc>, Box<dyn std::error::Error>>
    {
        let temp_clone = SuspiciousStewMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.custom_tag_container()
    }

    pub fn set_version(&self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = SuspiciousStewMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.set_version(arg0)
    }
    // SUPER CLASS: Cloneable
    // SUPER CLASS: ConfigurationSerializable
    pub fn serialize(
        &self,
    ) -> Result<blackboxmc_java::util::JavaMap<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::configuration::serialization::ConfigurationSerializable::from_raw(
            &self.0,
            unsafe { jni::objects::JObject::from_raw(self.1.clone()) },
        )?;
        let real: crate::configuration::serialization::ConfigurationSerializable =
            temp_clone.into();
        real.serialize()
    }
    // SUPER CLASS: PersistentDataHolder
    pub fn persistent_data_container(
        &self,
    ) -> Result<crate::persistence::PersistentDataContainer<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::persistence::PersistentDataHolder::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::persistence::PersistentDataHolder = temp_clone.into();
        real.persistent_data_container()
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::inventory::meta::ItemMeta<'mc>> for SuspiciousStewMeta<'mc> {
    fn into(self) -> crate::inventory::meta::ItemMeta<'mc> {
        crate::inventory::meta::ItemMeta::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting SuspiciousStewMeta into crate::inventory::meta::ItemMeta")
    }
}

///
/// This is a representation of an abstract class.
#[repr(C)]
pub struct CrossbowMeta<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for CrossbowMeta<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for CrossbowMeta<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate CrossbowMeta from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/inventory/meta/CrossbowMeta")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a CrossbowMeta object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> CrossbowMeta<'mc> {
    pub fn has_charged_projectiles(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "hasChargedProjectiles",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn charged_projectiles(
        &self,
    ) -> Result<Vec<crate::inventory::ItemStack<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/List;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getChargedProjectiles",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let list = blackboxmc_java::util::JavaList::from_raw(&self.jni_ref(), res.l()?)?;
        let iter = list.iterator()?;
        while iter.has_next()? {
            let obj = iter.next()?;
            new_vec.push(crate::inventory::ItemStack::from_raw(&self.jni_ref(), obj)?);
        }
        Ok(new_vec)
    }

    pub fn set_charged_projectiles(
        &self,
        arg0: Vec<impl Into<crate::inventory::ItemStack<'mc>>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/util/List;)V");
        let raw_val_1 = self
            .jni_ref()
            .new_object("java/util/ArrayList", "()V", vec![])?;
        for v in arg0 {
            let map_val_0 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(v.into().jni_object().clone())
            });
            self.jni_ref().call_method(
                &raw_val_1,
                "add",
                "(Ljava/lang/Object;)Z",
                vec![jni::objects::JValueGen::from(map_val_0)],
            )?;
        }
        let val_1 = jni::objects::JValueGen::Object(raw_val_1);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setChargedProjectiles",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn add_charged_projectile(
        &self,
        arg0: impl Into<crate::inventory::ItemStack<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/inventory/ItemStack;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "addChargedProjectile",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub fn display_name(&self) -> Result<String, Box<dyn std::error::Error>> {
        let temp_clone = CrossbowMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.display_name()
    }
    pub fn as_string(&self) -> Result<String, Box<dyn std::error::Error>> {
        let temp_clone = CrossbowMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.as_string()
    }
    pub fn has_display_name(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = CrossbowMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.has_display_name()
    }
    pub fn set_display_name(
        &self,
        arg0: impl Into<String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = CrossbowMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.set_display_name(arg0)
    }
    pub fn has_localized_name(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = CrossbowMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.has_localized_name()
    }
    pub fn localized_name(&self) -> Result<String, Box<dyn std::error::Error>> {
        let temp_clone = CrossbowMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.localized_name()
    }
    pub fn set_localized_name(
        &self,
        arg0: impl Into<String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = CrossbowMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.set_localized_name(arg0)
    }
    pub fn has_lore(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = CrossbowMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.has_lore()
    }
    pub fn lore(&self) -> Result<Option<Vec<String>>, Box<dyn std::error::Error>> {
        let temp_clone = CrossbowMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.lore()
    }
    pub fn set_lore(&self, arg0: Vec<impl Into<String>>) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = CrossbowMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.set_lore(arg0)
    }
    pub fn has_custom_model_data(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = CrossbowMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.has_custom_model_data()
    }
    pub fn custom_model_data(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let temp_clone = CrossbowMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.custom_model_data()
    }
    pub fn set_custom_model_data(&self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = CrossbowMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.set_custom_model_data(arg0)
    }
    pub fn has_enchants(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "hasEnchants", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    pub fn has_enchant(
        &self,
        arg0: impl Into<crate::enchantments::Enchantment<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = CrossbowMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.has_enchant(arg0)
    }
    pub fn get_enchant_level(
        &self,
        arg0: impl Into<crate::enchantments::Enchantment<'mc>>,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        let temp_clone = CrossbowMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.get_enchant_level(arg0)
    }
    pub fn enchants(
        &self,
    ) -> Result<blackboxmc_java::util::JavaMap<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/Map;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getEnchants", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::util::JavaMap::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    pub fn add_enchant(
        &self,
        arg0: impl Into<crate::enchantments::Enchantment<'mc>>,
        arg1: i32,
        arg2: bool,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = CrossbowMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.add_enchant(arg0, arg1, arg2)
    }
    pub fn remove_enchant(
        &self,
        arg0: impl Into<crate::enchantments::Enchantment<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = CrossbowMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.remove_enchant(arg0)
    }
    pub fn has_conflicting_enchant(
        &self,
        arg0: impl Into<crate::enchantments::Enchantment<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = CrossbowMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.has_conflicting_enchant(arg0)
    }
    pub fn add_item_flags(
        &self,
        arg0: Vec<crate::inventory::ItemFlag<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("([Lorg/bukkit/inventory/ItemFlag;)V");
        let arr = self.jni_ref().new_object_array(
            arg0.len() as i32,
            "org/bukkit/inventory/ItemFlag",
            jni::objects::JObject::null(),
        );
        let arr = self.jni_ref().translate_error_no_gen(arr)?;
        for i in 0..arg0.len() {
            let val_1 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(arg0.get(i).unwrap().jni_object().clone())
            });
            self.jni_ref()
                .set_object_array_element(&arr, i as i32, val_1.l()?)?;
        }
        let val_1 = jni::objects::JValueGen::Object(arr);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "addItemFlags",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1.l()?)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub fn remove_item_flags(
        &self,
        arg0: Vec<crate::inventory::ItemFlag<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("([Lorg/bukkit/inventory/ItemFlag;)V");
        let arr = self.jni_ref().new_object_array(
            arg0.len() as i32,
            "org/bukkit/inventory/ItemFlag",
            jni::objects::JObject::null(),
        );
        let arr = self.jni_ref().translate_error_no_gen(arr)?;
        for i in 0..arg0.len() {
            let val_1 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(arg0.get(i).unwrap().jni_object().clone())
            });
            self.jni_ref()
                .set_object_array_element(&arr, i as i32, val_1.l()?)?;
        }
        let val_1 = jni::objects::JValueGen::Object(arr);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeItemFlags",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1.l()?)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub fn item_flags(
        &self,
    ) -> Result<blackboxmc_java::util::JavaSet<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/Set;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getItemFlags", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::util::JavaSet::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    pub fn has_item_flag(
        &self,
        arg0: impl Into<crate::inventory::ItemFlag<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = CrossbowMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.has_item_flag(arg0)
    }
    pub fn is_unbreakable(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = CrossbowMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.is_unbreakable()
    }
    pub fn set_unbreakable(&self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = CrossbowMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.set_unbreakable(arg0)
    }
    pub fn has_attribute_modifiers(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "hasAttributeModifiers",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    pub fn attribute_modifiers(
        &self,
    ) -> Result<Option<jni::objects::JObject<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lcom/google/common/collect/Multimap;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getAttributeModifiers",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {
            return Ok(None);
        }
        Ok(Some(res.l()?))
    }
    pub fn add_attribute_modifier(
        &self,
        arg0: impl Into<crate::attribute::Attribute<'mc>>,
        arg1: impl Into<crate::attribute::AttributeModifier<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = CrossbowMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.add_attribute_modifier(arg0, arg1)
    }
    pub fn set_attribute_modifiers(
        &self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lcom/google/common/collect/Multimap;)V");
        let val_1 = jni::objects::JValueGen::Object(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setAttributeModifiers",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub fn remove_attribute_modifier_with_equipment_slot(
        &self,
        arg0: impl Into<crate::inventory::EquipmentSlot<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/inventory/EquipmentSlot;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        sig += ")Z";
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeAttributeModifier",
            sig.as_str(),
            args,
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    pub fn remove_attribute_modifier_with_attribute(
        &self,
        arg0: impl Into<crate::attribute::Attribute<'mc>>,
        arg1: std::option::Option<impl Into<crate::attribute::AttributeModifier<'mc>>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/attribute/Attribute;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        if let Some(a) = arg1 {
            sig += "Lorg/bukkit/attribute/AttributeModifier;";
            let val_2 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(a.into().jni_object().clone())
            });
            args.push(val_2);
        }
        sig += ")Z";
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeAttributeModifier",
            sig.as_str(),
            args,
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn custom_tag_container(
        &self,
    ) -> Result<crate::inventory::meta::tags::CustomItemTagContainer<'mc>, Box<dyn std::error::Error>>
    {
        let temp_clone = CrossbowMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.custom_tag_container()
    }

    pub fn set_version(&self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = CrossbowMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.set_version(arg0)
    }
    pub fn clone(&self) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = CrossbowMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.clone()
    }
    // SUPER CLASS: Cloneable
    // SUPER CLASS: ConfigurationSerializable
    pub fn serialize(
        &self,
    ) -> Result<blackboxmc_java::util::JavaMap<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::configuration::serialization::ConfigurationSerializable::from_raw(
            &self.0,
            unsafe { jni::objects::JObject::from_raw(self.1.clone()) },
        )?;
        let real: crate::configuration::serialization::ConfigurationSerializable =
            temp_clone.into();
        real.serialize()
    }
    // SUPER CLASS: PersistentDataHolder
    pub fn persistent_data_container(
        &self,
    ) -> Result<crate::persistence::PersistentDataContainer<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::persistence::PersistentDataHolder::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::persistence::PersistentDataHolder = temp_clone.into();
        real.persistent_data_container()
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::inventory::meta::ItemMeta<'mc>> for CrossbowMeta<'mc> {
    fn into(self) -> crate::inventory::meta::ItemMeta<'mc> {
        crate::inventory::meta::ItemMeta::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting CrossbowMeta into crate::inventory::meta::ItemMeta")
    }
}
/// This type represents the storage mechanism for auxiliary item data.
/// <p>An implementation will handle the creation and application for ItemMeta. This class should not be implemented by a plugin in a live environment.</p>
///
/// This is a representation of an abstract class.
#[repr(C)]
pub struct ItemMeta<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for ItemMeta<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for ItemMeta<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate ItemMeta from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/inventory/meta/ItemMeta")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a ItemMeta object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> ItemMeta<'mc> {
    pub fn display_name(&self) -> Result<String, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/String;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getDisplayName", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }

    pub fn as_string(&self) -> Result<String, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/String;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getAsString", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }

    pub fn has_display_name(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "hasDisplayName", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn set_display_name(
        &self,
        arg0: impl Into<String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/String;)V");
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(arg0.into())?,
        ));
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setDisplayName",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn has_localized_name(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "hasLocalizedName",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn localized_name(&self) -> Result<String, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/String;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getLocalizedName",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }

    pub fn set_localized_name(
        &self,
        arg0: impl Into<String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/String;)V");
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(arg0.into())?,
        ));
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setLocalizedName",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn has_lore(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasLore", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn lore(&self) -> Result<Option<Vec<String>>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/List;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getLore", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {
            return Ok(None);
        }
        let mut new_vec = Vec::new();
        let list = blackboxmc_java::util::JavaList::from_raw(&self.jni_ref(), res.l()?)?;
        let iter = list.iterator()?;
        while iter.has_next()? {
            let obj = iter.next()?;
            new_vec.push(
                self.jni_ref()
                    .get_string(unsafe { &jni::objects::JString::from_raw(*obj) })?
                    .to_string_lossy()
                    .to_string(),
            );
        }
        Ok(Some(new_vec))
    }

    pub fn set_lore(&self, arg0: Vec<impl Into<String>>) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/util/List;)V");
        let raw_val_1 = self
            .jni_ref()
            .new_object("java/util/ArrayList", "()V", vec![])?;
        for v in arg0 {
            let map_val_0 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
                self.jni_ref().new_string(v.into())?,
            ));
            self.jni_ref().call_method(
                &raw_val_1,
                "add",
                "(Ljava/lang/Object;)Z",
                vec![jni::objects::JValueGen::from(map_val_0)],
            )?;
        }
        let val_1 = jni::objects::JValueGen::Object(raw_val_1);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setLore",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn has_custom_model_data(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "hasCustomModelData",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn custom_model_data(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getCustomModelData",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }

    pub fn set_custom_model_data(&self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/Integer;)V");
        let val_1 = jni::objects::JValueGen::Object(self.jni_ref().new_object(
            "java/lang/Integer",
            "(I)V",
            vec![arg0.into()],
        )?);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setCustomModelData",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn has_enchants(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "hasEnchants", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn has_enchant(
        &self,
        arg0: impl Into<crate::enchantments::Enchantment<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/enchantments/Enchantment;)Z");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "hasEnchant",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn get_enchant_level(
        &self,
        arg0: impl Into<crate::enchantments::Enchantment<'mc>>,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/enchantments/Enchantment;)I");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getEnchantLevel",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }

    pub fn enchants(
        &self,
    ) -> Result<blackboxmc_java::util::JavaMap<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/Map;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getEnchants", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::util::JavaMap::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn add_enchant(
        &self,
        arg0: impl Into<crate::enchantments::Enchantment<'mc>>,
        arg1: i32,
        arg2: bool,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/enchantments/Enchantment;IZ)Z");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let val_2 = jni::objects::JValueGen::Int(arg1);
        let val_3 = jni::objects::JValueGen::Bool(arg2.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "addEnchant",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
                jni::objects::JValueGen::from(val_3),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn remove_enchant(
        &self,
        arg0: impl Into<crate::enchantments::Enchantment<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/enchantments/Enchantment;)Z");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeEnchant",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn has_conflicting_enchant(
        &self,
        arg0: impl Into<crate::enchantments::Enchantment<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/enchantments/Enchantment;)Z");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "hasConflictingEnchant",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn add_item_flags(
        &self,
        arg0: Vec<crate::inventory::ItemFlag<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("([Lorg/bukkit/inventory/ItemFlag;)V");
        let arr = self.jni_ref().new_object_array(
            arg0.len() as i32,
            "org/bukkit/inventory/ItemFlag",
            jni::objects::JObject::null(),
        );
        let arr = self.jni_ref().translate_error_no_gen(arr)?;
        for i in 0..arg0.len() {
            let val_1 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(arg0.get(i).unwrap().jni_object().clone())
            });
            self.jni_ref()
                .set_object_array_element(&arr, i as i32, val_1.l()?)?;
        }
        let val_1 = jni::objects::JValueGen::Object(arr);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "addItemFlags",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1.l()?)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn remove_item_flags(
        &self,
        arg0: Vec<crate::inventory::ItemFlag<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("([Lorg/bukkit/inventory/ItemFlag;)V");
        let arr = self.jni_ref().new_object_array(
            arg0.len() as i32,
            "org/bukkit/inventory/ItemFlag",
            jni::objects::JObject::null(),
        );
        let arr = self.jni_ref().translate_error_no_gen(arr)?;
        for i in 0..arg0.len() {
            let val_1 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(arg0.get(i).unwrap().jni_object().clone())
            });
            self.jni_ref()
                .set_object_array_element(&arr, i as i32, val_1.l()?)?;
        }
        let val_1 = jni::objects::JValueGen::Object(arr);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeItemFlags",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1.l()?)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn item_flags(
        &self,
    ) -> Result<blackboxmc_java::util::JavaSet<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/Set;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getItemFlags", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::util::JavaSet::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn has_item_flag(
        &self,
        arg0: impl Into<crate::inventory::ItemFlag<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/inventory/ItemFlag;)Z");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "hasItemFlag",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn is_unbreakable(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isUnbreakable", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Sets the unbreakable tag. An unbreakable item will not lose durability.
    pub fn set_unbreakable(&self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        let val_1 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setUnbreakable",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn has_attribute_modifiers(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "hasAttributeModifiers",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn attribute_modifiers(
        &self,
    ) -> Result<Option<jni::objects::JObject<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lcom/google/common/collect/Multimap;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getAttributeModifiers",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {
            return Ok(None);
        }
        Ok(Some(res.l()?))
    }

    pub fn add_attribute_modifier(
        &self,
        arg0: impl Into<crate::attribute::Attribute<'mc>>,
        arg1: impl Into<crate::attribute::AttributeModifier<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from(
            "(Lorg/bukkit/attribute/Attribute;Lorg/bukkit/attribute/AttributeModifier;)Z",
        );
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let val_2 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg1.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "addAttributeModifier",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn set_attribute_modifiers(
        &self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lcom/google/common/collect/Multimap;)V");
        let val_1 = jni::objects::JValueGen::Object(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setAttributeModifiers",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn remove_attribute_modifier_with_equipment_slot(
        &self,
        arg0: impl Into<crate::inventory::EquipmentSlot<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/inventory/EquipmentSlot;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        sig += ")Z";
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeAttributeModifier",
            sig.as_str(),
            args,
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn remove_attribute_modifier_with_attribute(
        &self,
        arg0: impl Into<crate::attribute::Attribute<'mc>>,
        arg1: std::option::Option<impl Into<crate::attribute::AttributeModifier<'mc>>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/attribute/Attribute;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        if let Some(a) = arg1 {
            sig += "Lorg/bukkit/attribute/AttributeModifier;";
            let val_2 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(a.into().jni_object().clone())
            });
            args.push(val_2);
        }
        sig += ")Z";
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeAttributeModifier",
            sig.as_str(),
            args,
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    #[deprecated]

    pub fn custom_tag_container(
        &self,
    ) -> Result<crate::inventory::meta::tags::CustomItemTagContainer<'mc>, Box<dyn std::error::Error>>
    {
        let sig = String::from("()Lorg/bukkit/inventory/meta/tags/CustomItemTagContainer;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getCustomTagContainer",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::meta::tags::CustomItemTagContainer::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    #[deprecated = "internal use only "]
    /// Internal use only! Do not use under any circumstances!
    pub fn set_version(&self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(I)V");
        let val_1 = jni::objects::JValueGen::Int(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setVersion",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn clone(&self) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Ljava/lang/Object;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "clone", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l()?)
    }
    pub fn serialize(
        &self,
    ) -> Result<blackboxmc_java::util::JavaMap<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = ItemMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::configuration::serialization::ConfigurationSerializable =
            temp_clone.into();
        real.serialize()
    }
    pub fn persistent_data_container(
        &self,
    ) -> Result<crate::persistence::PersistentDataContainer<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = ItemMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::persistence::PersistentDataHolder = temp_clone.into();
        real.persistent_data_container()
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::configuration::serialization::ConfigurationSerializable<'mc>>
    for ItemMeta<'mc>
{
    fn into(self) -> crate::configuration::serialization::ConfigurationSerializable<'mc> {
        crate::configuration::serialization::ConfigurationSerializable::from_raw(&self.jni_ref(), self.1).expect("Error converting ItemMeta into crate::configuration::serialization::ConfigurationSerializable")
    }
}
impl<'mc> Into<crate::persistence::PersistentDataHolder<'mc>> for ItemMeta<'mc> {
    fn into(self) -> crate::persistence::PersistentDataHolder<'mc> {
        crate::persistence::PersistentDataHolder::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting ItemMeta into crate::persistence::PersistentDataHolder")
    }
}
/// Represents an item that can be repaired at an anvil.
///
/// This is a representation of an abstract class.
#[repr(C)]
pub struct Repairable<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for Repairable<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for Repairable<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Repairable from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/inventory/meta/Repairable")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Repairable object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> Repairable<'mc> {
    pub fn repair_cost(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getRepairCost", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    /// Sets the repair penalty
    pub fn set_repair_cost(&self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(I)V");
        let val_1 = jni::objects::JValueGen::Int(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setRepairCost",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn has_repair_cost(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "hasRepairCost", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn clone(
        &self,
    ) -> Result<crate::inventory::meta::Repairable<'mc>, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Lorg/bukkit/inventory/meta/Repairable;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "clone", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::meta::Repairable::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    pub fn display_name(&self) -> Result<String, Box<dyn std::error::Error>> {
        let temp_clone = Repairable::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.display_name()
    }
    pub fn as_string(&self) -> Result<String, Box<dyn std::error::Error>> {
        let temp_clone = Repairable::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.as_string()
    }
    pub fn has_display_name(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = Repairable::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.has_display_name()
    }
    pub fn set_display_name(
        &self,
        arg0: impl Into<String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = Repairable::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.set_display_name(arg0)
    }
    pub fn has_localized_name(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = Repairable::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.has_localized_name()
    }
    pub fn localized_name(&self) -> Result<String, Box<dyn std::error::Error>> {
        let temp_clone = Repairable::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.localized_name()
    }
    pub fn set_localized_name(
        &self,
        arg0: impl Into<String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = Repairable::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.set_localized_name(arg0)
    }
    pub fn has_lore(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = Repairable::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.has_lore()
    }
    pub fn lore(&self) -> Result<Option<Vec<String>>, Box<dyn std::error::Error>> {
        let temp_clone = Repairable::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.lore()
    }
    pub fn set_lore(&self, arg0: Vec<impl Into<String>>) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = Repairable::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.set_lore(arg0)
    }
    pub fn has_custom_model_data(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = Repairable::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.has_custom_model_data()
    }
    pub fn custom_model_data(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let temp_clone = Repairable::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.custom_model_data()
    }
    pub fn set_custom_model_data(&self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = Repairable::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.set_custom_model_data(arg0)
    }
    pub fn has_enchants(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "hasEnchants", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    pub fn has_enchant(
        &self,
        arg0: impl Into<crate::enchantments::Enchantment<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = Repairable::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.has_enchant(arg0)
    }
    pub fn get_enchant_level(
        &self,
        arg0: impl Into<crate::enchantments::Enchantment<'mc>>,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        let temp_clone = Repairable::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.get_enchant_level(arg0)
    }
    pub fn enchants(
        &self,
    ) -> Result<blackboxmc_java::util::JavaMap<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/Map;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getEnchants", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::util::JavaMap::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    pub fn add_enchant(
        &self,
        arg0: impl Into<crate::enchantments::Enchantment<'mc>>,
        arg1: i32,
        arg2: bool,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = Repairable::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.add_enchant(arg0, arg1, arg2)
    }
    pub fn remove_enchant(
        &self,
        arg0: impl Into<crate::enchantments::Enchantment<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = Repairable::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.remove_enchant(arg0)
    }
    pub fn has_conflicting_enchant(
        &self,
        arg0: impl Into<crate::enchantments::Enchantment<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = Repairable::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.has_conflicting_enchant(arg0)
    }
    pub fn add_item_flags(
        &self,
        arg0: Vec<crate::inventory::ItemFlag<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("([Lorg/bukkit/inventory/ItemFlag;)V");
        let arr = self.jni_ref().new_object_array(
            arg0.len() as i32,
            "org/bukkit/inventory/ItemFlag",
            jni::objects::JObject::null(),
        );
        let arr = self.jni_ref().translate_error_no_gen(arr)?;
        for i in 0..arg0.len() {
            let val_1 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(arg0.get(i).unwrap().jni_object().clone())
            });
            self.jni_ref()
                .set_object_array_element(&arr, i as i32, val_1.l()?)?;
        }
        let val_1 = jni::objects::JValueGen::Object(arr);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "addItemFlags",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1.l()?)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub fn remove_item_flags(
        &self,
        arg0: Vec<crate::inventory::ItemFlag<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("([Lorg/bukkit/inventory/ItemFlag;)V");
        let arr = self.jni_ref().new_object_array(
            arg0.len() as i32,
            "org/bukkit/inventory/ItemFlag",
            jni::objects::JObject::null(),
        );
        let arr = self.jni_ref().translate_error_no_gen(arr)?;
        for i in 0..arg0.len() {
            let val_1 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(arg0.get(i).unwrap().jni_object().clone())
            });
            self.jni_ref()
                .set_object_array_element(&arr, i as i32, val_1.l()?)?;
        }
        let val_1 = jni::objects::JValueGen::Object(arr);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeItemFlags",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1.l()?)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub fn item_flags(
        &self,
    ) -> Result<blackboxmc_java::util::JavaSet<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/Set;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getItemFlags", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::util::JavaSet::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    pub fn has_item_flag(
        &self,
        arg0: impl Into<crate::inventory::ItemFlag<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = Repairable::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.has_item_flag(arg0)
    }
    pub fn is_unbreakable(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = Repairable::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.is_unbreakable()
    }
    pub fn set_unbreakable(&self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = Repairable::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.set_unbreakable(arg0)
    }
    pub fn has_attribute_modifiers(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "hasAttributeModifiers",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    pub fn attribute_modifiers(
        &self,
    ) -> Result<Option<jni::objects::JObject<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lcom/google/common/collect/Multimap;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getAttributeModifiers",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {
            return Ok(None);
        }
        Ok(Some(res.l()?))
    }
    pub fn add_attribute_modifier(
        &self,
        arg0: impl Into<crate::attribute::Attribute<'mc>>,
        arg1: impl Into<crate::attribute::AttributeModifier<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = Repairable::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.add_attribute_modifier(arg0, arg1)
    }
    pub fn set_attribute_modifiers(
        &self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lcom/google/common/collect/Multimap;)V");
        let val_1 = jni::objects::JValueGen::Object(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setAttributeModifiers",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub fn remove_attribute_modifier_with_equipment_slot(
        &self,
        arg0: impl Into<crate::inventory::EquipmentSlot<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/inventory/EquipmentSlot;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        sig += ")Z";
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeAttributeModifier",
            sig.as_str(),
            args,
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    pub fn remove_attribute_modifier_with_attribute(
        &self,
        arg0: impl Into<crate::attribute::Attribute<'mc>>,
        arg1: std::option::Option<impl Into<crate::attribute::AttributeModifier<'mc>>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/attribute/Attribute;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        if let Some(a) = arg1 {
            sig += "Lorg/bukkit/attribute/AttributeModifier;";
            let val_2 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(a.into().jni_object().clone())
            });
            args.push(val_2);
        }
        sig += ")Z";
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeAttributeModifier",
            sig.as_str(),
            args,
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn custom_tag_container(
        &self,
    ) -> Result<crate::inventory::meta::tags::CustomItemTagContainer<'mc>, Box<dyn std::error::Error>>
    {
        let temp_clone = Repairable::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.custom_tag_container()
    }

    pub fn set_version(&self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = Repairable::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.set_version(arg0)
    }
    // SUPER CLASS: Cloneable
    // SUPER CLASS: ConfigurationSerializable
    pub fn serialize(
        &self,
    ) -> Result<blackboxmc_java::util::JavaMap<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::configuration::serialization::ConfigurationSerializable::from_raw(
            &self.0,
            unsafe { jni::objects::JObject::from_raw(self.1.clone()) },
        )?;
        let real: crate::configuration::serialization::ConfigurationSerializable =
            temp_clone.into();
        real.serialize()
    }
    // SUPER CLASS: PersistentDataHolder
    pub fn persistent_data_container(
        &self,
    ) -> Result<crate::persistence::PersistentDataContainer<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::persistence::PersistentDataHolder::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::persistence::PersistentDataHolder = temp_clone.into();
        real.persistent_data_container()
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::inventory::meta::ItemMeta<'mc>> for Repairable<'mc> {
    fn into(self) -> crate::inventory::meta::ItemMeta<'mc> {
        crate::inventory::meta::ItemMeta::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting Repairable into crate::inventory::meta::ItemMeta")
    }
}

///
/// This is a representation of an abstract class.
#[repr(C)]
pub struct BannerMeta<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for BannerMeta<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for BannerMeta<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate BannerMeta from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/inventory/meta/BannerMeta")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a BannerMeta object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> BannerMeta<'mc> {
    #[deprecated]

    pub fn base_color(&self) -> Result<Option<crate::DyeColor<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/DyeColor;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getBaseColor", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {
            return Ok(None);
        }
        Ok(Some(crate::DyeColor::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })?))
    }

    pub fn set_base_color(
        &self,
        arg0: impl Into<crate::DyeColor<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/DyeColor;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setBaseColor",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn patterns(
        &self,
    ) -> Result<Vec<crate::block::banner::Pattern<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/List;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getPatterns", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let list = blackboxmc_java::util::JavaList::from_raw(&self.jni_ref(), res.l()?)?;
        let iter = list.iterator()?;
        while iter.has_next()? {
            let obj = iter.next()?;
            new_vec.push(crate::block::banner::Pattern::from_raw(
                &self.jni_ref(),
                obj,
            )?);
        }
        Ok(new_vec)
    }

    pub fn set_patterns(
        &self,
        arg0: Vec<impl Into<crate::block::banner::Pattern<'mc>>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/util/List;)V");
        let raw_val_1 = self
            .jni_ref()
            .new_object("java/util/ArrayList", "()V", vec![])?;
        for v in arg0 {
            let map_val_0 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(v.into().jni_object().clone())
            });
            self.jni_ref().call_method(
                &raw_val_1,
                "add",
                "(Ljava/lang/Object;)Z",
                vec![jni::objects::JValueGen::from(map_val_0)],
            )?;
        }
        let val_1 = jni::objects::JValueGen::Object(raw_val_1);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setPatterns",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn add_pattern(
        &self,
        arg0: impl Into<crate::block::banner::Pattern<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/banner/Pattern;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "addPattern",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Returns the pattern at the specified index
    pub fn get_pattern(
        &self,
        arg0: i32,
    ) -> Result<crate::block::banner::Pattern<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(I)Lorg/bukkit/block/banner/Pattern;");
        let val_1 = jni::objects::JValueGen::Int(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getPattern",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::block::banner::Pattern::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Removes the pattern at the specified index
    pub fn remove_pattern(
        &self,
        arg0: i32,
    ) -> Result<crate::block::banner::Pattern<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(I)Lorg/bukkit/block/banner/Pattern;");
        let val_1 = jni::objects::JValueGen::Int(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removePattern",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::block::banner::Pattern::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn set_pattern(
        &self,
        arg0: i32,
        arg1: impl Into<crate::block::banner::Pattern<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(ILorg/bukkit/block/banner/Pattern;)V");
        let val_1 = jni::objects::JValueGen::Int(arg0);
        let val_2 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg1.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setPattern",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
            ],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn number_of_patterns(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "numberOfPatterns",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    pub fn display_name(&self) -> Result<String, Box<dyn std::error::Error>> {
        let temp_clone = BannerMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.display_name()
    }
    pub fn as_string(&self) -> Result<String, Box<dyn std::error::Error>> {
        let temp_clone = BannerMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.as_string()
    }
    pub fn has_display_name(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = BannerMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.has_display_name()
    }
    pub fn set_display_name(
        &self,
        arg0: impl Into<String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = BannerMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.set_display_name(arg0)
    }
    pub fn has_localized_name(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = BannerMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.has_localized_name()
    }
    pub fn localized_name(&self) -> Result<String, Box<dyn std::error::Error>> {
        let temp_clone = BannerMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.localized_name()
    }
    pub fn set_localized_name(
        &self,
        arg0: impl Into<String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = BannerMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.set_localized_name(arg0)
    }
    pub fn has_lore(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = BannerMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.has_lore()
    }
    pub fn lore(&self) -> Result<Option<Vec<String>>, Box<dyn std::error::Error>> {
        let temp_clone = BannerMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.lore()
    }
    pub fn set_lore(&self, arg0: Vec<impl Into<String>>) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = BannerMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.set_lore(arg0)
    }
    pub fn has_custom_model_data(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = BannerMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.has_custom_model_data()
    }
    pub fn custom_model_data(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let temp_clone = BannerMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.custom_model_data()
    }
    pub fn set_custom_model_data(&self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = BannerMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.set_custom_model_data(arg0)
    }
    pub fn has_enchants(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "hasEnchants", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    pub fn has_enchant(
        &self,
        arg0: impl Into<crate::enchantments::Enchantment<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = BannerMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.has_enchant(arg0)
    }
    pub fn get_enchant_level(
        &self,
        arg0: impl Into<crate::enchantments::Enchantment<'mc>>,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        let temp_clone = BannerMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.get_enchant_level(arg0)
    }
    pub fn enchants(
        &self,
    ) -> Result<blackboxmc_java::util::JavaMap<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/Map;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getEnchants", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::util::JavaMap::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    pub fn add_enchant(
        &self,
        arg0: impl Into<crate::enchantments::Enchantment<'mc>>,
        arg1: i32,
        arg2: bool,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = BannerMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.add_enchant(arg0, arg1, arg2)
    }
    pub fn remove_enchant(
        &self,
        arg0: impl Into<crate::enchantments::Enchantment<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = BannerMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.remove_enchant(arg0)
    }
    pub fn has_conflicting_enchant(
        &self,
        arg0: impl Into<crate::enchantments::Enchantment<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = BannerMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.has_conflicting_enchant(arg0)
    }
    pub fn add_item_flags(
        &self,
        arg0: Vec<crate::inventory::ItemFlag<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("([Lorg/bukkit/inventory/ItemFlag;)V");
        let arr = self.jni_ref().new_object_array(
            arg0.len() as i32,
            "org/bukkit/inventory/ItemFlag",
            jni::objects::JObject::null(),
        );
        let arr = self.jni_ref().translate_error_no_gen(arr)?;
        for i in 0..arg0.len() {
            let val_1 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(arg0.get(i).unwrap().jni_object().clone())
            });
            self.jni_ref()
                .set_object_array_element(&arr, i as i32, val_1.l()?)?;
        }
        let val_1 = jni::objects::JValueGen::Object(arr);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "addItemFlags",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1.l()?)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub fn remove_item_flags(
        &self,
        arg0: Vec<crate::inventory::ItemFlag<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("([Lorg/bukkit/inventory/ItemFlag;)V");
        let arr = self.jni_ref().new_object_array(
            arg0.len() as i32,
            "org/bukkit/inventory/ItemFlag",
            jni::objects::JObject::null(),
        );
        let arr = self.jni_ref().translate_error_no_gen(arr)?;
        for i in 0..arg0.len() {
            let val_1 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(arg0.get(i).unwrap().jni_object().clone())
            });
            self.jni_ref()
                .set_object_array_element(&arr, i as i32, val_1.l()?)?;
        }
        let val_1 = jni::objects::JValueGen::Object(arr);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeItemFlags",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1.l()?)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub fn item_flags(
        &self,
    ) -> Result<blackboxmc_java::util::JavaSet<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/Set;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getItemFlags", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::util::JavaSet::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    pub fn has_item_flag(
        &self,
        arg0: impl Into<crate::inventory::ItemFlag<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = BannerMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.has_item_flag(arg0)
    }
    pub fn is_unbreakable(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = BannerMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.is_unbreakable()
    }
    pub fn set_unbreakable(&self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = BannerMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.set_unbreakable(arg0)
    }
    pub fn has_attribute_modifiers(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "hasAttributeModifiers",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    pub fn attribute_modifiers(
        &self,
    ) -> Result<Option<jni::objects::JObject<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lcom/google/common/collect/Multimap;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getAttributeModifiers",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {
            return Ok(None);
        }
        Ok(Some(res.l()?))
    }
    pub fn add_attribute_modifier(
        &self,
        arg0: impl Into<crate::attribute::Attribute<'mc>>,
        arg1: impl Into<crate::attribute::AttributeModifier<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = BannerMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.add_attribute_modifier(arg0, arg1)
    }
    pub fn set_attribute_modifiers(
        &self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lcom/google/common/collect/Multimap;)V");
        let val_1 = jni::objects::JValueGen::Object(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setAttributeModifiers",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub fn remove_attribute_modifier_with_equipment_slot(
        &self,
        arg0: impl Into<crate::inventory::EquipmentSlot<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/inventory/EquipmentSlot;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        sig += ")Z";
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeAttributeModifier",
            sig.as_str(),
            args,
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    pub fn remove_attribute_modifier_with_attribute(
        &self,
        arg0: impl Into<crate::attribute::Attribute<'mc>>,
        arg1: std::option::Option<impl Into<crate::attribute::AttributeModifier<'mc>>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/attribute/Attribute;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        if let Some(a) = arg1 {
            sig += "Lorg/bukkit/attribute/AttributeModifier;";
            let val_2 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(a.into().jni_object().clone())
            });
            args.push(val_2);
        }
        sig += ")Z";
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeAttributeModifier",
            sig.as_str(),
            args,
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn custom_tag_container(
        &self,
    ) -> Result<crate::inventory::meta::tags::CustomItemTagContainer<'mc>, Box<dyn std::error::Error>>
    {
        let temp_clone = BannerMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.custom_tag_container()
    }

    pub fn set_version(&self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = BannerMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.set_version(arg0)
    }
    pub fn clone(&self) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = BannerMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.clone()
    }
    // SUPER CLASS: Cloneable
    // SUPER CLASS: ConfigurationSerializable
    pub fn serialize(
        &self,
    ) -> Result<blackboxmc_java::util::JavaMap<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::configuration::serialization::ConfigurationSerializable::from_raw(
            &self.0,
            unsafe { jni::objects::JObject::from_raw(self.1.clone()) },
        )?;
        let real: crate::configuration::serialization::ConfigurationSerializable =
            temp_clone.into();
        real.serialize()
    }
    // SUPER CLASS: PersistentDataHolder
    pub fn persistent_data_container(
        &self,
    ) -> Result<crate::persistence::PersistentDataContainer<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::persistence::PersistentDataHolder::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::persistence::PersistentDataHolder = temp_clone.into();
        real.persistent_data_container()
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::inventory::meta::ItemMeta<'mc>> for BannerMeta<'mc> {
    fn into(self) -> crate::inventory::meta::ItemMeta<'mc> {
        crate::inventory::meta::ItemMeta::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting BannerMeta into crate::inventory::meta::ItemMeta")
    }
}

///
/// This is a representation of an abstract class.
#[repr(C)]
pub struct BundleMeta<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for BundleMeta<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for BundleMeta<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate BundleMeta from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/inventory/meta/BundleMeta")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a BundleMeta object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> BundleMeta<'mc> {
    pub fn add_item(
        &self,
        arg0: impl Into<crate::inventory::ItemStack<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/inventory/ItemStack;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "addItem",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn has_items(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasItems", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn items(
        &self,
    ) -> Result<Vec<crate::inventory::ItemStack<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/List;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getItems", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let list = blackboxmc_java::util::JavaList::from_raw(&self.jni_ref(), res.l()?)?;
        let iter = list.iterator()?;
        while iter.has_next()? {
            let obj = iter.next()?;
            new_vec.push(crate::inventory::ItemStack::from_raw(&self.jni_ref(), obj)?);
        }
        Ok(new_vec)
    }

    pub fn set_items(
        &self,
        arg0: Vec<impl Into<crate::inventory::ItemStack<'mc>>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/util/List;)V");
        let raw_val_1 = self
            .jni_ref()
            .new_object("java/util/ArrayList", "()V", vec![])?;
        for v in arg0 {
            let map_val_0 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(v.into().jni_object().clone())
            });
            self.jni_ref().call_method(
                &raw_val_1,
                "add",
                "(Ljava/lang/Object;)Z",
                vec![jni::objects::JValueGen::from(map_val_0)],
            )?;
        }
        let val_1 = jni::objects::JValueGen::Object(raw_val_1);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setItems",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub fn display_name(&self) -> Result<String, Box<dyn std::error::Error>> {
        let temp_clone = BundleMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.display_name()
    }
    pub fn as_string(&self) -> Result<String, Box<dyn std::error::Error>> {
        let temp_clone = BundleMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.as_string()
    }
    pub fn has_display_name(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = BundleMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.has_display_name()
    }
    pub fn set_display_name(
        &self,
        arg0: impl Into<String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = BundleMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.set_display_name(arg0)
    }
    pub fn has_localized_name(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = BundleMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.has_localized_name()
    }
    pub fn localized_name(&self) -> Result<String, Box<dyn std::error::Error>> {
        let temp_clone = BundleMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.localized_name()
    }
    pub fn set_localized_name(
        &self,
        arg0: impl Into<String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = BundleMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.set_localized_name(arg0)
    }
    pub fn has_lore(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = BundleMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.has_lore()
    }
    pub fn lore(&self) -> Result<Option<Vec<String>>, Box<dyn std::error::Error>> {
        let temp_clone = BundleMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.lore()
    }
    pub fn set_lore(&self, arg0: Vec<impl Into<String>>) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = BundleMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.set_lore(arg0)
    }
    pub fn has_custom_model_data(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = BundleMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.has_custom_model_data()
    }
    pub fn custom_model_data(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let temp_clone = BundleMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.custom_model_data()
    }
    pub fn set_custom_model_data(&self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = BundleMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.set_custom_model_data(arg0)
    }
    pub fn has_enchants(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "hasEnchants", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    pub fn has_enchant(
        &self,
        arg0: impl Into<crate::enchantments::Enchantment<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = BundleMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.has_enchant(arg0)
    }
    pub fn get_enchant_level(
        &self,
        arg0: impl Into<crate::enchantments::Enchantment<'mc>>,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        let temp_clone = BundleMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.get_enchant_level(arg0)
    }
    pub fn enchants(
        &self,
    ) -> Result<blackboxmc_java::util::JavaMap<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/Map;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getEnchants", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::util::JavaMap::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    pub fn add_enchant(
        &self,
        arg0: impl Into<crate::enchantments::Enchantment<'mc>>,
        arg1: i32,
        arg2: bool,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = BundleMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.add_enchant(arg0, arg1, arg2)
    }
    pub fn remove_enchant(
        &self,
        arg0: impl Into<crate::enchantments::Enchantment<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = BundleMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.remove_enchant(arg0)
    }
    pub fn has_conflicting_enchant(
        &self,
        arg0: impl Into<crate::enchantments::Enchantment<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = BundleMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.has_conflicting_enchant(arg0)
    }
    pub fn add_item_flags(
        &self,
        arg0: Vec<crate::inventory::ItemFlag<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("([Lorg/bukkit/inventory/ItemFlag;)V");
        let arr = self.jni_ref().new_object_array(
            arg0.len() as i32,
            "org/bukkit/inventory/ItemFlag",
            jni::objects::JObject::null(),
        );
        let arr = self.jni_ref().translate_error_no_gen(arr)?;
        for i in 0..arg0.len() {
            let val_1 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(arg0.get(i).unwrap().jni_object().clone())
            });
            self.jni_ref()
                .set_object_array_element(&arr, i as i32, val_1.l()?)?;
        }
        let val_1 = jni::objects::JValueGen::Object(arr);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "addItemFlags",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1.l()?)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub fn remove_item_flags(
        &self,
        arg0: Vec<crate::inventory::ItemFlag<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("([Lorg/bukkit/inventory/ItemFlag;)V");
        let arr = self.jni_ref().new_object_array(
            arg0.len() as i32,
            "org/bukkit/inventory/ItemFlag",
            jni::objects::JObject::null(),
        );
        let arr = self.jni_ref().translate_error_no_gen(arr)?;
        for i in 0..arg0.len() {
            let val_1 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(arg0.get(i).unwrap().jni_object().clone())
            });
            self.jni_ref()
                .set_object_array_element(&arr, i as i32, val_1.l()?)?;
        }
        let val_1 = jni::objects::JValueGen::Object(arr);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeItemFlags",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1.l()?)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub fn item_flags(
        &self,
    ) -> Result<blackboxmc_java::util::JavaSet<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/Set;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getItemFlags", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::util::JavaSet::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    pub fn has_item_flag(
        &self,
        arg0: impl Into<crate::inventory::ItemFlag<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = BundleMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.has_item_flag(arg0)
    }
    pub fn is_unbreakable(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = BundleMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.is_unbreakable()
    }
    pub fn set_unbreakable(&self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = BundleMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.set_unbreakable(arg0)
    }
    pub fn has_attribute_modifiers(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "hasAttributeModifiers",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    pub fn attribute_modifiers(
        &self,
    ) -> Result<Option<jni::objects::JObject<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lcom/google/common/collect/Multimap;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getAttributeModifiers",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {
            return Ok(None);
        }
        Ok(Some(res.l()?))
    }
    pub fn add_attribute_modifier(
        &self,
        arg0: impl Into<crate::attribute::Attribute<'mc>>,
        arg1: impl Into<crate::attribute::AttributeModifier<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = BundleMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.add_attribute_modifier(arg0, arg1)
    }
    pub fn set_attribute_modifiers(
        &self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lcom/google/common/collect/Multimap;)V");
        let val_1 = jni::objects::JValueGen::Object(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setAttributeModifiers",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub fn remove_attribute_modifier_with_equipment_slot(
        &self,
        arg0: impl Into<crate::inventory::EquipmentSlot<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/inventory/EquipmentSlot;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        sig += ")Z";
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeAttributeModifier",
            sig.as_str(),
            args,
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    pub fn remove_attribute_modifier_with_attribute(
        &self,
        arg0: impl Into<crate::attribute::Attribute<'mc>>,
        arg1: std::option::Option<impl Into<crate::attribute::AttributeModifier<'mc>>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/attribute/Attribute;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        if let Some(a) = arg1 {
            sig += "Lorg/bukkit/attribute/AttributeModifier;";
            let val_2 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(a.into().jni_object().clone())
            });
            args.push(val_2);
        }
        sig += ")Z";
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeAttributeModifier",
            sig.as_str(),
            args,
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn custom_tag_container(
        &self,
    ) -> Result<crate::inventory::meta::tags::CustomItemTagContainer<'mc>, Box<dyn std::error::Error>>
    {
        let temp_clone = BundleMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.custom_tag_container()
    }

    pub fn set_version(&self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = BundleMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.set_version(arg0)
    }
    pub fn clone(&self) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = BundleMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.clone()
    }
    // SUPER CLASS: Cloneable
    // SUPER CLASS: ConfigurationSerializable
    pub fn serialize(
        &self,
    ) -> Result<blackboxmc_java::util::JavaMap<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::configuration::serialization::ConfigurationSerializable::from_raw(
            &self.0,
            unsafe { jni::objects::JObject::from_raw(self.1.clone()) },
        )?;
        let real: crate::configuration::serialization::ConfigurationSerializable =
            temp_clone.into();
        real.serialize()
    }
    // SUPER CLASS: PersistentDataHolder
    pub fn persistent_data_container(
        &self,
    ) -> Result<crate::persistence::PersistentDataContainer<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::persistence::PersistentDataHolder::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::persistence::PersistentDataHolder = temp_clone.into();
        real.persistent_data_container()
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::inventory::meta::ItemMeta<'mc>> for BundleMeta<'mc> {
    fn into(self) -> crate::inventory::meta::ItemMeta<'mc> {
        crate::inventory::meta::ItemMeta::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting BundleMeta into crate::inventory::meta::ItemMeta")
    }
}
/// Represents armor that an entity can equip and can also be colored.
///
/// This is a representation of an abstract class.
#[repr(C)]
pub struct ColorableArmorMeta<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for ColorableArmorMeta<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for ColorableArmorMeta<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate ColorableArmorMeta from null object.").into(),
            );
        }
        let (valid, name) =
            env.validate_name(&obj, "org/bukkit/inventory/meta/ColorableArmorMeta")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a ColorableArmorMeta object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> ColorableArmorMeta<'mc> {
    pub fn clone(
        &self,
    ) -> Result<crate::inventory::meta::ColorableArmorMeta<'mc>, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Lorg/bukkit/inventory/meta/ColorableArmorMeta;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "clone", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::meta::ColorableArmorMeta::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    pub fn has_trim(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = ColorableArmorMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ArmorMeta = temp_clone.into();
        real.has_trim()
    }
    pub fn set_trim(
        &self,
        arg0: impl Into<crate::inventory::meta::trim::ArmorTrim<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = ColorableArmorMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ArmorMeta = temp_clone.into();
        real.set_trim(arg0)
    }
    pub fn trim(
        &self,
    ) -> Result<Option<crate::inventory::meta::trim::ArmorTrim<'mc>>, Box<dyn std::error::Error>>
    {
        let temp_clone = ColorableArmorMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ArmorMeta = temp_clone.into();
        real.trim()
    }
    // SUPER CLASS: ItemMeta
    pub fn serialize(
        &self,
    ) -> Result<blackboxmc_java::util::JavaMap<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::inventory::meta::ItemMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::configuration::serialization::ConfigurationSerializable =
            temp_clone.into();
        real.serialize()
    }
    pub fn persistent_data_container(
        &self,
    ) -> Result<crate::persistence::PersistentDataContainer<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::inventory::meta::ItemMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::persistence::PersistentDataHolder = temp_clone.into();
        real.persistent_data_container()
    }
    pub fn display_name(&self) -> Result<String, Box<dyn std::error::Error>> {
        let temp_clone = crate::inventory::meta::ItemMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.display_name()
    }
    pub fn as_string(&self) -> Result<String, Box<dyn std::error::Error>> {
        let temp_clone = crate::inventory::meta::ItemMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.as_string()
    }
    pub fn has_display_name(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::inventory::meta::ItemMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.has_display_name()
    }
    pub fn set_display_name(
        &self,
        arg0: impl Into<String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::inventory::meta::ItemMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.set_display_name(arg0)
    }
    pub fn has_localized_name(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::inventory::meta::ItemMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.has_localized_name()
    }
    pub fn localized_name(&self) -> Result<String, Box<dyn std::error::Error>> {
        let temp_clone = crate::inventory::meta::ItemMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.localized_name()
    }
    pub fn set_localized_name(
        &self,
        arg0: impl Into<String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::inventory::meta::ItemMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.set_localized_name(arg0)
    }
    pub fn has_lore(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::inventory::meta::ItemMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.has_lore()
    }
    pub fn lore(&self) -> Result<Option<Vec<String>>, Box<dyn std::error::Error>> {
        let temp_clone = crate::inventory::meta::ItemMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.lore()
    }
    pub fn set_lore(&self, arg0: Vec<impl Into<String>>) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::inventory::meta::ItemMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.set_lore(arg0)
    }
    pub fn has_custom_model_data(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::inventory::meta::ItemMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.has_custom_model_data()
    }
    pub fn custom_model_data(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let temp_clone = crate::inventory::meta::ItemMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.custom_model_data()
    }
    pub fn set_custom_model_data(&self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::inventory::meta::ItemMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.set_custom_model_data(arg0)
    }
    pub fn has_enchants(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "hasEnchants", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    pub fn has_enchant(
        &self,
        arg0: impl Into<crate::enchantments::Enchantment<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::inventory::meta::ItemMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.has_enchant(arg0)
    }
    pub fn get_enchant_level(
        &self,
        arg0: impl Into<crate::enchantments::Enchantment<'mc>>,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        let temp_clone = crate::inventory::meta::ItemMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.get_enchant_level(arg0)
    }
    pub fn enchants(
        &self,
    ) -> Result<blackboxmc_java::util::JavaMap<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/Map;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getEnchants", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::util::JavaMap::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    pub fn add_enchant(
        &self,
        arg0: impl Into<crate::enchantments::Enchantment<'mc>>,
        arg1: i32,
        arg2: bool,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::inventory::meta::ItemMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.add_enchant(arg0, arg1, arg2)
    }
    pub fn remove_enchant(
        &self,
        arg0: impl Into<crate::enchantments::Enchantment<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::inventory::meta::ItemMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.remove_enchant(arg0)
    }
    pub fn has_conflicting_enchant(
        &self,
        arg0: impl Into<crate::enchantments::Enchantment<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::inventory::meta::ItemMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.has_conflicting_enchant(arg0)
    }
    pub fn add_item_flags(
        &self,
        arg0: Vec<crate::inventory::ItemFlag<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("([Lorg/bukkit/inventory/ItemFlag;)V");
        let arr = self.jni_ref().new_object_array(
            arg0.len() as i32,
            "org/bukkit/inventory/ItemFlag",
            jni::objects::JObject::null(),
        );
        let arr = self.jni_ref().translate_error_no_gen(arr)?;
        for i in 0..arg0.len() {
            let val_1 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(arg0.get(i).unwrap().jni_object().clone())
            });
            self.jni_ref()
                .set_object_array_element(&arr, i as i32, val_1.l()?)?;
        }
        let val_1 = jni::objects::JValueGen::Object(arr);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "addItemFlags",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1.l()?)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub fn remove_item_flags(
        &self,
        arg0: Vec<crate::inventory::ItemFlag<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("([Lorg/bukkit/inventory/ItemFlag;)V");
        let arr = self.jni_ref().new_object_array(
            arg0.len() as i32,
            "org/bukkit/inventory/ItemFlag",
            jni::objects::JObject::null(),
        );
        let arr = self.jni_ref().translate_error_no_gen(arr)?;
        for i in 0..arg0.len() {
            let val_1 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(arg0.get(i).unwrap().jni_object().clone())
            });
            self.jni_ref()
                .set_object_array_element(&arr, i as i32, val_1.l()?)?;
        }
        let val_1 = jni::objects::JValueGen::Object(arr);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeItemFlags",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1.l()?)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub fn item_flags(
        &self,
    ) -> Result<blackboxmc_java::util::JavaSet<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/Set;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getItemFlags", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::util::JavaSet::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    pub fn has_item_flag(
        &self,
        arg0: impl Into<crate::inventory::ItemFlag<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::inventory::meta::ItemMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.has_item_flag(arg0)
    }
    pub fn is_unbreakable(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::inventory::meta::ItemMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.is_unbreakable()
    }
    pub fn set_unbreakable(&self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::inventory::meta::ItemMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.set_unbreakable(arg0)
    }
    pub fn has_attribute_modifiers(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "hasAttributeModifiers",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    pub fn attribute_modifiers(
        &self,
    ) -> Result<Option<jni::objects::JObject<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lcom/google/common/collect/Multimap;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getAttributeModifiers",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {
            return Ok(None);
        }
        Ok(Some(res.l()?))
    }
    pub fn add_attribute_modifier(
        &self,
        arg0: impl Into<crate::attribute::Attribute<'mc>>,
        arg1: impl Into<crate::attribute::AttributeModifier<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::inventory::meta::ItemMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.add_attribute_modifier(arg0, arg1)
    }
    pub fn set_attribute_modifiers(
        &self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lcom/google/common/collect/Multimap;)V");
        let val_1 = jni::objects::JValueGen::Object(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setAttributeModifiers",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub fn remove_attribute_modifier_with_equipment_slot(
        &self,
        arg0: impl Into<crate::inventory::EquipmentSlot<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/inventory/EquipmentSlot;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        sig += ")Z";
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeAttributeModifier",
            sig.as_str(),
            args,
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    pub fn remove_attribute_modifier_with_attribute(
        &self,
        arg0: impl Into<crate::attribute::Attribute<'mc>>,
        arg1: std::option::Option<impl Into<crate::attribute::AttributeModifier<'mc>>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/attribute/Attribute;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        if let Some(a) = arg1 {
            sig += "Lorg/bukkit/attribute/AttributeModifier;";
            let val_2 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(a.into().jni_object().clone())
            });
            args.push(val_2);
        }
        sig += ")Z";
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeAttributeModifier",
            sig.as_str(),
            args,
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn custom_tag_container(
        &self,
    ) -> Result<crate::inventory::meta::tags::CustomItemTagContainer<'mc>, Box<dyn std::error::Error>>
    {
        let temp_clone = crate::inventory::meta::ItemMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.custom_tag_container()
    }

    pub fn set_version(&self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::inventory::meta::ItemMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.set_version(arg0)
    }
    pub fn set_color(
        &self,
        arg0: impl Into<crate::Color<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = ColorableArmorMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::LeatherArmorMeta = temp_clone.into();
        real.set_color(arg0)
    }
    pub fn color(&self) -> Result<Option<crate::Color<'mc>>, Box<dyn std::error::Error>> {
        let temp_clone = ColorableArmorMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::LeatherArmorMeta = temp_clone.into();
        real.color()
    }
    // SUPER CLASS: ItemMeta

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::inventory::meta::ArmorMeta<'mc>> for ColorableArmorMeta<'mc> {
    fn into(self) -> crate::inventory::meta::ArmorMeta<'mc> {
        crate::inventory::meta::ArmorMeta::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting ColorableArmorMeta into crate::inventory::meta::ArmorMeta")
    }
}
impl<'mc> Into<crate::inventory::meta::LeatherArmorMeta<'mc>> for ColorableArmorMeta<'mc> {
    fn into(self) -> crate::inventory::meta::LeatherArmorMeta<'mc> {
        crate::inventory::meta::LeatherArmorMeta::from_raw(&self.jni_ref(), self.1).expect(
            "Error converting ColorableArmorMeta into crate::inventory::meta::LeatherArmorMeta",
        )
    }
}
/// Represents a spawn egg and it's spawned type.
///
/// This is a representation of an abstract class.
#[repr(C)]
pub struct SpawnEggMeta<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for SpawnEggMeta<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for SpawnEggMeta<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate SpawnEggMeta from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/inventory/meta/SpawnEggMeta")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a SpawnEggMeta object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> SpawnEggMeta<'mc> {
    pub fn spawned_type(
        &self,
    ) -> Result<crate::entity::EntityType<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/entity/EntityType;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getSpawnedType", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::entity::EntityType::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn set_spawned_type(
        &self,
        arg0: impl Into<crate::entity::EntityType<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/entity/EntityType;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setSpawnedType",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn clone(
        &self,
    ) -> Result<crate::inventory::meta::SpawnEggMeta<'mc>, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Lorg/bukkit/inventory/meta/SpawnEggMeta;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "clone", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::meta::SpawnEggMeta::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    pub fn display_name(&self) -> Result<String, Box<dyn std::error::Error>> {
        let temp_clone = SpawnEggMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.display_name()
    }
    pub fn as_string(&self) -> Result<String, Box<dyn std::error::Error>> {
        let temp_clone = SpawnEggMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.as_string()
    }
    pub fn has_display_name(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = SpawnEggMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.has_display_name()
    }
    pub fn set_display_name(
        &self,
        arg0: impl Into<String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = SpawnEggMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.set_display_name(arg0)
    }
    pub fn has_localized_name(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = SpawnEggMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.has_localized_name()
    }
    pub fn localized_name(&self) -> Result<String, Box<dyn std::error::Error>> {
        let temp_clone = SpawnEggMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.localized_name()
    }
    pub fn set_localized_name(
        &self,
        arg0: impl Into<String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = SpawnEggMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.set_localized_name(arg0)
    }
    pub fn has_lore(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = SpawnEggMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.has_lore()
    }
    pub fn lore(&self) -> Result<Option<Vec<String>>, Box<dyn std::error::Error>> {
        let temp_clone = SpawnEggMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.lore()
    }
    pub fn set_lore(&self, arg0: Vec<impl Into<String>>) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = SpawnEggMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.set_lore(arg0)
    }
    pub fn has_custom_model_data(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = SpawnEggMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.has_custom_model_data()
    }
    pub fn custom_model_data(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let temp_clone = SpawnEggMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.custom_model_data()
    }
    pub fn set_custom_model_data(&self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = SpawnEggMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.set_custom_model_data(arg0)
    }
    pub fn has_enchants(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "hasEnchants", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    pub fn has_enchant(
        &self,
        arg0: impl Into<crate::enchantments::Enchantment<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = SpawnEggMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.has_enchant(arg0)
    }
    pub fn get_enchant_level(
        &self,
        arg0: impl Into<crate::enchantments::Enchantment<'mc>>,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        let temp_clone = SpawnEggMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.get_enchant_level(arg0)
    }
    pub fn enchants(
        &self,
    ) -> Result<blackboxmc_java::util::JavaMap<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/Map;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getEnchants", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::util::JavaMap::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    pub fn add_enchant(
        &self,
        arg0: impl Into<crate::enchantments::Enchantment<'mc>>,
        arg1: i32,
        arg2: bool,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = SpawnEggMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.add_enchant(arg0, arg1, arg2)
    }
    pub fn remove_enchant(
        &self,
        arg0: impl Into<crate::enchantments::Enchantment<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = SpawnEggMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.remove_enchant(arg0)
    }
    pub fn has_conflicting_enchant(
        &self,
        arg0: impl Into<crate::enchantments::Enchantment<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = SpawnEggMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.has_conflicting_enchant(arg0)
    }
    pub fn add_item_flags(
        &self,
        arg0: Vec<crate::inventory::ItemFlag<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("([Lorg/bukkit/inventory/ItemFlag;)V");
        let arr = self.jni_ref().new_object_array(
            arg0.len() as i32,
            "org/bukkit/inventory/ItemFlag",
            jni::objects::JObject::null(),
        );
        let arr = self.jni_ref().translate_error_no_gen(arr)?;
        for i in 0..arg0.len() {
            let val_1 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(arg0.get(i).unwrap().jni_object().clone())
            });
            self.jni_ref()
                .set_object_array_element(&arr, i as i32, val_1.l()?)?;
        }
        let val_1 = jni::objects::JValueGen::Object(arr);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "addItemFlags",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1.l()?)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub fn remove_item_flags(
        &self,
        arg0: Vec<crate::inventory::ItemFlag<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("([Lorg/bukkit/inventory/ItemFlag;)V");
        let arr = self.jni_ref().new_object_array(
            arg0.len() as i32,
            "org/bukkit/inventory/ItemFlag",
            jni::objects::JObject::null(),
        );
        let arr = self.jni_ref().translate_error_no_gen(arr)?;
        for i in 0..arg0.len() {
            let val_1 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(arg0.get(i).unwrap().jni_object().clone())
            });
            self.jni_ref()
                .set_object_array_element(&arr, i as i32, val_1.l()?)?;
        }
        let val_1 = jni::objects::JValueGen::Object(arr);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeItemFlags",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1.l()?)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub fn item_flags(
        &self,
    ) -> Result<blackboxmc_java::util::JavaSet<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/Set;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getItemFlags", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::util::JavaSet::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    pub fn has_item_flag(
        &self,
        arg0: impl Into<crate::inventory::ItemFlag<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = SpawnEggMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.has_item_flag(arg0)
    }
    pub fn is_unbreakable(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = SpawnEggMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.is_unbreakable()
    }
    pub fn set_unbreakable(&self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = SpawnEggMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.set_unbreakable(arg0)
    }
    pub fn has_attribute_modifiers(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "hasAttributeModifiers",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    pub fn attribute_modifiers(
        &self,
    ) -> Result<Option<jni::objects::JObject<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lcom/google/common/collect/Multimap;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getAttributeModifiers",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {
            return Ok(None);
        }
        Ok(Some(res.l()?))
    }
    pub fn add_attribute_modifier(
        &self,
        arg0: impl Into<crate::attribute::Attribute<'mc>>,
        arg1: impl Into<crate::attribute::AttributeModifier<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = SpawnEggMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.add_attribute_modifier(arg0, arg1)
    }
    pub fn set_attribute_modifiers(
        &self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lcom/google/common/collect/Multimap;)V");
        let val_1 = jni::objects::JValueGen::Object(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setAttributeModifiers",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub fn remove_attribute_modifier_with_equipment_slot(
        &self,
        arg0: impl Into<crate::inventory::EquipmentSlot<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/inventory/EquipmentSlot;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        sig += ")Z";
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeAttributeModifier",
            sig.as_str(),
            args,
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    pub fn remove_attribute_modifier_with_attribute(
        &self,
        arg0: impl Into<crate::attribute::Attribute<'mc>>,
        arg1: std::option::Option<impl Into<crate::attribute::AttributeModifier<'mc>>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/attribute/Attribute;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        if let Some(a) = arg1 {
            sig += "Lorg/bukkit/attribute/AttributeModifier;";
            let val_2 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(a.into().jni_object().clone())
            });
            args.push(val_2);
        }
        sig += ")Z";
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeAttributeModifier",
            sig.as_str(),
            args,
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn custom_tag_container(
        &self,
    ) -> Result<crate::inventory::meta::tags::CustomItemTagContainer<'mc>, Box<dyn std::error::Error>>
    {
        let temp_clone = SpawnEggMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.custom_tag_container()
    }

    pub fn set_version(&self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = SpawnEggMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.set_version(arg0)
    }
    // SUPER CLASS: Cloneable
    // SUPER CLASS: ConfigurationSerializable
    pub fn serialize(
        &self,
    ) -> Result<blackboxmc_java::util::JavaMap<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::configuration::serialization::ConfigurationSerializable::from_raw(
            &self.0,
            unsafe { jni::objects::JObject::from_raw(self.1.clone()) },
        )?;
        let real: crate::configuration::serialization::ConfigurationSerializable =
            temp_clone.into();
        real.serialize()
    }
    // SUPER CLASS: PersistentDataHolder
    pub fn persistent_data_container(
        &self,
    ) -> Result<crate::persistence::PersistentDataContainer<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::persistence::PersistentDataHolder::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::persistence::PersistentDataHolder = temp_clone.into();
        real.persistent_data_container()
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::inventory::meta::ItemMeta<'mc>> for SpawnEggMeta<'mc> {
    fn into(self) -> crate::inventory::meta::ItemMeta<'mc> {
        crate::inventory::meta::ItemMeta::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting SpawnEggMeta into crate::inventory::meta::ItemMeta")
    }
}
/// Represents an item that has durability and can take damage.
///
/// This is a representation of an abstract class.
#[repr(C)]
pub struct Damageable<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for Damageable<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for Damageable<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Damageable from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/inventory/meta/Damageable")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Damageable object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> Damageable<'mc> {
    /// Sets the damage
    pub fn set_damage(&self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(I)V");
        let val_1 = jni::objects::JValueGen::Int(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setDamage",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn damage(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getDamage", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }

    pub fn has_damage(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasDamage", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn clone(
        &self,
    ) -> Result<crate::inventory::meta::Damageable<'mc>, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Lorg/bukkit/inventory/meta/Damageable;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "clone", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::meta::Damageable::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    pub fn display_name(&self) -> Result<String, Box<dyn std::error::Error>> {
        let temp_clone = Damageable::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.display_name()
    }
    pub fn as_string(&self) -> Result<String, Box<dyn std::error::Error>> {
        let temp_clone = Damageable::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.as_string()
    }
    pub fn has_display_name(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = Damageable::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.has_display_name()
    }
    pub fn set_display_name(
        &self,
        arg0: impl Into<String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = Damageable::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.set_display_name(arg0)
    }
    pub fn has_localized_name(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = Damageable::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.has_localized_name()
    }
    pub fn localized_name(&self) -> Result<String, Box<dyn std::error::Error>> {
        let temp_clone = Damageable::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.localized_name()
    }
    pub fn set_localized_name(
        &self,
        arg0: impl Into<String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = Damageable::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.set_localized_name(arg0)
    }
    pub fn has_lore(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = Damageable::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.has_lore()
    }
    pub fn lore(&self) -> Result<Option<Vec<String>>, Box<dyn std::error::Error>> {
        let temp_clone = Damageable::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.lore()
    }
    pub fn set_lore(&self, arg0: Vec<impl Into<String>>) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = Damageable::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.set_lore(arg0)
    }
    pub fn has_custom_model_data(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = Damageable::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.has_custom_model_data()
    }
    pub fn custom_model_data(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let temp_clone = Damageable::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.custom_model_data()
    }
    pub fn set_custom_model_data(&self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = Damageable::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.set_custom_model_data(arg0)
    }
    pub fn has_enchants(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "hasEnchants", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    pub fn has_enchant(
        &self,
        arg0: impl Into<crate::enchantments::Enchantment<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = Damageable::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.has_enchant(arg0)
    }
    pub fn get_enchant_level(
        &self,
        arg0: impl Into<crate::enchantments::Enchantment<'mc>>,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        let temp_clone = Damageable::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.get_enchant_level(arg0)
    }
    pub fn enchants(
        &self,
    ) -> Result<blackboxmc_java::util::JavaMap<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/Map;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getEnchants", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::util::JavaMap::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    pub fn add_enchant(
        &self,
        arg0: impl Into<crate::enchantments::Enchantment<'mc>>,
        arg1: i32,
        arg2: bool,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = Damageable::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.add_enchant(arg0, arg1, arg2)
    }
    pub fn remove_enchant(
        &self,
        arg0: impl Into<crate::enchantments::Enchantment<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = Damageable::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.remove_enchant(arg0)
    }
    pub fn has_conflicting_enchant(
        &self,
        arg0: impl Into<crate::enchantments::Enchantment<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = Damageable::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.has_conflicting_enchant(arg0)
    }
    pub fn add_item_flags(
        &self,
        arg0: Vec<crate::inventory::ItemFlag<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("([Lorg/bukkit/inventory/ItemFlag;)V");
        let arr = self.jni_ref().new_object_array(
            arg0.len() as i32,
            "org/bukkit/inventory/ItemFlag",
            jni::objects::JObject::null(),
        );
        let arr = self.jni_ref().translate_error_no_gen(arr)?;
        for i in 0..arg0.len() {
            let val_1 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(arg0.get(i).unwrap().jni_object().clone())
            });
            self.jni_ref()
                .set_object_array_element(&arr, i as i32, val_1.l()?)?;
        }
        let val_1 = jni::objects::JValueGen::Object(arr);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "addItemFlags",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1.l()?)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub fn remove_item_flags(
        &self,
        arg0: Vec<crate::inventory::ItemFlag<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("([Lorg/bukkit/inventory/ItemFlag;)V");
        let arr = self.jni_ref().new_object_array(
            arg0.len() as i32,
            "org/bukkit/inventory/ItemFlag",
            jni::objects::JObject::null(),
        );
        let arr = self.jni_ref().translate_error_no_gen(arr)?;
        for i in 0..arg0.len() {
            let val_1 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(arg0.get(i).unwrap().jni_object().clone())
            });
            self.jni_ref()
                .set_object_array_element(&arr, i as i32, val_1.l()?)?;
        }
        let val_1 = jni::objects::JValueGen::Object(arr);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeItemFlags",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1.l()?)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub fn item_flags(
        &self,
    ) -> Result<blackboxmc_java::util::JavaSet<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/Set;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getItemFlags", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::util::JavaSet::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    pub fn has_item_flag(
        &self,
        arg0: impl Into<crate::inventory::ItemFlag<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = Damageable::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.has_item_flag(arg0)
    }
    pub fn is_unbreakable(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = Damageable::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.is_unbreakable()
    }
    pub fn set_unbreakable(&self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = Damageable::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.set_unbreakable(arg0)
    }
    pub fn has_attribute_modifiers(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "hasAttributeModifiers",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    pub fn attribute_modifiers(
        &self,
    ) -> Result<Option<jni::objects::JObject<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lcom/google/common/collect/Multimap;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getAttributeModifiers",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {
            return Ok(None);
        }
        Ok(Some(res.l()?))
    }
    pub fn add_attribute_modifier(
        &self,
        arg0: impl Into<crate::attribute::Attribute<'mc>>,
        arg1: impl Into<crate::attribute::AttributeModifier<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = Damageable::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.add_attribute_modifier(arg0, arg1)
    }
    pub fn set_attribute_modifiers(
        &self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lcom/google/common/collect/Multimap;)V");
        let val_1 = jni::objects::JValueGen::Object(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setAttributeModifiers",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub fn remove_attribute_modifier_with_equipment_slot(
        &self,
        arg0: impl Into<crate::inventory::EquipmentSlot<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/inventory/EquipmentSlot;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        sig += ")Z";
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeAttributeModifier",
            sig.as_str(),
            args,
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    pub fn remove_attribute_modifier_with_attribute(
        &self,
        arg0: impl Into<crate::attribute::Attribute<'mc>>,
        arg1: std::option::Option<impl Into<crate::attribute::AttributeModifier<'mc>>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/attribute/Attribute;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        if let Some(a) = arg1 {
            sig += "Lorg/bukkit/attribute/AttributeModifier;";
            let val_2 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(a.into().jni_object().clone())
            });
            args.push(val_2);
        }
        sig += ")Z";
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeAttributeModifier",
            sig.as_str(),
            args,
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn custom_tag_container(
        &self,
    ) -> Result<crate::inventory::meta::tags::CustomItemTagContainer<'mc>, Box<dyn std::error::Error>>
    {
        let temp_clone = Damageable::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.custom_tag_container()
    }

    pub fn set_version(&self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = Damageable::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.set_version(arg0)
    }
    // SUPER CLASS: Cloneable
    // SUPER CLASS: ConfigurationSerializable
    pub fn serialize(
        &self,
    ) -> Result<blackboxmc_java::util::JavaMap<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::configuration::serialization::ConfigurationSerializable::from_raw(
            &self.0,
            unsafe { jni::objects::JObject::from_raw(self.1.clone()) },
        )?;
        let real: crate::configuration::serialization::ConfigurationSerializable =
            temp_clone.into();
        real.serialize()
    }
    // SUPER CLASS: PersistentDataHolder
    pub fn persistent_data_container(
        &self,
    ) -> Result<crate::persistence::PersistentDataContainer<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::persistence::PersistentDataHolder::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::persistence::PersistentDataHolder = temp_clone.into();
        real.persistent_data_container()
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::inventory::meta::ItemMeta<'mc>> for Damageable<'mc> {
    fn into(self) -> crate::inventory::meta::ItemMeta<'mc> {
        crate::inventory::meta::ItemMeta::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting Damageable into crate::inventory::meta::ItemMeta")
    }
}
/// Represents a <a href="../../Material.html#FIREWORK_ROCKET"><code>Material.FIREWORK_ROCKET</code></a> and its effects.
///
/// This is a representation of an abstract class.
#[repr(C)]
pub struct FireworkMeta<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for FireworkMeta<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for FireworkMeta<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate FireworkMeta from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/inventory/meta/FireworkMeta")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a FireworkMeta object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> FireworkMeta<'mc> {
    pub fn effects(&self) -> Result<Vec<crate::FireworkEffect<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/List;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getEffects", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let list = blackboxmc_java::util::JavaList::from_raw(&self.jni_ref(), res.l()?)?;
        let iter = list.iterator()?;
        while iter.has_next()? {
            let obj = iter.next()?;
            new_vec.push(crate::FireworkEffect::from_raw(&self.jni_ref(), obj)?);
        }
        Ok(new_vec)
    }

    pub fn power(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getPower", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    /// Sets the approximate power of the firework. Each level of power is half a second of flight time.
    pub fn set_power(&self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(I)V");
        let val_1 = jni::objects::JValueGen::Int(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setPower",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn add_effect(
        &self,
        arg0: impl Into<crate::FireworkEffect<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/FireworkEffect;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "addEffect",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn effects_size(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getEffectsSize", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    /// Remove an effect from this firework.
    pub fn remove_effect(&self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(I)V");
        let val_1 = jni::objects::JValueGen::Int(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeEffect",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn clear_effects(&self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "clearEffects", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn has_effects(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "hasEffects", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn clone(
        &self,
    ) -> Result<crate::inventory::meta::FireworkMeta<'mc>, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Lorg/bukkit/inventory/meta/FireworkMeta;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "clone", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::meta::FireworkMeta::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    pub fn display_name(&self) -> Result<String, Box<dyn std::error::Error>> {
        let temp_clone = FireworkMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.display_name()
    }
    pub fn as_string(&self) -> Result<String, Box<dyn std::error::Error>> {
        let temp_clone = FireworkMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.as_string()
    }
    pub fn has_display_name(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = FireworkMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.has_display_name()
    }
    pub fn set_display_name(
        &self,
        arg0: impl Into<String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = FireworkMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.set_display_name(arg0)
    }
    pub fn has_localized_name(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = FireworkMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.has_localized_name()
    }
    pub fn localized_name(&self) -> Result<String, Box<dyn std::error::Error>> {
        let temp_clone = FireworkMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.localized_name()
    }
    pub fn set_localized_name(
        &self,
        arg0: impl Into<String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = FireworkMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.set_localized_name(arg0)
    }
    pub fn has_lore(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = FireworkMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.has_lore()
    }
    pub fn lore(&self) -> Result<Option<Vec<String>>, Box<dyn std::error::Error>> {
        let temp_clone = FireworkMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.lore()
    }
    pub fn set_lore(&self, arg0: Vec<impl Into<String>>) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = FireworkMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.set_lore(arg0)
    }
    pub fn has_custom_model_data(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = FireworkMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.has_custom_model_data()
    }
    pub fn custom_model_data(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let temp_clone = FireworkMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.custom_model_data()
    }
    pub fn set_custom_model_data(&self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = FireworkMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.set_custom_model_data(arg0)
    }
    pub fn has_enchants(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "hasEnchants", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    pub fn has_enchant(
        &self,
        arg0: impl Into<crate::enchantments::Enchantment<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = FireworkMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.has_enchant(arg0)
    }
    pub fn get_enchant_level(
        &self,
        arg0: impl Into<crate::enchantments::Enchantment<'mc>>,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        let temp_clone = FireworkMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.get_enchant_level(arg0)
    }
    pub fn enchants(
        &self,
    ) -> Result<blackboxmc_java::util::JavaMap<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/Map;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getEnchants", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::util::JavaMap::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    pub fn add_enchant(
        &self,
        arg0: impl Into<crate::enchantments::Enchantment<'mc>>,
        arg1: i32,
        arg2: bool,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = FireworkMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.add_enchant(arg0, arg1, arg2)
    }
    pub fn remove_enchant(
        &self,
        arg0: impl Into<crate::enchantments::Enchantment<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = FireworkMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.remove_enchant(arg0)
    }
    pub fn has_conflicting_enchant(
        &self,
        arg0: impl Into<crate::enchantments::Enchantment<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = FireworkMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.has_conflicting_enchant(arg0)
    }
    pub fn add_item_flags(
        &self,
        arg0: Vec<crate::inventory::ItemFlag<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("([Lorg/bukkit/inventory/ItemFlag;)V");
        let arr = self.jni_ref().new_object_array(
            arg0.len() as i32,
            "org/bukkit/inventory/ItemFlag",
            jni::objects::JObject::null(),
        );
        let arr = self.jni_ref().translate_error_no_gen(arr)?;
        for i in 0..arg0.len() {
            let val_1 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(arg0.get(i).unwrap().jni_object().clone())
            });
            self.jni_ref()
                .set_object_array_element(&arr, i as i32, val_1.l()?)?;
        }
        let val_1 = jni::objects::JValueGen::Object(arr);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "addItemFlags",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1.l()?)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub fn remove_item_flags(
        &self,
        arg0: Vec<crate::inventory::ItemFlag<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("([Lorg/bukkit/inventory/ItemFlag;)V");
        let arr = self.jni_ref().new_object_array(
            arg0.len() as i32,
            "org/bukkit/inventory/ItemFlag",
            jni::objects::JObject::null(),
        );
        let arr = self.jni_ref().translate_error_no_gen(arr)?;
        for i in 0..arg0.len() {
            let val_1 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(arg0.get(i).unwrap().jni_object().clone())
            });
            self.jni_ref()
                .set_object_array_element(&arr, i as i32, val_1.l()?)?;
        }
        let val_1 = jni::objects::JValueGen::Object(arr);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeItemFlags",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1.l()?)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub fn item_flags(
        &self,
    ) -> Result<blackboxmc_java::util::JavaSet<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/Set;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getItemFlags", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::util::JavaSet::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    pub fn has_item_flag(
        &self,
        arg0: impl Into<crate::inventory::ItemFlag<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = FireworkMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.has_item_flag(arg0)
    }
    pub fn is_unbreakable(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = FireworkMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.is_unbreakable()
    }
    pub fn set_unbreakable(&self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = FireworkMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.set_unbreakable(arg0)
    }
    pub fn has_attribute_modifiers(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "hasAttributeModifiers",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    pub fn attribute_modifiers(
        &self,
    ) -> Result<Option<jni::objects::JObject<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lcom/google/common/collect/Multimap;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getAttributeModifiers",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {
            return Ok(None);
        }
        Ok(Some(res.l()?))
    }
    pub fn add_attribute_modifier(
        &self,
        arg0: impl Into<crate::attribute::Attribute<'mc>>,
        arg1: impl Into<crate::attribute::AttributeModifier<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = FireworkMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.add_attribute_modifier(arg0, arg1)
    }
    pub fn set_attribute_modifiers(
        &self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lcom/google/common/collect/Multimap;)V");
        let val_1 = jni::objects::JValueGen::Object(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setAttributeModifiers",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub fn remove_attribute_modifier_with_equipment_slot(
        &self,
        arg0: impl Into<crate::inventory::EquipmentSlot<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/inventory/EquipmentSlot;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        sig += ")Z";
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeAttributeModifier",
            sig.as_str(),
            args,
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    pub fn remove_attribute_modifier_with_attribute(
        &self,
        arg0: impl Into<crate::attribute::Attribute<'mc>>,
        arg1: std::option::Option<impl Into<crate::attribute::AttributeModifier<'mc>>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/attribute/Attribute;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        if let Some(a) = arg1 {
            sig += "Lorg/bukkit/attribute/AttributeModifier;";
            let val_2 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(a.into().jni_object().clone())
            });
            args.push(val_2);
        }
        sig += ")Z";
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeAttributeModifier",
            sig.as_str(),
            args,
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn custom_tag_container(
        &self,
    ) -> Result<crate::inventory::meta::tags::CustomItemTagContainer<'mc>, Box<dyn std::error::Error>>
    {
        let temp_clone = FireworkMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.custom_tag_container()
    }

    pub fn set_version(&self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = FireworkMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.set_version(arg0)
    }
    // SUPER CLASS: Cloneable
    // SUPER CLASS: ConfigurationSerializable
    pub fn serialize(
        &self,
    ) -> Result<blackboxmc_java::util::JavaMap<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::configuration::serialization::ConfigurationSerializable::from_raw(
            &self.0,
            unsafe { jni::objects::JObject::from_raw(self.1.clone()) },
        )?;
        let real: crate::configuration::serialization::ConfigurationSerializable =
            temp_clone.into();
        real.serialize()
    }
    // SUPER CLASS: PersistentDataHolder
    pub fn persistent_data_container(
        &self,
    ) -> Result<crate::persistence::PersistentDataContainer<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::persistence::PersistentDataHolder::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::persistence::PersistentDataHolder = temp_clone.into();
        real.persistent_data_container()
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::inventory::meta::ItemMeta<'mc>> for FireworkMeta<'mc> {
    fn into(self) -> crate::inventory::meta::ItemMeta<'mc> {
        crate::inventory::meta::ItemMeta::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting FireworkMeta into crate::inventory::meta::ItemMeta")
    }
}
/// Represents a potion or item that can have custom effects.
///
/// This is a representation of an abstract class.
#[repr(C)]
pub struct PotionMeta<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for PotionMeta<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for PotionMeta<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate PotionMeta from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/inventory/meta/PotionMeta")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a PotionMeta object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> PotionMeta<'mc> {
    pub fn set_color(
        &self,
        arg0: impl Into<crate::Color<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/Color;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setColor",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn color(&self) -> Result<Option<crate::Color<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/Color;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getColor", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {
            return Ok(None);
        }
        Ok(Some(crate::Color::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })?))
    }

    pub fn has_custom_effects(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "hasCustomEffects",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn custom_effects(
        &self,
    ) -> Result<Vec<crate::potion::PotionEffect<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/List;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getCustomEffects",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let list = blackboxmc_java::util::JavaList::from_raw(&self.jni_ref(), res.l()?)?;
        let iter = list.iterator()?;
        while iter.has_next()? {
            let obj = iter.next()?;
            new_vec.push(crate::potion::PotionEffect::from_raw(&self.jni_ref(), obj)?);
        }
        Ok(new_vec)
    }

    pub fn add_custom_effect(
        &self,
        arg0: impl Into<crate::potion::PotionEffect<'mc>>,
        arg1: bool,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/potion/PotionEffect;Z)Z");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let val_2 = jni::objects::JValueGen::Bool(arg1.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "addCustomEffect",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn remove_custom_effect(
        &self,
        arg0: impl Into<crate::potion::PotionEffectType<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/potion/PotionEffectType;)Z");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeCustomEffect",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn has_custom_effect(
        &self,
        arg0: impl Into<crate::potion::PotionEffectType<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/potion/PotionEffectType;)Z");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "hasCustomEffect",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn clear_custom_effects(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "clearCustomEffects",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn set_base_potion_data(
        &self,
        arg0: impl Into<crate::potion::PotionData<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/potion/PotionData;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setBasePotionData",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn base_potion_data(
        &self,
    ) -> Result<crate::potion::PotionData<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/potion/PotionData;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getBasePotionData",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::potion::PotionData::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn set_main_effect(
        &self,
        arg0: impl Into<crate::potion::PotionEffectType<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/potion/PotionEffectType;)Z");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setMainEffect",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn has_color(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasColor", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn clone(
        &self,
    ) -> Result<crate::inventory::meta::PotionMeta<'mc>, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Lorg/bukkit/inventory/meta/PotionMeta;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "clone", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::meta::PotionMeta::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    pub fn display_name(&self) -> Result<String, Box<dyn std::error::Error>> {
        let temp_clone = PotionMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.display_name()
    }
    pub fn as_string(&self) -> Result<String, Box<dyn std::error::Error>> {
        let temp_clone = PotionMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.as_string()
    }
    pub fn has_display_name(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = PotionMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.has_display_name()
    }
    pub fn set_display_name(
        &self,
        arg0: impl Into<String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = PotionMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.set_display_name(arg0)
    }
    pub fn has_localized_name(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = PotionMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.has_localized_name()
    }
    pub fn localized_name(&self) -> Result<String, Box<dyn std::error::Error>> {
        let temp_clone = PotionMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.localized_name()
    }
    pub fn set_localized_name(
        &self,
        arg0: impl Into<String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = PotionMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.set_localized_name(arg0)
    }
    pub fn has_lore(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = PotionMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.has_lore()
    }
    pub fn lore(&self) -> Result<Option<Vec<String>>, Box<dyn std::error::Error>> {
        let temp_clone = PotionMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.lore()
    }
    pub fn set_lore(&self, arg0: Vec<impl Into<String>>) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = PotionMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.set_lore(arg0)
    }
    pub fn has_custom_model_data(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = PotionMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.has_custom_model_data()
    }
    pub fn custom_model_data(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let temp_clone = PotionMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.custom_model_data()
    }
    pub fn set_custom_model_data(&self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = PotionMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.set_custom_model_data(arg0)
    }
    pub fn has_enchants(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "hasEnchants", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    pub fn has_enchant(
        &self,
        arg0: impl Into<crate::enchantments::Enchantment<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = PotionMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.has_enchant(arg0)
    }
    pub fn get_enchant_level(
        &self,
        arg0: impl Into<crate::enchantments::Enchantment<'mc>>,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        let temp_clone = PotionMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.get_enchant_level(arg0)
    }
    pub fn enchants(
        &self,
    ) -> Result<blackboxmc_java::util::JavaMap<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/Map;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getEnchants", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::util::JavaMap::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    pub fn add_enchant(
        &self,
        arg0: impl Into<crate::enchantments::Enchantment<'mc>>,
        arg1: i32,
        arg2: bool,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = PotionMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.add_enchant(arg0, arg1, arg2)
    }
    pub fn remove_enchant(
        &self,
        arg0: impl Into<crate::enchantments::Enchantment<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = PotionMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.remove_enchant(arg0)
    }
    pub fn has_conflicting_enchant(
        &self,
        arg0: impl Into<crate::enchantments::Enchantment<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = PotionMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.has_conflicting_enchant(arg0)
    }
    pub fn add_item_flags(
        &self,
        arg0: Vec<crate::inventory::ItemFlag<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("([Lorg/bukkit/inventory/ItemFlag;)V");
        let arr = self.jni_ref().new_object_array(
            arg0.len() as i32,
            "org/bukkit/inventory/ItemFlag",
            jni::objects::JObject::null(),
        );
        let arr = self.jni_ref().translate_error_no_gen(arr)?;
        for i in 0..arg0.len() {
            let val_1 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(arg0.get(i).unwrap().jni_object().clone())
            });
            self.jni_ref()
                .set_object_array_element(&arr, i as i32, val_1.l()?)?;
        }
        let val_1 = jni::objects::JValueGen::Object(arr);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "addItemFlags",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1.l()?)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub fn remove_item_flags(
        &self,
        arg0: Vec<crate::inventory::ItemFlag<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("([Lorg/bukkit/inventory/ItemFlag;)V");
        let arr = self.jni_ref().new_object_array(
            arg0.len() as i32,
            "org/bukkit/inventory/ItemFlag",
            jni::objects::JObject::null(),
        );
        let arr = self.jni_ref().translate_error_no_gen(arr)?;
        for i in 0..arg0.len() {
            let val_1 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(arg0.get(i).unwrap().jni_object().clone())
            });
            self.jni_ref()
                .set_object_array_element(&arr, i as i32, val_1.l()?)?;
        }
        let val_1 = jni::objects::JValueGen::Object(arr);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeItemFlags",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1.l()?)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub fn item_flags(
        &self,
    ) -> Result<blackboxmc_java::util::JavaSet<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/Set;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getItemFlags", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::util::JavaSet::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    pub fn has_item_flag(
        &self,
        arg0: impl Into<crate::inventory::ItemFlag<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = PotionMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.has_item_flag(arg0)
    }
    pub fn is_unbreakable(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = PotionMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.is_unbreakable()
    }
    pub fn set_unbreakable(&self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = PotionMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.set_unbreakable(arg0)
    }
    pub fn has_attribute_modifiers(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "hasAttributeModifiers",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    pub fn attribute_modifiers(
        &self,
    ) -> Result<Option<jni::objects::JObject<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lcom/google/common/collect/Multimap;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getAttributeModifiers",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {
            return Ok(None);
        }
        Ok(Some(res.l()?))
    }
    pub fn add_attribute_modifier(
        &self,
        arg0: impl Into<crate::attribute::Attribute<'mc>>,
        arg1: impl Into<crate::attribute::AttributeModifier<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = PotionMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.add_attribute_modifier(arg0, arg1)
    }
    pub fn set_attribute_modifiers(
        &self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lcom/google/common/collect/Multimap;)V");
        let val_1 = jni::objects::JValueGen::Object(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setAttributeModifiers",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub fn remove_attribute_modifier_with_equipment_slot(
        &self,
        arg0: impl Into<crate::inventory::EquipmentSlot<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/inventory/EquipmentSlot;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        sig += ")Z";
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeAttributeModifier",
            sig.as_str(),
            args,
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    pub fn remove_attribute_modifier_with_attribute(
        &self,
        arg0: impl Into<crate::attribute::Attribute<'mc>>,
        arg1: std::option::Option<impl Into<crate::attribute::AttributeModifier<'mc>>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/attribute/Attribute;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        if let Some(a) = arg1 {
            sig += "Lorg/bukkit/attribute/AttributeModifier;";
            let val_2 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(a.into().jni_object().clone())
            });
            args.push(val_2);
        }
        sig += ")Z";
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeAttributeModifier",
            sig.as_str(),
            args,
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn custom_tag_container(
        &self,
    ) -> Result<crate::inventory::meta::tags::CustomItemTagContainer<'mc>, Box<dyn std::error::Error>>
    {
        let temp_clone = PotionMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.custom_tag_container()
    }

    pub fn set_version(&self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = PotionMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.set_version(arg0)
    }
    // SUPER CLASS: Cloneable
    // SUPER CLASS: ConfigurationSerializable
    pub fn serialize(
        &self,
    ) -> Result<blackboxmc_java::util::JavaMap<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::configuration::serialization::ConfigurationSerializable::from_raw(
            &self.0,
            unsafe { jni::objects::JObject::from_raw(self.1.clone()) },
        )?;
        let real: crate::configuration::serialization::ConfigurationSerializable =
            temp_clone.into();
        real.serialize()
    }
    // SUPER CLASS: PersistentDataHolder
    pub fn persistent_data_container(
        &self,
    ) -> Result<crate::persistence::PersistentDataContainer<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::persistence::PersistentDataHolder::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::persistence::PersistentDataHolder = temp_clone.into();
        real.persistent_data_container()
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::inventory::meta::ItemMeta<'mc>> for PotionMeta<'mc> {
    fn into(self) -> crate::inventory::meta::ItemMeta<'mc> {
        crate::inventory::meta::ItemMeta::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting PotionMeta into crate::inventory::meta::ItemMeta")
    }
}

///
/// This is a representation of an abstract class.
#[repr(C)]
pub struct BlockStateMeta<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for BlockStateMeta<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for BlockStateMeta<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate BlockStateMeta from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/inventory/meta/BlockStateMeta")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a BlockStateMeta object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> BlockStateMeta<'mc> {
    pub fn block_state(&self) -> Result<crate::block::BlockState<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/block/BlockState;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getBlockState", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::block::BlockState::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn has_block_state(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "hasBlockState", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn set_block_state(
        &self,
        arg0: impl Into<crate::block::BlockState<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/BlockState;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setBlockState",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub fn display_name(&self) -> Result<String, Box<dyn std::error::Error>> {
        let temp_clone = BlockStateMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.display_name()
    }
    pub fn as_string(&self) -> Result<String, Box<dyn std::error::Error>> {
        let temp_clone = BlockStateMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.as_string()
    }
    pub fn has_display_name(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = BlockStateMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.has_display_name()
    }
    pub fn set_display_name(
        &self,
        arg0: impl Into<String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = BlockStateMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.set_display_name(arg0)
    }
    pub fn has_localized_name(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = BlockStateMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.has_localized_name()
    }
    pub fn localized_name(&self) -> Result<String, Box<dyn std::error::Error>> {
        let temp_clone = BlockStateMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.localized_name()
    }
    pub fn set_localized_name(
        &self,
        arg0: impl Into<String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = BlockStateMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.set_localized_name(arg0)
    }
    pub fn has_lore(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = BlockStateMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.has_lore()
    }
    pub fn lore(&self) -> Result<Option<Vec<String>>, Box<dyn std::error::Error>> {
        let temp_clone = BlockStateMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.lore()
    }
    pub fn set_lore(&self, arg0: Vec<impl Into<String>>) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = BlockStateMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.set_lore(arg0)
    }
    pub fn has_custom_model_data(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = BlockStateMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.has_custom_model_data()
    }
    pub fn custom_model_data(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let temp_clone = BlockStateMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.custom_model_data()
    }
    pub fn set_custom_model_data(&self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = BlockStateMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.set_custom_model_data(arg0)
    }
    pub fn has_enchants(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "hasEnchants", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    pub fn has_enchant(
        &self,
        arg0: impl Into<crate::enchantments::Enchantment<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = BlockStateMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.has_enchant(arg0)
    }
    pub fn get_enchant_level(
        &self,
        arg0: impl Into<crate::enchantments::Enchantment<'mc>>,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        let temp_clone = BlockStateMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.get_enchant_level(arg0)
    }
    pub fn enchants(
        &self,
    ) -> Result<blackboxmc_java::util::JavaMap<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/Map;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getEnchants", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::util::JavaMap::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    pub fn add_enchant(
        &self,
        arg0: impl Into<crate::enchantments::Enchantment<'mc>>,
        arg1: i32,
        arg2: bool,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = BlockStateMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.add_enchant(arg0, arg1, arg2)
    }
    pub fn remove_enchant(
        &self,
        arg0: impl Into<crate::enchantments::Enchantment<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = BlockStateMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.remove_enchant(arg0)
    }
    pub fn has_conflicting_enchant(
        &self,
        arg0: impl Into<crate::enchantments::Enchantment<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = BlockStateMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.has_conflicting_enchant(arg0)
    }
    pub fn add_item_flags(
        &self,
        arg0: Vec<crate::inventory::ItemFlag<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("([Lorg/bukkit/inventory/ItemFlag;)V");
        let arr = self.jni_ref().new_object_array(
            arg0.len() as i32,
            "org/bukkit/inventory/ItemFlag",
            jni::objects::JObject::null(),
        );
        let arr = self.jni_ref().translate_error_no_gen(arr)?;
        for i in 0..arg0.len() {
            let val_1 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(arg0.get(i).unwrap().jni_object().clone())
            });
            self.jni_ref()
                .set_object_array_element(&arr, i as i32, val_1.l()?)?;
        }
        let val_1 = jni::objects::JValueGen::Object(arr);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "addItemFlags",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1.l()?)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub fn remove_item_flags(
        &self,
        arg0: Vec<crate::inventory::ItemFlag<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("([Lorg/bukkit/inventory/ItemFlag;)V");
        let arr = self.jni_ref().new_object_array(
            arg0.len() as i32,
            "org/bukkit/inventory/ItemFlag",
            jni::objects::JObject::null(),
        );
        let arr = self.jni_ref().translate_error_no_gen(arr)?;
        for i in 0..arg0.len() {
            let val_1 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(arg0.get(i).unwrap().jni_object().clone())
            });
            self.jni_ref()
                .set_object_array_element(&arr, i as i32, val_1.l()?)?;
        }
        let val_1 = jni::objects::JValueGen::Object(arr);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeItemFlags",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1.l()?)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub fn item_flags(
        &self,
    ) -> Result<blackboxmc_java::util::JavaSet<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/Set;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getItemFlags", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::util::JavaSet::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    pub fn has_item_flag(
        &self,
        arg0: impl Into<crate::inventory::ItemFlag<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = BlockStateMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.has_item_flag(arg0)
    }
    pub fn is_unbreakable(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = BlockStateMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.is_unbreakable()
    }
    pub fn set_unbreakable(&self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = BlockStateMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.set_unbreakable(arg0)
    }
    pub fn has_attribute_modifiers(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "hasAttributeModifiers",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    pub fn attribute_modifiers(
        &self,
    ) -> Result<Option<jni::objects::JObject<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lcom/google/common/collect/Multimap;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getAttributeModifiers",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {
            return Ok(None);
        }
        Ok(Some(res.l()?))
    }
    pub fn add_attribute_modifier(
        &self,
        arg0: impl Into<crate::attribute::Attribute<'mc>>,
        arg1: impl Into<crate::attribute::AttributeModifier<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = BlockStateMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.add_attribute_modifier(arg0, arg1)
    }
    pub fn set_attribute_modifiers(
        &self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lcom/google/common/collect/Multimap;)V");
        let val_1 = jni::objects::JValueGen::Object(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setAttributeModifiers",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub fn remove_attribute_modifier_with_equipment_slot(
        &self,
        arg0: impl Into<crate::inventory::EquipmentSlot<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/inventory/EquipmentSlot;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        sig += ")Z";
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeAttributeModifier",
            sig.as_str(),
            args,
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    pub fn remove_attribute_modifier_with_attribute(
        &self,
        arg0: impl Into<crate::attribute::Attribute<'mc>>,
        arg1: std::option::Option<impl Into<crate::attribute::AttributeModifier<'mc>>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/attribute/Attribute;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        if let Some(a) = arg1 {
            sig += "Lorg/bukkit/attribute/AttributeModifier;";
            let val_2 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(a.into().jni_object().clone())
            });
            args.push(val_2);
        }
        sig += ")Z";
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeAttributeModifier",
            sig.as_str(),
            args,
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn custom_tag_container(
        &self,
    ) -> Result<crate::inventory::meta::tags::CustomItemTagContainer<'mc>, Box<dyn std::error::Error>>
    {
        let temp_clone = BlockStateMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.custom_tag_container()
    }

    pub fn set_version(&self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = BlockStateMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.set_version(arg0)
    }
    pub fn clone(&self) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = BlockStateMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.clone()
    }
    // SUPER CLASS: Cloneable
    // SUPER CLASS: ConfigurationSerializable
    pub fn serialize(
        &self,
    ) -> Result<blackboxmc_java::util::JavaMap<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::configuration::serialization::ConfigurationSerializable::from_raw(
            &self.0,
            unsafe { jni::objects::JObject::from_raw(self.1.clone()) },
        )?;
        let real: crate::configuration::serialization::ConfigurationSerializable =
            temp_clone.into();
        real.serialize()
    }
    // SUPER CLASS: PersistentDataHolder
    pub fn persistent_data_container(
        &self,
    ) -> Result<crate::persistence::PersistentDataContainer<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::persistence::PersistentDataHolder::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::persistence::PersistentDataHolder = temp_clone.into();
        real.persistent_data_container()
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::inventory::meta::ItemMeta<'mc>> for BlockStateMeta<'mc> {
    fn into(self) -> crate::inventory::meta::ItemMeta<'mc> {
        crate::inventory::meta::ItemMeta::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting BlockStateMeta into crate::inventory::meta::ItemMeta")
    }
}
pub enum Generation<'mc> {
    Original { inner: GenerationStruct<'mc> },
    CopyOfOriginal { inner: GenerationStruct<'mc> },
    CopyOfCopy { inner: GenerationStruct<'mc> },
    Tattered { inner: GenerationStruct<'mc> },
}
impl<'mc> std::fmt::Display for Generation<'mc> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Generation::Original { .. } => f.write_str("ORIGINAL"),
            Generation::CopyOfOriginal { .. } => f.write_str("COPY_OF_ORIGINAL"),
            Generation::CopyOfCopy { .. } => f.write_str("COPY_OF_COPY"),
            Generation::Tattered { .. } => f.write_str("TATTERED"),
        }
    }
}

impl<'mc> Generation<'mc> {
    pub fn value_of(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<String>,
    ) -> Result<Generation<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(env.new_string(arg0.into())?);
        let cls = env.find_class("org/bukkit/inventory/meta/Generation");
        let cls = env.translate_error_with_class(cls)?;
        let res = env.call_static_method(
            cls,
            "valueOf",
            "(Ljava/lang/String;)Lorg/bukkit/inventory/meta/Generation;",
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
            "ORIGINAL" => Ok(Generation::Original {
                inner: GenerationStruct::from_raw(env, obj)?,
            }),
            "COPY_OF_ORIGINAL" => Ok(Generation::CopyOfOriginal {
                inner: GenerationStruct::from_raw(env, obj)?,
            }),
            "COPY_OF_COPY" => Ok(Generation::CopyOfCopy {
                inner: GenerationStruct::from_raw(env, obj)?,
            }),
            "TATTERED" => Ok(Generation::Tattered {
                inner: GenerationStruct::from_raw(env, obj)?,
            }),

            _ => Err(eyre::eyre!("String gaven for variant was invalid").into()),
        }
    }
}

#[repr(C)]
pub struct GenerationStruct<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for Generation<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        match self {
            Self::Original { inner } => inner.0.clone(),
            Self::CopyOfOriginal { inner } => inner.0.clone(),
            Self::CopyOfCopy { inner } => inner.0.clone(),
            Self::Tattered { inner } => inner.0.clone(),
        }
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        match self {
            Self::Original { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::CopyOfOriginal { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::CopyOfCopy { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::Tattered { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
        }
    }
}
impl<'mc> JNIInstantiatable<'mc> for Generation<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Generation from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/inventory/meta/Generation")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Generation object, got {}",
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
                "ORIGINAL" => Ok(Generation::Original {
                    inner: GenerationStruct::from_raw(env, obj)?,
                }),
                "COPY_OF_ORIGINAL" => Ok(Generation::CopyOfOriginal {
                    inner: GenerationStruct::from_raw(env, obj)?,
                }),
                "COPY_OF_COPY" => Ok(Generation::CopyOfCopy {
                    inner: GenerationStruct::from_raw(env, obj)?,
                }),
                "TATTERED" => Ok(Generation::Tattered {
                    inner: GenerationStruct::from_raw(env, obj)?,
                }),
                _ => Err(eyre::eyre!("String gaven for variant was invalid").into()),
            }
        }
    }
}

impl<'mc> JNIRaw<'mc> for GenerationStruct<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for GenerationStruct<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate GenerationStruct from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/inventory/meta/Generation")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a GenerationStruct object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> GenerationStruct<'mc> {
    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
/// EnchantmentMeta is specific to items that can <i>store</i> enchantments, as opposed to being enchanted. <a href="../../Material.html#ENCHANTED_BOOK"><code>Material.ENCHANTED_BOOK</code></a> is an example of an item with enchantment storage.
///
/// This is a representation of an abstract class.
#[repr(C)]
pub struct EnchantmentStorageMeta<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for EnchantmentStorageMeta<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for EnchantmentStorageMeta<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate EnchantmentStorageMeta from null object."
            )
            .into());
        }
        let (valid, name) =
            env.validate_name(&obj, "org/bukkit/inventory/meta/EnchantmentStorageMeta")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a EnchantmentStorageMeta object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> EnchantmentStorageMeta<'mc> {
    pub fn has_stored_enchant(
        &self,
        arg0: impl Into<crate::enchantments::Enchantment<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/enchantments/Enchantment;)Z");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "hasStoredEnchant",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn get_stored_enchant_level(
        &self,
        arg0: impl Into<crate::enchantments::Enchantment<'mc>>,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/enchantments/Enchantment;)I");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getStoredEnchantLevel",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }

    pub fn stored_enchants(
        &self,
    ) -> Result<blackboxmc_java::util::JavaMap<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/Map;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getStoredEnchants",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::util::JavaMap::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn add_stored_enchant(
        &self,
        arg0: impl Into<crate::enchantments::Enchantment<'mc>>,
        arg1: i32,
        arg2: bool,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/enchantments/Enchantment;IZ)Z");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let val_2 = jni::objects::JValueGen::Int(arg1);
        let val_3 = jni::objects::JValueGen::Bool(arg2.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "addStoredEnchant",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
                jni::objects::JValueGen::from(val_3),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn remove_stored_enchant(
        &self,
        arg0: impl Into<crate::enchantments::Enchantment<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/enchantments/Enchantment;)Z");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeStoredEnchant",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn has_conflicting_stored_enchant(
        &self,
        arg0: impl Into<crate::enchantments::Enchantment<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/enchantments/Enchantment;)Z");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "hasConflictingStoredEnchant",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn has_stored_enchants(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "hasStoredEnchants",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn clone(
        &self,
    ) -> Result<crate::inventory::meta::EnchantmentStorageMeta<'mc>, Box<dyn std::error::Error>>
    {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Lorg/bukkit/inventory/meta/EnchantmentStorageMeta;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "clone", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::meta::EnchantmentStorageMeta::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    pub fn display_name(&self) -> Result<String, Box<dyn std::error::Error>> {
        let temp_clone = EnchantmentStorageMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.display_name()
    }
    pub fn as_string(&self) -> Result<String, Box<dyn std::error::Error>> {
        let temp_clone = EnchantmentStorageMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.as_string()
    }
    pub fn has_display_name(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = EnchantmentStorageMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.has_display_name()
    }
    pub fn set_display_name(
        &self,
        arg0: impl Into<String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = EnchantmentStorageMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.set_display_name(arg0)
    }
    pub fn has_localized_name(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = EnchantmentStorageMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.has_localized_name()
    }
    pub fn localized_name(&self) -> Result<String, Box<dyn std::error::Error>> {
        let temp_clone = EnchantmentStorageMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.localized_name()
    }
    pub fn set_localized_name(
        &self,
        arg0: impl Into<String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = EnchantmentStorageMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.set_localized_name(arg0)
    }
    pub fn has_lore(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = EnchantmentStorageMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.has_lore()
    }
    pub fn lore(&self) -> Result<Option<Vec<String>>, Box<dyn std::error::Error>> {
        let temp_clone = EnchantmentStorageMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.lore()
    }
    pub fn set_lore(&self, arg0: Vec<impl Into<String>>) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = EnchantmentStorageMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.set_lore(arg0)
    }
    pub fn has_custom_model_data(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = EnchantmentStorageMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.has_custom_model_data()
    }
    pub fn custom_model_data(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let temp_clone = EnchantmentStorageMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.custom_model_data()
    }
    pub fn set_custom_model_data(&self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = EnchantmentStorageMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.set_custom_model_data(arg0)
    }
    pub fn has_enchants(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "hasEnchants", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    pub fn has_enchant(
        &self,
        arg0: impl Into<crate::enchantments::Enchantment<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = EnchantmentStorageMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.has_enchant(arg0)
    }
    pub fn get_enchant_level(
        &self,
        arg0: impl Into<crate::enchantments::Enchantment<'mc>>,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        let temp_clone = EnchantmentStorageMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.get_enchant_level(arg0)
    }
    pub fn enchants(
        &self,
    ) -> Result<blackboxmc_java::util::JavaMap<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/Map;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getEnchants", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::util::JavaMap::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    pub fn add_enchant(
        &self,
        arg0: impl Into<crate::enchantments::Enchantment<'mc>>,
        arg1: i32,
        arg2: bool,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = EnchantmentStorageMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.add_enchant(arg0, arg1, arg2)
    }
    pub fn remove_enchant(
        &self,
        arg0: impl Into<crate::enchantments::Enchantment<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = EnchantmentStorageMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.remove_enchant(arg0)
    }
    pub fn has_conflicting_enchant(
        &self,
        arg0: impl Into<crate::enchantments::Enchantment<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = EnchantmentStorageMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.has_conflicting_enchant(arg0)
    }
    pub fn add_item_flags(
        &self,
        arg0: Vec<crate::inventory::ItemFlag<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("([Lorg/bukkit/inventory/ItemFlag;)V");
        let arr = self.jni_ref().new_object_array(
            arg0.len() as i32,
            "org/bukkit/inventory/ItemFlag",
            jni::objects::JObject::null(),
        );
        let arr = self.jni_ref().translate_error_no_gen(arr)?;
        for i in 0..arg0.len() {
            let val_1 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(arg0.get(i).unwrap().jni_object().clone())
            });
            self.jni_ref()
                .set_object_array_element(&arr, i as i32, val_1.l()?)?;
        }
        let val_1 = jni::objects::JValueGen::Object(arr);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "addItemFlags",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1.l()?)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub fn remove_item_flags(
        &self,
        arg0: Vec<crate::inventory::ItemFlag<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("([Lorg/bukkit/inventory/ItemFlag;)V");
        let arr = self.jni_ref().new_object_array(
            arg0.len() as i32,
            "org/bukkit/inventory/ItemFlag",
            jni::objects::JObject::null(),
        );
        let arr = self.jni_ref().translate_error_no_gen(arr)?;
        for i in 0..arg0.len() {
            let val_1 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(arg0.get(i).unwrap().jni_object().clone())
            });
            self.jni_ref()
                .set_object_array_element(&arr, i as i32, val_1.l()?)?;
        }
        let val_1 = jni::objects::JValueGen::Object(arr);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeItemFlags",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1.l()?)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub fn item_flags(
        &self,
    ) -> Result<blackboxmc_java::util::JavaSet<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/Set;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getItemFlags", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::util::JavaSet::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    pub fn has_item_flag(
        &self,
        arg0: impl Into<crate::inventory::ItemFlag<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = EnchantmentStorageMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.has_item_flag(arg0)
    }
    pub fn is_unbreakable(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = EnchantmentStorageMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.is_unbreakable()
    }
    pub fn set_unbreakable(&self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = EnchantmentStorageMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.set_unbreakable(arg0)
    }
    pub fn has_attribute_modifiers(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "hasAttributeModifiers",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    pub fn attribute_modifiers(
        &self,
    ) -> Result<Option<jni::objects::JObject<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lcom/google/common/collect/Multimap;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getAttributeModifiers",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {
            return Ok(None);
        }
        Ok(Some(res.l()?))
    }
    pub fn add_attribute_modifier(
        &self,
        arg0: impl Into<crate::attribute::Attribute<'mc>>,
        arg1: impl Into<crate::attribute::AttributeModifier<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = EnchantmentStorageMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.add_attribute_modifier(arg0, arg1)
    }
    pub fn set_attribute_modifiers(
        &self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lcom/google/common/collect/Multimap;)V");
        let val_1 = jni::objects::JValueGen::Object(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setAttributeModifiers",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub fn remove_attribute_modifier_with_equipment_slot(
        &self,
        arg0: impl Into<crate::inventory::EquipmentSlot<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/inventory/EquipmentSlot;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        sig += ")Z";
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeAttributeModifier",
            sig.as_str(),
            args,
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    pub fn remove_attribute_modifier_with_attribute(
        &self,
        arg0: impl Into<crate::attribute::Attribute<'mc>>,
        arg1: std::option::Option<impl Into<crate::attribute::AttributeModifier<'mc>>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/attribute/Attribute;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        if let Some(a) = arg1 {
            sig += "Lorg/bukkit/attribute/AttributeModifier;";
            let val_2 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(a.into().jni_object().clone())
            });
            args.push(val_2);
        }
        sig += ")Z";
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeAttributeModifier",
            sig.as_str(),
            args,
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn custom_tag_container(
        &self,
    ) -> Result<crate::inventory::meta::tags::CustomItemTagContainer<'mc>, Box<dyn std::error::Error>>
    {
        let temp_clone = EnchantmentStorageMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.custom_tag_container()
    }

    pub fn set_version(&self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = EnchantmentStorageMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.set_version(arg0)
    }
    // SUPER CLASS: Cloneable
    // SUPER CLASS: ConfigurationSerializable
    pub fn serialize(
        &self,
    ) -> Result<blackboxmc_java::util::JavaMap<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::configuration::serialization::ConfigurationSerializable::from_raw(
            &self.0,
            unsafe { jni::objects::JObject::from_raw(self.1.clone()) },
        )?;
        let real: crate::configuration::serialization::ConfigurationSerializable =
            temp_clone.into();
        real.serialize()
    }
    // SUPER CLASS: PersistentDataHolder
    pub fn persistent_data_container(
        &self,
    ) -> Result<crate::persistence::PersistentDataContainer<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::persistence::PersistentDataHolder::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::persistence::PersistentDataHolder = temp_clone.into();
        real.persistent_data_container()
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::inventory::meta::ItemMeta<'mc>> for EnchantmentStorageMeta<'mc> {
    fn into(self) -> crate::inventory::meta::ItemMeta<'mc> {
        crate::inventory::meta::ItemMeta::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting EnchantmentStorageMeta into crate::inventory::meta::ItemMeta")
    }
}

///
/// This is a representation of an abstract class.
#[repr(C)]
pub struct KnowledgeBookMeta<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for KnowledgeBookMeta<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for KnowledgeBookMeta<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate KnowledgeBookMeta from null object.").into(),
            );
        }
        let (valid, name) =
            env.validate_name(&obj, "org/bukkit/inventory/meta/KnowledgeBookMeta")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a KnowledgeBookMeta object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> KnowledgeBookMeta<'mc> {
    pub fn recipes(&self) -> Result<Vec<crate::NamespacedKey<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/List;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getRecipes", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let list = blackboxmc_java::util::JavaList::from_raw(&self.jni_ref(), res.l()?)?;
        let iter = list.iterator()?;
        while iter.has_next()? {
            let obj = iter.next()?;
            new_vec.push(crate::NamespacedKey::from_raw(&self.jni_ref(), obj)?);
        }
        Ok(new_vec)
    }

    pub fn set_recipes(
        &self,
        arg0: Vec<impl Into<crate::NamespacedKey<'mc>>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/util/List;)V");
        let raw_val_1 = self
            .jni_ref()
            .new_object("java/util/ArrayList", "()V", vec![])?;
        for v in arg0 {
            let map_val_0 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(v.into().jni_object().clone())
            });
            self.jni_ref().call_method(
                &raw_val_1,
                "add",
                "(Ljava/lang/Object;)Z",
                vec![jni::objects::JValueGen::from(map_val_0)],
            )?;
        }
        let val_1 = jni::objects::JValueGen::Object(raw_val_1);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setRecipes",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn add_recipe(
        &self,
        arg0: Vec<crate::NamespacedKey<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("([Lorg/bukkit/NamespacedKey;)V");
        let arr = self.jni_ref().new_object_array(
            arg0.len() as i32,
            "org/bukkit/NamespacedKey",
            jni::objects::JObject::null(),
        );
        let arr = self.jni_ref().translate_error_no_gen(arr)?;
        for i in 0..arg0.len() {
            let val_1 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(arg0.get(i).unwrap().jni_object().clone())
            });
            self.jni_ref()
                .set_object_array_element(&arr, i as i32, val_1.l()?)?;
        }
        let val_1 = jni::objects::JValueGen::Object(arr);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "addRecipe",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1.l()?)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn has_recipes(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "hasRecipes", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn clone(&self) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Ljava/lang/Object;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "clone", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l()?)
    }
    pub fn display_name(&self) -> Result<String, Box<dyn std::error::Error>> {
        let temp_clone = KnowledgeBookMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.display_name()
    }
    pub fn as_string(&self) -> Result<String, Box<dyn std::error::Error>> {
        let temp_clone = KnowledgeBookMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.as_string()
    }
    pub fn has_display_name(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = KnowledgeBookMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.has_display_name()
    }
    pub fn set_display_name(
        &self,
        arg0: impl Into<String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = KnowledgeBookMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.set_display_name(arg0)
    }
    pub fn has_localized_name(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = KnowledgeBookMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.has_localized_name()
    }
    pub fn localized_name(&self) -> Result<String, Box<dyn std::error::Error>> {
        let temp_clone = KnowledgeBookMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.localized_name()
    }
    pub fn set_localized_name(
        &self,
        arg0: impl Into<String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = KnowledgeBookMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.set_localized_name(arg0)
    }
    pub fn has_lore(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = KnowledgeBookMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.has_lore()
    }
    pub fn lore(&self) -> Result<Option<Vec<String>>, Box<dyn std::error::Error>> {
        let temp_clone = KnowledgeBookMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.lore()
    }
    pub fn set_lore(&self, arg0: Vec<impl Into<String>>) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = KnowledgeBookMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.set_lore(arg0)
    }
    pub fn has_custom_model_data(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = KnowledgeBookMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.has_custom_model_data()
    }
    pub fn custom_model_data(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let temp_clone = KnowledgeBookMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.custom_model_data()
    }
    pub fn set_custom_model_data(&self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = KnowledgeBookMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.set_custom_model_data(arg0)
    }
    pub fn has_enchants(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "hasEnchants", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    pub fn has_enchant(
        &self,
        arg0: impl Into<crate::enchantments::Enchantment<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = KnowledgeBookMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.has_enchant(arg0)
    }
    pub fn get_enchant_level(
        &self,
        arg0: impl Into<crate::enchantments::Enchantment<'mc>>,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        let temp_clone = KnowledgeBookMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.get_enchant_level(arg0)
    }
    pub fn enchants(
        &self,
    ) -> Result<blackboxmc_java::util::JavaMap<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/Map;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getEnchants", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::util::JavaMap::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    pub fn add_enchant(
        &self,
        arg0: impl Into<crate::enchantments::Enchantment<'mc>>,
        arg1: i32,
        arg2: bool,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = KnowledgeBookMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.add_enchant(arg0, arg1, arg2)
    }
    pub fn remove_enchant(
        &self,
        arg0: impl Into<crate::enchantments::Enchantment<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = KnowledgeBookMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.remove_enchant(arg0)
    }
    pub fn has_conflicting_enchant(
        &self,
        arg0: impl Into<crate::enchantments::Enchantment<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = KnowledgeBookMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.has_conflicting_enchant(arg0)
    }
    pub fn add_item_flags(
        &self,
        arg0: Vec<crate::inventory::ItemFlag<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("([Lorg/bukkit/inventory/ItemFlag;)V");
        let arr = self.jni_ref().new_object_array(
            arg0.len() as i32,
            "org/bukkit/inventory/ItemFlag",
            jni::objects::JObject::null(),
        );
        let arr = self.jni_ref().translate_error_no_gen(arr)?;
        for i in 0..arg0.len() {
            let val_1 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(arg0.get(i).unwrap().jni_object().clone())
            });
            self.jni_ref()
                .set_object_array_element(&arr, i as i32, val_1.l()?)?;
        }
        let val_1 = jni::objects::JValueGen::Object(arr);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "addItemFlags",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1.l()?)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub fn remove_item_flags(
        &self,
        arg0: Vec<crate::inventory::ItemFlag<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("([Lorg/bukkit/inventory/ItemFlag;)V");
        let arr = self.jni_ref().new_object_array(
            arg0.len() as i32,
            "org/bukkit/inventory/ItemFlag",
            jni::objects::JObject::null(),
        );
        let arr = self.jni_ref().translate_error_no_gen(arr)?;
        for i in 0..arg0.len() {
            let val_1 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(arg0.get(i).unwrap().jni_object().clone())
            });
            self.jni_ref()
                .set_object_array_element(&arr, i as i32, val_1.l()?)?;
        }
        let val_1 = jni::objects::JValueGen::Object(arr);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeItemFlags",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1.l()?)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub fn item_flags(
        &self,
    ) -> Result<blackboxmc_java::util::JavaSet<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/Set;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getItemFlags", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::util::JavaSet::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    pub fn has_item_flag(
        &self,
        arg0: impl Into<crate::inventory::ItemFlag<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = KnowledgeBookMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.has_item_flag(arg0)
    }
    pub fn is_unbreakable(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = KnowledgeBookMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.is_unbreakable()
    }
    pub fn set_unbreakable(&self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = KnowledgeBookMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.set_unbreakable(arg0)
    }
    pub fn has_attribute_modifiers(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "hasAttributeModifiers",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    pub fn attribute_modifiers(
        &self,
    ) -> Result<Option<jni::objects::JObject<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lcom/google/common/collect/Multimap;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getAttributeModifiers",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {
            return Ok(None);
        }
        Ok(Some(res.l()?))
    }
    pub fn add_attribute_modifier(
        &self,
        arg0: impl Into<crate::attribute::Attribute<'mc>>,
        arg1: impl Into<crate::attribute::AttributeModifier<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = KnowledgeBookMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.add_attribute_modifier(arg0, arg1)
    }
    pub fn set_attribute_modifiers(
        &self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lcom/google/common/collect/Multimap;)V");
        let val_1 = jni::objects::JValueGen::Object(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setAttributeModifiers",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub fn remove_attribute_modifier_with_equipment_slot(
        &self,
        arg0: impl Into<crate::inventory::EquipmentSlot<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/inventory/EquipmentSlot;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        sig += ")Z";
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeAttributeModifier",
            sig.as_str(),
            args,
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    pub fn remove_attribute_modifier_with_attribute(
        &self,
        arg0: impl Into<crate::attribute::Attribute<'mc>>,
        arg1: std::option::Option<impl Into<crate::attribute::AttributeModifier<'mc>>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/attribute/Attribute;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        if let Some(a) = arg1 {
            sig += "Lorg/bukkit/attribute/AttributeModifier;";
            let val_2 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(a.into().jni_object().clone())
            });
            args.push(val_2);
        }
        sig += ")Z";
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeAttributeModifier",
            sig.as_str(),
            args,
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn custom_tag_container(
        &self,
    ) -> Result<crate::inventory::meta::tags::CustomItemTagContainer<'mc>, Box<dyn std::error::Error>>
    {
        let temp_clone = KnowledgeBookMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.custom_tag_container()
    }

    pub fn set_version(&self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = KnowledgeBookMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.set_version(arg0)
    }
    // SUPER CLASS: Cloneable
    // SUPER CLASS: ConfigurationSerializable
    pub fn serialize(
        &self,
    ) -> Result<blackboxmc_java::util::JavaMap<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::configuration::serialization::ConfigurationSerializable::from_raw(
            &self.0,
            unsafe { jni::objects::JObject::from_raw(self.1.clone()) },
        )?;
        let real: crate::configuration::serialization::ConfigurationSerializable =
            temp_clone.into();
        real.serialize()
    }
    // SUPER CLASS: PersistentDataHolder
    pub fn persistent_data_container(
        &self,
    ) -> Result<crate::persistence::PersistentDataContainer<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::persistence::PersistentDataHolder::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::persistence::PersistentDataHolder = temp_clone.into();
        real.persistent_data_container()
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::inventory::meta::ItemMeta<'mc>> for KnowledgeBookMeta<'mc> {
    fn into(self) -> crate::inventory::meta::ItemMeta<'mc> {
        crate::inventory::meta::ItemMeta::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting KnowledgeBookMeta into crate::inventory::meta::ItemMeta")
    }
}
/// Represents a map that can be scalable.
///
/// This is a representation of an abstract class.
#[repr(C)]
pub struct MapMeta<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for MapMeta<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for MapMeta<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate MapMeta from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/inventory/meta/MapMeta")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a MapMeta object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> MapMeta<'mc> {
    pub fn set_color(
        &self,
        arg0: impl Into<crate::Color<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/Color;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setColor",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn color(&self) -> Result<Option<crate::Color<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/Color;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getColor", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {
            return Ok(None);
        }
        Ok(Some(crate::Color::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })?))
    }

    pub fn has_color(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasColor", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn map_view(&self) -> Result<crate::map::MapView<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/map/MapView;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getMapView", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::map::MapView::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    #[deprecated]

    pub fn has_map_id(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasMapId", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    #[deprecated]

    pub fn map_id(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getMapId", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    #[deprecated = "These methods are poor API: They rely on the caller to pass in an only an integer property, and have poorly defined implementation behavior if that integer is not a valid map (the current implementation for example will generate a new map with a different ID). The xxxMapView family of methods should be used instead. "]
    /// Sets the map ID. This is used to determine what map is displayed.
    pub fn set_map_id(&self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(I)V");
        let val_1 = jni::objects::JValueGen::Int(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setMapId",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn has_map_view(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "hasMapView", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn set_map_view(
        &self,
        arg0: impl Into<crate::map::MapView<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/map/MapView;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setMapView",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn is_scaling(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isScaling", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Sets if this map is scaling or not.
    pub fn set_scaling(&self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        let val_1 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setScaling",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    #[deprecated]

    pub fn has_location_name(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "hasLocationName", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    #[deprecated]

    pub fn location_name(&self) -> Result<Option<String>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/String;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getLocationName", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {
            return Ok(None);
        }
        Ok(Some(
            self.jni_ref()
                .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
                .to_string_lossy()
                .to_string(),
        ))
    }

    pub fn set_location_name(
        &self,
        arg0: impl Into<String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/String;)V");
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(arg0.into())?,
        ));
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setLocationName",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn clone(
        &self,
    ) -> Result<crate::inventory::meta::MapMeta<'mc>, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Lorg/bukkit/inventory/meta/MapMeta;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "clone", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::meta::MapMeta::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    pub fn display_name(&self) -> Result<String, Box<dyn std::error::Error>> {
        let temp_clone = MapMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.display_name()
    }
    pub fn as_string(&self) -> Result<String, Box<dyn std::error::Error>> {
        let temp_clone = MapMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.as_string()
    }
    pub fn has_display_name(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = MapMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.has_display_name()
    }
    pub fn set_display_name(
        &self,
        arg0: impl Into<String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = MapMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.set_display_name(arg0)
    }
    pub fn has_localized_name(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = MapMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.has_localized_name()
    }
    pub fn localized_name(&self) -> Result<String, Box<dyn std::error::Error>> {
        let temp_clone = MapMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.localized_name()
    }
    pub fn set_localized_name(
        &self,
        arg0: impl Into<String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = MapMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.set_localized_name(arg0)
    }
    pub fn has_lore(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = MapMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.has_lore()
    }
    pub fn lore(&self) -> Result<Option<Vec<String>>, Box<dyn std::error::Error>> {
        let temp_clone = MapMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.lore()
    }
    pub fn set_lore(&self, arg0: Vec<impl Into<String>>) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = MapMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.set_lore(arg0)
    }
    pub fn has_custom_model_data(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = MapMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.has_custom_model_data()
    }
    pub fn custom_model_data(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let temp_clone = MapMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.custom_model_data()
    }
    pub fn set_custom_model_data(&self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = MapMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.set_custom_model_data(arg0)
    }
    pub fn has_enchants(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "hasEnchants", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    pub fn has_enchant(
        &self,
        arg0: impl Into<crate::enchantments::Enchantment<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = MapMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.has_enchant(arg0)
    }
    pub fn get_enchant_level(
        &self,
        arg0: impl Into<crate::enchantments::Enchantment<'mc>>,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        let temp_clone = MapMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.get_enchant_level(arg0)
    }
    pub fn enchants(
        &self,
    ) -> Result<blackboxmc_java::util::JavaMap<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/Map;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getEnchants", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::util::JavaMap::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    pub fn add_enchant(
        &self,
        arg0: impl Into<crate::enchantments::Enchantment<'mc>>,
        arg1: i32,
        arg2: bool,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = MapMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.add_enchant(arg0, arg1, arg2)
    }
    pub fn remove_enchant(
        &self,
        arg0: impl Into<crate::enchantments::Enchantment<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = MapMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.remove_enchant(arg0)
    }
    pub fn has_conflicting_enchant(
        &self,
        arg0: impl Into<crate::enchantments::Enchantment<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = MapMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.has_conflicting_enchant(arg0)
    }
    pub fn add_item_flags(
        &self,
        arg0: Vec<crate::inventory::ItemFlag<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("([Lorg/bukkit/inventory/ItemFlag;)V");
        let arr = self.jni_ref().new_object_array(
            arg0.len() as i32,
            "org/bukkit/inventory/ItemFlag",
            jni::objects::JObject::null(),
        );
        let arr = self.jni_ref().translate_error_no_gen(arr)?;
        for i in 0..arg0.len() {
            let val_1 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(arg0.get(i).unwrap().jni_object().clone())
            });
            self.jni_ref()
                .set_object_array_element(&arr, i as i32, val_1.l()?)?;
        }
        let val_1 = jni::objects::JValueGen::Object(arr);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "addItemFlags",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1.l()?)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub fn remove_item_flags(
        &self,
        arg0: Vec<crate::inventory::ItemFlag<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("([Lorg/bukkit/inventory/ItemFlag;)V");
        let arr = self.jni_ref().new_object_array(
            arg0.len() as i32,
            "org/bukkit/inventory/ItemFlag",
            jni::objects::JObject::null(),
        );
        let arr = self.jni_ref().translate_error_no_gen(arr)?;
        for i in 0..arg0.len() {
            let val_1 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(arg0.get(i).unwrap().jni_object().clone())
            });
            self.jni_ref()
                .set_object_array_element(&arr, i as i32, val_1.l()?)?;
        }
        let val_1 = jni::objects::JValueGen::Object(arr);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeItemFlags",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1.l()?)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub fn item_flags(
        &self,
    ) -> Result<blackboxmc_java::util::JavaSet<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/Set;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getItemFlags", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::util::JavaSet::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    pub fn has_item_flag(
        &self,
        arg0: impl Into<crate::inventory::ItemFlag<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = MapMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.has_item_flag(arg0)
    }
    pub fn is_unbreakable(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = MapMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.is_unbreakable()
    }
    pub fn set_unbreakable(&self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = MapMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.set_unbreakable(arg0)
    }
    pub fn has_attribute_modifiers(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "hasAttributeModifiers",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    pub fn attribute_modifiers(
        &self,
    ) -> Result<Option<jni::objects::JObject<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lcom/google/common/collect/Multimap;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getAttributeModifiers",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {
            return Ok(None);
        }
        Ok(Some(res.l()?))
    }
    pub fn add_attribute_modifier(
        &self,
        arg0: impl Into<crate::attribute::Attribute<'mc>>,
        arg1: impl Into<crate::attribute::AttributeModifier<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = MapMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.add_attribute_modifier(arg0, arg1)
    }
    pub fn set_attribute_modifiers(
        &self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lcom/google/common/collect/Multimap;)V");
        let val_1 = jni::objects::JValueGen::Object(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setAttributeModifiers",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub fn remove_attribute_modifier_with_equipment_slot(
        &self,
        arg0: impl Into<crate::inventory::EquipmentSlot<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/inventory/EquipmentSlot;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        sig += ")Z";
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeAttributeModifier",
            sig.as_str(),
            args,
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    pub fn remove_attribute_modifier_with_attribute(
        &self,
        arg0: impl Into<crate::attribute::Attribute<'mc>>,
        arg1: std::option::Option<impl Into<crate::attribute::AttributeModifier<'mc>>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/attribute/Attribute;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        if let Some(a) = arg1 {
            sig += "Lorg/bukkit/attribute/AttributeModifier;";
            let val_2 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(a.into().jni_object().clone())
            });
            args.push(val_2);
        }
        sig += ")Z";
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeAttributeModifier",
            sig.as_str(),
            args,
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn custom_tag_container(
        &self,
    ) -> Result<crate::inventory::meta::tags::CustomItemTagContainer<'mc>, Box<dyn std::error::Error>>
    {
        let temp_clone = MapMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.custom_tag_container()
    }

    pub fn set_version(&self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = MapMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.set_version(arg0)
    }
    // SUPER CLASS: Cloneable
    // SUPER CLASS: ConfigurationSerializable
    pub fn serialize(
        &self,
    ) -> Result<blackboxmc_java::util::JavaMap<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::configuration::serialization::ConfigurationSerializable::from_raw(
            &self.0,
            unsafe { jni::objects::JObject::from_raw(self.1.clone()) },
        )?;
        let real: crate::configuration::serialization::ConfigurationSerializable =
            temp_clone.into();
        real.serialize()
    }
    // SUPER CLASS: PersistentDataHolder
    pub fn persistent_data_container(
        &self,
    ) -> Result<crate::persistence::PersistentDataContainer<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::persistence::PersistentDataHolder::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::persistence::PersistentDataHolder = temp_clone.into();
        real.persistent_data_container()
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::inventory::meta::ItemMeta<'mc>> for MapMeta<'mc> {
    fn into(self) -> crate::inventory::meta::ItemMeta<'mc> {
        crate::inventory::meta::ItemMeta::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting MapMeta into crate::inventory::meta::ItemMeta")
    }
}

#[repr(C)]
pub struct BookMetaSpigot<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for BookMetaSpigot<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for BookMetaSpigot<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate BookMetaSpigot from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/inventory/meta/BookMeta$Spigot")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a BookMetaSpigot object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> BookMetaSpigot<'mc> {
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<crate::inventory::meta::BookMetaSpigot<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let cls = jni.find_class("org/bukkit/inventory/meta/BookMeta$Spigot");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(cls, sig.as_str(), vec![]);
        let res = jni.translate_error_no_gen(res)?;
        crate::inventory::meta::BookMetaSpigot::from_raw(&jni, res)
    }
    /// Gets the specified page in the book. The given page must exist.
    pub fn get_page(
        &self,
        arg0: i32,
    ) -> Result<
        Vec<blackboxmc_bungee::bungee::api::chat::BaseComponent<'mc>>,
        Box<dyn std::error::Error>,
    > {
        let sig = String::from("(I)Lnet/md_5/bungee/api/chat/BaseComponent;");
        let val_1 = jni::objects::JValueGen::Int(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getPage",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        let arr = Into::<jni::objects::JObjectArray>::into(res.l()?);
        let len = self.jni_ref().get_array_length(&arr)?;
        let mut vec = Vec::new();
        for i in 0..len {
            let res = self.jni_ref().get_object_array_element(&arr, i)?;
            vec.push({
                blackboxmc_bungee::bungee::api::chat::BaseComponent::from_raw(&self.jni_ref(), res)?
            });
        }
        Ok(vec)
    }

    pub fn set_page(
        &self,
        arg0: i32,
        arg1: Vec<blackboxmc_bungee::bungee::api::chat::BaseComponent<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(I[Lnet/md_5/bungee/api/chat/BaseComponent;)V");
        let val_1 = jni::objects::JValueGen::Int(arg0);
        let arr = self.jni_ref().new_object_array(
            arg1.len() as i32,
            "net/md_5/bungee/api/chat/BaseComponent",
            jni::objects::JObject::null(),
        );
        let arr = self.jni_ref().translate_error_no_gen(arr)?;
        for i in 0..arg1.len() {
            let val_2 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(arg1.get(i).unwrap().jni_object().clone())
            });
            self.jni_ref()
                .set_object_array_element(&arr, i as i32, val_2.l()?)?;
        }
        let val_2 = jni::objects::JValueGen::Object(arr);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setPage",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2.l()?),
            ],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn pages(
        &self,
    ) -> Result<
        Vec<blackboxmc_bungee::bungee::api::chat::BaseComponent<'mc>>,
        Box<dyn std::error::Error>,
    > {
        let sig = String::from("()Ljava/util/List;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getPages", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let list = blackboxmc_java::util::JavaList::from_raw(&self.0, res.l()?)?;
        let iter = list.iterator()?;
        while iter.has_next()? {
            let obj = iter.next()?;
            new_vec
                .push(blackboxmc_bungee::bungee::api::chat::BaseComponent::from_raw(&self.0, obj)?);
        }
        Ok(new_vec)
    }

    pub fn set_pages_with_list(
        &self,
        arg0: Vec<impl Into<blackboxmc_bungee::bungee::api::chat::BaseComponent<'mc>>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Ljava/util/List;";
        let raw_val_1 = self
            .jni_ref()
            .new_object("java/util/ArrayList", "()V", vec![])?;
        for v in arg0 {
            sig += "Lnet/md_5/bungee/api/chat/blackboxmc_bungee::bungee::api::chat::BaseComponent;";
            let map_val_0 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(v.into().jni_object().clone())
            });
            self.jni_ref().call_method(
                &raw_val_1,
                "add",
                "(Ljava/lang/Object;)Z",
                vec![jni::objects::JValueGen::from(map_val_0)],
            )?;
        }
        let val_1 = jni::objects::JValueGen::Object(raw_val_1);
        args.push(val_1);
        sig += ")V";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "setPages", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn add_page(
        &self,
        arg0: Vec<blackboxmc_bungee::bungee::api::chat::BaseComponent<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("([Lnet/md_5/bungee/api/chat/BaseComponent;)V");
        let arr = self.jni_ref().new_object_array(
            arg0.len() as i32,
            "net/md_5/bungee/api/chat/BaseComponent",
            jni::objects::JObject::null(),
        );
        let arr = self.jni_ref().translate_error_no_gen(arr)?;
        for i in 0..arg0.len() {
            let val_1 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(arg0.get(i).unwrap().jni_object().clone())
            });
            self.jni_ref()
                .set_object_array_element(&arr, i as i32, val_1.l()?)?;
        }
        let val_1 = jni::objects::JValueGen::Object(arr);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "addPage",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1.l()?)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    // SUPER CLASS: Object

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
/// Represents leather armor (<a href="../../Material.html#LEATHER_BOOTS"><code>Material.LEATHER_BOOTS</code></a>, <a href="../../Material.html#LEATHER_CHESTPLATE"><code>Material.LEATHER_CHESTPLATE</code></a>, <a href="../../Material.html#LEATHER_HELMET"><code>Material.LEATHER_HELMET</code></a>, or <a href="../../Material.html#LEATHER_LEGGINGS"><code>Material.LEATHER_LEGGINGS</code></a>) that can be colored.
///
/// This is a representation of an abstract class.
#[repr(C)]
pub struct LeatherArmorMeta<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for LeatherArmorMeta<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for LeatherArmorMeta<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate LeatherArmorMeta from null object.").into(),
            );
        }
        let (valid, name) =
            env.validate_name(&obj, "org/bukkit/inventory/meta/LeatherArmorMeta")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a LeatherArmorMeta object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> LeatherArmorMeta<'mc> {
    pub fn set_color(
        &self,
        arg0: impl Into<crate::Color<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/Color;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setColor",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn color(&self) -> Result<Option<crate::Color<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/Color;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getColor", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {
            return Ok(None);
        }
        Ok(Some(crate::Color::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })?))
    }

    pub fn clone(
        &self,
    ) -> Result<crate::inventory::meta::LeatherArmorMeta<'mc>, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Lorg/bukkit/inventory/meta/LeatherArmorMeta;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "clone", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::meta::LeatherArmorMeta::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    pub fn display_name(&self) -> Result<String, Box<dyn std::error::Error>> {
        let temp_clone = LeatherArmorMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.display_name()
    }
    pub fn as_string(&self) -> Result<String, Box<dyn std::error::Error>> {
        let temp_clone = LeatherArmorMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.as_string()
    }
    pub fn has_display_name(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = LeatherArmorMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.has_display_name()
    }
    pub fn set_display_name(
        &self,
        arg0: impl Into<String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = LeatherArmorMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.set_display_name(arg0)
    }
    pub fn has_localized_name(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = LeatherArmorMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.has_localized_name()
    }
    pub fn localized_name(&self) -> Result<String, Box<dyn std::error::Error>> {
        let temp_clone = LeatherArmorMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.localized_name()
    }
    pub fn set_localized_name(
        &self,
        arg0: impl Into<String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = LeatherArmorMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.set_localized_name(arg0)
    }
    pub fn has_lore(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = LeatherArmorMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.has_lore()
    }
    pub fn lore(&self) -> Result<Option<Vec<String>>, Box<dyn std::error::Error>> {
        let temp_clone = LeatherArmorMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.lore()
    }
    pub fn set_lore(&self, arg0: Vec<impl Into<String>>) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = LeatherArmorMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.set_lore(arg0)
    }
    pub fn has_custom_model_data(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = LeatherArmorMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.has_custom_model_data()
    }
    pub fn custom_model_data(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let temp_clone = LeatherArmorMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.custom_model_data()
    }
    pub fn set_custom_model_data(&self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = LeatherArmorMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.set_custom_model_data(arg0)
    }
    pub fn has_enchants(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "hasEnchants", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    pub fn has_enchant(
        &self,
        arg0: impl Into<crate::enchantments::Enchantment<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = LeatherArmorMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.has_enchant(arg0)
    }
    pub fn get_enchant_level(
        &self,
        arg0: impl Into<crate::enchantments::Enchantment<'mc>>,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        let temp_clone = LeatherArmorMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.get_enchant_level(arg0)
    }
    pub fn enchants(
        &self,
    ) -> Result<blackboxmc_java::util::JavaMap<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/Map;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getEnchants", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::util::JavaMap::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    pub fn add_enchant(
        &self,
        arg0: impl Into<crate::enchantments::Enchantment<'mc>>,
        arg1: i32,
        arg2: bool,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = LeatherArmorMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.add_enchant(arg0, arg1, arg2)
    }
    pub fn remove_enchant(
        &self,
        arg0: impl Into<crate::enchantments::Enchantment<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = LeatherArmorMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.remove_enchant(arg0)
    }
    pub fn has_conflicting_enchant(
        &self,
        arg0: impl Into<crate::enchantments::Enchantment<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = LeatherArmorMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.has_conflicting_enchant(arg0)
    }
    pub fn add_item_flags(
        &self,
        arg0: Vec<crate::inventory::ItemFlag<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("([Lorg/bukkit/inventory/ItemFlag;)V");
        let arr = self.jni_ref().new_object_array(
            arg0.len() as i32,
            "org/bukkit/inventory/ItemFlag",
            jni::objects::JObject::null(),
        );
        let arr = self.jni_ref().translate_error_no_gen(arr)?;
        for i in 0..arg0.len() {
            let val_1 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(arg0.get(i).unwrap().jni_object().clone())
            });
            self.jni_ref()
                .set_object_array_element(&arr, i as i32, val_1.l()?)?;
        }
        let val_1 = jni::objects::JValueGen::Object(arr);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "addItemFlags",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1.l()?)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub fn remove_item_flags(
        &self,
        arg0: Vec<crate::inventory::ItemFlag<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("([Lorg/bukkit/inventory/ItemFlag;)V");
        let arr = self.jni_ref().new_object_array(
            arg0.len() as i32,
            "org/bukkit/inventory/ItemFlag",
            jni::objects::JObject::null(),
        );
        let arr = self.jni_ref().translate_error_no_gen(arr)?;
        for i in 0..arg0.len() {
            let val_1 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(arg0.get(i).unwrap().jni_object().clone())
            });
            self.jni_ref()
                .set_object_array_element(&arr, i as i32, val_1.l()?)?;
        }
        let val_1 = jni::objects::JValueGen::Object(arr);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeItemFlags",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1.l()?)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub fn item_flags(
        &self,
    ) -> Result<blackboxmc_java::util::JavaSet<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/Set;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getItemFlags", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::util::JavaSet::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    pub fn has_item_flag(
        &self,
        arg0: impl Into<crate::inventory::ItemFlag<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = LeatherArmorMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.has_item_flag(arg0)
    }
    pub fn is_unbreakable(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = LeatherArmorMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.is_unbreakable()
    }
    pub fn set_unbreakable(&self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = LeatherArmorMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.set_unbreakable(arg0)
    }
    pub fn has_attribute_modifiers(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "hasAttributeModifiers",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    pub fn attribute_modifiers(
        &self,
    ) -> Result<Option<jni::objects::JObject<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lcom/google/common/collect/Multimap;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getAttributeModifiers",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {
            return Ok(None);
        }
        Ok(Some(res.l()?))
    }
    pub fn add_attribute_modifier(
        &self,
        arg0: impl Into<crate::attribute::Attribute<'mc>>,
        arg1: impl Into<crate::attribute::AttributeModifier<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = LeatherArmorMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.add_attribute_modifier(arg0, arg1)
    }
    pub fn set_attribute_modifiers(
        &self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lcom/google/common/collect/Multimap;)V");
        let val_1 = jni::objects::JValueGen::Object(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setAttributeModifiers",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub fn remove_attribute_modifier_with_equipment_slot(
        &self,
        arg0: impl Into<crate::inventory::EquipmentSlot<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/inventory/EquipmentSlot;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        sig += ")Z";
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeAttributeModifier",
            sig.as_str(),
            args,
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    pub fn remove_attribute_modifier_with_attribute(
        &self,
        arg0: impl Into<crate::attribute::Attribute<'mc>>,
        arg1: std::option::Option<impl Into<crate::attribute::AttributeModifier<'mc>>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/attribute/Attribute;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        if let Some(a) = arg1 {
            sig += "Lorg/bukkit/attribute/AttributeModifier;";
            let val_2 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(a.into().jni_object().clone())
            });
            args.push(val_2);
        }
        sig += ")Z";
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeAttributeModifier",
            sig.as_str(),
            args,
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn custom_tag_container(
        &self,
    ) -> Result<crate::inventory::meta::tags::CustomItemTagContainer<'mc>, Box<dyn std::error::Error>>
    {
        let temp_clone = LeatherArmorMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.custom_tag_container()
    }

    pub fn set_version(&self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = LeatherArmorMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.set_version(arg0)
    }
    // SUPER CLASS: Cloneable
    // SUPER CLASS: ConfigurationSerializable
    pub fn serialize(
        &self,
    ) -> Result<blackboxmc_java::util::JavaMap<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::configuration::serialization::ConfigurationSerializable::from_raw(
            &self.0,
            unsafe { jni::objects::JObject::from_raw(self.1.clone()) },
        )?;
        let real: crate::configuration::serialization::ConfigurationSerializable =
            temp_clone.into();
        real.serialize()
    }
    // SUPER CLASS: PersistentDataHolder
    pub fn persistent_data_container(
        &self,
    ) -> Result<crate::persistence::PersistentDataContainer<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::persistence::PersistentDataHolder::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::persistence::PersistentDataHolder = temp_clone.into();
        real.persistent_data_container()
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::inventory::meta::ItemMeta<'mc>> for LeatherArmorMeta<'mc> {
    fn into(self) -> crate::inventory::meta::ItemMeta<'mc> {
        crate::inventory::meta::ItemMeta::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting LeatherArmorMeta into crate::inventory::meta::ItemMeta")
    }
}
/// Represents a meta that can store a single FireworkEffect. An example includes <a href="../../Material.html#FIREWORK_STAR"><code>Material.FIREWORK_STAR</code></a>.
///
/// This is a representation of an abstract class.
#[repr(C)]
pub struct FireworkEffectMeta<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for FireworkEffectMeta<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for FireworkEffectMeta<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate FireworkEffectMeta from null object.").into(),
            );
        }
        let (valid, name) =
            env.validate_name(&obj, "org/bukkit/inventory/meta/FireworkEffectMeta")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a FireworkEffectMeta object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> FireworkEffectMeta<'mc> {
    pub fn set_effect(
        &self,
        arg0: impl Into<crate::FireworkEffect<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/FireworkEffect;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setEffect",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn has_effect(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasEffect", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn effect(&self) -> Result<Option<crate::FireworkEffect<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/FireworkEffect;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getEffect", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {
            return Ok(None);
        }
        Ok(Some(crate::FireworkEffect::from_raw(
            &self.jni_ref(),
            unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) },
        )?))
    }

    pub fn clone(
        &self,
    ) -> Result<crate::inventory::meta::FireworkEffectMeta<'mc>, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Lorg/bukkit/inventory/meta/FireworkEffectMeta;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "clone", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::meta::FireworkEffectMeta::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    pub fn display_name(&self) -> Result<String, Box<dyn std::error::Error>> {
        let temp_clone = FireworkEffectMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.display_name()
    }
    pub fn as_string(&self) -> Result<String, Box<dyn std::error::Error>> {
        let temp_clone = FireworkEffectMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.as_string()
    }
    pub fn has_display_name(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = FireworkEffectMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.has_display_name()
    }
    pub fn set_display_name(
        &self,
        arg0: impl Into<String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = FireworkEffectMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.set_display_name(arg0)
    }
    pub fn has_localized_name(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = FireworkEffectMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.has_localized_name()
    }
    pub fn localized_name(&self) -> Result<String, Box<dyn std::error::Error>> {
        let temp_clone = FireworkEffectMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.localized_name()
    }
    pub fn set_localized_name(
        &self,
        arg0: impl Into<String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = FireworkEffectMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.set_localized_name(arg0)
    }
    pub fn has_lore(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = FireworkEffectMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.has_lore()
    }
    pub fn lore(&self) -> Result<Option<Vec<String>>, Box<dyn std::error::Error>> {
        let temp_clone = FireworkEffectMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.lore()
    }
    pub fn set_lore(&self, arg0: Vec<impl Into<String>>) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = FireworkEffectMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.set_lore(arg0)
    }
    pub fn has_custom_model_data(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = FireworkEffectMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.has_custom_model_data()
    }
    pub fn custom_model_data(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let temp_clone = FireworkEffectMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.custom_model_data()
    }
    pub fn set_custom_model_data(&self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = FireworkEffectMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.set_custom_model_data(arg0)
    }
    pub fn has_enchants(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "hasEnchants", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    pub fn has_enchant(
        &self,
        arg0: impl Into<crate::enchantments::Enchantment<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = FireworkEffectMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.has_enchant(arg0)
    }
    pub fn get_enchant_level(
        &self,
        arg0: impl Into<crate::enchantments::Enchantment<'mc>>,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        let temp_clone = FireworkEffectMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.get_enchant_level(arg0)
    }
    pub fn enchants(
        &self,
    ) -> Result<blackboxmc_java::util::JavaMap<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/Map;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getEnchants", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::util::JavaMap::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    pub fn add_enchant(
        &self,
        arg0: impl Into<crate::enchantments::Enchantment<'mc>>,
        arg1: i32,
        arg2: bool,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = FireworkEffectMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.add_enchant(arg0, arg1, arg2)
    }
    pub fn remove_enchant(
        &self,
        arg0: impl Into<crate::enchantments::Enchantment<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = FireworkEffectMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.remove_enchant(arg0)
    }
    pub fn has_conflicting_enchant(
        &self,
        arg0: impl Into<crate::enchantments::Enchantment<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = FireworkEffectMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.has_conflicting_enchant(arg0)
    }
    pub fn add_item_flags(
        &self,
        arg0: Vec<crate::inventory::ItemFlag<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("([Lorg/bukkit/inventory/ItemFlag;)V");
        let arr = self.jni_ref().new_object_array(
            arg0.len() as i32,
            "org/bukkit/inventory/ItemFlag",
            jni::objects::JObject::null(),
        );
        let arr = self.jni_ref().translate_error_no_gen(arr)?;
        for i in 0..arg0.len() {
            let val_1 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(arg0.get(i).unwrap().jni_object().clone())
            });
            self.jni_ref()
                .set_object_array_element(&arr, i as i32, val_1.l()?)?;
        }
        let val_1 = jni::objects::JValueGen::Object(arr);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "addItemFlags",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1.l()?)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub fn remove_item_flags(
        &self,
        arg0: Vec<crate::inventory::ItemFlag<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("([Lorg/bukkit/inventory/ItemFlag;)V");
        let arr = self.jni_ref().new_object_array(
            arg0.len() as i32,
            "org/bukkit/inventory/ItemFlag",
            jni::objects::JObject::null(),
        );
        let arr = self.jni_ref().translate_error_no_gen(arr)?;
        for i in 0..arg0.len() {
            let val_1 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(arg0.get(i).unwrap().jni_object().clone())
            });
            self.jni_ref()
                .set_object_array_element(&arr, i as i32, val_1.l()?)?;
        }
        let val_1 = jni::objects::JValueGen::Object(arr);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeItemFlags",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1.l()?)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub fn item_flags(
        &self,
    ) -> Result<blackboxmc_java::util::JavaSet<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/Set;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getItemFlags", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::util::JavaSet::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    pub fn has_item_flag(
        &self,
        arg0: impl Into<crate::inventory::ItemFlag<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = FireworkEffectMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.has_item_flag(arg0)
    }
    pub fn is_unbreakable(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = FireworkEffectMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.is_unbreakable()
    }
    pub fn set_unbreakable(&self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = FireworkEffectMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.set_unbreakable(arg0)
    }
    pub fn has_attribute_modifiers(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "hasAttributeModifiers",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    pub fn attribute_modifiers(
        &self,
    ) -> Result<Option<jni::objects::JObject<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lcom/google/common/collect/Multimap;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getAttributeModifiers",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {
            return Ok(None);
        }
        Ok(Some(res.l()?))
    }
    pub fn add_attribute_modifier(
        &self,
        arg0: impl Into<crate::attribute::Attribute<'mc>>,
        arg1: impl Into<crate::attribute::AttributeModifier<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = FireworkEffectMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.add_attribute_modifier(arg0, arg1)
    }
    pub fn set_attribute_modifiers(
        &self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lcom/google/common/collect/Multimap;)V");
        let val_1 = jni::objects::JValueGen::Object(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setAttributeModifiers",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub fn remove_attribute_modifier_with_equipment_slot(
        &self,
        arg0: impl Into<crate::inventory::EquipmentSlot<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/inventory/EquipmentSlot;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        sig += ")Z";
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeAttributeModifier",
            sig.as_str(),
            args,
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    pub fn remove_attribute_modifier_with_attribute(
        &self,
        arg0: impl Into<crate::attribute::Attribute<'mc>>,
        arg1: std::option::Option<impl Into<crate::attribute::AttributeModifier<'mc>>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/attribute/Attribute;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        if let Some(a) = arg1 {
            sig += "Lorg/bukkit/attribute/AttributeModifier;";
            let val_2 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(a.into().jni_object().clone())
            });
            args.push(val_2);
        }
        sig += ")Z";
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeAttributeModifier",
            sig.as_str(),
            args,
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn custom_tag_container(
        &self,
    ) -> Result<crate::inventory::meta::tags::CustomItemTagContainer<'mc>, Box<dyn std::error::Error>>
    {
        let temp_clone = FireworkEffectMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.custom_tag_container()
    }

    pub fn set_version(&self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = FireworkEffectMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.set_version(arg0)
    }
    // SUPER CLASS: Cloneable
    // SUPER CLASS: ConfigurationSerializable
    pub fn serialize(
        &self,
    ) -> Result<blackboxmc_java::util::JavaMap<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::configuration::serialization::ConfigurationSerializable::from_raw(
            &self.0,
            unsafe { jni::objects::JObject::from_raw(self.1.clone()) },
        )?;
        let real: crate::configuration::serialization::ConfigurationSerializable =
            temp_clone.into();
        real.serialize()
    }
    // SUPER CLASS: PersistentDataHolder
    pub fn persistent_data_container(
        &self,
    ) -> Result<crate::persistence::PersistentDataContainer<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::persistence::PersistentDataHolder::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::persistence::PersistentDataHolder = temp_clone.into();
        real.persistent_data_container()
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::inventory::meta::ItemMeta<'mc>> for FireworkEffectMeta<'mc> {
    fn into(self) -> crate::inventory::meta::ItemMeta<'mc> {
        crate::inventory::meta::ItemMeta::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting FireworkEffectMeta into crate::inventory::meta::ItemMeta")
    }
}
/// Represents a bucket of tropical fish.
///
/// This is a representation of an abstract class.
#[repr(C)]
pub struct TropicalFishBucketMeta<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for TropicalFishBucketMeta<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for TropicalFishBucketMeta<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate TropicalFishBucketMeta from null object."
            )
            .into());
        }
        let (valid, name) =
            env.validate_name(&obj, "org/bukkit/inventory/meta/TropicalFishBucketMeta")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a TropicalFishBucketMeta object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> TropicalFishBucketMeta<'mc> {
    pub fn pattern(
        &self,
    ) -> Result<crate::entity::TropicalFishPattern<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/entity/TropicalFish$Pattern;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getPattern", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::entity::TropicalFishPattern::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn set_pattern(
        &self,
        arg0: impl Into<crate::entity::TropicalFishPattern<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/entity/TropicalFish$Pattern;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setPattern",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn pattern_color(&self) -> Result<crate::DyeColor<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/DyeColor;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getPatternColor", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::DyeColor::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn set_pattern_color(
        &self,
        arg0: impl Into<crate::DyeColor<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/DyeColor;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setPatternColor",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn body_color(&self) -> Result<crate::DyeColor<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/DyeColor;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getBodyColor", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::DyeColor::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn set_body_color(
        &self,
        arg0: impl Into<crate::DyeColor<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/DyeColor;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setBodyColor",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn has_variant(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "hasVariant", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn clone(&self) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Ljava/lang/Object;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "clone", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l()?)
    }
    pub fn display_name(&self) -> Result<String, Box<dyn std::error::Error>> {
        let temp_clone = TropicalFishBucketMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.display_name()
    }
    pub fn as_string(&self) -> Result<String, Box<dyn std::error::Error>> {
        let temp_clone = TropicalFishBucketMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.as_string()
    }
    pub fn has_display_name(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = TropicalFishBucketMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.has_display_name()
    }
    pub fn set_display_name(
        &self,
        arg0: impl Into<String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = TropicalFishBucketMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.set_display_name(arg0)
    }
    pub fn has_localized_name(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = TropicalFishBucketMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.has_localized_name()
    }
    pub fn localized_name(&self) -> Result<String, Box<dyn std::error::Error>> {
        let temp_clone = TropicalFishBucketMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.localized_name()
    }
    pub fn set_localized_name(
        &self,
        arg0: impl Into<String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = TropicalFishBucketMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.set_localized_name(arg0)
    }
    pub fn has_lore(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = TropicalFishBucketMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.has_lore()
    }
    pub fn lore(&self) -> Result<Option<Vec<String>>, Box<dyn std::error::Error>> {
        let temp_clone = TropicalFishBucketMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.lore()
    }
    pub fn set_lore(&self, arg0: Vec<impl Into<String>>) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = TropicalFishBucketMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.set_lore(arg0)
    }
    pub fn has_custom_model_data(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = TropicalFishBucketMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.has_custom_model_data()
    }
    pub fn custom_model_data(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let temp_clone = TropicalFishBucketMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.custom_model_data()
    }
    pub fn set_custom_model_data(&self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = TropicalFishBucketMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.set_custom_model_data(arg0)
    }
    pub fn has_enchants(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "hasEnchants", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    pub fn has_enchant(
        &self,
        arg0: impl Into<crate::enchantments::Enchantment<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = TropicalFishBucketMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.has_enchant(arg0)
    }
    pub fn get_enchant_level(
        &self,
        arg0: impl Into<crate::enchantments::Enchantment<'mc>>,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        let temp_clone = TropicalFishBucketMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.get_enchant_level(arg0)
    }
    pub fn enchants(
        &self,
    ) -> Result<blackboxmc_java::util::JavaMap<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/Map;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getEnchants", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::util::JavaMap::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    pub fn add_enchant(
        &self,
        arg0: impl Into<crate::enchantments::Enchantment<'mc>>,
        arg1: i32,
        arg2: bool,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = TropicalFishBucketMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.add_enchant(arg0, arg1, arg2)
    }
    pub fn remove_enchant(
        &self,
        arg0: impl Into<crate::enchantments::Enchantment<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = TropicalFishBucketMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.remove_enchant(arg0)
    }
    pub fn has_conflicting_enchant(
        &self,
        arg0: impl Into<crate::enchantments::Enchantment<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = TropicalFishBucketMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.has_conflicting_enchant(arg0)
    }
    pub fn add_item_flags(
        &self,
        arg0: Vec<crate::inventory::ItemFlag<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("([Lorg/bukkit/inventory/ItemFlag;)V");
        let arr = self.jni_ref().new_object_array(
            arg0.len() as i32,
            "org/bukkit/inventory/ItemFlag",
            jni::objects::JObject::null(),
        );
        let arr = self.jni_ref().translate_error_no_gen(arr)?;
        for i in 0..arg0.len() {
            let val_1 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(arg0.get(i).unwrap().jni_object().clone())
            });
            self.jni_ref()
                .set_object_array_element(&arr, i as i32, val_1.l()?)?;
        }
        let val_1 = jni::objects::JValueGen::Object(arr);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "addItemFlags",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1.l()?)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub fn remove_item_flags(
        &self,
        arg0: Vec<crate::inventory::ItemFlag<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("([Lorg/bukkit/inventory/ItemFlag;)V");
        let arr = self.jni_ref().new_object_array(
            arg0.len() as i32,
            "org/bukkit/inventory/ItemFlag",
            jni::objects::JObject::null(),
        );
        let arr = self.jni_ref().translate_error_no_gen(arr)?;
        for i in 0..arg0.len() {
            let val_1 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(arg0.get(i).unwrap().jni_object().clone())
            });
            self.jni_ref()
                .set_object_array_element(&arr, i as i32, val_1.l()?)?;
        }
        let val_1 = jni::objects::JValueGen::Object(arr);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeItemFlags",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1.l()?)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub fn item_flags(
        &self,
    ) -> Result<blackboxmc_java::util::JavaSet<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/Set;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getItemFlags", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::util::JavaSet::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    pub fn has_item_flag(
        &self,
        arg0: impl Into<crate::inventory::ItemFlag<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = TropicalFishBucketMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.has_item_flag(arg0)
    }
    pub fn is_unbreakable(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = TropicalFishBucketMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.is_unbreakable()
    }
    pub fn set_unbreakable(&self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = TropicalFishBucketMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.set_unbreakable(arg0)
    }
    pub fn has_attribute_modifiers(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "hasAttributeModifiers",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    pub fn attribute_modifiers(
        &self,
    ) -> Result<Option<jni::objects::JObject<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lcom/google/common/collect/Multimap;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getAttributeModifiers",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {
            return Ok(None);
        }
        Ok(Some(res.l()?))
    }
    pub fn add_attribute_modifier(
        &self,
        arg0: impl Into<crate::attribute::Attribute<'mc>>,
        arg1: impl Into<crate::attribute::AttributeModifier<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = TropicalFishBucketMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.add_attribute_modifier(arg0, arg1)
    }
    pub fn set_attribute_modifiers(
        &self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lcom/google/common/collect/Multimap;)V");
        let val_1 = jni::objects::JValueGen::Object(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setAttributeModifiers",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub fn remove_attribute_modifier_with_equipment_slot(
        &self,
        arg0: impl Into<crate::inventory::EquipmentSlot<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/inventory/EquipmentSlot;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        sig += ")Z";
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeAttributeModifier",
            sig.as_str(),
            args,
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    pub fn remove_attribute_modifier_with_attribute(
        &self,
        arg0: impl Into<crate::attribute::Attribute<'mc>>,
        arg1: std::option::Option<impl Into<crate::attribute::AttributeModifier<'mc>>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/attribute/Attribute;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        if let Some(a) = arg1 {
            sig += "Lorg/bukkit/attribute/AttributeModifier;";
            let val_2 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(a.into().jni_object().clone())
            });
            args.push(val_2);
        }
        sig += ")Z";
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeAttributeModifier",
            sig.as_str(),
            args,
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn custom_tag_container(
        &self,
    ) -> Result<crate::inventory::meta::tags::CustomItemTagContainer<'mc>, Box<dyn std::error::Error>>
    {
        let temp_clone = TropicalFishBucketMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.custom_tag_container()
    }

    pub fn set_version(&self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = TropicalFishBucketMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.set_version(arg0)
    }
    // SUPER CLASS: Cloneable
    // SUPER CLASS: ConfigurationSerializable
    pub fn serialize(
        &self,
    ) -> Result<blackboxmc_java::util::JavaMap<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::configuration::serialization::ConfigurationSerializable::from_raw(
            &self.0,
            unsafe { jni::objects::JObject::from_raw(self.1.clone()) },
        )?;
        let real: crate::configuration::serialization::ConfigurationSerializable =
            temp_clone.into();
        real.serialize()
    }
    // SUPER CLASS: PersistentDataHolder
    pub fn persistent_data_container(
        &self,
    ) -> Result<crate::persistence::PersistentDataContainer<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::persistence::PersistentDataHolder::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::persistence::PersistentDataHolder = temp_clone.into();
        real.persistent_data_container()
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::inventory::meta::ItemMeta<'mc>> for TropicalFishBucketMeta<'mc> {
    fn into(self) -> crate::inventory::meta::ItemMeta<'mc> {
        crate::inventory::meta::ItemMeta::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting TropicalFishBucketMeta into crate::inventory::meta::ItemMeta")
    }
}
/// Represents a book (<a href="../../Material.html#WRITABLE_BOOK"><code>Material.WRITABLE_BOOK</code></a> or <a href="../../Material.html#WRITTEN_BOOK"><code>Material.WRITTEN_BOOK</code></a>) that can have a title, an author, and pages.
///
/// This is a representation of an abstract class.
#[repr(C)]
pub struct BookMeta<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for BookMeta<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for BookMeta<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate BookMeta from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/inventory/meta/BookMeta")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a BookMeta object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> BookMeta<'mc> {
    pub fn spigot(
        &self,
    ) -> Result<crate::inventory::meta::BookMetaSpigot<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/inventory/meta/BookMeta$Spigot;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "spigot", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::meta::BookMetaSpigot::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Gets the specified page in the book. The given page must exist.
    /// <p>Pages are 1-indexed.</p>
    pub fn get_page(&self, arg0: i32) -> Result<String, Box<dyn std::error::Error>> {
        let sig = String::from("(I)Ljava/lang/String;");
        let val_1 = jni::objects::JValueGen::Int(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getPage",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }

    pub fn set_page(
        &self,
        arg0: i32,
        arg1: impl Into<String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(ILjava/lang/String;)V");
        let val_1 = jni::objects::JValueGen::Int(arg0);
        let val_2 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(arg1.into())?,
        ));
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setPage",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
            ],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
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

    pub fn author(&self) -> Result<String, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/String;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getAuthor", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }

    pub fn set_author(&self, arg0: impl Into<String>) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/String;)V");
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(arg0.into())?,
        ));
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setAuthor",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn pages(&self) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/List;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getPages", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let list = blackboxmc_java::util::JavaList::from_raw(&self.jni_ref(), res.l()?)?;
        let iter = list.iterator()?;
        while iter.has_next()? {
            let obj = iter.next()?;
            new_vec.push(
                self.jni_ref()
                    .get_string(unsafe { &jni::objects::JString::from_raw(*obj) })?
                    .to_string_lossy()
                    .to_string(),
            );
        }
        Ok(new_vec)
    }

    pub fn set_pages_with_strings(
        &self,
        arg0: Vec<String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "[Ljava/lang/String;";
        let arr = self.jni_ref().new_object_array(
            arg0.len() as i32,
            "java/lang/String",
            jni::objects::JObject::null(),
        );
        let arr = self.jni_ref().translate_error_no_gen(arr)?;
        for i in 0..arg0.len() {
            let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
                self.jni_ref().new_string(arg0.get(i).unwrap().clone())?,
            ));
            self.jni_ref()
                .set_object_array_element(&arr, i as i32, val_1.l()?)?;
        }
        let val_1 = jni::objects::JValueGen::Object(arr);
        args.push(val_1.l()?.into());
        sig += ")V";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "setPages", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn add_page(&self, arg0: Vec<String>) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("([Ljava/lang/String;)V");
        let arr = self.jni_ref().new_object_array(
            arg0.len() as i32,
            "java/lang/String",
            jni::objects::JObject::null(),
        );
        let arr = self.jni_ref().translate_error_no_gen(arr)?;
        for i in 0..arg0.len() {
            let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
                self.jni_ref().new_string(arg0.get(i).unwrap().clone())?,
            ));
            self.jni_ref()
                .set_object_array_element(&arr, i as i32, val_1.l()?)?;
        }
        let val_1 = jni::objects::JValueGen::Object(arr);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "addPage",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1.l()?)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn set_title(&self, arg0: impl Into<String>) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/String;)Z");
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(arg0.into())?,
        ));
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setTitle",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn has_title(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasTitle", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn has_author(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasAuthor", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn has_generation(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "hasGeneration", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn generation(
        &self,
    ) -> Result<Option<crate::inventory::meta::BookMetaGeneration<'mc>>, Box<dyn std::error::Error>>
    {
        let sig = String::from("()Lorg/bukkit/inventory/meta/BookMeta$Generation;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getGeneration", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {
            return Ok(None);
        }
        Ok(Some(crate::inventory::meta::BookMetaGeneration::from_raw(
            &self.jni_ref(),
            unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) },
        )?))
    }

    pub fn set_generation(
        &self,
        arg0: impl Into<crate::inventory::meta::BookMetaGeneration<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/inventory/meta/BookMeta$Generation;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setGeneration",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn has_pages(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasPages", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn page_count(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getPageCount", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }

    pub fn clone(
        &self,
    ) -> Result<crate::inventory::meta::BookMeta<'mc>, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Lorg/bukkit/inventory/meta/BookMeta;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "clone", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::meta::BookMeta::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    pub fn display_name(&self) -> Result<String, Box<dyn std::error::Error>> {
        let temp_clone = BookMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.display_name()
    }
    pub fn as_string(&self) -> Result<String, Box<dyn std::error::Error>> {
        let temp_clone = BookMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.as_string()
    }
    pub fn has_display_name(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = BookMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.has_display_name()
    }
    pub fn set_display_name(
        &self,
        arg0: impl Into<String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = BookMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.set_display_name(arg0)
    }
    pub fn has_localized_name(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = BookMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.has_localized_name()
    }
    pub fn localized_name(&self) -> Result<String, Box<dyn std::error::Error>> {
        let temp_clone = BookMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.localized_name()
    }
    pub fn set_localized_name(
        &self,
        arg0: impl Into<String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = BookMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.set_localized_name(arg0)
    }
    pub fn has_lore(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = BookMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.has_lore()
    }
    pub fn lore(&self) -> Result<Option<Vec<String>>, Box<dyn std::error::Error>> {
        let temp_clone = BookMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.lore()
    }
    pub fn set_lore(&self, arg0: Vec<impl Into<String>>) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = BookMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.set_lore(arg0)
    }
    pub fn has_custom_model_data(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = BookMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.has_custom_model_data()
    }
    pub fn custom_model_data(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let temp_clone = BookMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.custom_model_data()
    }
    pub fn set_custom_model_data(&self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = BookMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.set_custom_model_data(arg0)
    }
    pub fn has_enchants(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "hasEnchants", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    pub fn has_enchant(
        &self,
        arg0: impl Into<crate::enchantments::Enchantment<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = BookMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.has_enchant(arg0)
    }
    pub fn get_enchant_level(
        &self,
        arg0: impl Into<crate::enchantments::Enchantment<'mc>>,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        let temp_clone = BookMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.get_enchant_level(arg0)
    }
    pub fn enchants(
        &self,
    ) -> Result<blackboxmc_java::util::JavaMap<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/Map;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getEnchants", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::util::JavaMap::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    pub fn add_enchant(
        &self,
        arg0: impl Into<crate::enchantments::Enchantment<'mc>>,
        arg1: i32,
        arg2: bool,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = BookMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.add_enchant(arg0, arg1, arg2)
    }
    pub fn remove_enchant(
        &self,
        arg0: impl Into<crate::enchantments::Enchantment<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = BookMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.remove_enchant(arg0)
    }
    pub fn has_conflicting_enchant(
        &self,
        arg0: impl Into<crate::enchantments::Enchantment<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = BookMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.has_conflicting_enchant(arg0)
    }
    pub fn add_item_flags(
        &self,
        arg0: Vec<crate::inventory::ItemFlag<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("([Lorg/bukkit/inventory/ItemFlag;)V");
        let arr = self.jni_ref().new_object_array(
            arg0.len() as i32,
            "org/bukkit/inventory/ItemFlag",
            jni::objects::JObject::null(),
        );
        let arr = self.jni_ref().translate_error_no_gen(arr)?;
        for i in 0..arg0.len() {
            let val_1 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(arg0.get(i).unwrap().jni_object().clone())
            });
            self.jni_ref()
                .set_object_array_element(&arr, i as i32, val_1.l()?)?;
        }
        let val_1 = jni::objects::JValueGen::Object(arr);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "addItemFlags",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1.l()?)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub fn remove_item_flags(
        &self,
        arg0: Vec<crate::inventory::ItemFlag<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("([Lorg/bukkit/inventory/ItemFlag;)V");
        let arr = self.jni_ref().new_object_array(
            arg0.len() as i32,
            "org/bukkit/inventory/ItemFlag",
            jni::objects::JObject::null(),
        );
        let arr = self.jni_ref().translate_error_no_gen(arr)?;
        for i in 0..arg0.len() {
            let val_1 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(arg0.get(i).unwrap().jni_object().clone())
            });
            self.jni_ref()
                .set_object_array_element(&arr, i as i32, val_1.l()?)?;
        }
        let val_1 = jni::objects::JValueGen::Object(arr);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeItemFlags",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1.l()?)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub fn item_flags(
        &self,
    ) -> Result<blackboxmc_java::util::JavaSet<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/Set;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getItemFlags", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::util::JavaSet::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    pub fn has_item_flag(
        &self,
        arg0: impl Into<crate::inventory::ItemFlag<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = BookMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.has_item_flag(arg0)
    }
    pub fn is_unbreakable(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = BookMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.is_unbreakable()
    }
    pub fn set_unbreakable(&self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = BookMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.set_unbreakable(arg0)
    }
    pub fn has_attribute_modifiers(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "hasAttributeModifiers",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    pub fn attribute_modifiers(
        &self,
    ) -> Result<Option<jni::objects::JObject<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lcom/google/common/collect/Multimap;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getAttributeModifiers",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {
            return Ok(None);
        }
        Ok(Some(res.l()?))
    }
    pub fn add_attribute_modifier(
        &self,
        arg0: impl Into<crate::attribute::Attribute<'mc>>,
        arg1: impl Into<crate::attribute::AttributeModifier<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = BookMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.add_attribute_modifier(arg0, arg1)
    }
    pub fn set_attribute_modifiers(
        &self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lcom/google/common/collect/Multimap;)V");
        let val_1 = jni::objects::JValueGen::Object(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setAttributeModifiers",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub fn remove_attribute_modifier_with_equipment_slot(
        &self,
        arg0: impl Into<crate::inventory::EquipmentSlot<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/inventory/EquipmentSlot;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        sig += ")Z";
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeAttributeModifier",
            sig.as_str(),
            args,
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    pub fn remove_attribute_modifier_with_attribute(
        &self,
        arg0: impl Into<crate::attribute::Attribute<'mc>>,
        arg1: std::option::Option<impl Into<crate::attribute::AttributeModifier<'mc>>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/attribute/Attribute;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        if let Some(a) = arg1 {
            sig += "Lorg/bukkit/attribute/AttributeModifier;";
            let val_2 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(a.into().jni_object().clone())
            });
            args.push(val_2);
        }
        sig += ")Z";
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeAttributeModifier",
            sig.as_str(),
            args,
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn custom_tag_container(
        &self,
    ) -> Result<crate::inventory::meta::tags::CustomItemTagContainer<'mc>, Box<dyn std::error::Error>>
    {
        let temp_clone = BookMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.custom_tag_container()
    }

    pub fn set_version(&self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = BookMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.set_version(arg0)
    }
    // SUPER CLASS: Cloneable
    // SUPER CLASS: ConfigurationSerializable
    pub fn serialize(
        &self,
    ) -> Result<blackboxmc_java::util::JavaMap<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::configuration::serialization::ConfigurationSerializable::from_raw(
            &self.0,
            unsafe { jni::objects::JObject::from_raw(self.1.clone()) },
        )?;
        let real: crate::configuration::serialization::ConfigurationSerializable =
            temp_clone.into();
        real.serialize()
    }
    // SUPER CLASS: PersistentDataHolder
    pub fn persistent_data_container(
        &self,
    ) -> Result<crate::persistence::PersistentDataContainer<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::persistence::PersistentDataHolder::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::persistence::PersistentDataHolder = temp_clone.into();
        real.persistent_data_container()
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::inventory::meta::ItemMeta<'mc>> for BookMeta<'mc> {
    fn into(self) -> crate::inventory::meta::ItemMeta<'mc> {
        crate::inventory::meta::ItemMeta::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting BookMeta into crate::inventory::meta::ItemMeta")
    }
}

///
/// This is a representation of an abstract class.
#[repr(C)]
pub struct MusicInstrumentMeta<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for MusicInstrumentMeta<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for MusicInstrumentMeta<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate MusicInstrumentMeta from null object.").into(),
            );
        }
        let (valid, name) =
            env.validate_name(&obj, "org/bukkit/inventory/meta/MusicInstrumentMeta")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a MusicInstrumentMeta object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> MusicInstrumentMeta<'mc> {
    pub fn instrument(
        &self,
    ) -> Result<Option<crate::MusicInstrument<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/MusicInstrument;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getInstrument", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {
            return Ok(None);
        }
        Ok(Some(crate::MusicInstrument::from_raw(
            &self.jni_ref(),
            unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) },
        )?))
    }

    pub fn set_instrument(
        &self,
        arg0: impl Into<crate::MusicInstrument<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/MusicInstrument;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setInstrument",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn clone(
        &self,
    ) -> Result<crate::inventory::meta::MusicInstrumentMeta<'mc>, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Lorg/bukkit/inventory/meta/MusicInstrumentMeta;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "clone", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::meta::MusicInstrumentMeta::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    pub fn display_name(&self) -> Result<String, Box<dyn std::error::Error>> {
        let temp_clone = MusicInstrumentMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.display_name()
    }
    pub fn as_string(&self) -> Result<String, Box<dyn std::error::Error>> {
        let temp_clone = MusicInstrumentMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.as_string()
    }
    pub fn has_display_name(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = MusicInstrumentMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.has_display_name()
    }
    pub fn set_display_name(
        &self,
        arg0: impl Into<String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = MusicInstrumentMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.set_display_name(arg0)
    }
    pub fn has_localized_name(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = MusicInstrumentMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.has_localized_name()
    }
    pub fn localized_name(&self) -> Result<String, Box<dyn std::error::Error>> {
        let temp_clone = MusicInstrumentMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.localized_name()
    }
    pub fn set_localized_name(
        &self,
        arg0: impl Into<String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = MusicInstrumentMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.set_localized_name(arg0)
    }
    pub fn has_lore(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = MusicInstrumentMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.has_lore()
    }
    pub fn lore(&self) -> Result<Option<Vec<String>>, Box<dyn std::error::Error>> {
        let temp_clone = MusicInstrumentMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.lore()
    }
    pub fn set_lore(&self, arg0: Vec<impl Into<String>>) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = MusicInstrumentMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.set_lore(arg0)
    }
    pub fn has_custom_model_data(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = MusicInstrumentMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.has_custom_model_data()
    }
    pub fn custom_model_data(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let temp_clone = MusicInstrumentMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.custom_model_data()
    }
    pub fn set_custom_model_data(&self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = MusicInstrumentMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.set_custom_model_data(arg0)
    }
    pub fn has_enchants(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "hasEnchants", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    pub fn has_enchant(
        &self,
        arg0: impl Into<crate::enchantments::Enchantment<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = MusicInstrumentMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.has_enchant(arg0)
    }
    pub fn get_enchant_level(
        &self,
        arg0: impl Into<crate::enchantments::Enchantment<'mc>>,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        let temp_clone = MusicInstrumentMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.get_enchant_level(arg0)
    }
    pub fn enchants(
        &self,
    ) -> Result<blackboxmc_java::util::JavaMap<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/Map;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getEnchants", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::util::JavaMap::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    pub fn add_enchant(
        &self,
        arg0: impl Into<crate::enchantments::Enchantment<'mc>>,
        arg1: i32,
        arg2: bool,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = MusicInstrumentMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.add_enchant(arg0, arg1, arg2)
    }
    pub fn remove_enchant(
        &self,
        arg0: impl Into<crate::enchantments::Enchantment<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = MusicInstrumentMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.remove_enchant(arg0)
    }
    pub fn has_conflicting_enchant(
        &self,
        arg0: impl Into<crate::enchantments::Enchantment<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = MusicInstrumentMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.has_conflicting_enchant(arg0)
    }
    pub fn add_item_flags(
        &self,
        arg0: Vec<crate::inventory::ItemFlag<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("([Lorg/bukkit/inventory/ItemFlag;)V");
        let arr = self.jni_ref().new_object_array(
            arg0.len() as i32,
            "org/bukkit/inventory/ItemFlag",
            jni::objects::JObject::null(),
        );
        let arr = self.jni_ref().translate_error_no_gen(arr)?;
        for i in 0..arg0.len() {
            let val_1 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(arg0.get(i).unwrap().jni_object().clone())
            });
            self.jni_ref()
                .set_object_array_element(&arr, i as i32, val_1.l()?)?;
        }
        let val_1 = jni::objects::JValueGen::Object(arr);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "addItemFlags",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1.l()?)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub fn remove_item_flags(
        &self,
        arg0: Vec<crate::inventory::ItemFlag<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("([Lorg/bukkit/inventory/ItemFlag;)V");
        let arr = self.jni_ref().new_object_array(
            arg0.len() as i32,
            "org/bukkit/inventory/ItemFlag",
            jni::objects::JObject::null(),
        );
        let arr = self.jni_ref().translate_error_no_gen(arr)?;
        for i in 0..arg0.len() {
            let val_1 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(arg0.get(i).unwrap().jni_object().clone())
            });
            self.jni_ref()
                .set_object_array_element(&arr, i as i32, val_1.l()?)?;
        }
        let val_1 = jni::objects::JValueGen::Object(arr);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeItemFlags",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1.l()?)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub fn item_flags(
        &self,
    ) -> Result<blackboxmc_java::util::JavaSet<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/Set;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getItemFlags", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::util::JavaSet::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    pub fn has_item_flag(
        &self,
        arg0: impl Into<crate::inventory::ItemFlag<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = MusicInstrumentMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.has_item_flag(arg0)
    }
    pub fn is_unbreakable(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = MusicInstrumentMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.is_unbreakable()
    }
    pub fn set_unbreakable(&self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = MusicInstrumentMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.set_unbreakable(arg0)
    }
    pub fn has_attribute_modifiers(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "hasAttributeModifiers",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    pub fn attribute_modifiers(
        &self,
    ) -> Result<Option<jni::objects::JObject<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lcom/google/common/collect/Multimap;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getAttributeModifiers",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {
            return Ok(None);
        }
        Ok(Some(res.l()?))
    }
    pub fn add_attribute_modifier(
        &self,
        arg0: impl Into<crate::attribute::Attribute<'mc>>,
        arg1: impl Into<crate::attribute::AttributeModifier<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = MusicInstrumentMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.add_attribute_modifier(arg0, arg1)
    }
    pub fn set_attribute_modifiers(
        &self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lcom/google/common/collect/Multimap;)V");
        let val_1 = jni::objects::JValueGen::Object(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setAttributeModifiers",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub fn remove_attribute_modifier_with_equipment_slot(
        &self,
        arg0: impl Into<crate::inventory::EquipmentSlot<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/inventory/EquipmentSlot;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        sig += ")Z";
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeAttributeModifier",
            sig.as_str(),
            args,
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    pub fn remove_attribute_modifier_with_attribute(
        &self,
        arg0: impl Into<crate::attribute::Attribute<'mc>>,
        arg1: std::option::Option<impl Into<crate::attribute::AttributeModifier<'mc>>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/attribute/Attribute;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        if let Some(a) = arg1 {
            sig += "Lorg/bukkit/attribute/AttributeModifier;";
            let val_2 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(a.into().jni_object().clone())
            });
            args.push(val_2);
        }
        sig += ")Z";
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeAttributeModifier",
            sig.as_str(),
            args,
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn custom_tag_container(
        &self,
    ) -> Result<crate::inventory::meta::tags::CustomItemTagContainer<'mc>, Box<dyn std::error::Error>>
    {
        let temp_clone = MusicInstrumentMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.custom_tag_container()
    }

    pub fn set_version(&self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = MusicInstrumentMeta::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::meta::ItemMeta = temp_clone.into();
        real.set_version(arg0)
    }
    // SUPER CLASS: Cloneable
    // SUPER CLASS: ConfigurationSerializable
    pub fn serialize(
        &self,
    ) -> Result<blackboxmc_java::util::JavaMap<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::configuration::serialization::ConfigurationSerializable::from_raw(
            &self.0,
            unsafe { jni::objects::JObject::from_raw(self.1.clone()) },
        )?;
        let real: crate::configuration::serialization::ConfigurationSerializable =
            temp_clone.into();
        real.serialize()
    }
    // SUPER CLASS: PersistentDataHolder
    pub fn persistent_data_container(
        &self,
    ) -> Result<crate::persistence::PersistentDataContainer<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::persistence::PersistentDataHolder::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::persistence::PersistentDataHolder = temp_clone.into();
        real.persistent_data_container()
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::inventory::meta::ItemMeta<'mc>> for MusicInstrumentMeta<'mc> {
    fn into(self) -> crate::inventory::meta::ItemMeta<'mc> {
        crate::inventory::meta::ItemMeta::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting MusicInstrumentMeta into crate::inventory::meta::ItemMeta")
    }
}
pub mod tags;
pub mod trim;
