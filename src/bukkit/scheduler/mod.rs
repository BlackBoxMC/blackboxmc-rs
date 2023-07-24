#![allow(deprecated)]
use crate::JNIRaw;
use color_eyre::eyre::Result;
/// An instantiatable struct that implements BukkitTask. Needed for returning it from Java.
pub struct BukkitTask<'mc>(
    pub(crate) crate::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> BukkitTask<'mc> {
    pub fn from_raw(
        env: &crate::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate BukkitTask from null object.").into());
        }
        let cls = env.jni.borrow().get_object_class(&obj)?;
        let name_raw = env.call_method(cls, "getName", "()Ljava/lang/String;", &[])?;
        let oh = name_raw.l()?.into();
        let what = env.get_string(&oh)?;
        let name = what.to_string_lossy();
        if !name.ends_with("BukkitTask") {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a BukkitTask object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    pub fn is_sync(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isSync", "()Z", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
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
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            crate::bukkit::plugin::Plugin(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn cancel(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "cancel", "()V", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn is_cancelled(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isCancelled", "()Z", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.z().unwrap())
    }
    pub fn task_id(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getTaskId", "()I", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.i().unwrap())
    }
}
impl<'mc> crate::JNIRaw<'mc> for BukkitTask<'mc> {
    fn jni_ref(&self) -> crate::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
/// An instantiatable struct that implements BukkitWorker. Needed for returning it from Java.
pub struct BukkitWorker<'mc>(
    pub(crate) crate::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> BukkitWorker<'mc> {
    pub fn from_raw(
        env: &crate::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate BukkitWorker from null object.").into());
        }
        let cls = env.jni.borrow().get_object_class(&obj)?;
        let name_raw = env.call_method(cls, "getName", "()Ljava/lang/String;", &[])?;
        let oh = name_raw.l()?.into();
        let what = env.get_string(&oh)?;
        let name = what.to_string_lossy();
        if !name.ends_with("BukkitWorker") {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a BukkitWorker object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    pub fn owner(
        &mut self,
    ) -> Result<crate::bukkit::plugin::Plugin<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getOwner",
            "()Lorg/bukkit/plugin/Plugin;",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
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
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.l().unwrap())
    }
    pub fn task_id(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getTaskId", "()I", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.i().unwrap())
    }
}
impl<'mc> crate::JNIRaw<'mc> for BukkitWorker<'mc> {
    fn jni_ref(&self) -> crate::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
/// An instantiatable struct that implements BukkitScheduler. Needed for returning it from Java.
pub struct BukkitScheduler<'mc>(
    pub(crate) crate::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> BukkitScheduler<'mc> {
    pub fn from_raw(
        env: &crate::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate BukkitScheduler from null object.").into(),
            );
        }
        let cls = env.jni.borrow().get_object_class(&obj)?;
        let name_raw = env.call_method(cls, "getName", "()Ljava/lang/String;", &[])?;
        let oh = name_raw.l()?.into();
        let what = env.get_string(&oh)?;
        let name = what.to_string_lossy();
        if !name.ends_with("BukkitScheduler") {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a BukkitScheduler object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    pub fn is_queued(&mut self, arg0: i32) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Int(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "isQueued",
            "(I)Z",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.z().unwrap())
    }
    #[deprecated]
    pub fn schedule_sync_delayed_task_with_plugin(
        &mut self,
        arg0: impl Into<&'mc crate::bukkit::plugin::Plugin<'mc>>,
        arg1: impl Into<&'mc crate::bukkit::scheduler::BukkitRunnable<'mc>>,
        arg2: std::option::Option<i64>,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg1.into().1.clone()) };
        let val_2 = jni::objects::JValueGen::Long(arg2.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "scheduleSyncDelayedTask",
            "(Lorg/bukkit/plugin/Plugin;Lorg/bukkit/scheduler/BukkitRunnable;J)I",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.i().unwrap())
    }
    pub fn schedule_sync_repeating_task_with_plugin(
        &mut self,
        arg0: impl Into<&'mc crate::bukkit::plugin::Plugin<'mc>>,
        arg1: jni::objects::JObject<'mc>,
        arg2: i64,
        arg3: std::option::Option<i64>,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let val_1 = arg1;
        let val_2 = jni::objects::JValueGen::Long(arg2.into());
        let val_3 = jni::objects::JValueGen::Long(arg3.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "scheduleSyncRepeatingTask",
            "(Lorg/bukkit/plugin/Plugin;Ljava/lang/Runnable;JJ)I",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
                jni::objects::JValueGen::from(&val_3),
            ],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.i().unwrap())
    }
    #[deprecated]
    pub fn schedule_async_delayed_task_with_plugin(
        &mut self,
        arg0: impl Into<&'mc crate::bukkit::plugin::Plugin<'mc>>,
        arg1: std::option::Option<jni::objects::JObject<'mc>>,
        arg2: std::option::Option<i64>,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
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
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.i().unwrap())
    }
    #[deprecated]
    pub fn schedule_async_repeating_task(
        &mut self,
        arg0: impl Into<&'mc crate::bukkit::plugin::Plugin<'mc>>,
        arg1: jni::objects::JObject<'mc>,
        arg2: i64,
        arg3: i64,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
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
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.i().unwrap())
    }
    pub fn call_sync_method(
        &mut self,
        arg0: impl Into<&'mc crate::bukkit::plugin::Plugin<'mc>>,
        arg1: jni::objects::JObject<'mc>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let val_1 = arg1;
        let res = self.jni_ref().call_method(&self.jni_object(),"callSyncMethod","(Lorg/bukkit/plugin/Plugin;Ljava/util/concurrent/Callable;)Ljava/util/concurrent/Future;",&[jni::objects::JValueGen::from(&val_0),jni::objects::JValueGen::from(&val_1)]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.l().unwrap())
    }
    pub fn cancel_task(&mut self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Int(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "cancelTask",
            "(I)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn cancel_tasks(
        &mut self,
        arg0: impl Into<&'mc crate::bukkit::plugin::Plugin<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "cancelTasks",
            "(Lorg/bukkit/plugin/Plugin;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn is_currently_running(&mut self, arg0: i32) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Int(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "isCurrentlyRunning",
            "(I)Z",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.z().unwrap())
    }
    pub fn run_task_with_plugin(
        &mut self,
        arg0: impl Into<&'mc crate::bukkit::plugin::Plugin<'mc>>,
        arg1: std::option::Option<jni::objects::JObject<'mc>>,
    ) -> Result<crate::bukkit::scheduler::BukkitTask<'mc>, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let val_1 = arg1.unwrap();
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "runTask",
            "(Lorg/bukkit/plugin/Plugin;Ljava/lang/Runnable;)Lorg/bukkit/scheduler/BukkitTask;",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            crate::bukkit::scheduler::BukkitTask(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn run_task_asynchronously_with_plugin(
        &mut self,
        arg0: impl Into<&'mc crate::bukkit::plugin::Plugin<'mc>>,
        arg1: std::option::Option<jni::objects::JObject<'mc>>,
    ) -> Result<crate::bukkit::scheduler::BukkitTask<'mc>, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let val_1 = arg1.unwrap();
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "runTaskAsynchronously",
            "(Lorg/bukkit/plugin/Plugin;Ljava/lang/Runnable;)Lorg/bukkit/scheduler/BukkitTask;",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            crate::bukkit::scheduler::BukkitTask(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn run_task_later_with_plugin(
        &mut self,
        arg0: impl Into<&'mc crate::bukkit::plugin::Plugin<'mc>>,
        arg1: jni::objects::JObject<'mc>,
        arg2: std::option::Option<i64>,
    ) -> Result<crate::bukkit::scheduler::BukkitTask<'mc>, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let val_1 = arg1;
        let val_2 = jni::objects::JValueGen::Long(arg2.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "runTaskLater",
            "(Lorg/bukkit/plugin/Plugin;Ljava/lang/Runnable;J)Lorg/bukkit/scheduler/BukkitTask;",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            crate::bukkit::scheduler::BukkitTask(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn run_task_later_asynchronously_with_plugin(
        &mut self,
        arg0: impl Into<&'mc crate::bukkit::plugin::Plugin<'mc>>,
        arg1: jni::objects::JObject<'mc>,
        arg2: std::option::Option<i64>,
    ) -> Result<crate::bukkit::scheduler::BukkitTask<'mc>, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let val_1 = arg1;
        let val_2 = jni::objects::JValueGen::Long(arg2.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "runTaskLaterAsynchronously",
            "(Lorg/bukkit/plugin/Plugin;Ljava/lang/Runnable;J)Lorg/bukkit/scheduler/BukkitTask;",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            crate::bukkit::scheduler::BukkitTask(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn run_task_timer_with_plugin(
        &mut self,
        arg0: impl Into<&'mc crate::bukkit::plugin::Plugin<'mc>>,
        arg1: jni::objects::JObject<'mc>,
        arg2: i64,
        arg3: std::option::Option<i64>,
    ) -> Result<crate::bukkit::scheduler::BukkitTask<'mc>, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let val_1 = arg1;
        let val_2 = jni::objects::JValueGen::Long(arg2.into());
        let val_3 = jni::objects::JValueGen::Long(arg3.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "runTaskTimer",
            "(Lorg/bukkit/plugin/Plugin;Ljava/lang/Runnable;JJ)Lorg/bukkit/scheduler/BukkitTask;",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
                jni::objects::JValueGen::from(&val_3),
            ],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            crate::bukkit::scheduler::BukkitTask(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn run_task_timer_asynchronously_with_plugin(
        &mut self,
        arg0: impl Into<&'mc crate::bukkit::plugin::Plugin<'mc>>,
        arg1: jni::objects::JObject<'mc>,
        arg2: i64,
        arg3: std::option::Option<i64>,
    ) -> Result<crate::bukkit::scheduler::BukkitTask<'mc>, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let val_1 = arg1;
        let val_2 = jni::objects::JValueGen::Long(arg2.into());
        let val_3 = jni::objects::JValueGen::Long(arg3.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "runTaskTimerAsynchronously",
            "(Lorg/bukkit/plugin/Plugin;Ljava/lang/Runnable;JJ)Lorg/bukkit/scheduler/BukkitTask;",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
                jni::objects::JValueGen::from(&val_3),
            ],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            crate::bukkit::scheduler::BukkitTask(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
}
impl<'mc> crate::JNIRaw<'mc> for BukkitScheduler<'mc> {
    fn jni_ref(&self) -> crate::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
pub struct BukkitRunnable<'mc>(
    pub(crate) crate::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> crate::JNIRaw<'mc> for BukkitRunnable<'mc> {
    fn jni_ref(&self) -> crate::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> BukkitRunnable<'mc> {
    pub fn from_extendable(
        env: &crate::SharedJNIEnv<'mc>,
        plugin: &'mc crate::bukkit::plugin::Plugin,
        address: i32,
        lib_name: String,
        name: String,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let obj = unsafe { plugin.new_extendable(address, "Runnable", name, lib_name) }?;
        Self::from_raw(env, obj)
    }
    pub fn new(
        jni: crate::SharedJNIEnv<'mc>,
    ) -> Result<crate::bukkit::scheduler::BukkitRunnable<'mc>, Box<dyn std::error::Error>> {
        let cls = &jni.find_class("org/bukkit/scheduler/BukkitRunnable")?;
        let res = jni.new_object(cls, "()V", &[])?;
        let ret = { crate::bukkit::scheduler::BukkitRunnable(jni, res) };
        Ok(ret)
    }
    pub fn from_raw(
        env: &crate::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate BukkitRunnable from null object.").into(),
            );
        }
        let cls = env.jni.borrow().get_object_class(&obj)?;
        let name_raw = env.call_method(cls, "getName", "()Ljava/lang/String;", &[])?;
        let oh = name_raw.l()?.into();
        let what = env.get_string(&oh)?;
        let name = what.to_string_lossy();
        if !name.ends_with("BukkitRunnable") {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a BukkitRunnable object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    pub fn cancel(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "cancel", "()V", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn is_cancelled(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isCancelled", "()Z", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.z().unwrap())
    }
    pub fn run_task(
        &mut self,
        arg0: impl Into<&'mc crate::bukkit::plugin::Plugin<'mc>>,
    ) -> Result<crate::bukkit::scheduler::BukkitTask<'mc>, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "runTask",
            "(Lorg/bukkit/plugin/Plugin;)Lorg/bukkit/scheduler/BukkitTask;",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            crate::bukkit::scheduler::BukkitTask(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn run_task_asynchronously(
        &mut self,
        arg0: impl Into<&'mc crate::bukkit::plugin::Plugin<'mc>>,
    ) -> Result<crate::bukkit::scheduler::BukkitTask<'mc>, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "runTaskAsynchronously",
            "(Lorg/bukkit/plugin/Plugin;)Lorg/bukkit/scheduler/BukkitTask;",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            crate::bukkit::scheduler::BukkitTask(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn run_task_later(
        &mut self,
        arg0: impl Into<&'mc crate::bukkit::plugin::Plugin<'mc>>,
        arg1: i64,
    ) -> Result<crate::bukkit::scheduler::BukkitTask<'mc>, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let val_1 = jni::objects::JValueGen::Long(arg1.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "runTaskLater",
            "(Lorg/bukkit/plugin/Plugin;J)Lorg/bukkit/scheduler/BukkitTask;",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            crate::bukkit::scheduler::BukkitTask(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn run_task_later_asynchronously(
        &mut self,
        arg0: impl Into<&'mc crate::bukkit::plugin::Plugin<'mc>>,
        arg1: i64,
    ) -> Result<crate::bukkit::scheduler::BukkitTask<'mc>, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let val_1 = jni::objects::JValueGen::Long(arg1.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "runTaskLaterAsynchronously",
            "(Lorg/bukkit/plugin/Plugin;J)Lorg/bukkit/scheduler/BukkitTask;",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            crate::bukkit::scheduler::BukkitTask(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn run_task_timer(
        &mut self,
        arg0: impl Into<&'mc crate::bukkit::plugin::Plugin<'mc>>,
        arg1: i64,
        arg2: i64,
    ) -> Result<crate::bukkit::scheduler::BukkitTask<'mc>, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
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
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            crate::bukkit::scheduler::BukkitTask(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn run_task_timer_asynchronously(
        &mut self,
        arg0: impl Into<&'mc crate::bukkit::plugin::Plugin<'mc>>,
        arg1: i64,
        arg2: i64,
    ) -> Result<crate::bukkit::scheduler::BukkitTask<'mc>, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
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
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
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
            .call_method(&self.jni_object(), "getTaskId", "()I", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.i().unwrap())
    }
    pub fn wait(
        &mut self,
        arg0: std::option::Option<i64>,
        arg1: std::option::Option<i32>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Long(arg0.unwrap().into());
        let val_1 = jni::objects::JValueGen::Int(arg1.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "wait",
            "(JI)V",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
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
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.z().unwrap())
    }
    pub fn to_string(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "toString", "()Ljava/lang/String;", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
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
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.i().unwrap())
    }
    pub fn class(&mut self) -> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getClass", "()Ljava/lang/Class;", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(unsafe { jni::objects::JClass::from_raw(res.as_jni().l) })
    }
    pub fn notify(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notify", "()V", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn notify_all(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notifyAll", "()V", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn run(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "run", "()V", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
}
