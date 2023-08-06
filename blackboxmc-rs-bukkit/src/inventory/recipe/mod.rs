#![allow(deprecated)]
#![feature(anonymous_lifetime_in_impl_trait)]
use blackboxmc_general::JNIRaw;
use color_eyre::eyre::Result;
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
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
    pub CookingBookCategoryEnum,
);
impl<'mc> std::ops::Deref for CookingBookCategory<'mc> {
    type Target = CookingBookCategoryEnum;
    fn deref(&self) -> &Self::Target {
        return &self.2;
    }
}
impl<'mc> JNIRaw<'mc> for CookingBookCategory<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> CookingBookCategory<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
        e: CookingBookCategoryEnum,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate CookingBookCategory from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "CookingBookCategory")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a CookingBookCategory object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj, e))
        }
    }
    pub const FOOD: CookingBookCategoryEnum = CookingBookCategoryEnum::Food;
    pub const BLOCKS: CookingBookCategoryEnum = CookingBookCategoryEnum::Blocks;
    pub const MISC: CookingBookCategoryEnum = CookingBookCategoryEnum::Misc;
    pub fn from_string(str: String) -> std::option::Option<CookingBookCategoryEnum> {
        match str.as_str() {
            "FOOD" => Some(CookingBookCategoryEnum::Food),
            "BLOCKS" => Some(CookingBookCategoryEnum::Blocks),
            "MISC" => Some(CookingBookCategoryEnum::Misc),
            _ => None,
        }
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
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
    pub CraftingBookCategoryEnum,
);
impl<'mc> std::ops::Deref for CraftingBookCategory<'mc> {
    type Target = CraftingBookCategoryEnum;
    fn deref(&self) -> &Self::Target {
        return &self.2;
    }
}
impl<'mc> JNIRaw<'mc> for CraftingBookCategory<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> CraftingBookCategory<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
        e: CraftingBookCategoryEnum,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate CraftingBookCategory from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "CraftingBookCategory")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a CraftingBookCategory object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj, e))
        }
    }
    pub const BUILDING: CraftingBookCategoryEnum = CraftingBookCategoryEnum::Building;
    pub const REDSTONE: CraftingBookCategoryEnum = CraftingBookCategoryEnum::Redstone;
    pub const EQUIPMENT: CraftingBookCategoryEnum = CraftingBookCategoryEnum::Equipment;
    pub const MISC: CraftingBookCategoryEnum = CraftingBookCategoryEnum::Misc;
    pub fn from_string(str: String) -> std::option::Option<CraftingBookCategoryEnum> {
        match str.as_str() {
            "BUILDING" => Some(CraftingBookCategoryEnum::Building),
            "REDSTONE" => Some(CraftingBookCategoryEnum::Redstone),
            "EQUIPMENT" => Some(CraftingBookCategoryEnum::Equipment),
            "MISC" => Some(CraftingBookCategoryEnum::Misc),
            _ => None,
        }
    }
}
