#![allow(deprecated)]
use blackboxmc_general::JNIRaw;
use color_eyre::eyre::Result;
pub enum ChatMessageTypeEnum {
    Chat,
    System,
    ActionBar,
}
impl std::fmt::Display for ChatMessageTypeEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            ChatMessageTypeEnum::Chat => f.write_str("CHAT"),
            ChatMessageTypeEnum::System => f.write_str("SYSTEM"),
            ChatMessageTypeEnum::ActionBar => f.write_str("ACTION_BAR"),
        }
    }
}
pub struct ChatMessageType<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
    pub ChatMessageTypeEnum,
);
impl<'mc> std::ops::Deref for ChatMessageType<'mc> {
    type Target = ChatMessageTypeEnum;
    fn deref(&self) -> &Self::Target {
        return &self.2;
    }
}
impl<'mc> JNIRaw<'mc> for ChatMessageType<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> ChatMessageType<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
        e: ChatMessageTypeEnum,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate ChatMessageType from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "ChatMessageType")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a ChatMessageType object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj, e))
        }
    }
    pub const CHAT: ChatMessageTypeEnum = ChatMessageTypeEnum::Chat;
    pub const SYSTEM: ChatMessageTypeEnum = ChatMessageTypeEnum::System;
    pub const ACTION_BAR: ChatMessageTypeEnum = ChatMessageTypeEnum::ActionBar;
    pub fn from_string(str: String) -> std::option::Option<ChatMessageTypeEnum> {
        match str.as_str() {
            "CHAT" => Some(ChatMessageTypeEnum::Chat),
            "SYSTEM" => Some(ChatMessageTypeEnum::System),
            "ACTION_BAR" => Some(ChatMessageTypeEnum::ActionBar),
            _ => None,
        }
    }
    pub fn value_of(
        jni: blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<&'mc String>,
    ) -> Result<crate::bungee::api::ChatMessageType<'mc>, Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JObject::from(jni.new_string(arg0.into()).unwrap());
        let cls = &jni.find_class("net/md_5/bungee/api/ChatMessageType")?;
        let res = jni.call_static_method(
            cls,
            "valueOf",
            "(Ljava/lang/String;)Lnet/md_5/bungee/api/ChatMessageType;",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        let mut obj = res.l()?;
        let raw_obj = obj;
        let variant = jni.call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
        let variant_str = jni
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::bungee::api::ChatMessageType::from_raw(
            &jni,
            raw_obj,
            crate::bungee::api::ChatMessageType::from_string(variant_str).unwrap(),
        )
    }
}
pub struct ChatColor<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for ChatColor<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> ChatColor<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate ChatColor from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "ChatColor")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a ChatColor object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    #[deprecated]
    pub fn name(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "name", "()Ljava/lang/String;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    pub fn equals(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "equals",
            "(Ljava/lang/Object;)Z",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    pub fn to_string(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "toString", "()Ljava/lang/String;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    pub fn hash_code(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hashCode", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }
    #[deprecated]
    pub fn value_of(
        jni: blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<&'mc String>,
    ) -> Result<crate::bungee::api::ChatColor<'mc>, Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JObject::from(jni.new_string(arg0.into()).unwrap());
        let cls = &jni.find_class("net/md_5/bungee/api/ChatColor")?;
        let res = jni.call_static_method(
            cls,
            "valueOf",
            "(Ljava/lang/String;)Lnet/md_5/bungee/api/ChatColor;",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        let mut obj = res.l()?;
        crate::bungee::api::ChatColor::from_raw(&jni, obj)
    }
    pub fn of_with_color(
        jni: blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: std::option::Option<impl Into<&'mc String>>,
    ) -> Result<crate::bungee::api::ChatColor<'mc>, Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JObject::from(jni.new_string(arg0.unwrap().into()).unwrap());
        let cls = &jni.find_class("net/md_5/bungee/api/ChatColor")?;
        let res = jni.call_static_method(
            cls,
            "of",
            "(Ljava/lang/String;)Lnet/md_5/bungee/api/ChatColor;",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        let mut obj = res.l()?;
        crate::bungee::api::ChatColor::from_raw(&jni, obj)
    }
    #[deprecated]
    pub fn ordinal(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "ordinal", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }
    pub fn color(&mut self) -> Result<(u8, u8, u8), Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getColor", "()Ljava/awt/Color;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        let r = self
            .jni_ref()
            .call_method(
                unsafe { jni::objects::JObject::from_raw(res.as_jni().l) },
                "getRed",
                "(V)I",
                &[],
            )?
            .i()? as u8;
        let g = self
            .jni_ref()
            .call_method(
                unsafe { jni::objects::JObject::from_raw(res.as_jni().l) },
                "getGreen",
                "(V)I",
                &[],
            )?
            .i()? as u8;
        let b = self
            .jni_ref()
            .call_method(
                unsafe { jni::objects::JObject::from_raw(res.as_jni().l) },
                "getBlue",
                "(V)I",
                &[],
            )?
            .i()? as u8;
        (r, g, b);
        Ok((r, g, b))
    }
    pub fn get_by_char(
        jni: blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: u16,
    ) -> Result<crate::bungee::api::ChatColor<'mc>, Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Char(arg0.into());
        let cls = &jni.find_class("net/md_5/bungee/api/ChatColor")?;
        let res = jni.call_static_method(
            cls,
            "getByChar",
            "(C)Lnet/md_5/bungee/api/ChatColor;",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        let mut obj = res.l()?;
        crate::bungee::api::ChatColor::from_raw(&jni, obj)
    }
    pub fn strip_color(
        jni: blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<&'mc String>,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JObject::from(jni.new_string(arg0.into()).unwrap());
        let cls = &jni.find_class("java/lang/String")?;
        let res = jni.call_static_method(
            cls,
            "stripColor",
            "(Ljava/lang/String;)Ljava/lang/String;",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(jni
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    pub fn translate_alternate_color_codes(
        jni: blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: u16,
        arg1: impl Into<&'mc String>,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Char(arg0.into());
        let val_1 = jni::objects::JObject::from(jni.new_string(arg1.into()).unwrap());
        let cls = &jni.find_class("java/lang/String")?;
        let res = jni.call_static_method(
            cls,
            "translateAlternateColorCodes",
            "(CLjava/lang/String;)Ljava/lang/String;",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        )?;
        Ok(jni
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    pub fn wait(
        &mut self,
        arg0: std::option::Option<i64>,
        arg1: std::option::Option<i32>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Long(arg0.unwrap().into());
        let val_1 = jni::objects::JValueGen::Int(arg1.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "wait",
            "(JI)V",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub fn class(&mut self) -> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getClass", "()Ljava/lang/Class;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(unsafe { jni::objects::JClass::from_raw(res.as_jni().l) })
    }
    pub fn notify(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notify", "()V", &[]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub fn notify_all(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notifyAll", "()V", &[]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
}
pub mod chat;
