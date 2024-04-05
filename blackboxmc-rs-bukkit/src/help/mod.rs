#![allow(deprecated)]
use blackboxmc_general::JNIInstantiatable;
use blackboxmc_general::JNIRaw;
use color_eyre::eyre::Result;
#[repr(C)]
pub struct HelpTopicFactory<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for HelpTopicFactory<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for HelpTopicFactory<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate HelpTopicFactory from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/help/HelpTopicFactory")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a HelpTopicFactory object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> HelpTopicFactory<'mc> {
    pub fn from_extendable(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        plugin: &'mc crate::plugin::Plugin,
        address: i32,
        lib_name: String,
        name: String,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let obj = unsafe { plugin.new_extendable(address, "HelpTopicFactory", name, lib_name) }?;
        Self::from_raw(env, obj)
    }
    pub fn create_topic(
        &self,
        command: impl Into<crate::command::Command<'mc>>,
    ) -> Result<Option<crate::help::HelpTopic<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/command/Command;)Lcrate::help::HelpTopic;");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(command.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "createTopic",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {
            return Ok(None);
        }
        Ok(Some(crate::help::HelpTopic::from_raw(
            &self.jni_ref(),
            unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) },
        )?))
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
#[repr(C)]
pub struct HelpTopicComparator<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for HelpTopicComparator<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for HelpTopicComparator<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate HelpTopicComparator from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/help/HelpTopicComparator")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a HelpTopicComparator object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> HelpTopicComparator<'mc> {
    pub fn topic_name_comparator_instance(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<crate::help::HelpTopicComparatorTopicNameComparator<'mc>, Box<dyn std::error::Error>>
    {
        let sig = String::from("()Lcrate::help::HelpTopicComparatorTopicNameComparator;");
        let cls = jni.find_class("org/bukkit/help/HelpTopicComparator");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "topicNameComparatorInstance", sig.as_str(), vec![]);
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        crate::help::HelpTopicComparatorTopicNameComparator::from_raw(&jni, obj)
    }
    pub fn help_topic_comparator_instance(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<crate::help::HelpTopicComparator<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lcrate::help::HelpTopicComparator;");
        let cls = jni.find_class("org/bukkit/help/HelpTopicComparator");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "helpTopicComparatorInstance", sig.as_str(), vec![]);
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        crate::help::HelpTopicComparator::from_raw(&jni, obj)
    }
    pub fn compare(
        &self,
        lhs: impl Into<crate::help::HelpTopic<'mc>>,
        rhs: impl Into<crate::help::HelpTopic<'mc>>,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/help/HelpTopic;Lorg/bukkit/help/HelpTopic;)Li32;");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(lhs.into().jni_object().clone())
        });
        let val_2 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(rhs.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "compare",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    // SUPER CLASS: Object

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<blackboxmc_java::util::JavaComparator<'mc>> for HelpTopicComparator<'mc> {
    fn into(self) -> blackboxmc_java::util::JavaComparator<'mc> {
        blackboxmc_java::util::JavaComparator::from_raw(&self.jni_ref(), self.1).expect(
            "Error converting HelpTopicComparator into blackboxmc_java::util::JavaComparator",
        )
    }
}
#[repr(C)]
pub struct HelpTopicComparatorTopicNameComparator<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for HelpTopicComparatorTopicNameComparator<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for HelpTopicComparatorTopicNameComparator<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate HelpTopicComparatorTopicNameComparator from null object."
            )
            .into());
        }
        let (valid, name) = env.validate_name(
            &obj,
            "org/bukkit/help/HelpTopicComparator/TopicNameComparator",
        )?;
        if !valid {
            Err(eyre::eyre!(
                    "Invalid argument passed. Expected a HelpTopicComparatorTopicNameComparator object, got {}",
                    name
                )
                .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> HelpTopicComparatorTopicNameComparator<'mc> {
    pub fn compare(
        &self,
        lhs: impl Into<String>,
        rhs: impl Into<String>,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/String;Ljava/lang/String;)Li32;");
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(lhs.into())?,
        ));
        let val_2 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(rhs.into())?,
        ));
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "compare",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    // SUPER CLASS: Object

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<blackboxmc_java::util::JavaComparator<'mc>>
    for HelpTopicComparatorTopicNameComparator<'mc>
{
    fn into(self) -> blackboxmc_java::util::JavaComparator<'mc> {
        blackboxmc_java::util::JavaComparator::from_raw(&self.jni_ref(), self.1).expect("Error converting HelpTopicComparatorTopicNameComparator into blackboxmc_java::util::JavaComparator")
    }
}
#[repr(C)]
pub struct HelpTopic<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for HelpTopic<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for HelpTopic<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate HelpTopic from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/help/HelpTopic")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a HelpTopic object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> HelpTopic<'mc> {
    pub fn from_extendable(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        plugin: &'mc crate::plugin::Plugin,
        address: i32,
        lib_name: String,
        name: String,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let obj = unsafe { plugin.new_extendable(address, "HelpTopic", name, lib_name) }?;
        Self::from_raw(env, obj)
    }
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<crate::help::HelpTopic<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let cls = jni.find_class("org/bukkit/help/HelpTopic");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(cls, sig.as_str(), vec![]);
        let res = jni.translate_error_no_gen(res)?;
        crate::help::HelpTopic::from_raw(&jni, res)
    }
    pub fn amend_can_see(
        &self,
        amended_permission: impl Into<String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/String;)L();");
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(amended_permission.into())?,
        ));
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "amendCanSee",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub fn name(&self) -> Result<String, Box<dyn std::error::Error>> {
        let sig = String::from("()LString;");
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
    pub fn short_text(&self) -> Result<String, Box<dyn std::error::Error>> {
        let sig = String::from("()LString;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getShortText", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    pub fn get_full_text(
        &self,
        for_who: impl Into<crate::command::CommandSender<'mc>>,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/command/CommandSender;)LString;");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(for_who.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getFullText",
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
    pub fn amend_topic(
        &self,
        amended_short_text: impl Into<String>,
        amended_full_text: impl Into<String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/String;Ljava/lang/String;)L();");
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(amended_short_text.into())?,
        ));
        let val_2 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(amended_full_text.into())?,
        ));
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "amendTopic",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
            ],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    // SUPER CLASS: Object

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
#[repr(C)]
pub struct IndexHelpTopic<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for IndexHelpTopic<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for IndexHelpTopic<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate IndexHelpTopic from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/help/IndexHelpTopic")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a IndexHelpTopic object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> IndexHelpTopic<'mc> {
    pub fn can_see(
        &self,
        sender: impl Into<crate::command::CommandSender<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/command/CommandSender;)Lbool;");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(sender.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "canSee",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    pub fn amend_can_see(
        &self,
        amended_permission: impl Into<String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/String;)L();");
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(amended_permission.into())?,
        ));
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "amendCanSee",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub fn get_full_text(
        &self,
        sender: impl Into<crate::command::CommandSender<'mc>>,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/command/CommandSender;)LString;");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(sender.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getFullText",
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
    // SUPER CLASS: HelpTopic

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::help::HelpTopic<'mc>> for IndexHelpTopic<'mc> {
    fn into(self) -> crate::help::HelpTopic<'mc> {
        crate::help::HelpTopic::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting IndexHelpTopic into crate::help::HelpTopic")
    }
}
#[repr(C)]
pub struct GenericCommandHelpTopic<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for GenericCommandHelpTopic<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for GenericCommandHelpTopic<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate GenericCommandHelpTopic from null object."
            )
            .into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/help/GenericCommandHelpTopic")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a GenericCommandHelpTopic object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> GenericCommandHelpTopic<'mc> {
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        command: impl Into<crate::command::Command<'mc>>,
    ) -> Result<crate::help::GenericCommandHelpTopic<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/command/Command;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(command.into().jni_object().clone())
        });
        let cls = jni.find_class("org/bukkit/help/GenericCommandHelpTopic");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(
            cls,
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = jni.translate_error_no_gen(res)?;
        crate::help::GenericCommandHelpTopic::from_raw(&jni, res)
    }
    pub fn can_see(
        &self,
        sender: impl Into<crate::command::CommandSender<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/command/CommandSender;)Lbool;");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(sender.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "canSee",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    // SUPER CLASS: HelpTopic

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::help::HelpTopic<'mc>> for GenericCommandHelpTopic<'mc> {
    fn into(self) -> crate::help::HelpTopic<'mc> {
        crate::help::HelpTopic::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting GenericCommandHelpTopic into crate::help::HelpTopic")
    }
}
#[repr(C)]
pub struct HelpMap<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for HelpMap<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for HelpMap<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate HelpMap from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/help/HelpMap")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a HelpMap object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> HelpMap<'mc> {
    pub fn get_help_topic(
        &self,
        topic_name: impl Into<String>,
    ) -> Result<Option<crate::help::HelpTopic<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/String;)Lcrate::help::HelpTopic;");
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(topic_name.into())?,
        ));
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getHelpTopic",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {
            return Ok(None);
        }
        Ok(Some(crate::help::HelpTopic::from_raw(
            &self.jni_ref(),
            unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) },
        )?))
    }
    pub fn help_topics(
        &self,
    ) -> Result<Vec<jni::objects::JObject<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()LVec;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getHelpTopics", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let col = blackboxmc_java::util::JavaCollection::from_raw(&self.jni_ref(), res.l()?)?;
        let iter = col.iterator()?;
        while iter.has_next()? {
            let obj = iter.next()?;
            new_vec.push(obj);
        }
        Ok(new_vec)
    }
    pub fn add_topic(
        &self,
        topic: impl Into<crate::help::HelpTopic<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/help/HelpTopic;)L();");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(topic.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "addTopic",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub fn clear(&self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()L();");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "clear", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub fn register_help_topic_factory(
        &self,
        command_class: jni::objects::JClass<'mc>,
        factory: impl Into<crate::help::HelpTopicFactory<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/Class;Lorg/bukkit/help/HelpTopicFactory;)L();");
        let val_1 = jni::objects::JValueGen::Object(command_class.into());
        let val_2 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(factory.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "registerHelpTopicFactory",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
            ],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub fn ignored_plugins(
        &self,
    ) -> Result<Vec<jni::objects::JObject<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()LVec;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getIgnoredPlugins",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let list = blackboxmc_java::util::JavaList::from_raw(&self.jni_ref(), res.l()?)?;
        let iter = list.iterator()?;
        while iter.has_next()? {
            let obj = iter.next()?;
            new_vec.push(obj);
        }
        Ok(new_vec)
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
