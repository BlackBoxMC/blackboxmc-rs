#![allow(deprecated)]
use blackboxmc_general::JNIInstantiatable;
use blackboxmc_general::JNIRaw;
use color_eyre::eyre::Result;

#[repr(C)]
pub struct CommandSenderSpigot<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for CommandSenderSpigot<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for CommandSenderSpigot<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate CommandSenderSpigot from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/command/CommandSender$Spigot")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a CommandSenderSpigot object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> CommandSenderSpigot<'mc> {
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<crate::command::CommandSenderSpigot<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let cls = jni.find_class("org/bukkit/command/CommandSender$Spigot");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(cls, sig.as_str(), vec![]);
        let res = jni.translate_error_no_gen(res)?;
        crate::command::CommandSenderSpigot::from_raw(&jni, res)
    }

    pub fn send_message_with_base_components(
        &self,
        arg0: Vec<blackboxmc_bungee::bungee::api::chat::BaseComponent<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
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
        sig += ")V";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "sendMessage", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn send_message_with_uuid(
        &self,
        arg0: impl Into<blackboxmc_java::util::JavaUUID<'mc>>,
        arg1: std::option::Option<Vec<blackboxmc_bungee::bungee::api::chat::BaseComponent<'mc>>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Ljava/util/UUID;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        if let Some(a) = arg1 {
            sig += "[Lnet/md_5/bungee/api/chat/BaseComponent;";
            let arr = self.jni_ref().new_object_array(
                a.len() as i32,
                "net/md_5/bungee/api/chat/BaseComponent",
                jni::objects::JObject::null(),
            );
            let arr = self.jni_ref().translate_error_no_gen(arr)?;
            for i in 0..a.len() {
                let val_2 = jni::objects::JValueGen::Object(unsafe {
                    jni::objects::JObject::from_raw(a.get(i).unwrap().jni_object().clone())
                });
                self.jni_ref()
                    .set_object_array_element(&arr, i as i32, val_2.l()?)?;
            }
            let val_2 = jni::objects::JValueGen::Object(arr);
            args.push(val_2.l()?.into());
        }
        sig += ")V";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "sendMessage", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    // SUPER CLASS: Object

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}

///
/// This is a representation of an abstract class.
#[repr(C)]
pub struct RemoteConsoleCommandSender<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for RemoteConsoleCommandSender<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for RemoteConsoleCommandSender<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate RemoteConsoleCommandSender from null object."
            )
            .into());
        }
        let (valid, name) =
            env.validate_name(&obj, "org/bukkit/command/RemoteConsoleCommandSender")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a RemoteConsoleCommandSender object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> RemoteConsoleCommandSender<'mc> {
    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::command::CommandSender<'mc>> for RemoteConsoleCommandSender<'mc> {
    fn into(self) -> crate::command::CommandSender<'mc> {
        crate::command::CommandSender::from_raw(&self.jni_ref(), self.1).expect(
            "Error converting RemoteConsoleCommandSender into crate::command::CommandSender",
        )
    }
}

#[repr(C)]
pub struct SimpleCommandMap<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for SimpleCommandMap<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for SimpleCommandMap<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate SimpleCommandMap from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/command/SimpleCommandMap")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a SimpleCommandMap object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> SimpleCommandMap<'mc> {
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<crate::Server<'mc>>,
    ) -> Result<crate::command::SimpleCommandMap<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/Server;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let cls = jni.find_class("org/bukkit/command/SimpleCommandMap");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(
            cls,
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = jni.translate_error_no_gen(res)?;
        crate::command::SimpleCommandMap::from_raw(&jni, res)
    }

    pub fn register_all(
        &self,
        arg0: impl Into<String>,
        arg1: Vec<impl Into<crate::command::Command<'mc>>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/String;Ljava/util/List;)V");
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(arg0.into())?,
        ));
        let raw_val_2 = self
            .jni_ref()
            .new_object("java/util/ArrayList", "()V", vec![])?;
        for v in arg1 {
            let map_val_0 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(v.into().jni_object().clone())
            });
            self.jni_ref().call_method(
                &raw_val_2,
                "add",
                "(Lorg/bukkit/command/crate::command::Command)V",
                vec![jni::objects::JValueGen::from(map_val_0)],
            )?;
        }
        let val_2 = jni::objects::JValueGen::Object(raw_val_2);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "registerAll",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
            ],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn clear_commands(&self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "clearCommands", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn get_command(
        &self,
        arg0: impl Into<String>,
    ) -> Result<crate::command::Command<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/String;)Lorg/bukkit/command/Command;");
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(arg0.into())?,
        ));
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getCommand",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::command::Command::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn tab_complete_with_command_sender(
        &self,
        arg0: impl Into<crate::command::CommandSender<'mc>>,
        arg1: impl Into<String>,
        arg2: std::option::Option<impl Into<crate::Location<'mc>>>,
    ) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/command/CommandSender;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        sig += "Ljava/lang/String;";
        let val_2 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(arg1.into())?,
        ));
        args.push(val_2);
        if let Some(a) = arg2 {
            sig += "Lorg/bukkit/Location;";
            let val_3 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(a.into().jni_object().clone())
            });
            args.push(val_3);
        }
        sig += ")Ljava/util/List;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "tabComplete", sig.as_str(), args);
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

    pub fn commands(
        &self,
    ) -> Result<Vec<crate::command::Command<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/Collection;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getCommands", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let col = blackboxmc_java::util::JavaCollection::from_raw(&self.jni_ref(), res.l()?)?;
        let iter = col.iterator()?;
        while iter.has_next()? {
            let obj = iter.next()?;
            new_vec.push(crate::command::Command::from_raw(&self.0, obj)?);
        }
        Ok(new_vec)
    }

    pub fn set_fallback_commands(&self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setFallbackCommands",
            sig.as_str(),
            vec![],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn register_server_aliases(&self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "registerServerAliases",
            sig.as_str(),
            vec![],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn dispatch(
        &self,
        arg0: impl Into<crate::command::CommandSender<'mc>>,
        arg1: impl Into<String>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/command/CommandSender;Ljava/lang/String;)Z");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let val_2 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(arg1.into())?,
        ));
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "dispatch",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn register_with_string(
        &self,
        arg0: impl Into<String>,
        arg1: impl Into<String>,
        arg2: std::option::Option<impl Into<crate::command::Command<'mc>>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Ljava/lang/String;";
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(arg0.into())?,
        ));
        args.push(val_1);
        sig += "Ljava/lang/String;";
        let val_2 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(arg1.into())?,
        ));
        args.push(val_2);
        if let Some(a) = arg2 {
            sig += "Lorg/bukkit/command/Command;";
            let val_3 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(a.into().jni_object().clone())
            });
            args.push(val_3);
        }
        sig += ")Z";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "register", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    // SUPER CLASS: Object

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::command::CommandMap<'mc>> for SimpleCommandMap<'mc> {
    fn into(self) -> crate::command::CommandMap<'mc> {
        crate::command::CommandMap::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting SimpleCommandMap into crate::command::CommandMap")
    }
}

#[repr(C)]
pub struct FormattedCommandAlias<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for FormattedCommandAlias<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for FormattedCommandAlias<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate FormattedCommandAlias from null object."
            )
            .into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/command/FormattedCommandAlias")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a FormattedCommandAlias object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> FormattedCommandAlias<'mc> {
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<String>,
        arg1: Vec<String>,
    ) -> Result<crate::command::FormattedCommandAlias<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/String;[Ljava/lang/String;)V");
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            jni.new_string(arg0.into())?,
        ));
        let arr = jni.new_object_array(
            arg1.len() as i32,
            "java/lang/String",
            jni::objects::JObject::null(),
        );
        let arr = jni.translate_error_no_gen(arr)?;
        for i in 0..arg1.len() {
            let val_2 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
                jni.new_string(arg1.get(i).unwrap().clone())?,
            ));
            jni.set_object_array_element(&arr, i as i32, val_2.l()?)?;
        }
        let val_2 = jni::objects::JValueGen::Object(arr);
        let cls = jni.find_class("org/bukkit/command/FormattedCommandAlias");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(
            cls,
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2.l()?),
            ],
        );
        let res = jni.translate_error_no_gen(res)?;
        crate::command::FormattedCommandAlias::from_raw(&jni, res)
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
    pub fn tab_complete_with_command_sender(
        &self,
        arg0: impl Into<crate::command::CommandSender<'mc>>,
        arg1: impl Into<String>,
        arg2: Vec<String>,
        arg3: std::option::Option<impl Into<crate::Location<'mc>>>,
    ) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/command/CommandSender;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        sig += "Ljava/lang/String;";
        let val_2 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(arg1.into())?,
        ));
        args.push(val_2);
        sig += "[Ljava/lang/String;";
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
        args.push(val_3.l()?.into());
        if let Some(a) = arg3 {
            sig += "Lorg/bukkit/Location;";
            let val_4 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(a.into().jni_object().clone())
            });
            args.push(val_4);
        }
        sig += ")Ljava/util/List;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "tabComplete", sig.as_str(), args);
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
                "(Ljava/lang/String)V",
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
impl<'mc> Into<crate::command::Command<'mc>> for FormattedCommandAlias<'mc> {
    fn into(self) -> crate::command::Command<'mc> {
        crate::command::Command::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting FormattedCommandAlias into crate::command::Command")
    }
}
/// Represents a class which can suggest tab completions for commands.
///
/// This is a representation of an abstract class.
#[repr(C)]
pub struct TabCompleter<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for TabCompleter<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for TabCompleter<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate TabCompleter from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/command/TabCompleter")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a TabCompleter object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> TabCompleter<'mc> {
    pub fn from_extendable(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        plugin: &'mc crate::plugin::Plugin,
        address: i32,
        lib_name: String,
        name: String,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let obj = unsafe { plugin.new_extendable(address, "TabCompleter", name, lib_name) }?;
        Self::from_raw(env, obj)
    }

    pub fn on_tab_complete(
        &self,
        arg0: impl Into<crate::command::CommandSender<'mc>>,
        arg1: impl Into<crate::command::Command<'mc>>,
        arg2: impl Into<String>,
        arg3: Vec<String>,
    ) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/command/CommandSender;Lorg/bukkit/command/Command;Ljava/lang/String;[Ljava/lang/String;)Ljava/util/List;");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let val_2 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg1.into().jni_object().clone())
        });
        let val_3 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(arg2.into())?,
        ));
        let arr = self.jni_ref().new_object_array(
            arg3.len() as i32,
            "java/lang/String",
            jni::objects::JObject::null(),
        );
        let arr = self.jni_ref().translate_error_no_gen(arr)?;
        for i in 0..arg3.len() {
            let val_4 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
                self.jni_ref().new_string(arg3.get(i).unwrap().clone())?,
            ));
            self.jni_ref()
                .set_object_array_element(&arr, i as i32, val_4.l()?)?;
        }
        let val_4 = jni::objects::JValueGen::Object(arr);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "onTabComplete",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
                jni::objects::JValueGen::from(val_3),
                jni::objects::JValueGen::from(val_4.l()?),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let list = blackboxmc_java::util::JavaList::from_raw(&self.jni_ref(), res.l()?)?;
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

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
/// Represents a class which contains a single method for executing commands
///
/// This is a representation of an abstract class.
#[repr(C)]
pub struct CommandExecutor<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for CommandExecutor<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for CommandExecutor<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate CommandExecutor from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/command/CommandExecutor")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a CommandExecutor object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> CommandExecutor<'mc> {
    pub fn from_extendable(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        plugin: &'mc crate::plugin::Plugin,
        address: i32,
        lib_name: String,
        name: String,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let obj = unsafe { plugin.new_extendable(address, "CommandExecutor", name, lib_name) }?;
        Self::from_raw(env, obj)
    }

    pub fn on_command(
        &self,
        arg0: impl Into<crate::command::CommandSender<'mc>>,
        arg1: impl Into<crate::command::Command<'mc>>,
        arg2: impl Into<String>,
        arg3: Vec<String>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/command/CommandSender;Lorg/bukkit/command/Command;Ljava/lang/String;[Ljava/lang/String;)Z");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let val_2 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg1.into().jni_object().clone())
        });
        let val_3 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(arg2.into())?,
        ));
        let arr = self.jni_ref().new_object_array(
            arg3.len() as i32,
            "java/lang/String",
            jni::objects::JObject::null(),
        );
        let arr = self.jni_ref().translate_error_no_gen(arr)?;
        for i in 0..arg3.len() {
            let val_4 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
                self.jni_ref().new_string(arg3.get(i).unwrap().clone())?,
            ));
            self.jni_ref()
                .set_object_array_element(&arr, i as i32, val_4.l()?)?;
        }
        let val_4 = jni::objects::JValueGen::Object(arr);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "onCommand",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
                jni::objects::JValueGen::from(val_3),
                jni::objects::JValueGen::from(val_4.l()?),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}

///
/// This is a representation of an abstract class.
#[repr(C)]
pub struct ConsoleCommandSender<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for ConsoleCommandSender<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for ConsoleCommandSender<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate ConsoleCommandSender from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/command/ConsoleCommandSender")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a ConsoleCommandSender object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> ConsoleCommandSender<'mc> {
    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::command::CommandSender<'mc>> for ConsoleCommandSender<'mc> {
    fn into(self) -> crate::command::CommandSender<'mc> {
        crate::command::CommandSender::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting ConsoleCommandSender into crate::command::CommandSender")
    }
}
impl<'mc> Into<crate::conversations::Conversable<'mc>> for ConsoleCommandSender<'mc> {
    fn into(self) -> crate::conversations::Conversable<'mc> {
        crate::conversations::Conversable::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting ConsoleCommandSender into crate::conversations::Conversable")
    }
}
/// This interface is used by the help system to group commands into sub-indexes based on the <a href="../plugin/Plugin.html" title="interface in org.bukkit.plugin"><code>Plugin</code></a> they are a part of. Custom command implementations will need to implement this interface to have a sub-index automatically generated on the plugin's behalf.
///
/// This is a representation of an abstract class.
#[repr(C)]
pub struct PluginIdentifiableCommand<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for PluginIdentifiableCommand<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for PluginIdentifiableCommand<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate PluginIdentifiableCommand from null object."
            )
            .into());
        }
        let (valid, name) =
            env.validate_name(&obj, "org/bukkit/command/PluginIdentifiableCommand")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a PluginIdentifiableCommand object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> PluginIdentifiableCommand<'mc> {
    pub fn plugin(&self) -> Result<crate::plugin::Plugin<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/plugin/Plugin;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getPlugin", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::plugin::Plugin::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
/// Represents a Command, which executes various tasks upon user input
#[repr(C)]
pub struct Command<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for Command<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for Command<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Command from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/command/Command")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Command object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> Command<'mc> {
    pub fn permission(&self) -> Result<Option<String>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/String;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getPermission", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {
            return Ok(None);
        }
        Ok(Some(
            self.jni_ref()
                .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
                .to_string_lossy()
                .to_string(),
        ))
    }

    pub fn set_permission(
        &self,
        arg0: impl Into<String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/String;)V");
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(arg0.into())?,
        ));
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setPermission",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn unregister(
        &self,
        arg0: impl Into<crate::command::CommandMap<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/command/CommandMap;)Z");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "unregister",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn description(&self) -> Result<String, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/String;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getDescription", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }

    pub fn tab_complete_with_command_sender(
        &self,
        arg0: impl Into<crate::command::CommandSender<'mc>>,
        arg1: impl Into<String>,
        arg2: Vec<String>,
        arg3: std::option::Option<impl Into<crate::Location<'mc>>>,
    ) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/command/CommandSender;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        sig += "Ljava/lang/String;";
        let val_2 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(arg1.into())?,
        ));
        args.push(val_2);
        sig += "[Ljava/lang/String;";
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
        args.push(val_3.l()?.into());
        if let Some(a) = arg3 {
            sig += "Lorg/bukkit/Location;";
            let val_4 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(a.into().jni_object().clone())
            });
            args.push(val_4);
        }
        sig += ")Ljava/util/List;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "tabComplete", sig.as_str(), args);
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

    pub fn set_description(
        &self,
        arg0: impl Into<String>,
    ) -> Result<crate::command::Command<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/String;)Lorg/bukkit/command/Command;");
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(arg0.into())?,
        ));
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setDescription",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::command::Command::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn set_usage(
        &self,
        arg0: impl Into<String>,
    ) -> Result<crate::command::Command<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/String;)Lorg/bukkit/command/Command;");
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(arg0.into())?,
        ));
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setUsage",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::command::Command::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
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
                "(Ljava/lang/String)V",
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
        let sig = String::from("(Ljava/lang/String;)Lorg/bukkit/command/Command;");
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(arg0.into())?,
        ));
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setPermissionMessage",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::command::Command::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
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
        let sig = String::from("(Ljava/lang/String;)Z");
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(arg0.into())?,
        ));
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setLabel",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn label(&self) -> Result<String, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/String;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getLabel", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }

    pub fn test_permission_silent(
        &self,
        arg0: impl Into<crate::command::CommandSender<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/command/CommandSender;)Z");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "testPermissionSilent",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn test_permission(
        &self,
        arg0: impl Into<crate::command::CommandSender<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/command/CommandSender;)Z");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "testPermission",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn permission_message(&self) -> Result<Option<String>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/String;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getPermissionMessage",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {
            return Ok(None);
        }
        Ok(Some(
            self.jni_ref()
                .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
                .to_string_lossy()
                .to_string(),
        ))
    }

    pub fn usage(&self) -> Result<String, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/String;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getUsage", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
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

    pub fn is_registered(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isRegistered", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn register(
        &self,
        arg0: impl Into<crate::command::CommandMap<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/command/CommandMap;)Z");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "register",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
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

    pub fn set_name(&self, arg0: impl Into<String>) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/String;)Z");
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(arg0.into())?,
        ));
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setName",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    // SUPER CLASS: Object

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}

impl<'mc> std::string::ToString for Command<'mc> {
    fn to_string(&self) -> String {
        match &self.internal_to_string() {
            Ok(a) => a.clone(),
            Err(err) => format!("Error calling Command.toString: {}", err),
        }
    }
}

/// Represents a command that delegates to one or more other commands
#[repr(C)]
pub struct MultipleCommandAlias<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for MultipleCommandAlias<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for MultipleCommandAlias<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate MultipleCommandAlias from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/command/MultipleCommandAlias")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a MultipleCommandAlias object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> MultipleCommandAlias<'mc> {
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<String>,
        arg1: Vec<crate::command::Command<'mc>>,
    ) -> Result<crate::command::MultipleCommandAlias<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/String;[Lorg/bukkit/command/Command;)V");
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            jni.new_string(arg0.into())?,
        ));
        let arr = jni.new_object_array(
            arg1.len() as i32,
            "org/bukkit/command/Command",
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
        let cls = jni.find_class("org/bukkit/command/MultipleCommandAlias");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(
            cls,
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2.l()?),
            ],
        );
        let res = jni.translate_error_no_gen(res)?;
        crate::command::MultipleCommandAlias::from_raw(&jni, res)
    }

    pub fn commands(
        &self,
    ) -> Result<Vec<crate::command::Command<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/command/Command;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getCommands", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        let arr = Into::<jni::objects::JObjectArray>::into(res.l()?);
        let len = self.jni_ref().get_array_length(&arr)?;
        let mut vec = Vec::new();
        for i in 0..len {
            let res = self.jni_ref().get_object_array_element(&arr, i)?;
            vec.push({ crate::command::Command::from_raw(&self.jni_ref(), res)? });
        }
        Ok(vec)
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
    pub fn tab_complete_with_command_sender(
        &self,
        arg0: impl Into<crate::command::CommandSender<'mc>>,
        arg1: impl Into<String>,
        arg2: Vec<String>,
        arg3: std::option::Option<impl Into<crate::Location<'mc>>>,
    ) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/command/CommandSender;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        sig += "Ljava/lang/String;";
        let val_2 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(arg1.into())?,
        ));
        args.push(val_2);
        sig += "[Ljava/lang/String;";
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
        args.push(val_3.l()?.into());
        if let Some(a) = arg3 {
            sig += "Lorg/bukkit/Location;";
            let val_4 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(a.into().jni_object().clone())
            });
            args.push(val_4);
        }
        sig += ")Ljava/util/List;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "tabComplete", sig.as_str(), args);
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
                "(Ljava/lang/String)V",
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
impl<'mc> Into<crate::command::Command<'mc>> for MultipleCommandAlias<'mc> {
    fn into(self) -> crate::command::Command<'mc> {
        crate::command::Command::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting MultipleCommandAlias into crate::command::Command")
    }
}

#[repr(C)]
pub struct PluginCommandYamlParser<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for PluginCommandYamlParser<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for PluginCommandYamlParser<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate PluginCommandYamlParser from null object."
            )
            .into());
        }
        let (valid, name) =
            env.validate_name(&obj, "org/bukkit/command/PluginCommandYamlParser")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a PluginCommandYamlParser object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> PluginCommandYamlParser<'mc> {
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<crate::command::PluginCommandYamlParser<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let cls = jni.find_class("org/bukkit/command/PluginCommandYamlParser");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(cls, sig.as_str(), vec![]);
        let res = jni.translate_error_no_gen(res)?;
        crate::command::PluginCommandYamlParser::from_raw(&jni, res)
    }

    pub fn parse(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<crate::plugin::Plugin<'mc>>,
    ) -> Result<Vec<crate::command::Command<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/plugin/Plugin;)Ljava/util/List;");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let cls = jni.find_class("java/util/List");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(
            cls,
            "parse",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = jni.translate_error(res)?;
        let mut new_vec = Vec::new();
        let list = blackboxmc_java::util::JavaList::from_raw(&jni, res.l()?)?;
        let iter = list.iterator()?;
        while iter.has_next()? {
            let obj = iter.next()?;
            new_vec.push(crate::command::Command::from_raw(&jni, obj)?);
        }
        Ok(new_vec)
    }
    // SUPER CLASS: Object

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}

///
/// This is a representation of an abstract class.
#[repr(C)]
pub struct ProxiedCommandSender<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for ProxiedCommandSender<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for ProxiedCommandSender<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate ProxiedCommandSender from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/command/ProxiedCommandSender")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a ProxiedCommandSender object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> ProxiedCommandSender<'mc> {
    pub fn caller(&self) -> Result<crate::command::CommandSender<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/command/CommandSender;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getCaller", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::command::CommandSender::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn callee(&self) -> Result<crate::command::CommandSender<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/command/CommandSender;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getCallee", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::command::CommandSender::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    pub fn server(&self) -> Result<crate::Server<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = ProxiedCommandSender::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::command::CommandSender = temp_clone.into();
        real.server()
    }
    pub fn spigot(
        &self,
    ) -> Result<crate::command::CommandSenderSpigot<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = ProxiedCommandSender::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::command::CommandSender = temp_clone.into();
        real.spigot()
    }
    pub fn send_message_with_strings(
        &self,
        arg0: Vec<String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "[Ljava/lang/String;";
        let arr = self.jni_ref().new_object_array(
            arg0.len() as i32,
            "java/lang/String",
            jni::objects::JObject::null(),
        );
        let arr = self.jni_ref().translate_error_no_gen(arr)?;
        for i in 0..arg0.len() {
            let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
                self.jni_ref().new_string(arg0.get(i).unwrap().clone())?,
            ));
            self.jni_ref()
                .set_object_array_element(&arr, i as i32, val_1.l()?)?;
        }
        let val_1 = jni::objects::JValueGen::Object(arr);
        args.push(val_1.l()?.into());
        sig += ")V";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "sendMessage", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub fn send_message_with_uuid(
        &self,
        arg0: impl Into<blackboxmc_java::util::JavaUUID<'mc>>,
        arg1: std::option::Option<impl Into<String>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Ljava/util/UUID;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        if let Some(a) = arg1 {
            sig += "Ljava/lang/String;";
            let val_2 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
                self.jni_ref().new_string(a.into())?,
            ));
            args.push(val_2);
        }
        sig += ")V";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "sendMessage", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub fn name(&self) -> Result<String, Box<dyn std::error::Error>> {
        let temp_clone = ProxiedCommandSender::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::command::CommandSender = temp_clone.into();
        real.name()
    }
    // SUPER CLASS: Permissible
    pub fn is_op(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::permissions::Permissible::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::permissions::ServerOperator = temp_clone.into();
        real.is_op()
    }
    pub fn set_op(&self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::permissions::Permissible::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::permissions::ServerOperator = temp_clone.into();
        real.set_op(arg0)
    }
    pub fn is_permission_set_with_permission(
        &self,
        arg0: impl Into<crate::permissions::Permission<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/permissions/Permission;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        sig += ")Z";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isPermissionSet", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    pub fn has_permission_with_string(
        &self,
        arg0: impl Into<String>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Ljava/lang/String;";
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(arg0.into())?,
        ));
        args.push(val_1);
        sig += ")Z";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "hasPermission", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    pub fn add_attachment_with_plugin(
        &self,
        arg0: impl Into<crate::plugin::Plugin<'mc>>,
        arg1: std::option::Option<impl Into<String>>,
        arg2: std::option::Option<bool>,
        arg3: std::option::Option<i32>,
    ) -> Result<crate::permissions::PermissionAttachment<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/plugin/Plugin;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        if let Some(a) = arg1 {
            sig += "Ljava/lang/String;";
            let val_2 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
                self.jni_ref().new_string(a.into())?,
            ));
            args.push(val_2);
        }
        if let Some(a) = arg2 {
            sig += "Z";
            let val_3 = jni::objects::JValueGen::Bool(a.into());
            args.push(val_3);
        }
        if let Some(a) = arg3 {
            sig += "I";
            let val_4 = jni::objects::JValueGen::Int(a);
            args.push(val_4);
        }
        sig += ")Lorg/bukkit/permissions/PermissionAttachment;";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "addAttachment", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        crate::permissions::PermissionAttachment::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    pub fn remove_attachment(
        &self,
        arg0: impl Into<crate::permissions::PermissionAttachment<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::permissions::Permissible::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::permissions::Permissible = temp_clone.into();
        real.remove_attachment(arg0)
    }
    pub fn recalculate_permissions(&self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "recalculatePermissions",
            sig.as_str(),
            vec![],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub fn effective_permissions(
        &self,
    ) -> Result<blackboxmc_java::util::JavaSet<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/Set;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getEffectivePermissions",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::util::JavaSet::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::command::CommandSender<'mc>> for ProxiedCommandSender<'mc> {
    fn into(self) -> crate::command::CommandSender<'mc> {
        crate::command::CommandSender::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting ProxiedCommandSender into crate::command::CommandSender")
    }
}
/// This class is provided as a convenience to implement both TabCompleter and CommandExecutor.
///
/// This is a representation of an abstract class.
#[repr(C)]
pub struct TabExecutor<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for TabExecutor<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for TabExecutor<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate TabExecutor from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/command/TabExecutor")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a TabExecutor object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> TabExecutor<'mc> {
    pub fn from_extendable(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        plugin: &'mc crate::plugin::Plugin,
        address: i32,
        lib_name: String,
        name: String,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let obj = unsafe { plugin.new_extendable(address, "TabExecutor", name, lib_name) }?;
        Self::from_raw(env, obj)
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::command::TabCompleter<'mc>> for TabExecutor<'mc> {
    fn into(self) -> crate::command::TabCompleter<'mc> {
        crate::command::TabCompleter::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting TabExecutor into crate::command::TabCompleter")
    }
}
impl<'mc> Into<crate::command::CommandExecutor<'mc>> for TabExecutor<'mc> {
    fn into(self) -> crate::command::CommandExecutor<'mc> {
        crate::command::CommandExecutor::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting TabExecutor into crate::command::CommandExecutor")
    }
}

///
/// This is a representation of an abstract class.
#[repr(C)]
pub struct CommandSender<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for CommandSender<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for CommandSender<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate CommandSender from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/command/CommandSender")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a CommandSender object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> CommandSender<'mc> {
    pub fn server(&self) -> Result<crate::Server<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/Server;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getServer", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::Server::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn spigot(
        &self,
    ) -> Result<crate::command::CommandSenderSpigot<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/command/CommandSender$Spigot;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "spigot", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::command::CommandSenderSpigot::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn send_message_with_strings(
        &self,
        arg0: Vec<String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "[Ljava/lang/String;";
        let arr = self.jni_ref().new_object_array(
            arg0.len() as i32,
            "java/lang/String",
            jni::objects::JObject::null(),
        );
        let arr = self.jni_ref().translate_error_no_gen(arr)?;
        for i in 0..arg0.len() {
            let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
                self.jni_ref().new_string(arg0.get(i).unwrap().clone())?,
            ));
            self.jni_ref()
                .set_object_array_element(&arr, i as i32, val_1.l()?)?;
        }
        let val_1 = jni::objects::JValueGen::Object(arr);
        args.push(val_1.l()?.into());
        sig += ")V";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "sendMessage", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn send_message_with_uuid(
        &self,
        arg0: impl Into<blackboxmc_java::util::JavaUUID<'mc>>,
        arg1: std::option::Option<impl Into<String>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Ljava/util/UUID;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        if let Some(a) = arg1 {
            sig += "Ljava/lang/String;";
            let val_2 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
                self.jni_ref().new_string(a.into())?,
            ));
            args.push(val_2);
        }
        sig += ")V";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "sendMessage", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
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
    pub fn is_permission_set_with_permission(
        &self,
        arg0: impl Into<crate::permissions::Permission<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/permissions/Permission;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        sig += ")Z";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isPermissionSet", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    pub fn has_permission_with_string(
        &self,
        arg0: impl Into<String>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Ljava/lang/String;";
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(arg0.into())?,
        ));
        args.push(val_1);
        sig += ")Z";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "hasPermission", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    pub fn add_attachment_with_plugin(
        &self,
        arg0: impl Into<crate::plugin::Plugin<'mc>>,
        arg1: std::option::Option<impl Into<String>>,
        arg2: std::option::Option<bool>,
        arg3: std::option::Option<i32>,
    ) -> Result<crate::permissions::PermissionAttachment<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/plugin/Plugin;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        if let Some(a) = arg1 {
            sig += "Ljava/lang/String;";
            let val_2 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
                self.jni_ref().new_string(a.into())?,
            ));
            args.push(val_2);
        }
        if let Some(a) = arg2 {
            sig += "Z";
            let val_3 = jni::objects::JValueGen::Bool(a.into());
            args.push(val_3);
        }
        if let Some(a) = arg3 {
            sig += "I";
            let val_4 = jni::objects::JValueGen::Int(a);
            args.push(val_4);
        }
        sig += ")Lorg/bukkit/permissions/PermissionAttachment;";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "addAttachment", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        crate::permissions::PermissionAttachment::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    pub fn remove_attachment(
        &self,
        arg0: impl Into<crate::permissions::PermissionAttachment<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = CommandSender::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::permissions::Permissible = temp_clone.into();
        real.remove_attachment(arg0)
    }
    pub fn recalculate_permissions(&self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "recalculatePermissions",
            sig.as_str(),
            vec![],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub fn effective_permissions(
        &self,
    ) -> Result<blackboxmc_java::util::JavaSet<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/Set;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getEffectivePermissions",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::util::JavaSet::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    // SUPER CLASS: ServerOperator
    pub fn is_op(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::permissions::ServerOperator::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::permissions::ServerOperator = temp_clone.into();
        real.is_op()
    }
    pub fn set_op(&self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::permissions::ServerOperator::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::permissions::ServerOperator = temp_clone.into();
        real.set_op(arg0)
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::permissions::Permissible<'mc>> for CommandSender<'mc> {
    fn into(self) -> crate::permissions::Permissible<'mc> {
        crate::permissions::Permissible::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting CommandSender into crate::permissions::Permissible")
    }
}

///
/// This is a representation of an abstract class.
#[repr(C)]
pub struct CommandMap<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for CommandMap<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for CommandMap<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate CommandMap from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/command/CommandMap")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a CommandMap object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> CommandMap<'mc> {
    pub fn register_all(
        &self,
        arg0: impl Into<String>,
        arg1: Vec<impl Into<crate::command::Command<'mc>>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/String;Ljava/util/List;)V");
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(arg0.into())?,
        ));
        let raw_val_2 = self
            .jni_ref()
            .new_object("java/util/ArrayList", "()V", vec![])?;
        for v in arg1 {
            let map_val_0 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(v.into().jni_object().clone())
            });
            self.jni_ref().call_method(
                &raw_val_2,
                "add",
                "(Lorg/bukkit/command/crate::command::Command)V",
                vec![jni::objects::JValueGen::from(map_val_0)],
            )?;
        }
        let val_2 = jni::objects::JValueGen::Object(raw_val_2);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "registerAll",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
            ],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn clear_commands(&self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "clearCommands", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn get_command(
        &self,
        arg0: impl Into<String>,
    ) -> Result<crate::command::Command<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/String;)Lorg/bukkit/command/Command;");
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(arg0.into())?,
        ));
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getCommand",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::command::Command::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn tab_complete_with_command_sender(
        &self,
        arg0: impl Into<crate::command::CommandSender<'mc>>,
        arg1: impl Into<String>,
        arg2: std::option::Option<impl Into<crate::Location<'mc>>>,
    ) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/command/CommandSender;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        sig += "Ljava/lang/String;";
        let val_2 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(arg1.into())?,
        ));
        args.push(val_2);
        if let Some(a) = arg2 {
            sig += "Lorg/bukkit/Location;";
            let val_3 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(a.into().jni_object().clone())
            });
            args.push(val_3);
        }
        sig += ")Ljava/util/List;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "tabComplete", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let list = blackboxmc_java::util::JavaList::from_raw(&self.jni_ref(), res.l()?)?;
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

    pub fn dispatch(
        &self,
        arg0: impl Into<crate::command::CommandSender<'mc>>,
        arg1: impl Into<String>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/command/CommandSender;Ljava/lang/String;)Z");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let val_2 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(arg1.into())?,
        ));
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "dispatch",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn register_with_string(
        &self,
        arg0: impl Into<String>,
        arg1: impl Into<String>,
        arg2: std::option::Option<impl Into<crate::command::Command<'mc>>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Ljava/lang/String;";
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(arg0.into())?,
        ));
        args.push(val_1);
        sig += "Ljava/lang/String;";
        let val_2 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(arg1.into())?,
        ));
        args.push(val_2);
        if let Some(a) = arg2 {
            sig += "Lorg/bukkit/command/Command;";
            let val_3 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(a.into().jni_object().clone())
            });
            args.push(val_3);
        }
        sig += ")Z";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "register", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
/// Represents a <a href="Command.html" title="class in org.bukkit.command"><code>Command</code></a> belonging to a plugin
#[repr(C)]
pub struct PluginCommand<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for PluginCommand<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for PluginCommand<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate PluginCommand from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/command/PluginCommand")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a PluginCommand object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> PluginCommand<'mc> {
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

    pub fn plugin(&self) -> Result<crate::plugin::Plugin<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/plugin/Plugin;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getPlugin", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::plugin::Plugin::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn executor(
        &self,
    ) -> Result<crate::command::CommandExecutor<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/command/CommandExecutor;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getExecutor", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::command::CommandExecutor::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn set_tab_completer(
        &self,
        arg0: impl Into<crate::command::TabCompleter<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/command/TabCompleter;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setTabCompleter",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn tab_completer(
        &self,
    ) -> Result<Option<crate::command::TabCompleter<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/command/TabCompleter;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getTabCompleter", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {
            return Ok(None);
        }
        Ok(Some(crate::command::TabCompleter::from_raw(
            &self.jni_ref(),
            unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) },
        )?))
    }

    pub fn set_executor(
        &self,
        arg0: impl Into<crate::command::CommandExecutor<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/command/CommandExecutor;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setExecutor",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
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
                "(Ljava/lang/String)V",
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

impl<'mc> std::string::ToString for PluginCommand<'mc> {
    fn to_string(&self) -> String {
        match &self.internal_to_string() {
            Ok(a) => a.clone(),
            Err(err) => format!("Error calling PluginCommand.toString: {}", err),
        }
    }
}

impl<'mc> Into<crate::command::PluginIdentifiableCommand<'mc>> for PluginCommand<'mc> {
    fn into(self) -> crate::command::PluginIdentifiableCommand<'mc> {
        crate::command::PluginIdentifiableCommand::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting PluginCommand into crate::command::PluginIdentifiableCommand")
    }
}
impl<'mc> Into<crate::command::Command<'mc>> for PluginCommand<'mc> {
    fn into(self) -> crate::command::Command<'mc> {
        crate::command::Command::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting PluginCommand into crate::command::Command")
    }
}

///
/// This is a representation of an abstract class.
#[repr(C)]
pub struct BlockCommandSender<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for BlockCommandSender<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for BlockCommandSender<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate BlockCommandSender from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/command/BlockCommandSender")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a BlockCommandSender object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> BlockCommandSender<'mc> {
    pub fn block(&self) -> Result<crate::block::Block<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/block/Block;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getBlock", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::block::Block::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    pub fn server(&self) -> Result<crate::Server<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = BlockCommandSender::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::command::CommandSender = temp_clone.into();
        real.server()
    }
    pub fn spigot(
        &self,
    ) -> Result<crate::command::CommandSenderSpigot<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = BlockCommandSender::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::command::CommandSender = temp_clone.into();
        real.spigot()
    }
    pub fn send_message_with_strings(
        &self,
        arg0: Vec<String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "[Ljava/lang/String;";
        let arr = self.jni_ref().new_object_array(
            arg0.len() as i32,
            "java/lang/String",
            jni::objects::JObject::null(),
        );
        let arr = self.jni_ref().translate_error_no_gen(arr)?;
        for i in 0..arg0.len() {
            let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
                self.jni_ref().new_string(arg0.get(i).unwrap().clone())?,
            ));
            self.jni_ref()
                .set_object_array_element(&arr, i as i32, val_1.l()?)?;
        }
        let val_1 = jni::objects::JValueGen::Object(arr);
        args.push(val_1.l()?.into());
        sig += ")V";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "sendMessage", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub fn send_message_with_uuid(
        &self,
        arg0: impl Into<blackboxmc_java::util::JavaUUID<'mc>>,
        arg1: std::option::Option<impl Into<String>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Ljava/util/UUID;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        if let Some(a) = arg1 {
            sig += "Ljava/lang/String;";
            let val_2 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
                self.jni_ref().new_string(a.into())?,
            ));
            args.push(val_2);
        }
        sig += ")V";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "sendMessage", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub fn name(&self) -> Result<String, Box<dyn std::error::Error>> {
        let temp_clone = BlockCommandSender::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::command::CommandSender = temp_clone.into();
        real.name()
    }
    // SUPER CLASS: Permissible
    pub fn is_op(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::permissions::Permissible::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::permissions::ServerOperator = temp_clone.into();
        real.is_op()
    }
    pub fn set_op(&self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::permissions::Permissible::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::permissions::ServerOperator = temp_clone.into();
        real.set_op(arg0)
    }
    pub fn is_permission_set_with_permission(
        &self,
        arg0: impl Into<crate::permissions::Permission<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/permissions/Permission;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        sig += ")Z";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isPermissionSet", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    pub fn has_permission_with_string(
        &self,
        arg0: impl Into<String>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Ljava/lang/String;";
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(arg0.into())?,
        ));
        args.push(val_1);
        sig += ")Z";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "hasPermission", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    pub fn add_attachment_with_plugin(
        &self,
        arg0: impl Into<crate::plugin::Plugin<'mc>>,
        arg1: std::option::Option<impl Into<String>>,
        arg2: std::option::Option<bool>,
        arg3: std::option::Option<i32>,
    ) -> Result<crate::permissions::PermissionAttachment<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/plugin/Plugin;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        if let Some(a) = arg1 {
            sig += "Ljava/lang/String;";
            let val_2 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
                self.jni_ref().new_string(a.into())?,
            ));
            args.push(val_2);
        }
        if let Some(a) = arg2 {
            sig += "Z";
            let val_3 = jni::objects::JValueGen::Bool(a.into());
            args.push(val_3);
        }
        if let Some(a) = arg3 {
            sig += "I";
            let val_4 = jni::objects::JValueGen::Int(a);
            args.push(val_4);
        }
        sig += ")Lorg/bukkit/permissions/PermissionAttachment;";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "addAttachment", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        crate::permissions::PermissionAttachment::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    pub fn remove_attachment(
        &self,
        arg0: impl Into<crate::permissions::PermissionAttachment<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::permissions::Permissible::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::permissions::Permissible = temp_clone.into();
        real.remove_attachment(arg0)
    }
    pub fn recalculate_permissions(&self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "recalculatePermissions",
            sig.as_str(),
            vec![],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub fn effective_permissions(
        &self,
    ) -> Result<blackboxmc_java::util::JavaSet<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/Set;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getEffectivePermissions",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::util::JavaSet::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::command::CommandSender<'mc>> for BlockCommandSender<'mc> {
    fn into(self) -> crate::command::CommandSender<'mc> {
        crate::command::CommandSender::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting BlockCommandSender into crate::command::CommandSender")
    }
}
pub mod defaults;
