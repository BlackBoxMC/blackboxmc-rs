#![allow(deprecated)]
use blackboxmc_general::JNIRaw;
use color_eyre::eyre::Result;
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
        match &self {
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
impl<'mc> MushroomBlockTexture<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
        e: MushroomBlockTextureEnum,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate MushroomBlockTexture from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "MushroomBlockTexture")?;
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
        jni: blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<&'mc String>,
    ) -> Result<crate::material::types::MushroomBlockTexture<'mc>, Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JObject::from(jni.new_string(arg0.into()).unwrap());
        let cls = &jni.find_class("org/bukkit/material/types/MushroomBlockTexture")?;
        let res = jni.call_static_method(
            cls,
            "valueOf",
            "(Ljava/lang/String;)Lorg/bukkit/material/types/MushroomBlockTexture;",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        let mut obj = res.l()?;
        let raw_obj = obj;
        let variant = jni.call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
        let variant_str = jni
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::material::types::MushroomBlockTexture::from_raw(
            &jni,
            raw_obj,
            crate::material::types::MushroomBlockTexture::from_string(variant_str).unwrap(),
        )
    }
    #[deprecated]
    pub fn data(&mut self) -> Result<i8, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getData", "()B", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.b().unwrap())
    }
    #[deprecated]
    pub fn get_by_data(
        jni: blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: i8,
    ) -> Result<crate::material::types::MushroomBlockTexture<'mc>, Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Byte(arg0.into());
        let cls = &jni.find_class("org/bukkit/material/types/MushroomBlockTexture")?;
        let res = jni.call_static_method(
            cls,
            "getByData",
            "(B)Lorg/bukkit/material/types/MushroomBlockTexture;",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        let mut obj = res.l()?;
        let raw_obj = obj;
        let variant = jni.call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
        let variant_str = jni
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::material::types::MushroomBlockTexture::from_raw(
            &jni,
            raw_obj,
            crate::material::types::MushroomBlockTexture::from_string(variant_str).unwrap(),
        )
    }
    pub fn get_cap_by_face(
        jni: blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<&'mc crate::block::BlockFace<'mc>>,
    ) -> Result<crate::material::types::MushroomBlockTexture<'mc>, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let cls = &jni.find_class("org/bukkit/material/types/MushroomBlockTexture")?;
        let res = jni.call_static_method(
            cls,
            "getCapByFace",
            "(Lorg/bukkit/block/BlockFace;)Lorg/bukkit/material/types/MushroomBlockTexture;",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        let mut obj = res.l()?;
        let raw_obj = obj;
        let variant = jni.call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
        let variant_str = jni
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::material::types::MushroomBlockTexture::from_raw(
            &jni,
            raw_obj,
            crate::material::types::MushroomBlockTexture::from_string(variant_str).unwrap(),
        )
    }
    pub fn cap_face(&mut self) -> Result<crate::block::BlockFace<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getCapFace",
            "()Lorg/bukkit/block/BlockFace;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant = self
            .0
            .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
        let variant_str = self
            .0
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::block::BlockFace::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::block::BlockFace::from_string(variant_str).unwrap(),
        )
    }
}
