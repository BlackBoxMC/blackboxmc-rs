#![allow(deprecated)]
use blackboxmc_general::JNIInstantiatable;
use blackboxmc_general::JNIRaw;
use color_eyre::eyre::Result;

#[repr(C)]
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
    pub fn new_with_string(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: std::option::Option<impl Into<String>>,
    ) -> Result<crate::bungee::api::chat::TextComponent<'mc>, Box<dyn std::error::Error>> {
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

    pub fn from_legacy_text_with_string(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<String>,
        arg1: std::option::Option<impl Into<crate::bungee::api::ChatColor<'mc>>>,
    ) -> Result<Vec<crate::bungee::api::chat::BaseComponent<'mc>>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Ljava/lang/String;";
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            jni.new_string(arg0.into())?,
        ));
        args.push(val_1);
        if let Some(a) = arg1 {
            sig += "Lnet/md_5/bungee/api/ChatColor;";
            let val_2 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(a.into().jni_object().clone())
            });
            args.push(val_2);
        }
        sig += ")[Lnet/md_5/bungee/api/chat/BaseComponent;";
        let cls = jni.find_class("net/md_5/bungee/api/chat/BaseComponent");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "fromLegacyText", sig.as_str(), args);
        let res = jni.translate_error(res)?;
        let arr = Into::<jni::objects::JObjectArray>::into(res.l()?);
        let len = jni.get_array_length(&arr)?;
        let mut vec = Vec::new();
        for i in 0..len {
            let res = jni.get_object_array_element(&arr, i)?;
            vec.push({ crate::bungee::api::chat::BaseComponent::from_raw(&jni, res)? });
        }
        Ok(vec)
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
    ) -> Result<crate::bungee::api::chat::TextComponent<'mc>, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Lnet/md_5/bungee/api/chat/TextComponent;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "duplicate", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        crate::bungee::api::chat::TextComponent::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    // SUPER CLASS: BaseComponent
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
        let temp_clone = crate::bungee::api::chat::BaseComponent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::bungee::api::chat::BaseComponent = temp_clone.into();
        real.retain(arg0)
    }
    pub fn to_legacy_text_with_base_components(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: std::option::Option<Vec<crate::bungee::api::chat::BaseComponent<'mc>>>,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "[Lnet/md_5/bungee/api/chat/BaseComponent;";
            let arr = jni.new_object_array(
                a.len() as i32,
                "net/md_5/bungee/api/chat/BaseComponent",
                jni::objects::JObject::null(),
            );
            let arr = jni.translate_error_no_gen(arr)?;
            for i in 0..a.len() {
                let val_1 = jni::objects::JValueGen::Object(unsafe {
                    jni::objects::JObject::from_raw(a.get(i).unwrap().jni_object().clone())
                });
                jni.set_object_array_element(&arr, i as i32, val_1.l()?)?;
            }
            let val_1 = jni::objects::JValueGen::Object(arr);
            args.push(val_1.l()?.into());
        }
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
        arg0: std::option::Option<Vec<crate::bungee::api::chat::BaseComponent<'mc>>>,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "[Lnet/md_5/bungee/api/chat/BaseComponent;";
            let arr = jni.new_object_array(
                a.len() as i32,
                "net/md_5/bungee/api/chat/BaseComponent",
                jni::objects::JObject::null(),
            );
            let arr = jni.translate_error_no_gen(arr)?;
            for i in 0..a.len() {
                let val_1 = jni::objects::JValueGen::Object(unsafe {
                    jni::objects::JObject::from_raw(a.get(i).unwrap().jni_object().clone())
                });
                jni.set_object_array_element(&arr, i as i32, val_1.l()?)?;
            }
            let val_1 = jni::objects::JValueGen::Object(arr);
            args.push(val_1.l()?.into());
        }
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
        let temp_clone = crate::bungee::api::chat::BaseComponent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::bungee::api::chat::BaseComponent = temp_clone.into();
        real.color_raw()
    }
    pub fn insertion(&self) -> Result<String, Box<dyn std::error::Error>> {
        let temp_clone = crate::bungee::api::chat::BaseComponent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::bungee::api::chat::BaseComponent = temp_clone.into();
        real.insertion()
    }
    pub fn click_event(
        &self,
    ) -> Result<crate::bungee::api::chat::ClickEvent<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::bungee::api::chat::BaseComponent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::bungee::api::chat::BaseComponent = temp_clone.into();
        real.click_event()
    }
    pub fn hover_event(
        &self,
    ) -> Result<crate::bungee::api::chat::HoverEvent<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::bungee::api::chat::BaseComponent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::bungee::api::chat::BaseComponent = temp_clone.into();
        real.hover_event()
    }
    pub fn is_obfuscated(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::bungee::api::chat::BaseComponent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::bungee::api::chat::BaseComponent = temp_clone.into();
        real.is_obfuscated()
    }
    pub fn is_obfuscated_raw(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::bungee::api::chat::BaseComponent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::bungee::api::chat::BaseComponent = temp_clone.into();
        real.is_obfuscated_raw()
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
        let temp_clone = crate::bungee::api::chat::BaseComponent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::bungee::api::chat::BaseComponent = temp_clone.into();
        real.has_formatting()
    }
    pub fn set_font(&self, arg0: impl Into<String>) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::bungee::api::chat::BaseComponent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::bungee::api::chat::BaseComponent = temp_clone.into();
        real.set_font(arg0)
    }
    pub fn set_bold(&self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::bungee::api::chat::BaseComponent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::bungee::api::chat::BaseComponent = temp_clone.into();
        real.set_bold(arg0)
    }
    pub fn set_italic(&self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::bungee::api::chat::BaseComponent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::bungee::api::chat::BaseComponent = temp_clone.into();
        real.set_italic(arg0)
    }
    pub fn set_underlined(&self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::bungee::api::chat::BaseComponent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::bungee::api::chat::BaseComponent = temp_clone.into();
        real.set_underlined(arg0)
    }
    pub fn set_strikethrough(&self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::bungee::api::chat::BaseComponent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::bungee::api::chat::BaseComponent = temp_clone.into();
        real.set_strikethrough(arg0)
    }
    pub fn set_obfuscated(&self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::bungee::api::chat::BaseComponent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::bungee::api::chat::BaseComponent = temp_clone.into();
        real.set_obfuscated(arg0)
    }
    pub fn set_insertion(&self, arg0: impl Into<String>) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::bungee::api::chat::BaseComponent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::bungee::api::chat::BaseComponent = temp_clone.into();
        real.set_insertion(arg0)
    }
    pub fn set_click_event(
        &self,
        arg0: impl Into<crate::bungee::api::chat::ClickEvent<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::bungee::api::chat::BaseComponent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::bungee::api::chat::BaseComponent = temp_clone.into();
        real.set_click_event(arg0)
    }
    pub fn set_hover_event(
        &self,
        arg0: impl Into<crate::bungee::api::chat::HoverEvent<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::bungee::api::chat::BaseComponent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::bungee::api::chat::BaseComponent = temp_clone.into();
        real.set_hover_event(arg0)
    }
    pub fn set_reset(&self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::bungee::api::chat::BaseComponent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::bungee::api::chat::BaseComponent = temp_clone.into();
        real.set_reset(arg0)
    }
    pub fn font(&self) -> Result<String, Box<dyn std::error::Error>> {
        let temp_clone = crate::bungee::api::chat::BaseComponent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::bungee::api::chat::BaseComponent = temp_clone.into();
        real.font()
    }
    pub fn font_raw(&self) -> Result<String, Box<dyn std::error::Error>> {
        let temp_clone = crate::bungee::api::chat::BaseComponent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::bungee::api::chat::BaseComponent = temp_clone.into();
        real.font_raw()
    }
    pub fn is_bold(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::bungee::api::chat::BaseComponent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::bungee::api::chat::BaseComponent = temp_clone.into();
        real.is_bold()
    }
    pub fn is_bold_raw(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::bungee::api::chat::BaseComponent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::bungee::api::chat::BaseComponent = temp_clone.into();
        real.is_bold_raw()
    }
    pub fn is_italic(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::bungee::api::chat::BaseComponent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::bungee::api::chat::BaseComponent = temp_clone.into();
        real.is_italic()
    }
    pub fn is_italic_raw(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::bungee::api::chat::BaseComponent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::bungee::api::chat::BaseComponent = temp_clone.into();
        real.is_italic_raw()
    }
    pub fn is_underlined(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::bungee::api::chat::BaseComponent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::bungee::api::chat::BaseComponent = temp_clone.into();
        real.is_underlined()
    }
    pub fn is_underlined_raw(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::bungee::api::chat::BaseComponent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::bungee::api::chat::BaseComponent = temp_clone.into();
        real.is_underlined_raw()
    }
    pub fn is_strikethrough(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::bungee::api::chat::BaseComponent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::bungee::api::chat::BaseComponent = temp_clone.into();
        real.is_strikethrough()
    }
    pub fn is_strikethrough_raw(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::bungee::api::chat::BaseComponent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::bungee::api::chat::BaseComponent = temp_clone.into();
        real.is_strikethrough_raw()
    }
    pub fn duplicate_without_formatting(
        &self,
    ) -> Result<crate::bungee::api::chat::BaseComponent<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::bungee::api::chat::BaseComponent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::bungee::api::chat::BaseComponent = temp_clone.into();
        real.duplicate_without_formatting()
    }
    pub fn set_extra(
        &self,
        arg0: Vec<impl Into<crate::bungee::api::chat::BaseComponent<'mc>>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::bungee::api::chat::BaseComponent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::bungee::api::chat::BaseComponent = temp_clone.into();
        real.set_extra(arg0)
    }
    pub fn extra(
        &self,
    ) -> Result<Vec<crate::bungee::api::chat::BaseComponent<'mc>>, Box<dyn std::error::Error>> {
        let temp_clone = crate::bungee::api::chat::BaseComponent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::bungee::api::chat::BaseComponent = temp_clone.into();
        real.extra()
    }
    pub fn set_color(
        &self,
        arg0: impl Into<crate::bungee::api::ChatColor<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::bungee::api::chat::BaseComponent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::bungee::api::chat::BaseComponent = temp_clone.into();
        real.set_color(arg0)
    }
    pub fn color(&self) -> Result<crate::bungee::api::ChatColor<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::bungee::api::chat::BaseComponent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::bungee::api::chat::BaseComponent = temp_clone.into();
        real.color()
    }
    pub fn is_reset(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::bungee::api::chat::BaseComponent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::bungee::api::chat::BaseComponent = temp_clone.into();
        real.is_reset()
    }
    // SUPER CLASS: Object

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
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
pub enum FormatRetention<'mc> {
    None { inner: FormatRetentionStruct<'mc> },
    Formatting { inner: FormatRetentionStruct<'mc> },
    Events { inner: FormatRetentionStruct<'mc> },
    All { inner: FormatRetentionStruct<'mc> },
}
impl<'mc> std::fmt::Display for FormatRetention<'mc> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FormatRetention::None { .. } => f.write_str("NONE"),
            FormatRetention::Formatting { .. } => f.write_str("FORMATTING"),
            FormatRetention::Events { .. } => f.write_str("EVENTS"),
            FormatRetention::All { .. } => f.write_str("ALL"),
        }
    }
}

impl<'mc> FormatRetention<'mc> {
    pub fn value_of(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<String>,
    ) -> Result<FormatRetention<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(env.new_string(arg0.into())?);
        let cls = env.find_class("net/md_5/bungee/api/chat/FormatRetention");
        let cls = env.translate_error_with_class(cls)?;
        let res = env.call_static_method(
            cls,
            "valueOf",
            "(Ljava/lang/String;)Lnet/md_5/bungee/api/chat/FormatRetention;",
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
            "NONE" => Ok(FormatRetention::None {
                inner: FormatRetentionStruct::from_raw(env, obj)?,
            }),
            "FORMATTING" => Ok(FormatRetention::Formatting {
                inner: FormatRetentionStruct::from_raw(env, obj)?,
            }),
            "EVENTS" => Ok(FormatRetention::Events {
                inner: FormatRetentionStruct::from_raw(env, obj)?,
            }),
            "ALL" => Ok(FormatRetention::All {
                inner: FormatRetentionStruct::from_raw(env, obj)?,
            }),

            _ => Err(eyre::eyre!("String gaven for variant was invalid").into()),
        }
    }
}

#[repr(C)]
pub struct FormatRetentionStruct<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for FormatRetention<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        match self {
            Self::None { inner } => inner.0.clone(),
            Self::Formatting { inner } => inner.0.clone(),
            Self::Events { inner } => inner.0.clone(),
            Self::All { inner } => inner.0.clone(),
        }
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        match self {
            Self::None { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Formatting { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::Events { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::All { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
        }
    }
}
impl<'mc> JNIInstantiatable<'mc> for FormatRetention<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
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
            let variant = env.call_method(&obj, "toString", "()Ljava/lang/String;", vec![]);
            let variant = env.translate_error(variant)?;
            let variant_str = env
                .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
                .to_string_lossy()
                .to_string();
            match variant_str.as_str() {
                "NONE" => Ok(FormatRetention::None {
                    inner: FormatRetentionStruct::from_raw(env, obj)?,
                }),
                "FORMATTING" => Ok(FormatRetention::Formatting {
                    inner: FormatRetentionStruct::from_raw(env, obj)?,
                }),
                "EVENTS" => Ok(FormatRetention::Events {
                    inner: FormatRetentionStruct::from_raw(env, obj)?,
                }),
                "ALL" => Ok(FormatRetention::All {
                    inner: FormatRetentionStruct::from_raw(env, obj)?,
                }),
                _ => Err(eyre::eyre!("String gaven for variant was invalid").into()),
            }
        }
    }
}

impl<'mc> JNIRaw<'mc> for FormatRetentionStruct<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for FormatRetentionStruct<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate FormatRetentionStruct from null object."
            )
            .into());
        }
        let (valid, name) = env.validate_name(&obj, "net/md_5/bungee/api/chat/FormatRetention")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a FormatRetentionStruct object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> FormatRetentionStruct<'mc> {
    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}

#[repr(C)]
pub struct HoverEvent<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
pub enum HoverEventAction<'mc> {
    ShowText { inner: HoverEventActionStruct<'mc> },
    ShowItem { inner: HoverEventActionStruct<'mc> },
    ShowEntity { inner: HoverEventActionStruct<'mc> },
    ShowAchievement { inner: HoverEventActionStruct<'mc> },
}
impl<'mc> std::fmt::Display for HoverEventAction<'mc> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HoverEventAction::ShowText { .. } => f.write_str("SHOW_TEXT"),
            HoverEventAction::ShowItem { .. } => f.write_str("SHOW_ITEM"),
            HoverEventAction::ShowEntity { .. } => f.write_str("SHOW_ENTITY"),
            HoverEventAction::ShowAchievement { .. } => f.write_str("SHOW_ACHIEVEMENT"),
        }
    }
}

impl<'mc> HoverEventAction<'mc> {
    pub fn value_of(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<String>,
    ) -> Result<HoverEventAction<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(env.new_string(arg0.into())?);
        let cls = env.find_class("net/md_5/bungee/api/chat/HoverEvent$Action");
        let cls = env.translate_error_with_class(cls)?;
        let res = env.call_static_method(
            cls,
            "valueOf",
            "(Ljava/lang/String;)Lnet/md_5/bungee/api/chat/HoverEvent$Action;",
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
            "SHOW_TEXT" => Ok(HoverEventAction::ShowText {
                inner: HoverEventActionStruct::from_raw(env, obj)?,
            }),
            "SHOW_ITEM" => Ok(HoverEventAction::ShowItem {
                inner: HoverEventActionStruct::from_raw(env, obj)?,
            }),
            "SHOW_ENTITY" => Ok(HoverEventAction::ShowEntity {
                inner: HoverEventActionStruct::from_raw(env, obj)?,
            }),
            "SHOW_ACHIEVEMENT" => Ok(HoverEventAction::ShowAchievement {
                inner: HoverEventActionStruct::from_raw(env, obj)?,
            }),

            _ => Err(eyre::eyre!("String gaven for variant was invalid").into()),
        }
    }
}

#[repr(C)]
pub struct HoverEventActionStruct<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for HoverEventAction<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        match self {
            Self::ShowText { inner } => inner.0.clone(),
            Self::ShowItem { inner } => inner.0.clone(),
            Self::ShowEntity { inner } => inner.0.clone(),
            Self::ShowAchievement { inner } => inner.0.clone(),
        }
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        match self {
            Self::ShowText { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::ShowItem { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::ShowEntity { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::ShowAchievement { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
        }
    }
}
impl<'mc> JNIInstantiatable<'mc> for HoverEventAction<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
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
            let variant = env.call_method(&obj, "toString", "()Ljava/lang/String;", vec![]);
            let variant = env.translate_error(variant)?;
            let variant_str = env
                .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
                .to_string_lossy()
                .to_string();
            match variant_str.as_str() {
                "SHOW_TEXT" => Ok(HoverEventAction::ShowText {
                    inner: HoverEventActionStruct::from_raw(env, obj)?,
                }),
                "SHOW_ITEM" => Ok(HoverEventAction::ShowItem {
                    inner: HoverEventActionStruct::from_raw(env, obj)?,
                }),
                "SHOW_ENTITY" => Ok(HoverEventAction::ShowEntity {
                    inner: HoverEventActionStruct::from_raw(env, obj)?,
                }),
                "SHOW_ACHIEVEMENT" => Ok(HoverEventAction::ShowAchievement {
                    inner: HoverEventActionStruct::from_raw(env, obj)?,
                }),
                _ => Err(eyre::eyre!("String gaven for variant was invalid").into()),
            }
        }
    }
}

impl<'mc> JNIRaw<'mc> for HoverEventActionStruct<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for HoverEventActionStruct<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate HoverEventActionStruct from null object."
            )
            .into());
        }
        let (valid, name) =
            env.validate_name(&obj, "net/md_5/bungee/api/chat/HoverEvent$Action")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a HoverEventActionStruct object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> HoverEventActionStruct<'mc> {
    pub fn values(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<Vec<crate::bungee::api::chat::HoverEventAction<'mc>>, Box<dyn std::error::Error>>
    {
        let sig = String::from("()Lnet/md_5/bungee/api/chat/HoverEvent$Action;");
        let cls = jni.find_class("net/md_5/bungee/api/chat/HoverEvent$Action");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "values", sig.as_str(), vec![]);
        let res = jni.translate_error(res)?;
        let arr = Into::<jni::objects::JObjectArray>::into(res.l()?);
        let len = jni.get_array_length(&arr)?;
        let mut vec = Vec::new();
        for i in 0..len {
            let res = jni.get_object_array_element(&arr, i)?;
            vec.push({ crate::bungee::api::chat::HoverEventAction::from_raw(&jni, res)? });
        }
        Ok(vec)
    }
    // SUPER CLASS: Enum

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
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
    pub fn new_with_hover_eventaction(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<crate::bungee::api::chat::HoverEventAction<'mc>>,
        arg1: Vec<crate::bungee::api::chat::hover::content::Content<'mc>>,
    ) -> Result<crate::bungee::api::chat::HoverEvent<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lnet/md_5/bungee/api/chat/HoverEvent$Action;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        sig += "[Lnet/md_5/bungee/api/chat/hover/content/Content;";
        let arr = jni.new_object_array(
            arg1.len() as i32,
            "net/md_5/bungee/api/chat/hover/content/Content",
            jni::objects::JObject::null(),
        );
        let arr = jni.translate_error_no_gen(arr)?;
        for i in 0..arg1.len() {
            let val_2 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(arg1.get(i).unwrap().jni_object().clone())
            });
            jni.set_object_array_element(&arr, i as i32, val_2.l()?)?;
        }
        let val_2 = jni::objects::JValueGen::Object(arr);
        args.push(val_2.l()?.into());
        sig += ")V";
        let cls = jni.find_class("net/md_5/bungee/api/chat/HoverEvent");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(cls, sig.as_str(), args);
        let res = jni.translate_error_no_gen(res)?;
        crate::bungee::api::chat::HoverEvent::from_raw(&jni, res)
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
        crate::bungee::api::chat::HoverEventAction::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
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

    pub fn get_class(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<crate::bungee::api::chat::HoverEventAction<'mc>>,
        arg1: bool,
    ) -> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Lnet/md_5/bungee/api/chat/HoverEvent$Action;Z)Ljava/lang/Class;");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let val_2 = jni::objects::JValueGen::Bool(arg1.into());
        let cls = jni.find_class("java/lang/Class");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(
            cls,
            "getClass",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
            ],
        );
        let res = jni.translate_error(res)?;
        Ok(unsafe { jni::objects::JClass::from_raw(res.as_jni().l) })
    }

    pub fn value(
        &self,
    ) -> Result<Vec<crate::bungee::api::chat::BaseComponent<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lnet/md_5/bungee/api/chat/BaseComponent;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getValue", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        let arr = Into::<jni::objects::JObjectArray>::into(res.l()?);
        let len = self.jni_ref().get_array_length(&arr)?;
        let mut vec = Vec::new();
        for i in 0..len {
            let res = self.jni_ref().get_object_array_element(&arr, i)?;
            vec.push({ crate::bungee::api::chat::BaseComponent::from_raw(&self.jni_ref(), res)? });
        }
        Ok(vec)
    }
    // SUPER CLASS: Object

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
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

#[repr(C)]
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
        arg0: std::option::Option<Vec<crate::bungee::api::chat::BaseComponent<'mc>>>,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "[Lnet/md_5/bungee/api/chat/BaseComponent;";
            let arr = jni.new_object_array(
                a.len() as i32,
                "net/md_5/bungee/api/chat/BaseComponent",
                jni::objects::JObject::null(),
            );
            let arr = jni.translate_error_no_gen(arr)?;
            for i in 0..a.len() {
                let val_1 = jni::objects::JValueGen::Object(unsafe {
                    jni::objects::JObject::from_raw(a.get(i).unwrap().jni_object().clone())
                });
                jni.set_object_array_element(&arr, i as i32, val_1.l()?)?;
            }
            let val_1 = jni::objects::JValueGen::Object(arr);
            args.push(val_1.l()?.into());
        }
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
        arg0: std::option::Option<Vec<crate::bungee::api::chat::BaseComponent<'mc>>>,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "[Lnet/md_5/bungee/api/chat/BaseComponent;";
            let arr = jni.new_object_array(
                a.len() as i32,
                "net/md_5/bungee/api/chat/BaseComponent",
                jni::objects::JObject::null(),
            );
            let arr = jni.translate_error_no_gen(arr)?;
            for i in 0..a.len() {
                let val_1 = jni::objects::JValueGen::Object(unsafe {
                    jni::objects::JObject::from_raw(a.get(i).unwrap().jni_object().clone())
                });
                jni.set_object_array_element(&arr, i as i32, val_1.l()?)?;
            }
            let val_1 = jni::objects::JValueGen::Object(arr);
            args.push(val_1.l()?.into());
        }
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

    pub fn set_extra(
        &self,
        arg0: Vec<impl Into<crate::bungee::api::chat::BaseComponent<'mc>>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/util/List;)V");
        let raw_val_1 = self
            .jni_ref()
            .new_object("java/util/ArrayList", "()V", vec![])?;
        for v in arg0 {
            let map_val_0 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(v.into().jni_object().clone())
            });
            self.jni_ref().call_method(
                &raw_val_1,
                "add",
                "(Ljava/lang/Object;)Z",
                vec![jni::objects::JValueGen::from(map_val_0)],
            )?;
        }
        let val_1 = jni::objects::JValueGen::Object(raw_val_1);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setExtra",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
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
    // SUPER CLASS: Object

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
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

#[repr(C)]
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
    // SUPER CLASS: BaseComponent
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
        let temp_clone = crate::bungee::api::chat::BaseComponent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::bungee::api::chat::BaseComponent = temp_clone.into();
        real.retain(arg0)
    }
    pub fn to_legacy_text_with_base_components(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: std::option::Option<Vec<crate::bungee::api::chat::BaseComponent<'mc>>>,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "[Lnet/md_5/bungee/api/chat/BaseComponent;";
            let arr = jni.new_object_array(
                a.len() as i32,
                "net/md_5/bungee/api/chat/BaseComponent",
                jni::objects::JObject::null(),
            );
            let arr = jni.translate_error_no_gen(arr)?;
            for i in 0..a.len() {
                let val_1 = jni::objects::JValueGen::Object(unsafe {
                    jni::objects::JObject::from_raw(a.get(i).unwrap().jni_object().clone())
                });
                jni.set_object_array_element(&arr, i as i32, val_1.l()?)?;
            }
            let val_1 = jni::objects::JValueGen::Object(arr);
            args.push(val_1.l()?.into());
        }
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
        arg0: std::option::Option<Vec<crate::bungee::api::chat::BaseComponent<'mc>>>,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "[Lnet/md_5/bungee/api/chat/BaseComponent;";
            let arr = jni.new_object_array(
                a.len() as i32,
                "net/md_5/bungee/api/chat/BaseComponent",
                jni::objects::JObject::null(),
            );
            let arr = jni.translate_error_no_gen(arr)?;
            for i in 0..a.len() {
                let val_1 = jni::objects::JValueGen::Object(unsafe {
                    jni::objects::JObject::from_raw(a.get(i).unwrap().jni_object().clone())
                });
                jni.set_object_array_element(&arr, i as i32, val_1.l()?)?;
            }
            let val_1 = jni::objects::JValueGen::Object(arr);
            args.push(val_1.l()?.into());
        }
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
        let temp_clone = crate::bungee::api::chat::BaseComponent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::bungee::api::chat::BaseComponent = temp_clone.into();
        real.color_raw()
    }
    pub fn insertion(&self) -> Result<String, Box<dyn std::error::Error>> {
        let temp_clone = crate::bungee::api::chat::BaseComponent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::bungee::api::chat::BaseComponent = temp_clone.into();
        real.insertion()
    }
    pub fn click_event(
        &self,
    ) -> Result<crate::bungee::api::chat::ClickEvent<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::bungee::api::chat::BaseComponent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::bungee::api::chat::BaseComponent = temp_clone.into();
        real.click_event()
    }
    pub fn hover_event(
        &self,
    ) -> Result<crate::bungee::api::chat::HoverEvent<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::bungee::api::chat::BaseComponent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::bungee::api::chat::BaseComponent = temp_clone.into();
        real.hover_event()
    }
    pub fn is_obfuscated(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::bungee::api::chat::BaseComponent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::bungee::api::chat::BaseComponent = temp_clone.into();
        real.is_obfuscated()
    }
    pub fn is_obfuscated_raw(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::bungee::api::chat::BaseComponent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::bungee::api::chat::BaseComponent = temp_clone.into();
        real.is_obfuscated_raw()
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
        let temp_clone = crate::bungee::api::chat::BaseComponent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::bungee::api::chat::BaseComponent = temp_clone.into();
        real.has_formatting()
    }
    pub fn set_font(&self, arg0: impl Into<String>) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::bungee::api::chat::BaseComponent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::bungee::api::chat::BaseComponent = temp_clone.into();
        real.set_font(arg0)
    }
    pub fn set_bold(&self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::bungee::api::chat::BaseComponent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::bungee::api::chat::BaseComponent = temp_clone.into();
        real.set_bold(arg0)
    }
    pub fn set_italic(&self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::bungee::api::chat::BaseComponent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::bungee::api::chat::BaseComponent = temp_clone.into();
        real.set_italic(arg0)
    }
    pub fn set_underlined(&self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::bungee::api::chat::BaseComponent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::bungee::api::chat::BaseComponent = temp_clone.into();
        real.set_underlined(arg0)
    }
    pub fn set_strikethrough(&self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::bungee::api::chat::BaseComponent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::bungee::api::chat::BaseComponent = temp_clone.into();
        real.set_strikethrough(arg0)
    }
    pub fn set_obfuscated(&self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::bungee::api::chat::BaseComponent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::bungee::api::chat::BaseComponent = temp_clone.into();
        real.set_obfuscated(arg0)
    }
    pub fn set_insertion(&self, arg0: impl Into<String>) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::bungee::api::chat::BaseComponent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::bungee::api::chat::BaseComponent = temp_clone.into();
        real.set_insertion(arg0)
    }
    pub fn set_click_event(
        &self,
        arg0: impl Into<crate::bungee::api::chat::ClickEvent<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::bungee::api::chat::BaseComponent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::bungee::api::chat::BaseComponent = temp_clone.into();
        real.set_click_event(arg0)
    }
    pub fn set_hover_event(
        &self,
        arg0: impl Into<crate::bungee::api::chat::HoverEvent<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::bungee::api::chat::BaseComponent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::bungee::api::chat::BaseComponent = temp_clone.into();
        real.set_hover_event(arg0)
    }
    pub fn set_reset(&self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::bungee::api::chat::BaseComponent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::bungee::api::chat::BaseComponent = temp_clone.into();
        real.set_reset(arg0)
    }
    pub fn font(&self) -> Result<String, Box<dyn std::error::Error>> {
        let temp_clone = crate::bungee::api::chat::BaseComponent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::bungee::api::chat::BaseComponent = temp_clone.into();
        real.font()
    }
    pub fn font_raw(&self) -> Result<String, Box<dyn std::error::Error>> {
        let temp_clone = crate::bungee::api::chat::BaseComponent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::bungee::api::chat::BaseComponent = temp_clone.into();
        real.font_raw()
    }
    pub fn is_bold(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::bungee::api::chat::BaseComponent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::bungee::api::chat::BaseComponent = temp_clone.into();
        real.is_bold()
    }
    pub fn is_bold_raw(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::bungee::api::chat::BaseComponent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::bungee::api::chat::BaseComponent = temp_clone.into();
        real.is_bold_raw()
    }
    pub fn is_italic(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::bungee::api::chat::BaseComponent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::bungee::api::chat::BaseComponent = temp_clone.into();
        real.is_italic()
    }
    pub fn is_italic_raw(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::bungee::api::chat::BaseComponent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::bungee::api::chat::BaseComponent = temp_clone.into();
        real.is_italic_raw()
    }
    pub fn is_underlined(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::bungee::api::chat::BaseComponent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::bungee::api::chat::BaseComponent = temp_clone.into();
        real.is_underlined()
    }
    pub fn is_underlined_raw(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::bungee::api::chat::BaseComponent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::bungee::api::chat::BaseComponent = temp_clone.into();
        real.is_underlined_raw()
    }
    pub fn is_strikethrough(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::bungee::api::chat::BaseComponent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::bungee::api::chat::BaseComponent = temp_clone.into();
        real.is_strikethrough()
    }
    pub fn is_strikethrough_raw(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::bungee::api::chat::BaseComponent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::bungee::api::chat::BaseComponent = temp_clone.into();
        real.is_strikethrough_raw()
    }
    pub fn duplicate_without_formatting(
        &self,
    ) -> Result<crate::bungee::api::chat::BaseComponent<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::bungee::api::chat::BaseComponent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::bungee::api::chat::BaseComponent = temp_clone.into();
        real.duplicate_without_formatting()
    }
    pub fn set_extra(
        &self,
        arg0: Vec<impl Into<crate::bungee::api::chat::BaseComponent<'mc>>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::bungee::api::chat::BaseComponent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::bungee::api::chat::BaseComponent = temp_clone.into();
        real.set_extra(arg0)
    }
    pub fn extra(
        &self,
    ) -> Result<Vec<crate::bungee::api::chat::BaseComponent<'mc>>, Box<dyn std::error::Error>> {
        let temp_clone = crate::bungee::api::chat::BaseComponent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::bungee::api::chat::BaseComponent = temp_clone.into();
        real.extra()
    }
    pub fn set_color(
        &self,
        arg0: impl Into<crate::bungee::api::ChatColor<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::bungee::api::chat::BaseComponent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::bungee::api::chat::BaseComponent = temp_clone.into();
        real.set_color(arg0)
    }
    pub fn color(&self) -> Result<crate::bungee::api::ChatColor<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::bungee::api::chat::BaseComponent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::bungee::api::chat::BaseComponent = temp_clone.into();
        real.color()
    }
    pub fn is_reset(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::bungee::api::chat::BaseComponent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::bungee::api::chat::BaseComponent = temp_clone.into();
        real.is_reset()
    }
    // SUPER CLASS: Object

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
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

#[repr(C)]
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
    ) -> Result<crate::bungee::api::chat::ScoreComponent<'mc>, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Lnet/md_5/bungee/api/chat/ScoreComponent;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "duplicate", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        crate::bungee::api::chat::ScoreComponent::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    // SUPER CLASS: BaseComponent
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
        let temp_clone = crate::bungee::api::chat::BaseComponent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::bungee::api::chat::BaseComponent = temp_clone.into();
        real.retain(arg0)
    }
    pub fn to_legacy_text_with_base_components(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: std::option::Option<Vec<crate::bungee::api::chat::BaseComponent<'mc>>>,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "[Lnet/md_5/bungee/api/chat/BaseComponent;";
            let arr = jni.new_object_array(
                a.len() as i32,
                "net/md_5/bungee/api/chat/BaseComponent",
                jni::objects::JObject::null(),
            );
            let arr = jni.translate_error_no_gen(arr)?;
            for i in 0..a.len() {
                let val_1 = jni::objects::JValueGen::Object(unsafe {
                    jni::objects::JObject::from_raw(a.get(i).unwrap().jni_object().clone())
                });
                jni.set_object_array_element(&arr, i as i32, val_1.l()?)?;
            }
            let val_1 = jni::objects::JValueGen::Object(arr);
            args.push(val_1.l()?.into());
        }
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
        arg0: std::option::Option<Vec<crate::bungee::api::chat::BaseComponent<'mc>>>,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "[Lnet/md_5/bungee/api/chat/BaseComponent;";
            let arr = jni.new_object_array(
                a.len() as i32,
                "net/md_5/bungee/api/chat/BaseComponent",
                jni::objects::JObject::null(),
            );
            let arr = jni.translate_error_no_gen(arr)?;
            for i in 0..a.len() {
                let val_1 = jni::objects::JValueGen::Object(unsafe {
                    jni::objects::JObject::from_raw(a.get(i).unwrap().jni_object().clone())
                });
                jni.set_object_array_element(&arr, i as i32, val_1.l()?)?;
            }
            let val_1 = jni::objects::JValueGen::Object(arr);
            args.push(val_1.l()?.into());
        }
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
        let temp_clone = crate::bungee::api::chat::BaseComponent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::bungee::api::chat::BaseComponent = temp_clone.into();
        real.color_raw()
    }
    pub fn insertion(&self) -> Result<String, Box<dyn std::error::Error>> {
        let temp_clone = crate::bungee::api::chat::BaseComponent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::bungee::api::chat::BaseComponent = temp_clone.into();
        real.insertion()
    }
    pub fn click_event(
        &self,
    ) -> Result<crate::bungee::api::chat::ClickEvent<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::bungee::api::chat::BaseComponent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::bungee::api::chat::BaseComponent = temp_clone.into();
        real.click_event()
    }
    pub fn hover_event(
        &self,
    ) -> Result<crate::bungee::api::chat::HoverEvent<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::bungee::api::chat::BaseComponent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::bungee::api::chat::BaseComponent = temp_clone.into();
        real.hover_event()
    }
    pub fn is_obfuscated(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::bungee::api::chat::BaseComponent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::bungee::api::chat::BaseComponent = temp_clone.into();
        real.is_obfuscated()
    }
    pub fn is_obfuscated_raw(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::bungee::api::chat::BaseComponent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::bungee::api::chat::BaseComponent = temp_clone.into();
        real.is_obfuscated_raw()
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
        let temp_clone = crate::bungee::api::chat::BaseComponent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::bungee::api::chat::BaseComponent = temp_clone.into();
        real.has_formatting()
    }
    pub fn set_font(&self, arg0: impl Into<String>) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::bungee::api::chat::BaseComponent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::bungee::api::chat::BaseComponent = temp_clone.into();
        real.set_font(arg0)
    }
    pub fn set_bold(&self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::bungee::api::chat::BaseComponent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::bungee::api::chat::BaseComponent = temp_clone.into();
        real.set_bold(arg0)
    }
    pub fn set_italic(&self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::bungee::api::chat::BaseComponent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::bungee::api::chat::BaseComponent = temp_clone.into();
        real.set_italic(arg0)
    }
    pub fn set_underlined(&self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::bungee::api::chat::BaseComponent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::bungee::api::chat::BaseComponent = temp_clone.into();
        real.set_underlined(arg0)
    }
    pub fn set_strikethrough(&self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::bungee::api::chat::BaseComponent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::bungee::api::chat::BaseComponent = temp_clone.into();
        real.set_strikethrough(arg0)
    }
    pub fn set_obfuscated(&self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::bungee::api::chat::BaseComponent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::bungee::api::chat::BaseComponent = temp_clone.into();
        real.set_obfuscated(arg0)
    }
    pub fn set_insertion(&self, arg0: impl Into<String>) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::bungee::api::chat::BaseComponent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::bungee::api::chat::BaseComponent = temp_clone.into();
        real.set_insertion(arg0)
    }
    pub fn set_click_event(
        &self,
        arg0: impl Into<crate::bungee::api::chat::ClickEvent<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::bungee::api::chat::BaseComponent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::bungee::api::chat::BaseComponent = temp_clone.into();
        real.set_click_event(arg0)
    }
    pub fn set_hover_event(
        &self,
        arg0: impl Into<crate::bungee::api::chat::HoverEvent<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::bungee::api::chat::BaseComponent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::bungee::api::chat::BaseComponent = temp_clone.into();
        real.set_hover_event(arg0)
    }
    pub fn set_reset(&self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::bungee::api::chat::BaseComponent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::bungee::api::chat::BaseComponent = temp_clone.into();
        real.set_reset(arg0)
    }
    pub fn font(&self) -> Result<String, Box<dyn std::error::Error>> {
        let temp_clone = crate::bungee::api::chat::BaseComponent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::bungee::api::chat::BaseComponent = temp_clone.into();
        real.font()
    }
    pub fn font_raw(&self) -> Result<String, Box<dyn std::error::Error>> {
        let temp_clone = crate::bungee::api::chat::BaseComponent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::bungee::api::chat::BaseComponent = temp_clone.into();
        real.font_raw()
    }
    pub fn is_bold(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::bungee::api::chat::BaseComponent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::bungee::api::chat::BaseComponent = temp_clone.into();
        real.is_bold()
    }
    pub fn is_bold_raw(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::bungee::api::chat::BaseComponent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::bungee::api::chat::BaseComponent = temp_clone.into();
        real.is_bold_raw()
    }
    pub fn is_italic(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::bungee::api::chat::BaseComponent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::bungee::api::chat::BaseComponent = temp_clone.into();
        real.is_italic()
    }
    pub fn is_italic_raw(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::bungee::api::chat::BaseComponent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::bungee::api::chat::BaseComponent = temp_clone.into();
        real.is_italic_raw()
    }
    pub fn is_underlined(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::bungee::api::chat::BaseComponent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::bungee::api::chat::BaseComponent = temp_clone.into();
        real.is_underlined()
    }
    pub fn is_underlined_raw(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::bungee::api::chat::BaseComponent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::bungee::api::chat::BaseComponent = temp_clone.into();
        real.is_underlined_raw()
    }
    pub fn is_strikethrough(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::bungee::api::chat::BaseComponent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::bungee::api::chat::BaseComponent = temp_clone.into();
        real.is_strikethrough()
    }
    pub fn is_strikethrough_raw(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::bungee::api::chat::BaseComponent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::bungee::api::chat::BaseComponent = temp_clone.into();
        real.is_strikethrough_raw()
    }
    pub fn duplicate_without_formatting(
        &self,
    ) -> Result<crate::bungee::api::chat::BaseComponent<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::bungee::api::chat::BaseComponent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::bungee::api::chat::BaseComponent = temp_clone.into();
        real.duplicate_without_formatting()
    }
    pub fn set_extra(
        &self,
        arg0: Vec<impl Into<crate::bungee::api::chat::BaseComponent<'mc>>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::bungee::api::chat::BaseComponent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::bungee::api::chat::BaseComponent = temp_clone.into();
        real.set_extra(arg0)
    }
    pub fn extra(
        &self,
    ) -> Result<Vec<crate::bungee::api::chat::BaseComponent<'mc>>, Box<dyn std::error::Error>> {
        let temp_clone = crate::bungee::api::chat::BaseComponent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::bungee::api::chat::BaseComponent = temp_clone.into();
        real.extra()
    }
    pub fn set_color(
        &self,
        arg0: impl Into<crate::bungee::api::ChatColor<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::bungee::api::chat::BaseComponent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::bungee::api::chat::BaseComponent = temp_clone.into();
        real.set_color(arg0)
    }
    pub fn color(&self) -> Result<crate::bungee::api::ChatColor<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::bungee::api::chat::BaseComponent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::bungee::api::chat::BaseComponent = temp_clone.into();
        real.color()
    }
    pub fn is_reset(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::bungee::api::chat::BaseComponent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::bungee::api::chat::BaseComponent = temp_clone.into();
        real.is_reset()
    }
    // SUPER CLASS: Object

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
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

#[repr(C)]
pub struct ClickEvent<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
pub enum ClickEventAction<'mc> {
    OpenUrl { inner: ClickEventActionStruct<'mc> },
    OpenFile { inner: ClickEventActionStruct<'mc> },
    RunCommand { inner: ClickEventActionStruct<'mc> },
    SuggestCommand { inner: ClickEventActionStruct<'mc> },
    ChangePage { inner: ClickEventActionStruct<'mc> },
    CopyToClipboard { inner: ClickEventActionStruct<'mc> },
}
impl<'mc> std::fmt::Display for ClickEventAction<'mc> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ClickEventAction::OpenUrl { .. } => f.write_str("OPEN_URL"),
            ClickEventAction::OpenFile { .. } => f.write_str("OPEN_FILE"),
            ClickEventAction::RunCommand { .. } => f.write_str("RUN_COMMAND"),
            ClickEventAction::SuggestCommand { .. } => f.write_str("SUGGEST_COMMAND"),
            ClickEventAction::ChangePage { .. } => f.write_str("CHANGE_PAGE"),
            ClickEventAction::CopyToClipboard { .. } => f.write_str("COPY_TO_CLIPBOARD"),
        }
    }
}

impl<'mc> ClickEventAction<'mc> {
    pub fn value_of(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<String>,
    ) -> Result<ClickEventAction<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(env.new_string(arg0.into())?);
        let cls = env.find_class("net/md_5/bungee/api/chat/ClickEvent$Action");
        let cls = env.translate_error_with_class(cls)?;
        let res = env.call_static_method(
            cls,
            "valueOf",
            "(Ljava/lang/String;)Lnet/md_5/bungee/api/chat/ClickEvent$Action;",
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
            "OPEN_URL" => Ok(ClickEventAction::OpenUrl {
                inner: ClickEventActionStruct::from_raw(env, obj)?,
            }),
            "OPEN_FILE" => Ok(ClickEventAction::OpenFile {
                inner: ClickEventActionStruct::from_raw(env, obj)?,
            }),
            "RUN_COMMAND" => Ok(ClickEventAction::RunCommand {
                inner: ClickEventActionStruct::from_raw(env, obj)?,
            }),
            "SUGGEST_COMMAND" => Ok(ClickEventAction::SuggestCommand {
                inner: ClickEventActionStruct::from_raw(env, obj)?,
            }),
            "CHANGE_PAGE" => Ok(ClickEventAction::ChangePage {
                inner: ClickEventActionStruct::from_raw(env, obj)?,
            }),
            "COPY_TO_CLIPBOARD" => Ok(ClickEventAction::CopyToClipboard {
                inner: ClickEventActionStruct::from_raw(env, obj)?,
            }),

            _ => Err(eyre::eyre!("String gaven for variant was invalid").into()),
        }
    }
}

#[repr(C)]
pub struct ClickEventActionStruct<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for ClickEventAction<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        match self {
            Self::OpenUrl { inner } => inner.0.clone(),
            Self::OpenFile { inner } => inner.0.clone(),
            Self::RunCommand { inner } => inner.0.clone(),
            Self::SuggestCommand { inner } => inner.0.clone(),
            Self::ChangePage { inner } => inner.0.clone(),
            Self::CopyToClipboard { inner } => inner.0.clone(),
        }
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        match self {
            Self::OpenUrl { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::OpenFile { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::RunCommand { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::SuggestCommand { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::ChangePage { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::CopyToClipboard { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
        }
    }
}
impl<'mc> JNIInstantiatable<'mc> for ClickEventAction<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
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
            let variant = env.call_method(&obj, "toString", "()Ljava/lang/String;", vec![]);
            let variant = env.translate_error(variant)?;
            let variant_str = env
                .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
                .to_string_lossy()
                .to_string();
            match variant_str.as_str() {
                "OPEN_URL" => Ok(ClickEventAction::OpenUrl {
                    inner: ClickEventActionStruct::from_raw(env, obj)?,
                }),
                "OPEN_FILE" => Ok(ClickEventAction::OpenFile {
                    inner: ClickEventActionStruct::from_raw(env, obj)?,
                }),
                "RUN_COMMAND" => Ok(ClickEventAction::RunCommand {
                    inner: ClickEventActionStruct::from_raw(env, obj)?,
                }),
                "SUGGEST_COMMAND" => Ok(ClickEventAction::SuggestCommand {
                    inner: ClickEventActionStruct::from_raw(env, obj)?,
                }),
                "CHANGE_PAGE" => Ok(ClickEventAction::ChangePage {
                    inner: ClickEventActionStruct::from_raw(env, obj)?,
                }),
                "COPY_TO_CLIPBOARD" => Ok(ClickEventAction::CopyToClipboard {
                    inner: ClickEventActionStruct::from_raw(env, obj)?,
                }),
                _ => Err(eyre::eyre!("String gaven for variant was invalid").into()),
            }
        }
    }
}

impl<'mc> JNIRaw<'mc> for ClickEventActionStruct<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for ClickEventActionStruct<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate ClickEventActionStruct from null object."
            )
            .into());
        }
        let (valid, name) =
            env.validate_name(&obj, "net/md_5/bungee/api/chat/ClickEvent$Action")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a ClickEventActionStruct object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> ClickEventActionStruct<'mc> {
    pub fn values(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<Vec<crate::bungee::api::chat::ClickEventAction<'mc>>, Box<dyn std::error::Error>>
    {
        let sig = String::from("()Lnet/md_5/bungee/api/chat/ClickEvent$Action;");
        let cls = jni.find_class("net/md_5/bungee/api/chat/ClickEvent$Action");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "values", sig.as_str(), vec![]);
        let res = jni.translate_error(res)?;
        let arr = Into::<jni::objects::JObjectArray>::into(res.l()?);
        let len = jni.get_array_length(&arr)?;
        let mut vec = Vec::new();
        for i in 0..len {
            let res = jni.get_object_array_element(&arr, i)?;
            vec.push({ crate::bungee::api::chat::ClickEventAction::from_raw(&jni, res)? });
        }
        Ok(vec)
    }
    // SUPER CLASS: Enum

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
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

    pub fn action(
        &self,
    ) -> Result<crate::bungee::api::chat::ClickEventAction<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lnet/md_5/bungee/api/chat/ClickEvent$Action;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getAction", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::bungee::api::chat::ClickEventAction::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
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
    // SUPER CLASS: Object

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
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

///
/// This is a representation of an abstract class.
#[repr(C)]
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
    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}

#[repr(C)]
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

    pub fn serialize_with_object(
        &self,
        arg0: jni::objects::JObject<'mc>,
        arg1: jni::objects::JObject<'mc>,
        arg2: jni::objects::JObject<'mc>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Ljava/lang/Object;";
        let val_1 = jni::objects::JValueGen::Object(arg0);
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
    ) -> Result<crate::bungee::api::chat::ItemTag<'mc>, Box<dyn std::error::Error>> {
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
        sig += ")Lnet/md_5/bungee/api/chat/ItemTag;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "deserialize", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        crate::bungee::api::chat::ItemTag::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    // SUPER CLASS: Object

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<jni::objects::JObject<'mc>> for ItemTagSerializer<'mc> {
    fn into(self) -> jni::objects::JObject<'mc> {
        self.1
    }
}

#[repr(C)]
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
        if let Some(a) = arg1 {
            sig += "[Ljava/lang/Object;";
            let arr = jni.new_object_array(
                a.len() as i32,
                "java/lang/Object",
                jni::objects::JObject::null(),
            );
            let arr = jni.translate_error_no_gen(arr)?;
            for i in 0..a.len() {
                let val_2 = jni::objects::JValueGen::Object(a.get(i).unwrap());
                jni.set_object_array_element(&arr, i as i32, val_2.l()?)?;
            }
            let val_2 = jni::objects::JValueGen::Object(arr);
            args.push(val_2.l()?.into());
        }
        sig += ")V";
        let cls = jni.find_class("net/md_5/bungee/api/chat/TranslatableComponent");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(cls, sig.as_str(), args);
        let res = jni.translate_error_no_gen(res)?;
        crate::bungee::api::chat::TranslatableComponent::from_raw(&jni, res)
    }

    pub fn set_with(
        &self,
        arg0: Vec<impl Into<crate::bungee::api::chat::BaseComponent<'mc>>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/util/List;)V");
        let raw_val_1 = self
            .jni_ref()
            .new_object("java/util/ArrayList", "()V", vec![])?;
        for v in arg0 {
            let map_val_0 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(v.into().jni_object().clone())
            });
            self.jni_ref().call_method(
                &raw_val_1,
                "add",
                "(Ljava/lang/Object;)Z",
                vec![jni::objects::JValueGen::from(map_val_0)],
            )?;
        }
        let val_1 = jni::objects::JValueGen::Object(raw_val_1);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setWith",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
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
    // SUPER CLASS: BaseComponent
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
        let temp_clone = crate::bungee::api::chat::BaseComponent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::bungee::api::chat::BaseComponent = temp_clone.into();
        real.retain(arg0)
    }
    pub fn to_legacy_text_with_base_components(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: std::option::Option<Vec<crate::bungee::api::chat::BaseComponent<'mc>>>,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "[Lnet/md_5/bungee/api/chat/BaseComponent;";
            let arr = jni.new_object_array(
                a.len() as i32,
                "net/md_5/bungee/api/chat/BaseComponent",
                jni::objects::JObject::null(),
            );
            let arr = jni.translate_error_no_gen(arr)?;
            for i in 0..a.len() {
                let val_1 = jni::objects::JValueGen::Object(unsafe {
                    jni::objects::JObject::from_raw(a.get(i).unwrap().jni_object().clone())
                });
                jni.set_object_array_element(&arr, i as i32, val_1.l()?)?;
            }
            let val_1 = jni::objects::JValueGen::Object(arr);
            args.push(val_1.l()?.into());
        }
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
        arg0: std::option::Option<Vec<crate::bungee::api::chat::BaseComponent<'mc>>>,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "[Lnet/md_5/bungee/api/chat/BaseComponent;";
            let arr = jni.new_object_array(
                a.len() as i32,
                "net/md_5/bungee/api/chat/BaseComponent",
                jni::objects::JObject::null(),
            );
            let arr = jni.translate_error_no_gen(arr)?;
            for i in 0..a.len() {
                let val_1 = jni::objects::JValueGen::Object(unsafe {
                    jni::objects::JObject::from_raw(a.get(i).unwrap().jni_object().clone())
                });
                jni.set_object_array_element(&arr, i as i32, val_1.l()?)?;
            }
            let val_1 = jni::objects::JValueGen::Object(arr);
            args.push(val_1.l()?.into());
        }
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
        let temp_clone = crate::bungee::api::chat::BaseComponent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::bungee::api::chat::BaseComponent = temp_clone.into();
        real.color_raw()
    }
    pub fn insertion(&self) -> Result<String, Box<dyn std::error::Error>> {
        let temp_clone = crate::bungee::api::chat::BaseComponent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::bungee::api::chat::BaseComponent = temp_clone.into();
        real.insertion()
    }
    pub fn click_event(
        &self,
    ) -> Result<crate::bungee::api::chat::ClickEvent<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::bungee::api::chat::BaseComponent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::bungee::api::chat::BaseComponent = temp_clone.into();
        real.click_event()
    }
    pub fn hover_event(
        &self,
    ) -> Result<crate::bungee::api::chat::HoverEvent<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::bungee::api::chat::BaseComponent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::bungee::api::chat::BaseComponent = temp_clone.into();
        real.hover_event()
    }
    pub fn is_obfuscated(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::bungee::api::chat::BaseComponent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::bungee::api::chat::BaseComponent = temp_clone.into();
        real.is_obfuscated()
    }
    pub fn is_obfuscated_raw(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::bungee::api::chat::BaseComponent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::bungee::api::chat::BaseComponent = temp_clone.into();
        real.is_obfuscated_raw()
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
        let temp_clone = crate::bungee::api::chat::BaseComponent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::bungee::api::chat::BaseComponent = temp_clone.into();
        real.has_formatting()
    }
    pub fn set_font(&self, arg0: impl Into<String>) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::bungee::api::chat::BaseComponent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::bungee::api::chat::BaseComponent = temp_clone.into();
        real.set_font(arg0)
    }
    pub fn set_bold(&self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::bungee::api::chat::BaseComponent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::bungee::api::chat::BaseComponent = temp_clone.into();
        real.set_bold(arg0)
    }
    pub fn set_italic(&self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::bungee::api::chat::BaseComponent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::bungee::api::chat::BaseComponent = temp_clone.into();
        real.set_italic(arg0)
    }
    pub fn set_underlined(&self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::bungee::api::chat::BaseComponent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::bungee::api::chat::BaseComponent = temp_clone.into();
        real.set_underlined(arg0)
    }
    pub fn set_strikethrough(&self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::bungee::api::chat::BaseComponent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::bungee::api::chat::BaseComponent = temp_clone.into();
        real.set_strikethrough(arg0)
    }
    pub fn set_obfuscated(&self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::bungee::api::chat::BaseComponent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::bungee::api::chat::BaseComponent = temp_clone.into();
        real.set_obfuscated(arg0)
    }
    pub fn set_insertion(&self, arg0: impl Into<String>) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::bungee::api::chat::BaseComponent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::bungee::api::chat::BaseComponent = temp_clone.into();
        real.set_insertion(arg0)
    }
    pub fn set_click_event(
        &self,
        arg0: impl Into<crate::bungee::api::chat::ClickEvent<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::bungee::api::chat::BaseComponent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::bungee::api::chat::BaseComponent = temp_clone.into();
        real.set_click_event(arg0)
    }
    pub fn set_hover_event(
        &self,
        arg0: impl Into<crate::bungee::api::chat::HoverEvent<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::bungee::api::chat::BaseComponent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::bungee::api::chat::BaseComponent = temp_clone.into();
        real.set_hover_event(arg0)
    }
    pub fn set_reset(&self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::bungee::api::chat::BaseComponent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::bungee::api::chat::BaseComponent = temp_clone.into();
        real.set_reset(arg0)
    }
    pub fn font(&self) -> Result<String, Box<dyn std::error::Error>> {
        let temp_clone = crate::bungee::api::chat::BaseComponent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::bungee::api::chat::BaseComponent = temp_clone.into();
        real.font()
    }
    pub fn font_raw(&self) -> Result<String, Box<dyn std::error::Error>> {
        let temp_clone = crate::bungee::api::chat::BaseComponent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::bungee::api::chat::BaseComponent = temp_clone.into();
        real.font_raw()
    }
    pub fn is_bold(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::bungee::api::chat::BaseComponent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::bungee::api::chat::BaseComponent = temp_clone.into();
        real.is_bold()
    }
    pub fn is_bold_raw(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::bungee::api::chat::BaseComponent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::bungee::api::chat::BaseComponent = temp_clone.into();
        real.is_bold_raw()
    }
    pub fn is_italic(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::bungee::api::chat::BaseComponent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::bungee::api::chat::BaseComponent = temp_clone.into();
        real.is_italic()
    }
    pub fn is_italic_raw(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::bungee::api::chat::BaseComponent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::bungee::api::chat::BaseComponent = temp_clone.into();
        real.is_italic_raw()
    }
    pub fn is_underlined(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::bungee::api::chat::BaseComponent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::bungee::api::chat::BaseComponent = temp_clone.into();
        real.is_underlined()
    }
    pub fn is_underlined_raw(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::bungee::api::chat::BaseComponent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::bungee::api::chat::BaseComponent = temp_clone.into();
        real.is_underlined_raw()
    }
    pub fn is_strikethrough(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::bungee::api::chat::BaseComponent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::bungee::api::chat::BaseComponent = temp_clone.into();
        real.is_strikethrough()
    }
    pub fn is_strikethrough_raw(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::bungee::api::chat::BaseComponent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::bungee::api::chat::BaseComponent = temp_clone.into();
        real.is_strikethrough_raw()
    }
    pub fn duplicate_without_formatting(
        &self,
    ) -> Result<crate::bungee::api::chat::BaseComponent<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::bungee::api::chat::BaseComponent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::bungee::api::chat::BaseComponent = temp_clone.into();
        real.duplicate_without_formatting()
    }
    pub fn set_extra(
        &self,
        arg0: Vec<impl Into<crate::bungee::api::chat::BaseComponent<'mc>>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::bungee::api::chat::BaseComponent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::bungee::api::chat::BaseComponent = temp_clone.into();
        real.set_extra(arg0)
    }
    pub fn extra(
        &self,
    ) -> Result<Vec<crate::bungee::api::chat::BaseComponent<'mc>>, Box<dyn std::error::Error>> {
        let temp_clone = crate::bungee::api::chat::BaseComponent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::bungee::api::chat::BaseComponent = temp_clone.into();
        real.extra()
    }
    pub fn set_color(
        &self,
        arg0: impl Into<crate::bungee::api::ChatColor<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::bungee::api::chat::BaseComponent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::bungee::api::chat::BaseComponent = temp_clone.into();
        real.set_color(arg0)
    }
    pub fn color(&self) -> Result<crate::bungee::api::ChatColor<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::bungee::api::chat::BaseComponent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::bungee::api::chat::BaseComponent = temp_clone.into();
        real.color()
    }
    pub fn is_reset(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::bungee::api::chat::BaseComponent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::bungee::api::chat::BaseComponent = temp_clone.into();
        real.is_reset()
    }
    // SUPER CLASS: Object

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
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

///
/// This is a representation of an abstract class.
#[repr(C)]
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

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}

#[repr(C)]
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
    // SUPER CLASS: Object

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
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

#[repr(C)]
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
    // SUPER CLASS: BaseComponent
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
        let temp_clone = crate::bungee::api::chat::BaseComponent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::bungee::api::chat::BaseComponent = temp_clone.into();
        real.retain(arg0)
    }
    pub fn to_legacy_text_with_base_components(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: std::option::Option<Vec<crate::bungee::api::chat::BaseComponent<'mc>>>,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "[Lnet/md_5/bungee/api/chat/BaseComponent;";
            let arr = jni.new_object_array(
                a.len() as i32,
                "net/md_5/bungee/api/chat/BaseComponent",
                jni::objects::JObject::null(),
            );
            let arr = jni.translate_error_no_gen(arr)?;
            for i in 0..a.len() {
                let val_1 = jni::objects::JValueGen::Object(unsafe {
                    jni::objects::JObject::from_raw(a.get(i).unwrap().jni_object().clone())
                });
                jni.set_object_array_element(&arr, i as i32, val_1.l()?)?;
            }
            let val_1 = jni::objects::JValueGen::Object(arr);
            args.push(val_1.l()?.into());
        }
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
        arg0: std::option::Option<Vec<crate::bungee::api::chat::BaseComponent<'mc>>>,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "[Lnet/md_5/bungee/api/chat/BaseComponent;";
            let arr = jni.new_object_array(
                a.len() as i32,
                "net/md_5/bungee/api/chat/BaseComponent",
                jni::objects::JObject::null(),
            );
            let arr = jni.translate_error_no_gen(arr)?;
            for i in 0..a.len() {
                let val_1 = jni::objects::JValueGen::Object(unsafe {
                    jni::objects::JObject::from_raw(a.get(i).unwrap().jni_object().clone())
                });
                jni.set_object_array_element(&arr, i as i32, val_1.l()?)?;
            }
            let val_1 = jni::objects::JValueGen::Object(arr);
            args.push(val_1.l()?.into());
        }
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
        let temp_clone = crate::bungee::api::chat::BaseComponent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::bungee::api::chat::BaseComponent = temp_clone.into();
        real.color_raw()
    }
    pub fn insertion(&self) -> Result<String, Box<dyn std::error::Error>> {
        let temp_clone = crate::bungee::api::chat::BaseComponent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::bungee::api::chat::BaseComponent = temp_clone.into();
        real.insertion()
    }
    pub fn click_event(
        &self,
    ) -> Result<crate::bungee::api::chat::ClickEvent<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::bungee::api::chat::BaseComponent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::bungee::api::chat::BaseComponent = temp_clone.into();
        real.click_event()
    }
    pub fn hover_event(
        &self,
    ) -> Result<crate::bungee::api::chat::HoverEvent<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::bungee::api::chat::BaseComponent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::bungee::api::chat::BaseComponent = temp_clone.into();
        real.hover_event()
    }
    pub fn is_obfuscated(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::bungee::api::chat::BaseComponent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::bungee::api::chat::BaseComponent = temp_clone.into();
        real.is_obfuscated()
    }
    pub fn is_obfuscated_raw(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::bungee::api::chat::BaseComponent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::bungee::api::chat::BaseComponent = temp_clone.into();
        real.is_obfuscated_raw()
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
        let temp_clone = crate::bungee::api::chat::BaseComponent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::bungee::api::chat::BaseComponent = temp_clone.into();
        real.has_formatting()
    }
    pub fn set_font(&self, arg0: impl Into<String>) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::bungee::api::chat::BaseComponent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::bungee::api::chat::BaseComponent = temp_clone.into();
        real.set_font(arg0)
    }
    pub fn set_bold(&self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::bungee::api::chat::BaseComponent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::bungee::api::chat::BaseComponent = temp_clone.into();
        real.set_bold(arg0)
    }
    pub fn set_italic(&self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::bungee::api::chat::BaseComponent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::bungee::api::chat::BaseComponent = temp_clone.into();
        real.set_italic(arg0)
    }
    pub fn set_underlined(&self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::bungee::api::chat::BaseComponent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::bungee::api::chat::BaseComponent = temp_clone.into();
        real.set_underlined(arg0)
    }
    pub fn set_strikethrough(&self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::bungee::api::chat::BaseComponent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::bungee::api::chat::BaseComponent = temp_clone.into();
        real.set_strikethrough(arg0)
    }
    pub fn set_obfuscated(&self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::bungee::api::chat::BaseComponent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::bungee::api::chat::BaseComponent = temp_clone.into();
        real.set_obfuscated(arg0)
    }
    pub fn set_insertion(&self, arg0: impl Into<String>) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::bungee::api::chat::BaseComponent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::bungee::api::chat::BaseComponent = temp_clone.into();
        real.set_insertion(arg0)
    }
    pub fn set_click_event(
        &self,
        arg0: impl Into<crate::bungee::api::chat::ClickEvent<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::bungee::api::chat::BaseComponent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::bungee::api::chat::BaseComponent = temp_clone.into();
        real.set_click_event(arg0)
    }
    pub fn set_hover_event(
        &self,
        arg0: impl Into<crate::bungee::api::chat::HoverEvent<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::bungee::api::chat::BaseComponent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::bungee::api::chat::BaseComponent = temp_clone.into();
        real.set_hover_event(arg0)
    }
    pub fn set_reset(&self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::bungee::api::chat::BaseComponent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::bungee::api::chat::BaseComponent = temp_clone.into();
        real.set_reset(arg0)
    }
    pub fn font(&self) -> Result<String, Box<dyn std::error::Error>> {
        let temp_clone = crate::bungee::api::chat::BaseComponent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::bungee::api::chat::BaseComponent = temp_clone.into();
        real.font()
    }
    pub fn font_raw(&self) -> Result<String, Box<dyn std::error::Error>> {
        let temp_clone = crate::bungee::api::chat::BaseComponent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::bungee::api::chat::BaseComponent = temp_clone.into();
        real.font_raw()
    }
    pub fn is_bold(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::bungee::api::chat::BaseComponent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::bungee::api::chat::BaseComponent = temp_clone.into();
        real.is_bold()
    }
    pub fn is_bold_raw(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::bungee::api::chat::BaseComponent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::bungee::api::chat::BaseComponent = temp_clone.into();
        real.is_bold_raw()
    }
    pub fn is_italic(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::bungee::api::chat::BaseComponent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::bungee::api::chat::BaseComponent = temp_clone.into();
        real.is_italic()
    }
    pub fn is_italic_raw(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::bungee::api::chat::BaseComponent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::bungee::api::chat::BaseComponent = temp_clone.into();
        real.is_italic_raw()
    }
    pub fn is_underlined(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::bungee::api::chat::BaseComponent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::bungee::api::chat::BaseComponent = temp_clone.into();
        real.is_underlined()
    }
    pub fn is_underlined_raw(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::bungee::api::chat::BaseComponent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::bungee::api::chat::BaseComponent = temp_clone.into();
        real.is_underlined_raw()
    }
    pub fn is_strikethrough(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::bungee::api::chat::BaseComponent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::bungee::api::chat::BaseComponent = temp_clone.into();
        real.is_strikethrough()
    }
    pub fn is_strikethrough_raw(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::bungee::api::chat::BaseComponent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::bungee::api::chat::BaseComponent = temp_clone.into();
        real.is_strikethrough_raw()
    }
    pub fn duplicate_without_formatting(
        &self,
    ) -> Result<crate::bungee::api::chat::BaseComponent<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::bungee::api::chat::BaseComponent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::bungee::api::chat::BaseComponent = temp_clone.into();
        real.duplicate_without_formatting()
    }
    pub fn set_extra(
        &self,
        arg0: Vec<impl Into<crate::bungee::api::chat::BaseComponent<'mc>>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::bungee::api::chat::BaseComponent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::bungee::api::chat::BaseComponent = temp_clone.into();
        real.set_extra(arg0)
    }
    pub fn extra(
        &self,
    ) -> Result<Vec<crate::bungee::api::chat::BaseComponent<'mc>>, Box<dyn std::error::Error>> {
        let temp_clone = crate::bungee::api::chat::BaseComponent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::bungee::api::chat::BaseComponent = temp_clone.into();
        real.extra()
    }
    pub fn set_color(
        &self,
        arg0: impl Into<crate::bungee::api::ChatColor<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::bungee::api::chat::BaseComponent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::bungee::api::chat::BaseComponent = temp_clone.into();
        real.set_color(arg0)
    }
    pub fn color(&self) -> Result<crate::bungee::api::ChatColor<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::bungee::api::chat::BaseComponent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::bungee::api::chat::BaseComponent = temp_clone.into();
        real.color()
    }
    pub fn is_reset(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::bungee::api::chat::BaseComponent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::bungee::api::chat::BaseComponent = temp_clone.into();
        real.is_reset()
    }
    // SUPER CLASS: Object

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
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

#[repr(C)]
pub struct ComponentBuilder<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
pub enum ComponentBuilderFormatRetention<'mc> {
    None {
        inner: ComponentBuilderFormatRetentionStruct<'mc>,
    },
    Formatting {
        inner: ComponentBuilderFormatRetentionStruct<'mc>,
    },
    Events {
        inner: ComponentBuilderFormatRetentionStruct<'mc>,
    },
    All {
        inner: ComponentBuilderFormatRetentionStruct<'mc>,
    },
}
impl<'mc> std::fmt::Display for ComponentBuilderFormatRetention<'mc> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ComponentBuilderFormatRetention::None { .. } => f.write_str("NONE"),
            ComponentBuilderFormatRetention::Formatting { .. } => f.write_str("FORMATTING"),
            ComponentBuilderFormatRetention::Events { .. } => f.write_str("EVENTS"),
            ComponentBuilderFormatRetention::All { .. } => f.write_str("ALL"),
        }
    }
}

impl<'mc> ComponentBuilderFormatRetention<'mc> {
    pub fn value_of(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<String>,
    ) -> Result<ComponentBuilderFormatRetention<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(env.new_string(arg0.into())?);
        let cls = env.find_class("net/md_5/bungee/api/chat/ComponentBuilder$FormatRetention");
        let cls = env.translate_error_with_class(cls)?;
        let res = env.call_static_method(
            cls,
            "valueOf",
            "(Ljava/lang/String;)Lnet/md_5/bungee/api/chat/ComponentBuilder$FormatRetention;",
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
            "NONE" => Ok(ComponentBuilderFormatRetention::None {
                inner: ComponentBuilderFormatRetentionStruct::from_raw(env, obj)?,
            }),
            "FORMATTING" => Ok(ComponentBuilderFormatRetention::Formatting {
                inner: ComponentBuilderFormatRetentionStruct::from_raw(env, obj)?,
            }),
            "EVENTS" => Ok(ComponentBuilderFormatRetention::Events {
                inner: ComponentBuilderFormatRetentionStruct::from_raw(env, obj)?,
            }),
            "ALL" => Ok(ComponentBuilderFormatRetention::All {
                inner: ComponentBuilderFormatRetentionStruct::from_raw(env, obj)?,
            }),

            _ => Err(eyre::eyre!("String gaven for variant was invalid").into()),
        }
    }
}

#[repr(C)]
pub struct ComponentBuilderFormatRetentionStruct<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for ComponentBuilderFormatRetention<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        match self {
            Self::None { inner } => inner.0.clone(),
            Self::Formatting { inner } => inner.0.clone(),
            Self::Events { inner } => inner.0.clone(),
            Self::All { inner } => inner.0.clone(),
        }
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        match self {
            Self::None { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Formatting { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::Events { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::All { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
        }
    }
}
impl<'mc> JNIInstantiatable<'mc> for ComponentBuilderFormatRetention<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
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
            let variant = env.call_method(&obj, "toString", "()Ljava/lang/String;", vec![]);
            let variant = env.translate_error(variant)?;
            let variant_str = env
                .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
                .to_string_lossy()
                .to_string();
            match variant_str.as_str() {
                "NONE" => Ok(ComponentBuilderFormatRetention::None {
                    inner: ComponentBuilderFormatRetentionStruct::from_raw(env, obj)?,
                }),
                "FORMATTING" => Ok(ComponentBuilderFormatRetention::Formatting {
                    inner: ComponentBuilderFormatRetentionStruct::from_raw(env, obj)?,
                }),
                "EVENTS" => Ok(ComponentBuilderFormatRetention::Events {
                    inner: ComponentBuilderFormatRetentionStruct::from_raw(env, obj)?,
                }),
                "ALL" => Ok(ComponentBuilderFormatRetention::All {
                    inner: ComponentBuilderFormatRetentionStruct::from_raw(env, obj)?,
                }),
                _ => Err(eyre::eyre!("String gaven for variant was invalid").into()),
            }
        }
    }
}

impl<'mc> JNIRaw<'mc> for ComponentBuilderFormatRetentionStruct<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for ComponentBuilderFormatRetentionStruct<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate ComponentBuilderFormatRetentionStruct from null object."
            )
            .into());
        }
        let (valid, name) = env.validate_name(
            &obj,
            "net/md_5/bungee/api/chat/ComponentBuilder$FormatRetention",
        )?;
        if !valid {
            Err(eyre::eyre!(
                    "Invalid argument passed. Expected a ComponentBuilderFormatRetentionStruct object, got {}",
                    name
                )
                .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> ComponentBuilderFormatRetentionStruct<'mc> {
    pub fn values(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<
        Vec<crate::bungee::api::chat::ComponentBuilderFormatRetention<'mc>>,
        Box<dyn std::error::Error>,
    > {
        let sig = String::from("()Lnet/md_5/bungee/api/chat/ComponentBuilder$FormatRetention;");
        let cls = jni.find_class("net/md_5/bungee/api/chat/ComponentBuilder$FormatRetention");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "values", sig.as_str(), vec![]);
        let res = jni.translate_error(res)?;
        let arr = Into::<jni::objects::JObjectArray>::into(res.l()?);
        let len = jni.get_array_length(&arr)?;
        let mut vec = Vec::new();
        for i in 0..len {
            let res = jni.get_object_array_element(&arr, i)?;
            vec.push({
                crate::bungee::api::chat::ComponentBuilderFormatRetention::from_raw(&jni, res)?
            });
        }
        Ok(vec)
    }
    // SUPER CLASS: Enum

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
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
    pub fn new_with_string(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: std::option::Option<impl Into<String>>,
    ) -> Result<crate::bungee::api::chat::ComponentBuilder<'mc>, Box<dyn std::error::Error>> {
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
        let cls = jni.find_class("net/md_5/bungee/api/chat/ComponentBuilder");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(cls, sig.as_str(), args);
        let res = jni.translate_error_no_gen(res)?;
        crate::bungee::api::chat::ComponentBuilder::from_raw(&jni, res)
    }
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

    pub fn set_cursor(
        &self,
        arg0: i32,
    ) -> Result<crate::bungee::api::chat::ComponentBuilder<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(I)Lnet/md_5/bungee/api/chat/ComponentBuilder;");
        let val_1 = jni::objects::JValueGen::Int(arg0);
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

    pub fn remove_component(&self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(I)V");
        let val_1 = jni::objects::JValueGen::Int(arg0);
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
        let val_1 = jni::objects::JValueGen::Int(arg0);
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

    pub fn append_with_base_components(
        &self,
        arg0: Vec<crate::bungee::api::chat::BaseComponent<'mc>>,
        arg1: std::option::Option<
            impl Into<crate::bungee::api::chat::ComponentBuilderFormatRetention<'mc>>,
        >,
    ) -> Result<crate::bungee::api::chat::ComponentBuilder<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "[Lnet/md_5/bungee/api/chat/BaseComponent;";
        let arr = self.jni_ref().new_object_array(
            arg0.len() as i32,
            "net/md_5/bungee/api/chat/BaseComponent",
            jni::objects::JObject::null(),
        );
        let arr = self.jni_ref().translate_error_no_gen(arr)?;
        for i in 0..arg0.len() {
            let val_1 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(arg0.get(i).unwrap().jni_object().clone())
            });
            self.jni_ref()
                .set_object_array_element(&arr, i as i32, val_1.l()?)?;
        }
        let val_1 = jni::objects::JValueGen::Object(arr);
        args.push(val_1.l()?.into());
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

    pub fn event_with_click_event(
        &self,
        arg0: impl Into<crate::bungee::api::chat::ClickEvent<'mc>>,
    ) -> Result<crate::bungee::api::chat::ComponentBuilder<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lnet/md_5/bungee/api/chat/ClickEvent;";
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

    pub fn create(
        &self,
    ) -> Result<Vec<crate::bungee::api::chat::BaseComponent<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lnet/md_5/bungee/api/chat/BaseComponent;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "create", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        let arr = Into::<jni::objects::JObjectArray>::into(res.l()?);
        let len = self.jni_ref().get_array_length(&arr)?;
        let mut vec = Vec::new();
        for i in 0..len {
            let res = self.jni_ref().get_object_array_element(&arr, i)?;
            vec.push({ crate::bungee::api::chat::BaseComponent::from_raw(&self.jni_ref(), res)? });
        }
        Ok(vec)
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
    // SUPER CLASS: Object

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
pub mod hover;
