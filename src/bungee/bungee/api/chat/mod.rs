#![allow(deprecated)]
use crate::JNIRaw;
pub struct TextComponent<'mc>(
    pub(crate) crate::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> crate::JNIRaw<'mc> for TextComponent<'mc> {
    fn jni_ref(&self) -> crate::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> TextComponent<'mc> {
    pub fn new(
        jni: crate::SharedJNIEnv<'mc>,
        arg0: std::option::Option<impl Into<String>>,
    ) -> Result<crate::bungee::bungee::api::chat::TextComponent<'mc>, Box<dyn std::error::Error>>
    {
        let val_0 = jni::objects::JObject::from(jni.new_string(arg0.unwrap().into()).unwrap());
        let cls = &jni.find_class("net/md_5/bungee/api/chat/TextComponent")?;
        let res = jni.new_object(
            cls,
            "(Ljava/lang/String;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        let ret = { crate::bungee::bungee::api::chat::TextComponent(jni, res) };
        Ok(ret)
    }
    pub fn new_with_text_component(
        jni: crate::SharedJNIEnv<'mc>,
        arg0: std::option::Option<impl Into<crate::bungee::bungee::api::chat::TextComponent<'mc>>>,
    ) -> Result<crate::bungee::bungee::api::chat::TextComponent<'mc>, Box<dyn std::error::Error>>
    {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.unwrap().into().1.clone()) };
        let cls = &jni.find_class("net/md_5/bungee/api/chat/TextComponent")?;
        let res = jni.new_object(
            cls,
            "(Lnet/md_5/bungee/api/chat/TextComponent;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        let ret = { crate::bungee::bungee::api::chat::TextComponent(jni, res) };
        Ok(ret)
    }
    pub fn from_raw(
        env: &crate::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate TextComponent from null object.").into());
        }
        let cls = env.jni.borrow().get_object_class(&obj)?;
        let name_raw = env.call_method(cls, "getName", "()Ljava/lang/String;", &[])?;
        let oh = name_raw.l()?.into();
        let what = env.get_string(&oh)?;
        let name = what.to_string_lossy();
        if !name.ends_with("TextComponent") {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a TextComponent object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
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
    pub fn duplicate(
        &mut self,
    ) -> Result<crate::bungee::bungee::api::chat::TextComponent<'mc>, Box<dyn std::error::Error>>
    {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "duplicate",
            "()Lnet/md_5/bungee/api/chat/TextComponent;",
            &[],
        )?;
        let ret = {
            crate::bungee::bungee::api::chat::TextComponent(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn text(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getText",
            "()Ljava/lang/String;",
            &[],
        )?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    pub fn set_text(&mut self, arg0: impl Into<String>) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        self.jni_ref().call_method(
            &self.jni_object(),
            "setText",
            "(Ljava/lang/String;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn set_extra(
        &mut self,
        arg0: Vec<impl Into<crate::bungee::bungee::api::chat::BaseComponent<'mc>>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let raw_val_0 = self
            .jni_ref()
            .new_object("java/util/ArrayList", "()V", &[])
            .unwrap();
        for v in arg0 {
            let map_val_0 = unsafe { jni::objects::JObject::from_raw(v.into().1.clone()) };
            self.jni_ref().call_method(
                &raw_val_0,
                "add",
                "(Ljava/Lang/Object)V",
                &[jni::objects::JValueGen::from(&map_val_0)],
            )?;
        }
        let val_0 = jni::objects::JValueGen::Object(raw_val_0);
        self.jni_ref().call_method(
            &self.jni_object(),
            "setExtra",
            "(Ljava/util/List;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn set_color(
        &mut self,
        arg0: impl Into<crate::bungee::bungee::api::ChatColor<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        self.jni_ref().call_method(
            &self.jni_object(),
            "setColor",
            "(Lnet/md_5/bungee/api/ChatColor;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn copy_formatting_with_base_component(
        &mut self,
        arg0: std::option::Option<impl Into<crate::bungee::bungee::api::chat::BaseComponent<'mc>>>,
        arg1: std::option::Option<
            impl Into<crate::bungee::bungee::api::chat::ComponentBuilderFormatRetention<'mc>>,
        >,
        arg2: std::option::Option<bool>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.unwrap().into().1.clone()) };
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg1.unwrap().into().1.clone()) };
        // 0
        let val_2 = jni::objects::JValueGen::Bool(arg2.unwrap().into());
        self.jni_ref().call_method(&self.jni_object(),"copyFormatting","(Lnet/md_5/bungee/api/chat/BaseComponent;Lnet/md_5/bungee/api/chat/ComponentBuilder$FormatRetention;Z)V",&[jni::objects::JValueGen::from(&val_0),jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)])?;
        Ok(())
    }
    pub fn color(
        &mut self,
    ) -> Result<crate::bungee::bungee::api::ChatColor<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getColor",
            "()Lnet/md_5/bungee/api/ChatColor;",
            &[],
        )?;
        let ret = {
            crate::bungee::bungee::api::ChatColor(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn retain(
        &mut self,
        arg0: impl Into<crate::bungee::bungee::api::chat::ComponentBuilderFormatRetention<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        self.jni_ref().call_method(
            &self.jni_object(),
            "retain",
            "(Lnet/md_5/bungee/api/chat/ComponentBuilder$FormatRetention;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    #[deprecated]
    pub fn duplicate_without_formatting(
        &mut self,
    ) -> Result<crate::bungee::bungee::api::chat::BaseComponent<'mc>, Box<dyn std::error::Error>>
    {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "duplicateWithoutFormatting",
            "()Lnet/md_5/bungee/api/chat/BaseComponent;",
            &[],
        )?;
        let ret = {
            crate::bungee::bungee::api::chat::BaseComponent(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn to_legacy_text(
        jni: crate::SharedJNIEnv<'mc>,
        _arg0: std::option::Option<
            Vec<impl Into<crate::bungee::bungee::api::chat::BaseComponent<'mc>>>,
        >,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let cls = &jni.find_class("java/lang/String")?;
        let res = jni.call_static_method(
            cls,
            "toLegacyText",
            "(Lnet/md_5/bungee/api/chat/BaseComponent;)Ljava/lang/String;",
            &[],
        )?;
        Ok(jni
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    pub fn to_plain_text(
        jni: crate::SharedJNIEnv<'mc>,
        _arg0: std::option::Option<
            Vec<impl Into<crate::bungee::bungee::api::chat::BaseComponent<'mc>>>,
        >,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let cls = &jni.find_class("java/lang/String")?;
        let res = jni.call_static_method(
            cls,
            "toPlainText",
            "(Lnet/md_5/bungee/api/chat/BaseComponent;)Ljava/lang/String;",
            &[],
        )?;
        Ok(jni
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    pub fn color_raw(
        &mut self,
    ) -> Result<crate::bungee::bungee::api::ChatColor<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getColorRaw",
            "()Lnet/md_5/bungee/api/ChatColor;",
            &[],
        )?;
        let ret = {
            crate::bungee::bungee::api::ChatColor(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn font(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getFont",
            "()Ljava/lang/String;",
            &[],
        )?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    pub fn font_raw(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getFontRaw",
            "()Ljava/lang/String;",
            &[],
        )?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    pub fn is_bold(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isBold", "()Z", &[])?;
        Ok(res.z().unwrap())
    }
    pub fn is_bold_raw(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isBoldRaw", "()Z", &[])?;
        Ok(res.z().unwrap())
    }
    pub fn is_italic(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isItalic", "()Z", &[])?;
        Ok(res.z().unwrap())
    }
    pub fn is_italic_raw(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isItalicRaw", "()Z", &[])?;
        Ok(res.z().unwrap())
    }
    pub fn is_underlined(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isUnderlined", "()Z", &[])?;
        Ok(res.z().unwrap())
    }
    pub fn is_underlined_raw(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isUnderlinedRaw", "()Z", &[])?;
        Ok(res.z().unwrap())
    }
    pub fn is_strikethrough(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isStrikethrough", "()Z", &[])?;
        Ok(res.z().unwrap())
    }
    pub fn is_strikethrough_raw(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isStrikethroughRaw", "()Z", &[])?;
        Ok(res.z().unwrap())
    }
    pub fn is_obfuscated(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isObfuscated", "()Z", &[])?;
        Ok(res.z().unwrap())
    }
    pub fn is_obfuscated_raw(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isObfuscatedRaw", "()Z", &[])?;
        Ok(res.z().unwrap())
    }
    pub fn add_extra_with_base_component(
        &mut self,
        arg0: std::option::Option<impl Into<String>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 =
            jni::objects::JObject::from(self.jni_ref().new_string(arg0.unwrap().into()).unwrap());
        self.jni_ref().call_method(
            &self.jni_object(),
            "addExtra",
            "(Ljava/lang/String;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn has_formatting(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasFormatting", "()Z", &[])?;
        Ok(res.z().unwrap())
    }
    pub fn set_font(&mut self, arg0: impl Into<String>) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        self.jni_ref().call_method(
            &self.jni_object(),
            "setFont",
            "(Ljava/lang/String;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn set_bold(&mut self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        // -2
        let val_0 = jni::objects::JValueGen::Bool(arg0.into());
        self.jni_ref().call_method(
            &self.jni_object(),
            "setBold",
            "(Z)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn set_italic(&mut self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        // -2
        let val_0 = jni::objects::JValueGen::Bool(arg0.into());
        self.jni_ref().call_method(
            &self.jni_object(),
            "setItalic",
            "(Z)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn set_underlined(&mut self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        // -2
        let val_0 = jni::objects::JValueGen::Bool(arg0.into());
        self.jni_ref().call_method(
            &self.jni_object(),
            "setUnderlined",
            "(Z)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn set_strikethrough(&mut self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        // -2
        let val_0 = jni::objects::JValueGen::Bool(arg0.into());
        self.jni_ref().call_method(
            &self.jni_object(),
            "setStrikethrough",
            "(Z)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn set_obfuscated(&mut self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        // -2
        let val_0 = jni::objects::JValueGen::Bool(arg0.into());
        self.jni_ref().call_method(
            &self.jni_object(),
            "setObfuscated",
            "(Z)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn set_insertion(
        &mut self,
        arg0: impl Into<String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        self.jni_ref().call_method(
            &self.jni_object(),
            "setInsertion",
            "(Ljava/lang/String;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn set_click_event(
        &mut self,
        arg0: impl Into<crate::bungee::bungee::api::chat::ClickEvent<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        self.jni_ref().call_method(
            &self.jni_object(),
            "setClickEvent",
            "(Lnet/md_5/bungee/api/chat/ClickEvent;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn set_hover_event(
        &mut self,
        arg0: impl Into<crate::bungee::bungee::api::chat::HoverEvent<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        self.jni_ref().call_method(
            &self.jni_object(),
            "setHoverEvent",
            "(Lnet/md_5/bungee/api/chat/HoverEvent;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn insertion(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getInsertion",
            "()Ljava/lang/String;",
            &[],
        )?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    pub fn click_event(
        &mut self,
    ) -> Result<crate::bungee::bungee::api::chat::ClickEvent<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getClickEvent",
            "()Lnet/md_5/bungee/api/chat/ClickEvent;",
            &[],
        )?;
        let ret = {
            crate::bungee::bungee::api::chat::ClickEvent(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn hover_event(
        &mut self,
    ) -> Result<crate::bungee::bungee::api::chat::HoverEvent<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getHoverEvent",
            "()Lnet/md_5/bungee/api/chat/HoverEvent;",
            &[],
        )?;
        let ret = {
            crate::bungee::bungee::api::chat::HoverEvent(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
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
impl<'mc> Into<crate::bungee::bungee::api::chat::BaseComponent<'mc>> for TextComponent<'mc> {
    fn into(self) -> crate::bungee::bungee::api::chat::BaseComponent<'mc> {
        crate::bungee::bungee::api::chat::BaseComponent::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
pub struct HoverEvent<'mc>(
    pub(crate) crate::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
pub struct HoverEventAction<'mc>(
    pub(crate) crate::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> crate::JNIRaw<'mc> for HoverEventAction<'mc> {
    fn jni_ref(&self) -> crate::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> HoverEventAction<'mc> {
    pub fn from_raw(
        env: &crate::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate HoverEventAction from null object.").into(),
            );
        }
        let cls = env.jni.borrow().get_object_class(&obj)?;
        let name_raw = env.call_method(cls, "getName", "()Ljava/lang/String;", &[])?;
        let oh = name_raw.l()?.into();
        let what = env.get_string(&oh)?;
        let name = what.to_string_lossy();
        if !name.ends_with("HoverEventAction") {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a HoverEventAction object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    pub fn value_of_with_string(
        jni: crate::SharedJNIEnv<'mc>,
        arg0: std::option::Option<jni::objects::JClass<'mc>>,
        arg1: std::option::Option<impl Into<String>>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let val_0 = arg0.unwrap();
        let val_1 = jni::objects::JObject::from(jni.new_string(arg1.unwrap().into()).unwrap());
        let cls = &jni.find_class("java/lang/Enum")?;
        let res = jni.call_static_method(
            cls,
            "valueOf",
            "(Ljava/lang/Class;Ljava/lang/String;)Ljava/lang/Enum;",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        )?;
        Ok(res.l().unwrap())
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
    pub fn compare_to_with_object(
        &mut self,
        arg0: std::option::Option<jni::objects::JObject<'mc>>,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        let val_0 = arg0.unwrap();
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "compareTo",
            "(Ljava/lang/Enum;)I",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(res.i().unwrap())
    }
    pub fn describe_constable(
        &mut self,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "describeConstable",
            "()Ljava/util/Optional;",
            &[],
        )?;
        Ok(res.l().unwrap())
    }
    pub fn declaring_class(
        &mut self,
    ) -> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getDeclaringClass",
            "()Ljava/lang/Class;",
            &[],
        )?;
        Ok(unsafe { jni::objects::JClass::from_raw(res.as_jni().l) })
    }
    pub fn ordinal(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "ordinal", "()I", &[])?;
        Ok(res.i().unwrap())
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
impl<'mc> crate::JNIRaw<'mc> for HoverEvent<'mc> {
    fn jni_ref(&self) -> crate::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> HoverEvent<'mc> {
    pub fn new_with_hover_eventaction(
        jni: crate::SharedJNIEnv<'mc>,
        arg0: impl Into<crate::bungee::bungee::api::chat::HoverEventAction<'mc>>,
        arg1: std::option::Option<
            Vec<impl Into<crate::bungee::bungee::api::chat::hover::content::Content<'mc>>>,
        >,
    ) -> Result<crate::bungee::bungee::api::chat::HoverEvent<'mc>, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let raw_val_1 = jni.new_object("java/util/ArrayList", "()V", &[]).unwrap();
        for v in arg1.unwrap() {
            let map_val_0 = unsafe { jni::objects::JObject::from_raw(v.into().1.clone()) };
            jni.call_method(
                &raw_val_1,
                "add",
                "(Ljava/Lang/Object)V",
                &[jni::objects::JValueGen::from(&map_val_0)],
            )?;
        }
        let val_1 = jni::objects::JValueGen::Object(raw_val_1);
        let cls = &jni.find_class("net/md_5/bungee/api/chat/HoverEvent")?;
        let res = jni.new_object(
            cls,
            "(Lnet/md_5/bungee/api/chat/HoverEvent$Action;Ljava/util/List;)V",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        )?;
        let ret = { crate::bungee::bungee::api::chat::HoverEvent(jni, res) };
        Ok(ret)
    }
    pub fn from_raw(
        env: &crate::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate HoverEvent from null object.").into());
        }
        let cls = env.jni.borrow().get_object_class(&obj)?;
        let name_raw = env.call_method(cls, "getName", "()Ljava/lang/String;", &[])?;
        let oh = name_raw.l()?.into();
        let what = env.get_string(&oh)?;
        let name = what.to_string_lossy();
        if !name.ends_with("HoverEvent") {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a HoverEvent object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
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
    pub fn class(&mut self) -> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getClass",
            "()Ljava/lang/Class;",
            &[],
        )?;
        Ok(unsafe { jni::objects::JClass::from_raw(res.as_jni().l) })
    }
    pub fn is_legacy(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isLegacy", "()Z", &[])?;
        Ok(res.z().unwrap())
    }
    pub fn action(
        &mut self,
    ) -> Result<crate::bungee::bungee::api::chat::HoverEventAction<'mc>, Box<dyn std::error::Error>>
    {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getAction",
            "()Lnet/md_5/bungee/api/chat/HoverEvent$Action;",
            &[],
        )?;
        let ret = {
            crate::bungee::bungee::api::chat::HoverEventAction(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn add_content(
        &mut self,
        arg0: impl Into<crate::bungee::bungee::api::chat::hover::content::Content<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        self.jni_ref().call_method(
            &self.jni_object(),
            "addContent",
            "(Lnet/md_5/bungee/api/chat/hover/content/Content;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn set_legacy(&mut self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        // -2
        let val_0 = jni::objects::JValueGen::Bool(arg0.into());
        self.jni_ref().call_method(
            &self.jni_object(),
            "setLegacy",
            "(Z)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
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
pub struct BaseComponent<'mc>(
    pub(crate) crate::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> crate::JNIRaw<'mc> for BaseComponent<'mc> {
    fn jni_ref(&self) -> crate::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> BaseComponent<'mc> {
    #[deprecated]
    pub fn new(
        jni: crate::SharedJNIEnv<'mc>,
    ) -> Result<crate::bungee::bungee::api::chat::BaseComponent<'mc>, Box<dyn std::error::Error>>
    {
        let cls = &jni.find_class("net/md_5/bungee/api/chat/BaseComponent")?;
        let res = jni.new_object(cls, "()V", &[])?;
        let ret = { crate::bungee::bungee::api::chat::BaseComponent(jni, res) };
        Ok(ret)
    }
    pub fn from_raw(
        env: &crate::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate BaseComponent from null object.").into());
        }
        let cls = env.jni.borrow().get_object_class(&obj)?;
        let name_raw = env.call_method(cls, "getName", "()Ljava/lang/String;", &[])?;
        let oh = name_raw.l()?.into();
        let what = env.get_string(&oh)?;
        let name = what.to_string_lossy();
        if !name.ends_with("BaseComponent") {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a BaseComponent object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
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
    pub fn duplicate(
        &mut self,
    ) -> Result<crate::bungee::bungee::api::chat::BaseComponent<'mc>, Box<dyn std::error::Error>>
    {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "duplicate",
            "()Lnet/md_5/bungee/api/chat/BaseComponent;",
            &[],
        )?;
        let ret = {
            crate::bungee::bungee::api::chat::BaseComponent(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn set_extra(
        &mut self,
        arg0: Vec<impl Into<crate::bungee::bungee::api::chat::BaseComponent<'mc>>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let raw_val_0 = self
            .jni_ref()
            .new_object("java/util/ArrayList", "()V", &[])
            .unwrap();
        for v in arg0 {
            let map_val_0 = unsafe { jni::objects::JObject::from_raw(v.into().1.clone()) };
            self.jni_ref().call_method(
                &raw_val_0,
                "add",
                "(Ljava/Lang/Object)V",
                &[jni::objects::JValueGen::from(&map_val_0)],
            )?;
        }
        let val_0 = jni::objects::JValueGen::Object(raw_val_0);
        self.jni_ref().call_method(
            &self.jni_object(),
            "setExtra",
            "(Ljava/util/List;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn set_color(
        &mut self,
        arg0: impl Into<crate::bungee::bungee::api::ChatColor<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        self.jni_ref().call_method(
            &self.jni_object(),
            "setColor",
            "(Lnet/md_5/bungee/api/ChatColor;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn copy_formatting_with_base_component(
        &mut self,
        arg0: std::option::Option<impl Into<crate::bungee::bungee::api::chat::BaseComponent<'mc>>>,
        arg1: std::option::Option<
            impl Into<crate::bungee::bungee::api::chat::ComponentBuilderFormatRetention<'mc>>,
        >,
        arg2: std::option::Option<bool>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.unwrap().into().1.clone()) };
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg1.unwrap().into().1.clone()) };
        // 0
        let val_2 = jni::objects::JValueGen::Bool(arg2.unwrap().into());
        self.jni_ref().call_method(&self.jni_object(),"copyFormatting","(Lnet/md_5/bungee/api/chat/BaseComponent;Lnet/md_5/bungee/api/chat/ComponentBuilder$FormatRetention;Z)V",&[jni::objects::JValueGen::from(&val_0),jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)])?;
        Ok(())
    }
    pub fn color(
        &mut self,
    ) -> Result<crate::bungee::bungee::api::ChatColor<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getColor",
            "()Lnet/md_5/bungee/api/ChatColor;",
            &[],
        )?;
        let ret = {
            crate::bungee::bungee::api::ChatColor(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn retain(
        &mut self,
        arg0: impl Into<crate::bungee::bungee::api::chat::ComponentBuilderFormatRetention<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        self.jni_ref().call_method(
            &self.jni_object(),
            "retain",
            "(Lnet/md_5/bungee/api/chat/ComponentBuilder$FormatRetention;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    #[deprecated]
    pub fn duplicate_without_formatting(
        &mut self,
    ) -> Result<crate::bungee::bungee::api::chat::BaseComponent<'mc>, Box<dyn std::error::Error>>
    {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "duplicateWithoutFormatting",
            "()Lnet/md_5/bungee/api/chat/BaseComponent;",
            &[],
        )?;
        let ret = {
            crate::bungee::bungee::api::chat::BaseComponent(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn to_legacy_text(
        jni: crate::SharedJNIEnv<'mc>,
        _arg0: std::option::Option<
            Vec<impl Into<crate::bungee::bungee::api::chat::BaseComponent<'mc>>>,
        >,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let cls = &jni.find_class("java/lang/String")?;
        let res = jni.call_static_method(
            cls,
            "toLegacyText",
            "(Lnet/md_5/bungee/api/chat/BaseComponent;)Ljava/lang/String;",
            &[],
        )?;
        Ok(jni
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    pub fn to_plain_text(
        jni: crate::SharedJNIEnv<'mc>,
        _arg0: std::option::Option<
            Vec<impl Into<crate::bungee::bungee::api::chat::BaseComponent<'mc>>>,
        >,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let cls = &jni.find_class("java/lang/String")?;
        let res = jni.call_static_method(
            cls,
            "toPlainText",
            "(Lnet/md_5/bungee/api/chat/BaseComponent;)Ljava/lang/String;",
            &[],
        )?;
        Ok(jni
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    pub fn color_raw(
        &mut self,
    ) -> Result<crate::bungee::bungee::api::ChatColor<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getColorRaw",
            "()Lnet/md_5/bungee/api/ChatColor;",
            &[],
        )?;
        let ret = {
            crate::bungee::bungee::api::ChatColor(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn font(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getFont",
            "()Ljava/lang/String;",
            &[],
        )?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    pub fn font_raw(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getFontRaw",
            "()Ljava/lang/String;",
            &[],
        )?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    pub fn is_bold(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isBold", "()Z", &[])?;
        Ok(res.z().unwrap())
    }
    pub fn is_bold_raw(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isBoldRaw", "()Z", &[])?;
        Ok(res.z().unwrap())
    }
    pub fn is_italic(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isItalic", "()Z", &[])?;
        Ok(res.z().unwrap())
    }
    pub fn is_italic_raw(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isItalicRaw", "()Z", &[])?;
        Ok(res.z().unwrap())
    }
    pub fn is_underlined(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isUnderlined", "()Z", &[])?;
        Ok(res.z().unwrap())
    }
    pub fn is_underlined_raw(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isUnderlinedRaw", "()Z", &[])?;
        Ok(res.z().unwrap())
    }
    pub fn is_strikethrough(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isStrikethrough", "()Z", &[])?;
        Ok(res.z().unwrap())
    }
    pub fn is_strikethrough_raw(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isStrikethroughRaw", "()Z", &[])?;
        Ok(res.z().unwrap())
    }
    pub fn is_obfuscated(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isObfuscated", "()Z", &[])?;
        Ok(res.z().unwrap())
    }
    pub fn is_obfuscated_raw(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isObfuscatedRaw", "()Z", &[])?;
        Ok(res.z().unwrap())
    }
    pub fn add_extra_with_base_component(
        &mut self,
        arg0: std::option::Option<impl Into<String>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 =
            jni::objects::JObject::from(self.jni_ref().new_string(arg0.unwrap().into()).unwrap());
        self.jni_ref().call_method(
            &self.jni_object(),
            "addExtra",
            "(Ljava/lang/String;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn has_formatting(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasFormatting", "()Z", &[])?;
        Ok(res.z().unwrap())
    }
    pub fn set_font(&mut self, arg0: impl Into<String>) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        self.jni_ref().call_method(
            &self.jni_object(),
            "setFont",
            "(Ljava/lang/String;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn set_bold(&mut self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        // -2
        let val_0 = jni::objects::JValueGen::Bool(arg0.into());
        self.jni_ref().call_method(
            &self.jni_object(),
            "setBold",
            "(Z)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn set_italic(&mut self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        // -2
        let val_0 = jni::objects::JValueGen::Bool(arg0.into());
        self.jni_ref().call_method(
            &self.jni_object(),
            "setItalic",
            "(Z)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn set_underlined(&mut self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        // -2
        let val_0 = jni::objects::JValueGen::Bool(arg0.into());
        self.jni_ref().call_method(
            &self.jni_object(),
            "setUnderlined",
            "(Z)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn set_strikethrough(&mut self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        // -2
        let val_0 = jni::objects::JValueGen::Bool(arg0.into());
        self.jni_ref().call_method(
            &self.jni_object(),
            "setStrikethrough",
            "(Z)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn set_obfuscated(&mut self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        // -2
        let val_0 = jni::objects::JValueGen::Bool(arg0.into());
        self.jni_ref().call_method(
            &self.jni_object(),
            "setObfuscated",
            "(Z)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn set_insertion(
        &mut self,
        arg0: impl Into<String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        self.jni_ref().call_method(
            &self.jni_object(),
            "setInsertion",
            "(Ljava/lang/String;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn set_click_event(
        &mut self,
        arg0: impl Into<crate::bungee::bungee::api::chat::ClickEvent<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        self.jni_ref().call_method(
            &self.jni_object(),
            "setClickEvent",
            "(Lnet/md_5/bungee/api/chat/ClickEvent;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn set_hover_event(
        &mut self,
        arg0: impl Into<crate::bungee::bungee::api::chat::HoverEvent<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        self.jni_ref().call_method(
            &self.jni_object(),
            "setHoverEvent",
            "(Lnet/md_5/bungee/api/chat/HoverEvent;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn insertion(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getInsertion",
            "()Ljava/lang/String;",
            &[],
        )?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    pub fn click_event(
        &mut self,
    ) -> Result<crate::bungee::bungee::api::chat::ClickEvent<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getClickEvent",
            "()Lnet/md_5/bungee/api/chat/ClickEvent;",
            &[],
        )?;
        let ret = {
            crate::bungee::bungee::api::chat::ClickEvent(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn hover_event(
        &mut self,
    ) -> Result<crate::bungee::bungee::api::chat::HoverEvent<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getHoverEvent",
            "()Lnet/md_5/bungee/api/chat/HoverEvent;",
            &[],
        )?;
        let ret = {
            crate::bungee::bungee::api::chat::HoverEvent(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
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
pub struct SelectorComponent<'mc>(
    pub(crate) crate::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> crate::JNIRaw<'mc> for SelectorComponent<'mc> {
    fn jni_ref(&self) -> crate::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> SelectorComponent<'mc> {
    pub fn new_with_string(
        jni: crate::SharedJNIEnv<'mc>,
        arg0: std::option::Option<
            impl Into<crate::bungee::bungee::api::chat::SelectorComponent<'mc>>,
        >,
    ) -> Result<crate::bungee::bungee::api::chat::SelectorComponent<'mc>, Box<dyn std::error::Error>>
    {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.unwrap().into().1.clone()) };
        let cls = &jni.find_class("net/md_5/bungee/api/chat/SelectorComponent")?;
        let res = jni.new_object(
            cls,
            "(Lnet/md_5/bungee/api/chat/SelectorComponent;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        let ret = { crate::bungee::bungee::api::chat::SelectorComponent(jni, res) };
        Ok(ret)
    }
    pub fn from_raw(
        env: &crate::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate SelectorComponent from null object.").into(),
            );
        }
        let cls = env.jni.borrow().get_object_class(&obj)?;
        let name_raw = env.call_method(cls, "getName", "()Ljava/lang/String;", &[])?;
        let oh = name_raw.l()?.into();
        let what = env.get_string(&oh)?;
        let name = what.to_string_lossy();
        if !name.ends_with("SelectorComponent") {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a SelectorComponent object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
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
    pub fn duplicate(
        &mut self,
    ) -> Result<crate::bungee::bungee::api::chat::SelectorComponent<'mc>, Box<dyn std::error::Error>>
    {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "duplicate",
            "()Lnet/md_5/bungee/api/chat/SelectorComponent;",
            &[],
        )?;
        let ret = {
            crate::bungee::bungee::api::chat::SelectorComponent(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn selector(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getSelector",
            "()Ljava/lang/String;",
            &[],
        )?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    pub fn set_selector(
        &mut self,
        arg0: impl Into<String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        self.jni_ref().call_method(
            &self.jni_object(),
            "setSelector",
            "(Ljava/lang/String;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn set_extra(
        &mut self,
        arg0: Vec<impl Into<crate::bungee::bungee::api::chat::BaseComponent<'mc>>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let raw_val_0 = self
            .jni_ref()
            .new_object("java/util/ArrayList", "()V", &[])
            .unwrap();
        for v in arg0 {
            let map_val_0 = unsafe { jni::objects::JObject::from_raw(v.into().1.clone()) };
            self.jni_ref().call_method(
                &raw_val_0,
                "add",
                "(Ljava/Lang/Object)V",
                &[jni::objects::JValueGen::from(&map_val_0)],
            )?;
        }
        let val_0 = jni::objects::JValueGen::Object(raw_val_0);
        self.jni_ref().call_method(
            &self.jni_object(),
            "setExtra",
            "(Ljava/util/List;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn set_color(
        &mut self,
        arg0: impl Into<crate::bungee::bungee::api::ChatColor<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        self.jni_ref().call_method(
            &self.jni_object(),
            "setColor",
            "(Lnet/md_5/bungee/api/ChatColor;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn copy_formatting_with_base_component(
        &mut self,
        arg0: std::option::Option<impl Into<crate::bungee::bungee::api::chat::BaseComponent<'mc>>>,
        arg1: std::option::Option<
            impl Into<crate::bungee::bungee::api::chat::ComponentBuilderFormatRetention<'mc>>,
        >,
        arg2: std::option::Option<bool>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.unwrap().into().1.clone()) };
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg1.unwrap().into().1.clone()) };
        // 0
        let val_2 = jni::objects::JValueGen::Bool(arg2.unwrap().into());
        self.jni_ref().call_method(&self.jni_object(),"copyFormatting","(Lnet/md_5/bungee/api/chat/BaseComponent;Lnet/md_5/bungee/api/chat/ComponentBuilder$FormatRetention;Z)V",&[jni::objects::JValueGen::from(&val_0),jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)])?;
        Ok(())
    }
    pub fn color(
        &mut self,
    ) -> Result<crate::bungee::bungee::api::ChatColor<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getColor",
            "()Lnet/md_5/bungee/api/ChatColor;",
            &[],
        )?;
        let ret = {
            crate::bungee::bungee::api::ChatColor(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn retain(
        &mut self,
        arg0: impl Into<crate::bungee::bungee::api::chat::ComponentBuilderFormatRetention<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        self.jni_ref().call_method(
            &self.jni_object(),
            "retain",
            "(Lnet/md_5/bungee/api/chat/ComponentBuilder$FormatRetention;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    #[deprecated]
    pub fn duplicate_without_formatting(
        &mut self,
    ) -> Result<crate::bungee::bungee::api::chat::BaseComponent<'mc>, Box<dyn std::error::Error>>
    {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "duplicateWithoutFormatting",
            "()Lnet/md_5/bungee/api/chat/BaseComponent;",
            &[],
        )?;
        let ret = {
            crate::bungee::bungee::api::chat::BaseComponent(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn to_legacy_text(
        jni: crate::SharedJNIEnv<'mc>,
        _arg0: std::option::Option<
            Vec<impl Into<crate::bungee::bungee::api::chat::BaseComponent<'mc>>>,
        >,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let cls = &jni.find_class("java/lang/String")?;
        let res = jni.call_static_method(
            cls,
            "toLegacyText",
            "(Lnet/md_5/bungee/api/chat/BaseComponent;)Ljava/lang/String;",
            &[],
        )?;
        Ok(jni
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    pub fn to_plain_text(
        jni: crate::SharedJNIEnv<'mc>,
        _arg0: std::option::Option<
            Vec<impl Into<crate::bungee::bungee::api::chat::BaseComponent<'mc>>>,
        >,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let cls = &jni.find_class("java/lang/String")?;
        let res = jni.call_static_method(
            cls,
            "toPlainText",
            "(Lnet/md_5/bungee/api/chat/BaseComponent;)Ljava/lang/String;",
            &[],
        )?;
        Ok(jni
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    pub fn color_raw(
        &mut self,
    ) -> Result<crate::bungee::bungee::api::ChatColor<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getColorRaw",
            "()Lnet/md_5/bungee/api/ChatColor;",
            &[],
        )?;
        let ret = {
            crate::bungee::bungee::api::ChatColor(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn font(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getFont",
            "()Ljava/lang/String;",
            &[],
        )?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    pub fn font_raw(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getFontRaw",
            "()Ljava/lang/String;",
            &[],
        )?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    pub fn is_bold(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isBold", "()Z", &[])?;
        Ok(res.z().unwrap())
    }
    pub fn is_bold_raw(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isBoldRaw", "()Z", &[])?;
        Ok(res.z().unwrap())
    }
    pub fn is_italic(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isItalic", "()Z", &[])?;
        Ok(res.z().unwrap())
    }
    pub fn is_italic_raw(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isItalicRaw", "()Z", &[])?;
        Ok(res.z().unwrap())
    }
    pub fn is_underlined(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isUnderlined", "()Z", &[])?;
        Ok(res.z().unwrap())
    }
    pub fn is_underlined_raw(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isUnderlinedRaw", "()Z", &[])?;
        Ok(res.z().unwrap())
    }
    pub fn is_strikethrough(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isStrikethrough", "()Z", &[])?;
        Ok(res.z().unwrap())
    }
    pub fn is_strikethrough_raw(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isStrikethroughRaw", "()Z", &[])?;
        Ok(res.z().unwrap())
    }
    pub fn is_obfuscated(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isObfuscated", "()Z", &[])?;
        Ok(res.z().unwrap())
    }
    pub fn is_obfuscated_raw(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isObfuscatedRaw", "()Z", &[])?;
        Ok(res.z().unwrap())
    }
    pub fn add_extra_with_base_component(
        &mut self,
        arg0: std::option::Option<impl Into<String>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 =
            jni::objects::JObject::from(self.jni_ref().new_string(arg0.unwrap().into()).unwrap());
        self.jni_ref().call_method(
            &self.jni_object(),
            "addExtra",
            "(Ljava/lang/String;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn has_formatting(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasFormatting", "()Z", &[])?;
        Ok(res.z().unwrap())
    }
    pub fn set_font(&mut self, arg0: impl Into<String>) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        self.jni_ref().call_method(
            &self.jni_object(),
            "setFont",
            "(Ljava/lang/String;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn set_bold(&mut self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        // -2
        let val_0 = jni::objects::JValueGen::Bool(arg0.into());
        self.jni_ref().call_method(
            &self.jni_object(),
            "setBold",
            "(Z)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn set_italic(&mut self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        // -2
        let val_0 = jni::objects::JValueGen::Bool(arg0.into());
        self.jni_ref().call_method(
            &self.jni_object(),
            "setItalic",
            "(Z)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn set_underlined(&mut self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        // -2
        let val_0 = jni::objects::JValueGen::Bool(arg0.into());
        self.jni_ref().call_method(
            &self.jni_object(),
            "setUnderlined",
            "(Z)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn set_strikethrough(&mut self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        // -2
        let val_0 = jni::objects::JValueGen::Bool(arg0.into());
        self.jni_ref().call_method(
            &self.jni_object(),
            "setStrikethrough",
            "(Z)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn set_obfuscated(&mut self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        // -2
        let val_0 = jni::objects::JValueGen::Bool(arg0.into());
        self.jni_ref().call_method(
            &self.jni_object(),
            "setObfuscated",
            "(Z)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn set_insertion(
        &mut self,
        arg0: impl Into<String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        self.jni_ref().call_method(
            &self.jni_object(),
            "setInsertion",
            "(Ljava/lang/String;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn set_click_event(
        &mut self,
        arg0: impl Into<crate::bungee::bungee::api::chat::ClickEvent<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        self.jni_ref().call_method(
            &self.jni_object(),
            "setClickEvent",
            "(Lnet/md_5/bungee/api/chat/ClickEvent;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn set_hover_event(
        &mut self,
        arg0: impl Into<crate::bungee::bungee::api::chat::HoverEvent<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        self.jni_ref().call_method(
            &self.jni_object(),
            "setHoverEvent",
            "(Lnet/md_5/bungee/api/chat/HoverEvent;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn insertion(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getInsertion",
            "()Ljava/lang/String;",
            &[],
        )?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    pub fn click_event(
        &mut self,
    ) -> Result<crate::bungee::bungee::api::chat::ClickEvent<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getClickEvent",
            "()Lnet/md_5/bungee/api/chat/ClickEvent;",
            &[],
        )?;
        let ret = {
            crate::bungee::bungee::api::chat::ClickEvent(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn hover_event(
        &mut self,
    ) -> Result<crate::bungee::bungee::api::chat::HoverEvent<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getHoverEvent",
            "()Lnet/md_5/bungee/api/chat/HoverEvent;",
            &[],
        )?;
        let ret = {
            crate::bungee::bungee::api::chat::HoverEvent(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
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
impl<'mc> Into<crate::bungee::bungee::api::chat::BaseComponent<'mc>> for SelectorComponent<'mc> {
    fn into(self) -> crate::bungee::bungee::api::chat::BaseComponent<'mc> {
        crate::bungee::bungee::api::chat::BaseComponent::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
pub struct ScoreComponent<'mc>(
    pub(crate) crate::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> crate::JNIRaw<'mc> for ScoreComponent<'mc> {
    fn jni_ref(&self) -> crate::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> ScoreComponent<'mc> {
    pub fn new_with_score_component(
        jni: crate::SharedJNIEnv<'mc>,
        arg0: std::option::Option<impl Into<String>>,
        arg1: std::option::Option<impl Into<String>>,
    ) -> Result<crate::bungee::bungee::api::chat::ScoreComponent<'mc>, Box<dyn std::error::Error>>
    {
        let val_0 = jni::objects::JObject::from(jni.new_string(arg0.unwrap().into()).unwrap());
        let val_1 = jni::objects::JObject::from(jni.new_string(arg1.unwrap().into()).unwrap());
        let cls = &jni.find_class("net/md_5/bungee/api/chat/ScoreComponent")?;
        let res = jni.new_object(
            cls,
            "(Ljava/lang/String;Ljava/lang/String;)V",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        )?;
        let ret = { crate::bungee::bungee::api::chat::ScoreComponent(jni, res) };
        Ok(ret)
    }
    pub fn new_with_string(
        jni: crate::SharedJNIEnv<'mc>,
        arg0: impl Into<String>,
        arg1: impl Into<String>,
        arg2: std::option::Option<impl Into<String>>,
    ) -> Result<crate::bungee::bungee::api::chat::ScoreComponent<'mc>, Box<dyn std::error::Error>>
    {
        let val_0 = jni::objects::JObject::from(jni.new_string(arg0.into()).unwrap());
        let val_1 = jni::objects::JObject::from(jni.new_string(arg1.into()).unwrap());
        let val_2 = jni::objects::JObject::from(jni.new_string(arg2.unwrap().into()).unwrap());
        let cls = &jni.find_class("net/md_5/bungee/api/chat/ScoreComponent")?;
        let res = jni.new_object(
            cls,
            "(Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;)V",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        )?;
        let ret = { crate::bungee::bungee::api::chat::ScoreComponent(jni, res) };
        Ok(ret)
    }
    pub fn from_raw(
        env: &crate::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate ScoreComponent from null object.").into(),
            );
        }
        let cls = env.jni.borrow().get_object_class(&obj)?;
        let name_raw = env.call_method(cls, "getName", "()Ljava/lang/String;", &[])?;
        let oh = name_raw.l()?.into();
        let what = env.get_string(&oh)?;
        let name = what.to_string_lossy();
        if !name.ends_with("ScoreComponent") {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a ScoreComponent object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    pub fn name(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getName",
            "()Ljava/lang/String;",
            &[],
        )?;
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
    pub fn value(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getValue",
            "()Ljava/lang/String;",
            &[],
        )?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    pub fn set_name(&mut self, arg0: impl Into<String>) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        self.jni_ref().call_method(
            &self.jni_object(),
            "setName",
            "(Ljava/lang/String;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn set_value(&mut self, arg0: impl Into<String>) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        self.jni_ref().call_method(
            &self.jni_object(),
            "setValue",
            "(Ljava/lang/String;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn duplicate(
        &mut self,
    ) -> Result<crate::bungee::bungee::api::chat::BaseComponent<'mc>, Box<dyn std::error::Error>>
    {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "duplicate",
            "()Lnet/md_5/bungee/api/chat/BaseComponent;",
            &[],
        )?;
        let ret = {
            crate::bungee::bungee::api::chat::BaseComponent(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn objective(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getObjective",
            "()Ljava/lang/String;",
            &[],
        )?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    pub fn set_objective(
        &mut self,
        arg0: impl Into<String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        self.jni_ref().call_method(
            &self.jni_object(),
            "setObjective",
            "(Ljava/lang/String;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn set_extra(
        &mut self,
        arg0: Vec<impl Into<crate::bungee::bungee::api::chat::BaseComponent<'mc>>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let raw_val_0 = self
            .jni_ref()
            .new_object("java/util/ArrayList", "()V", &[])
            .unwrap();
        for v in arg0 {
            let map_val_0 = unsafe { jni::objects::JObject::from_raw(v.into().1.clone()) };
            self.jni_ref().call_method(
                &raw_val_0,
                "add",
                "(Ljava/Lang/Object)V",
                &[jni::objects::JValueGen::from(&map_val_0)],
            )?;
        }
        let val_0 = jni::objects::JValueGen::Object(raw_val_0);
        self.jni_ref().call_method(
            &self.jni_object(),
            "setExtra",
            "(Ljava/util/List;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn set_color(
        &mut self,
        arg0: impl Into<crate::bungee::bungee::api::ChatColor<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        self.jni_ref().call_method(
            &self.jni_object(),
            "setColor",
            "(Lnet/md_5/bungee/api/ChatColor;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn copy_formatting_with_base_component(
        &mut self,
        arg0: std::option::Option<impl Into<crate::bungee::bungee::api::chat::BaseComponent<'mc>>>,
        arg1: std::option::Option<
            impl Into<crate::bungee::bungee::api::chat::ComponentBuilderFormatRetention<'mc>>,
        >,
        arg2: std::option::Option<bool>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.unwrap().into().1.clone()) };
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg1.unwrap().into().1.clone()) };
        // 0
        let val_2 = jni::objects::JValueGen::Bool(arg2.unwrap().into());
        self.jni_ref().call_method(&self.jni_object(),"copyFormatting","(Lnet/md_5/bungee/api/chat/BaseComponent;Lnet/md_5/bungee/api/chat/ComponentBuilder$FormatRetention;Z)V",&[jni::objects::JValueGen::from(&val_0),jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)])?;
        Ok(())
    }
    pub fn color(
        &mut self,
    ) -> Result<crate::bungee::bungee::api::ChatColor<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getColor",
            "()Lnet/md_5/bungee/api/ChatColor;",
            &[],
        )?;
        let ret = {
            crate::bungee::bungee::api::ChatColor(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn retain(
        &mut self,
        arg0: impl Into<crate::bungee::bungee::api::chat::ComponentBuilderFormatRetention<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        self.jni_ref().call_method(
            &self.jni_object(),
            "retain",
            "(Lnet/md_5/bungee/api/chat/ComponentBuilder$FormatRetention;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    #[deprecated]
    pub fn duplicate_without_formatting(
        &mut self,
    ) -> Result<crate::bungee::bungee::api::chat::BaseComponent<'mc>, Box<dyn std::error::Error>>
    {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "duplicateWithoutFormatting",
            "()Lnet/md_5/bungee/api/chat/BaseComponent;",
            &[],
        )?;
        let ret = {
            crate::bungee::bungee::api::chat::BaseComponent(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn to_legacy_text(
        jni: crate::SharedJNIEnv<'mc>,
        _arg0: std::option::Option<
            Vec<impl Into<crate::bungee::bungee::api::chat::BaseComponent<'mc>>>,
        >,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let cls = &jni.find_class("java/lang/String")?;
        let res = jni.call_static_method(
            cls,
            "toLegacyText",
            "(Lnet/md_5/bungee/api/chat/BaseComponent;)Ljava/lang/String;",
            &[],
        )?;
        Ok(jni
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    pub fn to_plain_text(
        jni: crate::SharedJNIEnv<'mc>,
        _arg0: std::option::Option<
            Vec<impl Into<crate::bungee::bungee::api::chat::BaseComponent<'mc>>>,
        >,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let cls = &jni.find_class("java/lang/String")?;
        let res = jni.call_static_method(
            cls,
            "toPlainText",
            "(Lnet/md_5/bungee/api/chat/BaseComponent;)Ljava/lang/String;",
            &[],
        )?;
        Ok(jni
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    pub fn color_raw(
        &mut self,
    ) -> Result<crate::bungee::bungee::api::ChatColor<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getColorRaw",
            "()Lnet/md_5/bungee/api/ChatColor;",
            &[],
        )?;
        let ret = {
            crate::bungee::bungee::api::ChatColor(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn font(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getFont",
            "()Ljava/lang/String;",
            &[],
        )?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    pub fn font_raw(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getFontRaw",
            "()Ljava/lang/String;",
            &[],
        )?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    pub fn is_bold(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isBold", "()Z", &[])?;
        Ok(res.z().unwrap())
    }
    pub fn is_bold_raw(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isBoldRaw", "()Z", &[])?;
        Ok(res.z().unwrap())
    }
    pub fn is_italic(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isItalic", "()Z", &[])?;
        Ok(res.z().unwrap())
    }
    pub fn is_italic_raw(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isItalicRaw", "()Z", &[])?;
        Ok(res.z().unwrap())
    }
    pub fn is_underlined(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isUnderlined", "()Z", &[])?;
        Ok(res.z().unwrap())
    }
    pub fn is_underlined_raw(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isUnderlinedRaw", "()Z", &[])?;
        Ok(res.z().unwrap())
    }
    pub fn is_strikethrough(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isStrikethrough", "()Z", &[])?;
        Ok(res.z().unwrap())
    }
    pub fn is_strikethrough_raw(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isStrikethroughRaw", "()Z", &[])?;
        Ok(res.z().unwrap())
    }
    pub fn is_obfuscated(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isObfuscated", "()Z", &[])?;
        Ok(res.z().unwrap())
    }
    pub fn is_obfuscated_raw(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isObfuscatedRaw", "()Z", &[])?;
        Ok(res.z().unwrap())
    }
    pub fn add_extra_with_base_component(
        &mut self,
        arg0: std::option::Option<impl Into<String>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 =
            jni::objects::JObject::from(self.jni_ref().new_string(arg0.unwrap().into()).unwrap());
        self.jni_ref().call_method(
            &self.jni_object(),
            "addExtra",
            "(Ljava/lang/String;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn has_formatting(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasFormatting", "()Z", &[])?;
        Ok(res.z().unwrap())
    }
    pub fn set_font(&mut self, arg0: impl Into<String>) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        self.jni_ref().call_method(
            &self.jni_object(),
            "setFont",
            "(Ljava/lang/String;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn set_bold(&mut self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        // -2
        let val_0 = jni::objects::JValueGen::Bool(arg0.into());
        self.jni_ref().call_method(
            &self.jni_object(),
            "setBold",
            "(Z)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn set_italic(&mut self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        // -2
        let val_0 = jni::objects::JValueGen::Bool(arg0.into());
        self.jni_ref().call_method(
            &self.jni_object(),
            "setItalic",
            "(Z)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn set_underlined(&mut self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        // -2
        let val_0 = jni::objects::JValueGen::Bool(arg0.into());
        self.jni_ref().call_method(
            &self.jni_object(),
            "setUnderlined",
            "(Z)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn set_strikethrough(&mut self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        // -2
        let val_0 = jni::objects::JValueGen::Bool(arg0.into());
        self.jni_ref().call_method(
            &self.jni_object(),
            "setStrikethrough",
            "(Z)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn set_obfuscated(&mut self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        // -2
        let val_0 = jni::objects::JValueGen::Bool(arg0.into());
        self.jni_ref().call_method(
            &self.jni_object(),
            "setObfuscated",
            "(Z)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn set_insertion(
        &mut self,
        arg0: impl Into<String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        self.jni_ref().call_method(
            &self.jni_object(),
            "setInsertion",
            "(Ljava/lang/String;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn set_click_event(
        &mut self,
        arg0: impl Into<crate::bungee::bungee::api::chat::ClickEvent<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        self.jni_ref().call_method(
            &self.jni_object(),
            "setClickEvent",
            "(Lnet/md_5/bungee/api/chat/ClickEvent;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn set_hover_event(
        &mut self,
        arg0: impl Into<crate::bungee::bungee::api::chat::HoverEvent<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        self.jni_ref().call_method(
            &self.jni_object(),
            "setHoverEvent",
            "(Lnet/md_5/bungee/api/chat/HoverEvent;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn insertion(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getInsertion",
            "()Ljava/lang/String;",
            &[],
        )?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    pub fn click_event(
        &mut self,
    ) -> Result<crate::bungee::bungee::api::chat::ClickEvent<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getClickEvent",
            "()Lnet/md_5/bungee/api/chat/ClickEvent;",
            &[],
        )?;
        let ret = {
            crate::bungee::bungee::api::chat::ClickEvent(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn hover_event(
        &mut self,
    ) -> Result<crate::bungee::bungee::api::chat::HoverEvent<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getHoverEvent",
            "()Lnet/md_5/bungee/api/chat/HoverEvent;",
            &[],
        )?;
        let ret = {
            crate::bungee::bungee::api::chat::HoverEvent(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
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
impl<'mc> Into<crate::bungee::bungee::api::chat::BaseComponent<'mc>> for ScoreComponent<'mc> {
    fn into(self) -> crate::bungee::bungee::api::chat::BaseComponent<'mc> {
        crate::bungee::bungee::api::chat::BaseComponent::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
pub struct ClickEvent<'mc>(
    pub(crate) crate::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
pub struct ClickEventAction<'mc>(
    pub(crate) crate::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> crate::JNIRaw<'mc> for ClickEventAction<'mc> {
    fn jni_ref(&self) -> crate::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> ClickEventAction<'mc> {
    pub fn from_raw(
        env: &crate::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate ClickEventAction from null object.").into(),
            );
        }
        let cls = env.jni.borrow().get_object_class(&obj)?;
        let name_raw = env.call_method(cls, "getName", "()Ljava/lang/String;", &[])?;
        let oh = name_raw.l()?.into();
        let what = env.get_string(&oh)?;
        let name = what.to_string_lossy();
        if !name.ends_with("ClickEventAction") {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a ClickEventAction object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    pub fn value_of_with_string(
        jni: crate::SharedJNIEnv<'mc>,
        arg0: std::option::Option<jni::objects::JClass<'mc>>,
        arg1: std::option::Option<impl Into<String>>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let val_0 = arg0.unwrap();
        let val_1 = jni::objects::JObject::from(jni.new_string(arg1.unwrap().into()).unwrap());
        let cls = &jni.find_class("java/lang/Enum")?;
        let res = jni.call_static_method(
            cls,
            "valueOf",
            "(Ljava/lang/Class;Ljava/lang/String;)Ljava/lang/Enum;",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        )?;
        Ok(res.l().unwrap())
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
    pub fn compare_to_with_object(
        &mut self,
        arg0: std::option::Option<jni::objects::JObject<'mc>>,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        let val_0 = arg0.unwrap();
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "compareTo",
            "(Ljava/lang/Enum;)I",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(res.i().unwrap())
    }
    pub fn describe_constable(
        &mut self,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "describeConstable",
            "()Ljava/util/Optional;",
            &[],
        )?;
        Ok(res.l().unwrap())
    }
    pub fn declaring_class(
        &mut self,
    ) -> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getDeclaringClass",
            "()Ljava/lang/Class;",
            &[],
        )?;
        Ok(unsafe { jni::objects::JClass::from_raw(res.as_jni().l) })
    }
    pub fn ordinal(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "ordinal", "()I", &[])?;
        Ok(res.i().unwrap())
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
impl<'mc> crate::JNIRaw<'mc> for ClickEvent<'mc> {
    fn jni_ref(&self) -> crate::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> ClickEvent<'mc> {
    pub fn new(
        jni: crate::SharedJNIEnv<'mc>,
        arg0: impl Into<crate::bungee::bungee::api::chat::ClickEventAction<'mc>>,
        arg1: impl Into<String>,
    ) -> Result<crate::bungee::bungee::api::chat::ClickEvent<'mc>, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let val_1 = jni::objects::JObject::from(jni.new_string(arg1.into()).unwrap());
        let cls = &jni.find_class("net/md_5/bungee/api/chat/ClickEvent")?;
        let res = jni.new_object(
            cls,
            "(Lnet/md_5/bungee/api/chat/ClickEvent$Action;Ljava/lang/String;)V",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        )?;
        let ret = { crate::bungee::bungee::api::chat::ClickEvent(jni, res) };
        Ok(ret)
    }
    pub fn from_raw(
        env: &crate::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate ClickEvent from null object.").into());
        }
        let cls = env.jni.borrow().get_object_class(&obj)?;
        let name_raw = env.call_method(cls, "getName", "()Ljava/lang/String;", &[])?;
        let oh = name_raw.l()?.into();
        let what = env.get_string(&oh)?;
        let name = what.to_string_lossy();
        if !name.ends_with("ClickEvent") {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a ClickEvent object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
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
    pub fn value(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getValue",
            "()Ljava/lang/String;",
            &[],
        )?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    pub fn action(
        &mut self,
    ) -> Result<crate::bungee::bungee::api::chat::ClickEventAction<'mc>, Box<dyn std::error::Error>>
    {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getAction",
            "()Lnet/md_5/bungee/api/chat/ClickEvent$Action;",
            &[],
        )?;
        let ret = {
            crate::bungee::bungee::api::chat::ClickEventAction(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
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
/// An instantiatable struct that implements Keybinds. Needed for returning it from Java.
pub struct Keybinds<'mc>(
    pub(crate) crate::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> Keybinds<'mc> {}
impl<'mc> crate::JNIRaw<'mc> for Keybinds<'mc> {
    fn jni_ref(&self) -> crate::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
pub struct ItemTagSerializer<'mc>(
    pub(crate) crate::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> crate::JNIRaw<'mc> for ItemTagSerializer<'mc> {
    fn jni_ref(&self) -> crate::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> ItemTagSerializer<'mc> {
    pub fn new(
        jni: crate::SharedJNIEnv<'mc>,
    ) -> Result<crate::bungee::bungee::api::chat::ItemTagSerializer<'mc>, Box<dyn std::error::Error>>
    {
        let cls = &jni.find_class("net/md_5/bungee/api/chat/ItemTag$Serializer")?;
        let res = jni.new_object(cls, "()V", &[])?;
        let ret = { crate::bungee::bungee::api::chat::ItemTagSerializer(jni, res) };
        Ok(ret)
    }
    pub fn from_raw(
        env: &crate::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate ItemTagSerializer from null object.").into(),
            );
        }
        let cls = env.jni.borrow().get_object_class(&obj)?;
        let name_raw = env.call_method(cls, "getName", "()Ljava/lang/String;", &[])?;
        let oh = name_raw.l()?.into();
        let what = env.get_string(&oh)?;
        let name = what.to_string_lossy();
        if !name.ends_with("ItemTagSerializer") {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a ItemTagSerializer object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
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
pub struct TranslatableComponent<'mc>(
    pub(crate) crate::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> crate::JNIRaw<'mc> for TranslatableComponent<'mc> {
    fn jni_ref(&self) -> crate::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> TranslatableComponent<'mc> {
    pub fn new(
        jni: crate::SharedJNIEnv<'mc>,
        arg0: std::option::Option<impl Into<String>>,
        _arg1: std::option::Option<Vec<jni::objects::JObject<'mc>>>,
    ) -> Result<
        crate::bungee::bungee::api::chat::TranslatableComponent<'mc>,
        Box<dyn std::error::Error>,
    > {
        let val_0 = jni::objects::JObject::from(jni.new_string(arg0.unwrap().into()).unwrap());
        let cls = &jni.find_class("net/md_5/bungee/api/chat/TranslatableComponent")?;
        let res = jni.new_object(
            cls,
            "(Ljava/lang/String;Ljava/lang/Object;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        let ret = { crate::bungee::bungee::api::chat::TranslatableComponent(jni, res) };
        Ok(ret)
    }
    pub fn from_raw(
        env: &crate::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate TranslatableComponent from null object."
            )
            .into());
        }
        let cls = env.jni.borrow().get_object_class(&obj)?;
        let name_raw = env.call_method(cls, "getName", "()Ljava/lang/String;", &[])?;
        let oh = name_raw.l()?.into();
        let what = env.get_string(&oh)?;
        let name = what.to_string_lossy();
        if !name.ends_with("TranslatableComponent") {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a TranslatableComponent object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
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
    pub fn duplicate(
        &mut self,
    ) -> Result<
        crate::bungee::bungee::api::chat::TranslatableComponent<'mc>,
        Box<dyn std::error::Error>,
    > {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "duplicate",
            "()Lnet/md_5/bungee/api/chat/TranslatableComponent;",
            &[],
        )?;
        let ret = {
            crate::bungee::bungee::api::chat::TranslatableComponent(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn format(&mut self) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getFormat",
            "()Ljava/util/regex/Pattern;",
            &[],
        )?;
        Ok(res.l().unwrap())
    }
    pub fn set_with(
        &mut self,
        arg0: Vec<impl Into<crate::bungee::bungee::api::chat::BaseComponent<'mc>>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let raw_val_0 = self
            .jni_ref()
            .new_object("java/util/ArrayList", "()V", &[])
            .unwrap();
        for v in arg0 {
            let map_val_0 = unsafe { jni::objects::JObject::from_raw(v.into().1.clone()) };
            self.jni_ref().call_method(
                &raw_val_0,
                "add",
                "(Ljava/Lang/Object)V",
                &[jni::objects::JValueGen::from(&map_val_0)],
            )?;
        }
        let val_0 = jni::objects::JValueGen::Object(raw_val_0);
        self.jni_ref().call_method(
            &self.jni_object(),
            "setWith",
            "(Ljava/util/List;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn add_with_with_base_component(
        &mut self,
        arg0: std::option::Option<impl Into<String>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 =
            jni::objects::JObject::from(self.jni_ref().new_string(arg0.unwrap().into()).unwrap());
        self.jni_ref().call_method(
            &self.jni_object(),
            "addWith",
            "(Ljava/lang/String;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn translate(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getTranslate",
            "()Ljava/lang/String;",
            &[],
        )?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    pub fn set_translate(
        &mut self,
        arg0: impl Into<String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        self.jni_ref().call_method(
            &self.jni_object(),
            "setTranslate",
            "(Ljava/lang/String;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn set_extra(
        &mut self,
        arg0: Vec<impl Into<crate::bungee::bungee::api::chat::BaseComponent<'mc>>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let raw_val_0 = self
            .jni_ref()
            .new_object("java/util/ArrayList", "()V", &[])
            .unwrap();
        for v in arg0 {
            let map_val_0 = unsafe { jni::objects::JObject::from_raw(v.into().1.clone()) };
            self.jni_ref().call_method(
                &raw_val_0,
                "add",
                "(Ljava/Lang/Object)V",
                &[jni::objects::JValueGen::from(&map_val_0)],
            )?;
        }
        let val_0 = jni::objects::JValueGen::Object(raw_val_0);
        self.jni_ref().call_method(
            &self.jni_object(),
            "setExtra",
            "(Ljava/util/List;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn set_color(
        &mut self,
        arg0: impl Into<crate::bungee::bungee::api::ChatColor<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        self.jni_ref().call_method(
            &self.jni_object(),
            "setColor",
            "(Lnet/md_5/bungee/api/ChatColor;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn copy_formatting_with_base_component(
        &mut self,
        arg0: std::option::Option<impl Into<crate::bungee::bungee::api::chat::BaseComponent<'mc>>>,
        arg1: std::option::Option<
            impl Into<crate::bungee::bungee::api::chat::ComponentBuilderFormatRetention<'mc>>,
        >,
        arg2: std::option::Option<bool>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.unwrap().into().1.clone()) };
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg1.unwrap().into().1.clone()) };
        // 0
        let val_2 = jni::objects::JValueGen::Bool(arg2.unwrap().into());
        self.jni_ref().call_method(&self.jni_object(),"copyFormatting","(Lnet/md_5/bungee/api/chat/BaseComponent;Lnet/md_5/bungee/api/chat/ComponentBuilder$FormatRetention;Z)V",&[jni::objects::JValueGen::from(&val_0),jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)])?;
        Ok(())
    }
    pub fn color(
        &mut self,
    ) -> Result<crate::bungee::bungee::api::ChatColor<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getColor",
            "()Lnet/md_5/bungee/api/ChatColor;",
            &[],
        )?;
        let ret = {
            crate::bungee::bungee::api::ChatColor(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn retain(
        &mut self,
        arg0: impl Into<crate::bungee::bungee::api::chat::ComponentBuilderFormatRetention<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        self.jni_ref().call_method(
            &self.jni_object(),
            "retain",
            "(Lnet/md_5/bungee/api/chat/ComponentBuilder$FormatRetention;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    #[deprecated]
    pub fn duplicate_without_formatting(
        &mut self,
    ) -> Result<crate::bungee::bungee::api::chat::BaseComponent<'mc>, Box<dyn std::error::Error>>
    {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "duplicateWithoutFormatting",
            "()Lnet/md_5/bungee/api/chat/BaseComponent;",
            &[],
        )?;
        let ret = {
            crate::bungee::bungee::api::chat::BaseComponent(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn to_legacy_text(
        jni: crate::SharedJNIEnv<'mc>,
        _arg0: std::option::Option<
            Vec<impl Into<crate::bungee::bungee::api::chat::BaseComponent<'mc>>>,
        >,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let cls = &jni.find_class("java/lang/String")?;
        let res = jni.call_static_method(
            cls,
            "toLegacyText",
            "(Lnet/md_5/bungee/api/chat/BaseComponent;)Ljava/lang/String;",
            &[],
        )?;
        Ok(jni
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    pub fn to_plain_text(
        jni: crate::SharedJNIEnv<'mc>,
        _arg0: std::option::Option<
            Vec<impl Into<crate::bungee::bungee::api::chat::BaseComponent<'mc>>>,
        >,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let cls = &jni.find_class("java/lang/String")?;
        let res = jni.call_static_method(
            cls,
            "toPlainText",
            "(Lnet/md_5/bungee/api/chat/BaseComponent;)Ljava/lang/String;",
            &[],
        )?;
        Ok(jni
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    pub fn color_raw(
        &mut self,
    ) -> Result<crate::bungee::bungee::api::ChatColor<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getColorRaw",
            "()Lnet/md_5/bungee/api/ChatColor;",
            &[],
        )?;
        let ret = {
            crate::bungee::bungee::api::ChatColor(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn font(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getFont",
            "()Ljava/lang/String;",
            &[],
        )?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    pub fn font_raw(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getFontRaw",
            "()Ljava/lang/String;",
            &[],
        )?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    pub fn is_bold(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isBold", "()Z", &[])?;
        Ok(res.z().unwrap())
    }
    pub fn is_bold_raw(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isBoldRaw", "()Z", &[])?;
        Ok(res.z().unwrap())
    }
    pub fn is_italic(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isItalic", "()Z", &[])?;
        Ok(res.z().unwrap())
    }
    pub fn is_italic_raw(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isItalicRaw", "()Z", &[])?;
        Ok(res.z().unwrap())
    }
    pub fn is_underlined(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isUnderlined", "()Z", &[])?;
        Ok(res.z().unwrap())
    }
    pub fn is_underlined_raw(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isUnderlinedRaw", "()Z", &[])?;
        Ok(res.z().unwrap())
    }
    pub fn is_strikethrough(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isStrikethrough", "()Z", &[])?;
        Ok(res.z().unwrap())
    }
    pub fn is_strikethrough_raw(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isStrikethroughRaw", "()Z", &[])?;
        Ok(res.z().unwrap())
    }
    pub fn is_obfuscated(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isObfuscated", "()Z", &[])?;
        Ok(res.z().unwrap())
    }
    pub fn is_obfuscated_raw(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isObfuscatedRaw", "()Z", &[])?;
        Ok(res.z().unwrap())
    }
    pub fn add_extra_with_base_component(
        &mut self,
        arg0: std::option::Option<impl Into<String>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 =
            jni::objects::JObject::from(self.jni_ref().new_string(arg0.unwrap().into()).unwrap());
        self.jni_ref().call_method(
            &self.jni_object(),
            "addExtra",
            "(Ljava/lang/String;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn has_formatting(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasFormatting", "()Z", &[])?;
        Ok(res.z().unwrap())
    }
    pub fn set_font(&mut self, arg0: impl Into<String>) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        self.jni_ref().call_method(
            &self.jni_object(),
            "setFont",
            "(Ljava/lang/String;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn set_bold(&mut self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        // -2
        let val_0 = jni::objects::JValueGen::Bool(arg0.into());
        self.jni_ref().call_method(
            &self.jni_object(),
            "setBold",
            "(Z)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn set_italic(&mut self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        // -2
        let val_0 = jni::objects::JValueGen::Bool(arg0.into());
        self.jni_ref().call_method(
            &self.jni_object(),
            "setItalic",
            "(Z)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn set_underlined(&mut self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        // -2
        let val_0 = jni::objects::JValueGen::Bool(arg0.into());
        self.jni_ref().call_method(
            &self.jni_object(),
            "setUnderlined",
            "(Z)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn set_strikethrough(&mut self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        // -2
        let val_0 = jni::objects::JValueGen::Bool(arg0.into());
        self.jni_ref().call_method(
            &self.jni_object(),
            "setStrikethrough",
            "(Z)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn set_obfuscated(&mut self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        // -2
        let val_0 = jni::objects::JValueGen::Bool(arg0.into());
        self.jni_ref().call_method(
            &self.jni_object(),
            "setObfuscated",
            "(Z)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn set_insertion(
        &mut self,
        arg0: impl Into<String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        self.jni_ref().call_method(
            &self.jni_object(),
            "setInsertion",
            "(Ljava/lang/String;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn set_click_event(
        &mut self,
        arg0: impl Into<crate::bungee::bungee::api::chat::ClickEvent<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        self.jni_ref().call_method(
            &self.jni_object(),
            "setClickEvent",
            "(Lnet/md_5/bungee/api/chat/ClickEvent;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn set_hover_event(
        &mut self,
        arg0: impl Into<crate::bungee::bungee::api::chat::HoverEvent<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        self.jni_ref().call_method(
            &self.jni_object(),
            "setHoverEvent",
            "(Lnet/md_5/bungee/api/chat/HoverEvent;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn insertion(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getInsertion",
            "()Ljava/lang/String;",
            &[],
        )?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    pub fn click_event(
        &mut self,
    ) -> Result<crate::bungee::bungee::api::chat::ClickEvent<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getClickEvent",
            "()Lnet/md_5/bungee/api/chat/ClickEvent;",
            &[],
        )?;
        let ret = {
            crate::bungee::bungee::api::chat::ClickEvent(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn hover_event(
        &mut self,
    ) -> Result<crate::bungee::bungee::api::chat::HoverEvent<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getHoverEvent",
            "()Lnet/md_5/bungee/api/chat/HoverEvent;",
            &[],
        )?;
        let ret = {
            crate::bungee::bungee::api::chat::HoverEvent(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
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
impl<'mc> Into<crate::bungee::bungee::api::chat::BaseComponent<'mc>>
    for TranslatableComponent<'mc>
{
    fn into(self) -> crate::bungee::bungee::api::chat::BaseComponent<'mc> {
        crate::bungee::bungee::api::chat::BaseComponent::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
/// An instantiatable struct that implements ComponentBuilderJoiner. Needed for returning it from Java.
pub struct ComponentBuilderJoiner<'mc>(
    pub(crate) crate::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> ComponentBuilderJoiner<'mc> {
    pub fn from_raw(
        env: &crate::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate ComponentBuilderJoiner from null object."
            )
            .into());
        }
        let cls = env.jni.borrow().get_object_class(&obj)?;
        let name_raw = env.call_method(cls, "getName", "()Ljava/lang/String;", &[])?;
        let oh = name_raw.l()?.into();
        let what = env.get_string(&oh)?;
        let name = what.to_string_lossy();
        if !name.ends_with("ComponentBuilderJoiner") {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a ComponentBuilderJoiner object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    pub fn join(
        &mut self,
        arg0: impl Into<crate::bungee::bungee::api::chat::ComponentBuilder<'mc>>,
        arg1: impl Into<crate::bungee::bungee::api::chat::ComponentBuilderFormatRetention<'mc>>,
    ) -> Result<crate::bungee::bungee::api::chat::ComponentBuilder<'mc>, Box<dyn std::error::Error>>
    {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg1.into().1.clone()) };
        let res =
self.jni_ref().call_method(&self.jni_object(),"join","(Lnet/md_5/bungee/api/chat/ComponentBuilder;Lnet/md_5/bungee/api/chat/ComponentBuilder$FormatRetention;)Lnet/md_5/bungee/api/chat/ComponentBuilder;",&[jni::objects::JValueGen::from(&val_0),jni::objects::JValueGen::from(&val_1)])?;
        let ret = {
            crate::bungee::bungee::api::chat::ComponentBuilder(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
}
impl<'mc> crate::JNIRaw<'mc> for ComponentBuilderJoiner<'mc> {
    fn jni_ref(&self) -> crate::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
pub struct ItemTag<'mc>(
    pub(crate) crate::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> crate::JNIRaw<'mc> for ItemTag<'mc> {
    fn jni_ref(&self) -> crate::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> ItemTag<'mc> {
    pub fn from_raw(
        env: &crate::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate ItemTag from null object.").into());
        }
        let cls = env.jni.borrow().get_object_class(&obj)?;
        let name_raw = env.call_method(cls, "getName", "()Ljava/lang/String;", &[])?;
        let oh = name_raw.l()?.into();
        let what = env.get_string(&oh)?;
        let name = what.to_string_lossy();
        if !name.ends_with("ItemTag") {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a ItemTag object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
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
    pub fn of_nbt(
        jni: crate::SharedJNIEnv<'mc>,
        arg0: impl Into<String>,
    ) -> Result<crate::bungee::bungee::api::chat::ItemTag<'mc>, Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JObject::from(jni.new_string(arg0.into()).unwrap());
        let cls = &jni.find_class("net/md_5/bungee/api/chat/ItemTag")?;
        let res = jni.call_static_method(
            cls,
            "ofNbt",
            "(Ljava/lang/String;)Lnet/md_5/bungee/api/chat/ItemTag;",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        let ret = {
            let obj = res.l()?;
            crate::bungee::bungee::api::chat::ItemTag(jni, obj)
        };
        Ok(ret)
    }
    pub fn nbt(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getNbt",
            "()Ljava/lang/String;",
            &[],
        )?;
        Ok(self
            .jni_ref()
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
pub struct KeybindComponent<'mc>(
    pub(crate) crate::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> crate::JNIRaw<'mc> for KeybindComponent<'mc> {
    fn jni_ref(&self) -> crate::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> KeybindComponent<'mc> {
    pub fn new(
        jni: crate::SharedJNIEnv<'mc>,
        arg0: std::option::Option<
            impl Into<crate::bungee::bungee::api::chat::KeybindComponent<'mc>>,
        >,
    ) -> Result<crate::bungee::bungee::api::chat::KeybindComponent<'mc>, Box<dyn std::error::Error>>
    {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.unwrap().into().1.clone()) };
        let cls = &jni.find_class("net/md_5/bungee/api/chat/KeybindComponent")?;
        let res = jni.new_object(
            cls,
            "(Lnet/md_5/bungee/api/chat/KeybindComponent;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        let ret = { crate::bungee::bungee::api::chat::KeybindComponent(jni, res) };
        Ok(ret)
    }
    pub fn from_raw(
        env: &crate::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate KeybindComponent from null object.").into(),
            );
        }
        let cls = env.jni.borrow().get_object_class(&obj)?;
        let name_raw = env.call_method(cls, "getName", "()Ljava/lang/String;", &[])?;
        let oh = name_raw.l()?.into();
        let what = env.get_string(&oh)?;
        let name = what.to_string_lossy();
        if !name.ends_with("KeybindComponent") {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a KeybindComponent object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
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
    pub fn duplicate(
        &mut self,
    ) -> Result<crate::bungee::bungee::api::chat::KeybindComponent<'mc>, Box<dyn std::error::Error>>
    {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "duplicate",
            "()Lnet/md_5/bungee/api/chat/KeybindComponent;",
            &[],
        )?;
        let ret = {
            crate::bungee::bungee::api::chat::KeybindComponent(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn keybind(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getKeybind",
            "()Ljava/lang/String;",
            &[],
        )?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    pub fn set_keybind(
        &mut self,
        arg0: impl Into<String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        self.jni_ref().call_method(
            &self.jni_object(),
            "setKeybind",
            "(Ljava/lang/String;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn set_extra(
        &mut self,
        arg0: Vec<impl Into<crate::bungee::bungee::api::chat::BaseComponent<'mc>>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let raw_val_0 = self
            .jni_ref()
            .new_object("java/util/ArrayList", "()V", &[])
            .unwrap();
        for v in arg0 {
            let map_val_0 = unsafe { jni::objects::JObject::from_raw(v.into().1.clone()) };
            self.jni_ref().call_method(
                &raw_val_0,
                "add",
                "(Ljava/Lang/Object)V",
                &[jni::objects::JValueGen::from(&map_val_0)],
            )?;
        }
        let val_0 = jni::objects::JValueGen::Object(raw_val_0);
        self.jni_ref().call_method(
            &self.jni_object(),
            "setExtra",
            "(Ljava/util/List;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn set_color(
        &mut self,
        arg0: impl Into<crate::bungee::bungee::api::ChatColor<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        self.jni_ref().call_method(
            &self.jni_object(),
            "setColor",
            "(Lnet/md_5/bungee/api/ChatColor;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn copy_formatting_with_base_component(
        &mut self,
        arg0: std::option::Option<impl Into<crate::bungee::bungee::api::chat::BaseComponent<'mc>>>,
        arg1: std::option::Option<
            impl Into<crate::bungee::bungee::api::chat::ComponentBuilderFormatRetention<'mc>>,
        >,
        arg2: std::option::Option<bool>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.unwrap().into().1.clone()) };
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg1.unwrap().into().1.clone()) };
        // 0
        let val_2 = jni::objects::JValueGen::Bool(arg2.unwrap().into());
        self.jni_ref().call_method(&self.jni_object(),"copyFormatting","(Lnet/md_5/bungee/api/chat/BaseComponent;Lnet/md_5/bungee/api/chat/ComponentBuilder$FormatRetention;Z)V",&[jni::objects::JValueGen::from(&val_0),jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)])?;
        Ok(())
    }
    pub fn color(
        &mut self,
    ) -> Result<crate::bungee::bungee::api::ChatColor<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getColor",
            "()Lnet/md_5/bungee/api/ChatColor;",
            &[],
        )?;
        let ret = {
            crate::bungee::bungee::api::ChatColor(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn retain(
        &mut self,
        arg0: impl Into<crate::bungee::bungee::api::chat::ComponentBuilderFormatRetention<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        self.jni_ref().call_method(
            &self.jni_object(),
            "retain",
            "(Lnet/md_5/bungee/api/chat/ComponentBuilder$FormatRetention;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    #[deprecated]
    pub fn duplicate_without_formatting(
        &mut self,
    ) -> Result<crate::bungee::bungee::api::chat::BaseComponent<'mc>, Box<dyn std::error::Error>>
    {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "duplicateWithoutFormatting",
            "()Lnet/md_5/bungee/api/chat/BaseComponent;",
            &[],
        )?;
        let ret = {
            crate::bungee::bungee::api::chat::BaseComponent(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn to_legacy_text(
        jni: crate::SharedJNIEnv<'mc>,
        _arg0: std::option::Option<
            Vec<impl Into<crate::bungee::bungee::api::chat::BaseComponent<'mc>>>,
        >,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let cls = &jni.find_class("java/lang/String")?;
        let res = jni.call_static_method(
            cls,
            "toLegacyText",
            "(Lnet/md_5/bungee/api/chat/BaseComponent;)Ljava/lang/String;",
            &[],
        )?;
        Ok(jni
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    pub fn to_plain_text(
        jni: crate::SharedJNIEnv<'mc>,
        _arg0: std::option::Option<
            Vec<impl Into<crate::bungee::bungee::api::chat::BaseComponent<'mc>>>,
        >,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let cls = &jni.find_class("java/lang/String")?;
        let res = jni.call_static_method(
            cls,
            "toPlainText",
            "(Lnet/md_5/bungee/api/chat/BaseComponent;)Ljava/lang/String;",
            &[],
        )?;
        Ok(jni
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    pub fn color_raw(
        &mut self,
    ) -> Result<crate::bungee::bungee::api::ChatColor<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getColorRaw",
            "()Lnet/md_5/bungee/api/ChatColor;",
            &[],
        )?;
        let ret = {
            crate::bungee::bungee::api::ChatColor(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn font(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getFont",
            "()Ljava/lang/String;",
            &[],
        )?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    pub fn font_raw(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getFontRaw",
            "()Ljava/lang/String;",
            &[],
        )?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    pub fn is_bold(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isBold", "()Z", &[])?;
        Ok(res.z().unwrap())
    }
    pub fn is_bold_raw(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isBoldRaw", "()Z", &[])?;
        Ok(res.z().unwrap())
    }
    pub fn is_italic(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isItalic", "()Z", &[])?;
        Ok(res.z().unwrap())
    }
    pub fn is_italic_raw(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isItalicRaw", "()Z", &[])?;
        Ok(res.z().unwrap())
    }
    pub fn is_underlined(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isUnderlined", "()Z", &[])?;
        Ok(res.z().unwrap())
    }
    pub fn is_underlined_raw(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isUnderlinedRaw", "()Z", &[])?;
        Ok(res.z().unwrap())
    }
    pub fn is_strikethrough(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isStrikethrough", "()Z", &[])?;
        Ok(res.z().unwrap())
    }
    pub fn is_strikethrough_raw(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isStrikethroughRaw", "()Z", &[])?;
        Ok(res.z().unwrap())
    }
    pub fn is_obfuscated(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isObfuscated", "()Z", &[])?;
        Ok(res.z().unwrap())
    }
    pub fn is_obfuscated_raw(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isObfuscatedRaw", "()Z", &[])?;
        Ok(res.z().unwrap())
    }
    pub fn add_extra_with_base_component(
        &mut self,
        arg0: std::option::Option<impl Into<String>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 =
            jni::objects::JObject::from(self.jni_ref().new_string(arg0.unwrap().into()).unwrap());
        self.jni_ref().call_method(
            &self.jni_object(),
            "addExtra",
            "(Ljava/lang/String;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn has_formatting(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasFormatting", "()Z", &[])?;
        Ok(res.z().unwrap())
    }
    pub fn set_font(&mut self, arg0: impl Into<String>) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        self.jni_ref().call_method(
            &self.jni_object(),
            "setFont",
            "(Ljava/lang/String;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn set_bold(&mut self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        // -2
        let val_0 = jni::objects::JValueGen::Bool(arg0.into());
        self.jni_ref().call_method(
            &self.jni_object(),
            "setBold",
            "(Z)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn set_italic(&mut self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        // -2
        let val_0 = jni::objects::JValueGen::Bool(arg0.into());
        self.jni_ref().call_method(
            &self.jni_object(),
            "setItalic",
            "(Z)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn set_underlined(&mut self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        // -2
        let val_0 = jni::objects::JValueGen::Bool(arg0.into());
        self.jni_ref().call_method(
            &self.jni_object(),
            "setUnderlined",
            "(Z)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn set_strikethrough(&mut self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        // -2
        let val_0 = jni::objects::JValueGen::Bool(arg0.into());
        self.jni_ref().call_method(
            &self.jni_object(),
            "setStrikethrough",
            "(Z)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn set_obfuscated(&mut self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        // -2
        let val_0 = jni::objects::JValueGen::Bool(arg0.into());
        self.jni_ref().call_method(
            &self.jni_object(),
            "setObfuscated",
            "(Z)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn set_insertion(
        &mut self,
        arg0: impl Into<String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        self.jni_ref().call_method(
            &self.jni_object(),
            "setInsertion",
            "(Ljava/lang/String;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn set_click_event(
        &mut self,
        arg0: impl Into<crate::bungee::bungee::api::chat::ClickEvent<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        self.jni_ref().call_method(
            &self.jni_object(),
            "setClickEvent",
            "(Lnet/md_5/bungee/api/chat/ClickEvent;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn set_hover_event(
        &mut self,
        arg0: impl Into<crate::bungee::bungee::api::chat::HoverEvent<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        self.jni_ref().call_method(
            &self.jni_object(),
            "setHoverEvent",
            "(Lnet/md_5/bungee/api/chat/HoverEvent;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn insertion(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getInsertion",
            "()Ljava/lang/String;",
            &[],
        )?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    pub fn click_event(
        &mut self,
    ) -> Result<crate::bungee::bungee::api::chat::ClickEvent<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getClickEvent",
            "()Lnet/md_5/bungee/api/chat/ClickEvent;",
            &[],
        )?;
        let ret = {
            crate::bungee::bungee::api::chat::ClickEvent(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn hover_event(
        &mut self,
    ) -> Result<crate::bungee::bungee::api::chat::HoverEvent<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getHoverEvent",
            "()Lnet/md_5/bungee/api/chat/HoverEvent;",
            &[],
        )?;
        let ret = {
            crate::bungee::bungee::api::chat::HoverEvent(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
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
impl<'mc> Into<crate::bungee::bungee::api::chat::BaseComponent<'mc>> for KeybindComponent<'mc> {
    fn into(self) -> crate::bungee::bungee::api::chat::BaseComponent<'mc> {
        crate::bungee::bungee::api::chat::BaseComponent::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
pub struct ComponentBuilder<'mc>(
    pub(crate) crate::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
pub struct ComponentBuilderFormatRetention<'mc>(
    pub(crate) crate::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> crate::JNIRaw<'mc> for ComponentBuilderFormatRetention<'mc> {
    fn jni_ref(&self) -> crate::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> ComponentBuilderFormatRetention<'mc> {
    pub fn from_raw(
        env: &crate::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate ComponentBuilderFormatRetention from null object."
            )
            .into());
        }
        let cls = env.jni.borrow().get_object_class(&obj)?;
        let name_raw = env.call_method(cls, "getName", "()Ljava/lang/String;", &[])?;
        let oh = name_raw.l()?.into();
        let what = env.get_string(&oh)?;
        let name = what.to_string_lossy();
        if !name.ends_with("ComponentBuilderFormatRetention") {
            Err(eyre::eyre!(
        "Invalid argument passed. Expected a ComponentBuilderFormatRetention object, got {}",
        name
    )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    pub fn value_of_with_string(
        jni: crate::SharedJNIEnv<'mc>,
        arg0: std::option::Option<jni::objects::JClass<'mc>>,
        arg1: std::option::Option<impl Into<String>>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let val_0 = arg0.unwrap();
        let val_1 = jni::objects::JObject::from(jni.new_string(arg1.unwrap().into()).unwrap());
        let cls = &jni.find_class("java/lang/Enum")?;
        let res = jni.call_static_method(
            cls,
            "valueOf",
            "(Ljava/lang/Class;Ljava/lang/String;)Ljava/lang/Enum;",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        )?;
        Ok(res.l().unwrap())
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
    pub fn compare_to_with_object(
        &mut self,
        arg0: std::option::Option<jni::objects::JObject<'mc>>,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        let val_0 = arg0.unwrap();
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "compareTo",
            "(Ljava/lang/Enum;)I",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(res.i().unwrap())
    }
    pub fn describe_constable(
        &mut self,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "describeConstable",
            "()Ljava/util/Optional;",
            &[],
        )?;
        Ok(res.l().unwrap())
    }
    pub fn declaring_class(
        &mut self,
    ) -> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getDeclaringClass",
            "()Ljava/lang/Class;",
            &[],
        )?;
        Ok(unsafe { jni::objects::JClass::from_raw(res.as_jni().l) })
    }
    pub fn ordinal(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "ordinal", "()I", &[])?;
        Ok(res.i().unwrap())
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
impl<'mc> crate::JNIRaw<'mc> for ComponentBuilder<'mc> {
    fn jni_ref(&self) -> crate::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> ComponentBuilder<'mc> {
    pub fn new(
        jni: crate::SharedJNIEnv<'mc>,
        arg0: std::option::Option<impl Into<String>>,
    ) -> Result<crate::bungee::bungee::api::chat::ComponentBuilder<'mc>, Box<dyn std::error::Error>>
    {
        let val_0 = jni::objects::JObject::from(jni.new_string(arg0.unwrap().into()).unwrap());
        let cls = &jni.find_class("net/md_5/bungee/api/chat/ComponentBuilder")?;
        let res = jni.new_object(
            cls,
            "(Ljava/lang/String;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        let ret = { crate::bungee::bungee::api::chat::ComponentBuilder(jni, res) };
        Ok(ret)
    }
    pub fn new_with_component_builder(
        jni: crate::SharedJNIEnv<'mc>,
        arg0: std::option::Option<
            impl Into<crate::bungee::bungee::api::chat::ComponentBuilder<'mc>>,
        >,
    ) -> Result<crate::bungee::bungee::api::chat::ComponentBuilder<'mc>, Box<dyn std::error::Error>>
    {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.unwrap().into().1.clone()) };
        let cls = &jni.find_class("net/md_5/bungee/api/chat/ComponentBuilder")?;
        let res = jni.new_object(
            cls,
            "(Lnet/md_5/bungee/api/chat/ComponentBuilder;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        let ret = { crate::bungee::bungee::api::chat::ComponentBuilder(jni, res) };
        Ok(ret)
    }
    pub fn from_raw(
        env: &crate::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate ComponentBuilder from null object.").into(),
            );
        }
        let cls = env.jni.borrow().get_object_class(&obj)?;
        let name_raw = env.call_method(cls, "getName", "()Ljava/lang/String;", &[])?;
        let oh = name_raw.l()?.into();
        let what = env.get_string(&oh)?;
        let name = what.to_string_lossy();
        if !name.ends_with("ComponentBuilder") {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a ComponentBuilder object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    pub fn append_with_string(
        &mut self,
        _arg0: Vec<impl Into<crate::bungee::bungee::api::chat::BaseComponent<'mc>>>,
        arg1: std::option::Option<
            impl Into<crate::bungee::bungee::api::chat::ComponentBuilderFormatRetention<'mc>>,
        >,
    ) -> Result<crate::bungee::bungee::api::chat::ComponentBuilder<'mc>, Box<dyn std::error::Error>>
    {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg1.unwrap().into().1.clone()) };
        let res =
self.jni_ref().call_method(&self.jni_object(),"append","(Lnet/md_5/bungee/api/chat/BaseComponent;Lnet/md_5/bungee/api/chat/ComponentBuilder$FormatRetention;)Lnet/md_5/bungee/api/chat/ComponentBuilder;",&[jni::objects::JValueGen::from(&val_1)])?;
        let ret = {
            crate::bungee::bungee::api::chat::ComponentBuilder(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn append_with_base_component(
        &mut self,
        _arg0: std::option::Option<
            Vec<impl Into<crate::bungee::bungee::api::chat::BaseComponent<'mc>>>,
        >,
    ) -> Result<crate::bungee::bungee::api::chat::ComponentBuilder<'mc>, Box<dyn std::error::Error>>
    {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "append",
            "(Lnet/md_5/bungee/api/chat/BaseComponent;)Lnet/md_5/bungee/api/chat/ComponentBuilder;",
            &[],
        )?;
        let ret = {
            crate::bungee::bungee::api::chat::ComponentBuilder(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn append_with_component_builderjoiner(
        &mut self,
        arg0: impl Into<crate::bungee::bungee::api::chat::BaseComponent<'mc>>,
        arg1: std::option::Option<
            impl Into<crate::bungee::bungee::api::chat::ComponentBuilderFormatRetention<'mc>>,
        >,
    ) -> Result<crate::bungee::bungee::api::chat::ComponentBuilder<'mc>, Box<dyn std::error::Error>>
    {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg1.unwrap().into().1.clone()) };
        let res =
self.jni_ref().call_method(&self.jni_object(),"append","(Lnet/md_5/bungee/api/chat/BaseComponent;Lnet/md_5/bungee/api/chat/ComponentBuilder$FormatRetention;)Lnet/md_5/bungee/api/chat/ComponentBuilder;",&[jni::objects::JValueGen::from(&val_0),jni::objects::JValueGen::from(&val_1)])?;
        let ret = {
            crate::bungee::bungee::api::chat::ComponentBuilder(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn event_with_hover_event(
        &mut self,
        arg0: std::option::Option<impl Into<crate::bungee::bungee::api::chat::ClickEvent<'mc>>>,
    ) -> Result<crate::bungee::bungee::api::chat::ComponentBuilder<'mc>, Box<dyn std::error::Error>>
    {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.unwrap().into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "event",
            "(Lnet/md_5/bungee/api/chat/ClickEvent;)Lnet/md_5/bungee/api/chat/ComponentBuilder;",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        let ret = {
            crate::bungee::bungee::api::chat::ComponentBuilder(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn reset(
        &mut self,
    ) -> Result<crate::bungee::bungee::api::chat::ComponentBuilder<'mc>, Box<dyn std::error::Error>>
    {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "reset",
            "()Lnet/md_5/bungee/api/chat/ComponentBuilder;",
            &[],
        )?;
        let ret = {
            crate::bungee::bungee::api::chat::ComponentBuilder(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn color(
        &mut self,
        arg0: impl Into<crate::bungee::bungee::api::ChatColor<'mc>>,
    ) -> Result<crate::bungee::bungee::api::chat::ComponentBuilder<'mc>, Box<dyn std::error::Error>>
    {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "color",
            "(Lnet/md_5/bungee/api/ChatColor;)Lnet/md_5/bungee/api/chat/ComponentBuilder;",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        let ret = {
            crate::bungee::bungee::api::chat::ComponentBuilder(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn font(
        &mut self,
        arg0: impl Into<String>,
    ) -> Result<crate::bungee::bungee::api::chat::ComponentBuilder<'mc>, Box<dyn std::error::Error>>
    {
        let val_0 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "font",
            "(Ljava/lang/String;)Lnet/md_5/bungee/api/chat/ComponentBuilder;",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        let ret = {
            crate::bungee::bungee::api::chat::ComponentBuilder(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn bold(
        &mut self,
        arg0: bool,
    ) -> Result<crate::bungee::bungee::api::chat::ComponentBuilder<'mc>, Box<dyn std::error::Error>>
    {
        // -2
        let val_0 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "bold",
            "(Z)Lnet/md_5/bungee/api/chat/ComponentBuilder;",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        let ret = {
            crate::bungee::bungee::api::chat::ComponentBuilder(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn italic(
        &mut self,
        arg0: bool,
    ) -> Result<crate::bungee::bungee::api::chat::ComponentBuilder<'mc>, Box<dyn std::error::Error>>
    {
        // -2
        let val_0 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "italic",
            "(Z)Lnet/md_5/bungee/api/chat/ComponentBuilder;",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        let ret = {
            crate::bungee::bungee::api::chat::ComponentBuilder(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn underlined(
        &mut self,
        arg0: bool,
    ) -> Result<crate::bungee::bungee::api::chat::ComponentBuilder<'mc>, Box<dyn std::error::Error>>
    {
        // -2
        let val_0 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "underlined",
            "(Z)Lnet/md_5/bungee/api/chat/ComponentBuilder;",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        let ret = {
            crate::bungee::bungee::api::chat::ComponentBuilder(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn strikethrough(
        &mut self,
        arg0: bool,
    ) -> Result<crate::bungee::bungee::api::chat::ComponentBuilder<'mc>, Box<dyn std::error::Error>>
    {
        // -2
        let val_0 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "strikethrough",
            "(Z)Lnet/md_5/bungee/api/chat/ComponentBuilder;",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        let ret = {
            crate::bungee::bungee::api::chat::ComponentBuilder(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn obfuscated(
        &mut self,
        arg0: bool,
    ) -> Result<crate::bungee::bungee::api::chat::ComponentBuilder<'mc>, Box<dyn std::error::Error>>
    {
        // -2
        let val_0 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "obfuscated",
            "(Z)Lnet/md_5/bungee/api/chat/ComponentBuilder;",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        let ret = {
            crate::bungee::bungee::api::chat::ComponentBuilder(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn insertion(
        &mut self,
        arg0: impl Into<String>,
    ) -> Result<crate::bungee::bungee::api::chat::ComponentBuilder<'mc>, Box<dyn std::error::Error>>
    {
        let val_0 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "insertion",
            "(Ljava/lang/String;)Lnet/md_5/bungee/api/chat/ComponentBuilder;",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        let ret = {
            crate::bungee::bungee::api::chat::ComponentBuilder(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn set_cursor(
        &mut self,
        arg0: i32,
    ) -> Result<crate::bungee::bungee::api::chat::ComponentBuilder<'mc>, Box<dyn std::error::Error>>
    {
        let val_0 = jni::objects::JValueGen::Int(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setCursor",
            "(I)Lnet/md_5/bungee/api/chat/ComponentBuilder;",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        let ret = {
            crate::bungee::bungee::api::chat::ComponentBuilder(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn cursor(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getCursor", "()I", &[])?;
        Ok(res.i().unwrap())
    }
    pub fn retain(
        &mut self,
        arg0: impl Into<crate::bungee::bungee::api::chat::ComponentBuilderFormatRetention<'mc>>,
    ) -> Result<crate::bungee::bungee::api::chat::ComponentBuilder<'mc>, Box<dyn std::error::Error>>
    {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let res =
self.jni_ref().call_method(&self.jni_object(),"retain","(Lnet/md_5/bungee/api/chat/ComponentBuilder$FormatRetention;)Lnet/md_5/bungee/api/chat/ComponentBuilder;",&[jni::objects::JValueGen::from(&val_0)])?;
        let ret = {
            crate::bungee::bungee::api::chat::ComponentBuilder(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn reset_cursor(
        &mut self,
    ) -> Result<crate::bungee::bungee::api::chat::ComponentBuilder<'mc>, Box<dyn std::error::Error>>
    {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "resetCursor",
            "()Lnet/md_5/bungee/api/chat/ComponentBuilder;",
            &[],
        )?;
        let ret = {
            crate::bungee::bungee::api::chat::ComponentBuilder(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn append_legacy(
        &mut self,
        arg0: impl Into<String>,
    ) -> Result<crate::bungee::bungee::api::chat::ComponentBuilder<'mc>, Box<dyn std::error::Error>>
    {
        let val_0 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "appendLegacy",
            "(Ljava/lang/String;)Lnet/md_5/bungee/api/chat/ComponentBuilder;",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        let ret = {
            crate::bungee::bungee::api::chat::ComponentBuilder(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn remove_component(&mut self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Int(arg0.into());
        self.jni_ref().call_method(
            &self.jni_object(),
            "removeComponent",
            "(I)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn get_component(
        &mut self,
        arg0: i32,
    ) -> Result<crate::bungee::bungee::api::chat::BaseComponent<'mc>, Box<dyn std::error::Error>>
    {
        let val_0 = jni::objects::JValueGen::Int(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getComponent",
            "(I)Lnet/md_5/bungee/api/chat/BaseComponent;",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        let ret = {
            crate::bungee::bungee::api::chat::BaseComponent(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn current_component(
        &mut self,
    ) -> Result<crate::bungee::bungee::api::chat::BaseComponent<'mc>, Box<dyn std::error::Error>>
    {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getCurrentComponent",
            "()Lnet/md_5/bungee/api/chat/BaseComponent;",
            &[],
        )?;
        let ret = {
            crate::bungee::bungee::api::chat::BaseComponent(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
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
pub mod hover;
