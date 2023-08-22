#![allow(deprecated)]
use blackboxmc_general::JNIInstantiatable;
use blackboxmc_general::JNIRaw;
use color_eyre::eyre::Result;
pub enum EventResult<'mc> {
    Deny { inner: EventResultStruct<'mc> },
    Default { inner: EventResultStruct<'mc> },
    Allow { inner: EventResultStruct<'mc> },
}
impl<'mc> std::fmt::Display for EventResult<'mc> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EventResult::Deny { .. } => f.write_str("DENY"),
            EventResult::Default { .. } => f.write_str("DEFAULT"),
            EventResult::Allow { .. } => f.write_str("ALLOW"),
        }
    }
}

impl<'mc> EventResult<'mc> {
    pub fn value_of(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<String>,
    ) -> Result<EventResult<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(env.new_string(arg0.into())?);
        let cls = env.find_class("org/bukkit/event/Event$Result");
        let cls = env.translate_error_with_class(cls)?;
        let res = env.call_static_method(
            cls,
            "valueOf",
            "(Ljava/lang/String;)Lorg/bukkit/event/Event$Result;",
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = env.translate_error(res)?;
        let obj = res.l()?;
        let variant = env.call_method(&obj, "toString", "()Ljava/lang/String;", vec![]);
        let variant = env.translate_error(variant)?;
        let variant_str = env
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        match variant_str.as_str() {
            "DENY" => Ok(EventResult::Deny {
                inner: EventResultStruct::from_raw(env, obj)?,
            }),
            "DEFAULT" => Ok(EventResult::Default {
                inner: EventResultStruct::from_raw(env, obj)?,
            }),
            "ALLOW" => Ok(EventResult::Allow {
                inner: EventResultStruct::from_raw(env, obj)?,
            }),

            _ => Err(eyre::eyre!("String gaven for variant was invalid").into()),
        }
    }
}

#[repr(C)]
pub struct EventResultStruct<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for EventResult<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        match self {
            Self::Deny { inner } => inner.0.clone(),
            Self::Default { inner } => inner.0.clone(),
            Self::Allow { inner } => inner.0.clone(),
        }
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        match self {
            Self::Deny { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Default { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Allow { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
        }
    }
}
impl<'mc> JNIInstantiatable<'mc> for EventResult<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate EventResult from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/event/Event$Result")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a EventResult object, got {}",
                name
            )
            .into())
        } else {
            let variant = env.call_method(&obj, "toString", "()Ljava/lang/String;", vec![]);
            let variant = env.translate_error(variant)?;
            let variant_str = env
                .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
                .to_string_lossy()
                .to_string();
            match variant_str.as_str() {
                "DENY" => Ok(EventResult::Deny {
                    inner: EventResultStruct::from_raw(env, obj)?,
                }),
                "DEFAULT" => Ok(EventResult::Default {
                    inner: EventResultStruct::from_raw(env, obj)?,
                }),
                "ALLOW" => Ok(EventResult::Allow {
                    inner: EventResultStruct::from_raw(env, obj)?,
                }),
                _ => Err(eyre::eyre!("String gaven for variant was invalid").into()),
            }
        }
    }
}

impl<'mc> JNIRaw<'mc> for EventResultStruct<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for EventResultStruct<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate EventResultStruct from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/event/Event$Result")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a EventResultStruct object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> EventResultStruct<'mc> {
    // SUPER CLASS: Enum

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
/// A list of event handlers, stored per-event. Based on lahwran's fevents.
#[repr(C)]
pub struct HandlerList<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for HandlerList<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for HandlerList<'mc> {
    fn from_raw(
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
}

impl<'mc> HandlerList<'mc> {
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let cls = jni.find_class("org/bukkit/event/HandlerList");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(cls, sig.as_str(), vec![]);
        let res = jni.translate_error_no_gen(res)?;
        crate::event::HandlerList::from_raw(&jni, res)
    }

    pub fn unregister_with_plugin(
        &self,
        arg0: impl Into<crate::plugin::Plugin<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/plugin/Plugin;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        sig += ")V";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "unregister", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn unregister_with_listener(
        &self,
        arg0: impl Into<crate::event::Listener<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/event/Listener;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        sig += ")V";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "unregister", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn unregister_all_with_listener(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: std::option::Option<impl Into<crate::event::Listener<'mc>>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "Lorg/bukkit/event/Listener;";
            let val_1 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(a.into().jni_object().clone())
            });
            args.push(val_1);
        }
        sig += ")V";
        let cls = jni.find_class("void");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "unregisterAll", sig.as_str(), args);
        jni.translate_error(res)?;
        Ok(())
    }

    pub fn bake_all(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let cls = jni.find_class("void");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "bakeAll", sig.as_str(), vec![]);
        jni.translate_error(res)?;
        Ok(())
    }

    pub fn bake(&self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "bake", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn get_registered_listeners(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<crate::plugin::Plugin<'mc>>,
    ) -> Result<blackboxmc_java::util::JavaArrayList<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/plugin/Plugin;)Ljava/util/ArrayList;");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let cls = jni.find_class("java/util/ArrayList");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(
            cls,
            "getRegisteredListeners",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        blackboxmc_java::util::JavaArrayList::from_raw(&jni, obj)
    }

    pub fn handler_lists(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<blackboxmc_java::util::JavaArrayList<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/ArrayList;");
        let cls = jni.find_class("java/util/ArrayList");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "getHandlerLists", sig.as_str(), vec![]);
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        blackboxmc_java::util::JavaArrayList::from_raw(&jni, obj)
    }

    pub fn register(
        &self,
        arg0: impl Into<crate::plugin::RegisteredListener<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/plugin/RegisteredListener;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "register",
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
/// Simple interface for tagging all EventListeners
///
/// This is a representation of an abstract class.
#[repr(C)]
pub struct Listener<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for Listener<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for Listener<'mc> {
    fn from_raw(
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

impl<'mc> Listener<'mc> {
    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
/// A type characterizing events that may be cancelled by a plugin or the server.
///
/// This is a representation of an abstract class.
#[repr(C)]
pub struct Cancellable<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for Cancellable<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for Cancellable<'mc> {
    fn from_raw(
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
}

impl<'mc> Cancellable<'mc> {
    pub fn is_cancelled(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isCancelled", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Sets the cancellation state of this event. A cancelled event will not be executed in the server, but will still pass to other plugins.
    pub fn set_cancelled(&self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
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

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
/// An annotation to mark methods as being event handler methods
///
/// This is a representation of an abstract class.
#[repr(C)]
pub struct EventHandler<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for EventHandler<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for EventHandler<'mc> {
    fn from_raw(
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
}

impl<'mc> EventHandler<'mc> {
    pub fn ignore_cancelled(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "ignoreCancelled", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn priority(&self) -> Result<crate::event::EventPriority<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/event/EventPriority;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "priority", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::event::EventPriority::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
/// Represents an event. All events require a static method named getHandlerList() which returns the same <a href="HandlerList.html" title="class in org.bukkit.event"><code>HandlerList</code></a> as <a href="#getHandlers()"><code>getHandlers()</code></a>.
#[repr(C)]
pub struct Event<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for Event<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for Event<'mc> {
    fn from_raw(
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
}

impl<'mc> Event<'mc> {
    pub fn new_with_boolean(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: std::option::Option<bool>,
    ) -> Result<crate::event::Event<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "Z";
            let val_1 = jni::objects::JValueGen::Bool(a.into());
            args.push(val_1);
        }
        sig += ")V";
        let cls = jni.find_class("org/bukkit/event/Event");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(cls, sig.as_str(), args);
        let res = jni.translate_error_no_gen(res)?;
        crate::event::Event::from_raw(&jni, res)
    }

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

    pub fn is_asynchronous(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isAsynchronous", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    // SUPER CLASS: Object

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
pub enum EventPriority<'mc> {
    Lowest { inner: EventPriorityStruct<'mc> },
    Low { inner: EventPriorityStruct<'mc> },
    Normal { inner: EventPriorityStruct<'mc> },
    High { inner: EventPriorityStruct<'mc> },
    Highest { inner: EventPriorityStruct<'mc> },
    Monitor { inner: EventPriorityStruct<'mc> },
}
impl<'mc> std::fmt::Display for EventPriority<'mc> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EventPriority::Lowest { .. } => f.write_str("LOWEST"),
            EventPriority::Low { .. } => f.write_str("LOW"),
            EventPriority::Normal { .. } => f.write_str("NORMAL"),
            EventPriority::High { .. } => f.write_str("HIGH"),
            EventPriority::Highest { .. } => f.write_str("HIGHEST"),
            EventPriority::Monitor { .. } => f.write_str("MONITOR"),
        }
    }
}

impl<'mc> EventPriority<'mc> {
    pub fn value_of(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<String>,
    ) -> Result<EventPriority<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(env.new_string(arg0.into())?);
        let cls = env.find_class("org/bukkit/event/EventPriority");
        let cls = env.translate_error_with_class(cls)?;
        let res = env.call_static_method(
            cls,
            "valueOf",
            "(Ljava/lang/String;)Lorg/bukkit/event/EventPriority;",
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = env.translate_error(res)?;
        let obj = res.l()?;
        let variant = env.call_method(&obj, "toString", "()Ljava/lang/String;", vec![]);
        let variant = env.translate_error(variant)?;
        let variant_str = env
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        match variant_str.as_str() {
            "LOWEST" => Ok(EventPriority::Lowest {
                inner: EventPriorityStruct::from_raw(env, obj)?,
            }),
            "LOW" => Ok(EventPriority::Low {
                inner: EventPriorityStruct::from_raw(env, obj)?,
            }),
            "NORMAL" => Ok(EventPriority::Normal {
                inner: EventPriorityStruct::from_raw(env, obj)?,
            }),
            "HIGH" => Ok(EventPriority::High {
                inner: EventPriorityStruct::from_raw(env, obj)?,
            }),
            "HIGHEST" => Ok(EventPriority::Highest {
                inner: EventPriorityStruct::from_raw(env, obj)?,
            }),
            "MONITOR" => Ok(EventPriority::Monitor {
                inner: EventPriorityStruct::from_raw(env, obj)?,
            }),

            _ => Err(eyre::eyre!("String gaven for variant was invalid").into()),
        }
    }
}

#[repr(C)]
pub struct EventPriorityStruct<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for EventPriority<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        match self {
            Self::Lowest { inner } => inner.0.clone(),
            Self::Low { inner } => inner.0.clone(),
            Self::Normal { inner } => inner.0.clone(),
            Self::High { inner } => inner.0.clone(),
            Self::Highest { inner } => inner.0.clone(),
            Self::Monitor { inner } => inner.0.clone(),
        }
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        match self {
            Self::Lowest { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Low { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Normal { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::High { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Highest { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Monitor { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
        }
    }
}
impl<'mc> JNIInstantiatable<'mc> for EventPriority<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
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
            let variant = env.call_method(&obj, "toString", "()Ljava/lang/String;", vec![]);
            let variant = env.translate_error(variant)?;
            let variant_str = env
                .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
                .to_string_lossy()
                .to_string();
            match variant_str.as_str() {
                "LOWEST" => Ok(EventPriority::Lowest {
                    inner: EventPriorityStruct::from_raw(env, obj)?,
                }),
                "LOW" => Ok(EventPriority::Low {
                    inner: EventPriorityStruct::from_raw(env, obj)?,
                }),
                "NORMAL" => Ok(EventPriority::Normal {
                    inner: EventPriorityStruct::from_raw(env, obj)?,
                }),
                "HIGH" => Ok(EventPriority::High {
                    inner: EventPriorityStruct::from_raw(env, obj)?,
                }),
                "HIGHEST" => Ok(EventPriority::Highest {
                    inner: EventPriorityStruct::from_raw(env, obj)?,
                }),
                "MONITOR" => Ok(EventPriority::Monitor {
                    inner: EventPriorityStruct::from_raw(env, obj)?,
                }),
                _ => Err(eyre::eyre!("String gaven for variant was invalid").into()),
            }
        }
    }
}

impl<'mc> JNIRaw<'mc> for EventPriorityStruct<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for EventPriorityStruct<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate EventPriorityStruct from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/event/EventPriority")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a EventPriorityStruct object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> EventPriorityStruct<'mc> {
    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
pub enum SpigotResult<'mc> {
    Deny { inner: SpigotResultStruct<'mc> },
    Default { inner: SpigotResultStruct<'mc> },
    Allow { inner: SpigotResultStruct<'mc> },
}
impl<'mc> std::fmt::Display for SpigotResult<'mc> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SpigotResult::Deny { .. } => f.write_str("DENY"),
            SpigotResult::Default { .. } => f.write_str("DEFAULT"),
            SpigotResult::Allow { .. } => f.write_str("ALLOW"),
        }
    }
}

impl<'mc> SpigotResult<'mc> {
    pub fn value_of(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<String>,
    ) -> Result<SpigotResult<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(env.new_string(arg0.into())?);
        let cls = env.find_class("org/bukkit/event/SpigotResult");
        let cls = env.translate_error_with_class(cls)?;
        let res = env.call_static_method(
            cls,
            "valueOf",
            "(Ljava/lang/String;)Lorg/bukkit/event/SpigotResult;",
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = env.translate_error(res)?;
        let obj = res.l()?;
        let variant = env.call_method(&obj, "toString", "()Ljava/lang/String;", vec![]);
        let variant = env.translate_error(variant)?;
        let variant_str = env
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        match variant_str.as_str() {
            "DENY" => Ok(SpigotResult::Deny {
                inner: SpigotResultStruct::from_raw(env, obj)?,
            }),
            "DEFAULT" => Ok(SpigotResult::Default {
                inner: SpigotResultStruct::from_raw(env, obj)?,
            }),
            "ALLOW" => Ok(SpigotResult::Allow {
                inner: SpigotResultStruct::from_raw(env, obj)?,
            }),

            _ => Err(eyre::eyre!("String gaven for variant was invalid").into()),
        }
    }
}

#[repr(C)]
pub struct SpigotResultStruct<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for SpigotResult<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        match self {
            Self::Deny { inner } => inner.0.clone(),
            Self::Default { inner } => inner.0.clone(),
            Self::Allow { inner } => inner.0.clone(),
        }
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        match self {
            Self::Deny { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Default { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Allow { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
        }
    }
}
impl<'mc> JNIInstantiatable<'mc> for SpigotResult<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate SpigotResult from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/event/SpigotResult")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a SpigotResult object, got {}",
                name
            )
            .into())
        } else {
            let variant = env.call_method(&obj, "toString", "()Ljava/lang/String;", vec![]);
            let variant = env.translate_error(variant)?;
            let variant_str = env
                .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
                .to_string_lossy()
                .to_string();
            match variant_str.as_str() {
                "DENY" => Ok(SpigotResult::Deny {
                    inner: SpigotResultStruct::from_raw(env, obj)?,
                }),
                "DEFAULT" => Ok(SpigotResult::Default {
                    inner: SpigotResultStruct::from_raw(env, obj)?,
                }),
                "ALLOW" => Ok(SpigotResult::Allow {
                    inner: SpigotResultStruct::from_raw(env, obj)?,
                }),
                _ => Err(eyre::eyre!("String gaven for variant was invalid").into()),
            }
        }
    }
}

impl<'mc> JNIRaw<'mc> for SpigotResultStruct<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for SpigotResultStruct<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate SpigotResultStruct from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/event/SpigotResult")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a SpigotResultStruct object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> SpigotResultStruct<'mc> {
    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
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
