#![allow(deprecated)]
use blackboxmc_general::JNIRaw;
use color_eyre::eyre::Result;
/// Called when a World is unloaded
pub struct WorldUnloadEvent<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for WorldUnloadEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> WorldUnloadEvent<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate WorldUnloadEvent from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/event/world/WorldUnloadEvent")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a WorldUnloadEvent object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<crate::World<'mc>>,
    ) -> Result<crate::event::world::WorldUnloadEvent<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/World;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let cls = jni.find_class("org/bukkit/event/world/WorldUnloadEvent");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(
            cls,
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = jni.translate_error_no_gen(res)?;
        crate::event::world::WorldUnloadEvent::from_raw(&jni, res)
    }
    //

    pub fn is_cancelled(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isCancelled", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn handlers(&self) -> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/event/HandlerList;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getHandlers", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::event::HandlerList::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    /// <span class="descfrm-type-label">Description copied from interface:&nbsp;<code><a href="../Cancellable.html#setCancelled(boolean)">Cancellable</a></code></span>
    /// Sets the cancellation state of this event. A cancelled event will not be executed in the server, but will still pass to other plugins.
    pub fn set_cancelled(&self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        // -2
        let val_1 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setCancelled",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn handler_list(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/event/HandlerList;");
        let cls = jni.find_class("org/bukkit/event/HandlerList");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "getHandlerList", sig.as_str(), vec![]);
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        crate::event::HandlerList::from_raw(&jni, obj)
    }
    //

    pub fn world(&self) -> Result<crate::World<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/World;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getWorld", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::World::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn event_name(&self) -> Result<String, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/String;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getEventName", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    //

    pub fn is_asynchronous(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isAsynchronous", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn wait(
        &self,
        arg0: std::option::Option<i64>,
        arg1: std::option::Option<i32>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "J";
            let val_1 = jni::objects::JValueGen::Long(a.into());
            args.push(val_1);
        }
        if let Some(a) = arg1 {
            sig += "I";
            let val_2 = jni::objects::JValueGen::Int(a.into());
            args.push(val_2);
        }
        sig += ")V";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "wait", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn equals(
        &self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/Object;)Z");
        let val_1 = jni::objects::JValueGen::Object(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "equals",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

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
    //

    pub fn hash_code(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hashCode", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    //

    pub fn class(&self) -> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/Class;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getClass", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(unsafe { jni::objects::JClass::from_raw(res.as_jni().l) })
    }
    //

    pub fn notify(&self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notify", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn notify_all(&self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notifyAll", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
}

impl<'mc> std::string::ToString for WorldUnloadEvent<'mc> {
    fn to_string(&self) -> String {
        match &self.internal_to_string() {
            Ok(a) => a.clone(),
            Err(err) => format!("Error calling WorldUnloadEvent.toString: {}", err),
        }
    }
}

impl<'mc> Into<crate::event::Cancellable<'mc>> for WorldUnloadEvent<'mc> {
    fn into(self) -> crate::event::Cancellable<'mc> {
        crate::event::Cancellable::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting WorldUnloadEvent into crate::event::Cancellable")
    }
}
impl<'mc> Into<crate::event::world::WorldEvent<'mc>> for WorldUnloadEvent<'mc> {
    fn into(self) -> crate::event::world::WorldEvent<'mc> {
        crate::event::world::WorldEvent::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting WorldUnloadEvent into crate::event::world::WorldEvent")
    }
}
/// An event that is called when a world's spawn changes. The world's previous spawn location is included.
pub struct SpawnChangeEvent<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for SpawnChangeEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> SpawnChangeEvent<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate SpawnChangeEvent from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/event/world/SpawnChangeEvent")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a SpawnChangeEvent object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<crate::World<'mc>>,
        arg1: impl Into<crate::Location<'mc>>,
    ) -> Result<crate::event::world::SpawnChangeEvent<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/World;Lorg/bukkit/Location;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let val_2 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg1.into().jni_object().clone())
        });
        let cls = jni.find_class("org/bukkit/event/world/SpawnChangeEvent");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(
            cls,
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
            ],
        );
        let res = jni.translate_error_no_gen(res)?;
        crate::event::world::SpawnChangeEvent::from_raw(&jni, res)
    }
    //

    pub fn handlers(&self) -> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/event/HandlerList;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getHandlers", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::event::HandlerList::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn handler_list(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/event/HandlerList;");
        let cls = jni.find_class("org/bukkit/event/HandlerList");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "getHandlerList", sig.as_str(), vec![]);
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        crate::event::HandlerList::from_raw(&jni, obj)
    }
    //

    pub fn previous_location(&self) -> Result<crate::Location<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/Location;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getPreviousLocation",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::Location::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn world(&self) -> Result<crate::World<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/World;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getWorld", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::World::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn event_name(&self) -> Result<String, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/String;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getEventName", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    //

    pub fn is_asynchronous(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isAsynchronous", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn wait(
        &self,
        arg0: std::option::Option<i64>,
        arg1: std::option::Option<i32>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "J";
            let val_1 = jni::objects::JValueGen::Long(a.into());
            args.push(val_1);
        }
        if let Some(a) = arg1 {
            sig += "I";
            let val_2 = jni::objects::JValueGen::Int(a.into());
            args.push(val_2);
        }
        sig += ")V";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "wait", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn equals(
        &self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/Object;)Z");
        let val_1 = jni::objects::JValueGen::Object(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "equals",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

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
    //

    pub fn hash_code(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hashCode", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    //

    pub fn class(&self) -> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/Class;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getClass", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(unsafe { jni::objects::JClass::from_raw(res.as_jni().l) })
    }
    //

    pub fn notify(&self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notify", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn notify_all(&self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notifyAll", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
}

impl<'mc> std::string::ToString for SpawnChangeEvent<'mc> {
    fn to_string(&self) -> String {
        match &self.internal_to_string() {
            Ok(a) => a.clone(),
            Err(err) => format!("Error calling SpawnChangeEvent.toString: {}", err),
        }
    }
}

impl<'mc> Into<crate::event::world::WorldEvent<'mc>> for SpawnChangeEvent<'mc> {
    fn into(self) -> crate::event::world::WorldEvent<'mc> {
        crate::event::world::WorldEvent::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting SpawnChangeEvent into crate::event::world::WorldEvent")
    }
}
/// Called when a chunk is loaded
pub struct ChunkLoadEvent<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for ChunkLoadEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> ChunkLoadEvent<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate ChunkLoadEvent from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/event/world/ChunkLoadEvent")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a ChunkLoadEvent object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<crate::Chunk<'mc>>,
        arg1: bool,
    ) -> Result<crate::event::world::ChunkLoadEvent<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/Chunk;Z)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        // -2
        let val_2 = jni::objects::JValueGen::Bool(arg1.into());
        let cls = jni.find_class("org/bukkit/event/world/ChunkLoadEvent");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(
            cls,
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
            ],
        );
        let res = jni.translate_error_no_gen(res)?;
        crate::event::world::ChunkLoadEvent::from_raw(&jni, res)
    }
    //

    pub fn handlers(&self) -> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/event/HandlerList;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getHandlers", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::event::HandlerList::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn handler_list(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/event/HandlerList;");
        let cls = jni.find_class("org/bukkit/event/HandlerList");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "getHandlerList", sig.as_str(), vec![]);
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        crate::event::HandlerList::from_raw(&jni, obj)
    }
    //

    pub fn is_new_chunk(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isNewChunk", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn chunk(&self) -> Result<crate::Chunk<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/Chunk;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getChunk", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::Chunk::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn world(&self) -> Result<crate::World<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/World;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getWorld", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::World::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn event_name(&self) -> Result<String, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/String;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getEventName", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    //

    pub fn is_asynchronous(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isAsynchronous", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn wait(
        &self,
        arg0: std::option::Option<i64>,
        arg1: std::option::Option<i32>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "J";
            let val_1 = jni::objects::JValueGen::Long(a.into());
            args.push(val_1);
        }
        if let Some(a) = arg1 {
            sig += "I";
            let val_2 = jni::objects::JValueGen::Int(a.into());
            args.push(val_2);
        }
        sig += ")V";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "wait", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn equals(
        &self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/Object;)Z");
        let val_1 = jni::objects::JValueGen::Object(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "equals",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

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
    //

    pub fn hash_code(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hashCode", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    //

    pub fn class(&self) -> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/Class;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getClass", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(unsafe { jni::objects::JClass::from_raw(res.as_jni().l) })
    }
    //

    pub fn notify(&self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notify", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn notify_all(&self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notifyAll", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
}

impl<'mc> std::string::ToString for ChunkLoadEvent<'mc> {
    fn to_string(&self) -> String {
        match &self.internal_to_string() {
            Ok(a) => a.clone(),
            Err(err) => format!("Error calling ChunkLoadEvent.toString: {}", err),
        }
    }
}

impl<'mc> Into<crate::event::world::ChunkEvent<'mc>> for ChunkLoadEvent<'mc> {
    fn into(self) -> crate::event::world::ChunkEvent<'mc> {
        crate::event::world::ChunkEvent::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting ChunkLoadEvent into crate::event::world::ChunkEvent")
    }
}
/// Called when a portal is created
pub struct PortalCreateEvent<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
#[derive(PartialEq, Eq)]
pub enum PortalCreateEventCreateReasonEnum {
    Fire,
    NetherPair,
    EndPlatform,
}
impl std::fmt::Display for PortalCreateEventCreateReasonEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PortalCreateEventCreateReasonEnum::Fire => f.write_str("FIRE"),
            PortalCreateEventCreateReasonEnum::NetherPair => f.write_str("NETHER_PAIR"),
            PortalCreateEventCreateReasonEnum::EndPlatform => f.write_str("END_PLATFORM"),
        }
    }
}
impl<'mc> std::fmt::Display for PortalCreateEventCreateReason<'mc> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.2.fmt(f)
    }
}
pub struct PortalCreateEventCreateReason<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
    pub PortalCreateEventCreateReasonEnum,
);
impl<'mc> std::ops::Deref for PortalCreateEventCreateReason<'mc> {
    type Target = PortalCreateEventCreateReasonEnum;
    fn deref(&self) -> &Self::Target {
        return &self.2;
    }
}
impl<'mc> JNIRaw<'mc> for PortalCreateEventCreateReason<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> PortalCreateEventCreateReason<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
        e: PortalCreateEventCreateReasonEnum,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate PortalCreateEventCreateReason from null object."
            )
            .into());
        }
        let (valid, name) = env.validate_name(
            &obj,
            "org/bukkit/event/world/PortalCreateEvent$CreateReason",
        )?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a PortalCreateEventCreateReason object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj, e))
        }
    }
    pub const FIRE: PortalCreateEventCreateReasonEnum = PortalCreateEventCreateReasonEnum::Fire;
    pub const NETHER_PAIR: PortalCreateEventCreateReasonEnum =
        PortalCreateEventCreateReasonEnum::NetherPair;
    pub const END_PLATFORM: PortalCreateEventCreateReasonEnum =
        PortalCreateEventCreateReasonEnum::EndPlatform;
    pub fn from_string(str: String) -> std::option::Option<PortalCreateEventCreateReasonEnum> {
        match str.as_str() {
            "FIRE" => Some(PortalCreateEventCreateReasonEnum::Fire),
            "NETHER_PAIR" => Some(PortalCreateEventCreateReasonEnum::NetherPair),
            "END_PLATFORM" => Some(PortalCreateEventCreateReasonEnum::EndPlatform),
            _ => None,
        }
    }

    pub fn value_of(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<String>,
    ) -> Result<PortalCreateEventCreateReason<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(jni.new_string(arg0.into())?);
        let cls = jni.find_class("org/bukkit/event/world/PortalCreateEvent$CreateReason");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(
            cls,
            "valueOf",
            "(Ljava/lang/String;)Lorg/bukkit/event/world/PortalCreateEvent$CreateReason;",
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        let raw_obj = obj;
        let variant = jni.call_method(&raw_obj, "toString", "()Ljava/lang/String;", vec![]);
        let variant = jni.translate_error(variant)?;
        let variant_str = jni
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        PortalCreateEventCreateReason::from_raw(
            &jni,
            raw_obj,
            PortalCreateEventCreateReason::from_string(variant_str)
                .ok_or(eyre::eyre!("String gaven for variant was invalid"))?,
        )
    }

    //
}
impl<'mc> blackboxmc_general::JNIRaw<'mc> for PortalCreateEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> PortalCreateEvent<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate PortalCreateEvent from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/event/world/PortalCreateEvent")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a PortalCreateEvent object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    pub fn new_with_list(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: Vec<impl Into<crate::block::BlockState<'mc>>>,
        arg1: impl Into<crate::World<'mc>>,
        arg2: std::option::Option<impl Into<crate::entity::Entity<'mc>>>,
        arg3: std::option::Option<
            impl Into<crate::event::world::PortalCreateEventCreateReason<'mc>>,
        >,
    ) -> Result<crate::event::world::PortalCreateEvent<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Ljava/util/List;";
        let raw_val_1 = jni.new_object("java/util/ArrayList", "()V", vec![])?;
        for v in arg0 {
            let map_val_0 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(v.into().jni_object().clone())
            });
            jni.call_method(
                &raw_val_1,
                "add",
                "(Ljava/Lang/Object)V",
                vec![jni::objects::JValueGen::from(map_val_0)],
            )?;
        }
        let val_1 = jni::objects::JValueGen::Object(raw_val_1);
        args.push(val_1);
        sig += "Lorg/bukkit/World;";
        let val_2 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg1.into().jni_object().clone())
        });
        args.push(val_2);
        if let Some(a) = arg2 {
            sig += "Lorg/bukkit/entity/Entity;";
            let val_3 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(a.into().jni_object().clone())
            });
            args.push(val_3);
        }
        if let Some(a) = arg3 {
            sig += "Lorg/bukkit/event/world/PortalCreateEvent$CreateReason;";
            let val_4 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(a.into().jni_object().clone())
            });
            args.push(val_4);
        }
        sig += ")V";
        let cls = jni.find_class("org/bukkit/event/world/PortalCreateEvent");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(cls, sig.as_str(), args);
        let res = jni.translate_error_no_gen(res)?;
        crate::event::world::PortalCreateEvent::from_raw(&jni, res)
    }
    //

    pub fn reason(
        &self,
    ) -> Result<crate::event::world::PortalCreateEventCreateReason<'mc>, Box<dyn std::error::Error>>
    {
        let sig = String::from("()Lorg/bukkit/event/world/PortalCreateEvent$CreateReason;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getReason", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant = self
            .0
            .call_method(&raw_obj, "toString", "()Ljava/lang/String;", vec![]);
        let variant = self.jni_ref().translate_error(variant)?;
        let variant_str = self
            .0
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::event::world::PortalCreateEventCreateReason::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::event::world::PortalCreateEventCreateReason::from_string(variant_str)
                .ok_or(eyre::eyre!("String gaven for variant was invalid"))?,
        )
    }
    //

    pub fn is_cancelled(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isCancelled", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn handlers(&self) -> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/event/HandlerList;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getHandlers", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::event::HandlerList::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn entity(&self) -> Result<crate::entity::Entity<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/entity/Entity;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getEntity", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::entity::Entity::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn blocks(&self) -> Result<Vec<crate::block::BlockState<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/List;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getBlocks", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let mut list = blackboxmc_java::JavaList::from_raw(&self.0, res.l()?)?;
        let size = list.size()?;
        for i in 0..=size {
            let obj = list.get(i)?;
            new_vec.push(crate::block::BlockState::from_raw(&self.0, obj)?);
        }
        Ok(new_vec)
    }
    //

    /// <span class="descfrm-type-label">Description copied from interface:&nbsp;<code><a href="../Cancellable.html#setCancelled(boolean)">Cancellable</a></code></span>
    /// Sets the cancellation state of this event. A cancelled event will not be executed in the server, but will still pass to other plugins.
    pub fn set_cancelled(&self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        // -2
        let val_1 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setCancelled",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn handler_list(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/event/HandlerList;");
        let cls = jni.find_class("org/bukkit/event/HandlerList");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "getHandlerList", sig.as_str(), vec![]);
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        crate::event::HandlerList::from_raw(&jni, obj)
    }
    //

    pub fn world(&self) -> Result<crate::World<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/World;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getWorld", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::World::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn event_name(&self) -> Result<String, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/String;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getEventName", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    //

    pub fn is_asynchronous(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isAsynchronous", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn wait(
        &self,
        arg0: std::option::Option<i64>,
        arg1: std::option::Option<i32>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "J";
            let val_1 = jni::objects::JValueGen::Long(a.into());
            args.push(val_1);
        }
        if let Some(a) = arg1 {
            sig += "I";
            let val_2 = jni::objects::JValueGen::Int(a.into());
            args.push(val_2);
        }
        sig += ")V";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "wait", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn equals(
        &self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/Object;)Z");
        let val_1 = jni::objects::JValueGen::Object(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "equals",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

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
    //

    pub fn hash_code(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hashCode", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    //

    pub fn class(&self) -> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/Class;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getClass", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(unsafe { jni::objects::JClass::from_raw(res.as_jni().l) })
    }
    //

    pub fn notify(&self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notify", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn notify_all(&self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notifyAll", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
}

impl<'mc> std::string::ToString for PortalCreateEvent<'mc> {
    fn to_string(&self) -> String {
        match &self.internal_to_string() {
            Ok(a) => a.clone(),
            Err(err) => format!("Error calling PortalCreateEvent.toString: {}", err),
        }
    }
}

impl<'mc> Into<crate::event::Cancellable<'mc>> for PortalCreateEvent<'mc> {
    fn into(self) -> crate::event::Cancellable<'mc> {
        crate::event::Cancellable::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting PortalCreateEvent into crate::event::Cancellable")
    }
}
impl<'mc> Into<crate::event::world::WorldEvent<'mc>> for PortalCreateEvent<'mc> {
    fn into(self) -> crate::event::world::WorldEvent<'mc> {
        crate::event::world::WorldEvent::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting PortalCreateEvent into crate::event::world::WorldEvent")
    }
}
#[derive(PartialEq, Eq)]
pub enum CreateReasonEnum {
    Fire,
    NetherPair,
    EndPlatform,
}
impl std::fmt::Display for CreateReasonEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CreateReasonEnum::Fire => f.write_str("FIRE"),
            CreateReasonEnum::NetherPair => f.write_str("NETHER_PAIR"),
            CreateReasonEnum::EndPlatform => f.write_str("END_PLATFORM"),
        }
    }
}
impl<'mc> std::fmt::Display for CreateReason<'mc> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.2.fmt(f)
    }
}
pub struct CreateReason<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
    pub CreateReasonEnum,
);
impl<'mc> std::ops::Deref for CreateReason<'mc> {
    type Target = CreateReasonEnum;
    fn deref(&self) -> &Self::Target {
        return &self.2;
    }
}
impl<'mc> JNIRaw<'mc> for CreateReason<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> CreateReason<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
        e: CreateReasonEnum,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate CreateReason from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/event/world/CreateReason")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a CreateReason object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj, e))
        }
    }
    pub const FIRE: CreateReasonEnum = CreateReasonEnum::Fire;
    pub const NETHER_PAIR: CreateReasonEnum = CreateReasonEnum::NetherPair;
    pub const END_PLATFORM: CreateReasonEnum = CreateReasonEnum::EndPlatform;
    pub fn from_string(str: String) -> std::option::Option<CreateReasonEnum> {
        match str.as_str() {
            "FIRE" => Some(CreateReasonEnum::Fire),
            "NETHER_PAIR" => Some(CreateReasonEnum::NetherPair),
            "END_PLATFORM" => Some(CreateReasonEnum::EndPlatform),
            _ => None,
        }
    }

    pub fn value_of(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<String>,
    ) -> Result<CreateReason<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(jni.new_string(arg0.into())?);
        let cls = jni.find_class("org/bukkit/event/world/CreateReason");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(
            cls,
            "valueOf",
            "(Ljava/lang/String;)Lorg/bukkit/event/world/CreateReason;",
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        let raw_obj = obj;
        let variant = jni.call_method(&raw_obj, "toString", "()Ljava/lang/String;", vec![]);
        let variant = jni.translate_error(variant)?;
        let variant_str = jni
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        CreateReason::from_raw(
            &jni,
            raw_obj,
            CreateReason::from_string(variant_str)
                .ok_or(eyre::eyre!("String gaven for variant was invalid"))?,
        )
    }
}
/// Called when a World is initializing.
/// <p>To get every world it is recommended to add following to the plugin.yml.</p>
/// <pre>load: STARTUP</pre>
pub struct WorldInitEvent<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for WorldInitEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> WorldInitEvent<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate WorldInitEvent from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/event/world/WorldInitEvent")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a WorldInitEvent object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<crate::World<'mc>>,
    ) -> Result<crate::event::world::WorldInitEvent<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/World;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let cls = jni.find_class("org/bukkit/event/world/WorldInitEvent");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(
            cls,
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = jni.translate_error_no_gen(res)?;
        crate::event::world::WorldInitEvent::from_raw(&jni, res)
    }
    //

    pub fn handlers(&self) -> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/event/HandlerList;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getHandlers", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::event::HandlerList::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn handler_list(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/event/HandlerList;");
        let cls = jni.find_class("org/bukkit/event/HandlerList");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "getHandlerList", sig.as_str(), vec![]);
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        crate::event::HandlerList::from_raw(&jni, obj)
    }
    //

    pub fn world(&self) -> Result<crate::World<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/World;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getWorld", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::World::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn event_name(&self) -> Result<String, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/String;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getEventName", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    //

    pub fn is_asynchronous(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isAsynchronous", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn wait(
        &self,
        arg0: std::option::Option<i64>,
        arg1: std::option::Option<i32>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "J";
            let val_1 = jni::objects::JValueGen::Long(a.into());
            args.push(val_1);
        }
        if let Some(a) = arg1 {
            sig += "I";
            let val_2 = jni::objects::JValueGen::Int(a.into());
            args.push(val_2);
        }
        sig += ")V";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "wait", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn equals(
        &self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/Object;)Z");
        let val_1 = jni::objects::JValueGen::Object(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "equals",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

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
    //

    pub fn hash_code(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hashCode", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    //

    pub fn class(&self) -> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/Class;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getClass", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(unsafe { jni::objects::JClass::from_raw(res.as_jni().l) })
    }
    //

    pub fn notify(&self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notify", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn notify_all(&self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notifyAll", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
}

impl<'mc> std::string::ToString for WorldInitEvent<'mc> {
    fn to_string(&self) -> String {
        match &self.internal_to_string() {
            Ok(a) => a.clone(),
            Err(err) => format!("Error calling WorldInitEvent.toString: {}", err),
        }
    }
}

impl<'mc> Into<crate::event::world::WorldEvent<'mc>> for WorldInitEvent<'mc> {
    fn into(self) -> crate::event::world::WorldEvent<'mc> {
        crate::event::world::WorldEvent::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting WorldInitEvent into crate::event::world::WorldEvent")
    }
}
#[derive(PartialEq, Eq)]
pub enum SkipReasonEnum {
    Command,
    Custom,
    NightSkip,
}
impl std::fmt::Display for SkipReasonEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SkipReasonEnum::Command => f.write_str("COMMAND"),
            SkipReasonEnum::Custom => f.write_str("CUSTOM"),
            SkipReasonEnum::NightSkip => f.write_str("NIGHT_SKIP"),
        }
    }
}
impl<'mc> std::fmt::Display for SkipReason<'mc> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.2.fmt(f)
    }
}
pub struct SkipReason<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
    pub SkipReasonEnum,
);
impl<'mc> std::ops::Deref for SkipReason<'mc> {
    type Target = SkipReasonEnum;
    fn deref(&self) -> &Self::Target {
        return &self.2;
    }
}
impl<'mc> JNIRaw<'mc> for SkipReason<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> SkipReason<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
        e: SkipReasonEnum,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate SkipReason from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/event/world/SkipReason")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a SkipReason object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj, e))
        }
    }
    pub const COMMAND: SkipReasonEnum = SkipReasonEnum::Command;
    pub const CUSTOM: SkipReasonEnum = SkipReasonEnum::Custom;
    pub const NIGHT_SKIP: SkipReasonEnum = SkipReasonEnum::NightSkip;
    pub fn from_string(str: String) -> std::option::Option<SkipReasonEnum> {
        match str.as_str() {
            "COMMAND" => Some(SkipReasonEnum::Command),
            "CUSTOM" => Some(SkipReasonEnum::Custom),
            "NIGHT_SKIP" => Some(SkipReasonEnum::NightSkip),
            _ => None,
        }
    }

    pub fn value_of(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<String>,
    ) -> Result<SkipReason<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(jni.new_string(arg0.into())?);
        let cls = jni.find_class("org/bukkit/event/world/SkipReason");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(
            cls,
            "valueOf",
            "(Ljava/lang/String;)Lorg/bukkit/event/world/SkipReason;",
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        let raw_obj = obj;
        let variant = jni.call_method(&raw_obj, "toString", "()Ljava/lang/String;", vec![]);
        let variant = jni.translate_error(variant)?;
        let variant_str = jni
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        SkipReason::from_raw(
            &jni,
            raw_obj,
            SkipReason::from_string(variant_str)
                .ok_or(eyre::eyre!("String gaven for variant was invalid"))?,
        )
    }
}
/// Called when a <a title="interface in org.bukkit.loot" href="../../loot/LootTable.html"><code>LootTable</code></a> is generated in the world for an <a title="interface in org.bukkit.inventory" href="../../inventory/InventoryHolder.html"><code>InventoryHolder</code></a>. This event is NOT currently called when an entity's loot table has been generated (use <a href="../entity/EntityDeathEvent.html#getDrops()"><code>EntityDeathEvent.getDrops()</code></a>, but WILL be called by plugins invoking <a href="../../loot/LootTable.html#fillInventory(org.bukkit.inventory.Inventory,java.util.Random,org.bukkit.loot.LootContext)"><code>LootTable.fillInventory(org.bukkit.inventory.Inventory, java.util.Random, LootContext)</code></a>.
pub struct LootGenerateEvent<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for LootGenerateEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> LootGenerateEvent<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate LootGenerateEvent from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/event/world/LootGenerateEvent")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a LootGenerateEvent object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<crate::World<'mc>>,
        arg1: impl Into<crate::entity::Entity<'mc>>,
        arg2: impl Into<crate::inventory::InventoryHolder<'mc>>,
        arg3: impl Into<crate::loot::LootTable<'mc>>,
        arg4: impl Into<crate::loot::LootContext<'mc>>,
        arg5: Vec<impl Into<crate::inventory::ItemStack<'mc>>>,
        arg6: bool,
    ) -> Result<crate::event::world::LootGenerateEvent<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/World;Lorg/bukkit/entity/Entity;Lorg/bukkit/inventory/InventoryHolder;Lorg/bukkit/loot/LootTable;Lorg/bukkit/loot/LootContext;Ljava/util/List;Z)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let val_2 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg1.into().jni_object().clone())
        });
        let val_3 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg2.into().jni_object().clone())
        });
        let val_4 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg3.into().jni_object().clone())
        });
        let val_5 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg4.into().jni_object().clone())
        });
        let raw_val_6 = jni.new_object("java/util/ArrayList", "()V", vec![])?;
        for v in arg5 {
            let map_val_0 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(v.into().jni_object().clone())
            });
            jni.call_method(
                &raw_val_6,
                "add",
                "(Ljava/Lang/Object)V",
                vec![jni::objects::JValueGen::from(map_val_0)],
            )?;
        }
        let val_6 = jni::objects::JValueGen::Object(raw_val_6);
        // -2
        let val_7 = jni::objects::JValueGen::Bool(arg6.into());
        let cls = jni.find_class("org/bukkit/event/world/LootGenerateEvent");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(
            cls,
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
                jni::objects::JValueGen::from(val_3),
                jni::objects::JValueGen::from(val_4),
                jni::objects::JValueGen::from(val_5),
                jni::objects::JValueGen::from(val_6),
                jni::objects::JValueGen::from(val_7),
            ],
        );
        let res = jni.translate_error_no_gen(res)?;
        crate::event::world::LootGenerateEvent::from_raw(&jni, res)
    }
    //

    pub fn is_cancelled(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isCancelled", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn handlers(&self) -> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/event/HandlerList;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getHandlers", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::event::HandlerList::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn loot_table(&self) -> Result<crate::loot::LootTable<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/loot/LootTable;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getLootTable", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::loot::LootTable::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn entity(&self) -> Result<crate::entity::Entity<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/entity/Entity;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getEntity", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::entity::Entity::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    /// <span class="descfrm-type-label">Description copied from interface:&nbsp;<code><a href="../Cancellable.html#setCancelled(boolean)">Cancellable</a></code></span>
    /// Sets the cancellation state of this event. A cancelled event will not be executed in the server, but will still pass to other plugins.
    pub fn set_cancelled(&self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        // -2
        let val_1 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setCancelled",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn handler_list(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/event/HandlerList;");
        let cls = jni.find_class("org/bukkit/event/HandlerList");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "getHandlerList", sig.as_str(), vec![]);
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        crate::event::HandlerList::from_raw(&jni, obj)
    }
    //

    pub fn inventory_holder(
        &self,
    ) -> Result<crate::inventory::InventoryHolder<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/inventory/InventoryHolder;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getInventoryHolder",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::InventoryHolder::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn loot_context(
        &self,
    ) -> Result<crate::loot::LootContext<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/loot/LootContext;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getLootContext", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::loot::LootContext::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn loot(
        &self,
    ) -> Result<Vec<crate::inventory::ItemStack<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/List;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getLoot", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let mut list = blackboxmc_java::JavaList::from_raw(&self.0, res.l()?)?;
        let size = list.size()?;
        for i in 0..=size {
            let obj = list.get(i)?;
            new_vec.push(crate::inventory::ItemStack::from_raw(&self.0, obj)?);
        }
        Ok(new_vec)
    }
    //

    pub fn is_plugin(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isPlugin", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn world(&self) -> Result<crate::World<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/World;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getWorld", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::World::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn event_name(&self) -> Result<String, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/String;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getEventName", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    //

    pub fn is_asynchronous(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isAsynchronous", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn wait(
        &self,
        arg0: std::option::Option<i64>,
        arg1: std::option::Option<i32>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "J";
            let val_1 = jni::objects::JValueGen::Long(a.into());
            args.push(val_1);
        }
        if let Some(a) = arg1 {
            sig += "I";
            let val_2 = jni::objects::JValueGen::Int(a.into());
            args.push(val_2);
        }
        sig += ")V";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "wait", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn equals(
        &self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/Object;)Z");
        let val_1 = jni::objects::JValueGen::Object(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "equals",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

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
    //

    pub fn hash_code(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hashCode", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    //

    pub fn class(&self) -> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/Class;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getClass", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(unsafe { jni::objects::JClass::from_raw(res.as_jni().l) })
    }
    //

    pub fn notify(&self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notify", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn notify_all(&self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notifyAll", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
}

impl<'mc> std::string::ToString for LootGenerateEvent<'mc> {
    fn to_string(&self) -> String {
        match &self.internal_to_string() {
            Ok(a) => a.clone(),
            Err(err) => format!("Error calling LootGenerateEvent.toString: {}", err),
        }
    }
}

impl<'mc> Into<crate::event::Cancellable<'mc>> for LootGenerateEvent<'mc> {
    fn into(self) -> crate::event::Cancellable<'mc> {
        crate::event::Cancellable::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting LootGenerateEvent into crate::event::Cancellable")
    }
}
impl<'mc> Into<crate::event::world::WorldEvent<'mc>> for LootGenerateEvent<'mc> {
    fn into(self) -> crate::event::world::WorldEvent<'mc> {
        crate::event::world::WorldEvent::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting LootGenerateEvent into crate::event::world::WorldEvent")
    }
}
/// Called when a World is loaded
pub struct WorldLoadEvent<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for WorldLoadEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> WorldLoadEvent<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate WorldLoadEvent from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/event/world/WorldLoadEvent")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a WorldLoadEvent object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<crate::World<'mc>>,
    ) -> Result<crate::event::world::WorldLoadEvent<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/World;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let cls = jni.find_class("org/bukkit/event/world/WorldLoadEvent");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(
            cls,
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = jni.translate_error_no_gen(res)?;
        crate::event::world::WorldLoadEvent::from_raw(&jni, res)
    }
    //

    pub fn handlers(&self) -> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/event/HandlerList;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getHandlers", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::event::HandlerList::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn handler_list(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/event/HandlerList;");
        let cls = jni.find_class("org/bukkit/event/HandlerList");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "getHandlerList", sig.as_str(), vec![]);
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        crate::event::HandlerList::from_raw(&jni, obj)
    }
    //

    pub fn world(&self) -> Result<crate::World<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/World;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getWorld", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::World::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn event_name(&self) -> Result<String, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/String;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getEventName", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    //

    pub fn is_asynchronous(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isAsynchronous", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn wait(
        &self,
        arg0: std::option::Option<i64>,
        arg1: std::option::Option<i32>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "J";
            let val_1 = jni::objects::JValueGen::Long(a.into());
            args.push(val_1);
        }
        if let Some(a) = arg1 {
            sig += "I";
            let val_2 = jni::objects::JValueGen::Int(a.into());
            args.push(val_2);
        }
        sig += ")V";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "wait", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn equals(
        &self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/Object;)Z");
        let val_1 = jni::objects::JValueGen::Object(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "equals",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

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
    //

    pub fn hash_code(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hashCode", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    //

    pub fn class(&self) -> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/Class;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getClass", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(unsafe { jni::objects::JClass::from_raw(res.as_jni().l) })
    }
    //

    pub fn notify(&self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notify", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn notify_all(&self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notifyAll", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
}

impl<'mc> std::string::ToString for WorldLoadEvent<'mc> {
    fn to_string(&self) -> String {
        match &self.internal_to_string() {
            Ok(a) => a.clone(),
            Err(err) => format!("Error calling WorldLoadEvent.toString: {}", err),
        }
    }
}

impl<'mc> Into<crate::event::world::WorldEvent<'mc>> for WorldLoadEvent<'mc> {
    fn into(self) -> crate::event::world::WorldEvent<'mc> {
        crate::event::world::WorldEvent::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting WorldLoadEvent into crate::event::world::WorldEvent")
    }
}
/// Called when a <a href="../../generator/structure/Structure.html" title="class in org.bukkit.generator.structure"><code>Structure</code></a> is naturally generated in the world.
pub struct AsyncStructureSpawnEvent<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for AsyncStructureSpawnEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> AsyncStructureSpawnEvent<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate AsyncStructureSpawnEvent from null object."
            )
            .into());
        }
        let (valid, name) =
            env.validate_name(&obj, "org/bukkit/event/world/AsyncStructureSpawnEvent")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a AsyncStructureSpawnEvent object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<crate::World<'mc>>,
        arg1: impl Into<crate::generator::structure::Structure<'mc>>,
        arg2: impl Into<crate::util::BoundingBox<'mc>>,
        arg3: i32,
        arg4: i32,
    ) -> Result<crate::event::world::AsyncStructureSpawnEvent<'mc>, Box<dyn std::error::Error>>
    {
        let sig = String::from("(Lorg/bukkit/World;Lorg/bukkit/generator/structure/Structure;Lorg/bukkit/util/BoundingBox;II)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let val_2 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg1.into().jni_object().clone())
        });
        let val_3 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg2.into().jni_object().clone())
        });
        let val_4 = jni::objects::JValueGen::Int(arg3.into());
        let val_5 = jni::objects::JValueGen::Int(arg4.into());
        let cls = jni.find_class("org/bukkit/event/world/AsyncStructureSpawnEvent");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(
            cls,
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
                jni::objects::JValueGen::from(val_3),
                jni::objects::JValueGen::from(val_4),
                jni::objects::JValueGen::from(val_5),
            ],
        );
        let res = jni.translate_error_no_gen(res)?;
        crate::event::world::AsyncStructureSpawnEvent::from_raw(&jni, res)
    }
    //

    pub fn is_cancelled(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isCancelled", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn handlers(&self) -> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/event/HandlerList;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getHandlers", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::event::HandlerList::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn bounding_box(
        &self,
    ) -> Result<crate::util::BoundingBox<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/util/BoundingBox;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getBoundingBox", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::util::BoundingBox::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn structure(
        &self,
    ) -> Result<crate::generator::structure::Structure<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/generator/structure/Structure;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getStructure", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::generator::structure::Structure::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    /// <span class="descfrm-type-label">Description copied from interface:&nbsp;<code><a href="../Cancellable.html#setCancelled(boolean)">Cancellable</a></code></span>
    /// Sets the cancellation state of this event. A cancelled event will not be executed in the server, but will still pass to other plugins.
    pub fn set_cancelled(&self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        // -2
        let val_1 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setCancelled",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn handler_list(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/event/HandlerList;");
        let cls = jni.find_class("org/bukkit/event/HandlerList");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "getHandlerList", sig.as_str(), vec![]);
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        crate::event::HandlerList::from_raw(&jni, obj)
    }
    //

    pub fn chunk_x(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getChunkX", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    //

    pub fn chunk_z(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getChunkZ", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    //

    pub fn world(&self) -> Result<crate::World<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/World;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getWorld", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::World::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn event_name(&self) -> Result<String, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/String;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getEventName", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    //

    pub fn is_asynchronous(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isAsynchronous", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn wait(
        &self,
        arg0: std::option::Option<i64>,
        arg1: std::option::Option<i32>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "J";
            let val_1 = jni::objects::JValueGen::Long(a.into());
            args.push(val_1);
        }
        if let Some(a) = arg1 {
            sig += "I";
            let val_2 = jni::objects::JValueGen::Int(a.into());
            args.push(val_2);
        }
        sig += ")V";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "wait", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn equals(
        &self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/Object;)Z");
        let val_1 = jni::objects::JValueGen::Object(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "equals",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

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
    //

    pub fn hash_code(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hashCode", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    //

    pub fn class(&self) -> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/Class;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getClass", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(unsafe { jni::objects::JClass::from_raw(res.as_jni().l) })
    }
    //

    pub fn notify(&self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notify", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn notify_all(&self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notifyAll", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
}

impl<'mc> std::string::ToString for AsyncStructureSpawnEvent<'mc> {
    fn to_string(&self) -> String {
        match &self.internal_to_string() {
            Ok(a) => a.clone(),
            Err(err) => format!("Error calling AsyncStructureSpawnEvent.toString: {}", err),
        }
    }
}

impl<'mc> Into<crate::event::Cancellable<'mc>> for AsyncStructureSpawnEvent<'mc> {
    fn into(self) -> crate::event::Cancellable<'mc> {
        crate::event::Cancellable::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting AsyncStructureSpawnEvent into crate::event::Cancellable")
    }
}
impl<'mc> Into<crate::event::world::WorldEvent<'mc>> for AsyncStructureSpawnEvent<'mc> {
    fn into(self) -> crate::event::world::WorldEvent<'mc> {
        crate::event::world::WorldEvent::from_raw(&self.jni_ref(), self.1).expect(
            "Error converting AsyncStructureSpawnEvent into crate::event::world::WorldEvent",
        )
    }
}
/// Thrown when a newly generated chunk has finished being populated.
/// <p><b>Note:</b> Do not use this to generated blocks in a newly generated chunk. Use a <a href="../../generator/BlockPopulator.html" title="class in org.bukkit.generator"><code>BlockPopulator</code></a> instead.</p>
pub struct ChunkPopulateEvent<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for ChunkPopulateEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> ChunkPopulateEvent<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate ChunkPopulateEvent from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/event/world/ChunkPopulateEvent")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a ChunkPopulateEvent object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<crate::Chunk<'mc>>,
    ) -> Result<crate::event::world::ChunkPopulateEvent<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/Chunk;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let cls = jni.find_class("org/bukkit/event/world/ChunkPopulateEvent");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(
            cls,
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = jni.translate_error_no_gen(res)?;
        crate::event::world::ChunkPopulateEvent::from_raw(&jni, res)
    }
    //

    pub fn handlers(&self) -> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/event/HandlerList;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getHandlers", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::event::HandlerList::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn handler_list(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/event/HandlerList;");
        let cls = jni.find_class("org/bukkit/event/HandlerList");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "getHandlerList", sig.as_str(), vec![]);
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        crate::event::HandlerList::from_raw(&jni, obj)
    }
    //

    pub fn chunk(&self) -> Result<crate::Chunk<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/Chunk;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getChunk", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::Chunk::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn world(&self) -> Result<crate::World<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/World;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getWorld", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::World::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn event_name(&self) -> Result<String, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/String;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getEventName", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    //

    pub fn is_asynchronous(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isAsynchronous", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn wait(
        &self,
        arg0: std::option::Option<i64>,
        arg1: std::option::Option<i32>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "J";
            let val_1 = jni::objects::JValueGen::Long(a.into());
            args.push(val_1);
        }
        if let Some(a) = arg1 {
            sig += "I";
            let val_2 = jni::objects::JValueGen::Int(a.into());
            args.push(val_2);
        }
        sig += ")V";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "wait", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn equals(
        &self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/Object;)Z");
        let val_1 = jni::objects::JValueGen::Object(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "equals",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

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
    //

    pub fn hash_code(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hashCode", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    //

    pub fn class(&self) -> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/Class;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getClass", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(unsafe { jni::objects::JClass::from_raw(res.as_jni().l) })
    }
    //

    pub fn notify(&self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notify", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn notify_all(&self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notifyAll", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
}

impl<'mc> std::string::ToString for ChunkPopulateEvent<'mc> {
    fn to_string(&self) -> String {
        match &self.internal_to_string() {
            Ok(a) => a.clone(),
            Err(err) => format!("Error calling ChunkPopulateEvent.toString: {}", err),
        }
    }
}

impl<'mc> Into<crate::event::world::ChunkEvent<'mc>> for ChunkPopulateEvent<'mc> {
    fn into(self) -> crate::event::world::ChunkEvent<'mc> {
        crate::event::world::ChunkEvent::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting ChunkPopulateEvent into crate::event::world::ChunkEvent")
    }
}
/// Called when a chunk is unloaded
pub struct ChunkUnloadEvent<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for ChunkUnloadEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> ChunkUnloadEvent<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate ChunkUnloadEvent from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/event/world/ChunkUnloadEvent")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a ChunkUnloadEvent object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    pub fn new_with_chunk(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: std::option::Option<impl Into<crate::Chunk<'mc>>>,
        arg1: std::option::Option<bool>,
    ) -> Result<crate::event::world::ChunkUnloadEvent<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "Lorg/bukkit/Chunk;";
            let val_1 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(a.into().jni_object().clone())
            });
            args.push(val_1);
        }
        if let Some(a) = arg1 {
            sig += "Z";
            // 0
            let val_2 = jni::objects::JValueGen::Bool(a.into());
            args.push(val_2);
        }
        sig += ")V";
        let cls = jni.find_class("org/bukkit/event/world/ChunkUnloadEvent");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(cls, sig.as_str(), args);
        let res = jni.translate_error_no_gen(res)?;
        crate::event::world::ChunkUnloadEvent::from_raw(&jni, res)
    }
    //

    pub fn handlers(&self) -> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/event/HandlerList;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getHandlers", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::event::HandlerList::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn handler_list(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/event/HandlerList;");
        let cls = jni.find_class("org/bukkit/event/HandlerList");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "getHandlerList", sig.as_str(), vec![]);
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        crate::event::HandlerList::from_raw(&jni, obj)
    }
    //

    pub fn is_save_chunk(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isSaveChunk", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    /// Set whether this chunk will be saved to disk.
    pub fn set_save_chunk(&self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        // -2
        let val_1 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setSaveChunk",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn chunk(&self) -> Result<crate::Chunk<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/Chunk;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getChunk", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::Chunk::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn world(&self) -> Result<crate::World<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/World;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getWorld", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::World::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn event_name(&self) -> Result<String, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/String;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getEventName", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    //

    pub fn is_asynchronous(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isAsynchronous", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn wait(
        &self,
        arg0: std::option::Option<i64>,
        arg1: std::option::Option<i32>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "J";
            let val_1 = jni::objects::JValueGen::Long(a.into());
            args.push(val_1);
        }
        if let Some(a) = arg1 {
            sig += "I";
            let val_2 = jni::objects::JValueGen::Int(a.into());
            args.push(val_2);
        }
        sig += ")V";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "wait", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn equals(
        &self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/Object;)Z");
        let val_1 = jni::objects::JValueGen::Object(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "equals",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

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
    //

    pub fn hash_code(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hashCode", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    //

    pub fn class(&self) -> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/Class;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getClass", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(unsafe { jni::objects::JClass::from_raw(res.as_jni().l) })
    }
    //

    pub fn notify(&self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notify", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn notify_all(&self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notifyAll", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
}

impl<'mc> std::string::ToString for ChunkUnloadEvent<'mc> {
    fn to_string(&self) -> String {
        match &self.internal_to_string() {
            Ok(a) => a.clone(),
            Err(err) => format!("Error calling ChunkUnloadEvent.toString: {}", err),
        }
    }
}

impl<'mc> Into<crate::event::world::ChunkEvent<'mc>> for ChunkUnloadEvent<'mc> {
    fn into(self) -> crate::event::world::ChunkEvent<'mc> {
        crate::event::world::ChunkEvent::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting ChunkUnloadEvent into crate::event::world::ChunkEvent")
    }
}
/// Called when entities are unloaded. The provided chunk may or may not be loaded.
pub struct EntitiesUnloadEvent<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for EntitiesUnloadEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> EntitiesUnloadEvent<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate EntitiesUnloadEvent from null object.").into(),
            );
        }
        let (valid, name) =
            env.validate_name(&obj, "org/bukkit/event/world/EntitiesUnloadEvent")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a EntitiesUnloadEvent object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<crate::Chunk<'mc>>,
        arg1: Vec<impl Into<crate::entity::Entity<'mc>>>,
    ) -> Result<crate::event::world::EntitiesUnloadEvent<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/Chunk;Ljava/util/List;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let raw_val_2 = jni.new_object("java/util/ArrayList", "()V", vec![])?;
        for v in arg1 {
            let map_val_0 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(v.into().jni_object().clone())
            });
            jni.call_method(
                &raw_val_2,
                "add",
                "(Ljava/Lang/Object)V",
                vec![jni::objects::JValueGen::from(map_val_0)],
            )?;
        }
        let val_2 = jni::objects::JValueGen::Object(raw_val_2);
        let cls = jni.find_class("org/bukkit/event/world/EntitiesUnloadEvent");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(
            cls,
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
            ],
        );
        let res = jni.translate_error_no_gen(res)?;
        crate::event::world::EntitiesUnloadEvent::from_raw(&jni, res)
    }
    //

    pub fn handlers(&self) -> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/event/HandlerList;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getHandlers", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::event::HandlerList::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn entities(&self) -> Result<Vec<crate::entity::Entity<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/List;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getEntities", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let mut list = blackboxmc_java::JavaList::from_raw(&self.0, res.l()?)?;
        let size = list.size()?;
        for i in 0..=size {
            let obj = list.get(i)?;
            new_vec.push(crate::entity::Entity::from_raw(&self.0, obj)?);
        }
        Ok(new_vec)
    }
    //

    pub fn handler_list(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/event/HandlerList;");
        let cls = jni.find_class("org/bukkit/event/HandlerList");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "getHandlerList", sig.as_str(), vec![]);
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        crate::event::HandlerList::from_raw(&jni, obj)
    }
    //

    pub fn chunk(&self) -> Result<crate::Chunk<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/Chunk;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getChunk", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::Chunk::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn world(&self) -> Result<crate::World<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/World;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getWorld", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::World::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn event_name(&self) -> Result<String, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/String;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getEventName", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    //

    pub fn is_asynchronous(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isAsynchronous", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn wait(
        &self,
        arg0: std::option::Option<i64>,
        arg1: std::option::Option<i32>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "J";
            let val_1 = jni::objects::JValueGen::Long(a.into());
            args.push(val_1);
        }
        if let Some(a) = arg1 {
            sig += "I";
            let val_2 = jni::objects::JValueGen::Int(a.into());
            args.push(val_2);
        }
        sig += ")V";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "wait", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn equals(
        &self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/Object;)Z");
        let val_1 = jni::objects::JValueGen::Object(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "equals",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

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
    //

    pub fn hash_code(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hashCode", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    //

    pub fn class(&self) -> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/Class;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getClass", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(unsafe { jni::objects::JClass::from_raw(res.as_jni().l) })
    }
    //

    pub fn notify(&self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notify", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn notify_all(&self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notifyAll", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
}

impl<'mc> std::string::ToString for EntitiesUnloadEvent<'mc> {
    fn to_string(&self) -> String {
        match &self.internal_to_string() {
            Ok(a) => a.clone(),
            Err(err) => format!("Error calling EntitiesUnloadEvent.toString: {}", err),
        }
    }
}

impl<'mc> Into<crate::event::world::ChunkEvent<'mc>> for EntitiesUnloadEvent<'mc> {
    fn into(self) -> crate::event::world::ChunkEvent<'mc> {
        crate::event::world::ChunkEvent::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting EntitiesUnloadEvent into crate::event::world::ChunkEvent")
    }
}
/// Called when entities are loaded. The provided chunk may or may not be loaded.
pub struct EntitiesLoadEvent<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for EntitiesLoadEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> EntitiesLoadEvent<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate EntitiesLoadEvent from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/event/world/EntitiesLoadEvent")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a EntitiesLoadEvent object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<crate::Chunk<'mc>>,
        arg1: Vec<impl Into<crate::entity::Entity<'mc>>>,
    ) -> Result<crate::event::world::EntitiesLoadEvent<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/Chunk;Ljava/util/List;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let raw_val_2 = jni.new_object("java/util/ArrayList", "()V", vec![])?;
        for v in arg1 {
            let map_val_0 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(v.into().jni_object().clone())
            });
            jni.call_method(
                &raw_val_2,
                "add",
                "(Ljava/Lang/Object)V",
                vec![jni::objects::JValueGen::from(map_val_0)],
            )?;
        }
        let val_2 = jni::objects::JValueGen::Object(raw_val_2);
        let cls = jni.find_class("org/bukkit/event/world/EntitiesLoadEvent");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(
            cls,
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
            ],
        );
        let res = jni.translate_error_no_gen(res)?;
        crate::event::world::EntitiesLoadEvent::from_raw(&jni, res)
    }
    //

    pub fn handlers(&self) -> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/event/HandlerList;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getHandlers", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::event::HandlerList::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn entities(&self) -> Result<Vec<crate::entity::Entity<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/List;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getEntities", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let mut list = blackboxmc_java::JavaList::from_raw(&self.0, res.l()?)?;
        let size = list.size()?;
        for i in 0..=size {
            let obj = list.get(i)?;
            new_vec.push(crate::entity::Entity::from_raw(&self.0, obj)?);
        }
        Ok(new_vec)
    }
    //

    pub fn handler_list(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/event/HandlerList;");
        let cls = jni.find_class("org/bukkit/event/HandlerList");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "getHandlerList", sig.as_str(), vec![]);
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        crate::event::HandlerList::from_raw(&jni, obj)
    }
    //

    pub fn chunk(&self) -> Result<crate::Chunk<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/Chunk;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getChunk", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::Chunk::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn world(&self) -> Result<crate::World<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/World;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getWorld", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::World::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn event_name(&self) -> Result<String, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/String;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getEventName", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    //

    pub fn is_asynchronous(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isAsynchronous", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn wait(
        &self,
        arg0: std::option::Option<i64>,
        arg1: std::option::Option<i32>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "J";
            let val_1 = jni::objects::JValueGen::Long(a.into());
            args.push(val_1);
        }
        if let Some(a) = arg1 {
            sig += "I";
            let val_2 = jni::objects::JValueGen::Int(a.into());
            args.push(val_2);
        }
        sig += ")V";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "wait", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn equals(
        &self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/Object;)Z");
        let val_1 = jni::objects::JValueGen::Object(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "equals",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

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
    //

    pub fn hash_code(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hashCode", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    //

    pub fn class(&self) -> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/Class;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getClass", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(unsafe { jni::objects::JClass::from_raw(res.as_jni().l) })
    }
    //

    pub fn notify(&self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notify", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn notify_all(&self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notifyAll", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
}

impl<'mc> std::string::ToString for EntitiesLoadEvent<'mc> {
    fn to_string(&self) -> String {
        match &self.internal_to_string() {
            Ok(a) => a.clone(),
            Err(err) => format!("Error calling EntitiesLoadEvent.toString: {}", err),
        }
    }
}

impl<'mc> Into<crate::event::world::ChunkEvent<'mc>> for EntitiesLoadEvent<'mc> {
    fn into(self) -> crate::event::world::ChunkEvent<'mc> {
        crate::event::world::ChunkEvent::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting EntitiesLoadEvent into crate::event::world::ChunkEvent")
    }
}
/// Represents a generic Mojang game event. Specific Bukkit events should be used where possible, this event is mainly used internally by Sculk sensors.
pub struct GenericGameEvent<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for GenericGameEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> GenericGameEvent<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate GenericGameEvent from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/event/world/GenericGameEvent")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a GenericGameEvent object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<crate::GameEvent<'mc>>,
        arg1: impl Into<crate::Location<'mc>>,
        arg2: impl Into<crate::entity::Entity<'mc>>,
        arg3: i32,
        arg4: bool,
    ) -> Result<crate::event::world::GenericGameEvent<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from(
            "(Lorg/bukkit/GameEvent;Lorg/bukkit/Location;Lorg/bukkit/entity/Entity;IZ)V",
        );
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let val_2 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg1.into().jni_object().clone())
        });
        let val_3 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg2.into().jni_object().clone())
        });
        let val_4 = jni::objects::JValueGen::Int(arg3.into());
        // -2
        let val_5 = jni::objects::JValueGen::Bool(arg4.into());
        let cls = jni.find_class("org/bukkit/event/world/GenericGameEvent");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(
            cls,
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
                jni::objects::JValueGen::from(val_3),
                jni::objects::JValueGen::from(val_4),
                jni::objects::JValueGen::from(val_5),
            ],
        );
        let res = jni.translate_error_no_gen(res)?;
        crate::event::world::GenericGameEvent::from_raw(&jni, res)
    }
    //

    pub fn is_cancelled(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isCancelled", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn handlers(&self) -> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/event/HandlerList;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getHandlers", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::event::HandlerList::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn entity(&self) -> Result<crate::entity::Entity<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/entity/Entity;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getEntity", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::entity::Entity::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn radius(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getRadius", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    //

    /// Set the radius to which the event should be broadcast.
    pub fn set_radius(&self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(I)V");
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setRadius",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn event(&self) -> Result<crate::GameEvent<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/GameEvent;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getEvent", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::GameEvent::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    /// <span class="descfrm-type-label">Description copied from interface:&nbsp;<code><a href="../Cancellable.html#setCancelled(boolean)">Cancellable</a></code></span>
    /// Sets the cancellation state of this event. A cancelled event will not be executed in the server, but will still pass to other plugins.
    pub fn set_cancelled(&self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        // -2
        let val_1 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setCancelled",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn handler_list(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/event/HandlerList;");
        let cls = jni.find_class("org/bukkit/event/HandlerList");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "getHandlerList", sig.as_str(), vec![]);
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        crate::event::HandlerList::from_raw(&jni, obj)
    }
    //

    pub fn location(&self) -> Result<crate::Location<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/Location;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getLocation", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::Location::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn world(&self) -> Result<crate::World<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/World;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getWorld", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::World::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn event_name(&self) -> Result<String, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/String;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getEventName", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    //

    pub fn is_asynchronous(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isAsynchronous", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn wait(
        &self,
        arg0: std::option::Option<i64>,
        arg1: std::option::Option<i32>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "J";
            let val_1 = jni::objects::JValueGen::Long(a.into());
            args.push(val_1);
        }
        if let Some(a) = arg1 {
            sig += "I";
            let val_2 = jni::objects::JValueGen::Int(a.into());
            args.push(val_2);
        }
        sig += ")V";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "wait", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn equals(
        &self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/Object;)Z");
        let val_1 = jni::objects::JValueGen::Object(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "equals",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

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
    //

    pub fn hash_code(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hashCode", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    //

    pub fn class(&self) -> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/Class;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getClass", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(unsafe { jni::objects::JClass::from_raw(res.as_jni().l) })
    }
    //

    pub fn notify(&self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notify", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn notify_all(&self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notifyAll", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
}

impl<'mc> std::string::ToString for GenericGameEvent<'mc> {
    fn to_string(&self) -> String {
        match &self.internal_to_string() {
            Ok(a) => a.clone(),
            Err(err) => format!("Error calling GenericGameEvent.toString: {}", err),
        }
    }
}

impl<'mc> Into<crate::event::Cancellable<'mc>> for GenericGameEvent<'mc> {
    fn into(self) -> crate::event::Cancellable<'mc> {
        crate::event::Cancellable::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting GenericGameEvent into crate::event::Cancellable")
    }
}
impl<'mc> Into<crate::event::world::WorldEvent<'mc>> for GenericGameEvent<'mc> {
    fn into(self) -> crate::event::world::WorldEvent<'mc> {
        crate::event::world::WorldEvent::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting GenericGameEvent into crate::event::world::WorldEvent")
    }
}
/// Represents events within a world
pub struct WorldEvent<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for WorldEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> WorldEvent<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate WorldEvent from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/event/world/WorldEvent")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a WorldEvent object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    pub fn new_with_world(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: std::option::Option<impl Into<crate::World<'mc>>>,
        arg1: std::option::Option<bool>,
    ) -> Result<crate::event::world::WorldEvent<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "Lorg/bukkit/World;";
            let val_1 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(a.into().jni_object().clone())
            });
            args.push(val_1);
        }
        if let Some(a) = arg1 {
            sig += "Z";
            // 0
            let val_2 = jni::objects::JValueGen::Bool(a.into());
            args.push(val_2);
        }
        sig += ")V";
        let cls = jni.find_class("org/bukkit/event/world/WorldEvent");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(cls, sig.as_str(), args);
        let res = jni.translate_error_no_gen(res)?;
        crate::event::world::WorldEvent::from_raw(&jni, res)
    }
    //

    pub fn world(&self) -> Result<crate::World<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/World;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getWorld", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::World::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn handlers(&self) -> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/event/HandlerList;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getHandlers", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::event::HandlerList::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn event_name(&self) -> Result<String, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/String;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getEventName", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    //

    pub fn is_asynchronous(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isAsynchronous", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn wait(
        &self,
        arg0: std::option::Option<i64>,
        arg1: std::option::Option<i32>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "J";
            let val_1 = jni::objects::JValueGen::Long(a.into());
            args.push(val_1);
        }
        if let Some(a) = arg1 {
            sig += "I";
            let val_2 = jni::objects::JValueGen::Int(a.into());
            args.push(val_2);
        }
        sig += ")V";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "wait", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn equals(
        &self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/Object;)Z");
        let val_1 = jni::objects::JValueGen::Object(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "equals",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

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
    //

    pub fn hash_code(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hashCode", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    //

    pub fn class(&self) -> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/Class;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getClass", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(unsafe { jni::objects::JClass::from_raw(res.as_jni().l) })
    }
    //

    pub fn notify(&self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notify", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn notify_all(&self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notifyAll", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
}

impl<'mc> std::string::ToString for WorldEvent<'mc> {
    fn to_string(&self) -> String {
        match &self.internal_to_string() {
            Ok(a) => a.clone(),
            Err(err) => format!("Error calling WorldEvent.toString: {}", err),
        }
    }
}

impl<'mc> Into<crate::event::Event<'mc>> for WorldEvent<'mc> {
    fn into(self) -> crate::event::Event<'mc> {
        crate::event::Event::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting WorldEvent into crate::event::Event")
    }
}
/// Represents a Chunk related event
pub struct ChunkEvent<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for ChunkEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> ChunkEvent<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate ChunkEvent from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/event/world/ChunkEvent")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a ChunkEvent object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    //

    pub fn chunk(&self) -> Result<crate::Chunk<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/Chunk;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getChunk", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::Chunk::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn world(&self) -> Result<crate::World<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/World;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getWorld", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::World::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn handlers(&self) -> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/event/HandlerList;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getHandlers", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::event::HandlerList::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn event_name(&self) -> Result<String, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/String;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getEventName", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    //

    pub fn is_asynchronous(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isAsynchronous", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn wait(
        &self,
        arg0: std::option::Option<i64>,
        arg1: std::option::Option<i32>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "J";
            let val_1 = jni::objects::JValueGen::Long(a.into());
            args.push(val_1);
        }
        if let Some(a) = arg1 {
            sig += "I";
            let val_2 = jni::objects::JValueGen::Int(a.into());
            args.push(val_2);
        }
        sig += ")V";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "wait", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn equals(
        &self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/Object;)Z");
        let val_1 = jni::objects::JValueGen::Object(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "equals",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

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
    //

    pub fn hash_code(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hashCode", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    //

    pub fn class(&self) -> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/Class;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getClass", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(unsafe { jni::objects::JClass::from_raw(res.as_jni().l) })
    }
    //

    pub fn notify(&self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notify", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn notify_all(&self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notifyAll", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
}

impl<'mc> std::string::ToString for ChunkEvent<'mc> {
    fn to_string(&self) -> String {
        match &self.internal_to_string() {
            Ok(a) => a.clone(),
            Err(err) => format!("Error calling ChunkEvent.toString: {}", err),
        }
    }
}

impl<'mc> Into<crate::event::world::WorldEvent<'mc>> for ChunkEvent<'mc> {
    fn into(self) -> crate::event::world::WorldEvent<'mc> {
        crate::event::world::WorldEvent::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting ChunkEvent into crate::event::world::WorldEvent")
    }
}
/// Event that is called when an organic structure attempts to grow (Sapling -&gt; Tree), (Mushroom -&gt; Huge Mushroom), naturally or using bonemeal.
pub struct StructureGrowEvent<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for StructureGrowEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> StructureGrowEvent<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate StructureGrowEvent from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/event/world/StructureGrowEvent")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a StructureGrowEvent object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<crate::Location<'mc>>,
        arg1: impl Into<crate::TreeType<'mc>>,
        arg2: bool,
        arg3: impl Into<crate::entity::Player<'mc>>,
        arg4: Vec<impl Into<crate::block::BlockState<'mc>>>,
    ) -> Result<crate::event::world::StructureGrowEvent<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/Location;Lorg/bukkit/TreeType;ZLorg/bukkit/entity/Player;Ljava/util/List;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let val_2 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg1.into().jni_object().clone())
        });
        // -2
        let val_3 = jni::objects::JValueGen::Bool(arg2.into());
        let val_4 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg3.into().jni_object().clone())
        });
        let raw_val_5 = jni.new_object("java/util/ArrayList", "()V", vec![])?;
        for v in arg4 {
            let map_val_0 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(v.into().jni_object().clone())
            });
            jni.call_method(
                &raw_val_5,
                "add",
                "(Ljava/Lang/Object)V",
                vec![jni::objects::JValueGen::from(map_val_0)],
            )?;
        }
        let val_5 = jni::objects::JValueGen::Object(raw_val_5);
        let cls = jni.find_class("org/bukkit/event/world/StructureGrowEvent");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(
            cls,
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
                jni::objects::JValueGen::from(val_3),
                jni::objects::JValueGen::from(val_4),
                jni::objects::JValueGen::from(val_5),
            ],
        );
        let res = jni.translate_error_no_gen(res)?;
        crate::event::world::StructureGrowEvent::from_raw(&jni, res)
    }
    //

    pub fn is_cancelled(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isCancelled", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn handlers(&self) -> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/event/HandlerList;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getHandlers", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::event::HandlerList::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn player(&self) -> Result<crate::entity::Player<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/entity/Player;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getPlayer", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::entity::Player::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn blocks(&self) -> Result<Vec<crate::block::BlockState<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/List;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getBlocks", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let mut list = blackboxmc_java::JavaList::from_raw(&self.0, res.l()?)?;
        let size = list.size()?;
        for i in 0..=size {
            let obj = list.get(i)?;
            new_vec.push(crate::block::BlockState::from_raw(&self.0, obj)?);
        }
        Ok(new_vec)
    }
    //

    /// <span class="descfrm-type-label">Description copied from interface:&nbsp;<code><a href="../Cancellable.html#setCancelled(boolean)">Cancellable</a></code></span>
    /// Sets the cancellation state of this event. A cancelled event will not be executed in the server, but will still pass to other plugins.
    pub fn set_cancelled(&self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        // -2
        let val_1 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setCancelled",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn handler_list(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/event/HandlerList;");
        let cls = jni.find_class("org/bukkit/event/HandlerList");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "getHandlerList", sig.as_str(), vec![]);
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        crate::event::HandlerList::from_raw(&jni, obj)
    }
    //

    pub fn species(&self) -> Result<crate::TreeType<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/TreeType;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getSpecies", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant = self
            .0
            .call_method(&raw_obj, "toString", "()Ljava/lang/String;", vec![]);
        let variant = self.jni_ref().translate_error(variant)?;
        let variant_str = self
            .0
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::TreeType::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::TreeType::from_string(variant_str)
                .ok_or(eyre::eyre!("String gaven for variant was invalid"))?,
        )
    }
    //

    pub fn is_from_bonemeal(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isFromBonemeal", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn location(&self) -> Result<crate::Location<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/Location;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getLocation", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::Location::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn world(&self) -> Result<crate::World<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/World;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getWorld", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::World::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn event_name(&self) -> Result<String, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/String;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getEventName", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    //

    pub fn is_asynchronous(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isAsynchronous", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn wait(
        &self,
        arg0: std::option::Option<i64>,
        arg1: std::option::Option<i32>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "J";
            let val_1 = jni::objects::JValueGen::Long(a.into());
            args.push(val_1);
        }
        if let Some(a) = arg1 {
            sig += "I";
            let val_2 = jni::objects::JValueGen::Int(a.into());
            args.push(val_2);
        }
        sig += ")V";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "wait", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn equals(
        &self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/Object;)Z");
        let val_1 = jni::objects::JValueGen::Object(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "equals",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

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
    //

    pub fn hash_code(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hashCode", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    //

    pub fn class(&self) -> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/Class;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getClass", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(unsafe { jni::objects::JClass::from_raw(res.as_jni().l) })
    }
    //

    pub fn notify(&self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notify", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn notify_all(&self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notifyAll", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
}

impl<'mc> std::string::ToString for StructureGrowEvent<'mc> {
    fn to_string(&self) -> String {
        match &self.internal_to_string() {
            Ok(a) => a.clone(),
            Err(err) => format!("Error calling StructureGrowEvent.toString: {}", err),
        }
    }
}

impl<'mc> Into<crate::event::Cancellable<'mc>> for StructureGrowEvent<'mc> {
    fn into(self) -> crate::event::Cancellable<'mc> {
        crate::event::Cancellable::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting StructureGrowEvent into crate::event::Cancellable")
    }
}
impl<'mc> Into<crate::event::world::WorldEvent<'mc>> for StructureGrowEvent<'mc> {
    fn into(self) -> crate::event::world::WorldEvent<'mc> {
        crate::event::world::WorldEvent::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting StructureGrowEvent into crate::event::world::WorldEvent")
    }
}
/// Called when the time skips in a world.
/// <p>If the event is cancelled the time will not change.</p>
pub struct TimeSkipEvent<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
#[derive(PartialEq, Eq)]
pub enum TimeSkipEventSkipReasonEnum {
    Command,
    Custom,
    NightSkip,
}
impl std::fmt::Display for TimeSkipEventSkipReasonEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TimeSkipEventSkipReasonEnum::Command => f.write_str("COMMAND"),
            TimeSkipEventSkipReasonEnum::Custom => f.write_str("CUSTOM"),
            TimeSkipEventSkipReasonEnum::NightSkip => f.write_str("NIGHT_SKIP"),
        }
    }
}
impl<'mc> std::fmt::Display for TimeSkipEventSkipReason<'mc> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.2.fmt(f)
    }
}
pub struct TimeSkipEventSkipReason<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
    pub TimeSkipEventSkipReasonEnum,
);
impl<'mc> std::ops::Deref for TimeSkipEventSkipReason<'mc> {
    type Target = TimeSkipEventSkipReasonEnum;
    fn deref(&self) -> &Self::Target {
        return &self.2;
    }
}
impl<'mc> JNIRaw<'mc> for TimeSkipEventSkipReason<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> TimeSkipEventSkipReason<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
        e: TimeSkipEventSkipReasonEnum,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate TimeSkipEventSkipReason from null object."
            )
            .into());
        }
        let (valid, name) =
            env.validate_name(&obj, "org/bukkit/event/world/TimeSkipEvent$SkipReason")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a TimeSkipEventSkipReason object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj, e))
        }
    }
    pub const COMMAND: TimeSkipEventSkipReasonEnum = TimeSkipEventSkipReasonEnum::Command;
    pub const CUSTOM: TimeSkipEventSkipReasonEnum = TimeSkipEventSkipReasonEnum::Custom;
    pub const NIGHT_SKIP: TimeSkipEventSkipReasonEnum = TimeSkipEventSkipReasonEnum::NightSkip;
    pub fn from_string(str: String) -> std::option::Option<TimeSkipEventSkipReasonEnum> {
        match str.as_str() {
            "COMMAND" => Some(TimeSkipEventSkipReasonEnum::Command),
            "CUSTOM" => Some(TimeSkipEventSkipReasonEnum::Custom),
            "NIGHT_SKIP" => Some(TimeSkipEventSkipReasonEnum::NightSkip),
            _ => None,
        }
    }

    pub fn value_of(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<String>,
    ) -> Result<TimeSkipEventSkipReason<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(jni.new_string(arg0.into())?);
        let cls = jni.find_class("org/bukkit/event/world/TimeSkipEvent$SkipReason");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(
            cls,
            "valueOf",
            "(Ljava/lang/String;)Lorg/bukkit/event/world/TimeSkipEvent$SkipReason;",
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        let raw_obj = obj;
        let variant = jni.call_method(&raw_obj, "toString", "()Ljava/lang/String;", vec![]);
        let variant = jni.translate_error(variant)?;
        let variant_str = jni
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        TimeSkipEventSkipReason::from_raw(
            &jni,
            raw_obj,
            TimeSkipEventSkipReason::from_string(variant_str)
                .ok_or(eyre::eyre!("String gaven for variant was invalid"))?,
        )
    }

    //
}
impl<'mc> blackboxmc_general::JNIRaw<'mc> for TimeSkipEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> TimeSkipEvent<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate TimeSkipEvent from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/event/world/TimeSkipEvent")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a TimeSkipEvent object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<crate::World<'mc>>,
        arg1: impl Into<crate::event::world::TimeSkipEventSkipReason<'mc>>,
        arg2: i64,
    ) -> Result<crate::event::world::TimeSkipEvent<'mc>, Box<dyn std::error::Error>> {
        let sig =
            String::from("(Lorg/bukkit/World;Lorg/bukkit/event/world/TimeSkipEvent$SkipReason;J)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let val_2 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg1.into().jni_object().clone())
        });
        let val_3 = jni::objects::JValueGen::Long(arg2.into());
        let cls = jni.find_class("org/bukkit/event/world/TimeSkipEvent");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(
            cls,
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
                jni::objects::JValueGen::from(val_3),
            ],
        );
        let res = jni.translate_error_no_gen(res)?;
        crate::event::world::TimeSkipEvent::from_raw(&jni, res)
    }
    //

    pub fn is_cancelled(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isCancelled", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn handlers(&self) -> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/event/HandlerList;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getHandlers", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::event::HandlerList::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    /// <span class="descfrm-type-label">Description copied from interface:&nbsp;<code><a href="../Cancellable.html#setCancelled(boolean)">Cancellable</a></code></span>
    /// Sets the cancellation state of this event. A cancelled event will not be executed in the server, but will still pass to other plugins.
    pub fn set_cancelled(&self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        // -2
        let val_1 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setCancelled",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn handler_list(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/event/HandlerList;");
        let cls = jni.find_class("org/bukkit/event/HandlerList");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "getHandlerList", sig.as_str(), vec![]);
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        crate::event::HandlerList::from_raw(&jni, obj)
    }
    //

    pub fn skip_reason(
        &self,
    ) -> Result<crate::event::world::TimeSkipEventSkipReason<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/event/world/TimeSkipEvent$SkipReason;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getSkipReason", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant = self
            .0
            .call_method(&raw_obj, "toString", "()Ljava/lang/String;", vec![]);
        let variant = self.jni_ref().translate_error(variant)?;
        let variant_str = self
            .0
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::event::world::TimeSkipEventSkipReason::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::event::world::TimeSkipEventSkipReason::from_string(variant_str)
                .ok_or(eyre::eyre!("String gaven for variant was invalid"))?,
        )
    }
    //

    pub fn skip_amount(&self) -> Result<i64, Box<dyn std::error::Error>> {
        let sig = String::from("()J");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getSkipAmount", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.j()?)
    }
    //

    /// Sets the amount of time to skip.
    pub fn set_skip_amount(&self, arg0: i64) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(J)V");
        let val_1 = jni::objects::JValueGen::Long(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setSkipAmount",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn world(&self) -> Result<crate::World<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/World;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getWorld", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::World::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn event_name(&self) -> Result<String, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/String;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getEventName", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    //

    pub fn is_asynchronous(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isAsynchronous", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn wait(
        &self,
        arg0: std::option::Option<i64>,
        arg1: std::option::Option<i32>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "J";
            let val_1 = jni::objects::JValueGen::Long(a.into());
            args.push(val_1);
        }
        if let Some(a) = arg1 {
            sig += "I";
            let val_2 = jni::objects::JValueGen::Int(a.into());
            args.push(val_2);
        }
        sig += ")V";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "wait", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn equals(
        &self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/Object;)Z");
        let val_1 = jni::objects::JValueGen::Object(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "equals",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

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
    //

    pub fn hash_code(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hashCode", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    //

    pub fn class(&self) -> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/Class;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getClass", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(unsafe { jni::objects::JClass::from_raw(res.as_jni().l) })
    }
    //

    pub fn notify(&self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notify", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn notify_all(&self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notifyAll", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
}

impl<'mc> std::string::ToString for TimeSkipEvent<'mc> {
    fn to_string(&self) -> String {
        match &self.internal_to_string() {
            Ok(a) => a.clone(),
            Err(err) => format!("Error calling TimeSkipEvent.toString: {}", err),
        }
    }
}

impl<'mc> Into<crate::event::Cancellable<'mc>> for TimeSkipEvent<'mc> {
    fn into(self) -> crate::event::Cancellable<'mc> {
        crate::event::Cancellable::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting TimeSkipEvent into crate::event::Cancellable")
    }
}
impl<'mc> Into<crate::event::world::WorldEvent<'mc>> for TimeSkipEvent<'mc> {
    fn into(self) -> crate::event::world::WorldEvent<'mc> {
        crate::event::world::WorldEvent::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting TimeSkipEvent into crate::event::world::WorldEvent")
    }
}
/// Called when a World is saved.
pub struct WorldSaveEvent<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for WorldSaveEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> WorldSaveEvent<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate WorldSaveEvent from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/event/world/WorldSaveEvent")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a WorldSaveEvent object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<crate::World<'mc>>,
    ) -> Result<crate::event::world::WorldSaveEvent<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/World;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let cls = jni.find_class("org/bukkit/event/world/WorldSaveEvent");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(
            cls,
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = jni.translate_error_no_gen(res)?;
        crate::event::world::WorldSaveEvent::from_raw(&jni, res)
    }
    //

    pub fn handlers(&self) -> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/event/HandlerList;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getHandlers", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::event::HandlerList::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn handler_list(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/event/HandlerList;");
        let cls = jni.find_class("org/bukkit/event/HandlerList");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "getHandlerList", sig.as_str(), vec![]);
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        crate::event::HandlerList::from_raw(&jni, obj)
    }
    //

    pub fn world(&self) -> Result<crate::World<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/World;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getWorld", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::World::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn event_name(&self) -> Result<String, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/String;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getEventName", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    //

    pub fn is_asynchronous(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isAsynchronous", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn wait(
        &self,
        arg0: std::option::Option<i64>,
        arg1: std::option::Option<i32>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "J";
            let val_1 = jni::objects::JValueGen::Long(a.into());
            args.push(val_1);
        }
        if let Some(a) = arg1 {
            sig += "I";
            let val_2 = jni::objects::JValueGen::Int(a.into());
            args.push(val_2);
        }
        sig += ")V";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "wait", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn equals(
        &self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/Object;)Z");
        let val_1 = jni::objects::JValueGen::Object(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "equals",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

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
    //

    pub fn hash_code(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hashCode", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    //

    pub fn class(&self) -> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/Class;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getClass", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(unsafe { jni::objects::JClass::from_raw(res.as_jni().l) })
    }
    //

    pub fn notify(&self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notify", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn notify_all(&self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notifyAll", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
}

impl<'mc> std::string::ToString for WorldSaveEvent<'mc> {
    fn to_string(&self) -> String {
        match &self.internal_to_string() {
            Ok(a) => a.clone(),
            Err(err) => format!("Error calling WorldSaveEvent.toString: {}", err),
        }
    }
}

impl<'mc> Into<crate::event::world::WorldEvent<'mc>> for WorldSaveEvent<'mc> {
    fn into(self) -> crate::event::world::WorldEvent<'mc> {
        crate::event::world::WorldEvent::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting WorldSaveEvent into crate::event::world::WorldEvent")
    }
}
