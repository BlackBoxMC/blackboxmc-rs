use crate::JNIRaw;
pub struct EnchantmentWrapper<'mc>(
    pub(crate) crate::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> crate::JNIRaw<'mc> for EnchantmentWrapper<'mc> {
    fn jni_ref(&self) -> crate::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> EnchantmentWrapper<'mc> {
    pub fn new(
        jni: crate::SharedJNIEnv<'mc>,
        arg0: String,
    ) -> Result<crate::bukkit::enchantments::EnchantmentWrapper<'mc>, Box<dyn std::error::Error>>
    {
        let val_0 = jni::objects::JObject::from(jni.new_string(arg0).unwrap());
        let cls = &jni.find_class("org/bukkit/enchantments/EnchantmentWrapper")?;
        let res = jni.new_object(
            cls,
            "(Ljava/lang/String;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        let ret = { crate::bukkit::enchantments::EnchantmentWrapper(jni, res) };
        Ok(ret)
    }
    pub fn from_raw(
        env: &crate::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate EnchantmentWrapper from null object.").into(),
            );
        }
        let cls = env.jni.borrow().get_object_class(&obj)?;
        let name_raw = env.call_method(cls, "getName", "()Ljava/lang/String;", &[])?;
        let oh = name_raw.l()?.into();
        let what = env.get_string(&oh)?;
        let name = what.to_string_lossy();
        if !name.ends_with("EnchantmentWrapper") {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a EnchantmentWrapper object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
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
    pub fn enchantment(
        &mut self,
    ) -> Result<crate::bukkit::enchantments::Enchantment<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getEnchantment",
            "()Lorg/bukkit/enchantments/Enchantment;",
            &[],
        )?;
        let ret = {
            crate::bukkit::enchantments::Enchantment(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn start_level(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getStartLevel", "()I", &[])?;
        Ok(res.i().unwrap())
    }
    pub fn max_level(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getMaxLevel", "()I", &[])?;
        Ok(res.i().unwrap())
    }
    pub fn can_enchant_item(
        &mut self,
        arg0: impl Into<crate::bukkit::inventory::ItemStack<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "canEnchantItem",
            "(Lorg/bukkit/inventory/ItemStack;)Z",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(res.z().unwrap())
    }
    pub fn item_target(
        &mut self,
    ) -> Result<crate::bukkit::enchantments::EnchantmentTarget<'mc>, Box<dyn std::error::Error>>
    {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getItemTarget",
            "()Lorg/bukkit/enchantments/EnchantmentTarget;",
            &[],
        )?;
        let ret = {
            crate::bukkit::enchantments::EnchantmentTarget(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn is_treasure(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isTreasure", "()Z", &[])?;
        Ok(res.z().unwrap())
    }
    pub fn is_cursed(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isCursed", "()Z", &[])?;
        Ok(res.z().unwrap())
    }
    pub fn conflicts_with(
        &mut self,
        arg0: impl Into<crate::bukkit::enchantments::Enchantment<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "conflictsWith",
            "(Lorg/bukkit/enchantments/Enchantment;)Z",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(res.z().unwrap())
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
    pub fn key(&mut self) -> Result<crate::bukkit::NamespacedKey<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getKey",
            "()Lorg/bukkit/NamespacedKey;",
            &[],
        )?;
        let ret = {
            crate::bukkit::NamespacedKey(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn get_by_name(
        jni: crate::SharedJNIEnv<'mc>,
        arg0: String,
    ) -> Result<crate::bukkit::enchantments::Enchantment<'mc>, Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JObject::from(jni.new_string(arg0).unwrap());
        let cls = &jni.find_class("org/bukkit/enchantments/Enchantment")?;
        let res = jni.call_static_method(
            cls,
            "getByName",
            "(Ljava/lang/String;)Lorg/bukkit/enchantments/Enchantment;",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        let ret = {
            let obj = res.l()?;
            crate::bukkit::enchantments::Enchantment(jni, obj)
        };
        Ok(ret)
    }
    pub fn register_enchantment(
        jni: crate::SharedJNIEnv<'mc>,
        arg0: impl Into<crate::bukkit::enchantments::Enchantment<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let cls = &jni.find_class("void")?;
        let _res = jni.call_static_method(
            cls,
            "registerEnchantment",
            "(Lorg/bukkit/enchantments/Enchantment;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn get_by_key(
        jni: crate::SharedJNIEnv<'mc>,
        arg0: impl Into<crate::bukkit::NamespacedKey<'mc>>,
    ) -> Result<crate::bukkit::enchantments::Enchantment<'mc>, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let cls = &jni.find_class("org/bukkit/enchantments/Enchantment")?;
        let res = jni.call_static_method(
            cls,
            "getByKey",
            "(Lorg/bukkit/NamespacedKey;)Lorg/bukkit/enchantments/Enchantment;",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        let ret = {
            let obj = res.l()?;
            crate::bukkit::enchantments::Enchantment(jni, obj)
        };
        Ok(ret)
    }
    pub fn stop_accepting_registrations(
        jni: crate::SharedJNIEnv<'mc>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let cls = &jni.find_class("void")?;
        let _res = jni.call_static_method(cls, "stopAcceptingRegistrations", "()V", &[])?;
        Ok(())
    }
    pub fn is_accepting_registrations(
        jni: crate::SharedJNIEnv<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let cls = &jni.find_class("boolean")?;
        let res = jni.call_static_method(cls, "isAcceptingRegistrations", "()Z", &[])?;
        Ok(res.z().unwrap())
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
pub struct EnchantmentOffer<'mc>(
    pub(crate) crate::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> crate::JNIRaw<'mc> for EnchantmentOffer<'mc> {
    fn jni_ref(&self) -> crate::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> EnchantmentOffer<'mc> {
    pub fn new(
        jni: crate::SharedJNIEnv<'mc>,
        arg0: impl Into<crate::bukkit::enchantments::Enchantment<'mc>>,
        arg1: i32,
        arg2: i32,
    ) -> Result<crate::bukkit::enchantments::EnchantmentOffer<'mc>, Box<dyn std::error::Error>>
    {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let val_1 = jni::objects::JValueGen::Int(arg1.into());
        let val_2 = jni::objects::JValueGen::Int(arg2.into());
        let cls = &jni.find_class("org/bukkit/enchantments/EnchantmentOffer")?;
        let res = jni.new_object(
            cls,
            "(Lorg/bukkit/enchantments/Enchantment;II)V",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        )?;
        let ret = { crate::bukkit::enchantments::EnchantmentOffer(jni, res) };
        Ok(ret)
    }
    pub fn from_raw(
        env: &crate::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate EnchantmentOffer from null object.").into(),
            );
        }
        let cls = env.jni.borrow().get_object_class(&obj)?;
        let name_raw = env.call_method(cls, "getName", "()Ljava/lang/String;", &[])?;
        let oh = name_raw.l()?.into();
        let what = env.get_string(&oh)?;
        let name = what.to_string_lossy();
        if !name.ends_with("EnchantmentOffer") {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a EnchantmentOffer object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    pub fn enchantment(
        &mut self,
    ) -> Result<crate::bukkit::enchantments::Enchantment<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getEnchantment",
            "()Lorg/bukkit/enchantments/Enchantment;",
            &[],
        )?;
        let ret = {
            crate::bukkit::enchantments::Enchantment(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn set_enchantment(
        &mut self,
        arg0: impl Into<crate::bukkit::enchantments::Enchantment<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        self.jni_ref().call_method(
            &self.jni_object(),
            "setEnchantment",
            "(Lorg/bukkit/enchantments/Enchantment;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn enchantment_level(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getEnchantmentLevel", "()I", &[])?;
        Ok(res.i().unwrap())
    }
    pub fn set_enchantment_level(&mut self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Int(arg0.into());
        self.jni_ref().call_method(
            &self.jni_object(),
            "setEnchantmentLevel",
            "(I)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn cost(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getCost", "()I", &[])?;
        Ok(res.i().unwrap())
    }
    pub fn set_cost(&mut self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Int(arg0.into());
        self.jni_ref().call_method(
            &self.jni_object(),
            "setCost",
            "(I)V",
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
pub struct EnchantmentTarget<'mc>(
    pub(crate) crate::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> crate::JNIRaw<'mc> for EnchantmentTarget<'mc> {
    fn jni_ref(&self) -> crate::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> EnchantmentTarget<'mc> {
    pub fn from_raw(
        env: &crate::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate EnchantmentTarget from null object.").into(),
            );
        }
        let cls = env.jni.borrow().get_object_class(&obj)?;
        let name_raw = env.call_method(cls, "getName", "()Ljava/lang/String;", &[])?;
        let oh = name_raw.l()?.into();
        let what = env.get_string(&oh)?;
        let name = what.to_string_lossy();
        if !name.ends_with("EnchantmentTarget") {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a EnchantmentTarget object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    pub fn value_of_with_string(
        jni: crate::SharedJNIEnv<'mc>,
        arg0: std::option::Option<jni::objects::JClass<'mc>>,
        arg1: std::option::Option<String>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let val_0 = arg0.unwrap();
        let val_1 = jni::objects::JObject::from(jni.new_string(arg1.unwrap()).unwrap());
        let cls = &jni.find_class("java/lang/Enum")?;
        let res = jni.call_static_method(
            cls,
            "valueOf",
            "(Ljava/lang/Class;Ljava/lang/String;)Ljava/lang/Enum;",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        )?;
        Ok(res.l().unwrap())
    }
    pub fn includes_with_item_stack(
        &mut self,
        arg0: std::option::Option<impl Into<crate::bukkit::Material<'mc>>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.unwrap().into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "includes",
            "(Lorg/bukkit/Material;)Z",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(res.z().unwrap())
    }
    pub fn name(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "name", "()Ljava/lang/String;", &[])?;
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
    pub fn compare_to_with_object(
        &mut self,
        arg0: std::option::Option<jni::objects::JObject<'mc>>,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        let val_0 = arg0.unwrap();
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "compareTo",
            "(Ljava/lang/Enum;)I",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(res.i().unwrap())
    }
    pub fn describe_constable(
        &mut self,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "describeConstable",
            "()Ljava/util/Optional;",
            &[],
        )?;
        Ok(res.l().unwrap())
    }
    pub fn declaring_class(
        &mut self,
    ) -> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getDeclaringClass",
            "()Ljava/lang/Class;",
            &[],
        )?;
        Ok(unsafe { jni::objects::JClass::from_raw(res.as_jni().l) })
    }
    pub fn ordinal(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "ordinal", "()I", &[])?;
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
pub struct Enchantment<'mc>(
    pub(crate) crate::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> crate::JNIRaw<'mc> for Enchantment<'mc> {
    fn jni_ref(&self) -> crate::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Enchantment<'mc> {
    pub fn new(
        jni: crate::SharedJNIEnv<'mc>,
        arg0: impl Into<crate::bukkit::NamespacedKey<'mc>>,
    ) -> Result<crate::bukkit::enchantments::Enchantment<'mc>, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let cls = &jni.find_class("org/bukkit/enchantments/Enchantment")?;
        let res = jni.new_object(
            cls,
            "(Lorg/bukkit/NamespacedKey;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        let ret = { crate::bukkit::enchantments::Enchantment(jni, res) };
        Ok(ret)
    }
    pub fn from_raw(
        env: &crate::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Enchantment from null object.").into());
        }
        let cls = env.jni.borrow().get_object_class(&obj)?;
        let name_raw = env.call_method(cls, "getName", "()Ljava/lang/String;", &[])?;
        let oh = name_raw.l()?.into();
        let what = env.get_string(&oh)?;
        let name = what.to_string_lossy();
        if !name.ends_with("Enchantment") {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Enchantment object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
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
    pub fn key(&mut self) -> Result<crate::bukkit::NamespacedKey<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getKey",
            "()Lorg/bukkit/NamespacedKey;",
            &[],
        )?;
        let ret = {
            crate::bukkit::NamespacedKey(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn get_by_name(
        jni: crate::SharedJNIEnv<'mc>,
        arg0: String,
    ) -> Result<crate::bukkit::enchantments::Enchantment<'mc>, Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JObject::from(jni.new_string(arg0).unwrap());
        let cls = &jni.find_class("org/bukkit/enchantments/Enchantment")?;
        let res = jni.call_static_method(
            cls,
            "getByName",
            "(Ljava/lang/String;)Lorg/bukkit/enchantments/Enchantment;",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        let ret = {
            let obj = res.l()?;
            crate::bukkit::enchantments::Enchantment(jni, obj)
        };
        Ok(ret)
    }
    pub fn register_enchantment(
        jni: crate::SharedJNIEnv<'mc>,
        arg0: impl Into<crate::bukkit::enchantments::Enchantment<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let cls = &jni.find_class("void")?;
        let _res = jni.call_static_method(
            cls,
            "registerEnchantment",
            "(Lorg/bukkit/enchantments/Enchantment;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn get_by_key(
        jni: crate::SharedJNIEnv<'mc>,
        arg0: impl Into<crate::bukkit::NamespacedKey<'mc>>,
    ) -> Result<crate::bukkit::enchantments::Enchantment<'mc>, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let cls = &jni.find_class("org/bukkit/enchantments/Enchantment")?;
        let res = jni.call_static_method(
            cls,
            "getByKey",
            "(Lorg/bukkit/NamespacedKey;)Lorg/bukkit/enchantments/Enchantment;",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        let ret = {
            let obj = res.l()?;
            crate::bukkit::enchantments::Enchantment(jni, obj)
        };
        Ok(ret)
    }
    pub fn start_level(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getStartLevel", "()I", &[])?;
        Ok(res.i().unwrap())
    }
    pub fn max_level(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getMaxLevel", "()I", &[])?;
        Ok(res.i().unwrap())
    }
    pub fn can_enchant_item(
        &mut self,
        arg0: impl Into<crate::bukkit::inventory::ItemStack<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "canEnchantItem",
            "(Lorg/bukkit/inventory/ItemStack;)Z",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(res.z().unwrap())
    }
    pub fn stop_accepting_registrations(
        jni: crate::SharedJNIEnv<'mc>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let cls = &jni.find_class("void")?;
        let _res = jni.call_static_method(cls, "stopAcceptingRegistrations", "()V", &[])?;
        Ok(())
    }
    pub fn item_target(
        &mut self,
    ) -> Result<crate::bukkit::enchantments::EnchantmentTarget<'mc>, Box<dyn std::error::Error>>
    {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getItemTarget",
            "()Lorg/bukkit/enchantments/EnchantmentTarget;",
            &[],
        )?;
        let ret = {
            crate::bukkit::enchantments::EnchantmentTarget(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn is_treasure(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isTreasure", "()Z", &[])?;
        Ok(res.z().unwrap())
    }
    pub fn is_cursed(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isCursed", "()Z", &[])?;
        Ok(res.z().unwrap())
    }
    pub fn conflicts_with(
        &mut self,
        arg0: impl Into<crate::bukkit::enchantments::Enchantment<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "conflictsWith",
            "(Lorg/bukkit/enchantments/Enchantment;)Z",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(res.z().unwrap())
    }
    pub fn is_accepting_registrations(
        jni: crate::SharedJNIEnv<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let cls = &jni.find_class("boolean")?;
        let res = jni.call_static_method(cls, "isAcceptingRegistrations", "()Z", &[])?;
        Ok(res.z().unwrap())
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
impl<'mc> Into<crate::bukkit::Keyed<'mc>> for Enchantment<'mc> {
    fn into(self) -> crate::bukkit::Keyed<'mc> {
        crate::bukkit::Keyed::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
