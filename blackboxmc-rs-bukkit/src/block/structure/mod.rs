#![allow(deprecated)]
use blackboxmc_general::JNIRaw;
use color_eyre::eyre::Result;
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
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
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
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> StructureRotation<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
        e: StructureRotationEnum,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate StructureRotation from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "StructureRotation")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a StructureRotation object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj, e))
        }
    }
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
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
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
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> UsageMode<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
        e: UsageModeEnum,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate UsageMode from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "UsageMode")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a UsageMode object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj, e))
        }
    }
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
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
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
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Mirror<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
        e: MirrorEnum,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Mirror from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "Mirror")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Mirror object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj, e))
        }
    }
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
}
