#![allow(deprecated)]
use crate::JNIRaw;
pub struct PluginAwarenessFlags<'mc>(
    pub(crate) crate::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> crate::JNIRaw<'mc> for PluginAwarenessFlags<'mc> {
    fn jni_ref(&self) -> crate::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> PluginAwarenessFlags<'mc> {
    pub fn from_raw(
        env: &crate::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate PluginAwarenessFlags from null object.").into(),
            );
        }
        let cls = env.jni.borrow().get_object_class(&obj)?;
        let name_raw = env.call_method(cls, "getName", "()Ljava/lang/String;", &[])?;
        let oh = name_raw.l()?.into();
        let what = env.get_string(&oh)?;
        let name = what.to_string_lossy();
        if !name.ends_with("PluginAwarenessFlags") {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a PluginAwarenessFlags object, got {}",
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
        arg1: std::option::Option<impl Into<String>>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let val_0 = arg0.unwrap();
        let val_1 = jni::objects::JObject::from(jni.new_string(arg1.unwrap().into()).unwrap());
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
pub struct PluginDescriptionFile<'mc>(
    pub(crate) crate::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> crate::JNIRaw<'mc> for PluginDescriptionFile<'mc> {
    fn jni_ref(&self) -> crate::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> PluginDescriptionFile<'mc> {
    pub fn new_with_reader(
        jni: crate::SharedJNIEnv<'mc>,
        arg0: std::option::Option<jni::objects::JObject<'mc>>,
    ) -> Result<crate::bukkit::plugin::PluginDescriptionFile<'mc>, Box<dyn std::error::Error>> {
        let val_0 = arg0.unwrap();
        let cls = &jni.find_class("org/bukkit/plugin/PluginDescriptionFile")?;
        let res = jni.new_object(
            cls,
            "(Ljava/io/InputStream;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        let ret = { crate::bukkit::plugin::PluginDescriptionFile(jni, res) };
        Ok(ret)
    }
    pub fn new_with_string(
        jni: crate::SharedJNIEnv<'mc>,
        arg0: impl Into<String>,
        arg1: impl Into<String>,
        arg2: std::option::Option<impl Into<String>>,
    ) -> Result<crate::bukkit::plugin::PluginDescriptionFile<'mc>, Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JObject::from(jni.new_string(arg0.into()).unwrap());
        let val_1 = jni::objects::JObject::from(jni.new_string(arg1.into()).unwrap());
        let val_2 = jni::objects::JObject::from(jni.new_string(arg2.unwrap().into()).unwrap());
        let cls = &jni.find_class("org/bukkit/plugin/PluginDescriptionFile")?;
        let res = jni.new_object(
            cls,
            "(Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;)V",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        )?;
        let ret = { crate::bukkit::plugin::PluginDescriptionFile(jni, res) };
        Ok(ret)
    }
    pub fn from_raw(
        env: &crate::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate PluginDescriptionFile from null object."
            )
            .into());
        }
        let cls = env.jni.borrow().get_object_class(&obj)?;
        let name_raw = env.call_method(cls, "getName", "()Ljava/lang/String;", &[])?;
        let oh = name_raw.l()?.into();
        let what = env.get_string(&oh)?;
        let name = what.to_string_lossy();
        if !name.ends_with("PluginDescriptionFile") {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a PluginDescriptionFile object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    pub fn version(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getVersion",
            "()Ljava/lang/String;",
            &[],
        )?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    pub fn full_name(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getFullName",
            "()Ljava/lang/String;",
            &[],
        )?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
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
    #[deprecated]
    pub fn class_loader_of(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getClassLoaderOf",
            "()Ljava/lang/String;",
            &[],
        )?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    pub fn description(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getDescription",
            "()Ljava/lang/String;",
            &[],
        )?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    pub fn prefix(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getPrefix",
            "()Ljava/lang/String;",
            &[],
        )?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    pub fn main(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getMain",
            "()Ljava/lang/String;",
            &[],
        )?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    pub fn load(
        &mut self,
    ) -> Result<crate::bukkit::plugin::PluginLoadOrder<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getLoad",
            "()Lorg/bukkit/plugin/PluginLoadOrder;",
            &[],
        )?;
        let ret = {
            let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
            let variant = self
                .0
                .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
            let variant_str = self
                .0
                .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
                .to_string_lossy()
                .to_string();
            crate::bukkit::plugin::PluginLoadOrder(
                self.jni_ref(),
                raw_obj,
                crate::bukkit::plugin::PluginLoadOrder::from_string(variant_str).unwrap(),
            )
        };
        Ok(ret)
    }
    pub fn website(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getWebsite",
            "()Ljava/lang/String;",
            &[],
        )?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    pub fn permission_default(
        &mut self,
    ) -> Result<crate::bukkit::permissions::PermissionDefault<'mc>, Box<dyn std::error::Error>>
    {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getPermissionDefault",
            "()Lorg/bukkit/permissions/PermissionDefault;",
            &[],
        )?;
        let ret = {
            let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
            let variant = self
                .0
                .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
            let variant_str = self
                .0
                .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
                .to_string_lossy()
                .to_string();
            crate::bukkit::permissions::PermissionDefault(
                self.jni_ref(),
                raw_obj,
                crate::bukkit::permissions::PermissionDefault::from_string(variant_str).unwrap(),
            )
        };
        Ok(ret)
    }
    pub fn apiversion(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getAPIVersion",
            "()Ljava/lang/String;",
            &[],
        )?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    #[deprecated]
    pub fn raw_name(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getRawName",
            "()Ljava/lang/String;",
            &[],
        )?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
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
pub enum PluginLoadOrderEnum {
    Startup,
    Postworld,
}
impl std::fmt::Display for PluginLoadOrderEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            PluginLoadOrderEnum::Startup => f.write_str("STARTUP"),
            PluginLoadOrderEnum::Postworld => f.write_str("POSTWORLD"),
        }
    }
}
pub struct PluginLoadOrder<'mc>(
    pub(crate) crate::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
    pub PluginLoadOrderEnum,
);
impl<'mc> std::ops::Deref for PluginLoadOrder<'mc> {
    type Target = PluginLoadOrderEnum;
    fn deref(&self) -> &Self::Target {
        return &self.2;
    }
}
impl<'mc> JNIRaw<'mc> for PluginLoadOrder<'mc> {
    fn jni_ref(&self) -> crate::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> PluginLoadOrder<'mc> {
    pub const STARTUP: PluginLoadOrderEnum = PluginLoadOrderEnum::Startup;
    pub const POSTWORLD: PluginLoadOrderEnum = PluginLoadOrderEnum::Postworld;
    pub fn from_string(str: String) -> std::option::Option<PluginLoadOrderEnum> {
        match str.as_str() {
            "STARTUP" => Some(PluginLoadOrderEnum::Startup),
            "POSTWORLD" => Some(PluginLoadOrderEnum::Postworld),
            _ => None,
        }
    }
    pub fn value_of(
        jni: crate::SharedJNIEnv<'mc>,
        arg0: impl Into<String>,
    ) -> Result<crate::bukkit::plugin::PluginLoadOrder<'mc>, Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JObject::from(jni.new_string(arg0.into()).unwrap());
        let cls = &jni.find_class("org/bukkit/plugin/PluginLoadOrder")?;
        let res = jni.call_static_method(
            cls,
            "valueOf",
            "(Ljava/lang/String;)Lorg/bukkit/plugin/PluginLoadOrder;",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        let ret = {
            let obj = res.l()?;
            let raw_obj = obj;
            let variant = jni.call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
            let variant_str = jni
                .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
                .to_string_lossy()
                .to_string();
            crate::bukkit::plugin::PluginLoadOrder(
                jni,
                raw_obj,
                crate::bukkit::plugin::PluginLoadOrder::from_string(variant_str).unwrap(),
            )
        };
        Ok(ret)
    }
}
pub struct RegisteredServiceProvider<'mc>(
    pub(crate) crate::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> crate::JNIRaw<'mc> for RegisteredServiceProvider<'mc> {
    fn jni_ref(&self) -> crate::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> RegisteredServiceProvider<'mc> {
    pub fn new(
        jni: crate::SharedJNIEnv<'mc>,
        arg0: jni::objects::JClass<'mc>,
        arg1: jni::objects::JObject<'mc>,
        arg2: impl Into<crate::bukkit::plugin::ServicePriority<'mc>>,
        arg3: impl Into<crate::bukkit::plugin::Plugin<'mc>>,
    ) -> Result<crate::bukkit::plugin::RegisteredServiceProvider<'mc>, Box<dyn std::error::Error>>
    {
        let val_0 = arg0;
        let val_1 = arg1;
        let val_2 = unsafe { jni::objects::JObject::from_raw(arg2.into().1.clone()) };
        let val_3 = unsafe { jni::objects::JObject::from_raw(arg3.into().1.clone()) };
        let cls = &jni.find_class("org/bukkit/plugin/RegisteredServiceProvider")?;
        let res = jni.new_object(cls,
"(Ljava/lang/Class;Ljava/lang/Object;Lorg/bukkit/plugin/ServicePriority;Lorg/bukkit/plugin/Plugin;)V",&[jni::objects::JValueGen::from(&val_0),jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2),jni::objects::JValueGen::from(&val_3)])?;
        let ret = { crate::bukkit::plugin::RegisteredServiceProvider(jni, res) };
        Ok(ret)
    }
    pub fn from_raw(
        env: &crate::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate RegisteredServiceProvider from null object."
            )
            .into());
        }
        let cls = env.jni.borrow().get_object_class(&obj)?;
        let name_raw = env.call_method(cls, "getName", "()Ljava/lang/String;", &[])?;
        let oh = name_raw.l()?.into();
        let what = env.get_string(&oh)?;
        let name = what.to_string_lossy();
        if !name.ends_with("RegisteredServiceProvider") {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a RegisteredServiceProvider object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    pub fn provider(&mut self) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getProvider",
            "()Ljava/lang/Object;",
            &[],
        )?;
        Ok(res.l().unwrap())
    }
    pub fn compare_to_with_object(
        &mut self,
        arg0: std::option::Option<impl Into<crate::bukkit::plugin::RegisteredServiceProvider<'mc>>>,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.unwrap().into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "compareTo",
            "(Lorg/bukkit/plugin/RegisteredServiceProvider;)I",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(res.i().unwrap())
    }
    pub fn priority(
        &mut self,
    ) -> Result<crate::bukkit::plugin::ServicePriority<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getPriority",
            "()Lorg/bukkit/plugin/ServicePriority;",
            &[],
        )?;
        let ret = {
            crate::bukkit::plugin::ServicePriority(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn plugin(
        &mut self,
    ) -> Result<crate::bukkit::plugin::Plugin<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getPlugin",
            "()Lorg/bukkit/plugin/Plugin;",
            &[],
        )?;
        let ret = {
            crate::bukkit::plugin::Plugin(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn service(&mut self) -> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getService",
            "()Ljava/lang/Class;",
            &[],
        )?;
        Ok(unsafe { jni::objects::JClass::from_raw(res.as_jni().l) })
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
pub struct ServicePriority<'mc>(
    pub(crate) crate::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> crate::JNIRaw<'mc> for ServicePriority<'mc> {
    fn jni_ref(&self) -> crate::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> ServicePriority<'mc> {
    pub fn from_raw(
        env: &crate::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate ServicePriority from null object.").into(),
            );
        }
        let cls = env.jni.borrow().get_object_class(&obj)?;
        let name_raw = env.call_method(cls, "getName", "()Ljava/lang/String;", &[])?;
        let oh = name_raw.l()?.into();
        let what = env.get_string(&oh)?;
        let name = what.to_string_lossy();
        if !name.ends_with("ServicePriority") {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a ServicePriority object, got {}",
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
        arg1: std::option::Option<impl Into<String>>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let val_0 = arg0.unwrap();
        let val_1 = jni::objects::JObject::from(jni.new_string(arg1.unwrap().into()).unwrap());
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
/// An instantiatable struct that implements PluginAwareness. Needed for returning it from Java.
pub struct PluginAwareness<'mc>(
    pub(crate) crate::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> PluginAwareness<'mc> {}
impl<'mc> crate::JNIRaw<'mc> for PluginAwareness<'mc> {
    fn jni_ref(&self) -> crate::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
/// An instantiatable struct that implements PluginManager. Needed for returning it from Java.
pub struct PluginManager<'mc>(
    pub(crate) crate::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> PluginManager<'mc> {
    pub fn from_raw(
        env: &crate::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate PluginManager from null object.").into());
        }
        let cls = env.jni.borrow().get_object_class(&obj)?;
        let name_raw = env.call_method(cls, "getName", "()Ljava/lang/String;", &[])?;
        let oh = name_raw.l()?.into();
        let what = env.get_string(&oh)?;
        let name = what.to_string_lossy();
        if !name.ends_with("PluginManager") {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a PluginManager object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    pub fn get_permission(
        &mut self,
        arg0: impl Into<String>,
    ) -> Result<crate::bukkit::permissions::Permission<'mc>, Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getPermission",
            "(Ljava/lang/String;)Lorg/bukkit/permissions/Permission;",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        let ret = {
            crate::bukkit::permissions::Permission(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn get_plugin(
        &mut self,
        arg0: impl Into<String>,
    ) -> Result<crate::bukkit::plugin::Plugin<'mc>, Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getPlugin",
            "(Ljava/lang/String;)Lorg/bukkit/plugin/Plugin;",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        let ret = {
            crate::bukkit::plugin::Plugin(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn call_event(
        &mut self,
        arg0: impl Into<crate::bukkit::event::Event<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        self.jni_ref().call_method(
            &self.jni_object(),
            "callEvent",
            "(Lorg/bukkit/event/Event;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn recalculate_permission_defaults(
        &mut self,
        arg0: impl Into<crate::bukkit::permissions::Permission<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        self.jni_ref().call_method(
            &self.jni_object(),
            "recalculatePermissionDefaults",
            "(Lorg/bukkit/permissions/Permission;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn add_permission(
        &mut self,
        arg0: impl Into<crate::bukkit::permissions::Permission<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        self.jni_ref().call_method(
            &self.jni_object(),
            "addPermission",
            "(Lorg/bukkit/permissions/Permission;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn subscribe_to_default_perms(
        &mut self,
        arg0: bool,
        arg1: impl Into<crate::bukkit::permissions::Permissible<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // -2
        let val_0 = jni::objects::JValueGen::Bool(arg0.into());
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg1.into().1.clone()) };
        self.jni_ref().call_method(
            &self.jni_object(),
            "subscribeToDefaultPerms",
            "(ZLorg/bukkit/permissions/Permissible;)V",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        )?;
        Ok(())
    }
    pub fn subscribe_to_permission(
        &mut self,
        arg0: impl Into<String>,
        arg1: impl Into<crate::bukkit::permissions::Permissible<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg1.into().1.clone()) };
        self.jni_ref().call_method(
            &self.jni_object(),
            "subscribeToPermission",
            "(Ljava/lang/String;Lorg/bukkit/permissions/Permissible;)V",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        )?;
        Ok(())
    }
    pub fn unsubscribe_from_permission(
        &mut self,
        arg0: impl Into<String>,
        arg1: impl Into<crate::bukkit::permissions::Permissible<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg1.into().1.clone()) };
        self.jni_ref().call_method(
            &self.jni_object(),
            "unsubscribeFromPermission",
            "(Ljava/lang/String;Lorg/bukkit/permissions/Permissible;)V",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        )?;
        Ok(())
    }
    pub fn unsubscribe_from_default_perms(
        &mut self,
        arg0: bool,
        arg1: impl Into<crate::bukkit::permissions::Permissible<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // -2
        let val_0 = jni::objects::JValueGen::Bool(arg0.into());
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg1.into().1.clone()) };
        self.jni_ref().call_method(
            &self.jni_object(),
            "unsubscribeFromDefaultPerms",
            "(ZLorg/bukkit/permissions/Permissible;)V",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        )?;
        Ok(())
    }
    pub fn use_timings(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "useTimings", "()Z", &[])?;
        Ok(res.z().unwrap())
    }
    pub fn enable_plugin(
        &mut self,
        arg0: impl Into<crate::bukkit::plugin::Plugin<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
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
        arg0: impl Into<crate::bukkit::plugin::Plugin<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        self.jni_ref().call_method(
            &self.jni_object(),
            "disablePlugin",
            "(Lorg/bukkit/plugin/Plugin;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn register_interface(
        &mut self,
        arg0: jni::objects::JClass<'mc>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = arg0;
        self.jni_ref().call_method(
            &self.jni_object(),
            "registerInterface",
            "(Ljava/lang/Class;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn is_plugin_enabled_with_string(
        &mut self,
        arg0: std::option::Option<impl Into<crate::bukkit::plugin::Plugin<'mc>>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.unwrap().into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "isPluginEnabled",
            "(Lorg/bukkit/plugin/Plugin;)Z",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(res.z().unwrap())
    }
    pub fn disable_plugins(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.jni_ref()
            .call_method(&self.jni_object(), "disablePlugins", "()V", &[])?;
        Ok(())
    }
    pub fn clear_plugins(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.jni_ref()
            .call_method(&self.jni_object(), "clearPlugins", "()V", &[])?;
        Ok(())
    }
    pub fn register_events(
        &mut self,
        arg0: impl Into<crate::bukkit::event::Listener<'mc>>,
        arg1: impl Into<crate::bukkit::plugin::Plugin<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg1.into().1.clone()) };
        self.jni_ref().call_method(
            &self.jni_object(),
            "registerEvents",
            "(Lorg/bukkit/event/Listener;Lorg/bukkit/plugin/Plugin;)V",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        )?;
        Ok(())
    }
    pub fn register_event_with_class(
        &mut self,
        arg0: jni::objects::JClass<'mc>,
        arg1: impl Into<crate::bukkit::event::Listener<'mc>>,
        arg2: impl Into<crate::bukkit::event::EventPriority<'mc>>,
        arg3: impl Into<crate::bukkit::plugin::EventExecutor<'mc>>,
        arg4: std::option::Option<impl Into<crate::bukkit::plugin::Plugin<'mc>>>,
        arg5: std::option::Option<bool>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = arg0;
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg1.into().1.clone()) };
        let val_2 = unsafe { jni::objects::JObject::from_raw(arg2.into().1.clone()) };
        let val_3 = unsafe { jni::objects::JObject::from_raw(arg3.into().1.clone()) };
        let val_4 = unsafe { jni::objects::JObject::from_raw(arg4.unwrap().into().1.clone()) };
        // 4
        let val_5 = jni::objects::JValueGen::Bool(arg5.unwrap().into());
        self.jni_ref().call_method(&self.jni_object(),"registerEvent","(Ljava/lang/Class;Lorg/bukkit/event/Listener;Lorg/bukkit/event/EventPriority;Lorg/bukkit/plugin/EventExecutor;Lorg/bukkit/plugin/Plugin;Z)V",&[jni::objects::JValueGen::from(&val_0),jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2),jni::objects::JValueGen::from(&val_3),jni::objects::JValueGen::from(&val_4),jni::objects::JValueGen::from(&val_5)])?;
        Ok(())
    }
    pub fn remove_permission_with_permission(
        &mut self,
        arg0: std::option::Option<impl Into<String>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 =
            jni::objects::JObject::from(self.jni_ref().new_string(arg0.unwrap().into()).unwrap());
        self.jni_ref().call_method(
            &self.jni_object(),
            "removePermission",
            "(Ljava/lang/String;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
}
impl<'mc> crate::JNIRaw<'mc> for PluginManager<'mc> {
    fn jni_ref(&self) -> crate::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
pub struct PluginBase<'mc>(
    pub(crate) crate::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> crate::JNIRaw<'mc> for PluginBase<'mc> {
    fn jni_ref(&self) -> crate::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> PluginBase<'mc> {
    pub fn from_extendable(
        env: &crate::SharedJNIEnv<'mc>,
        plugin: &'mc crate::bukkit::plugin::Plugin,
        lib_name: String,
        name: String,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let obj = unsafe { plugin.new_extendable("PluginBase", name, lib_name) }?;
        Self::from_raw(env, obj)
    }
    pub fn new(
        jni: crate::SharedJNIEnv<'mc>,
    ) -> Result<crate::bukkit::plugin::PluginBase<'mc>, Box<dyn std::error::Error>> {
        let cls = &jni.find_class("org/bukkit/plugin/PluginBase")?;
        let res = jni.new_object(cls, "()V", &[])?;
        let ret = { crate::bukkit::plugin::PluginBase(jni, res) };
        Ok(ret)
    }
    pub fn from_raw(
        env: &crate::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate PluginBase from null object.").into());
        }
        let cls = env.jni.borrow().get_object_class(&obj)?;
        let name_raw = env.call_method(cls, "getName", "()Ljava/lang/String;", &[])?;
        let oh = name_raw.l()?.into();
        let what = env.get_string(&oh)?;
        let name = what.to_string_lossy();
        if !name.ends_with("PluginBase") {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a PluginBase object, got {}",
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
    pub fn get_resource(
        &mut self,
        arg0: impl Into<String>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
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
    pub fn get_default_biome_provider(
        &mut self,
        arg0: impl Into<String>,
        arg1: impl Into<String>,
    ) -> Result<crate::bukkit::generator::BiomeProvider<'mc>, Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg1.into()).unwrap());
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
        arg0: impl Into<String>,
        arg1: bool,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        // -2
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
        // -2
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
        arg0: impl Into<String>,
        arg1: impl Into<String>,
    ) -> Result<crate::bukkit::generator::ChunkGenerator<'mc>, Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg1.into()).unwrap());
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
    pub fn on_command(
        &mut self,
        arg0: impl Into<crate::bukkit::command::CommandSender<'mc>>,
        arg1: impl Into<crate::bukkit::command::Command<'mc>>,
        arg2: impl Into<String>,
        _arg3: Vec<impl Into<String>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg1.into().1.clone()) };
        let val_2 = jni::objects::JObject::from(self.jni_ref().new_string(arg2.into()).unwrap());
        let res =
self.jni_ref().call_method(&self.jni_object(),"onCommand","(Lorg/bukkit/command/CommandSender;Lorg/bukkit/command/Command;Ljava/lang/String;Ljava/lang/String;)Z",&[jni::objects::JValueGen::from(&val_0),jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)])?;
        Ok(res.z().unwrap())
    }
}
impl<'mc> Into<crate::bukkit::plugin::Plugin<'mc>> for PluginBase<'mc> {
    fn into(self) -> crate::bukkit::plugin::Plugin<'mc> {
        crate::bukkit::plugin::Plugin::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
/// An instantiatable struct that implements EventExecutor. Needed for returning it from Java.
pub struct EventExecutor<'mc>(
    pub(crate) crate::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> EventExecutor<'mc> {
    pub fn from_raw(
        env: &crate::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate EventExecutor from null object.").into());
        }
        let cls = env.jni.borrow().get_object_class(&obj)?;
        let name_raw = env.call_method(cls, "getName", "()Ljava/lang/String;", &[])?;
        let oh = name_raw.l()?.into();
        let what = env.get_string(&oh)?;
        let name = what.to_string_lossy();
        if !name.ends_with("EventExecutor") {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a EventExecutor object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    pub fn execute(
        &mut self,
        arg0: impl Into<crate::bukkit::event::Listener<'mc>>,
        arg1: impl Into<crate::bukkit::event::Event<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg1.into().1.clone()) };
        self.jni_ref().call_method(
            &self.jni_object(),
            "execute",
            "(Lorg/bukkit/event/Listener;Lorg/bukkit/event/Event;)V",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        )?;
        Ok(())
    }
}
impl<'mc> crate::JNIRaw<'mc> for EventExecutor<'mc> {
    fn jni_ref(&self) -> crate::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
/// An instantiatable struct that implements ServicesManager. Needed for returning it from Java.
pub struct ServicesManager<'mc>(
    pub(crate) crate::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> ServicesManager<'mc> {
    pub fn from_raw(
        env: &crate::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate ServicesManager from null object.").into(),
            );
        }
        let cls = env.jni.borrow().get_object_class(&obj)?;
        let name_raw = env.call_method(cls, "getName", "()Ljava/lang/String;", &[])?;
        let oh = name_raw.l()?.into();
        let what = env.get_string(&oh)?;
        let name = what.to_string_lossy();
        if !name.ends_with("ServicesManager") {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a ServicesManager object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    pub fn load(
        &mut self,
        arg0: jni::objects::JClass<'mc>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let val_0 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "load",
            "(Ljava/lang/Class;)Ljava/lang/Object;",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(res.l().unwrap())
    }
    pub fn register(
        &mut self,
        arg0: jni::objects::JClass<'mc>,
        arg1: jni::objects::JObject<'mc>,
        arg2: impl Into<crate::bukkit::plugin::Plugin<'mc>>,
        arg3: impl Into<crate::bukkit::plugin::ServicePriority<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = arg0;
        let val_1 = arg1;
        let val_2 = unsafe { jni::objects::JObject::from_raw(arg2.into().1.clone()) };
        let val_3 = unsafe { jni::objects::JObject::from_raw(arg3.into().1.clone()) };
        self.jni_ref().call_method(&self.jni_object(),"register","(Ljava/lang/Class;Ljava/lang/Object;Lorg/bukkit/plugin/Plugin;Lorg/bukkit/plugin/ServicePriority;)V",&[jni::objects::JValueGen::from(&val_0),jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2),jni::objects::JValueGen::from(&val_3)])?;
        Ok(())
    }
    pub fn unregister_with_object(
        &mut self,
        arg0: std::option::Option<jni::objects::JClass<'mc>>,
        arg1: std::option::Option<jni::objects::JObject<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = arg0.unwrap();
        let val_1 = arg1.unwrap();
        self.jni_ref().call_method(
            &self.jni_object(),
            "unregister",
            "(Ljava/lang/Class;Ljava/lang/Object;)V",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        )?;
        Ok(())
    }
    pub fn unregister_all(
        &mut self,
        arg0: impl Into<crate::bukkit::plugin::Plugin<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        self.jni_ref().call_method(
            &self.jni_object(),
            "unregisterAll",
            "(Lorg/bukkit/plugin/Plugin;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn get_registration(
        &mut self,
        arg0: jni::objects::JClass<'mc>,
    ) -> Result<crate::bukkit::plugin::RegisteredServiceProvider<'mc>, Box<dyn std::error::Error>>
    {
        let val_0 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getRegistration",
            "(Ljava/lang/Class;)Lorg/bukkit/plugin/RegisteredServiceProvider;",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        let ret = {
            crate::bukkit::plugin::RegisteredServiceProvider(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn is_provided_for(
        &mut self,
        arg0: jni::objects::JClass<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "isProvidedFor",
            "(Ljava/lang/Class;)Z",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(res.z().unwrap())
    }
}
impl<'mc> crate::JNIRaw<'mc> for ServicesManager<'mc> {
    fn jni_ref(&self) -> crate::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
pub struct SimpleServicesManager<'mc>(
    pub(crate) crate::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> crate::JNIRaw<'mc> for SimpleServicesManager<'mc> {
    fn jni_ref(&self) -> crate::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> SimpleServicesManager<'mc> {
    pub fn new(
        jni: crate::SharedJNIEnv<'mc>,
    ) -> Result<crate::bukkit::plugin::SimpleServicesManager<'mc>, Box<dyn std::error::Error>> {
        let cls = &jni.find_class("org/bukkit/plugin/SimpleServicesManager")?;
        let res = jni.new_object(cls, "()V", &[])?;
        let ret = { crate::bukkit::plugin::SimpleServicesManager(jni, res) };
        Ok(ret)
    }
    pub fn from_raw(
        env: &crate::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate SimpleServicesManager from null object."
            )
            .into());
        }
        let cls = env.jni.borrow().get_object_class(&obj)?;
        let name_raw = env.call_method(cls, "getName", "()Ljava/lang/String;", &[])?;
        let oh = name_raw.l()?.into();
        let what = env.get_string(&oh)?;
        let name = what.to_string_lossy();
        if !name.ends_with("SimpleServicesManager") {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a SimpleServicesManager object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    pub fn load(
        &mut self,
        arg0: jni::objects::JClass<'mc>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let val_0 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "load",
            "(Ljava/lang/Class;)Ljava/lang/Object;",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(res.l().unwrap())
    }
    pub fn register(
        &mut self,
        arg0: jni::objects::JClass<'mc>,
        arg1: jni::objects::JObject<'mc>,
        arg2: impl Into<crate::bukkit::plugin::Plugin<'mc>>,
        arg3: impl Into<crate::bukkit::plugin::ServicePriority<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = arg0;
        let val_1 = arg1;
        let val_2 = unsafe { jni::objects::JObject::from_raw(arg2.into().1.clone()) };
        let val_3 = unsafe { jni::objects::JObject::from_raw(arg3.into().1.clone()) };
        self.jni_ref().call_method(&self.jni_object(),"register","(Ljava/lang/Class;Ljava/lang/Object;Lorg/bukkit/plugin/Plugin;Lorg/bukkit/plugin/ServicePriority;)V",&[jni::objects::JValueGen::from(&val_0),jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2),jni::objects::JValueGen::from(&val_3)])?;
        Ok(())
    }
    pub fn unregister_with_object(
        &mut self,
        arg0: std::option::Option<jni::objects::JClass<'mc>>,
        arg1: std::option::Option<jni::objects::JObject<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = arg0.unwrap();
        let val_1 = arg1.unwrap();
        self.jni_ref().call_method(
            &self.jni_object(),
            "unregister",
            "(Ljava/lang/Class;Ljava/lang/Object;)V",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        )?;
        Ok(())
    }
    pub fn unregister_all(
        &mut self,
        arg0: impl Into<crate::bukkit::plugin::Plugin<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        self.jni_ref().call_method(
            &self.jni_object(),
            "unregisterAll",
            "(Lorg/bukkit/plugin/Plugin;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn get_registration(
        &mut self,
        arg0: jni::objects::JClass<'mc>,
    ) -> Result<crate::bukkit::plugin::RegisteredServiceProvider<'mc>, Box<dyn std::error::Error>>
    {
        let val_0 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getRegistration",
            "(Ljava/lang/Class;)Lorg/bukkit/plugin/RegisteredServiceProvider;",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        let ret = {
            crate::bukkit::plugin::RegisteredServiceProvider(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn is_provided_for(
        &mut self,
        arg0: jni::objects::JClass<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "isProvidedFor",
            "(Ljava/lang/Class;)Z",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
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
impl<'mc> Into<crate::bukkit::plugin::ServicesManager<'mc>> for SimpleServicesManager<'mc> {
    fn into(self) -> crate::bukkit::plugin::ServicesManager<'mc> {
        crate::bukkit::plugin::ServicesManager::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
pub struct TimedRegisteredListener<'mc>(
    pub(crate) crate::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> crate::JNIRaw<'mc> for TimedRegisteredListener<'mc> {
    fn jni_ref(&self) -> crate::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> TimedRegisteredListener<'mc> {
    pub fn new(
        jni: crate::SharedJNIEnv<'mc>,
        arg0: impl Into<crate::bukkit::event::Listener<'mc>>,
        arg1: impl Into<crate::bukkit::plugin::EventExecutor<'mc>>,
        arg2: impl Into<crate::bukkit::event::EventPriority<'mc>>,
        arg3: impl Into<crate::bukkit::plugin::Plugin<'mc>>,
        arg4: bool,
    ) -> Result<crate::bukkit::plugin::TimedRegisteredListener<'mc>, Box<dyn std::error::Error>>
    {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg1.into().1.clone()) };
        let val_2 = unsafe { jni::objects::JObject::from_raw(arg2.into().1.clone()) };
        let val_3 = unsafe { jni::objects::JObject::from_raw(arg3.into().1.clone()) };
        // -2
        let val_4 = jni::objects::JValueGen::Bool(arg4.into());
        let cls = &jni.find_class("org/bukkit/plugin/TimedRegisteredListener")?;
        let res = jni.new_object(cls,
"(Lorg/bukkit/event/Listener;Lorg/bukkit/plugin/EventExecutor;Lorg/bukkit/event/EventPriority;Lorg/bukkit/plugin/Plugin;Z)V",&[jni::objects::JValueGen::from(&val_0),jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2),jni::objects::JValueGen::from(&val_3),jni::objects::JValueGen::from(&val_4)])?;
        let ret = { crate::bukkit::plugin::TimedRegisteredListener(jni, res) };
        Ok(ret)
    }
    pub fn from_raw(
        env: &crate::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate TimedRegisteredListener from null object."
            )
            .into());
        }
        let cls = env.jni.borrow().get_object_class(&obj)?;
        let name_raw = env.call_method(cls, "getName", "()Ljava/lang/String;", &[])?;
        let oh = name_raw.l()?.into();
        let what = env.get_string(&oh)?;
        let name = what.to_string_lossy();
        if !name.ends_with("TimedRegisteredListener") {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a TimedRegisteredListener object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    pub fn reset(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.jni_ref()
            .call_method(&self.jni_object(), "reset", "()V", &[])?;
        Ok(())
    }
    pub fn count(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getCount", "()I", &[])?;
        Ok(res.i().unwrap())
    }
    pub fn call_event(
        &mut self,
        arg0: impl Into<crate::bukkit::event::Event<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        self.jni_ref().call_method(
            &self.jni_object(),
            "callEvent",
            "(Lorg/bukkit/event/Event;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn total_time(&mut self) -> Result<i64, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getTotalTime", "()J", &[])?;
        Ok(res.j().unwrap())
    }
    pub fn event_class(&mut self) -> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getEventClass",
            "()Ljava/lang/Class;",
            &[],
        )?;
        Ok(unsafe { jni::objects::JClass::from_raw(res.as_jni().l) })
    }
    pub fn has_multiple(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasMultiple", "()Z", &[])?;
        Ok(res.z().unwrap())
    }
    pub fn priority(
        &mut self,
    ) -> Result<crate::bukkit::event::EventPriority<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getPriority",
            "()Lorg/bukkit/event/EventPriority;",
            &[],
        )?;
        let ret = {
            let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
            let variant = self
                .0
                .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
            let variant_str = self
                .0
                .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
                .to_string_lossy()
                .to_string();
            crate::bukkit::event::EventPriority(
                self.jni_ref(),
                raw_obj,
                crate::bukkit::event::EventPriority::from_string(variant_str).unwrap(),
            )
        };
        Ok(ret)
    }
    pub fn listener(
        &mut self,
    ) -> Result<crate::bukkit::event::Listener<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getListener",
            "()Lorg/bukkit/event/Listener;",
            &[],
        )?;
        let ret = {
            crate::bukkit::event::Listener(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn plugin(
        &mut self,
    ) -> Result<crate::bukkit::plugin::Plugin<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getPlugin",
            "()Lorg/bukkit/plugin/Plugin;",
            &[],
        )?;
        let ret = {
            crate::bukkit::plugin::Plugin(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn is_ignoring_cancelled(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isIgnoringCancelled", "()Z", &[])?;
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
impl<'mc> Into<crate::bukkit::plugin::RegisteredListener<'mc>> for TimedRegisteredListener<'mc> {
    fn into(self) -> crate::bukkit::plugin::RegisteredListener<'mc> {
        crate::bukkit::plugin::RegisteredListener::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
/// An instantiatable struct that implements PluginLoader. Needed for returning it from Java.
pub struct PluginLoader<'mc>(
    pub(crate) crate::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> PluginLoader<'mc> {
    pub fn from_extendable(
        env: &crate::SharedJNIEnv<'mc>,
        plugin: &'mc crate::bukkit::plugin::Plugin,
        lib_name: String,
        name: String,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let obj = unsafe { plugin.new_extendable("PluginLoader", name, lib_name) }?;
        Self::from_raw(env, obj)
    }
    pub fn from_raw(
        env: &crate::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate PluginLoader from null object.").into());
        }
        let cls = env.jni.borrow().get_object_class(&obj)?;
        let name_raw = env.call_method(cls, "getName", "()Ljava/lang/String;", &[])?;
        let oh = name_raw.l()?.into();
        let what = env.get_string(&oh)?;
        let name = what.to_string_lossy();
        if !name.ends_with("PluginLoader") {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a PluginLoader object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    pub fn enable_plugin(
        &mut self,
        arg0: impl Into<crate::bukkit::plugin::Plugin<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
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
        arg0: impl Into<crate::bukkit::plugin::Plugin<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        self.jni_ref().call_method(
            &self.jni_object(),
            "disablePlugin",
            "(Lorg/bukkit/plugin/Plugin;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
}
impl<'mc> crate::JNIRaw<'mc> for PluginLoader<'mc> {
    fn jni_ref(&self) -> crate::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
pub struct RegisteredListener<'mc>(
    pub(crate) crate::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> crate::JNIRaw<'mc> for RegisteredListener<'mc> {
    fn jni_ref(&self) -> crate::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> RegisteredListener<'mc> {
    pub fn new(
        jni: crate::SharedJNIEnv<'mc>,
        arg0: impl Into<crate::bukkit::event::Listener<'mc>>,
        arg1: impl Into<crate::bukkit::plugin::EventExecutor<'mc>>,
        arg2: impl Into<crate::bukkit::event::EventPriority<'mc>>,
        arg3: impl Into<crate::bukkit::plugin::Plugin<'mc>>,
        arg4: bool,
    ) -> Result<crate::bukkit::plugin::RegisteredListener<'mc>, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg1.into().1.clone()) };
        let val_2 = unsafe { jni::objects::JObject::from_raw(arg2.into().1.clone()) };
        let val_3 = unsafe { jni::objects::JObject::from_raw(arg3.into().1.clone()) };
        // -2
        let val_4 = jni::objects::JValueGen::Bool(arg4.into());
        let cls = &jni.find_class("org/bukkit/plugin/RegisteredListener")?;
        let res = jni.new_object(cls,
"(Lorg/bukkit/event/Listener;Lorg/bukkit/plugin/EventExecutor;Lorg/bukkit/event/EventPriority;Lorg/bukkit/plugin/Plugin;Z)V",&[jni::objects::JValueGen::from(&val_0),jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2),jni::objects::JValueGen::from(&val_3),jni::objects::JValueGen::from(&val_4)])?;
        let ret = { crate::bukkit::plugin::RegisteredListener(jni, res) };
        Ok(ret)
    }
    pub fn from_raw(
        env: &crate::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate RegisteredListener from null object.").into(),
            );
        }
        let cls = env.jni.borrow().get_object_class(&obj)?;
        let name_raw = env.call_method(cls, "getName", "()Ljava/lang/String;", &[])?;
        let oh = name_raw.l()?.into();
        let what = env.get_string(&oh)?;
        let name = what.to_string_lossy();
        if !name.ends_with("RegisteredListener") {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a RegisteredListener object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    pub fn priority(
        &mut self,
    ) -> Result<crate::bukkit::event::EventPriority<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getPriority",
            "()Lorg/bukkit/event/EventPriority;",
            &[],
        )?;
        let ret = {
            let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
            let variant = self
                .0
                .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
            let variant_str = self
                .0
                .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
                .to_string_lossy()
                .to_string();
            crate::bukkit::event::EventPriority(
                self.jni_ref(),
                raw_obj,
                crate::bukkit::event::EventPriority::from_string(variant_str).unwrap(),
            )
        };
        Ok(ret)
    }
    pub fn listener(
        &mut self,
    ) -> Result<crate::bukkit::event::Listener<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getListener",
            "()Lorg/bukkit/event/Listener;",
            &[],
        )?;
        let ret = {
            crate::bukkit::event::Listener(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn plugin(
        &mut self,
    ) -> Result<crate::bukkit::plugin::Plugin<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getPlugin",
            "()Lorg/bukkit/plugin/Plugin;",
            &[],
        )?;
        let ret = {
            crate::bukkit::plugin::Plugin(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn call_event(
        &mut self,
        arg0: impl Into<crate::bukkit::event::Event<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        self.jni_ref().call_method(
            &self.jni_object(),
            "callEvent",
            "(Lorg/bukkit/event/Event;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn is_ignoring_cancelled(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isIgnoringCancelled", "()Z", &[])?;
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
/// An instantiatable struct that implements Plugin. Needed for returning it from Java.
pub struct Plugin<'mc>(
    pub(crate) crate::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> Plugin<'mc> {
    pub fn from_extendable(
        env: &crate::SharedJNIEnv<'mc>,
        plugin: &'mc crate::bukkit::plugin::Plugin,
        lib_name: String,
        name: String,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let obj = unsafe { plugin.new_extendable("Plugin", name, lib_name) }?;
        Self::from_raw(env, obj)
    }
    /// Return one of the extendable classes that BlackBox supports, based on the value given.
    ///
    /// ## Safety
    /// - It returns a Java Object that you must then cast into the proper object via JNI. You are responsible for the checks yourself.
    /// - This function is specific to the BlackboxPlugin class supplied within the plugin, and will error out if you pass a regular JavaPlugin.
    pub unsafe fn new_extendable(
        &self,
        class_name: impl Into<String>
            + std::convert::AsRef<str>
            + std::convert::AsRef<str>
            + std::convert::AsRef<str>,
        name: impl Into<String> + std::convert::AsRef<str> + std::convert::AsRef<str>,
        lib_name: impl Into<String> + std::convert::AsRef<str> + std::convert::AsRef<str>,
    ) -> Result<jni::objects::JObject, Box<dyn std::error::Error>> {
        let obj = self.0.call_method(
            &self.1,
            "newExtendable",
            "(Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;)Ljava/lang/Object",
            &[
                jni::objects::JValueGen::from(&jni::objects::JObject::from(
                    self.0.new_string(class_name).unwrap(),
                )),
                jni::objects::JValueGen::from(&jni::objects::JObject::from(
                    self.0.new_string(name).unwrap(),
                )),
                jni::objects::JValueGen::from(&jni::objects::JObject::from(
                    self.0.new_string(lib_name).unwrap(),
                )),
            ],
        )?;
        Ok(jni::objects::JObject::from_raw(obj.l()?.clone()))
    }
    pub fn from_raw(
        env: &crate::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Plugin from null object.").into());
        }
        let cls = env.jni.borrow().get_object_class(&obj)?;
        let name_raw = env.call_method(cls, "getName", "()Ljava/lang/String;", &[])?;
        let oh = name_raw.l()?.into();
        let what = env.get_string(&oh)?;
        let name = what.to_string_lossy();
        if !name.ends_with("Plugin") {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Plugin object, got {}",
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
    pub fn get_resource(
        &mut self,
        arg0: impl Into<String>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
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
    pub fn get_default_biome_provider(
        &mut self,
        arg0: impl Into<String>,
        arg1: impl Into<String>,
    ) -> Result<crate::bukkit::generator::BiomeProvider<'mc>, Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg1.into()).unwrap());
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
        arg0: impl Into<String>,
        arg1: bool,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        // -2
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
        // -2
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
        arg0: impl Into<String>,
        arg1: impl Into<String>,
    ) -> Result<crate::bukkit::generator::ChunkGenerator<'mc>, Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg1.into()).unwrap());
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
    pub fn on_command(
        &mut self,
        arg0: impl Into<crate::bukkit::command::CommandSender<'mc>>,
        arg1: impl Into<crate::bukkit::command::Command<'mc>>,
        arg2: impl Into<String>,
        _arg3: Vec<impl Into<String>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg1.into().1.clone()) };
        let val_2 = jni::objects::JObject::from(self.jni_ref().new_string(arg2.into()).unwrap());
        let res =
self.jni_ref().call_method(&self.jni_object(),"onCommand","(Lorg/bukkit/command/CommandSender;Lorg/bukkit/command/Command;Ljava/lang/String;Ljava/lang/String;)Z",&[jni::objects::JValueGen::from(&val_0),jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)])?;
        Ok(res.z().unwrap())
    }
}
impl<'mc> crate::JNIRaw<'mc> for Plugin<'mc> {
    fn jni_ref(&self) -> crate::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Into<crate::bukkit::command::TabExecutor<'mc>> for Plugin<'mc> {
    fn into(self) -> crate::bukkit::command::TabExecutor<'mc> {
        crate::bukkit::command::TabExecutor::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
pub mod java;
pub mod messaging;
