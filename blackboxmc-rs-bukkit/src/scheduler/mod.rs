#![allow(deprecated)]
use blackboxmc_general::JNIRaw;
use blackboxmc_general::JNIInstantiatable;
use color_eyre::eyre::Result;/*org/bukkit/scheduler/mod.rs*/

#[repr(C)]
pub struct BukkitScheduler<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for BukkitScheduler<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for BukkitScheduler<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate BukkitScheduler from null object.")
                .into());
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
    }
    
impl<'mc> BukkitSchedulerTrait<'mc> for BukkitScheduler<'mc> {}
pub trait BukkitSchedulerTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {
#[deprecated]

	fn schedule_sync_delayed_task(&self,plugin: impl Into<crate::plugin::Plugin<'mc>>,task: impl Into<crate::scheduler::BukkitRunnable<'mc>>,delay: std::option::Option<i64>) 
-> Result<i32, Box<dyn std::error::Error>>

{let mut args = Vec::new();
let mut sig = String::from("(");
sig += "Lorg/bukkit/plugin/Plugin;";
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(plugin.into().jni_object().clone())});
args.push(val_1);
sig += "Lorg/bukkit/scheduler/BukkitRunnable;";
let val_2 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(task.into().jni_object().clone())});
args.push(val_2);
if let Some(a) = delay {
sig += "J";
let val_3 = jni::objects::JValueGen::Long(a);
args.push(val_3);
}
sig += ")I";
let res = self.jni_ref().call_method(&self.jni_object(),"scheduleSyncDelayedTask",sig.as_str(),args);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.i()?
)}
#[deprecated]

	fn schedule_sync_repeating_task(&self,plugin: impl Into<crate::plugin::Plugin<'mc>>,task: impl Into<crate::scheduler::BukkitRunnable<'mc>>,delay: i64,period: i64) 
-> Result<i32, Box<dyn std::error::Error>>

{let mut args = Vec::new();
let mut sig = String::from("(");
sig += "Lorg/bukkit/plugin/Plugin;";
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(plugin.into().jni_object().clone())});
args.push(val_1);
sig += "Lorg/bukkit/scheduler/BukkitRunnable;";
let val_2 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(task.into().jni_object().clone())});
args.push(val_2);
sig += "J";
let val_3 = jni::objects::JValueGen::Long(delay);
args.push(val_3);
sig += "J";
let val_4 = jni::objects::JValueGen::Long(period);
args.push(val_4);
sig += ")I";
let res = self.jni_ref().call_method(&self.jni_object(),"scheduleSyncRepeatingTask",sig.as_str(),args);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.i()?
)}
/// Removes task from scheduler.
	fn cancel_task(&self,task_id: i32) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(I)V");
let val_1 = jni::objects::JValueGen::Int(task_id);
let res = self.jni_ref().call_method(&self.jni_object(),"cancelTask",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
/// Removes all tasks associated with a particular plugin from the
/// scheduler.
	fn cancel_tasks(&self,plugin: impl Into<crate::plugin::Plugin<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/plugin/Plugin;)V");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(plugin.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"cancelTasks",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
/// Check if the task currently running.
/// 
/// A repeating task might not be running currently, but will be running in
/// the future. A task that has finished, and does not repeat, will not be
/// running ever again.
/// 
/// Explicitly, a task is running if there exists a thread for it, and that
/// thread is alive.
	fn is_currently_running(&self,task_id: i32) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("(I)Z");
let val_1 = jni::objects::JValueGen::Int(task_id);
let res = self.jni_ref().call_method(&self.jni_object(),"isCurrentlyRunning",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
/// Check if the task queued to be run later.
/// 
/// If a repeating task is currently running, it might not be queued now
/// but could be in the future. A task that is not queued, and not running,
/// will not be queued again.
	fn is_queued(&self,task_id: i32) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("(I)Z");
let val_1 = jni::objects::JValueGen::Int(task_id);
let res = self.jni_ref().call_method(&self.jni_object(),"isQueued",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
/// Returns a list of all active workers.
/// 
/// This list contains asynch tasks that are being executed by separate
/// threads.
	fn active_workers(&self) 
-> Result<Vec<crate::scheduler::BukkitWorker<'mc>>, Box<dyn std::error::Error>>

{let sig = String::from("()Ljava/util/List;");
let res = self.jni_ref().call_method(&self.jni_object(),"getActiveWorkers",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
let mut new_vec = Vec::new();
let list = blackboxmc_java::util::JavaList::from_raw(&self.jni_ref(), res.l()?)?;let iter = list.iterator()?;
while iter.has_next()? {            let obj = iter.next()?;
new_vec.push(crate::scheduler::BukkitWorker::from_raw(&self.jni_ref(),obj,)?);
};
Ok(
new_vec
)}
/// Returns a list of all pending tasks. The ordering of the tasks is not
/// related to their order of execution.
	fn pending_tasks(&self) 
-> Result<Vec<crate::scheduler::BukkitTask<'mc>>, Box<dyn std::error::Error>>

{let sig = String::from("()Ljava/util/List;");
let res = self.jni_ref().call_method(&self.jni_object(),"getPendingTasks",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
let mut new_vec = Vec::new();
let list = blackboxmc_java::util::JavaList::from_raw(&self.jni_ref(), res.l()?)?;let iter = list.iterator()?;
while iter.has_next()? {            let obj = iter.next()?;
new_vec.push(crate::scheduler::BukkitTask::from_raw(&self.jni_ref(),obj,)?);
};
Ok(
new_vec
)}
#[deprecated]

	fn run_task(&self,plugin: impl Into<crate::plugin::Plugin<'mc>>,task: impl Into<crate::scheduler::BukkitRunnable<'mc>>) 
-> Result<crate::scheduler::BukkitTask<'mc>, Box<dyn std::error::Error>>

{let mut args = Vec::new();
let mut sig = String::from("(");
sig += "Lorg/bukkit/plugin/Plugin;";
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(plugin.into().jni_object().clone())});
args.push(val_1);
sig += "Lorg/bukkit/scheduler/BukkitRunnable;";
let val_2 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(task.into().jni_object().clone())});
args.push(val_2);
sig += ")Lorg/bukkit/scheduler/BukkitTask;";
let res = self.jni_ref().call_method(&self.jni_object(),"runTask",sig.as_str(),args);
let res = 
self.jni_ref().translate_error(res)?;
crate::scheduler::BukkitTask::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
#[deprecated]

	fn run_task_asynchronously(&self,plugin: impl Into<crate::plugin::Plugin<'mc>>,task: impl Into<crate::scheduler::BukkitRunnable<'mc>>) 
-> Result<crate::scheduler::BukkitTask<'mc>, Box<dyn std::error::Error>>

{let mut args = Vec::new();
let mut sig = String::from("(");
sig += "Lorg/bukkit/plugin/Plugin;";
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(plugin.into().jni_object().clone())});
args.push(val_1);
sig += "Lorg/bukkit/scheduler/BukkitRunnable;";
let val_2 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(task.into().jni_object().clone())});
args.push(val_2);
sig += ")Lorg/bukkit/scheduler/BukkitTask;";
let res = self.jni_ref().call_method(&self.jni_object(),"runTaskAsynchronously",sig.as_str(),args);
let res = 
self.jni_ref().translate_error(res)?;
crate::scheduler::BukkitTask::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
#[deprecated]

	fn run_task_later(&self,plugin: impl Into<crate::plugin::Plugin<'mc>>,task: impl Into<crate::scheduler::BukkitRunnable<'mc>>,delay: i64) 
-> Result<crate::scheduler::BukkitTask<'mc>, Box<dyn std::error::Error>>

{let mut args = Vec::new();
let mut sig = String::from("(");
sig += "Lorg/bukkit/plugin/Plugin;";
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(plugin.into().jni_object().clone())});
args.push(val_1);
sig += "Lorg/bukkit/scheduler/BukkitRunnable;";
let val_2 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(task.into().jni_object().clone())});
args.push(val_2);
sig += "J";
let val_3 = jni::objects::JValueGen::Long(delay);
args.push(val_3);
sig += ")Lorg/bukkit/scheduler/BukkitTask;";
let res = self.jni_ref().call_method(&self.jni_object(),"runTaskLater",sig.as_str(),args);
let res = 
self.jni_ref().translate_error(res)?;
crate::scheduler::BukkitTask::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
#[deprecated]

	fn run_task_later_asynchronously(&self,plugin: impl Into<crate::plugin::Plugin<'mc>>,task: impl Into<crate::scheduler::BukkitRunnable<'mc>>,delay: i64) 
-> Result<crate::scheduler::BukkitTask<'mc>, Box<dyn std::error::Error>>

{let mut args = Vec::new();
let mut sig = String::from("(");
sig += "Lorg/bukkit/plugin/Plugin;";
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(plugin.into().jni_object().clone())});
args.push(val_1);
sig += "Lorg/bukkit/scheduler/BukkitRunnable;";
let val_2 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(task.into().jni_object().clone())});
args.push(val_2);
sig += "J";
let val_3 = jni::objects::JValueGen::Long(delay);
args.push(val_3);
sig += ")Lorg/bukkit/scheduler/BukkitTask;";
let res = self.jni_ref().call_method(&self.jni_object(),"runTaskLaterAsynchronously",sig.as_str(),args);
let res = 
self.jni_ref().translate_error(res)?;
crate::scheduler::BukkitTask::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
#[deprecated]

	fn run_task_timer(&self,plugin: impl Into<crate::plugin::Plugin<'mc>>,task: impl Into<crate::scheduler::BukkitRunnable<'mc>>,delay: i64,period: i64) 
-> Result<crate::scheduler::BukkitTask<'mc>, Box<dyn std::error::Error>>

{let mut args = Vec::new();
let mut sig = String::from("(");
sig += "Lorg/bukkit/plugin/Plugin;";
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(plugin.into().jni_object().clone())});
args.push(val_1);
sig += "Lorg/bukkit/scheduler/BukkitRunnable;";
let val_2 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(task.into().jni_object().clone())});
args.push(val_2);
sig += "J";
let val_3 = jni::objects::JValueGen::Long(delay);
args.push(val_3);
sig += "J";
let val_4 = jni::objects::JValueGen::Long(period);
args.push(val_4);
sig += ")Lorg/bukkit/scheduler/BukkitTask;";
let res = self.jni_ref().call_method(&self.jni_object(),"runTaskTimer",sig.as_str(),args);
let res = 
self.jni_ref().translate_error(res)?;
crate::scheduler::BukkitTask::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
#[deprecated]

	fn run_task_timer_asynchronously(&self,plugin: impl Into<crate::plugin::Plugin<'mc>>,task: impl Into<crate::scheduler::BukkitRunnable<'mc>>,delay: i64,period: i64) 
-> Result<crate::scheduler::BukkitTask<'mc>, Box<dyn std::error::Error>>

{let mut args = Vec::new();
let mut sig = String::from("(");
sig += "Lorg/bukkit/plugin/Plugin;";
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(plugin.into().jni_object().clone())});
args.push(val_1);
sig += "Lorg/bukkit/scheduler/BukkitRunnable;";
let val_2 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(task.into().jni_object().clone())});
args.push(val_2);
sig += "J";
let val_3 = jni::objects::JValueGen::Long(delay);
args.push(val_3);
sig += "J";
let val_4 = jni::objects::JValueGen::Long(period);
args.push(val_4);
sig += ")Lorg/bukkit/scheduler/BukkitTask;";
let res = self.jni_ref().call_method(&self.jni_object(),"runTaskTimerAsynchronously",sig.as_str(),args);
let res = 
self.jni_ref().translate_error(res)?;
crate::scheduler::BukkitTask::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
#[repr(C)]
pub struct BukkitTask<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for BukkitTask<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for BukkitTask<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate BukkitTask from null object.")
                .into());
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
    }
    
impl<'mc> BukkitTaskTrait<'mc> for BukkitTask<'mc> {}
pub trait BukkitTaskTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {
/// Returns the taskId for the task.
	fn task_id(&self) 
-> Result<i32, Box<dyn std::error::Error>>

{let sig = String::from("()I");
let res = self.jni_ref().call_method(&self.jni_object(),"getTaskId",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.i()?
)}
/// Returns the Plugin that owns this task.
	fn owner(&self) 
-> Result<crate::plugin::Plugin<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/plugin/Plugin;");
let res = self.jni_ref().call_method(&self.jni_object(),"getOwner",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::plugin::Plugin::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
/// Returns true if the Task is a sync task.
	fn is_sync(&self) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("()Z");
let res = self.jni_ref().call_method(&self.jni_object(),"isSync",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
/// Returns true if this task has been cancelled.
	fn is_cancelled(&self) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("()Z");
let res = self.jni_ref().call_method(&self.jni_object(),"isCancelled",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
/// Will attempt to cancel this task.
	fn cancel(&self) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("()V");
let res = self.jni_ref().call_method(&self.jni_object(),"cancel",sig.as_str(),vec![]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
#[repr(C)]
pub struct BukkitWorker<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for BukkitWorker<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for BukkitWorker<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate BukkitWorker from null object.")
                .into());
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
    }
    
impl<'mc> BukkitWorkerTrait<'mc> for BukkitWorker<'mc> {}
pub trait BukkitWorkerTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {
/// Returns the taskId for the task being executed by this worker.
	fn task_id(&self) 
-> Result<i32, Box<dyn std::error::Error>>

{let sig = String::from("()I");
let res = self.jni_ref().call_method(&self.jni_object(),"getTaskId",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.i()?
)}
/// Returns the Plugin that owns this task.
	fn owner(&self) 
-> Result<crate::plugin::Plugin<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/plugin/Plugin;");
let res = self.jni_ref().call_method(&self.jni_object(),"getOwner",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::plugin::Plugin::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
#[repr(C)]
pub struct BukkitRunnable<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for BukkitRunnable<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for BukkitRunnable<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate BukkitRunnable from null object.")
                .into());
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
    }
    
impl<'mc> BukkitRunnableTrait<'mc> for BukkitRunnable<'mc> {}
pub trait BukkitRunnableTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {

	fn new(jni: &blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::scheduler::BukkitRunnable<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()V");
let cls = jni.find_class("org/bukkit/scheduler/BukkitRunnable"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.new_object(cls,
sig.as_str(),vec![]);
let res = 
jni.translate_error_no_gen(res)?;
crate::scheduler::BukkitRunnable::from_raw(&jni,res
)}
/// Returns true if this task has been cancelled.
	fn is_cancelled(&self) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("()Z");
let res = self.jni_ref().call_method(&self.jni_object(),"isCancelled",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
/// Attempts to cancel this task.
	fn cancel(&self) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("()V");
let res = self.jni_ref().call_method(&self.jni_object(),"cancel",sig.as_str(),vec![]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
/// Schedules this in the Bukkit scheduler to run on next tick.
	fn run_task(&self,plugin: impl Into<crate::plugin::Plugin<'mc>>) 
-> Result<crate::scheduler::BukkitTask<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/plugin/Plugin;)Lorg/bukkit/scheduler/BukkitTask;");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(plugin.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"runTask",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
crate::scheduler::BukkitTask::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
/// <b>Asynchronous tasks should never access any API in Bukkit. Great care
/// should be taken to assure the thread-safety of asynchronous tasks.</b>
/// 
/// Schedules this in the Bukkit scheduler to run asynchronously.
	fn run_task_asynchronously(&self,plugin: impl Into<crate::plugin::Plugin<'mc>>) 
-> Result<crate::scheduler::BukkitTask<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/plugin/Plugin;)Lorg/bukkit/scheduler/BukkitTask;");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(plugin.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"runTaskAsynchronously",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
crate::scheduler::BukkitTask::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
/// Schedules this to run after the specified number of server ticks.
	fn run_task_later(&self,plugin: impl Into<crate::plugin::Plugin<'mc>>,delay: i64) 
-> Result<crate::scheduler::BukkitTask<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/plugin/Plugin;J)Lorg/bukkit/scheduler/BukkitTask;");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(plugin.into().jni_object().clone())});
let val_2 = jni::objects::JValueGen::Long(delay);
let res = self.jni_ref().call_method(&self.jni_object(),"runTaskLater",sig.as_str(),vec![jni::objects::JValueGen::from(val_1),jni::objects::JValueGen::from(val_2)]);
let res = 
self.jni_ref().translate_error(res)?;
crate::scheduler::BukkitTask::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
/// <b>Asynchronous tasks should never access any API in Bukkit. Great care
/// should be taken to assure the thread-safety of asynchronous tasks.</b>
/// 
/// Schedules this to run asynchronously after the specified number of
/// server ticks.
	fn run_task_later_asynchronously(&self,plugin: impl Into<crate::plugin::Plugin<'mc>>,delay: i64) 
-> Result<crate::scheduler::BukkitTask<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/plugin/Plugin;J)Lorg/bukkit/scheduler/BukkitTask;");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(plugin.into().jni_object().clone())});
let val_2 = jni::objects::JValueGen::Long(delay);
let res = self.jni_ref().call_method(&self.jni_object(),"runTaskLaterAsynchronously",sig.as_str(),vec![jni::objects::JValueGen::from(val_1),jni::objects::JValueGen::from(val_2)]);
let res = 
self.jni_ref().translate_error(res)?;
crate::scheduler::BukkitTask::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
/// Schedules this to repeatedly run until cancelled, starting after the
/// specified number of server ticks.
	fn run_task_timer(&self,plugin: impl Into<crate::plugin::Plugin<'mc>>,delay: i64,period: i64) 
-> Result<crate::scheduler::BukkitTask<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/plugin/Plugin;JJ)Lorg/bukkit/scheduler/BukkitTask;");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(plugin.into().jni_object().clone())});
let val_2 = jni::objects::JValueGen::Long(delay);
let val_3 = jni::objects::JValueGen::Long(period);
let res = self.jni_ref().call_method(&self.jni_object(),"runTaskTimer",sig.as_str(),vec![jni::objects::JValueGen::from(val_1),jni::objects::JValueGen::from(val_2),jni::objects::JValueGen::from(val_3)]);
let res = 
self.jni_ref().translate_error(res)?;
crate::scheduler::BukkitTask::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
/// <b>Asynchronous tasks should never access any API in Bukkit. Great care
/// should be taken to assure the thread-safety of asynchronous tasks.</b>
/// 
/// Schedules this to repeatedly run asynchronously until cancelled,
/// starting after the specified number of server ticks.
	fn run_task_timer_asynchronously(&self,plugin: impl Into<crate::plugin::Plugin<'mc>>,delay: i64,period: i64) 
-> Result<crate::scheduler::BukkitTask<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/plugin/Plugin;JJ)Lorg/bukkit/scheduler/BukkitTask;");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(plugin.into().jni_object().clone())});
let val_2 = jni::objects::JValueGen::Long(delay);
let val_3 = jni::objects::JValueGen::Long(period);
let res = self.jni_ref().call_method(&self.jni_object(),"runTaskTimerAsynchronously",sig.as_str(),vec![jni::objects::JValueGen::from(val_1),jni::objects::JValueGen::from(val_2),jni::objects::JValueGen::from(val_3)]);
let res = 
self.jni_ref().translate_error(res)?;
crate::scheduler::BukkitTask::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
/// Gets the task id for this runnable.
	fn task_id(&self) 
-> Result<i32, Box<dyn std::error::Error>>

{let sig = String::from("()I");
let res = self.jni_ref().call_method(&self.jni_object(),"getTaskId",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.i()?
)}

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
