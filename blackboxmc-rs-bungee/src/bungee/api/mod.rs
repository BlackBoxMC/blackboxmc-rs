#![allow(deprecated)]
use blackboxmc_general::JNIInstantiatable;
use blackboxmc_general::JNIRaw;
use color_eyre::eyre::Result;
pub enum ChatMessageType<'mc> {
    Chat { inner: ChatMessageTypeStruct<'mc> },
    System { inner: ChatMessageTypeStruct<'mc> },
    ActionBar { inner: ChatMessageTypeStruct<'mc> },
}
impl<'mc> std::fmt::Display for ChatMessageType<'mc> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ChatMessageType::Chat { .. } => f.write_str("CHAT"),
            ChatMessageType::System { .. } => f.write_str("SYSTEM"),
            ChatMessageType::ActionBar { .. } => f.write_str("ACTION_BAR"),
        }
    }
}

impl<'mc> ChatMessageType<'mc> {
    pub fn value_of(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<String>,
    ) -> Result<ChatMessageType<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(env.new_string(arg0.into())?);
        let cls = env.find_class("net/md_5/bungee/api/ChatMessageType");
        let cls = env.translate_error_with_class(cls)?;
        let res = env.call_static_method(
            cls,
            "valueOf",
            "(Ljava/lang/String;)Lnet/md_5/bungee/api/ChatMessageType;",
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = env.translate_error(res)?;
        let obj = res.l()?;
        let variant = env.call_method(&obj, "toString", "()Ljava/lang/String;", vec![]);
        let variant = env.translate_error(variant)?;
        let variant_str = env
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        match variant_str.as_str() {
            "CHAT" => Ok(ChatMessageType::Chat {
                inner: ChatMessageTypeStruct::from_raw(env, obj)?,
            }),
            "SYSTEM" => Ok(ChatMessageType::System {
                inner: ChatMessageTypeStruct::from_raw(env, obj)?,
            }),
            "ACTION_BAR" => Ok(ChatMessageType::ActionBar {
                inner: ChatMessageTypeStruct::from_raw(env, obj)?,
            }),

            _ => Err(eyre::eyre!("String gaven for variant was invalid").into()),
        }
    }
}

#[repr(C)]
pub struct ChatMessageTypeStruct<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for ChatMessageType<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        match self {
            Self::Chat { inner } => inner.0.clone(),
            Self::System { inner } => inner.0.clone(),
            Self::ActionBar { inner } => inner.0.clone(),
        }
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        match self {
            Self::Chat { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::System { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::ActionBar { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
        }
    }
}
impl<'mc> JNIInstantiatable<'mc> for ChatMessageType<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate ChatMessageType from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "net/md_5/bungee/api/ChatMessageType")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a ChatMessageType object, got {}",
                name
            )
            .into())
        } else {
            let variant = env.call_method(&obj, "toString", "()Ljava/lang/String;", vec![]);
            let variant = env.translate_error(variant)?;
            let variant_str = env
                .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
                .to_string_lossy()
                .to_string();
            match variant_str.as_str() {
                "CHAT" => Ok(ChatMessageType::Chat {
                    inner: ChatMessageTypeStruct::from_raw(env, obj)?,
                }),
                "SYSTEM" => Ok(ChatMessageType::System {
                    inner: ChatMessageTypeStruct::from_raw(env, obj)?,
                }),
                "ACTION_BAR" => Ok(ChatMessageType::ActionBar {
                    inner: ChatMessageTypeStruct::from_raw(env, obj)?,
                }),
                _ => Err(eyre::eyre!("String gaven for variant was invalid").into()),
            }
        }
    }
}

impl<'mc> JNIRaw<'mc> for ChatMessageTypeStruct<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for ChatMessageTypeStruct<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate ChatMessageTypeStruct from null object."
            )
            .into());
        }
        let (valid, name) = env.validate_name(&obj, "net/md_5/bungee/api/ChatMessageType")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a ChatMessageTypeStruct object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> ChatMessageTypeStruct<'mc> {
    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}

#[repr(C)]
pub struct ChatColor<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for ChatColor<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for ChatColor<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate ChatColor from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "net/md_5/bungee/api/ChatColor")?;
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
}

impl<'mc> ChatColor<'mc> {
    pub fn color(&self) -> Result<(u8, u8, u8), Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/awt/Color;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getColor", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        let r = self.jni_ref().call_method(
            unsafe { jni::objects::JObject::from_raw(res.as_jni().l) },
            "getRed",
            "(V)I",
            vec![],
        );
        let r = self.jni_ref().translate_error(r)?.i()? as u8;
        let g = self.jni_ref().call_method(
            unsafe { jni::objects::JObject::from_raw(res.as_jni().l) },
            "getGreen",
            "(V)I",
            vec![],
        );
        let g = self.jni_ref().translate_error(g)?.i()? as u8;
        let b = self.jni_ref().call_method(
            unsafe { jni::objects::JObject::from_raw(res.as_jni().l) },
            "getBlue",
            "(V)I",
            vec![],
        );
        let b = self.jni_ref().translate_error(b)?.i()? as u8;
        Ok((r, g, b))
    }

    pub fn get_by_char(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: u16,
    ) -> Result<crate::bungee::api::ChatColor<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(C)Lnet/md_5/bungee/api/ChatColor;");
        let val_1 = jni::objects::JValueGen::Char(arg0);
        let cls = jni.find_class("net/md_5/bungee/api/ChatColor");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(
            cls,
            "getByChar",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        crate::bungee::api::ChatColor::from_raw(&jni, obj)
    }

    pub fn strip_color(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<String>,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/String;)Ljava/lang/String;");
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            jni.new_string(arg0.into())?,
        ));
        let cls = jni.find_class("java/lang/String");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(
            cls,
            "stripColor",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = jni.translate_error(res)?;
        Ok(jni
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }

    pub fn translate_alternate_color_codes(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: u16,
        arg1: impl Into<String>,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let sig = String::from("(CLjava/lang/String;)Ljava/lang/String;");
        let val_1 = jni::objects::JValueGen::Char(arg0);
        let val_2 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            jni.new_string(arg1.into())?,
        ));
        let cls = jni.find_class("java/lang/String");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(
            cls,
            "translateAlternateColorCodes",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
            ],
        );
        let res = jni.translate_error(res)?;
        Ok(jni
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }

    pub fn name(&self) -> Result<String, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Ljava/lang/String;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "name", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }

    pub fn equals(
        &self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/Object;)Z");
        let val_1 = jni::objects::JValueGen::Object(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "equals",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    #[doc(hidden)]
    pub fn internal_to_string(&self) -> Result<String, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/String;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "toString", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }

    pub fn hash_code(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hashCode", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }

    pub fn of_with_string(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<String>,
    ) -> Result<crate::bungee::api::ChatColor<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Ljava/lang/String;";
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            jni.new_string(arg0.into())?,
        ));
        args.push(val_1);
        sig += ")Lnet/md_5/bungee/api/ChatColor;";
        let cls = jni.find_class("net/md_5/bungee/api/ChatColor");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "of", sig.as_str(), args);
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        crate::bungee::api::ChatColor::from_raw(&jni, obj)
    }

    pub fn ordinal(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "ordinal", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    // SUPER CLASS: Object

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}

impl<'mc> std::string::ToString for ChatColor<'mc> {
    fn to_string(&self) -> String {
        match &self.internal_to_string() {
            Ok(a) => a.clone(),
            Err(err) => format!("Error calling ChatColor.toString: {}", err),
        }
    }
}

pub mod chat;
