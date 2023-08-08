#![allow(deprecated)]
use blackboxmc_general::JNIRaw;
use color_eyre::eyre::Result;
/// A list of event handlers, stored per-event. Based on lahwran's fevents.
pub struct HandlerList<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for HandlerList<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> HandlerList<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate HandlerList from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/event/HandlerList")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a HandlerList object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    pub fn new(
        jni: blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>> {
        let cls = &jni.find_class("org/bukkit/event/HandlerList")?;
        let res = jni.new_object(cls, "()V", &[])?;
        crate::event::HandlerList::from_raw(&jni, res)
    }
    /// Unregister all listeners from all handler lists.
    /// Unregister a specific plugin's listeners from all handler lists.
    /// Unregister a specific listener from all handler lists.
    /// Remove a listener from a specific order slot
    /// Remove a specific plugin's listeners from this handler
    /// Remove a specific listener from this handler
    pub fn unregister_with_registered_listener(
        &mut self,
        arg0: std::option::Option<impl Into<&'mc crate::plugin::Plugin<'mc>>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 =
            unsafe { jni::objects::JObject::from_raw(arg0.unwrap().into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "unregister",
            "(Lorg/bukkit/plugin/Plugin;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Unregister all listeners from all handler lists.
    /// Unregister a specific plugin's listeners from all handler lists.
    /// Unregister a specific listener from all handler lists.
    /// Remove a listener from a specific order slot
    /// Remove a specific plugin's listeners from this handler
    /// Remove a specific listener from this handler
    pub fn unregister_with_listener(
        &mut self,
        arg0: std::option::Option<impl Into<&'mc crate::event::Listener<'mc>>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 =
            unsafe { jni::objects::JObject::from_raw(arg0.unwrap().into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "unregister",
            "(Lorg/bukkit/event/Listener;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Unregister all listeners from all handler lists.
    /// Unregister a specific plugin's listeners from all handler lists.
    /// Unregister a specific listener from all handler lists.
    pub fn unregister_all(
        jni: blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: std::option::Option<impl Into<&'mc crate::event::Listener<'mc>>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 =
            unsafe { jni::objects::JObject::from_raw(arg0.unwrap().into().jni_object().clone()) };
        let cls = &jni.find_class("void")?;
        let res = jni.call_static_method(
            cls,
            "unregisterAll",
            "(Lorg/bukkit/event/Listener;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        )?;
        Ok(())
    }
    /// Bake all handler lists. Best used just after all normal event registration is complete, ie just after all plugins are loaded if you're using fevents in a plugin system.
    pub fn bake_all(
        jni: blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let cls = &jni.find_class("void")?;
        let res = jni.call_static_method(cls, "bakeAll", "()V", &[])?;
        Ok(())
    }
    /// Bake all handler lists. Best used just after all normal event registration is complete, ie just after all plugins are loaded if you're using fevents in a plugin system.
    /// Bake HashMap and ArrayLists to 2d array - does nothing if not necessary
    pub fn bake(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "bake", "()V", &[]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Get the baked registered listeners associated with this handler list
    /// Get a specific plugin's registered listeners associated with this handler list
    pub fn get_registered_listeners(
        jni: blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<&'mc crate::plugin::Plugin<'mc>>,
    ) -> Result<blackboxmc_java::JavaArrayList<'mc>, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let cls = &jni.find_class("java/util/ArrayList")?;
        let res = jni.call_static_method(
            cls,
            "getRegisteredListeners",
            "(Lorg/bukkit/plugin/Plugin;)Ljava/util/ArrayList;",
            &[jni::objects::JValueGen::from(&val_1)],
        )?;
        let obj = res.l()?;
        blackboxmc_java::JavaArrayList::from_raw(&jni, obj)
    }
    /// Get a list of all handler lists for every event type
    pub fn handler_lists(
        jni: blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<blackboxmc_java::JavaArrayList<'mc>, Box<dyn std::error::Error>> {
        let cls = &jni.find_class("java/util/ArrayList")?;
        let res = jni.call_static_method(cls, "getHandlerLists", "()Ljava/util/ArrayList;", &[])?;
        let obj = res.l()?;
        blackboxmc_java::JavaArrayList::from_raw(&jni, obj)
    }
    /// Register a new listener in this handler list
    /// Register a collection of new listeners in this handler list
    pub fn register(
        &mut self,
        arg0: impl Into<&'mc crate::plugin::RegisteredListener<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "register",
            "(Lorg/bukkit/plugin/RegisteredListener;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
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
/// Simple interface for tagging all EventListeners
///
/// This is a representation of an abstract class.
pub struct Listener<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> Listener<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Listener from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/event/Listener")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Listener object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}
impl<'mc> JNIRaw<'mc> for Listener<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
/// A type characterizing events that may be cancelled by a plugin or the server.
///
/// This is a representation of an abstract class.
pub struct Cancellable<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> Cancellable<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Cancellable from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/event/Cancellable")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Cancellable object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    /// Gets the cancellation state of this event. A cancelled event will not be executed in the server, but will still pass to other plugins
    pub fn is_cancelled(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isCancelled", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    /// Sets the cancellation state of this event. A cancelled event will not be executed in the server, but will still pass to other plugins.
    pub fn set_cancelled(&mut self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        // -2
        let val_1 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setCancelled",
            "(Z)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
}
impl<'mc> JNIRaw<'mc> for Cancellable<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
/// An annotation to mark methods as being event handler methods
///
/// This is a representation of an abstract class.
pub struct EventHandler<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> EventHandler<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate EventHandler from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/event/EventHandler")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a EventHandler object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    /// Define if the handler ignores a cancelled event.
    /// <p>If ignoreCancelled is true and the event is cancelled, the method is not called. Otherwise, the method is always called.</p>
    pub fn ignore_cancelled(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "ignoreCancelled", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    /// Define the priority of the event.
    /// <p>First priority to the last priority executed:</p>
    /// <ol>
    /// <li>LOWEST</li>
    /// <li>LOW</li>
    /// <li>NORMAL</li>
    /// <li>HIGH</li>
    /// <li>HIGHEST</li>
    /// <li>MONITOR</li>
    /// </ol>
    pub fn priority(
        &mut self,
    ) -> Result<crate::event::EventPriority<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "priority",
            "()Lorg/bukkit/event/EventPriority;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant =
            self.jni_ref()
                .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
        let variant_str = self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::event::EventPriority::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::event::EventPriority::from_string(variant_str).unwrap(),
        )
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

    pub fn annotation_type(
        &mut self,
    ) -> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "annotationType",
            "()Ljava/lang/Class;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(unsafe { jni::objects::JClass::from_raw(res.as_jni().l) })
    }
}
impl<'mc> JNIRaw<'mc> for EventHandler<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
/// Represents an event. All events require a static method named getHandlerList() which returns the same <a title="class in org.bukkit.event" href="HandlerList.html"><code>HandlerList</code></a> as <a href="#getHandlers()"><code>getHandlers()</code></a>.
pub struct Event<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
pub struct EventResult<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for EventResult<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> EventResult<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate EventResult from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/event/EventResult")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a EventResult object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    pub fn value_of_with_string(
        jni: blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: std::option::Option<jni::objects::JClass<'mc>>,
        arg1: std::option::Option<impl Into<&'mc String>>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let val_1 = arg0.unwrap();
        let val_2 = jni::objects::JObject::from(jni.new_string(arg1.unwrap().into()).unwrap());
        let cls = &jni.find_class("java/lang/Enum")?;
        let res = jni.call_static_method(
            cls,
            "valueOf",
            "(Ljava/lang/Class;Ljava/lang/String;)Ljava/lang/Enum;",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        )?;
        let obj = res.l()?;
        Self::from_raw(&jni, obj)
    }
    pub fn name(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "name", "()Ljava/lang/String;", &[]);
        let res = self.jni_ref().translate_error(res)?;
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
    pub fn describe_constable(
        &mut self,
    ) -> Result<blackboxmc_java::JavaOptional<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "describeConstable",
            "()Ljava/util/Optional;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::JavaOptional::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    pub fn declaring_class(
        &mut self,
    ) -> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getDeclaringClass",
            "()Ljava/lang/Class;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(unsafe { jni::objects::JClass::from_raw(res.as_jni().l) })
    }
    pub fn ordinal(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "ordinal", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
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
impl<'mc> blackboxmc_general::JNIRaw<'mc> for Event<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Event<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Event from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/event/Event")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Event object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    pub fn new(
        jni: blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: std::option::Option<bool>,
    ) -> Result<crate::event::Event<'mc>, Box<dyn std::error::Error>> {
        // -1
        let val_1 = jni::objects::JValueGen::Bool(arg0.unwrap().into());
        let cls = &jni.find_class("org/bukkit/event/Event")?;
        let res = jni.new_object(cls, "(Z)V", &[jni::objects::JValueGen::from(&val_1)])?;
        crate::event::Event::from_raw(&jni, res)
    }

    pub fn handlers(
        &mut self,
    ) -> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getHandlers",
            "()Lorg/bukkit/event/HandlerList;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::event::HandlerList::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Convenience method for providing a user-friendly identifier. By default, it is the event's class's <a class="external-link" href="https://docs.oracle.com/javase/8/docs/api/java/lang/Class.html#getSimpleName--" title="class or interface in java.lang">simple name</a>.
    pub fn event_name(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getEventName",
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
    /// Any custom event that should not by synchronized with other events must use the specific constructor. These are the caveats of using an asynchronous event:
    /// <ul>
    /// <li>The event is never fired from inside code triggered by a synchronous event. Attempting to do so results in an <a class="external-link" href="https://docs.oracle.com/javase/8/docs/api/java/lang/IllegalStateException.html" title="class or interface in java.lang"><code>IllegalStateException</code></a>.</li>
    /// <li>However, asynchronous event handlers may fire synchronous or asynchronous events</li>
    /// <li>The event may be fired multiple times simultaneously and in any order.</li>
    /// <li>Any newly registered or unregistered handler is ignored after an event starts execution.</li>
    /// <li>The handlers for this event may block for any length of time.</li>
    /// <li>Some implementations may selectively declare a specific event use as asynchronous. This behavior should be clearly defined.</li>
    /// <li>Asynchronous calls are not calculated in the plugin timing system.</li>
    /// </ul>
    pub fn is_asynchronous(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isAsynchronous", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
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
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
    pub EventPriorityEnum,
);
impl<'mc> std::ops::Deref for EventPriority<'mc> {
    type Target = EventPriorityEnum;
    fn deref(&self) -> &Self::Target {
        return &self.2;
    }
}
impl<'mc> JNIRaw<'mc> for EventPriority<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> EventPriority<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
        e: EventPriorityEnum,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate EventPriority from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/event/EventPriority")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a EventPriority object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj, e))
        }
    }
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
