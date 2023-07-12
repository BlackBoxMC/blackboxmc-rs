use crate::JNIRaw;
/// An instantiatable struct that implements BukkitTask. Needed for returning it from Java.
pub struct BukkitTask<'mc>(
    pub(crate) jni::JNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> BukkitTask<'mc> {
    pub fn from_raw(env: jni::JNIEnv<'mc>, obj: jni::objects::JObject<'mc>) -> Self {
        Self(env, obj)
    }
    pub fn cancel(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.jni_ref()
            .call_method(&self.jni_object(), "cancel", "()V", &[])?;
        Ok(())
    }
    pub fn is_cancelled(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isCancelled", "()Z", &[])?;
        Ok(res.z().unwrap())
    }
    pub fn is_sync(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isSync", "()Z", &[])?;
        Ok(res.z().unwrap())
    }
    pub fn owner(
        &mut self,
    ) -> Result<crate::bukkit::plugin::Plugin<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getOwner",
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
    pub fn task_id(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getTaskId", "()I", &[])?;
        Ok(res.i().unwrap())
    }
}
impl<'mc> crate::JNIRaw<'mc> for BukkitTask<'mc> {
    fn jni_ref(&self) -> jni::JNIEnv<'mc> {
        unsafe { self.0.unsafe_clone() }
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
/// An instantiatable struct that implements BukkitWorker. Needed for returning it from Java.
pub struct BukkitWorker<'mc>(
    pub(crate) jni::JNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> BukkitWorker<'mc> {
    pub fn from_raw(env: jni::JNIEnv<'mc>, obj: jni::objects::JObject<'mc>) -> Self {
        Self(env, obj)
    }
    pub fn owner(
        &mut self,
    ) -> Result<crate::bukkit::plugin::Plugin<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getOwner",
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
    pub fn thread(&mut self) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getThread",
            "()Ljava/lang/Thread;",
            &[],
        )?;
        Ok(res.l().unwrap())
    }
    pub fn task_id(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getTaskId", "()I", &[])?;
        Ok(res.i().unwrap())
    }
}
impl<'mc> crate::JNIRaw<'mc> for BukkitWorker<'mc> {
    fn jni_ref(&self) -> jni::JNIEnv<'mc> {
        unsafe { self.0.unsafe_clone() }
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
/// An instantiatable struct that implements BukkitScheduler. Needed for returning it from Java.
pub struct BukkitScheduler<'mc>(
    pub(crate) jni::JNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> BukkitScheduler<'mc> {
    pub fn from_raw(env: jni::JNIEnv<'mc>, obj: jni::objects::JObject<'mc>) -> Self {
        Self(env, obj)
    }
    pub fn is_queued(&mut self, arg0: i32) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Int(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "isQueued",
            "(I)Z",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(res.z().unwrap())
    }
    pub fn schedule_async_delayed_task_with_plugin(
        &mut self,
        arg0: crate::bukkit::plugin::Plugin<'mc>,
        arg1: std::option::Option<jni::objects::JObject<'mc>>,
        arg2: std::option::Option<i64>,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.1.clone()) };
        let val_1 = arg1.unwrap();
        let val_2 = jni::objects::JValueGen::Long(arg2.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "scheduleAsyncDelayedTask",
            "(Lorg/bukkit/plugin/Plugin;Ljava/lang/Runnable;J)I",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        )?;
        Ok(res.i().unwrap())
    }
    pub fn schedule_async_repeating_task(
        &mut self,
        arg0: crate::bukkit::plugin::Plugin<'mc>,
        arg1: jni::objects::JObject<'mc>,
        arg2: i64,
        arg3: i64,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.1.clone()) };
        let val_1 = arg1;
        let val_2 = jni::objects::JValueGen::Long(arg2.into());
        let val_3 = jni::objects::JValueGen::Long(arg3.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "scheduleAsyncRepeatingTask",
            "(Lorg/bukkit/plugin/Plugin;Ljava/lang/Runnable;JJ)I",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
                jni::objects::JValueGen::from(&val_3),
            ],
        )?;
        Ok(res.i().unwrap())
    }
    pub fn call_sync_method(
        &mut self,
        arg0: crate::bukkit::plugin::Plugin<'mc>,
        arg1: jni::objects::JObject<'mc>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.1.clone()) };
        let val_1 = arg1;
        let res =
self.jni_ref().call_method(&self.jni_object(),"callSyncMethod","(Lorg/bukkit/plugin/Plugin;Ljava/util/concurrent/Callable;)Ljava/util/concurrent/Future;",&[jni::objects::JValueGen::from(&val_0),jni::objects::JValueGen::from(&val_1)])?;
        Ok(res.l().unwrap())
    }
    pub fn cancel_task(&mut self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Int(arg0.into());
        self.jni_ref().call_method(
            &self.jni_object(),
            "cancelTask",
            "(I)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn cancel_tasks(
        &mut self,
        arg0: crate::bukkit::plugin::Plugin<'mc>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.1.clone()) };
        self.jni_ref().call_method(
            &self.jni_object(),
            "cancelTasks",
            "(Lorg/bukkit/plugin/Plugin;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn is_currently_running(&mut self, arg0: i32) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Int(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "isCurrentlyRunning",
            "(I)Z",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(res.z().unwrap())
    }
}
impl<'mc> crate::JNIRaw<'mc> for BukkitScheduler<'mc> {
    fn jni_ref(&self) -> jni::JNIEnv<'mc> {
        unsafe { self.0.unsafe_clone() }
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
pub struct BukkitRunnable<'mc>(
    pub(crate) jni::JNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> crate::JNIRaw<'mc> for BukkitRunnable<'mc> {
    fn jni_ref(&self) -> jni::JNIEnv<'mc> {
        unsafe { self.0.unsafe_clone() }
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> BukkitRunnable<'mc> {
    pub fn from_extendable<T>(
        plugin: &super::plugin::Plugin,
        event: &'mc T,
        lib_name: String,
        name: String,
    ) -> Result<Self, Box<dyn std::error::Error>>
    where
        T: crate::JNIRaw<'mc>,
    {
        let obj = jni::objects::JValueGen::Object(event.jni_ref().new_object(
            "net/ioixd/blackbox/extendables/ExtendableBukkitRunnable",
            "(Lorg/bukkit/plugin/Plugin;Ljava/lang/String;Ljava/lang/String;)V",
            &[
                jni::objects::JValueGen::from(&plugin.1),
                jni::objects::JValueGen::from(&jni::objects::JObject::from(
                    event.jni_ref().new_string(name).unwrap(),
                )),
                jni::objects::JValueGen::from(&jni::objects::JObject::from(
                    event.jni_ref().new_string(lib_name).unwrap(),
                )),
            ],
        )?);
        Ok(Self(event.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(obj.l()?.clone())
        }))
    }
    pub fn from_raw(env: jni::JNIEnv<'mc>, obj: jni::objects::JObject<'mc>) -> Self {
        Self(env, obj)
    }
    pub fn cancel(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.jni_ref()
            .call_method(&self.jni_object(), "cancel", "()V", &[])?;
        Ok(())
    }
    pub fn is_cancelled(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isCancelled", "()Z", &[])?;
        Ok(res.z().unwrap())
    }
    pub fn run_task(
        &mut self,
        arg0: crate::bukkit::plugin::Plugin<'mc>,
    ) -> Result<crate::bukkit::scheduler::BukkitTask<'mc>, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "runTask",
            "(Lorg/bukkit/plugin/Plugin;)Lorg/bukkit/scheduler/BukkitTask;",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        let ret = {
            crate::bukkit::scheduler::BukkitTask(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn run_task_asynchronously(
        &mut self,
        arg0: crate::bukkit::plugin::Plugin<'mc>,
    ) -> Result<crate::bukkit::scheduler::BukkitTask<'mc>, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "runTaskAsynchronously",
            "(Lorg/bukkit/plugin/Plugin;)Lorg/bukkit/scheduler/BukkitTask;",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        let ret = {
            crate::bukkit::scheduler::BukkitTask(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn run_task_later(
        &mut self,
        arg0: crate::bukkit::plugin::Plugin<'mc>,
        arg1: i64,
    ) -> Result<crate::bukkit::scheduler::BukkitTask<'mc>, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.1.clone()) };
        let val_1 = jni::objects::JValueGen::Long(arg1.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "runTaskLater",
            "(Lorg/bukkit/plugin/Plugin;J)Lorg/bukkit/scheduler/BukkitTask;",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        )?;
        let ret = {
            crate::bukkit::scheduler::BukkitTask(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn run_task_later_asynchronously(
        &mut self,
        arg0: crate::bukkit::plugin::Plugin<'mc>,
        arg1: i64,
    ) -> Result<crate::bukkit::scheduler::BukkitTask<'mc>, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.1.clone()) };
        let val_1 = jni::objects::JValueGen::Long(arg1.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "runTaskLaterAsynchronously",
            "(Lorg/bukkit/plugin/Plugin;J)Lorg/bukkit/scheduler/BukkitTask;",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        )?;
        let ret = {
            crate::bukkit::scheduler::BukkitTask(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn run_task_timer(
        &mut self,
        arg0: crate::bukkit::plugin::Plugin<'mc>,
        arg1: i64,
        arg2: i64,
    ) -> Result<crate::bukkit::scheduler::BukkitTask<'mc>, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.1.clone()) };
        let val_1 = jni::objects::JValueGen::Long(arg1.into());
        let val_2 = jni::objects::JValueGen::Long(arg2.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "runTaskTimer",
            "(Lorg/bukkit/plugin/Plugin;JJ)Lorg/bukkit/scheduler/BukkitTask;",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        )?;
        let ret = {
            crate::bukkit::scheduler::BukkitTask(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn run_task_timer_asynchronously(
        &mut self,
        arg0: crate::bukkit::plugin::Plugin<'mc>,
        arg1: i64,
        arg2: i64,
    ) -> Result<crate::bukkit::scheduler::BukkitTask<'mc>, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.1.clone()) };
        let val_1 = jni::objects::JValueGen::Long(arg1.into());
        let val_2 = jni::objects::JValueGen::Long(arg2.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "runTaskTimerAsynchronously",
            "(Lorg/bukkit/plugin/Plugin;JJ)Lorg/bukkit/scheduler/BukkitTask;",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        )?;
        let ret = {
            crate::bukkit::scheduler::BukkitTask(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn task_id(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getTaskId", "()I", &[])?;
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
    pub fn run(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.jni_ref()
            .call_method(&self.jni_object(), "run", "()V", &[])?;
        Ok(())
    }
}
