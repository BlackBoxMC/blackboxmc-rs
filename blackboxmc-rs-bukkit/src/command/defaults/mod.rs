#![allow(deprecated)]
use blackboxmc_general::JNIInstantiatable;
use blackboxmc_general::JNIRaw;
use color_eyre::eyre::Result;

#[repr(C)]
pub struct ReloadCommand<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for ReloadCommand<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for ReloadCommand<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate ReloadCommand from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/command/defaults/ReloadCommand")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a ReloadCommand object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> ReloadCommand<'mc> {
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<String>,
    ) -> Result<crate::command::defaults::ReloadCommand<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/String;)V");
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            jni.new_string(arg0.into())?,
        ));
        let cls = jni.find_class("org/bukkit/command/defaults/ReloadCommand");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(
            cls,
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = jni.translate_error_no_gen(res)?;
        crate::command::defaults::ReloadCommand::from_raw(&jni, res)
    }

    pub fn tab_complete(
        &self,
        arg0: impl Into<crate::command::CommandSender<'mc>>,
        arg1: impl Into<String>,
        arg2: Vec<String>,
    ) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/command/CommandSender;Ljava/lang/String;[Ljava/lang/String;)Ljava/util/List;");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let val_2 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(arg1.into())?,
        ));
        let arr = self.jni_ref().new_object_array(
            arg2.len() as i32,
            "java/lang/String",
            jni::objects::JObject::null(),
        );
        let arr = self.jni_ref().translate_error_no_gen(arr)?;
        for i in 0..arg2.len() {
            let val_3 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
                self.jni_ref().new_string(arg2.get(i).unwrap().clone())?,
            ));
            self.jni_ref()
                .set_object_array_element(&arr, i as i32, val_3.l()?)?;
        }
        let val_3 = jni::objects::JValueGen::Object(arr);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "tabComplete",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
                jni::objects::JValueGen::from(val_3.l()?),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let list = blackboxmc_java::util::JavaList::from_raw(&self.0, res.l()?)?;
        let iter = list.iterator()?;
        while iter.has_next()? {
            let obj = iter.next()?;
            new_vec.push(
                self.jni_ref()
                    .get_string(unsafe { &jni::objects::JString::from_raw(*obj) })?
                    .to_string_lossy()
                    .to_string(),
            );
        }
        Ok(new_vec)
    }

    pub fn execute(
        &self,
        arg0: impl Into<crate::command::CommandSender<'mc>>,
        arg1: impl Into<String>,
        arg2: Vec<String>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from(
            "(Lorg/bukkit/command/CommandSender;Ljava/lang/String;[Ljava/lang/String;)Z",
        );
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let val_2 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(arg1.into())?,
        ));
        let arr = self.jni_ref().new_object_array(
            arg2.len() as i32,
            "java/lang/String",
            jni::objects::JObject::null(),
        );
        let arr = self.jni_ref().translate_error_no_gen(arr)?;
        for i in 0..arg2.len() {
            let val_3 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
                self.jni_ref().new_string(arg2.get(i).unwrap().clone())?,
            ));
            self.jni_ref()
                .set_object_array_element(&arr, i as i32, val_3.l()?)?;
        }
        let val_3 = jni::objects::JValueGen::Object(arr);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "execute",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
                jni::objects::JValueGen::from(val_3.l()?),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    // SUPER CLASS: BukkitCommand
    // SUPER CLASS: Command
    pub fn permission(&self) -> Result<Option<String>, Box<dyn std::error::Error>> {
        let temp_clone = crate::command::Command::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::command::Command = temp_clone.into();
        real.permission()
    }
    pub fn set_permission(
        &self,
        arg0: impl Into<String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::command::Command::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::command::Command = temp_clone.into();
        real.set_permission(arg0)
    }
    pub fn unregister(
        &self,
        arg0: impl Into<crate::command::CommandMap<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::command::Command::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::command::Command = temp_clone.into();
        real.unregister(arg0)
    }
    pub fn description(&self) -> Result<String, Box<dyn std::error::Error>> {
        let temp_clone = crate::command::Command::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::command::Command = temp_clone.into();
        real.description()
    }
    pub fn set_description(
        &self,
        arg0: impl Into<String>,
    ) -> Result<crate::command::Command<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::command::Command::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::command::Command = temp_clone.into();
        real.set_description(arg0)
    }
    pub fn set_usage(
        &self,
        arg0: impl Into<String>,
    ) -> Result<crate::command::Command<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::command::Command::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::command::Command = temp_clone.into();
        real.set_usage(arg0)
    }
    pub fn set_aliases(
        &self,
        arg0: Vec<impl Into<String>>,
    ) -> Result<crate::command::Command<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/util/List;)Lorg/bukkit/command/Command;");
        let raw_val_1 = self
            .jni_ref()
            .new_object("java/util/ArrayList", "()V", vec![])?;
        for v in arg0 {
            let map_val_0 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
                self.jni_ref().new_string(v.into())?,
            ));
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
            "setAliases",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::command::Command::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    pub fn set_permission_message(
        &self,
        arg0: impl Into<String>,
    ) -> Result<crate::command::Command<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::command::Command::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::command::Command = temp_clone.into();
        real.set_permission_message(arg0)
    }
    pub fn aliases(&self) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/List;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getAliases", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let list = blackboxmc_java::util::JavaList::from_raw(&self.0, res.l()?)?;
        let iter = list.iterator()?;
        while iter.has_next()? {
            let obj = iter.next()?;
            new_vec.push(
                self.jni_ref()
                    .get_string(unsafe { &jni::objects::JString::from_raw(*obj) })?
                    .to_string_lossy()
                    .to_string(),
            );
        }
        Ok(new_vec)
    }
    pub fn set_label(&self, arg0: impl Into<String>) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::command::Command::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::command::Command = temp_clone.into();
        real.set_label(arg0)
    }
    pub fn label(&self) -> Result<String, Box<dyn std::error::Error>> {
        let temp_clone = crate::command::Command::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::command::Command = temp_clone.into();
        real.label()
    }
    pub fn test_permission_silent(
        &self,
        arg0: impl Into<crate::command::CommandSender<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::command::Command::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::command::Command = temp_clone.into();
        real.test_permission_silent(arg0)
    }
    pub fn test_permission(
        &self,
        arg0: impl Into<crate::command::CommandSender<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::command::Command::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::command::Command = temp_clone.into();
        real.test_permission(arg0)
    }
    pub fn permission_message(&self) -> Result<Option<String>, Box<dyn std::error::Error>> {
        let temp_clone = crate::command::Command::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::command::Command = temp_clone.into();
        real.permission_message()
    }
    pub fn usage(&self) -> Result<String, Box<dyn std::error::Error>> {
        let temp_clone = crate::command::Command::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::command::Command = temp_clone.into();
        real.usage()
    }
    pub fn broadcast_command_message_with_command_sender(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<crate::command::CommandSender<'mc>>,
        arg1: impl Into<String>,
        arg2: std::option::Option<bool>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/command/CommandSender;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        sig += "Ljava/lang/String;";
        let val_2 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            jni.new_string(arg1.into())?,
        ));
        args.push(val_2);
        if let Some(a) = arg2 {
            sig += "Z";
            let val_3 = jni::objects::JValueGen::Bool(a.into());
            args.push(val_3);
        }
        sig += ")V";
        let cls = jni.find_class("void");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "broadcastCommandMessage", sig.as_str(), args);
        jni.translate_error(res)?;
        Ok(())
    }
    pub fn name(&self) -> Result<String, Box<dyn std::error::Error>> {
        let temp_clone = crate::command::Command::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::command::Command = temp_clone.into();
        real.name()
    }
    #[doc(hidden)]
    pub fn internal_to_string(&self) -> Result<String, Box<dyn std::error::Error>> {
        let temp_clone = crate::command::Command::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::command::Command = temp_clone.into();
        real.internal_to_string()
    }
    pub fn is_registered(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::command::Command::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::command::Command = temp_clone.into();
        real.is_registered()
    }
    pub fn register(
        &self,
        arg0: impl Into<crate::command::CommandMap<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::command::Command::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::command::Command = temp_clone.into();
        real.register(arg0)
    }
    pub fn set_name(&self, arg0: impl Into<String>) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::command::Command::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::command::Command = temp_clone.into();
        real.set_name(arg0)
    }
    // SUPER CLASS: Object

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::command::defaults::BukkitCommand<'mc>> for ReloadCommand<'mc> {
    fn into(self) -> crate::command::defaults::BukkitCommand<'mc> {
        crate::command::defaults::BukkitCommand::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting ReloadCommand into crate::command::defaults::BukkitCommand")
    }
}

#[repr(C)]
pub struct TimingsCommand<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for TimingsCommand<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for TimingsCommand<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate TimingsCommand from null object.").into(),
            );
        }
        let (valid, name) =
            env.validate_name(&obj, "org/bukkit/command/defaults/TimingsCommand")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a TimingsCommand object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> TimingsCommand<'mc> {
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<String>,
    ) -> Result<crate::command::defaults::TimingsCommand<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/String;)V");
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            jni.new_string(arg0.into())?,
        ));
        let cls = jni.find_class("org/bukkit/command/defaults/TimingsCommand");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(
            cls,
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = jni.translate_error_no_gen(res)?;
        crate::command::defaults::TimingsCommand::from_raw(&jni, res)
    }

    pub fn tab_complete(
        &self,
        arg0: impl Into<crate::command::CommandSender<'mc>>,
        arg1: impl Into<String>,
        arg2: Vec<String>,
    ) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/command/CommandSender;Ljava/lang/String;[Ljava/lang/String;)Ljava/util/List;");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let val_2 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(arg1.into())?,
        ));
        let arr = self.jni_ref().new_object_array(
            arg2.len() as i32,
            "java/lang/String",
            jni::objects::JObject::null(),
        );
        let arr = self.jni_ref().translate_error_no_gen(arr)?;
        for i in 0..arg2.len() {
            let val_3 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
                self.jni_ref().new_string(arg2.get(i).unwrap().clone())?,
            ));
            self.jni_ref()
                .set_object_array_element(&arr, i as i32, val_3.l()?)?;
        }
        let val_3 = jni::objects::JValueGen::Object(arr);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "tabComplete",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
                jni::objects::JValueGen::from(val_3.l()?),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let list = blackboxmc_java::util::JavaList::from_raw(&self.0, res.l()?)?;
        let iter = list.iterator()?;
        while iter.has_next()? {
            let obj = iter.next()?;
            new_vec.push(
                self.jni_ref()
                    .get_string(unsafe { &jni::objects::JString::from_raw(*obj) })?
                    .to_string_lossy()
                    .to_string(),
            );
        }
        Ok(new_vec)
    }

    pub fn execute_spigot_timings(
        &self,
        arg0: impl Into<crate::command::CommandSender<'mc>>,
        arg1: Vec<String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/command/CommandSender;[Ljava/lang/String;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let arr = self.jni_ref().new_object_array(
            arg1.len() as i32,
            "java/lang/String",
            jni::objects::JObject::null(),
        );
        let arr = self.jni_ref().translate_error_no_gen(arr)?;
        for i in 0..arg1.len() {
            let val_2 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
                self.jni_ref().new_string(arg1.get(i).unwrap().clone())?,
            ));
            self.jni_ref()
                .set_object_array_element(&arr, i as i32, val_2.l()?)?;
        }
        let val_2 = jni::objects::JValueGen::Object(arr);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "executeSpigotTimings",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2.l()?),
            ],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn execute(
        &self,
        arg0: impl Into<crate::command::CommandSender<'mc>>,
        arg1: impl Into<String>,
        arg2: Vec<String>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from(
            "(Lorg/bukkit/command/CommandSender;Ljava/lang/String;[Ljava/lang/String;)Z",
        );
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let val_2 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(arg1.into())?,
        ));
        let arr = self.jni_ref().new_object_array(
            arg2.len() as i32,
            "java/lang/String",
            jni::objects::JObject::null(),
        );
        let arr = self.jni_ref().translate_error_no_gen(arr)?;
        for i in 0..arg2.len() {
            let val_3 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
                self.jni_ref().new_string(arg2.get(i).unwrap().clone())?,
            ));
            self.jni_ref()
                .set_object_array_element(&arr, i as i32, val_3.l()?)?;
        }
        let val_3 = jni::objects::JValueGen::Object(arr);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "execute",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
                jni::objects::JValueGen::from(val_3.l()?),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    // SUPER CLASS: BukkitCommand
    // SUPER CLASS: Command
    pub fn permission(&self) -> Result<Option<String>, Box<dyn std::error::Error>> {
        let temp_clone = crate::command::Command::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::command::Command = temp_clone.into();
        real.permission()
    }
    pub fn set_permission(
        &self,
        arg0: impl Into<String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::command::Command::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::command::Command = temp_clone.into();
        real.set_permission(arg0)
    }
    pub fn unregister(
        &self,
        arg0: impl Into<crate::command::CommandMap<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::command::Command::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::command::Command = temp_clone.into();
        real.unregister(arg0)
    }
    pub fn description(&self) -> Result<String, Box<dyn std::error::Error>> {
        let temp_clone = crate::command::Command::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::command::Command = temp_clone.into();
        real.description()
    }
    pub fn set_description(
        &self,
        arg0: impl Into<String>,
    ) -> Result<crate::command::Command<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::command::Command::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::command::Command = temp_clone.into();
        real.set_description(arg0)
    }
    pub fn set_usage(
        &self,
        arg0: impl Into<String>,
    ) -> Result<crate::command::Command<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::command::Command::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::command::Command = temp_clone.into();
        real.set_usage(arg0)
    }
    pub fn set_aliases(
        &self,
        arg0: Vec<impl Into<String>>,
    ) -> Result<crate::command::Command<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/util/List;)Lorg/bukkit/command/Command;");
        let raw_val_1 = self
            .jni_ref()
            .new_object("java/util/ArrayList", "()V", vec![])?;
        for v in arg0 {
            let map_val_0 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
                self.jni_ref().new_string(v.into())?,
            ));
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
            "setAliases",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::command::Command::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    pub fn set_permission_message(
        &self,
        arg0: impl Into<String>,
    ) -> Result<crate::command::Command<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::command::Command::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::command::Command = temp_clone.into();
        real.set_permission_message(arg0)
    }
    pub fn aliases(&self) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/List;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getAliases", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let list = blackboxmc_java::util::JavaList::from_raw(&self.0, res.l()?)?;
        let iter = list.iterator()?;
        while iter.has_next()? {
            let obj = iter.next()?;
            new_vec.push(
                self.jni_ref()
                    .get_string(unsafe { &jni::objects::JString::from_raw(*obj) })?
                    .to_string_lossy()
                    .to_string(),
            );
        }
        Ok(new_vec)
    }
    pub fn set_label(&self, arg0: impl Into<String>) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::command::Command::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::command::Command = temp_clone.into();
        real.set_label(arg0)
    }
    pub fn label(&self) -> Result<String, Box<dyn std::error::Error>> {
        let temp_clone = crate::command::Command::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::command::Command = temp_clone.into();
        real.label()
    }
    pub fn test_permission_silent(
        &self,
        arg0: impl Into<crate::command::CommandSender<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::command::Command::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::command::Command = temp_clone.into();
        real.test_permission_silent(arg0)
    }
    pub fn test_permission(
        &self,
        arg0: impl Into<crate::command::CommandSender<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::command::Command::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::command::Command = temp_clone.into();
        real.test_permission(arg0)
    }
    pub fn permission_message(&self) -> Result<Option<String>, Box<dyn std::error::Error>> {
        let temp_clone = crate::command::Command::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::command::Command = temp_clone.into();
        real.permission_message()
    }
    pub fn usage(&self) -> Result<String, Box<dyn std::error::Error>> {
        let temp_clone = crate::command::Command::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::command::Command = temp_clone.into();
        real.usage()
    }
    pub fn broadcast_command_message_with_command_sender(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<crate::command::CommandSender<'mc>>,
        arg1: impl Into<String>,
        arg2: std::option::Option<bool>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/command/CommandSender;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        sig += "Ljava/lang/String;";
        let val_2 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            jni.new_string(arg1.into())?,
        ));
        args.push(val_2);
        if let Some(a) = arg2 {
            sig += "Z";
            let val_3 = jni::objects::JValueGen::Bool(a.into());
            args.push(val_3);
        }
        sig += ")V";
        let cls = jni.find_class("void");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "broadcastCommandMessage", sig.as_str(), args);
        jni.translate_error(res)?;
        Ok(())
    }
    pub fn name(&self) -> Result<String, Box<dyn std::error::Error>> {
        let temp_clone = crate::command::Command::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::command::Command = temp_clone.into();
        real.name()
    }
    #[doc(hidden)]
    pub fn internal_to_string(&self) -> Result<String, Box<dyn std::error::Error>> {
        let temp_clone = crate::command::Command::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::command::Command = temp_clone.into();
        real.internal_to_string()
    }
    pub fn is_registered(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::command::Command::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::command::Command = temp_clone.into();
        real.is_registered()
    }
    pub fn register(
        &self,
        arg0: impl Into<crate::command::CommandMap<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::command::Command::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::command::Command = temp_clone.into();
        real.register(arg0)
    }
    pub fn set_name(&self, arg0: impl Into<String>) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::command::Command::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::command::Command = temp_clone.into();
        real.set_name(arg0)
    }
    // SUPER CLASS: Object

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::command::defaults::BukkitCommand<'mc>> for TimingsCommand<'mc> {
    fn into(self) -> crate::command::defaults::BukkitCommand<'mc> {
        crate::command::defaults::BukkitCommand::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting TimingsCommand into crate::command::defaults::BukkitCommand")
    }
}

#[repr(C)]
pub struct BukkitCommand<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for BukkitCommand<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for BukkitCommand<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate BukkitCommand from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/command/defaults/BukkitCommand")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a BukkitCommand object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> BukkitCommand<'mc> {
    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::command::Command<'mc>> for BukkitCommand<'mc> {
    fn into(self) -> crate::command::Command<'mc> {
        crate::command::Command::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting BukkitCommand into crate::command::Command")
    }
}

#[repr(C)]
pub struct VersionCommand<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for VersionCommand<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for VersionCommand<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate VersionCommand from null object.").into(),
            );
        }
        let (valid, name) =
            env.validate_name(&obj, "org/bukkit/command/defaults/VersionCommand")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a VersionCommand object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> VersionCommand<'mc> {
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<String>,
    ) -> Result<crate::command::defaults::VersionCommand<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/String;)V");
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            jni.new_string(arg0.into())?,
        ));
        let cls = jni.find_class("org/bukkit/command/defaults/VersionCommand");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(
            cls,
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = jni.translate_error_no_gen(res)?;
        crate::command::defaults::VersionCommand::from_raw(&jni, res)
    }

    pub fn tab_complete(
        &self,
        arg0: impl Into<crate::command::CommandSender<'mc>>,
        arg1: impl Into<String>,
        arg2: Vec<String>,
    ) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/command/CommandSender;Ljava/lang/String;[Ljava/lang/String;)Ljava/util/List;");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let val_2 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(arg1.into())?,
        ));
        let arr = self.jni_ref().new_object_array(
            arg2.len() as i32,
            "java/lang/String",
            jni::objects::JObject::null(),
        );
        let arr = self.jni_ref().translate_error_no_gen(arr)?;
        for i in 0..arg2.len() {
            let val_3 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
                self.jni_ref().new_string(arg2.get(i).unwrap().clone())?,
            ));
            self.jni_ref()
                .set_object_array_element(&arr, i as i32, val_3.l()?)?;
        }
        let val_3 = jni::objects::JValueGen::Object(arr);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "tabComplete",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
                jni::objects::JValueGen::from(val_3.l()?),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let list = blackboxmc_java::util::JavaList::from_raw(&self.0, res.l()?)?;
        let iter = list.iterator()?;
        while iter.has_next()? {
            let obj = iter.next()?;
            new_vec.push(
                self.jni_ref()
                    .get_string(unsafe { &jni::objects::JString::from_raw(*obj) })?
                    .to_string_lossy()
                    .to_string(),
            );
        }
        Ok(new_vec)
    }

    pub fn execute(
        &self,
        arg0: impl Into<crate::command::CommandSender<'mc>>,
        arg1: impl Into<String>,
        arg2: Vec<String>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from(
            "(Lorg/bukkit/command/CommandSender;Ljava/lang/String;[Ljava/lang/String;)Z",
        );
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let val_2 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(arg1.into())?,
        ));
        let arr = self.jni_ref().new_object_array(
            arg2.len() as i32,
            "java/lang/String",
            jni::objects::JObject::null(),
        );
        let arr = self.jni_ref().translate_error_no_gen(arr)?;
        for i in 0..arg2.len() {
            let val_3 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
                self.jni_ref().new_string(arg2.get(i).unwrap().clone())?,
            ));
            self.jni_ref()
                .set_object_array_element(&arr, i as i32, val_3.l()?)?;
        }
        let val_3 = jni::objects::JValueGen::Object(arr);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "execute",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
                jni::objects::JValueGen::from(val_3.l()?),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    // SUPER CLASS: BukkitCommand
    // SUPER CLASS: Command
    pub fn permission(&self) -> Result<Option<String>, Box<dyn std::error::Error>> {
        let temp_clone = crate::command::Command::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::command::Command = temp_clone.into();
        real.permission()
    }
    pub fn set_permission(
        &self,
        arg0: impl Into<String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::command::Command::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::command::Command = temp_clone.into();
        real.set_permission(arg0)
    }
    pub fn unregister(
        &self,
        arg0: impl Into<crate::command::CommandMap<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::command::Command::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::command::Command = temp_clone.into();
        real.unregister(arg0)
    }
    pub fn description(&self) -> Result<String, Box<dyn std::error::Error>> {
        let temp_clone = crate::command::Command::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::command::Command = temp_clone.into();
        real.description()
    }
    pub fn set_description(
        &self,
        arg0: impl Into<String>,
    ) -> Result<crate::command::Command<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::command::Command::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::command::Command = temp_clone.into();
        real.set_description(arg0)
    }
    pub fn set_usage(
        &self,
        arg0: impl Into<String>,
    ) -> Result<crate::command::Command<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::command::Command::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::command::Command = temp_clone.into();
        real.set_usage(arg0)
    }
    pub fn set_aliases(
        &self,
        arg0: Vec<impl Into<String>>,
    ) -> Result<crate::command::Command<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/util/List;)Lorg/bukkit/command/Command;");
        let raw_val_1 = self
            .jni_ref()
            .new_object("java/util/ArrayList", "()V", vec![])?;
        for v in arg0 {
            let map_val_0 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
                self.jni_ref().new_string(v.into())?,
            ));
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
            "setAliases",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::command::Command::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    pub fn set_permission_message(
        &self,
        arg0: impl Into<String>,
    ) -> Result<crate::command::Command<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::command::Command::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::command::Command = temp_clone.into();
        real.set_permission_message(arg0)
    }
    pub fn aliases(&self) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/List;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getAliases", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let list = blackboxmc_java::util::JavaList::from_raw(&self.0, res.l()?)?;
        let iter = list.iterator()?;
        while iter.has_next()? {
            let obj = iter.next()?;
            new_vec.push(
                self.jni_ref()
                    .get_string(unsafe { &jni::objects::JString::from_raw(*obj) })?
                    .to_string_lossy()
                    .to_string(),
            );
        }
        Ok(new_vec)
    }
    pub fn set_label(&self, arg0: impl Into<String>) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::command::Command::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::command::Command = temp_clone.into();
        real.set_label(arg0)
    }
    pub fn label(&self) -> Result<String, Box<dyn std::error::Error>> {
        let temp_clone = crate::command::Command::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::command::Command = temp_clone.into();
        real.label()
    }
    pub fn test_permission_silent(
        &self,
        arg0: impl Into<crate::command::CommandSender<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::command::Command::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::command::Command = temp_clone.into();
        real.test_permission_silent(arg0)
    }
    pub fn test_permission(
        &self,
        arg0: impl Into<crate::command::CommandSender<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::command::Command::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::command::Command = temp_clone.into();
        real.test_permission(arg0)
    }
    pub fn permission_message(&self) -> Result<Option<String>, Box<dyn std::error::Error>> {
        let temp_clone = crate::command::Command::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::command::Command = temp_clone.into();
        real.permission_message()
    }
    pub fn usage(&self) -> Result<String, Box<dyn std::error::Error>> {
        let temp_clone = crate::command::Command::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::command::Command = temp_clone.into();
        real.usage()
    }
    pub fn broadcast_command_message_with_command_sender(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<crate::command::CommandSender<'mc>>,
        arg1: impl Into<String>,
        arg2: std::option::Option<bool>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/command/CommandSender;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        sig += "Ljava/lang/String;";
        let val_2 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            jni.new_string(arg1.into())?,
        ));
        args.push(val_2);
        if let Some(a) = arg2 {
            sig += "Z";
            let val_3 = jni::objects::JValueGen::Bool(a.into());
            args.push(val_3);
        }
        sig += ")V";
        let cls = jni.find_class("void");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "broadcastCommandMessage", sig.as_str(), args);
        jni.translate_error(res)?;
        Ok(())
    }
    pub fn name(&self) -> Result<String, Box<dyn std::error::Error>> {
        let temp_clone = crate::command::Command::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::command::Command = temp_clone.into();
        real.name()
    }
    #[doc(hidden)]
    pub fn internal_to_string(&self) -> Result<String, Box<dyn std::error::Error>> {
        let temp_clone = crate::command::Command::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::command::Command = temp_clone.into();
        real.internal_to_string()
    }
    pub fn is_registered(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::command::Command::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::command::Command = temp_clone.into();
        real.is_registered()
    }
    pub fn register(
        &self,
        arg0: impl Into<crate::command::CommandMap<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::command::Command::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::command::Command = temp_clone.into();
        real.register(arg0)
    }
    pub fn set_name(&self, arg0: impl Into<String>) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::command::Command::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::command::Command = temp_clone.into();
        real.set_name(arg0)
    }
    // SUPER CLASS: Object

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::command::defaults::BukkitCommand<'mc>> for VersionCommand<'mc> {
    fn into(self) -> crate::command::defaults::BukkitCommand<'mc> {
        crate::command::defaults::BukkitCommand::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting VersionCommand into crate::command::defaults::BukkitCommand")
    }
}

#[repr(C)]
pub struct HelpCommand<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for HelpCommand<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for HelpCommand<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate HelpCommand from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/command/defaults/HelpCommand")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a HelpCommand object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> HelpCommand<'mc> {
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<crate::command::defaults::HelpCommand<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let cls = jni.find_class("org/bukkit/command/defaults/HelpCommand");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(cls, sig.as_str(), vec![]);
        let res = jni.translate_error_no_gen(res)?;
        crate::command::defaults::HelpCommand::from_raw(&jni, res)
    }

    pub fn tab_complete(
        &self,
        arg0: impl Into<crate::command::CommandSender<'mc>>,
        arg1: impl Into<String>,
        arg2: Vec<String>,
    ) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/command/CommandSender;Ljava/lang/String;[Ljava/lang/String;)Ljava/util/List;");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let val_2 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(arg1.into())?,
        ));
        let arr = self.jni_ref().new_object_array(
            arg2.len() as i32,
            "java/lang/String",
            jni::objects::JObject::null(),
        );
        let arr = self.jni_ref().translate_error_no_gen(arr)?;
        for i in 0..arg2.len() {
            let val_3 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
                self.jni_ref().new_string(arg2.get(i).unwrap().clone())?,
            ));
            self.jni_ref()
                .set_object_array_element(&arr, i as i32, val_3.l()?)?;
        }
        let val_3 = jni::objects::JValueGen::Object(arr);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "tabComplete",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
                jni::objects::JValueGen::from(val_3.l()?),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let list = blackboxmc_java::util::JavaList::from_raw(&self.0, res.l()?)?;
        let iter = list.iterator()?;
        while iter.has_next()? {
            let obj = iter.next()?;
            new_vec.push(
                self.jni_ref()
                    .get_string(unsafe { &jni::objects::JString::from_raw(*obj) })?
                    .to_string_lossy()
                    .to_string(),
            );
        }
        Ok(new_vec)
    }

    pub fn execute(
        &self,
        arg0: impl Into<crate::command::CommandSender<'mc>>,
        arg1: impl Into<String>,
        arg2: Vec<String>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from(
            "(Lorg/bukkit/command/CommandSender;Ljava/lang/String;[Ljava/lang/String;)Z",
        );
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let val_2 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(arg1.into())?,
        ));
        let arr = self.jni_ref().new_object_array(
            arg2.len() as i32,
            "java/lang/String",
            jni::objects::JObject::null(),
        );
        let arr = self.jni_ref().translate_error_no_gen(arr)?;
        for i in 0..arg2.len() {
            let val_3 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
                self.jni_ref().new_string(arg2.get(i).unwrap().clone())?,
            ));
            self.jni_ref()
                .set_object_array_element(&arr, i as i32, val_3.l()?)?;
        }
        let val_3 = jni::objects::JValueGen::Object(arr);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "execute",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
                jni::objects::JValueGen::from(val_3.l()?),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    // SUPER CLASS: BukkitCommand
    // SUPER CLASS: Command
    pub fn permission(&self) -> Result<Option<String>, Box<dyn std::error::Error>> {
        let temp_clone = crate::command::Command::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::command::Command = temp_clone.into();
        real.permission()
    }
    pub fn set_permission(
        &self,
        arg0: impl Into<String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::command::Command::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::command::Command = temp_clone.into();
        real.set_permission(arg0)
    }
    pub fn unregister(
        &self,
        arg0: impl Into<crate::command::CommandMap<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::command::Command::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::command::Command = temp_clone.into();
        real.unregister(arg0)
    }
    pub fn description(&self) -> Result<String, Box<dyn std::error::Error>> {
        let temp_clone = crate::command::Command::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::command::Command = temp_clone.into();
        real.description()
    }
    pub fn set_description(
        &self,
        arg0: impl Into<String>,
    ) -> Result<crate::command::Command<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::command::Command::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::command::Command = temp_clone.into();
        real.set_description(arg0)
    }
    pub fn set_usage(
        &self,
        arg0: impl Into<String>,
    ) -> Result<crate::command::Command<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::command::Command::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::command::Command = temp_clone.into();
        real.set_usage(arg0)
    }
    pub fn set_aliases(
        &self,
        arg0: Vec<impl Into<String>>,
    ) -> Result<crate::command::Command<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/util/List;)Lorg/bukkit/command/Command;");
        let raw_val_1 = self
            .jni_ref()
            .new_object("java/util/ArrayList", "()V", vec![])?;
        for v in arg0 {
            let map_val_0 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
                self.jni_ref().new_string(v.into())?,
            ));
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
            "setAliases",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::command::Command::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    pub fn set_permission_message(
        &self,
        arg0: impl Into<String>,
    ) -> Result<crate::command::Command<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::command::Command::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::command::Command = temp_clone.into();
        real.set_permission_message(arg0)
    }
    pub fn aliases(&self) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/List;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getAliases", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let list = blackboxmc_java::util::JavaList::from_raw(&self.0, res.l()?)?;
        let iter = list.iterator()?;
        while iter.has_next()? {
            let obj = iter.next()?;
            new_vec.push(
                self.jni_ref()
                    .get_string(unsafe { &jni::objects::JString::from_raw(*obj) })?
                    .to_string_lossy()
                    .to_string(),
            );
        }
        Ok(new_vec)
    }
    pub fn set_label(&self, arg0: impl Into<String>) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::command::Command::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::command::Command = temp_clone.into();
        real.set_label(arg0)
    }
    pub fn label(&self) -> Result<String, Box<dyn std::error::Error>> {
        let temp_clone = crate::command::Command::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::command::Command = temp_clone.into();
        real.label()
    }
    pub fn test_permission_silent(
        &self,
        arg0: impl Into<crate::command::CommandSender<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::command::Command::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::command::Command = temp_clone.into();
        real.test_permission_silent(arg0)
    }
    pub fn test_permission(
        &self,
        arg0: impl Into<crate::command::CommandSender<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::command::Command::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::command::Command = temp_clone.into();
        real.test_permission(arg0)
    }
    pub fn permission_message(&self) -> Result<Option<String>, Box<dyn std::error::Error>> {
        let temp_clone = crate::command::Command::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::command::Command = temp_clone.into();
        real.permission_message()
    }
    pub fn usage(&self) -> Result<String, Box<dyn std::error::Error>> {
        let temp_clone = crate::command::Command::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::command::Command = temp_clone.into();
        real.usage()
    }
    pub fn broadcast_command_message_with_command_sender(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<crate::command::CommandSender<'mc>>,
        arg1: impl Into<String>,
        arg2: std::option::Option<bool>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/command/CommandSender;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        sig += "Ljava/lang/String;";
        let val_2 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            jni.new_string(arg1.into())?,
        ));
        args.push(val_2);
        if let Some(a) = arg2 {
            sig += "Z";
            let val_3 = jni::objects::JValueGen::Bool(a.into());
            args.push(val_3);
        }
        sig += ")V";
        let cls = jni.find_class("void");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "broadcastCommandMessage", sig.as_str(), args);
        jni.translate_error(res)?;
        Ok(())
    }
    pub fn name(&self) -> Result<String, Box<dyn std::error::Error>> {
        let temp_clone = crate::command::Command::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::command::Command = temp_clone.into();
        real.name()
    }
    #[doc(hidden)]
    pub fn internal_to_string(&self) -> Result<String, Box<dyn std::error::Error>> {
        let temp_clone = crate::command::Command::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::command::Command = temp_clone.into();
        real.internal_to_string()
    }
    pub fn is_registered(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::command::Command::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::command::Command = temp_clone.into();
        real.is_registered()
    }
    pub fn register(
        &self,
        arg0: impl Into<crate::command::CommandMap<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::command::Command::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::command::Command = temp_clone.into();
        real.register(arg0)
    }
    pub fn set_name(&self, arg0: impl Into<String>) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::command::Command::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::command::Command = temp_clone.into();
        real.set_name(arg0)
    }
    // SUPER CLASS: Object

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::command::defaults::BukkitCommand<'mc>> for HelpCommand<'mc> {
    fn into(self) -> crate::command::defaults::BukkitCommand<'mc> {
        crate::command::defaults::BukkitCommand::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting HelpCommand into crate::command::defaults::BukkitCommand")
    }
}

#[repr(C)]
pub struct PluginsCommand<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for PluginsCommand<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for PluginsCommand<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate PluginsCommand from null object.").into(),
            );
        }
        let (valid, name) =
            env.validate_name(&obj, "org/bukkit/command/defaults/PluginsCommand")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a PluginsCommand object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> PluginsCommand<'mc> {
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<String>,
    ) -> Result<crate::command::defaults::PluginsCommand<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/String;)V");
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            jni.new_string(arg0.into())?,
        ));
        let cls = jni.find_class("org/bukkit/command/defaults/PluginsCommand");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(
            cls,
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = jni.translate_error_no_gen(res)?;
        crate::command::defaults::PluginsCommand::from_raw(&jni, res)
    }

    pub fn tab_complete(
        &self,
        arg0: impl Into<crate::command::CommandSender<'mc>>,
        arg1: impl Into<String>,
        arg2: Vec<String>,
    ) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/command/CommandSender;Ljava/lang/String;[Ljava/lang/String;)Ljava/util/List;");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let val_2 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(arg1.into())?,
        ));
        let arr = self.jni_ref().new_object_array(
            arg2.len() as i32,
            "java/lang/String",
            jni::objects::JObject::null(),
        );
        let arr = self.jni_ref().translate_error_no_gen(arr)?;
        for i in 0..arg2.len() {
            let val_3 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
                self.jni_ref().new_string(arg2.get(i).unwrap().clone())?,
            ));
            self.jni_ref()
                .set_object_array_element(&arr, i as i32, val_3.l()?)?;
        }
        let val_3 = jni::objects::JValueGen::Object(arr);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "tabComplete",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
                jni::objects::JValueGen::from(val_3.l()?),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let list = blackboxmc_java::util::JavaList::from_raw(&self.0, res.l()?)?;
        let iter = list.iterator()?;
        while iter.has_next()? {
            let obj = iter.next()?;
            new_vec.push(
                self.jni_ref()
                    .get_string(unsafe { &jni::objects::JString::from_raw(*obj) })?
                    .to_string_lossy()
                    .to_string(),
            );
        }
        Ok(new_vec)
    }

    pub fn execute(
        &self,
        arg0: impl Into<crate::command::CommandSender<'mc>>,
        arg1: impl Into<String>,
        arg2: Vec<String>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from(
            "(Lorg/bukkit/command/CommandSender;Ljava/lang/String;[Ljava/lang/String;)Z",
        );
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let val_2 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(arg1.into())?,
        ));
        let arr = self.jni_ref().new_object_array(
            arg2.len() as i32,
            "java/lang/String",
            jni::objects::JObject::null(),
        );
        let arr = self.jni_ref().translate_error_no_gen(arr)?;
        for i in 0..arg2.len() {
            let val_3 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
                self.jni_ref().new_string(arg2.get(i).unwrap().clone())?,
            ));
            self.jni_ref()
                .set_object_array_element(&arr, i as i32, val_3.l()?)?;
        }
        let val_3 = jni::objects::JValueGen::Object(arr);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "execute",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
                jni::objects::JValueGen::from(val_3.l()?),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    // SUPER CLASS: BukkitCommand
    // SUPER CLASS: Command
    pub fn permission(&self) -> Result<Option<String>, Box<dyn std::error::Error>> {
        let temp_clone = crate::command::Command::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::command::Command = temp_clone.into();
        real.permission()
    }
    pub fn set_permission(
        &self,
        arg0: impl Into<String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::command::Command::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::command::Command = temp_clone.into();
        real.set_permission(arg0)
    }
    pub fn unregister(
        &self,
        arg0: impl Into<crate::command::CommandMap<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::command::Command::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::command::Command = temp_clone.into();
        real.unregister(arg0)
    }
    pub fn description(&self) -> Result<String, Box<dyn std::error::Error>> {
        let temp_clone = crate::command::Command::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::command::Command = temp_clone.into();
        real.description()
    }
    pub fn set_description(
        &self,
        arg0: impl Into<String>,
    ) -> Result<crate::command::Command<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::command::Command::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::command::Command = temp_clone.into();
        real.set_description(arg0)
    }
    pub fn set_usage(
        &self,
        arg0: impl Into<String>,
    ) -> Result<crate::command::Command<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::command::Command::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::command::Command = temp_clone.into();
        real.set_usage(arg0)
    }
    pub fn set_aliases(
        &self,
        arg0: Vec<impl Into<String>>,
    ) -> Result<crate::command::Command<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/util/List;)Lorg/bukkit/command/Command;");
        let raw_val_1 = self
            .jni_ref()
            .new_object("java/util/ArrayList", "()V", vec![])?;
        for v in arg0 {
            let map_val_0 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
                self.jni_ref().new_string(v.into())?,
            ));
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
            "setAliases",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::command::Command::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    pub fn set_permission_message(
        &self,
        arg0: impl Into<String>,
    ) -> Result<crate::command::Command<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::command::Command::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::command::Command = temp_clone.into();
        real.set_permission_message(arg0)
    }
    pub fn aliases(&self) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/List;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getAliases", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let list = blackboxmc_java::util::JavaList::from_raw(&self.0, res.l()?)?;
        let iter = list.iterator()?;
        while iter.has_next()? {
            let obj = iter.next()?;
            new_vec.push(
                self.jni_ref()
                    .get_string(unsafe { &jni::objects::JString::from_raw(*obj) })?
                    .to_string_lossy()
                    .to_string(),
            );
        }
        Ok(new_vec)
    }
    pub fn set_label(&self, arg0: impl Into<String>) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::command::Command::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::command::Command = temp_clone.into();
        real.set_label(arg0)
    }
    pub fn label(&self) -> Result<String, Box<dyn std::error::Error>> {
        let temp_clone = crate::command::Command::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::command::Command = temp_clone.into();
        real.label()
    }
    pub fn test_permission_silent(
        &self,
        arg0: impl Into<crate::command::CommandSender<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::command::Command::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::command::Command = temp_clone.into();
        real.test_permission_silent(arg0)
    }
    pub fn test_permission(
        &self,
        arg0: impl Into<crate::command::CommandSender<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::command::Command::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::command::Command = temp_clone.into();
        real.test_permission(arg0)
    }
    pub fn permission_message(&self) -> Result<Option<String>, Box<dyn std::error::Error>> {
        let temp_clone = crate::command::Command::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::command::Command = temp_clone.into();
        real.permission_message()
    }
    pub fn usage(&self) -> Result<String, Box<dyn std::error::Error>> {
        let temp_clone = crate::command::Command::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::command::Command = temp_clone.into();
        real.usage()
    }
    pub fn broadcast_command_message_with_command_sender(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<crate::command::CommandSender<'mc>>,
        arg1: impl Into<String>,
        arg2: std::option::Option<bool>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/command/CommandSender;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        sig += "Ljava/lang/String;";
        let val_2 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            jni.new_string(arg1.into())?,
        ));
        args.push(val_2);
        if let Some(a) = arg2 {
            sig += "Z";
            let val_3 = jni::objects::JValueGen::Bool(a.into());
            args.push(val_3);
        }
        sig += ")V";
        let cls = jni.find_class("void");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "broadcastCommandMessage", sig.as_str(), args);
        jni.translate_error(res)?;
        Ok(())
    }
    pub fn name(&self) -> Result<String, Box<dyn std::error::Error>> {
        let temp_clone = crate::command::Command::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::command::Command = temp_clone.into();
        real.name()
    }
    #[doc(hidden)]
    pub fn internal_to_string(&self) -> Result<String, Box<dyn std::error::Error>> {
        let temp_clone = crate::command::Command::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::command::Command = temp_clone.into();
        real.internal_to_string()
    }
    pub fn is_registered(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::command::Command::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::command::Command = temp_clone.into();
        real.is_registered()
    }
    pub fn register(
        &self,
        arg0: impl Into<crate::command::CommandMap<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::command::Command::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::command::Command = temp_clone.into();
        real.register(arg0)
    }
    pub fn set_name(&self, arg0: impl Into<String>) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::command::Command::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::command::Command = temp_clone.into();
        real.set_name(arg0)
    }
    // SUPER CLASS: Object

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::command::defaults::BukkitCommand<'mc>> for PluginsCommand<'mc> {
    fn into(self) -> crate::command::defaults::BukkitCommand<'mc> {
        crate::command::defaults::BukkitCommand::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting PluginsCommand into crate::command::defaults::BukkitCommand")
    }
}
