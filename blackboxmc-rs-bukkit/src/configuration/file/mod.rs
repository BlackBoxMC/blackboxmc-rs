#![allow(deprecated)]
use blackboxmc_general::JNIRaw;
use color_eyre::eyre::Result;
/// This is a base class for all File based implementations of <a href="../Configuration.html" title="interface in org.bukkit.configuration"><code>Configuration</code></a>
pub struct FileConfiguration<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for FileConfiguration<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> FileConfiguration<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate FileConfiguration from null object.").into(),
            );
        }
        let (valid, name) =
            env.validate_name(&obj, "org/bukkit/configuration/file/FileConfiguration")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a FileConfiguration object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    pub fn new(
        jni: blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: std::option::Option<impl Into<&'mc crate::configuration::Configuration<'mc>>>,
    ) -> Result<crate::configuration::file::FileConfiguration<'mc>, Box<dyn std::error::Error>>
    {
        let val_1 =
            unsafe { jni::objects::JObject::from_raw(arg0.unwrap().into().jni_object().clone()) };
        let cls = &jni.find_class("org/bukkit/configuration/file/FileConfiguration")?;
        let res = jni.new_object(
            cls,
            "(Lorg/bukkit/configuration/Configuration;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        )?;
        crate::configuration::file::FileConfiguration::from_raw(&jni, res)
    }
    /// Saves this <a title="class in org.bukkit.configuration.file" href="FileConfiguration.html"><code>FileConfiguration</code></a> to a string, and returns it.
    pub fn save_to_string(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "saveToString",
            "()Ljava/lang/String;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    /// Loads this <a title="class in org.bukkit.configuration.file" href="FileConfiguration.html"><code>FileConfiguration</code></a> from the specified string, as opposed to from file.
    /// <p>All the values contained within this configuration will be removed, leaving only settings and defaults, and the new values will be loaded from the given string.</p>
    /// <p>If the string is invalid in any way, an exception will be thrown.</p>
    pub fn load_from_string(
        &mut self,
        arg0: impl Into<&'mc String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "loadFromString",
            "(Ljava/lang/String;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Loads this <a href="FileConfiguration.html" title="class in org.bukkit.configuration.file"><code>FileConfiguration</code></a> from the specified location.
    /// <p>All the values contained within this configuration will be removed, leaving only settings and defaults, and the new values will be loaded from the given file.</p>
    /// <p>If the file cannot be loaded for any reason, an exception will be thrown.</p>
    /// Loads this <a href="FileConfiguration.html" title="class in org.bukkit.configuration.file"><code>FileConfiguration</code></a> from the specified reader.
    /// <p>All the values contained within this configuration will be removed, leaving only settings and defaults, and the new values will be loaded from the given stream.</p>
    /// Loads this <a href="FileConfiguration.html" title="class in org.bukkit.configuration.file"><code>FileConfiguration</code></a> from the specified location.
    /// <p>All the values contained within this configuration will be removed, leaving only settings and defaults, and the new values will be loaded from the given file.</p>
    /// <p>If the file cannot be loaded for any reason, an exception will be thrown.</p>
    /// Loads this <a href="FileConfiguration.html" title="class in org.bukkit.configuration.file"><code>FileConfiguration</code></a> from the specified string, as opposed to from file.
    /// <p>All the values contained within this configuration will be removed, leaving only settings and defaults, and the new values will be loaded from the given string.</p>
    /// <p>If the string is invalid in any way, an exception will be thrown.</p>
    pub unsafe fn load_with_string(
        &mut self,
        arg0: std::option::Option<jni::objects::JObject<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = arg0.unwrap();
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "load",
            "(Ljava/io/Reader;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Loads this <a title="class in org.bukkit.configuration.file" href="FileConfiguration.html"><code>FileConfiguration</code></a> from the specified location.
    /// <p>All the values contained within this configuration will be removed, leaving only settings and defaults, and the new values will be loaded from the given file.</p>
    /// <p>If the file cannot be loaded for any reason, an exception will be thrown.</p>
    /// Loads this <a href="FileConfiguration.html" title="class in org.bukkit.configuration.file"><code>FileConfiguration</code></a> from the specified reader.
    /// <p>All the values contained within this configuration will be removed, leaving only settings and defaults, and the new values will be loaded from the given stream.</p>
    /// Loads this <a href="FileConfiguration.html" title="class in org.bukkit.configuration.file"><code>FileConfiguration</code></a> from the specified location.
    /// <p>All the values contained within this configuration will be removed, leaving only settings and defaults, and the new values will be loaded from the given file.</p>
    /// <p>If the file cannot be loaded for any reason, an exception will be thrown.</p>
    /// Loads this <a href="FileConfiguration.html" title="class in org.bukkit.configuration.file"><code>FileConfiguration</code></a> from the specified string, as opposed to from file.
    /// <p>All the values contained within this configuration will be removed, leaving only settings and defaults, and the new values will be loaded from the given string.</p>
    /// <p>If the string is invalid in any way, an exception will be thrown.</p>
    pub unsafe fn load_with_file(
        &mut self,
        arg0: std::option::Option<jni::objects::JObject<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = arg0.unwrap();
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "load",
            "(Ljava/io/File;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Saves this <a href="FileConfiguration.html" title="class in org.bukkit.configuration.file"><code>FileConfiguration</code></a> to the specified location.
    /// <p>If the file does not exist, it will be created. If already exists, it will be overwritten. If it cannot be overwritten or created, an exception will be thrown.</p>
    /// <p>This method will save using the system default encoding, or possibly using UTF8.</p>
    /// Saves this <a href="FileConfiguration.html" title="class in org.bukkit.configuration.file"><code>FileConfiguration</code></a> to the specified location.
    /// <p>If the file does not exist, it will be created. If already exists, it will be overwritten. If it cannot be overwritten or created, an exception will be thrown.</p>
    /// <p>This method will save using the system default encoding, or possibly using UTF8.</p>
    /// Saves this <a title="class in org.bukkit.configuration.file" href="FileConfiguration.html"><code>FileConfiguration</code></a> to a string, and returns it.
    pub fn save_with_file(
        &mut self,
        arg0: std::option::Option<impl Into<&'mc String>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 =
            jni::objects::JObject::from(self.jni_ref().new_string(arg0.unwrap().into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "save",
            "(Ljava/lang/String;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// <span class="descfrm-type-label">Description copied from interface:&nbsp;<code><a href="../Configuration.html#options()">Configuration</a></code></span>
    /// Gets the <a title="class in org.bukkit.configuration" href="../ConfigurationOptions.html"><code>ConfigurationOptions</code></a> for this <a href="../Configuration.html" title="interface in org.bukkit.configuration"><code>Configuration</code></a>.
    /// <p>All setters through this method are chainable.</p>
    pub fn options(
        &mut self,
    ) -> Result<crate::configuration::file::FileConfigurationOptions<'mc>, Box<dyn std::error::Error>>
    {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "options",
            "()Lorg/bukkit/configuration/file/FileConfigurationOptions;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::configuration::file::FileConfigurationOptions::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn add_default(
        &mut self,
        arg0: impl Into<&'mc String>,
        arg1: jni::objects::JObject<'mc>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let val_2 = arg1;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "addDefault",
            "(Ljava/lang/String;Ljava/lang/Object;)V",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn defaults(
        &mut self,
    ) -> Result<crate::configuration::Configuration<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getDefaults",
            "()Lorg/bukkit/configuration/Configuration;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::configuration::Configuration::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn add_defaults_with_configuration(
        &mut self,
        arg0: std::option::Option<impl Into<&'mc blackboxmc_java::JavaMap<'mc>>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 =
            unsafe { jni::objects::JObject::from_raw(arg0.unwrap().into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "addDefaults",
            "(Ljava/util/Map;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn set_defaults(
        &mut self,
        arg0: impl Into<&'mc crate::configuration::Configuration<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setDefaults",
            "(Lorg/bukkit/configuration/Configuration;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn parent(
        &mut self,
    ) -> Result<crate::configuration::ConfigurationSection<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getParent",
            "()Lorg/bukkit/configuration/ConfigurationSection;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::configuration::ConfigurationSection::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn get_string_with_string(
        &mut self,
        arg0: std::option::Option<impl Into<&'mc String>>,
        arg1: std::option::Option<impl Into<&'mc String>>,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let val_1 =
            jni::objects::JObject::from(self.jni_ref().new_string(arg0.unwrap().into()).unwrap());
        let val_2 =
            jni::objects::JObject::from(self.jni_ref().new_string(arg1.unwrap().into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getString",
            "(Ljava/lang/String;Ljava/lang/String;)Ljava/lang/String;",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }

    pub fn get_keys(
        &mut self,
        arg0: bool,
    ) -> Result<blackboxmc_java::JavaSet<'mc>, Box<dyn std::error::Error>> {
        // -2
        let val_1 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getKeys",
            "(Z)Ljava/util/Set;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::JavaSet::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn get_offline_player_with_string(
        &mut self,
        arg0: std::option::Option<impl Into<&'mc String>>,
        arg1: std::option::Option<impl Into<&'mc crate::OfflinePlayer<'mc>>>,
    ) -> Result<crate::OfflinePlayer<'mc>, Box<dyn std::error::Error>> {
        let val_1 =
            jni::objects::JObject::from(self.jni_ref().new_string(arg0.unwrap().into()).unwrap());
        let val_2 =
            unsafe { jni::objects::JObject::from_raw(arg1.unwrap().into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getOfflinePlayer",
            "(Ljava/lang/String;Lorg/bukkit/OfflinePlayer;)Lorg/bukkit/OfflinePlayer;",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::OfflinePlayer::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn get_color_with_string(
        &mut self,
        arg0: std::option::Option<impl Into<&'mc String>>,
        arg1: std::option::Option<impl Into<&'mc crate::Color<'mc>>>,
    ) -> Result<crate::Color<'mc>, Box<dyn std::error::Error>> {
        let val_1 =
            jni::objects::JObject::from(self.jni_ref().new_string(arg0.unwrap().into()).unwrap());
        let val_2 =
            unsafe { jni::objects::JObject::from_raw(arg1.unwrap().into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getColor",
            "(Ljava/lang/String;Lorg/bukkit/Color;)Lorg/bukkit/Color;",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::Color::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn get_item_stack_with_string(
        &mut self,
        arg0: std::option::Option<impl Into<&'mc String>>,
        arg1: std::option::Option<impl Into<&'mc crate::inventory::ItemStack<'mc>>>,
    ) -> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>> {
        let val_1 =
            jni::objects::JObject::from(self.jni_ref().new_string(arg0.unwrap().into()).unwrap());
        let val_2 =
            unsafe { jni::objects::JObject::from_raw(arg1.unwrap().into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getItemStack",
            "(Ljava/lang/String;Lorg/bukkit/inventory/ItemStack;)Lorg/bukkit/inventory/ItemStack;",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::ItemStack::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn get_values(
        &mut self,
        arg0: bool,
    ) -> Result<blackboxmc_java::JavaMap<'mc>, Box<dyn std::error::Error>> {
        // -2
        let val_1 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getValues",
            "(Z)Ljava/util/Map;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::JavaMap::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn create_path_with_configuration_section(
        jni: blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<&'mc crate::configuration::ConfigurationSection<'mc>>,
        arg1: std::option::Option<impl Into<&'mc String>>,
        arg2: std::option::Option<impl Into<&'mc crate::configuration::ConfigurationSection<'mc>>>,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let val_2 = jni::objects::JObject::from(jni.new_string(arg1.unwrap().into()).unwrap());
        let val_3 =
            unsafe { jni::objects::JObject::from_raw(arg2.unwrap().into().jni_object().clone()) };
        let cls = &jni.find_class("java/lang/String")?;
        let res = jni.call_static_method(cls,"createPath",
"(Lorg/bukkit/configuration/ConfigurationSection;Ljava/lang/String;Lorg/bukkit/configuration/ConfigurationSection;)Ljava/lang/String;",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2),jni::objects::JValueGen::from(&val_3)])?;
        Ok(jni
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }

    pub fn default_section(
        &mut self,
    ) -> Result<crate::configuration::ConfigurationSection<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getDefaultSection",
            "()Lorg/bukkit/configuration/ConfigurationSection;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::configuration::ConfigurationSection::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn current_path(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getCurrentPath",
            "()Ljava/lang/String;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }

    pub fn is_configuration_section(
        &mut self,
        arg0: impl Into<&'mc String>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "isConfigurationSection",
            "(Ljava/lang/String;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn get_configuration_section(
        &mut self,
        arg0: impl Into<&'mc String>,
    ) -> Result<crate::configuration::ConfigurationSection<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getConfigurationSection",
            "(Ljava/lang/String;)Lorg/bukkit/configuration/ConfigurationSection;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::configuration::ConfigurationSection::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn create_section_with_string(
        &mut self,
        arg0: std::option::Option<impl Into<&'mc String>>,
        arg1: std::option::Option<impl Into<&'mc blackboxmc_java::JavaMap<'mc>>>,
    ) -> Result<crate::configuration::ConfigurationSection<'mc>, Box<dyn std::error::Error>> {
        let val_1 =
            jni::objects::JObject::from(self.jni_ref().new_string(arg0.unwrap().into()).unwrap());
        let val_2 =
            unsafe { jni::objects::JObject::from_raw(arg1.unwrap().into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "createSection",
            "(Ljava/lang/String;Ljava/util/Map;)Lorg/bukkit/configuration/ConfigurationSection;",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::configuration::ConfigurationSection::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn is_string(
        &mut self,
        arg0: impl Into<&'mc String>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "isString",
            "(Ljava/lang/String;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn is_int(
        &mut self,
        arg0: impl Into<&'mc String>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "isInt",
            "(Ljava/lang/String;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn is_boolean(
        &mut self,
        arg0: impl Into<&'mc String>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "isBoolean",
            "(Ljava/lang/String;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn is_double(
        &mut self,
        arg0: impl Into<&'mc String>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "isDouble",
            "(Ljava/lang/String;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn is_long(
        &mut self,
        arg0: impl Into<&'mc String>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "isLong",
            "(Ljava/lang/String;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn is_list(
        &mut self,
        arg0: impl Into<&'mc String>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "isList",
            "(Ljava/lang/String;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn get_string_list(
        &mut self,
        arg0: impl Into<&'mc String>,
    ) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getStringList",
            "(Ljava/lang/String;)Ljava/util/List;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let mut list = blackboxmc_java::JavaList::from_raw(&self.0, res.l()?)?;
        let size = list.size()?;
        for i in 0..=size {
            let obj = list.get(i)?;
            new_vec.push(
                self.jni_ref()
                    .get_string(unsafe { &jni::objects::JString::from_raw(*obj) })?
                    .to_string_lossy()
                    .to_string(),
            );
        }
        Ok(new_vec)
    }

    pub fn get_serializable_with_string(
        &mut self,
        arg0: impl Into<&'mc String>,
        arg1: std::option::Option<jni::objects::JClass<'mc>>,
        arg2: std::option::Option<
            impl Into<&'mc crate::configuration::serialization::ConfigurationSerializable<'mc>>,
        >,
    ) -> Result<
        crate::configuration::serialization::ConfigurationSerializable<'mc>,
        Box<dyn std::error::Error>,
    > {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let val_2 = arg1.unwrap();
        let val_3 =
            unsafe { jni::objects::JObject::from_raw(arg2.unwrap().into().jni_object().clone()) };
        let res = self.jni_ref().call_method(&self.jni_object(),"getSerializable","(Ljava/lang/String;Ljava/lang/Class;Lorg/bukkit/configuration/serialization/ConfigurationSerializable;)Lorg/bukkit/configuration/serialization/ConfigurationSerializable;",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2),jni::objects::JValueGen::from(&val_3)]);
        let res = self.jni_ref().translate_error(res)?;
        crate::configuration::serialization::ConfigurationSerializable::from_raw(
            &self.jni_ref(),
            unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) },
        )
    }

    pub fn get_vector_with_string(
        &mut self,
        arg0: std::option::Option<impl Into<&'mc String>>,
        arg1: std::option::Option<impl Into<&'mc crate::util::Vector<'mc>>>,
    ) -> Result<crate::util::Vector<'mc>, Box<dyn std::error::Error>> {
        let val_1 =
            jni::objects::JObject::from(self.jni_ref().new_string(arg0.unwrap().into()).unwrap());
        let val_2 =
            unsafe { jni::objects::JObject::from_raw(arg1.unwrap().into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getVector",
            "(Ljava/lang/String;Lorg/bukkit/util/Vector;)Lorg/bukkit/util/Vector;",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::util::Vector::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn is_vector(
        &mut self,
        arg0: impl Into<&'mc String>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "isVector",
            "(Ljava/lang/String;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn is_offline_player(
        &mut self,
        arg0: impl Into<&'mc String>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "isOfflinePlayer",
            "(Ljava/lang/String;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn is_item_stack(
        &mut self,
        arg0: impl Into<&'mc String>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "isItemStack",
            "(Ljava/lang/String;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn is_color(
        &mut self,
        arg0: impl Into<&'mc String>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "isColor",
            "(Ljava/lang/String;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn is_location(
        &mut self,
        arg0: impl Into<&'mc String>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "isLocation",
            "(Ljava/lang/String;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn get_comments(
        &mut self,
        arg0: impl Into<&'mc String>,
    ) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getComments",
            "(Ljava/lang/String;)Ljava/util/List;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let mut list = blackboxmc_java::JavaList::from_raw(&self.0, res.l()?)?;
        let size = list.size()?;
        for i in 0..=size {
            let obj = list.get(i)?;
            new_vec.push(
                self.jni_ref()
                    .get_string(unsafe { &jni::objects::JString::from_raw(*obj) })?
                    .to_string_lossy()
                    .to_string(),
            );
        }
        Ok(new_vec)
    }

    pub fn get_inline_comments(
        &mut self,
        arg0: impl Into<&'mc String>,
    ) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getInlineComments",
            "(Ljava/lang/String;)Ljava/util/List;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let mut list = blackboxmc_java::JavaList::from_raw(&self.0, res.l()?)?;
        let size = list.size()?;
        for i in 0..=size {
            let obj = list.get(i)?;
            new_vec.push(
                self.jni_ref()
                    .get_string(unsafe { &jni::objects::JString::from_raw(*obj) })?
                    .to_string_lossy()
                    .to_string(),
            );
        }
        Ok(new_vec)
    }

    pub fn set_comments(
        &mut self,
        arg0: impl Into<&'mc String>,
        arg1: Vec<impl Into<String>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let raw_val_2 = self
            .jni_ref()
            .new_object("java/util/ArrayList", "()V", &[])
            .unwrap();
        for v in arg1 {
            let map_val_0 =
                jni::objects::JObject::from(self.jni_ref().new_string(v.into()).unwrap());
            self.jni_ref().call_method(
                &raw_val_2,
                "add",
                "(Ljava/Lang/Object)V",
                &[jni::objects::JValueGen::from(&map_val_0)],
            )?;
        }
        let val_2 = jni::objects::JValueGen::Object(raw_val_2);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setComments",
            "(Ljava/lang/String;Ljava/util/List;)V",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn set_inline_comments(
        &mut self,
        arg0: impl Into<&'mc String>,
        arg1: Vec<impl Into<String>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let raw_val_2 = self
            .jni_ref()
            .new_object("java/util/ArrayList", "()V", &[])
            .unwrap();
        for v in arg1 {
            let map_val_0 =
                jni::objects::JObject::from(self.jni_ref().new_string(v.into()).unwrap());
            self.jni_ref().call_method(
                &raw_val_2,
                "add",
                "(Ljava/Lang/Object)V",
                &[jni::objects::JValueGen::from(&map_val_0)],
            )?;
        }
        let val_2 = jni::objects::JValueGen::Object(raw_val_2);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setInlineComments",
            "(Ljava/lang/String;Ljava/util/List;)V",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn name(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getName", "()Ljava/lang/String;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }

    pub fn get_with_string(
        &mut self,
        arg0: std::option::Option<impl Into<&'mc String>>,
        arg1: std::option::Option<jni::objects::JObject<'mc>>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let val_1 =
            jni::objects::JObject::from(self.jni_ref().new_string(arg0.unwrap().into()).unwrap());
        let val_2 = arg1.unwrap();
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "get",
            "(Ljava/lang/String;Ljava/lang/Object;)Ljava/lang/Object;",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }

    pub fn to_string(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "toString", "()Ljava/lang/String;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }

    pub fn get_boolean_with_string(
        &mut self,
        arg0: std::option::Option<impl Into<&'mc String>>,
        arg1: std::option::Option<bool>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 =
            jni::objects::JObject::from(self.jni_ref().new_string(arg0.unwrap().into()).unwrap());
        // 0
        let val_2 = jni::objects::JValueGen::Bool(arg1.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getBoolean",
            "(Ljava/lang/String;Z)Z",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn get_int_with_string(
        &mut self,
        arg0: std::option::Option<impl Into<&'mc String>>,
        arg1: std::option::Option<i32>,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        let val_1 =
            jni::objects::JObject::from(self.jni_ref().new_string(arg0.unwrap().into()).unwrap());
        let val_2 = jni::objects::JValueGen::Int(arg1.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getInt",
            "(Ljava/lang/String;I)I",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }

    pub fn get_long_with_string(
        &mut self,
        arg0: std::option::Option<impl Into<&'mc String>>,
        arg1: std::option::Option<i64>,
    ) -> Result<i64, Box<dyn std::error::Error>> {
        let val_1 =
            jni::objects::JObject::from(self.jni_ref().new_string(arg0.unwrap().into()).unwrap());
        let val_2 = jni::objects::JValueGen::Long(arg1.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getLong",
            "(Ljava/lang/String;J)J",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.j().unwrap())
    }

    pub fn get_double_with_string(
        &mut self,
        arg0: std::option::Option<impl Into<&'mc String>>,
        arg1: std::option::Option<f64>,
    ) -> Result<f64, Box<dyn std::error::Error>> {
        let val_1 =
            jni::objects::JObject::from(self.jni_ref().new_string(arg0.unwrap().into()).unwrap());
        let val_2 = jni::objects::JValueGen::Double(arg1.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getDouble",
            "(Ljava/lang/String;D)D",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.d().unwrap())
    }

    pub fn contains_with_string(
        &mut self,
        arg0: std::option::Option<impl Into<&'mc String>>,
        arg1: std::option::Option<bool>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 =
            jni::objects::JObject::from(self.jni_ref().new_string(arg0.unwrap().into()).unwrap());
        // 0
        let val_2 = jni::objects::JValueGen::Bool(arg1.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "contains",
            "(Ljava/lang/String;Z)Z",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn get_location_with_string(
        &mut self,
        arg0: std::option::Option<impl Into<&'mc String>>,
        arg1: std::option::Option<impl Into<&'mc crate::Location<'mc>>>,
    ) -> Result<crate::Location<'mc>, Box<dyn std::error::Error>> {
        let val_1 =
            jni::objects::JObject::from(self.jni_ref().new_string(arg0.unwrap().into()).unwrap());
        let val_2 =
            unsafe { jni::objects::JObject::from_raw(arg1.unwrap().into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getLocation",
            "(Ljava/lang/String;Lorg/bukkit/Location;)Lorg/bukkit/Location;",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::Location::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn set(
        &mut self,
        arg0: impl Into<&'mc String>,
        arg1: jni::objects::JObject<'mc>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let val_2 = arg1;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "set",
            "(Ljava/lang/String;Ljava/lang/Object;)V",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn is_set(
        &mut self,
        arg0: impl Into<&'mc String>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "isSet",
            "(Ljava/lang/String;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn root(
        &mut self,
    ) -> Result<crate::configuration::Configuration<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getRoot",
            "()Lorg/bukkit/configuration/Configuration;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::configuration::Configuration::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn get_object_with_string(
        &mut self,
        arg0: impl Into<&'mc String>,
        arg1: std::option::Option<jni::objects::JClass<'mc>>,
        arg2: std::option::Option<jni::objects::JObject<'mc>>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let val_2 = arg1.unwrap();
        let val_3 = arg2.unwrap();
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getObject",
            "(Ljava/lang/String;Ljava/lang/Class;Ljava/lang/Object;)Ljava/lang/Object;",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
                jni::objects::JValueGen::from(&val_3),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }

    pub fn wait(
        &mut self,
        arg0: std::option::Option<i64>,
        arg1: std::option::Option<i32>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Long(arg0.unwrap().into());
        let val_2 = jni::objects::JValueGen::Int(arg1.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "wait",
            "(JI)V",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn equals(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "equals",
            "(Ljava/lang/Object;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn hash_code(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hashCode", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }

    pub fn class(&mut self) -> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getClass", "()Ljava/lang/Class;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(unsafe { jni::objects::JClass::from_raw(res.as_jni().l) })
    }

    pub fn notify(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notify", "()V", &[]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn notify_all(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notifyAll", "()V", &[]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
}
impl<'mc> Into<crate::configuration::MemoryConfiguration<'mc>> for FileConfiguration<'mc> {
    fn into(self) -> crate::configuration::MemoryConfiguration<'mc> {
        crate::configuration::MemoryConfiguration::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
/// Various settings for controlling the input and output of a <a title="class in org.bukkit.configuration.file" href="FileConfiguration.html"><code>FileConfiguration</code></a>
pub struct FileConfigurationOptions<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for FileConfigurationOptions<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> FileConfigurationOptions<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate FileConfigurationOptions from null object."
            )
            .into());
        }
        let (valid, name) = env.validate_name(
            &obj,
            "org/bukkit/configuration/file/FileConfigurationOptions",
        )?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a FileConfigurationOptions object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    #[deprecated]
    /// <span class="deprecated-label">Deprecated.</span>
    /// <div class="deprecation-comment">
    /// use getHeader() instead.
    /// </div>
    /// use getHeader() instead.
    ///
    /// <span class="deprecated-label">Deprecated.</span>
    /// <div class="deprecation-comment">
    /// use setHeader() instead
    /// </div>
    /// use setHeader() instead
    ///
    pub fn header(
        &mut self,
        arg0: std::option::Option<impl Into<&'mc String>>,
    ) -> Result<crate::configuration::file::FileConfigurationOptions<'mc>, Box<dyn std::error::Error>>
    {
        let val_1 =
            jni::objects::JObject::from(self.jni_ref().new_string(arg0.unwrap().into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "header",
            "(Ljava/lang/String;)Lorg/bukkit/configuration/file/FileConfigurationOptions;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::configuration::file::FileConfigurationOptions::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// <span class="descfrm-type-label">Description copied from class:&nbsp;<code><a href="../ConfigurationOptions.html#pathSeparator(char)">ConfigurationOptions</a></code></span>
    /// Sets the char that will be used to separate <a href="../ConfigurationSection.html" title="interface in org.bukkit.configuration"><code>ConfigurationSection</code></a>s
    /// <p>This value does not affect how the <a href="../Configuration.html" title="interface in org.bukkit.configuration"><code>Configuration</code></a> is stored, only in how you access the data. The default value is '.'.</p>
    pub fn path_separator(
        &mut self,
        arg0: std::option::Option<u16>,
    ) -> Result<crate::configuration::file::FileConfigurationOptions<'mc>, Box<dyn std::error::Error>>
    {
        let val_1 = jni::objects::JValueGen::Char(arg0.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "pathSeparator",
            "(C)Lorg/bukkit/configuration/file/FileConfigurationOptions;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::configuration::file::FileConfigurationOptions::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// <span class="descfrm-type-label">Description copied from class:&nbsp;<code><a href="../ConfigurationOptions.html#copyDefaults(boolean)">ConfigurationOptions</a></code></span>
    /// Sets if the <a title="interface in org.bukkit.configuration" href="../Configuration.html"><code>Configuration</code></a> should copy values from its default <a href="../Configuration.html" title="interface in org.bukkit.configuration"><code>Configuration</code></a> directly.
    /// <p>If this is true, all values in the default Configuration will be directly copied, making it impossible to distinguish between values that were set and values that are provided by default. As a result, <a href="../ConfigurationSection.html#contains(java.lang.String)"><code>ConfigurationSection.contains(java.lang.String)</code></a> will always return the same value as <a href="../ConfigurationSection.html#isSet(java.lang.String)"><code>ConfigurationSection.isSet(java.lang.String)</code></a>. The default value is false.</p>
    pub fn copy_defaults(
        &mut self,
        arg0: std::option::Option<bool>,
    ) -> Result<crate::configuration::file::FileConfigurationOptions<'mc>, Box<dyn std::error::Error>>
    {
        // -1
        let val_1 = jni::objects::JValueGen::Bool(arg0.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "copyDefaults",
            "(Z)Lorg/bukkit/configuration/file/FileConfigurationOptions;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::configuration::file::FileConfigurationOptions::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Gets whether or not comments should be loaded and saved.
    /// <p>Defaults to true.</p>
    /// Sets whether or not comments should be loaded and saved.
    /// <p>Defaults to true.</p>
    pub fn parse_comments(
        &mut self,
        arg0: std::option::Option<bool>,
    ) -> Result<crate::configuration::MemoryConfigurationOptions<'mc>, Box<dyn std::error::Error>>
    {
        // -1
        let val_1 = jni::objects::JValueGen::Bool(arg0.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "parseComments",
            "(Z)Lorg/bukkit/configuration/MemoryConfigurationOptions;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::configuration::MemoryConfigurationOptions::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Sets the header that will be applied to the top of the saved output.
    /// <p>This header will be commented out and applied directly at the top of the generated output of the <a title="class in org.bukkit.configuration.file" href="FileConfiguration.html"><code>FileConfiguration</code></a>. It is not required to include a newline at the end of the header as it will automatically be applied, but you may include one if you wish for extra spacing.</p>
    /// <p>If no comments exist, an empty list will be returned. A null entry represents an empty line and an empty String represents an empty comment line.</p>
    pub fn set_header(
        &mut self,
        arg0: Vec<impl Into<String>>,
    ) -> Result<crate::configuration::file::FileConfigurationOptions<'mc>, Box<dyn std::error::Error>>
    {
        let raw_val_1 = self
            .jni_ref()
            .new_object("java/util/ArrayList", "()V", &[])
            .unwrap();
        for v in arg0 {
            let map_val_0 =
                jni::objects::JObject::from(self.jni_ref().new_string(v.into()).unwrap());
            self.jni_ref().call_method(
                &raw_val_1,
                "add",
                "(Ljava/Lang/Object)V",
                &[jni::objects::JValueGen::from(&map_val_0)],
            )?;
        }
        let val_1 = jni::objects::JValueGen::Object(raw_val_1);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setHeader",
            "(Ljava/util/List;)Lorg/bukkit/configuration/file/FileConfigurationOptions;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::configuration::file::FileConfigurationOptions::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Gets the footer that will be applied to the bottom of the saved output.
    /// <p>This footer will be commented out and applied directly at the bottom of the generated output of the <a href="FileConfiguration.html" title="class in org.bukkit.configuration.file"><code>FileConfiguration</code></a>. It is not required to include a newline at the beginning of the footer as it will automatically be applied, but you may include one if you wish for extra spacing.</p>
    /// <p>If no comments exist, an empty list will be returned. A null entry represents an empty line and an empty String represents an empty comment line.</p>
    pub fn footer(&mut self) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getFooter", "()Ljava/util/List;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let mut list = blackboxmc_java::JavaList::from_raw(&self.0, res.l()?)?;
        let size = list.size()?;
        for i in 0..=size {
            let obj = list.get(i)?;
            new_vec.push(
                self.jni_ref()
                    .get_string(unsafe { &jni::objects::JString::from_raw(*obj) })?
                    .to_string_lossy()
                    .to_string(),
            );
        }
        Ok(new_vec)
    }
    /// Sets the footer that will be applied to the bottom of the saved output.
    /// <p>This footer will be commented out and applied directly at the bottom of the generated output of the <a href="FileConfiguration.html" title="class in org.bukkit.configuration.file"><code>FileConfiguration</code></a>. It is not required to include a newline at the beginning of the footer as it will automatically be applied, but you may include one if you wish for extra spacing.</p>
    /// <p>If no comments exist, an empty list will be returned. A null entry represents an empty line and an empty String represents an empty comment line.</p>
    pub fn set_footer(
        &mut self,
        arg0: Vec<impl Into<String>>,
    ) -> Result<crate::configuration::file::FileConfigurationOptions<'mc>, Box<dyn std::error::Error>>
    {
        let raw_val_1 = self
            .jni_ref()
            .new_object("java/util/ArrayList", "()V", &[])
            .unwrap();
        for v in arg0 {
            let map_val_0 =
                jni::objects::JObject::from(self.jni_ref().new_string(v.into()).unwrap());
            self.jni_ref().call_method(
                &raw_val_1,
                "add",
                "(Ljava/Lang/Object)V",
                &[jni::objects::JValueGen::from(&map_val_0)],
            )?;
        }
        let val_1 = jni::objects::JValueGen::Object(raw_val_1);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setFooter",
            "(Ljava/util/List;)Lorg/bukkit/configuration/file/FileConfigurationOptions;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::configuration::file::FileConfigurationOptions::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    #[deprecated]
    /// <span class="deprecated-label">Deprecated.</span>
    /// <div class="deprecation-comment">
    /// Call <a href="#parseComments()"><code>parseComments()</code></a> instead.
    /// </div>
    /// Call <a href="#parseComments()"><code>parseComments()</code></a> instead.
    ///
    /// <span class="deprecated-label">Deprecated.</span>
    /// <div class="deprecation-comment">
    /// Call <a href="#parseComments(boolean)"><code>parseComments(boolean)</code></a> instead.
    /// </div>
    /// Call <a href="#parseComments(boolean)"><code>parseComments(boolean)</code></a> instead.
    ///
    pub fn copy_header(
        &mut self,
        arg0: std::option::Option<bool>,
    ) -> Result<crate::configuration::file::FileConfigurationOptions<'mc>, Box<dyn std::error::Error>>
    {
        // -1
        let val_1 = jni::objects::JValueGen::Bool(arg0.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "copyHeader",
            "(Z)Lorg/bukkit/configuration/file/FileConfigurationOptions;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::configuration::file::FileConfigurationOptions::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// <span class="descfrm-type-label">Description copied from class:&nbsp;<code><a href="../ConfigurationOptions.html#configuration()">ConfigurationOptions</a></code></span>
    /// Returns the <a title="interface in org.bukkit.configuration" href="../Configuration.html"><code>Configuration</code></a> that this object is responsible for.
    pub fn configuration(
        &mut self,
    ) -> Result<crate::configuration::MemoryConfiguration<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "configuration",
            "()Lorg/bukkit/configuration/MemoryConfiguration;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::configuration::MemoryConfiguration::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn wait(
        &mut self,
        arg0: std::option::Option<i64>,
        arg1: std::option::Option<i32>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Long(arg0.unwrap().into());
        let val_2 = jni::objects::JValueGen::Int(arg1.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "wait",
            "(JI)V",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn equals(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "equals",
            "(Ljava/lang/Object;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn to_string(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "toString", "()Ljava/lang/String;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }

    pub fn hash_code(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hashCode", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }

    pub fn class(&mut self) -> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getClass", "()Ljava/lang/Class;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(unsafe { jni::objects::JClass::from_raw(res.as_jni().l) })
    }

    pub fn notify(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notify", "()V", &[]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn notify_all(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notifyAll", "()V", &[]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
}
impl<'mc> Into<crate::configuration::MemoryConfigurationOptions<'mc>>
    for FileConfigurationOptions<'mc>
{
    fn into(self) -> crate::configuration::MemoryConfigurationOptions<'mc> {
        crate::configuration::MemoryConfigurationOptions::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
/// Various settings for controlling the input and output of a <a href="YamlConfiguration.html" title="class in org.bukkit.configuration.file"><code>YamlConfiguration</code></a>
pub struct YamlConfigurationOptions<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for YamlConfigurationOptions<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> YamlConfigurationOptions<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate YamlConfigurationOptions from null object."
            )
            .into());
        }
        let (valid, name) = env.validate_name(
            &obj,
            "org/bukkit/configuration/file/YamlConfigurationOptions",
        )?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a YamlConfigurationOptions object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    /// Gets how long a line can be, before it gets split.
    /// Sets how long a line can be, before it gets split.
    pub fn width(
        &mut self,
        arg0: std::option::Option<i32>,
    ) -> Result<crate::configuration::file::YamlConfigurationOptions<'mc>, Box<dyn std::error::Error>>
    {
        let val_1 = jni::objects::JValueGen::Int(arg0.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "width",
            "(I)Lorg/bukkit/configuration/file/YamlConfigurationOptions;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::configuration::file::YamlConfigurationOptions::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// <span class="deprecated-label">Deprecated.</span>
    pub fn header(
        &mut self,
        arg0: std::option::Option<impl Into<&'mc String>>,
    ) -> Result<crate::configuration::file::FileConfigurationOptions<'mc>, Box<dyn std::error::Error>>
    {
        let val_1 =
            jni::objects::JObject::from(self.jni_ref().new_string(arg0.unwrap().into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getHeader",
            "(Ljava/lang/String;)Lorg/bukkit/configuration/file/FileConfigurationOptions;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::configuration::file::FileConfigurationOptions::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// <span class="descfrm-type-label">Description copied from class:&nbsp;<code><a href="../ConfigurationOptions.html#pathSeparator(char)">ConfigurationOptions</a></code></span>
    /// Sets the char that will be used to separate <a href="../ConfigurationSection.html" title="interface in org.bukkit.configuration"><code>ConfigurationSection</code></a>s
    /// <p>This value does not affect how the <a href="../Configuration.html" title="interface in org.bukkit.configuration"><code>Configuration</code></a> is stored, only in how you access the data. The default value is '.'.</p>
    pub fn path_separator(
        &mut self,
        arg0: std::option::Option<u16>,
    ) -> Result<crate::configuration::file::YamlConfigurationOptions<'mc>, Box<dyn std::error::Error>>
    {
        let val_1 = jni::objects::JValueGen::Char(arg0.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "pathSeparator",
            "(C)Lorg/bukkit/configuration/file/YamlConfigurationOptions;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::configuration::file::YamlConfigurationOptions::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// <span class="descfrm-type-label">Description copied from class:&nbsp;<code><a href="../ConfigurationOptions.html#copyDefaults(boolean)">ConfigurationOptions</a></code></span>
    /// Sets if the <a href="../Configuration.html" title="interface in org.bukkit.configuration"><code>Configuration</code></a> should copy values from its default <a href="../Configuration.html" title="interface in org.bukkit.configuration"><code>Configuration</code></a> directly.
    /// <p>If this is true, all values in the default Configuration will be directly copied, making it impossible to distinguish between values that were set and values that are provided by default. As a result, <a href="../ConfigurationSection.html#contains(java.lang.String)"><code>ConfigurationSection.contains(java.lang.String)</code></a> will always return the same value as <a href="../ConfigurationSection.html#isSet(java.lang.String)"><code>ConfigurationSection.isSet(java.lang.String)</code></a>. The default value is false.</p>
    pub fn copy_defaults(
        &mut self,
        arg0: std::option::Option<bool>,
    ) -> Result<crate::configuration::file::FileConfigurationOptions<'mc>, Box<dyn std::error::Error>>
    {
        // -1
        let val_1 = jni::objects::JValueGen::Bool(arg0.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "copyDefaults",
            "(Z)Lorg/bukkit/configuration/file/FileConfigurationOptions;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::configuration::file::FileConfigurationOptions::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// <span class="descfrm-type-label">Description copied from class:&nbsp;<code><a href="FileConfigurationOptions.html#parseComments(boolean)">FileConfigurationOptions</a></code></span>
    /// Sets whether or not comments should be loaded and saved.
    /// <p>Defaults to true.</p>
    pub fn parse_comments(
        &mut self,
        arg0: std::option::Option<bool>,
    ) -> Result<crate::configuration::file::YamlConfigurationOptions<'mc>, Box<dyn std::error::Error>>
    {
        // -1
        let val_1 = jni::objects::JValueGen::Bool(arg0.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "parseComments",
            "(Z)Lorg/bukkit/configuration/file/YamlConfigurationOptions;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::configuration::file::YamlConfigurationOptions::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// <span class="descfrm-type-label">Description copied from class:&nbsp;<code><a href="FileConfigurationOptions.html#setHeader(java.util.List)">FileConfigurationOptions</a></code></span>
    /// Sets the header that will be applied to the top of the saved output.
    /// <p>This header will be commented out and applied directly at the top of the generated output of the <a href="FileConfiguration.html" title="class in org.bukkit.configuration.file"><code>FileConfiguration</code></a>. It is not required to include a newline at the end of the header as it will automatically be applied, but you may include one if you wish for extra spacing.</p>
    /// <p>If no comments exist, an empty list will be returned. A null entry represents an empty line and an empty String represents an empty comment line.</p>
    pub fn set_header_with_list(
        &mut self,
        arg0: std::option::Option<Vec<impl Into<String>>>,
    ) -> Result<crate::configuration::file::YamlConfigurationOptions<'mc>, Box<dyn std::error::Error>>
    {
        let raw_val_1 = self
            .jni_ref()
            .new_object("java/util/ArrayList", "()V", &[])
            .unwrap();
        for v in arg0.unwrap() {
            let map_val_0 =
                jni::objects::JObject::from(self.jni_ref().new_string(v.into()).unwrap());
            self.jni_ref().call_method(
                &raw_val_1,
                "add",
                "(Ljava/Lang/Object)V",
                &[jni::objects::JValueGen::from(&map_val_0)],
            )?;
        }
        let val_1 = jni::objects::JValueGen::Object(raw_val_1);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setHeader",
            "(Ljava/util/List;)Lorg/bukkit/configuration/file/YamlConfigurationOptions;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::configuration::file::YamlConfigurationOptions::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// <span class="deprecated-label">Deprecated.</span>
    pub fn copy_header(
        &mut self,
        arg0: std::option::Option<bool>,
    ) -> Result<crate::configuration::file::FileConfigurationOptions<'mc>, Box<dyn std::error::Error>>
    {
        // -1
        let val_1 = jni::objects::JValueGen::Bool(arg0.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "copyHeader",
            "(Z)Lorg/bukkit/configuration/file/FileConfigurationOptions;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::configuration::file::FileConfigurationOptions::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Gets how much spaces should be used to indent each line.
    /// <p>The minimum value this may be is 2, and the maximum is 9.</p>
    /// Sets how much spaces should be used to indent each line.
    /// <p>The minimum value this may be is 2, and the maximum is 9.</p>
    pub fn indent(
        &mut self,
        arg0: std::option::Option<i32>,
    ) -> Result<crate::configuration::file::YamlConfigurationOptions<'mc>, Box<dyn std::error::Error>>
    {
        let val_1 = jni::objects::JValueGen::Int(arg0.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "indent",
            "(I)Lorg/bukkit/configuration/file/YamlConfigurationOptions;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::configuration::file::YamlConfigurationOptions::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// <span class="descfrm-type-label">Description copied from class:&nbsp;<code><a href="../ConfigurationOptions.html#configuration()">ConfigurationOptions</a></code></span>
    /// Returns the <a href="../Configuration.html" title="interface in org.bukkit.configuration"><code>Configuration</code></a> that this object is responsible for.
    pub fn configuration(
        &mut self,
    ) -> Result<crate::configuration::Configuration<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "configuration",
            "()Lorg/bukkit/configuration/Configuration;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::configuration::Configuration::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn footer(&mut self) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getFooter", "()Ljava/util/List;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let mut list = blackboxmc_java::JavaList::from_raw(&self.0, res.l()?)?;
        let size = list.size()?;
        for i in 0..=size {
            let obj = list.get(i)?;
            new_vec.push(
                self.jni_ref()
                    .get_string(unsafe { &jni::objects::JString::from_raw(*obj) })?
                    .to_string_lossy()
                    .to_string(),
            );
        }
        Ok(new_vec)
    }

    pub fn wait(
        &mut self,
        arg0: std::option::Option<i64>,
        arg1: std::option::Option<i32>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Long(arg0.unwrap().into());
        let val_2 = jni::objects::JValueGen::Int(arg1.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "wait",
            "(JI)V",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn equals(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "equals",
            "(Ljava/lang/Object;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn to_string(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "toString", "()Ljava/lang/String;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }

    pub fn hash_code(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hashCode", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }

    pub fn class(&mut self) -> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getClass", "()Ljava/lang/Class;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(unsafe { jni::objects::JClass::from_raw(res.as_jni().l) })
    }

    pub fn notify(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notify", "()V", &[]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn notify_all(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notifyAll", "()V", &[]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
}
impl<'mc> Into<crate::configuration::file::FileConfigurationOptions<'mc>>
    for YamlConfigurationOptions<'mc>
{
    fn into(self) -> crate::configuration::file::FileConfigurationOptions<'mc> {
        crate::configuration::file::FileConfigurationOptions::from_raw(&self.jni_ref(), self.1)
            .unwrap()
    }
}
/// An implementation of <a title="interface in org.bukkit.configuration" href="../Configuration.html"><code>Configuration</code></a> which saves all files in Yaml. Note that this implementation is not synchronized.
pub struct YamlConfiguration<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for YamlConfiguration<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> YamlConfiguration<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate YamlConfiguration from null object.").into(),
            );
        }
        let (valid, name) =
            env.validate_name(&obj, "org/bukkit/configuration/file/YamlConfiguration")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a YamlConfiguration object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    pub fn new(
        jni: blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<crate::configuration::file::YamlConfiguration<'mc>, Box<dyn std::error::Error>>
    {
        let cls = &jni.find_class("org/bukkit/configuration/file/YamlConfiguration")?;
        let res = jni.new_object(cls, "()V", &[])?;
        crate::configuration::file::YamlConfiguration::from_raw(&jni, res)
    }
    /// Creates a new <a href="YamlConfiguration.html" title="class in org.bukkit.configuration.file"><code>YamlConfiguration</code></a>, loading from the given file.
    /// <p>Any errors loading the Configuration will be logged and then ignored. If the specified input is not a valid config, a blank config will be returned.</p>
    /// <p>The encoding used may follow the system dependent default.</p>
    /// Creates a new <a href="YamlConfiguration.html" title="class in org.bukkit.configuration.file"><code>YamlConfiguration</code></a>, loading from the given reader.
    /// <p>Any errors loading the Configuration will be logged and then ignored. If the specified input is not a valid config, a blank config will be returned.</p>
    pub unsafe fn load_configuration_with_file(
        jni: blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: std::option::Option<jni::objects::JObject<'mc>>,
    ) -> Result<crate::configuration::file::YamlConfiguration<'mc>, Box<dyn std::error::Error>>
    {
        let val_1 = arg0.unwrap();
        let cls = &jni.find_class("org/bukkit/configuration/file/YamlConfiguration")?;
        let res = jni.call_static_method(
            cls,
            "loadConfiguration",
            "(Ljava/io/Reader;)Lorg/bukkit/configuration/file/YamlConfiguration;",
            &[jni::objects::JValueGen::from(&val_1)],
        )?;
        let obj = res.l()?;
        crate::configuration::file::YamlConfiguration::from_raw(&jni, obj)
    }
    /// <span class="descfrm-type-label">Description copied from class:&nbsp;<code><a href="FileConfiguration.html#saveToString()">FileConfiguration</a></code></span>
    /// Saves this <a href="FileConfiguration.html" title="class in org.bukkit.configuration.file"><code>FileConfiguration</code></a> to a string, and returns it.
    pub fn save_to_string(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "saveToString",
            "()Ljava/lang/String;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    /// <span class="descfrm-type-label">Description copied from class:&nbsp;<code><a href="FileConfiguration.html#loadFromString(java.lang.String)">FileConfiguration</a></code></span>
    /// Loads this <a href="FileConfiguration.html" title="class in org.bukkit.configuration.file"><code>FileConfiguration</code></a> from the specified string, as opposed to from file.
    /// <p>All the values contained within this configuration will be removed, leaving only settings and defaults, and the new values will be loaded from the given string.</p>
    /// <p>If the string is invalid in any way, an exception will be thrown.</p>
    pub fn load_from_string(
        &mut self,
        arg0: impl Into<&'mc String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "loadFromString",
            "(Ljava/lang/String;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// <span class="descfrm-type-label">Description copied from interface:&nbsp;<code><a href="../Configuration.html#options()">Configuration</a></code></span>
    /// Gets the <a href="../ConfigurationOptions.html" title="class in org.bukkit.configuration"><code>ConfigurationOptions</code></a> for this <a title="interface in org.bukkit.configuration" href="../Configuration.html"><code>Configuration</code></a>.
    /// <p>All setters through this method are chainable.</p>
    pub fn options(
        &mut self,
    ) -> Result<crate::configuration::file::YamlConfigurationOptions<'mc>, Box<dyn std::error::Error>>
    {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "options",
            "()Lorg/bukkit/configuration/file/YamlConfigurationOptions;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::configuration::file::YamlConfigurationOptions::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// <span class="descfrm-type-label">Description copied from class:&nbsp;<code><a href="FileConfiguration.html#loadFromString(java.lang.String)">FileConfiguration</a></code></span>
    /// Loads this <a href="FileConfiguration.html" title="class in org.bukkit.configuration.file"><code>FileConfiguration</code></a> from the specified string, as opposed to from file.
    /// <p>All the values contained within this configuration will be removed, leaving only settings and defaults, and the new values will be loaded from the given string.</p>
    /// <p>If the string is invalid in any way, an exception will be thrown.</p>
    /// Creates a new <a href="YamlConfiguration.html" title="class in org.bukkit.configuration.file"><code>YamlConfiguration</code></a>, loading from the given file.
    /// <p>Any errors loading the Configuration will be logged and then ignored. If the specified input is not a valid config, a blank config will be returned.</p>
    /// <p>The encoding used may follow the system dependent default.</p>
    /// Creates a new <a href="YamlConfiguration.html" title="class in org.bukkit.configuration.file"><code>YamlConfiguration</code></a>, loading from the given reader.
    /// <p>Any errors loading the Configuration will be logged and then ignored. If the specified input is not a valid config, a blank config will be returned.</p>
    pub unsafe fn load_with_string(
        &mut self,
        arg0: std::option::Option<jni::objects::JObject<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = arg0.unwrap();
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "load",
            "(Ljava/io/Reader;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// <span class="descfrm-type-label">Description copied from class:&nbsp;<code><a href="FileConfiguration.html#loadFromString(java.lang.String)">FileConfiguration</a></code></span>
    /// Loads this <a href="FileConfiguration.html" title="class in org.bukkit.configuration.file"><code>FileConfiguration</code></a> from the specified string, as opposed to from file.
    /// <p>All the values contained within this configuration will be removed, leaving only settings and defaults, and the new values will be loaded from the given string.</p>
    /// <p>If the string is invalid in any way, an exception will be thrown.</p>
    /// Creates a new <a href="YamlConfiguration.html" title="class in org.bukkit.configuration.file"><code>YamlConfiguration</code></a>, loading from the given file.
    /// <p>Any errors loading the Configuration will be logged and then ignored. If the specified input is not a valid config, a blank config will be returned.</p>
    /// <p>The encoding used may follow the system dependent default.</p>
    /// Creates a new <a href="YamlConfiguration.html" title="class in org.bukkit.configuration.file"><code>YamlConfiguration</code></a>, loading from the given reader.
    /// <p>Any errors loading the Configuration will be logged and then ignored. If the specified input is not a valid config, a blank config will be returned.</p>
    pub unsafe fn load_with_file(
        &mut self,
        arg0: std::option::Option<jni::objects::JObject<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = arg0.unwrap();
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "load",
            "(Ljava/io/File;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// <span class="descfrm-type-label">Description copied from class:&nbsp;<code><a href="FileConfiguration.html#saveToString()">FileConfiguration</a></code></span>
    /// Saves this <a href="FileConfiguration.html" title="class in org.bukkit.configuration.file"><code>FileConfiguration</code></a> to a string, and returns it.
    pub fn save_with_file(
        &mut self,
        arg0: std::option::Option<impl Into<&'mc String>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 =
            jni::objects::JObject::from(self.jni_ref().new_string(arg0.unwrap().into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "save",
            "(Ljava/lang/String;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn add_default(
        &mut self,
        arg0: impl Into<&'mc String>,
        arg1: jni::objects::JObject<'mc>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let val_2 = arg1;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "addDefault",
            "(Ljava/lang/String;Ljava/lang/Object;)V",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn defaults(
        &mut self,
    ) -> Result<crate::configuration::Configuration<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getDefaults",
            "()Lorg/bukkit/configuration/Configuration;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::configuration::Configuration::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn add_defaults_with_configuration(
        &mut self,
        arg0: std::option::Option<impl Into<&'mc blackboxmc_java::JavaMap<'mc>>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 =
            unsafe { jni::objects::JObject::from_raw(arg0.unwrap().into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "addDefaults",
            "(Ljava/util/Map;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn set_defaults(
        &mut self,
        arg0: impl Into<&'mc crate::configuration::Configuration<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setDefaults",
            "(Lorg/bukkit/configuration/Configuration;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn parent(
        &mut self,
    ) -> Result<crate::configuration::ConfigurationSection<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getParent",
            "()Lorg/bukkit/configuration/ConfigurationSection;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::configuration::ConfigurationSection::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn get_string_with_string(
        &mut self,
        arg0: std::option::Option<impl Into<&'mc String>>,
        arg1: std::option::Option<impl Into<&'mc String>>,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let val_1 =
            jni::objects::JObject::from(self.jni_ref().new_string(arg0.unwrap().into()).unwrap());
        let val_2 =
            jni::objects::JObject::from(self.jni_ref().new_string(arg1.unwrap().into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getString",
            "(Ljava/lang/String;Ljava/lang/String;)Ljava/lang/String;",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }

    pub fn get_keys(
        &mut self,
        arg0: bool,
    ) -> Result<blackboxmc_java::JavaSet<'mc>, Box<dyn std::error::Error>> {
        // -2
        let val_1 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getKeys",
            "(Z)Ljava/util/Set;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::JavaSet::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn get_offline_player_with_string(
        &mut self,
        arg0: std::option::Option<impl Into<&'mc String>>,
        arg1: std::option::Option<impl Into<&'mc crate::OfflinePlayer<'mc>>>,
    ) -> Result<crate::OfflinePlayer<'mc>, Box<dyn std::error::Error>> {
        let val_1 =
            jni::objects::JObject::from(self.jni_ref().new_string(arg0.unwrap().into()).unwrap());
        let val_2 =
            unsafe { jni::objects::JObject::from_raw(arg1.unwrap().into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getOfflinePlayer",
            "(Ljava/lang/String;Lorg/bukkit/OfflinePlayer;)Lorg/bukkit/OfflinePlayer;",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::OfflinePlayer::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn get_color_with_string(
        &mut self,
        arg0: std::option::Option<impl Into<&'mc String>>,
        arg1: std::option::Option<impl Into<&'mc crate::Color<'mc>>>,
    ) -> Result<crate::Color<'mc>, Box<dyn std::error::Error>> {
        let val_1 =
            jni::objects::JObject::from(self.jni_ref().new_string(arg0.unwrap().into()).unwrap());
        let val_2 =
            unsafe { jni::objects::JObject::from_raw(arg1.unwrap().into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getColor",
            "(Ljava/lang/String;Lorg/bukkit/Color;)Lorg/bukkit/Color;",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::Color::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn get_item_stack_with_string(
        &mut self,
        arg0: std::option::Option<impl Into<&'mc String>>,
        arg1: std::option::Option<impl Into<&'mc crate::inventory::ItemStack<'mc>>>,
    ) -> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>> {
        let val_1 =
            jni::objects::JObject::from(self.jni_ref().new_string(arg0.unwrap().into()).unwrap());
        let val_2 =
            unsafe { jni::objects::JObject::from_raw(arg1.unwrap().into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getItemStack",
            "(Ljava/lang/String;Lorg/bukkit/inventory/ItemStack;)Lorg/bukkit/inventory/ItemStack;",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::ItemStack::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn get_values(
        &mut self,
        arg0: bool,
    ) -> Result<blackboxmc_java::JavaMap<'mc>, Box<dyn std::error::Error>> {
        // -2
        let val_1 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getValues",
            "(Z)Ljava/util/Map;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::JavaMap::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn create_path_with_configuration_section(
        jni: blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<&'mc crate::configuration::ConfigurationSection<'mc>>,
        arg1: std::option::Option<impl Into<&'mc String>>,
        arg2: std::option::Option<impl Into<&'mc crate::configuration::ConfigurationSection<'mc>>>,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let val_2 = jni::objects::JObject::from(jni.new_string(arg1.unwrap().into()).unwrap());
        let val_3 =
            unsafe { jni::objects::JObject::from_raw(arg2.unwrap().into().jni_object().clone()) };
        let cls = &jni.find_class("java/lang/String")?;
        let res = jni.call_static_method(cls,"createPath",
"(Lorg/bukkit/configuration/ConfigurationSection;Ljava/lang/String;Lorg/bukkit/configuration/ConfigurationSection;)Ljava/lang/String;",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2),jni::objects::JValueGen::from(&val_3)])?;
        Ok(jni
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }

    pub fn default_section(
        &mut self,
    ) -> Result<crate::configuration::ConfigurationSection<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getDefaultSection",
            "()Lorg/bukkit/configuration/ConfigurationSection;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::configuration::ConfigurationSection::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn current_path(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getCurrentPath",
            "()Ljava/lang/String;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }

    pub fn is_configuration_section(
        &mut self,
        arg0: impl Into<&'mc String>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "isConfigurationSection",
            "(Ljava/lang/String;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn get_configuration_section(
        &mut self,
        arg0: impl Into<&'mc String>,
    ) -> Result<crate::configuration::ConfigurationSection<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getConfigurationSection",
            "(Ljava/lang/String;)Lorg/bukkit/configuration/ConfigurationSection;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::configuration::ConfigurationSection::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn create_section_with_string(
        &mut self,
        arg0: std::option::Option<impl Into<&'mc String>>,
        arg1: std::option::Option<impl Into<&'mc blackboxmc_java::JavaMap<'mc>>>,
    ) -> Result<crate::configuration::ConfigurationSection<'mc>, Box<dyn std::error::Error>> {
        let val_1 =
            jni::objects::JObject::from(self.jni_ref().new_string(arg0.unwrap().into()).unwrap());
        let val_2 =
            unsafe { jni::objects::JObject::from_raw(arg1.unwrap().into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "createSection",
            "(Ljava/lang/String;Ljava/util/Map;)Lorg/bukkit/configuration/ConfigurationSection;",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::configuration::ConfigurationSection::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn is_string(
        &mut self,
        arg0: impl Into<&'mc String>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "isString",
            "(Ljava/lang/String;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn is_int(
        &mut self,
        arg0: impl Into<&'mc String>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "isInt",
            "(Ljava/lang/String;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn is_boolean(
        &mut self,
        arg0: impl Into<&'mc String>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "isBoolean",
            "(Ljava/lang/String;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn is_double(
        &mut self,
        arg0: impl Into<&'mc String>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "isDouble",
            "(Ljava/lang/String;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn is_long(
        &mut self,
        arg0: impl Into<&'mc String>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "isLong",
            "(Ljava/lang/String;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn is_list(
        &mut self,
        arg0: impl Into<&'mc String>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "isList",
            "(Ljava/lang/String;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn get_string_list(
        &mut self,
        arg0: impl Into<&'mc String>,
    ) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getStringList",
            "(Ljava/lang/String;)Ljava/util/List;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let mut list = blackboxmc_java::JavaList::from_raw(&self.0, res.l()?)?;
        let size = list.size()?;
        for i in 0..=size {
            let obj = list.get(i)?;
            new_vec.push(
                self.jni_ref()
                    .get_string(unsafe { &jni::objects::JString::from_raw(*obj) })?
                    .to_string_lossy()
                    .to_string(),
            );
        }
        Ok(new_vec)
    }

    pub fn get_serializable_with_string(
        &mut self,
        arg0: impl Into<&'mc String>,
        arg1: std::option::Option<jni::objects::JClass<'mc>>,
        arg2: std::option::Option<
            impl Into<&'mc crate::configuration::serialization::ConfigurationSerializable<'mc>>,
        >,
    ) -> Result<
        crate::configuration::serialization::ConfigurationSerializable<'mc>,
        Box<dyn std::error::Error>,
    > {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let val_2 = arg1.unwrap();
        let val_3 =
            unsafe { jni::objects::JObject::from_raw(arg2.unwrap().into().jni_object().clone()) };
        let res = self.jni_ref().call_method(&self.jni_object(),"getSerializable","(Ljava/lang/String;Ljava/lang/Class;Lorg/bukkit/configuration/serialization/ConfigurationSerializable;)Lorg/bukkit/configuration/serialization/ConfigurationSerializable;",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2),jni::objects::JValueGen::from(&val_3)]);
        let res = self.jni_ref().translate_error(res)?;
        crate::configuration::serialization::ConfigurationSerializable::from_raw(
            &self.jni_ref(),
            unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) },
        )
    }

    pub fn get_vector_with_string(
        &mut self,
        arg0: std::option::Option<impl Into<&'mc String>>,
        arg1: std::option::Option<impl Into<&'mc crate::util::Vector<'mc>>>,
    ) -> Result<crate::util::Vector<'mc>, Box<dyn std::error::Error>> {
        let val_1 =
            jni::objects::JObject::from(self.jni_ref().new_string(arg0.unwrap().into()).unwrap());
        let val_2 =
            unsafe { jni::objects::JObject::from_raw(arg1.unwrap().into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getVector",
            "(Ljava/lang/String;Lorg/bukkit/util/Vector;)Lorg/bukkit/util/Vector;",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::util::Vector::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn is_vector(
        &mut self,
        arg0: impl Into<&'mc String>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "isVector",
            "(Ljava/lang/String;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn is_offline_player(
        &mut self,
        arg0: impl Into<&'mc String>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "isOfflinePlayer",
            "(Ljava/lang/String;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn is_item_stack(
        &mut self,
        arg0: impl Into<&'mc String>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "isItemStack",
            "(Ljava/lang/String;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn is_color(
        &mut self,
        arg0: impl Into<&'mc String>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "isColor",
            "(Ljava/lang/String;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn is_location(
        &mut self,
        arg0: impl Into<&'mc String>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "isLocation",
            "(Ljava/lang/String;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn get_comments(
        &mut self,
        arg0: impl Into<&'mc String>,
    ) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getComments",
            "(Ljava/lang/String;)Ljava/util/List;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let mut list = blackboxmc_java::JavaList::from_raw(&self.0, res.l()?)?;
        let size = list.size()?;
        for i in 0..=size {
            let obj = list.get(i)?;
            new_vec.push(
                self.jni_ref()
                    .get_string(unsafe { &jni::objects::JString::from_raw(*obj) })?
                    .to_string_lossy()
                    .to_string(),
            );
        }
        Ok(new_vec)
    }

    pub fn get_inline_comments(
        &mut self,
        arg0: impl Into<&'mc String>,
    ) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getInlineComments",
            "(Ljava/lang/String;)Ljava/util/List;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let mut list = blackboxmc_java::JavaList::from_raw(&self.0, res.l()?)?;
        let size = list.size()?;
        for i in 0..=size {
            let obj = list.get(i)?;
            new_vec.push(
                self.jni_ref()
                    .get_string(unsafe { &jni::objects::JString::from_raw(*obj) })?
                    .to_string_lossy()
                    .to_string(),
            );
        }
        Ok(new_vec)
    }

    pub fn set_comments(
        &mut self,
        arg0: impl Into<&'mc String>,
        arg1: Vec<impl Into<String>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let raw_val_2 = self
            .jni_ref()
            .new_object("java/util/ArrayList", "()V", &[])
            .unwrap();
        for v in arg1 {
            let map_val_0 =
                jni::objects::JObject::from(self.jni_ref().new_string(v.into()).unwrap());
            self.jni_ref().call_method(
                &raw_val_2,
                "add",
                "(Ljava/Lang/Object)V",
                &[jni::objects::JValueGen::from(&map_val_0)],
            )?;
        }
        let val_2 = jni::objects::JValueGen::Object(raw_val_2);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setComments",
            "(Ljava/lang/String;Ljava/util/List;)V",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn set_inline_comments(
        &mut self,
        arg0: impl Into<&'mc String>,
        arg1: Vec<impl Into<String>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let raw_val_2 = self
            .jni_ref()
            .new_object("java/util/ArrayList", "()V", &[])
            .unwrap();
        for v in arg1 {
            let map_val_0 =
                jni::objects::JObject::from(self.jni_ref().new_string(v.into()).unwrap());
            self.jni_ref().call_method(
                &raw_val_2,
                "add",
                "(Ljava/Lang/Object)V",
                &[jni::objects::JValueGen::from(&map_val_0)],
            )?;
        }
        let val_2 = jni::objects::JValueGen::Object(raw_val_2);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setInlineComments",
            "(Ljava/lang/String;Ljava/util/List;)V",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn name(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getName", "()Ljava/lang/String;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }

    pub fn get_with_string(
        &mut self,
        arg0: std::option::Option<impl Into<&'mc String>>,
        arg1: std::option::Option<jni::objects::JObject<'mc>>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let val_1 =
            jni::objects::JObject::from(self.jni_ref().new_string(arg0.unwrap().into()).unwrap());
        let val_2 = arg1.unwrap();
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "get",
            "(Ljava/lang/String;Ljava/lang/Object;)Ljava/lang/Object;",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }

    pub fn to_string(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "toString", "()Ljava/lang/String;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }

    pub fn get_boolean_with_string(
        &mut self,
        arg0: std::option::Option<impl Into<&'mc String>>,
        arg1: std::option::Option<bool>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 =
            jni::objects::JObject::from(self.jni_ref().new_string(arg0.unwrap().into()).unwrap());
        // 0
        let val_2 = jni::objects::JValueGen::Bool(arg1.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getBoolean",
            "(Ljava/lang/String;Z)Z",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn get_int_with_string(
        &mut self,
        arg0: std::option::Option<impl Into<&'mc String>>,
        arg1: std::option::Option<i32>,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        let val_1 =
            jni::objects::JObject::from(self.jni_ref().new_string(arg0.unwrap().into()).unwrap());
        let val_2 = jni::objects::JValueGen::Int(arg1.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getInt",
            "(Ljava/lang/String;I)I",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }

    pub fn get_long_with_string(
        &mut self,
        arg0: std::option::Option<impl Into<&'mc String>>,
        arg1: std::option::Option<i64>,
    ) -> Result<i64, Box<dyn std::error::Error>> {
        let val_1 =
            jni::objects::JObject::from(self.jni_ref().new_string(arg0.unwrap().into()).unwrap());
        let val_2 = jni::objects::JValueGen::Long(arg1.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getLong",
            "(Ljava/lang/String;J)J",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.j().unwrap())
    }

    pub fn get_double_with_string(
        &mut self,
        arg0: std::option::Option<impl Into<&'mc String>>,
        arg1: std::option::Option<f64>,
    ) -> Result<f64, Box<dyn std::error::Error>> {
        let val_1 =
            jni::objects::JObject::from(self.jni_ref().new_string(arg0.unwrap().into()).unwrap());
        let val_2 = jni::objects::JValueGen::Double(arg1.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getDouble",
            "(Ljava/lang/String;D)D",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.d().unwrap())
    }

    pub fn contains_with_string(
        &mut self,
        arg0: std::option::Option<impl Into<&'mc String>>,
        arg1: std::option::Option<bool>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 =
            jni::objects::JObject::from(self.jni_ref().new_string(arg0.unwrap().into()).unwrap());
        // 0
        let val_2 = jni::objects::JValueGen::Bool(arg1.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "contains",
            "(Ljava/lang/String;Z)Z",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn get_location_with_string(
        &mut self,
        arg0: std::option::Option<impl Into<&'mc String>>,
        arg1: std::option::Option<impl Into<&'mc crate::Location<'mc>>>,
    ) -> Result<crate::Location<'mc>, Box<dyn std::error::Error>> {
        let val_1 =
            jni::objects::JObject::from(self.jni_ref().new_string(arg0.unwrap().into()).unwrap());
        let val_2 =
            unsafe { jni::objects::JObject::from_raw(arg1.unwrap().into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getLocation",
            "(Ljava/lang/String;Lorg/bukkit/Location;)Lorg/bukkit/Location;",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::Location::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn set(
        &mut self,
        arg0: impl Into<&'mc String>,
        arg1: jni::objects::JObject<'mc>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let val_2 = arg1;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "set",
            "(Ljava/lang/String;Ljava/lang/Object;)V",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn is_set(
        &mut self,
        arg0: impl Into<&'mc String>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "isSet",
            "(Ljava/lang/String;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn root(
        &mut self,
    ) -> Result<crate::configuration::Configuration<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getRoot",
            "()Lorg/bukkit/configuration/Configuration;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::configuration::Configuration::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn get_object_with_string(
        &mut self,
        arg0: impl Into<&'mc String>,
        arg1: std::option::Option<jni::objects::JClass<'mc>>,
        arg2: std::option::Option<jni::objects::JObject<'mc>>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let val_2 = arg1.unwrap();
        let val_3 = arg2.unwrap();
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getObject",
            "(Ljava/lang/String;Ljava/lang/Class;Ljava/lang/Object;)Ljava/lang/Object;",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
                jni::objects::JValueGen::from(&val_3),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }

    pub fn wait(
        &mut self,
        arg0: std::option::Option<i64>,
        arg1: std::option::Option<i32>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Long(arg0.unwrap().into());
        let val_2 = jni::objects::JValueGen::Int(arg1.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "wait",
            "(JI)V",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn equals(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "equals",
            "(Ljava/lang/Object;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn hash_code(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hashCode", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }

    pub fn class(&mut self) -> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getClass", "()Ljava/lang/Class;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(unsafe { jni::objects::JClass::from_raw(res.as_jni().l) })
    }

    pub fn notify(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notify", "()V", &[]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn notify_all(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notifyAll", "()V", &[]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
}
impl<'mc> Into<crate::configuration::file::FileConfiguration<'mc>> for YamlConfiguration<'mc> {
    fn into(self) -> crate::configuration::file::FileConfiguration<'mc> {
        crate::configuration::file::FileConfiguration::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
