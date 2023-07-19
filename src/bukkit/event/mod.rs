use crate::JNIRaw;
pub struct HandlerList<'mc>(
    pub(crate) crate::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> crate::JNIRaw<'mc> for HandlerList<'mc> {
    fn jni_ref(&self) -> crate::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> HandlerList<'mc> {
    pub fn new(
        jni: crate::SharedJNIEnv<'mc>,
    ) -> Result<crate::bukkit::event::HandlerList<'mc>, Box<dyn std::error::Error>> {
        let cls = &jni.find_class("org/bukkit/event/HandlerList")?;
        let res = jni.new_object(cls, "()V", &[])?;
        let ret = { crate::bukkit::event::HandlerList(jni, res) };
        Ok(ret)
    }
    pub fn from_raw(
        env: &crate::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate HandlerList from null object.").into());
        }
        let cls = env.jni.borrow().get_object_class(&obj)?;
        let name_raw = env.call_method(cls, "getName", "()Ljava/lang/String;", &[])?;
        let oh = name_raw.l()?.into();
        let what = env.get_string(&oh)?;
        let name = what.to_string_lossy();
        if !name.ends_with("HandlerList") {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a HandlerList object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    pub fn register(
        &mut self,
        arg0: impl Into<crate::bukkit::plugin::RegisteredListener<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        self.jni_ref().call_method(
            &self.jni_object(),
            "register",
            "(Lorg/bukkit/plugin/RegisteredListener;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn bake_all(mut jni: crate::SharedJNIEnv<'mc>) -> Result<(), Box<dyn std::error::Error>> {
        let cls = &jni.find_class("void")?;
        let _res = jni.call_static_method(cls, "bakeAll", "()V", &[])?;
        Ok(())
    }
    pub fn bake(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.jni_ref()
            .call_method(&self.jni_object(), "bake", "()V", &[])?;
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
/// An instantiatable struct that implements Listener. Needed for returning it from Java.
pub struct Listener<'mc>(
    pub(crate) crate::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> Listener<'mc> {}
impl<'mc> crate::JNIRaw<'mc> for Listener<'mc> {
    fn jni_ref(&self) -> crate::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
/// An instantiatable struct that implements Cancellable. Needed for returning it from Java.
pub struct Cancellable<'mc>(
    pub(crate) crate::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> Cancellable<'mc> {
    pub fn from_raw(
        env: &crate::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Cancellable from null object.").into());
        }
        let cls = env.jni.borrow().get_object_class(&obj)?;
        let name_raw = env.call_method(cls, "getName", "()Ljava/lang/String;", &[])?;
        let oh = name_raw.l()?.into();
        let what = env.get_string(&oh)?;
        let name = what.to_string_lossy();
        if !name.ends_with("Cancellable") {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Cancellable object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    pub fn is_cancelled(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isCancelled", "()Z", &[])?;
        Ok(res.z().unwrap())
    }
    pub fn set_cancelled(&mut self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Bool(arg0.into());
        self.jni_ref().call_method(
            &self.jni_object(),
            "setCancelled",
            "(Z)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
}
impl<'mc> crate::JNIRaw<'mc> for Cancellable<'mc> {
    fn jni_ref(&self) -> crate::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
/// An instantiatable struct that implements EventHandler. Needed for returning it from Java.
pub struct EventHandler<'mc>(
    pub(crate) crate::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> EventHandler<'mc> {
    pub fn from_raw(
        env: &crate::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate EventHandler from null object.").into());
        }
        let cls = env.jni.borrow().get_object_class(&obj)?;
        let name_raw = env.call_method(cls, "getName", "()Ljava/lang/String;", &[])?;
        let oh = name_raw.l()?.into();
        let what = env.get_string(&oh)?;
        let name = what.to_string_lossy();
        if !name.ends_with("EventHandler") {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a EventHandler object, got {}",
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
            "priority",
            "()Lorg/bukkit/event/EventPriority;",
            &[],
        )?;
        let ret = {
            let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
            let variant =
                self.jni_ref()
                    .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
            let variant_str = self
                .jni_ref()
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
    pub fn ignore_cancelled(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "ignoreCancelled", "()Z", &[])?;
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
    pub fn annotation_type(
        &mut self,
    ) -> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "annotationType",
            "()Ljava/lang/Class;",
            &[],
        )?;
        Ok(unsafe { jni::objects::JClass::from_raw(res.as_jni().l) })
    }
}
impl<'mc> crate::JNIRaw<'mc> for EventHandler<'mc> {
    fn jni_ref(&self) -> crate::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
pub struct Event<'mc>(
    pub(crate) crate::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
pub struct EventResult<'mc>(
    pub(crate) crate::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> crate::JNIRaw<'mc> for EventResult<'mc> {
    fn jni_ref(&self) -> crate::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> EventResult<'mc> {
    pub fn from_raw(
        env: &crate::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate EventResult from null object.").into());
        }
        let cls = env.jni.borrow().get_object_class(&obj)?;
        let name_raw = env.call_method(cls, "getName", "()Ljava/lang/String;", &[])?;
        let oh = name_raw.l()?.into();
        let what = env.get_string(&oh)?;
        let name = what.to_string_lossy();
        if !name.ends_with("EventResult") {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a EventResult object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
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
impl<'mc> crate::JNIRaw<'mc> for Event<'mc> {
    fn jni_ref(&self) -> crate::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Event<'mc> {
    pub fn new(
        jni: crate::SharedJNIEnv<'mc>,
        arg0: std::option::Option<bool>,
    ) -> Result<crate::bukkit::event::Event<'mc>, Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Bool(arg0.unwrap().into());
        let cls = &jni.find_class("org/bukkit/event/Event")?;
        let res = jni.new_object(cls, "(Z)V", &[jni::objects::JValueGen::from(&val_0)])?;
        let ret = { crate::bukkit::event::Event(jni, res) };
        Ok(ret)
    }
    pub fn from_raw(
        env: &crate::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Event from null object.").into());
        }
        let cls = env.jni.borrow().get_object_class(&obj)?;
        let name_raw = env.call_method(cls, "getName", "()Ljava/lang/String;", &[])?;
        let oh = name_raw.l()?.into();
        let what = env.get_string(&oh)?;
        let name = what.to_string_lossy();
        if !name.ends_with("Event") {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Event object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    pub fn handlers(
        &mut self,
    ) -> Result<crate::bukkit::event::HandlerList<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getHandlers",
            "()Lorg/bukkit/event/HandlerList;",
            &[],
        )?;
        let ret = {
            crate::bukkit::event::HandlerList(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn event_name(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getEventName",
            "()Ljava/lang/String;",
            &[],
        )?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    pub fn is_asynchronous(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isAsynchronous", "()Z", &[])?;
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
pub enum EventPriorityEnum {
    Lowest,
    Low,
    Normal,
    High,
    Highest,
    Monitor,
}
impl std::fmt::Display for EventPriorityEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            EventPriorityEnum::Lowest => f.write_str("LOWEST"),
            EventPriorityEnum::Low => f.write_str("LOW"),
            EventPriorityEnum::Normal => f.write_str("NORMAL"),
            EventPriorityEnum::High => f.write_str("HIGH"),
            EventPriorityEnum::Highest => f.write_str("HIGHEST"),
            EventPriorityEnum::Monitor => f.write_str("MONITOR"),
        }
    }
}
pub struct EventPriority<'mc>(
    pub(crate) crate::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
    pub EventPriorityEnum,
);
impl<'mc> std::ops::Deref for EventPriority<'mc> {
    type Target = EventPriorityEnum;
    fn deref(&self) -> &Self::Target {
        return &self.2;
    }
}
impl<'mc> crate::JNIRaw<'mc> for EventPriority<'mc> {
    fn jni_ref(&self) -> crate::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> EventPriority<'mc> {
    pub const LOWEST: EventPriorityEnum = EventPriorityEnum::Lowest;
    pub const LOW: EventPriorityEnum = EventPriorityEnum::Low;
    pub const NORMAL: EventPriorityEnum = EventPriorityEnum::Normal;
    pub const HIGH: EventPriorityEnum = EventPriorityEnum::High;
    pub const HIGHEST: EventPriorityEnum = EventPriorityEnum::Highest;
    pub const MONITOR: EventPriorityEnum = EventPriorityEnum::Monitor;
    pub fn from_string(str: String) -> std::option::Option<EventPriorityEnum> {
        match str.as_str() {
            "LOWEST" => Some(EventPriorityEnum::Lowest),
            "LOW" => Some(EventPriorityEnum::Low),
            "NORMAL" => Some(EventPriorityEnum::Normal),
            "HIGH" => Some(EventPriorityEnum::High),
            "HIGHEST" => Some(EventPriorityEnum::Highest),
            "MONITOR" => Some(EventPriorityEnum::Monitor),
            _ => None,
        }
    }
    pub fn value_of(
        mut jni: crate::SharedJNIEnv<'mc>,
        arg0: String,
    ) -> Result<crate::bukkit::event::EventPriority<'mc>, Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JObject::from(jni.new_string(arg0).unwrap());
        let cls = &jni.find_class("org/bukkit/event/EventPriority")?;
        let res = jni.call_static_method(
            cls,
            "valueOf",
            "(Ljava/lang/String;)Lorg/bukkit/event/EventPriority;",
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
            crate::bukkit::event::EventPriority(
                jni,
                raw_obj,
                crate::bukkit::event::EventPriority::from_string(variant_str).unwrap(),
            )
        };
        Ok(ret)
    }
    pub fn slot(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getSlot", "()I", &[])?;
        Ok(res.i().unwrap())
    }
}
pub mod block;
pub mod enchantment;
pub mod entity;
pub mod hanging;
pub mod inventory;
pub mod player;
pub mod raid;
pub mod server;
pub mod vehicle;
pub mod weather;
pub mod world;
