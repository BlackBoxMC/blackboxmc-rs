use crate::JNIRaw;
pub struct FileConfiguration<'mc>(
    pub(crate) jni::JNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> crate::JNIRaw<'mc> for FileConfiguration<'mc> {
    fn jni_ref(&self) -> jni::JNIEnv<'mc> {
        unsafe { self.0.unsafe_clone() }
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> FileConfiguration<'mc> {
    pub fn from_raw(env: jni::JNIEnv<'mc>, obj: jni::objects::JObject<'mc>) -> Self {
        Self(env, obj)
    }
    pub fn save_to_string(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "saveToString",
            "()Ljava/lang/String;",
            &[],
        )?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    pub fn load_from_string(&mut self, arg0: String) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JObject::from(self.jni_ref().new_string(arg0).unwrap());
        self.jni_ref().call_method(
            &self.jni_object(),
            "loadFromString",
            "(Ljava/lang/String;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn options(
        &mut self,
    ) -> Result<
        crate::bukkit::configuration::file::FileConfigurationOptions<'mc>,
        Box<dyn std::error::Error>,
    > {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "options",
            "()Lorg/bukkit/configuration/file/FileConfigurationOptions;",
            &[],
        )?;
        let ret = {
            crate::bukkit::configuration::file::FileConfigurationOptions(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn add_default(
        &mut self,
        arg0: String,
        arg1: jni::objects::JObject<'mc>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JObject::from(self.jni_ref().new_string(arg0).unwrap());
        let val_1 = arg1;
        self.jni_ref().call_method(
            &self.jni_object(),
            "addDefault",
            "(Ljava/lang/String;Ljava/lang/Object;)V",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        )?;
        Ok(())
    }
    pub fn defaults(
        &mut self,
    ) -> Result<crate::bukkit::configuration::Configuration<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getDefaults",
            "()Lorg/bukkit/configuration/Configuration;",
            &[],
        )?;
        let ret = {
            crate::bukkit::configuration::Configuration(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn set_defaults(
        &mut self,
        arg0: crate::bukkit::configuration::Configuration<'mc>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.1.clone()) };
        self.jni_ref().call_method(
            &self.jni_object(),
            "setDefaults",
            "(Lorg/bukkit/configuration/Configuration;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn parent(
        &mut self,
    ) -> Result<crate::bukkit::configuration::ConfigurationSection<'mc>, Box<dyn std::error::Error>>
    {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getParent",
            "()Lorg/bukkit/configuration/ConfigurationSection;",
            &[],
        )?;
        let ret = {
            crate::bukkit::configuration::ConfigurationSection(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn get_string_with_string(
        &mut self,
        arg0: std::option::Option<String>,
        arg1: std::option::Option<String>,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.unwrap()).unwrap());
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg1.unwrap()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getString",
            "(Ljava/lang/String;Ljava/lang/String;)Ljava/lang/String;",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        )?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    pub fn get_color_with_string(
        &mut self,
        arg0: std::option::Option<String>,
        arg1: std::option::Option<crate::bukkit::Color<'mc>>,
    ) -> Result<crate::bukkit::Color<'mc>, Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.unwrap()).unwrap());
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg1.unwrap().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getColor",
            "(Ljava/lang/String;Lorg/bukkit/Color;)Lorg/bukkit/Color;",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        )?;
        let ret = {
            crate::bukkit::Color(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn get_item_stack_with_string(
        &mut self,
        arg0: std::option::Option<String>,
        arg1: std::option::Option<crate::bukkit::inventory::ItemStack<'mc>>,
    ) -> Result<crate::bukkit::inventory::ItemStack<'mc>, Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.unwrap()).unwrap());
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg1.unwrap().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getItemStack",
            "(Ljava/lang/String;Lorg/bukkit/inventory/ItemStack;)Lorg/bukkit/inventory/ItemStack;",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        )?;
        let ret = {
            crate::bukkit::inventory::ItemStack(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn get_offline_player_with_string(
        &mut self,
        arg0: std::option::Option<String>,
        arg1: std::option::Option<crate::bukkit::OfflinePlayer<'mc>>,
    ) -> Result<crate::bukkit::OfflinePlayer<'mc>, Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.unwrap()).unwrap());
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg1.unwrap().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getOfflinePlayer",
            "(Ljava/lang/String;Lorg/bukkit/OfflinePlayer;)Lorg/bukkit/OfflinePlayer;",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        )?;
        let ret = {
            crate::bukkit::OfflinePlayer(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn create_path_with_configuration_section(
        mut jni: jni::JNIEnv<'mc>,
        arg0: crate::bukkit::configuration::ConfigurationSection<'mc>,
        arg1: std::option::Option<String>,
        arg2: std::option::Option<crate::bukkit::configuration::ConfigurationSection<'mc>>,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.1.clone()) };
        let val_1 = jni::objects::JObject::from(jni.new_string(arg1.unwrap()).unwrap());
        let val_2 = unsafe { jni::objects::JObject::from_raw(arg2.unwrap().1.clone()) };
        let cls = &jni.find_class("java/lang/String")?;
        let res =jni.call_static_method(cls,"createPath","(Lorg/bukkit/configuration/ConfigurationSection;Ljava/lang/String;Lorg/bukkit/configuration/ConfigurationSection;)Ljava/lang/String;",&[jni::objects::JValueGen::from(&val_0),jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)])?;
        Ok(jni
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    pub fn default_section(
        &mut self,
    ) -> Result<crate::bukkit::configuration::ConfigurationSection<'mc>, Box<dyn std::error::Error>>
    {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getDefaultSection",
            "()Lorg/bukkit/configuration/ConfigurationSection;",
            &[],
        )?;
        let ret = {
            crate::bukkit::configuration::ConfigurationSection(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn current_path(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getCurrentPath",
            "()Ljava/lang/String;",
            &[],
        )?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    pub fn is_configuration_section(
        &mut self,
        arg0: String,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JObject::from(self.jni_ref().new_string(arg0).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "isConfigurationSection",
            "(Ljava/lang/String;)Z",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(res.z().unwrap())
    }
    pub fn get_configuration_section(
        &mut self,
        arg0: String,
    ) -> Result<crate::bukkit::configuration::ConfigurationSection<'mc>, Box<dyn std::error::Error>>
    {
        let val_0 = jni::objects::JObject::from(self.jni_ref().new_string(arg0).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getConfigurationSection",
            "(Ljava/lang/String;)Lorg/bukkit/configuration/ConfigurationSection;",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        let ret = {
            crate::bukkit::configuration::ConfigurationSection(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn is_string(&mut self, arg0: String) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JObject::from(self.jni_ref().new_string(arg0).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "isString",
            "(Ljava/lang/String;)Z",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(res.z().unwrap())
    }
    pub fn is_int(&mut self, arg0: String) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JObject::from(self.jni_ref().new_string(arg0).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "isInt",
            "(Ljava/lang/String;)Z",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(res.z().unwrap())
    }
    pub fn is_boolean(&mut self, arg0: String) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JObject::from(self.jni_ref().new_string(arg0).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "isBoolean",
            "(Ljava/lang/String;)Z",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(res.z().unwrap())
    }
    pub fn is_double(&mut self, arg0: String) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JObject::from(self.jni_ref().new_string(arg0).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "isDouble",
            "(Ljava/lang/String;)Z",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(res.z().unwrap())
    }
    pub fn is_long(&mut self, arg0: String) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JObject::from(self.jni_ref().new_string(arg0).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "isLong",
            "(Ljava/lang/String;)Z",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(res.z().unwrap())
    }
    pub fn is_list(&mut self, arg0: String) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JObject::from(self.jni_ref().new_string(arg0).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "isList",
            "(Ljava/lang/String;)Z",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(res.z().unwrap())
    }
    pub fn get_serializable_with_string(
        &mut self,
        arg0: String,
        arg1: std::option::Option<jni::objects::JClass<'mc>>,
        arg2: std::option::Option<
            crate::bukkit::configuration::serialization::ConfigurationSerializable<'mc>,
        >,
    ) -> Result<
        crate::bukkit::configuration::serialization::ConfigurationSerializable<'mc>,
        Box<dyn std::error::Error>,
    > {
        let val_0 = jni::objects::JObject::from(self.jni_ref().new_string(arg0).unwrap());
        let val_1 = arg1.unwrap();
        let val_2 = unsafe { jni::objects::JObject::from_raw(arg2.unwrap().1.clone()) };
        let res =
self.jni_ref().call_method(&self.jni_object(),"getSerializable","(Ljava/lang/String;Ljava/lang/Class;Lorg/bukkit/configuration/serialization/ConfigurationSerializable;)Lorg/bukkit/configuration/serialization/ConfigurationSerializable;",&[jni::objects::JValueGen::from(&val_0),jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)])?;
        let ret = {
            crate::bukkit::configuration::serialization::ConfigurationSerializable(
                self.jni_ref(),
                unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) },
            )
        };
        Ok(ret)
    }
    pub fn get_vector_with_string(
        &mut self,
        arg0: std::option::Option<String>,
        arg1: std::option::Option<crate::bukkit::util::Vector<'mc>>,
    ) -> Result<crate::bukkit::util::Vector<'mc>, Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.unwrap()).unwrap());
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg1.unwrap().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getVector",
            "(Ljava/lang/String;Lorg/bukkit/util/Vector;)Lorg/bukkit/util/Vector;",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        )?;
        let ret = {
            crate::bukkit::util::Vector(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn is_vector(&mut self, arg0: String) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JObject::from(self.jni_ref().new_string(arg0).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "isVector",
            "(Ljava/lang/String;)Z",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(res.z().unwrap())
    }
    pub fn is_offline_player(&mut self, arg0: String) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JObject::from(self.jni_ref().new_string(arg0).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "isOfflinePlayer",
            "(Ljava/lang/String;)Z",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(res.z().unwrap())
    }
    pub fn is_item_stack(&mut self, arg0: String) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JObject::from(self.jni_ref().new_string(arg0).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "isItemStack",
            "(Ljava/lang/String;)Z",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(res.z().unwrap())
    }
    pub fn is_color(&mut self, arg0: String) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JObject::from(self.jni_ref().new_string(arg0).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "isColor",
            "(Ljava/lang/String;)Z",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(res.z().unwrap())
    }
    pub fn is_location(&mut self, arg0: String) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JObject::from(self.jni_ref().new_string(arg0).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "isLocation",
            "(Ljava/lang/String;)Z",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(res.z().unwrap())
    }
    pub fn set_comments(
        &mut self,
        arg0: String,
        arg1: Vec<String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JObject::from(self.jni_ref().new_string(arg0).unwrap());
        let raw_val_1 = self
            .jni_ref()
            .new_object("java/util/ArrayList", "()V", &[])
            .unwrap();
        for v in arg1 {
            let map_val_0 = jni::objects::JObject::from(self.jni_ref().new_string(v).unwrap());
            self.jni_ref().call_method(
                &raw_val_1,
                "add",
                "(Ljava/Lang/Object)V",
                &[jni::objects::JValueGen::from(&map_val_0)],
            )?;
        }
        let val_1 = jni::objects::JValueGen::Object(raw_val_1);
        self.jni_ref().call_method(
            &self.jni_object(),
            "setComments",
            "(Ljava/lang/String;Ljava/util/List;)V",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        )?;
        Ok(())
    }
    pub fn set_inline_comments(
        &mut self,
        arg0: String,
        arg1: Vec<String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JObject::from(self.jni_ref().new_string(arg0).unwrap());
        let raw_val_1 = self
            .jni_ref()
            .new_object("java/util/ArrayList", "()V", &[])
            .unwrap();
        for v in arg1 {
            let map_val_0 = jni::objects::JObject::from(self.jni_ref().new_string(v).unwrap());
            self.jni_ref().call_method(
                &raw_val_1,
                "add",
                "(Ljava/Lang/Object)V",
                &[jni::objects::JValueGen::from(&map_val_0)],
            )?;
        }
        let val_1 = jni::objects::JValueGen::Object(raw_val_1);
        self.jni_ref().call_method(
            &self.jni_object(),
            "setInlineComments",
            "(Ljava/lang/String;Ljava/util/List;)V",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        )?;
        Ok(())
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
    pub fn get_with_string(
        &mut self,
        arg0: std::option::Option<String>,
        arg1: std::option::Option<jni::objects::JObject<'mc>>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.unwrap()).unwrap());
        let val_1 = arg1.unwrap();
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "get",
            "(Ljava/lang/String;Ljava/lang/Object;)Ljava/lang/Object;",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        )?;
        Ok(res.l().unwrap())
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
    pub fn get_boolean_with_string(
        &mut self,
        arg0: std::option::Option<String>,
        arg1: std::option::Option<bool>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.unwrap()).unwrap());
        let val_1 = jni::objects::JValueGen::Bool(arg1.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getBoolean",
            "(Ljava/lang/String;Z)Z",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        )?;
        Ok(res.z().unwrap())
    }
    pub fn get_int_with_string(
        &mut self,
        arg0: std::option::Option<String>,
        arg1: std::option::Option<i32>,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.unwrap()).unwrap());
        let val_1 = jni::objects::JValueGen::Int(arg1.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getInt",
            "(Ljava/lang/String;I)I",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        )?;
        Ok(res.i().unwrap())
    }
    pub fn get_long_with_string(
        &mut self,
        arg0: std::option::Option<String>,
        arg1: std::option::Option<i64>,
    ) -> Result<i64, Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.unwrap()).unwrap());
        let val_1 = jni::objects::JValueGen::Long(arg1.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getLong",
            "(Ljava/lang/String;J)J",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        )?;
        Ok(res.j().unwrap())
    }
    pub fn get_double_with_string(
        &mut self,
        arg0: std::option::Option<String>,
        arg1: std::option::Option<f64>,
    ) -> Result<f64, Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.unwrap()).unwrap());
        let val_1 = jni::objects::JValueGen::Double(arg1.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getDouble",
            "(Ljava/lang/String;D)D",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        )?;
        Ok(res.d().unwrap())
    }
    pub fn contains_with_string(
        &mut self,
        arg0: std::option::Option<String>,
        arg1: std::option::Option<bool>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.unwrap()).unwrap());
        let val_1 = jni::objects::JValueGen::Bool(arg1.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "contains",
            "(Ljava/lang/String;Z)Z",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        )?;
        Ok(res.z().unwrap())
    }
    pub fn get_location_with_string(
        &mut self,
        arg0: std::option::Option<String>,
        arg1: std::option::Option<crate::bukkit::Location<'mc>>,
    ) -> Result<crate::bukkit::Location<'mc>, Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.unwrap()).unwrap());
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg1.unwrap().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getLocation",
            "(Ljava/lang/String;Lorg/bukkit/Location;)Lorg/bukkit/Location;",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        )?;
        let ret = {
            crate::bukkit::Location(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn set(
        &mut self,
        arg0: String,
        arg1: jni::objects::JObject<'mc>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JObject::from(self.jni_ref().new_string(arg0).unwrap());
        let val_1 = arg1;
        self.jni_ref().call_method(
            &self.jni_object(),
            "set",
            "(Ljava/lang/String;Ljava/lang/Object;)V",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        )?;
        Ok(())
    }
    pub fn is_set(&mut self, arg0: String) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JObject::from(self.jni_ref().new_string(arg0).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "isSet",
            "(Ljava/lang/String;)Z",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(res.z().unwrap())
    }
    pub fn root(
        &mut self,
    ) -> Result<crate::bukkit::configuration::Configuration<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getRoot",
            "()Lorg/bukkit/configuration/Configuration;",
            &[],
        )?;
        let ret = {
            crate::bukkit::configuration::Configuration(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn get_object_with_string(
        &mut self,
        arg0: String,
        arg1: std::option::Option<jni::objects::JClass<'mc>>,
        arg2: std::option::Option<jni::objects::JObject<'mc>>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JObject::from(self.jni_ref().new_string(arg0).unwrap());
        let val_1 = arg1.unwrap();
        let val_2 = arg2.unwrap();
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getObject",
            "(Ljava/lang/String;Ljava/lang/Class;Ljava/lang/Object;)Ljava/lang/Object;",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        )?;
        Ok(res.l().unwrap())
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
pub struct FileConfigurationOptions<'mc>(
    pub(crate) jni::JNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> crate::JNIRaw<'mc> for FileConfigurationOptions<'mc> {
    fn jni_ref(&self) -> jni::JNIEnv<'mc> {
        unsafe { self.0.unsafe_clone() }
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> FileConfigurationOptions<'mc> {
    pub fn from_raw(env: jni::JNIEnv<'mc>, obj: jni::objects::JObject<'mc>) -> Self {
        Self(env, obj)
    }
    pub fn header(
        &mut self,
        arg0: std::option::Option<String>,
    ) -> Result<
        crate::bukkit::configuration::file::FileConfigurationOptions<'mc>,
        Box<dyn std::error::Error>,
    > {
        let val_0 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.unwrap()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "header",
            "(Ljava/lang/String;)Lorg/bukkit/configuration/file/FileConfigurationOptions;",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        let ret = {
            crate::bukkit::configuration::file::FileConfigurationOptions(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn path_separator(
        &mut self,
        arg0: std::option::Option<u16>,
    ) -> Result<
        crate::bukkit::configuration::file::FileConfigurationOptions<'mc>,
        Box<dyn std::error::Error>,
    > {
        let val_0 = jni::objects::JValueGen::Char(arg0.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "pathSeparator",
            "(C)Lorg/bukkit/configuration/file/FileConfigurationOptions;",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        let ret = {
            crate::bukkit::configuration::file::FileConfigurationOptions(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn copy_defaults(
        &mut self,
        arg0: std::option::Option<bool>,
    ) -> Result<
        crate::bukkit::configuration::file::FileConfigurationOptions<'mc>,
        Box<dyn std::error::Error>,
    > {
        let val_0 = jni::objects::JValueGen::Bool(arg0.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "copyDefaults",
            "(Z)Lorg/bukkit/configuration/file/FileConfigurationOptions;",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        let ret = {
            crate::bukkit::configuration::file::FileConfigurationOptions(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn parse_comments(
        &mut self,
        arg0: std::option::Option<bool>,
    ) -> Result<
        crate::bukkit::configuration::MemoryConfigurationOptions<'mc>,
        Box<dyn std::error::Error>,
    > {
        let val_0 = jni::objects::JValueGen::Bool(arg0.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "parseComments",
            "(Z)Lorg/bukkit/configuration/MemoryConfigurationOptions;",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        let ret = {
            crate::bukkit::configuration::MemoryConfigurationOptions(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn set_header(
        &mut self,
        arg0: Vec<String>,
    ) -> Result<
        crate::bukkit::configuration::file::FileConfigurationOptions<'mc>,
        Box<dyn std::error::Error>,
    > {
        let raw_val_0 = self
            .jni_ref()
            .new_object("java/util/ArrayList", "()V", &[])
            .unwrap();
        for v in arg0 {
            let map_val_0 = jni::objects::JObject::from(self.jni_ref().new_string(v).unwrap());
            self.jni_ref().call_method(
                &raw_val_0,
                "add",
                "(Ljava/Lang/Object)V",
                &[jni::objects::JValueGen::from(&map_val_0)],
            )?;
        }
        let val_0 = jni::objects::JValueGen::Object(raw_val_0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setHeader",
            "(Ljava/util/List;)Lorg/bukkit/configuration/file/FileConfigurationOptions;",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        let ret = {
            crate::bukkit::configuration::file::FileConfigurationOptions(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn set_footer(
        &mut self,
        arg0: Vec<String>,
    ) -> Result<
        crate::bukkit::configuration::file::FileConfigurationOptions<'mc>,
        Box<dyn std::error::Error>,
    > {
        let raw_val_0 = self
            .jni_ref()
            .new_object("java/util/ArrayList", "()V", &[])
            .unwrap();
        for v in arg0 {
            let map_val_0 = jni::objects::JObject::from(self.jni_ref().new_string(v).unwrap());
            self.jni_ref().call_method(
                &raw_val_0,
                "add",
                "(Ljava/Lang/Object)V",
                &[jni::objects::JValueGen::from(&map_val_0)],
            )?;
        }
        let val_0 = jni::objects::JValueGen::Object(raw_val_0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setFooter",
            "(Ljava/util/List;)Lorg/bukkit/configuration/file/FileConfigurationOptions;",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        let ret = {
            crate::bukkit::configuration::file::FileConfigurationOptions(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn copy_header(
        &mut self,
        arg0: std::option::Option<bool>,
    ) -> Result<
        crate::bukkit::configuration::file::FileConfigurationOptions<'mc>,
        Box<dyn std::error::Error>,
    > {
        let val_0 = jni::objects::JValueGen::Bool(arg0.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "copyHeader",
            "(Z)Lorg/bukkit/configuration/file/FileConfigurationOptions;",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        let ret = {
            crate::bukkit::configuration::file::FileConfigurationOptions(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn configuration(
        &mut self,
    ) -> Result<crate::bukkit::configuration::MemoryConfiguration<'mc>, Box<dyn std::error::Error>>
    {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "configuration",
            "()Lorg/bukkit/configuration/MemoryConfiguration;",
            &[],
        )?;
        let ret = {
            crate::bukkit::configuration::MemoryConfiguration(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
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
pub struct YamlConfigurationOptions<'mc>(
    pub(crate) jni::JNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> crate::JNIRaw<'mc> for YamlConfigurationOptions<'mc> {
    fn jni_ref(&self) -> jni::JNIEnv<'mc> {
        unsafe { self.0.unsafe_clone() }
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> YamlConfigurationOptions<'mc> {
    pub fn from_raw(env: jni::JNIEnv<'mc>, obj: jni::objects::JObject<'mc>) -> Self {
        Self(env, obj)
    }
    pub fn header(
        &mut self,
        arg0: std::option::Option<String>,
    ) -> Result<
        crate::bukkit::configuration::file::YamlConfigurationOptions<'mc>,
        Box<dyn std::error::Error>,
    > {
        let val_0 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.unwrap()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getHeader",
            "(Ljava/lang/String;)Lorg/bukkit/configuration/file/YamlConfigurationOptions;",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        let ret = {
            crate::bukkit::configuration::file::YamlConfigurationOptions(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn path_separator(
        &mut self,
        arg0: std::option::Option<u16>,
    ) -> Result<
        crate::bukkit::configuration::MemoryConfigurationOptions<'mc>,
        Box<dyn std::error::Error>,
    > {
        let val_0 = jni::objects::JValueGen::Char(arg0.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "pathSeparator",
            "(C)Lorg/bukkit/configuration/MemoryConfigurationOptions;",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        let ret = {
            crate::bukkit::configuration::MemoryConfigurationOptions(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn width(
        &mut self,
        arg0: std::option::Option<i32>,
    ) -> Result<
        crate::bukkit::configuration::file::YamlConfigurationOptions<'mc>,
        Box<dyn std::error::Error>,
    > {
        let val_0 = jni::objects::JValueGen::Int(arg0.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "width",
            "(I)Lorg/bukkit/configuration/file/YamlConfigurationOptions;",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        let ret = {
            crate::bukkit::configuration::file::YamlConfigurationOptions(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn copy_defaults(
        &mut self,
        arg0: std::option::Option<bool>,
    ) -> Result<
        crate::bukkit::configuration::MemoryConfigurationOptions<'mc>,
        Box<dyn std::error::Error>,
    > {
        let val_0 = jni::objects::JValueGen::Bool(arg0.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "copyDefaults",
            "(Z)Lorg/bukkit/configuration/MemoryConfigurationOptions;",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        let ret = {
            crate::bukkit::configuration::MemoryConfigurationOptions(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn parse_comments(
        &mut self,
        arg0: std::option::Option<bool>,
    ) -> Result<
        crate::bukkit::configuration::file::YamlConfigurationOptions<'mc>,
        Box<dyn std::error::Error>,
    > {
        let val_0 = jni::objects::JValueGen::Bool(arg0.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "parseComments",
            "(Z)Lorg/bukkit/configuration/file/YamlConfigurationOptions;",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        let ret = {
            crate::bukkit::configuration::file::YamlConfigurationOptions(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn set_header_with_list(
        &mut self,
        arg0: std::option::Option<Vec<String>>,
    ) -> Result<
        crate::bukkit::configuration::file::YamlConfigurationOptions<'mc>,
        Box<dyn std::error::Error>,
    > {
        let raw_val_0 = self
            .jni_ref()
            .new_object("java/util/ArrayList", "()V", &[])
            .unwrap();
        for v in arg0.unwrap() {
            let map_val_0 = jni::objects::JObject::from(self.jni_ref().new_string(v).unwrap());
            self.jni_ref().call_method(
                &raw_val_0,
                "add",
                "(Ljava/Lang/Object)V",
                &[jni::objects::JValueGen::from(&map_val_0)],
            )?;
        }
        let val_0 = jni::objects::JValueGen::Object(raw_val_0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setHeader",
            "(Ljava/util/List;)Lorg/bukkit/configuration/file/YamlConfigurationOptions;",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        let ret = {
            crate::bukkit::configuration::file::YamlConfigurationOptions(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn copy_header(
        &mut self,
        arg0: std::option::Option<bool>,
    ) -> Result<
        crate::bukkit::configuration::file::FileConfigurationOptions<'mc>,
        Box<dyn std::error::Error>,
    > {
        let val_0 = jni::objects::JValueGen::Bool(arg0.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "copyHeader",
            "(Z)Lorg/bukkit/configuration/file/FileConfigurationOptions;",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        let ret = {
            crate::bukkit::configuration::file::FileConfigurationOptions(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn indent(
        &mut self,
        arg0: std::option::Option<i32>,
    ) -> Result<
        crate::bukkit::configuration::file::YamlConfigurationOptions<'mc>,
        Box<dyn std::error::Error>,
    > {
        let val_0 = jni::objects::JValueGen::Int(arg0.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "indent",
            "(I)Lorg/bukkit/configuration/file/YamlConfigurationOptions;",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        let ret = {
            crate::bukkit::configuration::file::YamlConfigurationOptions(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn configuration(
        &mut self,
    ) -> Result<crate::bukkit::configuration::Configuration<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "configuration",
            "()Lorg/bukkit/configuration/Configuration;",
            &[],
        )?;
        let ret = {
            crate::bukkit::configuration::Configuration(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
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
pub struct YamlConfiguration<'mc>(
    pub(crate) jni::JNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> crate::JNIRaw<'mc> for YamlConfiguration<'mc> {
    fn jni_ref(&self) -> jni::JNIEnv<'mc> {
        unsafe { self.0.unsafe_clone() }
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> YamlConfiguration<'mc> {
    pub fn from_raw(env: jni::JNIEnv<'mc>, obj: jni::objects::JObject<'mc>) -> Self {
        Self(env, obj)
    }
    pub fn save_to_string(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "saveToString",
            "()Ljava/lang/String;",
            &[],
        )?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    pub fn load_from_string(&mut self, arg0: String) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JObject::from(self.jni_ref().new_string(arg0).unwrap());
        self.jni_ref().call_method(
            &self.jni_object(),
            "loadFromString",
            "(Ljava/lang/String;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn options(
        &mut self,
    ) -> Result<
        crate::bukkit::configuration::file::YamlConfigurationOptions<'mc>,
        Box<dyn std::error::Error>,
    > {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "options",
            "()Lorg/bukkit/configuration/file/YamlConfigurationOptions;",
            &[],
        )?;
        let ret = {
            crate::bukkit::configuration::file::YamlConfigurationOptions(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn add_default(
        &mut self,
        arg0: String,
        arg1: jni::objects::JObject<'mc>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JObject::from(self.jni_ref().new_string(arg0).unwrap());
        let val_1 = arg1;
        self.jni_ref().call_method(
            &self.jni_object(),
            "addDefault",
            "(Ljava/lang/String;Ljava/lang/Object;)V",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        )?;
        Ok(())
    }
    pub fn defaults(
        &mut self,
    ) -> Result<crate::bukkit::configuration::Configuration<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getDefaults",
            "()Lorg/bukkit/configuration/Configuration;",
            &[],
        )?;
        let ret = {
            crate::bukkit::configuration::Configuration(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn set_defaults(
        &mut self,
        arg0: crate::bukkit::configuration::Configuration<'mc>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.1.clone()) };
        self.jni_ref().call_method(
            &self.jni_object(),
            "setDefaults",
            "(Lorg/bukkit/configuration/Configuration;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn parent(
        &mut self,
    ) -> Result<crate::bukkit::configuration::ConfigurationSection<'mc>, Box<dyn std::error::Error>>
    {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getParent",
            "()Lorg/bukkit/configuration/ConfigurationSection;",
            &[],
        )?;
        let ret = {
            crate::bukkit::configuration::ConfigurationSection(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn get_string_with_string(
        &mut self,
        arg0: std::option::Option<String>,
        arg1: std::option::Option<String>,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.unwrap()).unwrap());
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg1.unwrap()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getString",
            "(Ljava/lang/String;Ljava/lang/String;)Ljava/lang/String;",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        )?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    pub fn get_color_with_string(
        &mut self,
        arg0: std::option::Option<String>,
        arg1: std::option::Option<crate::bukkit::Color<'mc>>,
    ) -> Result<crate::bukkit::Color<'mc>, Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.unwrap()).unwrap());
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg1.unwrap().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getColor",
            "(Ljava/lang/String;Lorg/bukkit/Color;)Lorg/bukkit/Color;",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        )?;
        let ret = {
            crate::bukkit::Color(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn get_item_stack_with_string(
        &mut self,
        arg0: std::option::Option<String>,
        arg1: std::option::Option<crate::bukkit::inventory::ItemStack<'mc>>,
    ) -> Result<crate::bukkit::inventory::ItemStack<'mc>, Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.unwrap()).unwrap());
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg1.unwrap().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getItemStack",
            "(Ljava/lang/String;Lorg/bukkit/inventory/ItemStack;)Lorg/bukkit/inventory/ItemStack;",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        )?;
        let ret = {
            crate::bukkit::inventory::ItemStack(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn get_offline_player_with_string(
        &mut self,
        arg0: std::option::Option<String>,
        arg1: std::option::Option<crate::bukkit::OfflinePlayer<'mc>>,
    ) -> Result<crate::bukkit::OfflinePlayer<'mc>, Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.unwrap()).unwrap());
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg1.unwrap().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getOfflinePlayer",
            "(Ljava/lang/String;Lorg/bukkit/OfflinePlayer;)Lorg/bukkit/OfflinePlayer;",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        )?;
        let ret = {
            crate::bukkit::OfflinePlayer(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn create_path_with_configuration_section(
        mut jni: jni::JNIEnv<'mc>,
        arg0: crate::bukkit::configuration::ConfigurationSection<'mc>,
        arg1: std::option::Option<String>,
        arg2: std::option::Option<crate::bukkit::configuration::ConfigurationSection<'mc>>,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.1.clone()) };
        let val_1 = jni::objects::JObject::from(jni.new_string(arg1.unwrap()).unwrap());
        let val_2 = unsafe { jni::objects::JObject::from_raw(arg2.unwrap().1.clone()) };
        let cls = &jni.find_class("java/lang/String")?;
        let res =jni.call_static_method(cls,"createPath","(Lorg/bukkit/configuration/ConfigurationSection;Ljava/lang/String;Lorg/bukkit/configuration/ConfigurationSection;)Ljava/lang/String;",&[jni::objects::JValueGen::from(&val_0),jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)])?;
        Ok(jni
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    pub fn default_section(
        &mut self,
    ) -> Result<crate::bukkit::configuration::ConfigurationSection<'mc>, Box<dyn std::error::Error>>
    {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getDefaultSection",
            "()Lorg/bukkit/configuration/ConfigurationSection;",
            &[],
        )?;
        let ret = {
            crate::bukkit::configuration::ConfigurationSection(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn current_path(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getCurrentPath",
            "()Ljava/lang/String;",
            &[],
        )?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    pub fn is_configuration_section(
        &mut self,
        arg0: String,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JObject::from(self.jni_ref().new_string(arg0).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "isConfigurationSection",
            "(Ljava/lang/String;)Z",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(res.z().unwrap())
    }
    pub fn get_configuration_section(
        &mut self,
        arg0: String,
    ) -> Result<crate::bukkit::configuration::ConfigurationSection<'mc>, Box<dyn std::error::Error>>
    {
        let val_0 = jni::objects::JObject::from(self.jni_ref().new_string(arg0).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getConfigurationSection",
            "(Ljava/lang/String;)Lorg/bukkit/configuration/ConfigurationSection;",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        let ret = {
            crate::bukkit::configuration::ConfigurationSection(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn is_string(&mut self, arg0: String) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JObject::from(self.jni_ref().new_string(arg0).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "isString",
            "(Ljava/lang/String;)Z",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(res.z().unwrap())
    }
    pub fn is_int(&mut self, arg0: String) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JObject::from(self.jni_ref().new_string(arg0).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "isInt",
            "(Ljava/lang/String;)Z",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(res.z().unwrap())
    }
    pub fn is_boolean(&mut self, arg0: String) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JObject::from(self.jni_ref().new_string(arg0).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "isBoolean",
            "(Ljava/lang/String;)Z",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(res.z().unwrap())
    }
    pub fn is_double(&mut self, arg0: String) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JObject::from(self.jni_ref().new_string(arg0).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "isDouble",
            "(Ljava/lang/String;)Z",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(res.z().unwrap())
    }
    pub fn is_long(&mut self, arg0: String) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JObject::from(self.jni_ref().new_string(arg0).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "isLong",
            "(Ljava/lang/String;)Z",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(res.z().unwrap())
    }
    pub fn is_list(&mut self, arg0: String) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JObject::from(self.jni_ref().new_string(arg0).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "isList",
            "(Ljava/lang/String;)Z",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(res.z().unwrap())
    }
    pub fn get_serializable_with_string(
        &mut self,
        arg0: String,
        arg1: std::option::Option<jni::objects::JClass<'mc>>,
        arg2: std::option::Option<
            crate::bukkit::configuration::serialization::ConfigurationSerializable<'mc>,
        >,
    ) -> Result<
        crate::bukkit::configuration::serialization::ConfigurationSerializable<'mc>,
        Box<dyn std::error::Error>,
    > {
        let val_0 = jni::objects::JObject::from(self.jni_ref().new_string(arg0).unwrap());
        let val_1 = arg1.unwrap();
        let val_2 = unsafe { jni::objects::JObject::from_raw(arg2.unwrap().1.clone()) };
        let res =
self.jni_ref().call_method(&self.jni_object(),"getSerializable","(Ljava/lang/String;Ljava/lang/Class;Lorg/bukkit/configuration/serialization/ConfigurationSerializable;)Lorg/bukkit/configuration/serialization/ConfigurationSerializable;",&[jni::objects::JValueGen::from(&val_0),jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)])?;
        let ret = {
            crate::bukkit::configuration::serialization::ConfigurationSerializable(
                self.jni_ref(),
                unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) },
            )
        };
        Ok(ret)
    }
    pub fn get_vector_with_string(
        &mut self,
        arg0: std::option::Option<String>,
        arg1: std::option::Option<crate::bukkit::util::Vector<'mc>>,
    ) -> Result<crate::bukkit::util::Vector<'mc>, Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.unwrap()).unwrap());
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg1.unwrap().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getVector",
            "(Ljava/lang/String;Lorg/bukkit/util/Vector;)Lorg/bukkit/util/Vector;",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        )?;
        let ret = {
            crate::bukkit::util::Vector(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn is_vector(&mut self, arg0: String) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JObject::from(self.jni_ref().new_string(arg0).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "isVector",
            "(Ljava/lang/String;)Z",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(res.z().unwrap())
    }
    pub fn is_offline_player(&mut self, arg0: String) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JObject::from(self.jni_ref().new_string(arg0).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "isOfflinePlayer",
            "(Ljava/lang/String;)Z",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(res.z().unwrap())
    }
    pub fn is_item_stack(&mut self, arg0: String) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JObject::from(self.jni_ref().new_string(arg0).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "isItemStack",
            "(Ljava/lang/String;)Z",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(res.z().unwrap())
    }
    pub fn is_color(&mut self, arg0: String) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JObject::from(self.jni_ref().new_string(arg0).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "isColor",
            "(Ljava/lang/String;)Z",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(res.z().unwrap())
    }
    pub fn is_location(&mut self, arg0: String) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JObject::from(self.jni_ref().new_string(arg0).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "isLocation",
            "(Ljava/lang/String;)Z",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(res.z().unwrap())
    }
    pub fn set_comments(
        &mut self,
        arg0: String,
        arg1: Vec<String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JObject::from(self.jni_ref().new_string(arg0).unwrap());
        let raw_val_1 = self
            .jni_ref()
            .new_object("java/util/ArrayList", "()V", &[])
            .unwrap();
        for v in arg1 {
            let map_val_0 = jni::objects::JObject::from(self.jni_ref().new_string(v).unwrap());
            self.jni_ref().call_method(
                &raw_val_1,
                "add",
                "(Ljava/Lang/Object)V",
                &[jni::objects::JValueGen::from(&map_val_0)],
            )?;
        }
        let val_1 = jni::objects::JValueGen::Object(raw_val_1);
        self.jni_ref().call_method(
            &self.jni_object(),
            "setComments",
            "(Ljava/lang/String;Ljava/util/List;)V",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        )?;
        Ok(())
    }
    pub fn set_inline_comments(
        &mut self,
        arg0: String,
        arg1: Vec<String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JObject::from(self.jni_ref().new_string(arg0).unwrap());
        let raw_val_1 = self
            .jni_ref()
            .new_object("java/util/ArrayList", "()V", &[])
            .unwrap();
        for v in arg1 {
            let map_val_0 = jni::objects::JObject::from(self.jni_ref().new_string(v).unwrap());
            self.jni_ref().call_method(
                &raw_val_1,
                "add",
                "(Ljava/Lang/Object)V",
                &[jni::objects::JValueGen::from(&map_val_0)],
            )?;
        }
        let val_1 = jni::objects::JValueGen::Object(raw_val_1);
        self.jni_ref().call_method(
            &self.jni_object(),
            "setInlineComments",
            "(Ljava/lang/String;Ljava/util/List;)V",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        )?;
        Ok(())
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
    pub fn get_with_string(
        &mut self,
        arg0: std::option::Option<String>,
        arg1: std::option::Option<jni::objects::JObject<'mc>>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.unwrap()).unwrap());
        let val_1 = arg1.unwrap();
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "get",
            "(Ljava/lang/String;Ljava/lang/Object;)Ljava/lang/Object;",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        )?;
        Ok(res.l().unwrap())
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
    pub fn get_boolean_with_string(
        &mut self,
        arg0: std::option::Option<String>,
        arg1: std::option::Option<bool>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.unwrap()).unwrap());
        let val_1 = jni::objects::JValueGen::Bool(arg1.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getBoolean",
            "(Ljava/lang/String;Z)Z",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        )?;
        Ok(res.z().unwrap())
    }
    pub fn get_int_with_string(
        &mut self,
        arg0: std::option::Option<String>,
        arg1: std::option::Option<i32>,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.unwrap()).unwrap());
        let val_1 = jni::objects::JValueGen::Int(arg1.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getInt",
            "(Ljava/lang/String;I)I",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        )?;
        Ok(res.i().unwrap())
    }
    pub fn get_long_with_string(
        &mut self,
        arg0: std::option::Option<String>,
        arg1: std::option::Option<i64>,
    ) -> Result<i64, Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.unwrap()).unwrap());
        let val_1 = jni::objects::JValueGen::Long(arg1.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getLong",
            "(Ljava/lang/String;J)J",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        )?;
        Ok(res.j().unwrap())
    }
    pub fn get_double_with_string(
        &mut self,
        arg0: std::option::Option<String>,
        arg1: std::option::Option<f64>,
    ) -> Result<f64, Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.unwrap()).unwrap());
        let val_1 = jni::objects::JValueGen::Double(arg1.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getDouble",
            "(Ljava/lang/String;D)D",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        )?;
        Ok(res.d().unwrap())
    }
    pub fn contains_with_string(
        &mut self,
        arg0: std::option::Option<String>,
        arg1: std::option::Option<bool>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.unwrap()).unwrap());
        let val_1 = jni::objects::JValueGen::Bool(arg1.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "contains",
            "(Ljava/lang/String;Z)Z",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        )?;
        Ok(res.z().unwrap())
    }
    pub fn get_location_with_string(
        &mut self,
        arg0: std::option::Option<String>,
        arg1: std::option::Option<crate::bukkit::Location<'mc>>,
    ) -> Result<crate::bukkit::Location<'mc>, Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.unwrap()).unwrap());
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg1.unwrap().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getLocation",
            "(Ljava/lang/String;Lorg/bukkit/Location;)Lorg/bukkit/Location;",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        )?;
        let ret = {
            crate::bukkit::Location(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn set(
        &mut self,
        arg0: String,
        arg1: jni::objects::JObject<'mc>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JObject::from(self.jni_ref().new_string(arg0).unwrap());
        let val_1 = arg1;
        self.jni_ref().call_method(
            &self.jni_object(),
            "set",
            "(Ljava/lang/String;Ljava/lang/Object;)V",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        )?;
        Ok(())
    }
    pub fn is_set(&mut self, arg0: String) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JObject::from(self.jni_ref().new_string(arg0).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "isSet",
            "(Ljava/lang/String;)Z",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(res.z().unwrap())
    }
    pub fn root(
        &mut self,
    ) -> Result<crate::bukkit::configuration::Configuration<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getRoot",
            "()Lorg/bukkit/configuration/Configuration;",
            &[],
        )?;
        let ret = {
            crate::bukkit::configuration::Configuration(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn get_object_with_string(
        &mut self,
        arg0: String,
        arg1: std::option::Option<jni::objects::JClass<'mc>>,
        arg2: std::option::Option<jni::objects::JObject<'mc>>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JObject::from(self.jni_ref().new_string(arg0).unwrap());
        let val_1 = arg1.unwrap();
        let val_2 = arg2.unwrap();
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getObject",
            "(Ljava/lang/String;Ljava/lang/Class;Ljava/lang/Object;)Ljava/lang/Object;",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        )?;
        Ok(res.l().unwrap())
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
