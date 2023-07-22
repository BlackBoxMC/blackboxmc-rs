#![allow(deprecated)]
use crate::JNIRaw;
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
    pub(crate) crate::SharedJNIEnv<'mc>,
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
    fn jni_ref(&self) -> crate::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Side<'mc> {
    pub const FRONT: SideEnum = SideEnum::Front;
    pub const BACK: SideEnum = SideEnum::Back;
    pub fn from_string(str: String) -> std::option::Option<SideEnum> {
        match str.as_str() {
            "FRONT" => Some(SideEnum::Front),
            "BACK" => Some(SideEnum::Back),
            _ => None,
        }
    }
    pub fn value_of(
        jni: crate::SharedJNIEnv<'mc>,
        arg0: impl Into<String>,
    ) -> Result<crate::bukkit::block::sign::Side<'mc>, Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JObject::from(jni.new_string(arg0.into()).unwrap());
        let cls = &jni.find_class("org/bukkit/block/sign/Side")?;
        let res = jni.call_static_method(
            cls,
            "valueOf",
            "(Ljava/lang/String;)Lorg/bukkit/block/sign/Side;",
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
            crate::bukkit::block::sign::Side(
                jni,
                raw_obj,
                crate::bukkit::block::sign::Side::from_string(variant_str).unwrap(),
            )
        };
        Ok(ret)
    }
}
/// An instantiatable struct that implements SignSide. Needed for returning it from Java.
pub struct SignSide<'mc>(
    pub(crate) crate::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> SignSide<'mc> {
    pub fn from_raw(
        env: &crate::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate SignSide from null object.").into());
        }
        let cls = env.jni.borrow().get_object_class(&obj)?;
        let name_raw = env.call_method(cls, "getName", "()Ljava/lang/String;", &[])?;
        let oh = name_raw.l()?.into();
        let what = env.get_string(&oh)?;
        let name = what.to_string_lossy();
        if !name.ends_with("SignSide") {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a SignSide object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    pub fn is_glowing_text(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isGlowingText", "()Z", &[])?;
        Ok(res.z().unwrap())
    }
    pub fn get_line(&mut self, arg0: i32) -> Result<String, Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Int(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getLine",
            "(I)Ljava/lang/String;",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    pub fn set_line(
        &mut self,
        arg0: i32,
        arg1: impl Into<String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Int(arg0.into());
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg1.into()).unwrap());
        self.jni_ref().call_method(
            &self.jni_object(),
            "setLine",
            "(ILjava/lang/String;)V",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        )?;
        Ok(())
    }
    pub fn set_glowing_text(&mut self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        // -2
        let val_0 = jni::objects::JValueGen::Bool(arg0.into());
        self.jni_ref().call_method(
            &self.jni_object(),
            "setGlowingText",
            "(Z)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn set_color(
        &mut self,
        arg0: impl Into<crate::bukkit::DyeColor<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        self.jni_ref().call_method(
            &self.jni_object(),
            "setColor",
            "(Lorg/bukkit/DyeColor;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn color(&mut self) -> Result<crate::bukkit::DyeColor<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getColor",
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
}
impl<'mc> crate::JNIRaw<'mc> for SignSide<'mc> {
    fn jni_ref(&self) -> crate::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Into<crate::bukkit::material::Colorable<'mc>> for SignSide<'mc> {
    fn into(self) -> crate::bukkit::material::Colorable<'mc> {
        crate::bukkit::material::Colorable::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
