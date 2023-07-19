use crate::JNIRaw;
pub enum StructureRotationEnum {
    None,
    Clockwise90,
    Clockwise180,
    Counterclockwise90,
}
impl std::fmt::Display for StructureRotationEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            StructureRotationEnum::None => f.write_str("NONE"),
            StructureRotationEnum::Clockwise90 => f.write_str("CLOCKWISE_90"),
            StructureRotationEnum::Clockwise180 => f.write_str("CLOCKWISE_180"),
            StructureRotationEnum::Counterclockwise90 => f.write_str("COUNTERCLOCKWISE_90"),
        }
    }
}
pub struct StructureRotation<'mc>(
    pub(crate) crate::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
    pub StructureRotationEnum,
);
impl<'mc> std::ops::Deref for StructureRotation<'mc> {
    type Target = StructureRotationEnum;
    fn deref(&self) -> &Self::Target {
        return &self.2;
    }
}
impl<'mc> JNIRaw<'mc> for StructureRotation<'mc> {
    fn jni_ref(&self) -> crate::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> StructureRotation<'mc> {
    pub const NONE: StructureRotationEnum = StructureRotationEnum::None;
    pub const CLOCKWISE_90: StructureRotationEnum = StructureRotationEnum::Clockwise90;
    pub const CLOCKWISE_180: StructureRotationEnum = StructureRotationEnum::Clockwise180;
    pub const COUNTERCLOCKWISE_90: StructureRotationEnum =
        StructureRotationEnum::Counterclockwise90;
    pub fn from_string(str: String) -> std::option::Option<StructureRotationEnum> {
        match str.as_str() {
            "NONE" => Some(StructureRotationEnum::None),
            "CLOCKWISE_90" => Some(StructureRotationEnum::Clockwise90),
            "CLOCKWISE_180" => Some(StructureRotationEnum::Clockwise180),
            "COUNTERCLOCKWISE_90" => Some(StructureRotationEnum::Counterclockwise90),
            _ => None,
        }
    }
    pub fn value_of(
        jni: crate::SharedJNIEnv<'mc>,
        arg0: String,
    ) -> Result<crate::bukkit::block::structure::StructureRotation<'mc>, Box<dyn std::error::Error>>
    {
        let val_0 = jni::objects::JObject::from(jni.new_string(arg0).unwrap());
        let cls = &jni.find_class("org/bukkit/block/structure/StructureRotation")?;
        let res = jni.call_static_method(
            cls,
            "valueOf",
            "(Ljava/lang/String;)Lorg/bukkit/block/structure/StructureRotation;",
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
            crate::bukkit::block::structure::StructureRotation(
                jni,
                raw_obj,
                crate::bukkit::block::structure::StructureRotation::from_string(variant_str)
                    .unwrap(),
            )
        };
        Ok(ret)
    }
}
pub enum UsageModeEnum {
    Save,
    Load,
    Corner,
    Data,
}
impl std::fmt::Display for UsageModeEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            UsageModeEnum::Save => f.write_str("SAVE"),
            UsageModeEnum::Load => f.write_str("LOAD"),
            UsageModeEnum::Corner => f.write_str("CORNER"),
            UsageModeEnum::Data => f.write_str("DATA"),
        }
    }
}
pub struct UsageMode<'mc>(
    pub(crate) crate::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
    pub UsageModeEnum,
);
impl<'mc> std::ops::Deref for UsageMode<'mc> {
    type Target = UsageModeEnum;
    fn deref(&self) -> &Self::Target {
        return &self.2;
    }
}
impl<'mc> JNIRaw<'mc> for UsageMode<'mc> {
    fn jni_ref(&self) -> crate::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> UsageMode<'mc> {
    pub const SAVE: UsageModeEnum = UsageModeEnum::Save;
    pub const LOAD: UsageModeEnum = UsageModeEnum::Load;
    pub const CORNER: UsageModeEnum = UsageModeEnum::Corner;
    pub const DATA: UsageModeEnum = UsageModeEnum::Data;
    pub fn from_string(str: String) -> std::option::Option<UsageModeEnum> {
        match str.as_str() {
            "SAVE" => Some(UsageModeEnum::Save),
            "LOAD" => Some(UsageModeEnum::Load),
            "CORNER" => Some(UsageModeEnum::Corner),
            "DATA" => Some(UsageModeEnum::Data),
            _ => None,
        }
    }
    pub fn value_of(
        jni: crate::SharedJNIEnv<'mc>,
        arg0: String,
    ) -> Result<crate::bukkit::block::structure::UsageMode<'mc>, Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JObject::from(jni.new_string(arg0).unwrap());
        let cls = &jni.find_class("org/bukkit/block/structure/UsageMode")?;
        let res = jni.call_static_method(
            cls,
            "valueOf",
            "(Ljava/lang/String;)Lorg/bukkit/block/structure/UsageMode;",
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
            crate::bukkit::block::structure::UsageMode(
                jni,
                raw_obj,
                crate::bukkit::block::structure::UsageMode::from_string(variant_str).unwrap(),
            )
        };
        Ok(ret)
    }
}
pub enum MirrorEnum {
    None,
    LeftRight,
    FrontBack,
}
impl std::fmt::Display for MirrorEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            MirrorEnum::None => f.write_str("NONE"),
            MirrorEnum::LeftRight => f.write_str("LEFT_RIGHT"),
            MirrorEnum::FrontBack => f.write_str("FRONT_BACK"),
        }
    }
}
pub struct Mirror<'mc>(
    pub(crate) crate::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
    pub MirrorEnum,
);
impl<'mc> std::ops::Deref for Mirror<'mc> {
    type Target = MirrorEnum;
    fn deref(&self) -> &Self::Target {
        return &self.2;
    }
}
impl<'mc> JNIRaw<'mc> for Mirror<'mc> {
    fn jni_ref(&self) -> crate::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Mirror<'mc> {
    pub const NONE: MirrorEnum = MirrorEnum::None;
    pub const LEFT_RIGHT: MirrorEnum = MirrorEnum::LeftRight;
    pub const FRONT_BACK: MirrorEnum = MirrorEnum::FrontBack;
    pub fn from_string(str: String) -> std::option::Option<MirrorEnum> {
        match str.as_str() {
            "NONE" => Some(MirrorEnum::None),
            "LEFT_RIGHT" => Some(MirrorEnum::LeftRight),
            "FRONT_BACK" => Some(MirrorEnum::FrontBack),
            _ => None,
        }
    }
    pub fn value_of(
        jni: crate::SharedJNIEnv<'mc>,
        arg0: String,
    ) -> Result<crate::bukkit::block::structure::Mirror<'mc>, Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JObject::from(jni.new_string(arg0).unwrap());
        let cls = &jni.find_class("org/bukkit/block/structure/Mirror")?;
        let res = jni.call_static_method(
            cls,
            "valueOf",
            "(Ljava/lang/String;)Lorg/bukkit/block/structure/Mirror;",
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
            crate::bukkit::block::structure::Mirror(
                jni,
                raw_obj,
                crate::bukkit::block::structure::Mirror::from_string(variant_str).unwrap(),
            )
        };
        Ok(ret)
    }
}
