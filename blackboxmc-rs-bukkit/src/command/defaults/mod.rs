#![allow(deprecated)]
use blackboxmc_general::JNIInstantiatable;
use blackboxmc_general::JNIRaw;
use color_eyre::eyre::Result;
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
        name: impl Into<String>,
    ) -> Result<crate::command::defaults::VersionCommand<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/String;)V");
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            jni.new_string(name.into())?,
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

    pub fn execute(
        &self,
        sender: impl Into<crate::command::CommandSender<'mc>>,
        current_alias: impl Into<String>,
        val_args: impl Into<String>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from(
            "(Lorg/bukkit/command/CommandSender;Ljava/lang/String;Ljava/lang/String;)Z",
        );
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(sender.into().jni_object().clone())
        });
        let val_2 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(current_alias.into())?,
        ));
        let val_3 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(val_args.into())?,
        ));
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "execute",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
                jni::objects::JValueGen::from(val_3),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn tab_complete(
        &self,
        sender: impl Into<crate::command::CommandSender<'mc>>,
        alias: impl Into<String>,
        val_args: impl Into<String>,
    ) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/command/CommandSender;Ljava/lang/String;Ljava/lang/String;)Ljava/util/List;");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(sender.into().jni_object().clone())
        });
        let val_2 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(alias.into())?,
        ));
        let val_3 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(val_args.into())?,
        ));
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "tabComplete",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
                jni::objects::JValueGen::from(val_3),
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
    // SUPER CLASS: org.bukkit.command.defaults.BukkitCommand ( ['execute', 'tabComplete'])

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
        name: impl Into<String>,
    ) -> Result<crate::command::defaults::TimingsCommand<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/String;)V");
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            jni.new_string(name.into())?,
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

    pub fn execute(
        &self,
        sender: impl Into<crate::command::CommandSender<'mc>>,
        current_alias: impl Into<String>,
        val_args: impl Into<String>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from(
            "(Lorg/bukkit/command/CommandSender;Ljava/lang/String;Ljava/lang/String;)Z",
        );
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(sender.into().jni_object().clone())
        });
        let val_2 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(current_alias.into())?,
        ));
        let val_3 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(val_args.into())?,
        ));
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "execute",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
                jni::objects::JValueGen::from(val_3),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn tab_complete(
        &self,
        sender: impl Into<crate::command::CommandSender<'mc>>,
        alias: impl Into<String>,
        val_args: impl Into<String>,
    ) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/command/CommandSender;Ljava/lang/String;Ljava/lang/String;)Ljava/util/List;");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(sender.into().jni_object().clone())
        });
        let val_2 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(alias.into())?,
        ));
        let val_3 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(val_args.into())?,
        ));
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "tabComplete",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
                jni::objects::JValueGen::from(val_3),
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
    // SUPER CLASS: org.bukkit.command.defaults.BukkitCommand ( ['execute', 'tabComplete'])

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
        name: impl Into<String>,
    ) -> Result<crate::command::defaults::PluginsCommand<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/String;)V");
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            jni.new_string(name.into())?,
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

    pub fn execute(
        &self,
        sender: impl Into<crate::command::CommandSender<'mc>>,
        current_alias: impl Into<String>,
        val_args: impl Into<String>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from(
            "(Lorg/bukkit/command/CommandSender;Ljava/lang/String;Ljava/lang/String;)Z",
        );
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(sender.into().jni_object().clone())
        });
        let val_2 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(current_alias.into())?,
        ));
        let val_3 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(val_args.into())?,
        ));
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "execute",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
                jni::objects::JValueGen::from(val_3),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn tab_complete(
        &self,
        sender: impl Into<crate::command::CommandSender<'mc>>,
        alias: impl Into<String>,
        val_args: impl Into<String>,
    ) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/command/CommandSender;Ljava/lang/String;Ljava/lang/String;)Ljava/util/List;");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(sender.into().jni_object().clone())
        });
        let val_2 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(alias.into())?,
        ));
        let val_3 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(val_args.into())?,
        ));
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "tabComplete",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
                jni::objects::JValueGen::from(val_3),
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
    // SUPER CLASS: org.bukkit.command.defaults.BukkitCommand ( ['execute', 'tabComplete'])

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

    pub fn execute(
        &self,
        sender: impl Into<crate::command::CommandSender<'mc>>,
        current_alias: impl Into<String>,
        val_args: impl Into<String>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from(
            "(Lorg/bukkit/command/CommandSender;Ljava/lang/String;Ljava/lang/String;)Z",
        );
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(sender.into().jni_object().clone())
        });
        let val_2 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(current_alias.into())?,
        ));
        let val_3 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(val_args.into())?,
        ));
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "execute",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
                jni::objects::JValueGen::from(val_3),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn tab_complete(
        &self,
        sender: impl Into<crate::command::CommandSender<'mc>>,
        alias: impl Into<String>,
        val_args: impl Into<String>,
    ) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/command/CommandSender;Ljava/lang/String;Ljava/lang/String;)Ljava/util/List;");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(sender.into().jni_object().clone())
        });
        let val_2 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(alias.into())?,
        ));
        let val_3 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(val_args.into())?,
        ));
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "tabComplete",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
                jni::objects::JValueGen::from(val_3),
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
    // SUPER CLASS: org.bukkit.command.defaults.BukkitCommand ( ['execute', 'tabComplete'])

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
    // SUPER CLASS: org.bukkit.command.Command ( [])
    /// Executed on tab completion for this command, returning a list of
    /// options the player can tab through.
    pub fn tab_complete(
        &self,
        sender: impl Into<crate::command::CommandSender<'mc>>,
        alias: impl Into<String>,
        val_args: impl Into<String>,
        location: std::option::Option<impl Into<crate::Location<'mc>>>,
    ) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let temp_clone = crate::command::Command::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::command::Command = temp_clone.into();
        real.tab_complete(sender, alias, val_args, location)
    }
    /// Returns the name of this command
    pub fn name(&self) -> Result<String, Box<dyn std::error::Error>> {
        let temp_clone = crate::command::Command::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::command::Command = temp_clone.into();
        real.name()
    }
    /// Sets the name of this command.
    ///
    /// May only be used before registering the command.
    /// Will return true if the new name is set, and false
    /// if the command has already been registered.
    pub fn set_name(&self, name: impl Into<String>) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::command::Command::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::command::Command = temp_clone.into();
        real.set_name(name)
    }
    /// Gets the permission required by users to be able to perform this
    /// command
    pub fn permission(&self) -> Result<Option<String>, Box<dyn std::error::Error>> {
        let temp_clone = crate::command::Command::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::command::Command = temp_clone.into();
        real.permission()
    }
    /// Sets the permission required by users to be able to perform this
    /// command
    pub fn set_permission(
        &self,
        permission: impl Into<String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::command::Command::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::command::Command = temp_clone.into();
        real.set_permission(permission)
    }
    /// Tests the given {@link CommandSender} to see if they can perform this
    /// command.
    ///
    /// If they do not have permission, they will be informed that they cannot
    /// do this.
    pub fn test_permission(
        &self,
        target: impl Into<crate::command::CommandSender<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::command::Command::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::command::Command = temp_clone.into();
        real.test_permission(target)
    }
    /// Tests the given {@link CommandSender} to see if they can perform this
    /// command.
    ///
    /// No error is sent to the sender.
    pub fn test_permission_silent(
        &self,
        target: impl Into<crate::command::CommandSender<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::command::Command::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::command::Command = temp_clone.into();
        real.test_permission_silent(target)
    }
    /// Returns the label for this command
    pub fn label(&self) -> Result<String, Box<dyn std::error::Error>> {
        let temp_clone = crate::command::Command::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::command::Command = temp_clone.into();
        real.label()
    }
    /// Sets the label of this command.
    ///
    /// May only be used before registering the command.
    /// Will return true if the new name is set, and false
    /// if the command has already been registered.
    pub fn set_label(&self, name: impl Into<String>) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::command::Command::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::command::Command = temp_clone.into();
        real.set_label(name)
    }
    /// Registers this command to a CommandMap.
    /// Once called it only allows changes the registered CommandMap
    pub fn register(
        &self,
        command_map: impl Into<crate::command::CommandMap<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::command::Command::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::command::Command = temp_clone.into();
        real.register(command_map)
    }
    /// Unregisters this command from the passed CommandMap applying any
    /// outstanding changes
    pub fn unregister(
        &self,
        command_map: impl Into<crate::command::CommandMap<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::command::Command::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::command::Command = temp_clone.into();
        real.unregister(command_map)
    }
    /// Returns the current registered state of this command
    pub fn is_registered(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::command::Command::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::command::Command = temp_clone.into();
        real.is_registered()
    }
    /// Returns a list of active aliases of this command
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
    /// Returns a message to be displayed on a failed permission check for this command
    pub fn permission_message(&self) -> Result<Option<String>, Box<dyn std::error::Error>> {
        let temp_clone = crate::command::Command::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::command::Command = temp_clone.into();
        real.permission_message()
    }
    /// Gets a brief description of this command
    pub fn description(&self) -> Result<String, Box<dyn std::error::Error>> {
        let temp_clone = crate::command::Command::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::command::Command = temp_clone.into();
        real.description()
    }
    /// Gets an example usage of this command
    pub fn usage(&self) -> Result<String, Box<dyn std::error::Error>> {
        let temp_clone = crate::command::Command::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::command::Command = temp_clone.into();
        real.usage()
    }
    /// Sets the list of aliases to request on registration for this command.
    /// This is not effective outside of defining aliases in the {@link
    /// PluginDescriptionFile#getCommands()} (under the
    /// `<code>aliases</code>' node) is equivalent to this method.
    pub fn set_aliases(
        &self,
        aliases: Vec<jni::objects::JObject<'mc>>,
    ) -> Result<crate::command::Command<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/util/List;)Lorg/bukkit/command/Command;");
        let raw_val_1 = self
            .jni_ref()
            .new_object("java/util/ArrayList", "()V", vec![])?;
        for v in aliases {
            let map_val_0 = jni::objects::JValueGen::Object(v);
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
    /// Sets a brief description of this command. Defining a description in the
    /// {@link PluginDescriptionFile#getCommands()} (under the
    /// `<code>description</code>' node) is equivalent to this method.
    pub fn set_description(
        &self,
        description: impl Into<String>,
    ) -> Result<crate::command::Command<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::command::Command::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::command::Command = temp_clone.into();
        real.set_description(description)
    }
    /// Sets the message sent when a permission check fails
    pub fn set_permission_message(
        &self,
        permission_message: impl Into<String>,
    ) -> Result<crate::command::Command<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::command::Command::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::command::Command = temp_clone.into();
        real.set_permission_message(permission_message)
    }
    /// Sets the example usage of this command
    pub fn set_usage(
        &self,
        usage: impl Into<String>,
    ) -> Result<crate::command::Command<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::command::Command::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::command::Command = temp_clone.into();
        real.set_usage(usage)
    }

    pub fn broadcast_command_message(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        source: impl Into<crate::command::CommandSender<'mc>>,
        message: impl Into<String>,
        send_to_source: std::option::Option<bool>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        crate::command::Command::broadcast_command_message(jni, source, message, send_to_source)
    }

    #[doc(hidden)]
    pub fn internal_to_string(&self) -> Result<String, Box<dyn std::error::Error>> {
        let temp_clone = crate::command::Command::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::command::Command = temp_clone.into();
        real.internal_to_string()
    }

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
        name: impl Into<String>,
    ) -> Result<crate::command::defaults::ReloadCommand<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/String;)V");
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            jni.new_string(name.into())?,
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

    pub fn execute(
        &self,
        sender: impl Into<crate::command::CommandSender<'mc>>,
        current_alias: impl Into<String>,
        val_args: impl Into<String>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from(
            "(Lorg/bukkit/command/CommandSender;Ljava/lang/String;Ljava/lang/String;)Z",
        );
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(sender.into().jni_object().clone())
        });
        let val_2 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(current_alias.into())?,
        ));
        let val_3 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(val_args.into())?,
        ));
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "execute",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
                jni::objects::JValueGen::from(val_3),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn tab_complete(
        &self,
        sender: impl Into<crate::command::CommandSender<'mc>>,
        alias: impl Into<String>,
        val_args: impl Into<String>,
    ) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/command/CommandSender;Ljava/lang/String;Ljava/lang/String;)Ljava/util/List;");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(sender.into().jni_object().clone())
        });
        let val_2 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(alias.into())?,
        ));
        let val_3 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(val_args.into())?,
        ));
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "tabComplete",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
                jni::objects::JValueGen::from(val_3),
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
    // SUPER CLASS: org.bukkit.command.defaults.BukkitCommand ( ['execute', 'tabComplete'])

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
