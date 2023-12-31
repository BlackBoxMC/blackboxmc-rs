#![allow(deprecated)]
use blackboxmc_general::JNIInstantiatable;
use blackboxmc_general::JNIRaw;
use color_eyre::eyre::Result;
/// Represents a Java plugin loader, allowing plugins in the form of .jar
#[repr(C)]
pub struct JavaPluginLoader<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for JavaPluginLoader<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for JavaPluginLoader<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate JavaPluginLoader from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/plugin/java/JavaPluginLoader")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a JavaPluginLoader object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> JavaPluginLoader<'mc> {
    #[deprecated]

    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<crate::Server<'mc>>,
    ) -> Result<crate::plugin::java::JavaPluginLoader<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/Server;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let cls = jni.find_class("org/bukkit/plugin/java/PluginLoader");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(
            cls,
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = jni.translate_error_no_gen(res)?;
        crate::plugin::java::JavaPluginLoader::from_raw(&jni, res)
    }

    pub fn load_plugin(
        &self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<crate::plugin::Plugin<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/io/File;)Lorg/bukkit/plugin/Plugin;");
        let val_1 = jni::objects::JValueGen::Object(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "loadPlugin",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::plugin::Plugin::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn get_plugin_description(
        &self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<crate::plugin::PluginDescriptionFile<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/io/File;)Lorg/bukkit/plugin/PluginDescriptionFile;");
        let val_1 = jni::objects::JValueGen::Object(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getPluginDescription",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::plugin::PluginDescriptionFile::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn plugin_file_filters(
        &self,
    ) -> Result<Vec<blackboxmc_java::util::regex::JavaPattern<'mc>>, Box<dyn std::error::Error>>
    {
        let sig = String::from("()[Ljava/util/regex/Pattern;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getPluginFileFilters",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        let arr = Into::<jni::objects::JObjectArray>::into(res.l()?);
        let len = self.jni_ref().get_array_length(&arr)?;
        let mut vec = Vec::new();
        for i in 0..len {
            let res = self.jni_ref().get_object_array_element(&arr, i)?;
            vec.push({
                blackboxmc_java::util::regex::JavaPattern::from_raw(&self.jni_ref(), res)?
            });
        }
        Ok(vec)
    }

    pub fn create_registered_listeners(
        &self,
        arg0: impl Into<crate::event::Listener<'mc>>,
        arg1: impl Into<crate::plugin::Plugin<'mc>>,
    ) -> Result<blackboxmc_java::util::JavaMap<'mc>, Box<dyn std::error::Error>> {
        let sig =
            String::from("(Lorg/bukkit/event/Listener;Lorg/bukkit/plugin/Plugin;)Ljava/util/Map;");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let val_2 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg1.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "createRegisteredListeners",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::util::JavaMap::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn enable_plugin(
        &self,
        arg0: impl Into<crate::plugin::Plugin<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/plugin/Plugin;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "enablePlugin",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn disable_plugin(
        &self,
        arg0: impl Into<crate::plugin::Plugin<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/plugin/Plugin;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "disablePlugin",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
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
impl<'mc> Into<crate::plugin::PluginLoader<'mc>> for JavaPluginLoader<'mc> {
    fn into(self) -> crate::plugin::PluginLoader<'mc> {
        crate::plugin::PluginLoader::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting JavaPluginLoader into crate::plugin::PluginLoader")
    }
}
/// Represents a Java plugin and its main class. It contains fundamental methods and fields for a plugin to be loaded and work properly. This is an indirect implementation of <a title="interface in org.bukkit.plugin" href="../Plugin.html"><code>Plugin</code></a>.
#[repr(C)]
pub struct JavaPlugin<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for JavaPlugin<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for JavaPlugin<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate JavaPlugin from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/plugin/java/JavaPlugin")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a JavaPlugin object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> JavaPlugin<'mc> {
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<crate::plugin::java::JavaPlugin<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let cls = jni.find_class("org/bukkit/plugin/java/Plugin");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(cls, sig.as_str(), vec![]);
        let res = jni.translate_error_no_gen(res)?;
        crate::plugin::java::JavaPlugin::from_raw(&jni, res)
    }

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

    pub fn description(
        &self,
    ) -> Result<crate::plugin::PluginDescriptionFile<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/plugin/PluginDescriptionFile;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getDescription", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::plugin::PluginDescriptionFile::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
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

    pub fn get_command(
        &self,
        arg0: impl Into<String>,
    ) -> Result<crate::command::PluginCommand<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/String;)Lorg/bukkit/command/PluginCommand;");
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
        crate::command::PluginCommand::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn get_default_biome_provider(
        &self,
        arg0: impl Into<String>,
        arg1: impl Into<String>,
    ) -> Result<crate::generator::BiomeProvider<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from(
            "(Ljava/lang/String;Ljava/lang/String;)Lorg/bukkit/generator/BiomeProvider;",
        );
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(arg0.into())?,
        ));
        let val_2 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(arg1.into())?,
        ));
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getDefaultBiomeProvider",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::generator::BiomeProvider::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn is_enabled(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isEnabled", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn get_plugin(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: jni::objects::JClass<'mc>,
    ) -> Result<crate::plugin::java::JavaPlugin<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/Class;)Lorg/bukkit/plugin/java/Plugin;");
        let val_1 = jni::objects::JValueGen::Object(arg0.into());
        let cls = jni.find_class("org/bukkit/plugin/java/Plugin");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(
            cls,
            "getPlugin",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        crate::plugin::java::JavaPlugin::from_raw(&jni, obj)
    }

    pub fn config(
        &self,
    ) -> Result<crate::configuration::file::FileConfiguration<'mc>, Box<dyn std::error::Error>>
    {
        let sig = String::from("()Lorg/bukkit/configuration/file/FileConfiguration;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getConfig", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::configuration::file::FileConfiguration::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn data_folder(&self) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/io/File;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getDataFolder", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l()?)
    }

    pub fn save_config(&self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "saveConfig", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn save_default_config(&self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "saveDefaultConfig",
            sig.as_str(),
            vec![],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn save_resource(
        &self,
        arg0: impl Into<String>,
        arg1: bool,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/String;Z)V");
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(arg0.into())?,
        ));
        let val_2 = jni::objects::JValueGen::Bool(arg1.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "saveResource",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
            ],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn reload_config(&self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "reloadConfig", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn plugin_loader(
        &self,
    ) -> Result<crate::plugin::PluginLoader<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/plugin/PluginLoader;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getPluginLoader", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::plugin::PluginLoader::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn on_disable(&self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "onDisable", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn on_load(&self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "onLoad", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn on_enable(&self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "onEnable", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn is_naggable(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isNaggable", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// <span class="descfrm-type-label">Description copied from interface:&nbsp;<code><a href="../Plugin.html#setNaggable(boolean)">Plugin</a></code></span>
    /// Set naggable state
    pub fn set_naggable(&self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        let val_1 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setNaggable",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn get_default_world_generator(
        &self,
        arg0: impl Into<String>,
        arg1: impl Into<String>,
    ) -> Result<crate::generator::ChunkGenerator<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from(
            "(Ljava/lang/String;Ljava/lang/String;)Lorg/bukkit/generator/ChunkGenerator;",
        );
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(arg0.into())?,
        ));
        let val_2 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(arg1.into())?,
        ));
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getDefaultWorldGenerator",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::generator::ChunkGenerator::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn get_providing_plugin(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: jni::objects::JClass<'mc>,
    ) -> Result<crate::plugin::java::JavaPlugin<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/Class;)Lorg/bukkit/plugin/java/Plugin;");
        let val_1 = jni::objects::JValueGen::Object(arg0.into());
        let cls = jni.find_class("org/bukkit/plugin/java/Plugin");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(
            cls,
            "getProvidingPlugin",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        crate::plugin::java::JavaPlugin::from_raw(&jni, obj)
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

    pub fn get_resource(
        &self,
        arg0: impl Into<String>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/String;)Ljava/io/InputStream;");
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(arg0.into())?,
        ));
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getResource",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l()?)
    }

    pub fn logger(
        &self,
    ) -> Result<blackboxmc_java::util::logging::JavaLogger<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/logging/Logger;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getLogger", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::util::logging::JavaLogger::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    // SUPER CLASS: PluginBase
    pub fn name(&self) -> Result<String, Box<dyn std::error::Error>> {
        let temp_clone = crate::plugin::PluginBase::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::plugin::Plugin = temp_clone.into();
        real.name()
    }
    // SUPER CLASS: TabExecutor
    pub fn equals(
        &self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::plugin::PluginBase::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::plugin::PluginBase = temp_clone.into();
        real.equals(arg0)
    }
    pub fn hash_code(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let temp_clone = crate::plugin::PluginBase::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::plugin::PluginBase = temp_clone.into();
        real.hash_code()
    }
    // SUPER CLASS: Object

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}

impl<'mc> std::string::ToString for JavaPlugin<'mc> {
    fn to_string(&self) -> String {
        match &self.internal_to_string() {
            Ok(a) => a.clone(),
            Err(err) => format!("Error calling JavaPlugin.toString: {}", err),
        }
    }
}

impl<'mc> Into<crate::plugin::PluginBase<'mc>> for JavaPlugin<'mc> {
    fn into(self) -> crate::plugin::PluginBase<'mc> {
        crate::plugin::PluginBase::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting JavaPlugin into crate::plugin::PluginBase")
    }
}
