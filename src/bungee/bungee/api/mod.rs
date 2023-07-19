use crate::JNIRaw;
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
    pub(crate) crate::SharedJNIEnv<'mc>,
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
    fn jni_ref(&self) -> crate::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> ChatMessageType<'mc> {
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
        jni: crate::SharedJNIEnv<'mc>,
        arg0: String,
    ) -> Result<crate::bungee::bungee::api::ChatMessageType<'mc>, Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JObject::from(jni.new_string(arg0).unwrap());
        let cls = &jni.find_class("net/md_5/bungee/api/ChatMessageType")?;
        let res = jni.call_static_method(
            cls,
            "valueOf",
            "(Ljava/lang/String;)Lnet/md_5/bungee/api/ChatMessageType;",
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
            crate::bungee::bungee::api::ChatMessageType(
                jni,
                raw_obj,
                crate::bungee::bungee::api::ChatMessageType::from_string(variant_str).unwrap(),
            )
        };
        Ok(ret)
    }
}
pub struct ChatColor<'mc>(
    pub(crate) crate::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> crate::JNIRaw<'mc> for ChatColor<'mc> {
    fn jni_ref(&self) -> crate::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> ChatColor<'mc> {
    pub fn from_raw(
        env: &crate::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate ChatColor from null object.").into());
        }
        let cls = env.jni.borrow().get_object_class(&obj)?;
        let name_raw = env.call_method(cls, "getName", "()Ljava/lang/String;", &[])?;
        let oh = name_raw.l()?.into();
        let what = env.get_string(&oh)?;
        let name = what.to_string_lossy();
        if !name.ends_with("ChatColor") {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a ChatColor object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    pub fn name(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "name", "()Ljava/lang/String;", &[])?;
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
        )?;
        Ok(res.z().unwrap())
    }
    pub fn to_string(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "toString",
            "()Ljava/lang/String;",
            &[],
        )?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    pub fn hash_code(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hashCode", "()I", &[])?;
        Ok(res.i().unwrap())
    }
    pub fn value_of(
        jni: crate::SharedJNIEnv<'mc>,
        arg0: String,
    ) -> Result<crate::bungee::bungee::api::ChatColor<'mc>, Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JObject::from(jni.new_string(arg0).unwrap());
        let cls = &jni.find_class("net/md_5/bungee/api/ChatColor")?;
        let res = jni.call_static_method(
            cls,
            "valueOf",
            "(Ljava/lang/String;)Lnet/md_5/bungee/api/ChatColor;",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        let ret = {
            let obj = res.l()?;
            crate::bungee::bungee::api::ChatColor(jni, obj)
        };
        Ok(ret)
    }
    pub fn of_with_color(
        jni: crate::SharedJNIEnv<'mc>,
        arg0: std::option::Option<String>,
    ) -> Result<crate::bungee::bungee::api::ChatColor<'mc>, Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JObject::from(jni.new_string(arg0.unwrap()).unwrap());
        let cls = &jni.find_class("net/md_5/bungee/api/ChatColor")?;
        let res = jni.call_static_method(
            cls,
            "of",
            "(Ljava/lang/String;)Lnet/md_5/bungee/api/ChatColor;",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        let ret = {
            let obj = res.l()?;
            crate::bungee::bungee::api::ChatColor(jni, obj)
        };
        Ok(ret)
    }
    pub fn ordinal(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "ordinal", "()I", &[])?;
        Ok(res.i().unwrap())
    }
    pub fn color(&mut self) -> Result<(u8, u8, u8), Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getColor",
            "()Ljava/awt/Color;",
            &[],
        )?;
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
        jni: crate::SharedJNIEnv<'mc>,
        arg0: u16,
    ) -> Result<crate::bungee::bungee::api::ChatColor<'mc>, Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Char(arg0.into());
        let cls = &jni.find_class("net/md_5/bungee/api/ChatColor")?;
        let res = jni.call_static_method(
            cls,
            "getByChar",
            "(C)Lnet/md_5/bungee/api/ChatColor;",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        let ret = {
            let obj = res.l()?;
            crate::bungee::bungee::api::ChatColor(jni, obj)
        };
        Ok(ret)
    }
    pub fn strip_color(
        jni: crate::SharedJNIEnv<'mc>,
        arg0: String,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JObject::from(jni.new_string(arg0).unwrap());
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
        jni: crate::SharedJNIEnv<'mc>,
        arg0: u16,
        arg1: String,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Char(arg0.into());
        let val_1 = jni::objects::JObject::from(jni.new_string(arg1).unwrap());
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
        self.jni_ref().call_method(
            &self.jni_object(),
            "wait",
            "(JI)V",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        )?;
        Ok(())
    }
    pub fn class(&mut self) -> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getClass",
            "()Ljava/lang/Class;",
            &[],
        )?;
        Ok(unsafe { jni::objects::JClass::from_raw(res.as_jni().l) })
    }
    pub fn notify(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.jni_ref()
            .call_method(&self.jni_object(), "notify", "()V", &[])?;
        Ok(())
    }
    pub fn notify_all(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.jni_ref()
            .call_method(&self.jni_object(), "notifyAll", "()V", &[])?;
        Ok(())
    }
}
pub mod chat;
