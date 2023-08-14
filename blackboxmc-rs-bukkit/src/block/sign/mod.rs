#![allow(deprecated)]
use blackboxmc_general::JNIRaw;
use color_eyre::eyre::Result;
#[derive(PartialEq, Eq)]
pub enum SideEnum {
    Front,
    Back,
}
impl std::fmt::Display for SideEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
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
        let (valid, name) = env.validate_name(&obj, "org/bukkit/block/sign/Side")?;
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

    pub fn value_of(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<String>,
    ) -> Result<Side<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(jni.new_string(arg0.into())?);
        let cls = jni.find_class("org/bukkit/block/sign/Side");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(
            cls,
            "valueOf",
            "(Ljava/lang/String;)Lorg/bukkit/block/sign/Side;",
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
        Side::from_raw(
            &jni,
            raw_obj,
            Side::from_string(variant_str)
                .ok_or(eyre::eyre!("String gaven for variant was invalid"))?,
        )
    }
}
/// Represents a side of a sign.
///
/// This is a representation of an abstract class.
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
        let (valid, name) = env.validate_name(&obj, "org/bukkit/block/sign/SignSide")?;
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
    //

    pub fn is_glowing_text(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isGlowingText", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    //@Deprecated

    #[deprecated]
    //@NotNull

    /// Gets the line of text at the specified index on this side of the sign.
    /// <p>For example, getLine(0) will return the first line of text.</p>
    pub fn get_line(&mut self, arg0: i32) -> Result<String, Box<dyn std::error::Error>> {
        let sig = String::from("(I)Ljava/lang/String;");
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getLine",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    //

    pub fn set_line(
        &mut self,
        arg0: i32,
        arg1: impl Into<String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(ILjava/lang/String;)V");
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let val_2 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(arg1.into())?,
        ));
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setLine",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
            ],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //@Deprecated

    #[deprecated]
    /// Sets whether this side of the sign has glowing text.
    pub fn set_glowing_text(&mut self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        // -2
        let val_1 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setGlowingText",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn set_color(
        &mut self,
        arg0: impl Into<crate::DyeColor<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/DyeColor;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setColor",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn color(&mut self) -> Result<crate::DyeColor<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/DyeColor;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getColor", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant =
            self.jni_ref()
                .call_method(&raw_obj, "toString", "()Ljava/lang/String;", vec![]);
        let variant = self.jni_ref().translate_error(variant)?;
        let variant_str = self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::DyeColor::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::DyeColor::from_string(variant_str)
                .ok_or(eyre::eyre!("String gaven for variant was invalid"))?,
        )
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
        crate::material::Colorable::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting SignSide into crate::material::Colorable")
    }
}
