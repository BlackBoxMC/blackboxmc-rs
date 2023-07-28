#![allow(deprecated)]
use blackboxmc_general::JNIRaw;
use color_eyre::eyre::Result;
/// An instantiatable struct that implements JavaCondition. Needed for returning it from Java.
pub struct JavaCondition<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> JavaCondition<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate JavaCondition from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "JavaCondition")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a JavaCondition object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}
impl<'mc> JNIRaw<'mc> for JavaCondition<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
pub struct JavaReentrantLock<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for JavaReentrantLock<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JavaReentrantLock<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate JavaReentrantLock from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "JavaReentrantLock")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a JavaReentrantLock object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}
impl<'mc> Into<crate::concurrent::locks::JavaLock<'mc>> for JavaReentrantLock<'mc> {
    fn into(self) -> crate::concurrent::locks::JavaLock<'mc> {
        crate::concurrent::locks::JavaLock::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
pub struct JavaAbstractQueuedSynchronizer<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for JavaAbstractQueuedSynchronizer<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JavaAbstractQueuedSynchronizer<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate JavaAbstractQueuedSynchronizer from null object."
            )
            .into());
        }
        let (valid, name) = env.validate_name(&obj, "JavaAbstractQueuedSynchronizer")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a JavaAbstractQueuedSynchronizer object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}
impl<'mc> Into<crate::concurrent::locks::JavaAbstractOwnableSynchronizer<'mc>>
    for JavaAbstractQueuedSynchronizer<'mc>
{
    fn into(self) -> crate::concurrent::locks::JavaAbstractOwnableSynchronizer<'mc> {
        crate::concurrent::locks::JavaAbstractOwnableSynchronizer::from_raw(&self.jni_ref(), self.1)
            .unwrap()
    }
}
pub struct JavaReentrantReadWriteLockWriteLock<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for JavaReentrantReadWriteLockWriteLock<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JavaReentrantReadWriteLockWriteLock<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate JavaReentrantReadWriteLockWriteLock from null object."
            )
            .into());
        }
        let (valid, name) = env.validate_name(&obj, "JavaReentrantReadWriteLockWriteLock")?;
        if !valid {
            Err(eyre::eyre!(
        "Invalid argument passed. Expected a JavaReentrantReadWriteLockWriteLock object, got {}",
        name
    )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}
impl<'mc> Into<crate::concurrent::locks::JavaLock<'mc>>
    for JavaReentrantReadWriteLockWriteLock<'mc>
{
    fn into(self) -> crate::concurrent::locks::JavaLock<'mc> {
        crate::concurrent::locks::JavaLock::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
pub struct JavaReentrantReadWriteLockReadLock<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for JavaReentrantReadWriteLockReadLock<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JavaReentrantReadWriteLockReadLock<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate JavaReentrantReadWriteLockReadLock from null object."
            )
            .into());
        }
        let (valid, name) = env.validate_name(&obj, "JavaReentrantReadWriteLockReadLock")?;
        if !valid {
            Err(eyre::eyre!(
        "Invalid argument passed. Expected a JavaReentrantReadWriteLockReadLock object, got {}",
        name
    )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}
impl<'mc> Into<crate::concurrent::locks::JavaLock<'mc>>
    for JavaReentrantReadWriteLockReadLock<'mc>
{
    fn into(self) -> crate::concurrent::locks::JavaLock<'mc> {
        crate::concurrent::locks::JavaLock::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
pub struct JavaReentrantReadWriteLock<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for JavaReentrantReadWriteLock<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JavaReentrantReadWriteLock<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate JavaReentrantReadWriteLock from null object."
            )
            .into());
        }
        let (valid, name) = env.validate_name(&obj, "JavaReentrantReadWriteLock")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a JavaReentrantReadWriteLock object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}
impl<'mc> Into<crate::concurrent::locks::JavaReadWriteLock<'mc>>
    for JavaReentrantReadWriteLock<'mc>
{
    fn into(self) -> crate::concurrent::locks::JavaReadWriteLock<'mc> {
        crate::concurrent::locks::JavaReadWriteLock::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
pub struct JavaAbstractQueuedLongSynchronizerConditionObject<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> blackboxmc_general::JNIRaw<'mc>
    for JavaAbstractQueuedLongSynchronizerConditionObject<'mc>
{
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JavaAbstractQueuedLongSynchronizerConditionObject<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!(
        "Tried to instantiate JavaAbstractQueuedLongSynchronizerConditionObject from null object.")
                .into(),
            );
        }
        let (valid, name) =
            env.validate_name(&obj, "JavaAbstractQueuedLongSynchronizerConditionObject")?;
        if !valid {
            Err(eyre::eyre!(
        "Invalid argument passed. Expected a JavaAbstractQueuedLongSynchronizerConditionObject object, got {}",
        name
    )
    .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}
impl<'mc> Into<crate::concurrent::locks::JavaCondition<'mc>>
    for JavaAbstractQueuedLongSynchronizerConditionObject<'mc>
{
    fn into(self) -> crate::concurrent::locks::JavaCondition<'mc> {
        crate::concurrent::locks::JavaCondition::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
/// An instantiatable struct that implements JavaLock. Needed for returning it from Java.
pub struct JavaLock<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> JavaLock<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate JavaLock from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "JavaLock")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a JavaLock object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}
impl<'mc> JNIRaw<'mc> for JavaLock<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
pub struct JavaAbstractQueuedSynchronizerConditionObject<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for JavaAbstractQueuedSynchronizerConditionObject<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JavaAbstractQueuedSynchronizerConditionObject<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
        "Tried to instantiate JavaAbstractQueuedSynchronizerConditionObject from null object.")
            .into());
        }
        let (valid, name) =
            env.validate_name(&obj, "JavaAbstractQueuedSynchronizerConditionObject")?;
        if !valid {
            Err(eyre::eyre!(
        "Invalid argument passed. Expected a JavaAbstractQueuedSynchronizerConditionObject object, got {}",
        name
    )
    .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}
impl<'mc> Into<crate::concurrent::locks::JavaCondition<'mc>>
    for JavaAbstractQueuedSynchronizerConditionObject<'mc>
{
    fn into(self) -> crate::concurrent::locks::JavaCondition<'mc> {
        crate::concurrent::locks::JavaCondition::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
pub struct JavaAbstractOwnableSynchronizer<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for JavaAbstractOwnableSynchronizer<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JavaAbstractOwnableSynchronizer<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate JavaAbstractOwnableSynchronizer from null object."
            )
            .into());
        }
        let (valid, name) = env.validate_name(&obj, "JavaAbstractOwnableSynchronizer")?;
        if !valid {
            Err(eyre::eyre!(
        "Invalid argument passed. Expected a JavaAbstractOwnableSynchronizer object, got {}",
        name
    )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}
/// An instantiatable struct that implements JavaReadWriteLock. Needed for returning it from Java.
pub struct JavaReadWriteLock<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> JavaReadWriteLock<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate JavaReadWriteLock from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "JavaReadWriteLock")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a JavaReadWriteLock object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}
impl<'mc> JNIRaw<'mc> for JavaReadWriteLock<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
pub struct JavaAbstractQueuedLongSynchronizer<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for JavaAbstractQueuedLongSynchronizer<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JavaAbstractQueuedLongSynchronizer<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate JavaAbstractQueuedLongSynchronizer from null object."
            )
            .into());
        }
        let (valid, name) = env.validate_name(&obj, "JavaAbstractQueuedLongSynchronizer")?;
        if !valid {
            Err(eyre::eyre!(
        "Invalid argument passed. Expected a JavaAbstractQueuedLongSynchronizer object, got {}",
        name
    )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}
impl<'mc> Into<crate::concurrent::locks::JavaAbstractOwnableSynchronizer<'mc>>
    for JavaAbstractQueuedLongSynchronizer<'mc>
{
    fn into(self) -> crate::concurrent::locks::JavaAbstractOwnableSynchronizer<'mc> {
        crate::concurrent::locks::JavaAbstractOwnableSynchronizer::from_raw(&self.jni_ref(), self.1)
            .unwrap()
    }
}
pub struct JavaLockSupport<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for JavaLockSupport<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JavaLockSupport<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate JavaLockSupport from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "JavaLockSupport")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a JavaLockSupport object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}
pub struct JavaStampedLock<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for JavaStampedLock<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JavaStampedLock<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate JavaStampedLock from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "JavaStampedLock")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a JavaStampedLock object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}
