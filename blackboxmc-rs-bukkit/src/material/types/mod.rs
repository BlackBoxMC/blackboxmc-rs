#![allow(deprecated)]
use blackboxmc_general::JNIInstantiatable;
use blackboxmc_general::JNIInstantiatableEnum;
use blackboxmc_general::JNIRaw;
use color_eyre::eyre::Result;
#[derive(PartialEq, Eq)]
pub enum MushroomBlockTextureEnum {
    AllPores,
    CapNorthWest,
    CapNorth,
    CapNorthEast,
    CapWest,
    CapTop,
    CapEast,
    CapSouthWest,
    CapSouth,
    CapSouthEast,
    StemSides,
    AllCap,
    AllStem,
}
impl std::fmt::Display for MushroomBlockTextureEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MushroomBlockTextureEnum::AllPores => f.write_str("ALL_PORES"),
            MushroomBlockTextureEnum::CapNorthWest => f.write_str("CAP_NORTH_WEST"),
            MushroomBlockTextureEnum::CapNorth => f.write_str("CAP_NORTH"),
            MushroomBlockTextureEnum::CapNorthEast => f.write_str("CAP_NORTH_EAST"),
            MushroomBlockTextureEnum::CapWest => f.write_str("CAP_WEST"),
            MushroomBlockTextureEnum::CapTop => f.write_str("CAP_TOP"),
            MushroomBlockTextureEnum::CapEast => f.write_str("CAP_EAST"),
            MushroomBlockTextureEnum::CapSouthWest => f.write_str("CAP_SOUTH_WEST"),
            MushroomBlockTextureEnum::CapSouth => f.write_str("CAP_SOUTH"),
            MushroomBlockTextureEnum::CapSouthEast => f.write_str("CAP_SOUTH_EAST"),
            MushroomBlockTextureEnum::StemSides => f.write_str("STEM_SIDES"),
            MushroomBlockTextureEnum::AllCap => f.write_str("ALL_CAP"),
            MushroomBlockTextureEnum::AllStem => f.write_str("ALL_STEM"),
        }
    }
}
impl<'mc> std::fmt::Display for MushroomBlockTexture<'mc> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.2.fmt(f)
    }
}
pub struct MushroomBlockTexture<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
    pub MushroomBlockTextureEnum,
);
impl<'mc> std::ops::Deref for MushroomBlockTexture<'mc> {
    type Target = MushroomBlockTextureEnum;
    fn deref(&self) -> &Self::Target {
        return &self.2;
    }
}

impl<'mc> JNIRaw<'mc> for MushroomBlockTexture<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}

impl<'mc> JNIInstantiatableEnum<'mc> for MushroomBlockTexture<'mc> {
    type Enum = MushroomBlockTextureEnum;

    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,

        e: Self::Enum,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate MushroomBlockTexture from null object.").into(),
            );
        }
        let (valid, name) =
            env.validate_name(&obj, "org/bukkit/material/types/MushroomBlockTexture")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a MushroomBlockTexture object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj, e))
        }
    }
}

impl<'mc> MushroomBlockTexture<'mc> {
    pub const ALL_PORES: MushroomBlockTextureEnum = MushroomBlockTextureEnum::AllPores;
    pub const CAP_NORTH_WEST: MushroomBlockTextureEnum = MushroomBlockTextureEnum::CapNorthWest;
    pub const CAP_NORTH: MushroomBlockTextureEnum = MushroomBlockTextureEnum::CapNorth;
    pub const CAP_NORTH_EAST: MushroomBlockTextureEnum = MushroomBlockTextureEnum::CapNorthEast;
    pub const CAP_WEST: MushroomBlockTextureEnum = MushroomBlockTextureEnum::CapWest;
    pub const CAP_TOP: MushroomBlockTextureEnum = MushroomBlockTextureEnum::CapTop;
    pub const CAP_EAST: MushroomBlockTextureEnum = MushroomBlockTextureEnum::CapEast;
    pub const CAP_SOUTH_WEST: MushroomBlockTextureEnum = MushroomBlockTextureEnum::CapSouthWest;
    pub const CAP_SOUTH: MushroomBlockTextureEnum = MushroomBlockTextureEnum::CapSouth;
    pub const CAP_SOUTH_EAST: MushroomBlockTextureEnum = MushroomBlockTextureEnum::CapSouthEast;
    pub const STEM_SIDES: MushroomBlockTextureEnum = MushroomBlockTextureEnum::StemSides;
    pub const ALL_CAP: MushroomBlockTextureEnum = MushroomBlockTextureEnum::AllCap;
    pub const ALL_STEM: MushroomBlockTextureEnum = MushroomBlockTextureEnum::AllStem;
    pub fn from_string(str: String) -> std::option::Option<MushroomBlockTextureEnum> {
        match str.as_str() {
            "ALL_PORES" => Some(MushroomBlockTextureEnum::AllPores),
            "CAP_NORTH_WEST" => Some(MushroomBlockTextureEnum::CapNorthWest),
            "CAP_NORTH" => Some(MushroomBlockTextureEnum::CapNorth),
            "CAP_NORTH_EAST" => Some(MushroomBlockTextureEnum::CapNorthEast),
            "CAP_WEST" => Some(MushroomBlockTextureEnum::CapWest),
            "CAP_TOP" => Some(MushroomBlockTextureEnum::CapTop),
            "CAP_EAST" => Some(MushroomBlockTextureEnum::CapEast),
            "CAP_SOUTH_WEST" => Some(MushroomBlockTextureEnum::CapSouthWest),
            "CAP_SOUTH" => Some(MushroomBlockTextureEnum::CapSouth),
            "CAP_SOUTH_EAST" => Some(MushroomBlockTextureEnum::CapSouthEast),
            "STEM_SIDES" => Some(MushroomBlockTextureEnum::StemSides),
            "ALL_CAP" => Some(MushroomBlockTextureEnum::AllCap),
            "ALL_STEM" => Some(MushroomBlockTextureEnum::AllStem),
            _ => None,
        }
    }

    pub fn value_of(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<String>,
    ) -> Result<MushroomBlockTexture<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(jni.new_string(arg0.into())?);
        let cls = jni.find_class("org/bukkit/material/types/MushroomBlockTexture");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(
            cls,
            "valueOf",
            "(Ljava/lang/String;)Lorg/bukkit/material/types/MushroomBlockTexture;",
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
        MushroomBlockTexture::from_raw(
            &jni,
            raw_obj,
            MushroomBlockTexture::from_string(variant_str)
                .ok_or(eyre::eyre!("String gaven for variant was invalid"))?,
        )
    }
}
