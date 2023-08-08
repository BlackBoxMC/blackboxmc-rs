#![allow(deprecated)]
use blackboxmc_general::JNIRaw;
use color_eyre::eyre::Result;
/// Represents the generation (or level of copying) of a written book
pub struct BookMetaGeneration<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for BookMetaGeneration<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> BookMetaGeneration<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate BookMetaGeneration from null object.").into(),
            );
        }
        let (valid, name) =
            env.validate_name(&obj, "org/bukkit/inventory/meta/BookMetaGeneration")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a BookMetaGeneration object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    /// Returns the enum constant of this type with the specified name. The string must match <i>exactly</i> an identifier used to declare an enum constant in this type. (Extraneous whitespace characters are not permitted.)
    pub fn value_of_with_string(
        jni: blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: std::option::Option<jni::objects::JClass<'mc>>,
        arg1: std::option::Option<impl Into<&'mc String>>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let val_1 = arg0.unwrap();
        let val_2 = jni::objects::JObject::from(jni.new_string(arg1.unwrap().into()).unwrap());
        let cls = &jni.find_class("java/lang/Enum")?;
        let res = jni.call_static_method(
            cls,
            "valueOf",
            "(Ljava/lang/Class;Ljava/lang/String;)Ljava/lang/Enum;",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        )?;
        let obj = res.l()?;
        Self::from_raw(&jni, obj)
    }

    pub fn name(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "name", "()Ljava/lang/String;", &[]);
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

    pub fn describe_constable(
        &mut self,
    ) -> Result<blackboxmc_java::JavaOptional<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "describeConstable",
            "()Ljava/util/Optional;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::JavaOptional::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn declaring_class(
        &mut self,
    ) -> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getDeclaringClass",
            "()Ljava/lang/Class;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(unsafe { jni::objects::JClass::from_raw(res.as_jni().l) })
    }

    pub fn ordinal(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "ordinal", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
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

///
/// This is a representation of an abstract class.
pub struct BlockDataMeta<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> BlockDataMeta<'mc> {
    pub fn from_raw(
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
    /// Returns the currently attached block data for this item or creates a new one if one doesn't exist. The state is a copy, it must be set back (or to another item) with <a href="#setBlockData(org.bukkit.block.data.BlockData)"><code>setBlockData(org.bukkit.block.data.BlockData)</code></a>
    pub fn get_block_data(
        &mut self,
        arg0: impl Into<&'mc crate::Material<'mc>>,
    ) -> Result<crate::block::data::BlockData<'mc>, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getBlockData",
            "(Lorg/bukkit/Material;)Lorg/bukkit/block/data/BlockData;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::block::data::BlockData::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Attaches a copy of the passed block data to the item.
    pub fn set_block_data(
        &mut self,
        arg0: impl Into<&'mc crate::block::data::BlockData<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setBlockData",
            "(Lorg/bukkit/block/data/BlockData;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Returns whether the item has block data currently attached to it.
    pub fn has_block_data(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasBlockData", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn display_name(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getDisplayName",
            "()Ljava/lang/String;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }

    pub fn localized_name(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getLocalizedName",
            "()Ljava/lang/String;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }

    pub fn get_enchant_level(
        &mut self,
        arg0: impl Into<&'mc crate::enchantments::Enchantment<'mc>>,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getEnchantLevel",
            "(Lorg/bukkit/enchantments/Enchantment;)I",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }

    pub fn set_lore(
        &mut self,
        arg0: Vec<impl Into<String>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let raw_val_1 = self
            .jni_ref()
            .new_object("java/util/ArrayList", "()V", &[])
            .unwrap();
        for v in arg0 {
            let map_val_0 =
                jni::objects::JObject::from(self.jni_ref().new_string(v.into()).unwrap());
            self.jni_ref().call_method(
                &raw_val_1,
                "add",
                "(Ljava/Lang/Object)V",
                &[jni::objects::JValueGen::from(&map_val_0)],
            )?;
        }
        let val_1 = jni::objects::JValueGen::Object(raw_val_1);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setLore",
            "(Ljava/util/List;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn as_string(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getAsString",
            "()Ljava/lang/String;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }

    pub fn has_display_name(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasDisplayName", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn set_display_name(
        &mut self,
        arg0: impl Into<&'mc String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setDisplayName",
            "(Ljava/lang/String;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn has_localized_name(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasLocalizedName", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn set_localized_name(
        &mut self,
        arg0: impl Into<&'mc String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setLocalizedName",
            "(Ljava/lang/String;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn has_lore(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasLore", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn lore(&mut self) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getLore", "()Ljava/util/List;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let mut list = blackboxmc_java::JavaList::from_raw(&self.jni_ref(), res.l()?)?;
        let size = list.size()?;
        for i in 0..=size {
            let obj = list.get(i)?;
            new_vec.push(
                self.jni_ref()
                    .get_string(unsafe { &jni::objects::JString::from_raw(*obj) })?
                    .to_string_lossy()
                    .to_string(),
            );
        }
        Ok(new_vec)
    }

    pub fn has_custom_model_data(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasCustomModelData", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn custom_model_data(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getCustomModelData", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }

    pub fn set_custom_model_data(&mut self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setCustomModelData",
            "(I)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn has_enchants(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasEnchants", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn has_enchant(
        &mut self,
        arg0: impl Into<&'mc crate::enchantments::Enchantment<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "hasEnchant",
            "(Lorg/bukkit/enchantments/Enchantment;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn enchants(
        &mut self,
    ) -> Result<blackboxmc_java::JavaMap<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getEnchants", "()Ljava/util/Map;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::JavaMap::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn add_enchant(
        &mut self,
        arg0: impl Into<&'mc crate::enchantments::Enchantment<'mc>>,
        arg1: i32,
        arg2: bool,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let val_2 = jni::objects::JValueGen::Int(arg1.into());
        // -2
        let val_3 = jni::objects::JValueGen::Bool(arg2.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "addEnchant",
            "(Lorg/bukkit/enchantments/Enchantment;IZ)Z",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
                jni::objects::JValueGen::from(&val_3),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn remove_enchant(
        &mut self,
        arg0: impl Into<&'mc crate::enchantments::Enchantment<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeEnchant",
            "(Lorg/bukkit/enchantments/Enchantment;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn has_conflicting_enchant(
        &mut self,
        arg0: impl Into<&'mc crate::enchantments::Enchantment<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "hasConflictingEnchant",
            "(Lorg/bukkit/enchantments/Enchantment;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn add_item_flags(
        &mut self,
        arg0: Vec<impl Into<crate::inventory::ItemFlag<'mc>>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "addItemFlags",
            "(Lorg/bukkit/inventory/ItemFlag;)V",
            &[],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn remove_item_flags(
        &mut self,
        arg0: Vec<impl Into<crate::inventory::ItemFlag<'mc>>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeItemFlags",
            "(Lorg/bukkit/inventory/ItemFlag;)V",
            &[],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn item_flags(
        &mut self,
    ) -> Result<blackboxmc_java::JavaSet<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getItemFlags",
            "()Ljava/util/Set;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::JavaSet::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn has_item_flag(
        &mut self,
        arg0: impl Into<&'mc crate::inventory::ItemFlag<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "hasItemFlag",
            "(Lorg/bukkit/inventory/ItemFlag;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn is_unbreakable(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isUnbreakable", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn set_unbreakable(&mut self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        // -2
        let val_1 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setUnbreakable",
            "(Z)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn has_attribute_modifiers(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "hasAttributeModifiers", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn attribute_modifiers(
        &mut self,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getAttributeModifiers",
            "()Lcom/google/common/collect/Multimap;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }

    pub fn add_attribute_modifier(
        &mut self,
        arg0: impl Into<&'mc crate::attribute::Attribute<'mc>>,
        arg1: impl Into<&'mc crate::attribute::AttributeModifier<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let val_2 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "addAttributeModifier",
            "(Lorg/bukkit/attribute/Attribute;Lorg/bukkit/attribute/AttributeModifier;)Z",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub unsafe fn set_attribute_modifiers(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setAttributeModifiers",
            "(Lcom/google/common/collect/Multimap;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn remove_attribute_modifier_with_attribute(
        &mut self,
        arg0: impl Into<&'mc crate::attribute::Attribute<'mc>>,
        arg1: std::option::Option<impl Into<&'mc crate::attribute::AttributeModifier<'mc>>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let val_2 =
            unsafe { jni::objects::JObject::from_raw(arg1.unwrap().into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeAttributeModifier",
            "(Lorg/bukkit/attribute/Attribute;Lorg/bukkit/attribute/AttributeModifier;)Z",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    #[deprecated]

    pub fn custom_tag_container(
        &mut self,
    ) -> Result<crate::inventory::meta::tags::CustomItemTagContainer<'mc>, Box<dyn std::error::Error>>
    {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getCustomTagContainer",
            "()Lorg/bukkit/inventory/meta/tags/CustomItemTagContainer;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::meta::tags::CustomItemTagContainer::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    #[deprecated]

    pub fn set_version(&mut self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setVersion",
            "(I)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn clone(&mut self) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "clone", "()Ljava/lang/Object;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }

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

    pub fn persistent_data_container(
        &mut self,
    ) -> Result<crate::persistence::PersistentDataContainer<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getPersistentDataContainer",
            "()Lorg/bukkit/persistence/PersistentDataContainer;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::persistence::PersistentDataContainer::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
}
impl<'mc> JNIRaw<'mc> for BlockDataMeta<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Into<crate::inventory::meta::ItemMeta<'mc>> for BlockDataMeta<'mc> {
    fn into(self) -> crate::inventory::meta::ItemMeta<'mc> {
        crate::inventory::meta::ItemMeta::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
/// Represents armor that an entity can equip.
/// <p><strong>Note: Armor trims are part of an experimental feature of Minecraft and hence subject to change.</strong></p>
///
/// This is a representation of an abstract class.
pub struct ArmorMeta<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> ArmorMeta<'mc> {
    pub fn from_raw(
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
    /// Check whether or not this item has an armor trim.
    pub fn has_trim(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasTrim", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    /// Set the <a href="trim/ArmorTrim.html" title="class in org.bukkit.inventory.meta.trim"><code>ArmorTrim</code></a>.
    pub fn set_trim(
        &mut self,
        arg0: impl Into<&'mc crate::inventory::meta::trim::ArmorTrim<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setTrim",
            "(Lorg/bukkit/inventory/meta/trim/ArmorTrim;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Get the <a title="class in org.bukkit.inventory.meta.trim" href="trim/ArmorTrim.html"><code>ArmorTrim</code></a>.
    pub fn trim(
        &mut self,
    ) -> Result<crate::inventory::meta::trim::ArmorTrim<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getTrim",
            "()Lorg/bukkit/inventory/meta/trim/ArmorTrim;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::meta::trim::ArmorTrim::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn clone(
        &mut self,
    ) -> Result<crate::inventory::meta::ArmorMeta<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "clone",
            "()Lorg/bukkit/inventory/meta/ArmorMeta;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::meta::ArmorMeta::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn display_name(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getDisplayName",
            "()Ljava/lang/String;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }

    pub fn localized_name(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getLocalizedName",
            "()Ljava/lang/String;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }

    pub fn get_enchant_level(
        &mut self,
        arg0: impl Into<&'mc crate::enchantments::Enchantment<'mc>>,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getEnchantLevel",
            "(Lorg/bukkit/enchantments/Enchantment;)I",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }

    pub fn set_lore(
        &mut self,
        arg0: Vec<impl Into<String>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let raw_val_1 = self
            .jni_ref()
            .new_object("java/util/ArrayList", "()V", &[])
            .unwrap();
        for v in arg0 {
            let map_val_0 =
                jni::objects::JObject::from(self.jni_ref().new_string(v.into()).unwrap());
            self.jni_ref().call_method(
                &raw_val_1,
                "add",
                "(Ljava/Lang/Object)V",
                &[jni::objects::JValueGen::from(&map_val_0)],
            )?;
        }
        let val_1 = jni::objects::JValueGen::Object(raw_val_1);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setLore",
            "(Ljava/util/List;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn as_string(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getAsString",
            "()Ljava/lang/String;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }

    pub fn has_display_name(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasDisplayName", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn set_display_name(
        &mut self,
        arg0: impl Into<&'mc String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setDisplayName",
            "(Ljava/lang/String;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn has_localized_name(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasLocalizedName", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn set_localized_name(
        &mut self,
        arg0: impl Into<&'mc String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setLocalizedName",
            "(Ljava/lang/String;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn has_lore(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasLore", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn lore(&mut self) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getLore", "()Ljava/util/List;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let mut list = blackboxmc_java::JavaList::from_raw(&self.jni_ref(), res.l()?)?;
        let size = list.size()?;
        for i in 0..=size {
            let obj = list.get(i)?;
            new_vec.push(
                self.jni_ref()
                    .get_string(unsafe { &jni::objects::JString::from_raw(*obj) })?
                    .to_string_lossy()
                    .to_string(),
            );
        }
        Ok(new_vec)
    }

    pub fn has_custom_model_data(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasCustomModelData", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn custom_model_data(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getCustomModelData", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }

    pub fn set_custom_model_data(&mut self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setCustomModelData",
            "(I)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn has_enchants(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasEnchants", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn has_enchant(
        &mut self,
        arg0: impl Into<&'mc crate::enchantments::Enchantment<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "hasEnchant",
            "(Lorg/bukkit/enchantments/Enchantment;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn enchants(
        &mut self,
    ) -> Result<blackboxmc_java::JavaMap<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getEnchants", "()Ljava/util/Map;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::JavaMap::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn add_enchant(
        &mut self,
        arg0: impl Into<&'mc crate::enchantments::Enchantment<'mc>>,
        arg1: i32,
        arg2: bool,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let val_2 = jni::objects::JValueGen::Int(arg1.into());
        // -2
        let val_3 = jni::objects::JValueGen::Bool(arg2.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "addEnchant",
            "(Lorg/bukkit/enchantments/Enchantment;IZ)Z",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
                jni::objects::JValueGen::from(&val_3),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn remove_enchant(
        &mut self,
        arg0: impl Into<&'mc crate::enchantments::Enchantment<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeEnchant",
            "(Lorg/bukkit/enchantments/Enchantment;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn has_conflicting_enchant(
        &mut self,
        arg0: impl Into<&'mc crate::enchantments::Enchantment<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "hasConflictingEnchant",
            "(Lorg/bukkit/enchantments/Enchantment;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn add_item_flags(
        &mut self,
        arg0: Vec<impl Into<crate::inventory::ItemFlag<'mc>>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "addItemFlags",
            "(Lorg/bukkit/inventory/ItemFlag;)V",
            &[],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn remove_item_flags(
        &mut self,
        arg0: Vec<impl Into<crate::inventory::ItemFlag<'mc>>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeItemFlags",
            "(Lorg/bukkit/inventory/ItemFlag;)V",
            &[],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn item_flags(
        &mut self,
    ) -> Result<blackboxmc_java::JavaSet<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getItemFlags",
            "()Ljava/util/Set;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::JavaSet::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn has_item_flag(
        &mut self,
        arg0: impl Into<&'mc crate::inventory::ItemFlag<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "hasItemFlag",
            "(Lorg/bukkit/inventory/ItemFlag;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn is_unbreakable(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isUnbreakable", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn set_unbreakable(&mut self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        // -2
        let val_1 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setUnbreakable",
            "(Z)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn has_attribute_modifiers(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "hasAttributeModifiers", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn attribute_modifiers(
        &mut self,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getAttributeModifiers",
            "()Lcom/google/common/collect/Multimap;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }

    pub fn add_attribute_modifier(
        &mut self,
        arg0: impl Into<&'mc crate::attribute::Attribute<'mc>>,
        arg1: impl Into<&'mc crate::attribute::AttributeModifier<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let val_2 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "addAttributeModifier",
            "(Lorg/bukkit/attribute/Attribute;Lorg/bukkit/attribute/AttributeModifier;)Z",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub unsafe fn set_attribute_modifiers(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setAttributeModifiers",
            "(Lcom/google/common/collect/Multimap;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn remove_attribute_modifier_with_attribute(
        &mut self,
        arg0: impl Into<&'mc crate::attribute::Attribute<'mc>>,
        arg1: std::option::Option<impl Into<&'mc crate::attribute::AttributeModifier<'mc>>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let val_2 =
            unsafe { jni::objects::JObject::from_raw(arg1.unwrap().into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeAttributeModifier",
            "(Lorg/bukkit/attribute/Attribute;Lorg/bukkit/attribute/AttributeModifier;)Z",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    #[deprecated]

    pub fn custom_tag_container(
        &mut self,
    ) -> Result<crate::inventory::meta::tags::CustomItemTagContainer<'mc>, Box<dyn std::error::Error>>
    {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getCustomTagContainer",
            "()Lorg/bukkit/inventory/meta/tags/CustomItemTagContainer;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::meta::tags::CustomItemTagContainer::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    #[deprecated]

    pub fn set_version(&mut self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setVersion",
            "(I)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

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

    pub fn persistent_data_container(
        &mut self,
    ) -> Result<crate::persistence::PersistentDataContainer<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getPersistentDataContainer",
            "()Lorg/bukkit/persistence/PersistentDataContainer;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::persistence::PersistentDataContainer::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
}
impl<'mc> JNIRaw<'mc> for ArmorMeta<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Into<crate::inventory::meta::ItemMeta<'mc>> for ArmorMeta<'mc> {
    fn into(self) -> crate::inventory::meta::ItemMeta<'mc> {
        crate::inventory::meta::ItemMeta::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
/// Represents a bucket of axolotl.
///
/// This is a representation of an abstract class.
pub struct AxolotlBucketMeta<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> AxolotlBucketMeta<'mc> {
    pub fn from_raw(
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
    /// Get the variant of the axolotl in the bucket.
    /// <p>Plugins should check that hasVariant() returns <code>true</code> before calling this method.</p>
    pub fn variant(
        &mut self,
    ) -> Result<crate::entity::AxolotlVariant<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getVariant",
            "()Lorg/bukkit/entity/Axolotl$Variant;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::entity::AxolotlVariant::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Set the variant of this axolotl in the bucket.
    pub fn set_variant(
        &mut self,
        arg0: impl Into<&'mc crate::entity::AxolotlVariant<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setVariant",
            "(Lorg/bukkit/entity/Axolotl$Variant;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Checks for existence of a variant tag indicating a specific axolotl will be spawned.
    pub fn has_variant(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasVariant", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn clone(
        &mut self,
    ) -> Result<crate::inventory::meta::AxolotlBucketMeta<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "clone",
            "()Lorg/bukkit/inventory/meta/AxolotlBucketMeta;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::meta::AxolotlBucketMeta::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn display_name(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getDisplayName",
            "()Ljava/lang/String;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }

    pub fn localized_name(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getLocalizedName",
            "()Ljava/lang/String;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }

    pub fn get_enchant_level(
        &mut self,
        arg0: impl Into<&'mc crate::enchantments::Enchantment<'mc>>,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getEnchantLevel",
            "(Lorg/bukkit/enchantments/Enchantment;)I",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }

    pub fn set_lore(
        &mut self,
        arg0: Vec<impl Into<String>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let raw_val_1 = self
            .jni_ref()
            .new_object("java/util/ArrayList", "()V", &[])
            .unwrap();
        for v in arg0 {
            let map_val_0 =
                jni::objects::JObject::from(self.jni_ref().new_string(v.into()).unwrap());
            self.jni_ref().call_method(
                &raw_val_1,
                "add",
                "(Ljava/Lang/Object)V",
                &[jni::objects::JValueGen::from(&map_val_0)],
            )?;
        }
        let val_1 = jni::objects::JValueGen::Object(raw_val_1);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setLore",
            "(Ljava/util/List;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn as_string(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getAsString",
            "()Ljava/lang/String;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }

    pub fn has_display_name(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasDisplayName", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn set_display_name(
        &mut self,
        arg0: impl Into<&'mc String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setDisplayName",
            "(Ljava/lang/String;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn has_localized_name(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasLocalizedName", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn set_localized_name(
        &mut self,
        arg0: impl Into<&'mc String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setLocalizedName",
            "(Ljava/lang/String;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn has_lore(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasLore", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn lore(&mut self) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getLore", "()Ljava/util/List;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let mut list = blackboxmc_java::JavaList::from_raw(&self.jni_ref(), res.l()?)?;
        let size = list.size()?;
        for i in 0..=size {
            let obj = list.get(i)?;
            new_vec.push(
                self.jni_ref()
                    .get_string(unsafe { &jni::objects::JString::from_raw(*obj) })?
                    .to_string_lossy()
                    .to_string(),
            );
        }
        Ok(new_vec)
    }

    pub fn has_custom_model_data(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasCustomModelData", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn custom_model_data(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getCustomModelData", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }

    pub fn set_custom_model_data(&mut self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setCustomModelData",
            "(I)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn has_enchants(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasEnchants", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn has_enchant(
        &mut self,
        arg0: impl Into<&'mc crate::enchantments::Enchantment<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "hasEnchant",
            "(Lorg/bukkit/enchantments/Enchantment;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn enchants(
        &mut self,
    ) -> Result<blackboxmc_java::JavaMap<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getEnchants", "()Ljava/util/Map;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::JavaMap::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn add_enchant(
        &mut self,
        arg0: impl Into<&'mc crate::enchantments::Enchantment<'mc>>,
        arg1: i32,
        arg2: bool,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let val_2 = jni::objects::JValueGen::Int(arg1.into());
        // -2
        let val_3 = jni::objects::JValueGen::Bool(arg2.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "addEnchant",
            "(Lorg/bukkit/enchantments/Enchantment;IZ)Z",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
                jni::objects::JValueGen::from(&val_3),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn remove_enchant(
        &mut self,
        arg0: impl Into<&'mc crate::enchantments::Enchantment<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeEnchant",
            "(Lorg/bukkit/enchantments/Enchantment;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn has_conflicting_enchant(
        &mut self,
        arg0: impl Into<&'mc crate::enchantments::Enchantment<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "hasConflictingEnchant",
            "(Lorg/bukkit/enchantments/Enchantment;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn add_item_flags(
        &mut self,
        arg0: Vec<impl Into<crate::inventory::ItemFlag<'mc>>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "addItemFlags",
            "(Lorg/bukkit/inventory/ItemFlag;)V",
            &[],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn remove_item_flags(
        &mut self,
        arg0: Vec<impl Into<crate::inventory::ItemFlag<'mc>>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeItemFlags",
            "(Lorg/bukkit/inventory/ItemFlag;)V",
            &[],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn item_flags(
        &mut self,
    ) -> Result<blackboxmc_java::JavaSet<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getItemFlags",
            "()Ljava/util/Set;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::JavaSet::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn has_item_flag(
        &mut self,
        arg0: impl Into<&'mc crate::inventory::ItemFlag<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "hasItemFlag",
            "(Lorg/bukkit/inventory/ItemFlag;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn is_unbreakable(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isUnbreakable", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn set_unbreakable(&mut self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        // -2
        let val_1 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setUnbreakable",
            "(Z)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn has_attribute_modifiers(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "hasAttributeModifiers", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn attribute_modifiers(
        &mut self,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getAttributeModifiers",
            "()Lcom/google/common/collect/Multimap;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }

    pub fn add_attribute_modifier(
        &mut self,
        arg0: impl Into<&'mc crate::attribute::Attribute<'mc>>,
        arg1: impl Into<&'mc crate::attribute::AttributeModifier<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let val_2 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "addAttributeModifier",
            "(Lorg/bukkit/attribute/Attribute;Lorg/bukkit/attribute/AttributeModifier;)Z",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub unsafe fn set_attribute_modifiers(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setAttributeModifiers",
            "(Lcom/google/common/collect/Multimap;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn remove_attribute_modifier_with_attribute(
        &mut self,
        arg0: impl Into<&'mc crate::attribute::Attribute<'mc>>,
        arg1: std::option::Option<impl Into<&'mc crate::attribute::AttributeModifier<'mc>>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let val_2 =
            unsafe { jni::objects::JObject::from_raw(arg1.unwrap().into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeAttributeModifier",
            "(Lorg/bukkit/attribute/Attribute;Lorg/bukkit/attribute/AttributeModifier;)Z",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    #[deprecated]

    pub fn custom_tag_container(
        &mut self,
    ) -> Result<crate::inventory::meta::tags::CustomItemTagContainer<'mc>, Box<dyn std::error::Error>>
    {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getCustomTagContainer",
            "()Lorg/bukkit/inventory/meta/tags/CustomItemTagContainer;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::meta::tags::CustomItemTagContainer::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    #[deprecated]

    pub fn set_version(&mut self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setVersion",
            "(I)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

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

    pub fn persistent_data_container(
        &mut self,
    ) -> Result<crate::persistence::PersistentDataContainer<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getPersistentDataContainer",
            "()Lorg/bukkit/persistence/PersistentDataContainer;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::persistence::PersistentDataContainer::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
}
impl<'mc> JNIRaw<'mc> for AxolotlBucketMeta<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Into<crate::inventory::meta::ItemMeta<'mc>> for AxolotlBucketMeta<'mc> {
    fn into(self) -> crate::inventory::meta::ItemMeta<'mc> {
        crate::inventory::meta::ItemMeta::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
/// Represents a skull that can have an owner.
///
/// This is a representation of an abstract class.
pub struct SkullMeta<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> SkullMeta<'mc> {
    pub fn from_raw(
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
    #[deprecated]
    /// <span class="deprecated-label">Deprecated.</span>
    /// <div class="deprecation-comment">
    /// see <a href="#getOwningPlayer()"><code>getOwningPlayer()</code></a>.
    /// </div>
    /// see <a href="#getOwningPlayer()"><code>getOwningPlayer()</code></a>.
    ///
    /// Gets the owner of the skull.
    /// Gets the profile of the player who owns the skull. This player profile may appear as the texture depending on skull type.
    pub fn owner(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getOwner", "()Ljava/lang/String;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    #[deprecated]
    /// <span class="deprecated-label">Deprecated.</span>
    /// <div class="deprecation-comment">
    /// see <a href="#setOwningPlayer(org.bukkit.OfflinePlayer)"><code>setOwningPlayer(org.bukkit.OfflinePlayer)</code></a>.
    /// </div>
    /// see <a href="#setOwningPlayer(org.bukkit.OfflinePlayer)"><code>setOwningPlayer(org.bukkit.OfflinePlayer)</code></a>.
    ///
    /// Sets the owner of the skull.
    /// Sets the profile of the player who owns the skull. This player profile may appear as the texture depending on skull type.
    /// <p>The profile must contain both a unique id and a skin texture. If either of these is missing, the profile must contain a name by which the server will then attempt to look up the unique id and skin texture.</p>
    pub fn set_owner(
        &mut self,
        arg0: impl Into<&'mc String>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setOwner",
            "(Ljava/lang/String;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    /// Gets the owner of the skull.
    pub fn owning_player(
        &mut self,
    ) -> Result<crate::OfflinePlayer<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getOwningPlayer",
            "()Lorg/bukkit/OfflinePlayer;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::OfflinePlayer::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Sets the owner of the skull.
    /// <p>Plugins should check that hasOwner() returns true before calling this plugin.</p>
    pub fn set_owning_player(
        &mut self,
        arg0: impl Into<&'mc crate::OfflinePlayer<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setOwningPlayer",
            "(Lorg/bukkit/OfflinePlayer;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    /// Checks to see if the skull has an owner.
    pub fn has_owner(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasOwner", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    /// Gets the profile of the player who owns the skull. This player profile may appear as the texture depending on skull type.
    pub fn owner_profile(
        &mut self,
    ) -> Result<crate::profile::PlayerProfile<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getOwnerProfile",
            "()Lorg/bukkit/profile/PlayerProfile;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::profile::PlayerProfile::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Sets the profile of the player who owns the skull. This player profile may appear as the texture depending on skull type.
    /// <p>The profile must contain both a unique id and a skin texture. If either of these is missing, the profile must contain a name by which the server will then attempt to look up the unique id and skin texture.</p>
    pub fn set_owner_profile(
        &mut self,
        arg0: impl Into<&'mc crate::profile::PlayerProfile<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setOwnerProfile",
            "(Lorg/bukkit/profile/PlayerProfile;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Gets the sound to play if the skull is placed on a note block.
    ///
    /// <strong>Note:</strong> This only works for player heads. For other heads, see <a title="enum in org.bukkit" href="../../Instrument.html"><code>Instrument</code></a>.
    pub fn note_block_sound(
        &mut self,
    ) -> Result<crate::NamespacedKey<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getNoteBlockSound",
            "()Lorg/bukkit/NamespacedKey;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::NamespacedKey::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Sets the sound to play if the skull is placed on a note block.
    ///
    /// <strong>Note:</strong> This only works for player heads. For other heads, see <a href="../../Instrument.html" title="enum in org.bukkit"><code>Instrument</code></a>.
    pub fn set_note_block_sound(
        &mut self,
        arg0: impl Into<&'mc crate::NamespacedKey<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setNoteBlockSound",
            "(Lorg/bukkit/NamespacedKey;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn clone(
        &mut self,
    ) -> Result<crate::inventory::meta::SkullMeta<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "clone",
            "()Lorg/bukkit/inventory/meta/SkullMeta;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::meta::SkullMeta::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn display_name(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getDisplayName",
            "()Ljava/lang/String;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }

    pub fn localized_name(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getLocalizedName",
            "()Ljava/lang/String;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }

    pub fn get_enchant_level(
        &mut self,
        arg0: impl Into<&'mc crate::enchantments::Enchantment<'mc>>,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getEnchantLevel",
            "(Lorg/bukkit/enchantments/Enchantment;)I",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }

    pub fn set_lore(
        &mut self,
        arg0: Vec<impl Into<String>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let raw_val_1 = self
            .jni_ref()
            .new_object("java/util/ArrayList", "()V", &[])
            .unwrap();
        for v in arg0 {
            let map_val_0 =
                jni::objects::JObject::from(self.jni_ref().new_string(v.into()).unwrap());
            self.jni_ref().call_method(
                &raw_val_1,
                "add",
                "(Ljava/Lang/Object)V",
                &[jni::objects::JValueGen::from(&map_val_0)],
            )?;
        }
        let val_1 = jni::objects::JValueGen::Object(raw_val_1);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setLore",
            "(Ljava/util/List;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn as_string(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getAsString",
            "()Ljava/lang/String;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }

    pub fn has_display_name(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasDisplayName", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn set_display_name(
        &mut self,
        arg0: impl Into<&'mc String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setDisplayName",
            "(Ljava/lang/String;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn has_localized_name(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasLocalizedName", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn set_localized_name(
        &mut self,
        arg0: impl Into<&'mc String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setLocalizedName",
            "(Ljava/lang/String;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn has_lore(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasLore", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn lore(&mut self) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getLore", "()Ljava/util/List;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let mut list = blackboxmc_java::JavaList::from_raw(&self.jni_ref(), res.l()?)?;
        let size = list.size()?;
        for i in 0..=size {
            let obj = list.get(i)?;
            new_vec.push(
                self.jni_ref()
                    .get_string(unsafe { &jni::objects::JString::from_raw(*obj) })?
                    .to_string_lossy()
                    .to_string(),
            );
        }
        Ok(new_vec)
    }

    pub fn has_custom_model_data(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasCustomModelData", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn custom_model_data(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getCustomModelData", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }

    pub fn set_custom_model_data(&mut self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setCustomModelData",
            "(I)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn has_enchants(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasEnchants", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn has_enchant(
        &mut self,
        arg0: impl Into<&'mc crate::enchantments::Enchantment<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "hasEnchant",
            "(Lorg/bukkit/enchantments/Enchantment;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn enchants(
        &mut self,
    ) -> Result<blackboxmc_java::JavaMap<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getEnchants", "()Ljava/util/Map;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::JavaMap::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn add_enchant(
        &mut self,
        arg0: impl Into<&'mc crate::enchantments::Enchantment<'mc>>,
        arg1: i32,
        arg2: bool,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let val_2 = jni::objects::JValueGen::Int(arg1.into());
        // -2
        let val_3 = jni::objects::JValueGen::Bool(arg2.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "addEnchant",
            "(Lorg/bukkit/enchantments/Enchantment;IZ)Z",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
                jni::objects::JValueGen::from(&val_3),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn remove_enchant(
        &mut self,
        arg0: impl Into<&'mc crate::enchantments::Enchantment<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeEnchant",
            "(Lorg/bukkit/enchantments/Enchantment;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn has_conflicting_enchant(
        &mut self,
        arg0: impl Into<&'mc crate::enchantments::Enchantment<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "hasConflictingEnchant",
            "(Lorg/bukkit/enchantments/Enchantment;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn add_item_flags(
        &mut self,
        arg0: Vec<impl Into<crate::inventory::ItemFlag<'mc>>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "addItemFlags",
            "(Lorg/bukkit/inventory/ItemFlag;)V",
            &[],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn remove_item_flags(
        &mut self,
        arg0: Vec<impl Into<crate::inventory::ItemFlag<'mc>>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeItemFlags",
            "(Lorg/bukkit/inventory/ItemFlag;)V",
            &[],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn item_flags(
        &mut self,
    ) -> Result<blackboxmc_java::JavaSet<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getItemFlags",
            "()Ljava/util/Set;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::JavaSet::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn has_item_flag(
        &mut self,
        arg0: impl Into<&'mc crate::inventory::ItemFlag<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "hasItemFlag",
            "(Lorg/bukkit/inventory/ItemFlag;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn is_unbreakable(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isUnbreakable", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn set_unbreakable(&mut self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        // -2
        let val_1 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setUnbreakable",
            "(Z)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn has_attribute_modifiers(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "hasAttributeModifiers", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn attribute_modifiers(
        &mut self,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getAttributeModifiers",
            "()Lcom/google/common/collect/Multimap;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }

    pub fn add_attribute_modifier(
        &mut self,
        arg0: impl Into<&'mc crate::attribute::Attribute<'mc>>,
        arg1: impl Into<&'mc crate::attribute::AttributeModifier<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let val_2 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "addAttributeModifier",
            "(Lorg/bukkit/attribute/Attribute;Lorg/bukkit/attribute/AttributeModifier;)Z",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub unsafe fn set_attribute_modifiers(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setAttributeModifiers",
            "(Lcom/google/common/collect/Multimap;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn remove_attribute_modifier_with_attribute(
        &mut self,
        arg0: impl Into<&'mc crate::attribute::Attribute<'mc>>,
        arg1: std::option::Option<impl Into<&'mc crate::attribute::AttributeModifier<'mc>>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let val_2 =
            unsafe { jni::objects::JObject::from_raw(arg1.unwrap().into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeAttributeModifier",
            "(Lorg/bukkit/attribute/Attribute;Lorg/bukkit/attribute/AttributeModifier;)Z",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    #[deprecated]

    pub fn custom_tag_container(
        &mut self,
    ) -> Result<crate::inventory::meta::tags::CustomItemTagContainer<'mc>, Box<dyn std::error::Error>>
    {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getCustomTagContainer",
            "()Lorg/bukkit/inventory/meta/tags/CustomItemTagContainer;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::meta::tags::CustomItemTagContainer::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    #[deprecated]

    pub fn set_version(&mut self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setVersion",
            "(I)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

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

    pub fn persistent_data_container(
        &mut self,
    ) -> Result<crate::persistence::PersistentDataContainer<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getPersistentDataContainer",
            "()Lorg/bukkit/persistence/PersistentDataContainer;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::persistence::PersistentDataContainer::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
}
impl<'mc> JNIRaw<'mc> for SkullMeta<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Into<crate::inventory::meta::ItemMeta<'mc>> for SkullMeta<'mc> {
    fn into(self) -> crate::inventory::meta::ItemMeta<'mc> {
        crate::inventory::meta::ItemMeta::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
/// Represents a compass that can track a specific location.
///
/// This is a representation of an abstract class.
pub struct CompassMeta<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> CompassMeta<'mc> {
    pub fn from_raw(
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
    /// Sets if this compass is tracking a specific lodestone. If true the compass will only work if there is a lodestone at the tracked location.
    pub fn set_lodestone_tracked(&mut self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        // -2
        let val_1 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setLodestoneTracked",
            "(Z)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Checks if this compass has been paired to a lodestone.
    pub fn has_lodestone(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasLodestone", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    /// Gets the location that this compass will point to. Check <a href="#hasLodestone()"><code>hasLodestone()</code></a> first!
    pub fn lodestone(&mut self) -> Result<crate::Location<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getLodestone",
            "()Lorg/bukkit/Location;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::Location::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Sets the location this lodestone compass will point to.
    /// Sets if this compass is tracking a specific lodestone. If true the compass will only work if there is a lodestone at the tracked location.
    pub fn set_lodestone(
        &mut self,
        arg0: impl Into<&'mc crate::Location<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setLodestone",
            "(Lorg/bukkit/Location;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Gets if this compass is tracking a specific lodestone. If true the compass will only work if there is a lodestone at the tracked location.
    pub fn is_lodestone_tracked(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isLodestoneTracked", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn clone(
        &mut self,
    ) -> Result<crate::inventory::meta::CompassMeta<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "clone",
            "()Lorg/bukkit/inventory/meta/CompassMeta;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::meta::CompassMeta::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn display_name(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getDisplayName",
            "()Ljava/lang/String;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }

    pub fn localized_name(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getLocalizedName",
            "()Ljava/lang/String;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }

    pub fn get_enchant_level(
        &mut self,
        arg0: impl Into<&'mc crate::enchantments::Enchantment<'mc>>,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getEnchantLevel",
            "(Lorg/bukkit/enchantments/Enchantment;)I",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }

    pub fn set_lore(
        &mut self,
        arg0: Vec<impl Into<String>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let raw_val_1 = self
            .jni_ref()
            .new_object("java/util/ArrayList", "()V", &[])
            .unwrap();
        for v in arg0 {
            let map_val_0 =
                jni::objects::JObject::from(self.jni_ref().new_string(v.into()).unwrap());
            self.jni_ref().call_method(
                &raw_val_1,
                "add",
                "(Ljava/Lang/Object)V",
                &[jni::objects::JValueGen::from(&map_val_0)],
            )?;
        }
        let val_1 = jni::objects::JValueGen::Object(raw_val_1);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setLore",
            "(Ljava/util/List;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn as_string(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getAsString",
            "()Ljava/lang/String;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }

    pub fn has_display_name(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasDisplayName", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn set_display_name(
        &mut self,
        arg0: impl Into<&'mc String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setDisplayName",
            "(Ljava/lang/String;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn has_localized_name(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasLocalizedName", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn set_localized_name(
        &mut self,
        arg0: impl Into<&'mc String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setLocalizedName",
            "(Ljava/lang/String;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn has_lore(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasLore", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn lore(&mut self) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getLore", "()Ljava/util/List;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let mut list = blackboxmc_java::JavaList::from_raw(&self.jni_ref(), res.l()?)?;
        let size = list.size()?;
        for i in 0..=size {
            let obj = list.get(i)?;
            new_vec.push(
                self.jni_ref()
                    .get_string(unsafe { &jni::objects::JString::from_raw(*obj) })?
                    .to_string_lossy()
                    .to_string(),
            );
        }
        Ok(new_vec)
    }

    pub fn has_custom_model_data(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasCustomModelData", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn custom_model_data(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getCustomModelData", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }

    pub fn set_custom_model_data(&mut self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setCustomModelData",
            "(I)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn has_enchants(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasEnchants", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn has_enchant(
        &mut self,
        arg0: impl Into<&'mc crate::enchantments::Enchantment<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "hasEnchant",
            "(Lorg/bukkit/enchantments/Enchantment;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn enchants(
        &mut self,
    ) -> Result<blackboxmc_java::JavaMap<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getEnchants", "()Ljava/util/Map;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::JavaMap::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn add_enchant(
        &mut self,
        arg0: impl Into<&'mc crate::enchantments::Enchantment<'mc>>,
        arg1: i32,
        arg2: bool,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let val_2 = jni::objects::JValueGen::Int(arg1.into());
        // -2
        let val_3 = jni::objects::JValueGen::Bool(arg2.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "addEnchant",
            "(Lorg/bukkit/enchantments/Enchantment;IZ)Z",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
                jni::objects::JValueGen::from(&val_3),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn remove_enchant(
        &mut self,
        arg0: impl Into<&'mc crate::enchantments::Enchantment<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeEnchant",
            "(Lorg/bukkit/enchantments/Enchantment;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn has_conflicting_enchant(
        &mut self,
        arg0: impl Into<&'mc crate::enchantments::Enchantment<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "hasConflictingEnchant",
            "(Lorg/bukkit/enchantments/Enchantment;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn add_item_flags(
        &mut self,
        arg0: Vec<impl Into<crate::inventory::ItemFlag<'mc>>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "addItemFlags",
            "(Lorg/bukkit/inventory/ItemFlag;)V",
            &[],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn remove_item_flags(
        &mut self,
        arg0: Vec<impl Into<crate::inventory::ItemFlag<'mc>>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeItemFlags",
            "(Lorg/bukkit/inventory/ItemFlag;)V",
            &[],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn item_flags(
        &mut self,
    ) -> Result<blackboxmc_java::JavaSet<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getItemFlags",
            "()Ljava/util/Set;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::JavaSet::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn has_item_flag(
        &mut self,
        arg0: impl Into<&'mc crate::inventory::ItemFlag<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "hasItemFlag",
            "(Lorg/bukkit/inventory/ItemFlag;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn is_unbreakable(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isUnbreakable", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn set_unbreakable(&mut self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        // -2
        let val_1 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setUnbreakable",
            "(Z)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn has_attribute_modifiers(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "hasAttributeModifiers", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn attribute_modifiers(
        &mut self,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getAttributeModifiers",
            "()Lcom/google/common/collect/Multimap;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }

    pub fn add_attribute_modifier(
        &mut self,
        arg0: impl Into<&'mc crate::attribute::Attribute<'mc>>,
        arg1: impl Into<&'mc crate::attribute::AttributeModifier<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let val_2 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "addAttributeModifier",
            "(Lorg/bukkit/attribute/Attribute;Lorg/bukkit/attribute/AttributeModifier;)Z",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub unsafe fn set_attribute_modifiers(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setAttributeModifiers",
            "(Lcom/google/common/collect/Multimap;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn remove_attribute_modifier_with_attribute(
        &mut self,
        arg0: impl Into<&'mc crate::attribute::Attribute<'mc>>,
        arg1: std::option::Option<impl Into<&'mc crate::attribute::AttributeModifier<'mc>>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let val_2 =
            unsafe { jni::objects::JObject::from_raw(arg1.unwrap().into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeAttributeModifier",
            "(Lorg/bukkit/attribute/Attribute;Lorg/bukkit/attribute/AttributeModifier;)Z",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    #[deprecated]

    pub fn custom_tag_container(
        &mut self,
    ) -> Result<crate::inventory::meta::tags::CustomItemTagContainer<'mc>, Box<dyn std::error::Error>>
    {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getCustomTagContainer",
            "()Lorg/bukkit/inventory/meta/tags/CustomItemTagContainer;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::meta::tags::CustomItemTagContainer::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    #[deprecated]

    pub fn set_version(&mut self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setVersion",
            "(I)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

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

    pub fn persistent_data_container(
        &mut self,
    ) -> Result<crate::persistence::PersistentDataContainer<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getPersistentDataContainer",
            "()Lorg/bukkit/persistence/PersistentDataContainer;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::persistence::PersistentDataContainer::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
}
impl<'mc> JNIRaw<'mc> for CompassMeta<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Into<crate::inventory::meta::ItemMeta<'mc>> for CompassMeta<'mc> {
    fn into(self) -> crate::inventory::meta::ItemMeta<'mc> {
        crate::inventory::meta::ItemMeta::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
/// Represents a suspicious stew that can have custom effects.
///
/// This is a representation of an abstract class.
pub struct SuspiciousStewMeta<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> SuspiciousStewMeta<'mc> {
    pub fn from_raw(
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
    /// Gets an immutable list containing all custom potion effects applied to this suspicious stew.
    /// <p>Plugins should check that hasCustomEffects() returns true before calling this method.</p>
    pub fn custom_effects(
        &mut self,
    ) -> Result<Vec<crate::potion::PotionEffect<'mc>>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getCustomEffects",
            "()Ljava/util/List;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let mut list = blackboxmc_java::JavaList::from_raw(&self.jni_ref(), res.l()?)?;
        let size = list.size()?;
        for i in 0..=size {
            let obj = list.get(i)?;
            new_vec.push(crate::potion::PotionEffect::from_raw(&self.jni_ref(), obj)?);
        }
        Ok(new_vec)
    }
    /// Checks for the presence of custom potion effects.
    pub fn has_custom_effects(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasCustomEffects", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    /// Adds a custom potion effect to this suspicious stew.
    pub fn add_custom_effect(
        &mut self,
        arg0: impl Into<&'mc crate::potion::PotionEffect<'mc>>,
        arg1: bool,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        // -2
        let val_2 = jni::objects::JValueGen::Bool(arg1.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "addCustomEffect",
            "(Lorg/bukkit/potion/PotionEffect;Z)Z",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    /// Removes a custom potion effect from this suspicious stew.
    pub fn remove_custom_effect(
        &mut self,
        arg0: impl Into<&'mc crate::potion::PotionEffectType<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeCustomEffect",
            "(Lorg/bukkit/potion/PotionEffectType;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    /// Checks for the presence of custom potion effects.
    /// Checks for a specific custom potion effect type on this suspicious stew.
    pub fn has_custom_effect(
        &mut self,
        arg0: impl Into<&'mc crate::potion::PotionEffectType<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "hasCustomEffect",
            "(Lorg/bukkit/potion/PotionEffectType;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    /// Removes all custom potion effects from this suspicious stew.
    pub fn clear_custom_effects(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "clearCustomEffects", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn clone(
        &mut self,
    ) -> Result<crate::inventory::meta::SuspiciousStewMeta<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "clone",
            "()Lorg/bukkit/inventory/meta/SuspiciousStewMeta;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::meta::SuspiciousStewMeta::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn display_name(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getDisplayName",
            "()Ljava/lang/String;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }

    pub fn localized_name(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getLocalizedName",
            "()Ljava/lang/String;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }

    pub fn get_enchant_level(
        &mut self,
        arg0: impl Into<&'mc crate::enchantments::Enchantment<'mc>>,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getEnchantLevel",
            "(Lorg/bukkit/enchantments/Enchantment;)I",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }

    pub fn set_lore(
        &mut self,
        arg0: Vec<impl Into<String>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let raw_val_1 = self
            .jni_ref()
            .new_object("java/util/ArrayList", "()V", &[])
            .unwrap();
        for v in arg0 {
            let map_val_0 =
                jni::objects::JObject::from(self.jni_ref().new_string(v.into()).unwrap());
            self.jni_ref().call_method(
                &raw_val_1,
                "add",
                "(Ljava/Lang/Object)V",
                &[jni::objects::JValueGen::from(&map_val_0)],
            )?;
        }
        let val_1 = jni::objects::JValueGen::Object(raw_val_1);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setLore",
            "(Ljava/util/List;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn as_string(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getAsString",
            "()Ljava/lang/String;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }

    pub fn has_display_name(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasDisplayName", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn set_display_name(
        &mut self,
        arg0: impl Into<&'mc String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setDisplayName",
            "(Ljava/lang/String;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn has_localized_name(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasLocalizedName", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn set_localized_name(
        &mut self,
        arg0: impl Into<&'mc String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setLocalizedName",
            "(Ljava/lang/String;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn has_lore(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasLore", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn lore(&mut self) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getLore", "()Ljava/util/List;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let mut list = blackboxmc_java::JavaList::from_raw(&self.jni_ref(), res.l()?)?;
        let size = list.size()?;
        for i in 0..=size {
            let obj = list.get(i)?;
            new_vec.push(
                self.jni_ref()
                    .get_string(unsafe { &jni::objects::JString::from_raw(*obj) })?
                    .to_string_lossy()
                    .to_string(),
            );
        }
        Ok(new_vec)
    }

    pub fn has_custom_model_data(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasCustomModelData", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn custom_model_data(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getCustomModelData", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }

    pub fn set_custom_model_data(&mut self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setCustomModelData",
            "(I)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn has_enchants(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasEnchants", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn has_enchant(
        &mut self,
        arg0: impl Into<&'mc crate::enchantments::Enchantment<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "hasEnchant",
            "(Lorg/bukkit/enchantments/Enchantment;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn enchants(
        &mut self,
    ) -> Result<blackboxmc_java::JavaMap<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getEnchants", "()Ljava/util/Map;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::JavaMap::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn add_enchant(
        &mut self,
        arg0: impl Into<&'mc crate::enchantments::Enchantment<'mc>>,
        arg1: i32,
        arg2: bool,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let val_2 = jni::objects::JValueGen::Int(arg1.into());
        // -2
        let val_3 = jni::objects::JValueGen::Bool(arg2.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "addEnchant",
            "(Lorg/bukkit/enchantments/Enchantment;IZ)Z",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
                jni::objects::JValueGen::from(&val_3),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn remove_enchant(
        &mut self,
        arg0: impl Into<&'mc crate::enchantments::Enchantment<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeEnchant",
            "(Lorg/bukkit/enchantments/Enchantment;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn has_conflicting_enchant(
        &mut self,
        arg0: impl Into<&'mc crate::enchantments::Enchantment<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "hasConflictingEnchant",
            "(Lorg/bukkit/enchantments/Enchantment;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn add_item_flags(
        &mut self,
        arg0: Vec<impl Into<crate::inventory::ItemFlag<'mc>>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "addItemFlags",
            "(Lorg/bukkit/inventory/ItemFlag;)V",
            &[],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn remove_item_flags(
        &mut self,
        arg0: Vec<impl Into<crate::inventory::ItemFlag<'mc>>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeItemFlags",
            "(Lorg/bukkit/inventory/ItemFlag;)V",
            &[],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn item_flags(
        &mut self,
    ) -> Result<blackboxmc_java::JavaSet<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getItemFlags",
            "()Ljava/util/Set;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::JavaSet::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn has_item_flag(
        &mut self,
        arg0: impl Into<&'mc crate::inventory::ItemFlag<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "hasItemFlag",
            "(Lorg/bukkit/inventory/ItemFlag;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn is_unbreakable(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isUnbreakable", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn set_unbreakable(&mut self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        // -2
        let val_1 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setUnbreakable",
            "(Z)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn has_attribute_modifiers(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "hasAttributeModifiers", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn attribute_modifiers(
        &mut self,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getAttributeModifiers",
            "()Lcom/google/common/collect/Multimap;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }

    pub fn add_attribute_modifier(
        &mut self,
        arg0: impl Into<&'mc crate::attribute::Attribute<'mc>>,
        arg1: impl Into<&'mc crate::attribute::AttributeModifier<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let val_2 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "addAttributeModifier",
            "(Lorg/bukkit/attribute/Attribute;Lorg/bukkit/attribute/AttributeModifier;)Z",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub unsafe fn set_attribute_modifiers(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setAttributeModifiers",
            "(Lcom/google/common/collect/Multimap;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn remove_attribute_modifier_with_attribute(
        &mut self,
        arg0: impl Into<&'mc crate::attribute::Attribute<'mc>>,
        arg1: std::option::Option<impl Into<&'mc crate::attribute::AttributeModifier<'mc>>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let val_2 =
            unsafe { jni::objects::JObject::from_raw(arg1.unwrap().into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeAttributeModifier",
            "(Lorg/bukkit/attribute/Attribute;Lorg/bukkit/attribute/AttributeModifier;)Z",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    #[deprecated]

    pub fn custom_tag_container(
        &mut self,
    ) -> Result<crate::inventory::meta::tags::CustomItemTagContainer<'mc>, Box<dyn std::error::Error>>
    {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getCustomTagContainer",
            "()Lorg/bukkit/inventory/meta/tags/CustomItemTagContainer;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::meta::tags::CustomItemTagContainer::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    #[deprecated]

    pub fn set_version(&mut self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setVersion",
            "(I)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

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

    pub fn persistent_data_container(
        &mut self,
    ) -> Result<crate::persistence::PersistentDataContainer<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getPersistentDataContainer",
            "()Lorg/bukkit/persistence/PersistentDataContainer;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::persistence::PersistentDataContainer::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
}
impl<'mc> JNIRaw<'mc> for SuspiciousStewMeta<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Into<crate::inventory::meta::ItemMeta<'mc>> for SuspiciousStewMeta<'mc> {
    fn into(self) -> crate::inventory::meta::ItemMeta<'mc> {
        crate::inventory::meta::ItemMeta::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}

///
/// This is a representation of an abstract class.
pub struct CrossbowMeta<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> CrossbowMeta<'mc> {
    pub fn from_raw(
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
    /// Returns whether the item has any charged projectiles.
    pub fn has_charged_projectiles(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "hasChargedProjectiles", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    /// Returns an immutable list of the projectiles charged on this item.
    pub fn charged_projectiles(
        &mut self,
    ) -> Result<Vec<crate::inventory::ItemStack<'mc>>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getChargedProjectiles",
            "()Ljava/util/List;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let mut list = blackboxmc_java::JavaList::from_raw(&self.jni_ref(), res.l()?)?;
        let size = list.size()?;
        for i in 0..=size {
            let obj = list.get(i)?;
            new_vec.push(crate::inventory::ItemStack::from_raw(&self.jni_ref(), obj)?);
        }
        Ok(new_vec)
    }
    /// Sets the projectiles charged on this item. Removes all projectiles when given null.
    pub fn set_charged_projectiles(
        &mut self,
        arg0: Vec<impl Into<crate::inventory::ItemStack<'mc>>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let raw_val_1 = self
            .jni_ref()
            .new_object("java/util/ArrayList", "()V", &[])
            .unwrap();
        for v in arg0 {
            let map_val_0 =
                unsafe { jni::objects::JObject::from_raw(v.into().jni_object().clone()) };
            self.jni_ref().call_method(
                &raw_val_1,
                "add",
                "(Ljava/Lang/Object)V",
                &[jni::objects::JValueGen::from(&map_val_0)],
            )?;
        }
        let val_1 = jni::objects::JValueGen::Object(raw_val_1);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setChargedProjectiles",
            "(Ljava/util/List;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Adds a charged projectile to this item.
    pub fn add_charged_projectile(
        &mut self,
        arg0: impl Into<&'mc crate::inventory::ItemStack<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "addChargedProjectile",
            "(Lorg/bukkit/inventory/ItemStack;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn display_name(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getDisplayName",
            "()Ljava/lang/String;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }

    pub fn localized_name(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getLocalizedName",
            "()Ljava/lang/String;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }

    pub fn get_enchant_level(
        &mut self,
        arg0: impl Into<&'mc crate::enchantments::Enchantment<'mc>>,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getEnchantLevel",
            "(Lorg/bukkit/enchantments/Enchantment;)I",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }

    pub fn set_lore(
        &mut self,
        arg0: Vec<impl Into<String>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let raw_val_1 = self
            .jni_ref()
            .new_object("java/util/ArrayList", "()V", &[])
            .unwrap();
        for v in arg0 {
            let map_val_0 =
                jni::objects::JObject::from(self.jni_ref().new_string(v.into()).unwrap());
            self.jni_ref().call_method(
                &raw_val_1,
                "add",
                "(Ljava/Lang/Object)V",
                &[jni::objects::JValueGen::from(&map_val_0)],
            )?;
        }
        let val_1 = jni::objects::JValueGen::Object(raw_val_1);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setLore",
            "(Ljava/util/List;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn as_string(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getAsString",
            "()Ljava/lang/String;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }

    pub fn has_display_name(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasDisplayName", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn set_display_name(
        &mut self,
        arg0: impl Into<&'mc String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setDisplayName",
            "(Ljava/lang/String;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn has_localized_name(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasLocalizedName", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn set_localized_name(
        &mut self,
        arg0: impl Into<&'mc String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setLocalizedName",
            "(Ljava/lang/String;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn has_lore(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasLore", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn lore(&mut self) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getLore", "()Ljava/util/List;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let mut list = blackboxmc_java::JavaList::from_raw(&self.jni_ref(), res.l()?)?;
        let size = list.size()?;
        for i in 0..=size {
            let obj = list.get(i)?;
            new_vec.push(
                self.jni_ref()
                    .get_string(unsafe { &jni::objects::JString::from_raw(*obj) })?
                    .to_string_lossy()
                    .to_string(),
            );
        }
        Ok(new_vec)
    }

    pub fn has_custom_model_data(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasCustomModelData", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn custom_model_data(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getCustomModelData", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }

    pub fn set_custom_model_data(&mut self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setCustomModelData",
            "(I)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn has_enchants(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasEnchants", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn has_enchant(
        &mut self,
        arg0: impl Into<&'mc crate::enchantments::Enchantment<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "hasEnchant",
            "(Lorg/bukkit/enchantments/Enchantment;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn enchants(
        &mut self,
    ) -> Result<blackboxmc_java::JavaMap<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getEnchants", "()Ljava/util/Map;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::JavaMap::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn add_enchant(
        &mut self,
        arg0: impl Into<&'mc crate::enchantments::Enchantment<'mc>>,
        arg1: i32,
        arg2: bool,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let val_2 = jni::objects::JValueGen::Int(arg1.into());
        // -2
        let val_3 = jni::objects::JValueGen::Bool(arg2.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "addEnchant",
            "(Lorg/bukkit/enchantments/Enchantment;IZ)Z",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
                jni::objects::JValueGen::from(&val_3),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn remove_enchant(
        &mut self,
        arg0: impl Into<&'mc crate::enchantments::Enchantment<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeEnchant",
            "(Lorg/bukkit/enchantments/Enchantment;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn has_conflicting_enchant(
        &mut self,
        arg0: impl Into<&'mc crate::enchantments::Enchantment<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "hasConflictingEnchant",
            "(Lorg/bukkit/enchantments/Enchantment;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn add_item_flags(
        &mut self,
        arg0: Vec<impl Into<crate::inventory::ItemFlag<'mc>>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "addItemFlags",
            "(Lorg/bukkit/inventory/ItemFlag;)V",
            &[],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn remove_item_flags(
        &mut self,
        arg0: Vec<impl Into<crate::inventory::ItemFlag<'mc>>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeItemFlags",
            "(Lorg/bukkit/inventory/ItemFlag;)V",
            &[],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn item_flags(
        &mut self,
    ) -> Result<blackboxmc_java::JavaSet<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getItemFlags",
            "()Ljava/util/Set;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::JavaSet::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn has_item_flag(
        &mut self,
        arg0: impl Into<&'mc crate::inventory::ItemFlag<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "hasItemFlag",
            "(Lorg/bukkit/inventory/ItemFlag;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn is_unbreakable(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isUnbreakable", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn set_unbreakable(&mut self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        // -2
        let val_1 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setUnbreakable",
            "(Z)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn has_attribute_modifiers(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "hasAttributeModifiers", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn attribute_modifiers(
        &mut self,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getAttributeModifiers",
            "()Lcom/google/common/collect/Multimap;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }

    pub fn add_attribute_modifier(
        &mut self,
        arg0: impl Into<&'mc crate::attribute::Attribute<'mc>>,
        arg1: impl Into<&'mc crate::attribute::AttributeModifier<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let val_2 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "addAttributeModifier",
            "(Lorg/bukkit/attribute/Attribute;Lorg/bukkit/attribute/AttributeModifier;)Z",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub unsafe fn set_attribute_modifiers(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setAttributeModifiers",
            "(Lcom/google/common/collect/Multimap;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn remove_attribute_modifier_with_attribute(
        &mut self,
        arg0: impl Into<&'mc crate::attribute::Attribute<'mc>>,
        arg1: std::option::Option<impl Into<&'mc crate::attribute::AttributeModifier<'mc>>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let val_2 =
            unsafe { jni::objects::JObject::from_raw(arg1.unwrap().into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeAttributeModifier",
            "(Lorg/bukkit/attribute/Attribute;Lorg/bukkit/attribute/AttributeModifier;)Z",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    #[deprecated]

    pub fn custom_tag_container(
        &mut self,
    ) -> Result<crate::inventory::meta::tags::CustomItemTagContainer<'mc>, Box<dyn std::error::Error>>
    {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getCustomTagContainer",
            "()Lorg/bukkit/inventory/meta/tags/CustomItemTagContainer;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::meta::tags::CustomItemTagContainer::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    #[deprecated]

    pub fn set_version(&mut self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setVersion",
            "(I)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn clone(&mut self) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "clone", "()Ljava/lang/Object;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }

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

    pub fn persistent_data_container(
        &mut self,
    ) -> Result<crate::persistence::PersistentDataContainer<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getPersistentDataContainer",
            "()Lorg/bukkit/persistence/PersistentDataContainer;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::persistence::PersistentDataContainer::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
}
impl<'mc> JNIRaw<'mc> for CrossbowMeta<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Into<crate::inventory::meta::ItemMeta<'mc>> for CrossbowMeta<'mc> {
    fn into(self) -> crate::inventory::meta::ItemMeta<'mc> {
        crate::inventory::meta::ItemMeta::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
/// This type represents the storage mechanism for auxiliary item data.
/// <p>An implementation will handle the creation and application for ItemMeta. This class should not be implemented by a plugin in a live environment.</p>
///
/// This is a representation of an abstract class.
pub struct ItemMeta<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> ItemMeta<'mc> {
    pub fn from_raw(
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
    /// Gets the display name that is set.
    /// <p>Plugins should check that hasDisplayName() returns <code>true</code> before calling this method.</p>
    pub fn display_name(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getDisplayName",
            "()Ljava/lang/String;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    /// Gets the localized display name that is set.
    /// <p>Plugins should check that hasLocalizedName() returns <code>true</code> before calling this method.</p>
    pub fn localized_name(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getLocalizedName",
            "()Ljava/lang/String;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    /// Checks for the level of the specified enchantment.
    pub fn get_enchant_level(
        &mut self,
        arg0: impl Into<&'mc crate::enchantments::Enchantment<'mc>>,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getEnchantLevel",
            "(Lorg/bukkit/enchantments/Enchantment;)I",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }
    /// Sets the lore for this item. Removes lore when given null.
    pub fn set_lore(
        &mut self,
        arg0: Vec<impl Into<String>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let raw_val_1 = self
            .jni_ref()
            .new_object("java/util/ArrayList", "()V", &[])
            .unwrap();
        for v in arg0 {
            let map_val_0 =
                jni::objects::JObject::from(self.jni_ref().new_string(v.into()).unwrap());
            self.jni_ref().call_method(
                &raw_val_1,
                "add",
                "(Ljava/Lang/Object)V",
                &[jni::objects::JValueGen::from(&map_val_0)],
            )?;
        }
        let val_1 = jni::objects::JValueGen::Object(raw_val_1);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setLore",
            "(Ljava/util/List;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Get this ItemMeta as an NBT string.
    /// <p>This string should not be relied upon as a serializable value. If serialization is desired, the <a title="interface in org.bukkit.configuration.serialization" href="../../configuration/serialization/ConfigurationSerializable.html"><code>ConfigurationSerializable</code></a> API should be used instead.</p>
    pub fn as_string(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getAsString",
            "()Ljava/lang/String;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    /// Checks for existence of a display name.
    pub fn has_display_name(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasDisplayName", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    /// Sets the display name.
    pub fn set_display_name(
        &mut self,
        arg0: impl Into<&'mc String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setDisplayName",
            "(Ljava/lang/String;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Checks for existence of a localized name.
    pub fn has_localized_name(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasLocalizedName", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    /// Sets the localized name.
    pub fn set_localized_name(
        &mut self,
        arg0: impl Into<&'mc String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setLocalizedName",
            "(Ljava/lang/String;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Checks for existence of lore.
    pub fn has_lore(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasLore", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    /// Gets the lore that is set.
    /// <p>Plugins should check if hasLore() returns <code>true</code> before calling this method.</p>
    pub fn lore(&mut self) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getLore", "()Ljava/util/List;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let mut list = blackboxmc_java::JavaList::from_raw(&self.jni_ref(), res.l()?)?;
        let size = list.size()?;
        for i in 0..=size {
            let obj = list.get(i)?;
            new_vec.push(
                self.jni_ref()
                    .get_string(unsafe { &jni::objects::JString::from_raw(*obj) })?
                    .to_string_lossy()
                    .to_string(),
            );
        }
        Ok(new_vec)
    }
    /// Checks for existence of custom model data.
    /// <p>CustomModelData is an integer that may be associated client side with a custom item model.</p>
    pub fn has_custom_model_data(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasCustomModelData", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    /// Gets the custom model data that is set.
    /// <p>CustomModelData is an integer that may be associated client side with a custom item model.</p>
    /// <p>Plugins should check that hasCustomModelData() returns <code>true</code> before calling this method.</p>
    pub fn custom_model_data(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getCustomModelData", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }
    /// Sets the custom model data.
    /// <p>CustomModelData is an integer that may be associated client side with a custom item model.</p>
    pub fn set_custom_model_data(&mut self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setCustomModelData",
            "(I)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Checks for the existence of any enchantments.
    pub fn has_enchants(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasEnchants", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    /// Checks for the existence of any enchantments.
    /// Checks for existence of the specified enchantment.
    pub fn has_enchant(
        &mut self,
        arg0: impl Into<&'mc crate::enchantments::Enchantment<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "hasEnchant",
            "(Lorg/bukkit/enchantments/Enchantment;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    /// Returns a copy the enchantments in this ItemMeta.
    ///
    /// Returns an empty map if none.
    pub fn enchants(
        &mut self,
    ) -> Result<blackboxmc_java::JavaMap<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getEnchants", "()Ljava/util/Map;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::JavaMap::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Adds the specified enchantment to this item meta.
    pub fn add_enchant(
        &mut self,
        arg0: impl Into<&'mc crate::enchantments::Enchantment<'mc>>,
        arg1: i32,
        arg2: bool,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let val_2 = jni::objects::JValueGen::Int(arg1.into());
        // -2
        let val_3 = jni::objects::JValueGen::Bool(arg2.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "addEnchant",
            "(Lorg/bukkit/enchantments/Enchantment;IZ)Z",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
                jni::objects::JValueGen::from(&val_3),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    /// Removes the specified enchantment from this item meta.
    pub fn remove_enchant(
        &mut self,
        arg0: impl Into<&'mc crate::enchantments::Enchantment<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeEnchant",
            "(Lorg/bukkit/enchantments/Enchantment;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    /// Checks if the specified enchantment conflicts with any enchantments in this ItemMeta.
    pub fn has_conflicting_enchant(
        &mut self,
        arg0: impl Into<&'mc crate::enchantments::Enchantment<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "hasConflictingEnchant",
            "(Lorg/bukkit/enchantments/Enchantment;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    /// Set itemflags which should be ignored when rendering a ItemStack in the Client. This Method does silently ignore double set itemFlags.
    pub fn add_item_flags(
        &mut self,
        arg0: Vec<impl Into<crate::inventory::ItemFlag<'mc>>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "addItemFlags",
            "(Lorg/bukkit/inventory/ItemFlag;)V",
            &[],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Remove specific set of itemFlags. This tells the Client it should render it again. This Method does silently ignore double removed itemFlags.
    pub fn remove_item_flags(
        &mut self,
        arg0: Vec<impl Into<crate::inventory::ItemFlag<'mc>>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeItemFlags",
            "(Lorg/bukkit/inventory/ItemFlag;)V",
            &[],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Get current set itemFlags. The collection returned is unmodifiable.
    pub fn item_flags(
        &mut self,
    ) -> Result<blackboxmc_java::JavaSet<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getItemFlags",
            "()Ljava/util/Set;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::JavaSet::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Check if the specified flag is present on this item.
    pub fn has_item_flag(
        &mut self,
        arg0: impl Into<&'mc crate::inventory::ItemFlag<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "hasItemFlag",
            "(Lorg/bukkit/inventory/ItemFlag;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    /// Return if the unbreakable tag is true. An unbreakable item will not lose durability.
    pub fn is_unbreakable(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isUnbreakable", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    /// Sets the unbreakable tag. An unbreakable item will not lose durability.
    pub fn set_unbreakable(&mut self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        // -2
        let val_1 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setUnbreakable",
            "(Z)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Checks for the existence of any AttributeModifiers.
    pub fn has_attribute_modifiers(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "hasAttributeModifiers", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    /// Return an immutable copy of all Attributes and their modifiers in this ItemMeta.
    ///
    /// Returns null if none exist.
    /// Return an immutable copy of all <a title="enum in org.bukkit.attribute" href="../../attribute/Attribute.html"><code>Attribute</code></a>s and their <a href="../../attribute/AttributeModifier.html" title="class in org.bukkit.attribute"><code>AttributeModifier</code></a>s for a given <a href="../EquipmentSlot.html" title="enum in org.bukkit.inventory"><code>EquipmentSlot</code></a>.
    ///
    /// Any <a href="../../attribute/AttributeModifier.html" title="class in org.bukkit.attribute"><code>AttributeModifier</code></a> that does have have a given <a title="enum in org.bukkit.inventory" href="../EquipmentSlot.html"><code>EquipmentSlot</code></a> will be returned. This is because AttributeModifiers without a slot are active in any slot.
    ///
    /// If there are no attributes set for the given slot, an empty map will be returned.
    /// Return an immutable copy of all <a href="../../attribute/AttributeModifier.html" title="class in org.bukkit.attribute"><code>AttributeModifier</code></a>s for a given <a href="../../attribute/Attribute.html" title="enum in org.bukkit.attribute"><code>Attribute</code></a>
    pub fn attribute_modifiers(
        &mut self,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getAttributeModifiers",
            "()Lcom/google/common/collect/Multimap;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
    /// Add an Attribute and it's Modifier. AttributeModifiers can now support <a href="../EquipmentSlot.html" title="enum in org.bukkit.inventory"><code>EquipmentSlot</code></a>s. If not set, the <a href="../../attribute/AttributeModifier.html" title="class in org.bukkit.attribute"><code>AttributeModifier</code></a> will be active in ALL slots.
    ///
    /// Two <a href="../../attribute/AttributeModifier.html" title="class in org.bukkit.attribute"><code>AttributeModifier</code></a>s that have the same <a title="class or interface in java.util" class="external-link" href="https://docs.oracle.com/javase/8/docs/api/java/util/UUID.html"><code>UUID</code></a> cannot exist on the same Attribute.
    pub fn add_attribute_modifier(
        &mut self,
        arg0: impl Into<&'mc crate::attribute::Attribute<'mc>>,
        arg1: impl Into<&'mc crate::attribute::AttributeModifier<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let val_2 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "addAttributeModifier",
            "(Lorg/bukkit/attribute/Attribute;Lorg/bukkit/attribute/AttributeModifier;)Z",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    /// Set all <a title="enum in org.bukkit.attribute" href="../../attribute/Attribute.html"><code>Attribute</code></a>s and their <a href="../../attribute/AttributeModifier.html" title="class in org.bukkit.attribute"><code>AttributeModifier</code></a>s. To clear all currently set Attributes and AttributeModifiers use null or an empty Multimap. If not null nor empty, this will filter all entries that are not-null and add them to the ItemStack.
    pub unsafe fn set_attribute_modifiers(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setAttributeModifiers",
            "(Lcom/google/common/collect/Multimap;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Remove all <a title="class in org.bukkit.attribute" href="../../attribute/AttributeModifier.html"><code>AttributeModifier</code></a>s associated with the given <a title="enum in org.bukkit.attribute" href="../../attribute/Attribute.html"><code>Attribute</code></a>. This will return false if nothing was removed.
    /// Remove all <a href="../../attribute/Attribute.html" title="enum in org.bukkit.attribute"><code>Attribute</code></a>s and <a title="class in org.bukkit.attribute" href="../../attribute/AttributeModifier.html"><code>AttributeModifier</code></a>s for a given <a href="../EquipmentSlot.html" title="enum in org.bukkit.inventory"><code>EquipmentSlot</code></a>.
    ///
    /// If the given <a title="enum in org.bukkit.inventory" href="../EquipmentSlot.html"><code>EquipmentSlot</code></a> is null, this will remove all <a href="../../attribute/AttributeModifier.html" title="class in org.bukkit.attribute"><code>AttributeModifier</code></a>s that do not have an EquipmentSlot set.
    /// Remove a specific <a href="../../attribute/Attribute.html" title="enum in org.bukkit.attribute"><code>Attribute</code></a> and <a title="class in org.bukkit.attribute" href="../../attribute/AttributeModifier.html"><code>AttributeModifier</code></a>. AttributeModifiers are matched according to their <a href="https://docs.oracle.com/javase/8/docs/api/java/util/UUID.html" title="class or interface in java.util" class="external-link"><code>UUID</code></a>.
    pub fn remove_attribute_modifier_with_attribute(
        &mut self,
        arg0: impl Into<&'mc crate::attribute::Attribute<'mc>>,
        arg1: std::option::Option<impl Into<&'mc crate::attribute::AttributeModifier<'mc>>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let val_2 =
            unsafe { jni::objects::JObject::from_raw(arg1.unwrap().into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeAttributeModifier",
            "(Lorg/bukkit/attribute/Attribute;Lorg/bukkit/attribute/AttributeModifier;)Z",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    #[deprecated]
    /// <span class="deprecated-label">Deprecated.</span>
    /// <div class="deprecation-comment">
    /// this API part has been replaced by the <a href="../../persistence/PersistentDataHolder.html" title="interface in org.bukkit.persistence"><code>PersistentDataHolder</code></a> API. Please use <a href="../../persistence/PersistentDataHolder.html#getPersistentDataContainer()"><code>PersistentDataHolder.getPersistentDataContainer()</code></a> instead of this.
    /// </div>
    /// this API part has been replaced by the <a href="../../persistence/PersistentDataHolder.html" title="interface in org.bukkit.persistence"><code>PersistentDataHolder</code></a> API. Please use <a href="../../persistence/PersistentDataHolder.html#getPersistentDataContainer()"><code>PersistentDataHolder.getPersistentDataContainer()</code></a> instead of this.
    ///
    /// Returns a public custom tag container capable of storing tags on the item. Those tags will be sent to the client with all of their content, so the client is capable of reading them. This will result in the player seeing a NBT Tag notification on the item. These tags can also be modified by the client once in creative mode
    pub fn custom_tag_container(
        &mut self,
    ) -> Result<crate::inventory::meta::tags::CustomItemTagContainer<'mc>, Box<dyn std::error::Error>>
    {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getCustomTagContainer",
            "()Lorg/bukkit/inventory/meta/tags/CustomItemTagContainer;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::meta::tags::CustomItemTagContainer::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    #[deprecated]
    /// <span class="deprecated-label">Deprecated.</span>
    /// <div class="deprecation-comment">
    /// internal use only
    /// </div>
    /// internal use only
    ///
    /// Internal use only! Do not use under any circumstances!
    pub fn set_version(&mut self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setVersion",
            "(I)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn clone(&mut self) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "clone", "()Ljava/lang/Object;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }

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

    pub fn persistent_data_container(
        &mut self,
    ) -> Result<crate::persistence::PersistentDataContainer<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getPersistentDataContainer",
            "()Lorg/bukkit/persistence/PersistentDataContainer;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::persistence::PersistentDataContainer::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
}
impl<'mc> JNIRaw<'mc> for ItemMeta<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Into<crate::configuration::serialization::ConfigurationSerializable<'mc>>
    for ItemMeta<'mc>
{
    fn into(self) -> crate::configuration::serialization::ConfigurationSerializable<'mc> {
        crate::configuration::serialization::ConfigurationSerializable::from_raw(
            &self.jni_ref(),
            self.1,
        )
        .unwrap()
    }
}
impl<'mc> Into<crate::persistence::PersistentDataHolder<'mc>> for ItemMeta<'mc> {
    fn into(self) -> crate::persistence::PersistentDataHolder<'mc> {
        crate::persistence::PersistentDataHolder::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
/// Represents an item that can be repaired at an anvil.
///
/// This is a representation of an abstract class.
pub struct Repairable<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> Repairable<'mc> {
    pub fn from_raw(
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
    /// Gets the repair penalty
    pub fn repair_cost(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getRepairCost", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }
    /// Sets the repair penalty
    pub fn set_repair_cost(&mut self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setRepairCost",
            "(I)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Checks to see if this has a repair penalty
    pub fn has_repair_cost(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasRepairCost", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn clone(
        &mut self,
    ) -> Result<crate::inventory::meta::Repairable<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "clone",
            "()Lorg/bukkit/inventory/meta/Repairable;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::meta::Repairable::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn display_name(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getDisplayName",
            "()Ljava/lang/String;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }

    pub fn localized_name(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getLocalizedName",
            "()Ljava/lang/String;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }

    pub fn get_enchant_level(
        &mut self,
        arg0: impl Into<&'mc crate::enchantments::Enchantment<'mc>>,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getEnchantLevel",
            "(Lorg/bukkit/enchantments/Enchantment;)I",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }

    pub fn set_lore(
        &mut self,
        arg0: Vec<impl Into<String>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let raw_val_1 = self
            .jni_ref()
            .new_object("java/util/ArrayList", "()V", &[])
            .unwrap();
        for v in arg0 {
            let map_val_0 =
                jni::objects::JObject::from(self.jni_ref().new_string(v.into()).unwrap());
            self.jni_ref().call_method(
                &raw_val_1,
                "add",
                "(Ljava/Lang/Object)V",
                &[jni::objects::JValueGen::from(&map_val_0)],
            )?;
        }
        let val_1 = jni::objects::JValueGen::Object(raw_val_1);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setLore",
            "(Ljava/util/List;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn as_string(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getAsString",
            "()Ljava/lang/String;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }

    pub fn has_display_name(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasDisplayName", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn set_display_name(
        &mut self,
        arg0: impl Into<&'mc String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setDisplayName",
            "(Ljava/lang/String;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn has_localized_name(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasLocalizedName", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn set_localized_name(
        &mut self,
        arg0: impl Into<&'mc String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setLocalizedName",
            "(Ljava/lang/String;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn has_lore(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasLore", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn lore(&mut self) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getLore", "()Ljava/util/List;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let mut list = blackboxmc_java::JavaList::from_raw(&self.jni_ref(), res.l()?)?;
        let size = list.size()?;
        for i in 0..=size {
            let obj = list.get(i)?;
            new_vec.push(
                self.jni_ref()
                    .get_string(unsafe { &jni::objects::JString::from_raw(*obj) })?
                    .to_string_lossy()
                    .to_string(),
            );
        }
        Ok(new_vec)
    }

    pub fn has_custom_model_data(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasCustomModelData", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn custom_model_data(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getCustomModelData", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }

    pub fn set_custom_model_data(&mut self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setCustomModelData",
            "(I)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn has_enchants(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasEnchants", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn has_enchant(
        &mut self,
        arg0: impl Into<&'mc crate::enchantments::Enchantment<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "hasEnchant",
            "(Lorg/bukkit/enchantments/Enchantment;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn enchants(
        &mut self,
    ) -> Result<blackboxmc_java::JavaMap<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getEnchants", "()Ljava/util/Map;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::JavaMap::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn add_enchant(
        &mut self,
        arg0: impl Into<&'mc crate::enchantments::Enchantment<'mc>>,
        arg1: i32,
        arg2: bool,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let val_2 = jni::objects::JValueGen::Int(arg1.into());
        // -2
        let val_3 = jni::objects::JValueGen::Bool(arg2.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "addEnchant",
            "(Lorg/bukkit/enchantments/Enchantment;IZ)Z",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
                jni::objects::JValueGen::from(&val_3),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn remove_enchant(
        &mut self,
        arg0: impl Into<&'mc crate::enchantments::Enchantment<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeEnchant",
            "(Lorg/bukkit/enchantments/Enchantment;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn has_conflicting_enchant(
        &mut self,
        arg0: impl Into<&'mc crate::enchantments::Enchantment<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "hasConflictingEnchant",
            "(Lorg/bukkit/enchantments/Enchantment;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn add_item_flags(
        &mut self,
        arg0: Vec<impl Into<crate::inventory::ItemFlag<'mc>>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "addItemFlags",
            "(Lorg/bukkit/inventory/ItemFlag;)V",
            &[],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn remove_item_flags(
        &mut self,
        arg0: Vec<impl Into<crate::inventory::ItemFlag<'mc>>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeItemFlags",
            "(Lorg/bukkit/inventory/ItemFlag;)V",
            &[],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn item_flags(
        &mut self,
    ) -> Result<blackboxmc_java::JavaSet<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getItemFlags",
            "()Ljava/util/Set;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::JavaSet::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn has_item_flag(
        &mut self,
        arg0: impl Into<&'mc crate::inventory::ItemFlag<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "hasItemFlag",
            "(Lorg/bukkit/inventory/ItemFlag;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn is_unbreakable(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isUnbreakable", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn set_unbreakable(&mut self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        // -2
        let val_1 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setUnbreakable",
            "(Z)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn has_attribute_modifiers(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "hasAttributeModifiers", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn attribute_modifiers(
        &mut self,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getAttributeModifiers",
            "()Lcom/google/common/collect/Multimap;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }

    pub fn add_attribute_modifier(
        &mut self,
        arg0: impl Into<&'mc crate::attribute::Attribute<'mc>>,
        arg1: impl Into<&'mc crate::attribute::AttributeModifier<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let val_2 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "addAttributeModifier",
            "(Lorg/bukkit/attribute/Attribute;Lorg/bukkit/attribute/AttributeModifier;)Z",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub unsafe fn set_attribute_modifiers(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setAttributeModifiers",
            "(Lcom/google/common/collect/Multimap;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn remove_attribute_modifier_with_attribute(
        &mut self,
        arg0: impl Into<&'mc crate::attribute::Attribute<'mc>>,
        arg1: std::option::Option<impl Into<&'mc crate::attribute::AttributeModifier<'mc>>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let val_2 =
            unsafe { jni::objects::JObject::from_raw(arg1.unwrap().into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeAttributeModifier",
            "(Lorg/bukkit/attribute/Attribute;Lorg/bukkit/attribute/AttributeModifier;)Z",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    #[deprecated]

    pub fn custom_tag_container(
        &mut self,
    ) -> Result<crate::inventory::meta::tags::CustomItemTagContainer<'mc>, Box<dyn std::error::Error>>
    {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getCustomTagContainer",
            "()Lorg/bukkit/inventory/meta/tags/CustomItemTagContainer;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::meta::tags::CustomItemTagContainer::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    #[deprecated]

    pub fn set_version(&mut self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setVersion",
            "(I)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

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

    pub fn persistent_data_container(
        &mut self,
    ) -> Result<crate::persistence::PersistentDataContainer<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getPersistentDataContainer",
            "()Lorg/bukkit/persistence/PersistentDataContainer;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::persistence::PersistentDataContainer::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
}
impl<'mc> JNIRaw<'mc> for Repairable<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Into<crate::inventory::meta::ItemMeta<'mc>> for Repairable<'mc> {
    fn into(self) -> crate::inventory::meta::ItemMeta<'mc> {
        crate::inventory::meta::ItemMeta::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}

///
/// This is a representation of an abstract class.
pub struct BannerMeta<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> BannerMeta<'mc> {
    pub fn from_raw(
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
    #[deprecated]
    /// <span class="deprecated-label">Deprecated.</span>
    /// <div class="deprecation-comment">
    /// banner color is now stored as the data value, not meta.
    /// </div>
    /// banner color is now stored as the data value, not meta.
    ///
    /// Returns the base color for this banner
    pub fn base_color(&mut self) -> Result<crate::DyeColor<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getBaseColor",
            "()Lorg/bukkit/DyeColor;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant =
            self.jni_ref()
                .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
        let variant_str = self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::DyeColor::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::DyeColor::from_string(variant_str).unwrap(),
        )
    }
    #[deprecated]
    /// <span class="deprecated-label">Deprecated.</span>
    /// <div class="deprecation-comment">
    /// banner color is now stored as the data value, not meta.
    /// </div>
    /// banner color is now stored as the data value, not meta.
    ///
    /// Sets the base color for this banner
    pub fn set_base_color(
        &mut self,
        arg0: impl Into<&'mc crate::DyeColor<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setBaseColor",
            "(Lorg/bukkit/DyeColor;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Returns a list of patterns on this banner
    pub fn patterns(
        &mut self,
    ) -> Result<Vec<crate::block::banner::Pattern<'mc>>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getPatterns",
            "()Ljava/util/List;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let mut list = blackboxmc_java::JavaList::from_raw(&self.jni_ref(), res.l()?)?;
        let size = list.size()?;
        for i in 0..=size {
            let obj = list.get(i)?;
            new_vec.push(crate::block::banner::Pattern::from_raw(
                &self.jni_ref(),
                obj,
            )?);
        }
        Ok(new_vec)
    }
    /// Sets the patterns used on this banner
    pub fn set_patterns(
        &mut self,
        arg0: Vec<impl Into<crate::block::banner::Pattern<'mc>>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let raw_val_1 = self
            .jni_ref()
            .new_object("java/util/ArrayList", "()V", &[])
            .unwrap();
        for v in arg0 {
            let map_val_0 =
                unsafe { jni::objects::JObject::from_raw(v.into().jni_object().clone()) };
            self.jni_ref().call_method(
                &raw_val_1,
                "add",
                "(Ljava/Lang/Object)V",
                &[jni::objects::JValueGen::from(&map_val_0)],
            )?;
        }
        let val_1 = jni::objects::JValueGen::Object(raw_val_1);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setPatterns",
            "(Ljava/util/List;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Adds a new pattern on top of the existing patterns
    pub fn add_pattern(
        &mut self,
        arg0: impl Into<&'mc crate::block::banner::Pattern<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "addPattern",
            "(Lorg/bukkit/block/banner/Pattern;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Returns a list of patterns on this banner
    /// Returns the pattern at the specified index
    pub fn get_pattern(
        &mut self,
        arg0: i32,
    ) -> Result<crate::block::banner::Pattern<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getPattern",
            "(I)Lorg/bukkit/block/banner/Pattern;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::block::banner::Pattern::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Removes the pattern at the specified index
    pub fn remove_pattern(
        &mut self,
        arg0: i32,
    ) -> Result<crate::block::banner::Pattern<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removePattern",
            "(I)Lorg/bukkit/block/banner/Pattern;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::block::banner::Pattern::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Sets the patterns used on this banner
    /// Sets the pattern at the specified index
    pub fn set_pattern(
        &mut self,
        arg0: i32,
        arg1: impl Into<&'mc crate::block::banner::Pattern<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let val_2 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setPattern",
            "(ILorg/bukkit/block/banner/Pattern;)V",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Returns the number of patterns on this banner
    pub fn number_of_patterns(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "numberOfPatterns", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }

    pub fn display_name(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getDisplayName",
            "()Ljava/lang/String;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }

    pub fn localized_name(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getLocalizedName",
            "()Ljava/lang/String;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }

    pub fn get_enchant_level(
        &mut self,
        arg0: impl Into<&'mc crate::enchantments::Enchantment<'mc>>,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getEnchantLevel",
            "(Lorg/bukkit/enchantments/Enchantment;)I",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }

    pub fn set_lore(
        &mut self,
        arg0: Vec<impl Into<String>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let raw_val_1 = self
            .jni_ref()
            .new_object("java/util/ArrayList", "()V", &[])
            .unwrap();
        for v in arg0 {
            let map_val_0 =
                jni::objects::JObject::from(self.jni_ref().new_string(v.into()).unwrap());
            self.jni_ref().call_method(
                &raw_val_1,
                "add",
                "(Ljava/Lang/Object)V",
                &[jni::objects::JValueGen::from(&map_val_0)],
            )?;
        }
        let val_1 = jni::objects::JValueGen::Object(raw_val_1);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setLore",
            "(Ljava/util/List;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn as_string(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getAsString",
            "()Ljava/lang/String;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }

    pub fn has_display_name(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasDisplayName", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn set_display_name(
        &mut self,
        arg0: impl Into<&'mc String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setDisplayName",
            "(Ljava/lang/String;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn has_localized_name(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasLocalizedName", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn set_localized_name(
        &mut self,
        arg0: impl Into<&'mc String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setLocalizedName",
            "(Ljava/lang/String;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn has_lore(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasLore", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn lore(&mut self) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getLore", "()Ljava/util/List;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let mut list = blackboxmc_java::JavaList::from_raw(&self.jni_ref(), res.l()?)?;
        let size = list.size()?;
        for i in 0..=size {
            let obj = list.get(i)?;
            new_vec.push(
                self.jni_ref()
                    .get_string(unsafe { &jni::objects::JString::from_raw(*obj) })?
                    .to_string_lossy()
                    .to_string(),
            );
        }
        Ok(new_vec)
    }

    pub fn has_custom_model_data(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasCustomModelData", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn custom_model_data(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getCustomModelData", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }

    pub fn set_custom_model_data(&mut self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setCustomModelData",
            "(I)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn has_enchants(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasEnchants", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn has_enchant(
        &mut self,
        arg0: impl Into<&'mc crate::enchantments::Enchantment<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "hasEnchant",
            "(Lorg/bukkit/enchantments/Enchantment;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn enchants(
        &mut self,
    ) -> Result<blackboxmc_java::JavaMap<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getEnchants", "()Ljava/util/Map;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::JavaMap::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn add_enchant(
        &mut self,
        arg0: impl Into<&'mc crate::enchantments::Enchantment<'mc>>,
        arg1: i32,
        arg2: bool,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let val_2 = jni::objects::JValueGen::Int(arg1.into());
        // -2
        let val_3 = jni::objects::JValueGen::Bool(arg2.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "addEnchant",
            "(Lorg/bukkit/enchantments/Enchantment;IZ)Z",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
                jni::objects::JValueGen::from(&val_3),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn remove_enchant(
        &mut self,
        arg0: impl Into<&'mc crate::enchantments::Enchantment<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeEnchant",
            "(Lorg/bukkit/enchantments/Enchantment;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn has_conflicting_enchant(
        &mut self,
        arg0: impl Into<&'mc crate::enchantments::Enchantment<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "hasConflictingEnchant",
            "(Lorg/bukkit/enchantments/Enchantment;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn add_item_flags(
        &mut self,
        arg0: Vec<impl Into<crate::inventory::ItemFlag<'mc>>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "addItemFlags",
            "(Lorg/bukkit/inventory/ItemFlag;)V",
            &[],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn remove_item_flags(
        &mut self,
        arg0: Vec<impl Into<crate::inventory::ItemFlag<'mc>>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeItemFlags",
            "(Lorg/bukkit/inventory/ItemFlag;)V",
            &[],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn item_flags(
        &mut self,
    ) -> Result<blackboxmc_java::JavaSet<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getItemFlags",
            "()Ljava/util/Set;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::JavaSet::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn has_item_flag(
        &mut self,
        arg0: impl Into<&'mc crate::inventory::ItemFlag<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "hasItemFlag",
            "(Lorg/bukkit/inventory/ItemFlag;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn is_unbreakable(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isUnbreakable", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn set_unbreakable(&mut self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        // -2
        let val_1 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setUnbreakable",
            "(Z)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn has_attribute_modifiers(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "hasAttributeModifiers", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn attribute_modifiers(
        &mut self,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getAttributeModifiers",
            "()Lcom/google/common/collect/Multimap;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }

    pub fn add_attribute_modifier(
        &mut self,
        arg0: impl Into<&'mc crate::attribute::Attribute<'mc>>,
        arg1: impl Into<&'mc crate::attribute::AttributeModifier<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let val_2 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "addAttributeModifier",
            "(Lorg/bukkit/attribute/Attribute;Lorg/bukkit/attribute/AttributeModifier;)Z",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub unsafe fn set_attribute_modifiers(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setAttributeModifiers",
            "(Lcom/google/common/collect/Multimap;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn remove_attribute_modifier_with_attribute(
        &mut self,
        arg0: impl Into<&'mc crate::attribute::Attribute<'mc>>,
        arg1: std::option::Option<impl Into<&'mc crate::attribute::AttributeModifier<'mc>>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let val_2 =
            unsafe { jni::objects::JObject::from_raw(arg1.unwrap().into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeAttributeModifier",
            "(Lorg/bukkit/attribute/Attribute;Lorg/bukkit/attribute/AttributeModifier;)Z",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    #[deprecated]

    pub fn custom_tag_container(
        &mut self,
    ) -> Result<crate::inventory::meta::tags::CustomItemTagContainer<'mc>, Box<dyn std::error::Error>>
    {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getCustomTagContainer",
            "()Lorg/bukkit/inventory/meta/tags/CustomItemTagContainer;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::meta::tags::CustomItemTagContainer::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    #[deprecated]

    pub fn set_version(&mut self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setVersion",
            "(I)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn clone(&mut self) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "clone", "()Ljava/lang/Object;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }

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

    pub fn persistent_data_container(
        &mut self,
    ) -> Result<crate::persistence::PersistentDataContainer<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getPersistentDataContainer",
            "()Lorg/bukkit/persistence/PersistentDataContainer;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::persistence::PersistentDataContainer::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
}
impl<'mc> JNIRaw<'mc> for BannerMeta<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Into<crate::inventory::meta::ItemMeta<'mc>> for BannerMeta<'mc> {
    fn into(self) -> crate::inventory::meta::ItemMeta<'mc> {
        crate::inventory::meta::ItemMeta::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}

///
/// This is a representation of an abstract class.
pub struct BundleMeta<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> BundleMeta<'mc> {
    pub fn from_raw(
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
    /// Adds an item to this item.
    pub fn add_item(
        &mut self,
        arg0: impl Into<&'mc crate::inventory::ItemStack<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "addItem",
            "(Lorg/bukkit/inventory/ItemStack;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Returns whether the item has any items.
    pub fn has_items(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasItems", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    /// Returns an immutable list of the items stored in this item.
    pub fn items(
        &mut self,
    ) -> Result<Vec<crate::inventory::ItemStack<'mc>>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getItems", "()Ljava/util/List;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let mut list = blackboxmc_java::JavaList::from_raw(&self.jni_ref(), res.l()?)?;
        let size = list.size()?;
        for i in 0..=size {
            let obj = list.get(i)?;
            new_vec.push(crate::inventory::ItemStack::from_raw(&self.jni_ref(), obj)?);
        }
        Ok(new_vec)
    }
    /// Sets the items stored in this item. Removes all items when given null.
    pub fn set_items(
        &mut self,
        arg0: Vec<impl Into<crate::inventory::ItemStack<'mc>>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let raw_val_1 = self
            .jni_ref()
            .new_object("java/util/ArrayList", "()V", &[])
            .unwrap();
        for v in arg0 {
            let map_val_0 =
                unsafe { jni::objects::JObject::from_raw(v.into().jni_object().clone()) };
            self.jni_ref().call_method(
                &raw_val_1,
                "add",
                "(Ljava/Lang/Object)V",
                &[jni::objects::JValueGen::from(&map_val_0)],
            )?;
        }
        let val_1 = jni::objects::JValueGen::Object(raw_val_1);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setItems",
            "(Ljava/util/List;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn display_name(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getDisplayName",
            "()Ljava/lang/String;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }

    pub fn localized_name(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getLocalizedName",
            "()Ljava/lang/String;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }

    pub fn get_enchant_level(
        &mut self,
        arg0: impl Into<&'mc crate::enchantments::Enchantment<'mc>>,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getEnchantLevel",
            "(Lorg/bukkit/enchantments/Enchantment;)I",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }

    pub fn set_lore(
        &mut self,
        arg0: Vec<impl Into<String>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let raw_val_1 = self
            .jni_ref()
            .new_object("java/util/ArrayList", "()V", &[])
            .unwrap();
        for v in arg0 {
            let map_val_0 =
                jni::objects::JObject::from(self.jni_ref().new_string(v.into()).unwrap());
            self.jni_ref().call_method(
                &raw_val_1,
                "add",
                "(Ljava/Lang/Object)V",
                &[jni::objects::JValueGen::from(&map_val_0)],
            )?;
        }
        let val_1 = jni::objects::JValueGen::Object(raw_val_1);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setLore",
            "(Ljava/util/List;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn as_string(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getAsString",
            "()Ljava/lang/String;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }

    pub fn has_display_name(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasDisplayName", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn set_display_name(
        &mut self,
        arg0: impl Into<&'mc String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setDisplayName",
            "(Ljava/lang/String;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn has_localized_name(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasLocalizedName", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn set_localized_name(
        &mut self,
        arg0: impl Into<&'mc String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setLocalizedName",
            "(Ljava/lang/String;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn has_lore(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasLore", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn lore(&mut self) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getLore", "()Ljava/util/List;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let mut list = blackboxmc_java::JavaList::from_raw(&self.jni_ref(), res.l()?)?;
        let size = list.size()?;
        for i in 0..=size {
            let obj = list.get(i)?;
            new_vec.push(
                self.jni_ref()
                    .get_string(unsafe { &jni::objects::JString::from_raw(*obj) })?
                    .to_string_lossy()
                    .to_string(),
            );
        }
        Ok(new_vec)
    }

    pub fn has_custom_model_data(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasCustomModelData", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn custom_model_data(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getCustomModelData", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }

    pub fn set_custom_model_data(&mut self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setCustomModelData",
            "(I)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn has_enchants(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasEnchants", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn has_enchant(
        &mut self,
        arg0: impl Into<&'mc crate::enchantments::Enchantment<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "hasEnchant",
            "(Lorg/bukkit/enchantments/Enchantment;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn enchants(
        &mut self,
    ) -> Result<blackboxmc_java::JavaMap<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getEnchants", "()Ljava/util/Map;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::JavaMap::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn add_enchant(
        &mut self,
        arg0: impl Into<&'mc crate::enchantments::Enchantment<'mc>>,
        arg1: i32,
        arg2: bool,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let val_2 = jni::objects::JValueGen::Int(arg1.into());
        // -2
        let val_3 = jni::objects::JValueGen::Bool(arg2.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "addEnchant",
            "(Lorg/bukkit/enchantments/Enchantment;IZ)Z",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
                jni::objects::JValueGen::from(&val_3),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn remove_enchant(
        &mut self,
        arg0: impl Into<&'mc crate::enchantments::Enchantment<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeEnchant",
            "(Lorg/bukkit/enchantments/Enchantment;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn has_conflicting_enchant(
        &mut self,
        arg0: impl Into<&'mc crate::enchantments::Enchantment<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "hasConflictingEnchant",
            "(Lorg/bukkit/enchantments/Enchantment;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn add_item_flags(
        &mut self,
        arg0: Vec<impl Into<crate::inventory::ItemFlag<'mc>>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "addItemFlags",
            "(Lorg/bukkit/inventory/ItemFlag;)V",
            &[],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn remove_item_flags(
        &mut self,
        arg0: Vec<impl Into<crate::inventory::ItemFlag<'mc>>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeItemFlags",
            "(Lorg/bukkit/inventory/ItemFlag;)V",
            &[],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn item_flags(
        &mut self,
    ) -> Result<blackboxmc_java::JavaSet<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getItemFlags",
            "()Ljava/util/Set;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::JavaSet::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn has_item_flag(
        &mut self,
        arg0: impl Into<&'mc crate::inventory::ItemFlag<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "hasItemFlag",
            "(Lorg/bukkit/inventory/ItemFlag;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn is_unbreakable(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isUnbreakable", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn set_unbreakable(&mut self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        // -2
        let val_1 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setUnbreakable",
            "(Z)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn has_attribute_modifiers(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "hasAttributeModifiers", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn attribute_modifiers(
        &mut self,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getAttributeModifiers",
            "()Lcom/google/common/collect/Multimap;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }

    pub fn add_attribute_modifier(
        &mut self,
        arg0: impl Into<&'mc crate::attribute::Attribute<'mc>>,
        arg1: impl Into<&'mc crate::attribute::AttributeModifier<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let val_2 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "addAttributeModifier",
            "(Lorg/bukkit/attribute/Attribute;Lorg/bukkit/attribute/AttributeModifier;)Z",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub unsafe fn set_attribute_modifiers(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setAttributeModifiers",
            "(Lcom/google/common/collect/Multimap;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn remove_attribute_modifier_with_attribute(
        &mut self,
        arg0: impl Into<&'mc crate::attribute::Attribute<'mc>>,
        arg1: std::option::Option<impl Into<&'mc crate::attribute::AttributeModifier<'mc>>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let val_2 =
            unsafe { jni::objects::JObject::from_raw(arg1.unwrap().into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeAttributeModifier",
            "(Lorg/bukkit/attribute/Attribute;Lorg/bukkit/attribute/AttributeModifier;)Z",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    #[deprecated]

    pub fn custom_tag_container(
        &mut self,
    ) -> Result<crate::inventory::meta::tags::CustomItemTagContainer<'mc>, Box<dyn std::error::Error>>
    {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getCustomTagContainer",
            "()Lorg/bukkit/inventory/meta/tags/CustomItemTagContainer;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::meta::tags::CustomItemTagContainer::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    #[deprecated]

    pub fn set_version(&mut self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setVersion",
            "(I)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn clone(&mut self) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "clone", "()Ljava/lang/Object;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }

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

    pub fn persistent_data_container(
        &mut self,
    ) -> Result<crate::persistence::PersistentDataContainer<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getPersistentDataContainer",
            "()Lorg/bukkit/persistence/PersistentDataContainer;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::persistence::PersistentDataContainer::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
}
impl<'mc> JNIRaw<'mc> for BundleMeta<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Into<crate::inventory::meta::ItemMeta<'mc>> for BundleMeta<'mc> {
    fn into(self) -> crate::inventory::meta::ItemMeta<'mc> {
        crate::inventory::meta::ItemMeta::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
/// Represents armor that an entity can equip and can also be colored.
///
/// This is a representation of an abstract class.
pub struct ColorableArmorMeta<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> ColorableArmorMeta<'mc> {
    pub fn from_raw(
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

    pub fn clone(
        &mut self,
    ) -> Result<crate::inventory::meta::ColorableArmorMeta<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "clone",
            "()Lorg/bukkit/inventory/meta/ColorableArmorMeta;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::meta::ColorableArmorMeta::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn has_trim(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasTrim", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn set_trim(
        &mut self,
        arg0: impl Into<&'mc crate::inventory::meta::trim::ArmorTrim<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setTrim",
            "(Lorg/bukkit/inventory/meta/trim/ArmorTrim;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn trim(
        &mut self,
    ) -> Result<crate::inventory::meta::trim::ArmorTrim<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getTrim",
            "()Lorg/bukkit/inventory/meta/trim/ArmorTrim;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::meta::trim::ArmorTrim::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn display_name(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getDisplayName",
            "()Ljava/lang/String;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }

    pub fn localized_name(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getLocalizedName",
            "()Ljava/lang/String;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }

    pub fn get_enchant_level(
        &mut self,
        arg0: impl Into<&'mc crate::enchantments::Enchantment<'mc>>,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getEnchantLevel",
            "(Lorg/bukkit/enchantments/Enchantment;)I",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }

    pub fn set_lore(
        &mut self,
        arg0: Vec<impl Into<String>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let raw_val_1 = self
            .jni_ref()
            .new_object("java/util/ArrayList", "()V", &[])
            .unwrap();
        for v in arg0 {
            let map_val_0 =
                jni::objects::JObject::from(self.jni_ref().new_string(v.into()).unwrap());
            self.jni_ref().call_method(
                &raw_val_1,
                "add",
                "(Ljava/Lang/Object)V",
                &[jni::objects::JValueGen::from(&map_val_0)],
            )?;
        }
        let val_1 = jni::objects::JValueGen::Object(raw_val_1);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setLore",
            "(Ljava/util/List;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn as_string(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getAsString",
            "()Ljava/lang/String;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }

    pub fn has_display_name(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasDisplayName", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn set_display_name(
        &mut self,
        arg0: impl Into<&'mc String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setDisplayName",
            "(Ljava/lang/String;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn has_localized_name(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasLocalizedName", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn set_localized_name(
        &mut self,
        arg0: impl Into<&'mc String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setLocalizedName",
            "(Ljava/lang/String;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn has_lore(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasLore", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn lore(&mut self) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getLore", "()Ljava/util/List;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let mut list = blackboxmc_java::JavaList::from_raw(&self.jni_ref(), res.l()?)?;
        let size = list.size()?;
        for i in 0..=size {
            let obj = list.get(i)?;
            new_vec.push(
                self.jni_ref()
                    .get_string(unsafe { &jni::objects::JString::from_raw(*obj) })?
                    .to_string_lossy()
                    .to_string(),
            );
        }
        Ok(new_vec)
    }

    pub fn has_custom_model_data(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasCustomModelData", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn custom_model_data(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getCustomModelData", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }

    pub fn set_custom_model_data(&mut self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setCustomModelData",
            "(I)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn has_enchants(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasEnchants", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn has_enchant(
        &mut self,
        arg0: impl Into<&'mc crate::enchantments::Enchantment<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "hasEnchant",
            "(Lorg/bukkit/enchantments/Enchantment;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn enchants(
        &mut self,
    ) -> Result<blackboxmc_java::JavaMap<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getEnchants", "()Ljava/util/Map;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::JavaMap::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn add_enchant(
        &mut self,
        arg0: impl Into<&'mc crate::enchantments::Enchantment<'mc>>,
        arg1: i32,
        arg2: bool,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let val_2 = jni::objects::JValueGen::Int(arg1.into());
        // -2
        let val_3 = jni::objects::JValueGen::Bool(arg2.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "addEnchant",
            "(Lorg/bukkit/enchantments/Enchantment;IZ)Z",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
                jni::objects::JValueGen::from(&val_3),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn remove_enchant(
        &mut self,
        arg0: impl Into<&'mc crate::enchantments::Enchantment<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeEnchant",
            "(Lorg/bukkit/enchantments/Enchantment;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn has_conflicting_enchant(
        &mut self,
        arg0: impl Into<&'mc crate::enchantments::Enchantment<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "hasConflictingEnchant",
            "(Lorg/bukkit/enchantments/Enchantment;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn add_item_flags(
        &mut self,
        arg0: Vec<impl Into<crate::inventory::ItemFlag<'mc>>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "addItemFlags",
            "(Lorg/bukkit/inventory/ItemFlag;)V",
            &[],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn remove_item_flags(
        &mut self,
        arg0: Vec<impl Into<crate::inventory::ItemFlag<'mc>>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeItemFlags",
            "(Lorg/bukkit/inventory/ItemFlag;)V",
            &[],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn item_flags(
        &mut self,
    ) -> Result<blackboxmc_java::JavaSet<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getItemFlags",
            "()Ljava/util/Set;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::JavaSet::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn has_item_flag(
        &mut self,
        arg0: impl Into<&'mc crate::inventory::ItemFlag<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "hasItemFlag",
            "(Lorg/bukkit/inventory/ItemFlag;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn is_unbreakable(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isUnbreakable", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn set_unbreakable(&mut self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        // -2
        let val_1 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setUnbreakable",
            "(Z)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn has_attribute_modifiers(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "hasAttributeModifiers", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn attribute_modifiers(
        &mut self,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getAttributeModifiers",
            "()Lcom/google/common/collect/Multimap;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }

    pub fn add_attribute_modifier(
        &mut self,
        arg0: impl Into<&'mc crate::attribute::Attribute<'mc>>,
        arg1: impl Into<&'mc crate::attribute::AttributeModifier<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let val_2 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "addAttributeModifier",
            "(Lorg/bukkit/attribute/Attribute;Lorg/bukkit/attribute/AttributeModifier;)Z",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub unsafe fn set_attribute_modifiers(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setAttributeModifiers",
            "(Lcom/google/common/collect/Multimap;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn remove_attribute_modifier_with_attribute(
        &mut self,
        arg0: impl Into<&'mc crate::attribute::Attribute<'mc>>,
        arg1: std::option::Option<impl Into<&'mc crate::attribute::AttributeModifier<'mc>>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let val_2 =
            unsafe { jni::objects::JObject::from_raw(arg1.unwrap().into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeAttributeModifier",
            "(Lorg/bukkit/attribute/Attribute;Lorg/bukkit/attribute/AttributeModifier;)Z",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    #[deprecated]

    pub fn custom_tag_container(
        &mut self,
    ) -> Result<crate::inventory::meta::tags::CustomItemTagContainer<'mc>, Box<dyn std::error::Error>>
    {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getCustomTagContainer",
            "()Lorg/bukkit/inventory/meta/tags/CustomItemTagContainer;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::meta::tags::CustomItemTagContainer::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    #[deprecated]

    pub fn set_version(&mut self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setVersion",
            "(I)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

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

    pub fn persistent_data_container(
        &mut self,
    ) -> Result<crate::persistence::PersistentDataContainer<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getPersistentDataContainer",
            "()Lorg/bukkit/persistence/PersistentDataContainer;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::persistence::PersistentDataContainer::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn set_color(
        &mut self,
        arg0: impl Into<&'mc crate::Color<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setColor",
            "(Lorg/bukkit/Color;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn color(&mut self) -> Result<crate::Color<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getColor", "()Lorg/bukkit/Color;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        crate::Color::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
}
impl<'mc> JNIRaw<'mc> for ColorableArmorMeta<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Into<crate::inventory::meta::ArmorMeta<'mc>> for ColorableArmorMeta<'mc> {
    fn into(self) -> crate::inventory::meta::ArmorMeta<'mc> {
        crate::inventory::meta::ArmorMeta::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
impl<'mc> Into<crate::inventory::meta::LeatherArmorMeta<'mc>> for ColorableArmorMeta<'mc> {
    fn into(self) -> crate::inventory::meta::LeatherArmorMeta<'mc> {
        crate::inventory::meta::LeatherArmorMeta::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
/// Represents a spawn egg and it's spawned type.
///
/// This is a representation of an abstract class.
pub struct SpawnEggMeta<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> SpawnEggMeta<'mc> {
    pub fn from_raw(
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
    #[deprecated]
    /// <span class="deprecated-label">Deprecated.</span>
    /// <div class="deprecation-comment">
    /// different types are different items
    /// </div>
    /// different types are different items
    ///
    /// Get the type of entity this egg will spawn.
    pub fn spawned_type(
        &mut self,
    ) -> Result<crate::entity::EntityType<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getSpawnedType",
            "()Lorg/bukkit/entity/EntityType;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant =
            self.jni_ref()
                .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
        let variant_str = self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::entity::EntityType::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::entity::EntityType::from_string(variant_str).unwrap(),
        )
    }
    #[deprecated]
    /// <span class="deprecated-label">Deprecated.</span>
    /// <div class="deprecation-comment">
    /// different types are different items
    /// </div>
    /// different types are different items
    ///
    /// Set the type of entity this egg will spawn.
    pub fn set_spawned_type(
        &mut self,
        arg0: impl Into<&'mc crate::entity::EntityType<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setSpawnedType",
            "(Lorg/bukkit/entity/EntityType;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn clone(
        &mut self,
    ) -> Result<crate::inventory::meta::SpawnEggMeta<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "clone",
            "()Lorg/bukkit/inventory/meta/SpawnEggMeta;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::meta::SpawnEggMeta::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn display_name(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getDisplayName",
            "()Ljava/lang/String;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }

    pub fn localized_name(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getLocalizedName",
            "()Ljava/lang/String;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }

    pub fn get_enchant_level(
        &mut self,
        arg0: impl Into<&'mc crate::enchantments::Enchantment<'mc>>,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getEnchantLevel",
            "(Lorg/bukkit/enchantments/Enchantment;)I",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }

    pub fn set_lore(
        &mut self,
        arg0: Vec<impl Into<String>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let raw_val_1 = self
            .jni_ref()
            .new_object("java/util/ArrayList", "()V", &[])
            .unwrap();
        for v in arg0 {
            let map_val_0 =
                jni::objects::JObject::from(self.jni_ref().new_string(v.into()).unwrap());
            self.jni_ref().call_method(
                &raw_val_1,
                "add",
                "(Ljava/Lang/Object)V",
                &[jni::objects::JValueGen::from(&map_val_0)],
            )?;
        }
        let val_1 = jni::objects::JValueGen::Object(raw_val_1);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setLore",
            "(Ljava/util/List;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn as_string(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getAsString",
            "()Ljava/lang/String;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }

    pub fn has_display_name(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasDisplayName", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn set_display_name(
        &mut self,
        arg0: impl Into<&'mc String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setDisplayName",
            "(Ljava/lang/String;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn has_localized_name(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasLocalizedName", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn set_localized_name(
        &mut self,
        arg0: impl Into<&'mc String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setLocalizedName",
            "(Ljava/lang/String;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn has_lore(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasLore", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn lore(&mut self) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getLore", "()Ljava/util/List;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let mut list = blackboxmc_java::JavaList::from_raw(&self.jni_ref(), res.l()?)?;
        let size = list.size()?;
        for i in 0..=size {
            let obj = list.get(i)?;
            new_vec.push(
                self.jni_ref()
                    .get_string(unsafe { &jni::objects::JString::from_raw(*obj) })?
                    .to_string_lossy()
                    .to_string(),
            );
        }
        Ok(new_vec)
    }

    pub fn has_custom_model_data(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasCustomModelData", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn custom_model_data(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getCustomModelData", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }

    pub fn set_custom_model_data(&mut self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setCustomModelData",
            "(I)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn has_enchants(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasEnchants", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn has_enchant(
        &mut self,
        arg0: impl Into<&'mc crate::enchantments::Enchantment<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "hasEnchant",
            "(Lorg/bukkit/enchantments/Enchantment;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn enchants(
        &mut self,
    ) -> Result<blackboxmc_java::JavaMap<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getEnchants", "()Ljava/util/Map;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::JavaMap::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn add_enchant(
        &mut self,
        arg0: impl Into<&'mc crate::enchantments::Enchantment<'mc>>,
        arg1: i32,
        arg2: bool,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let val_2 = jni::objects::JValueGen::Int(arg1.into());
        // -2
        let val_3 = jni::objects::JValueGen::Bool(arg2.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "addEnchant",
            "(Lorg/bukkit/enchantments/Enchantment;IZ)Z",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
                jni::objects::JValueGen::from(&val_3),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn remove_enchant(
        &mut self,
        arg0: impl Into<&'mc crate::enchantments::Enchantment<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeEnchant",
            "(Lorg/bukkit/enchantments/Enchantment;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn has_conflicting_enchant(
        &mut self,
        arg0: impl Into<&'mc crate::enchantments::Enchantment<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "hasConflictingEnchant",
            "(Lorg/bukkit/enchantments/Enchantment;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn add_item_flags(
        &mut self,
        arg0: Vec<impl Into<crate::inventory::ItemFlag<'mc>>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "addItemFlags",
            "(Lorg/bukkit/inventory/ItemFlag;)V",
            &[],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn remove_item_flags(
        &mut self,
        arg0: Vec<impl Into<crate::inventory::ItemFlag<'mc>>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeItemFlags",
            "(Lorg/bukkit/inventory/ItemFlag;)V",
            &[],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn item_flags(
        &mut self,
    ) -> Result<blackboxmc_java::JavaSet<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getItemFlags",
            "()Ljava/util/Set;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::JavaSet::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn has_item_flag(
        &mut self,
        arg0: impl Into<&'mc crate::inventory::ItemFlag<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "hasItemFlag",
            "(Lorg/bukkit/inventory/ItemFlag;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn is_unbreakable(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isUnbreakable", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn set_unbreakable(&mut self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        // -2
        let val_1 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setUnbreakable",
            "(Z)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn has_attribute_modifiers(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "hasAttributeModifiers", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn attribute_modifiers(
        &mut self,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getAttributeModifiers",
            "()Lcom/google/common/collect/Multimap;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }

    pub fn add_attribute_modifier(
        &mut self,
        arg0: impl Into<&'mc crate::attribute::Attribute<'mc>>,
        arg1: impl Into<&'mc crate::attribute::AttributeModifier<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let val_2 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "addAttributeModifier",
            "(Lorg/bukkit/attribute/Attribute;Lorg/bukkit/attribute/AttributeModifier;)Z",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub unsafe fn set_attribute_modifiers(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setAttributeModifiers",
            "(Lcom/google/common/collect/Multimap;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn remove_attribute_modifier_with_attribute(
        &mut self,
        arg0: impl Into<&'mc crate::attribute::Attribute<'mc>>,
        arg1: std::option::Option<impl Into<&'mc crate::attribute::AttributeModifier<'mc>>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let val_2 =
            unsafe { jni::objects::JObject::from_raw(arg1.unwrap().into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeAttributeModifier",
            "(Lorg/bukkit/attribute/Attribute;Lorg/bukkit/attribute/AttributeModifier;)Z",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    #[deprecated]

    pub fn custom_tag_container(
        &mut self,
    ) -> Result<crate::inventory::meta::tags::CustomItemTagContainer<'mc>, Box<dyn std::error::Error>>
    {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getCustomTagContainer",
            "()Lorg/bukkit/inventory/meta/tags/CustomItemTagContainer;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::meta::tags::CustomItemTagContainer::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    #[deprecated]

    pub fn set_version(&mut self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setVersion",
            "(I)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

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

    pub fn persistent_data_container(
        &mut self,
    ) -> Result<crate::persistence::PersistentDataContainer<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getPersistentDataContainer",
            "()Lorg/bukkit/persistence/PersistentDataContainer;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::persistence::PersistentDataContainer::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
}
impl<'mc> JNIRaw<'mc> for SpawnEggMeta<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Into<crate::inventory::meta::ItemMeta<'mc>> for SpawnEggMeta<'mc> {
    fn into(self) -> crate::inventory::meta::ItemMeta<'mc> {
        crate::inventory::meta::ItemMeta::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
/// Represents an item that has durability and can take damage.
///
/// This is a representation of an abstract class.
pub struct Damageable<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> Damageable<'mc> {
    pub fn from_raw(
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
    /// Sets the damage
    pub fn set_damage(&mut self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setDamage",
            "(I)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Gets the damage
    pub fn damage(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getDamage", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }
    /// Checks to see if this item has damage
    pub fn has_damage(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasDamage", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn clone(
        &mut self,
    ) -> Result<crate::inventory::meta::Damageable<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "clone",
            "()Lorg/bukkit/inventory/meta/Damageable;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::meta::Damageable::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn display_name(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getDisplayName",
            "()Ljava/lang/String;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }

    pub fn localized_name(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getLocalizedName",
            "()Ljava/lang/String;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }

    pub fn get_enchant_level(
        &mut self,
        arg0: impl Into<&'mc crate::enchantments::Enchantment<'mc>>,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getEnchantLevel",
            "(Lorg/bukkit/enchantments/Enchantment;)I",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }

    pub fn set_lore(
        &mut self,
        arg0: Vec<impl Into<String>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let raw_val_1 = self
            .jni_ref()
            .new_object("java/util/ArrayList", "()V", &[])
            .unwrap();
        for v in arg0 {
            let map_val_0 =
                jni::objects::JObject::from(self.jni_ref().new_string(v.into()).unwrap());
            self.jni_ref().call_method(
                &raw_val_1,
                "add",
                "(Ljava/Lang/Object)V",
                &[jni::objects::JValueGen::from(&map_val_0)],
            )?;
        }
        let val_1 = jni::objects::JValueGen::Object(raw_val_1);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setLore",
            "(Ljava/util/List;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn as_string(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getAsString",
            "()Ljava/lang/String;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }

    pub fn has_display_name(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasDisplayName", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn set_display_name(
        &mut self,
        arg0: impl Into<&'mc String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setDisplayName",
            "(Ljava/lang/String;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn has_localized_name(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasLocalizedName", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn set_localized_name(
        &mut self,
        arg0: impl Into<&'mc String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setLocalizedName",
            "(Ljava/lang/String;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn has_lore(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasLore", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn lore(&mut self) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getLore", "()Ljava/util/List;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let mut list = blackboxmc_java::JavaList::from_raw(&self.jni_ref(), res.l()?)?;
        let size = list.size()?;
        for i in 0..=size {
            let obj = list.get(i)?;
            new_vec.push(
                self.jni_ref()
                    .get_string(unsafe { &jni::objects::JString::from_raw(*obj) })?
                    .to_string_lossy()
                    .to_string(),
            );
        }
        Ok(new_vec)
    }

    pub fn has_custom_model_data(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasCustomModelData", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn custom_model_data(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getCustomModelData", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }

    pub fn set_custom_model_data(&mut self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setCustomModelData",
            "(I)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn has_enchants(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasEnchants", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn has_enchant(
        &mut self,
        arg0: impl Into<&'mc crate::enchantments::Enchantment<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "hasEnchant",
            "(Lorg/bukkit/enchantments/Enchantment;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn enchants(
        &mut self,
    ) -> Result<blackboxmc_java::JavaMap<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getEnchants", "()Ljava/util/Map;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::JavaMap::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn add_enchant(
        &mut self,
        arg0: impl Into<&'mc crate::enchantments::Enchantment<'mc>>,
        arg1: i32,
        arg2: bool,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let val_2 = jni::objects::JValueGen::Int(arg1.into());
        // -2
        let val_3 = jni::objects::JValueGen::Bool(arg2.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "addEnchant",
            "(Lorg/bukkit/enchantments/Enchantment;IZ)Z",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
                jni::objects::JValueGen::from(&val_3),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn remove_enchant(
        &mut self,
        arg0: impl Into<&'mc crate::enchantments::Enchantment<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeEnchant",
            "(Lorg/bukkit/enchantments/Enchantment;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn has_conflicting_enchant(
        &mut self,
        arg0: impl Into<&'mc crate::enchantments::Enchantment<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "hasConflictingEnchant",
            "(Lorg/bukkit/enchantments/Enchantment;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn add_item_flags(
        &mut self,
        arg0: Vec<impl Into<crate::inventory::ItemFlag<'mc>>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "addItemFlags",
            "(Lorg/bukkit/inventory/ItemFlag;)V",
            &[],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn remove_item_flags(
        &mut self,
        arg0: Vec<impl Into<crate::inventory::ItemFlag<'mc>>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeItemFlags",
            "(Lorg/bukkit/inventory/ItemFlag;)V",
            &[],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn item_flags(
        &mut self,
    ) -> Result<blackboxmc_java::JavaSet<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getItemFlags",
            "()Ljava/util/Set;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::JavaSet::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn has_item_flag(
        &mut self,
        arg0: impl Into<&'mc crate::inventory::ItemFlag<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "hasItemFlag",
            "(Lorg/bukkit/inventory/ItemFlag;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn is_unbreakable(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isUnbreakable", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn set_unbreakable(&mut self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        // -2
        let val_1 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setUnbreakable",
            "(Z)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn has_attribute_modifiers(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "hasAttributeModifiers", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn attribute_modifiers(
        &mut self,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getAttributeModifiers",
            "()Lcom/google/common/collect/Multimap;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }

    pub fn add_attribute_modifier(
        &mut self,
        arg0: impl Into<&'mc crate::attribute::Attribute<'mc>>,
        arg1: impl Into<&'mc crate::attribute::AttributeModifier<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let val_2 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "addAttributeModifier",
            "(Lorg/bukkit/attribute/Attribute;Lorg/bukkit/attribute/AttributeModifier;)Z",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub unsafe fn set_attribute_modifiers(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setAttributeModifiers",
            "(Lcom/google/common/collect/Multimap;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn remove_attribute_modifier_with_attribute(
        &mut self,
        arg0: impl Into<&'mc crate::attribute::Attribute<'mc>>,
        arg1: std::option::Option<impl Into<&'mc crate::attribute::AttributeModifier<'mc>>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let val_2 =
            unsafe { jni::objects::JObject::from_raw(arg1.unwrap().into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeAttributeModifier",
            "(Lorg/bukkit/attribute/Attribute;Lorg/bukkit/attribute/AttributeModifier;)Z",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    #[deprecated]

    pub fn custom_tag_container(
        &mut self,
    ) -> Result<crate::inventory::meta::tags::CustomItemTagContainer<'mc>, Box<dyn std::error::Error>>
    {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getCustomTagContainer",
            "()Lorg/bukkit/inventory/meta/tags/CustomItemTagContainer;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::meta::tags::CustomItemTagContainer::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    #[deprecated]

    pub fn set_version(&mut self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setVersion",
            "(I)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

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

    pub fn persistent_data_container(
        &mut self,
    ) -> Result<crate::persistence::PersistentDataContainer<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getPersistentDataContainer",
            "()Lorg/bukkit/persistence/PersistentDataContainer;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::persistence::PersistentDataContainer::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
}
impl<'mc> JNIRaw<'mc> for Damageable<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Into<crate::inventory::meta::ItemMeta<'mc>> for Damageable<'mc> {
    fn into(self) -> crate::inventory::meta::ItemMeta<'mc> {
        crate::inventory::meta::ItemMeta::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
/// Represents a <a href="../../Material.html#FIREWORK_ROCKET"><code>Material.FIREWORK_ROCKET</code></a> and its effects.
///
/// This is a representation of an abstract class.
pub struct FireworkMeta<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> FireworkMeta<'mc> {
    pub fn from_raw(
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
    /// Get the effects in this firework.
    /// Get the number of effects in this firework.
    pub fn effects(
        &mut self,
    ) -> Result<Vec<crate::FireworkEffect<'mc>>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getEffects", "()Ljava/util/List;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let mut list = blackboxmc_java::JavaList::from_raw(&self.jni_ref(), res.l()?)?;
        let size = list.size()?;
        for i in 0..=size {
            let obj = list.get(i)?;
            new_vec.push(crate::FireworkEffect::from_raw(&self.jni_ref(), obj)?);
        }
        Ok(new_vec)
    }
    /// Gets the approximate height the firework will fly.
    pub fn power(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getPower", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }
    /// Sets the approximate power of the firework. Each level of power is half a second of flight time.
    pub fn set_power(&mut self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setPower",
            "(I)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Add another effect to this firework.
    /// Add several effects to this firework.
    /// Add several firework effects to this firework.
    pub fn add_effect(
        &mut self,
        arg0: impl Into<&'mc crate::FireworkEffect<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "addEffect",
            "(Lorg/bukkit/FireworkEffect;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Get the number of effects in this firework.
    pub fn effects_size(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getEffectsSize", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }
    /// Remove an effect from this firework.
    pub fn remove_effect(&mut self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeEffect",
            "(I)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Remove all effects from this firework.
    pub fn clear_effects(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "clearEffects", "()V", &[]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Get whether this firework has any effects.
    pub fn has_effects(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasEffects", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn clone(
        &mut self,
    ) -> Result<crate::inventory::meta::FireworkMeta<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "clone",
            "()Lorg/bukkit/inventory/meta/FireworkMeta;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::meta::FireworkMeta::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn display_name(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getDisplayName",
            "()Ljava/lang/String;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }

    pub fn localized_name(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getLocalizedName",
            "()Ljava/lang/String;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }

    pub fn get_enchant_level(
        &mut self,
        arg0: impl Into<&'mc crate::enchantments::Enchantment<'mc>>,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getEnchantLevel",
            "(Lorg/bukkit/enchantments/Enchantment;)I",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }

    pub fn set_lore(
        &mut self,
        arg0: Vec<impl Into<String>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let raw_val_1 = self
            .jni_ref()
            .new_object("java/util/ArrayList", "()V", &[])
            .unwrap();
        for v in arg0 {
            let map_val_0 =
                jni::objects::JObject::from(self.jni_ref().new_string(v.into()).unwrap());
            self.jni_ref().call_method(
                &raw_val_1,
                "add",
                "(Ljava/Lang/Object)V",
                &[jni::objects::JValueGen::from(&map_val_0)],
            )?;
        }
        let val_1 = jni::objects::JValueGen::Object(raw_val_1);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setLore",
            "(Ljava/util/List;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn as_string(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getAsString",
            "()Ljava/lang/String;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }

    pub fn has_display_name(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasDisplayName", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn set_display_name(
        &mut self,
        arg0: impl Into<&'mc String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setDisplayName",
            "(Ljava/lang/String;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn has_localized_name(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasLocalizedName", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn set_localized_name(
        &mut self,
        arg0: impl Into<&'mc String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setLocalizedName",
            "(Ljava/lang/String;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn has_lore(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasLore", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn lore(&mut self) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getLore", "()Ljava/util/List;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let mut list = blackboxmc_java::JavaList::from_raw(&self.jni_ref(), res.l()?)?;
        let size = list.size()?;
        for i in 0..=size {
            let obj = list.get(i)?;
            new_vec.push(
                self.jni_ref()
                    .get_string(unsafe { &jni::objects::JString::from_raw(*obj) })?
                    .to_string_lossy()
                    .to_string(),
            );
        }
        Ok(new_vec)
    }

    pub fn has_custom_model_data(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasCustomModelData", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn custom_model_data(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getCustomModelData", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }

    pub fn set_custom_model_data(&mut self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setCustomModelData",
            "(I)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn has_enchants(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasEnchants", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn has_enchant(
        &mut self,
        arg0: impl Into<&'mc crate::enchantments::Enchantment<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "hasEnchant",
            "(Lorg/bukkit/enchantments/Enchantment;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn enchants(
        &mut self,
    ) -> Result<blackboxmc_java::JavaMap<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getEnchants", "()Ljava/util/Map;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::JavaMap::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn add_enchant(
        &mut self,
        arg0: impl Into<&'mc crate::enchantments::Enchantment<'mc>>,
        arg1: i32,
        arg2: bool,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let val_2 = jni::objects::JValueGen::Int(arg1.into());
        // -2
        let val_3 = jni::objects::JValueGen::Bool(arg2.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "addEnchant",
            "(Lorg/bukkit/enchantments/Enchantment;IZ)Z",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
                jni::objects::JValueGen::from(&val_3),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn remove_enchant(
        &mut self,
        arg0: impl Into<&'mc crate::enchantments::Enchantment<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeEnchant",
            "(Lorg/bukkit/enchantments/Enchantment;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn has_conflicting_enchant(
        &mut self,
        arg0: impl Into<&'mc crate::enchantments::Enchantment<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "hasConflictingEnchant",
            "(Lorg/bukkit/enchantments/Enchantment;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn add_item_flags(
        &mut self,
        arg0: Vec<impl Into<crate::inventory::ItemFlag<'mc>>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "addItemFlags",
            "(Lorg/bukkit/inventory/ItemFlag;)V",
            &[],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn remove_item_flags(
        &mut self,
        arg0: Vec<impl Into<crate::inventory::ItemFlag<'mc>>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeItemFlags",
            "(Lorg/bukkit/inventory/ItemFlag;)V",
            &[],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn item_flags(
        &mut self,
    ) -> Result<blackboxmc_java::JavaSet<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getItemFlags",
            "()Ljava/util/Set;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::JavaSet::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn has_item_flag(
        &mut self,
        arg0: impl Into<&'mc crate::inventory::ItemFlag<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "hasItemFlag",
            "(Lorg/bukkit/inventory/ItemFlag;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn is_unbreakable(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isUnbreakable", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn set_unbreakable(&mut self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        // -2
        let val_1 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setUnbreakable",
            "(Z)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn has_attribute_modifiers(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "hasAttributeModifiers", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn attribute_modifiers(
        &mut self,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getAttributeModifiers",
            "()Lcom/google/common/collect/Multimap;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }

    pub fn add_attribute_modifier(
        &mut self,
        arg0: impl Into<&'mc crate::attribute::Attribute<'mc>>,
        arg1: impl Into<&'mc crate::attribute::AttributeModifier<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let val_2 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "addAttributeModifier",
            "(Lorg/bukkit/attribute/Attribute;Lorg/bukkit/attribute/AttributeModifier;)Z",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub unsafe fn set_attribute_modifiers(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setAttributeModifiers",
            "(Lcom/google/common/collect/Multimap;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn remove_attribute_modifier_with_attribute(
        &mut self,
        arg0: impl Into<&'mc crate::attribute::Attribute<'mc>>,
        arg1: std::option::Option<impl Into<&'mc crate::attribute::AttributeModifier<'mc>>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let val_2 =
            unsafe { jni::objects::JObject::from_raw(arg1.unwrap().into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeAttributeModifier",
            "(Lorg/bukkit/attribute/Attribute;Lorg/bukkit/attribute/AttributeModifier;)Z",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    #[deprecated]

    pub fn custom_tag_container(
        &mut self,
    ) -> Result<crate::inventory::meta::tags::CustomItemTagContainer<'mc>, Box<dyn std::error::Error>>
    {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getCustomTagContainer",
            "()Lorg/bukkit/inventory/meta/tags/CustomItemTagContainer;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::meta::tags::CustomItemTagContainer::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    #[deprecated]

    pub fn set_version(&mut self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setVersion",
            "(I)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

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

    pub fn persistent_data_container(
        &mut self,
    ) -> Result<crate::persistence::PersistentDataContainer<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getPersistentDataContainer",
            "()Lorg/bukkit/persistence/PersistentDataContainer;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::persistence::PersistentDataContainer::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
}
impl<'mc> JNIRaw<'mc> for FireworkMeta<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Into<crate::inventory::meta::ItemMeta<'mc>> for FireworkMeta<'mc> {
    fn into(self) -> crate::inventory::meta::ItemMeta<'mc> {
        crate::inventory::meta::ItemMeta::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
/// Represents a potion or item that can have custom effects.
///
/// This is a representation of an abstract class.
pub struct PotionMeta<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> PotionMeta<'mc> {
    pub fn from_raw(
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
    /// Sets the potion color. A custom potion color will alter the display of the potion in an inventory slot.
    pub fn set_color(
        &mut self,
        arg0: impl Into<&'mc crate::Color<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setColor",
            "(Lorg/bukkit/Color;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Gets the potion color that is set. A custom potion color will alter the display of the potion in an inventory slot.
    /// <p>Plugins should check that hasColor() returns <code>true</code> before calling this method.</p>
    pub fn color(&mut self) -> Result<crate::Color<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getColor", "()Lorg/bukkit/Color;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        crate::Color::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Gets an immutable list containing all custom potion effects applied to this potion.
    /// <p>Plugins should check that hasCustomEffects() returns true before calling this method.</p>
    pub fn custom_effects(
        &mut self,
    ) -> Result<Vec<crate::potion::PotionEffect<'mc>>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getCustomEffects",
            "()Ljava/util/List;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let mut list = blackboxmc_java::JavaList::from_raw(&self.jni_ref(), res.l()?)?;
        let size = list.size()?;
        for i in 0..=size {
            let obj = list.get(i)?;
            new_vec.push(crate::potion::PotionEffect::from_raw(&self.jni_ref(), obj)?);
        }
        Ok(new_vec)
    }
    /// Checks for the presence of custom potion effects.
    pub fn has_custom_effects(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasCustomEffects", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    /// Adds a custom potion effect to this potion.
    pub fn add_custom_effect(
        &mut self,
        arg0: impl Into<&'mc crate::potion::PotionEffect<'mc>>,
        arg1: bool,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        // -2
        let val_2 = jni::objects::JValueGen::Bool(arg1.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "addCustomEffect",
            "(Lorg/bukkit/potion/PotionEffect;Z)Z",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    /// Removes a custom potion effect from this potion.
    pub fn remove_custom_effect(
        &mut self,
        arg0: impl Into<&'mc crate::potion::PotionEffectType<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeCustomEffect",
            "(Lorg/bukkit/potion/PotionEffectType;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    /// Checks for the presence of custom potion effects.
    /// Checks for a specific custom potion effect type on this potion.
    pub fn has_custom_effect(
        &mut self,
        arg0: impl Into<&'mc crate::potion::PotionEffectType<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "hasCustomEffect",
            "(Lorg/bukkit/potion/PotionEffectType;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    /// Removes all custom potion effects from this potion.
    pub fn clear_custom_effects(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "clearCustomEffects", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    /// Sets the underlying potion data
    pub fn set_base_potion_data(
        &mut self,
        arg0: impl Into<&'mc crate::potion::PotionData<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setBasePotionData",
            "(Lorg/bukkit/potion/PotionData;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Returns the potion data about the base potion
    pub fn base_potion_data(
        &mut self,
    ) -> Result<crate::potion::PotionData<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getBasePotionData",
            "()Lorg/bukkit/potion/PotionData;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::potion::PotionData::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    #[deprecated]
    /// <span class="deprecated-label">Deprecated.</span>
    /// <div class="deprecation-comment">
    /// use <a href="#setBasePotionData(org.bukkit.potion.PotionData)"><code>setBasePotionData(org.bukkit.potion.PotionData)</code></a>
    /// </div>
    /// use <a href="#setBasePotionData(org.bukkit.potion.PotionData)"><code>setBasePotionData(org.bukkit.potion.PotionData)</code></a>
    ///
    /// Moves a potion effect to the top of the potion effect list.
    /// <p>This causes the client to display the potion effect in the potion's name.</p>
    pub fn set_main_effect(
        &mut self,
        arg0: impl Into<&'mc crate::potion::PotionEffectType<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setMainEffect",
            "(Lorg/bukkit/potion/PotionEffectType;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    /// Checks for existence of a potion color.
    pub fn has_color(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasColor", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn clone(
        &mut self,
    ) -> Result<crate::inventory::meta::PotionMeta<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "clone",
            "()Lorg/bukkit/inventory/meta/PotionMeta;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::meta::PotionMeta::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn display_name(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getDisplayName",
            "()Ljava/lang/String;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }

    pub fn localized_name(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getLocalizedName",
            "()Ljava/lang/String;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }

    pub fn get_enchant_level(
        &mut self,
        arg0: impl Into<&'mc crate::enchantments::Enchantment<'mc>>,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getEnchantLevel",
            "(Lorg/bukkit/enchantments/Enchantment;)I",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }

    pub fn set_lore(
        &mut self,
        arg0: Vec<impl Into<String>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let raw_val_1 = self
            .jni_ref()
            .new_object("java/util/ArrayList", "()V", &[])
            .unwrap();
        for v in arg0 {
            let map_val_0 =
                jni::objects::JObject::from(self.jni_ref().new_string(v.into()).unwrap());
            self.jni_ref().call_method(
                &raw_val_1,
                "add",
                "(Ljava/Lang/Object)V",
                &[jni::objects::JValueGen::from(&map_val_0)],
            )?;
        }
        let val_1 = jni::objects::JValueGen::Object(raw_val_1);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setLore",
            "(Ljava/util/List;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn as_string(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getAsString",
            "()Ljava/lang/String;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }

    pub fn has_display_name(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasDisplayName", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn set_display_name(
        &mut self,
        arg0: impl Into<&'mc String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setDisplayName",
            "(Ljava/lang/String;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn has_localized_name(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasLocalizedName", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn set_localized_name(
        &mut self,
        arg0: impl Into<&'mc String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setLocalizedName",
            "(Ljava/lang/String;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn has_lore(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasLore", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn lore(&mut self) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getLore", "()Ljava/util/List;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let mut list = blackboxmc_java::JavaList::from_raw(&self.jni_ref(), res.l()?)?;
        let size = list.size()?;
        for i in 0..=size {
            let obj = list.get(i)?;
            new_vec.push(
                self.jni_ref()
                    .get_string(unsafe { &jni::objects::JString::from_raw(*obj) })?
                    .to_string_lossy()
                    .to_string(),
            );
        }
        Ok(new_vec)
    }

    pub fn has_custom_model_data(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasCustomModelData", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn custom_model_data(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getCustomModelData", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }

    pub fn set_custom_model_data(&mut self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setCustomModelData",
            "(I)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn has_enchants(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasEnchants", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn has_enchant(
        &mut self,
        arg0: impl Into<&'mc crate::enchantments::Enchantment<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "hasEnchant",
            "(Lorg/bukkit/enchantments/Enchantment;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn enchants(
        &mut self,
    ) -> Result<blackboxmc_java::JavaMap<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getEnchants", "()Ljava/util/Map;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::JavaMap::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn add_enchant(
        &mut self,
        arg0: impl Into<&'mc crate::enchantments::Enchantment<'mc>>,
        arg1: i32,
        arg2: bool,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let val_2 = jni::objects::JValueGen::Int(arg1.into());
        // -2
        let val_3 = jni::objects::JValueGen::Bool(arg2.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "addEnchant",
            "(Lorg/bukkit/enchantments/Enchantment;IZ)Z",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
                jni::objects::JValueGen::from(&val_3),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn remove_enchant(
        &mut self,
        arg0: impl Into<&'mc crate::enchantments::Enchantment<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeEnchant",
            "(Lorg/bukkit/enchantments/Enchantment;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn has_conflicting_enchant(
        &mut self,
        arg0: impl Into<&'mc crate::enchantments::Enchantment<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "hasConflictingEnchant",
            "(Lorg/bukkit/enchantments/Enchantment;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn add_item_flags(
        &mut self,
        arg0: Vec<impl Into<crate::inventory::ItemFlag<'mc>>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "addItemFlags",
            "(Lorg/bukkit/inventory/ItemFlag;)V",
            &[],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn remove_item_flags(
        &mut self,
        arg0: Vec<impl Into<crate::inventory::ItemFlag<'mc>>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeItemFlags",
            "(Lorg/bukkit/inventory/ItemFlag;)V",
            &[],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn item_flags(
        &mut self,
    ) -> Result<blackboxmc_java::JavaSet<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getItemFlags",
            "()Ljava/util/Set;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::JavaSet::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn has_item_flag(
        &mut self,
        arg0: impl Into<&'mc crate::inventory::ItemFlag<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "hasItemFlag",
            "(Lorg/bukkit/inventory/ItemFlag;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn is_unbreakable(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isUnbreakable", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn set_unbreakable(&mut self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        // -2
        let val_1 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setUnbreakable",
            "(Z)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn has_attribute_modifiers(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "hasAttributeModifiers", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn attribute_modifiers(
        &mut self,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getAttributeModifiers",
            "()Lcom/google/common/collect/Multimap;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }

    pub fn add_attribute_modifier(
        &mut self,
        arg0: impl Into<&'mc crate::attribute::Attribute<'mc>>,
        arg1: impl Into<&'mc crate::attribute::AttributeModifier<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let val_2 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "addAttributeModifier",
            "(Lorg/bukkit/attribute/Attribute;Lorg/bukkit/attribute/AttributeModifier;)Z",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub unsafe fn set_attribute_modifiers(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setAttributeModifiers",
            "(Lcom/google/common/collect/Multimap;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn remove_attribute_modifier_with_attribute(
        &mut self,
        arg0: impl Into<&'mc crate::attribute::Attribute<'mc>>,
        arg1: std::option::Option<impl Into<&'mc crate::attribute::AttributeModifier<'mc>>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let val_2 =
            unsafe { jni::objects::JObject::from_raw(arg1.unwrap().into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeAttributeModifier",
            "(Lorg/bukkit/attribute/Attribute;Lorg/bukkit/attribute/AttributeModifier;)Z",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    #[deprecated]

    pub fn custom_tag_container(
        &mut self,
    ) -> Result<crate::inventory::meta::tags::CustomItemTagContainer<'mc>, Box<dyn std::error::Error>>
    {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getCustomTagContainer",
            "()Lorg/bukkit/inventory/meta/tags/CustomItemTagContainer;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::meta::tags::CustomItemTagContainer::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    #[deprecated]

    pub fn set_version(&mut self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setVersion",
            "(I)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

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

    pub fn persistent_data_container(
        &mut self,
    ) -> Result<crate::persistence::PersistentDataContainer<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getPersistentDataContainer",
            "()Lorg/bukkit/persistence/PersistentDataContainer;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::persistence::PersistentDataContainer::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
}
impl<'mc> JNIRaw<'mc> for PotionMeta<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Into<crate::inventory::meta::ItemMeta<'mc>> for PotionMeta<'mc> {
    fn into(self) -> crate::inventory::meta::ItemMeta<'mc> {
        crate::inventory::meta::ItemMeta::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}

///
/// This is a representation of an abstract class.
pub struct BlockStateMeta<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> BlockStateMeta<'mc> {
    pub fn from_raw(
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
    /// Returns the currently attached block state for this item or creates a new one if one doesn't exist. The state is a copy, it must be set back (or to another item) with <a href="#setBlockState(org.bukkit.block.BlockState)"><code>setBlockState(org.bukkit.block.BlockState)</code></a>
    pub fn block_state(
        &mut self,
    ) -> Result<crate::block::BlockState<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getBlockState",
            "()Lorg/bukkit/block/BlockState;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::block::BlockState::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Returns whether the item has a block state currently attached to it.
    pub fn has_block_state(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasBlockState", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    /// Attaches a copy of the passed block state to the item.
    pub fn set_block_state(
        &mut self,
        arg0: impl Into<&'mc crate::block::BlockState<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setBlockState",
            "(Lorg/bukkit/block/BlockState;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn display_name(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getDisplayName",
            "()Ljava/lang/String;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }

    pub fn localized_name(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getLocalizedName",
            "()Ljava/lang/String;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }

    pub fn get_enchant_level(
        &mut self,
        arg0: impl Into<&'mc crate::enchantments::Enchantment<'mc>>,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getEnchantLevel",
            "(Lorg/bukkit/enchantments/Enchantment;)I",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }

    pub fn set_lore(
        &mut self,
        arg0: Vec<impl Into<String>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let raw_val_1 = self
            .jni_ref()
            .new_object("java/util/ArrayList", "()V", &[])
            .unwrap();
        for v in arg0 {
            let map_val_0 =
                jni::objects::JObject::from(self.jni_ref().new_string(v.into()).unwrap());
            self.jni_ref().call_method(
                &raw_val_1,
                "add",
                "(Ljava/Lang/Object)V",
                &[jni::objects::JValueGen::from(&map_val_0)],
            )?;
        }
        let val_1 = jni::objects::JValueGen::Object(raw_val_1);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setLore",
            "(Ljava/util/List;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn as_string(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getAsString",
            "()Ljava/lang/String;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }

    pub fn has_display_name(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasDisplayName", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn set_display_name(
        &mut self,
        arg0: impl Into<&'mc String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setDisplayName",
            "(Ljava/lang/String;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn has_localized_name(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasLocalizedName", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn set_localized_name(
        &mut self,
        arg0: impl Into<&'mc String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setLocalizedName",
            "(Ljava/lang/String;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn has_lore(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasLore", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn lore(&mut self) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getLore", "()Ljava/util/List;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let mut list = blackboxmc_java::JavaList::from_raw(&self.jni_ref(), res.l()?)?;
        let size = list.size()?;
        for i in 0..=size {
            let obj = list.get(i)?;
            new_vec.push(
                self.jni_ref()
                    .get_string(unsafe { &jni::objects::JString::from_raw(*obj) })?
                    .to_string_lossy()
                    .to_string(),
            );
        }
        Ok(new_vec)
    }

    pub fn has_custom_model_data(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasCustomModelData", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn custom_model_data(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getCustomModelData", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }

    pub fn set_custom_model_data(&mut self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setCustomModelData",
            "(I)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn has_enchants(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasEnchants", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn has_enchant(
        &mut self,
        arg0: impl Into<&'mc crate::enchantments::Enchantment<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "hasEnchant",
            "(Lorg/bukkit/enchantments/Enchantment;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn enchants(
        &mut self,
    ) -> Result<blackboxmc_java::JavaMap<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getEnchants", "()Ljava/util/Map;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::JavaMap::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn add_enchant(
        &mut self,
        arg0: impl Into<&'mc crate::enchantments::Enchantment<'mc>>,
        arg1: i32,
        arg2: bool,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let val_2 = jni::objects::JValueGen::Int(arg1.into());
        // -2
        let val_3 = jni::objects::JValueGen::Bool(arg2.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "addEnchant",
            "(Lorg/bukkit/enchantments/Enchantment;IZ)Z",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
                jni::objects::JValueGen::from(&val_3),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn remove_enchant(
        &mut self,
        arg0: impl Into<&'mc crate::enchantments::Enchantment<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeEnchant",
            "(Lorg/bukkit/enchantments/Enchantment;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn has_conflicting_enchant(
        &mut self,
        arg0: impl Into<&'mc crate::enchantments::Enchantment<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "hasConflictingEnchant",
            "(Lorg/bukkit/enchantments/Enchantment;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn add_item_flags(
        &mut self,
        arg0: Vec<impl Into<crate::inventory::ItemFlag<'mc>>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "addItemFlags",
            "(Lorg/bukkit/inventory/ItemFlag;)V",
            &[],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn remove_item_flags(
        &mut self,
        arg0: Vec<impl Into<crate::inventory::ItemFlag<'mc>>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeItemFlags",
            "(Lorg/bukkit/inventory/ItemFlag;)V",
            &[],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn item_flags(
        &mut self,
    ) -> Result<blackboxmc_java::JavaSet<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getItemFlags",
            "()Ljava/util/Set;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::JavaSet::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn has_item_flag(
        &mut self,
        arg0: impl Into<&'mc crate::inventory::ItemFlag<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "hasItemFlag",
            "(Lorg/bukkit/inventory/ItemFlag;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn is_unbreakable(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isUnbreakable", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn set_unbreakable(&mut self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        // -2
        let val_1 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setUnbreakable",
            "(Z)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn has_attribute_modifiers(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "hasAttributeModifiers", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn attribute_modifiers(
        &mut self,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getAttributeModifiers",
            "()Lcom/google/common/collect/Multimap;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }

    pub fn add_attribute_modifier(
        &mut self,
        arg0: impl Into<&'mc crate::attribute::Attribute<'mc>>,
        arg1: impl Into<&'mc crate::attribute::AttributeModifier<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let val_2 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "addAttributeModifier",
            "(Lorg/bukkit/attribute/Attribute;Lorg/bukkit/attribute/AttributeModifier;)Z",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub unsafe fn set_attribute_modifiers(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setAttributeModifiers",
            "(Lcom/google/common/collect/Multimap;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn remove_attribute_modifier_with_attribute(
        &mut self,
        arg0: impl Into<&'mc crate::attribute::Attribute<'mc>>,
        arg1: std::option::Option<impl Into<&'mc crate::attribute::AttributeModifier<'mc>>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let val_2 =
            unsafe { jni::objects::JObject::from_raw(arg1.unwrap().into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeAttributeModifier",
            "(Lorg/bukkit/attribute/Attribute;Lorg/bukkit/attribute/AttributeModifier;)Z",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    #[deprecated]

    pub fn custom_tag_container(
        &mut self,
    ) -> Result<crate::inventory::meta::tags::CustomItemTagContainer<'mc>, Box<dyn std::error::Error>>
    {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getCustomTagContainer",
            "()Lorg/bukkit/inventory/meta/tags/CustomItemTagContainer;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::meta::tags::CustomItemTagContainer::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    #[deprecated]

    pub fn set_version(&mut self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setVersion",
            "(I)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn clone(&mut self) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "clone", "()Ljava/lang/Object;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }

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

    pub fn persistent_data_container(
        &mut self,
    ) -> Result<crate::persistence::PersistentDataContainer<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getPersistentDataContainer",
            "()Lorg/bukkit/persistence/PersistentDataContainer;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::persistence::PersistentDataContainer::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
}
impl<'mc> JNIRaw<'mc> for BlockStateMeta<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Into<crate::inventory::meta::ItemMeta<'mc>> for BlockStateMeta<'mc> {
    fn into(self) -> crate::inventory::meta::ItemMeta<'mc> {
        crate::inventory::meta::ItemMeta::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
/// EnchantmentMeta is specific to items that can <i>store</i> enchantments, as opposed to being enchanted. <a href="../../Material.html#ENCHANTED_BOOK"><code>Material.ENCHANTED_BOOK</code></a> is an example of an item with enchantment storage.
///
/// This is a representation of an abstract class.
pub struct EnchantmentStorageMeta<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> EnchantmentStorageMeta<'mc> {
    pub fn from_raw(
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
    /// Checks for the existence of any stored enchantments.
    /// Checks for storage of the specified enchantment.
    pub fn has_stored_enchant(
        &mut self,
        arg0: impl Into<&'mc crate::enchantments::Enchantment<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "hasStoredEnchant",
            "(Lorg/bukkit/enchantments/Enchantment;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    /// Checks for the level of the stored enchantment.
    pub fn get_stored_enchant_level(
        &mut self,
        arg0: impl Into<&'mc crate::enchantments::Enchantment<'mc>>,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getStoredEnchantLevel",
            "(Lorg/bukkit/enchantments/Enchantment;)I",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }
    /// Gets a copy the stored enchantments in this ItemMeta.
    pub fn stored_enchants(
        &mut self,
    ) -> Result<blackboxmc_java::JavaMap<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getStoredEnchants",
            "()Ljava/util/Map;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::JavaMap::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Stores the specified enchantment in this item meta.
    pub fn add_stored_enchant(
        &mut self,
        arg0: impl Into<&'mc crate::enchantments::Enchantment<'mc>>,
        arg1: i32,
        arg2: bool,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let val_2 = jni::objects::JValueGen::Int(arg1.into());
        // -2
        let val_3 = jni::objects::JValueGen::Bool(arg2.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "addStoredEnchant",
            "(Lorg/bukkit/enchantments/Enchantment;IZ)Z",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
                jni::objects::JValueGen::from(&val_3),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    /// Remove the specified stored enchantment from this item meta.
    pub fn remove_stored_enchant(
        &mut self,
        arg0: impl Into<&'mc crate::enchantments::Enchantment<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeStoredEnchant",
            "(Lorg/bukkit/enchantments/Enchantment;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    /// Checks if the specified enchantment conflicts with any enchantments in this ItemMeta.
    pub fn has_conflicting_stored_enchant(
        &mut self,
        arg0: impl Into<&'mc crate::enchantments::Enchantment<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "hasConflictingStoredEnchant",
            "(Lorg/bukkit/enchantments/Enchantment;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    /// Checks for the existence of any stored enchantments.
    pub fn has_stored_enchants(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasStoredEnchants", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn clone(
        &mut self,
    ) -> Result<crate::inventory::meta::EnchantmentStorageMeta<'mc>, Box<dyn std::error::Error>>
    {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "clone",
            "()Lorg/bukkit/inventory/meta/EnchantmentStorageMeta;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::meta::EnchantmentStorageMeta::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn display_name(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getDisplayName",
            "()Ljava/lang/String;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }

    pub fn localized_name(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getLocalizedName",
            "()Ljava/lang/String;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }

    pub fn get_enchant_level(
        &mut self,
        arg0: impl Into<&'mc crate::enchantments::Enchantment<'mc>>,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getEnchantLevel",
            "(Lorg/bukkit/enchantments/Enchantment;)I",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }

    pub fn set_lore(
        &mut self,
        arg0: Vec<impl Into<String>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let raw_val_1 = self
            .jni_ref()
            .new_object("java/util/ArrayList", "()V", &[])
            .unwrap();
        for v in arg0 {
            let map_val_0 =
                jni::objects::JObject::from(self.jni_ref().new_string(v.into()).unwrap());
            self.jni_ref().call_method(
                &raw_val_1,
                "add",
                "(Ljava/Lang/Object)V",
                &[jni::objects::JValueGen::from(&map_val_0)],
            )?;
        }
        let val_1 = jni::objects::JValueGen::Object(raw_val_1);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setLore",
            "(Ljava/util/List;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn as_string(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getAsString",
            "()Ljava/lang/String;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }

    pub fn has_display_name(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasDisplayName", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn set_display_name(
        &mut self,
        arg0: impl Into<&'mc String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setDisplayName",
            "(Ljava/lang/String;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn has_localized_name(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasLocalizedName", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn set_localized_name(
        &mut self,
        arg0: impl Into<&'mc String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setLocalizedName",
            "(Ljava/lang/String;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn has_lore(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasLore", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn lore(&mut self) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getLore", "()Ljava/util/List;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let mut list = blackboxmc_java::JavaList::from_raw(&self.jni_ref(), res.l()?)?;
        let size = list.size()?;
        for i in 0..=size {
            let obj = list.get(i)?;
            new_vec.push(
                self.jni_ref()
                    .get_string(unsafe { &jni::objects::JString::from_raw(*obj) })?
                    .to_string_lossy()
                    .to_string(),
            );
        }
        Ok(new_vec)
    }

    pub fn has_custom_model_data(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasCustomModelData", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn custom_model_data(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getCustomModelData", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }

    pub fn set_custom_model_data(&mut self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setCustomModelData",
            "(I)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn has_enchants(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasEnchants", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn has_enchant(
        &mut self,
        arg0: impl Into<&'mc crate::enchantments::Enchantment<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "hasEnchant",
            "(Lorg/bukkit/enchantments/Enchantment;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn enchants(
        &mut self,
    ) -> Result<blackboxmc_java::JavaMap<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getEnchants", "()Ljava/util/Map;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::JavaMap::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn add_enchant(
        &mut self,
        arg0: impl Into<&'mc crate::enchantments::Enchantment<'mc>>,
        arg1: i32,
        arg2: bool,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let val_2 = jni::objects::JValueGen::Int(arg1.into());
        // -2
        let val_3 = jni::objects::JValueGen::Bool(arg2.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "addEnchant",
            "(Lorg/bukkit/enchantments/Enchantment;IZ)Z",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
                jni::objects::JValueGen::from(&val_3),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn remove_enchant(
        &mut self,
        arg0: impl Into<&'mc crate::enchantments::Enchantment<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeEnchant",
            "(Lorg/bukkit/enchantments/Enchantment;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn has_conflicting_enchant(
        &mut self,
        arg0: impl Into<&'mc crate::enchantments::Enchantment<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "hasConflictingEnchant",
            "(Lorg/bukkit/enchantments/Enchantment;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn add_item_flags(
        &mut self,
        arg0: Vec<impl Into<crate::inventory::ItemFlag<'mc>>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "addItemFlags",
            "(Lorg/bukkit/inventory/ItemFlag;)V",
            &[],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn remove_item_flags(
        &mut self,
        arg0: Vec<impl Into<crate::inventory::ItemFlag<'mc>>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeItemFlags",
            "(Lorg/bukkit/inventory/ItemFlag;)V",
            &[],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn item_flags(
        &mut self,
    ) -> Result<blackboxmc_java::JavaSet<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getItemFlags",
            "()Ljava/util/Set;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::JavaSet::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn has_item_flag(
        &mut self,
        arg0: impl Into<&'mc crate::inventory::ItemFlag<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "hasItemFlag",
            "(Lorg/bukkit/inventory/ItemFlag;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn is_unbreakable(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isUnbreakable", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn set_unbreakable(&mut self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        // -2
        let val_1 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setUnbreakable",
            "(Z)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn has_attribute_modifiers(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "hasAttributeModifiers", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn attribute_modifiers(
        &mut self,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getAttributeModifiers",
            "()Lcom/google/common/collect/Multimap;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }

    pub fn add_attribute_modifier(
        &mut self,
        arg0: impl Into<&'mc crate::attribute::Attribute<'mc>>,
        arg1: impl Into<&'mc crate::attribute::AttributeModifier<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let val_2 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "addAttributeModifier",
            "(Lorg/bukkit/attribute/Attribute;Lorg/bukkit/attribute/AttributeModifier;)Z",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub unsafe fn set_attribute_modifiers(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setAttributeModifiers",
            "(Lcom/google/common/collect/Multimap;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn remove_attribute_modifier_with_attribute(
        &mut self,
        arg0: impl Into<&'mc crate::attribute::Attribute<'mc>>,
        arg1: std::option::Option<impl Into<&'mc crate::attribute::AttributeModifier<'mc>>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let val_2 =
            unsafe { jni::objects::JObject::from_raw(arg1.unwrap().into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeAttributeModifier",
            "(Lorg/bukkit/attribute/Attribute;Lorg/bukkit/attribute/AttributeModifier;)Z",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    #[deprecated]

    pub fn custom_tag_container(
        &mut self,
    ) -> Result<crate::inventory::meta::tags::CustomItemTagContainer<'mc>, Box<dyn std::error::Error>>
    {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getCustomTagContainer",
            "()Lorg/bukkit/inventory/meta/tags/CustomItemTagContainer;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::meta::tags::CustomItemTagContainer::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    #[deprecated]

    pub fn set_version(&mut self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setVersion",
            "(I)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

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

    pub fn persistent_data_container(
        &mut self,
    ) -> Result<crate::persistence::PersistentDataContainer<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getPersistentDataContainer",
            "()Lorg/bukkit/persistence/PersistentDataContainer;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::persistence::PersistentDataContainer::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
}
impl<'mc> JNIRaw<'mc> for EnchantmentStorageMeta<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Into<crate::inventory::meta::ItemMeta<'mc>> for EnchantmentStorageMeta<'mc> {
    fn into(self) -> crate::inventory::meta::ItemMeta<'mc> {
        crate::inventory::meta::ItemMeta::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}

///
/// This is a representation of an abstract class.
pub struct KnowledgeBookMeta<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> KnowledgeBookMeta<'mc> {
    pub fn from_raw(
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
    /// Gets all the recipes in the book.
    pub fn recipes(
        &mut self,
    ) -> Result<Vec<crate::NamespacedKey<'mc>>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getRecipes", "()Ljava/util/List;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let mut list = blackboxmc_java::JavaList::from_raw(&self.jni_ref(), res.l()?)?;
        let size = list.size()?;
        for i in 0..=size {
            let obj = list.get(i)?;
            new_vec.push(crate::NamespacedKey::from_raw(&self.jni_ref(), obj)?);
        }
        Ok(new_vec)
    }
    /// Clears the existing book recipes, and sets the book to use the provided recipes.
    pub fn set_recipes(
        &mut self,
        arg0: Vec<impl Into<crate::NamespacedKey<'mc>>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let raw_val_1 = self
            .jni_ref()
            .new_object("java/util/ArrayList", "()V", &[])
            .unwrap();
        for v in arg0 {
            let map_val_0 =
                unsafe { jni::objects::JObject::from_raw(v.into().jni_object().clone()) };
            self.jni_ref().call_method(
                &raw_val_1,
                "add",
                "(Ljava/Lang/Object)V",
                &[jni::objects::JValueGen::from(&map_val_0)],
            )?;
        }
        let val_1 = jni::objects::JValueGen::Object(raw_val_1);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setRecipes",
            "(Ljava/util/List;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Adds new recipe to the end of the book.
    pub fn add_recipe(
        &mut self,
        arg0: Vec<impl Into<crate::NamespacedKey<'mc>>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "addRecipe",
            "(Lorg/bukkit/NamespacedKey;)V",
            &[],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Checks for the existence of recipes in the book.
    pub fn has_recipes(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasRecipes", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn clone(&mut self) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "clone", "()Ljava/lang/Object;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }

    pub fn display_name(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getDisplayName",
            "()Ljava/lang/String;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }

    pub fn localized_name(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getLocalizedName",
            "()Ljava/lang/String;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }

    pub fn get_enchant_level(
        &mut self,
        arg0: impl Into<&'mc crate::enchantments::Enchantment<'mc>>,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getEnchantLevel",
            "(Lorg/bukkit/enchantments/Enchantment;)I",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }

    pub fn set_lore(
        &mut self,
        arg0: Vec<impl Into<String>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let raw_val_1 = self
            .jni_ref()
            .new_object("java/util/ArrayList", "()V", &[])
            .unwrap();
        for v in arg0 {
            let map_val_0 =
                jni::objects::JObject::from(self.jni_ref().new_string(v.into()).unwrap());
            self.jni_ref().call_method(
                &raw_val_1,
                "add",
                "(Ljava/Lang/Object)V",
                &[jni::objects::JValueGen::from(&map_val_0)],
            )?;
        }
        let val_1 = jni::objects::JValueGen::Object(raw_val_1);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setLore",
            "(Ljava/util/List;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn as_string(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getAsString",
            "()Ljava/lang/String;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }

    pub fn has_display_name(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasDisplayName", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn set_display_name(
        &mut self,
        arg0: impl Into<&'mc String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setDisplayName",
            "(Ljava/lang/String;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn has_localized_name(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasLocalizedName", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn set_localized_name(
        &mut self,
        arg0: impl Into<&'mc String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setLocalizedName",
            "(Ljava/lang/String;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn has_lore(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasLore", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn lore(&mut self) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getLore", "()Ljava/util/List;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let mut list = blackboxmc_java::JavaList::from_raw(&self.jni_ref(), res.l()?)?;
        let size = list.size()?;
        for i in 0..=size {
            let obj = list.get(i)?;
            new_vec.push(
                self.jni_ref()
                    .get_string(unsafe { &jni::objects::JString::from_raw(*obj) })?
                    .to_string_lossy()
                    .to_string(),
            );
        }
        Ok(new_vec)
    }

    pub fn has_custom_model_data(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasCustomModelData", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn custom_model_data(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getCustomModelData", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }

    pub fn set_custom_model_data(&mut self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setCustomModelData",
            "(I)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn has_enchants(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasEnchants", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn has_enchant(
        &mut self,
        arg0: impl Into<&'mc crate::enchantments::Enchantment<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "hasEnchant",
            "(Lorg/bukkit/enchantments/Enchantment;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn enchants(
        &mut self,
    ) -> Result<blackboxmc_java::JavaMap<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getEnchants", "()Ljava/util/Map;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::JavaMap::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn add_enchant(
        &mut self,
        arg0: impl Into<&'mc crate::enchantments::Enchantment<'mc>>,
        arg1: i32,
        arg2: bool,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let val_2 = jni::objects::JValueGen::Int(arg1.into());
        // -2
        let val_3 = jni::objects::JValueGen::Bool(arg2.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "addEnchant",
            "(Lorg/bukkit/enchantments/Enchantment;IZ)Z",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
                jni::objects::JValueGen::from(&val_3),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn remove_enchant(
        &mut self,
        arg0: impl Into<&'mc crate::enchantments::Enchantment<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeEnchant",
            "(Lorg/bukkit/enchantments/Enchantment;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn has_conflicting_enchant(
        &mut self,
        arg0: impl Into<&'mc crate::enchantments::Enchantment<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "hasConflictingEnchant",
            "(Lorg/bukkit/enchantments/Enchantment;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn add_item_flags(
        &mut self,
        arg0: Vec<impl Into<crate::inventory::ItemFlag<'mc>>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "addItemFlags",
            "(Lorg/bukkit/inventory/ItemFlag;)V",
            &[],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn remove_item_flags(
        &mut self,
        arg0: Vec<impl Into<crate::inventory::ItemFlag<'mc>>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeItemFlags",
            "(Lorg/bukkit/inventory/ItemFlag;)V",
            &[],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn item_flags(
        &mut self,
    ) -> Result<blackboxmc_java::JavaSet<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getItemFlags",
            "()Ljava/util/Set;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::JavaSet::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn has_item_flag(
        &mut self,
        arg0: impl Into<&'mc crate::inventory::ItemFlag<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "hasItemFlag",
            "(Lorg/bukkit/inventory/ItemFlag;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn is_unbreakable(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isUnbreakable", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn set_unbreakable(&mut self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        // -2
        let val_1 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setUnbreakable",
            "(Z)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn has_attribute_modifiers(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "hasAttributeModifiers", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn attribute_modifiers(
        &mut self,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getAttributeModifiers",
            "()Lcom/google/common/collect/Multimap;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }

    pub fn add_attribute_modifier(
        &mut self,
        arg0: impl Into<&'mc crate::attribute::Attribute<'mc>>,
        arg1: impl Into<&'mc crate::attribute::AttributeModifier<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let val_2 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "addAttributeModifier",
            "(Lorg/bukkit/attribute/Attribute;Lorg/bukkit/attribute/AttributeModifier;)Z",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub unsafe fn set_attribute_modifiers(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setAttributeModifiers",
            "(Lcom/google/common/collect/Multimap;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn remove_attribute_modifier_with_attribute(
        &mut self,
        arg0: impl Into<&'mc crate::attribute::Attribute<'mc>>,
        arg1: std::option::Option<impl Into<&'mc crate::attribute::AttributeModifier<'mc>>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let val_2 =
            unsafe { jni::objects::JObject::from_raw(arg1.unwrap().into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeAttributeModifier",
            "(Lorg/bukkit/attribute/Attribute;Lorg/bukkit/attribute/AttributeModifier;)Z",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    #[deprecated]

    pub fn custom_tag_container(
        &mut self,
    ) -> Result<crate::inventory::meta::tags::CustomItemTagContainer<'mc>, Box<dyn std::error::Error>>
    {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getCustomTagContainer",
            "()Lorg/bukkit/inventory/meta/tags/CustomItemTagContainer;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::meta::tags::CustomItemTagContainer::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    #[deprecated]

    pub fn set_version(&mut self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setVersion",
            "(I)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

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

    pub fn persistent_data_container(
        &mut self,
    ) -> Result<crate::persistence::PersistentDataContainer<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getPersistentDataContainer",
            "()Lorg/bukkit/persistence/PersistentDataContainer;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::persistence::PersistentDataContainer::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
}
impl<'mc> JNIRaw<'mc> for KnowledgeBookMeta<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Into<crate::inventory::meta::ItemMeta<'mc>> for KnowledgeBookMeta<'mc> {
    fn into(self) -> crate::inventory::meta::ItemMeta<'mc> {
        crate::inventory::meta::ItemMeta::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
/// Represents a map that can be scalable.
///
/// This is a representation of an abstract class.
pub struct MapMeta<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> MapMeta<'mc> {
    pub fn from_raw(
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
    /// Sets the map color. A custom map color will alter the display of the map in an inventory slot.
    pub fn set_color(
        &mut self,
        arg0: impl Into<&'mc crate::Color<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setColor",
            "(Lorg/bukkit/Color;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Gets the map color that is set. A custom map color will alter the display of the map in an inventory slot.
    /// <p>Plugins should check that hasColor() returns <code>true</code> before calling this method.</p>
    pub fn color(&mut self) -> Result<crate::Color<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getColor", "()Lorg/bukkit/Color;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        crate::Color::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Checks for existence of a map color.
    pub fn has_color(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasColor", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    /// Gets the map view that is associated with this map item.
    /// <p>Plugins should check that hasMapView() returns <code>true</code> before calling this method.</p>
    pub fn map_view(&mut self) -> Result<crate::map::MapView<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getMapView",
            "()Lorg/bukkit/map/MapView;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::map::MapView::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    #[deprecated]
    /// <span class="deprecated-label">Deprecated.</span>
    /// <div class="deprecation-comment">
    /// These methods are poor API: They rely on the caller to pass in an only an integer property, and have poorly defined implementation behavior if that integer is not a valid map (the current implementation for example will generate a new map with a different ID). The xxxMapView family of methods should be used instead.
    /// </div>
    /// These methods are poor API: They rely on the caller to pass in an only an integer property, and have poorly defined implementation behavior if that integer is not a valid map (the current implementation for example will generate a new map with a different ID). The xxxMapView family of methods should be used instead.
    ///
    /// Checks for existence of a map ID number.
    pub fn has_map_id(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasMapId", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    #[deprecated]
    /// <span class="deprecated-label">Deprecated.</span>
    /// <div class="deprecation-comment">
    /// These methods are poor API: They rely on the caller to pass in an only an integer property, and have poorly defined implementation behavior if that integer is not a valid map (the current implementation for example will generate a new map with a different ID). The xxxMapView family of methods should be used instead.
    /// </div>
    /// These methods are poor API: They rely on the caller to pass in an only an integer property, and have poorly defined implementation behavior if that integer is not a valid map (the current implementation for example will generate a new map with a different ID). The xxxMapView family of methods should be used instead.
    ///
    /// Gets the map ID that is set. This is used to determine what map is displayed.
    /// <p>Plugins should check that hasMapId() returns <code>true</code> before calling this method.</p>
    pub fn map_id(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getMapId", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }
    #[deprecated]
    /// <span class="deprecated-label">Deprecated.</span>
    /// <div class="deprecation-comment">
    /// These methods are poor API: They rely on the caller to pass in an only an integer property, and have poorly defined implementation behavior if that integer is not a valid map (the current implementation for example will generate a new map with a different ID). The xxxMapView family of methods should be used instead.
    /// </div>
    /// These methods are poor API: They rely on the caller to pass in an only an integer property, and have poorly defined implementation behavior if that integer is not a valid map (the current implementation for example will generate a new map with a different ID). The xxxMapView family of methods should be used instead.
    ///
    /// Sets the map ID. This is used to determine what map is displayed.
    pub fn set_map_id(&mut self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setMapId",
            "(I)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Checks for existence of an associated map.
    pub fn has_map_view(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasMapView", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    /// Sets the associated map. This is used to determine what map is displayed.
    /// <p>The implementation <b>may</b> allow null to clear the associated map, but this is not required and is liable to generate a new (undefined) map when the item is first used.</p>
    pub fn set_map_view(
        &mut self,
        arg0: impl Into<&'mc crate::map::MapView<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setMapView",
            "(Lorg/bukkit/map/MapView;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Checks to see if this map is scaling.
    pub fn is_scaling(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isScaling", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    /// Sets if this map is scaling or not.
    pub fn set_scaling(&mut self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        // -2
        let val_1 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setScaling",
            "(Z)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    #[deprecated]
    /// <span class="deprecated-label">Deprecated.</span>
    /// <div class="deprecation-comment">
    /// This method does not have the expected effect and is actually an alias for <a href="ItemMeta.html#hasLocalizedName()"><code>ItemMeta.hasLocalizedName()</code></a>.
    /// </div>
    /// This method does not have the expected effect and is actually an alias for <a href="ItemMeta.html#hasLocalizedName()"><code>ItemMeta.hasLocalizedName()</code></a>.
    ///
    /// Checks for existence of a location name.
    pub fn has_location_name(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasLocationName", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    #[deprecated]
    /// <span class="deprecated-label">Deprecated.</span>
    /// <div class="deprecation-comment">
    /// This method does not have the expected effect and is actually an alias for <a href="ItemMeta.html#getLocalizedName()"><code>ItemMeta.getLocalizedName()</code></a>.
    /// </div>
    /// This method does not have the expected effect and is actually an alias for <a href="ItemMeta.html#getLocalizedName()"><code>ItemMeta.getLocalizedName()</code></a>.
    ///
    /// Gets the location name that is set.
    /// <p>Plugins should check that hasLocationName() returns <code>true</code> before calling this method.</p>
    pub fn location_name(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getLocationName",
            "()Ljava/lang/String;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    #[deprecated]
    /// <span class="deprecated-label">Deprecated.</span>
    /// <div class="deprecation-comment">
    /// This method does not have the expected effect and is actually an alias for <a href="ItemMeta.html#setLocalizedName(java.lang.String)"><code>ItemMeta.setLocalizedName(String)</code></a>.
    /// </div>
    /// This method does not have the expected effect and is actually an alias for <a href="ItemMeta.html#setLocalizedName(java.lang.String)"><code>ItemMeta.setLocalizedName(String)</code></a>.
    ///
    /// Sets the location name.
    pub fn set_location_name(
        &mut self,
        arg0: impl Into<&'mc String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setLocationName",
            "(Ljava/lang/String;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn clone(
        &mut self,
    ) -> Result<crate::inventory::meta::MapMeta<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "clone",
            "()Lorg/bukkit/inventory/meta/MapMeta;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::meta::MapMeta::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn display_name(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getDisplayName",
            "()Ljava/lang/String;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }

    pub fn localized_name(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getLocalizedName",
            "()Ljava/lang/String;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }

    pub fn get_enchant_level(
        &mut self,
        arg0: impl Into<&'mc crate::enchantments::Enchantment<'mc>>,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getEnchantLevel",
            "(Lorg/bukkit/enchantments/Enchantment;)I",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }

    pub fn set_lore(
        &mut self,
        arg0: Vec<impl Into<String>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let raw_val_1 = self
            .jni_ref()
            .new_object("java/util/ArrayList", "()V", &[])
            .unwrap();
        for v in arg0 {
            let map_val_0 =
                jni::objects::JObject::from(self.jni_ref().new_string(v.into()).unwrap());
            self.jni_ref().call_method(
                &raw_val_1,
                "add",
                "(Ljava/Lang/Object)V",
                &[jni::objects::JValueGen::from(&map_val_0)],
            )?;
        }
        let val_1 = jni::objects::JValueGen::Object(raw_val_1);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setLore",
            "(Ljava/util/List;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn as_string(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getAsString",
            "()Ljava/lang/String;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }

    pub fn has_display_name(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasDisplayName", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn set_display_name(
        &mut self,
        arg0: impl Into<&'mc String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setDisplayName",
            "(Ljava/lang/String;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn has_localized_name(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasLocalizedName", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn set_localized_name(
        &mut self,
        arg0: impl Into<&'mc String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setLocalizedName",
            "(Ljava/lang/String;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn has_lore(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasLore", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn lore(&mut self) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getLore", "()Ljava/util/List;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let mut list = blackboxmc_java::JavaList::from_raw(&self.jni_ref(), res.l()?)?;
        let size = list.size()?;
        for i in 0..=size {
            let obj = list.get(i)?;
            new_vec.push(
                self.jni_ref()
                    .get_string(unsafe { &jni::objects::JString::from_raw(*obj) })?
                    .to_string_lossy()
                    .to_string(),
            );
        }
        Ok(new_vec)
    }

    pub fn has_custom_model_data(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasCustomModelData", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn custom_model_data(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getCustomModelData", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }

    pub fn set_custom_model_data(&mut self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setCustomModelData",
            "(I)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn has_enchants(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasEnchants", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn has_enchant(
        &mut self,
        arg0: impl Into<&'mc crate::enchantments::Enchantment<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "hasEnchant",
            "(Lorg/bukkit/enchantments/Enchantment;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn enchants(
        &mut self,
    ) -> Result<blackboxmc_java::JavaMap<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getEnchants", "()Ljava/util/Map;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::JavaMap::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn add_enchant(
        &mut self,
        arg0: impl Into<&'mc crate::enchantments::Enchantment<'mc>>,
        arg1: i32,
        arg2: bool,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let val_2 = jni::objects::JValueGen::Int(arg1.into());
        // -2
        let val_3 = jni::objects::JValueGen::Bool(arg2.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "addEnchant",
            "(Lorg/bukkit/enchantments/Enchantment;IZ)Z",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
                jni::objects::JValueGen::from(&val_3),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn remove_enchant(
        &mut self,
        arg0: impl Into<&'mc crate::enchantments::Enchantment<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeEnchant",
            "(Lorg/bukkit/enchantments/Enchantment;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn has_conflicting_enchant(
        &mut self,
        arg0: impl Into<&'mc crate::enchantments::Enchantment<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "hasConflictingEnchant",
            "(Lorg/bukkit/enchantments/Enchantment;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn add_item_flags(
        &mut self,
        arg0: Vec<impl Into<crate::inventory::ItemFlag<'mc>>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "addItemFlags",
            "(Lorg/bukkit/inventory/ItemFlag;)V",
            &[],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn remove_item_flags(
        &mut self,
        arg0: Vec<impl Into<crate::inventory::ItemFlag<'mc>>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeItemFlags",
            "(Lorg/bukkit/inventory/ItemFlag;)V",
            &[],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn item_flags(
        &mut self,
    ) -> Result<blackboxmc_java::JavaSet<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getItemFlags",
            "()Ljava/util/Set;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::JavaSet::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn has_item_flag(
        &mut self,
        arg0: impl Into<&'mc crate::inventory::ItemFlag<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "hasItemFlag",
            "(Lorg/bukkit/inventory/ItemFlag;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn is_unbreakable(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isUnbreakable", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn set_unbreakable(&mut self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        // -2
        let val_1 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setUnbreakable",
            "(Z)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn has_attribute_modifiers(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "hasAttributeModifiers", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn attribute_modifiers(
        &mut self,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getAttributeModifiers",
            "()Lcom/google/common/collect/Multimap;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }

    pub fn add_attribute_modifier(
        &mut self,
        arg0: impl Into<&'mc crate::attribute::Attribute<'mc>>,
        arg1: impl Into<&'mc crate::attribute::AttributeModifier<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let val_2 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "addAttributeModifier",
            "(Lorg/bukkit/attribute/Attribute;Lorg/bukkit/attribute/AttributeModifier;)Z",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub unsafe fn set_attribute_modifiers(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setAttributeModifiers",
            "(Lcom/google/common/collect/Multimap;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn remove_attribute_modifier_with_attribute(
        &mut self,
        arg0: impl Into<&'mc crate::attribute::Attribute<'mc>>,
        arg1: std::option::Option<impl Into<&'mc crate::attribute::AttributeModifier<'mc>>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let val_2 =
            unsafe { jni::objects::JObject::from_raw(arg1.unwrap().into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeAttributeModifier",
            "(Lorg/bukkit/attribute/Attribute;Lorg/bukkit/attribute/AttributeModifier;)Z",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    #[deprecated]

    pub fn custom_tag_container(
        &mut self,
    ) -> Result<crate::inventory::meta::tags::CustomItemTagContainer<'mc>, Box<dyn std::error::Error>>
    {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getCustomTagContainer",
            "()Lorg/bukkit/inventory/meta/tags/CustomItemTagContainer;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::meta::tags::CustomItemTagContainer::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    #[deprecated]

    pub fn set_version(&mut self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setVersion",
            "(I)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

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

    pub fn persistent_data_container(
        &mut self,
    ) -> Result<crate::persistence::PersistentDataContainer<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getPersistentDataContainer",
            "()Lorg/bukkit/persistence/PersistentDataContainer;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::persistence::PersistentDataContainer::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
}
impl<'mc> JNIRaw<'mc> for MapMeta<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Into<crate::inventory::meta::ItemMeta<'mc>> for MapMeta<'mc> {
    fn into(self) -> crate::inventory::meta::ItemMeta<'mc> {
        crate::inventory::meta::ItemMeta::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}

pub struct BookMetaSpigot<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for BookMetaSpigot<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> BookMetaSpigot<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate BookMetaSpigot from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/inventory/meta/BookMetaSpigot")?;
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
    pub fn new(
        jni: blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<crate::inventory::meta::BookMetaSpigot<'mc>, Box<dyn std::error::Error>> {
        let cls = &jni.find_class("org/bukkit/inventory/meta/BookMeta$Spigot")?;
        let res = jni.new_object(cls, "()V", &[])?;
        crate::inventory::meta::BookMetaSpigot::from_raw(&jni, res)
    }
    /// Sets the specified page in the book. Pages of the book must be contiguous.
    /// <p>The data can be up to 256 characters in length, additional characters are truncated.</p>
    /// Clears the existing book pages, and sets the book to use the provided pages. Maximum 50 pages with 256 characters per page.
    /// Clears the existing book pages, and sets the book to use the provided pages. Maximum 50 pages with 256 characters per page.
    pub fn set_page(
        &mut self,
        arg0: i32,
        arg1: Vec<impl Into<blackboxmc_bungee::bungee::api::chat::BaseComponent<'mc>>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setPage",
            "(ILnet/md_5/bungee/api/chat/BaseComponent;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Gets all the pages in the book.
    pub fn pages(
        &mut self,
    ) -> Result<
        Vec<blackboxmc_bungee::bungee::api::chat::BaseComponent<'mc>>,
        Box<dyn std::error::Error>,
    > {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getPages", "()Ljava/util/List;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let mut list = blackboxmc_java::JavaList::from_raw(&self.0, res.l()?)?;
        let size = list.size()?;
        for i in 0..=size {
            let obj = list.get(i)?;
            new_vec
                .push(blackboxmc_bungee::bungee::api::chat::BaseComponent::from_raw(&self.0, obj)?);
        }
        Ok(new_vec)
    }
    /// Clears the existing book pages, and sets the book to use the provided pages. Maximum 50 pages with 256 characters per page.
    /// Clears the existing book pages, and sets the book to use the provided pages. Maximum 50 pages with 256 characters per page.
    pub fn set_pages_with_base_componentss(
        &mut self,
        arg0: std::option::Option<
            Vec<impl Into<blackboxmc_bungee::bungee::api::chat::BaseComponent<'mc>>>,
        >,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let raw_val_1 = self
            .jni_ref()
            .new_object("java/util/ArrayList", "()V", &[])
            .unwrap();
        for v in arg0.unwrap() {
            let map_val_0 =
                unsafe { jni::objects::JObject::from_raw(v.into().jni_object().clone()) };
            self.jni_ref().call_method(
                &raw_val_1,
                "add",
                "(Ljava/Lang/Object)V",
                &[jni::objects::JValueGen::from(&map_val_0)],
            )?;
        }
        let val_1 = jni::objects::JValueGen::Object(raw_val_1);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setPages",
            "(Ljava/util/List;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Adds new pages to the end of the book. Up to a maximum of 50 pages with 256 characters per page.
    pub fn add_page(
        &mut self,
        arg0: Vec<impl Into<blackboxmc_bungee::bungee::api::chat::BaseComponent<'mc>>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "addPage",
            "(Lnet/md_5/bungee/api/chat/BaseComponent;)V",
            &[],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
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
/// Represents leather armor (<a href="../../Material.html#LEATHER_BOOTS"><code>Material.LEATHER_BOOTS</code></a>, <a href="../../Material.html#LEATHER_CHESTPLATE"><code>Material.LEATHER_CHESTPLATE</code></a>, <a href="../../Material.html#LEATHER_HELMET"><code>Material.LEATHER_HELMET</code></a>, or <a href="../../Material.html#LEATHER_LEGGINGS"><code>Material.LEATHER_LEGGINGS</code></a>) that can be colored.
///
/// This is a representation of an abstract class.
pub struct LeatherArmorMeta<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> LeatherArmorMeta<'mc> {
    pub fn from_raw(
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
    /// Sets the color of the armor.
    pub fn set_color(
        &mut self,
        arg0: impl Into<&'mc crate::Color<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setColor",
            "(Lorg/bukkit/Color;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Gets the color of the armor. If it has not been set otherwise, it will be <a href="../ItemFactory.html#getDefaultLeatherColor()"><code>ItemFactory.getDefaultLeatherColor()</code></a>.
    pub fn color(&mut self) -> Result<crate::Color<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getColor", "()Lorg/bukkit/Color;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        crate::Color::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn clone(
        &mut self,
    ) -> Result<crate::inventory::meta::LeatherArmorMeta<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "clone",
            "()Lorg/bukkit/inventory/meta/LeatherArmorMeta;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::meta::LeatherArmorMeta::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn display_name(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getDisplayName",
            "()Ljava/lang/String;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }

    pub fn localized_name(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getLocalizedName",
            "()Ljava/lang/String;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }

    pub fn get_enchant_level(
        &mut self,
        arg0: impl Into<&'mc crate::enchantments::Enchantment<'mc>>,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getEnchantLevel",
            "(Lorg/bukkit/enchantments/Enchantment;)I",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }

    pub fn set_lore(
        &mut self,
        arg0: Vec<impl Into<String>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let raw_val_1 = self
            .jni_ref()
            .new_object("java/util/ArrayList", "()V", &[])
            .unwrap();
        for v in arg0 {
            let map_val_0 =
                jni::objects::JObject::from(self.jni_ref().new_string(v.into()).unwrap());
            self.jni_ref().call_method(
                &raw_val_1,
                "add",
                "(Ljava/Lang/Object)V",
                &[jni::objects::JValueGen::from(&map_val_0)],
            )?;
        }
        let val_1 = jni::objects::JValueGen::Object(raw_val_1);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setLore",
            "(Ljava/util/List;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn as_string(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getAsString",
            "()Ljava/lang/String;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }

    pub fn has_display_name(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasDisplayName", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn set_display_name(
        &mut self,
        arg0: impl Into<&'mc String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setDisplayName",
            "(Ljava/lang/String;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn has_localized_name(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasLocalizedName", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn set_localized_name(
        &mut self,
        arg0: impl Into<&'mc String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setLocalizedName",
            "(Ljava/lang/String;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn has_lore(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasLore", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn lore(&mut self) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getLore", "()Ljava/util/List;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let mut list = blackboxmc_java::JavaList::from_raw(&self.jni_ref(), res.l()?)?;
        let size = list.size()?;
        for i in 0..=size {
            let obj = list.get(i)?;
            new_vec.push(
                self.jni_ref()
                    .get_string(unsafe { &jni::objects::JString::from_raw(*obj) })?
                    .to_string_lossy()
                    .to_string(),
            );
        }
        Ok(new_vec)
    }

    pub fn has_custom_model_data(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasCustomModelData", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn custom_model_data(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getCustomModelData", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }

    pub fn set_custom_model_data(&mut self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setCustomModelData",
            "(I)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn has_enchants(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasEnchants", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn has_enchant(
        &mut self,
        arg0: impl Into<&'mc crate::enchantments::Enchantment<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "hasEnchant",
            "(Lorg/bukkit/enchantments/Enchantment;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn enchants(
        &mut self,
    ) -> Result<blackboxmc_java::JavaMap<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getEnchants", "()Ljava/util/Map;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::JavaMap::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn add_enchant(
        &mut self,
        arg0: impl Into<&'mc crate::enchantments::Enchantment<'mc>>,
        arg1: i32,
        arg2: bool,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let val_2 = jni::objects::JValueGen::Int(arg1.into());
        // -2
        let val_3 = jni::objects::JValueGen::Bool(arg2.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "addEnchant",
            "(Lorg/bukkit/enchantments/Enchantment;IZ)Z",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
                jni::objects::JValueGen::from(&val_3),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn remove_enchant(
        &mut self,
        arg0: impl Into<&'mc crate::enchantments::Enchantment<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeEnchant",
            "(Lorg/bukkit/enchantments/Enchantment;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn has_conflicting_enchant(
        &mut self,
        arg0: impl Into<&'mc crate::enchantments::Enchantment<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "hasConflictingEnchant",
            "(Lorg/bukkit/enchantments/Enchantment;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn add_item_flags(
        &mut self,
        arg0: Vec<impl Into<crate::inventory::ItemFlag<'mc>>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "addItemFlags",
            "(Lorg/bukkit/inventory/ItemFlag;)V",
            &[],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn remove_item_flags(
        &mut self,
        arg0: Vec<impl Into<crate::inventory::ItemFlag<'mc>>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeItemFlags",
            "(Lorg/bukkit/inventory/ItemFlag;)V",
            &[],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn item_flags(
        &mut self,
    ) -> Result<blackboxmc_java::JavaSet<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getItemFlags",
            "()Ljava/util/Set;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::JavaSet::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn has_item_flag(
        &mut self,
        arg0: impl Into<&'mc crate::inventory::ItemFlag<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "hasItemFlag",
            "(Lorg/bukkit/inventory/ItemFlag;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn is_unbreakable(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isUnbreakable", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn set_unbreakable(&mut self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        // -2
        let val_1 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setUnbreakable",
            "(Z)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn has_attribute_modifiers(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "hasAttributeModifiers", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn attribute_modifiers(
        &mut self,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getAttributeModifiers",
            "()Lcom/google/common/collect/Multimap;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }

    pub fn add_attribute_modifier(
        &mut self,
        arg0: impl Into<&'mc crate::attribute::Attribute<'mc>>,
        arg1: impl Into<&'mc crate::attribute::AttributeModifier<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let val_2 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "addAttributeModifier",
            "(Lorg/bukkit/attribute/Attribute;Lorg/bukkit/attribute/AttributeModifier;)Z",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub unsafe fn set_attribute_modifiers(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setAttributeModifiers",
            "(Lcom/google/common/collect/Multimap;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn remove_attribute_modifier_with_attribute(
        &mut self,
        arg0: impl Into<&'mc crate::attribute::Attribute<'mc>>,
        arg1: std::option::Option<impl Into<&'mc crate::attribute::AttributeModifier<'mc>>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let val_2 =
            unsafe { jni::objects::JObject::from_raw(arg1.unwrap().into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeAttributeModifier",
            "(Lorg/bukkit/attribute/Attribute;Lorg/bukkit/attribute/AttributeModifier;)Z",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    #[deprecated]

    pub fn custom_tag_container(
        &mut self,
    ) -> Result<crate::inventory::meta::tags::CustomItemTagContainer<'mc>, Box<dyn std::error::Error>>
    {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getCustomTagContainer",
            "()Lorg/bukkit/inventory/meta/tags/CustomItemTagContainer;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::meta::tags::CustomItemTagContainer::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    #[deprecated]

    pub fn set_version(&mut self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setVersion",
            "(I)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

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

    pub fn persistent_data_container(
        &mut self,
    ) -> Result<crate::persistence::PersistentDataContainer<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getPersistentDataContainer",
            "()Lorg/bukkit/persistence/PersistentDataContainer;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::persistence::PersistentDataContainer::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
}
impl<'mc> JNIRaw<'mc> for LeatherArmorMeta<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Into<crate::inventory::meta::ItemMeta<'mc>> for LeatherArmorMeta<'mc> {
    fn into(self) -> crate::inventory::meta::ItemMeta<'mc> {
        crate::inventory::meta::ItemMeta::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
/// Represents a meta that can store a single FireworkEffect. An example includes <a href="../../Material.html#FIREWORK_STAR"><code>Material.FIREWORK_STAR</code></a>.
///
/// This is a representation of an abstract class.
pub struct FireworkEffectMeta<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> FireworkEffectMeta<'mc> {
    pub fn from_raw(
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
    /// Sets the firework effect for this meta.
    pub fn set_effect(
        &mut self,
        arg0: impl Into<&'mc crate::FireworkEffect<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setEffect",
            "(Lorg/bukkit/FireworkEffect;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Checks if this meta has an effect.
    pub fn has_effect(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasEffect", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    /// Gets the firework effect for this meta.
    pub fn effect(&mut self) -> Result<crate::FireworkEffect<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getEffect",
            "()Lorg/bukkit/FireworkEffect;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::FireworkEffect::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn clone(
        &mut self,
    ) -> Result<crate::inventory::meta::FireworkEffectMeta<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "clone",
            "()Lorg/bukkit/inventory/meta/FireworkEffectMeta;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::meta::FireworkEffectMeta::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn display_name(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getDisplayName",
            "()Ljava/lang/String;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }

    pub fn localized_name(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getLocalizedName",
            "()Ljava/lang/String;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }

    pub fn get_enchant_level(
        &mut self,
        arg0: impl Into<&'mc crate::enchantments::Enchantment<'mc>>,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getEnchantLevel",
            "(Lorg/bukkit/enchantments/Enchantment;)I",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }

    pub fn set_lore(
        &mut self,
        arg0: Vec<impl Into<String>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let raw_val_1 = self
            .jni_ref()
            .new_object("java/util/ArrayList", "()V", &[])
            .unwrap();
        for v in arg0 {
            let map_val_0 =
                jni::objects::JObject::from(self.jni_ref().new_string(v.into()).unwrap());
            self.jni_ref().call_method(
                &raw_val_1,
                "add",
                "(Ljava/Lang/Object)V",
                &[jni::objects::JValueGen::from(&map_val_0)],
            )?;
        }
        let val_1 = jni::objects::JValueGen::Object(raw_val_1);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setLore",
            "(Ljava/util/List;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn as_string(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getAsString",
            "()Ljava/lang/String;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }

    pub fn has_display_name(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasDisplayName", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn set_display_name(
        &mut self,
        arg0: impl Into<&'mc String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setDisplayName",
            "(Ljava/lang/String;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn has_localized_name(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasLocalizedName", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn set_localized_name(
        &mut self,
        arg0: impl Into<&'mc String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setLocalizedName",
            "(Ljava/lang/String;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn has_lore(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasLore", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn lore(&mut self) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getLore", "()Ljava/util/List;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let mut list = blackboxmc_java::JavaList::from_raw(&self.jni_ref(), res.l()?)?;
        let size = list.size()?;
        for i in 0..=size {
            let obj = list.get(i)?;
            new_vec.push(
                self.jni_ref()
                    .get_string(unsafe { &jni::objects::JString::from_raw(*obj) })?
                    .to_string_lossy()
                    .to_string(),
            );
        }
        Ok(new_vec)
    }

    pub fn has_custom_model_data(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasCustomModelData", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn custom_model_data(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getCustomModelData", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }

    pub fn set_custom_model_data(&mut self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setCustomModelData",
            "(I)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn has_enchants(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasEnchants", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn has_enchant(
        &mut self,
        arg0: impl Into<&'mc crate::enchantments::Enchantment<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "hasEnchant",
            "(Lorg/bukkit/enchantments/Enchantment;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn enchants(
        &mut self,
    ) -> Result<blackboxmc_java::JavaMap<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getEnchants", "()Ljava/util/Map;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::JavaMap::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn add_enchant(
        &mut self,
        arg0: impl Into<&'mc crate::enchantments::Enchantment<'mc>>,
        arg1: i32,
        arg2: bool,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let val_2 = jni::objects::JValueGen::Int(arg1.into());
        // -2
        let val_3 = jni::objects::JValueGen::Bool(arg2.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "addEnchant",
            "(Lorg/bukkit/enchantments/Enchantment;IZ)Z",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
                jni::objects::JValueGen::from(&val_3),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn remove_enchant(
        &mut self,
        arg0: impl Into<&'mc crate::enchantments::Enchantment<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeEnchant",
            "(Lorg/bukkit/enchantments/Enchantment;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn has_conflicting_enchant(
        &mut self,
        arg0: impl Into<&'mc crate::enchantments::Enchantment<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "hasConflictingEnchant",
            "(Lorg/bukkit/enchantments/Enchantment;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn add_item_flags(
        &mut self,
        arg0: Vec<impl Into<crate::inventory::ItemFlag<'mc>>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "addItemFlags",
            "(Lorg/bukkit/inventory/ItemFlag;)V",
            &[],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn remove_item_flags(
        &mut self,
        arg0: Vec<impl Into<crate::inventory::ItemFlag<'mc>>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeItemFlags",
            "(Lorg/bukkit/inventory/ItemFlag;)V",
            &[],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn item_flags(
        &mut self,
    ) -> Result<blackboxmc_java::JavaSet<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getItemFlags",
            "()Ljava/util/Set;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::JavaSet::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn has_item_flag(
        &mut self,
        arg0: impl Into<&'mc crate::inventory::ItemFlag<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "hasItemFlag",
            "(Lorg/bukkit/inventory/ItemFlag;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn is_unbreakable(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isUnbreakable", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn set_unbreakable(&mut self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        // -2
        let val_1 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setUnbreakable",
            "(Z)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn has_attribute_modifiers(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "hasAttributeModifiers", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn attribute_modifiers(
        &mut self,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getAttributeModifiers",
            "()Lcom/google/common/collect/Multimap;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }

    pub fn add_attribute_modifier(
        &mut self,
        arg0: impl Into<&'mc crate::attribute::Attribute<'mc>>,
        arg1: impl Into<&'mc crate::attribute::AttributeModifier<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let val_2 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "addAttributeModifier",
            "(Lorg/bukkit/attribute/Attribute;Lorg/bukkit/attribute/AttributeModifier;)Z",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub unsafe fn set_attribute_modifiers(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setAttributeModifiers",
            "(Lcom/google/common/collect/Multimap;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn remove_attribute_modifier_with_attribute(
        &mut self,
        arg0: impl Into<&'mc crate::attribute::Attribute<'mc>>,
        arg1: std::option::Option<impl Into<&'mc crate::attribute::AttributeModifier<'mc>>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let val_2 =
            unsafe { jni::objects::JObject::from_raw(arg1.unwrap().into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeAttributeModifier",
            "(Lorg/bukkit/attribute/Attribute;Lorg/bukkit/attribute/AttributeModifier;)Z",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    #[deprecated]

    pub fn custom_tag_container(
        &mut self,
    ) -> Result<crate::inventory::meta::tags::CustomItemTagContainer<'mc>, Box<dyn std::error::Error>>
    {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getCustomTagContainer",
            "()Lorg/bukkit/inventory/meta/tags/CustomItemTagContainer;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::meta::tags::CustomItemTagContainer::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    #[deprecated]

    pub fn set_version(&mut self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setVersion",
            "(I)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

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

    pub fn persistent_data_container(
        &mut self,
    ) -> Result<crate::persistence::PersistentDataContainer<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getPersistentDataContainer",
            "()Lorg/bukkit/persistence/PersistentDataContainer;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::persistence::PersistentDataContainer::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
}
impl<'mc> JNIRaw<'mc> for FireworkEffectMeta<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Into<crate::inventory::meta::ItemMeta<'mc>> for FireworkEffectMeta<'mc> {
    fn into(self) -> crate::inventory::meta::ItemMeta<'mc> {
        crate::inventory::meta::ItemMeta::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
/// Represents a bucket of tropical fish.
///
/// This is a representation of an abstract class.
pub struct TropicalFishBucketMeta<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> TropicalFishBucketMeta<'mc> {
    pub fn from_raw(
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
    /// Gets the color of the fish's pattern.
    /// <p>Plugins should check that hasVariant() returns <code>true</code> before calling this method.</p>
    /// Gets the fish's pattern.
    /// <p>Plugins should check that hasVariant() returns <code>true</code> before calling this method.</p>
    pub fn pattern(
        &mut self,
    ) -> Result<crate::entity::TropicalFishPattern<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getPattern",
            "()Lorg/bukkit/entity/TropicalFish$Pattern;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::entity::TropicalFishPattern::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Sets the color of the fish's pattern.
    /// <p>Setting this when hasVariant() returns <code>false</code> will initialize all other values to unspecified defaults.</p>
    /// Sets the fish's pattern.
    /// <p>Setting this when hasVariant() returns <code>false</code> will initialize all other values to unspecified defaults.</p>
    pub fn set_pattern(
        &mut self,
        arg0: impl Into<&'mc crate::entity::TropicalFishPattern<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setPattern",
            "(Lorg/bukkit/entity/TropicalFish$Pattern;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Gets the color of the fish's pattern.
    /// <p>Plugins should check that hasVariant() returns <code>true</code> before calling this method.</p>
    pub fn pattern_color(&mut self) -> Result<crate::DyeColor<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getPatternColor",
            "()Lorg/bukkit/DyeColor;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant =
            self.jni_ref()
                .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
        let variant_str = self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::DyeColor::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::DyeColor::from_string(variant_str).unwrap(),
        )
    }
    /// Sets the color of the fish's pattern.
    /// <p>Setting this when hasVariant() returns <code>false</code> will initialize all other values to unspecified defaults.</p>
    pub fn set_pattern_color(
        &mut self,
        arg0: impl Into<&'mc crate::DyeColor<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setPatternColor",
            "(Lorg/bukkit/DyeColor;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Gets the color of the fish's body.
    /// <p>Plugins should check that hasVariant() returns <code>true</code> before calling this method.</p>
    pub fn body_color(&mut self) -> Result<crate::DyeColor<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getBodyColor",
            "()Lorg/bukkit/DyeColor;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant =
            self.jni_ref()
                .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
        let variant_str = self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::DyeColor::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::DyeColor::from_string(variant_str).unwrap(),
        )
    }
    /// Sets the color of the fish's body.
    /// <p>Setting this when hasVariant() returns <code>false</code> will initialize all other values to unspecified defaults.</p>
    pub fn set_body_color(
        &mut self,
        arg0: impl Into<&'mc crate::DyeColor<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setBodyColor",
            "(Lorg/bukkit/DyeColor;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Checks for existence of a variant tag indicating a specific fish will be spawned.
    pub fn has_variant(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasVariant", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn clone(&mut self) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "clone", "()Ljava/lang/Object;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }

    pub fn display_name(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getDisplayName",
            "()Ljava/lang/String;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }

    pub fn localized_name(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getLocalizedName",
            "()Ljava/lang/String;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }

    pub fn get_enchant_level(
        &mut self,
        arg0: impl Into<&'mc crate::enchantments::Enchantment<'mc>>,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getEnchantLevel",
            "(Lorg/bukkit/enchantments/Enchantment;)I",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }

    pub fn set_lore(
        &mut self,
        arg0: Vec<impl Into<String>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let raw_val_1 = self
            .jni_ref()
            .new_object("java/util/ArrayList", "()V", &[])
            .unwrap();
        for v in arg0 {
            let map_val_0 =
                jni::objects::JObject::from(self.jni_ref().new_string(v.into()).unwrap());
            self.jni_ref().call_method(
                &raw_val_1,
                "add",
                "(Ljava/Lang/Object)V",
                &[jni::objects::JValueGen::from(&map_val_0)],
            )?;
        }
        let val_1 = jni::objects::JValueGen::Object(raw_val_1);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setLore",
            "(Ljava/util/List;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn as_string(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getAsString",
            "()Ljava/lang/String;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }

    pub fn has_display_name(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasDisplayName", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn set_display_name(
        &mut self,
        arg0: impl Into<&'mc String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setDisplayName",
            "(Ljava/lang/String;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn has_localized_name(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasLocalizedName", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn set_localized_name(
        &mut self,
        arg0: impl Into<&'mc String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setLocalizedName",
            "(Ljava/lang/String;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn has_lore(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasLore", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn lore(&mut self) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getLore", "()Ljava/util/List;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let mut list = blackboxmc_java::JavaList::from_raw(&self.jni_ref(), res.l()?)?;
        let size = list.size()?;
        for i in 0..=size {
            let obj = list.get(i)?;
            new_vec.push(
                self.jni_ref()
                    .get_string(unsafe { &jni::objects::JString::from_raw(*obj) })?
                    .to_string_lossy()
                    .to_string(),
            );
        }
        Ok(new_vec)
    }

    pub fn has_custom_model_data(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasCustomModelData", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn custom_model_data(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getCustomModelData", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }

    pub fn set_custom_model_data(&mut self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setCustomModelData",
            "(I)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn has_enchants(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasEnchants", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn has_enchant(
        &mut self,
        arg0: impl Into<&'mc crate::enchantments::Enchantment<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "hasEnchant",
            "(Lorg/bukkit/enchantments/Enchantment;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn enchants(
        &mut self,
    ) -> Result<blackboxmc_java::JavaMap<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getEnchants", "()Ljava/util/Map;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::JavaMap::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn add_enchant(
        &mut self,
        arg0: impl Into<&'mc crate::enchantments::Enchantment<'mc>>,
        arg1: i32,
        arg2: bool,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let val_2 = jni::objects::JValueGen::Int(arg1.into());
        // -2
        let val_3 = jni::objects::JValueGen::Bool(arg2.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "addEnchant",
            "(Lorg/bukkit/enchantments/Enchantment;IZ)Z",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
                jni::objects::JValueGen::from(&val_3),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn remove_enchant(
        &mut self,
        arg0: impl Into<&'mc crate::enchantments::Enchantment<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeEnchant",
            "(Lorg/bukkit/enchantments/Enchantment;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn has_conflicting_enchant(
        &mut self,
        arg0: impl Into<&'mc crate::enchantments::Enchantment<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "hasConflictingEnchant",
            "(Lorg/bukkit/enchantments/Enchantment;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn add_item_flags(
        &mut self,
        arg0: Vec<impl Into<crate::inventory::ItemFlag<'mc>>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "addItemFlags",
            "(Lorg/bukkit/inventory/ItemFlag;)V",
            &[],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn remove_item_flags(
        &mut self,
        arg0: Vec<impl Into<crate::inventory::ItemFlag<'mc>>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeItemFlags",
            "(Lorg/bukkit/inventory/ItemFlag;)V",
            &[],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn item_flags(
        &mut self,
    ) -> Result<blackboxmc_java::JavaSet<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getItemFlags",
            "()Ljava/util/Set;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::JavaSet::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn has_item_flag(
        &mut self,
        arg0: impl Into<&'mc crate::inventory::ItemFlag<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "hasItemFlag",
            "(Lorg/bukkit/inventory/ItemFlag;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn is_unbreakable(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isUnbreakable", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn set_unbreakable(&mut self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        // -2
        let val_1 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setUnbreakable",
            "(Z)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn has_attribute_modifiers(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "hasAttributeModifiers", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn attribute_modifiers(
        &mut self,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getAttributeModifiers",
            "()Lcom/google/common/collect/Multimap;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }

    pub fn add_attribute_modifier(
        &mut self,
        arg0: impl Into<&'mc crate::attribute::Attribute<'mc>>,
        arg1: impl Into<&'mc crate::attribute::AttributeModifier<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let val_2 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "addAttributeModifier",
            "(Lorg/bukkit/attribute/Attribute;Lorg/bukkit/attribute/AttributeModifier;)Z",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub unsafe fn set_attribute_modifiers(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setAttributeModifiers",
            "(Lcom/google/common/collect/Multimap;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn remove_attribute_modifier_with_attribute(
        &mut self,
        arg0: impl Into<&'mc crate::attribute::Attribute<'mc>>,
        arg1: std::option::Option<impl Into<&'mc crate::attribute::AttributeModifier<'mc>>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let val_2 =
            unsafe { jni::objects::JObject::from_raw(arg1.unwrap().into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeAttributeModifier",
            "(Lorg/bukkit/attribute/Attribute;Lorg/bukkit/attribute/AttributeModifier;)Z",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    #[deprecated]

    pub fn custom_tag_container(
        &mut self,
    ) -> Result<crate::inventory::meta::tags::CustomItemTagContainer<'mc>, Box<dyn std::error::Error>>
    {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getCustomTagContainer",
            "()Lorg/bukkit/inventory/meta/tags/CustomItemTagContainer;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::meta::tags::CustomItemTagContainer::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    #[deprecated]

    pub fn set_version(&mut self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setVersion",
            "(I)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

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

    pub fn persistent_data_container(
        &mut self,
    ) -> Result<crate::persistence::PersistentDataContainer<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getPersistentDataContainer",
            "()Lorg/bukkit/persistence/PersistentDataContainer;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::persistence::PersistentDataContainer::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
}
impl<'mc> JNIRaw<'mc> for TropicalFishBucketMeta<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Into<crate::inventory::meta::ItemMeta<'mc>> for TropicalFishBucketMeta<'mc> {
    fn into(self) -> crate::inventory::meta::ItemMeta<'mc> {
        crate::inventory::meta::ItemMeta::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
/// Represents a book (<a href="../../Material.html#WRITABLE_BOOK"><code>Material.WRITABLE_BOOK</code></a> or <a href="../../Material.html#WRITTEN_BOOK"><code>Material.WRITTEN_BOOK</code></a>) that can have a title, an author, and pages.
///
/// This is a representation of an abstract class.
pub struct BookMeta<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> BookMeta<'mc> {
    pub fn from_raw(
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

    pub fn spigot(
        &mut self,
    ) -> Result<crate::inventory::meta::BookMetaSpigot<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "spigot",
            "()Lorg/bukkit/inventory/meta/BookMeta$Spigot;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::meta::BookMetaSpigot::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Gets the specified page in the book. The given page must exist.
    /// <p>Pages are 1-indexed.</p>
    /// Gets all the pages in the book.
    /// Gets the number of pages in the book.
    pub fn get_page(&mut self, arg0: i32) -> Result<String, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getPage",
            "(I)Ljava/lang/String;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    /// Sets the specified page in the book. Pages of the book must be contiguous.
    /// <p>The data can be up to 256 characters in length, additional characters are truncated.</p>
    /// <p>Pages are 1-indexed.</p>
    /// Clears the existing book pages, and sets the book to use the provided pages. Maximum 100 pages with 256 characters per page.
    /// Clears the existing book pages, and sets the book to use the provided pages. Maximum 50 pages with 256 characters per page.
    pub fn set_page(
        &mut self,
        arg0: i32,
        arg1: impl Into<&'mc String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let val_2 = jni::objects::JObject::from(self.jni_ref().new_string(arg1.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setPage",
            "(ILjava/lang/String;)V",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Gets the title of the book.
    /// <p>Plugins should check that hasTitle() returns true before calling this method.</p>
    pub fn title(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getTitle", "()Ljava/lang/String;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    /// Gets the author of the book.
    /// <p>Plugins should check that hasAuthor() returns true before calling this method.</p>
    pub fn author(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getAuthor",
            "()Ljava/lang/String;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    /// Sets the author of the book. Removes author when given null.
    pub fn set_author(
        &mut self,
        arg0: impl Into<&'mc String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setAuthor",
            "(Ljava/lang/String;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Gets all the pages in the book.
    pub fn pages(&mut self) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getPages", "()Ljava/util/List;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let mut list = blackboxmc_java::JavaList::from_raw(&self.jni_ref(), res.l()?)?;
        let size = list.size()?;
        for i in 0..=size {
            let obj = list.get(i)?;
            new_vec.push(
                self.jni_ref()
                    .get_string(unsafe { &jni::objects::JString::from_raw(*obj) })?
                    .to_string_lossy()
                    .to_string(),
            );
        }
        Ok(new_vec)
    }
    /// Clears the existing book pages, and sets the book to use the provided pages. Maximum 100 pages with 256 characters per page.
    /// Clears the existing book pages, and sets the book to use the provided pages. Maximum 50 pages with 256 characters per page.
    pub fn set_pages_with_list(
        &mut self,
        arg0: std::option::Option<Vec<impl Into<String>>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setPages",
            "(Ljava/lang/String;)V",
            &[],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Adds new pages to the end of the book. Up to a maximum of 50 pages with 256 characters per page.
    pub fn add_page(
        &mut self,
        arg0: Vec<impl Into<String>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "addPage", "(Ljava/lang/String;)V", &[]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Sets the title of the book.
    /// <p>Limited to 32 characters. Removes title when given null.</p>
    pub fn set_title(
        &mut self,
        arg0: impl Into<&'mc String>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setTitle",
            "(Ljava/lang/String;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    /// Checks for the existence of a title in the book.
    pub fn has_title(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasTitle", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    /// Checks for the existence of an author in the book.
    pub fn has_author(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasAuthor", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    /// Checks for the existence of generation level in the book.
    pub fn has_generation(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasGeneration", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    /// Gets the generation of the book.
    /// <p>Plugins should check that hasGeneration() returns true before calling this method.</p>
    pub fn generation(
        &mut self,
    ) -> Result<crate::inventory::meta::BookMetaGeneration<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getGeneration",
            "()Lorg/bukkit/inventory/meta/BookMeta$Generation;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::meta::BookMetaGeneration::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Sets the generation of the book. Removes generation when given null.
    pub fn set_generation(
        &mut self,
        arg0: impl Into<&'mc crate::inventory::meta::BookMetaGeneration<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setGeneration",
            "(Lorg/bukkit/inventory/meta/BookMeta$Generation;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Checks for the existence of pages in the book.
    pub fn has_pages(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasPages", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    /// Gets the number of pages in the book.
    pub fn page_count(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getPageCount", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }

    pub fn clone(
        &mut self,
    ) -> Result<crate::inventory::meta::BookMeta<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "clone",
            "()Lorg/bukkit/inventory/meta/BookMeta;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::meta::BookMeta::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn display_name(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getDisplayName",
            "()Ljava/lang/String;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }

    pub fn localized_name(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getLocalizedName",
            "()Ljava/lang/String;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }

    pub fn get_enchant_level(
        &mut self,
        arg0: impl Into<&'mc crate::enchantments::Enchantment<'mc>>,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getEnchantLevel",
            "(Lorg/bukkit/enchantments/Enchantment;)I",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }

    pub fn set_lore(
        &mut self,
        arg0: Vec<impl Into<String>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let raw_val_1 = self
            .jni_ref()
            .new_object("java/util/ArrayList", "()V", &[])
            .unwrap();
        for v in arg0 {
            let map_val_0 =
                jni::objects::JObject::from(self.jni_ref().new_string(v.into()).unwrap());
            self.jni_ref().call_method(
                &raw_val_1,
                "add",
                "(Ljava/Lang/Object)V",
                &[jni::objects::JValueGen::from(&map_val_0)],
            )?;
        }
        let val_1 = jni::objects::JValueGen::Object(raw_val_1);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setLore",
            "(Ljava/util/List;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn as_string(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getAsString",
            "()Ljava/lang/String;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }

    pub fn has_display_name(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasDisplayName", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn set_display_name(
        &mut self,
        arg0: impl Into<&'mc String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setDisplayName",
            "(Ljava/lang/String;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn has_localized_name(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasLocalizedName", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn set_localized_name(
        &mut self,
        arg0: impl Into<&'mc String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setLocalizedName",
            "(Ljava/lang/String;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn has_lore(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasLore", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn lore(&mut self) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getLore", "()Ljava/util/List;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let mut list = blackboxmc_java::JavaList::from_raw(&self.jni_ref(), res.l()?)?;
        let size = list.size()?;
        for i in 0..=size {
            let obj = list.get(i)?;
            new_vec.push(
                self.jni_ref()
                    .get_string(unsafe { &jni::objects::JString::from_raw(*obj) })?
                    .to_string_lossy()
                    .to_string(),
            );
        }
        Ok(new_vec)
    }

    pub fn has_custom_model_data(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasCustomModelData", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn custom_model_data(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getCustomModelData", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }

    pub fn set_custom_model_data(&mut self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setCustomModelData",
            "(I)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn has_enchants(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasEnchants", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn has_enchant(
        &mut self,
        arg0: impl Into<&'mc crate::enchantments::Enchantment<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "hasEnchant",
            "(Lorg/bukkit/enchantments/Enchantment;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn enchants(
        &mut self,
    ) -> Result<blackboxmc_java::JavaMap<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getEnchants", "()Ljava/util/Map;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::JavaMap::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn add_enchant(
        &mut self,
        arg0: impl Into<&'mc crate::enchantments::Enchantment<'mc>>,
        arg1: i32,
        arg2: bool,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let val_2 = jni::objects::JValueGen::Int(arg1.into());
        // -2
        let val_3 = jni::objects::JValueGen::Bool(arg2.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "addEnchant",
            "(Lorg/bukkit/enchantments/Enchantment;IZ)Z",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
                jni::objects::JValueGen::from(&val_3),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn remove_enchant(
        &mut self,
        arg0: impl Into<&'mc crate::enchantments::Enchantment<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeEnchant",
            "(Lorg/bukkit/enchantments/Enchantment;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn has_conflicting_enchant(
        &mut self,
        arg0: impl Into<&'mc crate::enchantments::Enchantment<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "hasConflictingEnchant",
            "(Lorg/bukkit/enchantments/Enchantment;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn add_item_flags(
        &mut self,
        arg0: Vec<impl Into<crate::inventory::ItemFlag<'mc>>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "addItemFlags",
            "(Lorg/bukkit/inventory/ItemFlag;)V",
            &[],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn remove_item_flags(
        &mut self,
        arg0: Vec<impl Into<crate::inventory::ItemFlag<'mc>>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeItemFlags",
            "(Lorg/bukkit/inventory/ItemFlag;)V",
            &[],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn item_flags(
        &mut self,
    ) -> Result<blackboxmc_java::JavaSet<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getItemFlags",
            "()Ljava/util/Set;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::JavaSet::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn has_item_flag(
        &mut self,
        arg0: impl Into<&'mc crate::inventory::ItemFlag<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "hasItemFlag",
            "(Lorg/bukkit/inventory/ItemFlag;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn is_unbreakable(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isUnbreakable", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn set_unbreakable(&mut self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        // -2
        let val_1 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setUnbreakable",
            "(Z)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn has_attribute_modifiers(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "hasAttributeModifiers", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn attribute_modifiers(
        &mut self,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getAttributeModifiers",
            "()Lcom/google/common/collect/Multimap;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }

    pub fn add_attribute_modifier(
        &mut self,
        arg0: impl Into<&'mc crate::attribute::Attribute<'mc>>,
        arg1: impl Into<&'mc crate::attribute::AttributeModifier<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let val_2 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "addAttributeModifier",
            "(Lorg/bukkit/attribute/Attribute;Lorg/bukkit/attribute/AttributeModifier;)Z",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub unsafe fn set_attribute_modifiers(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setAttributeModifiers",
            "(Lcom/google/common/collect/Multimap;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn remove_attribute_modifier_with_attribute(
        &mut self,
        arg0: impl Into<&'mc crate::attribute::Attribute<'mc>>,
        arg1: std::option::Option<impl Into<&'mc crate::attribute::AttributeModifier<'mc>>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let val_2 =
            unsafe { jni::objects::JObject::from_raw(arg1.unwrap().into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeAttributeModifier",
            "(Lorg/bukkit/attribute/Attribute;Lorg/bukkit/attribute/AttributeModifier;)Z",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    #[deprecated]

    pub fn custom_tag_container(
        &mut self,
    ) -> Result<crate::inventory::meta::tags::CustomItemTagContainer<'mc>, Box<dyn std::error::Error>>
    {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getCustomTagContainer",
            "()Lorg/bukkit/inventory/meta/tags/CustomItemTagContainer;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::meta::tags::CustomItemTagContainer::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    #[deprecated]

    pub fn set_version(&mut self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setVersion",
            "(I)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

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

    pub fn persistent_data_container(
        &mut self,
    ) -> Result<crate::persistence::PersistentDataContainer<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getPersistentDataContainer",
            "()Lorg/bukkit/persistence/PersistentDataContainer;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::persistence::PersistentDataContainer::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
}
impl<'mc> JNIRaw<'mc> for BookMeta<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Into<crate::inventory::meta::ItemMeta<'mc>> for BookMeta<'mc> {
    fn into(self) -> crate::inventory::meta::ItemMeta<'mc> {
        crate::inventory::meta::ItemMeta::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}

///
/// This is a representation of an abstract class.
pub struct MusicInstrumentMeta<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> MusicInstrumentMeta<'mc> {
    pub fn from_raw(
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
    /// Gets the instrument of the goat horn.
    pub fn instrument(
        &mut self,
    ) -> Result<crate::MusicInstrument<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getInstrument",
            "()Lorg/bukkit/MusicInstrument;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::MusicInstrument::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Sets the goat horn's instrument.
    pub fn set_instrument(
        &mut self,
        arg0: impl Into<&'mc crate::MusicInstrument<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setInstrument",
            "(Lorg/bukkit/MusicInstrument;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn clone(
        &mut self,
    ) -> Result<crate::inventory::meta::MusicInstrumentMeta<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "clone",
            "()Lorg/bukkit/inventory/meta/MusicInstrumentMeta;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::meta::MusicInstrumentMeta::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn display_name(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getDisplayName",
            "()Ljava/lang/String;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }

    pub fn localized_name(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getLocalizedName",
            "()Ljava/lang/String;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }

    pub fn get_enchant_level(
        &mut self,
        arg0: impl Into<&'mc crate::enchantments::Enchantment<'mc>>,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getEnchantLevel",
            "(Lorg/bukkit/enchantments/Enchantment;)I",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }

    pub fn set_lore(
        &mut self,
        arg0: Vec<impl Into<String>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let raw_val_1 = self
            .jni_ref()
            .new_object("java/util/ArrayList", "()V", &[])
            .unwrap();
        for v in arg0 {
            let map_val_0 =
                jni::objects::JObject::from(self.jni_ref().new_string(v.into()).unwrap());
            self.jni_ref().call_method(
                &raw_val_1,
                "add",
                "(Ljava/Lang/Object)V",
                &[jni::objects::JValueGen::from(&map_val_0)],
            )?;
        }
        let val_1 = jni::objects::JValueGen::Object(raw_val_1);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setLore",
            "(Ljava/util/List;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn as_string(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getAsString",
            "()Ljava/lang/String;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }

    pub fn has_display_name(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasDisplayName", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn set_display_name(
        &mut self,
        arg0: impl Into<&'mc String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setDisplayName",
            "(Ljava/lang/String;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn has_localized_name(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasLocalizedName", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn set_localized_name(
        &mut self,
        arg0: impl Into<&'mc String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setLocalizedName",
            "(Ljava/lang/String;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn has_lore(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasLore", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn lore(&mut self) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getLore", "()Ljava/util/List;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let mut list = blackboxmc_java::JavaList::from_raw(&self.jni_ref(), res.l()?)?;
        let size = list.size()?;
        for i in 0..=size {
            let obj = list.get(i)?;
            new_vec.push(
                self.jni_ref()
                    .get_string(unsafe { &jni::objects::JString::from_raw(*obj) })?
                    .to_string_lossy()
                    .to_string(),
            );
        }
        Ok(new_vec)
    }

    pub fn has_custom_model_data(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasCustomModelData", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn custom_model_data(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getCustomModelData", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }

    pub fn set_custom_model_data(&mut self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setCustomModelData",
            "(I)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn has_enchants(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasEnchants", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn has_enchant(
        &mut self,
        arg0: impl Into<&'mc crate::enchantments::Enchantment<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "hasEnchant",
            "(Lorg/bukkit/enchantments/Enchantment;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn enchants(
        &mut self,
    ) -> Result<blackboxmc_java::JavaMap<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getEnchants", "()Ljava/util/Map;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::JavaMap::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn add_enchant(
        &mut self,
        arg0: impl Into<&'mc crate::enchantments::Enchantment<'mc>>,
        arg1: i32,
        arg2: bool,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let val_2 = jni::objects::JValueGen::Int(arg1.into());
        // -2
        let val_3 = jni::objects::JValueGen::Bool(arg2.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "addEnchant",
            "(Lorg/bukkit/enchantments/Enchantment;IZ)Z",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
                jni::objects::JValueGen::from(&val_3),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn remove_enchant(
        &mut self,
        arg0: impl Into<&'mc crate::enchantments::Enchantment<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeEnchant",
            "(Lorg/bukkit/enchantments/Enchantment;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn has_conflicting_enchant(
        &mut self,
        arg0: impl Into<&'mc crate::enchantments::Enchantment<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "hasConflictingEnchant",
            "(Lorg/bukkit/enchantments/Enchantment;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn add_item_flags(
        &mut self,
        arg0: Vec<impl Into<crate::inventory::ItemFlag<'mc>>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "addItemFlags",
            "(Lorg/bukkit/inventory/ItemFlag;)V",
            &[],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn remove_item_flags(
        &mut self,
        arg0: Vec<impl Into<crate::inventory::ItemFlag<'mc>>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeItemFlags",
            "(Lorg/bukkit/inventory/ItemFlag;)V",
            &[],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn item_flags(
        &mut self,
    ) -> Result<blackboxmc_java::JavaSet<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getItemFlags",
            "()Ljava/util/Set;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::JavaSet::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn has_item_flag(
        &mut self,
        arg0: impl Into<&'mc crate::inventory::ItemFlag<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "hasItemFlag",
            "(Lorg/bukkit/inventory/ItemFlag;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn is_unbreakable(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isUnbreakable", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn set_unbreakable(&mut self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        // -2
        let val_1 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setUnbreakable",
            "(Z)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn has_attribute_modifiers(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "hasAttributeModifiers", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn attribute_modifiers(
        &mut self,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getAttributeModifiers",
            "()Lcom/google/common/collect/Multimap;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }

    pub fn add_attribute_modifier(
        &mut self,
        arg0: impl Into<&'mc crate::attribute::Attribute<'mc>>,
        arg1: impl Into<&'mc crate::attribute::AttributeModifier<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let val_2 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "addAttributeModifier",
            "(Lorg/bukkit/attribute/Attribute;Lorg/bukkit/attribute/AttributeModifier;)Z",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub unsafe fn set_attribute_modifiers(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setAttributeModifiers",
            "(Lcom/google/common/collect/Multimap;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn remove_attribute_modifier_with_attribute(
        &mut self,
        arg0: impl Into<&'mc crate::attribute::Attribute<'mc>>,
        arg1: std::option::Option<impl Into<&'mc crate::attribute::AttributeModifier<'mc>>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let val_2 =
            unsafe { jni::objects::JObject::from_raw(arg1.unwrap().into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeAttributeModifier",
            "(Lorg/bukkit/attribute/Attribute;Lorg/bukkit/attribute/AttributeModifier;)Z",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    #[deprecated]

    pub fn custom_tag_container(
        &mut self,
    ) -> Result<crate::inventory::meta::tags::CustomItemTagContainer<'mc>, Box<dyn std::error::Error>>
    {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getCustomTagContainer",
            "()Lorg/bukkit/inventory/meta/tags/CustomItemTagContainer;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::meta::tags::CustomItemTagContainer::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    #[deprecated]

    pub fn set_version(&mut self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setVersion",
            "(I)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

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

    pub fn persistent_data_container(
        &mut self,
    ) -> Result<crate::persistence::PersistentDataContainer<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getPersistentDataContainer",
            "()Lorg/bukkit/persistence/PersistentDataContainer;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::persistence::PersistentDataContainer::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
}
impl<'mc> JNIRaw<'mc> for MusicInstrumentMeta<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Into<crate::inventory::meta::ItemMeta<'mc>> for MusicInstrumentMeta<'mc> {
    fn into(self) -> crate::inventory::meta::ItemMeta<'mc> {
        crate::inventory::meta::ItemMeta::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
pub mod tags;
pub mod trim;
