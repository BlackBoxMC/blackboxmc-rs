
pub enum CookingBookCategoryEnum {
    Food,
    Blocks,
    Misc,
}
impl std::fmt::Display for CookingBookCategoryEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            CookingBookCategoryEnum::Food => f.write_str("FOOD"),
            CookingBookCategoryEnum::Blocks => f.write_str("BLOCKS"),
            CookingBookCategoryEnum::Misc => f.write_str("MISC"),
        }
    }
}
pub struct CookingBookCategory<'mc>(
    pub(crate) crate::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
    pub CookingBookCategoryEnum,
);
impl<'mc> std::ops::Deref for CookingBookCategory<'mc> {
    type Target = CookingBookCategoryEnum;
    fn deref(&self) -> &Self::Target {
        return &self.2;
    }
}
impl<'mc> crate::JNIRaw<'mc> for CookingBookCategory<'mc> {
    fn jni_ref(&self) -> crate::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> CookingBookCategory<'mc> {
    pub fn from_string(str: String) -> std::option::Option<CookingBookCategoryEnum> {
        match str.as_str() {
            "FOOD" => Some(CookingBookCategoryEnum::Food),
            "BLOCKS" => Some(CookingBookCategoryEnum::Blocks),
            "MISC" => Some(CookingBookCategoryEnum::Misc),
            _ => None,
        }
    }
    pub fn value_of(
        mut jni: crate::SharedJNIEnv<'mc>,
        arg0: String,
    ) -> Result<
        crate::bukkit::inventory::recipe::CookingBookCategory<'mc>,
        Box<dyn std::error::Error>,
    > {
        let val_0 = jni::objects::JObject::from(jni.new_string(arg0).unwrap());
        let cls = &jni.find_class("org/bukkit/inventory/recipe/CookingBookCategory")?;
        let res = jni.call_static_method(
            cls,
            "valueOf",
            "(Ljava/lang/String;)Lorg/bukkit/inventory/recipe/CookingBookCategory;",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        let ret = {
            let obj = res.l()?;
            let raw_obj = obj;
            let variant = jni.call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
            let variant_str = jni
                .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
                .to_string_lossy()
                .to_string();
            crate::bukkit::inventory::recipe::CookingBookCategory(
                jni,
                raw_obj,
                crate::bukkit::inventory::recipe::CookingBookCategory::from_string(variant_str)
                    .unwrap(),
            )
        };
        Ok(ret)
    }
}
pub enum CraftingBookCategoryEnum {
    Building,
    Redstone,
    Equipment,
    Misc,
}
impl std::fmt::Display for CraftingBookCategoryEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            CraftingBookCategoryEnum::Building => f.write_str("BUILDING"),
            CraftingBookCategoryEnum::Redstone => f.write_str("REDSTONE"),
            CraftingBookCategoryEnum::Equipment => f.write_str("EQUIPMENT"),
            CraftingBookCategoryEnum::Misc => f.write_str("MISC"),
        }
    }
}
pub struct CraftingBookCategory<'mc>(
    pub(crate) crate::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
    pub CraftingBookCategoryEnum,
);
impl<'mc> std::ops::Deref for CraftingBookCategory<'mc> {
    type Target = CraftingBookCategoryEnum;
    fn deref(&self) -> &Self::Target {
        return &self.2;
    }
}
impl<'mc> crate::JNIRaw<'mc> for CraftingBookCategory<'mc> {
    fn jni_ref(&self) -> crate::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> CraftingBookCategory<'mc> {
    pub fn from_string(str: String) -> std::option::Option<CraftingBookCategoryEnum> {
        match str.as_str() {
            "BUILDING" => Some(CraftingBookCategoryEnum::Building),
            "REDSTONE" => Some(CraftingBookCategoryEnum::Redstone),
            "EQUIPMENT" => Some(CraftingBookCategoryEnum::Equipment),
            "MISC" => Some(CraftingBookCategoryEnum::Misc),
            _ => None,
        }
    }
    pub fn value_of(
        mut jni: crate::SharedJNIEnv<'mc>,
        arg0: String,
    ) -> Result<
        crate::bukkit::inventory::recipe::CraftingBookCategory<'mc>,
        Box<dyn std::error::Error>,
    > {
        let val_0 = jni::objects::JObject::from(jni.new_string(arg0).unwrap());
        let cls = &jni.find_class("org/bukkit/inventory/recipe/CraftingBookCategory")?;
        let res = jni.call_static_method(
            cls,
            "valueOf",
            "(Ljava/lang/String;)Lorg/bukkit/inventory/recipe/CraftingBookCategory;",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        let ret = {
            let obj = res.l()?;
            let raw_obj = obj;
            let variant = jni.call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
            let variant_str = jni
                .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
                .to_string_lossy()
                .to_string();
            crate::bukkit::inventory::recipe::CraftingBookCategory(
                jni,
                raw_obj,
                crate::bukkit::inventory::recipe::CraftingBookCategory::from_string(variant_str)
                    .unwrap(),
            )
        };
        Ok(ret)
    }
}
