use crate::JNIRaw;
pub struct JavaPluginLoader<'mc>(
    pub(crate) crate::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> crate::JNIRaw<'mc> for JavaPluginLoader<'mc> {
    fn jni_ref(&self) -> crate::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JavaPluginLoader<'mc> {
    pub fn new(
        jni: crate::SharedJNIEnv<'mc>,
        arg0: crate::bukkit::Server<'mc>,
    ) -> Result<crate::bukkit::plugin::java::JavaPluginLoader<'mc>, Box<dyn std::error::Error>>
    {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.1.clone()) };
        let cls = &jni.find_class("org/bukkit/plugin/java/JavaPluginLoader")?;
        let res = jni.new_object(
            cls,
            "(Lorg/bukkit/Server;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        let ret = { crate::bukkit::plugin::java::JavaPluginLoader(jni, res) };
        Ok(ret)
    }
    pub fn from_raw(
        env: &crate::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate JavaPluginLoader from null object.").into(),
            );
        }
        let cls = env.jni.borrow().get_object_class(&obj)?;
        let name_raw = env.call_method(cls, "getName", "()Ljava/lang/String;", &[])?;
        let oh = name_raw.l()?.into();
        let what = env.get_string(&oh)?;
        let name = what.to_string_lossy();
        if !name.ends_with("JavaPluginLoader") {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a JavaPluginLoader object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    pub fn enable_plugin(
        &mut self,
        arg0: crate::bukkit::plugin::Plugin<'mc>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.1.clone()) };
        self.jni_ref().call_method(
            &self.jni_object(),
            "enablePlugin",
            "(Lorg/bukkit/plugin/Plugin;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn disable_plugin(
        &mut self,
        arg0: crate::bukkit::plugin::Plugin<'mc>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.1.clone()) };
        self.jni_ref().call_method(
            &self.jni_object(),
            "disablePlugin",
            "(Lorg/bukkit/plugin/Plugin;)V",
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
pub struct JavaPlugin<'mc>(
    pub(crate) crate::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> crate::JNIRaw<'mc> for JavaPlugin<'mc> {
    fn jni_ref(&self) -> crate::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JavaPlugin<'mc> {
    pub fn new(
        jni: crate::SharedJNIEnv<'mc>,
    ) -> Result<crate::bukkit::plugin::java::JavaPlugin<'mc>, Box<dyn std::error::Error>> {
        let cls = &jni.find_class("org/bukkit/plugin/java/JavaPlugin")?;
        let res = jni.new_object(cls, "()V", &[])?;
        let ret = { crate::bukkit::plugin::java::JavaPlugin(jni, res) };
        Ok(ret)
    }
    pub fn from_raw(
        env: &crate::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate JavaPlugin from null object.").into());
        }
        let cls = env.jni.borrow().get_object_class(&obj)?;
        let name_raw = env.call_method(cls, "getName", "()Ljava/lang/String;", &[])?;
        let oh = name_raw.l()?.into();
        let what = env.get_string(&oh)?;
        let name = what.to_string_lossy();
        if !name.ends_with("JavaPlugin") {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a JavaPlugin object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
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
    pub fn get_resource(
        &mut self,
        arg0: String,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JObject::from(self.jni_ref().new_string(arg0).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getResource",
            "(Ljava/lang/String;)Ljava/io/InputStream;",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(res.l().unwrap())
    }
    pub fn logger(&mut self) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getLogger",
            "()Ljava/util/logging/Logger;",
            &[],
        )?;
        Ok(res.l().unwrap())
    }
    pub fn server(&mut self) -> Result<crate::bukkit::Server<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getServer",
            "()Lorg/bukkit/Server;",
            &[],
        )?;
        let ret = {
            crate::bukkit::Server(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn description(
        &mut self,
    ) -> Result<crate::bukkit::plugin::PluginDescriptionFile<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getDescription",
            "()Lorg/bukkit/plugin/PluginDescriptionFile;",
            &[],
        )?;
        let ret = {
            crate::bukkit::plugin::PluginDescriptionFile(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn on_command(
        &mut self,
        arg0: crate::bukkit::command::CommandSender<'mc>,
        arg1: crate::bukkit::command::Command<'mc>,
        arg2: String,
        _arg3: Vec<String>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.1.clone()) };
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg1.1.clone()) };
        let val_2 = jni::objects::JObject::from(self.jni_ref().new_string(arg2).unwrap());
        let res =
self.jni_ref().call_method(&self.jni_object(),"onCommand","(Lorg/bukkit/command/CommandSender;Lorg/bukkit/command/Command;Ljava/lang/String;Ljava/lang/String;)Z",&[jni::objects::JValueGen::from(&val_0),jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)])?;
        Ok(res.z().unwrap())
    }
    pub fn get_command(
        &mut self,
        arg0: String,
    ) -> Result<crate::bukkit::command::PluginCommand<'mc>, Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JObject::from(self.jni_ref().new_string(arg0).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getCommand",
            "(Ljava/lang/String;)Lorg/bukkit/command/PluginCommand;",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        let ret = {
            crate::bukkit::command::PluginCommand(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn get_default_biome_provider(
        &mut self,
        arg0: String,
        arg1: String,
    ) -> Result<crate::bukkit::generator::BiomeProvider<'mc>, Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JObject::from(self.jni_ref().new_string(arg0).unwrap());
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg1).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getDefaultBiomeProvider",
            "(Ljava/lang/String;Ljava/lang/String;)Lorg/bukkit/generator/BiomeProvider;",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        )?;
        let ret = {
            crate::bukkit::generator::BiomeProvider(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn is_enabled(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isEnabled", "()Z", &[])?;
        Ok(res.z().unwrap())
    }
    pub fn get_plugin(
        mut jni: crate::SharedJNIEnv<'mc>,
        arg0: jni::objects::JClass<'mc>,
    ) -> Result<crate::bukkit::plugin::java::JavaPlugin<'mc>, Box<dyn std::error::Error>> {
        let val_0 = arg0;
        let cls = &jni.find_class("org/bukkit/plugin/java/JavaPlugin")?;
        let res = jni.call_static_method(
            cls,
            "getPlugin",
            "(Ljava/lang/Class;)Lorg/bukkit/plugin/java/JavaPlugin;",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        let ret = {
            let obj = res.l()?;
            crate::bukkit::plugin::java::JavaPlugin(jni, obj)
        };
        Ok(ret)
    }
    pub fn config(
        &mut self,
    ) -> Result<
        crate::bukkit::configuration::file::FileConfiguration<'mc>,
        Box<dyn std::error::Error>,
    > {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getConfig",
            "()Lorg/bukkit/configuration/file/FileConfiguration;",
            &[],
        )?;
        let ret = {
            crate::bukkit::configuration::file::FileConfiguration(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn save_config(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.jni_ref()
            .call_method(&self.jni_object(), "saveConfig", "()V", &[])?;
        Ok(())
    }
    pub fn save_default_config(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.jni_ref()
            .call_method(&self.jni_object(), "saveDefaultConfig", "()V", &[])?;
        Ok(())
    }
    pub fn save_resource(
        &mut self,
        arg0: String,
        arg1: bool,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JObject::from(self.jni_ref().new_string(arg0).unwrap());
        let val_1 = jni::objects::JValueGen::Bool(arg1.into());
        self.jni_ref().call_method(
            &self.jni_object(),
            "saveResource",
            "(Ljava/lang/String;Z)V",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        )?;
        Ok(())
    }
    pub fn reload_config(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.jni_ref()
            .call_method(&self.jni_object(), "reloadConfig", "()V", &[])?;
        Ok(())
    }
    pub fn plugin_loader(
        &mut self,
    ) -> Result<crate::bukkit::plugin::PluginLoader<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getPluginLoader",
            "()Lorg/bukkit/plugin/PluginLoader;",
            &[],
        )?;
        let ret = {
            crate::bukkit::plugin::PluginLoader(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn on_disable(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.jni_ref()
            .call_method(&self.jni_object(), "onDisable", "()V", &[])?;
        Ok(())
    }
    pub fn on_load(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.jni_ref()
            .call_method(&self.jni_object(), "onLoad", "()V", &[])?;
        Ok(())
    }
    pub fn on_enable(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.jni_ref()
            .call_method(&self.jni_object(), "onEnable", "()V", &[])?;
        Ok(())
    }
    pub fn is_naggable(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isNaggable", "()Z", &[])?;
        Ok(res.z().unwrap())
    }
    pub fn set_naggable(&mut self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Bool(arg0.into());
        self.jni_ref().call_method(
            &self.jni_object(),
            "setNaggable",
            "(Z)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn get_default_world_generator(
        &mut self,
        arg0: String,
        arg1: String,
    ) -> Result<crate::bukkit::generator::ChunkGenerator<'mc>, Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JObject::from(self.jni_ref().new_string(arg0).unwrap());
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg1).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getDefaultWorldGenerator",
            "(Ljava/lang/String;Ljava/lang/String;)Lorg/bukkit/generator/ChunkGenerator;",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        )?;
        let ret = {
            crate::bukkit::generator::ChunkGenerator(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn get_providing_plugin(
        mut jni: crate::SharedJNIEnv<'mc>,
        arg0: jni::objects::JClass<'mc>,
    ) -> Result<crate::bukkit::plugin::java::JavaPlugin<'mc>, Box<dyn std::error::Error>> {
        let val_0 = arg0;
        let cls = &jni.find_class("org/bukkit/plugin/java/JavaPlugin")?;
        let res = jni.call_static_method(
            cls,
            "getProvidingPlugin",
            "(Ljava/lang/Class;)Lorg/bukkit/plugin/java/JavaPlugin;",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        let ret = {
            let obj = res.l()?;
            crate::bukkit::plugin::java::JavaPlugin(jni, obj)
        };
        Ok(ret)
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
    pub fn hash_code(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hashCode", "()I", &[])?;
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
