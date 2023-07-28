#![allow(deprecated)]
use blackboxmc_general::JNIRaw;
use color_eyre::eyre::Result;
/// An instantiatable struct that implements JavaBlockingQueue. Needed for returning it from Java.
pub struct JavaBlockingQueue<'mc, E>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
)
where
    E: JNIRaw<'mc>;
impl<'mc, E> JavaBlockingQueue<'mc, E>
where
    E: JNIRaw<'mc>,
{
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate JavaBlockingQueue from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "JavaBlockingQueue")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a JavaBlockingQueue object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}
impl<'mc, E> JNIRaw<'mc> for JavaBlockingQueue<'mc, E>
where
    E: JNIRaw<'mc>,
{
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
/// An instantiatable struct that implements JavaThreadFactory. Needed for returning it from Java.
pub struct JavaThreadFactory<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> JavaThreadFactory<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate JavaThreadFactory from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "JavaThreadFactory")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a JavaThreadFactory object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}
impl<'mc> JNIRaw<'mc> for JavaThreadFactory<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
/// An instantiatable struct that implements JavaScheduledExecutorService. Needed for returning it from Java.
pub struct JavaScheduledExecutorService<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> JavaScheduledExecutorService<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate JavaScheduledExecutorService from null object."
            )
            .into());
        }
        let (valid, name) = env.validate_name(&obj, "JavaScheduledExecutorService")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a JavaScheduledExecutorService object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}
impl<'mc> JNIRaw<'mc> for JavaScheduledExecutorService<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Into<crate::concurrent::JavaExecutorService<'mc>> for JavaScheduledExecutorService<'mc> {
    fn into(self) -> crate::concurrent::JavaExecutorService<'mc> {
        crate::concurrent::JavaExecutorService::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
/// An instantiatable struct that implements JavaDelayed. Needed for returning it from Java.
pub struct JavaDelayed<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> JavaDelayed<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate JavaDelayed from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "JavaDelayed")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a JavaDelayed object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}
impl<'mc> JNIRaw<'mc> for JavaDelayed<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
/// An instantiatable struct that implements JavaRejectedExecutionHandler. Needed for returning it from Java.
pub struct JavaRejectedExecutionHandler<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> JavaRejectedExecutionHandler<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate JavaRejectedExecutionHandler from null object."
            )
            .into());
        }
        let (valid, name) = env.validate_name(&obj, "JavaRejectedExecutionHandler")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a JavaRejectedExecutionHandler object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}
impl<'mc> JNIRaw<'mc> for JavaRejectedExecutionHandler<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
/// An instantiatable struct that implements JavaConcurrentMap. Needed for returning it from Java.
pub struct JavaConcurrentMap<'mc, K, V>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
)
where
    K: JNIRaw<'mc>,
    V: JNIRaw<'mc>;
impl<'mc, K, V> JavaConcurrentMap<'mc, K, V>
where
    K: JNIRaw<'mc>,
    V: JNIRaw<'mc>,
{
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate JavaConcurrentMap from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "JavaConcurrentMap")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a JavaConcurrentMap object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}
impl<'mc, K, V> JNIRaw<'mc> for JavaConcurrentMap<'mc, K, V>
where
    K: JNIRaw<'mc>,
    V: JNIRaw<'mc>,
{
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
/// An instantiatable struct that implements JavaForkJoinPoolManagedBlocker. Needed for returning it from Java.
pub struct JavaForkJoinPoolManagedBlocker<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> JavaForkJoinPoolManagedBlocker<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate JavaForkJoinPoolManagedBlocker from null object."
            )
            .into());
        }
        let (valid, name) = env.validate_name(&obj, "JavaForkJoinPoolManagedBlocker")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a JavaForkJoinPoolManagedBlocker object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}
impl<'mc> JNIRaw<'mc> for JavaForkJoinPoolManagedBlocker<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
/// An instantiatable struct that implements JavaTransferQueue. Needed for returning it from Java.
pub struct JavaTransferQueue<'mc, E>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
)
where
    E: JNIRaw<'mc>;
impl<'mc, E> JavaTransferQueue<'mc, E>
where
    E: JNIRaw<'mc>,
{
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate JavaTransferQueue from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "JavaTransferQueue")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a JavaTransferQueue object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}
impl<'mc, E> JNIRaw<'mc> for JavaTransferQueue<'mc, E>
where
    E: JNIRaw<'mc>,
{
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
/// An instantiatable struct that implements JavaCallable. Needed for returning it from Java.
pub struct JavaCallable<'mc, V>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
)
where
    V: JNIRaw<'mc>;
impl<'mc, V> JavaCallable<'mc, V>
where
    V: JNIRaw<'mc>,
{
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate JavaCallable from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "JavaCallable")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a JavaCallable object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}
impl<'mc, V> JNIRaw<'mc> for JavaCallable<'mc, V>
where
    V: JNIRaw<'mc>,
{
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
/// An instantiatable struct that implements JavaScheduledFuture. Needed for returning it from Java.
pub struct JavaScheduledFuture<'mc, V>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
)
where
    V: JNIRaw<'mc>;
impl<'mc, V> JavaScheduledFuture<'mc, V>
where
    V: JNIRaw<'mc>,
{
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate JavaScheduledFuture from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "JavaScheduledFuture")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a JavaScheduledFuture object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}
impl<'mc, V> JNIRaw<'mc> for JavaScheduledFuture<'mc, V>
where
    V: JNIRaw<'mc>,
{
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc, V> Into<crate::concurrent::JavaDelayed<'mc>> for JavaScheduledFuture<'mc, V>
where
    V: JNIRaw<'mc>,
{
    fn into(self) -> crate::concurrent::JavaDelayed<'mc> {
        crate::concurrent::JavaDelayed::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
/// An instantiatable struct that implements JavaCompletableFutureAsynchronousCompletionTask. Needed for returning it from Java.
pub struct JavaCompletableFutureAsynchronousCompletionTask<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> JavaCompletableFutureAsynchronousCompletionTask<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
        "Tried to instantiate JavaCompletableFutureAsynchronousCompletionTask from null object.")
            .into());
        }
        let (valid, name) =
            env.validate_name(&obj, "JavaCompletableFutureAsynchronousCompletionTask")?;
        if !valid {
            Err(eyre::eyre!(
        "Invalid argument passed. Expected a JavaCompletableFutureAsynchronousCompletionTask object, got {}",
        name
    )
    .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}
impl<'mc> JNIRaw<'mc> for JavaCompletableFutureAsynchronousCompletionTask<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
/// An instantiatable struct that implements JavaRunnableScheduledFuture. Needed for returning it from Java.
pub struct JavaRunnableScheduledFuture<'mc, V>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
)
where
    V: JNIRaw<'mc>;
impl<'mc, V> JavaRunnableScheduledFuture<'mc, V>
where
    V: JNIRaw<'mc>,
{
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate JavaRunnableScheduledFuture from null object."
            )
            .into());
        }
        let (valid, name) = env.validate_name(&obj, "JavaRunnableScheduledFuture")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a JavaRunnableScheduledFuture object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}
impl<'mc, V> JNIRaw<'mc> for JavaRunnableScheduledFuture<'mc, V>
where
    V: JNIRaw<'mc>,
{
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
pub struct JavaCompletableFuture<'mc, T>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
)
where
    T: JNIRaw<'mc>;
impl<'mc, T> blackboxmc_general::JNIRaw<'mc> for JavaCompletableFuture<'mc, T>
where
    T: JNIRaw<'mc>,
{
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc, T> JavaCompletableFuture<'mc, T>
where
    T: JNIRaw<'mc>,
{
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate JavaCompletableFuture from null object."
            )
            .into());
        }
        let (valid, name) = env.validate_name(&obj, "JavaCompletableFuture")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a JavaCompletableFuture object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}
/// An instantiatable struct that implements JavaExecutor. Needed for returning it from Java.
pub struct JavaExecutor<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> JavaExecutor<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate JavaExecutor from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "JavaExecutor")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a JavaExecutor object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}
impl<'mc> JNIRaw<'mc> for JavaExecutor<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
/// An instantiatable struct that implements JavaBlockingDeque. Needed for returning it from Java.
pub struct JavaBlockingDeque<'mc, E>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
)
where
    E: JNIRaw<'mc>;
impl<'mc, E> JavaBlockingDeque<'mc, E>
where
    E: JNIRaw<'mc>,
{
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate JavaBlockingDeque from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "JavaBlockingDeque")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a JavaBlockingDeque object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}
impl<'mc, E> JNIRaw<'mc> for JavaBlockingDeque<'mc, E>
where
    E: JNIRaw<'mc>,
{
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
/// An instantiatable struct that implements JavaCompletionService. Needed for returning it from Java.
pub struct JavaCompletionService<'mc, V>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
)
where
    V: JNIRaw<'mc>;
impl<'mc, V> JavaCompletionService<'mc, V>
where
    V: JNIRaw<'mc>,
{
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate JavaCompletionService from null object."
            )
            .into());
        }
        let (valid, name) = env.validate_name(&obj, "JavaCompletionService")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a JavaCompletionService object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}
impl<'mc, V> JNIRaw<'mc> for JavaCompletionService<'mc, V>
where
    V: JNIRaw<'mc>,
{
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
/// An instantiatable struct that implements JavaExecutorService. Needed for returning it from Java.
pub struct JavaExecutorService<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> JavaExecutorService<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate JavaExecutorService from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "JavaExecutorService")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a JavaExecutorService object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}
impl<'mc> JNIRaw<'mc> for JavaExecutorService<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Into<crate::concurrent::JavaExecutor<'mc>> for JavaExecutorService<'mc> {
    fn into(self) -> crate::concurrent::JavaExecutor<'mc> {
        crate::concurrent::JavaExecutor::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
/// An instantiatable struct that implements JavaFuture. Needed for returning it from Java.
pub struct JavaFuture<'mc, V>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
)
where
    V: JNIRaw<'mc>;
impl<'mc, V> JavaFuture<'mc, V>
where
    V: JNIRaw<'mc>,
{
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate JavaFuture from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "JavaFuture")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a JavaFuture object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}
impl<'mc, V> JNIRaw<'mc> for JavaFuture<'mc, V>
where
    V: JNIRaw<'mc>,
{
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
/// An instantiatable struct that implements JavaForkJoinPoolForkJoinWorkerThreadFactory. Needed for returning it from Java.
pub struct JavaForkJoinPoolForkJoinWorkerThreadFactory<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> JavaForkJoinPoolForkJoinWorkerThreadFactory<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
        "Tried to instantiate JavaForkJoinPoolForkJoinWorkerThreadFactory from null object.")
            .into());
        }
        let (valid, name) =
            env.validate_name(&obj, "JavaForkJoinPoolForkJoinWorkerThreadFactory")?;
        if !valid {
            Err(eyre::eyre!(
        "Invalid argument passed. Expected a JavaForkJoinPoolForkJoinWorkerThreadFactory object, got {}",
        name
    )
    .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}
impl<'mc> JNIRaw<'mc> for JavaForkJoinPoolForkJoinWorkerThreadFactory<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
pub struct JavaTimeUnit<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for JavaTimeUnit<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JavaTimeUnit<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate JavaTimeUnit from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "JavaTimeUnit")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a JavaTimeUnit object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}
/// An instantiatable struct that implements JavaRunnableFuture. Needed for returning it from Java.
pub struct JavaRunnableFuture<'mc, V>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
)
where
    V: JNIRaw<'mc>;
impl<'mc, V> JavaRunnableFuture<'mc, V>
where
    V: JNIRaw<'mc>,
{
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate JavaRunnableFuture from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "JavaRunnableFuture")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a JavaRunnableFuture object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}
impl<'mc, V> JNIRaw<'mc> for JavaRunnableFuture<'mc, V>
where
    V: JNIRaw<'mc>,
{
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
/// An instantiatable struct that implements JavaCompletionStage. Needed for returning it from Java.
pub struct JavaCompletionStage<'mc, T>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
)
where
    T: JNIRaw<'mc>;
impl<'mc, T> JavaCompletionStage<'mc, T>
where
    T: JNIRaw<'mc>,
{
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate JavaCompletionStage from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "JavaCompletionStage")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a JavaCompletionStage object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}
impl<'mc, T> JNIRaw<'mc> for JavaCompletionStage<'mc, T>
where
    T: JNIRaw<'mc>,
{
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
/// An instantiatable struct that implements JavaConcurrentNavigableMap. Needed for returning it from Java.
pub struct JavaConcurrentNavigableMap<'mc, K, V>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
)
where
    K: JNIRaw<'mc>,
    V: JNIRaw<'mc>;
impl<'mc, K, V> JavaConcurrentNavigableMap<'mc, K, V>
where
    K: JNIRaw<'mc>,
    V: JNIRaw<'mc>,
{
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate JavaConcurrentNavigableMap from null object."
            )
            .into());
        }
        let (valid, name) = env.validate_name(&obj, "JavaConcurrentNavigableMap")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a JavaConcurrentNavigableMap object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}
impl<'mc, K, V> JNIRaw<'mc> for JavaConcurrentNavigableMap<'mc, K, V>
where
    K: JNIRaw<'mc>,
    V: JNIRaw<'mc>,
{
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
pub mod atomic;
pub mod locks;
