#![allow(deprecated)]
use blackboxmc_general::JNIInstantiatable;
use blackboxmc_general::JNIRaw;
use color_eyre::eyre::Result;

#[repr(C)]
pub struct SelectorComponentSerializer<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for SelectorComponentSerializer<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for SelectorComponentSerializer<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate SelectorComponentSerializer from null object."
            )
            .into());
        }
        let (valid, name) =
            env.validate_name(&obj, "net/md_5/bungee/chat/SelectorComponentSerializer")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a SelectorComponentSerializer object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> SelectorComponentSerializer<'mc> {
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<crate::bungee::chat::SelectorComponentSerializer<'mc>, Box<dyn std::error::Error>>
    {
        let sig = String::from("()V");
        let cls = jni.find_class("net/md_5/bungee/chat/SelectorComponentSerializer");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(cls, sig.as_str(), vec![]);
        let res = jni.translate_error_no_gen(res)?;
        crate::bungee::chat::SelectorComponentSerializer::from_raw(&jni, res)
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
    ) -> Result<crate::bungee::api::chat::SelectorComponent<'mc>, Box<dyn std::error::Error>> {
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
        sig += ")Lnet/md_5/bungee/api/chat/SelectorComponent;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "deserialize", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        crate::bungee::api::chat::SelectorComponent::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    // SUPER CLASS: BaseComponentSerializer
    // SUPER CLASS: Object

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<jni::objects::JObject<'mc>> for SelectorComponentSerializer<'mc> {
    fn into(self) -> jni::objects::JObject<'mc> {
        self.1
    }
}
impl<'mc> Into<crate::bungee::chat::BaseComponentSerializer<'mc>>
    for SelectorComponentSerializer<'mc>
{
    fn into(self) -> crate::bungee::chat::BaseComponentSerializer<'mc> {
        crate::bungee::chat::BaseComponentSerializer::from_raw(&self.jni_ref(), self.1).expect("Error converting SelectorComponentSerializer into crate::bungee::chat::BaseComponentSerializer")
    }
}

#[repr(C)]
pub struct TextComponentSerializer<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for TextComponentSerializer<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for TextComponentSerializer<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate TextComponentSerializer from null object."
            )
            .into());
        }
        let (valid, name) =
            env.validate_name(&obj, "net/md_5/bungee/chat/TextComponentSerializer")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a TextComponentSerializer object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> TextComponentSerializer<'mc> {
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<crate::bungee::chat::TextComponentSerializer<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let cls = jni.find_class("net/md_5/bungee/chat/TextComponentSerializer");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(cls, sig.as_str(), vec![]);
        let res = jni.translate_error_no_gen(res)?;
        crate::bungee::chat::TextComponentSerializer::from_raw(&jni, res)
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
    ) -> Result<crate::bungee::api::chat::TextComponent<'mc>, Box<dyn std::error::Error>> {
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
        sig += ")Lnet/md_5/bungee/api/chat/TextComponent;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "deserialize", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        crate::bungee::api::chat::TextComponent::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    // SUPER CLASS: BaseComponentSerializer
    // SUPER CLASS: Object

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<jni::objects::JObject<'mc>> for TextComponentSerializer<'mc> {
    fn into(self) -> jni::objects::JObject<'mc> {
        self.1
    }
}
impl<'mc> Into<crate::bungee::chat::BaseComponentSerializer<'mc>> for TextComponentSerializer<'mc> {
    fn into(self) -> crate::bungee::chat::BaseComponentSerializer<'mc> {
        crate::bungee::chat::BaseComponentSerializer::from_raw(&self.jni_ref(), self.1).expect("Error converting TextComponentSerializer into crate::bungee::chat::BaseComponentSerializer")
    }
}

#[repr(C)]
pub struct BaseComponentSerializer<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for BaseComponentSerializer<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for BaseComponentSerializer<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate BaseComponentSerializer from null object."
            )
            .into());
        }
        let (valid, name) =
            env.validate_name(&obj, "net/md_5/bungee/chat/BaseComponentSerializer")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a BaseComponentSerializer object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> BaseComponentSerializer<'mc> {
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<crate::bungee::chat::BaseComponentSerializer<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let cls = jni.find_class("net/md_5/bungee/chat/BaseComponentSerializer");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(cls, sig.as_str(), vec![]);
        let res = jni.translate_error_no_gen(res)?;
        crate::bungee::chat::BaseComponentSerializer::from_raw(&jni, res)
    }
    // SUPER CLASS: Object

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}

#[repr(C)]
pub struct ScoreComponentSerializer<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for ScoreComponentSerializer<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for ScoreComponentSerializer<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate ScoreComponentSerializer from null object."
            )
            .into());
        }
        let (valid, name) =
            env.validate_name(&obj, "net/md_5/bungee/chat/ScoreComponentSerializer")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a ScoreComponentSerializer object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> ScoreComponentSerializer<'mc> {
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<crate::bungee::chat::ScoreComponentSerializer<'mc>, Box<dyn std::error::Error>>
    {
        let sig = String::from("()V");
        let cls = jni.find_class("net/md_5/bungee/chat/ScoreComponentSerializer");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(cls, sig.as_str(), vec![]);
        let res = jni.translate_error_no_gen(res)?;
        crate::bungee::chat::ScoreComponentSerializer::from_raw(&jni, res)
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
    ) -> Result<crate::bungee::api::chat::ScoreComponent<'mc>, Box<dyn std::error::Error>> {
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
        sig += ")Lnet/md_5/bungee/api/chat/ScoreComponent;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "deserialize", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        crate::bungee::api::chat::ScoreComponent::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    // SUPER CLASS: BaseComponentSerializer
    // SUPER CLASS: Object

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<jni::objects::JObject<'mc>> for ScoreComponentSerializer<'mc> {
    fn into(self) -> jni::objects::JObject<'mc> {
        self.1
    }
}
impl<'mc> Into<crate::bungee::chat::BaseComponentSerializer<'mc>>
    for ScoreComponentSerializer<'mc>
{
    fn into(self) -> crate::bungee::chat::BaseComponentSerializer<'mc> {
        crate::bungee::chat::BaseComponentSerializer::from_raw(&self.jni_ref(), self.1).expect("Error converting ScoreComponentSerializer into crate::bungee::chat::BaseComponentSerializer")
    }
}

#[repr(C)]
pub struct TranslatableComponentSerializer<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for TranslatableComponentSerializer<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for TranslatableComponentSerializer<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate TranslatableComponentSerializer from null object."
            )
            .into());
        }
        let (valid, name) =
            env.validate_name(&obj, "net/md_5/bungee/chat/TranslatableComponentSerializer")?;
        if !valid {
            Err(eyre::eyre!(
                    "Invalid argument passed. Expected a TranslatableComponentSerializer object, got {}",
                    name
                )
                .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> TranslatableComponentSerializer<'mc> {
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<crate::bungee::chat::TranslatableComponentSerializer<'mc>, Box<dyn std::error::Error>>
    {
        let sig = String::from("()V");
        let cls = jni.find_class("net/md_5/bungee/chat/TranslatableComponentSerializer");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(cls, sig.as_str(), vec![]);
        let res = jni.translate_error_no_gen(res)?;
        crate::bungee::chat::TranslatableComponentSerializer::from_raw(&jni, res)
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
    ) -> Result<crate::bungee::api::chat::TranslatableComponent<'mc>, Box<dyn std::error::Error>>
    {
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
        sig += ")Lnet/md_5/bungee/api/chat/TranslatableComponent;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "deserialize", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        crate::bungee::api::chat::TranslatableComponent::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    // SUPER CLASS: BaseComponentSerializer
    // SUPER CLASS: Object

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<jni::objects::JObject<'mc>> for TranslatableComponentSerializer<'mc> {
    fn into(self) -> jni::objects::JObject<'mc> {
        self.1
    }
}
impl<'mc> Into<crate::bungee::chat::BaseComponentSerializer<'mc>>
    for TranslatableComponentSerializer<'mc>
{
    fn into(self) -> crate::bungee::chat::BaseComponentSerializer<'mc> {
        crate::bungee::chat::BaseComponentSerializer::from_raw(&self.jni_ref(), self.1).expect("Error converting TranslatableComponentSerializer into crate::bungee::chat::BaseComponentSerializer")
    }
}

#[repr(C)]
pub struct KeybindComponentSerializer<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for KeybindComponentSerializer<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for KeybindComponentSerializer<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate KeybindComponentSerializer from null object."
            )
            .into());
        }
        let (valid, name) =
            env.validate_name(&obj, "net/md_5/bungee/chat/KeybindComponentSerializer")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a KeybindComponentSerializer object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> KeybindComponentSerializer<'mc> {
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<crate::bungee::chat::KeybindComponentSerializer<'mc>, Box<dyn std::error::Error>>
    {
        let sig = String::from("()V");
        let cls = jni.find_class("net/md_5/bungee/chat/KeybindComponentSerializer");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(cls, sig.as_str(), vec![]);
        let res = jni.translate_error_no_gen(res)?;
        crate::bungee::chat::KeybindComponentSerializer::from_raw(&jni, res)
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
    ) -> Result<crate::bungee::api::chat::KeybindComponent<'mc>, Box<dyn std::error::Error>> {
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
        sig += ")Lnet/md_5/bungee/api/chat/KeybindComponent;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "deserialize", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        crate::bungee::api::chat::KeybindComponent::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    // SUPER CLASS: BaseComponentSerializer
    // SUPER CLASS: Object

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<jni::objects::JObject<'mc>> for KeybindComponentSerializer<'mc> {
    fn into(self) -> jni::objects::JObject<'mc> {
        self.1
    }
}
impl<'mc> Into<crate::bungee::chat::BaseComponentSerializer<'mc>>
    for KeybindComponentSerializer<'mc>
{
    fn into(self) -> crate::bungee::chat::BaseComponentSerializer<'mc> {
        crate::bungee::chat::BaseComponentSerializer::from_raw(&self.jni_ref(), self.1).expect("Error converting KeybindComponentSerializer into crate::bungee::chat::BaseComponentSerializer")
    }
}

#[repr(C)]
pub struct TranslationRegistry<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for TranslationRegistry<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for TranslationRegistry<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate TranslationRegistry from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "net/md_5/bungee/chat/TranslationRegistry")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a TranslationRegistry object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> TranslationRegistry<'mc> {
    pub fn translate(&self, arg0: impl Into<String>) -> Result<String, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/String;)Ljava/lang/String;");
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(arg0.into())?,
        ));
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "translate",
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

impl<'mc> std::string::ToString for TranslationRegistry<'mc> {
    fn to_string(&self) -> String {
        match &self.internal_to_string() {
            Ok(a) => a.clone(),
            Err(err) => format!("Error calling TranslationRegistry.toString: {}", err),
        }
    }
}

#[repr(C)]
pub struct ComponentSerializer<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for ComponentSerializer<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for ComponentSerializer<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate ComponentSerializer from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "net/md_5/bungee/chat/ComponentSerializer")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a ComponentSerializer object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> ComponentSerializer<'mc> {
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<crate::bungee::chat::ComponentSerializer<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let cls = jni.find_class("net/md_5/bungee/chat/ComponentSerializer");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(cls, sig.as_str(), vec![]);
        let res = jni.translate_error_no_gen(res)?;
        crate::bungee::chat::ComponentSerializer::from_raw(&jni, res)
    }

    pub fn deserialize_with_json_element(
        &self,
        arg0: jni::objects::JObject<'mc>,
        arg1: jni::objects::JObject<'mc>,
        arg2: jni::objects::JObject<'mc>,
    ) -> Result<crate::bungee::api::chat::BaseComponent<'mc>, Box<dyn std::error::Error>> {
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
        sig += ")Lnet/md_5/bungee/api/chat/BaseComponent;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "deserialize", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        crate::bungee::api::chat::BaseComponent::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn to_string_with_object(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Ljava/lang/Object;";
        let val_1 = jni::objects::JValueGen::Object(arg0);
        args.push(val_1);
        sig += ")Ljava/lang/String;";
        let cls = jni.find_class("java/lang/String");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "toString", sig.as_str(), args);
        let res = jni.translate_error(res)?;
        Ok(jni
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }

    pub fn to_string_with_base_components(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: Vec<crate::bungee::api::chat::BaseComponent<'mc>>,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "[Lnet/md_5/bungee/api/chat/BaseComponent;";
        let arr = jni.new_object_array(
            arg0.len() as i32,
            "net/md_5/bungee/api/chat/BaseComponent",
            jni::objects::JObject::null(),
        );
        let arr = jni.translate_error_no_gen(arr)?;
        for i in 0..arg0.len() {
            let val_1 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(arg0.get(i).unwrap().jni_object().clone())
            });
            jni.set_object_array_element(&arr, i as i32, val_1.l()?)?;
        }
        let val_1 = jni::objects::JValueGen::Object(arr);
        args.push(val_1.l()?.into());
        sig += ")Ljava/lang/String;";
        let cls = jni.find_class("java/lang/String");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "toString", sig.as_str(), args);
        let res = jni.translate_error(res)?;
        Ok(jni
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
impl<'mc> Into<jni::objects::JObject<'mc>> for ComponentSerializer<'mc> {
    fn into(self) -> jni::objects::JObject<'mc> {
        self.1
    }
}
