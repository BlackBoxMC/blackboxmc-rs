#![allow(deprecated)]
use blackboxmc_general::JNIInstantiatable;
use blackboxmc_general::JNIInstantiatableEnum;
use blackboxmc_general::JNIRaw;
use color_eyre::eyre::Result;

pub struct TextComponent<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for TextComponent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}

impl<'mc> JNIInstantiatable<'mc> for TextComponent<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate TextComponent from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "net/md_5/bungee/api/chat/TextComponent")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a TextComponent object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> TextComponent<'mc> {
    pub fn new_with_base_components(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: std::option::Option<Vec<impl Into<crate::bungee::api::chat::BaseComponent<'mc>>>>,
    ) -> Result<crate::bungee::api::chat::TextComponent<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += ")V";
        let cls = jni.find_class("net/md_5/bungee/api/chat/TextComponent");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(cls, sig.as_str(), args);
        let res = jni.translate_error_no_gen(res)?;
        crate::bungee::api::chat::TextComponent::from_raw(&jni, res)
    }
    pub fn new_with_text_component(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: std::option::Option<impl Into<crate::bungee::api::chat::TextComponent<'mc>>>,
    ) -> Result<crate::bungee::api::chat::TextComponent<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "Lnet/md_5/bungee/api/chat/TextComponent;";
            let val_1 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(a.into().jni_object().clone())
            });
            args.push(val_1);
        }
        sig += ")V";
        let cls = jni.find_class("net/md_5/bungee/api/chat/TextComponent");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(cls, sig.as_str(), args);
        let res = jni.translate_error_no_gen(res)?;
        crate::bungee::api::chat::TextComponent::from_raw(&jni, res)
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

    pub fn duplicate(
        &self,
    ) -> Result<crate::bungee::api::chat::BaseComponent<'mc>, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Lnet/md_5/bungee/api/chat/BaseComponent;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "duplicate", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        crate::bungee::api::chat::BaseComponent::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn text(&self) -> Result<String, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/String;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getText", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }

    pub fn set_text(&self, arg0: impl Into<String>) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/String;)V");
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(arg0.into())?,
        ));
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setText",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn duplicate_without_formatting(
        &self,
    ) -> Result<crate::bungee::api::chat::BaseComponent<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lnet/md_5/bungee/api/chat/BaseComponent;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "duplicateWithoutFormatting",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::bungee::api::chat::BaseComponent::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn copy_formatting_with_base_component(
        &self,
        arg0: impl Into<crate::bungee::api::chat::BaseComponent<'mc>>,
        arg1: std::option::Option<
            impl Into<crate::bungee::api::chat::ComponentBuilderFormatRetention<'mc>>,
        >,
        arg2: std::option::Option<bool>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lnet/md_5/bungee/api/chat/BaseComponent;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        if let Some(a) = arg1 {
            sig += "Lnet/md_5/bungee/api/chat/ComponentBuilder$FormatRetention;";
            let val_2 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(a.into().jni_object().clone())
            });
            args.push(val_2);
        }
        if let Some(a) = arg2 {
            sig += "Z";
            let val_3 = jni::objects::JValueGen::Bool(a.into());
            args.push(val_3);
        }
        sig += ")V";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "copyFormatting", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn retain(
        &self,
        arg0: impl Into<crate::bungee::api::chat::ComponentBuilderFormatRetention<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lnet/md_5/bungee/api/chat/ComponentBuilder$FormatRetention;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "retain",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn to_legacy_text_with_base_components(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: std::option::Option<Vec<impl Into<crate::bungee::api::chat::BaseComponent<'mc>>>>,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Ljava/lang/String;";
        let cls = jni.find_class("java/lang/String");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "toLegacyText", sig.as_str(), args);
        let res = jni.translate_error(res)?;
        Ok(jni
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }

    pub fn to_plain_text_with_base_components(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: std::option::Option<Vec<impl Into<crate::bungee::api::chat::BaseComponent<'mc>>>>,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Ljava/lang/String;";
        let cls = jni.find_class("java/lang/String");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "toPlainText", sig.as_str(), args);
        let res = jni.translate_error(res)?;
        Ok(jni
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }

    pub fn color_raw(
        &self,
    ) -> Result<crate::bungee::api::ChatColor<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lnet/md_5/bungee/api/ChatColor;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getColorRaw", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::bungee::api::ChatColor::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn font(&self) -> Result<String, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/String;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getFont", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }

    pub fn font_raw(&self) -> Result<String, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/String;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getFontRaw", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }

    pub fn is_bold(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isBold", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn is_bold_raw(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/Boolean;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isBoldRaw", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn is_italic(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isItalic", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn is_italic_raw(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/Boolean;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isItalicRaw", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn is_underlined(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isUnderlined", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn is_underlined_raw(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/Boolean;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isUnderlinedRaw", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn is_strikethrough(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isStrikethrough", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn is_strikethrough_raw(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/Boolean;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "isStrikethroughRaw",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn is_obfuscated(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isObfuscated", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn is_obfuscated_raw(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/Boolean;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isObfuscatedRaw", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn add_extra_with_base_component(
        &self,
        arg0: impl Into<crate::bungee::api::chat::BaseComponent<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lnet/md_5/bungee/api/chat/BaseComponent;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        sig += ")V";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "addExtra", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn has_formatting(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "hasFormatting", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn set_font(&self, arg0: impl Into<String>) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/String;)V");
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(arg0.into())?,
        ));
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setFont",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn set_bold(&self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/Boolean;)V");
        let val_1 = jni::objects::JValueGen::Object(self.jni_ref().new_object(
            "java/lang/Boolean",
            "(Z)V",
            vec![arg0.into()],
        )?);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setBold",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn set_italic(&self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/Boolean;)V");
        let val_1 = jni::objects::JValueGen::Object(self.jni_ref().new_object(
            "java/lang/Boolean",
            "(Z)V",
            vec![arg0.into()],
        )?);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setItalic",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn set_underlined(&self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/Boolean;)V");
        let val_1 = jni::objects::JValueGen::Object(self.jni_ref().new_object(
            "java/lang/Boolean",
            "(Z)V",
            vec![arg0.into()],
        )?);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setUnderlined",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn set_strikethrough(&self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/Boolean;)V");
        let val_1 = jni::objects::JValueGen::Object(self.jni_ref().new_object(
            "java/lang/Boolean",
            "(Z)V",
            vec![arg0.into()],
        )?);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setStrikethrough",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn set_obfuscated(&self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/Boolean;)V");
        let val_1 = jni::objects::JValueGen::Object(self.jni_ref().new_object(
            "java/lang/Boolean",
            "(Z)V",
            vec![arg0.into()],
        )?);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setObfuscated",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn set_insertion(&self, arg0: impl Into<String>) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/String;)V");
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(arg0.into())?,
        ));
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setInsertion",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn set_click_event(
        &self,
        arg0: impl Into<crate::bungee::api::chat::ClickEvent<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lnet/md_5/bungee/api/chat/ClickEvent;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setClickEvent",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn set_hover_event(
        &self,
        arg0: impl Into<crate::bungee::api::chat::HoverEvent<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lnet/md_5/bungee/api/chat/HoverEvent;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setHoverEvent",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn set_reset(&self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        let val_1 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setReset",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn insertion(&self) -> Result<String, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/String;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getInsertion", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }

    pub fn click_event(
        &self,
    ) -> Result<crate::bungee::api::chat::ClickEvent<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lnet/md_5/bungee/api/chat/ClickEvent;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getClickEvent", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::bungee::api::chat::ClickEvent::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn hover_event(
        &self,
    ) -> Result<crate::bungee::api::chat::HoverEvent<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lnet/md_5/bungee/api/chat/HoverEvent;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getHoverEvent", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::bungee::api::chat::HoverEvent::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn extra(
        &self,
    ) -> Result<Vec<crate::bungee::api::chat::BaseComponent<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/List;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getExtra", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let list = blackboxmc_java::util::JavaList::from_raw(&self.0, res.l()?)?;
        let iter = list.iterator()?;
        while iter.has_next()? {
            let obj = iter.next()?;
            new_vec.push(crate::bungee::api::chat::BaseComponent::from_raw(
                &self.0, obj,
            )?);
        }
        Ok(new_vec)
    }

    pub fn set_color(
        &self,
        arg0: impl Into<crate::bungee::api::ChatColor<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lnet/md_5/bungee/api/ChatColor;)V");
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

    pub fn color(&self) -> Result<crate::bungee::api::ChatColor<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lnet/md_5/bungee/api/ChatColor;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getColor", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::bungee::api::ChatColor::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn is_reset(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isReset", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn wait_with_long(
        &self,
        arg0: std::option::Option<i64>,
        arg1: std::option::Option<i32>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "J";
            let val_1 = jni::objects::JValueGen::Long(a.into());
            args.push(val_1);
        }
        if let Some(a) = arg1 {
            sig += "I";
            let val_2 = jni::objects::JValueGen::Int(a.into());
            args.push(val_2);
        }
        sig += ")V";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "wait", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn class(&self) -> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/Class;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getClass", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(unsafe { jni::objects::JClass::from_raw(res.as_jni().l) })
    }

    pub fn notify(&self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notify", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn notify_all(&self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notifyAll", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn instance_of<A>(&self, other: A) -> bool
    where
        A: blackboxmc_general::JNIProvidesClassName,
    {
        let cls = &self.jni_ref().find_class(other.class_name()).unwrap();
        self.jni_ref()
            .is_instance_of(&self.jni_object(), cls)
            .unwrap()
    }
}

impl<'mc> std::string::ToString for TextComponent<'mc> {
    fn to_string(&self) -> String {
        match &self.internal_to_string() {
            Ok(a) => a.clone(),
            Err(err) => format!("Error calling TextComponent.toString: {}", err),
        }
    }
}

impl<'mc> Into<crate::bungee::api::chat::BaseComponent<'mc>> for TextComponent<'mc> {
    fn into(self) -> crate::bungee::api::chat::BaseComponent<'mc> {
        crate::bungee::api::chat::BaseComponent::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting TextComponent into crate::bungee::api::chat::BaseComponent")
    }
}

pub struct TextComponentClass;
impl blackboxmc_general::JNIProvidesClassName for TextComponentClass {
    fn class_name(&self) -> &str {
        "net/md_5/bungee/api/chat/TextComponent"
    }
}

#[derive(PartialEq, Eq)]
pub enum FormatRetentionEnum {
    None,
    Formatting,
    Events,
    All,
}
impl std::fmt::Display for FormatRetentionEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FormatRetentionEnum::None => f.write_str("NONE"),
            FormatRetentionEnum::Formatting => f.write_str("FORMATTING"),
            FormatRetentionEnum::Events => f.write_str("EVENTS"),
            FormatRetentionEnum::All => f.write_str("ALL"),
        }
    }
}
impl<'mc> std::fmt::Display for FormatRetention<'mc> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.2.fmt(f)
    }
}
pub struct FormatRetention<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
    pub FormatRetentionEnum,
);
impl<'mc> std::ops::Deref for FormatRetention<'mc> {
    type Target = FormatRetentionEnum;
    fn deref(&self) -> &Self::Target {
        return &self.2;
    }
}

impl<'mc> JNIRaw<'mc> for FormatRetention<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}

impl<'mc> JNIInstantiatableEnum<'mc> for FormatRetention<'mc> {
    type Enum = FormatRetentionEnum;

    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,

        e: Self::Enum,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate FormatRetention from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "net/md_5/bungee/api/chat/FormatRetention")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a FormatRetention object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj, e))
        }
    }
}

impl<'mc> FormatRetention<'mc> {
    pub const NONE: FormatRetentionEnum = FormatRetentionEnum::None;
    pub const FORMATTING: FormatRetentionEnum = FormatRetentionEnum::Formatting;
    pub const EVENTS: FormatRetentionEnum = FormatRetentionEnum::Events;
    pub const ALL: FormatRetentionEnum = FormatRetentionEnum::All;
    pub fn from_string(str: String) -> std::option::Option<FormatRetentionEnum> {
        match str.as_str() {
            "NONE" => Some(FormatRetentionEnum::None),
            "FORMATTING" => Some(FormatRetentionEnum::Formatting),
            "EVENTS" => Some(FormatRetentionEnum::Events),
            "ALL" => Some(FormatRetentionEnum::All),
            _ => None,
        }
    }

    pub fn value_of(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<String>,
    ) -> Result<FormatRetention<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(jni.new_string(arg0.into())?);
        let cls = jni.find_class("net/md_5/bungee/api/chat/FormatRetention");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(
            cls,
            "valueOf",
            "(Ljava/lang/String;)Lnet/md_5/bungee/api/chat/FormatRetention;",
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
        FormatRetention::from_raw(
            &jni,
            raw_obj,
            FormatRetention::from_string(variant_str)
                .ok_or(eyre::eyre!("String gaven for variant was invalid"))?,
        )
    }

    pub fn instance_of<A>(&self, other: A) -> bool
    where
        A: blackboxmc_general::JNIProvidesClassName,
    {
        let cls = &self.jni_ref().find_class(other.class_name()).unwrap();
        self.jni_ref()
            .is_instance_of(&self.jni_object(), cls)
            .unwrap()
    }
}

pub struct FormatRetentionClass;
impl blackboxmc_general::JNIProvidesClassName for FormatRetentionClass {
    fn class_name(&self) -> &str {
        "net/md_5/bungee/api/chat/FormatRetention"
    }
}

pub struct HoverEvent<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
#[derive(PartialEq, Eq)]
pub enum HoverEventActionEnum {
    ShowText,
    ShowItem,
    ShowEntity,
    ShowAchievement,
}
impl std::fmt::Display for HoverEventActionEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HoverEventActionEnum::ShowText => f.write_str("SHOW_TEXT"),
            HoverEventActionEnum::ShowItem => f.write_str("SHOW_ITEM"),
            HoverEventActionEnum::ShowEntity => f.write_str("SHOW_ENTITY"),
            HoverEventActionEnum::ShowAchievement => f.write_str("SHOW_ACHIEVEMENT"),
        }
    }
}
impl<'mc> std::fmt::Display for HoverEventAction<'mc> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.2.fmt(f)
    }
}
pub struct HoverEventAction<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
    pub HoverEventActionEnum,
);
impl<'mc> std::ops::Deref for HoverEventAction<'mc> {
    type Target = HoverEventActionEnum;
    fn deref(&self) -> &Self::Target {
        return &self.2;
    }
}

impl<'mc> JNIRaw<'mc> for HoverEventAction<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}

impl<'mc> JNIInstantiatableEnum<'mc> for HoverEventAction<'mc> {
    type Enum = HoverEventActionEnum;

    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,

        e: Self::Enum,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate HoverEventAction from null object.").into(),
            );
        }
        let (valid, name) =
            env.validate_name(&obj, "net/md_5/bungee/api/chat/HoverEvent$Action")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a HoverEventAction object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj, e))
        }
    }
}

impl<'mc> HoverEventAction<'mc> {
    pub const SHOW_TEXT: HoverEventActionEnum = HoverEventActionEnum::ShowText;
    pub const SHOW_ITEM: HoverEventActionEnum = HoverEventActionEnum::ShowItem;
    pub const SHOW_ENTITY: HoverEventActionEnum = HoverEventActionEnum::ShowEntity;
    pub const SHOW_ACHIEVEMENT: HoverEventActionEnum = HoverEventActionEnum::ShowAchievement;
    pub fn from_string(str: String) -> std::option::Option<HoverEventActionEnum> {
        match str.as_str() {
            "SHOW_TEXT" => Some(HoverEventActionEnum::ShowText),
            "SHOW_ITEM" => Some(HoverEventActionEnum::ShowItem),
            "SHOW_ENTITY" => Some(HoverEventActionEnum::ShowEntity),
            "SHOW_ACHIEVEMENT" => Some(HoverEventActionEnum::ShowAchievement),
            _ => None,
        }
    }

    pub fn value_of(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<String>,
    ) -> Result<HoverEventAction<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(jni.new_string(arg0.into())?);
        let cls = jni.find_class("net/md_5/bungee/api/chat/HoverEvent$Action");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(
            cls,
            "valueOf",
            "(Ljava/lang/String;)Lnet/md_5/bungee/api/chat/HoverEvent$Action;",
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
        HoverEventAction::from_raw(
            &jni,
            raw_obj,
            HoverEventAction::from_string(variant_str)
                .ok_or(eyre::eyre!("String gaven for variant was invalid"))?,
        )
    }

    pub fn instance_of<A>(&self, other: A) -> bool
    where
        A: blackboxmc_general::JNIProvidesClassName,
    {
        let cls = &self.jni_ref().find_class(other.class_name()).unwrap();
        self.jni_ref()
            .is_instance_of(&self.jni_object(), cls)
            .unwrap()
    }
}

pub struct HoverEventActionClass;
impl blackboxmc_general::JNIProvidesClassName for HoverEventActionClass {
    fn class_name(&self) -> &str {
        "net/md_5/bungee/api/chat/HoverEvent$Action"
    }
}

impl<'mc> JNIRaw<'mc> for HoverEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}

impl<'mc> JNIInstantiatable<'mc> for HoverEvent<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate HoverEvent from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "net/md_5/bungee/api/chat/HoverEvent")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a HoverEvent object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> HoverEvent<'mc> {
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

    pub fn class(&self) -> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/Class;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getClass", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(unsafe { jni::objects::JClass::from_raw(res.as_jni().l) })
    }

    pub fn contents(
        &self,
    ) -> Result<
        Vec<crate::bungee::api::chat::hover::content::Content<'mc>>,
        Box<dyn std::error::Error>,
    > {
        let sig = String::from("()Ljava/util/List;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getContents", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let list = blackboxmc_java::util::JavaList::from_raw(&self.0, res.l()?)?;
        let iter = list.iterator()?;
        while iter.has_next()? {
            let obj = iter.next()?;
            new_vec.push(crate::bungee::api::chat::hover::content::Content::from_raw(
                &self.0, obj,
            )?);
        }
        Ok(new_vec)
    }

    pub fn is_legacy(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isLegacy", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn action(
        &self,
    ) -> Result<crate::bungee::api::chat::HoverEventAction<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lnet/md_5/bungee/api/chat/HoverEvent$Action;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getAction", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant = self
            .0
            .call_method(&raw_obj, "toString", "()Ljava/lang/String;", vec![]);
        let variant = self.jni_ref().translate_error(variant)?;
        let variant_str = self
            .0
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::bungee::api::chat::HoverEventAction::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::bungee::api::chat::HoverEventAction::from_string(variant_str)
                .ok_or(eyre::eyre!("String gaven for variant was invalid"))?,
        )
    }

    pub fn add_content(
        &self,
        arg0: impl Into<crate::bungee::api::chat::hover::content::Content<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lnet/md_5/bungee/api/chat/hover/content/Content;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "addContent",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn set_legacy(&self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        let val_1 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setLegacy",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn wait_with_long(
        &self,
        arg0: std::option::Option<i64>,
        arg1: std::option::Option<i32>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "J";
            let val_1 = jni::objects::JValueGen::Long(a.into());
            args.push(val_1);
        }
        if let Some(a) = arg1 {
            sig += "I";
            let val_2 = jni::objects::JValueGen::Int(a.into());
            args.push(val_2);
        }
        sig += ")V";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "wait", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn notify(&self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notify", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn notify_all(&self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notifyAll", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn instance_of<A>(&self, other: A) -> bool
    where
        A: blackboxmc_general::JNIProvidesClassName,
    {
        let cls = &self.jni_ref().find_class(other.class_name()).unwrap();
        self.jni_ref()
            .is_instance_of(&self.jni_object(), cls)
            .unwrap()
    }
}

impl<'mc> std::string::ToString for HoverEvent<'mc> {
    fn to_string(&self) -> String {
        match &self.internal_to_string() {
            Ok(a) => a.clone(),
            Err(err) => format!("Error calling HoverEvent.toString: {}", err),
        }
    }
}

pub struct HoverEventClass;
impl blackboxmc_general::JNIProvidesClassName for HoverEventClass {
    fn class_name(&self) -> &str {
        "net/md_5/bungee/api/chat/HoverEvent"
    }
}

pub struct BaseComponent<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for BaseComponent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}

impl<'mc> JNIInstantiatable<'mc> for BaseComponent<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate BaseComponent from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "net/md_5/bungee/api/chat/BaseComponent")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a BaseComponent object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> BaseComponent<'mc> {
    #[deprecated]

    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<crate::bungee::api::chat::BaseComponent<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let cls = jni.find_class("net/md_5/bungee/api/chat/BaseComponent");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(cls, sig.as_str(), vec![]);
        let res = jni.translate_error_no_gen(res)?;
        crate::bungee::api::chat::BaseComponent::from_raw(&jni, res)
    }

    pub fn duplicate_without_formatting(
        &self,
    ) -> Result<crate::bungee::api::chat::BaseComponent<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lnet/md_5/bungee/api/chat/BaseComponent;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "duplicateWithoutFormatting",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::bungee::api::chat::BaseComponent::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn copy_formatting_with_base_component(
        &self,
        arg0: impl Into<crate::bungee::api::chat::BaseComponent<'mc>>,
        arg1: std::option::Option<
            impl Into<crate::bungee::api::chat::ComponentBuilderFormatRetention<'mc>>,
        >,
        arg2: std::option::Option<bool>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lnet/md_5/bungee/api/chat/BaseComponent;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        if let Some(a) = arg1 {
            sig += "Lnet/md_5/bungee/api/chat/ComponentBuilder$FormatRetention;";
            let val_2 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(a.into().jni_object().clone())
            });
            args.push(val_2);
        }
        if let Some(a) = arg2 {
            sig += "Z";
            let val_3 = jni::objects::JValueGen::Bool(a.into());
            args.push(val_3);
        }
        sig += ")V";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "copyFormatting", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn retain(
        &self,
        arg0: impl Into<crate::bungee::api::chat::ComponentBuilderFormatRetention<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lnet/md_5/bungee/api/chat/ComponentBuilder$FormatRetention;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "retain",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn to_legacy_text_with_base_components(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: std::option::Option<Vec<impl Into<crate::bungee::api::chat::BaseComponent<'mc>>>>,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Ljava/lang/String;";
        let cls = jni.find_class("java/lang/String");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "toLegacyText", sig.as_str(), args);
        let res = jni.translate_error(res)?;
        Ok(jni
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }

    pub fn to_plain_text_with_base_components(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: std::option::Option<Vec<impl Into<crate::bungee::api::chat::BaseComponent<'mc>>>>,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Ljava/lang/String;";
        let cls = jni.find_class("java/lang/String");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "toPlainText", sig.as_str(), args);
        let res = jni.translate_error(res)?;
        Ok(jni
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }

    pub fn color_raw(
        &self,
    ) -> Result<crate::bungee::api::ChatColor<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lnet/md_5/bungee/api/ChatColor;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getColorRaw", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::bungee::api::ChatColor::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn font(&self) -> Result<String, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/String;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getFont", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }

    pub fn font_raw(&self) -> Result<String, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/String;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getFontRaw", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }

    pub fn is_bold(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isBold", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn is_bold_raw(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/Boolean;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isBoldRaw", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn is_italic(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isItalic", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn is_italic_raw(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/Boolean;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isItalicRaw", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn is_underlined(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isUnderlined", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn is_underlined_raw(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/Boolean;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isUnderlinedRaw", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn is_strikethrough(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isStrikethrough", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn is_strikethrough_raw(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/Boolean;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "isStrikethroughRaw",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn is_obfuscated(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isObfuscated", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn is_obfuscated_raw(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/Boolean;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isObfuscatedRaw", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn add_extra_with_base_component(
        &self,
        arg0: impl Into<crate::bungee::api::chat::BaseComponent<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lnet/md_5/bungee/api/chat/BaseComponent;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        sig += ")V";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "addExtra", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn has_formatting(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "hasFormatting", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn set_font(&self, arg0: impl Into<String>) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/String;)V");
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(arg0.into())?,
        ));
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setFont",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn set_bold(&self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/Boolean;)V");
        let val_1 = jni::objects::JValueGen::Object(self.jni_ref().new_object(
            "java/lang/Boolean",
            "(Z)V",
            vec![arg0.into()],
        )?);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setBold",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn set_italic(&self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/Boolean;)V");
        let val_1 = jni::objects::JValueGen::Object(self.jni_ref().new_object(
            "java/lang/Boolean",
            "(Z)V",
            vec![arg0.into()],
        )?);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setItalic",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn set_underlined(&self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/Boolean;)V");
        let val_1 = jni::objects::JValueGen::Object(self.jni_ref().new_object(
            "java/lang/Boolean",
            "(Z)V",
            vec![arg0.into()],
        )?);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setUnderlined",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn set_strikethrough(&self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/Boolean;)V");
        let val_1 = jni::objects::JValueGen::Object(self.jni_ref().new_object(
            "java/lang/Boolean",
            "(Z)V",
            vec![arg0.into()],
        )?);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setStrikethrough",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn set_obfuscated(&self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/Boolean;)V");
        let val_1 = jni::objects::JValueGen::Object(self.jni_ref().new_object(
            "java/lang/Boolean",
            "(Z)V",
            vec![arg0.into()],
        )?);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setObfuscated",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn set_insertion(&self, arg0: impl Into<String>) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/String;)V");
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(arg0.into())?,
        ));
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setInsertion",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn set_click_event(
        &self,
        arg0: impl Into<crate::bungee::api::chat::ClickEvent<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lnet/md_5/bungee/api/chat/ClickEvent;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setClickEvent",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn set_hover_event(
        &self,
        arg0: impl Into<crate::bungee::api::chat::HoverEvent<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lnet/md_5/bungee/api/chat/HoverEvent;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setHoverEvent",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn set_reset(&self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        let val_1 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setReset",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn insertion(&self) -> Result<String, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/String;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getInsertion", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }

    pub fn click_event(
        &self,
    ) -> Result<crate::bungee::api::chat::ClickEvent<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lnet/md_5/bungee/api/chat/ClickEvent;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getClickEvent", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::bungee::api::chat::ClickEvent::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn hover_event(
        &self,
    ) -> Result<crate::bungee::api::chat::HoverEvent<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lnet/md_5/bungee/api/chat/HoverEvent;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getHoverEvent", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::bungee::api::chat::HoverEvent::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn extra(
        &self,
    ) -> Result<Vec<crate::bungee::api::chat::BaseComponent<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/List;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getExtra", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let list = blackboxmc_java::util::JavaList::from_raw(&self.0, res.l()?)?;
        let iter = list.iterator()?;
        while iter.has_next()? {
            let obj = iter.next()?;
            new_vec.push(crate::bungee::api::chat::BaseComponent::from_raw(
                &self.0, obj,
            )?);
        }
        Ok(new_vec)
    }

    pub fn set_color(
        &self,
        arg0: impl Into<crate::bungee::api::ChatColor<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lnet/md_5/bungee/api/ChatColor;)V");
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

    pub fn duplicate(
        &self,
    ) -> Result<crate::bungee::api::chat::BaseComponent<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lnet/md_5/bungee/api/chat/BaseComponent;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "duplicate", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::bungee::api::chat::BaseComponent::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn color(&self) -> Result<crate::bungee::api::ChatColor<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lnet/md_5/bungee/api/ChatColor;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getColor", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::bungee::api::ChatColor::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn is_reset(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isReset", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn wait_with_long(
        &self,
        arg0: std::option::Option<i64>,
        arg1: std::option::Option<i32>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "J";
            let val_1 = jni::objects::JValueGen::Long(a.into());
            args.push(val_1);
        }
        if let Some(a) = arg1 {
            sig += "I";
            let val_2 = jni::objects::JValueGen::Int(a.into());
            args.push(val_2);
        }
        sig += ")V";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "wait", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn class(&self) -> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/Class;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getClass", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(unsafe { jni::objects::JClass::from_raw(res.as_jni().l) })
    }

    pub fn notify(&self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notify", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn notify_all(&self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notifyAll", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn instance_of<A>(&self, other: A) -> bool
    where
        A: blackboxmc_general::JNIProvidesClassName,
    {
        let cls = &self.jni_ref().find_class(other.class_name()).unwrap();
        self.jni_ref()
            .is_instance_of(&self.jni_object(), cls)
            .unwrap()
    }
}

impl<'mc> std::string::ToString for BaseComponent<'mc> {
    fn to_string(&self) -> String {
        match &self.internal_to_string() {
            Ok(a) => a.clone(),
            Err(err) => format!("Error calling BaseComponent.toString: {}", err),
        }
    }
}

pub struct BaseComponentClass;
impl blackboxmc_general::JNIProvidesClassName for BaseComponentClass {
    fn class_name(&self) -> &str {
        "net/md_5/bungee/api/chat/BaseComponent"
    }
}

pub struct SelectorComponent<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for SelectorComponent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}

impl<'mc> JNIInstantiatable<'mc> for SelectorComponent<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate SelectorComponent from null object.").into(),
            );
        }
        let (valid, name) =
            env.validate_name(&obj, "net/md_5/bungee/api/chat/SelectorComponent")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a SelectorComponent object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> SelectorComponent<'mc> {
    pub fn new_with_selector_component(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<crate::bungee::api::chat::SelectorComponent<'mc>>,
    ) -> Result<crate::bungee::api::chat::SelectorComponent<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lnet/md_5/bungee/api/chat/SelectorComponent;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        sig += ")V";
        let cls = jni.find_class("net/md_5/bungee/api/chat/SelectorComponent");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(cls, sig.as_str(), args);
        let res = jni.translate_error_no_gen(res)?;
        crate::bungee::api::chat::SelectorComponent::from_raw(&jni, res)
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

    pub fn duplicate(
        &self,
    ) -> Result<crate::bungee::api::chat::SelectorComponent<'mc>, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Lnet/md_5/bungee/api/chat/SelectorComponent;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "duplicate", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        crate::bungee::api::chat::SelectorComponent::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn selector(&self) -> Result<String, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/String;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getSelector", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }

    pub fn set_selector(&self, arg0: impl Into<String>) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/String;)V");
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(arg0.into())?,
        ));
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setSelector",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn duplicate_without_formatting(
        &self,
    ) -> Result<crate::bungee::api::chat::BaseComponent<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lnet/md_5/bungee/api/chat/BaseComponent;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "duplicateWithoutFormatting",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::bungee::api::chat::BaseComponent::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn copy_formatting_with_base_component(
        &self,
        arg0: impl Into<crate::bungee::api::chat::BaseComponent<'mc>>,
        arg1: std::option::Option<
            impl Into<crate::bungee::api::chat::ComponentBuilderFormatRetention<'mc>>,
        >,
        arg2: std::option::Option<bool>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lnet/md_5/bungee/api/chat/BaseComponent;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        if let Some(a) = arg1 {
            sig += "Lnet/md_5/bungee/api/chat/ComponentBuilder$FormatRetention;";
            let val_2 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(a.into().jni_object().clone())
            });
            args.push(val_2);
        }
        if let Some(a) = arg2 {
            sig += "Z";
            let val_3 = jni::objects::JValueGen::Bool(a.into());
            args.push(val_3);
        }
        sig += ")V";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "copyFormatting", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn retain(
        &self,
        arg0: impl Into<crate::bungee::api::chat::ComponentBuilderFormatRetention<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lnet/md_5/bungee/api/chat/ComponentBuilder$FormatRetention;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "retain",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn to_legacy_text_with_base_components(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: std::option::Option<Vec<impl Into<crate::bungee::api::chat::BaseComponent<'mc>>>>,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Ljava/lang/String;";
        let cls = jni.find_class("java/lang/String");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "toLegacyText", sig.as_str(), args);
        let res = jni.translate_error(res)?;
        Ok(jni
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }

    pub fn to_plain_text_with_base_components(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: std::option::Option<Vec<impl Into<crate::bungee::api::chat::BaseComponent<'mc>>>>,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Ljava/lang/String;";
        let cls = jni.find_class("java/lang/String");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "toPlainText", sig.as_str(), args);
        let res = jni.translate_error(res)?;
        Ok(jni
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }

    pub fn color_raw(
        &self,
    ) -> Result<crate::bungee::api::ChatColor<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lnet/md_5/bungee/api/ChatColor;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getColorRaw", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::bungee::api::ChatColor::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn font(&self) -> Result<String, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/String;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getFont", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }

    pub fn font_raw(&self) -> Result<String, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/String;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getFontRaw", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }

    pub fn is_bold(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isBold", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn is_bold_raw(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/Boolean;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isBoldRaw", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn is_italic(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isItalic", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn is_italic_raw(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/Boolean;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isItalicRaw", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn is_underlined(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isUnderlined", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn is_underlined_raw(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/Boolean;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isUnderlinedRaw", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn is_strikethrough(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isStrikethrough", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn is_strikethrough_raw(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/Boolean;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "isStrikethroughRaw",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn is_obfuscated(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isObfuscated", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn is_obfuscated_raw(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/Boolean;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isObfuscatedRaw", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn add_extra_with_base_component(
        &self,
        arg0: impl Into<crate::bungee::api::chat::BaseComponent<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lnet/md_5/bungee/api/chat/BaseComponent;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        sig += ")V";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "addExtra", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn has_formatting(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "hasFormatting", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn set_font(&self, arg0: impl Into<String>) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/String;)V");
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(arg0.into())?,
        ));
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setFont",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn set_bold(&self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/Boolean;)V");
        let val_1 = jni::objects::JValueGen::Object(self.jni_ref().new_object(
            "java/lang/Boolean",
            "(Z)V",
            vec![arg0.into()],
        )?);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setBold",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn set_italic(&self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/Boolean;)V");
        let val_1 = jni::objects::JValueGen::Object(self.jni_ref().new_object(
            "java/lang/Boolean",
            "(Z)V",
            vec![arg0.into()],
        )?);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setItalic",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn set_underlined(&self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/Boolean;)V");
        let val_1 = jni::objects::JValueGen::Object(self.jni_ref().new_object(
            "java/lang/Boolean",
            "(Z)V",
            vec![arg0.into()],
        )?);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setUnderlined",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn set_strikethrough(&self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/Boolean;)V");
        let val_1 = jni::objects::JValueGen::Object(self.jni_ref().new_object(
            "java/lang/Boolean",
            "(Z)V",
            vec![arg0.into()],
        )?);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setStrikethrough",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn set_obfuscated(&self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/Boolean;)V");
        let val_1 = jni::objects::JValueGen::Object(self.jni_ref().new_object(
            "java/lang/Boolean",
            "(Z)V",
            vec![arg0.into()],
        )?);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setObfuscated",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn set_insertion(&self, arg0: impl Into<String>) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/String;)V");
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(arg0.into())?,
        ));
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setInsertion",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn set_click_event(
        &self,
        arg0: impl Into<crate::bungee::api::chat::ClickEvent<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lnet/md_5/bungee/api/chat/ClickEvent;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setClickEvent",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn set_hover_event(
        &self,
        arg0: impl Into<crate::bungee::api::chat::HoverEvent<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lnet/md_5/bungee/api/chat/HoverEvent;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setHoverEvent",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn set_reset(&self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        let val_1 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setReset",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn insertion(&self) -> Result<String, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/String;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getInsertion", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }

    pub fn click_event(
        &self,
    ) -> Result<crate::bungee::api::chat::ClickEvent<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lnet/md_5/bungee/api/chat/ClickEvent;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getClickEvent", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::bungee::api::chat::ClickEvent::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn hover_event(
        &self,
    ) -> Result<crate::bungee::api::chat::HoverEvent<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lnet/md_5/bungee/api/chat/HoverEvent;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getHoverEvent", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::bungee::api::chat::HoverEvent::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn extra(
        &self,
    ) -> Result<Vec<crate::bungee::api::chat::BaseComponent<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/List;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getExtra", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let list = blackboxmc_java::util::JavaList::from_raw(&self.0, res.l()?)?;
        let iter = list.iterator()?;
        while iter.has_next()? {
            let obj = iter.next()?;
            new_vec.push(crate::bungee::api::chat::BaseComponent::from_raw(
                &self.0, obj,
            )?);
        }
        Ok(new_vec)
    }

    pub fn set_color(
        &self,
        arg0: impl Into<crate::bungee::api::ChatColor<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lnet/md_5/bungee/api/ChatColor;)V");
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

    pub fn color(&self) -> Result<crate::bungee::api::ChatColor<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lnet/md_5/bungee/api/ChatColor;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getColor", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::bungee::api::ChatColor::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn is_reset(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isReset", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn wait_with_long(
        &self,
        arg0: std::option::Option<i64>,
        arg1: std::option::Option<i32>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "J";
            let val_1 = jni::objects::JValueGen::Long(a.into());
            args.push(val_1);
        }
        if let Some(a) = arg1 {
            sig += "I";
            let val_2 = jni::objects::JValueGen::Int(a.into());
            args.push(val_2);
        }
        sig += ")V";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "wait", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn class(&self) -> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/Class;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getClass", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(unsafe { jni::objects::JClass::from_raw(res.as_jni().l) })
    }

    pub fn notify(&self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notify", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn notify_all(&self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notifyAll", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn instance_of<A>(&self, other: A) -> bool
    where
        A: blackboxmc_general::JNIProvidesClassName,
    {
        let cls = &self.jni_ref().find_class(other.class_name()).unwrap();
        self.jni_ref()
            .is_instance_of(&self.jni_object(), cls)
            .unwrap()
    }
}

impl<'mc> std::string::ToString for SelectorComponent<'mc> {
    fn to_string(&self) -> String {
        match &self.internal_to_string() {
            Ok(a) => a.clone(),
            Err(err) => format!("Error calling SelectorComponent.toString: {}", err),
        }
    }
}

impl<'mc> Into<crate::bungee::api::chat::BaseComponent<'mc>> for SelectorComponent<'mc> {
    fn into(self) -> crate::bungee::api::chat::BaseComponent<'mc> {
        crate::bungee::api::chat::BaseComponent::from_raw(&self.jni_ref(), self.1).expect(
            "Error converting SelectorComponent into crate::bungee::api::chat::BaseComponent",
        )
    }
}

pub struct SelectorComponentClass;
impl blackboxmc_general::JNIProvidesClassName for SelectorComponentClass {
    fn class_name(&self) -> &str {
        "net/md_5/bungee/api/chat/SelectorComponent"
    }
}

pub struct ScoreComponent<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for ScoreComponent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}

impl<'mc> JNIInstantiatable<'mc> for ScoreComponent<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate ScoreComponent from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "net/md_5/bungee/api/chat/ScoreComponent")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a ScoreComponent object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> ScoreComponent<'mc> {
    pub fn new_with_string(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<String>,
        arg1: std::option::Option<impl Into<String>>,
        arg2: std::option::Option<impl Into<String>>,
    ) -> Result<crate::bungee::api::chat::ScoreComponent<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Ljava/lang/String;";
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            jni.new_string(arg0.into())?,
        ));
        args.push(val_1);
        if let Some(a) = arg1 {
            sig += "Ljava/lang/String;";
            let val_2 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
                jni.new_string(a.into())?,
            ));
            args.push(val_2);
        }
        if let Some(a) = arg2 {
            sig += "Ljava/lang/String;";
            let val_3 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
                jni.new_string(a.into())?,
            ));
            args.push(val_3);
        }
        sig += ")V";
        let cls = jni.find_class("net/md_5/bungee/api/chat/ScoreComponent");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(cls, sig.as_str(), args);
        let res = jni.translate_error_no_gen(res)?;
        crate::bungee::api::chat::ScoreComponent::from_raw(&jni, res)
    }

    pub fn name(&self) -> Result<String, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/String;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getName", sig.as_str(), vec![]);
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

    pub fn value(&self) -> Result<String, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/String;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getValue", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }

    pub fn set_name(&self, arg0: impl Into<String>) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/String;)V");
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(arg0.into())?,
        ));
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setName",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn set_value(&self, arg0: impl Into<String>) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/String;)V");
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(arg0.into())?,
        ));
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setValue",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn duplicate(
        &self,
    ) -> Result<crate::bungee::api::chat::BaseComponent<'mc>, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Lnet/md_5/bungee/api/chat/BaseComponent;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "duplicate", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        crate::bungee::api::chat::BaseComponent::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn set_objective(&self, arg0: impl Into<String>) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/String;)V");
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(arg0.into())?,
        ));
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setObjective",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn objective(&self) -> Result<String, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/String;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getObjective", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }

    pub fn duplicate_without_formatting(
        &self,
    ) -> Result<crate::bungee::api::chat::BaseComponent<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lnet/md_5/bungee/api/chat/BaseComponent;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "duplicateWithoutFormatting",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::bungee::api::chat::BaseComponent::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn copy_formatting_with_base_component(
        &self,
        arg0: impl Into<crate::bungee::api::chat::BaseComponent<'mc>>,
        arg1: std::option::Option<
            impl Into<crate::bungee::api::chat::ComponentBuilderFormatRetention<'mc>>,
        >,
        arg2: std::option::Option<bool>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lnet/md_5/bungee/api/chat/BaseComponent;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        if let Some(a) = arg1 {
            sig += "Lnet/md_5/bungee/api/chat/ComponentBuilder$FormatRetention;";
            let val_2 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(a.into().jni_object().clone())
            });
            args.push(val_2);
        }
        if let Some(a) = arg2 {
            sig += "Z";
            let val_3 = jni::objects::JValueGen::Bool(a.into());
            args.push(val_3);
        }
        sig += ")V";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "copyFormatting", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn retain(
        &self,
        arg0: impl Into<crate::bungee::api::chat::ComponentBuilderFormatRetention<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lnet/md_5/bungee/api/chat/ComponentBuilder$FormatRetention;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "retain",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn to_legacy_text_with_base_components(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: std::option::Option<Vec<impl Into<crate::bungee::api::chat::BaseComponent<'mc>>>>,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Ljava/lang/String;";
        let cls = jni.find_class("java/lang/String");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "toLegacyText", sig.as_str(), args);
        let res = jni.translate_error(res)?;
        Ok(jni
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }

    pub fn to_plain_text_with_base_components(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: std::option::Option<Vec<impl Into<crate::bungee::api::chat::BaseComponent<'mc>>>>,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Ljava/lang/String;";
        let cls = jni.find_class("java/lang/String");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "toPlainText", sig.as_str(), args);
        let res = jni.translate_error(res)?;
        Ok(jni
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }

    pub fn color_raw(
        &self,
    ) -> Result<crate::bungee::api::ChatColor<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lnet/md_5/bungee/api/ChatColor;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getColorRaw", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::bungee::api::ChatColor::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn font(&self) -> Result<String, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/String;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getFont", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }

    pub fn font_raw(&self) -> Result<String, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/String;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getFontRaw", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }

    pub fn is_bold(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isBold", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn is_bold_raw(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/Boolean;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isBoldRaw", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn is_italic(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isItalic", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn is_italic_raw(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/Boolean;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isItalicRaw", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn is_underlined(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isUnderlined", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn is_underlined_raw(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/Boolean;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isUnderlinedRaw", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn is_strikethrough(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isStrikethrough", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn is_strikethrough_raw(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/Boolean;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "isStrikethroughRaw",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn is_obfuscated(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isObfuscated", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn is_obfuscated_raw(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/Boolean;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isObfuscatedRaw", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn add_extra_with_base_component(
        &self,
        arg0: impl Into<crate::bungee::api::chat::BaseComponent<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lnet/md_5/bungee/api/chat/BaseComponent;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        sig += ")V";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "addExtra", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn has_formatting(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "hasFormatting", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn set_font(&self, arg0: impl Into<String>) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/String;)V");
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(arg0.into())?,
        ));
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setFont",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn set_bold(&self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/Boolean;)V");
        let val_1 = jni::objects::JValueGen::Object(self.jni_ref().new_object(
            "java/lang/Boolean",
            "(Z)V",
            vec![arg0.into()],
        )?);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setBold",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn set_italic(&self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/Boolean;)V");
        let val_1 = jni::objects::JValueGen::Object(self.jni_ref().new_object(
            "java/lang/Boolean",
            "(Z)V",
            vec![arg0.into()],
        )?);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setItalic",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn set_underlined(&self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/Boolean;)V");
        let val_1 = jni::objects::JValueGen::Object(self.jni_ref().new_object(
            "java/lang/Boolean",
            "(Z)V",
            vec![arg0.into()],
        )?);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setUnderlined",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn set_strikethrough(&self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/Boolean;)V");
        let val_1 = jni::objects::JValueGen::Object(self.jni_ref().new_object(
            "java/lang/Boolean",
            "(Z)V",
            vec![arg0.into()],
        )?);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setStrikethrough",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn set_obfuscated(&self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/Boolean;)V");
        let val_1 = jni::objects::JValueGen::Object(self.jni_ref().new_object(
            "java/lang/Boolean",
            "(Z)V",
            vec![arg0.into()],
        )?);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setObfuscated",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn set_insertion(&self, arg0: impl Into<String>) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/String;)V");
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(arg0.into())?,
        ));
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setInsertion",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn set_click_event(
        &self,
        arg0: impl Into<crate::bungee::api::chat::ClickEvent<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lnet/md_5/bungee/api/chat/ClickEvent;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setClickEvent",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn set_hover_event(
        &self,
        arg0: impl Into<crate::bungee::api::chat::HoverEvent<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lnet/md_5/bungee/api/chat/HoverEvent;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setHoverEvent",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn set_reset(&self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        let val_1 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setReset",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn insertion(&self) -> Result<String, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/String;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getInsertion", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }

    pub fn click_event(
        &self,
    ) -> Result<crate::bungee::api::chat::ClickEvent<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lnet/md_5/bungee/api/chat/ClickEvent;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getClickEvent", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::bungee::api::chat::ClickEvent::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn hover_event(
        &self,
    ) -> Result<crate::bungee::api::chat::HoverEvent<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lnet/md_5/bungee/api/chat/HoverEvent;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getHoverEvent", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::bungee::api::chat::HoverEvent::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn extra(
        &self,
    ) -> Result<Vec<crate::bungee::api::chat::BaseComponent<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/List;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getExtra", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let list = blackboxmc_java::util::JavaList::from_raw(&self.0, res.l()?)?;
        let iter = list.iterator()?;
        while iter.has_next()? {
            let obj = iter.next()?;
            new_vec.push(crate::bungee::api::chat::BaseComponent::from_raw(
                &self.0, obj,
            )?);
        }
        Ok(new_vec)
    }

    pub fn set_color(
        &self,
        arg0: impl Into<crate::bungee::api::ChatColor<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lnet/md_5/bungee/api/ChatColor;)V");
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

    pub fn color(&self) -> Result<crate::bungee::api::ChatColor<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lnet/md_5/bungee/api/ChatColor;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getColor", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::bungee::api::ChatColor::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn is_reset(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isReset", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn wait_with_long(
        &self,
        arg0: std::option::Option<i64>,
        arg1: std::option::Option<i32>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "J";
            let val_1 = jni::objects::JValueGen::Long(a.into());
            args.push(val_1);
        }
        if let Some(a) = arg1 {
            sig += "I";
            let val_2 = jni::objects::JValueGen::Int(a.into());
            args.push(val_2);
        }
        sig += ")V";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "wait", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn class(&self) -> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/Class;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getClass", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(unsafe { jni::objects::JClass::from_raw(res.as_jni().l) })
    }

    pub fn notify(&self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notify", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn notify_all(&self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notifyAll", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn instance_of<A>(&self, other: A) -> bool
    where
        A: blackboxmc_general::JNIProvidesClassName,
    {
        let cls = &self.jni_ref().find_class(other.class_name()).unwrap();
        self.jni_ref()
            .is_instance_of(&self.jni_object(), cls)
            .unwrap()
    }
}

impl<'mc> std::string::ToString for ScoreComponent<'mc> {
    fn to_string(&self) -> String {
        match &self.internal_to_string() {
            Ok(a) => a.clone(),
            Err(err) => format!("Error calling ScoreComponent.toString: {}", err),
        }
    }
}

impl<'mc> Into<crate::bungee::api::chat::BaseComponent<'mc>> for ScoreComponent<'mc> {
    fn into(self) -> crate::bungee::api::chat::BaseComponent<'mc> {
        crate::bungee::api::chat::BaseComponent::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting ScoreComponent into crate::bungee::api::chat::BaseComponent")
    }
}

pub struct ScoreComponentClass;
impl blackboxmc_general::JNIProvidesClassName for ScoreComponentClass {
    fn class_name(&self) -> &str {
        "net/md_5/bungee/api/chat/ScoreComponent"
    }
}

pub struct ClickEvent<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
#[derive(PartialEq, Eq)]
pub enum ClickEventActionEnum {
    OpenUrl,
    OpenFile,
    RunCommand,
    SuggestCommand,
    ChangePage,
    CopyToClipboard,
}
impl std::fmt::Display for ClickEventActionEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ClickEventActionEnum::OpenUrl => f.write_str("OPEN_URL"),
            ClickEventActionEnum::OpenFile => f.write_str("OPEN_FILE"),
            ClickEventActionEnum::RunCommand => f.write_str("RUN_COMMAND"),
            ClickEventActionEnum::SuggestCommand => f.write_str("SUGGEST_COMMAND"),
            ClickEventActionEnum::ChangePage => f.write_str("CHANGE_PAGE"),
            ClickEventActionEnum::CopyToClipboard => f.write_str("COPY_TO_CLIPBOARD"),
        }
    }
}
impl<'mc> std::fmt::Display for ClickEventAction<'mc> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.2.fmt(f)
    }
}
pub struct ClickEventAction<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
    pub ClickEventActionEnum,
);
impl<'mc> std::ops::Deref for ClickEventAction<'mc> {
    type Target = ClickEventActionEnum;
    fn deref(&self) -> &Self::Target {
        return &self.2;
    }
}

impl<'mc> JNIRaw<'mc> for ClickEventAction<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}

impl<'mc> JNIInstantiatableEnum<'mc> for ClickEventAction<'mc> {
    type Enum = ClickEventActionEnum;

    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,

        e: Self::Enum,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate ClickEventAction from null object.").into(),
            );
        }
        let (valid, name) =
            env.validate_name(&obj, "net/md_5/bungee/api/chat/ClickEvent$Action")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a ClickEventAction object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj, e))
        }
    }
}

impl<'mc> ClickEventAction<'mc> {
    pub const OPEN_URL: ClickEventActionEnum = ClickEventActionEnum::OpenUrl;
    pub const OPEN_FILE: ClickEventActionEnum = ClickEventActionEnum::OpenFile;
    pub const RUN_COMMAND: ClickEventActionEnum = ClickEventActionEnum::RunCommand;
    pub const SUGGEST_COMMAND: ClickEventActionEnum = ClickEventActionEnum::SuggestCommand;
    pub const CHANGE_PAGE: ClickEventActionEnum = ClickEventActionEnum::ChangePage;
    pub const COPY_TO_CLIPBOARD: ClickEventActionEnum = ClickEventActionEnum::CopyToClipboard;
    pub fn from_string(str: String) -> std::option::Option<ClickEventActionEnum> {
        match str.as_str() {
            "OPEN_URL" => Some(ClickEventActionEnum::OpenUrl),
            "OPEN_FILE" => Some(ClickEventActionEnum::OpenFile),
            "RUN_COMMAND" => Some(ClickEventActionEnum::RunCommand),
            "SUGGEST_COMMAND" => Some(ClickEventActionEnum::SuggestCommand),
            "CHANGE_PAGE" => Some(ClickEventActionEnum::ChangePage),
            "COPY_TO_CLIPBOARD" => Some(ClickEventActionEnum::CopyToClipboard),
            _ => None,
        }
    }

    pub fn value_of(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<String>,
    ) -> Result<ClickEventAction<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(jni.new_string(arg0.into())?);
        let cls = jni.find_class("net/md_5/bungee/api/chat/ClickEvent$Action");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(
            cls,
            "valueOf",
            "(Ljava/lang/String;)Lnet/md_5/bungee/api/chat/ClickEvent$Action;",
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
        ClickEventAction::from_raw(
            &jni,
            raw_obj,
            ClickEventAction::from_string(variant_str)
                .ok_or(eyre::eyre!("String gaven for variant was invalid"))?,
        )
    }

    pub fn instance_of<A>(&self, other: A) -> bool
    where
        A: blackboxmc_general::JNIProvidesClassName,
    {
        let cls = &self.jni_ref().find_class(other.class_name()).unwrap();
        self.jni_ref()
            .is_instance_of(&self.jni_object(), cls)
            .unwrap()
    }
}

pub struct ClickEventActionClass;
impl blackboxmc_general::JNIProvidesClassName for ClickEventActionClass {
    fn class_name(&self) -> &str {
        "net/md_5/bungee/api/chat/ClickEvent$Action"
    }
}

impl<'mc> JNIRaw<'mc> for ClickEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}

impl<'mc> JNIInstantiatable<'mc> for ClickEvent<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate ClickEvent from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "net/md_5/bungee/api/chat/ClickEvent")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a ClickEvent object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> ClickEvent<'mc> {
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<crate::bungee::api::chat::ClickEventAction<'mc>>,
        arg1: impl Into<String>,
    ) -> Result<crate::bungee::api::chat::ClickEvent<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Lnet/md_5/bungee/api/chat/ClickEvent$Action;Ljava/lang/String;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let val_2 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            jni.new_string(arg1.into())?,
        ));
        let cls = jni.find_class("net/md_5/bungee/api/chat/ClickEvent");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(
            cls,
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
            ],
        );
        let res = jni.translate_error_no_gen(res)?;
        crate::bungee::api::chat::ClickEvent::from_raw(&jni, res)
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

    pub fn value(&self) -> Result<String, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/String;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getValue", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }

    pub fn action(
        &self,
    ) -> Result<crate::bungee::api::chat::ClickEventAction<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lnet/md_5/bungee/api/chat/ClickEvent$Action;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getAction", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant = self
            .0
            .call_method(&raw_obj, "toString", "()Ljava/lang/String;", vec![]);
        let variant = self.jni_ref().translate_error(variant)?;
        let variant_str = self
            .0
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::bungee::api::chat::ClickEventAction::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::bungee::api::chat::ClickEventAction::from_string(variant_str)
                .ok_or(eyre::eyre!("String gaven for variant was invalid"))?,
        )
    }

    pub fn wait_with_long(
        &self,
        arg0: std::option::Option<i64>,
        arg1: std::option::Option<i32>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "J";
            let val_1 = jni::objects::JValueGen::Long(a.into());
            args.push(val_1);
        }
        if let Some(a) = arg1 {
            sig += "I";
            let val_2 = jni::objects::JValueGen::Int(a.into());
            args.push(val_2);
        }
        sig += ")V";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "wait", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn class(&self) -> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/Class;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getClass", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(unsafe { jni::objects::JClass::from_raw(res.as_jni().l) })
    }

    pub fn notify(&self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notify", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn notify_all(&self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notifyAll", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn instance_of<A>(&self, other: A) -> bool
    where
        A: blackboxmc_general::JNIProvidesClassName,
    {
        let cls = &self.jni_ref().find_class(other.class_name()).unwrap();
        self.jni_ref()
            .is_instance_of(&self.jni_object(), cls)
            .unwrap()
    }
}

impl<'mc> std::string::ToString for ClickEvent<'mc> {
    fn to_string(&self) -> String {
        match &self.internal_to_string() {
            Ok(a) => a.clone(),
            Err(err) => format!("Error calling ClickEvent.toString: {}", err),
        }
    }
}

pub struct ClickEventClass;
impl blackboxmc_general::JNIProvidesClassName for ClickEventClass {
    fn class_name(&self) -> &str {
        "net/md_5/bungee/api/chat/ClickEvent"
    }
}

///
/// This is a representation of an abstract class.
pub struct Keybinds<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for Keybinds<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}

impl<'mc> JNIInstantiatable<'mc> for Keybinds<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Keybinds from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "net/md_5/bungee/api/chat/Keybinds")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Keybinds object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> Keybinds<'mc> {
    pub fn instance_of<A>(&self, other: A) -> bool
    where
        A: blackboxmc_general::JNIProvidesClassName,
    {
        let cls = &self.jni_ref().find_class(other.class_name()).unwrap();
        self.jni_ref()
            .is_instance_of(&self.jni_object(), cls)
            .unwrap()
    }
}

pub struct KeybindsClass;
impl blackboxmc_general::JNIProvidesClassName for KeybindsClass {
    fn class_name(&self) -> &str {
        "net/md_5/bungee/api/chat/Keybinds"
    }
}

pub struct ItemTagSerializer<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for ItemTagSerializer<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}

impl<'mc> JNIInstantiatable<'mc> for ItemTagSerializer<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate ItemTagSerializer from null object.").into(),
            );
        }
        let (valid, name) =
            env.validate_name(&obj, "net/md_5/bungee/api/chat/ItemTag$Serializer")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a ItemTagSerializer object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> ItemTagSerializer<'mc> {
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<crate::bungee::api::chat::ItemTagSerializer<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let cls = jni.find_class("net/md_5/bungee/api/chat/ItemTag$Serializer");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(cls, sig.as_str(), vec![]);
        let res = jni.translate_error_no_gen(res)?;
        crate::bungee::api::chat::ItemTagSerializer::from_raw(&jni, res)
    }

    pub fn serialize_with_item_tag(
        &self,
        arg0: impl Into<crate::bungee::api::chat::ItemTag<'mc>>,
        arg1: jni::objects::JObject<'mc>,
        arg2: jni::objects::JObject<'mc>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lnet/md_5/bungee/api/chat/ItemTag;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        sig += "Ljava/lang/reflect/Type;";
        let val_2 = jni::objects::JValueGen::Object(arg1);
        args.push(val_2);
        sig += "Lcom/google/gson/JsonSerializationContext;";
        let val_3 = jni::objects::JValueGen::Object(arg2);
        args.push(val_3);
        sig += ")Lcom/google/gson/JsonElement;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "serialize", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l()?)
    }

    pub fn deserialize_with_json_element(
        &self,
        arg0: jni::objects::JObject<'mc>,
        arg1: jni::objects::JObject<'mc>,
        arg2: jni::objects::JObject<'mc>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lcom/google/gson/JsonElement;";
        let val_1 = jni::objects::JValueGen::Object(arg0);
        args.push(val_1);
        sig += "Ljava/lang/reflect/Type;";
        let val_2 = jni::objects::JValueGen::Object(arg1);
        args.push(val_2);
        sig += "Lcom/google/gson/JsonDeserializationContext;";
        let val_3 = jni::objects::JValueGen::Object(arg2);
        args.push(val_3);
        sig += ")Ljava/lang/Object;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "deserialize", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l()?)
    }

    pub fn wait_with_long(
        &self,
        arg0: std::option::Option<i64>,
        arg1: std::option::Option<i32>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "J";
            let val_1 = jni::objects::JValueGen::Long(a.into());
            args.push(val_1);
        }
        if let Some(a) = arg1 {
            sig += "I";
            let val_2 = jni::objects::JValueGen::Int(a.into());
            args.push(val_2);
        }
        sig += ")V";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "wait", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
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

    pub fn class(&self) -> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/Class;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getClass", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(unsafe { jni::objects::JClass::from_raw(res.as_jni().l) })
    }

    pub fn notify(&self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notify", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn notify_all(&self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notifyAll", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn instance_of<A>(&self, other: A) -> bool
    where
        A: blackboxmc_general::JNIProvidesClassName,
    {
        let cls = &self.jni_ref().find_class(other.class_name()).unwrap();
        self.jni_ref()
            .is_instance_of(&self.jni_object(), cls)
            .unwrap()
    }
}

impl<'mc> std::string::ToString for ItemTagSerializer<'mc> {
    fn to_string(&self) -> String {
        match &self.internal_to_string() {
            Ok(a) => a.clone(),
            Err(err) => format!("Error calling ItemTagSerializer.toString: {}", err),
        }
    }
}

impl<'mc> Into<jni::objects::JObject<'mc>> for ItemTagSerializer<'mc> {
    fn into(self) -> jni::objects::JObject<'mc> {
        self.1
    }
}

pub struct ItemTagSerializerClass;
impl blackboxmc_general::JNIProvidesClassName for ItemTagSerializerClass {
    fn class_name(&self) -> &str {
        "net/md_5/bungee/api/chat/ItemTag$Serializer"
    }
}

pub struct TranslatableComponent<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for TranslatableComponent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}

impl<'mc> JNIInstantiatable<'mc> for TranslatableComponent<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate TranslatableComponent from null object."
            )
            .into());
        }
        let (valid, name) =
            env.validate_name(&obj, "net/md_5/bungee/api/chat/TranslatableComponent")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a TranslatableComponent object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> TranslatableComponent<'mc> {
    pub fn new_with_string(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: std::option::Option<impl Into<String>>,
        arg1: std::option::Option<Vec<jni::objects::JObject<'mc>>>,
    ) -> Result<crate::bungee::api::chat::TranslatableComponent<'mc>, Box<dyn std::error::Error>>
    {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "Ljava/lang/String;";
            let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
                jni.new_string(a.into())?,
            ));
            args.push(val_1);
        }
        sig += ")V";
        let cls = jni.find_class("net/md_5/bungee/api/chat/TranslatableComponent");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(cls, sig.as_str(), args);
        let res = jni.translate_error_no_gen(res)?;
        crate::bungee::api::chat::TranslatableComponent::from_raw(&jni, res)
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

    pub fn duplicate(
        &self,
    ) -> Result<crate::bungee::api::chat::TranslatableComponent<'mc>, Box<dyn std::error::Error>>
    {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Lnet/md_5/bungee/api/chat/TranslatableComponent;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "duplicate", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        crate::bungee::api::chat::TranslatableComponent::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn add_with_with_string(
        &self,
        arg0: impl Into<String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Ljava/lang/String;";
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(arg0.into())?,
        ));
        args.push(val_1);
        sig += ")V";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "addWith", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn translate(&self) -> Result<String, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/String;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getTranslate", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }

    pub fn with(
        &self,
    ) -> Result<Vec<crate::bungee::api::chat::BaseComponent<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/List;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getWith", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let list = blackboxmc_java::util::JavaList::from_raw(&self.0, res.l()?)?;
        let iter = list.iterator()?;
        while iter.has_next()? {
            let obj = iter.next()?;
            new_vec.push(crate::bungee::api::chat::BaseComponent::from_raw(
                &self.0, obj,
            )?);
        }
        Ok(new_vec)
    }

    pub fn fallback(&self) -> Result<String, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/String;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getFallback", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }

    pub fn set_translate(&self, arg0: impl Into<String>) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/String;)V");
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(arg0.into())?,
        ));
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setTranslate",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn set_fallback(&self, arg0: impl Into<String>) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/String;)V");
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(arg0.into())?,
        ));
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setFallback",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn format(
        &self,
    ) -> Result<blackboxmc_java::util::regex::JavaPattern<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/regex/Pattern;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getFormat", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::util::regex::JavaPattern::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn duplicate_without_formatting(
        &self,
    ) -> Result<crate::bungee::api::chat::BaseComponent<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lnet/md_5/bungee/api/chat/BaseComponent;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "duplicateWithoutFormatting",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::bungee::api::chat::BaseComponent::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn copy_formatting_with_base_component(
        &self,
        arg0: impl Into<crate::bungee::api::chat::BaseComponent<'mc>>,
        arg1: std::option::Option<
            impl Into<crate::bungee::api::chat::ComponentBuilderFormatRetention<'mc>>,
        >,
        arg2: std::option::Option<bool>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lnet/md_5/bungee/api/chat/BaseComponent;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        if let Some(a) = arg1 {
            sig += "Lnet/md_5/bungee/api/chat/ComponentBuilder$FormatRetention;";
            let val_2 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(a.into().jni_object().clone())
            });
            args.push(val_2);
        }
        if let Some(a) = arg2 {
            sig += "Z";
            let val_3 = jni::objects::JValueGen::Bool(a.into());
            args.push(val_3);
        }
        sig += ")V";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "copyFormatting", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn retain(
        &self,
        arg0: impl Into<crate::bungee::api::chat::ComponentBuilderFormatRetention<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lnet/md_5/bungee/api/chat/ComponentBuilder$FormatRetention;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "retain",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn to_legacy_text_with_base_components(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: std::option::Option<Vec<impl Into<crate::bungee::api::chat::BaseComponent<'mc>>>>,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Ljava/lang/String;";
        let cls = jni.find_class("java/lang/String");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "toLegacyText", sig.as_str(), args);
        let res = jni.translate_error(res)?;
        Ok(jni
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }

    pub fn to_plain_text_with_base_components(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: std::option::Option<Vec<impl Into<crate::bungee::api::chat::BaseComponent<'mc>>>>,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Ljava/lang/String;";
        let cls = jni.find_class("java/lang/String");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "toPlainText", sig.as_str(), args);
        let res = jni.translate_error(res)?;
        Ok(jni
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }

    pub fn color_raw(
        &self,
    ) -> Result<crate::bungee::api::ChatColor<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lnet/md_5/bungee/api/ChatColor;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getColorRaw", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::bungee::api::ChatColor::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn font(&self) -> Result<String, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/String;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getFont", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }

    pub fn font_raw(&self) -> Result<String, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/String;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getFontRaw", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }

    pub fn is_bold(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isBold", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn is_bold_raw(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/Boolean;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isBoldRaw", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn is_italic(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isItalic", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn is_italic_raw(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/Boolean;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isItalicRaw", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn is_underlined(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isUnderlined", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn is_underlined_raw(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/Boolean;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isUnderlinedRaw", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn is_strikethrough(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isStrikethrough", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn is_strikethrough_raw(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/Boolean;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "isStrikethroughRaw",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn is_obfuscated(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isObfuscated", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn is_obfuscated_raw(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/Boolean;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isObfuscatedRaw", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn add_extra_with_base_component(
        &self,
        arg0: impl Into<crate::bungee::api::chat::BaseComponent<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lnet/md_5/bungee/api/chat/BaseComponent;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        sig += ")V";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "addExtra", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn has_formatting(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "hasFormatting", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn set_font(&self, arg0: impl Into<String>) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/String;)V");
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(arg0.into())?,
        ));
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setFont",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn set_bold(&self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/Boolean;)V");
        let val_1 = jni::objects::JValueGen::Object(self.jni_ref().new_object(
            "java/lang/Boolean",
            "(Z)V",
            vec![arg0.into()],
        )?);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setBold",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn set_italic(&self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/Boolean;)V");
        let val_1 = jni::objects::JValueGen::Object(self.jni_ref().new_object(
            "java/lang/Boolean",
            "(Z)V",
            vec![arg0.into()],
        )?);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setItalic",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn set_underlined(&self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/Boolean;)V");
        let val_1 = jni::objects::JValueGen::Object(self.jni_ref().new_object(
            "java/lang/Boolean",
            "(Z)V",
            vec![arg0.into()],
        )?);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setUnderlined",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn set_strikethrough(&self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/Boolean;)V");
        let val_1 = jni::objects::JValueGen::Object(self.jni_ref().new_object(
            "java/lang/Boolean",
            "(Z)V",
            vec![arg0.into()],
        )?);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setStrikethrough",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn set_obfuscated(&self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/Boolean;)V");
        let val_1 = jni::objects::JValueGen::Object(self.jni_ref().new_object(
            "java/lang/Boolean",
            "(Z)V",
            vec![arg0.into()],
        )?);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setObfuscated",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn set_insertion(&self, arg0: impl Into<String>) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/String;)V");
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(arg0.into())?,
        ));
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setInsertion",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn set_click_event(
        &self,
        arg0: impl Into<crate::bungee::api::chat::ClickEvent<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lnet/md_5/bungee/api/chat/ClickEvent;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setClickEvent",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn set_hover_event(
        &self,
        arg0: impl Into<crate::bungee::api::chat::HoverEvent<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lnet/md_5/bungee/api/chat/HoverEvent;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setHoverEvent",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn set_reset(&self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        let val_1 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setReset",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn insertion(&self) -> Result<String, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/String;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getInsertion", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }

    pub fn click_event(
        &self,
    ) -> Result<crate::bungee::api::chat::ClickEvent<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lnet/md_5/bungee/api/chat/ClickEvent;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getClickEvent", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::bungee::api::chat::ClickEvent::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn hover_event(
        &self,
    ) -> Result<crate::bungee::api::chat::HoverEvent<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lnet/md_5/bungee/api/chat/HoverEvent;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getHoverEvent", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::bungee::api::chat::HoverEvent::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn extra(
        &self,
    ) -> Result<Vec<crate::bungee::api::chat::BaseComponent<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/List;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getExtra", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let list = blackboxmc_java::util::JavaList::from_raw(&self.0, res.l()?)?;
        let iter = list.iterator()?;
        while iter.has_next()? {
            let obj = iter.next()?;
            new_vec.push(crate::bungee::api::chat::BaseComponent::from_raw(
                &self.0, obj,
            )?);
        }
        Ok(new_vec)
    }

    pub fn set_color(
        &self,
        arg0: impl Into<crate::bungee::api::ChatColor<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lnet/md_5/bungee/api/ChatColor;)V");
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

    pub fn color(&self) -> Result<crate::bungee::api::ChatColor<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lnet/md_5/bungee/api/ChatColor;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getColor", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::bungee::api::ChatColor::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn is_reset(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isReset", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn wait_with_long(
        &self,
        arg0: std::option::Option<i64>,
        arg1: std::option::Option<i32>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "J";
            let val_1 = jni::objects::JValueGen::Long(a.into());
            args.push(val_1);
        }
        if let Some(a) = arg1 {
            sig += "I";
            let val_2 = jni::objects::JValueGen::Int(a.into());
            args.push(val_2);
        }
        sig += ")V";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "wait", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn class(&self) -> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/Class;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getClass", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(unsafe { jni::objects::JClass::from_raw(res.as_jni().l) })
    }

    pub fn notify(&self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notify", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn notify_all(&self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notifyAll", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn instance_of<A>(&self, other: A) -> bool
    where
        A: blackboxmc_general::JNIProvidesClassName,
    {
        let cls = &self.jni_ref().find_class(other.class_name()).unwrap();
        self.jni_ref()
            .is_instance_of(&self.jni_object(), cls)
            .unwrap()
    }
}

impl<'mc> std::string::ToString for TranslatableComponent<'mc> {
    fn to_string(&self) -> String {
        match &self.internal_to_string() {
            Ok(a) => a.clone(),
            Err(err) => format!("Error calling TranslatableComponent.toString: {}", err),
        }
    }
}

impl<'mc> Into<crate::bungee::api::chat::BaseComponent<'mc>> for TranslatableComponent<'mc> {
    fn into(self) -> crate::bungee::api::chat::BaseComponent<'mc> {
        crate::bungee::api::chat::BaseComponent::from_raw(&self.jni_ref(), self.1).expect(
            "Error converting TranslatableComponent into crate::bungee::api::chat::BaseComponent",
        )
    }
}

pub struct TranslatableComponentClass;
impl blackboxmc_general::JNIProvidesClassName for TranslatableComponentClass {
    fn class_name(&self) -> &str {
        "net/md_5/bungee/api/chat/TranslatableComponent"
    }
}

///
/// This is a representation of an abstract class.
pub struct ComponentBuilderJoiner<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for ComponentBuilderJoiner<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}

impl<'mc> JNIInstantiatable<'mc> for ComponentBuilderJoiner<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate ComponentBuilderJoiner from null object."
            )
            .into());
        }
        let (valid, name) =
            env.validate_name(&obj, "net/md_5/bungee/api/chat/ComponentBuilder$Joiner")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a ComponentBuilderJoiner object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> ComponentBuilderJoiner<'mc> {
    pub fn join(
        &self,
        arg0: impl Into<crate::bungee::api::chat::ComponentBuilder<'mc>>,
        arg1: impl Into<crate::bungee::api::chat::ComponentBuilderFormatRetention<'mc>>,
    ) -> Result<crate::bungee::api::chat::ComponentBuilder<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Lnet/md_5/bungee/api/chat/ComponentBuilder;Lnet/md_5/bungee/api/chat/ComponentBuilder$FormatRetention;)Lnet/md_5/bungee/api/chat/ComponentBuilder;");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let val_2 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg1.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "join",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::bungee::api::chat::ComponentBuilder::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn instance_of<A>(&self, other: A) -> bool
    where
        A: blackboxmc_general::JNIProvidesClassName,
    {
        let cls = &self.jni_ref().find_class(other.class_name()).unwrap();
        self.jni_ref()
            .is_instance_of(&self.jni_object(), cls)
            .unwrap()
    }
}

pub struct ComponentBuilderJoinerClass;
impl blackboxmc_general::JNIProvidesClassName for ComponentBuilderJoinerClass {
    fn class_name(&self) -> &str {
        "net/md_5/bungee/api/chat/ComponentBuilder$Joiner"
    }
}

pub struct ItemTag<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for ItemTag<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}

impl<'mc> JNIInstantiatable<'mc> for ItemTag<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate ItemTag from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "net/md_5/bungee/api/chat/ItemTag")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a ItemTag object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> ItemTag<'mc> {
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

    pub fn of_nbt(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<String>,
    ) -> Result<crate::bungee::api::chat::ItemTag<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/String;)Lnet/md_5/bungee/api/chat/ItemTag;");
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            jni.new_string(arg0.into())?,
        ));
        let cls = jni.find_class("net/md_5/bungee/api/chat/ItemTag");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(
            cls,
            "ofNbt",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        crate::bungee::api::chat::ItemTag::from_raw(&jni, obj)
    }

    pub fn nbt(&self) -> Result<String, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/String;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getNbt", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }

    pub fn wait_with_long(
        &self,
        arg0: std::option::Option<i64>,
        arg1: std::option::Option<i32>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "J";
            let val_1 = jni::objects::JValueGen::Long(a.into());
            args.push(val_1);
        }
        if let Some(a) = arg1 {
            sig += "I";
            let val_2 = jni::objects::JValueGen::Int(a.into());
            args.push(val_2);
        }
        sig += ")V";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "wait", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn class(&self) -> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/Class;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getClass", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(unsafe { jni::objects::JClass::from_raw(res.as_jni().l) })
    }

    pub fn notify(&self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notify", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn notify_all(&self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notifyAll", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn instance_of<A>(&self, other: A) -> bool
    where
        A: blackboxmc_general::JNIProvidesClassName,
    {
        let cls = &self.jni_ref().find_class(other.class_name()).unwrap();
        self.jni_ref()
            .is_instance_of(&self.jni_object(), cls)
            .unwrap()
    }
}

impl<'mc> std::string::ToString for ItemTag<'mc> {
    fn to_string(&self) -> String {
        match &self.internal_to_string() {
            Ok(a) => a.clone(),
            Err(err) => format!("Error calling ItemTag.toString: {}", err),
        }
    }
}

pub struct ItemTagClass;
impl blackboxmc_general::JNIProvidesClassName for ItemTagClass {
    fn class_name(&self) -> &str {
        "net/md_5/bungee/api/chat/ItemTag"
    }
}

pub struct KeybindComponent<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for KeybindComponent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}

impl<'mc> JNIInstantiatable<'mc> for KeybindComponent<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate KeybindComponent from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "net/md_5/bungee/api/chat/KeybindComponent")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a KeybindComponent object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> KeybindComponent<'mc> {
    pub fn new_with_string(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: std::option::Option<impl Into<String>>,
    ) -> Result<crate::bungee::api::chat::KeybindComponent<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "Ljava/lang/String;";
            let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
                jni.new_string(a.into())?,
            ));
            args.push(val_1);
        }
        sig += ")V";
        let cls = jni.find_class("net/md_5/bungee/api/chat/KeybindComponent");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(cls, sig.as_str(), args);
        let res = jni.translate_error_no_gen(res)?;
        crate::bungee::api::chat::KeybindComponent::from_raw(&jni, res)
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

    pub fn duplicate(
        &self,
    ) -> Result<crate::bungee::api::chat::BaseComponent<'mc>, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Lnet/md_5/bungee/api/chat/BaseComponent;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "duplicate", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        crate::bungee::api::chat::BaseComponent::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn keybind(&self) -> Result<String, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/String;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getKeybind", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }

    pub fn set_keybind(&self, arg0: impl Into<String>) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/String;)V");
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(arg0.into())?,
        ));
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setKeybind",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn duplicate_without_formatting(
        &self,
    ) -> Result<crate::bungee::api::chat::BaseComponent<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lnet/md_5/bungee/api/chat/BaseComponent;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "duplicateWithoutFormatting",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::bungee::api::chat::BaseComponent::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn copy_formatting_with_base_component(
        &self,
        arg0: impl Into<crate::bungee::api::chat::BaseComponent<'mc>>,
        arg1: std::option::Option<
            impl Into<crate::bungee::api::chat::ComponentBuilderFormatRetention<'mc>>,
        >,
        arg2: std::option::Option<bool>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lnet/md_5/bungee/api/chat/BaseComponent;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        if let Some(a) = arg1 {
            sig += "Lnet/md_5/bungee/api/chat/ComponentBuilder$FormatRetention;";
            let val_2 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(a.into().jni_object().clone())
            });
            args.push(val_2);
        }
        if let Some(a) = arg2 {
            sig += "Z";
            let val_3 = jni::objects::JValueGen::Bool(a.into());
            args.push(val_3);
        }
        sig += ")V";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "copyFormatting", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn retain(
        &self,
        arg0: impl Into<crate::bungee::api::chat::ComponentBuilderFormatRetention<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lnet/md_5/bungee/api/chat/ComponentBuilder$FormatRetention;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "retain",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn to_legacy_text_with_base_components(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: std::option::Option<Vec<impl Into<crate::bungee::api::chat::BaseComponent<'mc>>>>,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Ljava/lang/String;";
        let cls = jni.find_class("java/lang/String");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "toLegacyText", sig.as_str(), args);
        let res = jni.translate_error(res)?;
        Ok(jni
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }

    pub fn to_plain_text_with_base_components(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: std::option::Option<Vec<impl Into<crate::bungee::api::chat::BaseComponent<'mc>>>>,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Ljava/lang/String;";
        let cls = jni.find_class("java/lang/String");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "toPlainText", sig.as_str(), args);
        let res = jni.translate_error(res)?;
        Ok(jni
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }

    pub fn color_raw(
        &self,
    ) -> Result<crate::bungee::api::ChatColor<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lnet/md_5/bungee/api/ChatColor;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getColorRaw", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::bungee::api::ChatColor::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn font(&self) -> Result<String, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/String;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getFont", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }

    pub fn font_raw(&self) -> Result<String, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/String;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getFontRaw", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }

    pub fn is_bold(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isBold", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn is_bold_raw(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/Boolean;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isBoldRaw", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn is_italic(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isItalic", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn is_italic_raw(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/Boolean;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isItalicRaw", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn is_underlined(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isUnderlined", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn is_underlined_raw(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/Boolean;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isUnderlinedRaw", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn is_strikethrough(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isStrikethrough", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn is_strikethrough_raw(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/Boolean;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "isStrikethroughRaw",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn is_obfuscated(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isObfuscated", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn is_obfuscated_raw(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/Boolean;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isObfuscatedRaw", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn add_extra_with_base_component(
        &self,
        arg0: impl Into<crate::bungee::api::chat::BaseComponent<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lnet/md_5/bungee/api/chat/BaseComponent;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        sig += ")V";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "addExtra", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn has_formatting(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "hasFormatting", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn set_font(&self, arg0: impl Into<String>) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/String;)V");
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(arg0.into())?,
        ));
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setFont",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn set_bold(&self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/Boolean;)V");
        let val_1 = jni::objects::JValueGen::Object(self.jni_ref().new_object(
            "java/lang/Boolean",
            "(Z)V",
            vec![arg0.into()],
        )?);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setBold",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn set_italic(&self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/Boolean;)V");
        let val_1 = jni::objects::JValueGen::Object(self.jni_ref().new_object(
            "java/lang/Boolean",
            "(Z)V",
            vec![arg0.into()],
        )?);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setItalic",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn set_underlined(&self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/Boolean;)V");
        let val_1 = jni::objects::JValueGen::Object(self.jni_ref().new_object(
            "java/lang/Boolean",
            "(Z)V",
            vec![arg0.into()],
        )?);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setUnderlined",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn set_strikethrough(&self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/Boolean;)V");
        let val_1 = jni::objects::JValueGen::Object(self.jni_ref().new_object(
            "java/lang/Boolean",
            "(Z)V",
            vec![arg0.into()],
        )?);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setStrikethrough",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn set_obfuscated(&self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/Boolean;)V");
        let val_1 = jni::objects::JValueGen::Object(self.jni_ref().new_object(
            "java/lang/Boolean",
            "(Z)V",
            vec![arg0.into()],
        )?);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setObfuscated",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn set_insertion(&self, arg0: impl Into<String>) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/String;)V");
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(arg0.into())?,
        ));
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setInsertion",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn set_click_event(
        &self,
        arg0: impl Into<crate::bungee::api::chat::ClickEvent<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lnet/md_5/bungee/api/chat/ClickEvent;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setClickEvent",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn set_hover_event(
        &self,
        arg0: impl Into<crate::bungee::api::chat::HoverEvent<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lnet/md_5/bungee/api/chat/HoverEvent;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setHoverEvent",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn set_reset(&self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        let val_1 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setReset",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn insertion(&self) -> Result<String, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/String;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getInsertion", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }

    pub fn click_event(
        &self,
    ) -> Result<crate::bungee::api::chat::ClickEvent<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lnet/md_5/bungee/api/chat/ClickEvent;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getClickEvent", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::bungee::api::chat::ClickEvent::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn hover_event(
        &self,
    ) -> Result<crate::bungee::api::chat::HoverEvent<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lnet/md_5/bungee/api/chat/HoverEvent;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getHoverEvent", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::bungee::api::chat::HoverEvent::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn extra(
        &self,
    ) -> Result<Vec<crate::bungee::api::chat::BaseComponent<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/List;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getExtra", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let list = blackboxmc_java::util::JavaList::from_raw(&self.0, res.l()?)?;
        let iter = list.iterator()?;
        while iter.has_next()? {
            let obj = iter.next()?;
            new_vec.push(crate::bungee::api::chat::BaseComponent::from_raw(
                &self.0, obj,
            )?);
        }
        Ok(new_vec)
    }

    pub fn set_color(
        &self,
        arg0: impl Into<crate::bungee::api::ChatColor<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lnet/md_5/bungee/api/ChatColor;)V");
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

    pub fn color(&self) -> Result<crate::bungee::api::ChatColor<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lnet/md_5/bungee/api/ChatColor;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getColor", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::bungee::api::ChatColor::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn is_reset(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isReset", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn wait_with_long(
        &self,
        arg0: std::option::Option<i64>,
        arg1: std::option::Option<i32>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "J";
            let val_1 = jni::objects::JValueGen::Long(a.into());
            args.push(val_1);
        }
        if let Some(a) = arg1 {
            sig += "I";
            let val_2 = jni::objects::JValueGen::Int(a.into());
            args.push(val_2);
        }
        sig += ")V";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "wait", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn class(&self) -> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/Class;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getClass", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(unsafe { jni::objects::JClass::from_raw(res.as_jni().l) })
    }

    pub fn notify(&self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notify", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn notify_all(&self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notifyAll", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn instance_of<A>(&self, other: A) -> bool
    where
        A: blackboxmc_general::JNIProvidesClassName,
    {
        let cls = &self.jni_ref().find_class(other.class_name()).unwrap();
        self.jni_ref()
            .is_instance_of(&self.jni_object(), cls)
            .unwrap()
    }
}

impl<'mc> std::string::ToString for KeybindComponent<'mc> {
    fn to_string(&self) -> String {
        match &self.internal_to_string() {
            Ok(a) => a.clone(),
            Err(err) => format!("Error calling KeybindComponent.toString: {}", err),
        }
    }
}

impl<'mc> Into<crate::bungee::api::chat::BaseComponent<'mc>> for KeybindComponent<'mc> {
    fn into(self) -> crate::bungee::api::chat::BaseComponent<'mc> {
        crate::bungee::api::chat::BaseComponent::from_raw(&self.jni_ref(), self.1).expect(
            "Error converting KeybindComponent into crate::bungee::api::chat::BaseComponent",
        )
    }
}

pub struct KeybindComponentClass;
impl blackboxmc_general::JNIProvidesClassName for KeybindComponentClass {
    fn class_name(&self) -> &str {
        "net/md_5/bungee/api/chat/KeybindComponent"
    }
}

pub struct ComponentBuilder<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
#[derive(PartialEq, Eq)]
pub enum ComponentBuilderFormatRetentionEnum {
    None,
    Formatting,
    Events,
    All,
}
impl std::fmt::Display for ComponentBuilderFormatRetentionEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ComponentBuilderFormatRetentionEnum::None => f.write_str("NONE"),
            ComponentBuilderFormatRetentionEnum::Formatting => f.write_str("FORMATTING"),
            ComponentBuilderFormatRetentionEnum::Events => f.write_str("EVENTS"),
            ComponentBuilderFormatRetentionEnum::All => f.write_str("ALL"),
        }
    }
}
impl<'mc> std::fmt::Display for ComponentBuilderFormatRetention<'mc> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.2.fmt(f)
    }
}
pub struct ComponentBuilderFormatRetention<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
    pub ComponentBuilderFormatRetentionEnum,
);
impl<'mc> std::ops::Deref for ComponentBuilderFormatRetention<'mc> {
    type Target = ComponentBuilderFormatRetentionEnum;
    fn deref(&self) -> &Self::Target {
        return &self.2;
    }
}

impl<'mc> JNIRaw<'mc> for ComponentBuilderFormatRetention<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}

impl<'mc> JNIInstantiatableEnum<'mc> for ComponentBuilderFormatRetention<'mc> {
    type Enum = ComponentBuilderFormatRetentionEnum;

    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,

        e: Self::Enum,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate ComponentBuilderFormatRetention from null object."
            )
            .into());
        }
        let (valid, name) = env.validate_name(
            &obj,
            "net/md_5/bungee/api/chat/ComponentBuilder$FormatRetention",
        )?;
        if !valid {
            Err(eyre::eyre!(
                    "Invalid argument passed. Expected a ComponentBuilderFormatRetention object, got {}",
                    name
                )
                .into())
        } else {
            Ok(Self(env.clone(), obj, e))
        }
    }
}

impl<'mc> ComponentBuilderFormatRetention<'mc> {
    pub const NONE: ComponentBuilderFormatRetentionEnum = ComponentBuilderFormatRetentionEnum::None;
    pub const FORMATTING: ComponentBuilderFormatRetentionEnum =
        ComponentBuilderFormatRetentionEnum::Formatting;
    pub const EVENTS: ComponentBuilderFormatRetentionEnum =
        ComponentBuilderFormatRetentionEnum::Events;
    pub const ALL: ComponentBuilderFormatRetentionEnum = ComponentBuilderFormatRetentionEnum::All;
    pub fn from_string(str: String) -> std::option::Option<ComponentBuilderFormatRetentionEnum> {
        match str.as_str() {
            "NONE" => Some(ComponentBuilderFormatRetentionEnum::None),
            "FORMATTING" => Some(ComponentBuilderFormatRetentionEnum::Formatting),
            "EVENTS" => Some(ComponentBuilderFormatRetentionEnum::Events),
            "ALL" => Some(ComponentBuilderFormatRetentionEnum::All),
            _ => None,
        }
    }

    pub fn value_of(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<String>,
    ) -> Result<ComponentBuilderFormatRetention<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(jni.new_string(arg0.into())?);
        let cls = jni.find_class("net/md_5/bungee/api/chat/ComponentBuilder$FormatRetention");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(
            cls,
            "valueOf",
            "(Ljava/lang/String;)Lnet/md_5/bungee/api/chat/ComponentBuilder$FormatRetention;",
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
        ComponentBuilderFormatRetention::from_raw(
            &jni,
            raw_obj,
            ComponentBuilderFormatRetention::from_string(variant_str)
                .ok_or(eyre::eyre!("String gaven for variant was invalid"))?,
        )
    }

    pub fn instance_of<A>(&self, other: A) -> bool
    where
        A: blackboxmc_general::JNIProvidesClassName,
    {
        let cls = &self.jni_ref().find_class(other.class_name()).unwrap();
        self.jni_ref()
            .is_instance_of(&self.jni_object(), cls)
            .unwrap()
    }
}

pub struct ComponentBuilderFormatRetentionClass;
impl blackboxmc_general::JNIProvidesClassName for ComponentBuilderFormatRetentionClass {
    fn class_name(&self) -> &str {
        "net/md_5/bungee/api/chat/ComponentBuilder$FormatRetention"
    }
}

impl<'mc> JNIRaw<'mc> for ComponentBuilder<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}

impl<'mc> JNIInstantiatable<'mc> for ComponentBuilder<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate ComponentBuilder from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "net/md_5/bungee/api/chat/ComponentBuilder")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a ComponentBuilder object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> ComponentBuilder<'mc> {
    pub fn new_with_component_builder(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: std::option::Option<impl Into<crate::bungee::api::chat::ComponentBuilder<'mc>>>,
    ) -> Result<crate::bungee::api::chat::ComponentBuilder<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "Lnet/md_5/bungee/api/chat/ComponentBuilder;";
            let val_1 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(a.into().jni_object().clone())
            });
            args.push(val_1);
        }
        sig += ")V";
        let cls = jni.find_class("net/md_5/bungee/api/chat/ComponentBuilder");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(cls, sig.as_str(), args);
        let res = jni.translate_error_no_gen(res)?;
        crate::bungee::api::chat::ComponentBuilder::from_raw(&jni, res)
    }
    pub fn new_with_base_component(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: std::option::Option<impl Into<crate::bungee::api::chat::BaseComponent<'mc>>>,
    ) -> Result<crate::bungee::api::chat::ComponentBuilder<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "Lnet/md_5/bungee/api/chat/BaseComponent;";
            let val_1 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(a.into().jni_object().clone())
            });
            args.push(val_1);
        }
        sig += ")V";
        let cls = jni.find_class("net/md_5/bungee/api/chat/ComponentBuilder");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(cls, sig.as_str(), args);
        let res = jni.translate_error_no_gen(res)?;
        crate::bungee::api::chat::ComponentBuilder::from_raw(&jni, res)
    }

    pub fn font(
        &self,
        arg0: impl Into<String>,
    ) -> Result<crate::bungee::api::chat::ComponentBuilder<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/String;)Lnet/md_5/bungee/api/chat/ComponentBuilder;");
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(arg0.into())?,
        ));
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "font",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::bungee::api::chat::ComponentBuilder::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn bold(
        &self,
        arg0: bool,
    ) -> Result<crate::bungee::api::chat::ComponentBuilder<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Z)Lnet/md_5/bungee/api/chat/ComponentBuilder;");
        let val_1 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "bold",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::bungee::api::chat::ComponentBuilder::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn italic(
        &self,
        arg0: bool,
    ) -> Result<crate::bungee::api::chat::ComponentBuilder<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Z)Lnet/md_5/bungee/api/chat/ComponentBuilder;");
        let val_1 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "italic",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::bungee::api::chat::ComponentBuilder::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn underlined(
        &self,
        arg0: bool,
    ) -> Result<crate::bungee::api::chat::ComponentBuilder<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Z)Lnet/md_5/bungee/api/chat/ComponentBuilder;");
        let val_1 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "underlined",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::bungee::api::chat::ComponentBuilder::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn strikethrough(
        &self,
        arg0: bool,
    ) -> Result<crate::bungee::api::chat::ComponentBuilder<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Z)Lnet/md_5/bungee/api/chat/ComponentBuilder;");
        let val_1 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "strikethrough",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::bungee::api::chat::ComponentBuilder::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn obfuscated(
        &self,
        arg0: bool,
    ) -> Result<crate::bungee::api::chat::ComponentBuilder<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Z)Lnet/md_5/bungee/api/chat/ComponentBuilder;");
        let val_1 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "obfuscated",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::bungee::api::chat::ComponentBuilder::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn insertion(
        &self,
        arg0: impl Into<String>,
    ) -> Result<crate::bungee::api::chat::ComponentBuilder<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/String;)Lnet/md_5/bungee/api/chat/ComponentBuilder;");
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(arg0.into())?,
        ));
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "insertion",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::bungee::api::chat::ComponentBuilder::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn retain(
        &self,
        arg0: impl Into<crate::bungee::api::chat::ComponentBuilderFormatRetention<'mc>>,
    ) -> Result<crate::bungee::api::chat::ComponentBuilder<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Lnet/md_5/bungee/api/chat/ComponentBuilder$FormatRetention;)Lnet/md_5/bungee/api/chat/ComponentBuilder;");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "retain",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::bungee::api::chat::ComponentBuilder::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn append_with_component_builderjoiner(
        &self,
        arg0: impl Into<crate::bungee::api::chat::ComponentBuilderJoiner<'mc>>,
        arg1: std::option::Option<
            impl Into<crate::bungee::api::chat::ComponentBuilderFormatRetention<'mc>>,
        >,
    ) -> Result<crate::bungee::api::chat::ComponentBuilder<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lnet/md_5/bungee/api/chat/ComponentBuilder$Joiner;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        if let Some(a) = arg1 {
            sig += "Lnet/md_5/bungee/api/chat/ComponentBuilder$FormatRetention;";
            let val_2 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(a.into().jni_object().clone())
            });
            args.push(val_2);
        }
        sig += ")Lnet/md_5/bungee/api/chat/ComponentBuilder;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "append", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        crate::bungee::api::chat::ComponentBuilder::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn append_with_string(
        &self,
        arg0: impl Into<String>,
        arg1: std::option::Option<
            impl Into<crate::bungee::api::chat::ComponentBuilderFormatRetention<'mc>>,
        >,
    ) -> Result<crate::bungee::api::chat::ComponentBuilder<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Ljava/lang/String;";
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(arg0.into())?,
        ));
        args.push(val_1);
        if let Some(a) = arg1 {
            sig += "Lnet/md_5/bungee/api/chat/ComponentBuilder$FormatRetention;";
            let val_2 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(a.into().jni_object().clone())
            });
            args.push(val_2);
        }
        sig += ")Lnet/md_5/bungee/api/chat/ComponentBuilder;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "append", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        crate::bungee::api::chat::ComponentBuilder::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn append_with_base_components(
        &self,
        arg0: Vec<impl Into<crate::bungee::api::chat::BaseComponent<'mc>>>,
        arg1: std::option::Option<
            impl Into<crate::bungee::api::chat::ComponentBuilderFormatRetention<'mc>>,
        >,
    ) -> Result<crate::bungee::api::chat::ComponentBuilder<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg1 {
            sig += "Lnet/md_5/bungee/api/chat/ComponentBuilder$FormatRetention;";
            let val_2 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(a.into().jni_object().clone())
            });
            args.push(val_2);
        }
        sig += ")Lnet/md_5/bungee/api/chat/ComponentBuilder;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "append", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        crate::bungee::api::chat::ComponentBuilder::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn append_with_base_component(
        &self,
        arg0: impl Into<crate::bungee::api::chat::BaseComponent<'mc>>,
        arg1: std::option::Option<
            impl Into<crate::bungee::api::chat::ComponentBuilderFormatRetention<'mc>>,
        >,
    ) -> Result<crate::bungee::api::chat::ComponentBuilder<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lnet/md_5/bungee/api/chat/BaseComponent;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        if let Some(a) = arg1 {
            sig += "Lnet/md_5/bungee/api/chat/ComponentBuilder$FormatRetention;";
            let val_2 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(a.into().jni_object().clone())
            });
            args.push(val_2);
        }
        sig += ")Lnet/md_5/bungee/api/chat/ComponentBuilder;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "append", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        crate::bungee::api::chat::ComponentBuilder::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn event_with_hover_event(
        &self,
        arg0: impl Into<crate::bungee::api::chat::HoverEvent<'mc>>,
    ) -> Result<crate::bungee::api::chat::ComponentBuilder<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lnet/md_5/bungee/api/chat/HoverEvent;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        sig += ")Lnet/md_5/bungee/api/chat/ComponentBuilder;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "event", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        crate::bungee::api::chat::ComponentBuilder::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn reset(
        &self,
    ) -> Result<crate::bungee::api::chat::ComponentBuilder<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lnet/md_5/bungee/api/chat/ComponentBuilder;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "reset", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::bungee::api::chat::ComponentBuilder::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn color(
        &self,
        arg0: impl Into<crate::bungee::api::ChatColor<'mc>>,
    ) -> Result<crate::bungee::api::chat::ComponentBuilder<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from(
            "(Lnet/md_5/bungee/api/ChatColor;)Lnet/md_5/bungee/api/chat/ComponentBuilder;",
        );
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "color",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::bungee::api::chat::ComponentBuilder::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn current_component(
        &self,
    ) -> Result<crate::bungee::api::chat::BaseComponent<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lnet/md_5/bungee/api/chat/BaseComponent;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getCurrentComponent",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::bungee::api::chat::BaseComponent::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn reset_cursor(
        &self,
    ) -> Result<crate::bungee::api::chat::ComponentBuilder<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lnet/md_5/bungee/api/chat/ComponentBuilder;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "resetCursor", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::bungee::api::chat::ComponentBuilder::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn set_cursor(
        &self,
        arg0: i32,
    ) -> Result<crate::bungee::api::chat::ComponentBuilder<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(I)Lnet/md_5/bungee/api/chat/ComponentBuilder;");
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setCursor",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::bungee::api::chat::ComponentBuilder::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn cursor(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getCursor", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }

    pub fn parts(
        &self,
    ) -> Result<Vec<crate::bungee::api::chat::BaseComponent<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/List;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getParts", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let list = blackboxmc_java::util::JavaList::from_raw(&self.0, res.l()?)?;
        let iter = list.iterator()?;
        while iter.has_next()? {
            let obj = iter.next()?;
            new_vec.push(crate::bungee::api::chat::BaseComponent::from_raw(
                &self.0, obj,
            )?);
        }
        Ok(new_vec)
    }

    pub fn append_legacy(
        &self,
        arg0: impl Into<String>,
    ) -> Result<crate::bungee::api::chat::ComponentBuilder<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/String;)Lnet/md_5/bungee/api/chat/ComponentBuilder;");
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(arg0.into())?,
        ));
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "appendLegacy",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::bungee::api::chat::ComponentBuilder::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn remove_component(&self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(I)V");
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeComponent",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn get_component(
        &self,
        arg0: i32,
    ) -> Result<crate::bungee::api::chat::BaseComponent<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(I)Lnet/md_5/bungee/api/chat/BaseComponent;");
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getComponent",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::bungee::api::chat::BaseComponent::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn wait_with_long(
        &self,
        arg0: std::option::Option<i64>,
        arg1: std::option::Option<i32>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "J";
            let val_1 = jni::objects::JValueGen::Long(a.into());
            args.push(val_1);
        }
        if let Some(a) = arg1 {
            sig += "I";
            let val_2 = jni::objects::JValueGen::Int(a.into());
            args.push(val_2);
        }
        sig += ")V";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "wait", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
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

    pub fn class(&self) -> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/Class;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getClass", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(unsafe { jni::objects::JClass::from_raw(res.as_jni().l) })
    }

    pub fn notify(&self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notify", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn notify_all(&self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notifyAll", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn instance_of<A>(&self, other: A) -> bool
    where
        A: blackboxmc_general::JNIProvidesClassName,
    {
        let cls = &self.jni_ref().find_class(other.class_name()).unwrap();
        self.jni_ref()
            .is_instance_of(&self.jni_object(), cls)
            .unwrap()
    }
}

impl<'mc> std::string::ToString for ComponentBuilder<'mc> {
    fn to_string(&self) -> String {
        match &self.internal_to_string() {
            Ok(a) => a.clone(),
            Err(err) => format!("Error calling ComponentBuilder.toString: {}", err),
        }
    }
}

pub struct ComponentBuilderClass;
impl blackboxmc_general::JNIProvidesClassName for ComponentBuilderClass {
    fn class_name(&self) -> &str {
        "net/md_5/bungee/api/chat/ComponentBuilder"
    }
}

pub mod hover;
