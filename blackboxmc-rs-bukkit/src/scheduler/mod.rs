#![allow(deprecated)]
use blackboxmc_general::JNIRaw;
use color_eyre::eyre::Result;
/// Represents a task being executed by the scheduler
///
/// This is a representation of an abstract class.
pub struct BukkitTask<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> BukkitTask<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate BukkitTask from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/scheduler/BukkitTask")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a BukkitTask object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    /// Will attempt to cancel this task.
    pub fn cancel(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "cancel", "()V", &[]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Returns true if this task has been cancelled.
    pub fn is_cancelled(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isCancelled", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    /// Returns true if the Task is a sync task.
    pub fn is_sync(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isSync", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    /// Returns the Plugin that owns this task.
    pub fn owner(&mut self) -> Result<crate::plugin::Plugin<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getOwner",
            "()Lorg/bukkit/plugin/Plugin;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::plugin::Plugin::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Returns the taskId for the task.
    pub fn task_id(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getTaskId", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }
}
impl<'mc> JNIRaw<'mc> for BukkitTask<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
/// Represents a worker thread for the scheduler. This gives information about the Thread object for the task, owner of the task and the taskId.
/// <p>Workers are used to execute async tasks.</p>
///
/// This is a representation of an abstract class.
pub struct BukkitWorker<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> BukkitWorker<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate BukkitWorker from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/scheduler/BukkitWorker")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a BukkitWorker object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    /// Returns the Plugin that owns this task.
    pub fn owner(&mut self) -> Result<crate::plugin::Plugin<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getOwner",
            "()Lorg/bukkit/plugin/Plugin;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::plugin::Plugin::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Returns the taskId for the task being executed by this worker.
    pub fn task_id(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getTaskId", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }
}
impl<'mc> JNIRaw<'mc> for BukkitWorker<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}

///
/// This is a representation of an abstract class.
pub struct BukkitScheduler<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> BukkitScheduler<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate BukkitScheduler from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/scheduler/BukkitScheduler")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a BukkitScheduler object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    /// Check if the task queued to be run later.
    /// <p>If a repeating task is currently running, it might not be queued now but could be in the future. A task that is not queued, and not running, will not be queued again.</p>
    pub fn is_queued(&mut self, arg0: i32) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "isQueued",
            "(I)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    #[deprecated]
    /// Schedules a once off task to occur after a delay.
    /// <p>This task will be executed by the main server thread.</p>
    /// <span class="deprecated-label">Deprecated.</span>
    /// <div class="deprecation-comment">
    /// Use <a href="BukkitRunnable.html#runTaskLater(org.bukkit.plugin.Plugin,long)"><code>BukkitRunnable.runTaskLater(Plugin, long)</code></a>
    /// </div>
    /// Use <a href="BukkitRunnable.html#runTaskLater(org.bukkit.plugin.Plugin,long)"><code>BukkitRunnable.runTaskLater(Plugin, long)</code></a>
    ///
    /// Schedules a once off task to occur as soon as possible.
    /// <p>This task will be executed by the main server thread.</p>
    /// <span class="deprecated-label">Deprecated.</span>
    /// <div class="deprecation-comment">
    /// Use <a href="BukkitRunnable.html#runTask(org.bukkit.plugin.Plugin)"><code>BukkitRunnable.runTask(Plugin)</code></a>
    /// </div>
    /// Use <a href="BukkitRunnable.html#runTask(org.bukkit.plugin.Plugin)"><code>BukkitRunnable.runTask(Plugin)</code></a>
    ///
    pub fn schedule_sync_delayed_task_with_plugin(
        &mut self,
        arg0: impl Into<&'mc crate::plugin::Plugin<'mc>>,
        arg1: impl Into<&'mc crate::scheduler::BukkitRunnable<'mc>>,
        arg2: std::option::Option<i64>,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let val_2 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone()) };
        let val_3 = jni::objects::JValueGen::Long(arg2.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "scheduleSyncDelayedTask",
            "(Lorg/bukkit/plugin/Plugin;Lorg/bukkit/scheduler/BukkitRunnable;J)I",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
                jni::objects::JValueGen::from(&val_3),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }
    /// Removes task from scheduler.
    /// Removes all tasks associated with a particular plugin from the scheduler.
    pub fn cancel_task(&mut self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "cancelTask",
            "(I)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Removes all tasks associated with a particular plugin from the scheduler.
    pub fn cancel_tasks(
        &mut self,
        arg0: impl Into<&'mc crate::plugin::Plugin<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "cancelTasks",
            "(Lorg/bukkit/plugin/Plugin;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Check if the task currently running.
    /// <p>A repeating task might not be running currently, but will be running in the future. A task that has finished, and does not repeat, will not be running ever again.</p>
    /// <p>Explicitly, a task is running if there exists a thread for it, and that thread is alive.</p>
    pub fn is_currently_running(&mut self, arg0: i32) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "isCurrentlyRunning",
            "(I)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    /// Returns a list of all active workers.
    /// <p>This list contains asynch tasks that are being executed by separate threads.</p>
    pub fn active_workers(
        &mut self,
    ) -> Result<Vec<crate::scheduler::BukkitWorker<'mc>>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getActiveWorkers",
            "()Ljava/util/List;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let mut list = blackboxmc_java::JavaList::from_raw(&self.jni_ref(), res.l()?)?;
        let size = list.size()?;
        for i in 0..=size {
            let obj = list.get(i)?;
            new_vec.push(crate::scheduler::BukkitWorker::from_raw(
                &self.jni_ref(),
                obj,
            )?);
        }
        Ok(new_vec)
    }
    /// Returns a list of all pending tasks. The ordering of the tasks is not related to their order of execution.
    pub fn pending_tasks(
        &mut self,
    ) -> Result<Vec<crate::scheduler::BukkitTask<'mc>>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getPendingTasks",
            "()Ljava/util/List;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let mut list = blackboxmc_java::JavaList::from_raw(&self.jni_ref(), res.l()?)?;
        let size = list.size()?;
        for i in 0..=size {
            let obj = list.get(i)?;
            new_vec.push(crate::scheduler::BukkitTask::from_raw(
                &self.jni_ref(),
                obj,
            )?);
        }
        Ok(new_vec)
    }
}
impl<'mc> JNIRaw<'mc> for BukkitScheduler<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
/// This class is provided as an easy way to handle scheduling tasks.
pub struct BukkitRunnable<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for BukkitRunnable<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> BukkitRunnable<'mc> {
    pub fn from_extendable(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        plugin: &'mc crate::plugin::Plugin,
        address: i32,
        lib_name: String,
        name: String,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let obj = unsafe { plugin.new_extendable(address, "BukkitRunnable", name, lib_name) }?;
        Self::from_raw(env, obj)
    }
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate BukkitRunnable from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/scheduler/BukkitRunnable")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a BukkitRunnable object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    pub fn new(
        jni: blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<crate::scheduler::BukkitRunnable<'mc>, Box<dyn std::error::Error>> {
        let cls = &jni.find_class("org/bukkit/scheduler/BukkitRunnable")?;
        let res = jni.new_object(cls, "()V", &[])?;
        crate::scheduler::BukkitRunnable::from_raw(&jni, res)
    }
    /// Attempts to cancel this task.
    ///
    pub fn cancel(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "cancel", "()V", &[]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Returns true if this task has been cancelled.
    ///
    pub fn is_cancelled(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isCancelled", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    /// Schedules this in the Bukkit scheduler to run on next tick.
    ///
    /// <b>Asynchronous tasks should never access any API in Bukkit. Great care should be taken to assure the thread-safety of asynchronous tasks.</b>
    /// <p>Schedules this in the Bukkit scheduler to run asynchronously.</p>
    ///
    /// Schedules this to run after the specified number of server ticks.
    ///
    /// <b>Asynchronous tasks should never access any API in Bukkit. Great care should be taken to assure the thread-safety of asynchronous tasks.</b>
    /// <p>Schedules this to run asynchronously after the specified number of server ticks.</p>
    ///
    /// Schedules this to repeatedly run until cancelled, starting after the specified number of server ticks.
    ///
    /// <b>Asynchronous tasks should never access any API in Bukkit. Great care should be taken to assure the thread-safety of asynchronous tasks.</b>
    /// <p>Schedules this to repeatedly run asynchronously until cancelled, starting after the specified number of server ticks.</p>
    ///
    pub fn run_task(
        &mut self,
        arg0: impl Into<&'mc crate::plugin::Plugin<'mc>>,
    ) -> Result<crate::scheduler::BukkitTask<'mc>, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "runTask",
            "(Lorg/bukkit/plugin/Plugin;)Lorg/bukkit/scheduler/BukkitTask;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::scheduler::BukkitTask::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// <b>Asynchronous tasks should never access any API in Bukkit. Great care should be taken to assure the thread-safety of asynchronous tasks.</b>
    /// <p>Schedules this in the Bukkit scheduler to run asynchronously.</p>
    ///
    pub fn run_task_asynchronously(
        &mut self,
        arg0: impl Into<&'mc crate::plugin::Plugin<'mc>>,
    ) -> Result<crate::scheduler::BukkitTask<'mc>, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "runTaskAsynchronously",
            "(Lorg/bukkit/plugin/Plugin;)Lorg/bukkit/scheduler/BukkitTask;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::scheduler::BukkitTask::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Schedules this to run after the specified number of server ticks.
    ///
    /// <b>Asynchronous tasks should never access any API in Bukkit. Great care should be taken to assure the thread-safety of asynchronous tasks.</b>
    /// <p>Schedules this to run asynchronously after the specified number of server ticks.</p>
    ///
    pub fn run_task_later(
        &mut self,
        arg0: impl Into<&'mc crate::plugin::Plugin<'mc>>,
        arg1: i64,
    ) -> Result<crate::scheduler::BukkitTask<'mc>, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let val_2 = jni::objects::JValueGen::Long(arg1.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "runTaskLater",
            "(Lorg/bukkit/plugin/Plugin;J)Lorg/bukkit/scheduler/BukkitTask;",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::scheduler::BukkitTask::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// <b>Asynchronous tasks should never access any API in Bukkit. Great care should be taken to assure the thread-safety of asynchronous tasks.</b>
    /// <p>Schedules this to run asynchronously after the specified number of server ticks.</p>
    ///
    pub fn run_task_later_asynchronously(
        &mut self,
        arg0: impl Into<&'mc crate::plugin::Plugin<'mc>>,
        arg1: i64,
    ) -> Result<crate::scheduler::BukkitTask<'mc>, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let val_2 = jni::objects::JValueGen::Long(arg1.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "runTaskLaterAsynchronously",
            "(Lorg/bukkit/plugin/Plugin;J)Lorg/bukkit/scheduler/BukkitTask;",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::scheduler::BukkitTask::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Schedules this to repeatedly run until cancelled, starting after the specified number of server ticks.
    ///
    /// <b>Asynchronous tasks should never access any API in Bukkit. Great care should be taken to assure the thread-safety of asynchronous tasks.</b>
    /// <p>Schedules this to repeatedly run asynchronously until cancelled, starting after the specified number of server ticks.</p>
    ///
    pub fn run_task_timer(
        &mut self,
        arg0: impl Into<&'mc crate::plugin::Plugin<'mc>>,
        arg1: i64,
        arg2: i64,
    ) -> Result<crate::scheduler::BukkitTask<'mc>, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let val_2 = jni::objects::JValueGen::Long(arg1.into());
        let val_3 = jni::objects::JValueGen::Long(arg2.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "runTaskTimer",
            "(Lorg/bukkit/plugin/Plugin;JJ)Lorg/bukkit/scheduler/BukkitTask;",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
                jni::objects::JValueGen::from(&val_3),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::scheduler::BukkitTask::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// <b>Asynchronous tasks should never access any API in Bukkit. Great care should be taken to assure the thread-safety of asynchronous tasks.</b>
    /// <p>Schedules this to repeatedly run asynchronously until cancelled, starting after the specified number of server ticks.</p>
    ///
    pub fn run_task_timer_asynchronously(
        &mut self,
        arg0: impl Into<&'mc crate::plugin::Plugin<'mc>>,
        arg1: i64,
        arg2: i64,
    ) -> Result<crate::scheduler::BukkitTask<'mc>, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let val_2 = jni::objects::JValueGen::Long(arg1.into());
        let val_3 = jni::objects::JValueGen::Long(arg2.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "runTaskTimerAsynchronously",
            "(Lorg/bukkit/plugin/Plugin;JJ)Lorg/bukkit/scheduler/BukkitTask;",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
                jni::objects::JValueGen::from(&val_3),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::scheduler::BukkitTask::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Gets the task id for this runnable.
    ///
    pub fn task_id(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getTaskId", "()I", &[]);
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
    /// Schedules this in the Bukkit scheduler to run on next tick.
    ///
    /// <b>Asynchronous tasks should never access any API in Bukkit. Great care should be taken to assure the thread-safety of asynchronous tasks.</b>
    /// <p>Schedules this in the Bukkit scheduler to run asynchronously.</p>
    ///
    /// Schedules this to run after the specified number of server ticks.
    ///
    /// <b>Asynchronous tasks should never access any API in Bukkit. Great care should be taken to assure the thread-safety of asynchronous tasks.</b>
    /// <p>Schedules this to run asynchronously after the specified number of server ticks.</p>
    ///
    /// Schedules this to repeatedly run until cancelled, starting after the specified number of server ticks.
    ///
    /// <b>Asynchronous tasks should never access any API in Bukkit. Great care should be taken to assure the thread-safety of asynchronous tasks.</b>
    /// <p>Schedules this to repeatedly run asynchronously until cancelled, starting after the specified number of server ticks.</p>
    ///
    pub fn run(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "run", "()V", &[]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
}
