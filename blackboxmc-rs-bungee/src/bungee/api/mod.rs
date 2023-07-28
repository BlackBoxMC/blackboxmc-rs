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
}
pub mod chat;
