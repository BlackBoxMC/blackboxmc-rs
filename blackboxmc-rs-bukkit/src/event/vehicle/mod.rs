#![allow(deprecated)]
use blackboxmc_general::JNIRaw;
use color_eyre::eyre::Result;
pub struct VehicleUpdateEvent<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for VehicleUpdateEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> VehicleUpdateEvent<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate VehicleUpdateEvent from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "VehicleUpdateEvent")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a VehicleUpdateEvent object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}
impl<'mc> Into<crate::event::vehicle::VehicleEvent<'mc>> for VehicleUpdateEvent<'mc> {
    fn into(self) -> crate::event::vehicle::VehicleEvent<'mc> {
        crate::event::vehicle::VehicleEvent::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
pub struct VehicleEnterEvent<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for VehicleEnterEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> VehicleEnterEvent<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate VehicleEnterEvent from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "VehicleEnterEvent")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a VehicleEnterEvent object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}
impl<'mc> Into<crate::event::Cancellable<'mc>> for VehicleEnterEvent<'mc> {
    fn into(self) -> crate::event::Cancellable<'mc> {
        crate::event::Cancellable::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
impl<'mc> Into<crate::event::vehicle::VehicleEvent<'mc>> for VehicleEnterEvent<'mc> {
    fn into(self) -> crate::event::vehicle::VehicleEvent<'mc> {
        crate::event::vehicle::VehicleEvent::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
pub struct VehicleEntityCollisionEvent<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for VehicleEntityCollisionEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> VehicleEntityCollisionEvent<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate VehicleEntityCollisionEvent from null object."
            )
            .into());
        }
        let (valid, name) = env.validate_name(&obj, "VehicleEntityCollisionEvent")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a VehicleEntityCollisionEvent object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}
impl<'mc> Into<crate::event::Cancellable<'mc>> for VehicleEntityCollisionEvent<'mc> {
    fn into(self) -> crate::event::Cancellable<'mc> {
        crate::event::Cancellable::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
impl<'mc> Into<crate::event::vehicle::VehicleCollisionEvent<'mc>>
    for VehicleEntityCollisionEvent<'mc>
{
    fn into(self) -> crate::event::vehicle::VehicleCollisionEvent<'mc> {
        crate::event::vehicle::VehicleCollisionEvent::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
pub struct VehicleExitEvent<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for VehicleExitEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> VehicleExitEvent<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate VehicleExitEvent from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "VehicleExitEvent")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a VehicleExitEvent object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}
impl<'mc> Into<crate::event::Cancellable<'mc>> for VehicleExitEvent<'mc> {
    fn into(self) -> crate::event::Cancellable<'mc> {
        crate::event::Cancellable::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
impl<'mc> Into<crate::event::vehicle::VehicleEvent<'mc>> for VehicleExitEvent<'mc> {
    fn into(self) -> crate::event::vehicle::VehicleEvent<'mc> {
        crate::event::vehicle::VehicleEvent::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
pub struct VehicleCollisionEvent<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for VehicleCollisionEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> VehicleCollisionEvent<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate VehicleCollisionEvent from null object."
            )
            .into());
        }
        let (valid, name) = env.validate_name(&obj, "VehicleCollisionEvent")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a VehicleCollisionEvent object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}
impl<'mc> Into<crate::event::vehicle::VehicleEvent<'mc>> for VehicleCollisionEvent<'mc> {
    fn into(self) -> crate::event::vehicle::VehicleEvent<'mc> {
        crate::event::vehicle::VehicleEvent::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
pub struct VehicleBlockCollisionEvent<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for VehicleBlockCollisionEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> VehicleBlockCollisionEvent<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate VehicleBlockCollisionEvent from null object."
            )
            .into());
        }
        let (valid, name) = env.validate_name(&obj, "VehicleBlockCollisionEvent")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a VehicleBlockCollisionEvent object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}
impl<'mc> Into<crate::event::vehicle::VehicleCollisionEvent<'mc>>
    for VehicleBlockCollisionEvent<'mc>
{
    fn into(self) -> crate::event::vehicle::VehicleCollisionEvent<'mc> {
        crate::event::vehicle::VehicleCollisionEvent::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
pub struct VehicleCreateEvent<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for VehicleCreateEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> VehicleCreateEvent<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate VehicleCreateEvent from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "VehicleCreateEvent")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a VehicleCreateEvent object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}
impl<'mc> Into<crate::event::Cancellable<'mc>> for VehicleCreateEvent<'mc> {
    fn into(self) -> crate::event::Cancellable<'mc> {
        crate::event::Cancellable::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
impl<'mc> Into<crate::event::vehicle::VehicleEvent<'mc>> for VehicleCreateEvent<'mc> {
    fn into(self) -> crate::event::vehicle::VehicleEvent<'mc> {
        crate::event::vehicle::VehicleEvent::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
pub struct VehicleMoveEvent<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for VehicleMoveEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> VehicleMoveEvent<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate VehicleMoveEvent from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "VehicleMoveEvent")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a VehicleMoveEvent object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}
impl<'mc> Into<crate::event::vehicle::VehicleEvent<'mc>> for VehicleMoveEvent<'mc> {
    fn into(self) -> crate::event::vehicle::VehicleEvent<'mc> {
        crate::event::vehicle::VehicleEvent::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
pub struct VehicleDestroyEvent<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for VehicleDestroyEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> VehicleDestroyEvent<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate VehicleDestroyEvent from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "VehicleDestroyEvent")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a VehicleDestroyEvent object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}
impl<'mc> Into<crate::event::Cancellable<'mc>> for VehicleDestroyEvent<'mc> {
    fn into(self) -> crate::event::Cancellable<'mc> {
        crate::event::Cancellable::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
impl<'mc> Into<crate::event::vehicle::VehicleEvent<'mc>> for VehicleDestroyEvent<'mc> {
    fn into(self) -> crate::event::vehicle::VehicleEvent<'mc> {
        crate::event::vehicle::VehicleEvent::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
pub struct VehicleDamageEvent<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for VehicleDamageEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> VehicleDamageEvent<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate VehicleDamageEvent from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "VehicleDamageEvent")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a VehicleDamageEvent object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}
impl<'mc> Into<crate::event::Cancellable<'mc>> for VehicleDamageEvent<'mc> {
    fn into(self) -> crate::event::Cancellable<'mc> {
        crate::event::Cancellable::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
impl<'mc> Into<crate::event::vehicle::VehicleEvent<'mc>> for VehicleDamageEvent<'mc> {
    fn into(self) -> crate::event::vehicle::VehicleEvent<'mc> {
        crate::event::vehicle::VehicleEvent::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
pub struct VehicleEvent<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for VehicleEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> VehicleEvent<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate VehicleEvent from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "VehicleEvent")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a VehicleEvent object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}
impl<'mc> Into<crate::event::Event<'mc>> for VehicleEvent<'mc> {
    fn into(self) -> crate::event::Event<'mc> {
        crate::event::Event::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
