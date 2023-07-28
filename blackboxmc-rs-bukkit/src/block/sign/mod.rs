#![allow(deprecated)]
use blackboxmc_general::JNIRaw;
use color_eyre::eyre::Result;
pub enum SideEnum {
    Front,
    Back,
}
impl std::fmt::Display for SideEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            SideEnum::Front => f.write_str("FRONT"),
            SideEnum::Back => f.write_str("BACK"),
        }
    }
}
pub struct Side<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
    pub SideEnum,
);
impl<'mc> std::ops::Deref for Side<'mc> {
    type Target = SideEnum;
    fn deref(&self) -> &Self::Target {
        return &self.2;
    }
}
impl<'mc> JNIRaw<'mc> for Side<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Side<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
        e: SideEnum,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Side from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "Side")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Side object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj, e))
        }
    }
    pub const FRONT: SideEnum = SideEnum::Front;
    pub const BACK: SideEnum = SideEnum::Back;
    pub fn from_string(str: String) -> std::option::Option<SideEnum> {
        match str.as_str() {
            "FRONT" => Some(SideEnum::Front),
            "BACK" => Some(SideEnum::Back),
            _ => None,
        }
    }
}
/// An instantiatable struct that implements SignSide. Needed for returning it from Java.
pub struct SignSide<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> SignSide<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate SignSide from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "SignSide")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a SignSide object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}
impl<'mc> JNIRaw<'mc> for SignSide<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Into<crate::material::Colorable<'mc>> for SignSide<'mc> {
    fn into(self) -> crate::material::Colorable<'mc> {
        crate::material::Colorable::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
