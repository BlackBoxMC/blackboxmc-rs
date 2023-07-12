use crate::JNIRaw;
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
    pub(crate) jni::JNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
    pub MushroomBlockTextureEnum,
);
impl<'mc> std::ops::Deref for MushroomBlockTexture<'mc> {
    type Target = MushroomBlockTextureEnum;
    fn deref(&self) -> &Self::Target {
        return &self.2;
    }
}
impl<'mc> crate::JNIRaw<'mc> for MushroomBlockTexture<'mc> {
    fn jni_ref(&self) -> jni::JNIEnv<'mc> {
        unsafe { self.0.unsafe_clone() }
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> MushroomBlockTexture<'mc> {
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
    pub fn data(&mut self) -> Result<i8, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getData", "()B", &[])?;
        Ok(res.b().unwrap())
    }
    pub fn get_by_data(
        mut jni: jni::JNIEnv<'mc>,
        arg0: i8,
    ) -> Result<crate::bukkit::material::types::MushroomBlockTexture<'mc>, Box<dyn std::error::Error>>
    {
        let val_0 = jni::objects::JValueGen::Byte(arg0.into());
        let cls = &jni.find_class("org/bukkit/material/types/MushroomBlockTexture")?;
        let res = jni.call_static_method(
            cls,
            "getByData",
            "(B)Lorg/bukkit/material/types/MushroomBlockTexture;",
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
            crate::bukkit::material::types::MushroomBlockTexture(
                jni,
                raw_obj,
                crate::bukkit::material::types::MushroomBlockTexture::from_string(variant_str)
                    .unwrap(),
            )
        };
        Ok(ret)
    }
    pub fn get_cap_by_face(
        mut jni: jni::JNIEnv<'mc>,
        arg0: crate::bukkit::block::BlockFace<'mc>,
    ) -> Result<crate::bukkit::material::types::MushroomBlockTexture<'mc>, Box<dyn std::error::Error>>
    {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.1.clone()) };
        let cls = &jni.find_class("org/bukkit/material/types/MushroomBlockTexture")?;
        let res = jni.call_static_method(
            cls,
            "getCapByFace",
            "(Lorg/bukkit/block/BlockFace;)Lorg/bukkit/material/types/MushroomBlockTexture;",
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
            crate::bukkit::material::types::MushroomBlockTexture(
                jni,
                raw_obj,
                crate::bukkit::material::types::MushroomBlockTexture::from_string(variant_str)
                    .unwrap(),
            )
        };
        Ok(ret)
    }
    pub fn cap_face(
        &mut self,
    ) -> Result<crate::bukkit::block::BlockFace<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getCapFace",
            "()Lorg/bukkit/block/BlockFace;",
            &[],
        )?;
        let ret = {
            let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
            let variant = self
                .0
                .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
            let variant_str = self
                .0
                .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
                .to_string_lossy()
                .to_string();
            crate::bukkit::block::BlockFace(
                self.jni_ref(),
                raw_obj,
                crate::bukkit::block::BlockFace::from_string(variant_str).unwrap(),
            )
        };
        Ok(ret)
    }
    pub fn value_of(
        mut jni: jni::JNIEnv<'mc>,
        arg0: String,
    ) -> Result<crate::bukkit::material::types::MushroomBlockTexture<'mc>, Box<dyn std::error::Error>>
    {
        let val_0 = jni::objects::JObject::from(jni.new_string(arg0).unwrap());
        let cls = &jni.find_class("org/bukkit/material/types/MushroomBlockTexture")?;
        let res = jni.call_static_method(
            cls,
            "valueOf",
            "(Ljava/lang/String;)Lorg/bukkit/material/types/MushroomBlockTexture;",
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
            crate::bukkit::material::types::MushroomBlockTexture(
                jni,
                raw_obj,
                crate::bukkit::material::types::MushroomBlockTexture::from_string(variant_str)
                    .unwrap(),
            )
        };
        Ok(ret)
    }
}
