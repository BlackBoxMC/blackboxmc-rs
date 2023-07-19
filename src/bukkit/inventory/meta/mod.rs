#![allow(deprecated)]
use crate::JNIRaw;
pub struct BookMetaGeneration<'mc>(
    pub(crate) crate::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> crate::JNIRaw<'mc> for BookMetaGeneration<'mc> {
    fn jni_ref(&self) -> crate::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> BookMetaGeneration<'mc> {
    pub fn from_raw(
        env: &crate::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate BookMetaGeneration from null object.").into(),
            );
        }
        let cls = env.jni.borrow().get_object_class(&obj)?;
        let name_raw = env.call_method(cls, "getName", "()Ljava/lang/String;", &[])?;
        let oh = name_raw.l()?.into();
        let what = env.get_string(&oh)?;
        let name = what.to_string_lossy();
        if !name.ends_with("BookMetaGeneration") {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a BookMetaGeneration object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    pub fn value_of_with_string(
        jni: crate::SharedJNIEnv<'mc>,
        arg0: std::option::Option<jni::objects::JClass<'mc>>,
        arg1: std::option::Option<String>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let val_0 = arg0.unwrap();
        let val_1 = jni::objects::JObject::from(jni.new_string(arg1.unwrap()).unwrap());
        let cls = &jni.find_class("java/lang/Enum")?;
        let res = jni.call_static_method(
            cls,
            "valueOf",
            "(Ljava/lang/Class;Ljava/lang/String;)Ljava/lang/Enum;",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        )?;
        Ok(res.l().unwrap())
    }
    pub fn name(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "name", "()Ljava/lang/String;", &[])?;
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
        let val_0 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "equals",
            "(Ljava/lang/Object;)Z",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(res.z().unwrap())
    }
    pub fn to_string(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "toString",
            "()Ljava/lang/String;",
            &[],
        )?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    pub fn hash_code(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hashCode", "()I", &[])?;
        Ok(res.i().unwrap())
    }
    pub fn compare_to_with_object(
        &mut self,
        arg0: std::option::Option<jni::objects::JObject<'mc>>,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        let val_0 = arg0.unwrap();
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "compareTo",
            "(Ljava/lang/Enum;)I",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(res.i().unwrap())
    }
    pub fn describe_constable(
        &mut self,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "describeConstable",
            "()Ljava/util/Optional;",
            &[],
        )?;
        Ok(res.l().unwrap())
    }
    pub fn declaring_class(
        &mut self,
    ) -> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getDeclaringClass",
            "()Ljava/lang/Class;",
            &[],
        )?;
        Ok(unsafe { jni::objects::JClass::from_raw(res.as_jni().l) })
    }
    pub fn ordinal(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "ordinal", "()I", &[])?;
        Ok(res.i().unwrap())
    }
    pub fn wait(
        &mut self,
        arg0: std::option::Option<i64>,
        arg1: std::option::Option<i32>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Long(arg0.unwrap().into());
        let val_1 = jni::objects::JValueGen::Int(arg1.unwrap().into());
        self.jni_ref().call_method(
            &self.jni_object(),
            "wait",
            "(JI)V",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        )?;
        Ok(())
    }
    pub fn class(&mut self) -> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getClass",
            "()Ljava/lang/Class;",
            &[],
        )?;
        Ok(unsafe { jni::objects::JClass::from_raw(res.as_jni().l) })
    }
    pub fn notify(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.jni_ref()
            .call_method(&self.jni_object(), "notify", "()V", &[])?;
        Ok(())
    }
    pub fn notify_all(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.jni_ref()
            .call_method(&self.jni_object(), "notifyAll", "()V", &[])?;
        Ok(())
    }
}
/// An instantiatable struct that implements BlockDataMeta. Needed for returning it from Java.
pub struct BlockDataMeta<'mc>(
    pub(crate) crate::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> BlockDataMeta<'mc> {
    pub fn from_raw(
        env: &crate::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate BlockDataMeta from null object.").into());
        }
        let cls = env.jni.borrow().get_object_class(&obj)?;
        let name_raw = env.call_method(cls, "getName", "()Ljava/lang/String;", &[])?;
        let oh = name_raw.l()?.into();
        let what = env.get_string(&oh)?;
        let name = what.to_string_lossy();
        if !name.ends_with("BlockDataMeta") {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a BlockDataMeta object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    pub fn get_block_data(
        &mut self,
        arg0: impl Into<crate::bukkit::Material<'mc>>,
    ) -> Result<crate::bukkit::block::data::BlockData<'mc>, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getBlockData",
            "(Lorg/bukkit/Material;)Lorg/bukkit/block/data/BlockData;",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        let ret = {
            crate::bukkit::block::data::BlockData(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn set_block_data(
        &mut self,
        arg0: impl Into<crate::bukkit::block::data::BlockData<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        self.jni_ref().call_method(
            &self.jni_object(),
            "setBlockData",
            "(Lorg/bukkit/block/data/BlockData;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn has_block_data(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasBlockData", "()Z", &[])?;
        Ok(res.z().unwrap())
    }
    pub fn display_name(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getDisplayName",
            "()Ljava/lang/String;",
            &[],
        )?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    pub fn as_string(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getAsString",
            "()Ljava/lang/String;",
            &[],
        )?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    pub fn has_display_name(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasDisplayName", "()Z", &[])?;
        Ok(res.z().unwrap())
    }
    pub fn set_display_name(&mut self, arg0: String) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JObject::from(self.jni_ref().new_string(arg0).unwrap());
        self.jni_ref().call_method(
            &self.jni_object(),
            "setDisplayName",
            "(Ljava/lang/String;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn has_localized_name(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasLocalizedName", "()Z", &[])?;
        Ok(res.z().unwrap())
    }
    pub fn localized_name(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getLocalizedName",
            "()Ljava/lang/String;",
            &[],
        )?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    pub fn set_localized_name(&mut self, arg0: String) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JObject::from(self.jni_ref().new_string(arg0).unwrap());
        self.jni_ref().call_method(
            &self.jni_object(),
            "setLocalizedName",
            "(Ljava/lang/String;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn has_lore(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasLore", "()Z", &[])?;
        Ok(res.z().unwrap())
    }
    pub fn set_lore(&mut self, arg0: Vec<String>) -> Result<(), Box<dyn std::error::Error>> {
        let raw_val_0 = self
            .jni_ref()
            .new_object("java/util/ArrayList", "()V", &[])
            .unwrap();
        for v in arg0 {
            let map_val_0 = jni::objects::JObject::from(self.jni_ref().new_string(v).unwrap());
            self.jni_ref().call_method(
                &raw_val_0,
                "add",
                "(Ljava/Lang/Object)V",
                &[jni::objects::JValueGen::from(&map_val_0)],
            )?;
        }
        let val_0 = jni::objects::JValueGen::Object(raw_val_0);
        self.jni_ref().call_method(
            &self.jni_object(),
            "setLore",
            "(Ljava/util/List;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn has_custom_model_data(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "hasCustomModelData", "()Z", &[])?;
        Ok(res.z().unwrap())
    }
    pub fn custom_model_data(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getCustomModelData", "()I", &[])?;
        Ok(res.i().unwrap())
    }
    pub fn set_custom_model_data(&mut self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Int(arg0.into());
        self.jni_ref().call_method(
            &self.jni_object(),
            "setCustomModelData",
            "(I)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn has_enchants(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasEnchants", "()Z", &[])?;
        Ok(res.z().unwrap())
    }
    pub fn has_enchant(
        &mut self,
        arg0: impl Into<crate::bukkit::enchantments::Enchantment<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "hasEnchant",
            "(Lorg/bukkit/enchantments/Enchantment;)Z",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(res.z().unwrap())
    }
    pub fn get_enchant_level(
        &mut self,
        arg0: impl Into<crate::bukkit::enchantments::Enchantment<'mc>>,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getEnchantLevel",
            "(Lorg/bukkit/enchantments/Enchantment;)I",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(res.i().unwrap())
    }
    pub fn add_enchant(
        &mut self,
        arg0: impl Into<crate::bukkit::enchantments::Enchantment<'mc>>,
        arg1: i32,
        arg2: bool,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let val_1 = jni::objects::JValueGen::Int(arg1.into());
        // -2
        let val_2 = jni::objects::JValueGen::Bool(arg2.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "addEnchant",
            "(Lorg/bukkit/enchantments/Enchantment;IZ)Z",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        )?;
        Ok(res.z().unwrap())
    }
    pub fn remove_enchant(
        &mut self,
        arg0: impl Into<crate::bukkit::enchantments::Enchantment<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeEnchant",
            "(Lorg/bukkit/enchantments/Enchantment;)Z",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(res.z().unwrap())
    }
    pub fn has_conflicting_enchant(
        &mut self,
        arg0: impl Into<crate::bukkit::enchantments::Enchantment<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "hasConflictingEnchant",
            "(Lorg/bukkit/enchantments/Enchantment;)Z",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(res.z().unwrap())
    }
    pub fn add_item_flags(
        &mut self,
        _arg0: Vec<impl Into<crate::bukkit::inventory::ItemFlag<'mc>>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.jni_ref().call_method(
            &self.jni_object(),
            "addItemFlags",
            "(Lorg/bukkit/inventory/ItemFlag;)V",
            &[],
        )?;
        Ok(())
    }
    pub fn remove_item_flags(
        &mut self,
        _arg0: Vec<impl Into<crate::bukkit::inventory::ItemFlag<'mc>>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.jni_ref().call_method(
            &self.jni_object(),
            "removeItemFlags",
            "(Lorg/bukkit/inventory/ItemFlag;)V",
            &[],
        )?;
        Ok(())
    }
    pub fn has_item_flag(
        &mut self,
        arg0: impl Into<crate::bukkit::inventory::ItemFlag<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "hasItemFlag",
            "(Lorg/bukkit/inventory/ItemFlag;)Z",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(res.z().unwrap())
    }
    pub fn is_unbreakable(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isUnbreakable", "()Z", &[])?;
        Ok(res.z().unwrap())
    }
    pub fn set_unbreakable(&mut self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        // -2
        let val_0 = jni::objects::JValueGen::Bool(arg0.into());
        self.jni_ref().call_method(
            &self.jni_object(),
            "setUnbreakable",
            "(Z)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn has_attribute_modifiers(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "hasAttributeModifiers", "()Z", &[])?;
        Ok(res.z().unwrap())
    }
    pub fn add_attribute_modifier(
        &mut self,
        arg0: impl Into<crate::bukkit::attribute::Attribute<'mc>>,
        arg1: impl Into<crate::bukkit::attribute::AttributeModifier<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg1.into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "addAttributeModifier",
            "(Lorg/bukkit/attribute/Attribute;Lorg/bukkit/attribute/AttributeModifier;)Z",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        )?;
        Ok(res.z().unwrap())
    }
    pub fn remove_attribute_modifier_with_attribute(
        &mut self,
        arg0: impl Into<crate::bukkit::attribute::Attribute<'mc>>,
        arg1: std::option::Option<impl Into<crate::bukkit::attribute::AttributeModifier<'mc>>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg1.unwrap().into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeAttributeModifier",
            "(Lorg/bukkit/attribute/Attribute;Lorg/bukkit/attribute/AttributeModifier;)Z",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        )?;
        Ok(res.z().unwrap())
    }
    #[deprecated]
    pub fn custom_tag_container(
        &mut self,
    ) -> Result<
        crate::bukkit::inventory::meta::tags::CustomItemTagContainer<'mc>,
        Box<dyn std::error::Error>,
    > {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getCustomTagContainer",
            "()Lorg/bukkit/inventory/meta/tags/CustomItemTagContainer;",
            &[],
        )?;
        let ret = {
            crate::bukkit::inventory::meta::tags::CustomItemTagContainer(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    #[deprecated]
    pub fn set_version(&mut self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Int(arg0.into());
        self.jni_ref().call_method(
            &self.jni_object(),
            "setVersion",
            "(I)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn clone(&mut self) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "clone", "()Ljava/lang/Object;", &[])?;
        Ok(res.l().unwrap())
    }
    pub fn persistent_data_container(
        &mut self,
    ) -> Result<crate::bukkit::persistence::PersistentDataContainer<'mc>, Box<dyn std::error::Error>>
    {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getPersistentDataContainer",
            "()Lorg/bukkit/persistence/PersistentDataContainer;",
            &[],
        )?;
        let ret = {
            crate::bukkit::persistence::PersistentDataContainer(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
}
impl<'mc> crate::JNIRaw<'mc> for BlockDataMeta<'mc> {
    fn jni_ref(&self) -> crate::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Into<crate::bukkit::inventory::meta::ItemMeta<'mc>> for BlockDataMeta<'mc> {
    fn into(self) -> crate::bukkit::inventory::meta::ItemMeta<'mc> {
        crate::bukkit::inventory::meta::ItemMeta::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
/// An instantiatable struct that implements ArmorMeta. Needed for returning it from Java.
pub struct ArmorMeta<'mc>(
    pub(crate) crate::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> ArmorMeta<'mc> {
    pub fn from_raw(
        env: &crate::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate ArmorMeta from null object.").into());
        }
        let cls = env.jni.borrow().get_object_class(&obj)?;
        let name_raw = env.call_method(cls, "getName", "()Ljava/lang/String;", &[])?;
        let oh = name_raw.l()?.into();
        let what = env.get_string(&oh)?;
        let name = what.to_string_lossy();
        if !name.ends_with("ArmorMeta") {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a ArmorMeta object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    pub fn has_trim(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasTrim", "()Z", &[])?;
        Ok(res.z().unwrap())
    }
    pub fn set_trim(
        &mut self,
        arg0: impl Into<crate::bukkit::inventory::meta::trim::ArmorTrim<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        self.jni_ref().call_method(
            &self.jni_object(),
            "setTrim",
            "(Lorg/bukkit/inventory/meta/trim/ArmorTrim;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn trim(
        &mut self,
    ) -> Result<crate::bukkit::inventory::meta::trim::ArmorTrim<'mc>, Box<dyn std::error::Error>>
    {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getTrim",
            "()Lorg/bukkit/inventory/meta/trim/ArmorTrim;",
            &[],
        )?;
        let ret = {
            crate::bukkit::inventory::meta::trim::ArmorTrim(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn clone(
        &mut self,
    ) -> Result<crate::bukkit::inventory::meta::ArmorMeta<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "clone",
            "()Lorg/bukkit/inventory/meta/ArmorMeta;",
            &[],
        )?;
        let ret = {
            crate::bukkit::inventory::meta::ArmorMeta(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn display_name(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getDisplayName",
            "()Ljava/lang/String;",
            &[],
        )?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    pub fn as_string(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getAsString",
            "()Ljava/lang/String;",
            &[],
        )?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    pub fn has_display_name(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasDisplayName", "()Z", &[])?;
        Ok(res.z().unwrap())
    }
    pub fn set_display_name(&mut self, arg0: String) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JObject::from(self.jni_ref().new_string(arg0).unwrap());
        self.jni_ref().call_method(
            &self.jni_object(),
            "setDisplayName",
            "(Ljava/lang/String;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn has_localized_name(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasLocalizedName", "()Z", &[])?;
        Ok(res.z().unwrap())
    }
    pub fn localized_name(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getLocalizedName",
            "()Ljava/lang/String;",
            &[],
        )?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    pub fn set_localized_name(&mut self, arg0: String) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JObject::from(self.jni_ref().new_string(arg0).unwrap());
        self.jni_ref().call_method(
            &self.jni_object(),
            "setLocalizedName",
            "(Ljava/lang/String;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn has_lore(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasLore", "()Z", &[])?;
        Ok(res.z().unwrap())
    }
    pub fn set_lore(&mut self, arg0: Vec<String>) -> Result<(), Box<dyn std::error::Error>> {
        let raw_val_0 = self
            .jni_ref()
            .new_object("java/util/ArrayList", "()V", &[])
            .unwrap();
        for v in arg0 {
            let map_val_0 = jni::objects::JObject::from(self.jni_ref().new_string(v).unwrap());
            self.jni_ref().call_method(
                &raw_val_0,
                "add",
                "(Ljava/Lang/Object)V",
                &[jni::objects::JValueGen::from(&map_val_0)],
            )?;
        }
        let val_0 = jni::objects::JValueGen::Object(raw_val_0);
        self.jni_ref().call_method(
            &self.jni_object(),
            "setLore",
            "(Ljava/util/List;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn has_custom_model_data(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "hasCustomModelData", "()Z", &[])?;
        Ok(res.z().unwrap())
    }
    pub fn custom_model_data(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getCustomModelData", "()I", &[])?;
        Ok(res.i().unwrap())
    }
    pub fn set_custom_model_data(&mut self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Int(arg0.into());
        self.jni_ref().call_method(
            &self.jni_object(),
            "setCustomModelData",
            "(I)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn has_enchants(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasEnchants", "()Z", &[])?;
        Ok(res.z().unwrap())
    }
    pub fn has_enchant(
        &mut self,
        arg0: impl Into<crate::bukkit::enchantments::Enchantment<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "hasEnchant",
            "(Lorg/bukkit/enchantments/Enchantment;)Z",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(res.z().unwrap())
    }
    pub fn get_enchant_level(
        &mut self,
        arg0: impl Into<crate::bukkit::enchantments::Enchantment<'mc>>,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getEnchantLevel",
            "(Lorg/bukkit/enchantments/Enchantment;)I",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(res.i().unwrap())
    }
    pub fn add_enchant(
        &mut self,
        arg0: impl Into<crate::bukkit::enchantments::Enchantment<'mc>>,
        arg1: i32,
        arg2: bool,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let val_1 = jni::objects::JValueGen::Int(arg1.into());
        // -2
        let val_2 = jni::objects::JValueGen::Bool(arg2.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "addEnchant",
            "(Lorg/bukkit/enchantments/Enchantment;IZ)Z",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        )?;
        Ok(res.z().unwrap())
    }
    pub fn remove_enchant(
        &mut self,
        arg0: impl Into<crate::bukkit::enchantments::Enchantment<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeEnchant",
            "(Lorg/bukkit/enchantments/Enchantment;)Z",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(res.z().unwrap())
    }
    pub fn has_conflicting_enchant(
        &mut self,
        arg0: impl Into<crate::bukkit::enchantments::Enchantment<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "hasConflictingEnchant",
            "(Lorg/bukkit/enchantments/Enchantment;)Z",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(res.z().unwrap())
    }
    pub fn add_item_flags(
        &mut self,
        _arg0: Vec<impl Into<crate::bukkit::inventory::ItemFlag<'mc>>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.jni_ref().call_method(
            &self.jni_object(),
            "addItemFlags",
            "(Lorg/bukkit/inventory/ItemFlag;)V",
            &[],
        )?;
        Ok(())
    }
    pub fn remove_item_flags(
        &mut self,
        _arg0: Vec<impl Into<crate::bukkit::inventory::ItemFlag<'mc>>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.jni_ref().call_method(
            &self.jni_object(),
            "removeItemFlags",
            "(Lorg/bukkit/inventory/ItemFlag;)V",
            &[],
        )?;
        Ok(())
    }
    pub fn has_item_flag(
        &mut self,
        arg0: impl Into<crate::bukkit::inventory::ItemFlag<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "hasItemFlag",
            "(Lorg/bukkit/inventory/ItemFlag;)Z",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(res.z().unwrap())
    }
    pub fn is_unbreakable(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isUnbreakable", "()Z", &[])?;
        Ok(res.z().unwrap())
    }
    pub fn set_unbreakable(&mut self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        // -2
        let val_0 = jni::objects::JValueGen::Bool(arg0.into());
        self.jni_ref().call_method(
            &self.jni_object(),
            "setUnbreakable",
            "(Z)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn has_attribute_modifiers(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "hasAttributeModifiers", "()Z", &[])?;
        Ok(res.z().unwrap())
    }
    pub fn add_attribute_modifier(
        &mut self,
        arg0: impl Into<crate::bukkit::attribute::Attribute<'mc>>,
        arg1: impl Into<crate::bukkit::attribute::AttributeModifier<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg1.into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "addAttributeModifier",
            "(Lorg/bukkit/attribute/Attribute;Lorg/bukkit/attribute/AttributeModifier;)Z",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        )?;
        Ok(res.z().unwrap())
    }
    pub fn remove_attribute_modifier_with_attribute(
        &mut self,
        arg0: impl Into<crate::bukkit::attribute::Attribute<'mc>>,
        arg1: std::option::Option<impl Into<crate::bukkit::attribute::AttributeModifier<'mc>>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg1.unwrap().into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeAttributeModifier",
            "(Lorg/bukkit/attribute/Attribute;Lorg/bukkit/attribute/AttributeModifier;)Z",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        )?;
        Ok(res.z().unwrap())
    }
    #[deprecated]
    pub fn custom_tag_container(
        &mut self,
    ) -> Result<
        crate::bukkit::inventory::meta::tags::CustomItemTagContainer<'mc>,
        Box<dyn std::error::Error>,
    > {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getCustomTagContainer",
            "()Lorg/bukkit/inventory/meta/tags/CustomItemTagContainer;",
            &[],
        )?;
        let ret = {
            crate::bukkit::inventory::meta::tags::CustomItemTagContainer(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    #[deprecated]
    pub fn set_version(&mut self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Int(arg0.into());
        self.jni_ref().call_method(
            &self.jni_object(),
            "setVersion",
            "(I)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn persistent_data_container(
        &mut self,
    ) -> Result<crate::bukkit::persistence::PersistentDataContainer<'mc>, Box<dyn std::error::Error>>
    {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getPersistentDataContainer",
            "()Lorg/bukkit/persistence/PersistentDataContainer;",
            &[],
        )?;
        let ret = {
            crate::bukkit::persistence::PersistentDataContainer(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
}
impl<'mc> crate::JNIRaw<'mc> for ArmorMeta<'mc> {
    fn jni_ref(&self) -> crate::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Into<crate::bukkit::inventory::meta::ItemMeta<'mc>> for ArmorMeta<'mc> {
    fn into(self) -> crate::bukkit::inventory::meta::ItemMeta<'mc> {
        crate::bukkit::inventory::meta::ItemMeta::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
/// An instantiatable struct that implements AxolotlBucketMeta. Needed for returning it from Java.
pub struct AxolotlBucketMeta<'mc>(
    pub(crate) crate::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> AxolotlBucketMeta<'mc> {
    pub fn from_raw(
        env: &crate::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate AxolotlBucketMeta from null object.").into(),
            );
        }
        let cls = env.jni.borrow().get_object_class(&obj)?;
        let name_raw = env.call_method(cls, "getName", "()Ljava/lang/String;", &[])?;
        let oh = name_raw.l()?.into();
        let what = env.get_string(&oh)?;
        let name = what.to_string_lossy();
        if !name.ends_with("AxolotlBucketMeta") {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a AxolotlBucketMeta object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    pub fn variant(
        &mut self,
    ) -> Result<crate::bukkit::entity::AxolotlVariant<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getVariant",
            "()Lorg/bukkit/entity/Axolotl$Variant;",
            &[],
        )?;
        let ret = {
            crate::bukkit::entity::AxolotlVariant(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn set_variant(
        &mut self,
        arg0: impl Into<crate::bukkit::entity::AxolotlVariant<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        self.jni_ref().call_method(
            &self.jni_object(),
            "setVariant",
            "(Lorg/bukkit/entity/Axolotl$Variant;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn has_variant(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasVariant", "()Z", &[])?;
        Ok(res.z().unwrap())
    }
    pub fn clone(
        &mut self,
    ) -> Result<crate::bukkit::inventory::meta::AxolotlBucketMeta<'mc>, Box<dyn std::error::Error>>
    {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "clone",
            "()Lorg/bukkit/inventory/meta/AxolotlBucketMeta;",
            &[],
        )?;
        let ret = {
            crate::bukkit::inventory::meta::AxolotlBucketMeta(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn display_name(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getDisplayName",
            "()Ljava/lang/String;",
            &[],
        )?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    pub fn as_string(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getAsString",
            "()Ljava/lang/String;",
            &[],
        )?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    pub fn has_display_name(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasDisplayName", "()Z", &[])?;
        Ok(res.z().unwrap())
    }
    pub fn set_display_name(&mut self, arg0: String) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JObject::from(self.jni_ref().new_string(arg0).unwrap());
        self.jni_ref().call_method(
            &self.jni_object(),
            "setDisplayName",
            "(Ljava/lang/String;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn has_localized_name(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasLocalizedName", "()Z", &[])?;
        Ok(res.z().unwrap())
    }
    pub fn localized_name(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getLocalizedName",
            "()Ljava/lang/String;",
            &[],
        )?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    pub fn set_localized_name(&mut self, arg0: String) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JObject::from(self.jni_ref().new_string(arg0).unwrap());
        self.jni_ref().call_method(
            &self.jni_object(),
            "setLocalizedName",
            "(Ljava/lang/String;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn has_lore(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasLore", "()Z", &[])?;
        Ok(res.z().unwrap())
    }
    pub fn set_lore(&mut self, arg0: Vec<String>) -> Result<(), Box<dyn std::error::Error>> {
        let raw_val_0 = self
            .jni_ref()
            .new_object("java/util/ArrayList", "()V", &[])
            .unwrap();
        for v in arg0 {
            let map_val_0 = jni::objects::JObject::from(self.jni_ref().new_string(v).unwrap());
            self.jni_ref().call_method(
                &raw_val_0,
                "add",
                "(Ljava/Lang/Object)V",
                &[jni::objects::JValueGen::from(&map_val_0)],
            )?;
        }
        let val_0 = jni::objects::JValueGen::Object(raw_val_0);
        self.jni_ref().call_method(
            &self.jni_object(),
            "setLore",
            "(Ljava/util/List;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn has_custom_model_data(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "hasCustomModelData", "()Z", &[])?;
        Ok(res.z().unwrap())
    }
    pub fn custom_model_data(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getCustomModelData", "()I", &[])?;
        Ok(res.i().unwrap())
    }
    pub fn set_custom_model_data(&mut self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Int(arg0.into());
        self.jni_ref().call_method(
            &self.jni_object(),
            "setCustomModelData",
            "(I)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn has_enchants(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasEnchants", "()Z", &[])?;
        Ok(res.z().unwrap())
    }
    pub fn has_enchant(
        &mut self,
        arg0: impl Into<crate::bukkit::enchantments::Enchantment<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "hasEnchant",
            "(Lorg/bukkit/enchantments/Enchantment;)Z",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(res.z().unwrap())
    }
    pub fn get_enchant_level(
        &mut self,
        arg0: impl Into<crate::bukkit::enchantments::Enchantment<'mc>>,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getEnchantLevel",
            "(Lorg/bukkit/enchantments/Enchantment;)I",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(res.i().unwrap())
    }
    pub fn add_enchant(
        &mut self,
        arg0: impl Into<crate::bukkit::enchantments::Enchantment<'mc>>,
        arg1: i32,
        arg2: bool,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let val_1 = jni::objects::JValueGen::Int(arg1.into());
        // -2
        let val_2 = jni::objects::JValueGen::Bool(arg2.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "addEnchant",
            "(Lorg/bukkit/enchantments/Enchantment;IZ)Z",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        )?;
        Ok(res.z().unwrap())
    }
    pub fn remove_enchant(
        &mut self,
        arg0: impl Into<crate::bukkit::enchantments::Enchantment<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeEnchant",
            "(Lorg/bukkit/enchantments/Enchantment;)Z",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(res.z().unwrap())
    }
    pub fn has_conflicting_enchant(
        &mut self,
        arg0: impl Into<crate::bukkit::enchantments::Enchantment<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "hasConflictingEnchant",
            "(Lorg/bukkit/enchantments/Enchantment;)Z",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(res.z().unwrap())
    }
    pub fn add_item_flags(
        &mut self,
        _arg0: Vec<impl Into<crate::bukkit::inventory::ItemFlag<'mc>>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.jni_ref().call_method(
            &self.jni_object(),
            "addItemFlags",
            "(Lorg/bukkit/inventory/ItemFlag;)V",
            &[],
        )?;
        Ok(())
    }
    pub fn remove_item_flags(
        &mut self,
        _arg0: Vec<impl Into<crate::bukkit::inventory::ItemFlag<'mc>>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.jni_ref().call_method(
            &self.jni_object(),
            "removeItemFlags",
            "(Lorg/bukkit/inventory/ItemFlag;)V",
            &[],
        )?;
        Ok(())
    }
    pub fn has_item_flag(
        &mut self,
        arg0: impl Into<crate::bukkit::inventory::ItemFlag<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "hasItemFlag",
            "(Lorg/bukkit/inventory/ItemFlag;)Z",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(res.z().unwrap())
    }
    pub fn is_unbreakable(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isUnbreakable", "()Z", &[])?;
        Ok(res.z().unwrap())
    }
    pub fn set_unbreakable(&mut self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        // -2
        let val_0 = jni::objects::JValueGen::Bool(arg0.into());
        self.jni_ref().call_method(
            &self.jni_object(),
            "setUnbreakable",
            "(Z)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn has_attribute_modifiers(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "hasAttributeModifiers", "()Z", &[])?;
        Ok(res.z().unwrap())
    }
    pub fn add_attribute_modifier(
        &mut self,
        arg0: impl Into<crate::bukkit::attribute::Attribute<'mc>>,
        arg1: impl Into<crate::bukkit::attribute::AttributeModifier<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg1.into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "addAttributeModifier",
            "(Lorg/bukkit/attribute/Attribute;Lorg/bukkit/attribute/AttributeModifier;)Z",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        )?;
        Ok(res.z().unwrap())
    }
    pub fn remove_attribute_modifier_with_attribute(
        &mut self,
        arg0: impl Into<crate::bukkit::attribute::Attribute<'mc>>,
        arg1: std::option::Option<impl Into<crate::bukkit::attribute::AttributeModifier<'mc>>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg1.unwrap().into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeAttributeModifier",
            "(Lorg/bukkit/attribute/Attribute;Lorg/bukkit/attribute/AttributeModifier;)Z",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        )?;
        Ok(res.z().unwrap())
    }
    #[deprecated]
    pub fn custom_tag_container(
        &mut self,
    ) -> Result<
        crate::bukkit::inventory::meta::tags::CustomItemTagContainer<'mc>,
        Box<dyn std::error::Error>,
    > {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getCustomTagContainer",
            "()Lorg/bukkit/inventory/meta/tags/CustomItemTagContainer;",
            &[],
        )?;
        let ret = {
            crate::bukkit::inventory::meta::tags::CustomItemTagContainer(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    #[deprecated]
    pub fn set_version(&mut self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Int(arg0.into());
        self.jni_ref().call_method(
            &self.jni_object(),
            "setVersion",
            "(I)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn persistent_data_container(
        &mut self,
    ) -> Result<crate::bukkit::persistence::PersistentDataContainer<'mc>, Box<dyn std::error::Error>>
    {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getPersistentDataContainer",
            "()Lorg/bukkit/persistence/PersistentDataContainer;",
            &[],
        )?;
        let ret = {
            crate::bukkit::persistence::PersistentDataContainer(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
}
impl<'mc> crate::JNIRaw<'mc> for AxolotlBucketMeta<'mc> {
    fn jni_ref(&self) -> crate::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Into<crate::bukkit::inventory::meta::ItemMeta<'mc>> for AxolotlBucketMeta<'mc> {
    fn into(self) -> crate::bukkit::inventory::meta::ItemMeta<'mc> {
        crate::bukkit::inventory::meta::ItemMeta::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
/// An instantiatable struct that implements SkullMeta. Needed for returning it from Java.
pub struct SkullMeta<'mc>(
    pub(crate) crate::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> SkullMeta<'mc> {
    pub fn from_raw(
        env: &crate::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate SkullMeta from null object.").into());
        }
        let cls = env.jni.borrow().get_object_class(&obj)?;
        let name_raw = env.call_method(cls, "getName", "()Ljava/lang/String;", &[])?;
        let oh = name_raw.l()?.into();
        let what = env.get_string(&oh)?;
        let name = what.to_string_lossy();
        if !name.ends_with("SkullMeta") {
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
    pub fn set_owner(&mut self, arg0: String) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JObject::from(self.jni_ref().new_string(arg0).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setOwner",
            "(Ljava/lang/String;)Z",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(res.z().unwrap())
    }
    #[deprecated]
    pub fn owner(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getOwner",
            "()Ljava/lang/String;",
            &[],
        )?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    pub fn has_owner(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasOwner", "()Z", &[])?;
        Ok(res.z().unwrap())
    }
    pub fn owning_player(
        &mut self,
    ) -> Result<crate::bukkit::OfflinePlayer<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getOwningPlayer",
            "()Lorg/bukkit/OfflinePlayer;",
            &[],
        )?;
        let ret = {
            crate::bukkit::OfflinePlayer(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn set_owning_player(
        &mut self,
        arg0: impl Into<crate::bukkit::OfflinePlayer<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setOwningPlayer",
            "(Lorg/bukkit/OfflinePlayer;)Z",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(res.z().unwrap())
    }
    pub fn owner_profile(
        &mut self,
    ) -> Result<crate::bukkit::profile::PlayerProfile<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getOwnerProfile",
            "()Lorg/bukkit/profile/PlayerProfile;",
            &[],
        )?;
        let ret = {
            crate::bukkit::profile::PlayerProfile(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn set_owner_profile(
        &mut self,
        arg0: impl Into<crate::bukkit::profile::PlayerProfile<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        self.jni_ref().call_method(
            &self.jni_object(),
            "setOwnerProfile",
            "(Lorg/bukkit/profile/PlayerProfile;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn note_block_sound(
        &mut self,
    ) -> Result<crate::bukkit::NamespacedKey<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getNoteBlockSound",
            "()Lorg/bukkit/NamespacedKey;",
            &[],
        )?;
        let ret = {
            crate::bukkit::NamespacedKey(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn set_note_block_sound(
        &mut self,
        arg0: impl Into<crate::bukkit::NamespacedKey<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        self.jni_ref().call_method(
            &self.jni_object(),
            "setNoteBlockSound",
            "(Lorg/bukkit/NamespacedKey;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn clone(
        &mut self,
    ) -> Result<crate::bukkit::inventory::meta::SkullMeta<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "clone",
            "()Lorg/bukkit/inventory/meta/SkullMeta;",
            &[],
        )?;
        let ret = {
            crate::bukkit::inventory::meta::SkullMeta(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn display_name(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getDisplayName",
            "()Ljava/lang/String;",
            &[],
        )?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    pub fn as_string(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getAsString",
            "()Ljava/lang/String;",
            &[],
        )?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    pub fn has_display_name(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasDisplayName", "()Z", &[])?;
        Ok(res.z().unwrap())
    }
    pub fn set_display_name(&mut self, arg0: String) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JObject::from(self.jni_ref().new_string(arg0).unwrap());
        self.jni_ref().call_method(
            &self.jni_object(),
            "setDisplayName",
            "(Ljava/lang/String;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn has_localized_name(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasLocalizedName", "()Z", &[])?;
        Ok(res.z().unwrap())
    }
    pub fn localized_name(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getLocalizedName",
            "()Ljava/lang/String;",
            &[],
        )?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    pub fn set_localized_name(&mut self, arg0: String) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JObject::from(self.jni_ref().new_string(arg0).unwrap());
        self.jni_ref().call_method(
            &self.jni_object(),
            "setLocalizedName",
            "(Ljava/lang/String;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn has_lore(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasLore", "()Z", &[])?;
        Ok(res.z().unwrap())
    }
    pub fn set_lore(&mut self, arg0: Vec<String>) -> Result<(), Box<dyn std::error::Error>> {
        let raw_val_0 = self
            .jni_ref()
            .new_object("java/util/ArrayList", "()V", &[])
            .unwrap();
        for v in arg0 {
            let map_val_0 = jni::objects::JObject::from(self.jni_ref().new_string(v).unwrap());
            self.jni_ref().call_method(
                &raw_val_0,
                "add",
                "(Ljava/Lang/Object)V",
                &[jni::objects::JValueGen::from(&map_val_0)],
            )?;
        }
        let val_0 = jni::objects::JValueGen::Object(raw_val_0);
        self.jni_ref().call_method(
            &self.jni_object(),
            "setLore",
            "(Ljava/util/List;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn has_custom_model_data(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "hasCustomModelData", "()Z", &[])?;
        Ok(res.z().unwrap())
    }
    pub fn custom_model_data(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getCustomModelData", "()I", &[])?;
        Ok(res.i().unwrap())
    }
    pub fn set_custom_model_data(&mut self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Int(arg0.into());
        self.jni_ref().call_method(
            &self.jni_object(),
            "setCustomModelData",
            "(I)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn has_enchants(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasEnchants", "()Z", &[])?;
        Ok(res.z().unwrap())
    }
    pub fn has_enchant(
        &mut self,
        arg0: impl Into<crate::bukkit::enchantments::Enchantment<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "hasEnchant",
            "(Lorg/bukkit/enchantments/Enchantment;)Z",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(res.z().unwrap())
    }
    pub fn get_enchant_level(
        &mut self,
        arg0: impl Into<crate::bukkit::enchantments::Enchantment<'mc>>,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getEnchantLevel",
            "(Lorg/bukkit/enchantments/Enchantment;)I",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(res.i().unwrap())
    }
    pub fn add_enchant(
        &mut self,
        arg0: impl Into<crate::bukkit::enchantments::Enchantment<'mc>>,
        arg1: i32,
        arg2: bool,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let val_1 = jni::objects::JValueGen::Int(arg1.into());
        // -2
        let val_2 = jni::objects::JValueGen::Bool(arg2.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "addEnchant",
            "(Lorg/bukkit/enchantments/Enchantment;IZ)Z",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        )?;
        Ok(res.z().unwrap())
    }
    pub fn remove_enchant(
        &mut self,
        arg0: impl Into<crate::bukkit::enchantments::Enchantment<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeEnchant",
            "(Lorg/bukkit/enchantments/Enchantment;)Z",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(res.z().unwrap())
    }
    pub fn has_conflicting_enchant(
        &mut self,
        arg0: impl Into<crate::bukkit::enchantments::Enchantment<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "hasConflictingEnchant",
            "(Lorg/bukkit/enchantments/Enchantment;)Z",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(res.z().unwrap())
    }
    pub fn add_item_flags(
        &mut self,
        _arg0: Vec<impl Into<crate::bukkit::inventory::ItemFlag<'mc>>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.jni_ref().call_method(
            &self.jni_object(),
            "addItemFlags",
            "(Lorg/bukkit/inventory/ItemFlag;)V",
            &[],
        )?;
        Ok(())
    }
    pub fn remove_item_flags(
        &mut self,
        _arg0: Vec<impl Into<crate::bukkit::inventory::ItemFlag<'mc>>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.jni_ref().call_method(
            &self.jni_object(),
            "removeItemFlags",
            "(Lorg/bukkit/inventory/ItemFlag;)V",
            &[],
        )?;
        Ok(())
    }
    pub fn has_item_flag(
        &mut self,
        arg0: impl Into<crate::bukkit::inventory::ItemFlag<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "hasItemFlag",
            "(Lorg/bukkit/inventory/ItemFlag;)Z",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(res.z().unwrap())
    }
    pub fn is_unbreakable(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isUnbreakable", "()Z", &[])?;
        Ok(res.z().unwrap())
    }
    pub fn set_unbreakable(&mut self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        // -2
        let val_0 = jni::objects::JValueGen::Bool(arg0.into());
        self.jni_ref().call_method(
            &self.jni_object(),
            "setUnbreakable",
            "(Z)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn has_attribute_modifiers(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "hasAttributeModifiers", "()Z", &[])?;
        Ok(res.z().unwrap())
    }
    pub fn add_attribute_modifier(
        &mut self,
        arg0: impl Into<crate::bukkit::attribute::Attribute<'mc>>,
        arg1: impl Into<crate::bukkit::attribute::AttributeModifier<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg1.into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "addAttributeModifier",
            "(Lorg/bukkit/attribute/Attribute;Lorg/bukkit/attribute/AttributeModifier;)Z",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        )?;
        Ok(res.z().unwrap())
    }
    pub fn remove_attribute_modifier_with_attribute(
        &mut self,
        arg0: impl Into<crate::bukkit::attribute::Attribute<'mc>>,
        arg1: std::option::Option<impl Into<crate::bukkit::attribute::AttributeModifier<'mc>>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg1.unwrap().into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeAttributeModifier",
            "(Lorg/bukkit/attribute/Attribute;Lorg/bukkit/attribute/AttributeModifier;)Z",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        )?;
        Ok(res.z().unwrap())
    }
    #[deprecated]
    pub fn custom_tag_container(
        &mut self,
    ) -> Result<
        crate::bukkit::inventory::meta::tags::CustomItemTagContainer<'mc>,
        Box<dyn std::error::Error>,
    > {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getCustomTagContainer",
            "()Lorg/bukkit/inventory/meta/tags/CustomItemTagContainer;",
            &[],
        )?;
        let ret = {
            crate::bukkit::inventory::meta::tags::CustomItemTagContainer(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    #[deprecated]
    pub fn set_version(&mut self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Int(arg0.into());
        self.jni_ref().call_method(
            &self.jni_object(),
            "setVersion",
            "(I)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn persistent_data_container(
        &mut self,
    ) -> Result<crate::bukkit::persistence::PersistentDataContainer<'mc>, Box<dyn std::error::Error>>
    {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getPersistentDataContainer",
            "()Lorg/bukkit/persistence/PersistentDataContainer;",
            &[],
        )?;
        let ret = {
            crate::bukkit::persistence::PersistentDataContainer(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
}
impl<'mc> crate::JNIRaw<'mc> for SkullMeta<'mc> {
    fn jni_ref(&self) -> crate::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Into<crate::bukkit::inventory::meta::ItemMeta<'mc>> for SkullMeta<'mc> {
    fn into(self) -> crate::bukkit::inventory::meta::ItemMeta<'mc> {
        crate::bukkit::inventory::meta::ItemMeta::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
/// An instantiatable struct that implements CompassMeta. Needed for returning it from Java.
pub struct CompassMeta<'mc>(
    pub(crate) crate::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> CompassMeta<'mc> {
    pub fn from_raw(
        env: &crate::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate CompassMeta from null object.").into());
        }
        let cls = env.jni.borrow().get_object_class(&obj)?;
        let name_raw = env.call_method(cls, "getName", "()Ljava/lang/String;", &[])?;
        let oh = name_raw.l()?.into();
        let what = env.get_string(&oh)?;
        let name = what.to_string_lossy();
        if !name.ends_with("CompassMeta") {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a CompassMeta object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    pub fn has_lodestone(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasLodestone", "()Z", &[])?;
        Ok(res.z().unwrap())
    }
    pub fn lodestone(
        &mut self,
    ) -> Result<crate::bukkit::Location<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getLodestone",
            "()Lorg/bukkit/Location;",
            &[],
        )?;
        let ret = {
            crate::bukkit::Location(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn set_lodestone(
        &mut self,
        arg0: impl Into<crate::bukkit::Location<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        self.jni_ref().call_method(
            &self.jni_object(),
            "setLodestone",
            "(Lorg/bukkit/Location;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn is_lodestone_tracked(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isLodestoneTracked", "()Z", &[])?;
        Ok(res.z().unwrap())
    }
    pub fn set_lodestone_tracked(&mut self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        // -2
        let val_0 = jni::objects::JValueGen::Bool(arg0.into());
        self.jni_ref().call_method(
            &self.jni_object(),
            "setLodestoneTracked",
            "(Z)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn clone(&mut self) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "clone", "()Ljava/lang/Object;", &[])?;
        Ok(res.l().unwrap())
    }
    pub fn display_name(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getDisplayName",
            "()Ljava/lang/String;",
            &[],
        )?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    pub fn as_string(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getAsString",
            "()Ljava/lang/String;",
            &[],
        )?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    pub fn has_display_name(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasDisplayName", "()Z", &[])?;
        Ok(res.z().unwrap())
    }
    pub fn set_display_name(&mut self, arg0: String) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JObject::from(self.jni_ref().new_string(arg0).unwrap());
        self.jni_ref().call_method(
            &self.jni_object(),
            "setDisplayName",
            "(Ljava/lang/String;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn has_localized_name(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasLocalizedName", "()Z", &[])?;
        Ok(res.z().unwrap())
    }
    pub fn localized_name(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getLocalizedName",
            "()Ljava/lang/String;",
            &[],
        )?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    pub fn set_localized_name(&mut self, arg0: String) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JObject::from(self.jni_ref().new_string(arg0).unwrap());
        self.jni_ref().call_method(
            &self.jni_object(),
            "setLocalizedName",
            "(Ljava/lang/String;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn has_lore(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasLore", "()Z", &[])?;
        Ok(res.z().unwrap())
    }
    pub fn set_lore(&mut self, arg0: Vec<String>) -> Result<(), Box<dyn std::error::Error>> {
        let raw_val_0 = self
            .jni_ref()
            .new_object("java/util/ArrayList", "()V", &[])
            .unwrap();
        for v in arg0 {
            let map_val_0 = jni::objects::JObject::from(self.jni_ref().new_string(v).unwrap());
            self.jni_ref().call_method(
                &raw_val_0,
                "add",
                "(Ljava/Lang/Object)V",
                &[jni::objects::JValueGen::from(&map_val_0)],
            )?;
        }
        let val_0 = jni::objects::JValueGen::Object(raw_val_0);
        self.jni_ref().call_method(
            &self.jni_object(),
            "setLore",
            "(Ljava/util/List;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn has_custom_model_data(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "hasCustomModelData", "()Z", &[])?;
        Ok(res.z().unwrap())
    }
    pub fn custom_model_data(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getCustomModelData", "()I", &[])?;
        Ok(res.i().unwrap())
    }
    pub fn set_custom_model_data(&mut self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Int(arg0.into());
        self.jni_ref().call_method(
            &self.jni_object(),
            "setCustomModelData",
            "(I)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn has_enchants(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasEnchants", "()Z", &[])?;
        Ok(res.z().unwrap())
    }
    pub fn has_enchant(
        &mut self,
        arg0: impl Into<crate::bukkit::enchantments::Enchantment<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "hasEnchant",
            "(Lorg/bukkit/enchantments/Enchantment;)Z",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(res.z().unwrap())
    }
    pub fn get_enchant_level(
        &mut self,
        arg0: impl Into<crate::bukkit::enchantments::Enchantment<'mc>>,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getEnchantLevel",
            "(Lorg/bukkit/enchantments/Enchantment;)I",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(res.i().unwrap())
    }
    pub fn add_enchant(
        &mut self,
        arg0: impl Into<crate::bukkit::enchantments::Enchantment<'mc>>,
        arg1: i32,
        arg2: bool,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let val_1 = jni::objects::JValueGen::Int(arg1.into());
        // -2
        let val_2 = jni::objects::JValueGen::Bool(arg2.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "addEnchant",
            "(Lorg/bukkit/enchantments/Enchantment;IZ)Z",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        )?;
        Ok(res.z().unwrap())
    }
    pub fn remove_enchant(
        &mut self,
        arg0: impl Into<crate::bukkit::enchantments::Enchantment<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeEnchant",
            "(Lorg/bukkit/enchantments/Enchantment;)Z",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(res.z().unwrap())
    }
    pub fn has_conflicting_enchant(
        &mut self,
        arg0: impl Into<crate::bukkit::enchantments::Enchantment<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "hasConflictingEnchant",
            "(Lorg/bukkit/enchantments/Enchantment;)Z",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(res.z().unwrap())
    }
    pub fn add_item_flags(
        &mut self,
        _arg0: Vec<impl Into<crate::bukkit::inventory::ItemFlag<'mc>>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.jni_ref().call_method(
            &self.jni_object(),
            "addItemFlags",
            "(Lorg/bukkit/inventory/ItemFlag;)V",
            &[],
        )?;
        Ok(())
    }
    pub fn remove_item_flags(
        &mut self,
        _arg0: Vec<impl Into<crate::bukkit::inventory::ItemFlag<'mc>>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.jni_ref().call_method(
            &self.jni_object(),
            "removeItemFlags",
            "(Lorg/bukkit/inventory/ItemFlag;)V",
            &[],
        )?;
        Ok(())
    }
    pub fn has_item_flag(
        &mut self,
        arg0: impl Into<crate::bukkit::inventory::ItemFlag<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "hasItemFlag",
            "(Lorg/bukkit/inventory/ItemFlag;)Z",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(res.z().unwrap())
    }
    pub fn is_unbreakable(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isUnbreakable", "()Z", &[])?;
        Ok(res.z().unwrap())
    }
    pub fn set_unbreakable(&mut self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        // -2
        let val_0 = jni::objects::JValueGen::Bool(arg0.into());
        self.jni_ref().call_method(
            &self.jni_object(),
            "setUnbreakable",
            "(Z)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn has_attribute_modifiers(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "hasAttributeModifiers", "()Z", &[])?;
        Ok(res.z().unwrap())
    }
    pub fn add_attribute_modifier(
        &mut self,
        arg0: impl Into<crate::bukkit::attribute::Attribute<'mc>>,
        arg1: impl Into<crate::bukkit::attribute::AttributeModifier<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg1.into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "addAttributeModifier",
            "(Lorg/bukkit/attribute/Attribute;Lorg/bukkit/attribute/AttributeModifier;)Z",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        )?;
        Ok(res.z().unwrap())
    }
    pub fn remove_attribute_modifier_with_attribute(
        &mut self,
        arg0: impl Into<crate::bukkit::attribute::Attribute<'mc>>,
        arg1: std::option::Option<impl Into<crate::bukkit::attribute::AttributeModifier<'mc>>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg1.unwrap().into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeAttributeModifier",
            "(Lorg/bukkit/attribute/Attribute;Lorg/bukkit/attribute/AttributeModifier;)Z",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        )?;
        Ok(res.z().unwrap())
    }
    #[deprecated]
    pub fn custom_tag_container(
        &mut self,
    ) -> Result<
        crate::bukkit::inventory::meta::tags::CustomItemTagContainer<'mc>,
        Box<dyn std::error::Error>,
    > {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getCustomTagContainer",
            "()Lorg/bukkit/inventory/meta/tags/CustomItemTagContainer;",
            &[],
        )?;
        let ret = {
            crate::bukkit::inventory::meta::tags::CustomItemTagContainer(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    #[deprecated]
    pub fn set_version(&mut self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Int(arg0.into());
        self.jni_ref().call_method(
            &self.jni_object(),
            "setVersion",
            "(I)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn persistent_data_container(
        &mut self,
    ) -> Result<crate::bukkit::persistence::PersistentDataContainer<'mc>, Box<dyn std::error::Error>>
    {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getPersistentDataContainer",
            "()Lorg/bukkit/persistence/PersistentDataContainer;",
            &[],
        )?;
        let ret = {
            crate::bukkit::persistence::PersistentDataContainer(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
}
impl<'mc> crate::JNIRaw<'mc> for CompassMeta<'mc> {
    fn jni_ref(&self) -> crate::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Into<crate::bukkit::inventory::meta::ItemMeta<'mc>> for CompassMeta<'mc> {
    fn into(self) -> crate::bukkit::inventory::meta::ItemMeta<'mc> {
        crate::bukkit::inventory::meta::ItemMeta::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
/// An instantiatable struct that implements SuspiciousStewMeta. Needed for returning it from Java.
pub struct SuspiciousStewMeta<'mc>(
    pub(crate) crate::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> SuspiciousStewMeta<'mc> {
    pub fn from_raw(
        env: &crate::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate SuspiciousStewMeta from null object.").into(),
            );
        }
        let cls = env.jni.borrow().get_object_class(&obj)?;
        let name_raw = env.call_method(cls, "getName", "()Ljava/lang/String;", &[])?;
        let oh = name_raw.l()?.into();
        let what = env.get_string(&oh)?;
        let name = what.to_string_lossy();
        if !name.ends_with("SuspiciousStewMeta") {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a SuspiciousStewMeta object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    pub fn has_custom_effects(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasCustomEffects", "()Z", &[])?;
        Ok(res.z().unwrap())
    }
    pub fn add_custom_effect(
        &mut self,
        arg0: impl Into<crate::bukkit::potion::PotionEffect<'mc>>,
        arg1: bool,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        // -2
        let val_1 = jni::objects::JValueGen::Bool(arg1.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "addCustomEffect",
            "(Lorg/bukkit/potion/PotionEffect;Z)Z",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        )?;
        Ok(res.z().unwrap())
    }
    pub fn remove_custom_effect(
        &mut self,
        arg0: impl Into<crate::bukkit::potion::PotionEffectType<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeCustomEffect",
            "(Lorg/bukkit/potion/PotionEffectType;)Z",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(res.z().unwrap())
    }
    pub fn has_custom_effect(
        &mut self,
        arg0: impl Into<crate::bukkit::potion::PotionEffectType<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "hasCustomEffect",
            "(Lorg/bukkit/potion/PotionEffectType;)Z",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(res.z().unwrap())
    }
    pub fn clear_custom_effects(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "clearCustomEffects", "()Z", &[])?;
        Ok(res.z().unwrap())
    }
    pub fn clone(
        &mut self,
    ) -> Result<crate::bukkit::inventory::meta::SuspiciousStewMeta<'mc>, Box<dyn std::error::Error>>
    {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "clone",
            "()Lorg/bukkit/inventory/meta/SuspiciousStewMeta;",
            &[],
        )?;
        let ret = {
            crate::bukkit::inventory::meta::SuspiciousStewMeta(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn display_name(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getDisplayName",
            "()Ljava/lang/String;",
            &[],
        )?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    pub fn as_string(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getAsString",
            "()Ljava/lang/String;",
            &[],
        )?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    pub fn has_display_name(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasDisplayName", "()Z", &[])?;
        Ok(res.z().unwrap())
    }
    pub fn set_display_name(&mut self, arg0: String) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JObject::from(self.jni_ref().new_string(arg0).unwrap());
        self.jni_ref().call_method(
            &self.jni_object(),
            "setDisplayName",
            "(Ljava/lang/String;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn has_localized_name(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasLocalizedName", "()Z", &[])?;
        Ok(res.z().unwrap())
    }
    pub fn localized_name(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getLocalizedName",
            "()Ljava/lang/String;",
            &[],
        )?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    pub fn set_localized_name(&mut self, arg0: String) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JObject::from(self.jni_ref().new_string(arg0).unwrap());
        self.jni_ref().call_method(
            &self.jni_object(),
            "setLocalizedName",
            "(Ljava/lang/String;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn has_lore(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasLore", "()Z", &[])?;
        Ok(res.z().unwrap())
    }
    pub fn set_lore(&mut self, arg0: Vec<String>) -> Result<(), Box<dyn std::error::Error>> {
        let raw_val_0 = self
            .jni_ref()
            .new_object("java/util/ArrayList", "()V", &[])
            .unwrap();
        for v in arg0 {
            let map_val_0 = jni::objects::JObject::from(self.jni_ref().new_string(v).unwrap());
            self.jni_ref().call_method(
                &raw_val_0,
                "add",
                "(Ljava/Lang/Object)V",
                &[jni::objects::JValueGen::from(&map_val_0)],
            )?;
        }
        let val_0 = jni::objects::JValueGen::Object(raw_val_0);
        self.jni_ref().call_method(
            &self.jni_object(),
            "setLore",
            "(Ljava/util/List;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn has_custom_model_data(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "hasCustomModelData", "()Z", &[])?;
        Ok(res.z().unwrap())
    }
    pub fn custom_model_data(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getCustomModelData", "()I", &[])?;
        Ok(res.i().unwrap())
    }
    pub fn set_custom_model_data(&mut self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Int(arg0.into());
        self.jni_ref().call_method(
            &self.jni_object(),
            "setCustomModelData",
            "(I)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn has_enchants(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasEnchants", "()Z", &[])?;
        Ok(res.z().unwrap())
    }
    pub fn has_enchant(
        &mut self,
        arg0: impl Into<crate::bukkit::enchantments::Enchantment<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "hasEnchant",
            "(Lorg/bukkit/enchantments/Enchantment;)Z",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(res.z().unwrap())
    }
    pub fn get_enchant_level(
        &mut self,
        arg0: impl Into<crate::bukkit::enchantments::Enchantment<'mc>>,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getEnchantLevel",
            "(Lorg/bukkit/enchantments/Enchantment;)I",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(res.i().unwrap())
    }
    pub fn add_enchant(
        &mut self,
        arg0: impl Into<crate::bukkit::enchantments::Enchantment<'mc>>,
        arg1: i32,
        arg2: bool,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let val_1 = jni::objects::JValueGen::Int(arg1.into());
        // -2
        let val_2 = jni::objects::JValueGen::Bool(arg2.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "addEnchant",
            "(Lorg/bukkit/enchantments/Enchantment;IZ)Z",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        )?;
        Ok(res.z().unwrap())
    }
    pub fn remove_enchant(
        &mut self,
        arg0: impl Into<crate::bukkit::enchantments::Enchantment<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeEnchant",
            "(Lorg/bukkit/enchantments/Enchantment;)Z",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(res.z().unwrap())
    }
    pub fn has_conflicting_enchant(
        &mut self,
        arg0: impl Into<crate::bukkit::enchantments::Enchantment<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "hasConflictingEnchant",
            "(Lorg/bukkit/enchantments/Enchantment;)Z",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(res.z().unwrap())
    }
    pub fn add_item_flags(
        &mut self,
        _arg0: Vec<impl Into<crate::bukkit::inventory::ItemFlag<'mc>>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.jni_ref().call_method(
            &self.jni_object(),
            "addItemFlags",
            "(Lorg/bukkit/inventory/ItemFlag;)V",
            &[],
        )?;
        Ok(())
    }
    pub fn remove_item_flags(
        &mut self,
        _arg0: Vec<impl Into<crate::bukkit::inventory::ItemFlag<'mc>>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.jni_ref().call_method(
            &self.jni_object(),
            "removeItemFlags",
            "(Lorg/bukkit/inventory/ItemFlag;)V",
            &[],
        )?;
        Ok(())
    }
    pub fn has_item_flag(
        &mut self,
        arg0: impl Into<crate::bukkit::inventory::ItemFlag<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "hasItemFlag",
            "(Lorg/bukkit/inventory/ItemFlag;)Z",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(res.z().unwrap())
    }
    pub fn is_unbreakable(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isUnbreakable", "()Z", &[])?;
        Ok(res.z().unwrap())
    }
    pub fn set_unbreakable(&mut self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        // -2
        let val_0 = jni::objects::JValueGen::Bool(arg0.into());
        self.jni_ref().call_method(
            &self.jni_object(),
            "setUnbreakable",
            "(Z)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn has_attribute_modifiers(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "hasAttributeModifiers", "()Z", &[])?;
        Ok(res.z().unwrap())
    }
    pub fn add_attribute_modifier(
        &mut self,
        arg0: impl Into<crate::bukkit::attribute::Attribute<'mc>>,
        arg1: impl Into<crate::bukkit::attribute::AttributeModifier<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg1.into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "addAttributeModifier",
            "(Lorg/bukkit/attribute/Attribute;Lorg/bukkit/attribute/AttributeModifier;)Z",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        )?;
        Ok(res.z().unwrap())
    }
    pub fn remove_attribute_modifier_with_attribute(
        &mut self,
        arg0: impl Into<crate::bukkit::attribute::Attribute<'mc>>,
        arg1: std::option::Option<impl Into<crate::bukkit::attribute::AttributeModifier<'mc>>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg1.unwrap().into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeAttributeModifier",
            "(Lorg/bukkit/attribute/Attribute;Lorg/bukkit/attribute/AttributeModifier;)Z",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        )?;
        Ok(res.z().unwrap())
    }
    #[deprecated]
    pub fn custom_tag_container(
        &mut self,
    ) -> Result<
        crate::bukkit::inventory::meta::tags::CustomItemTagContainer<'mc>,
        Box<dyn std::error::Error>,
    > {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getCustomTagContainer",
            "()Lorg/bukkit/inventory/meta/tags/CustomItemTagContainer;",
            &[],
        )?;
        let ret = {
            crate::bukkit::inventory::meta::tags::CustomItemTagContainer(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    #[deprecated]
    pub fn set_version(&mut self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Int(arg0.into());
        self.jni_ref().call_method(
            &self.jni_object(),
            "setVersion",
            "(I)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn persistent_data_container(
        &mut self,
    ) -> Result<crate::bukkit::persistence::PersistentDataContainer<'mc>, Box<dyn std::error::Error>>
    {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getPersistentDataContainer",
            "()Lorg/bukkit/persistence/PersistentDataContainer;",
            &[],
        )?;
        let ret = {
            crate::bukkit::persistence::PersistentDataContainer(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
}
impl<'mc> crate::JNIRaw<'mc> for SuspiciousStewMeta<'mc> {
    fn jni_ref(&self) -> crate::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Into<crate::bukkit::inventory::meta::ItemMeta<'mc>> for SuspiciousStewMeta<'mc> {
    fn into(self) -> crate::bukkit::inventory::meta::ItemMeta<'mc> {
        crate::bukkit::inventory::meta::ItemMeta::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
/// An instantiatable struct that implements CrossbowMeta. Needed for returning it from Java.
pub struct CrossbowMeta<'mc>(
    pub(crate) crate::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> CrossbowMeta<'mc> {
    pub fn from_raw(
        env: &crate::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate CrossbowMeta from null object.").into());
        }
        let cls = env.jni.borrow().get_object_class(&obj)?;
        let name_raw = env.call_method(cls, "getName", "()Ljava/lang/String;", &[])?;
        let oh = name_raw.l()?.into();
        let what = env.get_string(&oh)?;
        let name = what.to_string_lossy();
        if !name.ends_with("CrossbowMeta") {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a CrossbowMeta object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    pub fn has_charged_projectiles(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "hasChargedProjectiles", "()Z", &[])?;
        Ok(res.z().unwrap())
    }
    pub fn set_charged_projectiles(
        &mut self,
        arg0: Vec<impl Into<crate::bukkit::inventory::ItemStack<'mc>>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let raw_val_0 = self
            .jni_ref()
            .new_object("java/util/ArrayList", "()V", &[])
            .unwrap();
        for v in arg0 {
            let map_val_0 = unsafe { jni::objects::JObject::from_raw(v.into().1.clone()) };
            self.jni_ref().call_method(
                &raw_val_0,
                "add",
                "(Ljava/Lang/Object)V",
                &[jni::objects::JValueGen::from(&map_val_0)],
            )?;
        }
        let val_0 = jni::objects::JValueGen::Object(raw_val_0);
        self.jni_ref().call_method(
            &self.jni_object(),
            "setChargedProjectiles",
            "(Ljava/util/List;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn add_charged_projectile(
        &mut self,
        arg0: impl Into<crate::bukkit::inventory::ItemStack<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        self.jni_ref().call_method(
            &self.jni_object(),
            "addChargedProjectile",
            "(Lorg/bukkit/inventory/ItemStack;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn display_name(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getDisplayName",
            "()Ljava/lang/String;",
            &[],
        )?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    pub fn as_string(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getAsString",
            "()Ljava/lang/String;",
            &[],
        )?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    pub fn has_display_name(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasDisplayName", "()Z", &[])?;
        Ok(res.z().unwrap())
    }
    pub fn set_display_name(&mut self, arg0: String) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JObject::from(self.jni_ref().new_string(arg0).unwrap());
        self.jni_ref().call_method(
            &self.jni_object(),
            "setDisplayName",
            "(Ljava/lang/String;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn has_localized_name(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasLocalizedName", "()Z", &[])?;
        Ok(res.z().unwrap())
    }
    pub fn localized_name(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getLocalizedName",
            "()Ljava/lang/String;",
            &[],
        )?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    pub fn set_localized_name(&mut self, arg0: String) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JObject::from(self.jni_ref().new_string(arg0).unwrap());
        self.jni_ref().call_method(
            &self.jni_object(),
            "setLocalizedName",
            "(Ljava/lang/String;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn has_lore(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasLore", "()Z", &[])?;
        Ok(res.z().unwrap())
    }
    pub fn set_lore(&mut self, arg0: Vec<String>) -> Result<(), Box<dyn std::error::Error>> {
        let raw_val_0 = self
            .jni_ref()
            .new_object("java/util/ArrayList", "()V", &[])
            .unwrap();
        for v in arg0 {
            let map_val_0 = jni::objects::JObject::from(self.jni_ref().new_string(v).unwrap());
            self.jni_ref().call_method(
                &raw_val_0,
                "add",
                "(Ljava/Lang/Object)V",
                &[jni::objects::JValueGen::from(&map_val_0)],
            )?;
        }
        let val_0 = jni::objects::JValueGen::Object(raw_val_0);
        self.jni_ref().call_method(
            &self.jni_object(),
            "setLore",
            "(Ljava/util/List;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn has_custom_model_data(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "hasCustomModelData", "()Z", &[])?;
        Ok(res.z().unwrap())
    }
    pub fn custom_model_data(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getCustomModelData", "()I", &[])?;
        Ok(res.i().unwrap())
    }
    pub fn set_custom_model_data(&mut self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Int(arg0.into());
        self.jni_ref().call_method(
            &self.jni_object(),
            "setCustomModelData",
            "(I)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn has_enchants(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasEnchants", "()Z", &[])?;
        Ok(res.z().unwrap())
    }
    pub fn has_enchant(
        &mut self,
        arg0: impl Into<crate::bukkit::enchantments::Enchantment<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "hasEnchant",
            "(Lorg/bukkit/enchantments/Enchantment;)Z",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(res.z().unwrap())
    }
    pub fn get_enchant_level(
        &mut self,
        arg0: impl Into<crate::bukkit::enchantments::Enchantment<'mc>>,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getEnchantLevel",
            "(Lorg/bukkit/enchantments/Enchantment;)I",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(res.i().unwrap())
    }
    pub fn add_enchant(
        &mut self,
        arg0: impl Into<crate::bukkit::enchantments::Enchantment<'mc>>,
        arg1: i32,
        arg2: bool,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let val_1 = jni::objects::JValueGen::Int(arg1.into());
        // -2
        let val_2 = jni::objects::JValueGen::Bool(arg2.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "addEnchant",
            "(Lorg/bukkit/enchantments/Enchantment;IZ)Z",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        )?;
        Ok(res.z().unwrap())
    }
    pub fn remove_enchant(
        &mut self,
        arg0: impl Into<crate::bukkit::enchantments::Enchantment<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeEnchant",
            "(Lorg/bukkit/enchantments/Enchantment;)Z",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(res.z().unwrap())
    }
    pub fn has_conflicting_enchant(
        &mut self,
        arg0: impl Into<crate::bukkit::enchantments::Enchantment<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "hasConflictingEnchant",
            "(Lorg/bukkit/enchantments/Enchantment;)Z",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(res.z().unwrap())
    }
    pub fn add_item_flags(
        &mut self,
        _arg0: Vec<impl Into<crate::bukkit::inventory::ItemFlag<'mc>>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.jni_ref().call_method(
            &self.jni_object(),
            "addItemFlags",
            "(Lorg/bukkit/inventory/ItemFlag;)V",
            &[],
        )?;
        Ok(())
    }
    pub fn remove_item_flags(
        &mut self,
        _arg0: Vec<impl Into<crate::bukkit::inventory::ItemFlag<'mc>>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.jni_ref().call_method(
            &self.jni_object(),
            "removeItemFlags",
            "(Lorg/bukkit/inventory/ItemFlag;)V",
            &[],
        )?;
        Ok(())
    }
    pub fn has_item_flag(
        &mut self,
        arg0: impl Into<crate::bukkit::inventory::ItemFlag<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "hasItemFlag",
            "(Lorg/bukkit/inventory/ItemFlag;)Z",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(res.z().unwrap())
    }
    pub fn is_unbreakable(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isUnbreakable", "()Z", &[])?;
        Ok(res.z().unwrap())
    }
    pub fn set_unbreakable(&mut self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        // -2
        let val_0 = jni::objects::JValueGen::Bool(arg0.into());
        self.jni_ref().call_method(
            &self.jni_object(),
            "setUnbreakable",
            "(Z)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn has_attribute_modifiers(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "hasAttributeModifiers", "()Z", &[])?;
        Ok(res.z().unwrap())
    }
    pub fn add_attribute_modifier(
        &mut self,
        arg0: impl Into<crate::bukkit::attribute::Attribute<'mc>>,
        arg1: impl Into<crate::bukkit::attribute::AttributeModifier<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg1.into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "addAttributeModifier",
            "(Lorg/bukkit/attribute/Attribute;Lorg/bukkit/attribute/AttributeModifier;)Z",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        )?;
        Ok(res.z().unwrap())
    }
    pub fn remove_attribute_modifier_with_attribute(
        &mut self,
        arg0: impl Into<crate::bukkit::attribute::Attribute<'mc>>,
        arg1: std::option::Option<impl Into<crate::bukkit::attribute::AttributeModifier<'mc>>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg1.unwrap().into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeAttributeModifier",
            "(Lorg/bukkit/attribute/Attribute;Lorg/bukkit/attribute/AttributeModifier;)Z",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        )?;
        Ok(res.z().unwrap())
    }
    #[deprecated]
    pub fn custom_tag_container(
        &mut self,
    ) -> Result<
        crate::bukkit::inventory::meta::tags::CustomItemTagContainer<'mc>,
        Box<dyn std::error::Error>,
    > {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getCustomTagContainer",
            "()Lorg/bukkit/inventory/meta/tags/CustomItemTagContainer;",
            &[],
        )?;
        let ret = {
            crate::bukkit::inventory::meta::tags::CustomItemTagContainer(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    #[deprecated]
    pub fn set_version(&mut self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Int(arg0.into());
        self.jni_ref().call_method(
            &self.jni_object(),
            "setVersion",
            "(I)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn clone(&mut self) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "clone", "()Ljava/lang/Object;", &[])?;
        Ok(res.l().unwrap())
    }
    pub fn persistent_data_container(
        &mut self,
    ) -> Result<crate::bukkit::persistence::PersistentDataContainer<'mc>, Box<dyn std::error::Error>>
    {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getPersistentDataContainer",
            "()Lorg/bukkit/persistence/PersistentDataContainer;",
            &[],
        )?;
        let ret = {
            crate::bukkit::persistence::PersistentDataContainer(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
}
impl<'mc> crate::JNIRaw<'mc> for CrossbowMeta<'mc> {
    fn jni_ref(&self) -> crate::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Into<crate::bukkit::inventory::meta::ItemMeta<'mc>> for CrossbowMeta<'mc> {
    fn into(self) -> crate::bukkit::inventory::meta::ItemMeta<'mc> {
        crate::bukkit::inventory::meta::ItemMeta::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
/// An instantiatable struct that implements ItemMeta. Needed for returning it from Java.
pub struct ItemMeta<'mc>(
    pub(crate) crate::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> ItemMeta<'mc> {
    pub fn from_raw(
        env: &crate::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate ItemMeta from null object.").into());
        }
        let cls = env.jni.borrow().get_object_class(&obj)?;
        let name_raw = env.call_method(cls, "getName", "()Ljava/lang/String;", &[])?;
        let oh = name_raw.l()?.into();
        let what = env.get_string(&oh)?;
        let name = what.to_string_lossy();
        if !name.ends_with("ItemMeta") {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a ItemMeta object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    pub fn display_name(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getDisplayName",
            "()Ljava/lang/String;",
            &[],
        )?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    pub fn as_string(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getAsString",
            "()Ljava/lang/String;",
            &[],
        )?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    pub fn has_display_name(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasDisplayName", "()Z", &[])?;
        Ok(res.z().unwrap())
    }
    pub fn set_display_name(&mut self, arg0: String) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JObject::from(self.jni_ref().new_string(arg0).unwrap());
        self.jni_ref().call_method(
            &self.jni_object(),
            "setDisplayName",
            "(Ljava/lang/String;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn has_localized_name(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasLocalizedName", "()Z", &[])?;
        Ok(res.z().unwrap())
    }
    pub fn localized_name(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getLocalizedName",
            "()Ljava/lang/String;",
            &[],
        )?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    pub fn set_localized_name(&mut self, arg0: String) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JObject::from(self.jni_ref().new_string(arg0).unwrap());
        self.jni_ref().call_method(
            &self.jni_object(),
            "setLocalizedName",
            "(Ljava/lang/String;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn has_lore(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasLore", "()Z", &[])?;
        Ok(res.z().unwrap())
    }
    pub fn set_lore(&mut self, arg0: Vec<String>) -> Result<(), Box<dyn std::error::Error>> {
        let raw_val_0 = self
            .jni_ref()
            .new_object("java/util/ArrayList", "()V", &[])
            .unwrap();
        for v in arg0 {
            let map_val_0 = jni::objects::JObject::from(self.jni_ref().new_string(v).unwrap());
            self.jni_ref().call_method(
                &raw_val_0,
                "add",
                "(Ljava/Lang/Object)V",
                &[jni::objects::JValueGen::from(&map_val_0)],
            )?;
        }
        let val_0 = jni::objects::JValueGen::Object(raw_val_0);
        self.jni_ref().call_method(
            &self.jni_object(),
            "setLore",
            "(Ljava/util/List;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn has_custom_model_data(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "hasCustomModelData", "()Z", &[])?;
        Ok(res.z().unwrap())
    }
    pub fn custom_model_data(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getCustomModelData", "()I", &[])?;
        Ok(res.i().unwrap())
    }
    pub fn set_custom_model_data(&mut self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Int(arg0.into());
        self.jni_ref().call_method(
            &self.jni_object(),
            "setCustomModelData",
            "(I)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn has_enchants(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasEnchants", "()Z", &[])?;
        Ok(res.z().unwrap())
    }
    pub fn has_enchant(
        &mut self,
        arg0: impl Into<crate::bukkit::enchantments::Enchantment<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "hasEnchant",
            "(Lorg/bukkit/enchantments/Enchantment;)Z",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(res.z().unwrap())
    }
    pub fn get_enchant_level(
        &mut self,
        arg0: impl Into<crate::bukkit::enchantments::Enchantment<'mc>>,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getEnchantLevel",
            "(Lorg/bukkit/enchantments/Enchantment;)I",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(res.i().unwrap())
    }
    pub fn add_enchant(
        &mut self,
        arg0: impl Into<crate::bukkit::enchantments::Enchantment<'mc>>,
        arg1: i32,
        arg2: bool,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let val_1 = jni::objects::JValueGen::Int(arg1.into());
        // -2
        let val_2 = jni::objects::JValueGen::Bool(arg2.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "addEnchant",
            "(Lorg/bukkit/enchantments/Enchantment;IZ)Z",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        )?;
        Ok(res.z().unwrap())
    }
    pub fn remove_enchant(
        &mut self,
        arg0: impl Into<crate::bukkit::enchantments::Enchantment<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeEnchant",
            "(Lorg/bukkit/enchantments/Enchantment;)Z",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(res.z().unwrap())
    }
    pub fn has_conflicting_enchant(
        &mut self,
        arg0: impl Into<crate::bukkit::enchantments::Enchantment<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "hasConflictingEnchant",
            "(Lorg/bukkit/enchantments/Enchantment;)Z",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(res.z().unwrap())
    }
    pub fn add_item_flags(
        &mut self,
        _arg0: Vec<impl Into<crate::bukkit::inventory::ItemFlag<'mc>>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.jni_ref().call_method(
            &self.jni_object(),
            "addItemFlags",
            "(Lorg/bukkit/inventory/ItemFlag;)V",
            &[],
        )?;
        Ok(())
    }
    pub fn remove_item_flags(
        &mut self,
        _arg0: Vec<impl Into<crate::bukkit::inventory::ItemFlag<'mc>>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.jni_ref().call_method(
            &self.jni_object(),
            "removeItemFlags",
            "(Lorg/bukkit/inventory/ItemFlag;)V",
            &[],
        )?;
        Ok(())
    }
    pub fn has_item_flag(
        &mut self,
        arg0: impl Into<crate::bukkit::inventory::ItemFlag<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "hasItemFlag",
            "(Lorg/bukkit/inventory/ItemFlag;)Z",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(res.z().unwrap())
    }
    pub fn is_unbreakable(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isUnbreakable", "()Z", &[])?;
        Ok(res.z().unwrap())
    }
    pub fn set_unbreakable(&mut self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        // -2
        let val_0 = jni::objects::JValueGen::Bool(arg0.into());
        self.jni_ref().call_method(
            &self.jni_object(),
            "setUnbreakable",
            "(Z)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn has_attribute_modifiers(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "hasAttributeModifiers", "()Z", &[])?;
        Ok(res.z().unwrap())
    }
    pub fn add_attribute_modifier(
        &mut self,
        arg0: impl Into<crate::bukkit::attribute::Attribute<'mc>>,
        arg1: impl Into<crate::bukkit::attribute::AttributeModifier<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg1.into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "addAttributeModifier",
            "(Lorg/bukkit/attribute/Attribute;Lorg/bukkit/attribute/AttributeModifier;)Z",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        )?;
        Ok(res.z().unwrap())
    }
    pub fn remove_attribute_modifier_with_attribute(
        &mut self,
        arg0: impl Into<crate::bukkit::attribute::Attribute<'mc>>,
        arg1: std::option::Option<impl Into<crate::bukkit::attribute::AttributeModifier<'mc>>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg1.unwrap().into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeAttributeModifier",
            "(Lorg/bukkit/attribute/Attribute;Lorg/bukkit/attribute/AttributeModifier;)Z",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        )?;
        Ok(res.z().unwrap())
    }
    #[deprecated]
    pub fn custom_tag_container(
        &mut self,
    ) -> Result<
        crate::bukkit::inventory::meta::tags::CustomItemTagContainer<'mc>,
        Box<dyn std::error::Error>,
    > {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getCustomTagContainer",
            "()Lorg/bukkit/inventory/meta/tags/CustomItemTagContainer;",
            &[],
        )?;
        let ret = {
            crate::bukkit::inventory::meta::tags::CustomItemTagContainer(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    #[deprecated]
    pub fn set_version(&mut self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Int(arg0.into());
        self.jni_ref().call_method(
            &self.jni_object(),
            "setVersion",
            "(I)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn clone(&mut self) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "clone", "()Ljava/lang/Object;", &[])?;
        Ok(res.l().unwrap())
    }
    pub fn persistent_data_container(
        &mut self,
    ) -> Result<crate::bukkit::persistence::PersistentDataContainer<'mc>, Box<dyn std::error::Error>>
    {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getPersistentDataContainer",
            "()Lorg/bukkit/persistence/PersistentDataContainer;",
            &[],
        )?;
        let ret = {
            crate::bukkit::persistence::PersistentDataContainer(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
}
impl<'mc> crate::JNIRaw<'mc> for ItemMeta<'mc> {
    fn jni_ref(&self) -> crate::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Into<crate::bukkit::configuration::serialization::ConfigurationSerializable<'mc>>
    for ItemMeta<'mc>
{
    fn into(self) -> crate::bukkit::configuration::serialization::ConfigurationSerializable<'mc> {
        crate::bukkit::configuration::serialization::ConfigurationSerializable::from_raw(
            &self.jni_ref(),
            self.1,
        )
        .unwrap()
    }
}
impl<'mc> Into<crate::bukkit::persistence::PersistentDataHolder<'mc>> for ItemMeta<'mc> {
    fn into(self) -> crate::bukkit::persistence::PersistentDataHolder<'mc> {
        crate::bukkit::persistence::PersistentDataHolder::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
/// An instantiatable struct that implements Repairable. Needed for returning it from Java.
pub struct Repairable<'mc>(
    pub(crate) crate::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> Repairable<'mc> {
    pub fn from_raw(
        env: &crate::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Repairable from null object.").into());
        }
        let cls = env.jni.borrow().get_object_class(&obj)?;
        let name_raw = env.call_method(cls, "getName", "()Ljava/lang/String;", &[])?;
        let oh = name_raw.l()?.into();
        let what = env.get_string(&oh)?;
        let name = what.to_string_lossy();
        if !name.ends_with("Repairable") {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Repairable object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    pub fn repair_cost(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getRepairCost", "()I", &[])?;
        Ok(res.i().unwrap())
    }
    pub fn set_repair_cost(&mut self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Int(arg0.into());
        self.jni_ref().call_method(
            &self.jni_object(),
            "setRepairCost",
            "(I)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn has_repair_cost(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasRepairCost", "()Z", &[])?;
        Ok(res.z().unwrap())
    }
    pub fn clone(
        &mut self,
    ) -> Result<crate::bukkit::inventory::meta::Repairable<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "clone",
            "()Lorg/bukkit/inventory/meta/Repairable;",
            &[],
        )?;
        let ret = {
            crate::bukkit::inventory::meta::Repairable(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn display_name(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getDisplayName",
            "()Ljava/lang/String;",
            &[],
        )?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    pub fn as_string(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getAsString",
            "()Ljava/lang/String;",
            &[],
        )?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    pub fn has_display_name(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasDisplayName", "()Z", &[])?;
        Ok(res.z().unwrap())
    }
    pub fn set_display_name(&mut self, arg0: String) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JObject::from(self.jni_ref().new_string(arg0).unwrap());
        self.jni_ref().call_method(
            &self.jni_object(),
            "setDisplayName",
            "(Ljava/lang/String;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn has_localized_name(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasLocalizedName", "()Z", &[])?;
        Ok(res.z().unwrap())
    }
    pub fn localized_name(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getLocalizedName",
            "()Ljava/lang/String;",
            &[],
        )?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    pub fn set_localized_name(&mut self, arg0: String) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JObject::from(self.jni_ref().new_string(arg0).unwrap());
        self.jni_ref().call_method(
            &self.jni_object(),
            "setLocalizedName",
            "(Ljava/lang/String;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn has_lore(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasLore", "()Z", &[])?;
        Ok(res.z().unwrap())
    }
    pub fn set_lore(&mut self, arg0: Vec<String>) -> Result<(), Box<dyn std::error::Error>> {
        let raw_val_0 = self
            .jni_ref()
            .new_object("java/util/ArrayList", "()V", &[])
            .unwrap();
        for v in arg0 {
            let map_val_0 = jni::objects::JObject::from(self.jni_ref().new_string(v).unwrap());
            self.jni_ref().call_method(
                &raw_val_0,
                "add",
                "(Ljava/Lang/Object)V",
                &[jni::objects::JValueGen::from(&map_val_0)],
            )?;
        }
        let val_0 = jni::objects::JValueGen::Object(raw_val_0);
        self.jni_ref().call_method(
            &self.jni_object(),
            "setLore",
            "(Ljava/util/List;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn has_custom_model_data(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "hasCustomModelData", "()Z", &[])?;
        Ok(res.z().unwrap())
    }
    pub fn custom_model_data(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getCustomModelData", "()I", &[])?;
        Ok(res.i().unwrap())
    }
    pub fn set_custom_model_data(&mut self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Int(arg0.into());
        self.jni_ref().call_method(
            &self.jni_object(),
            "setCustomModelData",
            "(I)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn has_enchants(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasEnchants", "()Z", &[])?;
        Ok(res.z().unwrap())
    }
    pub fn has_enchant(
        &mut self,
        arg0: impl Into<crate::bukkit::enchantments::Enchantment<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "hasEnchant",
            "(Lorg/bukkit/enchantments/Enchantment;)Z",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(res.z().unwrap())
    }
    pub fn get_enchant_level(
        &mut self,
        arg0: impl Into<crate::bukkit::enchantments::Enchantment<'mc>>,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getEnchantLevel",
            "(Lorg/bukkit/enchantments/Enchantment;)I",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(res.i().unwrap())
    }
    pub fn add_enchant(
        &mut self,
        arg0: impl Into<crate::bukkit::enchantments::Enchantment<'mc>>,
        arg1: i32,
        arg2: bool,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let val_1 = jni::objects::JValueGen::Int(arg1.into());
        // -2
        let val_2 = jni::objects::JValueGen::Bool(arg2.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "addEnchant",
            "(Lorg/bukkit/enchantments/Enchantment;IZ)Z",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        )?;
        Ok(res.z().unwrap())
    }
    pub fn remove_enchant(
        &mut self,
        arg0: impl Into<crate::bukkit::enchantments::Enchantment<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeEnchant",
            "(Lorg/bukkit/enchantments/Enchantment;)Z",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(res.z().unwrap())
    }
    pub fn has_conflicting_enchant(
        &mut self,
        arg0: impl Into<crate::bukkit::enchantments::Enchantment<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "hasConflictingEnchant",
            "(Lorg/bukkit/enchantments/Enchantment;)Z",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(res.z().unwrap())
    }
    pub fn add_item_flags(
        &mut self,
        _arg0: Vec<impl Into<crate::bukkit::inventory::ItemFlag<'mc>>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.jni_ref().call_method(
            &self.jni_object(),
            "addItemFlags",
            "(Lorg/bukkit/inventory/ItemFlag;)V",
            &[],
        )?;
        Ok(())
    }
    pub fn remove_item_flags(
        &mut self,
        _arg0: Vec<impl Into<crate::bukkit::inventory::ItemFlag<'mc>>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.jni_ref().call_method(
            &self.jni_object(),
            "removeItemFlags",
            "(Lorg/bukkit/inventory/ItemFlag;)V",
            &[],
        )?;
        Ok(())
    }
    pub fn has_item_flag(
        &mut self,
        arg0: impl Into<crate::bukkit::inventory::ItemFlag<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "hasItemFlag",
            "(Lorg/bukkit/inventory/ItemFlag;)Z",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(res.z().unwrap())
    }
    pub fn is_unbreakable(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isUnbreakable", "()Z", &[])?;
        Ok(res.z().unwrap())
    }
    pub fn set_unbreakable(&mut self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        // -2
        let val_0 = jni::objects::JValueGen::Bool(arg0.into());
        self.jni_ref().call_method(
            &self.jni_object(),
            "setUnbreakable",
            "(Z)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn has_attribute_modifiers(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "hasAttributeModifiers", "()Z", &[])?;
        Ok(res.z().unwrap())
    }
    pub fn add_attribute_modifier(
        &mut self,
        arg0: impl Into<crate::bukkit::attribute::Attribute<'mc>>,
        arg1: impl Into<crate::bukkit::attribute::AttributeModifier<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg1.into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "addAttributeModifier",
            "(Lorg/bukkit/attribute/Attribute;Lorg/bukkit/attribute/AttributeModifier;)Z",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        )?;
        Ok(res.z().unwrap())
    }
    pub fn remove_attribute_modifier_with_attribute(
        &mut self,
        arg0: impl Into<crate::bukkit::attribute::Attribute<'mc>>,
        arg1: std::option::Option<impl Into<crate::bukkit::attribute::AttributeModifier<'mc>>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg1.unwrap().into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeAttributeModifier",
            "(Lorg/bukkit/attribute/Attribute;Lorg/bukkit/attribute/AttributeModifier;)Z",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        )?;
        Ok(res.z().unwrap())
    }
    #[deprecated]
    pub fn custom_tag_container(
        &mut self,
    ) -> Result<
        crate::bukkit::inventory::meta::tags::CustomItemTagContainer<'mc>,
        Box<dyn std::error::Error>,
    > {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getCustomTagContainer",
            "()Lorg/bukkit/inventory/meta/tags/CustomItemTagContainer;",
            &[],
        )?;
        let ret = {
            crate::bukkit::inventory::meta::tags::CustomItemTagContainer(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    #[deprecated]
    pub fn set_version(&mut self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Int(arg0.into());
        self.jni_ref().call_method(
            &self.jni_object(),
            "setVersion",
            "(I)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn persistent_data_container(
        &mut self,
    ) -> Result<crate::bukkit::persistence::PersistentDataContainer<'mc>, Box<dyn std::error::Error>>
    {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getPersistentDataContainer",
            "()Lorg/bukkit/persistence/PersistentDataContainer;",
            &[],
        )?;
        let ret = {
            crate::bukkit::persistence::PersistentDataContainer(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
}
impl<'mc> crate::JNIRaw<'mc> for Repairable<'mc> {
    fn jni_ref(&self) -> crate::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Into<crate::bukkit::inventory::meta::ItemMeta<'mc>> for Repairable<'mc> {
    fn into(self) -> crate::bukkit::inventory::meta::ItemMeta<'mc> {
        crate::bukkit::inventory::meta::ItemMeta::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
/// An instantiatable struct that implements BannerMeta. Needed for returning it from Java.
pub struct BannerMeta<'mc>(
    pub(crate) crate::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> BannerMeta<'mc> {
    pub fn from_raw(
        env: &crate::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate BannerMeta from null object.").into());
        }
        let cls = env.jni.borrow().get_object_class(&obj)?;
        let name_raw = env.call_method(cls, "getName", "()Ljava/lang/String;", &[])?;
        let oh = name_raw.l()?.into();
        let what = env.get_string(&oh)?;
        let name = what.to_string_lossy();
        if !name.ends_with("BannerMeta") {
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
    pub fn base_color(
        &mut self,
    ) -> Result<crate::bukkit::DyeColor<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getBaseColor",
            "()Lorg/bukkit/DyeColor;",
            &[],
        )?;
        let ret = {
            let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
            let variant =
                self.jni_ref()
                    .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
            let variant_str = self
                .jni_ref()
                .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
                .to_string_lossy()
                .to_string();
            crate::bukkit::DyeColor(
                self.jni_ref(),
                raw_obj,
                crate::bukkit::DyeColor::from_string(variant_str).unwrap(),
            )
        };
        Ok(ret)
    }
    #[deprecated]
    pub fn set_base_color(
        &mut self,
        arg0: impl Into<crate::bukkit::DyeColor<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        self.jni_ref().call_method(
            &self.jni_object(),
            "setBaseColor",
            "(Lorg/bukkit/DyeColor;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn set_patterns(
        &mut self,
        arg0: Vec<impl Into<crate::bukkit::block::banner::Pattern<'mc>>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let raw_val_0 = self
            .jni_ref()
            .new_object("java/util/ArrayList", "()V", &[])
            .unwrap();
        for v in arg0 {
            let map_val_0 = unsafe { jni::objects::JObject::from_raw(v.into().1.clone()) };
            self.jni_ref().call_method(
                &raw_val_0,
                "add",
                "(Ljava/Lang/Object)V",
                &[jni::objects::JValueGen::from(&map_val_0)],
            )?;
        }
        let val_0 = jni::objects::JValueGen::Object(raw_val_0);
        self.jni_ref().call_method(
            &self.jni_object(),
            "setPatterns",
            "(Ljava/util/List;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn add_pattern(
        &mut self,
        arg0: impl Into<crate::bukkit::block::banner::Pattern<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        self.jni_ref().call_method(
            &self.jni_object(),
            "addPattern",
            "(Lorg/bukkit/block/banner/Pattern;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn get_pattern(
        &mut self,
        arg0: i32,
    ) -> Result<crate::bukkit::block::banner::Pattern<'mc>, Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Int(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getPattern",
            "(I)Lorg/bukkit/block/banner/Pattern;",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        let ret = {
            crate::bukkit::block::banner::Pattern(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn remove_pattern(
        &mut self,
        arg0: i32,
    ) -> Result<crate::bukkit::block::banner::Pattern<'mc>, Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Int(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removePattern",
            "(I)Lorg/bukkit/block/banner/Pattern;",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        let ret = {
            crate::bukkit::block::banner::Pattern(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn set_pattern(
        &mut self,
        arg0: i32,
        arg1: impl Into<crate::bukkit::block::banner::Pattern<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Int(arg0.into());
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg1.into().1.clone()) };
        self.jni_ref().call_method(
            &self.jni_object(),
            "setPattern",
            "(ILorg/bukkit/block/banner/Pattern;)V",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        )?;
        Ok(())
    }
    pub fn number_of_patterns(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "numberOfPatterns", "()I", &[])?;
        Ok(res.i().unwrap())
    }
    pub fn display_name(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getDisplayName",
            "()Ljava/lang/String;",
            &[],
        )?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    pub fn as_string(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getAsString",
            "()Ljava/lang/String;",
            &[],
        )?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    pub fn has_display_name(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasDisplayName", "()Z", &[])?;
        Ok(res.z().unwrap())
    }
    pub fn set_display_name(&mut self, arg0: String) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JObject::from(self.jni_ref().new_string(arg0).unwrap());
        self.jni_ref().call_method(
            &self.jni_object(),
            "setDisplayName",
            "(Ljava/lang/String;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn has_localized_name(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasLocalizedName", "()Z", &[])?;
        Ok(res.z().unwrap())
    }
    pub fn localized_name(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getLocalizedName",
            "()Ljava/lang/String;",
            &[],
        )?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    pub fn set_localized_name(&mut self, arg0: String) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JObject::from(self.jni_ref().new_string(arg0).unwrap());
        self.jni_ref().call_method(
            &self.jni_object(),
            "setLocalizedName",
            "(Ljava/lang/String;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn has_lore(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasLore", "()Z", &[])?;
        Ok(res.z().unwrap())
    }
    pub fn set_lore(&mut self, arg0: Vec<String>) -> Result<(), Box<dyn std::error::Error>> {
        let raw_val_0 = self
            .jni_ref()
            .new_object("java/util/ArrayList", "()V", &[])
            .unwrap();
        for v in arg0 {
            let map_val_0 = jni::objects::JObject::from(self.jni_ref().new_string(v).unwrap());
            self.jni_ref().call_method(
                &raw_val_0,
                "add",
                "(Ljava/Lang/Object)V",
                &[jni::objects::JValueGen::from(&map_val_0)],
            )?;
        }
        let val_0 = jni::objects::JValueGen::Object(raw_val_0);
        self.jni_ref().call_method(
            &self.jni_object(),
            "setLore",
            "(Ljava/util/List;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn has_custom_model_data(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "hasCustomModelData", "()Z", &[])?;
        Ok(res.z().unwrap())
    }
    pub fn custom_model_data(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getCustomModelData", "()I", &[])?;
        Ok(res.i().unwrap())
    }
    pub fn set_custom_model_data(&mut self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Int(arg0.into());
        self.jni_ref().call_method(
            &self.jni_object(),
            "setCustomModelData",
            "(I)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn has_enchants(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasEnchants", "()Z", &[])?;
        Ok(res.z().unwrap())
    }
    pub fn has_enchant(
        &mut self,
        arg0: impl Into<crate::bukkit::enchantments::Enchantment<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "hasEnchant",
            "(Lorg/bukkit/enchantments/Enchantment;)Z",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(res.z().unwrap())
    }
    pub fn get_enchant_level(
        &mut self,
        arg0: impl Into<crate::bukkit::enchantments::Enchantment<'mc>>,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getEnchantLevel",
            "(Lorg/bukkit/enchantments/Enchantment;)I",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(res.i().unwrap())
    }
    pub fn add_enchant(
        &mut self,
        arg0: impl Into<crate::bukkit::enchantments::Enchantment<'mc>>,
        arg1: i32,
        arg2: bool,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let val_1 = jni::objects::JValueGen::Int(arg1.into());
        // -2
        let val_2 = jni::objects::JValueGen::Bool(arg2.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "addEnchant",
            "(Lorg/bukkit/enchantments/Enchantment;IZ)Z",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        )?;
        Ok(res.z().unwrap())
    }
    pub fn remove_enchant(
        &mut self,
        arg0: impl Into<crate::bukkit::enchantments::Enchantment<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeEnchant",
            "(Lorg/bukkit/enchantments/Enchantment;)Z",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(res.z().unwrap())
    }
    pub fn has_conflicting_enchant(
        &mut self,
        arg0: impl Into<crate::bukkit::enchantments::Enchantment<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "hasConflictingEnchant",
            "(Lorg/bukkit/enchantments/Enchantment;)Z",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(res.z().unwrap())
    }
    pub fn add_item_flags(
        &mut self,
        _arg0: Vec<impl Into<crate::bukkit::inventory::ItemFlag<'mc>>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.jni_ref().call_method(
            &self.jni_object(),
            "addItemFlags",
            "(Lorg/bukkit/inventory/ItemFlag;)V",
            &[],
        )?;
        Ok(())
    }
    pub fn remove_item_flags(
        &mut self,
        _arg0: Vec<impl Into<crate::bukkit::inventory::ItemFlag<'mc>>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.jni_ref().call_method(
            &self.jni_object(),
            "removeItemFlags",
            "(Lorg/bukkit/inventory/ItemFlag;)V",
            &[],
        )?;
        Ok(())
    }
    pub fn has_item_flag(
        &mut self,
        arg0: impl Into<crate::bukkit::inventory::ItemFlag<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "hasItemFlag",
            "(Lorg/bukkit/inventory/ItemFlag;)Z",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(res.z().unwrap())
    }
    pub fn is_unbreakable(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isUnbreakable", "()Z", &[])?;
        Ok(res.z().unwrap())
    }
    pub fn set_unbreakable(&mut self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        // -2
        let val_0 = jni::objects::JValueGen::Bool(arg0.into());
        self.jni_ref().call_method(
            &self.jni_object(),
            "setUnbreakable",
            "(Z)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn has_attribute_modifiers(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "hasAttributeModifiers", "()Z", &[])?;
        Ok(res.z().unwrap())
    }
    pub fn add_attribute_modifier(
        &mut self,
        arg0: impl Into<crate::bukkit::attribute::Attribute<'mc>>,
        arg1: impl Into<crate::bukkit::attribute::AttributeModifier<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg1.into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "addAttributeModifier",
            "(Lorg/bukkit/attribute/Attribute;Lorg/bukkit/attribute/AttributeModifier;)Z",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        )?;
        Ok(res.z().unwrap())
    }
    pub fn remove_attribute_modifier_with_attribute(
        &mut self,
        arg0: impl Into<crate::bukkit::attribute::Attribute<'mc>>,
        arg1: std::option::Option<impl Into<crate::bukkit::attribute::AttributeModifier<'mc>>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg1.unwrap().into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeAttributeModifier",
            "(Lorg/bukkit/attribute/Attribute;Lorg/bukkit/attribute/AttributeModifier;)Z",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        )?;
        Ok(res.z().unwrap())
    }
    #[deprecated]
    pub fn custom_tag_container(
        &mut self,
    ) -> Result<
        crate::bukkit::inventory::meta::tags::CustomItemTagContainer<'mc>,
        Box<dyn std::error::Error>,
    > {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getCustomTagContainer",
            "()Lorg/bukkit/inventory/meta/tags/CustomItemTagContainer;",
            &[],
        )?;
        let ret = {
            crate::bukkit::inventory::meta::tags::CustomItemTagContainer(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    #[deprecated]
    pub fn set_version(&mut self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Int(arg0.into());
        self.jni_ref().call_method(
            &self.jni_object(),
            "setVersion",
            "(I)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn clone(&mut self) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "clone", "()Ljava/lang/Object;", &[])?;
        Ok(res.l().unwrap())
    }
    pub fn persistent_data_container(
        &mut self,
    ) -> Result<crate::bukkit::persistence::PersistentDataContainer<'mc>, Box<dyn std::error::Error>>
    {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getPersistentDataContainer",
            "()Lorg/bukkit/persistence/PersistentDataContainer;",
            &[],
        )?;
        let ret = {
            crate::bukkit::persistence::PersistentDataContainer(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
}
impl<'mc> crate::JNIRaw<'mc> for BannerMeta<'mc> {
    fn jni_ref(&self) -> crate::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Into<crate::bukkit::inventory::meta::ItemMeta<'mc>> for BannerMeta<'mc> {
    fn into(self) -> crate::bukkit::inventory::meta::ItemMeta<'mc> {
        crate::bukkit::inventory::meta::ItemMeta::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
/// An instantiatable struct that implements BundleMeta. Needed for returning it from Java.
pub struct BundleMeta<'mc>(
    pub(crate) crate::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> BundleMeta<'mc> {
    pub fn from_raw(
        env: &crate::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate BundleMeta from null object.").into());
        }
        let cls = env.jni.borrow().get_object_class(&obj)?;
        let name_raw = env.call_method(cls, "getName", "()Ljava/lang/String;", &[])?;
        let oh = name_raw.l()?.into();
        let what = env.get_string(&oh)?;
        let name = what.to_string_lossy();
        if !name.ends_with("BundleMeta") {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a BundleMeta object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    pub fn add_item(
        &mut self,
        arg0: impl Into<crate::bukkit::inventory::ItemStack<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        self.jni_ref().call_method(
            &self.jni_object(),
            "addItem",
            "(Lorg/bukkit/inventory/ItemStack;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn has_items(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasItems", "()Z", &[])?;
        Ok(res.z().unwrap())
    }
    pub fn set_items(
        &mut self,
        arg0: Vec<impl Into<crate::bukkit::inventory::ItemStack<'mc>>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let raw_val_0 = self
            .jni_ref()
            .new_object("java/util/ArrayList", "()V", &[])
            .unwrap();
        for v in arg0 {
            let map_val_0 = unsafe { jni::objects::JObject::from_raw(v.into().1.clone()) };
            self.jni_ref().call_method(
                &raw_val_0,
                "add",
                "(Ljava/Lang/Object)V",
                &[jni::objects::JValueGen::from(&map_val_0)],
            )?;
        }
        let val_0 = jni::objects::JValueGen::Object(raw_val_0);
        self.jni_ref().call_method(
            &self.jni_object(),
            "setItems",
            "(Ljava/util/List;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn display_name(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getDisplayName",
            "()Ljava/lang/String;",
            &[],
        )?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    pub fn as_string(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getAsString",
            "()Ljava/lang/String;",
            &[],
        )?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    pub fn has_display_name(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasDisplayName", "()Z", &[])?;
        Ok(res.z().unwrap())
    }
    pub fn set_display_name(&mut self, arg0: String) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JObject::from(self.jni_ref().new_string(arg0).unwrap());
        self.jni_ref().call_method(
            &self.jni_object(),
            "setDisplayName",
            "(Ljava/lang/String;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn has_localized_name(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasLocalizedName", "()Z", &[])?;
        Ok(res.z().unwrap())
    }
    pub fn localized_name(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getLocalizedName",
            "()Ljava/lang/String;",
            &[],
        )?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    pub fn set_localized_name(&mut self, arg0: String) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JObject::from(self.jni_ref().new_string(arg0).unwrap());
        self.jni_ref().call_method(
            &self.jni_object(),
            "setLocalizedName",
            "(Ljava/lang/String;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn has_lore(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasLore", "()Z", &[])?;
        Ok(res.z().unwrap())
    }
    pub fn set_lore(&mut self, arg0: Vec<String>) -> Result<(), Box<dyn std::error::Error>> {
        let raw_val_0 = self
            .jni_ref()
            .new_object("java/util/ArrayList", "()V", &[])
            .unwrap();
        for v in arg0 {
            let map_val_0 = jni::objects::JObject::from(self.jni_ref().new_string(v).unwrap());
            self.jni_ref().call_method(
                &raw_val_0,
                "add",
                "(Ljava/Lang/Object)V",
                &[jni::objects::JValueGen::from(&map_val_0)],
            )?;
        }
        let val_0 = jni::objects::JValueGen::Object(raw_val_0);
        self.jni_ref().call_method(
            &self.jni_object(),
            "setLore",
            "(Ljava/util/List;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn has_custom_model_data(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "hasCustomModelData", "()Z", &[])?;
        Ok(res.z().unwrap())
    }
    pub fn custom_model_data(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getCustomModelData", "()I", &[])?;
        Ok(res.i().unwrap())
    }
    pub fn set_custom_model_data(&mut self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Int(arg0.into());
        self.jni_ref().call_method(
            &self.jni_object(),
            "setCustomModelData",
            "(I)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn has_enchants(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasEnchants", "()Z", &[])?;
        Ok(res.z().unwrap())
    }
    pub fn has_enchant(
        &mut self,
        arg0: impl Into<crate::bukkit::enchantments::Enchantment<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "hasEnchant",
            "(Lorg/bukkit/enchantments/Enchantment;)Z",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(res.z().unwrap())
    }
    pub fn get_enchant_level(
        &mut self,
        arg0: impl Into<crate::bukkit::enchantments::Enchantment<'mc>>,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getEnchantLevel",
            "(Lorg/bukkit/enchantments/Enchantment;)I",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(res.i().unwrap())
    }
    pub fn add_enchant(
        &mut self,
        arg0: impl Into<crate::bukkit::enchantments::Enchantment<'mc>>,
        arg1: i32,
        arg2: bool,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let val_1 = jni::objects::JValueGen::Int(arg1.into());
        // -2
        let val_2 = jni::objects::JValueGen::Bool(arg2.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "addEnchant",
            "(Lorg/bukkit/enchantments/Enchantment;IZ)Z",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        )?;
        Ok(res.z().unwrap())
    }
    pub fn remove_enchant(
        &mut self,
        arg0: impl Into<crate::bukkit::enchantments::Enchantment<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeEnchant",
            "(Lorg/bukkit/enchantments/Enchantment;)Z",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(res.z().unwrap())
    }
    pub fn has_conflicting_enchant(
        &mut self,
        arg0: impl Into<crate::bukkit::enchantments::Enchantment<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "hasConflictingEnchant",
            "(Lorg/bukkit/enchantments/Enchantment;)Z",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(res.z().unwrap())
    }
    pub fn add_item_flags(
        &mut self,
        _arg0: Vec<impl Into<crate::bukkit::inventory::ItemFlag<'mc>>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.jni_ref().call_method(
            &self.jni_object(),
            "addItemFlags",
            "(Lorg/bukkit/inventory/ItemFlag;)V",
            &[],
        )?;
        Ok(())
    }
    pub fn remove_item_flags(
        &mut self,
        _arg0: Vec<impl Into<crate::bukkit::inventory::ItemFlag<'mc>>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.jni_ref().call_method(
            &self.jni_object(),
            "removeItemFlags",
            "(Lorg/bukkit/inventory/ItemFlag;)V",
            &[],
        )?;
        Ok(())
    }
    pub fn has_item_flag(
        &mut self,
        arg0: impl Into<crate::bukkit::inventory::ItemFlag<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "hasItemFlag",
            "(Lorg/bukkit/inventory/ItemFlag;)Z",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(res.z().unwrap())
    }
    pub fn is_unbreakable(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isUnbreakable", "()Z", &[])?;
        Ok(res.z().unwrap())
    }
    pub fn set_unbreakable(&mut self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        // -2
        let val_0 = jni::objects::JValueGen::Bool(arg0.into());
        self.jni_ref().call_method(
            &self.jni_object(),
            "setUnbreakable",
            "(Z)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn has_attribute_modifiers(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "hasAttributeModifiers", "()Z", &[])?;
        Ok(res.z().unwrap())
    }
    pub fn add_attribute_modifier(
        &mut self,
        arg0: impl Into<crate::bukkit::attribute::Attribute<'mc>>,
        arg1: impl Into<crate::bukkit::attribute::AttributeModifier<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg1.into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "addAttributeModifier",
            "(Lorg/bukkit/attribute/Attribute;Lorg/bukkit/attribute/AttributeModifier;)Z",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        )?;
        Ok(res.z().unwrap())
    }
    pub fn remove_attribute_modifier_with_attribute(
        &mut self,
        arg0: impl Into<crate::bukkit::attribute::Attribute<'mc>>,
        arg1: std::option::Option<impl Into<crate::bukkit::attribute::AttributeModifier<'mc>>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg1.unwrap().into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeAttributeModifier",
            "(Lorg/bukkit/attribute/Attribute;Lorg/bukkit/attribute/AttributeModifier;)Z",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        )?;
        Ok(res.z().unwrap())
    }
    #[deprecated]
    pub fn custom_tag_container(
        &mut self,
    ) -> Result<
        crate::bukkit::inventory::meta::tags::CustomItemTagContainer<'mc>,
        Box<dyn std::error::Error>,
    > {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getCustomTagContainer",
            "()Lorg/bukkit/inventory/meta/tags/CustomItemTagContainer;",
            &[],
        )?;
        let ret = {
            crate::bukkit::inventory::meta::tags::CustomItemTagContainer(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    #[deprecated]
    pub fn set_version(&mut self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Int(arg0.into());
        self.jni_ref().call_method(
            &self.jni_object(),
            "setVersion",
            "(I)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn clone(&mut self) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "clone", "()Ljava/lang/Object;", &[])?;
        Ok(res.l().unwrap())
    }
    pub fn persistent_data_container(
        &mut self,
    ) -> Result<crate::bukkit::persistence::PersistentDataContainer<'mc>, Box<dyn std::error::Error>>
    {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getPersistentDataContainer",
            "()Lorg/bukkit/persistence/PersistentDataContainer;",
            &[],
        )?;
        let ret = {
            crate::bukkit::persistence::PersistentDataContainer(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
}
impl<'mc> crate::JNIRaw<'mc> for BundleMeta<'mc> {
    fn jni_ref(&self) -> crate::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Into<crate::bukkit::inventory::meta::ItemMeta<'mc>> for BundleMeta<'mc> {
    fn into(self) -> crate::bukkit::inventory::meta::ItemMeta<'mc> {
        crate::bukkit::inventory::meta::ItemMeta::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
/// An instantiatable struct that implements ColorableArmorMeta. Needed for returning it from Java.
pub struct ColorableArmorMeta<'mc>(
    pub(crate) crate::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> ColorableArmorMeta<'mc> {
    pub fn from_raw(
        env: &crate::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate ColorableArmorMeta from null object.").into(),
            );
        }
        let cls = env.jni.borrow().get_object_class(&obj)?;
        let name_raw = env.call_method(cls, "getName", "()Ljava/lang/String;", &[])?;
        let oh = name_raw.l()?.into();
        let what = env.get_string(&oh)?;
        let name = what.to_string_lossy();
        if !name.ends_with("ColorableArmorMeta") {
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
    ) -> Result<crate::bukkit::inventory::meta::ColorableArmorMeta<'mc>, Box<dyn std::error::Error>>
    {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "clone",
            "()Lorg/bukkit/inventory/meta/ColorableArmorMeta;",
            &[],
        )?;
        let ret = {
            crate::bukkit::inventory::meta::ColorableArmorMeta(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn has_trim(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasTrim", "()Z", &[])?;
        Ok(res.z().unwrap())
    }
    pub fn set_trim(
        &mut self,
        arg0: impl Into<crate::bukkit::inventory::meta::trim::ArmorTrim<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        self.jni_ref().call_method(
            &self.jni_object(),
            "setTrim",
            "(Lorg/bukkit/inventory/meta/trim/ArmorTrim;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn trim(
        &mut self,
    ) -> Result<crate::bukkit::inventory::meta::trim::ArmorTrim<'mc>, Box<dyn std::error::Error>>
    {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getTrim",
            "()Lorg/bukkit/inventory/meta/trim/ArmorTrim;",
            &[],
        )?;
        let ret = {
            crate::bukkit::inventory::meta::trim::ArmorTrim(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn display_name(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getDisplayName",
            "()Ljava/lang/String;",
            &[],
        )?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    pub fn as_string(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getAsString",
            "()Ljava/lang/String;",
            &[],
        )?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    pub fn has_display_name(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasDisplayName", "()Z", &[])?;
        Ok(res.z().unwrap())
    }
    pub fn set_display_name(&mut self, arg0: String) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JObject::from(self.jni_ref().new_string(arg0).unwrap());
        self.jni_ref().call_method(
            &self.jni_object(),
            "setDisplayName",
            "(Ljava/lang/String;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn has_localized_name(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasLocalizedName", "()Z", &[])?;
        Ok(res.z().unwrap())
    }
    pub fn localized_name(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getLocalizedName",
            "()Ljava/lang/String;",
            &[],
        )?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    pub fn set_localized_name(&mut self, arg0: String) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JObject::from(self.jni_ref().new_string(arg0).unwrap());
        self.jni_ref().call_method(
            &self.jni_object(),
            "setLocalizedName",
            "(Ljava/lang/String;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn has_lore(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasLore", "()Z", &[])?;
        Ok(res.z().unwrap())
    }
    pub fn set_lore(&mut self, arg0: Vec<String>) -> Result<(), Box<dyn std::error::Error>> {
        let raw_val_0 = self
            .jni_ref()
            .new_object("java/util/ArrayList", "()V", &[])
            .unwrap();
        for v in arg0 {
            let map_val_0 = jni::objects::JObject::from(self.jni_ref().new_string(v).unwrap());
            self.jni_ref().call_method(
                &raw_val_0,
                "add",
                "(Ljava/Lang/Object)V",
                &[jni::objects::JValueGen::from(&map_val_0)],
            )?;
        }
        let val_0 = jni::objects::JValueGen::Object(raw_val_0);
        self.jni_ref().call_method(
            &self.jni_object(),
            "setLore",
            "(Ljava/util/List;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn has_custom_model_data(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "hasCustomModelData", "()Z", &[])?;
        Ok(res.z().unwrap())
    }
    pub fn custom_model_data(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getCustomModelData", "()I", &[])?;
        Ok(res.i().unwrap())
    }
    pub fn set_custom_model_data(&mut self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Int(arg0.into());
        self.jni_ref().call_method(
            &self.jni_object(),
            "setCustomModelData",
            "(I)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn has_enchants(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasEnchants", "()Z", &[])?;
        Ok(res.z().unwrap())
    }
    pub fn has_enchant(
        &mut self,
        arg0: impl Into<crate::bukkit::enchantments::Enchantment<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "hasEnchant",
            "(Lorg/bukkit/enchantments/Enchantment;)Z",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(res.z().unwrap())
    }
    pub fn get_enchant_level(
        &mut self,
        arg0: impl Into<crate::bukkit::enchantments::Enchantment<'mc>>,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getEnchantLevel",
            "(Lorg/bukkit/enchantments/Enchantment;)I",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(res.i().unwrap())
    }
    pub fn add_enchant(
        &mut self,
        arg0: impl Into<crate::bukkit::enchantments::Enchantment<'mc>>,
        arg1: i32,
        arg2: bool,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let val_1 = jni::objects::JValueGen::Int(arg1.into());
        // -2
        let val_2 = jni::objects::JValueGen::Bool(arg2.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "addEnchant",
            "(Lorg/bukkit/enchantments/Enchantment;IZ)Z",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        )?;
        Ok(res.z().unwrap())
    }
    pub fn remove_enchant(
        &mut self,
        arg0: impl Into<crate::bukkit::enchantments::Enchantment<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeEnchant",
            "(Lorg/bukkit/enchantments/Enchantment;)Z",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(res.z().unwrap())
    }
    pub fn has_conflicting_enchant(
        &mut self,
        arg0: impl Into<crate::bukkit::enchantments::Enchantment<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "hasConflictingEnchant",
            "(Lorg/bukkit/enchantments/Enchantment;)Z",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(res.z().unwrap())
    }
    pub fn add_item_flags(
        &mut self,
        _arg0: Vec<impl Into<crate::bukkit::inventory::ItemFlag<'mc>>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.jni_ref().call_method(
            &self.jni_object(),
            "addItemFlags",
            "(Lorg/bukkit/inventory/ItemFlag;)V",
            &[],
        )?;
        Ok(())
    }
    pub fn remove_item_flags(
        &mut self,
        _arg0: Vec<impl Into<crate::bukkit::inventory::ItemFlag<'mc>>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.jni_ref().call_method(
            &self.jni_object(),
            "removeItemFlags",
            "(Lorg/bukkit/inventory/ItemFlag;)V",
            &[],
        )?;
        Ok(())
    }
    pub fn has_item_flag(
        &mut self,
        arg0: impl Into<crate::bukkit::inventory::ItemFlag<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "hasItemFlag",
            "(Lorg/bukkit/inventory/ItemFlag;)Z",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(res.z().unwrap())
    }
    pub fn is_unbreakable(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isUnbreakable", "()Z", &[])?;
        Ok(res.z().unwrap())
    }
    pub fn set_unbreakable(&mut self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        // -2
        let val_0 = jni::objects::JValueGen::Bool(arg0.into());
        self.jni_ref().call_method(
            &self.jni_object(),
            "setUnbreakable",
            "(Z)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn has_attribute_modifiers(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "hasAttributeModifiers", "()Z", &[])?;
        Ok(res.z().unwrap())
    }
    pub fn add_attribute_modifier(
        &mut self,
        arg0: impl Into<crate::bukkit::attribute::Attribute<'mc>>,
        arg1: impl Into<crate::bukkit::attribute::AttributeModifier<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg1.into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "addAttributeModifier",
            "(Lorg/bukkit/attribute/Attribute;Lorg/bukkit/attribute/AttributeModifier;)Z",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        )?;
        Ok(res.z().unwrap())
    }
    pub fn remove_attribute_modifier_with_attribute(
        &mut self,
        arg0: impl Into<crate::bukkit::attribute::Attribute<'mc>>,
        arg1: std::option::Option<impl Into<crate::bukkit::attribute::AttributeModifier<'mc>>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg1.unwrap().into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeAttributeModifier",
            "(Lorg/bukkit/attribute/Attribute;Lorg/bukkit/attribute/AttributeModifier;)Z",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        )?;
        Ok(res.z().unwrap())
    }
    #[deprecated]
    pub fn custom_tag_container(
        &mut self,
    ) -> Result<
        crate::bukkit::inventory::meta::tags::CustomItemTagContainer<'mc>,
        Box<dyn std::error::Error>,
    > {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getCustomTagContainer",
            "()Lorg/bukkit/inventory/meta/tags/CustomItemTagContainer;",
            &[],
        )?;
        let ret = {
            crate::bukkit::inventory::meta::tags::CustomItemTagContainer(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    #[deprecated]
    pub fn set_version(&mut self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Int(arg0.into());
        self.jni_ref().call_method(
            &self.jni_object(),
            "setVersion",
            "(I)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn persistent_data_container(
        &mut self,
    ) -> Result<crate::bukkit::persistence::PersistentDataContainer<'mc>, Box<dyn std::error::Error>>
    {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getPersistentDataContainer",
            "()Lorg/bukkit/persistence/PersistentDataContainer;",
            &[],
        )?;
        let ret = {
            crate::bukkit::persistence::PersistentDataContainer(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn set_color(
        &mut self,
        arg0: impl Into<crate::bukkit::Color<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        self.jni_ref().call_method(
            &self.jni_object(),
            "setColor",
            "(Lorg/bukkit/Color;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn color(&mut self) -> Result<crate::bukkit::Color<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getColor",
            "()Lorg/bukkit/Color;",
            &[],
        )?;
        let ret = {
            crate::bukkit::Color(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
}
impl<'mc> crate::JNIRaw<'mc> for ColorableArmorMeta<'mc> {
    fn jni_ref(&self) -> crate::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Into<crate::bukkit::inventory::meta::ArmorMeta<'mc>> for ColorableArmorMeta<'mc> {
    fn into(self) -> crate::bukkit::inventory::meta::ArmorMeta<'mc> {
        crate::bukkit::inventory::meta::ArmorMeta::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
impl<'mc> Into<crate::bukkit::inventory::meta::LeatherArmorMeta<'mc>> for ColorableArmorMeta<'mc> {
    fn into(self) -> crate::bukkit::inventory::meta::LeatherArmorMeta<'mc> {
        crate::bukkit::inventory::meta::LeatherArmorMeta::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
/// An instantiatable struct that implements SpawnEggMeta. Needed for returning it from Java.
pub struct SpawnEggMeta<'mc>(
    pub(crate) crate::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> SpawnEggMeta<'mc> {
    pub fn from_raw(
        env: &crate::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate SpawnEggMeta from null object.").into());
        }
        let cls = env.jni.borrow().get_object_class(&obj)?;
        let name_raw = env.call_method(cls, "getName", "()Ljava/lang/String;", &[])?;
        let oh = name_raw.l()?.into();
        let what = env.get_string(&oh)?;
        let name = what.to_string_lossy();
        if !name.ends_with("SpawnEggMeta") {
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
    pub fn spawned_type(
        &mut self,
    ) -> Result<crate::bukkit::entity::EntityType<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getSpawnedType",
            "()Lorg/bukkit/entity/EntityType;",
            &[],
        )?;
        let ret = {
            let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
            let variant =
                self.jni_ref()
                    .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
            let variant_str = self
                .jni_ref()
                .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
                .to_string_lossy()
                .to_string();
            crate::bukkit::entity::EntityType(
                self.jni_ref(),
                raw_obj,
                crate::bukkit::entity::EntityType::from_string(variant_str).unwrap(),
            )
        };
        Ok(ret)
    }
    #[deprecated]
    pub fn set_spawned_type(
        &mut self,
        arg0: impl Into<crate::bukkit::entity::EntityType<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        self.jni_ref().call_method(
            &self.jni_object(),
            "setSpawnedType",
            "(Lorg/bukkit/entity/EntityType;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn clone(
        &mut self,
    ) -> Result<crate::bukkit::inventory::meta::SpawnEggMeta<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "clone",
            "()Lorg/bukkit/inventory/meta/SpawnEggMeta;",
            &[],
        )?;
        let ret = {
            crate::bukkit::inventory::meta::SpawnEggMeta(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn display_name(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getDisplayName",
            "()Ljava/lang/String;",
            &[],
        )?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    pub fn as_string(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getAsString",
            "()Ljava/lang/String;",
            &[],
        )?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    pub fn has_display_name(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasDisplayName", "()Z", &[])?;
        Ok(res.z().unwrap())
    }
    pub fn set_display_name(&mut self, arg0: String) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JObject::from(self.jni_ref().new_string(arg0).unwrap());
        self.jni_ref().call_method(
            &self.jni_object(),
            "setDisplayName",
            "(Ljava/lang/String;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn has_localized_name(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasLocalizedName", "()Z", &[])?;
        Ok(res.z().unwrap())
    }
    pub fn localized_name(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getLocalizedName",
            "()Ljava/lang/String;",
            &[],
        )?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    pub fn set_localized_name(&mut self, arg0: String) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JObject::from(self.jni_ref().new_string(arg0).unwrap());
        self.jni_ref().call_method(
            &self.jni_object(),
            "setLocalizedName",
            "(Ljava/lang/String;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn has_lore(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasLore", "()Z", &[])?;
        Ok(res.z().unwrap())
    }
    pub fn set_lore(&mut self, arg0: Vec<String>) -> Result<(), Box<dyn std::error::Error>> {
        let raw_val_0 = self
            .jni_ref()
            .new_object("java/util/ArrayList", "()V", &[])
            .unwrap();
        for v in arg0 {
            let map_val_0 = jni::objects::JObject::from(self.jni_ref().new_string(v).unwrap());
            self.jni_ref().call_method(
                &raw_val_0,
                "add",
                "(Ljava/Lang/Object)V",
                &[jni::objects::JValueGen::from(&map_val_0)],
            )?;
        }
        let val_0 = jni::objects::JValueGen::Object(raw_val_0);
        self.jni_ref().call_method(
            &self.jni_object(),
            "setLore",
            "(Ljava/util/List;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn has_custom_model_data(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "hasCustomModelData", "()Z", &[])?;
        Ok(res.z().unwrap())
    }
    pub fn custom_model_data(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getCustomModelData", "()I", &[])?;
        Ok(res.i().unwrap())
    }
    pub fn set_custom_model_data(&mut self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Int(arg0.into());
        self.jni_ref().call_method(
            &self.jni_object(),
            "setCustomModelData",
            "(I)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn has_enchants(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasEnchants", "()Z", &[])?;
        Ok(res.z().unwrap())
    }
    pub fn has_enchant(
        &mut self,
        arg0: impl Into<crate::bukkit::enchantments::Enchantment<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "hasEnchant",
            "(Lorg/bukkit/enchantments/Enchantment;)Z",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(res.z().unwrap())
    }
    pub fn get_enchant_level(
        &mut self,
        arg0: impl Into<crate::bukkit::enchantments::Enchantment<'mc>>,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getEnchantLevel",
            "(Lorg/bukkit/enchantments/Enchantment;)I",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(res.i().unwrap())
    }
    pub fn add_enchant(
        &mut self,
        arg0: impl Into<crate::bukkit::enchantments::Enchantment<'mc>>,
        arg1: i32,
        arg2: bool,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let val_1 = jni::objects::JValueGen::Int(arg1.into());
        // -2
        let val_2 = jni::objects::JValueGen::Bool(arg2.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "addEnchant",
            "(Lorg/bukkit/enchantments/Enchantment;IZ)Z",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        )?;
        Ok(res.z().unwrap())
    }
    pub fn remove_enchant(
        &mut self,
        arg0: impl Into<crate::bukkit::enchantments::Enchantment<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeEnchant",
            "(Lorg/bukkit/enchantments/Enchantment;)Z",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(res.z().unwrap())
    }
    pub fn has_conflicting_enchant(
        &mut self,
        arg0: impl Into<crate::bukkit::enchantments::Enchantment<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "hasConflictingEnchant",
            "(Lorg/bukkit/enchantments/Enchantment;)Z",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(res.z().unwrap())
    }
    pub fn add_item_flags(
        &mut self,
        _arg0: Vec<impl Into<crate::bukkit::inventory::ItemFlag<'mc>>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.jni_ref().call_method(
            &self.jni_object(),
            "addItemFlags",
            "(Lorg/bukkit/inventory/ItemFlag;)V",
            &[],
        )?;
        Ok(())
    }
    pub fn remove_item_flags(
        &mut self,
        _arg0: Vec<impl Into<crate::bukkit::inventory::ItemFlag<'mc>>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.jni_ref().call_method(
            &self.jni_object(),
            "removeItemFlags",
            "(Lorg/bukkit/inventory/ItemFlag;)V",
            &[],
        )?;
        Ok(())
    }
    pub fn has_item_flag(
        &mut self,
        arg0: impl Into<crate::bukkit::inventory::ItemFlag<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "hasItemFlag",
            "(Lorg/bukkit/inventory/ItemFlag;)Z",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(res.z().unwrap())
    }
    pub fn is_unbreakable(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isUnbreakable", "()Z", &[])?;
        Ok(res.z().unwrap())
    }
    pub fn set_unbreakable(&mut self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        // -2
        let val_0 = jni::objects::JValueGen::Bool(arg0.into());
        self.jni_ref().call_method(
            &self.jni_object(),
            "setUnbreakable",
            "(Z)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn has_attribute_modifiers(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "hasAttributeModifiers", "()Z", &[])?;
        Ok(res.z().unwrap())
    }
    pub fn add_attribute_modifier(
        &mut self,
        arg0: impl Into<crate::bukkit::attribute::Attribute<'mc>>,
        arg1: impl Into<crate::bukkit::attribute::AttributeModifier<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg1.into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "addAttributeModifier",
            "(Lorg/bukkit/attribute/Attribute;Lorg/bukkit/attribute/AttributeModifier;)Z",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        )?;
        Ok(res.z().unwrap())
    }
    pub fn remove_attribute_modifier_with_attribute(
        &mut self,
        arg0: impl Into<crate::bukkit::attribute::Attribute<'mc>>,
        arg1: std::option::Option<impl Into<crate::bukkit::attribute::AttributeModifier<'mc>>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg1.unwrap().into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeAttributeModifier",
            "(Lorg/bukkit/attribute/Attribute;Lorg/bukkit/attribute/AttributeModifier;)Z",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        )?;
        Ok(res.z().unwrap())
    }
    #[deprecated]
    pub fn custom_tag_container(
        &mut self,
    ) -> Result<
        crate::bukkit::inventory::meta::tags::CustomItemTagContainer<'mc>,
        Box<dyn std::error::Error>,
    > {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getCustomTagContainer",
            "()Lorg/bukkit/inventory/meta/tags/CustomItemTagContainer;",
            &[],
        )?;
        let ret = {
            crate::bukkit::inventory::meta::tags::CustomItemTagContainer(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    #[deprecated]
    pub fn set_version(&mut self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Int(arg0.into());
        self.jni_ref().call_method(
            &self.jni_object(),
            "setVersion",
            "(I)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn persistent_data_container(
        &mut self,
    ) -> Result<crate::bukkit::persistence::PersistentDataContainer<'mc>, Box<dyn std::error::Error>>
    {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getPersistentDataContainer",
            "()Lorg/bukkit/persistence/PersistentDataContainer;",
            &[],
        )?;
        let ret = {
            crate::bukkit::persistence::PersistentDataContainer(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
}
impl<'mc> crate::JNIRaw<'mc> for SpawnEggMeta<'mc> {
    fn jni_ref(&self) -> crate::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Into<crate::bukkit::inventory::meta::ItemMeta<'mc>> for SpawnEggMeta<'mc> {
    fn into(self) -> crate::bukkit::inventory::meta::ItemMeta<'mc> {
        crate::bukkit::inventory::meta::ItemMeta::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
/// An instantiatable struct that implements Damageable. Needed for returning it from Java.
pub struct Damageable<'mc>(
    pub(crate) crate::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> Damageable<'mc> {
    pub fn from_raw(
        env: &crate::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Damageable from null object.").into());
        }
        let cls = env.jni.borrow().get_object_class(&obj)?;
        let name_raw = env.call_method(cls, "getName", "()Ljava/lang/String;", &[])?;
        let oh = name_raw.l()?.into();
        let what = env.get_string(&oh)?;
        let name = what.to_string_lossy();
        if !name.ends_with("Damageable") {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Damageable object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    pub fn set_damage(&mut self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Int(arg0.into());
        self.jni_ref().call_method(
            &self.jni_object(),
            "setDamage",
            "(I)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn damage(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getDamage", "()I", &[])?;
        Ok(res.i().unwrap())
    }
    pub fn has_damage(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasDamage", "()Z", &[])?;
        Ok(res.z().unwrap())
    }
    pub fn clone(
        &mut self,
    ) -> Result<crate::bukkit::inventory::meta::Damageable<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "clone",
            "()Lorg/bukkit/inventory/meta/Damageable;",
            &[],
        )?;
        let ret = {
            crate::bukkit::inventory::meta::Damageable(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn display_name(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getDisplayName",
            "()Ljava/lang/String;",
            &[],
        )?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    pub fn as_string(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getAsString",
            "()Ljava/lang/String;",
            &[],
        )?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    pub fn has_display_name(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasDisplayName", "()Z", &[])?;
        Ok(res.z().unwrap())
    }
    pub fn set_display_name(&mut self, arg0: String) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JObject::from(self.jni_ref().new_string(arg0).unwrap());
        self.jni_ref().call_method(
            &self.jni_object(),
            "setDisplayName",
            "(Ljava/lang/String;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn has_localized_name(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasLocalizedName", "()Z", &[])?;
        Ok(res.z().unwrap())
    }
    pub fn localized_name(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getLocalizedName",
            "()Ljava/lang/String;",
            &[],
        )?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    pub fn set_localized_name(&mut self, arg0: String) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JObject::from(self.jni_ref().new_string(arg0).unwrap());
        self.jni_ref().call_method(
            &self.jni_object(),
            "setLocalizedName",
            "(Ljava/lang/String;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn has_lore(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasLore", "()Z", &[])?;
        Ok(res.z().unwrap())
    }
    pub fn set_lore(&mut self, arg0: Vec<String>) -> Result<(), Box<dyn std::error::Error>> {
        let raw_val_0 = self
            .jni_ref()
            .new_object("java/util/ArrayList", "()V", &[])
            .unwrap();
        for v in arg0 {
            let map_val_0 = jni::objects::JObject::from(self.jni_ref().new_string(v).unwrap());
            self.jni_ref().call_method(
                &raw_val_0,
                "add",
                "(Ljava/Lang/Object)V",
                &[jni::objects::JValueGen::from(&map_val_0)],
            )?;
        }
        let val_0 = jni::objects::JValueGen::Object(raw_val_0);
        self.jni_ref().call_method(
            &self.jni_object(),
            "setLore",
            "(Ljava/util/List;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn has_custom_model_data(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "hasCustomModelData", "()Z", &[])?;
        Ok(res.z().unwrap())
    }
    pub fn custom_model_data(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getCustomModelData", "()I", &[])?;
        Ok(res.i().unwrap())
    }
    pub fn set_custom_model_data(&mut self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Int(arg0.into());
        self.jni_ref().call_method(
            &self.jni_object(),
            "setCustomModelData",
            "(I)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn has_enchants(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasEnchants", "()Z", &[])?;
        Ok(res.z().unwrap())
    }
    pub fn has_enchant(
        &mut self,
        arg0: impl Into<crate::bukkit::enchantments::Enchantment<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "hasEnchant",
            "(Lorg/bukkit/enchantments/Enchantment;)Z",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(res.z().unwrap())
    }
    pub fn get_enchant_level(
        &mut self,
        arg0: impl Into<crate::bukkit::enchantments::Enchantment<'mc>>,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getEnchantLevel",
            "(Lorg/bukkit/enchantments/Enchantment;)I",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(res.i().unwrap())
    }
    pub fn add_enchant(
        &mut self,
        arg0: impl Into<crate::bukkit::enchantments::Enchantment<'mc>>,
        arg1: i32,
        arg2: bool,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let val_1 = jni::objects::JValueGen::Int(arg1.into());
        // -2
        let val_2 = jni::objects::JValueGen::Bool(arg2.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "addEnchant",
            "(Lorg/bukkit/enchantments/Enchantment;IZ)Z",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        )?;
        Ok(res.z().unwrap())
    }
    pub fn remove_enchant(
        &mut self,
        arg0: impl Into<crate::bukkit::enchantments::Enchantment<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeEnchant",
            "(Lorg/bukkit/enchantments/Enchantment;)Z",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(res.z().unwrap())
    }
    pub fn has_conflicting_enchant(
        &mut self,
        arg0: impl Into<crate::bukkit::enchantments::Enchantment<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "hasConflictingEnchant",
            "(Lorg/bukkit/enchantments/Enchantment;)Z",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(res.z().unwrap())
    }
    pub fn add_item_flags(
        &mut self,
        _arg0: Vec<impl Into<crate::bukkit::inventory::ItemFlag<'mc>>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.jni_ref().call_method(
            &self.jni_object(),
            "addItemFlags",
            "(Lorg/bukkit/inventory/ItemFlag;)V",
            &[],
        )?;
        Ok(())
    }
    pub fn remove_item_flags(
        &mut self,
        _arg0: Vec<impl Into<crate::bukkit::inventory::ItemFlag<'mc>>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.jni_ref().call_method(
            &self.jni_object(),
            "removeItemFlags",
            "(Lorg/bukkit/inventory/ItemFlag;)V",
            &[],
        )?;
        Ok(())
    }
    pub fn has_item_flag(
        &mut self,
        arg0: impl Into<crate::bukkit::inventory::ItemFlag<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "hasItemFlag",
            "(Lorg/bukkit/inventory/ItemFlag;)Z",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(res.z().unwrap())
    }
    pub fn is_unbreakable(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isUnbreakable", "()Z", &[])?;
        Ok(res.z().unwrap())
    }
    pub fn set_unbreakable(&mut self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        // -2
        let val_0 = jni::objects::JValueGen::Bool(arg0.into());
        self.jni_ref().call_method(
            &self.jni_object(),
            "setUnbreakable",
            "(Z)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn has_attribute_modifiers(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "hasAttributeModifiers", "()Z", &[])?;
        Ok(res.z().unwrap())
    }
    pub fn add_attribute_modifier(
        &mut self,
        arg0: impl Into<crate::bukkit::attribute::Attribute<'mc>>,
        arg1: impl Into<crate::bukkit::attribute::AttributeModifier<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg1.into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "addAttributeModifier",
            "(Lorg/bukkit/attribute/Attribute;Lorg/bukkit/attribute/AttributeModifier;)Z",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        )?;
        Ok(res.z().unwrap())
    }
    pub fn remove_attribute_modifier_with_attribute(
        &mut self,
        arg0: impl Into<crate::bukkit::attribute::Attribute<'mc>>,
        arg1: std::option::Option<impl Into<crate::bukkit::attribute::AttributeModifier<'mc>>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg1.unwrap().into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeAttributeModifier",
            "(Lorg/bukkit/attribute/Attribute;Lorg/bukkit/attribute/AttributeModifier;)Z",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        )?;
        Ok(res.z().unwrap())
    }
    #[deprecated]
    pub fn custom_tag_container(
        &mut self,
    ) -> Result<
        crate::bukkit::inventory::meta::tags::CustomItemTagContainer<'mc>,
        Box<dyn std::error::Error>,
    > {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getCustomTagContainer",
            "()Lorg/bukkit/inventory/meta/tags/CustomItemTagContainer;",
            &[],
        )?;
        let ret = {
            crate::bukkit::inventory::meta::tags::CustomItemTagContainer(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    #[deprecated]
    pub fn set_version(&mut self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Int(arg0.into());
        self.jni_ref().call_method(
            &self.jni_object(),
            "setVersion",
            "(I)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn persistent_data_container(
        &mut self,
    ) -> Result<crate::bukkit::persistence::PersistentDataContainer<'mc>, Box<dyn std::error::Error>>
    {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getPersistentDataContainer",
            "()Lorg/bukkit/persistence/PersistentDataContainer;",
            &[],
        )?;
        let ret = {
            crate::bukkit::persistence::PersistentDataContainer(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
}
impl<'mc> crate::JNIRaw<'mc> for Damageable<'mc> {
    fn jni_ref(&self) -> crate::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Into<crate::bukkit::inventory::meta::ItemMeta<'mc>> for Damageable<'mc> {
    fn into(self) -> crate::bukkit::inventory::meta::ItemMeta<'mc> {
        crate::bukkit::inventory::meta::ItemMeta::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
/// An instantiatable struct that implements FireworkMeta. Needed for returning it from Java.
pub struct FireworkMeta<'mc>(
    pub(crate) crate::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> FireworkMeta<'mc> {
    pub fn from_raw(
        env: &crate::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate FireworkMeta from null object.").into());
        }
        let cls = env.jni.borrow().get_object_class(&obj)?;
        let name_raw = env.call_method(cls, "getName", "()Ljava/lang/String;", &[])?;
        let oh = name_raw.l()?.into();
        let what = env.get_string(&oh)?;
        let name = what.to_string_lossy();
        if !name.ends_with("FireworkMeta") {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a FireworkMeta object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    pub fn power(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getPower", "()I", &[])?;
        Ok(res.i().unwrap())
    }
    pub fn set_power(&mut self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Int(arg0.into());
        self.jni_ref().call_method(
            &self.jni_object(),
            "setPower",
            "(I)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn add_effect(
        &mut self,
        arg0: impl Into<crate::bukkit::FireworkEffect<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        self.jni_ref().call_method(
            &self.jni_object(),
            "addEffect",
            "(Lorg/bukkit/FireworkEffect;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn effects_size(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getEffectsSize", "()I", &[])?;
        Ok(res.i().unwrap())
    }
    pub fn remove_effect(&mut self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Int(arg0.into());
        self.jni_ref().call_method(
            &self.jni_object(),
            "removeEffect",
            "(I)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn clear_effects(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.jni_ref()
            .call_method(&self.jni_object(), "clearEffects", "()V", &[])?;
        Ok(())
    }
    pub fn has_effects(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasEffects", "()Z", &[])?;
        Ok(res.z().unwrap())
    }
    pub fn clone(
        &mut self,
    ) -> Result<crate::bukkit::inventory::meta::FireworkMeta<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "clone",
            "()Lorg/bukkit/inventory/meta/FireworkMeta;",
            &[],
        )?;
        let ret = {
            crate::bukkit::inventory::meta::FireworkMeta(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn display_name(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getDisplayName",
            "()Ljava/lang/String;",
            &[],
        )?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    pub fn as_string(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getAsString",
            "()Ljava/lang/String;",
            &[],
        )?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    pub fn has_display_name(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasDisplayName", "()Z", &[])?;
        Ok(res.z().unwrap())
    }
    pub fn set_display_name(&mut self, arg0: String) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JObject::from(self.jni_ref().new_string(arg0).unwrap());
        self.jni_ref().call_method(
            &self.jni_object(),
            "setDisplayName",
            "(Ljava/lang/String;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn has_localized_name(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasLocalizedName", "()Z", &[])?;
        Ok(res.z().unwrap())
    }
    pub fn localized_name(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getLocalizedName",
            "()Ljava/lang/String;",
            &[],
        )?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    pub fn set_localized_name(&mut self, arg0: String) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JObject::from(self.jni_ref().new_string(arg0).unwrap());
        self.jni_ref().call_method(
            &self.jni_object(),
            "setLocalizedName",
            "(Ljava/lang/String;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn has_lore(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasLore", "()Z", &[])?;
        Ok(res.z().unwrap())
    }
    pub fn set_lore(&mut self, arg0: Vec<String>) -> Result<(), Box<dyn std::error::Error>> {
        let raw_val_0 = self
            .jni_ref()
            .new_object("java/util/ArrayList", "()V", &[])
            .unwrap();
        for v in arg0 {
            let map_val_0 = jni::objects::JObject::from(self.jni_ref().new_string(v).unwrap());
            self.jni_ref().call_method(
                &raw_val_0,
                "add",
                "(Ljava/Lang/Object)V",
                &[jni::objects::JValueGen::from(&map_val_0)],
            )?;
        }
        let val_0 = jni::objects::JValueGen::Object(raw_val_0);
        self.jni_ref().call_method(
            &self.jni_object(),
            "setLore",
            "(Ljava/util/List;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn has_custom_model_data(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "hasCustomModelData", "()Z", &[])?;
        Ok(res.z().unwrap())
    }
    pub fn custom_model_data(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getCustomModelData", "()I", &[])?;
        Ok(res.i().unwrap())
    }
    pub fn set_custom_model_data(&mut self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Int(arg0.into());
        self.jni_ref().call_method(
            &self.jni_object(),
            "setCustomModelData",
            "(I)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn has_enchants(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasEnchants", "()Z", &[])?;
        Ok(res.z().unwrap())
    }
    pub fn has_enchant(
        &mut self,
        arg0: impl Into<crate::bukkit::enchantments::Enchantment<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "hasEnchant",
            "(Lorg/bukkit/enchantments/Enchantment;)Z",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(res.z().unwrap())
    }
    pub fn get_enchant_level(
        &mut self,
        arg0: impl Into<crate::bukkit::enchantments::Enchantment<'mc>>,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getEnchantLevel",
            "(Lorg/bukkit/enchantments/Enchantment;)I",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(res.i().unwrap())
    }
    pub fn add_enchant(
        &mut self,
        arg0: impl Into<crate::bukkit::enchantments::Enchantment<'mc>>,
        arg1: i32,
        arg2: bool,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let val_1 = jni::objects::JValueGen::Int(arg1.into());
        // -2
        let val_2 = jni::objects::JValueGen::Bool(arg2.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "addEnchant",
            "(Lorg/bukkit/enchantments/Enchantment;IZ)Z",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        )?;
        Ok(res.z().unwrap())
    }
    pub fn remove_enchant(
        &mut self,
        arg0: impl Into<crate::bukkit::enchantments::Enchantment<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeEnchant",
            "(Lorg/bukkit/enchantments/Enchantment;)Z",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(res.z().unwrap())
    }
    pub fn has_conflicting_enchant(
        &mut self,
        arg0: impl Into<crate::bukkit::enchantments::Enchantment<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "hasConflictingEnchant",
            "(Lorg/bukkit/enchantments/Enchantment;)Z",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(res.z().unwrap())
    }
    pub fn add_item_flags(
        &mut self,
        _arg0: Vec<impl Into<crate::bukkit::inventory::ItemFlag<'mc>>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.jni_ref().call_method(
            &self.jni_object(),
            "addItemFlags",
            "(Lorg/bukkit/inventory/ItemFlag;)V",
            &[],
        )?;
        Ok(())
    }
    pub fn remove_item_flags(
        &mut self,
        _arg0: Vec<impl Into<crate::bukkit::inventory::ItemFlag<'mc>>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.jni_ref().call_method(
            &self.jni_object(),
            "removeItemFlags",
            "(Lorg/bukkit/inventory/ItemFlag;)V",
            &[],
        )?;
        Ok(())
    }
    pub fn has_item_flag(
        &mut self,
        arg0: impl Into<crate::bukkit::inventory::ItemFlag<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "hasItemFlag",
            "(Lorg/bukkit/inventory/ItemFlag;)Z",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(res.z().unwrap())
    }
    pub fn is_unbreakable(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isUnbreakable", "()Z", &[])?;
        Ok(res.z().unwrap())
    }
    pub fn set_unbreakable(&mut self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        // -2
        let val_0 = jni::objects::JValueGen::Bool(arg0.into());
        self.jni_ref().call_method(
            &self.jni_object(),
            "setUnbreakable",
            "(Z)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn has_attribute_modifiers(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "hasAttributeModifiers", "()Z", &[])?;
        Ok(res.z().unwrap())
    }
    pub fn add_attribute_modifier(
        &mut self,
        arg0: impl Into<crate::bukkit::attribute::Attribute<'mc>>,
        arg1: impl Into<crate::bukkit::attribute::AttributeModifier<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg1.into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "addAttributeModifier",
            "(Lorg/bukkit/attribute/Attribute;Lorg/bukkit/attribute/AttributeModifier;)Z",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        )?;
        Ok(res.z().unwrap())
    }
    pub fn remove_attribute_modifier_with_attribute(
        &mut self,
        arg0: impl Into<crate::bukkit::attribute::Attribute<'mc>>,
        arg1: std::option::Option<impl Into<crate::bukkit::attribute::AttributeModifier<'mc>>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg1.unwrap().into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeAttributeModifier",
            "(Lorg/bukkit/attribute/Attribute;Lorg/bukkit/attribute/AttributeModifier;)Z",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        )?;
        Ok(res.z().unwrap())
    }
    #[deprecated]
    pub fn custom_tag_container(
        &mut self,
    ) -> Result<
        crate::bukkit::inventory::meta::tags::CustomItemTagContainer<'mc>,
        Box<dyn std::error::Error>,
    > {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getCustomTagContainer",
            "()Lorg/bukkit/inventory/meta/tags/CustomItemTagContainer;",
            &[],
        )?;
        let ret = {
            crate::bukkit::inventory::meta::tags::CustomItemTagContainer(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    #[deprecated]
    pub fn set_version(&mut self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Int(arg0.into());
        self.jni_ref().call_method(
            &self.jni_object(),
            "setVersion",
            "(I)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn persistent_data_container(
        &mut self,
    ) -> Result<crate::bukkit::persistence::PersistentDataContainer<'mc>, Box<dyn std::error::Error>>
    {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getPersistentDataContainer",
            "()Lorg/bukkit/persistence/PersistentDataContainer;",
            &[],
        )?;
        let ret = {
            crate::bukkit::persistence::PersistentDataContainer(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
}
impl<'mc> crate::JNIRaw<'mc> for FireworkMeta<'mc> {
    fn jni_ref(&self) -> crate::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Into<crate::bukkit::inventory::meta::ItemMeta<'mc>> for FireworkMeta<'mc> {
    fn into(self) -> crate::bukkit::inventory::meta::ItemMeta<'mc> {
        crate::bukkit::inventory::meta::ItemMeta::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
/// An instantiatable struct that implements PotionMeta. Needed for returning it from Java.
pub struct PotionMeta<'mc>(
    pub(crate) crate::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> PotionMeta<'mc> {
    pub fn from_raw(
        env: &crate::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate PotionMeta from null object.").into());
        }
        let cls = env.jni.borrow().get_object_class(&obj)?;
        let name_raw = env.call_method(cls, "getName", "()Ljava/lang/String;", &[])?;
        let oh = name_raw.l()?.into();
        let what = env.get_string(&oh)?;
        let name = what.to_string_lossy();
        if !name.ends_with("PotionMeta") {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a PotionMeta object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    pub fn set_color(
        &mut self,
        arg0: impl Into<crate::bukkit::Color<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        self.jni_ref().call_method(
            &self.jni_object(),
            "setColor",
            "(Lorg/bukkit/Color;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn color(&mut self) -> Result<crate::bukkit::Color<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getColor",
            "()Lorg/bukkit/Color;",
            &[],
        )?;
        let ret = {
            crate::bukkit::Color(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn has_custom_effects(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasCustomEffects", "()Z", &[])?;
        Ok(res.z().unwrap())
    }
    pub fn add_custom_effect(
        &mut self,
        arg0: impl Into<crate::bukkit::potion::PotionEffect<'mc>>,
        arg1: bool,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        // -2
        let val_1 = jni::objects::JValueGen::Bool(arg1.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "addCustomEffect",
            "(Lorg/bukkit/potion/PotionEffect;Z)Z",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        )?;
        Ok(res.z().unwrap())
    }
    pub fn remove_custom_effect(
        &mut self,
        arg0: impl Into<crate::bukkit::potion::PotionEffectType<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeCustomEffect",
            "(Lorg/bukkit/potion/PotionEffectType;)Z",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(res.z().unwrap())
    }
    pub fn has_custom_effect(
        &mut self,
        arg0: impl Into<crate::bukkit::potion::PotionEffectType<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "hasCustomEffect",
            "(Lorg/bukkit/potion/PotionEffectType;)Z",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(res.z().unwrap())
    }
    pub fn clear_custom_effects(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "clearCustomEffects", "()Z", &[])?;
        Ok(res.z().unwrap())
    }
    pub fn set_base_potion_data(
        &mut self,
        arg0: impl Into<crate::bukkit::potion::PotionData<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        self.jni_ref().call_method(
            &self.jni_object(),
            "setBasePotionData",
            "(Lorg/bukkit/potion/PotionData;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn base_potion_data(
        &mut self,
    ) -> Result<crate::bukkit::potion::PotionData<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getBasePotionData",
            "()Lorg/bukkit/potion/PotionData;",
            &[],
        )?;
        let ret = {
            crate::bukkit::potion::PotionData(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    #[deprecated]
    pub fn set_main_effect(
        &mut self,
        arg0: impl Into<crate::bukkit::potion::PotionEffectType<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setMainEffect",
            "(Lorg/bukkit/potion/PotionEffectType;)Z",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(res.z().unwrap())
    }
    pub fn has_color(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasColor", "()Z", &[])?;
        Ok(res.z().unwrap())
    }
    pub fn clone(
        &mut self,
    ) -> Result<crate::bukkit::inventory::meta::PotionMeta<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "clone",
            "()Lorg/bukkit/inventory/meta/PotionMeta;",
            &[],
        )?;
        let ret = {
            crate::bukkit::inventory::meta::PotionMeta(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn display_name(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getDisplayName",
            "()Ljava/lang/String;",
            &[],
        )?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    pub fn as_string(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getAsString",
            "()Ljava/lang/String;",
            &[],
        )?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    pub fn has_display_name(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasDisplayName", "()Z", &[])?;
        Ok(res.z().unwrap())
    }
    pub fn set_display_name(&mut self, arg0: String) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JObject::from(self.jni_ref().new_string(arg0).unwrap());
        self.jni_ref().call_method(
            &self.jni_object(),
            "setDisplayName",
            "(Ljava/lang/String;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn has_localized_name(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasLocalizedName", "()Z", &[])?;
        Ok(res.z().unwrap())
    }
    pub fn localized_name(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getLocalizedName",
            "()Ljava/lang/String;",
            &[],
        )?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    pub fn set_localized_name(&mut self, arg0: String) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JObject::from(self.jni_ref().new_string(arg0).unwrap());
        self.jni_ref().call_method(
            &self.jni_object(),
            "setLocalizedName",
            "(Ljava/lang/String;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn has_lore(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasLore", "()Z", &[])?;
        Ok(res.z().unwrap())
    }
    pub fn set_lore(&mut self, arg0: Vec<String>) -> Result<(), Box<dyn std::error::Error>> {
        let raw_val_0 = self
            .jni_ref()
            .new_object("java/util/ArrayList", "()V", &[])
            .unwrap();
        for v in arg0 {
            let map_val_0 = jni::objects::JObject::from(self.jni_ref().new_string(v).unwrap());
            self.jni_ref().call_method(
                &raw_val_0,
                "add",
                "(Ljava/Lang/Object)V",
                &[jni::objects::JValueGen::from(&map_val_0)],
            )?;
        }
        let val_0 = jni::objects::JValueGen::Object(raw_val_0);
        self.jni_ref().call_method(
            &self.jni_object(),
            "setLore",
            "(Ljava/util/List;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn has_custom_model_data(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "hasCustomModelData", "()Z", &[])?;
        Ok(res.z().unwrap())
    }
    pub fn custom_model_data(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getCustomModelData", "()I", &[])?;
        Ok(res.i().unwrap())
    }
    pub fn set_custom_model_data(&mut self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Int(arg0.into());
        self.jni_ref().call_method(
            &self.jni_object(),
            "setCustomModelData",
            "(I)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn has_enchants(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasEnchants", "()Z", &[])?;
        Ok(res.z().unwrap())
    }
    pub fn has_enchant(
        &mut self,
        arg0: impl Into<crate::bukkit::enchantments::Enchantment<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "hasEnchant",
            "(Lorg/bukkit/enchantments/Enchantment;)Z",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(res.z().unwrap())
    }
    pub fn get_enchant_level(
        &mut self,
        arg0: impl Into<crate::bukkit::enchantments::Enchantment<'mc>>,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getEnchantLevel",
            "(Lorg/bukkit/enchantments/Enchantment;)I",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(res.i().unwrap())
    }
    pub fn add_enchant(
        &mut self,
        arg0: impl Into<crate::bukkit::enchantments::Enchantment<'mc>>,
        arg1: i32,
        arg2: bool,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let val_1 = jni::objects::JValueGen::Int(arg1.into());
        // -2
        let val_2 = jni::objects::JValueGen::Bool(arg2.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "addEnchant",
            "(Lorg/bukkit/enchantments/Enchantment;IZ)Z",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        )?;
        Ok(res.z().unwrap())
    }
    pub fn remove_enchant(
        &mut self,
        arg0: impl Into<crate::bukkit::enchantments::Enchantment<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeEnchant",
            "(Lorg/bukkit/enchantments/Enchantment;)Z",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(res.z().unwrap())
    }
    pub fn has_conflicting_enchant(
        &mut self,
        arg0: impl Into<crate::bukkit::enchantments::Enchantment<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "hasConflictingEnchant",
            "(Lorg/bukkit/enchantments/Enchantment;)Z",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(res.z().unwrap())
    }
    pub fn add_item_flags(
        &mut self,
        _arg0: Vec<impl Into<crate::bukkit::inventory::ItemFlag<'mc>>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.jni_ref().call_method(
            &self.jni_object(),
            "addItemFlags",
            "(Lorg/bukkit/inventory/ItemFlag;)V",
            &[],
        )?;
        Ok(())
    }
    pub fn remove_item_flags(
        &mut self,
        _arg0: Vec<impl Into<crate::bukkit::inventory::ItemFlag<'mc>>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.jni_ref().call_method(
            &self.jni_object(),
            "removeItemFlags",
            "(Lorg/bukkit/inventory/ItemFlag;)V",
            &[],
        )?;
        Ok(())
    }
    pub fn has_item_flag(
        &mut self,
        arg0: impl Into<crate::bukkit::inventory::ItemFlag<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "hasItemFlag",
            "(Lorg/bukkit/inventory/ItemFlag;)Z",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(res.z().unwrap())
    }
    pub fn is_unbreakable(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isUnbreakable", "()Z", &[])?;
        Ok(res.z().unwrap())
    }
    pub fn set_unbreakable(&mut self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        // -2
        let val_0 = jni::objects::JValueGen::Bool(arg0.into());
        self.jni_ref().call_method(
            &self.jni_object(),
            "setUnbreakable",
            "(Z)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn has_attribute_modifiers(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "hasAttributeModifiers", "()Z", &[])?;
        Ok(res.z().unwrap())
    }
    pub fn add_attribute_modifier(
        &mut self,
        arg0: impl Into<crate::bukkit::attribute::Attribute<'mc>>,
        arg1: impl Into<crate::bukkit::attribute::AttributeModifier<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg1.into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "addAttributeModifier",
            "(Lorg/bukkit/attribute/Attribute;Lorg/bukkit/attribute/AttributeModifier;)Z",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        )?;
        Ok(res.z().unwrap())
    }
    pub fn remove_attribute_modifier_with_attribute(
        &mut self,
        arg0: impl Into<crate::bukkit::attribute::Attribute<'mc>>,
        arg1: std::option::Option<impl Into<crate::bukkit::attribute::AttributeModifier<'mc>>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg1.unwrap().into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeAttributeModifier",
            "(Lorg/bukkit/attribute/Attribute;Lorg/bukkit/attribute/AttributeModifier;)Z",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        )?;
        Ok(res.z().unwrap())
    }
    #[deprecated]
    pub fn custom_tag_container(
        &mut self,
    ) -> Result<
        crate::bukkit::inventory::meta::tags::CustomItemTagContainer<'mc>,
        Box<dyn std::error::Error>,
    > {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getCustomTagContainer",
            "()Lorg/bukkit/inventory/meta/tags/CustomItemTagContainer;",
            &[],
        )?;
        let ret = {
            crate::bukkit::inventory::meta::tags::CustomItemTagContainer(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    #[deprecated]
    pub fn set_version(&mut self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Int(arg0.into());
        self.jni_ref().call_method(
            &self.jni_object(),
            "setVersion",
            "(I)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn persistent_data_container(
        &mut self,
    ) -> Result<crate::bukkit::persistence::PersistentDataContainer<'mc>, Box<dyn std::error::Error>>
    {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getPersistentDataContainer",
            "()Lorg/bukkit/persistence/PersistentDataContainer;",
            &[],
        )?;
        let ret = {
            crate::bukkit::persistence::PersistentDataContainer(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
}
impl<'mc> crate::JNIRaw<'mc> for PotionMeta<'mc> {
    fn jni_ref(&self) -> crate::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Into<crate::bukkit::inventory::meta::ItemMeta<'mc>> for PotionMeta<'mc> {
    fn into(self) -> crate::bukkit::inventory::meta::ItemMeta<'mc> {
        crate::bukkit::inventory::meta::ItemMeta::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
/// An instantiatable struct that implements BlockStateMeta. Needed for returning it from Java.
pub struct BlockStateMeta<'mc>(
    pub(crate) crate::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> BlockStateMeta<'mc> {
    pub fn from_raw(
        env: &crate::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate BlockStateMeta from null object.").into(),
            );
        }
        let cls = env.jni.borrow().get_object_class(&obj)?;
        let name_raw = env.call_method(cls, "getName", "()Ljava/lang/String;", &[])?;
        let oh = name_raw.l()?.into();
        let what = env.get_string(&oh)?;
        let name = what.to_string_lossy();
        if !name.ends_with("BlockStateMeta") {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a BlockStateMeta object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    pub fn block_state(
        &mut self,
    ) -> Result<crate::bukkit::block::BlockState<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getBlockState",
            "()Lorg/bukkit/block/BlockState;",
            &[],
        )?;
        let ret = {
            crate::bukkit::block::BlockState(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn has_block_state(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasBlockState", "()Z", &[])?;
        Ok(res.z().unwrap())
    }
    pub fn set_block_state(
        &mut self,
        arg0: impl Into<crate::bukkit::block::BlockState<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        self.jni_ref().call_method(
            &self.jni_object(),
            "setBlockState",
            "(Lorg/bukkit/block/BlockState;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn display_name(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getDisplayName",
            "()Ljava/lang/String;",
            &[],
        )?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    pub fn as_string(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getAsString",
            "()Ljava/lang/String;",
            &[],
        )?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    pub fn has_display_name(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasDisplayName", "()Z", &[])?;
        Ok(res.z().unwrap())
    }
    pub fn set_display_name(&mut self, arg0: String) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JObject::from(self.jni_ref().new_string(arg0).unwrap());
        self.jni_ref().call_method(
            &self.jni_object(),
            "setDisplayName",
            "(Ljava/lang/String;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn has_localized_name(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasLocalizedName", "()Z", &[])?;
        Ok(res.z().unwrap())
    }
    pub fn localized_name(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getLocalizedName",
            "()Ljava/lang/String;",
            &[],
        )?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    pub fn set_localized_name(&mut self, arg0: String) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JObject::from(self.jni_ref().new_string(arg0).unwrap());
        self.jni_ref().call_method(
            &self.jni_object(),
            "setLocalizedName",
            "(Ljava/lang/String;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn has_lore(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasLore", "()Z", &[])?;
        Ok(res.z().unwrap())
    }
    pub fn set_lore(&mut self, arg0: Vec<String>) -> Result<(), Box<dyn std::error::Error>> {
        let raw_val_0 = self
            .jni_ref()
            .new_object("java/util/ArrayList", "()V", &[])
            .unwrap();
        for v in arg0 {
            let map_val_0 = jni::objects::JObject::from(self.jni_ref().new_string(v).unwrap());
            self.jni_ref().call_method(
                &raw_val_0,
                "add",
                "(Ljava/Lang/Object)V",
                &[jni::objects::JValueGen::from(&map_val_0)],
            )?;
        }
        let val_0 = jni::objects::JValueGen::Object(raw_val_0);
        self.jni_ref().call_method(
            &self.jni_object(),
            "setLore",
            "(Ljava/util/List;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn has_custom_model_data(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "hasCustomModelData", "()Z", &[])?;
        Ok(res.z().unwrap())
    }
    pub fn custom_model_data(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getCustomModelData", "()I", &[])?;
        Ok(res.i().unwrap())
    }
    pub fn set_custom_model_data(&mut self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Int(arg0.into());
        self.jni_ref().call_method(
            &self.jni_object(),
            "setCustomModelData",
            "(I)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn has_enchants(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasEnchants", "()Z", &[])?;
        Ok(res.z().unwrap())
    }
    pub fn has_enchant(
        &mut self,
        arg0: impl Into<crate::bukkit::enchantments::Enchantment<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "hasEnchant",
            "(Lorg/bukkit/enchantments/Enchantment;)Z",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(res.z().unwrap())
    }
    pub fn get_enchant_level(
        &mut self,
        arg0: impl Into<crate::bukkit::enchantments::Enchantment<'mc>>,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getEnchantLevel",
            "(Lorg/bukkit/enchantments/Enchantment;)I",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(res.i().unwrap())
    }
    pub fn add_enchant(
        &mut self,
        arg0: impl Into<crate::bukkit::enchantments::Enchantment<'mc>>,
        arg1: i32,
        arg2: bool,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let val_1 = jni::objects::JValueGen::Int(arg1.into());
        // -2
        let val_2 = jni::objects::JValueGen::Bool(arg2.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "addEnchant",
            "(Lorg/bukkit/enchantments/Enchantment;IZ)Z",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        )?;
        Ok(res.z().unwrap())
    }
    pub fn remove_enchant(
        &mut self,
        arg0: impl Into<crate::bukkit::enchantments::Enchantment<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeEnchant",
            "(Lorg/bukkit/enchantments/Enchantment;)Z",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(res.z().unwrap())
    }
    pub fn has_conflicting_enchant(
        &mut self,
        arg0: impl Into<crate::bukkit::enchantments::Enchantment<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "hasConflictingEnchant",
            "(Lorg/bukkit/enchantments/Enchantment;)Z",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(res.z().unwrap())
    }
    pub fn add_item_flags(
        &mut self,
        _arg0: Vec<impl Into<crate::bukkit::inventory::ItemFlag<'mc>>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.jni_ref().call_method(
            &self.jni_object(),
            "addItemFlags",
            "(Lorg/bukkit/inventory/ItemFlag;)V",
            &[],
        )?;
        Ok(())
    }
    pub fn remove_item_flags(
        &mut self,
        _arg0: Vec<impl Into<crate::bukkit::inventory::ItemFlag<'mc>>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.jni_ref().call_method(
            &self.jni_object(),
            "removeItemFlags",
            "(Lorg/bukkit/inventory/ItemFlag;)V",
            &[],
        )?;
        Ok(())
    }
    pub fn has_item_flag(
        &mut self,
        arg0: impl Into<crate::bukkit::inventory::ItemFlag<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "hasItemFlag",
            "(Lorg/bukkit/inventory/ItemFlag;)Z",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(res.z().unwrap())
    }
    pub fn is_unbreakable(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isUnbreakable", "()Z", &[])?;
        Ok(res.z().unwrap())
    }
    pub fn set_unbreakable(&mut self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        // -2
        let val_0 = jni::objects::JValueGen::Bool(arg0.into());
        self.jni_ref().call_method(
            &self.jni_object(),
            "setUnbreakable",
            "(Z)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn has_attribute_modifiers(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "hasAttributeModifiers", "()Z", &[])?;
        Ok(res.z().unwrap())
    }
    pub fn add_attribute_modifier(
        &mut self,
        arg0: impl Into<crate::bukkit::attribute::Attribute<'mc>>,
        arg1: impl Into<crate::bukkit::attribute::AttributeModifier<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg1.into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "addAttributeModifier",
            "(Lorg/bukkit/attribute/Attribute;Lorg/bukkit/attribute/AttributeModifier;)Z",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        )?;
        Ok(res.z().unwrap())
    }
    pub fn remove_attribute_modifier_with_attribute(
        &mut self,
        arg0: impl Into<crate::bukkit::attribute::Attribute<'mc>>,
        arg1: std::option::Option<impl Into<crate::bukkit::attribute::AttributeModifier<'mc>>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg1.unwrap().into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeAttributeModifier",
            "(Lorg/bukkit/attribute/Attribute;Lorg/bukkit/attribute/AttributeModifier;)Z",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        )?;
        Ok(res.z().unwrap())
    }
    #[deprecated]
    pub fn custom_tag_container(
        &mut self,
    ) -> Result<
        crate::bukkit::inventory::meta::tags::CustomItemTagContainer<'mc>,
        Box<dyn std::error::Error>,
    > {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getCustomTagContainer",
            "()Lorg/bukkit/inventory/meta/tags/CustomItemTagContainer;",
            &[],
        )?;
        let ret = {
            crate::bukkit::inventory::meta::tags::CustomItemTagContainer(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    #[deprecated]
    pub fn set_version(&mut self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Int(arg0.into());
        self.jni_ref().call_method(
            &self.jni_object(),
            "setVersion",
            "(I)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn clone(&mut self) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "clone", "()Ljava/lang/Object;", &[])?;
        Ok(res.l().unwrap())
    }
    pub fn persistent_data_container(
        &mut self,
    ) -> Result<crate::bukkit::persistence::PersistentDataContainer<'mc>, Box<dyn std::error::Error>>
    {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getPersistentDataContainer",
            "()Lorg/bukkit/persistence/PersistentDataContainer;",
            &[],
        )?;
        let ret = {
            crate::bukkit::persistence::PersistentDataContainer(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
}
impl<'mc> crate::JNIRaw<'mc> for BlockStateMeta<'mc> {
    fn jni_ref(&self) -> crate::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Into<crate::bukkit::inventory::meta::ItemMeta<'mc>> for BlockStateMeta<'mc> {
    fn into(self) -> crate::bukkit::inventory::meta::ItemMeta<'mc> {
        crate::bukkit::inventory::meta::ItemMeta::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
/// An instantiatable struct that implements EnchantmentStorageMeta. Needed for returning it from Java.
pub struct EnchantmentStorageMeta<'mc>(
    pub(crate) crate::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> EnchantmentStorageMeta<'mc> {
    pub fn from_raw(
        env: &crate::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate EnchantmentStorageMeta from null object."
            )
            .into());
        }
        let cls = env.jni.borrow().get_object_class(&obj)?;
        let name_raw = env.call_method(cls, "getName", "()Ljava/lang/String;", &[])?;
        let oh = name_raw.l()?.into();
        let what = env.get_string(&oh)?;
        let name = what.to_string_lossy();
        if !name.ends_with("EnchantmentStorageMeta") {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a EnchantmentStorageMeta object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    pub fn has_stored_enchant(
        &mut self,
        arg0: impl Into<crate::bukkit::enchantments::Enchantment<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "hasStoredEnchant",
            "(Lorg/bukkit/enchantments/Enchantment;)Z",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(res.z().unwrap())
    }
    pub fn get_stored_enchant_level(
        &mut self,
        arg0: impl Into<crate::bukkit::enchantments::Enchantment<'mc>>,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getStoredEnchantLevel",
            "(Lorg/bukkit/enchantments/Enchantment;)I",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(res.i().unwrap())
    }
    pub fn add_stored_enchant(
        &mut self,
        arg0: impl Into<crate::bukkit::enchantments::Enchantment<'mc>>,
        arg1: i32,
        arg2: bool,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let val_1 = jni::objects::JValueGen::Int(arg1.into());
        // -2
        let val_2 = jni::objects::JValueGen::Bool(arg2.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "addStoredEnchant",
            "(Lorg/bukkit/enchantments/Enchantment;IZ)Z",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        )?;
        Ok(res.z().unwrap())
    }
    pub fn remove_stored_enchant(
        &mut self,
        arg0: impl Into<crate::bukkit::enchantments::Enchantment<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeStoredEnchant",
            "(Lorg/bukkit/enchantments/Enchantment;)Z",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(res.z().unwrap())
    }
    pub fn has_conflicting_stored_enchant(
        &mut self,
        arg0: impl Into<crate::bukkit::enchantments::Enchantment<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "hasConflictingStoredEnchant",
            "(Lorg/bukkit/enchantments/Enchantment;)Z",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(res.z().unwrap())
    }
    pub fn has_stored_enchants(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "hasStoredEnchants", "()Z", &[])?;
        Ok(res.z().unwrap())
    }
    pub fn clone(
        &mut self,
    ) -> Result<
        crate::bukkit::inventory::meta::EnchantmentStorageMeta<'mc>,
        Box<dyn std::error::Error>,
    > {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "clone",
            "()Lorg/bukkit/inventory/meta/EnchantmentStorageMeta;",
            &[],
        )?;
        let ret = {
            crate::bukkit::inventory::meta::EnchantmentStorageMeta(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn display_name(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getDisplayName",
            "()Ljava/lang/String;",
            &[],
        )?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    pub fn as_string(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getAsString",
            "()Ljava/lang/String;",
            &[],
        )?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    pub fn has_display_name(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasDisplayName", "()Z", &[])?;
        Ok(res.z().unwrap())
    }
    pub fn set_display_name(&mut self, arg0: String) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JObject::from(self.jni_ref().new_string(arg0).unwrap());
        self.jni_ref().call_method(
            &self.jni_object(),
            "setDisplayName",
            "(Ljava/lang/String;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn has_localized_name(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasLocalizedName", "()Z", &[])?;
        Ok(res.z().unwrap())
    }
    pub fn localized_name(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getLocalizedName",
            "()Ljava/lang/String;",
            &[],
        )?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    pub fn set_localized_name(&mut self, arg0: String) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JObject::from(self.jni_ref().new_string(arg0).unwrap());
        self.jni_ref().call_method(
            &self.jni_object(),
            "setLocalizedName",
            "(Ljava/lang/String;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn has_lore(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasLore", "()Z", &[])?;
        Ok(res.z().unwrap())
    }
    pub fn set_lore(&mut self, arg0: Vec<String>) -> Result<(), Box<dyn std::error::Error>> {
        let raw_val_0 = self
            .jni_ref()
            .new_object("java/util/ArrayList", "()V", &[])
            .unwrap();
        for v in arg0 {
            let map_val_0 = jni::objects::JObject::from(self.jni_ref().new_string(v).unwrap());
            self.jni_ref().call_method(
                &raw_val_0,
                "add",
                "(Ljava/Lang/Object)V",
                &[jni::objects::JValueGen::from(&map_val_0)],
            )?;
        }
        let val_0 = jni::objects::JValueGen::Object(raw_val_0);
        self.jni_ref().call_method(
            &self.jni_object(),
            "setLore",
            "(Ljava/util/List;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn has_custom_model_data(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "hasCustomModelData", "()Z", &[])?;
        Ok(res.z().unwrap())
    }
    pub fn custom_model_data(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getCustomModelData", "()I", &[])?;
        Ok(res.i().unwrap())
    }
    pub fn set_custom_model_data(&mut self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Int(arg0.into());
        self.jni_ref().call_method(
            &self.jni_object(),
            "setCustomModelData",
            "(I)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn has_enchants(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasEnchants", "()Z", &[])?;
        Ok(res.z().unwrap())
    }
    pub fn has_enchant(
        &mut self,
        arg0: impl Into<crate::bukkit::enchantments::Enchantment<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "hasEnchant",
            "(Lorg/bukkit/enchantments/Enchantment;)Z",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(res.z().unwrap())
    }
    pub fn get_enchant_level(
        &mut self,
        arg0: impl Into<crate::bukkit::enchantments::Enchantment<'mc>>,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getEnchantLevel",
            "(Lorg/bukkit/enchantments/Enchantment;)I",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(res.i().unwrap())
    }
    pub fn add_enchant(
        &mut self,
        arg0: impl Into<crate::bukkit::enchantments::Enchantment<'mc>>,
        arg1: i32,
        arg2: bool,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let val_1 = jni::objects::JValueGen::Int(arg1.into());
        // -2
        let val_2 = jni::objects::JValueGen::Bool(arg2.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "addEnchant",
            "(Lorg/bukkit/enchantments/Enchantment;IZ)Z",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        )?;
        Ok(res.z().unwrap())
    }
    pub fn remove_enchant(
        &mut self,
        arg0: impl Into<crate::bukkit::enchantments::Enchantment<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeEnchant",
            "(Lorg/bukkit/enchantments/Enchantment;)Z",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(res.z().unwrap())
    }
    pub fn has_conflicting_enchant(
        &mut self,
        arg0: impl Into<crate::bukkit::enchantments::Enchantment<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "hasConflictingEnchant",
            "(Lorg/bukkit/enchantments/Enchantment;)Z",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(res.z().unwrap())
    }
    pub fn add_item_flags(
        &mut self,
        _arg0: Vec<impl Into<crate::bukkit::inventory::ItemFlag<'mc>>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.jni_ref().call_method(
            &self.jni_object(),
            "addItemFlags",
            "(Lorg/bukkit/inventory/ItemFlag;)V",
            &[],
        )?;
        Ok(())
    }
    pub fn remove_item_flags(
        &mut self,
        _arg0: Vec<impl Into<crate::bukkit::inventory::ItemFlag<'mc>>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.jni_ref().call_method(
            &self.jni_object(),
            "removeItemFlags",
            "(Lorg/bukkit/inventory/ItemFlag;)V",
            &[],
        )?;
        Ok(())
    }
    pub fn has_item_flag(
        &mut self,
        arg0: impl Into<crate::bukkit::inventory::ItemFlag<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "hasItemFlag",
            "(Lorg/bukkit/inventory/ItemFlag;)Z",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(res.z().unwrap())
    }
    pub fn is_unbreakable(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isUnbreakable", "()Z", &[])?;
        Ok(res.z().unwrap())
    }
    pub fn set_unbreakable(&mut self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        // -2
        let val_0 = jni::objects::JValueGen::Bool(arg0.into());
        self.jni_ref().call_method(
            &self.jni_object(),
            "setUnbreakable",
            "(Z)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn has_attribute_modifiers(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "hasAttributeModifiers", "()Z", &[])?;
        Ok(res.z().unwrap())
    }
    pub fn add_attribute_modifier(
        &mut self,
        arg0: impl Into<crate::bukkit::attribute::Attribute<'mc>>,
        arg1: impl Into<crate::bukkit::attribute::AttributeModifier<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg1.into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "addAttributeModifier",
            "(Lorg/bukkit/attribute/Attribute;Lorg/bukkit/attribute/AttributeModifier;)Z",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        )?;
        Ok(res.z().unwrap())
    }
    pub fn remove_attribute_modifier_with_attribute(
        &mut self,
        arg0: impl Into<crate::bukkit::attribute::Attribute<'mc>>,
        arg1: std::option::Option<impl Into<crate::bukkit::attribute::AttributeModifier<'mc>>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg1.unwrap().into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeAttributeModifier",
            "(Lorg/bukkit/attribute/Attribute;Lorg/bukkit/attribute/AttributeModifier;)Z",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        )?;
        Ok(res.z().unwrap())
    }
    #[deprecated]
    pub fn custom_tag_container(
        &mut self,
    ) -> Result<
        crate::bukkit::inventory::meta::tags::CustomItemTagContainer<'mc>,
        Box<dyn std::error::Error>,
    > {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getCustomTagContainer",
            "()Lorg/bukkit/inventory/meta/tags/CustomItemTagContainer;",
            &[],
        )?;
        let ret = {
            crate::bukkit::inventory::meta::tags::CustomItemTagContainer(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    #[deprecated]
    pub fn set_version(&mut self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Int(arg0.into());
        self.jni_ref().call_method(
            &self.jni_object(),
            "setVersion",
            "(I)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn persistent_data_container(
        &mut self,
    ) -> Result<crate::bukkit::persistence::PersistentDataContainer<'mc>, Box<dyn std::error::Error>>
    {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getPersistentDataContainer",
            "()Lorg/bukkit/persistence/PersistentDataContainer;",
            &[],
        )?;
        let ret = {
            crate::bukkit::persistence::PersistentDataContainer(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
}
impl<'mc> crate::JNIRaw<'mc> for EnchantmentStorageMeta<'mc> {
    fn jni_ref(&self) -> crate::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Into<crate::bukkit::inventory::meta::ItemMeta<'mc>> for EnchantmentStorageMeta<'mc> {
    fn into(self) -> crate::bukkit::inventory::meta::ItemMeta<'mc> {
        crate::bukkit::inventory::meta::ItemMeta::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
/// An instantiatable struct that implements KnowledgeBookMeta. Needed for returning it from Java.
pub struct KnowledgeBookMeta<'mc>(
    pub(crate) crate::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> KnowledgeBookMeta<'mc> {
    pub fn from_raw(
        env: &crate::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate KnowledgeBookMeta from null object.").into(),
            );
        }
        let cls = env.jni.borrow().get_object_class(&obj)?;
        let name_raw = env.call_method(cls, "getName", "()Ljava/lang/String;", &[])?;
        let oh = name_raw.l()?.into();
        let what = env.get_string(&oh)?;
        let name = what.to_string_lossy();
        if !name.ends_with("KnowledgeBookMeta") {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a KnowledgeBookMeta object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    pub fn set_recipes(
        &mut self,
        arg0: Vec<impl Into<crate::bukkit::NamespacedKey<'mc>>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let raw_val_0 = self
            .jni_ref()
            .new_object("java/util/ArrayList", "()V", &[])
            .unwrap();
        for v in arg0 {
            let map_val_0 = unsafe { jni::objects::JObject::from_raw(v.into().1.clone()) };
            self.jni_ref().call_method(
                &raw_val_0,
                "add",
                "(Ljava/Lang/Object)V",
                &[jni::objects::JValueGen::from(&map_val_0)],
            )?;
        }
        let val_0 = jni::objects::JValueGen::Object(raw_val_0);
        self.jni_ref().call_method(
            &self.jni_object(),
            "setRecipes",
            "(Ljava/util/List;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn add_recipe(
        &mut self,
        _arg0: Vec<impl Into<crate::bukkit::NamespacedKey<'mc>>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.jni_ref().call_method(
            &self.jni_object(),
            "addRecipe",
            "(Lorg/bukkit/NamespacedKey;)V",
            &[],
        )?;
        Ok(())
    }
    pub fn has_recipes(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasRecipes", "()Z", &[])?;
        Ok(res.z().unwrap())
    }
    pub fn clone(&mut self) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "clone", "()Ljava/lang/Object;", &[])?;
        Ok(res.l().unwrap())
    }
    pub fn display_name(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getDisplayName",
            "()Ljava/lang/String;",
            &[],
        )?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    pub fn as_string(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getAsString",
            "()Ljava/lang/String;",
            &[],
        )?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    pub fn has_display_name(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasDisplayName", "()Z", &[])?;
        Ok(res.z().unwrap())
    }
    pub fn set_display_name(&mut self, arg0: String) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JObject::from(self.jni_ref().new_string(arg0).unwrap());
        self.jni_ref().call_method(
            &self.jni_object(),
            "setDisplayName",
            "(Ljava/lang/String;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn has_localized_name(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasLocalizedName", "()Z", &[])?;
        Ok(res.z().unwrap())
    }
    pub fn localized_name(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getLocalizedName",
            "()Ljava/lang/String;",
            &[],
        )?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    pub fn set_localized_name(&mut self, arg0: String) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JObject::from(self.jni_ref().new_string(arg0).unwrap());
        self.jni_ref().call_method(
            &self.jni_object(),
            "setLocalizedName",
            "(Ljava/lang/String;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn has_lore(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasLore", "()Z", &[])?;
        Ok(res.z().unwrap())
    }
    pub fn set_lore(&mut self, arg0: Vec<String>) -> Result<(), Box<dyn std::error::Error>> {
        let raw_val_0 = self
            .jni_ref()
            .new_object("java/util/ArrayList", "()V", &[])
            .unwrap();
        for v in arg0 {
            let map_val_0 = jni::objects::JObject::from(self.jni_ref().new_string(v).unwrap());
            self.jni_ref().call_method(
                &raw_val_0,
                "add",
                "(Ljava/Lang/Object)V",
                &[jni::objects::JValueGen::from(&map_val_0)],
            )?;
        }
        let val_0 = jni::objects::JValueGen::Object(raw_val_0);
        self.jni_ref().call_method(
            &self.jni_object(),
            "setLore",
            "(Ljava/util/List;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn has_custom_model_data(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "hasCustomModelData", "()Z", &[])?;
        Ok(res.z().unwrap())
    }
    pub fn custom_model_data(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getCustomModelData", "()I", &[])?;
        Ok(res.i().unwrap())
    }
    pub fn set_custom_model_data(&mut self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Int(arg0.into());
        self.jni_ref().call_method(
            &self.jni_object(),
            "setCustomModelData",
            "(I)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn has_enchants(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasEnchants", "()Z", &[])?;
        Ok(res.z().unwrap())
    }
    pub fn has_enchant(
        &mut self,
        arg0: impl Into<crate::bukkit::enchantments::Enchantment<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "hasEnchant",
            "(Lorg/bukkit/enchantments/Enchantment;)Z",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(res.z().unwrap())
    }
    pub fn get_enchant_level(
        &mut self,
        arg0: impl Into<crate::bukkit::enchantments::Enchantment<'mc>>,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getEnchantLevel",
            "(Lorg/bukkit/enchantments/Enchantment;)I",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(res.i().unwrap())
    }
    pub fn add_enchant(
        &mut self,
        arg0: impl Into<crate::bukkit::enchantments::Enchantment<'mc>>,
        arg1: i32,
        arg2: bool,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let val_1 = jni::objects::JValueGen::Int(arg1.into());
        // -2
        let val_2 = jni::objects::JValueGen::Bool(arg2.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "addEnchant",
            "(Lorg/bukkit/enchantments/Enchantment;IZ)Z",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        )?;
        Ok(res.z().unwrap())
    }
    pub fn remove_enchant(
        &mut self,
        arg0: impl Into<crate::bukkit::enchantments::Enchantment<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeEnchant",
            "(Lorg/bukkit/enchantments/Enchantment;)Z",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(res.z().unwrap())
    }
    pub fn has_conflicting_enchant(
        &mut self,
        arg0: impl Into<crate::bukkit::enchantments::Enchantment<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "hasConflictingEnchant",
            "(Lorg/bukkit/enchantments/Enchantment;)Z",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(res.z().unwrap())
    }
    pub fn add_item_flags(
        &mut self,
        _arg0: Vec<impl Into<crate::bukkit::inventory::ItemFlag<'mc>>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.jni_ref().call_method(
            &self.jni_object(),
            "addItemFlags",
            "(Lorg/bukkit/inventory/ItemFlag;)V",
            &[],
        )?;
        Ok(())
    }
    pub fn remove_item_flags(
        &mut self,
        _arg0: Vec<impl Into<crate::bukkit::inventory::ItemFlag<'mc>>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.jni_ref().call_method(
            &self.jni_object(),
            "removeItemFlags",
            "(Lorg/bukkit/inventory/ItemFlag;)V",
            &[],
        )?;
        Ok(())
    }
    pub fn has_item_flag(
        &mut self,
        arg0: impl Into<crate::bukkit::inventory::ItemFlag<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "hasItemFlag",
            "(Lorg/bukkit/inventory/ItemFlag;)Z",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(res.z().unwrap())
    }
    pub fn is_unbreakable(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isUnbreakable", "()Z", &[])?;
        Ok(res.z().unwrap())
    }
    pub fn set_unbreakable(&mut self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        // -2
        let val_0 = jni::objects::JValueGen::Bool(arg0.into());
        self.jni_ref().call_method(
            &self.jni_object(),
            "setUnbreakable",
            "(Z)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn has_attribute_modifiers(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "hasAttributeModifiers", "()Z", &[])?;
        Ok(res.z().unwrap())
    }
    pub fn add_attribute_modifier(
        &mut self,
        arg0: impl Into<crate::bukkit::attribute::Attribute<'mc>>,
        arg1: impl Into<crate::bukkit::attribute::AttributeModifier<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg1.into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "addAttributeModifier",
            "(Lorg/bukkit/attribute/Attribute;Lorg/bukkit/attribute/AttributeModifier;)Z",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        )?;
        Ok(res.z().unwrap())
    }
    pub fn remove_attribute_modifier_with_attribute(
        &mut self,
        arg0: impl Into<crate::bukkit::attribute::Attribute<'mc>>,
        arg1: std::option::Option<impl Into<crate::bukkit::attribute::AttributeModifier<'mc>>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg1.unwrap().into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeAttributeModifier",
            "(Lorg/bukkit/attribute/Attribute;Lorg/bukkit/attribute/AttributeModifier;)Z",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        )?;
        Ok(res.z().unwrap())
    }
    #[deprecated]
    pub fn custom_tag_container(
        &mut self,
    ) -> Result<
        crate::bukkit::inventory::meta::tags::CustomItemTagContainer<'mc>,
        Box<dyn std::error::Error>,
    > {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getCustomTagContainer",
            "()Lorg/bukkit/inventory/meta/tags/CustomItemTagContainer;",
            &[],
        )?;
        let ret = {
            crate::bukkit::inventory::meta::tags::CustomItemTagContainer(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    #[deprecated]
    pub fn set_version(&mut self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Int(arg0.into());
        self.jni_ref().call_method(
            &self.jni_object(),
            "setVersion",
            "(I)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn persistent_data_container(
        &mut self,
    ) -> Result<crate::bukkit::persistence::PersistentDataContainer<'mc>, Box<dyn std::error::Error>>
    {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getPersistentDataContainer",
            "()Lorg/bukkit/persistence/PersistentDataContainer;",
            &[],
        )?;
        let ret = {
            crate::bukkit::persistence::PersistentDataContainer(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
}
impl<'mc> crate::JNIRaw<'mc> for KnowledgeBookMeta<'mc> {
    fn jni_ref(&self) -> crate::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Into<crate::bukkit::inventory::meta::ItemMeta<'mc>> for KnowledgeBookMeta<'mc> {
    fn into(self) -> crate::bukkit::inventory::meta::ItemMeta<'mc> {
        crate::bukkit::inventory::meta::ItemMeta::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
/// An instantiatable struct that implements MapMeta. Needed for returning it from Java.
pub struct MapMeta<'mc>(
    pub(crate) crate::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> MapMeta<'mc> {
    pub fn from_raw(
        env: &crate::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate MapMeta from null object.").into());
        }
        let cls = env.jni.borrow().get_object_class(&obj)?;
        let name_raw = env.call_method(cls, "getName", "()Ljava/lang/String;", &[])?;
        let oh = name_raw.l()?.into();
        let what = env.get_string(&oh)?;
        let name = what.to_string_lossy();
        if !name.ends_with("MapMeta") {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a MapMeta object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    pub fn set_color(
        &mut self,
        arg0: impl Into<crate::bukkit::Color<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        self.jni_ref().call_method(
            &self.jni_object(),
            "setColor",
            "(Lorg/bukkit/Color;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn color(&mut self) -> Result<crate::bukkit::Color<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getColor",
            "()Lorg/bukkit/Color;",
            &[],
        )?;
        let ret = {
            crate::bukkit::Color(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn has_color(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasColor", "()Z", &[])?;
        Ok(res.z().unwrap())
    }
    pub fn map_view(
        &mut self,
    ) -> Result<crate::bukkit::map::MapView<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getMapView",
            "()Lorg/bukkit/map/MapView;",
            &[],
        )?;
        let ret = {
            crate::bukkit::map::MapView(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    #[deprecated]
    pub fn has_map_id(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasMapId", "()Z", &[])?;
        Ok(res.z().unwrap())
    }
    #[deprecated]
    pub fn map_id(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getMapId", "()I", &[])?;
        Ok(res.i().unwrap())
    }
    #[deprecated]
    pub fn set_map_id(&mut self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Int(arg0.into());
        self.jni_ref().call_method(
            &self.jni_object(),
            "setMapId",
            "(I)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn has_map_view(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasMapView", "()Z", &[])?;
        Ok(res.z().unwrap())
    }
    pub fn set_map_view(
        &mut self,
        arg0: impl Into<crate::bukkit::map::MapView<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        self.jni_ref().call_method(
            &self.jni_object(),
            "setMapView",
            "(Lorg/bukkit/map/MapView;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn is_scaling(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isScaling", "()Z", &[])?;
        Ok(res.z().unwrap())
    }
    pub fn set_scaling(&mut self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        // -2
        let val_0 = jni::objects::JValueGen::Bool(arg0.into());
        self.jni_ref().call_method(
            &self.jni_object(),
            "setScaling",
            "(Z)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    #[deprecated]
    pub fn has_location_name(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasLocationName", "()Z", &[])?;
        Ok(res.z().unwrap())
    }
    #[deprecated]
    pub fn location_name(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getLocationName",
            "()Ljava/lang/String;",
            &[],
        )?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    #[deprecated]
    pub fn set_location_name(&mut self, arg0: String) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JObject::from(self.jni_ref().new_string(arg0).unwrap());
        self.jni_ref().call_method(
            &self.jni_object(),
            "setLocationName",
            "(Ljava/lang/String;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn clone(
        &mut self,
    ) -> Result<crate::bukkit::inventory::meta::MapMeta<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "clone",
            "()Lorg/bukkit/inventory/meta/MapMeta;",
            &[],
        )?;
        let ret = {
            crate::bukkit::inventory::meta::MapMeta(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn display_name(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getDisplayName",
            "()Ljava/lang/String;",
            &[],
        )?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    pub fn as_string(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getAsString",
            "()Ljava/lang/String;",
            &[],
        )?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    pub fn has_display_name(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasDisplayName", "()Z", &[])?;
        Ok(res.z().unwrap())
    }
    pub fn set_display_name(&mut self, arg0: String) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JObject::from(self.jni_ref().new_string(arg0).unwrap());
        self.jni_ref().call_method(
            &self.jni_object(),
            "setDisplayName",
            "(Ljava/lang/String;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn has_localized_name(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasLocalizedName", "()Z", &[])?;
        Ok(res.z().unwrap())
    }
    pub fn localized_name(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getLocalizedName",
            "()Ljava/lang/String;",
            &[],
        )?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    pub fn set_localized_name(&mut self, arg0: String) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JObject::from(self.jni_ref().new_string(arg0).unwrap());
        self.jni_ref().call_method(
            &self.jni_object(),
            "setLocalizedName",
            "(Ljava/lang/String;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn has_lore(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasLore", "()Z", &[])?;
        Ok(res.z().unwrap())
    }
    pub fn set_lore(&mut self, arg0: Vec<String>) -> Result<(), Box<dyn std::error::Error>> {
        let raw_val_0 = self
            .jni_ref()
            .new_object("java/util/ArrayList", "()V", &[])
            .unwrap();
        for v in arg0 {
            let map_val_0 = jni::objects::JObject::from(self.jni_ref().new_string(v).unwrap());
            self.jni_ref().call_method(
                &raw_val_0,
                "add",
                "(Ljava/Lang/Object)V",
                &[jni::objects::JValueGen::from(&map_val_0)],
            )?;
        }
        let val_0 = jni::objects::JValueGen::Object(raw_val_0);
        self.jni_ref().call_method(
            &self.jni_object(),
            "setLore",
            "(Ljava/util/List;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn has_custom_model_data(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "hasCustomModelData", "()Z", &[])?;
        Ok(res.z().unwrap())
    }
    pub fn custom_model_data(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getCustomModelData", "()I", &[])?;
        Ok(res.i().unwrap())
    }
    pub fn set_custom_model_data(&mut self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Int(arg0.into());
        self.jni_ref().call_method(
            &self.jni_object(),
            "setCustomModelData",
            "(I)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn has_enchants(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasEnchants", "()Z", &[])?;
        Ok(res.z().unwrap())
    }
    pub fn has_enchant(
        &mut self,
        arg0: impl Into<crate::bukkit::enchantments::Enchantment<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "hasEnchant",
            "(Lorg/bukkit/enchantments/Enchantment;)Z",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(res.z().unwrap())
    }
    pub fn get_enchant_level(
        &mut self,
        arg0: impl Into<crate::bukkit::enchantments::Enchantment<'mc>>,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getEnchantLevel",
            "(Lorg/bukkit/enchantments/Enchantment;)I",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(res.i().unwrap())
    }
    pub fn add_enchant(
        &mut self,
        arg0: impl Into<crate::bukkit::enchantments::Enchantment<'mc>>,
        arg1: i32,
        arg2: bool,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let val_1 = jni::objects::JValueGen::Int(arg1.into());
        // -2
        let val_2 = jni::objects::JValueGen::Bool(arg2.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "addEnchant",
            "(Lorg/bukkit/enchantments/Enchantment;IZ)Z",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        )?;
        Ok(res.z().unwrap())
    }
    pub fn remove_enchant(
        &mut self,
        arg0: impl Into<crate::bukkit::enchantments::Enchantment<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeEnchant",
            "(Lorg/bukkit/enchantments/Enchantment;)Z",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(res.z().unwrap())
    }
    pub fn has_conflicting_enchant(
        &mut self,
        arg0: impl Into<crate::bukkit::enchantments::Enchantment<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "hasConflictingEnchant",
            "(Lorg/bukkit/enchantments/Enchantment;)Z",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(res.z().unwrap())
    }
    pub fn add_item_flags(
        &mut self,
        _arg0: Vec<impl Into<crate::bukkit::inventory::ItemFlag<'mc>>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.jni_ref().call_method(
            &self.jni_object(),
            "addItemFlags",
            "(Lorg/bukkit/inventory/ItemFlag;)V",
            &[],
        )?;
        Ok(())
    }
    pub fn remove_item_flags(
        &mut self,
        _arg0: Vec<impl Into<crate::bukkit::inventory::ItemFlag<'mc>>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.jni_ref().call_method(
            &self.jni_object(),
            "removeItemFlags",
            "(Lorg/bukkit/inventory/ItemFlag;)V",
            &[],
        )?;
        Ok(())
    }
    pub fn has_item_flag(
        &mut self,
        arg0: impl Into<crate::bukkit::inventory::ItemFlag<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "hasItemFlag",
            "(Lorg/bukkit/inventory/ItemFlag;)Z",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(res.z().unwrap())
    }
    pub fn is_unbreakable(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isUnbreakable", "()Z", &[])?;
        Ok(res.z().unwrap())
    }
    pub fn set_unbreakable(&mut self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        // -2
        let val_0 = jni::objects::JValueGen::Bool(arg0.into());
        self.jni_ref().call_method(
            &self.jni_object(),
            "setUnbreakable",
            "(Z)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn has_attribute_modifiers(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "hasAttributeModifiers", "()Z", &[])?;
        Ok(res.z().unwrap())
    }
    pub fn add_attribute_modifier(
        &mut self,
        arg0: impl Into<crate::bukkit::attribute::Attribute<'mc>>,
        arg1: impl Into<crate::bukkit::attribute::AttributeModifier<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg1.into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "addAttributeModifier",
            "(Lorg/bukkit/attribute/Attribute;Lorg/bukkit/attribute/AttributeModifier;)Z",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        )?;
        Ok(res.z().unwrap())
    }
    pub fn remove_attribute_modifier_with_attribute(
        &mut self,
        arg0: impl Into<crate::bukkit::attribute::Attribute<'mc>>,
        arg1: std::option::Option<impl Into<crate::bukkit::attribute::AttributeModifier<'mc>>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg1.unwrap().into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeAttributeModifier",
            "(Lorg/bukkit/attribute/Attribute;Lorg/bukkit/attribute/AttributeModifier;)Z",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        )?;
        Ok(res.z().unwrap())
    }
    #[deprecated]
    pub fn custom_tag_container(
        &mut self,
    ) -> Result<
        crate::bukkit::inventory::meta::tags::CustomItemTagContainer<'mc>,
        Box<dyn std::error::Error>,
    > {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getCustomTagContainer",
            "()Lorg/bukkit/inventory/meta/tags/CustomItemTagContainer;",
            &[],
        )?;
        let ret = {
            crate::bukkit::inventory::meta::tags::CustomItemTagContainer(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    #[deprecated]
    pub fn set_version(&mut self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Int(arg0.into());
        self.jni_ref().call_method(
            &self.jni_object(),
            "setVersion",
            "(I)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn persistent_data_container(
        &mut self,
    ) -> Result<crate::bukkit::persistence::PersistentDataContainer<'mc>, Box<dyn std::error::Error>>
    {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getPersistentDataContainer",
            "()Lorg/bukkit/persistence/PersistentDataContainer;",
            &[],
        )?;
        let ret = {
            crate::bukkit::persistence::PersistentDataContainer(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
}
impl<'mc> crate::JNIRaw<'mc> for MapMeta<'mc> {
    fn jni_ref(&self) -> crate::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Into<crate::bukkit::inventory::meta::ItemMeta<'mc>> for MapMeta<'mc> {
    fn into(self) -> crate::bukkit::inventory::meta::ItemMeta<'mc> {
        crate::bukkit::inventory::meta::ItemMeta::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
pub struct BookMetaSpigot<'mc>(
    pub(crate) crate::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> crate::JNIRaw<'mc> for BookMetaSpigot<'mc> {
    fn jni_ref(&self) -> crate::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> BookMetaSpigot<'mc> {
    pub fn new(
        jni: crate::SharedJNIEnv<'mc>,
    ) -> Result<crate::bukkit::inventory::meta::BookMetaSpigot<'mc>, Box<dyn std::error::Error>>
    {
        let cls = &jni.find_class("org/bukkit/inventory/meta/BookMeta$Spigot")?;
        let res = jni.new_object(cls, "()V", &[])?;
        let ret = { crate::bukkit::inventory::meta::BookMetaSpigot(jni, res) };
        Ok(ret)
    }
    pub fn from_raw(
        env: &crate::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate BookMetaSpigot from null object.").into(),
            );
        }
        let cls = env.jni.borrow().get_object_class(&obj)?;
        let name_raw = env.call_method(cls, "getName", "()Ljava/lang/String;", &[])?;
        let oh = name_raw.l()?.into();
        let what = env.get_string(&oh)?;
        let name = what.to_string_lossy();
        if !name.ends_with("BookMetaSpigot") {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a BookMetaSpigot object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    pub fn set_page(
        &mut self,
        arg0: i32,
        _arg1: Vec<impl Into<crate::bungee::bungee::api::chat::BaseComponent<'mc>>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Int(arg0.into());
        self.jni_ref().call_method(
            &self.jni_object(),
            "setPage",
            "(ILnet/md_5/bungee/api/chat/BaseComponent;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn set_pages_with_base_componentss(
        &mut self,
        arg0: std::option::Option<
            Vec<impl Into<crate::bungee::bungee::api::chat::BaseComponent<'mc>>>,
        >,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let raw_val_0 = self
            .jni_ref()
            .new_object("java/util/ArrayList", "()V", &[])
            .unwrap();
        for v in arg0.unwrap() {
            let map_val_0 = unsafe { jni::objects::JObject::from_raw(v.into().1.clone()) };
            self.jni_ref().call_method(
                &raw_val_0,
                "add",
                "(Ljava/Lang/Object)V",
                &[jni::objects::JValueGen::from(&map_val_0)],
            )?;
        }
        let val_0 = jni::objects::JValueGen::Object(raw_val_0);
        self.jni_ref().call_method(
            &self.jni_object(),
            "setPages",
            "(Ljava/util/List;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn add_page(
        &mut self,
        _arg0: Vec<impl Into<crate::bungee::bungee::api::chat::BaseComponent<'mc>>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.jni_ref().call_method(
            &self.jni_object(),
            "addPage",
            "(Lnet/md_5/bungee/api/chat/BaseComponent;)V",
            &[],
        )?;
        Ok(())
    }
    pub fn wait(
        &mut self,
        arg0: std::option::Option<i64>,
        arg1: std::option::Option<i32>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Long(arg0.unwrap().into());
        let val_1 = jni::objects::JValueGen::Int(arg1.unwrap().into());
        self.jni_ref().call_method(
            &self.jni_object(),
            "wait",
            "(JI)V",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        )?;
        Ok(())
    }
    pub fn equals(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "equals",
            "(Ljava/lang/Object;)Z",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(res.z().unwrap())
    }
    pub fn to_string(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "toString",
            "()Ljava/lang/String;",
            &[],
        )?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    pub fn hash_code(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hashCode", "()I", &[])?;
        Ok(res.i().unwrap())
    }
    pub fn class(&mut self) -> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getClass",
            "()Ljava/lang/Class;",
            &[],
        )?;
        Ok(unsafe { jni::objects::JClass::from_raw(res.as_jni().l) })
    }
    pub fn notify(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.jni_ref()
            .call_method(&self.jni_object(), "notify", "()V", &[])?;
        Ok(())
    }
    pub fn notify_all(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.jni_ref()
            .call_method(&self.jni_object(), "notifyAll", "()V", &[])?;
        Ok(())
    }
}
/// An instantiatable struct that implements LeatherArmorMeta. Needed for returning it from Java.
pub struct LeatherArmorMeta<'mc>(
    pub(crate) crate::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> LeatherArmorMeta<'mc> {
    pub fn from_raw(
        env: &crate::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate LeatherArmorMeta from null object.").into(),
            );
        }
        let cls = env.jni.borrow().get_object_class(&obj)?;
        let name_raw = env.call_method(cls, "getName", "()Ljava/lang/String;", &[])?;
        let oh = name_raw.l()?.into();
        let what = env.get_string(&oh)?;
        let name = what.to_string_lossy();
        if !name.ends_with("LeatherArmorMeta") {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a LeatherArmorMeta object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    pub fn set_color(
        &mut self,
        arg0: impl Into<crate::bukkit::Color<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        self.jni_ref().call_method(
            &self.jni_object(),
            "setColor",
            "(Lorg/bukkit/Color;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn color(&mut self) -> Result<crate::bukkit::Color<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getColor",
            "()Lorg/bukkit/Color;",
            &[],
        )?;
        let ret = {
            crate::bukkit::Color(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn clone(
        &mut self,
    ) -> Result<crate::bukkit::inventory::meta::LeatherArmorMeta<'mc>, Box<dyn std::error::Error>>
    {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "clone",
            "()Lorg/bukkit/inventory/meta/LeatherArmorMeta;",
            &[],
        )?;
        let ret = {
            crate::bukkit::inventory::meta::LeatherArmorMeta(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn display_name(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getDisplayName",
            "()Ljava/lang/String;",
            &[],
        )?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    pub fn as_string(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getAsString",
            "()Ljava/lang/String;",
            &[],
        )?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    pub fn has_display_name(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasDisplayName", "()Z", &[])?;
        Ok(res.z().unwrap())
    }
    pub fn set_display_name(&mut self, arg0: String) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JObject::from(self.jni_ref().new_string(arg0).unwrap());
        self.jni_ref().call_method(
            &self.jni_object(),
            "setDisplayName",
            "(Ljava/lang/String;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn has_localized_name(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasLocalizedName", "()Z", &[])?;
        Ok(res.z().unwrap())
    }
    pub fn localized_name(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getLocalizedName",
            "()Ljava/lang/String;",
            &[],
        )?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    pub fn set_localized_name(&mut self, arg0: String) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JObject::from(self.jni_ref().new_string(arg0).unwrap());
        self.jni_ref().call_method(
            &self.jni_object(),
            "setLocalizedName",
            "(Ljava/lang/String;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn has_lore(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasLore", "()Z", &[])?;
        Ok(res.z().unwrap())
    }
    pub fn set_lore(&mut self, arg0: Vec<String>) -> Result<(), Box<dyn std::error::Error>> {
        let raw_val_0 = self
            .jni_ref()
            .new_object("java/util/ArrayList", "()V", &[])
            .unwrap();
        for v in arg0 {
            let map_val_0 = jni::objects::JObject::from(self.jni_ref().new_string(v).unwrap());
            self.jni_ref().call_method(
                &raw_val_0,
                "add",
                "(Ljava/Lang/Object)V",
                &[jni::objects::JValueGen::from(&map_val_0)],
            )?;
        }
        let val_0 = jni::objects::JValueGen::Object(raw_val_0);
        self.jni_ref().call_method(
            &self.jni_object(),
            "setLore",
            "(Ljava/util/List;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn has_custom_model_data(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "hasCustomModelData", "()Z", &[])?;
        Ok(res.z().unwrap())
    }
    pub fn custom_model_data(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getCustomModelData", "()I", &[])?;
        Ok(res.i().unwrap())
    }
    pub fn set_custom_model_data(&mut self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Int(arg0.into());
        self.jni_ref().call_method(
            &self.jni_object(),
            "setCustomModelData",
            "(I)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn has_enchants(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasEnchants", "()Z", &[])?;
        Ok(res.z().unwrap())
    }
    pub fn has_enchant(
        &mut self,
        arg0: impl Into<crate::bukkit::enchantments::Enchantment<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "hasEnchant",
            "(Lorg/bukkit/enchantments/Enchantment;)Z",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(res.z().unwrap())
    }
    pub fn get_enchant_level(
        &mut self,
        arg0: impl Into<crate::bukkit::enchantments::Enchantment<'mc>>,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getEnchantLevel",
            "(Lorg/bukkit/enchantments/Enchantment;)I",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(res.i().unwrap())
    }
    pub fn add_enchant(
        &mut self,
        arg0: impl Into<crate::bukkit::enchantments::Enchantment<'mc>>,
        arg1: i32,
        arg2: bool,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let val_1 = jni::objects::JValueGen::Int(arg1.into());
        // -2
        let val_2 = jni::objects::JValueGen::Bool(arg2.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "addEnchant",
            "(Lorg/bukkit/enchantments/Enchantment;IZ)Z",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        )?;
        Ok(res.z().unwrap())
    }
    pub fn remove_enchant(
        &mut self,
        arg0: impl Into<crate::bukkit::enchantments::Enchantment<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeEnchant",
            "(Lorg/bukkit/enchantments/Enchantment;)Z",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(res.z().unwrap())
    }
    pub fn has_conflicting_enchant(
        &mut self,
        arg0: impl Into<crate::bukkit::enchantments::Enchantment<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "hasConflictingEnchant",
            "(Lorg/bukkit/enchantments/Enchantment;)Z",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(res.z().unwrap())
    }
    pub fn add_item_flags(
        &mut self,
        _arg0: Vec<impl Into<crate::bukkit::inventory::ItemFlag<'mc>>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.jni_ref().call_method(
            &self.jni_object(),
            "addItemFlags",
            "(Lorg/bukkit/inventory/ItemFlag;)V",
            &[],
        )?;
        Ok(())
    }
    pub fn remove_item_flags(
        &mut self,
        _arg0: Vec<impl Into<crate::bukkit::inventory::ItemFlag<'mc>>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.jni_ref().call_method(
            &self.jni_object(),
            "removeItemFlags",
            "(Lorg/bukkit/inventory/ItemFlag;)V",
            &[],
        )?;
        Ok(())
    }
    pub fn has_item_flag(
        &mut self,
        arg0: impl Into<crate::bukkit::inventory::ItemFlag<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "hasItemFlag",
            "(Lorg/bukkit/inventory/ItemFlag;)Z",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(res.z().unwrap())
    }
    pub fn is_unbreakable(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isUnbreakable", "()Z", &[])?;
        Ok(res.z().unwrap())
    }
    pub fn set_unbreakable(&mut self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        // -2
        let val_0 = jni::objects::JValueGen::Bool(arg0.into());
        self.jni_ref().call_method(
            &self.jni_object(),
            "setUnbreakable",
            "(Z)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn has_attribute_modifiers(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "hasAttributeModifiers", "()Z", &[])?;
        Ok(res.z().unwrap())
    }
    pub fn add_attribute_modifier(
        &mut self,
        arg0: impl Into<crate::bukkit::attribute::Attribute<'mc>>,
        arg1: impl Into<crate::bukkit::attribute::AttributeModifier<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg1.into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "addAttributeModifier",
            "(Lorg/bukkit/attribute/Attribute;Lorg/bukkit/attribute/AttributeModifier;)Z",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        )?;
        Ok(res.z().unwrap())
    }
    pub fn remove_attribute_modifier_with_attribute(
        &mut self,
        arg0: impl Into<crate::bukkit::attribute::Attribute<'mc>>,
        arg1: std::option::Option<impl Into<crate::bukkit::attribute::AttributeModifier<'mc>>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg1.unwrap().into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeAttributeModifier",
            "(Lorg/bukkit/attribute/Attribute;Lorg/bukkit/attribute/AttributeModifier;)Z",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        )?;
        Ok(res.z().unwrap())
    }
    #[deprecated]
    pub fn custom_tag_container(
        &mut self,
    ) -> Result<
        crate::bukkit::inventory::meta::tags::CustomItemTagContainer<'mc>,
        Box<dyn std::error::Error>,
    > {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getCustomTagContainer",
            "()Lorg/bukkit/inventory/meta/tags/CustomItemTagContainer;",
            &[],
        )?;
        let ret = {
            crate::bukkit::inventory::meta::tags::CustomItemTagContainer(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    #[deprecated]
    pub fn set_version(&mut self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Int(arg0.into());
        self.jni_ref().call_method(
            &self.jni_object(),
            "setVersion",
            "(I)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn persistent_data_container(
        &mut self,
    ) -> Result<crate::bukkit::persistence::PersistentDataContainer<'mc>, Box<dyn std::error::Error>>
    {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getPersistentDataContainer",
            "()Lorg/bukkit/persistence/PersistentDataContainer;",
            &[],
        )?;
        let ret = {
            crate::bukkit::persistence::PersistentDataContainer(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
}
impl<'mc> crate::JNIRaw<'mc> for LeatherArmorMeta<'mc> {
    fn jni_ref(&self) -> crate::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Into<crate::bukkit::inventory::meta::ItemMeta<'mc>> for LeatherArmorMeta<'mc> {
    fn into(self) -> crate::bukkit::inventory::meta::ItemMeta<'mc> {
        crate::bukkit::inventory::meta::ItemMeta::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
/// An instantiatable struct that implements FireworkEffectMeta. Needed for returning it from Java.
pub struct FireworkEffectMeta<'mc>(
    pub(crate) crate::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> FireworkEffectMeta<'mc> {
    pub fn from_raw(
        env: &crate::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate FireworkEffectMeta from null object.").into(),
            );
        }
        let cls = env.jni.borrow().get_object_class(&obj)?;
        let name_raw = env.call_method(cls, "getName", "()Ljava/lang/String;", &[])?;
        let oh = name_raw.l()?.into();
        let what = env.get_string(&oh)?;
        let name = what.to_string_lossy();
        if !name.ends_with("FireworkEffectMeta") {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a FireworkEffectMeta object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    pub fn set_effect(
        &mut self,
        arg0: impl Into<crate::bukkit::FireworkEffect<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        self.jni_ref().call_method(
            &self.jni_object(),
            "setEffect",
            "(Lorg/bukkit/FireworkEffect;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn has_effect(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasEffect", "()Z", &[])?;
        Ok(res.z().unwrap())
    }
    pub fn effect(
        &mut self,
    ) -> Result<crate::bukkit::FireworkEffect<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getEffect",
            "()Lorg/bukkit/FireworkEffect;",
            &[],
        )?;
        let ret = {
            crate::bukkit::FireworkEffect(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn clone(
        &mut self,
    ) -> Result<crate::bukkit::inventory::meta::FireworkEffectMeta<'mc>, Box<dyn std::error::Error>>
    {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "clone",
            "()Lorg/bukkit/inventory/meta/FireworkEffectMeta;",
            &[],
        )?;
        let ret = {
            crate::bukkit::inventory::meta::FireworkEffectMeta(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn display_name(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getDisplayName",
            "()Ljava/lang/String;",
            &[],
        )?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    pub fn as_string(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getAsString",
            "()Ljava/lang/String;",
            &[],
        )?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    pub fn has_display_name(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasDisplayName", "()Z", &[])?;
        Ok(res.z().unwrap())
    }
    pub fn set_display_name(&mut self, arg0: String) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JObject::from(self.jni_ref().new_string(arg0).unwrap());
        self.jni_ref().call_method(
            &self.jni_object(),
            "setDisplayName",
            "(Ljava/lang/String;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn has_localized_name(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasLocalizedName", "()Z", &[])?;
        Ok(res.z().unwrap())
    }
    pub fn localized_name(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getLocalizedName",
            "()Ljava/lang/String;",
            &[],
        )?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    pub fn set_localized_name(&mut self, arg0: String) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JObject::from(self.jni_ref().new_string(arg0).unwrap());
        self.jni_ref().call_method(
            &self.jni_object(),
            "setLocalizedName",
            "(Ljava/lang/String;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn has_lore(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasLore", "()Z", &[])?;
        Ok(res.z().unwrap())
    }
    pub fn set_lore(&mut self, arg0: Vec<String>) -> Result<(), Box<dyn std::error::Error>> {
        let raw_val_0 = self
            .jni_ref()
            .new_object("java/util/ArrayList", "()V", &[])
            .unwrap();
        for v in arg0 {
            let map_val_0 = jni::objects::JObject::from(self.jni_ref().new_string(v).unwrap());
            self.jni_ref().call_method(
                &raw_val_0,
                "add",
                "(Ljava/Lang/Object)V",
                &[jni::objects::JValueGen::from(&map_val_0)],
            )?;
        }
        let val_0 = jni::objects::JValueGen::Object(raw_val_0);
        self.jni_ref().call_method(
            &self.jni_object(),
            "setLore",
            "(Ljava/util/List;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn has_custom_model_data(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "hasCustomModelData", "()Z", &[])?;
        Ok(res.z().unwrap())
    }
    pub fn custom_model_data(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getCustomModelData", "()I", &[])?;
        Ok(res.i().unwrap())
    }
    pub fn set_custom_model_data(&mut self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Int(arg0.into());
        self.jni_ref().call_method(
            &self.jni_object(),
            "setCustomModelData",
            "(I)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn has_enchants(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasEnchants", "()Z", &[])?;
        Ok(res.z().unwrap())
    }
    pub fn has_enchant(
        &mut self,
        arg0: impl Into<crate::bukkit::enchantments::Enchantment<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "hasEnchant",
            "(Lorg/bukkit/enchantments/Enchantment;)Z",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(res.z().unwrap())
    }
    pub fn get_enchant_level(
        &mut self,
        arg0: impl Into<crate::bukkit::enchantments::Enchantment<'mc>>,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getEnchantLevel",
            "(Lorg/bukkit/enchantments/Enchantment;)I",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(res.i().unwrap())
    }
    pub fn add_enchant(
        &mut self,
        arg0: impl Into<crate::bukkit::enchantments::Enchantment<'mc>>,
        arg1: i32,
        arg2: bool,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let val_1 = jni::objects::JValueGen::Int(arg1.into());
        // -2
        let val_2 = jni::objects::JValueGen::Bool(arg2.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "addEnchant",
            "(Lorg/bukkit/enchantments/Enchantment;IZ)Z",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        )?;
        Ok(res.z().unwrap())
    }
    pub fn remove_enchant(
        &mut self,
        arg0: impl Into<crate::bukkit::enchantments::Enchantment<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeEnchant",
            "(Lorg/bukkit/enchantments/Enchantment;)Z",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(res.z().unwrap())
    }
    pub fn has_conflicting_enchant(
        &mut self,
        arg0: impl Into<crate::bukkit::enchantments::Enchantment<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "hasConflictingEnchant",
            "(Lorg/bukkit/enchantments/Enchantment;)Z",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(res.z().unwrap())
    }
    pub fn add_item_flags(
        &mut self,
        _arg0: Vec<impl Into<crate::bukkit::inventory::ItemFlag<'mc>>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.jni_ref().call_method(
            &self.jni_object(),
            "addItemFlags",
            "(Lorg/bukkit/inventory/ItemFlag;)V",
            &[],
        )?;
        Ok(())
    }
    pub fn remove_item_flags(
        &mut self,
        _arg0: Vec<impl Into<crate::bukkit::inventory::ItemFlag<'mc>>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.jni_ref().call_method(
            &self.jni_object(),
            "removeItemFlags",
            "(Lorg/bukkit/inventory/ItemFlag;)V",
            &[],
        )?;
        Ok(())
    }
    pub fn has_item_flag(
        &mut self,
        arg0: impl Into<crate::bukkit::inventory::ItemFlag<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "hasItemFlag",
            "(Lorg/bukkit/inventory/ItemFlag;)Z",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(res.z().unwrap())
    }
    pub fn is_unbreakable(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isUnbreakable", "()Z", &[])?;
        Ok(res.z().unwrap())
    }
    pub fn set_unbreakable(&mut self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        // -2
        let val_0 = jni::objects::JValueGen::Bool(arg0.into());
        self.jni_ref().call_method(
            &self.jni_object(),
            "setUnbreakable",
            "(Z)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn has_attribute_modifiers(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "hasAttributeModifiers", "()Z", &[])?;
        Ok(res.z().unwrap())
    }
    pub fn add_attribute_modifier(
        &mut self,
        arg0: impl Into<crate::bukkit::attribute::Attribute<'mc>>,
        arg1: impl Into<crate::bukkit::attribute::AttributeModifier<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg1.into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "addAttributeModifier",
            "(Lorg/bukkit/attribute/Attribute;Lorg/bukkit/attribute/AttributeModifier;)Z",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        )?;
        Ok(res.z().unwrap())
    }
    pub fn remove_attribute_modifier_with_attribute(
        &mut self,
        arg0: impl Into<crate::bukkit::attribute::Attribute<'mc>>,
        arg1: std::option::Option<impl Into<crate::bukkit::attribute::AttributeModifier<'mc>>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg1.unwrap().into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeAttributeModifier",
            "(Lorg/bukkit/attribute/Attribute;Lorg/bukkit/attribute/AttributeModifier;)Z",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        )?;
        Ok(res.z().unwrap())
    }
    #[deprecated]
    pub fn custom_tag_container(
        &mut self,
    ) -> Result<
        crate::bukkit::inventory::meta::tags::CustomItemTagContainer<'mc>,
        Box<dyn std::error::Error>,
    > {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getCustomTagContainer",
            "()Lorg/bukkit/inventory/meta/tags/CustomItemTagContainer;",
            &[],
        )?;
        let ret = {
            crate::bukkit::inventory::meta::tags::CustomItemTagContainer(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    #[deprecated]
    pub fn set_version(&mut self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Int(arg0.into());
        self.jni_ref().call_method(
            &self.jni_object(),
            "setVersion",
            "(I)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn persistent_data_container(
        &mut self,
    ) -> Result<crate::bukkit::persistence::PersistentDataContainer<'mc>, Box<dyn std::error::Error>>
    {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getPersistentDataContainer",
            "()Lorg/bukkit/persistence/PersistentDataContainer;",
            &[],
        )?;
        let ret = {
            crate::bukkit::persistence::PersistentDataContainer(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
}
impl<'mc> crate::JNIRaw<'mc> for FireworkEffectMeta<'mc> {
    fn jni_ref(&self) -> crate::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Into<crate::bukkit::inventory::meta::ItemMeta<'mc>> for FireworkEffectMeta<'mc> {
    fn into(self) -> crate::bukkit::inventory::meta::ItemMeta<'mc> {
        crate::bukkit::inventory::meta::ItemMeta::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
/// An instantiatable struct that implements TropicalFishBucketMeta. Needed for returning it from Java.
pub struct TropicalFishBucketMeta<'mc>(
    pub(crate) crate::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> TropicalFishBucketMeta<'mc> {
    pub fn from_raw(
        env: &crate::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate TropicalFishBucketMeta from null object."
            )
            .into());
        }
        let cls = env.jni.borrow().get_object_class(&obj)?;
        let name_raw = env.call_method(cls, "getName", "()Ljava/lang/String;", &[])?;
        let oh = name_raw.l()?.into();
        let what = env.get_string(&oh)?;
        let name = what.to_string_lossy();
        if !name.ends_with("TropicalFishBucketMeta") {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a TropicalFishBucketMeta object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    pub fn pattern(
        &mut self,
    ) -> Result<crate::bukkit::entity::TropicalFishPattern<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getPattern",
            "()Lorg/bukkit/entity/TropicalFish$Pattern;",
            &[],
        )?;
        let ret = {
            crate::bukkit::entity::TropicalFishPattern(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn set_pattern(
        &mut self,
        arg0: impl Into<crate::bukkit::entity::TropicalFishPattern<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        self.jni_ref().call_method(
            &self.jni_object(),
            "setPattern",
            "(Lorg/bukkit/entity/TropicalFish$Pattern;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn pattern_color(
        &mut self,
    ) -> Result<crate::bukkit::DyeColor<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getPatternColor",
            "()Lorg/bukkit/DyeColor;",
            &[],
        )?;
        let ret = {
            let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
            let variant =
                self.jni_ref()
                    .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
            let variant_str = self
                .jni_ref()
                .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
                .to_string_lossy()
                .to_string();
            crate::bukkit::DyeColor(
                self.jni_ref(),
                raw_obj,
                crate::bukkit::DyeColor::from_string(variant_str).unwrap(),
            )
        };
        Ok(ret)
    }
    pub fn set_pattern_color(
        &mut self,
        arg0: impl Into<crate::bukkit::DyeColor<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        self.jni_ref().call_method(
            &self.jni_object(),
            "setPatternColor",
            "(Lorg/bukkit/DyeColor;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn body_color(
        &mut self,
    ) -> Result<crate::bukkit::DyeColor<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getBodyColor",
            "()Lorg/bukkit/DyeColor;",
            &[],
        )?;
        let ret = {
            let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
            let variant =
                self.jni_ref()
                    .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
            let variant_str = self
                .jni_ref()
                .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
                .to_string_lossy()
                .to_string();
            crate::bukkit::DyeColor(
                self.jni_ref(),
                raw_obj,
                crate::bukkit::DyeColor::from_string(variant_str).unwrap(),
            )
        };
        Ok(ret)
    }
    pub fn set_body_color(
        &mut self,
        arg0: impl Into<crate::bukkit::DyeColor<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        self.jni_ref().call_method(
            &self.jni_object(),
            "setBodyColor",
            "(Lorg/bukkit/DyeColor;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn has_variant(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasVariant", "()Z", &[])?;
        Ok(res.z().unwrap())
    }
    pub fn clone(&mut self) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "clone", "()Ljava/lang/Object;", &[])?;
        Ok(res.l().unwrap())
    }
    pub fn display_name(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getDisplayName",
            "()Ljava/lang/String;",
            &[],
        )?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    pub fn as_string(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getAsString",
            "()Ljava/lang/String;",
            &[],
        )?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    pub fn has_display_name(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasDisplayName", "()Z", &[])?;
        Ok(res.z().unwrap())
    }
    pub fn set_display_name(&mut self, arg0: String) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JObject::from(self.jni_ref().new_string(arg0).unwrap());
        self.jni_ref().call_method(
            &self.jni_object(),
            "setDisplayName",
            "(Ljava/lang/String;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn has_localized_name(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasLocalizedName", "()Z", &[])?;
        Ok(res.z().unwrap())
    }
    pub fn localized_name(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getLocalizedName",
            "()Ljava/lang/String;",
            &[],
        )?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    pub fn set_localized_name(&mut self, arg0: String) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JObject::from(self.jni_ref().new_string(arg0).unwrap());
        self.jni_ref().call_method(
            &self.jni_object(),
            "setLocalizedName",
            "(Ljava/lang/String;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn has_lore(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasLore", "()Z", &[])?;
        Ok(res.z().unwrap())
    }
    pub fn set_lore(&mut self, arg0: Vec<String>) -> Result<(), Box<dyn std::error::Error>> {
        let raw_val_0 = self
            .jni_ref()
            .new_object("java/util/ArrayList", "()V", &[])
            .unwrap();
        for v in arg0 {
            let map_val_0 = jni::objects::JObject::from(self.jni_ref().new_string(v).unwrap());
            self.jni_ref().call_method(
                &raw_val_0,
                "add",
                "(Ljava/Lang/Object)V",
                &[jni::objects::JValueGen::from(&map_val_0)],
            )?;
        }
        let val_0 = jni::objects::JValueGen::Object(raw_val_0);
        self.jni_ref().call_method(
            &self.jni_object(),
            "setLore",
            "(Ljava/util/List;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn has_custom_model_data(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "hasCustomModelData", "()Z", &[])?;
        Ok(res.z().unwrap())
    }
    pub fn custom_model_data(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getCustomModelData", "()I", &[])?;
        Ok(res.i().unwrap())
    }
    pub fn set_custom_model_data(&mut self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Int(arg0.into());
        self.jni_ref().call_method(
            &self.jni_object(),
            "setCustomModelData",
            "(I)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn has_enchants(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasEnchants", "()Z", &[])?;
        Ok(res.z().unwrap())
    }
    pub fn has_enchant(
        &mut self,
        arg0: impl Into<crate::bukkit::enchantments::Enchantment<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "hasEnchant",
            "(Lorg/bukkit/enchantments/Enchantment;)Z",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(res.z().unwrap())
    }
    pub fn get_enchant_level(
        &mut self,
        arg0: impl Into<crate::bukkit::enchantments::Enchantment<'mc>>,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getEnchantLevel",
            "(Lorg/bukkit/enchantments/Enchantment;)I",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(res.i().unwrap())
    }
    pub fn add_enchant(
        &mut self,
        arg0: impl Into<crate::bukkit::enchantments::Enchantment<'mc>>,
        arg1: i32,
        arg2: bool,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let val_1 = jni::objects::JValueGen::Int(arg1.into());
        // -2
        let val_2 = jni::objects::JValueGen::Bool(arg2.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "addEnchant",
            "(Lorg/bukkit/enchantments/Enchantment;IZ)Z",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        )?;
        Ok(res.z().unwrap())
    }
    pub fn remove_enchant(
        &mut self,
        arg0: impl Into<crate::bukkit::enchantments::Enchantment<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeEnchant",
            "(Lorg/bukkit/enchantments/Enchantment;)Z",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(res.z().unwrap())
    }
    pub fn has_conflicting_enchant(
        &mut self,
        arg0: impl Into<crate::bukkit::enchantments::Enchantment<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "hasConflictingEnchant",
            "(Lorg/bukkit/enchantments/Enchantment;)Z",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(res.z().unwrap())
    }
    pub fn add_item_flags(
        &mut self,
        _arg0: Vec<impl Into<crate::bukkit::inventory::ItemFlag<'mc>>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.jni_ref().call_method(
            &self.jni_object(),
            "addItemFlags",
            "(Lorg/bukkit/inventory/ItemFlag;)V",
            &[],
        )?;
        Ok(())
    }
    pub fn remove_item_flags(
        &mut self,
        _arg0: Vec<impl Into<crate::bukkit::inventory::ItemFlag<'mc>>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.jni_ref().call_method(
            &self.jni_object(),
            "removeItemFlags",
            "(Lorg/bukkit/inventory/ItemFlag;)V",
            &[],
        )?;
        Ok(())
    }
    pub fn has_item_flag(
        &mut self,
        arg0: impl Into<crate::bukkit::inventory::ItemFlag<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "hasItemFlag",
            "(Lorg/bukkit/inventory/ItemFlag;)Z",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(res.z().unwrap())
    }
    pub fn is_unbreakable(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isUnbreakable", "()Z", &[])?;
        Ok(res.z().unwrap())
    }
    pub fn set_unbreakable(&mut self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        // -2
        let val_0 = jni::objects::JValueGen::Bool(arg0.into());
        self.jni_ref().call_method(
            &self.jni_object(),
            "setUnbreakable",
            "(Z)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn has_attribute_modifiers(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "hasAttributeModifiers", "()Z", &[])?;
        Ok(res.z().unwrap())
    }
    pub fn add_attribute_modifier(
        &mut self,
        arg0: impl Into<crate::bukkit::attribute::Attribute<'mc>>,
        arg1: impl Into<crate::bukkit::attribute::AttributeModifier<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg1.into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "addAttributeModifier",
            "(Lorg/bukkit/attribute/Attribute;Lorg/bukkit/attribute/AttributeModifier;)Z",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        )?;
        Ok(res.z().unwrap())
    }
    pub fn remove_attribute_modifier_with_attribute(
        &mut self,
        arg0: impl Into<crate::bukkit::attribute::Attribute<'mc>>,
        arg1: std::option::Option<impl Into<crate::bukkit::attribute::AttributeModifier<'mc>>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg1.unwrap().into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeAttributeModifier",
            "(Lorg/bukkit/attribute/Attribute;Lorg/bukkit/attribute/AttributeModifier;)Z",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        )?;
        Ok(res.z().unwrap())
    }
    #[deprecated]
    pub fn custom_tag_container(
        &mut self,
    ) -> Result<
        crate::bukkit::inventory::meta::tags::CustomItemTagContainer<'mc>,
        Box<dyn std::error::Error>,
    > {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getCustomTagContainer",
            "()Lorg/bukkit/inventory/meta/tags/CustomItemTagContainer;",
            &[],
        )?;
        let ret = {
            crate::bukkit::inventory::meta::tags::CustomItemTagContainer(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    #[deprecated]
    pub fn set_version(&mut self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Int(arg0.into());
        self.jni_ref().call_method(
            &self.jni_object(),
            "setVersion",
            "(I)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn persistent_data_container(
        &mut self,
    ) -> Result<crate::bukkit::persistence::PersistentDataContainer<'mc>, Box<dyn std::error::Error>>
    {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getPersistentDataContainer",
            "()Lorg/bukkit/persistence/PersistentDataContainer;",
            &[],
        )?;
        let ret = {
            crate::bukkit::persistence::PersistentDataContainer(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
}
impl<'mc> crate::JNIRaw<'mc> for TropicalFishBucketMeta<'mc> {
    fn jni_ref(&self) -> crate::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Into<crate::bukkit::inventory::meta::ItemMeta<'mc>> for TropicalFishBucketMeta<'mc> {
    fn into(self) -> crate::bukkit::inventory::meta::ItemMeta<'mc> {
        crate::bukkit::inventory::meta::ItemMeta::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
/// An instantiatable struct that implements BookMeta. Needed for returning it from Java.
pub struct BookMeta<'mc>(
    pub(crate) crate::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> BookMeta<'mc> {
    pub fn from_raw(
        env: &crate::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate BookMeta from null object.").into());
        }
        let cls = env.jni.borrow().get_object_class(&obj)?;
        let name_raw = env.call_method(cls, "getName", "()Ljava/lang/String;", &[])?;
        let oh = name_raw.l()?.into();
        let what = env.get_string(&oh)?;
        let name = what.to_string_lossy();
        if !name.ends_with("BookMeta") {
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
    ) -> Result<crate::bukkit::inventory::meta::BookMetaSpigot<'mc>, Box<dyn std::error::Error>>
    {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "spigot",
            "()Lorg/bukkit/inventory/meta/BookMeta$Spigot;",
            &[],
        )?;
        let ret = {
            crate::bukkit::inventory::meta::BookMetaSpigot(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn get_page(&mut self, arg0: i32) -> Result<String, Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Int(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getPage",
            "(I)Ljava/lang/String;",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    pub fn set_page(&mut self, arg0: i32, arg1: String) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Int(arg0.into());
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg1).unwrap());
        self.jni_ref().call_method(
            &self.jni_object(),
            "setPage",
            "(ILjava/lang/String;)V",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        )?;
        Ok(())
    }
    pub fn title(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getTitle",
            "()Ljava/lang/String;",
            &[],
        )?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    pub fn author(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getAuthor",
            "()Ljava/lang/String;",
            &[],
        )?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    pub fn set_author(&mut self, arg0: String) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JObject::from(self.jni_ref().new_string(arg0).unwrap());
        self.jni_ref().call_method(
            &self.jni_object(),
            "setAuthor",
            "(Ljava/lang/String;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn set_pages_with_list(
        &mut self,
        _arg0: std::option::Option<Vec<String>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.jni_ref()
            .call_method(&self.jni_object(), "setPages", "(Ljava/lang/String;)V", &[])?;
        Ok(())
    }
    pub fn add_page(&mut self, _arg0: Vec<String>) -> Result<(), Box<dyn std::error::Error>> {
        self.jni_ref()
            .call_method(&self.jni_object(), "addPage", "(Ljava/lang/String;)V", &[])?;
        Ok(())
    }
    pub fn set_title(&mut self, arg0: String) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JObject::from(self.jni_ref().new_string(arg0).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setTitle",
            "(Ljava/lang/String;)Z",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(res.z().unwrap())
    }
    pub fn has_title(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasTitle", "()Z", &[])?;
        Ok(res.z().unwrap())
    }
    pub fn has_author(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasAuthor", "()Z", &[])?;
        Ok(res.z().unwrap())
    }
    pub fn has_generation(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasGeneration", "()Z", &[])?;
        Ok(res.z().unwrap())
    }
    pub fn generation(
        &mut self,
    ) -> Result<crate::bukkit::inventory::meta::BookMetaGeneration<'mc>, Box<dyn std::error::Error>>
    {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getGeneration",
            "()Lorg/bukkit/inventory/meta/BookMeta$Generation;",
            &[],
        )?;
        let ret = {
            crate::bukkit::inventory::meta::BookMetaGeneration(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn set_generation(
        &mut self,
        arg0: impl Into<crate::bukkit::inventory::meta::BookMetaGeneration<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        self.jni_ref().call_method(
            &self.jni_object(),
            "setGeneration",
            "(Lorg/bukkit/inventory/meta/BookMeta$Generation;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn has_pages(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasPages", "()Z", &[])?;
        Ok(res.z().unwrap())
    }
    pub fn page_count(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getPageCount", "()I", &[])?;
        Ok(res.i().unwrap())
    }
    pub fn clone(
        &mut self,
    ) -> Result<crate::bukkit::inventory::meta::BookMeta<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "clone",
            "()Lorg/bukkit/inventory/meta/BookMeta;",
            &[],
        )?;
        let ret = {
            crate::bukkit::inventory::meta::BookMeta(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn display_name(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getDisplayName",
            "()Ljava/lang/String;",
            &[],
        )?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    pub fn as_string(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getAsString",
            "()Ljava/lang/String;",
            &[],
        )?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    pub fn has_display_name(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasDisplayName", "()Z", &[])?;
        Ok(res.z().unwrap())
    }
    pub fn set_display_name(&mut self, arg0: String) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JObject::from(self.jni_ref().new_string(arg0).unwrap());
        self.jni_ref().call_method(
            &self.jni_object(),
            "setDisplayName",
            "(Ljava/lang/String;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn has_localized_name(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasLocalizedName", "()Z", &[])?;
        Ok(res.z().unwrap())
    }
    pub fn localized_name(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getLocalizedName",
            "()Ljava/lang/String;",
            &[],
        )?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    pub fn set_localized_name(&mut self, arg0: String) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JObject::from(self.jni_ref().new_string(arg0).unwrap());
        self.jni_ref().call_method(
            &self.jni_object(),
            "setLocalizedName",
            "(Ljava/lang/String;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn has_lore(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasLore", "()Z", &[])?;
        Ok(res.z().unwrap())
    }
    pub fn set_lore(&mut self, arg0: Vec<String>) -> Result<(), Box<dyn std::error::Error>> {
        let raw_val_0 = self
            .jni_ref()
            .new_object("java/util/ArrayList", "()V", &[])
            .unwrap();
        for v in arg0 {
            let map_val_0 = jni::objects::JObject::from(self.jni_ref().new_string(v).unwrap());
            self.jni_ref().call_method(
                &raw_val_0,
                "add",
                "(Ljava/Lang/Object)V",
                &[jni::objects::JValueGen::from(&map_val_0)],
            )?;
        }
        let val_0 = jni::objects::JValueGen::Object(raw_val_0);
        self.jni_ref().call_method(
            &self.jni_object(),
            "setLore",
            "(Ljava/util/List;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn has_custom_model_data(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "hasCustomModelData", "()Z", &[])?;
        Ok(res.z().unwrap())
    }
    pub fn custom_model_data(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getCustomModelData", "()I", &[])?;
        Ok(res.i().unwrap())
    }
    pub fn set_custom_model_data(&mut self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Int(arg0.into());
        self.jni_ref().call_method(
            &self.jni_object(),
            "setCustomModelData",
            "(I)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn has_enchants(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasEnchants", "()Z", &[])?;
        Ok(res.z().unwrap())
    }
    pub fn has_enchant(
        &mut self,
        arg0: impl Into<crate::bukkit::enchantments::Enchantment<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "hasEnchant",
            "(Lorg/bukkit/enchantments/Enchantment;)Z",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(res.z().unwrap())
    }
    pub fn get_enchant_level(
        &mut self,
        arg0: impl Into<crate::bukkit::enchantments::Enchantment<'mc>>,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getEnchantLevel",
            "(Lorg/bukkit/enchantments/Enchantment;)I",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(res.i().unwrap())
    }
    pub fn add_enchant(
        &mut self,
        arg0: impl Into<crate::bukkit::enchantments::Enchantment<'mc>>,
        arg1: i32,
        arg2: bool,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let val_1 = jni::objects::JValueGen::Int(arg1.into());
        // -2
        let val_2 = jni::objects::JValueGen::Bool(arg2.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "addEnchant",
            "(Lorg/bukkit/enchantments/Enchantment;IZ)Z",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        )?;
        Ok(res.z().unwrap())
    }
    pub fn remove_enchant(
        &mut self,
        arg0: impl Into<crate::bukkit::enchantments::Enchantment<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeEnchant",
            "(Lorg/bukkit/enchantments/Enchantment;)Z",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(res.z().unwrap())
    }
    pub fn has_conflicting_enchant(
        &mut self,
        arg0: impl Into<crate::bukkit::enchantments::Enchantment<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "hasConflictingEnchant",
            "(Lorg/bukkit/enchantments/Enchantment;)Z",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(res.z().unwrap())
    }
    pub fn add_item_flags(
        &mut self,
        _arg0: Vec<impl Into<crate::bukkit::inventory::ItemFlag<'mc>>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.jni_ref().call_method(
            &self.jni_object(),
            "addItemFlags",
            "(Lorg/bukkit/inventory/ItemFlag;)V",
            &[],
        )?;
        Ok(())
    }
    pub fn remove_item_flags(
        &mut self,
        _arg0: Vec<impl Into<crate::bukkit::inventory::ItemFlag<'mc>>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.jni_ref().call_method(
            &self.jni_object(),
            "removeItemFlags",
            "(Lorg/bukkit/inventory/ItemFlag;)V",
            &[],
        )?;
        Ok(())
    }
    pub fn has_item_flag(
        &mut self,
        arg0: impl Into<crate::bukkit::inventory::ItemFlag<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "hasItemFlag",
            "(Lorg/bukkit/inventory/ItemFlag;)Z",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(res.z().unwrap())
    }
    pub fn is_unbreakable(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isUnbreakable", "()Z", &[])?;
        Ok(res.z().unwrap())
    }
    pub fn set_unbreakable(&mut self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        // -2
        let val_0 = jni::objects::JValueGen::Bool(arg0.into());
        self.jni_ref().call_method(
            &self.jni_object(),
            "setUnbreakable",
            "(Z)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn has_attribute_modifiers(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "hasAttributeModifiers", "()Z", &[])?;
        Ok(res.z().unwrap())
    }
    pub fn add_attribute_modifier(
        &mut self,
        arg0: impl Into<crate::bukkit::attribute::Attribute<'mc>>,
        arg1: impl Into<crate::bukkit::attribute::AttributeModifier<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg1.into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "addAttributeModifier",
            "(Lorg/bukkit/attribute/Attribute;Lorg/bukkit/attribute/AttributeModifier;)Z",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        )?;
        Ok(res.z().unwrap())
    }
    pub fn remove_attribute_modifier_with_attribute(
        &mut self,
        arg0: impl Into<crate::bukkit::attribute::Attribute<'mc>>,
        arg1: std::option::Option<impl Into<crate::bukkit::attribute::AttributeModifier<'mc>>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg1.unwrap().into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeAttributeModifier",
            "(Lorg/bukkit/attribute/Attribute;Lorg/bukkit/attribute/AttributeModifier;)Z",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        )?;
        Ok(res.z().unwrap())
    }
    #[deprecated]
    pub fn custom_tag_container(
        &mut self,
    ) -> Result<
        crate::bukkit::inventory::meta::tags::CustomItemTagContainer<'mc>,
        Box<dyn std::error::Error>,
    > {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getCustomTagContainer",
            "()Lorg/bukkit/inventory/meta/tags/CustomItemTagContainer;",
            &[],
        )?;
        let ret = {
            crate::bukkit::inventory::meta::tags::CustomItemTagContainer(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    #[deprecated]
    pub fn set_version(&mut self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Int(arg0.into());
        self.jni_ref().call_method(
            &self.jni_object(),
            "setVersion",
            "(I)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn persistent_data_container(
        &mut self,
    ) -> Result<crate::bukkit::persistence::PersistentDataContainer<'mc>, Box<dyn std::error::Error>>
    {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getPersistentDataContainer",
            "()Lorg/bukkit/persistence/PersistentDataContainer;",
            &[],
        )?;
        let ret = {
            crate::bukkit::persistence::PersistentDataContainer(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
}
impl<'mc> crate::JNIRaw<'mc> for BookMeta<'mc> {
    fn jni_ref(&self) -> crate::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Into<crate::bukkit::inventory::meta::ItemMeta<'mc>> for BookMeta<'mc> {
    fn into(self) -> crate::bukkit::inventory::meta::ItemMeta<'mc> {
        crate::bukkit::inventory::meta::ItemMeta::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
/// An instantiatable struct that implements MusicInstrumentMeta. Needed for returning it from Java.
pub struct MusicInstrumentMeta<'mc>(
    pub(crate) crate::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> MusicInstrumentMeta<'mc> {
    pub fn from_raw(
        env: &crate::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate MusicInstrumentMeta from null object.").into(),
            );
        }
        let cls = env.jni.borrow().get_object_class(&obj)?;
        let name_raw = env.call_method(cls, "getName", "()Ljava/lang/String;", &[])?;
        let oh = name_raw.l()?.into();
        let what = env.get_string(&oh)?;
        let name = what.to_string_lossy();
        if !name.ends_with("MusicInstrumentMeta") {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a MusicInstrumentMeta object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    pub fn instrument(
        &mut self,
    ) -> Result<crate::bukkit::MusicInstrument<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getInstrument",
            "()Lorg/bukkit/MusicInstrument;",
            &[],
        )?;
        let ret = {
            crate::bukkit::MusicInstrument(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn set_instrument(
        &mut self,
        arg0: impl Into<crate::bukkit::MusicInstrument<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        self.jni_ref().call_method(
            &self.jni_object(),
            "setInstrument",
            "(Lorg/bukkit/MusicInstrument;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn clone(
        &mut self,
    ) -> Result<crate::bukkit::inventory::meta::MusicInstrumentMeta<'mc>, Box<dyn std::error::Error>>
    {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "clone",
            "()Lorg/bukkit/inventory/meta/MusicInstrumentMeta;",
            &[],
        )?;
        let ret = {
            crate::bukkit::inventory::meta::MusicInstrumentMeta(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn display_name(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getDisplayName",
            "()Ljava/lang/String;",
            &[],
        )?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    pub fn as_string(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getAsString",
            "()Ljava/lang/String;",
            &[],
        )?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    pub fn has_display_name(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasDisplayName", "()Z", &[])?;
        Ok(res.z().unwrap())
    }
    pub fn set_display_name(&mut self, arg0: String) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JObject::from(self.jni_ref().new_string(arg0).unwrap());
        self.jni_ref().call_method(
            &self.jni_object(),
            "setDisplayName",
            "(Ljava/lang/String;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn has_localized_name(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasLocalizedName", "()Z", &[])?;
        Ok(res.z().unwrap())
    }
    pub fn localized_name(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getLocalizedName",
            "()Ljava/lang/String;",
            &[],
        )?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    pub fn set_localized_name(&mut self, arg0: String) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JObject::from(self.jni_ref().new_string(arg0).unwrap());
        self.jni_ref().call_method(
            &self.jni_object(),
            "setLocalizedName",
            "(Ljava/lang/String;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn has_lore(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasLore", "()Z", &[])?;
        Ok(res.z().unwrap())
    }
    pub fn set_lore(&mut self, arg0: Vec<String>) -> Result<(), Box<dyn std::error::Error>> {
        let raw_val_0 = self
            .jni_ref()
            .new_object("java/util/ArrayList", "()V", &[])
            .unwrap();
        for v in arg0 {
            let map_val_0 = jni::objects::JObject::from(self.jni_ref().new_string(v).unwrap());
            self.jni_ref().call_method(
                &raw_val_0,
                "add",
                "(Ljava/Lang/Object)V",
                &[jni::objects::JValueGen::from(&map_val_0)],
            )?;
        }
        let val_0 = jni::objects::JValueGen::Object(raw_val_0);
        self.jni_ref().call_method(
            &self.jni_object(),
            "setLore",
            "(Ljava/util/List;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn has_custom_model_data(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "hasCustomModelData", "()Z", &[])?;
        Ok(res.z().unwrap())
    }
    pub fn custom_model_data(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getCustomModelData", "()I", &[])?;
        Ok(res.i().unwrap())
    }
    pub fn set_custom_model_data(&mut self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Int(arg0.into());
        self.jni_ref().call_method(
            &self.jni_object(),
            "setCustomModelData",
            "(I)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn has_enchants(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasEnchants", "()Z", &[])?;
        Ok(res.z().unwrap())
    }
    pub fn has_enchant(
        &mut self,
        arg0: impl Into<crate::bukkit::enchantments::Enchantment<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "hasEnchant",
            "(Lorg/bukkit/enchantments/Enchantment;)Z",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(res.z().unwrap())
    }
    pub fn get_enchant_level(
        &mut self,
        arg0: impl Into<crate::bukkit::enchantments::Enchantment<'mc>>,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getEnchantLevel",
            "(Lorg/bukkit/enchantments/Enchantment;)I",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(res.i().unwrap())
    }
    pub fn add_enchant(
        &mut self,
        arg0: impl Into<crate::bukkit::enchantments::Enchantment<'mc>>,
        arg1: i32,
        arg2: bool,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let val_1 = jni::objects::JValueGen::Int(arg1.into());
        // -2
        let val_2 = jni::objects::JValueGen::Bool(arg2.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "addEnchant",
            "(Lorg/bukkit/enchantments/Enchantment;IZ)Z",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        )?;
        Ok(res.z().unwrap())
    }
    pub fn remove_enchant(
        &mut self,
        arg0: impl Into<crate::bukkit::enchantments::Enchantment<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeEnchant",
            "(Lorg/bukkit/enchantments/Enchantment;)Z",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(res.z().unwrap())
    }
    pub fn has_conflicting_enchant(
        &mut self,
        arg0: impl Into<crate::bukkit::enchantments::Enchantment<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "hasConflictingEnchant",
            "(Lorg/bukkit/enchantments/Enchantment;)Z",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(res.z().unwrap())
    }
    pub fn add_item_flags(
        &mut self,
        _arg0: Vec<impl Into<crate::bukkit::inventory::ItemFlag<'mc>>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.jni_ref().call_method(
            &self.jni_object(),
            "addItemFlags",
            "(Lorg/bukkit/inventory/ItemFlag;)V",
            &[],
        )?;
        Ok(())
    }
    pub fn remove_item_flags(
        &mut self,
        _arg0: Vec<impl Into<crate::bukkit::inventory::ItemFlag<'mc>>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.jni_ref().call_method(
            &self.jni_object(),
            "removeItemFlags",
            "(Lorg/bukkit/inventory/ItemFlag;)V",
            &[],
        )?;
        Ok(())
    }
    pub fn has_item_flag(
        &mut self,
        arg0: impl Into<crate::bukkit::inventory::ItemFlag<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "hasItemFlag",
            "(Lorg/bukkit/inventory/ItemFlag;)Z",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(res.z().unwrap())
    }
    pub fn is_unbreakable(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isUnbreakable", "()Z", &[])?;
        Ok(res.z().unwrap())
    }
    pub fn set_unbreakable(&mut self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        // -2
        let val_0 = jni::objects::JValueGen::Bool(arg0.into());
        self.jni_ref().call_method(
            &self.jni_object(),
            "setUnbreakable",
            "(Z)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn has_attribute_modifiers(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "hasAttributeModifiers", "()Z", &[])?;
        Ok(res.z().unwrap())
    }
    pub fn add_attribute_modifier(
        &mut self,
        arg0: impl Into<crate::bukkit::attribute::Attribute<'mc>>,
        arg1: impl Into<crate::bukkit::attribute::AttributeModifier<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg1.into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "addAttributeModifier",
            "(Lorg/bukkit/attribute/Attribute;Lorg/bukkit/attribute/AttributeModifier;)Z",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        )?;
        Ok(res.z().unwrap())
    }
    pub fn remove_attribute_modifier_with_attribute(
        &mut self,
        arg0: impl Into<crate::bukkit::attribute::Attribute<'mc>>,
        arg1: std::option::Option<impl Into<crate::bukkit::attribute::AttributeModifier<'mc>>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg1.unwrap().into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeAttributeModifier",
            "(Lorg/bukkit/attribute/Attribute;Lorg/bukkit/attribute/AttributeModifier;)Z",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        )?;
        Ok(res.z().unwrap())
    }
    #[deprecated]
    pub fn custom_tag_container(
        &mut self,
    ) -> Result<
        crate::bukkit::inventory::meta::tags::CustomItemTagContainer<'mc>,
        Box<dyn std::error::Error>,
    > {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getCustomTagContainer",
            "()Lorg/bukkit/inventory/meta/tags/CustomItemTagContainer;",
            &[],
        )?;
        let ret = {
            crate::bukkit::inventory::meta::tags::CustomItemTagContainer(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    #[deprecated]
    pub fn set_version(&mut self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Int(arg0.into());
        self.jni_ref().call_method(
            &self.jni_object(),
            "setVersion",
            "(I)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn persistent_data_container(
        &mut self,
    ) -> Result<crate::bukkit::persistence::PersistentDataContainer<'mc>, Box<dyn std::error::Error>>
    {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getPersistentDataContainer",
            "()Lorg/bukkit/persistence/PersistentDataContainer;",
            &[],
        )?;
        let ret = {
            crate::bukkit::persistence::PersistentDataContainer(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
}
impl<'mc> crate::JNIRaw<'mc> for MusicInstrumentMeta<'mc> {
    fn jni_ref(&self) -> crate::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Into<crate::bukkit::inventory::meta::ItemMeta<'mc>> for MusicInstrumentMeta<'mc> {
    fn into(self) -> crate::bukkit::inventory::meta::ItemMeta<'mc> {
        crate::bukkit::inventory::meta::ItemMeta::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
pub mod tags;
pub mod trim;
