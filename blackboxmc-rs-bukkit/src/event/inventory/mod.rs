#![allow(deprecated)]
use blackboxmc_general::JNIInstantiatable;
use blackboxmc_general::JNIRaw;
use color_eyre::eyre::Result;
/// Represents a player related inventory event
#[repr(C)]
pub struct InventoryOpenEvent<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for InventoryOpenEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for InventoryOpenEvent<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate InventoryOpenEvent from null object.").into(),
            );
        }
        let (valid, name) =
            env.validate_name(&obj, "org/bukkit/event/inventory/InventoryOpenEvent")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a InventoryOpenEvent object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> InventoryOpenEvent<'mc> {
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<crate::inventory::InventoryView<'mc>>,
    ) -> Result<crate::event::inventory::InventoryOpenEvent<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/inventory/InventoryView;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let cls = jni.find_class("org/bukkit/event/inventory/InventoryOpenEvent");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(
            cls,
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = jni.translate_error_no_gen(res)?;
        crate::event::inventory::InventoryOpenEvent::from_raw(&jni, res)
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

    pub fn is_cancelled(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isCancelled", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn player(
        &self,
    ) -> Result<Option<crate::entity::HumanEntity<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/entity/HumanEntity;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getPlayer", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {
            return Ok(None);
        }
        Ok(Some(crate::entity::HumanEntity::from_raw(
            &self.jni_ref(),
            unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) },
        )?))
    }
    /// Sets the cancellation state of this event. A cancelled event will not be executed in the server, but will still pass to other plugins.
    /// <p>If an inventory open event is cancelled, the inventory screen will not show.</p>
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
    // SUPER CLASS: InventoryEvent
    pub fn inventory(
        &self,
    ) -> Result<crate::inventory::Inventory<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::inventory::InventoryEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::inventory::InventoryEvent = temp_clone.into();
        real.inventory()
    }
    pub fn viewers(
        &self,
    ) -> Result<Vec<crate::entity::HumanEntity<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/List;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getViewers", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let list = blackboxmc_java::util::JavaList::from_raw(&self.0, res.l()?)?;
        let iter = list.iterator()?;
        while iter.has_next()? {
            let obj = iter.next()?;
            new_vec.push(crate::entity::HumanEntity::from_raw(&self.0, obj)?);
        }
        Ok(new_vec)
    }
    pub fn view(&self) -> Result<crate::inventory::InventoryView<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::inventory::InventoryEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::inventory::InventoryEvent = temp_clone.into();
        real.view()
    }
    // SUPER CLASS: Event
    pub fn event_name(&self) -> Result<String, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::Event::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::Event = temp_clone.into();
        real.event_name()
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
impl<'mc> Into<crate::event::Cancellable<'mc>> for InventoryOpenEvent<'mc> {
    fn into(self) -> crate::event::Cancellable<'mc> {
        crate::event::Cancellable::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting InventoryOpenEvent into crate::event::Cancellable")
    }
}
impl<'mc> Into<crate::event::inventory::InventoryEvent<'mc>> for InventoryOpenEvent<'mc> {
    fn into(self) -> crate::event::inventory::InventoryEvent<'mc> {
        crate::event::inventory::InventoryEvent::from_raw(&self.jni_ref(), self.1).expect(
            "Error converting InventoryOpenEvent into crate::event::inventory::InventoryEvent",
        )
    }
}

#[repr(C)]
pub struct PrepareItemCraftEvent<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for PrepareItemCraftEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for PrepareItemCraftEvent<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate PrepareItemCraftEvent from null object."
            )
            .into());
        }
        let (valid, name) =
            env.validate_name(&obj, "org/bukkit/event/inventory/PrepareItemCraftEvent")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a PrepareItemCraftEvent object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> PrepareItemCraftEvent<'mc> {
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<crate::inventory::CraftingInventory<'mc>>,
        arg1: impl Into<crate::inventory::InventoryView<'mc>>,
        arg2: bool,
    ) -> Result<crate::event::inventory::PrepareItemCraftEvent<'mc>, Box<dyn std::error::Error>>
    {
        let sig = String::from(
            "(Lorg/bukkit/inventory/CraftingInventory;Lorg/bukkit/inventory/InventoryView;Z)V",
        );
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let val_2 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg1.into().jni_object().clone())
        });
        let val_3 = jni::objects::JValueGen::Bool(arg2.into());
        let cls = jni.find_class("org/bukkit/event/inventory/PrepareItemCraftEvent");
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
        crate::event::inventory::PrepareItemCraftEvent::from_raw(&jni, res)
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

    pub fn inventory(
        &self,
    ) -> Result<crate::inventory::Inventory<'mc>, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Lorg/bukkit/inventory/Inventory;";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getInventory", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::Inventory::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn recipe(
        &self,
    ) -> Result<Option<crate::inventory::Recipe<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/inventory/Recipe;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getRecipe", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {
            return Ok(None);
        }
        Ok(Some(crate::inventory::Recipe::from_raw(
            &self.jni_ref(),
            unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) },
        )?))
    }

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

    pub fn is_repair(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isRepair", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    // SUPER CLASS: InventoryEvent
    pub fn viewers(
        &self,
    ) -> Result<Vec<crate::entity::HumanEntity<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/List;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getViewers", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let list = blackboxmc_java::util::JavaList::from_raw(&self.0, res.l()?)?;
        let iter = list.iterator()?;
        while iter.has_next()? {
            let obj = iter.next()?;
            new_vec.push(crate::entity::HumanEntity::from_raw(&self.0, obj)?);
        }
        Ok(new_vec)
    }
    pub fn view(&self) -> Result<crate::inventory::InventoryView<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::inventory::InventoryEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::inventory::InventoryEvent = temp_clone.into();
        real.view()
    }
    // SUPER CLASS: Event
    pub fn event_name(&self) -> Result<String, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::Event::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::Event = temp_clone.into();
        real.event_name()
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
impl<'mc> Into<crate::event::inventory::InventoryEvent<'mc>> for PrepareItemCraftEvent<'mc> {
    fn into(self) -> crate::event::inventory::InventoryEvent<'mc> {
        crate::event::inventory::InventoryEvent::from_raw(&self.jni_ref(), self.1).expect(
            "Error converting PrepareItemCraftEvent into crate::event::inventory::InventoryEvent",
        )
    }
}
/// Called when an item is put in a slot for repair by an anvil.
#[repr(C)]
pub struct PrepareAnvilEvent<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for PrepareAnvilEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for PrepareAnvilEvent<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate PrepareAnvilEvent from null object.").into(),
            );
        }
        let (valid, name) =
            env.validate_name(&obj, "org/bukkit/event/inventory/PrepareAnvilEvent")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a PrepareAnvilEvent object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> PrepareAnvilEvent<'mc> {
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<crate::inventory::InventoryView<'mc>>,
        arg1: impl Into<crate::inventory::ItemStack<'mc>>,
    ) -> Result<crate::event::inventory::PrepareAnvilEvent<'mc>, Box<dyn std::error::Error>> {
        let sig =
            String::from("(Lorg/bukkit/inventory/InventoryView;Lorg/bukkit/inventory/ItemStack;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let val_2 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg1.into().jni_object().clone())
        });
        let cls = jni.find_class("org/bukkit/event/inventory/PrepareAnvilEvent");
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
        crate::event::inventory::PrepareAnvilEvent::from_raw(&jni, res)
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

    pub fn inventory(
        &self,
    ) -> Result<crate::inventory::AnvilInventory<'mc>, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Lorg/bukkit/inventory/AnvilInventory;";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getInventory", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::AnvilInventory::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

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
    // SUPER CLASS: PrepareInventoryResultEvent
    pub fn result(&self) -> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>> {
        let temp_clone =
            crate::event::inventory::PrepareInventoryResultEvent::from_raw(&self.0, unsafe {
                jni::objects::JObject::from_raw(self.1.clone())
            })?;
        let real: crate::event::inventory::PrepareInventoryResultEvent = temp_clone.into();
        real.result()
    }
    pub fn set_result(
        &self,
        arg0: impl Into<crate::inventory::ItemStack<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone =
            crate::event::inventory::PrepareInventoryResultEvent::from_raw(&self.0, unsafe {
                jni::objects::JObject::from_raw(self.1.clone())
            })?;
        let real: crate::event::inventory::PrepareInventoryResultEvent = temp_clone.into();
        real.set_result(arg0)
    }
    // SUPER CLASS: InventoryEvent
    pub fn viewers(
        &self,
    ) -> Result<Vec<crate::entity::HumanEntity<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/List;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getViewers", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let list = blackboxmc_java::util::JavaList::from_raw(&self.0, res.l()?)?;
        let iter = list.iterator()?;
        while iter.has_next()? {
            let obj = iter.next()?;
            new_vec.push(crate::entity::HumanEntity::from_raw(&self.0, obj)?);
        }
        Ok(new_vec)
    }
    pub fn view(&self) -> Result<crate::inventory::InventoryView<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::inventory::InventoryEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::inventory::InventoryEvent = temp_clone.into();
        real.view()
    }
    // SUPER CLASS: Event
    pub fn event_name(&self) -> Result<String, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::Event::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::Event = temp_clone.into();
        real.event_name()
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
impl<'mc> Into<crate::event::inventory::PrepareInventoryResultEvent<'mc>>
    for PrepareAnvilEvent<'mc>
{
    fn into(self) -> crate::event::inventory::PrepareInventoryResultEvent<'mc> {
        crate::event::inventory::PrepareInventoryResultEvent::from_raw(&self.jni_ref(), self.1).expect("Error converting PrepareAnvilEvent into crate::event::inventory::PrepareInventoryResultEvent")
    }
}
pub enum DragType<'mc> {
    Single { inner: DragTypeStruct<'mc> },
    Even { inner: DragTypeStruct<'mc> },
}
impl<'mc> std::fmt::Display for DragType<'mc> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DragType::Single { .. } => f.write_str("SINGLE"),
            DragType::Even { .. } => f.write_str("EVEN"),
        }
    }
}

impl<'mc> DragType<'mc> {
    pub fn value_of(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<String>,
    ) -> Result<DragType<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(env.new_string(arg0.into())?);
        let cls = env.find_class("org/bukkit/event/inventory/DragType");
        let cls = env.translate_error_with_class(cls)?;
        let res = env.call_static_method(
            cls,
            "valueOf",
            "(Ljava/lang/String;)Lorg/bukkit/event/inventory/DragType;",
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
            "SINGLE" => Ok(DragType::Single {
                inner: DragTypeStruct::from_raw(env, obj)?,
            }),
            "EVEN" => Ok(DragType::Even {
                inner: DragTypeStruct::from_raw(env, obj)?,
            }),

            _ => Err(eyre::eyre!("String gaven for variant was invalid").into()),
        }
    }
}

#[repr(C)]
pub struct DragTypeStruct<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for DragType<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        match self {
            Self::Single { inner } => inner.0.clone(),
            Self::Even { inner } => inner.0.clone(),
        }
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        match self {
            Self::Single { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Even { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
        }
    }
}
impl<'mc> JNIInstantiatable<'mc> for DragType<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate DragType from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/event/inventory/DragType")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a DragType object, got {}",
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
                "SINGLE" => Ok(DragType::Single {
                    inner: DragTypeStruct::from_raw(env, obj)?,
                }),
                "EVEN" => Ok(DragType::Even {
                    inner: DragTypeStruct::from_raw(env, obj)?,
                }),
                _ => Err(eyre::eyre!("String gaven for variant was invalid").into()),
            }
        }
    }
}

impl<'mc> JNIRaw<'mc> for DragTypeStruct<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for DragTypeStruct<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate DragTypeStruct from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/event/inventory/DragType")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a DragTypeStruct object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> DragTypeStruct<'mc> {
    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
/// Called when the brewing of the contents inside the Brewing Stand is complete.
#[repr(C)]
pub struct BrewEvent<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for BrewEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for BrewEvent<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate BrewEvent from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/event/inventory/BrewEvent")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a BrewEvent object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> BrewEvent<'mc> {
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<crate::block::Block<'mc>>,
        arg1: impl Into<crate::inventory::BrewerInventory<'mc>>,
        arg2: Vec<impl Into<crate::event::inventory::BrewEvent<'mc>>>,
        arg3: i32,
    ) -> Result<crate::event::inventory::BrewEvent<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from(
            "(Lorg/bukkit/block/Block;Lorg/bukkit/inventory/BrewerInventory;Ljava/util/List;I)V",
        );
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let val_2 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg1.into().jni_object().clone())
        });
        let raw_val_3 = jni.new_object("java/util/ArrayList", "()V", vec![])?;
        for v in arg2 {
            let map_val_0 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(v.into().jni_object().clone())
            });
            jni.call_method(
                &raw_val_3,
                "add",
                "(Lorg/bukkit/event/inventory/crate::event::inventory::BrewEvent;)Z",
                vec![jni::objects::JValueGen::from(map_val_0)],
            )?;
        }
        let val_3 = jni::objects::JValueGen::Object(raw_val_3);
        let val_4 = jni::objects::JValueGen::Int(arg3);
        let cls = jni.find_class("org/bukkit/event/inventory/BrewEvent");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(
            cls,
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
                jni::objects::JValueGen::from(val_3),
                jni::objects::JValueGen::from(val_4),
            ],
        );
        let res = jni.translate_error_no_gen(res)?;
        crate::event::inventory::BrewEvent::from_raw(&jni, res)
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

    pub fn is_cancelled(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isCancelled", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn contents(
        &self,
    ) -> Result<crate::inventory::BrewerInventory<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/inventory/BrewerInventory;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getContents", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::BrewerInventory::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn fuel_level(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getFuelLevel", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    /// <span class="descfrm-type-label">Description copied from interface:&nbsp;<code><a href="../Cancellable.html#setCancelled(boolean)">Cancellable</a></code></span>
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

    pub fn results(
        &self,
    ) -> Result<Vec<crate::inventory::ItemStack<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/List;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getResults", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let list = blackboxmc_java::util::JavaList::from_raw(&self.0, res.l()?)?;
        let iter = list.iterator()?;
        while iter.has_next()? {
            let obj = iter.next()?;
            new_vec.push(crate::inventory::ItemStack::from_raw(&self.0, obj)?);
        }
        Ok(new_vec)
    }
    // SUPER CLASS: BlockEvent
    pub fn block(&self) -> Result<crate::block::Block<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::block::BlockEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::block::BlockEvent = temp_clone.into();
        real.block()
    }
    // SUPER CLASS: Event
    pub fn event_name(&self) -> Result<String, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::Event::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::Event = temp_clone.into();
        real.event_name()
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
impl<'mc> Into<crate::event::Cancellable<'mc>> for BrewEvent<'mc> {
    fn into(self) -> crate::event::Cancellable<'mc> {
        crate::event::Cancellable::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting BrewEvent into crate::event::Cancellable")
    }
}
impl<'mc> Into<crate::event::block::BlockEvent<'mc>> for BrewEvent<'mc> {
    fn into(self) -> crate::event::block::BlockEvent<'mc> {
        crate::event::block::BlockEvent::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting BrewEvent into crate::event::block::BlockEvent")
    }
}
/// Called when some entity or block (e.g. hopper) tries to move items directly from one inventory to another.
/// <p>When this event is called, the initiator may already have removed the item from the source inventory and is ready to move it into the destination inventory.</p>
/// <p>If this event is cancelled, the items will be returned to the source inventory, if needed.</p>
/// <p>If this event is not cancelled, the initiator will try to put the ItemStack into the destination inventory. If this is not possible and the ItemStack has not been modified, the source inventory slot will be restored to its former state. Otherwise any additional items will be discarded.</p>
#[repr(C)]
pub struct InventoryMoveItemEvent<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for InventoryMoveItemEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for InventoryMoveItemEvent<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate InventoryMoveItemEvent from null object."
            )
            .into());
        }
        let (valid, name) =
            env.validate_name(&obj, "org/bukkit/event/inventory/InventoryMoveItemEvent")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a InventoryMoveItemEvent object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> InventoryMoveItemEvent<'mc> {
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<crate::inventory::Inventory<'mc>>,
        arg1: impl Into<crate::inventory::ItemStack<'mc>>,
        arg2: impl Into<crate::inventory::Inventory<'mc>>,
        arg3: bool,
    ) -> Result<crate::event::inventory::InventoryMoveItemEvent<'mc>, Box<dyn std::error::Error>>
    {
        let sig = String::from("(Lorg/bukkit/inventory/Inventory;Lorg/bukkit/inventory/ItemStack;Lorg/bukkit/inventory/Inventory;Z)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let val_2 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg1.into().jni_object().clone())
        });
        let val_3 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg2.into().jni_object().clone())
        });
        let val_4 = jni::objects::JValueGen::Bool(arg3.into());
        let cls = jni.find_class("org/bukkit/event/inventory/InventoryMoveItemEvent");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(
            cls,
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
                jni::objects::JValueGen::from(val_3),
                jni::objects::JValueGen::from(val_4),
            ],
        );
        let res = jni.translate_error_no_gen(res)?;
        crate::event::inventory::InventoryMoveItemEvent::from_raw(&jni, res)
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

    pub fn is_cancelled(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isCancelled", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn item(&self) -> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/inventory/ItemStack;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getItem", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::ItemStack::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn source(&self) -> Result<crate::inventory::Inventory<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/inventory/Inventory;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getSource", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::Inventory::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn set_item(
        &self,
        arg0: impl Into<crate::inventory::ItemStack<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/inventory/ItemStack;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setItem",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn destination(
        &self,
    ) -> Result<crate::inventory::Inventory<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/inventory/Inventory;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getDestination", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::Inventory::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// <span class="descfrm-type-label">Description copied from interface:&nbsp;<code><a href="../Cancellable.html#setCancelled(boolean)">Cancellable</a></code></span>
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

    pub fn initiator(
        &self,
    ) -> Result<crate::inventory::Inventory<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/inventory/Inventory;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getInitiator", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::Inventory::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    // SUPER CLASS: Event
    pub fn event_name(&self) -> Result<String, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::Event::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::Event = temp_clone.into();
        real.event_name()
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
impl<'mc> Into<crate::event::Cancellable<'mc>> for InventoryMoveItemEvent<'mc> {
    fn into(self) -> crate::event::Cancellable<'mc> {
        crate::event::Cancellable::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting InventoryMoveItemEvent into crate::event::Cancellable")
    }
}
impl<'mc> Into<crate::event::Event<'mc>> for InventoryMoveItemEvent<'mc> {
    fn into(self) -> crate::event::Event<'mc> {
        crate::event::Event::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting InventoryMoveItemEvent into crate::event::Event")
    }
}
/// Called when an ItemStack is successfully smelted in a furnace.
#[repr(C)]
pub struct FurnaceSmeltEvent<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for FurnaceSmeltEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for FurnaceSmeltEvent<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate FurnaceSmeltEvent from null object.").into(),
            );
        }
        let (valid, name) =
            env.validate_name(&obj, "org/bukkit/event/inventory/FurnaceSmeltEvent")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a FurnaceSmeltEvent object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> FurnaceSmeltEvent<'mc> {
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<crate::block::Block<'mc>>,
        arg1: impl Into<crate::inventory::ItemStack<'mc>>,
        arg2: impl Into<crate::inventory::ItemStack<'mc>>,
    ) -> Result<crate::event::inventory::FurnaceSmeltEvent<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/Block;Lorg/bukkit/inventory/ItemStack;Lorg/bukkit/inventory/ItemStack;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let val_2 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg1.into().jni_object().clone())
        });
        let val_3 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg2.into().jni_object().clone())
        });
        let cls = jni.find_class("org/bukkit/event/inventory/FurnaceSmeltEvent");
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
        crate::event::inventory::FurnaceSmeltEvent::from_raw(&jni, res)
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::event::block::BlockCookEvent<'mc>> for FurnaceSmeltEvent<'mc> {
    fn into(self) -> crate::event::block::BlockCookEvent<'mc> {
        crate::event::block::BlockCookEvent::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting FurnaceSmeltEvent into crate::event::block::BlockCookEvent")
    }
}
/// Called when the recipe of an Item is completed inside a smithing table.
#[repr(C)]
pub struct SmithItemEvent<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for SmithItemEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for SmithItemEvent<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate SmithItemEvent from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/event/inventory/SmithItemEvent")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a SmithItemEvent object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> SmithItemEvent<'mc> {
    pub fn new_with_inventory_view(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<crate::inventory::InventoryView<'mc>>,
        arg1: impl Into<crate::event::inventory::InventoryTypeSlotType<'mc>>,
        arg2: i32,
        arg3: impl Into<crate::event::inventory::ClickType<'mc>>,
        arg4: impl Into<crate::event::inventory::InventoryAction<'mc>>,
        arg5: std::option::Option<i32>,
    ) -> Result<crate::event::inventory::SmithItemEvent<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/inventory/InventoryView;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        sig += "Lorg/bukkit/event/inventory/InventoryType$SlotType;";
        let val_2 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg1.into().jni_object().clone())
        });
        args.push(val_2);
        sig += "I";
        let val_3 = jni::objects::JValueGen::Int(arg2);
        args.push(val_3);
        sig += "Lorg/bukkit/event/inventory/ClickType;";
        let val_4 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg3.into().jni_object().clone())
        });
        args.push(val_4);
        sig += "Lorg/bukkit/event/inventory/InventoryAction;";
        let val_5 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg4.into().jni_object().clone())
        });
        args.push(val_5);
        if let Some(a) = arg5 {
            sig += "I";
            let val_6 = jni::objects::JValueGen::Int(a);
            args.push(val_6);
        }
        sig += ")V";
        let cls = jni.find_class("org/bukkit/event/inventory/SmithItemEvent");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(cls, sig.as_str(), args);
        let res = jni.translate_error_no_gen(res)?;
        crate::event::inventory::SmithItemEvent::from_raw(&jni, res)
    }

    pub fn inventory(
        &self,
    ) -> Result<crate::inventory::SmithingInventory<'mc>, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Lorg/bukkit/inventory/SmithingInventory;";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getInventory", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::SmithingInventory::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    // SUPER CLASS: InventoryClickEvent
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
    pub fn set_cursor(
        &self,
        arg0: impl Into<crate::inventory::ItemStack<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::event::inventory::InventoryClickEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::inventory::InventoryClickEvent = temp_clone.into();
        real.set_cursor(arg0)
    }
    pub fn cursor(
        &self,
    ) -> Result<Option<crate::inventory::ItemStack<'mc>>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::inventory::InventoryClickEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::inventory::InventoryClickEvent = temp_clone.into();
        real.cursor()
    }
    pub fn slot_type(
        &self,
    ) -> Result<crate::event::inventory::InventoryTypeSlotType<'mc>, Box<dyn std::error::Error>>
    {
        let temp_clone = crate::event::inventory::InventoryClickEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::inventory::InventoryClickEvent = temp_clone.into();
        real.slot_type()
    }
    pub fn handler_list(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>> {
        crate::event::inventory::InventoryClickEvent::handler_list(jni)
    }
    pub fn action(
        &self,
    ) -> Result<crate::event::inventory::InventoryAction<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::inventory::InventoryClickEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::inventory::InventoryClickEvent = temp_clone.into();
        real.action()
    }
    pub fn is_shift_click(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::inventory::InventoryClickEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::inventory::InventoryClickEvent = temp_clone.into();
        real.is_shift_click()
    }
    pub fn current_item(
        &self,
    ) -> Result<Option<crate::inventory::ItemStack<'mc>>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::inventory::InventoryClickEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::inventory::InventoryClickEvent = temp_clone.into();
        real.current_item()
    }
    pub fn is_right_click(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::inventory::InventoryClickEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::inventory::InventoryClickEvent = temp_clone.into();
        real.is_right_click()
    }
    pub fn is_left_click(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::inventory::InventoryClickEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::inventory::InventoryClickEvent = temp_clone.into();
        real.is_left_click()
    }
    pub fn set_current_item(
        &self,
        arg0: impl Into<crate::inventory::ItemStack<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::event::inventory::InventoryClickEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::inventory::InventoryClickEvent = temp_clone.into();
        real.set_current_item(arg0)
    }
    pub fn clicked_inventory(
        &self,
    ) -> Result<Option<crate::inventory::Inventory<'mc>>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::inventory::InventoryClickEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::inventory::InventoryClickEvent = temp_clone.into();
        real.clicked_inventory()
    }
    pub fn raw_slot(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::inventory::InventoryClickEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::inventory::InventoryClickEvent = temp_clone.into();
        real.raw_slot()
    }
    pub fn hotbar_button(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::inventory::InventoryClickEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::inventory::InventoryClickEvent = temp_clone.into();
        real.hotbar_button()
    }
    pub fn click(
        &self,
    ) -> Result<Option<crate::event::inventory::ClickType<'mc>>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::inventory::InventoryClickEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::inventory::InventoryClickEvent = temp_clone.into();
        real.click()
    }
    pub fn slot(&self) -> Result<Option<i32>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::inventory::InventoryClickEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::inventory::InventoryClickEvent = temp_clone.into();
        real.slot()
    }
    // SUPER CLASS: InventoryInteractEvent
    pub fn is_cancelled(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone =
            crate::event::inventory::InventoryInteractEvent::from_raw(&self.0, unsafe {
                jni::objects::JObject::from_raw(self.1.clone())
            })?;
        let real: crate::event::Cancellable = temp_clone.into();
        real.is_cancelled()
    }
    pub fn set_cancelled(&self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone =
            crate::event::inventory::InventoryInteractEvent::from_raw(&self.0, unsafe {
                jni::objects::JObject::from_raw(self.1.clone())
            })?;
        let real: crate::event::Cancellable = temp_clone.into();
        real.set_cancelled(arg0)
    }
    pub fn result(&self) -> Result<crate::event::EventResult<'mc>, Box<dyn std::error::Error>> {
        let temp_clone =
            crate::event::inventory::InventoryInteractEvent::from_raw(&self.0, unsafe {
                jni::objects::JObject::from_raw(self.1.clone())
            })?;
        let real: crate::event::inventory::InventoryInteractEvent = temp_clone.into();
        real.result()
    }
    pub fn set_result(
        &self,
        arg0: impl Into<crate::event::EventResult<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone =
            crate::event::inventory::InventoryInteractEvent::from_raw(&self.0, unsafe {
                jni::objects::JObject::from_raw(self.1.clone())
            })?;
        let real: crate::event::inventory::InventoryInteractEvent = temp_clone.into();
        real.set_result(arg0)
    }
    pub fn who_clicked(
        &self,
    ) -> Result<crate::entity::HumanEntity<'mc>, Box<dyn std::error::Error>> {
        let temp_clone =
            crate::event::inventory::InventoryInteractEvent::from_raw(&self.0, unsafe {
                jni::objects::JObject::from_raw(self.1.clone())
            })?;
        let real: crate::event::inventory::InventoryInteractEvent = temp_clone.into();
        real.who_clicked()
    }
    // SUPER CLASS: InventoryEvent
    pub fn viewers(
        &self,
    ) -> Result<Vec<crate::entity::HumanEntity<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/List;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getViewers", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let list = blackboxmc_java::util::JavaList::from_raw(&self.0, res.l()?)?;
        let iter = list.iterator()?;
        while iter.has_next()? {
            let obj = iter.next()?;
            new_vec.push(crate::entity::HumanEntity::from_raw(&self.0, obj)?);
        }
        Ok(new_vec)
    }
    pub fn view(&self) -> Result<crate::inventory::InventoryView<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::inventory::InventoryEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::inventory::InventoryEvent = temp_clone.into();
        real.view()
    }
    // SUPER CLASS: Event
    pub fn event_name(&self) -> Result<String, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::Event::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::Event = temp_clone.into();
        real.event_name()
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
impl<'mc> Into<crate::event::inventory::InventoryClickEvent<'mc>> for SmithItemEvent<'mc> {
    fn into(self) -> crate::event::inventory::InventoryClickEvent<'mc> {
        crate::event::inventory::InventoryClickEvent::from_raw(&self.jni_ref(), self.1).expect(
            "Error converting SmithItemEvent into crate::event::inventory::InventoryClickEvent",
        )
    }
}
pub enum ClickType<'mc> {
    Left { inner: ClickTypeStruct<'mc> },
    ShiftLeft { inner: ClickTypeStruct<'mc> },
    Right { inner: ClickTypeStruct<'mc> },
    ShiftRight { inner: ClickTypeStruct<'mc> },
    WindowBorderLeft { inner: ClickTypeStruct<'mc> },
    WindowBorderRight { inner: ClickTypeStruct<'mc> },
    Middle { inner: ClickTypeStruct<'mc> },
    NumberKey { inner: ClickTypeStruct<'mc> },
    DoubleClick { inner: ClickTypeStruct<'mc> },
    Drop { inner: ClickTypeStruct<'mc> },
    ControlDrop { inner: ClickTypeStruct<'mc> },
    Creative { inner: ClickTypeStruct<'mc> },
    SwapOffhand { inner: ClickTypeStruct<'mc> },
    Unknown { inner: ClickTypeStruct<'mc> },
}
impl<'mc> std::fmt::Display for ClickType<'mc> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ClickType::Left { .. } => f.write_str("LEFT"),
            ClickType::ShiftLeft { .. } => f.write_str("SHIFT_LEFT"),
            ClickType::Right { .. } => f.write_str("RIGHT"),
            ClickType::ShiftRight { .. } => f.write_str("SHIFT_RIGHT"),
            ClickType::WindowBorderLeft { .. } => f.write_str("WINDOW_BORDER_LEFT"),
            ClickType::WindowBorderRight { .. } => f.write_str("WINDOW_BORDER_RIGHT"),
            ClickType::Middle { .. } => f.write_str("MIDDLE"),
            ClickType::NumberKey { .. } => f.write_str("NUMBER_KEY"),
            ClickType::DoubleClick { .. } => f.write_str("DOUBLE_CLICK"),
            ClickType::Drop { .. } => f.write_str("DROP"),
            ClickType::ControlDrop { .. } => f.write_str("CONTROL_DROP"),
            ClickType::Creative { .. } => f.write_str("CREATIVE"),
            ClickType::SwapOffhand { .. } => f.write_str("SWAP_OFFHAND"),
            ClickType::Unknown { .. } => f.write_str("UNKNOWN"),
        }
    }
}

impl<'mc> ClickType<'mc> {
    pub fn value_of(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<String>,
    ) -> Result<ClickType<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(env.new_string(arg0.into())?);
        let cls = env.find_class("org/bukkit/event/inventory/ClickType");
        let cls = env.translate_error_with_class(cls)?;
        let res = env.call_static_method(
            cls,
            "valueOf",
            "(Ljava/lang/String;)Lorg/bukkit/event/inventory/ClickType;",
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
            "LEFT" => Ok(ClickType::Left {
                inner: ClickTypeStruct::from_raw(env, obj)?,
            }),
            "SHIFT_LEFT" => Ok(ClickType::ShiftLeft {
                inner: ClickTypeStruct::from_raw(env, obj)?,
            }),
            "RIGHT" => Ok(ClickType::Right {
                inner: ClickTypeStruct::from_raw(env, obj)?,
            }),
            "SHIFT_RIGHT" => Ok(ClickType::ShiftRight {
                inner: ClickTypeStruct::from_raw(env, obj)?,
            }),
            "WINDOW_BORDER_LEFT" => Ok(ClickType::WindowBorderLeft {
                inner: ClickTypeStruct::from_raw(env, obj)?,
            }),
            "WINDOW_BORDER_RIGHT" => Ok(ClickType::WindowBorderRight {
                inner: ClickTypeStruct::from_raw(env, obj)?,
            }),
            "MIDDLE" => Ok(ClickType::Middle {
                inner: ClickTypeStruct::from_raw(env, obj)?,
            }),
            "NUMBER_KEY" => Ok(ClickType::NumberKey {
                inner: ClickTypeStruct::from_raw(env, obj)?,
            }),
            "DOUBLE_CLICK" => Ok(ClickType::DoubleClick {
                inner: ClickTypeStruct::from_raw(env, obj)?,
            }),
            "DROP" => Ok(ClickType::Drop {
                inner: ClickTypeStruct::from_raw(env, obj)?,
            }),
            "CONTROL_DROP" => Ok(ClickType::ControlDrop {
                inner: ClickTypeStruct::from_raw(env, obj)?,
            }),
            "CREATIVE" => Ok(ClickType::Creative {
                inner: ClickTypeStruct::from_raw(env, obj)?,
            }),
            "SWAP_OFFHAND" => Ok(ClickType::SwapOffhand {
                inner: ClickTypeStruct::from_raw(env, obj)?,
            }),
            "UNKNOWN" => Ok(ClickType::Unknown {
                inner: ClickTypeStruct::from_raw(env, obj)?,
            }),

            _ => Err(eyre::eyre!("String gaven for variant was invalid").into()),
        }
    }
}

#[repr(C)]
pub struct ClickTypeStruct<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for ClickType<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        match self {
            Self::Left { inner } => inner.0.clone(),
            Self::ShiftLeft { inner } => inner.0.clone(),
            Self::Right { inner } => inner.0.clone(),
            Self::ShiftRight { inner } => inner.0.clone(),
            Self::WindowBorderLeft { inner } => inner.0.clone(),
            Self::WindowBorderRight { inner } => inner.0.clone(),
            Self::Middle { inner } => inner.0.clone(),
            Self::NumberKey { inner } => inner.0.clone(),
            Self::DoubleClick { inner } => inner.0.clone(),
            Self::Drop { inner } => inner.0.clone(),
            Self::ControlDrop { inner } => inner.0.clone(),
            Self::Creative { inner } => inner.0.clone(),
            Self::SwapOffhand { inner } => inner.0.clone(),
            Self::Unknown { inner } => inner.0.clone(),
        }
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        match self {
            Self::Left { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::ShiftLeft { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::Right { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::ShiftRight { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::WindowBorderLeft { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::WindowBorderRight { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::Middle { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::NumberKey { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::DoubleClick { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::Drop { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::ControlDrop { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::Creative { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::SwapOffhand { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::Unknown { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
        }
    }
}
impl<'mc> JNIInstantiatable<'mc> for ClickType<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate ClickType from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/event/inventory/ClickType")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a ClickType object, got {}",
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
                "LEFT" => Ok(ClickType::Left {
                    inner: ClickTypeStruct::from_raw(env, obj)?,
                }),
                "SHIFT_LEFT" => Ok(ClickType::ShiftLeft {
                    inner: ClickTypeStruct::from_raw(env, obj)?,
                }),
                "RIGHT" => Ok(ClickType::Right {
                    inner: ClickTypeStruct::from_raw(env, obj)?,
                }),
                "SHIFT_RIGHT" => Ok(ClickType::ShiftRight {
                    inner: ClickTypeStruct::from_raw(env, obj)?,
                }),
                "WINDOW_BORDER_LEFT" => Ok(ClickType::WindowBorderLeft {
                    inner: ClickTypeStruct::from_raw(env, obj)?,
                }),
                "WINDOW_BORDER_RIGHT" => Ok(ClickType::WindowBorderRight {
                    inner: ClickTypeStruct::from_raw(env, obj)?,
                }),
                "MIDDLE" => Ok(ClickType::Middle {
                    inner: ClickTypeStruct::from_raw(env, obj)?,
                }),
                "NUMBER_KEY" => Ok(ClickType::NumberKey {
                    inner: ClickTypeStruct::from_raw(env, obj)?,
                }),
                "DOUBLE_CLICK" => Ok(ClickType::DoubleClick {
                    inner: ClickTypeStruct::from_raw(env, obj)?,
                }),
                "DROP" => Ok(ClickType::Drop {
                    inner: ClickTypeStruct::from_raw(env, obj)?,
                }),
                "CONTROL_DROP" => Ok(ClickType::ControlDrop {
                    inner: ClickTypeStruct::from_raw(env, obj)?,
                }),
                "CREATIVE" => Ok(ClickType::Creative {
                    inner: ClickTypeStruct::from_raw(env, obj)?,
                }),
                "SWAP_OFFHAND" => Ok(ClickType::SwapOffhand {
                    inner: ClickTypeStruct::from_raw(env, obj)?,
                }),
                "UNKNOWN" => Ok(ClickType::Unknown {
                    inner: ClickTypeStruct::from_raw(env, obj)?,
                }),
                _ => Err(eyre::eyre!("String gaven for variant was invalid").into()),
            }
        }
    }
}

impl<'mc> JNIRaw<'mc> for ClickTypeStruct<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for ClickTypeStruct<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate ClickTypeStruct from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/event/inventory/ClickType")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a ClickTypeStruct object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> ClickTypeStruct<'mc> {
    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
/// Called when an ItemStack is about to increase the fuel level of a brewing stand.
#[repr(C)]
pub struct BrewingStandFuelEvent<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for BrewingStandFuelEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for BrewingStandFuelEvent<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate BrewingStandFuelEvent from null object."
            )
            .into());
        }
        let (valid, name) =
            env.validate_name(&obj, "org/bukkit/event/inventory/BrewingStandFuelEvent")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a BrewingStandFuelEvent object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> BrewingStandFuelEvent<'mc> {
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<crate::block::Block<'mc>>,
        arg1: impl Into<crate::inventory::ItemStack<'mc>>,
        arg2: i32,
    ) -> Result<crate::event::inventory::BrewingStandFuelEvent<'mc>, Box<dyn std::error::Error>>
    {
        let sig = String::from("(Lorg/bukkit/block/Block;Lorg/bukkit/inventory/ItemStack;I)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let val_2 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg1.into().jni_object().clone())
        });
        let val_3 = jni::objects::JValueGen::Int(arg2);
        let cls = jni.find_class("org/bukkit/event/inventory/BrewingStandFuelEvent");
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
        crate::event::inventory::BrewingStandFuelEvent::from_raw(&jni, res)
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

    pub fn is_cancelled(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isCancelled", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn fuel(
        &self,
    ) -> Result<Option<crate::inventory::ItemStack<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/inventory/ItemStack;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getFuel", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {
            return Ok(None);
        }
        Ok(Some(crate::inventory::ItemStack::from_raw(
            &self.jni_ref(),
            unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) },
        )?))
    }
    /// <span class="descfrm-type-label">Description copied from interface:&nbsp;<code><a href="../Cancellable.html#setCancelled(boolean)">Cancellable</a></code></span>
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

    pub fn fuel_power(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getFuelPower", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    /// Sets the fuel power for this fuel. Each unit of power can fuel one brewing operation.
    pub fn set_fuel_power(&self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(I)V");
        let val_1 = jni::objects::JValueGen::Int(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setFuelPower",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn is_consuming(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isConsuming", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Sets whether the brewing stand's fuel will be reduced / consumed or not.
    pub fn set_consuming(&self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        let val_1 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setConsuming",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    // SUPER CLASS: BlockEvent
    pub fn block(&self) -> Result<crate::block::Block<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::block::BlockEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::block::BlockEvent = temp_clone.into();
        real.block()
    }
    // SUPER CLASS: Event
    pub fn event_name(&self) -> Result<String, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::Event::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::Event = temp_clone.into();
        real.event_name()
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
impl<'mc> Into<crate::event::Cancellable<'mc>> for BrewingStandFuelEvent<'mc> {
    fn into(self) -> crate::event::Cancellable<'mc> {
        crate::event::Cancellable::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting BrewingStandFuelEvent into crate::event::Cancellable")
    }
}
impl<'mc> Into<crate::event::block::BlockEvent<'mc>> for BrewingStandFuelEvent<'mc> {
    fn into(self) -> crate::event::block::BlockEvent<'mc> {
        crate::event::block::BlockEvent::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting BrewingStandFuelEvent into crate::event::block::BlockEvent")
    }
}
/// Called when an item is put in a slot for repair or unenchanting in a grindstone.
#[repr(C)]
pub struct PrepareGrindstoneEvent<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for PrepareGrindstoneEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for PrepareGrindstoneEvent<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate PrepareGrindstoneEvent from null object."
            )
            .into());
        }
        let (valid, name) =
            env.validate_name(&obj, "org/bukkit/event/inventory/PrepareGrindstoneEvent")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a PrepareGrindstoneEvent object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> PrepareGrindstoneEvent<'mc> {
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<crate::inventory::InventoryView<'mc>>,
        arg1: impl Into<crate::inventory::ItemStack<'mc>>,
    ) -> Result<crate::event::inventory::PrepareGrindstoneEvent<'mc>, Box<dyn std::error::Error>>
    {
        let sig =
            String::from("(Lorg/bukkit/inventory/InventoryView;Lorg/bukkit/inventory/ItemStack;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let val_2 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg1.into().jni_object().clone())
        });
        let cls = jni.find_class("org/bukkit/event/inventory/PrepareGrindstoneEvent");
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
        crate::event::inventory::PrepareGrindstoneEvent::from_raw(&jni, res)
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

    pub fn inventory(
        &self,
    ) -> Result<crate::inventory::GrindstoneInventory<'mc>, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Lorg/bukkit/inventory/GrindstoneInventory;";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getInventory", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::GrindstoneInventory::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

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
    // SUPER CLASS: PrepareInventoryResultEvent
    pub fn result(&self) -> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>> {
        let temp_clone =
            crate::event::inventory::PrepareInventoryResultEvent::from_raw(&self.0, unsafe {
                jni::objects::JObject::from_raw(self.1.clone())
            })?;
        let real: crate::event::inventory::PrepareInventoryResultEvent = temp_clone.into();
        real.result()
    }
    pub fn set_result(
        &self,
        arg0: impl Into<crate::inventory::ItemStack<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone =
            crate::event::inventory::PrepareInventoryResultEvent::from_raw(&self.0, unsafe {
                jni::objects::JObject::from_raw(self.1.clone())
            })?;
        let real: crate::event::inventory::PrepareInventoryResultEvent = temp_clone.into();
        real.set_result(arg0)
    }
    // SUPER CLASS: InventoryEvent
    pub fn viewers(
        &self,
    ) -> Result<Vec<crate::entity::HumanEntity<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/List;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getViewers", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let list = blackboxmc_java::util::JavaList::from_raw(&self.0, res.l()?)?;
        let iter = list.iterator()?;
        while iter.has_next()? {
            let obj = iter.next()?;
            new_vec.push(crate::entity::HumanEntity::from_raw(&self.0, obj)?);
        }
        Ok(new_vec)
    }
    pub fn view(&self) -> Result<crate::inventory::InventoryView<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::inventory::InventoryEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::inventory::InventoryEvent = temp_clone.into();
        real.view()
    }
    // SUPER CLASS: Event
    pub fn event_name(&self) -> Result<String, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::Event::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::Event = temp_clone.into();
        real.event_name()
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
impl<'mc> Into<crate::event::inventory::PrepareInventoryResultEvent<'mc>>
    for PrepareGrindstoneEvent<'mc>
{
    fn into(self) -> crate::event::inventory::PrepareInventoryResultEvent<'mc> {
        crate::event::inventory::PrepareInventoryResultEvent::from_raw(&self.jni_ref(), self.1).expect("Error converting PrepareGrindstoneEvent into crate::event::inventory::PrepareInventoryResultEvent")
    }
}
/// Event that gets called each time a Hopper attempts to find its source/attached containers.
#[repr(C)]
pub struct HopperInventorySearchEvent<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
pub enum HopperInventorySearchEventContainerType<'mc> {
    Source {
        inner: HopperInventorySearchEventContainerTypeStruct<'mc>,
    },
    Destination {
        inner: HopperInventorySearchEventContainerTypeStruct<'mc>,
    },
}
impl<'mc> std::fmt::Display for HopperInventorySearchEventContainerType<'mc> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HopperInventorySearchEventContainerType::Source { .. } => f.write_str("SOURCE"),
            HopperInventorySearchEventContainerType::Destination { .. } => {
                f.write_str("DESTINATION")
            }
        }
    }
}

impl<'mc> HopperInventorySearchEventContainerType<'mc> {
    pub fn value_of(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<String>,
    ) -> Result<HopperInventorySearchEventContainerType<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(env.new_string(arg0.into())?);
        let cls =
            env.find_class("org/bukkit/event/inventory/HopperInventorySearchEvent$ContainerType");
        let cls = env.translate_error_with_class(cls)?;
        let res = env.call_static_method(
                    cls,
                    "valueOf",
                    "(Ljava/lang/String;)Lorg/bukkit/event/inventory/HopperInventorySearchEvent$ContainerType;",
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
            "SOURCE" => Ok(HopperInventorySearchEventContainerType::Source {
                inner: HopperInventorySearchEventContainerTypeStruct::from_raw(env, obj)?,
            }),
            "DESTINATION" => Ok(HopperInventorySearchEventContainerType::Destination {
                inner: HopperInventorySearchEventContainerTypeStruct::from_raw(env, obj)?,
            }),

            _ => Err(eyre::eyre!("String gaven for variant was invalid").into()),
        }
    }
}

#[repr(C)]
pub struct HopperInventorySearchEventContainerTypeStruct<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for HopperInventorySearchEventContainerType<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        match self {
            Self::Source { inner } => inner.0.clone(),
            Self::Destination { inner } => inner.0.clone(),
        }
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        match self {
            Self::Source { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Destination { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
        }
    }
}
impl<'mc> JNIInstantiatable<'mc> for HopperInventorySearchEventContainerType<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate HopperInventorySearchEventContainerType from null object."
            )
            .into());
        }
        let (valid, name) = env.validate_name(
            &obj,
            "org/bukkit/event/inventory/HopperInventorySearchEvent$ContainerType",
        )?;
        if !valid {
            Err(eyre::eyre!(
                    "Invalid argument passed. Expected a HopperInventorySearchEventContainerType object, got {}",
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
                "SOURCE" => Ok(HopperInventorySearchEventContainerType::Source {
                    inner: HopperInventorySearchEventContainerTypeStruct::from_raw(env, obj)?,
                }),
                "DESTINATION" => Ok(HopperInventorySearchEventContainerType::Destination {
                    inner: HopperInventorySearchEventContainerTypeStruct::from_raw(env, obj)?,
                }),
                _ => Err(eyre::eyre!("String gaven for variant was invalid").into()),
            }
        }
    }
}

impl<'mc> JNIRaw<'mc> for HopperInventorySearchEventContainerTypeStruct<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for HopperInventorySearchEventContainerTypeStruct<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                    "Tried to instantiate HopperInventorySearchEventContainerTypeStruct from null object.")
                .into());
        }
        let (valid, name) = env.validate_name(
            &obj,
            "org/bukkit/event/inventory/HopperInventorySearchEvent$ContainerType",
        )?;
        if !valid {
            Err(eyre::eyre!(
                    "Invalid argument passed. Expected a HopperInventorySearchEventContainerTypeStruct object, got {}",
                    name
                )
                .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> HopperInventorySearchEventContainerTypeStruct<'mc> {
    pub fn values(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<
        Vec<crate::event::inventory::HopperInventorySearchEventContainerType<'mc>>,
        Box<dyn std::error::Error>,
    > {
        let sig =
            String::from("()Lorg/bukkit/event/inventory/HopperInventorySearchEvent$ContainerType;");
        let cls =
            jni.find_class("org/bukkit/event/inventory/HopperInventorySearchEvent$ContainerType");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "values", sig.as_str(), vec![]);
        let res = jni.translate_error(res)?;
        let arr = Into::<jni::objects::JObjectArray>::into(res.l()?);
        let len = jni.get_array_length(&arr)?;
        let mut vec = Vec::new();
        for i in 0..len {
            let res = jni.get_object_array_element(&arr, i)?;
            vec.push({
                crate::event::inventory::HopperInventorySearchEventContainerType::from_raw(
                    &jni, res,
                )?
            });
        }
        Ok(vec)
    }
    // SUPER CLASS: Enum

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}

impl<'mc> JNIRaw<'mc> for HopperInventorySearchEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for HopperInventorySearchEvent<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate HopperInventorySearchEvent from null object."
            )
            .into());
        }
        let (valid, name) = env.validate_name(
            &obj,
            "org/bukkit/event/inventory/HopperInventorySearchEvent",
        )?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a HopperInventorySearchEvent object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> HopperInventorySearchEvent<'mc> {
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<crate::inventory::Inventory<'mc>>,
        arg1: impl Into<crate::event::inventory::HopperInventorySearchEventContainerType<'mc>>,
        arg2: impl Into<crate::block::Block<'mc>>,
        arg3: impl Into<crate::block::Block<'mc>>,
    ) -> Result<crate::event::inventory::HopperInventorySearchEvent<'mc>, Box<dyn std::error::Error>>
    {
        let sig = String::from("(Lorg/bukkit/inventory/Inventory;Lorg/bukkit/event/inventory/HopperInventorySearchEvent$ContainerType;Lorg/bukkit/block/Block;Lorg/bukkit/block/Block;)V");
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
        let cls = jni.find_class("org/bukkit/event/inventory/HopperInventorySearchEvent");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(
            cls,
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
                jni::objects::JValueGen::from(val_3),
                jni::objects::JValueGen::from(val_4),
            ],
        );
        let res = jni.translate_error_no_gen(res)?;
        crate::event::inventory::HopperInventorySearchEvent::from_raw(&jni, res)
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

    pub fn inventory(
        &self,
    ) -> Result<crate::inventory::Inventory<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/inventory/Inventory;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getInventory", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::Inventory::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

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

    pub fn set_inventory(
        &self,
        arg0: impl Into<crate::inventory::Inventory<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/inventory/Inventory;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setInventory",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn container_type(
        &self,
    ) -> Result<
        crate::event::inventory::HopperInventorySearchEventContainerType<'mc>,
        Box<dyn std::error::Error>,
    > {
        let sig =
            String::from("()Lorg/bukkit/event/inventory/HopperInventorySearchEvent$ContainerType;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getContainerType",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::event::inventory::HopperInventorySearchEventContainerType::from_raw(
            &self.jni_ref(),
            unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) },
        )
    }

    pub fn search_block(&self) -> Result<crate::block::Block<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/block/Block;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getSearchBlock", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::block::Block::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    // SUPER CLASS: BlockEvent
    pub fn block(&self) -> Result<crate::block::Block<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::block::BlockEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::block::BlockEvent = temp_clone.into();
        real.block()
    }
    // SUPER CLASS: Event
    pub fn event_name(&self) -> Result<String, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::Event::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::Event = temp_clone.into();
        real.event_name()
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
impl<'mc> Into<crate::event::block::BlockEvent<'mc>> for HopperInventorySearchEvent<'mc> {
    fn into(self) -> crate::event::block::BlockEvent<'mc> {
        crate::event::block::BlockEvent::from_raw(&self.jni_ref(), self.1).expect(
            "Error converting HopperInventorySearchEvent into crate::event::block::BlockEvent",
        )
    }
}
/// Called when the recipe of an Item is completed inside a crafting matrix.
#[repr(C)]
pub struct CraftItemEvent<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for CraftItemEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for CraftItemEvent<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate CraftItemEvent from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/event/inventory/CraftItemEvent")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a CraftItemEvent object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> CraftItemEvent<'mc> {
    pub fn new_with_recipe(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<crate::inventory::Recipe<'mc>>,
        arg1: impl Into<crate::inventory::InventoryView<'mc>>,
        arg2: impl Into<crate::event::inventory::InventoryTypeSlotType<'mc>>,
        arg3: i32,
        arg4: impl Into<crate::event::inventory::ClickType<'mc>>,
        arg5: impl Into<crate::event::inventory::InventoryAction<'mc>>,
        arg6: std::option::Option<i32>,
    ) -> Result<crate::event::inventory::CraftItemEvent<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/inventory/Recipe;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        sig += "Lorg/bukkit/inventory/InventoryView;";
        let val_2 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg1.into().jni_object().clone())
        });
        args.push(val_2);
        sig += "Lorg/bukkit/event/inventory/InventoryType$SlotType;";
        let val_3 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg2.into().jni_object().clone())
        });
        args.push(val_3);
        sig += "I";
        let val_4 = jni::objects::JValueGen::Int(arg3);
        args.push(val_4);
        sig += "Lorg/bukkit/event/inventory/ClickType;";
        let val_5 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg4.into().jni_object().clone())
        });
        args.push(val_5);
        sig += "Lorg/bukkit/event/inventory/InventoryAction;";
        let val_6 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg5.into().jni_object().clone())
        });
        args.push(val_6);
        if let Some(a) = arg6 {
            sig += "I";
            let val_7 = jni::objects::JValueGen::Int(a);
            args.push(val_7);
        }
        sig += ")V";
        let cls = jni.find_class("org/bukkit/event/inventory/CraftItemEvent");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(cls, sig.as_str(), args);
        let res = jni.translate_error_no_gen(res)?;
        crate::event::inventory::CraftItemEvent::from_raw(&jni, res)
    }

    pub fn inventory(
        &self,
    ) -> Result<crate::inventory::CraftingInventory<'mc>, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Lorg/bukkit/inventory/CraftingInventory;";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getInventory", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::CraftingInventory::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn recipe(
        &self,
    ) -> Result<Option<crate::inventory::Recipe<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/inventory/Recipe;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getRecipe", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {
            return Ok(None);
        }
        Ok(Some(crate::inventory::Recipe::from_raw(
            &self.jni_ref(),
            unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) },
        )?))
    }
    // SUPER CLASS: InventoryClickEvent
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
    pub fn set_cursor(
        &self,
        arg0: impl Into<crate::inventory::ItemStack<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::event::inventory::InventoryClickEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::inventory::InventoryClickEvent = temp_clone.into();
        real.set_cursor(arg0)
    }
    pub fn cursor(
        &self,
    ) -> Result<Option<crate::inventory::ItemStack<'mc>>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::inventory::InventoryClickEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::inventory::InventoryClickEvent = temp_clone.into();
        real.cursor()
    }
    pub fn slot_type(
        &self,
    ) -> Result<crate::event::inventory::InventoryTypeSlotType<'mc>, Box<dyn std::error::Error>>
    {
        let temp_clone = crate::event::inventory::InventoryClickEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::inventory::InventoryClickEvent = temp_clone.into();
        real.slot_type()
    }
    pub fn handler_list(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>> {
        crate::event::inventory::InventoryClickEvent::handler_list(jni)
    }
    pub fn action(
        &self,
    ) -> Result<crate::event::inventory::InventoryAction<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::inventory::InventoryClickEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::inventory::InventoryClickEvent = temp_clone.into();
        real.action()
    }
    pub fn is_shift_click(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::inventory::InventoryClickEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::inventory::InventoryClickEvent = temp_clone.into();
        real.is_shift_click()
    }
    pub fn current_item(
        &self,
    ) -> Result<Option<crate::inventory::ItemStack<'mc>>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::inventory::InventoryClickEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::inventory::InventoryClickEvent = temp_clone.into();
        real.current_item()
    }
    pub fn is_right_click(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::inventory::InventoryClickEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::inventory::InventoryClickEvent = temp_clone.into();
        real.is_right_click()
    }
    pub fn is_left_click(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::inventory::InventoryClickEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::inventory::InventoryClickEvent = temp_clone.into();
        real.is_left_click()
    }
    pub fn set_current_item(
        &self,
        arg0: impl Into<crate::inventory::ItemStack<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::event::inventory::InventoryClickEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::inventory::InventoryClickEvent = temp_clone.into();
        real.set_current_item(arg0)
    }
    pub fn clicked_inventory(
        &self,
    ) -> Result<Option<crate::inventory::Inventory<'mc>>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::inventory::InventoryClickEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::inventory::InventoryClickEvent = temp_clone.into();
        real.clicked_inventory()
    }
    pub fn raw_slot(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::inventory::InventoryClickEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::inventory::InventoryClickEvent = temp_clone.into();
        real.raw_slot()
    }
    pub fn hotbar_button(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::inventory::InventoryClickEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::inventory::InventoryClickEvent = temp_clone.into();
        real.hotbar_button()
    }
    pub fn click(
        &self,
    ) -> Result<Option<crate::event::inventory::ClickType<'mc>>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::inventory::InventoryClickEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::inventory::InventoryClickEvent = temp_clone.into();
        real.click()
    }
    pub fn slot(&self) -> Result<Option<i32>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::inventory::InventoryClickEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::inventory::InventoryClickEvent = temp_clone.into();
        real.slot()
    }
    // SUPER CLASS: InventoryInteractEvent
    pub fn is_cancelled(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone =
            crate::event::inventory::InventoryInteractEvent::from_raw(&self.0, unsafe {
                jni::objects::JObject::from_raw(self.1.clone())
            })?;
        let real: crate::event::Cancellable = temp_clone.into();
        real.is_cancelled()
    }
    pub fn set_cancelled(&self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone =
            crate::event::inventory::InventoryInteractEvent::from_raw(&self.0, unsafe {
                jni::objects::JObject::from_raw(self.1.clone())
            })?;
        let real: crate::event::Cancellable = temp_clone.into();
        real.set_cancelled(arg0)
    }
    pub fn result(&self) -> Result<crate::event::EventResult<'mc>, Box<dyn std::error::Error>> {
        let temp_clone =
            crate::event::inventory::InventoryInteractEvent::from_raw(&self.0, unsafe {
                jni::objects::JObject::from_raw(self.1.clone())
            })?;
        let real: crate::event::inventory::InventoryInteractEvent = temp_clone.into();
        real.result()
    }
    pub fn set_result(
        &self,
        arg0: impl Into<crate::event::EventResult<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone =
            crate::event::inventory::InventoryInteractEvent::from_raw(&self.0, unsafe {
                jni::objects::JObject::from_raw(self.1.clone())
            })?;
        let real: crate::event::inventory::InventoryInteractEvent = temp_clone.into();
        real.set_result(arg0)
    }
    pub fn who_clicked(
        &self,
    ) -> Result<crate::entity::HumanEntity<'mc>, Box<dyn std::error::Error>> {
        let temp_clone =
            crate::event::inventory::InventoryInteractEvent::from_raw(&self.0, unsafe {
                jni::objects::JObject::from_raw(self.1.clone())
            })?;
        let real: crate::event::inventory::InventoryInteractEvent = temp_clone.into();
        real.who_clicked()
    }
    // SUPER CLASS: InventoryEvent
    pub fn viewers(
        &self,
    ) -> Result<Vec<crate::entity::HumanEntity<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/List;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getViewers", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let list = blackboxmc_java::util::JavaList::from_raw(&self.0, res.l()?)?;
        let iter = list.iterator()?;
        while iter.has_next()? {
            let obj = iter.next()?;
            new_vec.push(crate::entity::HumanEntity::from_raw(&self.0, obj)?);
        }
        Ok(new_vec)
    }
    pub fn view(&self) -> Result<crate::inventory::InventoryView<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::inventory::InventoryEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::inventory::InventoryEvent = temp_clone.into();
        real.view()
    }
    // SUPER CLASS: Event
    pub fn event_name(&self) -> Result<String, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::Event::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::Event = temp_clone.into();
        real.event_name()
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
impl<'mc> Into<crate::event::inventory::InventoryClickEvent<'mc>> for CraftItemEvent<'mc> {
    fn into(self) -> crate::event::inventory::InventoryClickEvent<'mc> {
        crate::event::inventory::InventoryClickEvent::from_raw(&self.jni_ref(), self.1).expect(
            "Error converting CraftItemEvent into crate::event::inventory::InventoryClickEvent",
        )
    }
}
/// Called when a Furnace starts smelting.
#[repr(C)]
pub struct FurnaceStartSmeltEvent<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for FurnaceStartSmeltEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for FurnaceStartSmeltEvent<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate FurnaceStartSmeltEvent from null object."
            )
            .into());
        }
        let (valid, name) =
            env.validate_name(&obj, "org/bukkit/event/inventory/FurnaceStartSmeltEvent")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a FurnaceStartSmeltEvent object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> FurnaceStartSmeltEvent<'mc> {
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<crate::block::Block<'mc>>,
        arg1: impl Into<crate::inventory::ItemStack<'mc>>,
        arg2: impl Into<crate::inventory::CookingRecipe<'mc>>,
    ) -> Result<crate::event::inventory::FurnaceStartSmeltEvent<'mc>, Box<dyn std::error::Error>>
    {
        let sig = String::from("(Lorg/bukkit/block/Block;Lorg/bukkit/inventory/ItemStack;Lorg/bukkit/inventory/CookingRecipe;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let val_2 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg1.into().jni_object().clone())
        });
        let val_3 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg2.into().jni_object().clone())
        });
        let cls = jni.find_class("org/bukkit/event/inventory/FurnaceStartSmeltEvent");
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
        crate::event::inventory::FurnaceStartSmeltEvent::from_raw(&jni, res)
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

    pub fn recipe(
        &self,
    ) -> Result<Option<crate::inventory::CookingRecipe<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/inventory/CookingRecipe;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getRecipe", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {
            return Ok(None);
        }
        Ok(Some(crate::inventory::CookingRecipe::from_raw(
            &self.jni_ref(),
            unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) },
        )?))
    }

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

    pub fn total_cook_time(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getTotalCookTime",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    /// Sets the total cook time for this event
    pub fn set_total_cook_time(&self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(I)V");
        let val_1 = jni::objects::JValueGen::Int(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setTotalCookTime",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    // SUPER CLASS: InventoryBlockStartEvent
    pub fn source(&self) -> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>> {
        let temp_clone =
            crate::event::block::InventoryBlockStartEvent::from_raw(&self.0, unsafe {
                jni::objects::JObject::from_raw(self.1.clone())
            })?;
        let real: crate::event::block::InventoryBlockStartEvent = temp_clone.into();
        real.source()
    }
    // SUPER CLASS: BlockEvent
    pub fn block(&self) -> Result<crate::block::Block<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::block::BlockEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::block::BlockEvent = temp_clone.into();
        real.block()
    }
    // SUPER CLASS: Event
    pub fn event_name(&self) -> Result<String, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::Event::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::Event = temp_clone.into();
        real.event_name()
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
impl<'mc> Into<crate::event::block::InventoryBlockStartEvent<'mc>> for FurnaceStartSmeltEvent<'mc> {
    fn into(self) -> crate::event::block::InventoryBlockStartEvent<'mc> {
        crate::event::block::InventoryBlockStartEvent::from_raw(&self.jni_ref(), self.1).expect("Error converting FurnaceStartSmeltEvent into crate::event::block::InventoryBlockStartEvent")
    }
}
/// Called when a hopper or hopper minecart picks up a dropped item.
#[repr(C)]
pub struct InventoryPickupItemEvent<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for InventoryPickupItemEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for InventoryPickupItemEvent<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate InventoryPickupItemEvent from null object."
            )
            .into());
        }
        let (valid, name) =
            env.validate_name(&obj, "org/bukkit/event/inventory/InventoryPickupItemEvent")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a InventoryPickupItemEvent object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> InventoryPickupItemEvent<'mc> {
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<crate::inventory::Inventory<'mc>>,
        arg1: impl Into<crate::entity::Item<'mc>>,
    ) -> Result<crate::event::inventory::InventoryPickupItemEvent<'mc>, Box<dyn std::error::Error>>
    {
        let sig = String::from("(Lorg/bukkit/inventory/Inventory;Lorg/bukkit/entity/Item;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let val_2 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg1.into().jni_object().clone())
        });
        let cls = jni.find_class("org/bukkit/event/inventory/InventoryPickupItemEvent");
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
        crate::event::inventory::InventoryPickupItemEvent::from_raw(&jni, res)
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

    pub fn is_cancelled(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isCancelled", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn item(&self) -> Result<crate::entity::Item<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/entity/Item;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getItem", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::entity::Item::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn inventory(
        &self,
    ) -> Result<crate::inventory::Inventory<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/inventory/Inventory;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getInventory", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::Inventory::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// <span class="descfrm-type-label">Description copied from interface:&nbsp;<code><a href="../Cancellable.html#setCancelled(boolean)">Cancellable</a></code></span>
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
    // SUPER CLASS: Event
    pub fn event_name(&self) -> Result<String, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::Event::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::Event = temp_clone.into();
        real.event_name()
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
impl<'mc> Into<crate::event::Cancellable<'mc>> for InventoryPickupItemEvent<'mc> {
    fn into(self) -> crate::event::Cancellable<'mc> {
        crate::event::Cancellable::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting InventoryPickupItemEvent into crate::event::Cancellable")
    }
}
impl<'mc> Into<crate::event::Event<'mc>> for InventoryPickupItemEvent<'mc> {
    fn into(self) -> crate::event::Event<'mc> {
        crate::event::Event::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting InventoryPickupItemEvent into crate::event::Event")
    }
}
/// An abstract base class for events that describe an interaction between a HumanEntity and the contents of an Inventory.
#[repr(C)]
pub struct InventoryInteractEvent<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for InventoryInteractEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for InventoryInteractEvent<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate InventoryInteractEvent from null object."
            )
            .into());
        }
        let (valid, name) =
            env.validate_name(&obj, "org/bukkit/event/inventory/InventoryInteractEvent")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a InventoryInteractEvent object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> InventoryInteractEvent<'mc> {
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<crate::inventory::InventoryView<'mc>>,
    ) -> Result<crate::event::inventory::InventoryInteractEvent<'mc>, Box<dyn std::error::Error>>
    {
        let sig = String::from("(Lorg/bukkit/inventory/InventoryView;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let cls = jni.find_class("org/bukkit/event/inventory/InventoryInteractEvent");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(
            cls,
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = jni.translate_error_no_gen(res)?;
        crate::event::inventory::InventoryInteractEvent::from_raw(&jni, res)
    }

    pub fn result(&self) -> Result<crate::event::EventResult<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/event/Event$Result;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getResult", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::event::EventResult::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn set_result(
        &self,
        arg0: impl Into<crate::event::EventResult<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/event/Event$Result;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setResult",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn is_cancelled(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isCancelled", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Proxy method to <a href="#setResult(org.bukkit.event.Event.Result)"><code>setResult(org.bukkit.event.Event.Result)</code></a> for the Cancellable interface. <a href="#setResult(org.bukkit.event.Event.Result)"><code>setResult(org.bukkit.event.Event.Result)</code></a> is preferred, as it allows you to specify the Result beyond Result.DENY and Result.ALLOW.
    /// <p>Sets the cancellation state of this event. A cancelled event will not be executed in the server, but will still pass to other plugins.</p>
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

    pub fn who_clicked(
        &self,
    ) -> Result<crate::entity::HumanEntity<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/entity/HumanEntity;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getWhoClicked", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::entity::HumanEntity::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    // SUPER CLASS: InventoryEvent
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
    pub fn inventory(
        &self,
    ) -> Result<crate::inventory::Inventory<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::inventory::InventoryEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::inventory::InventoryEvent = temp_clone.into();
        real.inventory()
    }
    pub fn viewers(
        &self,
    ) -> Result<Vec<crate::entity::HumanEntity<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/List;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getViewers", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let list = blackboxmc_java::util::JavaList::from_raw(&self.0, res.l()?)?;
        let iter = list.iterator()?;
        while iter.has_next()? {
            let obj = iter.next()?;
            new_vec.push(crate::entity::HumanEntity::from_raw(&self.0, obj)?);
        }
        Ok(new_vec)
    }
    pub fn handler_list(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>> {
        crate::event::inventory::InventoryEvent::handler_list(jni)
    }
    pub fn view(&self) -> Result<crate::inventory::InventoryView<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::inventory::InventoryEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::inventory::InventoryEvent = temp_clone.into();
        real.view()
    }
    // SUPER CLASS: Event
    pub fn event_name(&self) -> Result<String, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::Event::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::Event = temp_clone.into();
        real.event_name()
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
impl<'mc> Into<crate::event::Cancellable<'mc>> for InventoryInteractEvent<'mc> {
    fn into(self) -> crate::event::Cancellable<'mc> {
        crate::event::Cancellable::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting InventoryInteractEvent into crate::event::Cancellable")
    }
}
impl<'mc> Into<crate::event::inventory::InventoryEvent<'mc>> for InventoryInteractEvent<'mc> {
    fn into(self) -> crate::event::inventory::InventoryEvent<'mc> {
        crate::event::inventory::InventoryEvent::from_raw(&self.jni_ref(), self.1).expect(
            "Error converting InventoryInteractEvent into crate::event::inventory::InventoryEvent",
        )
    }
}
/// Represents a player related inventory event
#[repr(C)]
pub struct InventoryEvent<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for InventoryEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for InventoryEvent<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate InventoryEvent from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/event/inventory/InventoryEvent")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a InventoryEvent object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> InventoryEvent<'mc> {
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<crate::inventory::InventoryView<'mc>>,
    ) -> Result<crate::event::inventory::InventoryEvent<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/inventory/InventoryView;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let cls = jni.find_class("org/bukkit/event/inventory/InventoryEvent");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(
            cls,
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = jni.translate_error_no_gen(res)?;
        crate::event::inventory::InventoryEvent::from_raw(&jni, res)
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

    pub fn inventory(
        &self,
    ) -> Result<crate::inventory::Inventory<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/inventory/Inventory;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getInventory", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::Inventory::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn viewers(
        &self,
    ) -> Result<Vec<crate::entity::HumanEntity<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/List;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getViewers", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let list = blackboxmc_java::util::JavaList::from_raw(&self.0, res.l()?)?;
        let iter = list.iterator()?;
        while iter.has_next()? {
            let obj = iter.next()?;
            new_vec.push(crate::entity::HumanEntity::from_raw(&self.0, obj)?);
        }
        Ok(new_vec)
    }

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

    pub fn view(&self) -> Result<crate::inventory::InventoryView<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/inventory/InventoryView;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getView", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::InventoryView::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    // SUPER CLASS: Event
    pub fn event_name(&self) -> Result<String, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::Event::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::Event = temp_clone.into();
        real.event_name()
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
impl<'mc> Into<crate::event::Event<'mc>> for InventoryEvent<'mc> {
    fn into(self) -> crate::event::Event<'mc> {
        crate::event::Event::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting InventoryEvent into crate::event::Event")
    }
}
/// This event is called when a player in creative mode puts down or picks up an item in their inventory / hotbar and when they drop items from their Inventory while in creative mode.
#[repr(C)]
pub struct InventoryCreativeEvent<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for InventoryCreativeEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for InventoryCreativeEvent<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate InventoryCreativeEvent from null object."
            )
            .into());
        }
        let (valid, name) =
            env.validate_name(&obj, "org/bukkit/event/inventory/InventoryCreativeEvent")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a InventoryCreativeEvent object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> InventoryCreativeEvent<'mc> {
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<crate::inventory::InventoryView<'mc>>,
        arg1: impl Into<crate::event::inventory::InventoryTypeSlotType<'mc>>,
        arg2: i32,
        arg3: impl Into<crate::inventory::ItemStack<'mc>>,
    ) -> Result<crate::event::inventory::InventoryCreativeEvent<'mc>, Box<dyn std::error::Error>>
    {
        let sig = String::from("(Lorg/bukkit/inventory/InventoryView;Lorg/bukkit/event/inventory/InventoryType$SlotType;ILorg/bukkit/inventory/ItemStack;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let val_2 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg1.into().jni_object().clone())
        });
        let val_3 = jni::objects::JValueGen::Int(arg2);
        let val_4 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg3.into().jni_object().clone())
        });
        let cls = jni.find_class("org/bukkit/event/inventory/InventoryCreativeEvent");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(
            cls,
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
                jni::objects::JValueGen::from(val_3),
                jni::objects::JValueGen::from(val_4),
            ],
        );
        let res = jni.translate_error_no_gen(res)?;
        crate::event::inventory::InventoryCreativeEvent::from_raw(&jni, res)
    }

    pub fn set_cursor(
        &self,
        arg0: impl Into<crate::inventory::ItemStack<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/inventory/ItemStack;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setCursor",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn cursor(
        &self,
    ) -> Result<Option<crate::inventory::ItemStack<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/inventory/ItemStack;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getCursor", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {
            return Ok(None);
        }
        Ok(Some(crate::inventory::ItemStack::from_raw(
            &self.jni_ref(),
            unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) },
        )?))
    }
    // SUPER CLASS: InventoryClickEvent
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
    pub fn slot_type(
        &self,
    ) -> Result<crate::event::inventory::InventoryTypeSlotType<'mc>, Box<dyn std::error::Error>>
    {
        let temp_clone = crate::event::inventory::InventoryClickEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::inventory::InventoryClickEvent = temp_clone.into();
        real.slot_type()
    }
    pub fn handler_list(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>> {
        crate::event::inventory::InventoryClickEvent::handler_list(jni)
    }
    pub fn action(
        &self,
    ) -> Result<crate::event::inventory::InventoryAction<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::inventory::InventoryClickEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::inventory::InventoryClickEvent = temp_clone.into();
        real.action()
    }
    pub fn is_shift_click(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::inventory::InventoryClickEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::inventory::InventoryClickEvent = temp_clone.into();
        real.is_shift_click()
    }
    pub fn current_item(
        &self,
    ) -> Result<Option<crate::inventory::ItemStack<'mc>>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::inventory::InventoryClickEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::inventory::InventoryClickEvent = temp_clone.into();
        real.current_item()
    }
    pub fn is_right_click(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::inventory::InventoryClickEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::inventory::InventoryClickEvent = temp_clone.into();
        real.is_right_click()
    }
    pub fn is_left_click(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::inventory::InventoryClickEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::inventory::InventoryClickEvent = temp_clone.into();
        real.is_left_click()
    }
    pub fn set_current_item(
        &self,
        arg0: impl Into<crate::inventory::ItemStack<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::event::inventory::InventoryClickEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::inventory::InventoryClickEvent = temp_clone.into();
        real.set_current_item(arg0)
    }
    pub fn clicked_inventory(
        &self,
    ) -> Result<Option<crate::inventory::Inventory<'mc>>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::inventory::InventoryClickEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::inventory::InventoryClickEvent = temp_clone.into();
        real.clicked_inventory()
    }
    pub fn raw_slot(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::inventory::InventoryClickEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::inventory::InventoryClickEvent = temp_clone.into();
        real.raw_slot()
    }
    pub fn hotbar_button(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::inventory::InventoryClickEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::inventory::InventoryClickEvent = temp_clone.into();
        real.hotbar_button()
    }
    pub fn click(
        &self,
    ) -> Result<Option<crate::event::inventory::ClickType<'mc>>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::inventory::InventoryClickEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::inventory::InventoryClickEvent = temp_clone.into();
        real.click()
    }
    pub fn slot(&self) -> Result<Option<i32>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::inventory::InventoryClickEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::inventory::InventoryClickEvent = temp_clone.into();
        real.slot()
    }
    // SUPER CLASS: InventoryInteractEvent
    pub fn is_cancelled(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone =
            crate::event::inventory::InventoryInteractEvent::from_raw(&self.0, unsafe {
                jni::objects::JObject::from_raw(self.1.clone())
            })?;
        let real: crate::event::Cancellable = temp_clone.into();
        real.is_cancelled()
    }
    pub fn set_cancelled(&self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone =
            crate::event::inventory::InventoryInteractEvent::from_raw(&self.0, unsafe {
                jni::objects::JObject::from_raw(self.1.clone())
            })?;
        let real: crate::event::Cancellable = temp_clone.into();
        real.set_cancelled(arg0)
    }
    pub fn result(&self) -> Result<crate::event::EventResult<'mc>, Box<dyn std::error::Error>> {
        let temp_clone =
            crate::event::inventory::InventoryInteractEvent::from_raw(&self.0, unsafe {
                jni::objects::JObject::from_raw(self.1.clone())
            })?;
        let real: crate::event::inventory::InventoryInteractEvent = temp_clone.into();
        real.result()
    }
    pub fn set_result(
        &self,
        arg0: impl Into<crate::event::EventResult<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone =
            crate::event::inventory::InventoryInteractEvent::from_raw(&self.0, unsafe {
                jni::objects::JObject::from_raw(self.1.clone())
            })?;
        let real: crate::event::inventory::InventoryInteractEvent = temp_clone.into();
        real.set_result(arg0)
    }
    pub fn who_clicked(
        &self,
    ) -> Result<crate::entity::HumanEntity<'mc>, Box<dyn std::error::Error>> {
        let temp_clone =
            crate::event::inventory::InventoryInteractEvent::from_raw(&self.0, unsafe {
                jni::objects::JObject::from_raw(self.1.clone())
            })?;
        let real: crate::event::inventory::InventoryInteractEvent = temp_clone.into();
        real.who_clicked()
    }
    // SUPER CLASS: InventoryEvent
    pub fn inventory(
        &self,
    ) -> Result<crate::inventory::Inventory<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::inventory::InventoryEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::inventory::InventoryEvent = temp_clone.into();
        real.inventory()
    }
    pub fn viewers(
        &self,
    ) -> Result<Vec<crate::entity::HumanEntity<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/List;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getViewers", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let list = blackboxmc_java::util::JavaList::from_raw(&self.0, res.l()?)?;
        let iter = list.iterator()?;
        while iter.has_next()? {
            let obj = iter.next()?;
            new_vec.push(crate::entity::HumanEntity::from_raw(&self.0, obj)?);
        }
        Ok(new_vec)
    }
    pub fn view(&self) -> Result<crate::inventory::InventoryView<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::inventory::InventoryEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::inventory::InventoryEvent = temp_clone.into();
        real.view()
    }
    // SUPER CLASS: Event
    pub fn event_name(&self) -> Result<String, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::Event::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::Event = temp_clone.into();
        real.event_name()
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
impl<'mc> Into<crate::event::inventory::InventoryClickEvent<'mc>> for InventoryCreativeEvent<'mc> {
    fn into(self) -> crate::event::inventory::InventoryClickEvent<'mc> {
        crate::event::inventory::InventoryClickEvent::from_raw(&self.jni_ref(), self.1).expect("Error converting InventoryCreativeEvent into crate::event::inventory::InventoryClickEvent")
    }
}
/// This event is called when a player takes items out of the furnace
#[repr(C)]
pub struct FurnaceExtractEvent<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for FurnaceExtractEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for FurnaceExtractEvent<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate FurnaceExtractEvent from null object.").into(),
            );
        }
        let (valid, name) =
            env.validate_name(&obj, "org/bukkit/event/inventory/FurnaceExtractEvent")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a FurnaceExtractEvent object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> FurnaceExtractEvent<'mc> {
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<crate::entity::Player<'mc>>,
        arg1: impl Into<crate::block::Block<'mc>>,
        arg2: impl Into<crate::Material<'mc>>,
        arg3: i32,
        arg4: i32,
    ) -> Result<crate::event::inventory::FurnaceExtractEvent<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from(
            "(Lorg/bukkit/entity/Player;Lorg/bukkit/block/Block;Lorg/bukkit/Material;II)V",
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
        let val_4 = jni::objects::JValueGen::Int(arg3);
        let val_5 = jni::objects::JValueGen::Int(arg4);
        let cls = jni.find_class("org/bukkit/event/inventory/FurnaceExtractEvent");
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
        crate::event::inventory::FurnaceExtractEvent::from_raw(&jni, res)
    }

    pub fn player(&self) -> Result<Option<crate::entity::Player<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/entity/Player;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getPlayer", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {
            return Ok(None);
        }
        Ok(Some(crate::entity::Player::from_raw(
            &self.jni_ref(),
            unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) },
        )?))
    }

    pub fn item_type(&self) -> Result<crate::Material<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/Material;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getItemType", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::Material::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn item_amount(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getItemAmount", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    // SUPER CLASS: BlockExpEvent
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
    pub fn handler_list(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>> {
        crate::event::block::BlockExpEvent::handler_list(jni)
    }
    pub fn exp_to_drop(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::block::BlockExpEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::block::BlockExpEvent = temp_clone.into();
        real.exp_to_drop()
    }
    pub fn set_exp_to_drop(&self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::event::block::BlockExpEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::block::BlockExpEvent = temp_clone.into();
        real.set_exp_to_drop(arg0)
    }
    // SUPER CLASS: BlockEvent
    pub fn block(&self) -> Result<crate::block::Block<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::block::BlockEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::block::BlockEvent = temp_clone.into();
        real.block()
    }
    // SUPER CLASS: Event
    pub fn event_name(&self) -> Result<String, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::Event::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::Event = temp_clone.into();
        real.event_name()
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
impl<'mc> Into<crate::event::block::BlockExpEvent<'mc>> for FurnaceExtractEvent<'mc> {
    fn into(self) -> crate::event::block::BlockExpEvent<'mc> {
        crate::event::block::BlockExpEvent::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting FurnaceExtractEvent into crate::event::block::BlockExpEvent")
    }
}
pub enum InventoryType<'mc> {
    Chest {
        inner: InventoryTypeStruct<'mc>,
    },
    Dispenser {
        inner: InventoryTypeStruct<'mc>,
    },
    Dropper {
        inner: InventoryTypeStruct<'mc>,
    },
    Furnace {
        inner: InventoryTypeStruct<'mc>,
    },
    Workbench {
        inner: InventoryTypeStruct<'mc>,
    },
    Crafting {
        inner: InventoryTypeStruct<'mc>,
    },
    Enchanting {
        inner: InventoryTypeStruct<'mc>,
    },
    Brewing {
        inner: InventoryTypeStruct<'mc>,
    },
    Player {
        inner: InventoryTypeStruct<'mc>,
    },
    Creative {
        inner: InventoryTypeStruct<'mc>,
    },
    Merchant {
        inner: InventoryTypeStruct<'mc>,
    },
    EnderChest {
        inner: InventoryTypeStruct<'mc>,
    },
    Anvil {
        inner: InventoryTypeStruct<'mc>,
    },
    Smithing {
        inner: InventoryTypeStruct<'mc>,
    },
    Beacon {
        inner: InventoryTypeStruct<'mc>,
    },
    Hopper {
        inner: InventoryTypeStruct<'mc>,
    },
    ShulkerBox {
        inner: InventoryTypeStruct<'mc>,
    },
    Barrel {
        inner: InventoryTypeStruct<'mc>,
    },
    BlastFurnace {
        inner: InventoryTypeStruct<'mc>,
    },
    Lectern {
        inner: InventoryTypeStruct<'mc>,
    },
    Smoker {
        inner: InventoryTypeStruct<'mc>,
    },
    Loom {
        inner: InventoryTypeStruct<'mc>,
    },
    Cartography {
        inner: InventoryTypeStruct<'mc>,
    },
    Grindstone {
        inner: InventoryTypeStruct<'mc>,
    },
    Stonecutter {
        inner: InventoryTypeStruct<'mc>,
    },
    Composter {
        inner: InventoryTypeStruct<'mc>,
    },
    ChiseledBookshelf {
        inner: InventoryTypeStruct<'mc>,
    },
    Jukebox {
        inner: InventoryTypeStruct<'mc>,
    },
    #[deprecated]
    SmithingNew {
        inner: InventoryTypeStruct<'mc>,
    },
}
impl<'mc> std::fmt::Display for InventoryType<'mc> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InventoryType::Chest { .. } => f.write_str("CHEST"),
            InventoryType::Dispenser { .. } => f.write_str("DISPENSER"),
            InventoryType::Dropper { .. } => f.write_str("DROPPER"),
            InventoryType::Furnace { .. } => f.write_str("FURNACE"),
            InventoryType::Workbench { .. } => f.write_str("WORKBENCH"),
            InventoryType::Crafting { .. } => f.write_str("CRAFTING"),
            InventoryType::Enchanting { .. } => f.write_str("ENCHANTING"),
            InventoryType::Brewing { .. } => f.write_str("BREWING"),
            InventoryType::Player { .. } => f.write_str("PLAYER"),
            InventoryType::Creative { .. } => f.write_str("CREATIVE"),
            InventoryType::Merchant { .. } => f.write_str("MERCHANT"),
            InventoryType::EnderChest { .. } => f.write_str("ENDER_CHEST"),
            InventoryType::Anvil { .. } => f.write_str("ANVIL"),
            InventoryType::Smithing { .. } => f.write_str("SMITHING"),
            InventoryType::Beacon { .. } => f.write_str("BEACON"),
            InventoryType::Hopper { .. } => f.write_str("HOPPER"),
            InventoryType::ShulkerBox { .. } => f.write_str("SHULKER_BOX"),
            InventoryType::Barrel { .. } => f.write_str("BARREL"),
            InventoryType::BlastFurnace { .. } => f.write_str("BLAST_FURNACE"),
            InventoryType::Lectern { .. } => f.write_str("LECTERN"),
            InventoryType::Smoker { .. } => f.write_str("SMOKER"),
            InventoryType::Loom { .. } => f.write_str("LOOM"),
            InventoryType::Cartography { .. } => f.write_str("CARTOGRAPHY"),
            InventoryType::Grindstone { .. } => f.write_str("GRINDSTONE"),
            InventoryType::Stonecutter { .. } => f.write_str("STONECUTTER"),
            InventoryType::Composter { .. } => f.write_str("COMPOSTER"),
            InventoryType::ChiseledBookshelf { .. } => f.write_str("CHISELED_BOOKSHELF"),
            InventoryType::Jukebox { .. } => f.write_str("JUKEBOX"),
            InventoryType::SmithingNew { .. } => f.write_str("SMITHING_NEW"),
        }
    }
}

impl<'mc> InventoryType<'mc> {
    pub fn value_of(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<String>,
    ) -> Result<InventoryType<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(env.new_string(arg0.into())?);
        let cls = env.find_class("org/bukkit/event/inventory/InventoryType");
        let cls = env.translate_error_with_class(cls)?;
        let res = env.call_static_method(
            cls,
            "valueOf",
            "(Ljava/lang/String;)Lorg/bukkit/event/inventory/InventoryType;",
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
            "CHEST" => Ok(InventoryType::Chest {
                inner: InventoryTypeStruct::from_raw(env, obj)?,
            }),
            "DISPENSER" => Ok(InventoryType::Dispenser {
                inner: InventoryTypeStruct::from_raw(env, obj)?,
            }),
            "DROPPER" => Ok(InventoryType::Dropper {
                inner: InventoryTypeStruct::from_raw(env, obj)?,
            }),
            "FURNACE" => Ok(InventoryType::Furnace {
                inner: InventoryTypeStruct::from_raw(env, obj)?,
            }),
            "WORKBENCH" => Ok(InventoryType::Workbench {
                inner: InventoryTypeStruct::from_raw(env, obj)?,
            }),
            "CRAFTING" => Ok(InventoryType::Crafting {
                inner: InventoryTypeStruct::from_raw(env, obj)?,
            }),
            "ENCHANTING" => Ok(InventoryType::Enchanting {
                inner: InventoryTypeStruct::from_raw(env, obj)?,
            }),
            "BREWING" => Ok(InventoryType::Brewing {
                inner: InventoryTypeStruct::from_raw(env, obj)?,
            }),
            "PLAYER" => Ok(InventoryType::Player {
                inner: InventoryTypeStruct::from_raw(env, obj)?,
            }),
            "CREATIVE" => Ok(InventoryType::Creative {
                inner: InventoryTypeStruct::from_raw(env, obj)?,
            }),
            "MERCHANT" => Ok(InventoryType::Merchant {
                inner: InventoryTypeStruct::from_raw(env, obj)?,
            }),
            "ENDER_CHEST" => Ok(InventoryType::EnderChest {
                inner: InventoryTypeStruct::from_raw(env, obj)?,
            }),
            "ANVIL" => Ok(InventoryType::Anvil {
                inner: InventoryTypeStruct::from_raw(env, obj)?,
            }),
            "SMITHING" => Ok(InventoryType::Smithing {
                inner: InventoryTypeStruct::from_raw(env, obj)?,
            }),
            "BEACON" => Ok(InventoryType::Beacon {
                inner: InventoryTypeStruct::from_raw(env, obj)?,
            }),
            "HOPPER" => Ok(InventoryType::Hopper {
                inner: InventoryTypeStruct::from_raw(env, obj)?,
            }),
            "SHULKER_BOX" => Ok(InventoryType::ShulkerBox {
                inner: InventoryTypeStruct::from_raw(env, obj)?,
            }),
            "BARREL" => Ok(InventoryType::Barrel {
                inner: InventoryTypeStruct::from_raw(env, obj)?,
            }),
            "BLAST_FURNACE" => Ok(InventoryType::BlastFurnace {
                inner: InventoryTypeStruct::from_raw(env, obj)?,
            }),
            "LECTERN" => Ok(InventoryType::Lectern {
                inner: InventoryTypeStruct::from_raw(env, obj)?,
            }),
            "SMOKER" => Ok(InventoryType::Smoker {
                inner: InventoryTypeStruct::from_raw(env, obj)?,
            }),
            "LOOM" => Ok(InventoryType::Loom {
                inner: InventoryTypeStruct::from_raw(env, obj)?,
            }),
            "CARTOGRAPHY" => Ok(InventoryType::Cartography {
                inner: InventoryTypeStruct::from_raw(env, obj)?,
            }),
            "GRINDSTONE" => Ok(InventoryType::Grindstone {
                inner: InventoryTypeStruct::from_raw(env, obj)?,
            }),
            "STONECUTTER" => Ok(InventoryType::Stonecutter {
                inner: InventoryTypeStruct::from_raw(env, obj)?,
            }),
            "COMPOSTER" => Ok(InventoryType::Composter {
                inner: InventoryTypeStruct::from_raw(env, obj)?,
            }),
            "CHISELED_BOOKSHELF" => Ok(InventoryType::ChiseledBookshelf {
                inner: InventoryTypeStruct::from_raw(env, obj)?,
            }),
            "JUKEBOX" => Ok(InventoryType::Jukebox {
                inner: InventoryTypeStruct::from_raw(env, obj)?,
            }),
            "SMITHING_NEW" => Ok(InventoryType::SmithingNew {
                inner: InventoryTypeStruct::from_raw(env, obj)?,
            }),

            _ => Err(eyre::eyre!("String gaven for variant was invalid").into()),
        }
    }
}

#[repr(C)]
pub struct InventoryTypeStruct<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
pub enum InventoryTypeSlotType<'mc> {
    Result {
        inner: InventoryTypeSlotTypeStruct<'mc>,
    },
    Crafting {
        inner: InventoryTypeSlotTypeStruct<'mc>,
    },
    Armor {
        inner: InventoryTypeSlotTypeStruct<'mc>,
    },
    Container {
        inner: InventoryTypeSlotTypeStruct<'mc>,
    },
    Quickbar {
        inner: InventoryTypeSlotTypeStruct<'mc>,
    },
    Outside {
        inner: InventoryTypeSlotTypeStruct<'mc>,
    },
    Fuel {
        inner: InventoryTypeSlotTypeStruct<'mc>,
    },
}
impl<'mc> std::fmt::Display for InventoryTypeSlotType<'mc> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InventoryTypeSlotType::Result { .. } => f.write_str("RESULT"),
            InventoryTypeSlotType::Crafting { .. } => f.write_str("CRAFTING"),
            InventoryTypeSlotType::Armor { .. } => f.write_str("ARMOR"),
            InventoryTypeSlotType::Container { .. } => f.write_str("CONTAINER"),
            InventoryTypeSlotType::Quickbar { .. } => f.write_str("QUICKBAR"),
            InventoryTypeSlotType::Outside { .. } => f.write_str("OUTSIDE"),
            InventoryTypeSlotType::Fuel { .. } => f.write_str("FUEL"),
        }
    }
}

impl<'mc> InventoryTypeSlotType<'mc> {
    pub fn value_of(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<String>,
    ) -> Result<InventoryTypeSlotType<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(env.new_string(arg0.into())?);
        let cls = env.find_class("org/bukkit/event/inventory/InventoryType$SlotType");
        let cls = env.translate_error_with_class(cls)?;
        let res = env.call_static_method(
            cls,
            "valueOf",
            "(Ljava/lang/String;)Lorg/bukkit/event/inventory/InventoryType$SlotType;",
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
            "RESULT" => Ok(InventoryTypeSlotType::Result {
                inner: InventoryTypeSlotTypeStruct::from_raw(env, obj)?,
            }),
            "CRAFTING" => Ok(InventoryTypeSlotType::Crafting {
                inner: InventoryTypeSlotTypeStruct::from_raw(env, obj)?,
            }),
            "ARMOR" => Ok(InventoryTypeSlotType::Armor {
                inner: InventoryTypeSlotTypeStruct::from_raw(env, obj)?,
            }),
            "CONTAINER" => Ok(InventoryTypeSlotType::Container {
                inner: InventoryTypeSlotTypeStruct::from_raw(env, obj)?,
            }),
            "QUICKBAR" => Ok(InventoryTypeSlotType::Quickbar {
                inner: InventoryTypeSlotTypeStruct::from_raw(env, obj)?,
            }),
            "OUTSIDE" => Ok(InventoryTypeSlotType::Outside {
                inner: InventoryTypeSlotTypeStruct::from_raw(env, obj)?,
            }),
            "FUEL" => Ok(InventoryTypeSlotType::Fuel {
                inner: InventoryTypeSlotTypeStruct::from_raw(env, obj)?,
            }),

            _ => Err(eyre::eyre!("String gaven for variant was invalid").into()),
        }
    }
}

#[repr(C)]
pub struct InventoryTypeSlotTypeStruct<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for InventoryTypeSlotType<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        match self {
            Self::Result { inner } => inner.0.clone(),
            Self::Crafting { inner } => inner.0.clone(),
            Self::Armor { inner } => inner.0.clone(),
            Self::Container { inner } => inner.0.clone(),
            Self::Quickbar { inner } => inner.0.clone(),
            Self::Outside { inner } => inner.0.clone(),
            Self::Fuel { inner } => inner.0.clone(),
        }
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        match self {
            Self::Result { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Crafting { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Armor { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Container { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::Quickbar { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Outside { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Fuel { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
        }
    }
}
impl<'mc> JNIInstantiatable<'mc> for InventoryTypeSlotType<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate InventoryTypeSlotType from null object."
            )
            .into());
        }
        let (valid, name) =
            env.validate_name(&obj, "org/bukkit/event/inventory/InventoryType$SlotType")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a InventoryTypeSlotType object, got {}",
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
                "RESULT" => Ok(InventoryTypeSlotType::Result {
                    inner: InventoryTypeSlotTypeStruct::from_raw(env, obj)?,
                }),
                "CRAFTING" => Ok(InventoryTypeSlotType::Crafting {
                    inner: InventoryTypeSlotTypeStruct::from_raw(env, obj)?,
                }),
                "ARMOR" => Ok(InventoryTypeSlotType::Armor {
                    inner: InventoryTypeSlotTypeStruct::from_raw(env, obj)?,
                }),
                "CONTAINER" => Ok(InventoryTypeSlotType::Container {
                    inner: InventoryTypeSlotTypeStruct::from_raw(env, obj)?,
                }),
                "QUICKBAR" => Ok(InventoryTypeSlotType::Quickbar {
                    inner: InventoryTypeSlotTypeStruct::from_raw(env, obj)?,
                }),
                "OUTSIDE" => Ok(InventoryTypeSlotType::Outside {
                    inner: InventoryTypeSlotTypeStruct::from_raw(env, obj)?,
                }),
                "FUEL" => Ok(InventoryTypeSlotType::Fuel {
                    inner: InventoryTypeSlotTypeStruct::from_raw(env, obj)?,
                }),
                _ => Err(eyre::eyre!("String gaven for variant was invalid").into()),
            }
        }
    }
}

impl<'mc> JNIRaw<'mc> for InventoryTypeSlotTypeStruct<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for InventoryTypeSlotTypeStruct<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate InventoryTypeSlotTypeStruct from null object."
            )
            .into());
        }
        let (valid, name) =
            env.validate_name(&obj, "org/bukkit/event/inventory/InventoryType$SlotType")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a InventoryTypeSlotTypeStruct object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> InventoryTypeSlotTypeStruct<'mc> {
    pub fn values(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<Vec<crate::event::inventory::InventoryTypeSlotType<'mc>>, Box<dyn std::error::Error>>
    {
        let sig = String::from("()Lorg/bukkit/event/inventory/InventoryType$SlotType;");
        let cls = jni.find_class("org/bukkit/event/inventory/InventoryType$SlotType");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "values", sig.as_str(), vec![]);
        let res = jni.translate_error(res)?;
        let arr = Into::<jni::objects::JObjectArray>::into(res.l()?);
        let len = jni.get_array_length(&arr)?;
        let mut vec = Vec::new();
        for i in 0..len {
            let res = jni.get_object_array_element(&arr, i)?;
            vec.push({ crate::event::inventory::InventoryTypeSlotType::from_raw(&jni, res)? });
        }
        Ok(vec)
    }
    // SUPER CLASS: Enum

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}

impl<'mc> JNIRaw<'mc> for InventoryType<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        match self {
            Self::Chest { inner } => inner.0.clone(),
            Self::Dispenser { inner } => inner.0.clone(),
            Self::Dropper { inner } => inner.0.clone(),
            Self::Furnace { inner } => inner.0.clone(),
            Self::Workbench { inner } => inner.0.clone(),
            Self::Crafting { inner } => inner.0.clone(),
            Self::Enchanting { inner } => inner.0.clone(),
            Self::Brewing { inner } => inner.0.clone(),
            Self::Player { inner } => inner.0.clone(),
            Self::Creative { inner } => inner.0.clone(),
            Self::Merchant { inner } => inner.0.clone(),
            Self::EnderChest { inner } => inner.0.clone(),
            Self::Anvil { inner } => inner.0.clone(),
            Self::Smithing { inner } => inner.0.clone(),
            Self::Beacon { inner } => inner.0.clone(),
            Self::Hopper { inner } => inner.0.clone(),
            Self::ShulkerBox { inner } => inner.0.clone(),
            Self::Barrel { inner } => inner.0.clone(),
            Self::BlastFurnace { inner } => inner.0.clone(),
            Self::Lectern { inner } => inner.0.clone(),
            Self::Smoker { inner } => inner.0.clone(),
            Self::Loom { inner } => inner.0.clone(),
            Self::Cartography { inner } => inner.0.clone(),
            Self::Grindstone { inner } => inner.0.clone(),
            Self::Stonecutter { inner } => inner.0.clone(),
            Self::Composter { inner } => inner.0.clone(),
            Self::ChiseledBookshelf { inner } => inner.0.clone(),
            Self::Jukebox { inner } => inner.0.clone(),
            Self::SmithingNew { inner } => inner.0.clone(),
        }
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        match self {
            Self::Chest { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Dispenser { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::Dropper { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Furnace { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Workbench { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::Crafting { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Enchanting { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::Brewing { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Player { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Creative { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Merchant { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::EnderChest { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::Anvil { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Smithing { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Beacon { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Hopper { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::ShulkerBox { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::Barrel { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::BlastFurnace { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::Lectern { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Smoker { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Loom { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Cartography { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::Grindstone { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::Stonecutter { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::Composter { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::ChiseledBookshelf { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::Jukebox { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::SmithingNew { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
        }
    }
}
impl<'mc> JNIInstantiatable<'mc> for InventoryType<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate InventoryType from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/event/inventory/InventoryType")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a InventoryType object, got {}",
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
                "CHEST" => Ok(InventoryType::Chest {
                    inner: InventoryTypeStruct::from_raw(env, obj)?,
                }),
                "DISPENSER" => Ok(InventoryType::Dispenser {
                    inner: InventoryTypeStruct::from_raw(env, obj)?,
                }),
                "DROPPER" => Ok(InventoryType::Dropper {
                    inner: InventoryTypeStruct::from_raw(env, obj)?,
                }),
                "FURNACE" => Ok(InventoryType::Furnace {
                    inner: InventoryTypeStruct::from_raw(env, obj)?,
                }),
                "WORKBENCH" => Ok(InventoryType::Workbench {
                    inner: InventoryTypeStruct::from_raw(env, obj)?,
                }),
                "CRAFTING" => Ok(InventoryType::Crafting {
                    inner: InventoryTypeStruct::from_raw(env, obj)?,
                }),
                "ENCHANTING" => Ok(InventoryType::Enchanting {
                    inner: InventoryTypeStruct::from_raw(env, obj)?,
                }),
                "BREWING" => Ok(InventoryType::Brewing {
                    inner: InventoryTypeStruct::from_raw(env, obj)?,
                }),
                "PLAYER" => Ok(InventoryType::Player {
                    inner: InventoryTypeStruct::from_raw(env, obj)?,
                }),
                "CREATIVE" => Ok(InventoryType::Creative {
                    inner: InventoryTypeStruct::from_raw(env, obj)?,
                }),
                "MERCHANT" => Ok(InventoryType::Merchant {
                    inner: InventoryTypeStruct::from_raw(env, obj)?,
                }),
                "ENDER_CHEST" => Ok(InventoryType::EnderChest {
                    inner: InventoryTypeStruct::from_raw(env, obj)?,
                }),
                "ANVIL" => Ok(InventoryType::Anvil {
                    inner: InventoryTypeStruct::from_raw(env, obj)?,
                }),
                "SMITHING" => Ok(InventoryType::Smithing {
                    inner: InventoryTypeStruct::from_raw(env, obj)?,
                }),
                "BEACON" => Ok(InventoryType::Beacon {
                    inner: InventoryTypeStruct::from_raw(env, obj)?,
                }),
                "HOPPER" => Ok(InventoryType::Hopper {
                    inner: InventoryTypeStruct::from_raw(env, obj)?,
                }),
                "SHULKER_BOX" => Ok(InventoryType::ShulkerBox {
                    inner: InventoryTypeStruct::from_raw(env, obj)?,
                }),
                "BARREL" => Ok(InventoryType::Barrel {
                    inner: InventoryTypeStruct::from_raw(env, obj)?,
                }),
                "BLAST_FURNACE" => Ok(InventoryType::BlastFurnace {
                    inner: InventoryTypeStruct::from_raw(env, obj)?,
                }),
                "LECTERN" => Ok(InventoryType::Lectern {
                    inner: InventoryTypeStruct::from_raw(env, obj)?,
                }),
                "SMOKER" => Ok(InventoryType::Smoker {
                    inner: InventoryTypeStruct::from_raw(env, obj)?,
                }),
                "LOOM" => Ok(InventoryType::Loom {
                    inner: InventoryTypeStruct::from_raw(env, obj)?,
                }),
                "CARTOGRAPHY" => Ok(InventoryType::Cartography {
                    inner: InventoryTypeStruct::from_raw(env, obj)?,
                }),
                "GRINDSTONE" => Ok(InventoryType::Grindstone {
                    inner: InventoryTypeStruct::from_raw(env, obj)?,
                }),
                "STONECUTTER" => Ok(InventoryType::Stonecutter {
                    inner: InventoryTypeStruct::from_raw(env, obj)?,
                }),
                "COMPOSTER" => Ok(InventoryType::Composter {
                    inner: InventoryTypeStruct::from_raw(env, obj)?,
                }),
                "CHISELED_BOOKSHELF" => Ok(InventoryType::ChiseledBookshelf {
                    inner: InventoryTypeStruct::from_raw(env, obj)?,
                }),
                "JUKEBOX" => Ok(InventoryType::Jukebox {
                    inner: InventoryTypeStruct::from_raw(env, obj)?,
                }),
                "SMITHING_NEW" => Ok(InventoryType::SmithingNew {
                    inner: InventoryTypeStruct::from_raw(env, obj)?,
                }),
                _ => Err(eyre::eyre!("String gaven for variant was invalid").into()),
            }
        }
    }
}

impl<'mc> JNIRaw<'mc> for InventoryTypeStruct<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for InventoryTypeStruct<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate InventoryTypeStruct from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/event/inventory/InventoryType")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a InventoryTypeStruct object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> InventoryTypeStruct<'mc> {
    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
pub enum ContainerType<'mc> {
    Source { inner: ContainerTypeStruct<'mc> },
    Destination { inner: ContainerTypeStruct<'mc> },
}
impl<'mc> std::fmt::Display for ContainerType<'mc> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ContainerType::Source { .. } => f.write_str("SOURCE"),
            ContainerType::Destination { .. } => f.write_str("DESTINATION"),
        }
    }
}

impl<'mc> ContainerType<'mc> {
    pub fn value_of(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<String>,
    ) -> Result<ContainerType<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(env.new_string(arg0.into())?);
        let cls = env.find_class("org/bukkit/event/inventory/ContainerType");
        let cls = env.translate_error_with_class(cls)?;
        let res = env.call_static_method(
            cls,
            "valueOf",
            "(Ljava/lang/String;)Lorg/bukkit/event/inventory/ContainerType;",
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
            "SOURCE" => Ok(ContainerType::Source {
                inner: ContainerTypeStruct::from_raw(env, obj)?,
            }),
            "DESTINATION" => Ok(ContainerType::Destination {
                inner: ContainerTypeStruct::from_raw(env, obj)?,
            }),

            _ => Err(eyre::eyre!("String gaven for variant was invalid").into()),
        }
    }
}

#[repr(C)]
pub struct ContainerTypeStruct<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for ContainerType<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        match self {
            Self::Source { inner } => inner.0.clone(),
            Self::Destination { inner } => inner.0.clone(),
        }
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        match self {
            Self::Source { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Destination { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
        }
    }
}
impl<'mc> JNIInstantiatable<'mc> for ContainerType<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate ContainerType from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/event/inventory/ContainerType")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a ContainerType object, got {}",
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
                "SOURCE" => Ok(ContainerType::Source {
                    inner: ContainerTypeStruct::from_raw(env, obj)?,
                }),
                "DESTINATION" => Ok(ContainerType::Destination {
                    inner: ContainerTypeStruct::from_raw(env, obj)?,
                }),
                _ => Err(eyre::eyre!("String gaven for variant was invalid").into()),
            }
        }
    }
}

impl<'mc> JNIRaw<'mc> for ContainerTypeStruct<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for ContainerTypeStruct<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate ContainerTypeStruct from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/event/inventory/ContainerType")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a ContainerTypeStruct object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> ContainerTypeStruct<'mc> {
    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
/// Called when an item is put in a slot for upgrade by a Smithing Table.
#[repr(C)]
pub struct PrepareSmithingEvent<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for PrepareSmithingEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for PrepareSmithingEvent<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate PrepareSmithingEvent from null object.").into(),
            );
        }
        let (valid, name) =
            env.validate_name(&obj, "org/bukkit/event/inventory/PrepareSmithingEvent")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a PrepareSmithingEvent object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> PrepareSmithingEvent<'mc> {
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<crate::inventory::InventoryView<'mc>>,
        arg1: impl Into<crate::inventory::ItemStack<'mc>>,
    ) -> Result<crate::event::inventory::PrepareSmithingEvent<'mc>, Box<dyn std::error::Error>>
    {
        let sig =
            String::from("(Lorg/bukkit/inventory/InventoryView;Lorg/bukkit/inventory/ItemStack;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let val_2 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg1.into().jni_object().clone())
        });
        let cls = jni.find_class("org/bukkit/event/inventory/PrepareSmithingEvent");
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
        crate::event::inventory::PrepareSmithingEvent::from_raw(&jni, res)
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

    pub fn inventory(
        &self,
    ) -> Result<crate::inventory::SmithingInventory<'mc>, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Lorg/bukkit/inventory/SmithingInventory;";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getInventory", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::SmithingInventory::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

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
    // SUPER CLASS: PrepareInventoryResultEvent
    pub fn result(&self) -> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>> {
        let temp_clone =
            crate::event::inventory::PrepareInventoryResultEvent::from_raw(&self.0, unsafe {
                jni::objects::JObject::from_raw(self.1.clone())
            })?;
        let real: crate::event::inventory::PrepareInventoryResultEvent = temp_clone.into();
        real.result()
    }
    pub fn set_result(
        &self,
        arg0: impl Into<crate::inventory::ItemStack<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone =
            crate::event::inventory::PrepareInventoryResultEvent::from_raw(&self.0, unsafe {
                jni::objects::JObject::from_raw(self.1.clone())
            })?;
        let real: crate::event::inventory::PrepareInventoryResultEvent = temp_clone.into();
        real.set_result(arg0)
    }
    // SUPER CLASS: InventoryEvent
    pub fn viewers(
        &self,
    ) -> Result<Vec<crate::entity::HumanEntity<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/List;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getViewers", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let list = blackboxmc_java::util::JavaList::from_raw(&self.0, res.l()?)?;
        let iter = list.iterator()?;
        while iter.has_next()? {
            let obj = iter.next()?;
            new_vec.push(crate::entity::HumanEntity::from_raw(&self.0, obj)?);
        }
        Ok(new_vec)
    }
    pub fn view(&self) -> Result<crate::inventory::InventoryView<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::inventory::InventoryEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::inventory::InventoryEvent = temp_clone.into();
        real.view()
    }
    // SUPER CLASS: Event
    pub fn event_name(&self) -> Result<String, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::Event::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::Event = temp_clone.into();
        real.event_name()
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
impl<'mc> Into<crate::event::inventory::PrepareInventoryResultEvent<'mc>>
    for PrepareSmithingEvent<'mc>
{
    fn into(self) -> crate::event::inventory::PrepareInventoryResultEvent<'mc> {
        crate::event::inventory::PrepareInventoryResultEvent::from_raw(&self.jni_ref(), self.1).expect("Error converting PrepareSmithingEvent into crate::event::inventory::PrepareInventoryResultEvent")
    }
}
/// This event is called whenever a player clicks a new trade on the trades sidebar.
/// <p>This event allows the user to get the index of the trade, letting them get the MerchantRecipe via the Merchant.</p>
#[repr(C)]
pub struct TradeSelectEvent<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for TradeSelectEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for TradeSelectEvent<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate TradeSelectEvent from null object.").into(),
            );
        }
        let (valid, name) =
            env.validate_name(&obj, "org/bukkit/event/inventory/TradeSelectEvent")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a TradeSelectEvent object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> TradeSelectEvent<'mc> {
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<crate::inventory::InventoryView<'mc>>,
        arg1: i32,
    ) -> Result<crate::event::inventory::TradeSelectEvent<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/inventory/InventoryView;I)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let val_2 = jni::objects::JValueGen::Int(arg1);
        let cls = jni.find_class("org/bukkit/event/inventory/TradeSelectEvent");
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
        crate::event::inventory::TradeSelectEvent::from_raw(&jni, res)
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

    pub fn inventory(
        &self,
    ) -> Result<crate::inventory::Inventory<'mc>, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Lorg/bukkit/inventory/Inventory;";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getInventory", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::Inventory::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn merchant(&self) -> Result<crate::inventory::Merchant<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/inventory/Merchant;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getMerchant", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::Merchant::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

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

    pub fn index(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getIndex", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    // SUPER CLASS: InventoryInteractEvent
    pub fn is_cancelled(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone =
            crate::event::inventory::InventoryInteractEvent::from_raw(&self.0, unsafe {
                jni::objects::JObject::from_raw(self.1.clone())
            })?;
        let real: crate::event::Cancellable = temp_clone.into();
        real.is_cancelled()
    }
    pub fn set_cancelled(&self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone =
            crate::event::inventory::InventoryInteractEvent::from_raw(&self.0, unsafe {
                jni::objects::JObject::from_raw(self.1.clone())
            })?;
        let real: crate::event::Cancellable = temp_clone.into();
        real.set_cancelled(arg0)
    }
    pub fn result(&self) -> Result<crate::event::EventResult<'mc>, Box<dyn std::error::Error>> {
        let temp_clone =
            crate::event::inventory::InventoryInteractEvent::from_raw(&self.0, unsafe {
                jni::objects::JObject::from_raw(self.1.clone())
            })?;
        let real: crate::event::inventory::InventoryInteractEvent = temp_clone.into();
        real.result()
    }
    pub fn set_result(
        &self,
        arg0: impl Into<crate::event::EventResult<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone =
            crate::event::inventory::InventoryInteractEvent::from_raw(&self.0, unsafe {
                jni::objects::JObject::from_raw(self.1.clone())
            })?;
        let real: crate::event::inventory::InventoryInteractEvent = temp_clone.into();
        real.set_result(arg0)
    }
    pub fn who_clicked(
        &self,
    ) -> Result<crate::entity::HumanEntity<'mc>, Box<dyn std::error::Error>> {
        let temp_clone =
            crate::event::inventory::InventoryInteractEvent::from_raw(&self.0, unsafe {
                jni::objects::JObject::from_raw(self.1.clone())
            })?;
        let real: crate::event::inventory::InventoryInteractEvent = temp_clone.into();
        real.who_clicked()
    }
    // SUPER CLASS: InventoryEvent
    pub fn viewers(
        &self,
    ) -> Result<Vec<crate::entity::HumanEntity<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/List;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getViewers", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let list = blackboxmc_java::util::JavaList::from_raw(&self.0, res.l()?)?;
        let iter = list.iterator()?;
        while iter.has_next()? {
            let obj = iter.next()?;
            new_vec.push(crate::entity::HumanEntity::from_raw(&self.0, obj)?);
        }
        Ok(new_vec)
    }
    pub fn view(&self) -> Result<crate::inventory::InventoryView<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::inventory::InventoryEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::inventory::InventoryEvent = temp_clone.into();
        real.view()
    }
    // SUPER CLASS: Event
    pub fn event_name(&self) -> Result<String, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::Event::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::Event = temp_clone.into();
        real.event_name()
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
impl<'mc> Into<crate::event::inventory::InventoryInteractEvent<'mc>> for TradeSelectEvent<'mc> {
    fn into(self) -> crate::event::inventory::InventoryInteractEvent<'mc> {
        crate::event::inventory::InventoryInteractEvent::from_raw(&self.jni_ref(), self.1).expect("Error converting TradeSelectEvent into crate::event::inventory::InventoryInteractEvent")
    }
}
pub enum SlotType<'mc> {
    Result { inner: SlotTypeStruct<'mc> },
    Crafting { inner: SlotTypeStruct<'mc> },
    Armor { inner: SlotTypeStruct<'mc> },
    Container { inner: SlotTypeStruct<'mc> },
    Quickbar { inner: SlotTypeStruct<'mc> },
    Outside { inner: SlotTypeStruct<'mc> },
    Fuel { inner: SlotTypeStruct<'mc> },
}
impl<'mc> std::fmt::Display for SlotType<'mc> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SlotType::Result { .. } => f.write_str("RESULT"),
            SlotType::Crafting { .. } => f.write_str("CRAFTING"),
            SlotType::Armor { .. } => f.write_str("ARMOR"),
            SlotType::Container { .. } => f.write_str("CONTAINER"),
            SlotType::Quickbar { .. } => f.write_str("QUICKBAR"),
            SlotType::Outside { .. } => f.write_str("OUTSIDE"),
            SlotType::Fuel { .. } => f.write_str("FUEL"),
        }
    }
}

impl<'mc> SlotType<'mc> {
    pub fn value_of(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<String>,
    ) -> Result<SlotType<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(env.new_string(arg0.into())?);
        let cls = env.find_class("org/bukkit/event/inventory/SlotType");
        let cls = env.translate_error_with_class(cls)?;
        let res = env.call_static_method(
            cls,
            "valueOf",
            "(Ljava/lang/String;)Lorg/bukkit/event/inventory/SlotType;",
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
            "RESULT" => Ok(SlotType::Result {
                inner: SlotTypeStruct::from_raw(env, obj)?,
            }),
            "CRAFTING" => Ok(SlotType::Crafting {
                inner: SlotTypeStruct::from_raw(env, obj)?,
            }),
            "ARMOR" => Ok(SlotType::Armor {
                inner: SlotTypeStruct::from_raw(env, obj)?,
            }),
            "CONTAINER" => Ok(SlotType::Container {
                inner: SlotTypeStruct::from_raw(env, obj)?,
            }),
            "QUICKBAR" => Ok(SlotType::Quickbar {
                inner: SlotTypeStruct::from_raw(env, obj)?,
            }),
            "OUTSIDE" => Ok(SlotType::Outside {
                inner: SlotTypeStruct::from_raw(env, obj)?,
            }),
            "FUEL" => Ok(SlotType::Fuel {
                inner: SlotTypeStruct::from_raw(env, obj)?,
            }),

            _ => Err(eyre::eyre!("String gaven for variant was invalid").into()),
        }
    }
}

#[repr(C)]
pub struct SlotTypeStruct<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for SlotType<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        match self {
            Self::Result { inner } => inner.0.clone(),
            Self::Crafting { inner } => inner.0.clone(),
            Self::Armor { inner } => inner.0.clone(),
            Self::Container { inner } => inner.0.clone(),
            Self::Quickbar { inner } => inner.0.clone(),
            Self::Outside { inner } => inner.0.clone(),
            Self::Fuel { inner } => inner.0.clone(),
        }
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        match self {
            Self::Result { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Crafting { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Armor { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Container { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::Quickbar { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Outside { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Fuel { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
        }
    }
}
impl<'mc> JNIInstantiatable<'mc> for SlotType<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate SlotType from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/event/inventory/SlotType")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a SlotType object, got {}",
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
                "RESULT" => Ok(SlotType::Result {
                    inner: SlotTypeStruct::from_raw(env, obj)?,
                }),
                "CRAFTING" => Ok(SlotType::Crafting {
                    inner: SlotTypeStruct::from_raw(env, obj)?,
                }),
                "ARMOR" => Ok(SlotType::Armor {
                    inner: SlotTypeStruct::from_raw(env, obj)?,
                }),
                "CONTAINER" => Ok(SlotType::Container {
                    inner: SlotTypeStruct::from_raw(env, obj)?,
                }),
                "QUICKBAR" => Ok(SlotType::Quickbar {
                    inner: SlotTypeStruct::from_raw(env, obj)?,
                }),
                "OUTSIDE" => Ok(SlotType::Outside {
                    inner: SlotTypeStruct::from_raw(env, obj)?,
                }),
                "FUEL" => Ok(SlotType::Fuel {
                    inner: SlotTypeStruct::from_raw(env, obj)?,
                }),
                _ => Err(eyre::eyre!("String gaven for variant was invalid").into()),
            }
        }
    }
}

impl<'mc> JNIRaw<'mc> for SlotTypeStruct<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for SlotTypeStruct<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate SlotTypeStruct from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/event/inventory/SlotType")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a SlotTypeStruct object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> SlotTypeStruct<'mc> {
    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
pub enum InventoryAction<'mc> {
    Nothing { inner: InventoryActionStruct<'mc> },
    PickupAll { inner: InventoryActionStruct<'mc> },
    PickupSome { inner: InventoryActionStruct<'mc> },
    PickupHalf { inner: InventoryActionStruct<'mc> },
    PickupOne { inner: InventoryActionStruct<'mc> },
    PlaceAll { inner: InventoryActionStruct<'mc> },
    PlaceSome { inner: InventoryActionStruct<'mc> },
    PlaceOne { inner: InventoryActionStruct<'mc> },
    SwapWithCursor { inner: InventoryActionStruct<'mc> },
    DropAllCursor { inner: InventoryActionStruct<'mc> },
    DropOneCursor { inner: InventoryActionStruct<'mc> },
    DropAllSlot { inner: InventoryActionStruct<'mc> },
    DropOneSlot { inner: InventoryActionStruct<'mc> },
    MoveToOtherInventory { inner: InventoryActionStruct<'mc> },
    HotbarMoveAndReadd { inner: InventoryActionStruct<'mc> },
    HotbarSwap { inner: InventoryActionStruct<'mc> },
    CloneStack { inner: InventoryActionStruct<'mc> },
    CollectToCursor { inner: InventoryActionStruct<'mc> },
    Unknown { inner: InventoryActionStruct<'mc> },
}
impl<'mc> std::fmt::Display for InventoryAction<'mc> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InventoryAction::Nothing { .. } => f.write_str("NOTHING"),
            InventoryAction::PickupAll { .. } => f.write_str("PICKUP_ALL"),
            InventoryAction::PickupSome { .. } => f.write_str("PICKUP_SOME"),
            InventoryAction::PickupHalf { .. } => f.write_str("PICKUP_HALF"),
            InventoryAction::PickupOne { .. } => f.write_str("PICKUP_ONE"),
            InventoryAction::PlaceAll { .. } => f.write_str("PLACE_ALL"),
            InventoryAction::PlaceSome { .. } => f.write_str("PLACE_SOME"),
            InventoryAction::PlaceOne { .. } => f.write_str("PLACE_ONE"),
            InventoryAction::SwapWithCursor { .. } => f.write_str("SWAP_WITH_CURSOR"),
            InventoryAction::DropAllCursor { .. } => f.write_str("DROP_ALL_CURSOR"),
            InventoryAction::DropOneCursor { .. } => f.write_str("DROP_ONE_CURSOR"),
            InventoryAction::DropAllSlot { .. } => f.write_str("DROP_ALL_SLOT"),
            InventoryAction::DropOneSlot { .. } => f.write_str("DROP_ONE_SLOT"),
            InventoryAction::MoveToOtherInventory { .. } => f.write_str("MOVE_TO_OTHER_INVENTORY"),
            InventoryAction::HotbarMoveAndReadd { .. } => f.write_str("HOTBAR_MOVE_AND_READD"),
            InventoryAction::HotbarSwap { .. } => f.write_str("HOTBAR_SWAP"),
            InventoryAction::CloneStack { .. } => f.write_str("CLONE_STACK"),
            InventoryAction::CollectToCursor { .. } => f.write_str("COLLECT_TO_CURSOR"),
            InventoryAction::Unknown { .. } => f.write_str("UNKNOWN"),
        }
    }
}

impl<'mc> InventoryAction<'mc> {
    pub fn value_of(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<String>,
    ) -> Result<InventoryAction<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(env.new_string(arg0.into())?);
        let cls = env.find_class("org/bukkit/event/inventory/InventoryAction");
        let cls = env.translate_error_with_class(cls)?;
        let res = env.call_static_method(
            cls,
            "valueOf",
            "(Ljava/lang/String;)Lorg/bukkit/event/inventory/InventoryAction;",
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
            "NOTHING" => Ok(InventoryAction::Nothing {
                inner: InventoryActionStruct::from_raw(env, obj)?,
            }),
            "PICKUP_ALL" => Ok(InventoryAction::PickupAll {
                inner: InventoryActionStruct::from_raw(env, obj)?,
            }),
            "PICKUP_SOME" => Ok(InventoryAction::PickupSome {
                inner: InventoryActionStruct::from_raw(env, obj)?,
            }),
            "PICKUP_HALF" => Ok(InventoryAction::PickupHalf {
                inner: InventoryActionStruct::from_raw(env, obj)?,
            }),
            "PICKUP_ONE" => Ok(InventoryAction::PickupOne {
                inner: InventoryActionStruct::from_raw(env, obj)?,
            }),
            "PLACE_ALL" => Ok(InventoryAction::PlaceAll {
                inner: InventoryActionStruct::from_raw(env, obj)?,
            }),
            "PLACE_SOME" => Ok(InventoryAction::PlaceSome {
                inner: InventoryActionStruct::from_raw(env, obj)?,
            }),
            "PLACE_ONE" => Ok(InventoryAction::PlaceOne {
                inner: InventoryActionStruct::from_raw(env, obj)?,
            }),
            "SWAP_WITH_CURSOR" => Ok(InventoryAction::SwapWithCursor {
                inner: InventoryActionStruct::from_raw(env, obj)?,
            }),
            "DROP_ALL_CURSOR" => Ok(InventoryAction::DropAllCursor {
                inner: InventoryActionStruct::from_raw(env, obj)?,
            }),
            "DROP_ONE_CURSOR" => Ok(InventoryAction::DropOneCursor {
                inner: InventoryActionStruct::from_raw(env, obj)?,
            }),
            "DROP_ALL_SLOT" => Ok(InventoryAction::DropAllSlot {
                inner: InventoryActionStruct::from_raw(env, obj)?,
            }),
            "DROP_ONE_SLOT" => Ok(InventoryAction::DropOneSlot {
                inner: InventoryActionStruct::from_raw(env, obj)?,
            }),
            "MOVE_TO_OTHER_INVENTORY" => Ok(InventoryAction::MoveToOtherInventory {
                inner: InventoryActionStruct::from_raw(env, obj)?,
            }),
            "HOTBAR_MOVE_AND_READD" => Ok(InventoryAction::HotbarMoveAndReadd {
                inner: InventoryActionStruct::from_raw(env, obj)?,
            }),
            "HOTBAR_SWAP" => Ok(InventoryAction::HotbarSwap {
                inner: InventoryActionStruct::from_raw(env, obj)?,
            }),
            "CLONE_STACK" => Ok(InventoryAction::CloneStack {
                inner: InventoryActionStruct::from_raw(env, obj)?,
            }),
            "COLLECT_TO_CURSOR" => Ok(InventoryAction::CollectToCursor {
                inner: InventoryActionStruct::from_raw(env, obj)?,
            }),
            "UNKNOWN" => Ok(InventoryAction::Unknown {
                inner: InventoryActionStruct::from_raw(env, obj)?,
            }),

            _ => Err(eyre::eyre!("String gaven for variant was invalid").into()),
        }
    }
}

#[repr(C)]
pub struct InventoryActionStruct<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for InventoryAction<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        match self {
            Self::Nothing { inner } => inner.0.clone(),
            Self::PickupAll { inner } => inner.0.clone(),
            Self::PickupSome { inner } => inner.0.clone(),
            Self::PickupHalf { inner } => inner.0.clone(),
            Self::PickupOne { inner } => inner.0.clone(),
            Self::PlaceAll { inner } => inner.0.clone(),
            Self::PlaceSome { inner } => inner.0.clone(),
            Self::PlaceOne { inner } => inner.0.clone(),
            Self::SwapWithCursor { inner } => inner.0.clone(),
            Self::DropAllCursor { inner } => inner.0.clone(),
            Self::DropOneCursor { inner } => inner.0.clone(),
            Self::DropAllSlot { inner } => inner.0.clone(),
            Self::DropOneSlot { inner } => inner.0.clone(),
            Self::MoveToOtherInventory { inner } => inner.0.clone(),
            Self::HotbarMoveAndReadd { inner } => inner.0.clone(),
            Self::HotbarSwap { inner } => inner.0.clone(),
            Self::CloneStack { inner } => inner.0.clone(),
            Self::CollectToCursor { inner } => inner.0.clone(),
            Self::Unknown { inner } => inner.0.clone(),
        }
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        match self {
            Self::Nothing { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::PickupAll { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::PickupSome { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::PickupHalf { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::PickupOne { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::PlaceAll { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::PlaceSome { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::PlaceOne { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::SwapWithCursor { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::DropAllCursor { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::DropOneCursor { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::DropAllSlot { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::DropOneSlot { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::MoveToOtherInventory { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::HotbarMoveAndReadd { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::HotbarSwap { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::CloneStack { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::CollectToCursor { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::Unknown { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
        }
    }
}
impl<'mc> JNIInstantiatable<'mc> for InventoryAction<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate InventoryAction from null object.").into(),
            );
        }
        let (valid, name) =
            env.validate_name(&obj, "org/bukkit/event/inventory/InventoryAction")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a InventoryAction object, got {}",
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
                "NOTHING" => Ok(InventoryAction::Nothing {
                    inner: InventoryActionStruct::from_raw(env, obj)?,
                }),
                "PICKUP_ALL" => Ok(InventoryAction::PickupAll {
                    inner: InventoryActionStruct::from_raw(env, obj)?,
                }),
                "PICKUP_SOME" => Ok(InventoryAction::PickupSome {
                    inner: InventoryActionStruct::from_raw(env, obj)?,
                }),
                "PICKUP_HALF" => Ok(InventoryAction::PickupHalf {
                    inner: InventoryActionStruct::from_raw(env, obj)?,
                }),
                "PICKUP_ONE" => Ok(InventoryAction::PickupOne {
                    inner: InventoryActionStruct::from_raw(env, obj)?,
                }),
                "PLACE_ALL" => Ok(InventoryAction::PlaceAll {
                    inner: InventoryActionStruct::from_raw(env, obj)?,
                }),
                "PLACE_SOME" => Ok(InventoryAction::PlaceSome {
                    inner: InventoryActionStruct::from_raw(env, obj)?,
                }),
                "PLACE_ONE" => Ok(InventoryAction::PlaceOne {
                    inner: InventoryActionStruct::from_raw(env, obj)?,
                }),
                "SWAP_WITH_CURSOR" => Ok(InventoryAction::SwapWithCursor {
                    inner: InventoryActionStruct::from_raw(env, obj)?,
                }),
                "DROP_ALL_CURSOR" => Ok(InventoryAction::DropAllCursor {
                    inner: InventoryActionStruct::from_raw(env, obj)?,
                }),
                "DROP_ONE_CURSOR" => Ok(InventoryAction::DropOneCursor {
                    inner: InventoryActionStruct::from_raw(env, obj)?,
                }),
                "DROP_ALL_SLOT" => Ok(InventoryAction::DropAllSlot {
                    inner: InventoryActionStruct::from_raw(env, obj)?,
                }),
                "DROP_ONE_SLOT" => Ok(InventoryAction::DropOneSlot {
                    inner: InventoryActionStruct::from_raw(env, obj)?,
                }),
                "MOVE_TO_OTHER_INVENTORY" => Ok(InventoryAction::MoveToOtherInventory {
                    inner: InventoryActionStruct::from_raw(env, obj)?,
                }),
                "HOTBAR_MOVE_AND_READD" => Ok(InventoryAction::HotbarMoveAndReadd {
                    inner: InventoryActionStruct::from_raw(env, obj)?,
                }),
                "HOTBAR_SWAP" => Ok(InventoryAction::HotbarSwap {
                    inner: InventoryActionStruct::from_raw(env, obj)?,
                }),
                "CLONE_STACK" => Ok(InventoryAction::CloneStack {
                    inner: InventoryActionStruct::from_raw(env, obj)?,
                }),
                "COLLECT_TO_CURSOR" => Ok(InventoryAction::CollectToCursor {
                    inner: InventoryActionStruct::from_raw(env, obj)?,
                }),
                "UNKNOWN" => Ok(InventoryAction::Unknown {
                    inner: InventoryActionStruct::from_raw(env, obj)?,
                }),
                _ => Err(eyre::eyre!("String gaven for variant was invalid").into()),
            }
        }
    }
}

impl<'mc> JNIRaw<'mc> for InventoryActionStruct<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for InventoryActionStruct<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate InventoryActionStruct from null object."
            )
            .into());
        }
        let (valid, name) =
            env.validate_name(&obj, "org/bukkit/event/inventory/InventoryAction")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a InventoryActionStruct object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> InventoryActionStruct<'mc> {
    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
/// This event is called when a player clicks in an inventory.
/// <p>Because InventoryClickEvent occurs within a modification of the Inventory, not all Inventory related methods are safe to use.</p>
/// <p>The following should never be invoked by an EventHandler for InventoryClickEvent using the HumanEntity or InventoryView associated with this event:</p>
/// <ul>
/// <li><a href="../../entity/HumanEntity.html#closeInventory()"><code>HumanEntity.closeInventory()</code></a></li>
/// <li><a href="../../entity/HumanEntity.html#openInventory(org.bukkit.inventory.Inventory)"><code>HumanEntity.openInventory(Inventory)</code></a></li>
/// <li><a href="../../entity/HumanEntity.html#openWorkbench(org.bukkit.Location,boolean)"><code>HumanEntity.openWorkbench(Location, boolean)</code></a></li>
/// <li><a href="../../entity/HumanEntity.html#openEnchanting(org.bukkit.Location,boolean)"><code>HumanEntity.openEnchanting(Location, boolean)</code></a></li>
/// <li><a href="../../inventory/InventoryView.html#close()"><code>InventoryView.close()</code></a></li>
/// </ul> To invoke one of these methods, schedule a task using <a href="../../scheduler/BukkitScheduler.html#runTask(org.bukkit.plugin.Plugin,java.lang.Runnable)"><code>BukkitScheduler.runTask(Plugin, Runnable)</code></a>, which will run the task on the next tick. Also be aware that this is not an exhaustive list, and other methods could potentially create issues as well.
/// <p>Assuming the EntityHuman associated with this event is an instance of a Player, manipulating the MaxStackSize or contents of an Inventory will require an Invocation of <a href="../../entity/Player.html#updateInventory()"><code>Player.updateInventory()</code></a>.</p>
/// <p>Modifications to slots that are modified by the results of this InventoryClickEvent can be overwritten. To change these slots, this event should be cancelled and all desired changes to the inventory applied. Alternatively, scheduling a task using <a href="../../scheduler/BukkitScheduler.html#runTask(org.bukkit.plugin.Plugin,java.lang.Runnable)"><code>BukkitScheduler.runTask(Plugin, Runnable)</code></a>, which would execute the task on the next tick, would work as well.</p>
#[repr(C)]
pub struct InventoryClickEvent<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for InventoryClickEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for InventoryClickEvent<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate InventoryClickEvent from null object.").into(),
            );
        }
        let (valid, name) =
            env.validate_name(&obj, "org/bukkit/event/inventory/InventoryClickEvent")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a InventoryClickEvent object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> InventoryClickEvent<'mc> {
    pub fn new_with_inventory_view(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<crate::inventory::InventoryView<'mc>>,
        arg1: impl Into<crate::event::inventory::InventoryTypeSlotType<'mc>>,
        arg2: i32,
        arg3: impl Into<crate::event::inventory::ClickType<'mc>>,
        arg4: impl Into<crate::event::inventory::InventoryAction<'mc>>,
        arg5: std::option::Option<i32>,
    ) -> Result<crate::event::inventory::InventoryClickEvent<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/inventory/InventoryView;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        sig += "Lorg/bukkit/event/inventory/InventoryType$SlotType;";
        let val_2 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg1.into().jni_object().clone())
        });
        args.push(val_2);
        sig += "I";
        let val_3 = jni::objects::JValueGen::Int(arg2);
        args.push(val_3);
        sig += "Lorg/bukkit/event/inventory/ClickType;";
        let val_4 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg3.into().jni_object().clone())
        });
        args.push(val_4);
        sig += "Lorg/bukkit/event/inventory/InventoryAction;";
        let val_5 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg4.into().jni_object().clone())
        });
        args.push(val_5);
        if let Some(a) = arg5 {
            sig += "I";
            let val_6 = jni::objects::JValueGen::Int(a);
            args.push(val_6);
        }
        sig += ")V";
        let cls = jni.find_class("org/bukkit/event/inventory/InventoryClickEvent");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(cls, sig.as_str(), args);
        let res = jni.translate_error_no_gen(res)?;
        crate::event::inventory::InventoryClickEvent::from_raw(&jni, res)
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

    pub fn set_cursor(
        &self,
        arg0: impl Into<crate::inventory::ItemStack<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/inventory/ItemStack;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setCursor",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn cursor(
        &self,
    ) -> Result<Option<crate::inventory::ItemStack<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/inventory/ItemStack;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getCursor", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {
            return Ok(None);
        }
        Ok(Some(crate::inventory::ItemStack::from_raw(
            &self.jni_ref(),
            unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) },
        )?))
    }

    pub fn slot_type(
        &self,
    ) -> Result<crate::event::inventory::InventoryTypeSlotType<'mc>, Box<dyn std::error::Error>>
    {
        let sig = String::from("()Lorg/bukkit/event/inventory/InventoryType$SlotType;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getSlotType", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::event::inventory::InventoryTypeSlotType::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

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

    pub fn action(
        &self,
    ) -> Result<crate::event::inventory::InventoryAction<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/event/inventory/InventoryAction;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getAction", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::event::inventory::InventoryAction::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn is_shift_click(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isShiftClick", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn current_item(
        &self,
    ) -> Result<Option<crate::inventory::ItemStack<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/inventory/ItemStack;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getCurrentItem", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {
            return Ok(None);
        }
        Ok(Some(crate::inventory::ItemStack::from_raw(
            &self.jni_ref(),
            unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) },
        )?))
    }

    pub fn is_right_click(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isRightClick", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn is_left_click(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isLeftClick", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn set_current_item(
        &self,
        arg0: impl Into<crate::inventory::ItemStack<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/inventory/ItemStack;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setCurrentItem",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn clicked_inventory(
        &self,
    ) -> Result<Option<crate::inventory::Inventory<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/inventory/Inventory;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getClickedInventory",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {
            return Ok(None);
        }
        Ok(Some(crate::inventory::Inventory::from_raw(
            &self.jni_ref(),
            unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) },
        )?))
    }

    pub fn raw_slot(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getRawSlot", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }

    pub fn hotbar_button(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getHotbarButton", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }

    pub fn click(
        &self,
    ) -> Result<Option<crate::event::inventory::ClickType<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/event/inventory/ClickType;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getClick", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {
            return Ok(None);
        }
        Ok(Some(crate::event::inventory::ClickType::from_raw(
            &self.jni_ref(),
            unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) },
        )?))
    }

    pub fn slot(&self) -> Result<Option<i32>, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getSlot", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(Some(res.i()?))
    }
    // SUPER CLASS: InventoryInteractEvent
    pub fn is_cancelled(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone =
            crate::event::inventory::InventoryInteractEvent::from_raw(&self.0, unsafe {
                jni::objects::JObject::from_raw(self.1.clone())
            })?;
        let real: crate::event::Cancellable = temp_clone.into();
        real.is_cancelled()
    }
    pub fn set_cancelled(&self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone =
            crate::event::inventory::InventoryInteractEvent::from_raw(&self.0, unsafe {
                jni::objects::JObject::from_raw(self.1.clone())
            })?;
        let real: crate::event::Cancellable = temp_clone.into();
        real.set_cancelled(arg0)
    }
    pub fn result(&self) -> Result<crate::event::EventResult<'mc>, Box<dyn std::error::Error>> {
        let temp_clone =
            crate::event::inventory::InventoryInteractEvent::from_raw(&self.0, unsafe {
                jni::objects::JObject::from_raw(self.1.clone())
            })?;
        let real: crate::event::inventory::InventoryInteractEvent = temp_clone.into();
        real.result()
    }
    pub fn set_result(
        &self,
        arg0: impl Into<crate::event::EventResult<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone =
            crate::event::inventory::InventoryInteractEvent::from_raw(&self.0, unsafe {
                jni::objects::JObject::from_raw(self.1.clone())
            })?;
        let real: crate::event::inventory::InventoryInteractEvent = temp_clone.into();
        real.set_result(arg0)
    }
    pub fn who_clicked(
        &self,
    ) -> Result<crate::entity::HumanEntity<'mc>, Box<dyn std::error::Error>> {
        let temp_clone =
            crate::event::inventory::InventoryInteractEvent::from_raw(&self.0, unsafe {
                jni::objects::JObject::from_raw(self.1.clone())
            })?;
        let real: crate::event::inventory::InventoryInteractEvent = temp_clone.into();
        real.who_clicked()
    }
    // SUPER CLASS: InventoryEvent
    pub fn inventory(
        &self,
    ) -> Result<crate::inventory::Inventory<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::inventory::InventoryEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::inventory::InventoryEvent = temp_clone.into();
        real.inventory()
    }
    pub fn viewers(
        &self,
    ) -> Result<Vec<crate::entity::HumanEntity<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/List;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getViewers", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let list = blackboxmc_java::util::JavaList::from_raw(&self.0, res.l()?)?;
        let iter = list.iterator()?;
        while iter.has_next()? {
            let obj = iter.next()?;
            new_vec.push(crate::entity::HumanEntity::from_raw(&self.0, obj)?);
        }
        Ok(new_vec)
    }
    pub fn view(&self) -> Result<crate::inventory::InventoryView<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::inventory::InventoryEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::inventory::InventoryEvent = temp_clone.into();
        real.view()
    }
    // SUPER CLASS: Event
    pub fn event_name(&self) -> Result<String, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::Event::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::Event = temp_clone.into();
        real.event_name()
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
impl<'mc> Into<crate::event::inventory::InventoryInteractEvent<'mc>> for InventoryClickEvent<'mc> {
    fn into(self) -> crate::event::inventory::InventoryInteractEvent<'mc> {
        crate::event::inventory::InventoryInteractEvent::from_raw(&self.jni_ref(), self.1).expect("Error converting InventoryClickEvent into crate::event::inventory::InventoryInteractEvent")
    }
}
/// This event is called when the player drags an item in their cursor across the inventory. The ItemStack is distributed across the slots the HumanEntity dragged over. The method of distribution is described by the DragType returned by <a href="#getType()"><code>getType()</code></a>.
/// <p>Canceling this event will result in none of the changes described in <a href="#getNewItems()"><code>getNewItems()</code></a> being applied to the Inventory.</p>
/// <p>Because InventoryDragEvent occurs within a modification of the Inventory, not all Inventory related methods are safe to use.</p>
/// <p>The following should never be invoked by an EventHandler for InventoryDragEvent using the HumanEntity or InventoryView associated with this event.</p>
/// <ul>
/// <li><a href="../../entity/HumanEntity.html#closeInventory()"><code>HumanEntity.closeInventory()</code></a></li>
/// <li><a href="../../entity/HumanEntity.html#openInventory(org.bukkit.inventory.Inventory)"><code>HumanEntity.openInventory(Inventory)</code></a></li>
/// <li><a href="../../entity/HumanEntity.html#openWorkbench(org.bukkit.Location,boolean)"><code>HumanEntity.openWorkbench(Location, boolean)</code></a></li>
/// <li><a href="../../entity/HumanEntity.html#openEnchanting(org.bukkit.Location,boolean)"><code>HumanEntity.openEnchanting(Location, boolean)</code></a></li>
/// <li><a href="../../inventory/InventoryView.html#close()"><code>InventoryView.close()</code></a></li>
/// </ul> To invoke one of these methods, schedule a task using <a href="../../scheduler/BukkitScheduler.html#runTask(org.bukkit.plugin.Plugin,java.lang.Runnable)"><code>BukkitScheduler.runTask(Plugin, Runnable)</code></a>, which will run the task on the next tick. Also be aware that this is not an exhaustive list, and other methods could potentially create issues as well.
/// <p>Assuming the EntityHuman associated with this event is an instance of a Player, manipulating the MaxStackSize or contents of an Inventory will require an Invocation of <a href="../../entity/Player.html#updateInventory()"><code>Player.updateInventory()</code></a>.</p>
/// <p>Any modifications to slots that are modified by the results of this InventoryDragEvent will be overwritten. To change these slots, this event should be cancelled and the changes applied. Alternatively, scheduling a task using <a href="../../scheduler/BukkitScheduler.html#runTask(org.bukkit.plugin.Plugin,java.lang.Runnable)"><code>BukkitScheduler.runTask(Plugin, Runnable)</code></a>, which would execute the task on the next tick, would work as well.</p>
#[repr(C)]
pub struct InventoryDragEvent<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for InventoryDragEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for InventoryDragEvent<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate InventoryDragEvent from null object.").into(),
            );
        }
        let (valid, name) =
            env.validate_name(&obj, "org/bukkit/event/inventory/InventoryDragEvent")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a InventoryDragEvent object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> InventoryDragEvent<'mc> {
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<crate::inventory::InventoryView<'mc>>,
        arg1: impl Into<crate::inventory::ItemStack<'mc>>,
        arg2: impl Into<crate::inventory::ItemStack<'mc>>,
        arg3: bool,
        arg4: impl Into<blackboxmc_java::util::JavaMap<'mc>>,
    ) -> Result<crate::event::inventory::InventoryDragEvent<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/inventory/InventoryView;Lorg/bukkit/inventory/ItemStack;Lorg/bukkit/inventory/ItemStack;ZLjava/util/Map;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let val_2 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg1.into().jni_object().clone())
        });
        let val_3 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg2.into().jni_object().clone())
        });
        let val_4 = jni::objects::JValueGen::Bool(arg3.into());
        let val_5 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg4.into().jni_object().clone())
        });
        let cls = jni.find_class("org/bukkit/event/inventory/InventoryDragEvent");
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
        crate::event::inventory::InventoryDragEvent::from_raw(&jni, res)
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

    pub fn set_cursor(
        &self,
        arg0: impl Into<crate::inventory::ItemStack<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/inventory/ItemStack;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setCursor",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn cursor(
        &self,
    ) -> Result<Option<crate::inventory::ItemStack<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/inventory/ItemStack;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getCursor", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {
            return Ok(None);
        }
        Ok(Some(crate::inventory::ItemStack::from_raw(
            &self.jni_ref(),
            unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) },
        )?))
    }

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

    pub fn new_items(
        &self,
    ) -> Result<blackboxmc_java::util::JavaMap<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/Map;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getNewItems", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::util::JavaMap::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn raw_slots(
        &self,
    ) -> Result<blackboxmc_java::util::JavaSet<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/Set;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getRawSlots", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::util::JavaSet::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn inventory_slots(
        &self,
    ) -> Result<blackboxmc_java::util::JavaSet<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/Set;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getInventorySlots",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::util::JavaSet::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn old_cursor(
        &self,
    ) -> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/inventory/ItemStack;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getOldCursor", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::ItemStack::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn get_type(
        &self,
    ) -> Result<crate::event::inventory::DragType<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/event/inventory/DragType;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getType", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::event::inventory::DragType::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    // SUPER CLASS: InventoryInteractEvent
    pub fn is_cancelled(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone =
            crate::event::inventory::InventoryInteractEvent::from_raw(&self.0, unsafe {
                jni::objects::JObject::from_raw(self.1.clone())
            })?;
        let real: crate::event::Cancellable = temp_clone.into();
        real.is_cancelled()
    }
    pub fn set_cancelled(&self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone =
            crate::event::inventory::InventoryInteractEvent::from_raw(&self.0, unsafe {
                jni::objects::JObject::from_raw(self.1.clone())
            })?;
        let real: crate::event::Cancellable = temp_clone.into();
        real.set_cancelled(arg0)
    }
    pub fn result(&self) -> Result<crate::event::EventResult<'mc>, Box<dyn std::error::Error>> {
        let temp_clone =
            crate::event::inventory::InventoryInteractEvent::from_raw(&self.0, unsafe {
                jni::objects::JObject::from_raw(self.1.clone())
            })?;
        let real: crate::event::inventory::InventoryInteractEvent = temp_clone.into();
        real.result()
    }
    pub fn set_result(
        &self,
        arg0: impl Into<crate::event::EventResult<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone =
            crate::event::inventory::InventoryInteractEvent::from_raw(&self.0, unsafe {
                jni::objects::JObject::from_raw(self.1.clone())
            })?;
        let real: crate::event::inventory::InventoryInteractEvent = temp_clone.into();
        real.set_result(arg0)
    }
    pub fn who_clicked(
        &self,
    ) -> Result<crate::entity::HumanEntity<'mc>, Box<dyn std::error::Error>> {
        let temp_clone =
            crate::event::inventory::InventoryInteractEvent::from_raw(&self.0, unsafe {
                jni::objects::JObject::from_raw(self.1.clone())
            })?;
        let real: crate::event::inventory::InventoryInteractEvent = temp_clone.into();
        real.who_clicked()
    }
    // SUPER CLASS: InventoryEvent
    pub fn inventory(
        &self,
    ) -> Result<crate::inventory::Inventory<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::inventory::InventoryEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::inventory::InventoryEvent = temp_clone.into();
        real.inventory()
    }
    pub fn viewers(
        &self,
    ) -> Result<Vec<crate::entity::HumanEntity<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/List;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getViewers", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let list = blackboxmc_java::util::JavaList::from_raw(&self.0, res.l()?)?;
        let iter = list.iterator()?;
        while iter.has_next()? {
            let obj = iter.next()?;
            new_vec.push(crate::entity::HumanEntity::from_raw(&self.0, obj)?);
        }
        Ok(new_vec)
    }
    pub fn view(&self) -> Result<crate::inventory::InventoryView<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::inventory::InventoryEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::inventory::InventoryEvent = temp_clone.into();
        real.view()
    }
    // SUPER CLASS: Event
    pub fn event_name(&self) -> Result<String, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::Event::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::Event = temp_clone.into();
        real.event_name()
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
impl<'mc> Into<crate::event::inventory::InventoryInteractEvent<'mc>> for InventoryDragEvent<'mc> {
    fn into(self) -> crate::event::inventory::InventoryInteractEvent<'mc> {
        crate::event::inventory::InventoryInteractEvent::from_raw(&self.jni_ref(), self.1).expect("Error converting InventoryDragEvent into crate::event::inventory::InventoryInteractEvent")
    }
}
/// Called when an ItemStack is successfully burned as fuel in a furnace.
#[repr(C)]
pub struct FurnaceBurnEvent<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for FurnaceBurnEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for FurnaceBurnEvent<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate FurnaceBurnEvent from null object.").into(),
            );
        }
        let (valid, name) =
            env.validate_name(&obj, "org/bukkit/event/inventory/FurnaceBurnEvent")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a FurnaceBurnEvent object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> FurnaceBurnEvent<'mc> {
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<crate::block::Block<'mc>>,
        arg1: impl Into<crate::inventory::ItemStack<'mc>>,
        arg2: i32,
    ) -> Result<crate::event::inventory::FurnaceBurnEvent<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/Block;Lorg/bukkit/inventory/ItemStack;I)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let val_2 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg1.into().jni_object().clone())
        });
        let val_3 = jni::objects::JValueGen::Int(arg2);
        let cls = jni.find_class("org/bukkit/event/inventory/FurnaceBurnEvent");
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
        crate::event::inventory::FurnaceBurnEvent::from_raw(&jni, res)
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

    pub fn is_cancelled(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isCancelled", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn fuel(
        &self,
    ) -> Result<Option<crate::inventory::ItemStack<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/inventory/ItemStack;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getFuel", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {
            return Ok(None);
        }
        Ok(Some(crate::inventory::ItemStack::from_raw(
            &self.jni_ref(),
            unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) },
        )?))
    }

    pub fn burn_time(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getBurnTime", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    /// Sets the burn time for this fuel
    pub fn set_burn_time(&self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(I)V");
        let val_1 = jni::objects::JValueGen::Int(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setBurnTime",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// <span class="descfrm-type-label">Description copied from interface:&nbsp;<code><a href="../Cancellable.html#setCancelled(boolean)">Cancellable</a></code></span>
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

    pub fn is_burning(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isBurning", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Sets whether the furnace's fuel is burning or not.
    pub fn set_burning(&self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        let val_1 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setBurning",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    // SUPER CLASS: BlockEvent
    pub fn block(&self) -> Result<crate::block::Block<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::block::BlockEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::block::BlockEvent = temp_clone.into();
        real.block()
    }
    // SUPER CLASS: Event
    pub fn event_name(&self) -> Result<String, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::Event::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::Event = temp_clone.into();
        real.event_name()
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
impl<'mc> Into<crate::event::Cancellable<'mc>> for FurnaceBurnEvent<'mc> {
    fn into(self) -> crate::event::Cancellable<'mc> {
        crate::event::Cancellable::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting FurnaceBurnEvent into crate::event::Cancellable")
    }
}
impl<'mc> Into<crate::event::block::BlockEvent<'mc>> for FurnaceBurnEvent<'mc> {
    fn into(self) -> crate::event::block::BlockEvent<'mc> {
        crate::event::block::BlockEvent::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting FurnaceBurnEvent into crate::event::block::BlockEvent")
    }
}
/// Represents a player related inventory event
#[repr(C)]
pub struct InventoryCloseEvent<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for InventoryCloseEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for InventoryCloseEvent<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate InventoryCloseEvent from null object.").into(),
            );
        }
        let (valid, name) =
            env.validate_name(&obj, "org/bukkit/event/inventory/InventoryCloseEvent")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a InventoryCloseEvent object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> InventoryCloseEvent<'mc> {
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<crate::inventory::InventoryView<'mc>>,
    ) -> Result<crate::event::inventory::InventoryCloseEvent<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/inventory/InventoryView;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let cls = jni.find_class("org/bukkit/event/inventory/InventoryCloseEvent");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(
            cls,
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = jni.translate_error_no_gen(res)?;
        crate::event::inventory::InventoryCloseEvent::from_raw(&jni, res)
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

    pub fn player(
        &self,
    ) -> Result<Option<crate::entity::HumanEntity<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/entity/HumanEntity;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getPlayer", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {
            return Ok(None);
        }
        Ok(Some(crate::entity::HumanEntity::from_raw(
            &self.jni_ref(),
            unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) },
        )?))
    }

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
    // SUPER CLASS: InventoryEvent
    pub fn inventory(
        &self,
    ) -> Result<crate::inventory::Inventory<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::inventory::InventoryEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::inventory::InventoryEvent = temp_clone.into();
        real.inventory()
    }
    pub fn viewers(
        &self,
    ) -> Result<Vec<crate::entity::HumanEntity<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/List;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getViewers", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let list = blackboxmc_java::util::JavaList::from_raw(&self.0, res.l()?)?;
        let iter = list.iterator()?;
        while iter.has_next()? {
            let obj = iter.next()?;
            new_vec.push(crate::entity::HumanEntity::from_raw(&self.0, obj)?);
        }
        Ok(new_vec)
    }
    pub fn view(&self) -> Result<crate::inventory::InventoryView<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::inventory::InventoryEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::inventory::InventoryEvent = temp_clone.into();
        real.view()
    }
    // SUPER CLASS: Event
    pub fn event_name(&self) -> Result<String, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::Event::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::Event = temp_clone.into();
        real.event_name()
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
impl<'mc> Into<crate::event::inventory::InventoryEvent<'mc>> for InventoryCloseEvent<'mc> {
    fn into(self) -> crate::event::inventory::InventoryEvent<'mc> {
        crate::event::inventory::InventoryEvent::from_raw(&self.jni_ref(), self.1).expect(
            "Error converting InventoryCloseEvent into crate::event::inventory::InventoryEvent",
        )
    }
}
/// Called when an item is put in a slot and the result is calculated.
#[repr(C)]
pub struct PrepareInventoryResultEvent<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for PrepareInventoryResultEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for PrepareInventoryResultEvent<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate PrepareInventoryResultEvent from null object."
            )
            .into());
        }
        let (valid, name) = env.validate_name(
            &obj,
            "org/bukkit/event/inventory/PrepareInventoryResultEvent",
        )?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a PrepareInventoryResultEvent object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> PrepareInventoryResultEvent<'mc> {
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<crate::inventory::InventoryView<'mc>>,
        arg1: impl Into<crate::inventory::ItemStack<'mc>>,
    ) -> Result<crate::event::inventory::PrepareInventoryResultEvent<'mc>, Box<dyn std::error::Error>>
    {
        let sig =
            String::from("(Lorg/bukkit/inventory/InventoryView;Lorg/bukkit/inventory/ItemStack;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let val_2 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg1.into().jni_object().clone())
        });
        let cls = jni.find_class("org/bukkit/event/inventory/PrepareInventoryResultEvent");
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
        crate::event::inventory::PrepareInventoryResultEvent::from_raw(&jni, res)
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

    pub fn result(&self) -> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/inventory/ItemStack;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getResult", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::ItemStack::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn set_result(
        &self,
        arg0: impl Into<crate::inventory::ItemStack<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/inventory/ItemStack;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setResult",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

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
    // SUPER CLASS: InventoryEvent
    pub fn inventory(
        &self,
    ) -> Result<crate::inventory::Inventory<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::inventory::InventoryEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::inventory::InventoryEvent = temp_clone.into();
        real.inventory()
    }
    pub fn viewers(
        &self,
    ) -> Result<Vec<crate::entity::HumanEntity<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/List;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getViewers", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let list = blackboxmc_java::util::JavaList::from_raw(&self.0, res.l()?)?;
        let iter = list.iterator()?;
        while iter.has_next()? {
            let obj = iter.next()?;
            new_vec.push(crate::entity::HumanEntity::from_raw(&self.0, obj)?);
        }
        Ok(new_vec)
    }
    pub fn view(&self) -> Result<crate::inventory::InventoryView<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::inventory::InventoryEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::inventory::InventoryEvent = temp_clone.into();
        real.view()
    }
    // SUPER CLASS: Event
    pub fn event_name(&self) -> Result<String, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::Event::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::Event = temp_clone.into();
        real.event_name()
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
impl<'mc> Into<crate::event::inventory::InventoryEvent<'mc>> for PrepareInventoryResultEvent<'mc> {
    fn into(self) -> crate::event::inventory::InventoryEvent<'mc> {
        crate::event::inventory::InventoryEvent::from_raw(&self.jni_ref(), self.1).expect("Error converting PrepareInventoryResultEvent into crate::event::inventory::InventoryEvent")
    }
}
