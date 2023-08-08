#![allow(deprecated)]
use blackboxmc_general::JNIRaw;
use color_eyre::eyre::Result;
/// Represents a player related inventory event
pub struct InventoryOpenEvent<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for InventoryOpenEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> InventoryOpenEvent<'mc> {
    pub fn from_raw(
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
    pub fn new(
        jni: blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<&'mc crate::inventory::InventoryView<'mc>>,
    ) -> Result<crate::event::inventory::InventoryOpenEvent<'mc>, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let cls = &jni.find_class("org/bukkit/event/inventory/InventoryOpenEvent")?;
        let res = jni.new_object(
            cls,
            "(Lorg/bukkit/inventory/InventoryView;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        )?;
        crate::event::inventory::InventoryOpenEvent::from_raw(&jni, res)
    }
    /// Gets the cancellation state of this event. A cancelled event will not be executed in the server, but will still pass to other plugins.
    /// <p>If an inventory open event is cancelled, the inventory screen will not show.</p>
    pub fn is_cancelled(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isCancelled", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
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
    /// Returns the player involved in this event
    pub fn player(
        &mut self,
    ) -> Result<crate::entity::HumanEntity<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getPlayer",
            "()Lorg/bukkit/entity/HumanEntity;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::entity::HumanEntity::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Sets the cancellation state of this event. A cancelled event will not be executed in the server, but will still pass to other plugins.
    /// <p>If an inventory open event is cancelled, the inventory screen will not show.</p>
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

    pub fn handler_list(
        jni: blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>> {
        let cls = &jni.find_class("org/bukkit/event/HandlerList")?;
        let res = jni.call_static_method(
            cls,
            "getHandlerList",
            "()Lorg/bukkit/event/HandlerList;",
            &[],
        )?;
        let obj = res.l()?;
        crate::event::HandlerList::from_raw(&jni, obj)
    }

    pub fn inventory(
        &mut self,
    ) -> Result<crate::inventory::Inventory<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getInventory",
            "()Lorg/bukkit/inventory/Inventory;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::Inventory::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn viewers(
        &mut self,
    ) -> Result<Vec<crate::entity::HumanEntity<'mc>>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getViewers", "()Ljava/util/List;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let mut list = blackboxmc_java::JavaList::from_raw(&self.0, res.l()?)?;
        let size = list.size()?;
        for i in 0..=size {
            let obj = list.get(i)?;
            new_vec.push(crate::entity::HumanEntity::from_raw(&self.0, obj)?);
        }
        Ok(new_vec)
    }

    pub fn view(
        &mut self,
    ) -> Result<crate::inventory::InventoryView<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getView",
            "()Lorg/bukkit/inventory/InventoryView;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::InventoryView::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

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
impl<'mc> Into<crate::event::Cancellable<'mc>> for InventoryOpenEvent<'mc> {
    fn into(self) -> crate::event::Cancellable<'mc> {
        crate::event::Cancellable::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
impl<'mc> Into<crate::event::inventory::InventoryEvent<'mc>> for InventoryOpenEvent<'mc> {
    fn into(self) -> crate::event::inventory::InventoryEvent<'mc> {
        crate::event::inventory::InventoryEvent::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}

pub struct PrepareItemCraftEvent<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for PrepareItemCraftEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> PrepareItemCraftEvent<'mc> {
    pub fn from_raw(
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
    pub fn new(
        jni: blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<&'mc crate::inventory::CraftingInventory<'mc>>,
        arg1: impl Into<&'mc crate::inventory::InventoryView<'mc>>,
        arg2: bool,
    ) -> Result<crate::event::inventory::PrepareItemCraftEvent<'mc>, Box<dyn std::error::Error>>
    {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let val_2 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone()) };
        // -2
        let val_3 = jni::objects::JValueGen::Bool(arg2.into());
        let cls = &jni.find_class("org/bukkit/event/inventory/PrepareItemCraftEvent")?;
        let res = jni.new_object(
            cls,
            "(Lorg/bukkit/inventory/CraftingInventory;Lorg/bukkit/inventory/InventoryView;Z)V",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
                jni::objects::JValueGen::from(&val_3),
            ],
        )?;
        crate::event::inventory::PrepareItemCraftEvent::from_raw(&jni, res)
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
    /// <span class="descfrm-type-label">Description copied from class:&nbsp;<code><a href="InventoryEvent.html#getInventory()">InventoryEvent</a></code></span>
    /// Gets the primary Inventory involved in this transaction
    pub fn inventory(
        &mut self,
    ) -> Result<crate::inventory::Inventory<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getInventory",
            "()Lorg/bukkit/inventory/Inventory;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::Inventory::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Get the recipe that has been formed. If this event was triggered by a tool repair, this will be a temporary shapeless recipe representing the repair.
    pub fn recipe(&mut self) -> Result<crate::inventory::Recipe<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getRecipe",
            "()Lorg/bukkit/inventory/Recipe;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::Recipe::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn handler_list(
        jni: blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>> {
        let cls = &jni.find_class("org/bukkit/event/HandlerList")?;
        let res = jni.call_static_method(
            cls,
            "getHandlerList",
            "()Lorg/bukkit/event/HandlerList;",
            &[],
        )?;
        let obj = res.l()?;
        crate::event::HandlerList::from_raw(&jni, obj)
    }
    /// Check if this event was triggered by a tool repair operation rather than a crafting recipe.
    pub fn is_repair(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isRepair", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn viewers(
        &mut self,
    ) -> Result<Vec<crate::entity::HumanEntity<'mc>>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getViewers", "()Ljava/util/List;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let mut list = blackboxmc_java::JavaList::from_raw(&self.0, res.l()?)?;
        let size = list.size()?;
        for i in 0..=size {
            let obj = list.get(i)?;
            new_vec.push(crate::entity::HumanEntity::from_raw(&self.0, obj)?);
        }
        Ok(new_vec)
    }

    pub fn view(
        &mut self,
    ) -> Result<crate::inventory::InventoryView<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getView",
            "()Lorg/bukkit/inventory/InventoryView;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::InventoryView::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

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
impl<'mc> Into<crate::event::inventory::InventoryEvent<'mc>> for PrepareItemCraftEvent<'mc> {
    fn into(self) -> crate::event::inventory::InventoryEvent<'mc> {
        crate::event::inventory::InventoryEvent::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
/// Called when an item is put in a slot for repair by an anvil.
pub struct PrepareAnvilEvent<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for PrepareAnvilEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> PrepareAnvilEvent<'mc> {
    pub fn from_raw(
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
    pub fn new(
        jni: blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<&'mc crate::inventory::InventoryView<'mc>>,
        arg1: impl Into<&'mc crate::inventory::ItemStack<'mc>>,
    ) -> Result<crate::event::inventory::PrepareAnvilEvent<'mc>, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let val_2 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone()) };
        let cls = &jni.find_class("org/bukkit/event/inventory/PrepareAnvilEvent")?;
        let res = jni.new_object(
            cls,
            "(Lorg/bukkit/inventory/InventoryView;Lorg/bukkit/inventory/ItemStack;)V",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        )?;
        crate::event::inventory::PrepareAnvilEvent::from_raw(&jni, res)
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
    /// <span class="descfrm-type-label">Description copied from class:&nbsp;<code><a href="InventoryEvent.html#getInventory()">InventoryEvent</a></code></span>
    /// Gets the primary Inventory involved in this transaction
    pub fn inventory(
        &mut self,
    ) -> Result<crate::inventory::AnvilInventory<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getInventory",
            "()Lorg/bukkit/inventory/AnvilInventory;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::AnvilInventory::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn handler_list(
        jni: blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>> {
        let cls = &jni.find_class("org/bukkit/event/HandlerList")?;
        let res = jni.call_static_method(
            cls,
            "getHandlerList",
            "()Lorg/bukkit/event/HandlerList;",
            &[],
        )?;
        let obj = res.l()?;
        crate::event::HandlerList::from_raw(&jni, obj)
    }

    pub fn result(
        &mut self,
    ) -> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getResult",
            "()Lorg/bukkit/inventory/ItemStack;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::ItemStack::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn set_result(
        &mut self,
        arg0: impl Into<&'mc crate::inventory::ItemStack<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setResult",
            "(Lorg/bukkit/inventory/ItemStack;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn viewers(
        &mut self,
    ) -> Result<Vec<crate::entity::HumanEntity<'mc>>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getViewers", "()Ljava/util/List;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let mut list = blackboxmc_java::JavaList::from_raw(&self.0, res.l()?)?;
        let size = list.size()?;
        for i in 0..=size {
            let obj = list.get(i)?;
            new_vec.push(crate::entity::HumanEntity::from_raw(&self.0, obj)?);
        }
        Ok(new_vec)
    }

    pub fn view(
        &mut self,
    ) -> Result<crate::inventory::InventoryView<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getView",
            "()Lorg/bukkit/inventory/InventoryView;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::InventoryView::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

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
impl<'mc> Into<crate::event::inventory::PrepareInventoryResultEvent<'mc>>
    for PrepareAnvilEvent<'mc>
{
    fn into(self) -> crate::event::inventory::PrepareInventoryResultEvent<'mc> {
        crate::event::inventory::PrepareInventoryResultEvent::from_raw(&self.jni_ref(), self.1)
            .unwrap()
    }
}
pub enum DragTypeEnum {
    Single,
    Even,
}
impl std::fmt::Display for DragTypeEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            DragTypeEnum::Single => f.write_str("SINGLE"),
            DragTypeEnum::Even => f.write_str("EVEN"),
        }
    }
}
pub struct DragType<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
    pub DragTypeEnum,
);
impl<'mc> std::ops::Deref for DragType<'mc> {
    type Target = DragTypeEnum;
    fn deref(&self) -> &Self::Target {
        return &self.2;
    }
}
impl<'mc> JNIRaw<'mc> for DragType<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> DragType<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
        e: DragTypeEnum,
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
            Ok(Self(env.clone(), obj, e))
        }
    }
    pub const SINGLE: DragTypeEnum = DragTypeEnum::Single;
    pub const EVEN: DragTypeEnum = DragTypeEnum::Even;
    pub fn from_string(str: String) -> std::option::Option<DragTypeEnum> {
        match str.as_str() {
            "SINGLE" => Some(DragTypeEnum::Single),
            "EVEN" => Some(DragTypeEnum::Even),
            _ => None,
        }
    }
}
/// Called when the brewing of the contents inside the Brewing Stand is complete.
pub struct BrewEvent<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for BrewEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> BrewEvent<'mc> {
    pub fn from_raw(
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
    pub fn new(
        jni: blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<&'mc crate::block::Block<'mc>>,
        arg1: impl Into<&'mc crate::inventory::BrewerInventory<'mc>>,
        arg2: Vec<impl Into<crate::inventory::ItemStack<'mc>>>,
        arg3: i32,
    ) -> Result<crate::event::inventory::BrewEvent<'mc>, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let val_2 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone()) };
        let raw_val_3 = jni.new_object("java/util/ArrayList", "()V", &[]).unwrap();
        for v in arg2 {
            let map_val_0 =
                unsafe { jni::objects::JObject::from_raw(v.into().jni_object().clone()) };
            jni.call_method(
                &raw_val_3,
                "add",
                "(Ljava/Lang/Object)V",
                &[jni::objects::JValueGen::from(&map_val_0)],
            )?;
        }
        let val_3 = jni::objects::JValueGen::Object(raw_val_3);
        let val_4 = jni::objects::JValueGen::Int(arg3.into());
        let cls = &jni.find_class("org/bukkit/event/inventory/BrewEvent")?;
        let res = jni.new_object(
            cls,
            "(Lorg/bukkit/block/Block;Lorg/bukkit/inventory/BrewerInventory;Ljava/util/List;I)V",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
                jni::objects::JValueGen::from(&val_3),
                jni::objects::JValueGen::from(&val_4),
            ],
        )?;
        crate::event::inventory::BrewEvent::from_raw(&jni, res)
    }
    /// <span class="descfrm-type-label">Description copied from interface:&nbsp;<code><a href="../Cancellable.html#isCancelled()">Cancellable</a></code></span>
    /// Gets the cancellation state of this event. A cancelled event will not be executed in the server, but will still pass to other plugins
    pub fn is_cancelled(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isCancelled", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
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
    /// Gets the contents of the Brewing Stand. <b>Note:</b> The brewer inventory still holds the items found prior to the finalization of the brewing process, e.g. the plain water bottles.
    pub fn contents(
        &mut self,
    ) -> Result<crate::inventory::BrewerInventory<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getContents",
            "()Lorg/bukkit/inventory/BrewerInventory;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::BrewerInventory::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Gets the remaining fuel level.
    pub fn fuel_level(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getFuelLevel", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }
    /// <span class="descfrm-type-label">Description copied from interface:&nbsp;<code><a href="../Cancellable.html#setCancelled(boolean)">Cancellable</a></code></span>
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

    pub fn handler_list(
        jni: blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>> {
        let cls = &jni.find_class("org/bukkit/event/HandlerList")?;
        let res = jni.call_static_method(
            cls,
            "getHandlerList",
            "()Lorg/bukkit/event/HandlerList;",
            &[],
        )?;
        let obj = res.l()?;
        crate::event::HandlerList::from_raw(&jni, obj)
    }
    /// Gets the resulting items in the Brewing Stand. The returned list, in case of a server-created event instance, is mutable. Any changes in the returned list will reflect in the brewing result if the event is not cancelled. If the size of the list is reduced, remaining items will be set to air.
    pub fn results(
        &mut self,
    ) -> Result<Vec<crate::inventory::ItemStack<'mc>>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getResults", "()Ljava/util/List;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let mut list = blackboxmc_java::JavaList::from_raw(&self.0, res.l()?)?;
        let size = list.size()?;
        for i in 0..=size {
            let obj = list.get(i)?;
            new_vec.push(crate::inventory::ItemStack::from_raw(&self.0, obj)?);
        }
        Ok(new_vec)
    }

    pub fn block(&mut self) -> Result<crate::block::Block<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getBlock",
            "()Lorg/bukkit/block/Block;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::block::Block::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

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
impl<'mc> Into<crate::event::Cancellable<'mc>> for BrewEvent<'mc> {
    fn into(self) -> crate::event::Cancellable<'mc> {
        crate::event::Cancellable::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
impl<'mc> Into<crate::event::block::BlockEvent<'mc>> for BrewEvent<'mc> {
    fn into(self) -> crate::event::block::BlockEvent<'mc> {
        crate::event::block::BlockEvent::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
/// Called when some entity or block (e.g. hopper) tries to move items directly from one inventory to another.
/// <p>When this event is called, the initiator may already have removed the item from the source inventory and is ready to move it into the destination inventory.</p>
/// <p>If this event is cancelled, the items will be returned to the source inventory, if needed.</p>
/// <p>If this event is not cancelled, the initiator will try to put the ItemStack into the destination inventory. If this is not possible and the ItemStack has not been modified, the source inventory slot will be restored to its former state. Otherwise any additional items will be discarded.</p>
pub struct InventoryMoveItemEvent<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for InventoryMoveItemEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> InventoryMoveItemEvent<'mc> {
    pub fn from_raw(
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
    pub fn new(
        jni: blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<&'mc crate::inventory::Inventory<'mc>>,
        arg1: impl Into<&'mc crate::inventory::ItemStack<'mc>>,
        arg2: impl Into<&'mc crate::inventory::Inventory<'mc>>,
        arg3: bool,
    ) -> Result<crate::event::inventory::InventoryMoveItemEvent<'mc>, Box<dyn std::error::Error>>
    {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let val_2 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone()) };
        let val_3 = unsafe { jni::objects::JObject::from_raw(arg2.into().jni_object().clone()) };
        // -2
        let val_4 = jni::objects::JValueGen::Bool(arg3.into());
        let cls = &jni.find_class("org/bukkit/event/inventory/InventoryMoveItemEvent")?;
        let res = jni.new_object(cls,
"(Lorg/bukkit/inventory/Inventory;Lorg/bukkit/inventory/ItemStack;Lorg/bukkit/inventory/Inventory;Z)V",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2),jni::objects::JValueGen::from(&val_3),jni::objects::JValueGen::from(&val_4)])?;
        crate::event::inventory::InventoryMoveItemEvent::from_raw(&jni, res)
    }
    /// Gets the ItemStack being moved; if modified, the original item will not be removed from the source inventory.
    pub fn item(&mut self) -> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getItem",
            "()Lorg/bukkit/inventory/ItemStack;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::ItemStack::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Gets the Inventory that the ItemStack is being taken from
    pub fn source(
        &mut self,
    ) -> Result<crate::inventory::Inventory<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getSource",
            "()Lorg/bukkit/inventory/Inventory;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::Inventory::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// <span class="descfrm-type-label">Description copied from interface:&nbsp;<code><a href="../Cancellable.html#isCancelled()">Cancellable</a></code></span>
    /// Gets the cancellation state of this event. A cancelled event will not be executed in the server, but will still pass to other plugins
    pub fn is_cancelled(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isCancelled", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
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
    /// Sets the ItemStack being moved; if this is different from the original ItemStack, the original item will not be removed from the source inventory.
    pub fn set_item(
        &mut self,
        arg0: impl Into<&'mc crate::inventory::ItemStack<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setItem",
            "(Lorg/bukkit/inventory/ItemStack;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Gets the Inventory that the ItemStack is being put into
    pub fn destination(
        &mut self,
    ) -> Result<crate::inventory::Inventory<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getDestination",
            "()Lorg/bukkit/inventory/Inventory;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::Inventory::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// <span class="descfrm-type-label">Description copied from interface:&nbsp;<code><a href="../Cancellable.html#setCancelled(boolean)">Cancellable</a></code></span>
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

    pub fn handler_list(
        jni: blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>> {
        let cls = &jni.find_class("org/bukkit/event/HandlerList")?;
        let res = jni.call_static_method(
            cls,
            "getHandlerList",
            "()Lorg/bukkit/event/HandlerList;",
            &[],
        )?;
        let obj = res.l()?;
        crate::event::HandlerList::from_raw(&jni, obj)
    }
    /// Gets the Inventory that initiated the transfer. This will always be either the destination or source Inventory.
    pub fn initiator(
        &mut self,
    ) -> Result<crate::inventory::Inventory<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getInitiator",
            "()Lorg/bukkit/inventory/Inventory;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::Inventory::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

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
impl<'mc> Into<crate::event::Cancellable<'mc>> for InventoryMoveItemEvent<'mc> {
    fn into(self) -> crate::event::Cancellable<'mc> {
        crate::event::Cancellable::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
impl<'mc> Into<crate::event::Event<'mc>> for InventoryMoveItemEvent<'mc> {
    fn into(self) -> crate::event::Event<'mc> {
        crate::event::Event::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
/// Called when an ItemStack is successfully smelted in a furnace.
pub struct FurnaceSmeltEvent<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for FurnaceSmeltEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> FurnaceSmeltEvent<'mc> {
    pub fn from_raw(
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
    pub fn new(
        jni: blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<&'mc crate::block::Block<'mc>>,
        arg1: impl Into<&'mc crate::inventory::ItemStack<'mc>>,
        arg2: impl Into<&'mc crate::inventory::ItemStack<'mc>>,
    ) -> Result<crate::event::inventory::FurnaceSmeltEvent<'mc>, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let val_2 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone()) };
        let val_3 = unsafe { jni::objects::JObject::from_raw(arg2.into().jni_object().clone()) };
        let cls = &jni.find_class("org/bukkit/event/inventory/FurnaceSmeltEvent")?;
        let res = jni.new_object(cls,
"(Lorg/bukkit/block/Block;Lorg/bukkit/inventory/ItemStack;Lorg/bukkit/inventory/ItemStack;)V",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2),jni::objects::JValueGen::from(&val_3)])?;
        crate::event::inventory::FurnaceSmeltEvent::from_raw(&jni, res)
    }

    pub fn result(
        &mut self,
    ) -> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getResult",
            "()Lorg/bukkit/inventory/ItemStack;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::ItemStack::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn source(
        &mut self,
    ) -> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getSource",
            "()Lorg/bukkit/inventory/ItemStack;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::ItemStack::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn set_result(
        &mut self,
        arg0: impl Into<&'mc crate::inventory::ItemStack<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setResult",
            "(Lorg/bukkit/inventory/ItemStack;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn is_cancelled(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isCancelled", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
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

    pub fn handler_list(
        jni: blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>> {
        let cls = &jni.find_class("org/bukkit/event/HandlerList")?;
        let res = jni.call_static_method(
            cls,
            "getHandlerList",
            "()Lorg/bukkit/event/HandlerList;",
            &[],
        )?;
        let obj = res.l()?;
        crate::event::HandlerList::from_raw(&jni, obj)
    }

    pub fn block(&mut self) -> Result<crate::block::Block<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getBlock",
            "()Lorg/bukkit/block/Block;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::block::Block::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

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
impl<'mc> Into<crate::event::block::BlockCookEvent<'mc>> for FurnaceSmeltEvent<'mc> {
    fn into(self) -> crate::event::block::BlockCookEvent<'mc> {
        crate::event::block::BlockCookEvent::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
/// Called when the recipe of an Item is completed inside a smithing table.
pub struct SmithItemEvent<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for SmithItemEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> SmithItemEvent<'mc> {
    pub fn from_raw(
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
    pub fn new_with_inventory_view(
        jni: blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<&'mc crate::inventory::InventoryView<'mc>>,
        arg1: impl Into<&'mc crate::event::inventory::InventoryTypeSlotType<'mc>>,
        arg2: i32,
        arg3: impl Into<&'mc crate::event::inventory::ClickType<'mc>>,
        arg4: std::option::Option<impl Into<&'mc crate::event::inventory::InventoryAction<'mc>>>,
        arg5: std::option::Option<i32>,
    ) -> Result<crate::event::inventory::SmithItemEvent<'mc>, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let val_2 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone()) };
        let val_3 = jni::objects::JValueGen::Int(arg2.into());
        let val_4 = unsafe { jni::objects::JObject::from_raw(arg3.into().jni_object().clone()) };
        let val_5 =
            unsafe { jni::objects::JObject::from_raw(arg4.unwrap().into().jni_object().clone()) };
        let val_6 = jni::objects::JValueGen::Int(arg5.unwrap().into());
        let cls = &jni.find_class("org/bukkit/event/inventory/SmithItemEvent")?;
        let res = jni.new_object(cls,
"(Lorg/bukkit/inventory/InventoryView;Lorg/bukkit/event/inventory/InventoryType$SlotType;ILorg/bukkit/event/inventory/ClickType;Lorg/bukkit/event/inventory/InventoryAction;I)V",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2),jni::objects::JValueGen::from(&val_3),jni::objects::JValueGen::from(&val_4),jni::objects::JValueGen::from(&val_5),jni::objects::JValueGen::from(&val_6)])?;
        crate::event::inventory::SmithItemEvent::from_raw(&jni, res)
    }
    /// <span class="descfrm-type-label">Description copied from class:&nbsp;<code><a href="InventoryEvent.html#getInventory()">InventoryEvent</a></code></span>
    /// Gets the primary Inventory involved in this transaction
    pub fn inventory(
        &mut self,
    ) -> Result<crate::inventory::SmithingInventory<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getInventory",
            "()Lorg/bukkit/inventory/SmithingInventory;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::SmithingInventory::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
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
    #[deprecated]

    pub fn set_cursor(
        &mut self,
        arg0: impl Into<&'mc crate::inventory::ItemStack<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setCursor",
            "(Lorg/bukkit/inventory/ItemStack;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn cursor(
        &mut self,
    ) -> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getCursor",
            "()Lorg/bukkit/inventory/ItemStack;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::ItemStack::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn slot_type(
        &mut self,
    ) -> Result<crate::event::inventory::InventoryTypeSlotType<'mc>, Box<dyn std::error::Error>>
    {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getSlotType",
            "()Lorg/bukkit/event/inventory/InventoryType$SlotType;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::event::inventory::InventoryTypeSlotType::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn handler_list(
        jni: blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>> {
        let cls = &jni.find_class("org/bukkit/event/HandlerList")?;
        let res = jni.call_static_method(
            cls,
            "getHandlerList",
            "()Lorg/bukkit/event/HandlerList;",
            &[],
        )?;
        let obj = res.l()?;
        crate::event::HandlerList::from_raw(&jni, obj)
    }

    pub fn action(
        &mut self,
    ) -> Result<crate::event::inventory::InventoryAction<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getAction",
            "()Lorg/bukkit/event/inventory/InventoryAction;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant = self
            .0
            .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
        let variant_str = self
            .0
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::event::inventory::InventoryAction::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::event::inventory::InventoryAction::from_string(variant_str).unwrap(),
        )
    }

    pub fn is_shift_click(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isShiftClick", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn current_item(
        &mut self,
    ) -> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getCurrentItem",
            "()Lorg/bukkit/inventory/ItemStack;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::ItemStack::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn is_right_click(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isRightClick", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn is_left_click(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isLeftClick", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn set_current_item(
        &mut self,
        arg0: impl Into<&'mc crate::inventory::ItemStack<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setCurrentItem",
            "(Lorg/bukkit/inventory/ItemStack;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn clicked_inventory(
        &mut self,
    ) -> Result<crate::inventory::Inventory<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getClickedInventory",
            "()Lorg/bukkit/inventory/Inventory;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::Inventory::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn raw_slot(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getRawSlot", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }

    pub fn hotbar_button(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getHotbarButton", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }

    pub fn click(
        &mut self,
    ) -> Result<crate::event::inventory::ClickType<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getClick",
            "()Lorg/bukkit/event/inventory/ClickType;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant = self
            .0
            .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
        let variant_str = self
            .0
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::event::inventory::ClickType::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::event::inventory::ClickType::from_string(variant_str).unwrap(),
        )
    }

    pub fn slot(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getSlot", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }

    pub fn result(&mut self) -> Result<crate::event::EventResult<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getResult",
            "()Lorg/bukkit/event/Event$Result;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::event::EventResult::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn set_result(
        &mut self,
        arg0: impl Into<&'mc crate::event::EventResult<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setResult",
            "(Lorg/bukkit/event/Event$Result;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn is_cancelled(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isCancelled", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

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

    pub fn who_clicked(
        &mut self,
    ) -> Result<crate::entity::HumanEntity<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getWhoClicked",
            "()Lorg/bukkit/entity/HumanEntity;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::entity::HumanEntity::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn viewers(
        &mut self,
    ) -> Result<Vec<crate::entity::HumanEntity<'mc>>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getViewers", "()Ljava/util/List;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let mut list = blackboxmc_java::JavaList::from_raw(&self.0, res.l()?)?;
        let size = list.size()?;
        for i in 0..=size {
            let obj = list.get(i)?;
            new_vec.push(crate::entity::HumanEntity::from_raw(&self.0, obj)?);
        }
        Ok(new_vec)
    }

    pub fn view(
        &mut self,
    ) -> Result<crate::inventory::InventoryView<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getView",
            "()Lorg/bukkit/inventory/InventoryView;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::InventoryView::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

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
impl<'mc> Into<crate::event::inventory::InventoryClickEvent<'mc>> for SmithItemEvent<'mc> {
    fn into(self) -> crate::event::inventory::InventoryClickEvent<'mc> {
        crate::event::inventory::InventoryClickEvent::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
pub enum ClickTypeEnum {
    Left,
    ShiftLeft,
    Right,
    ShiftRight,
    WindowBorderLeft,
    WindowBorderRight,
    Middle,
    NumberKey,
    DoubleClick,
    Drop,
    ControlDrop,
    Creative,
    SwapOffhand,
    Unknown,
}
impl std::fmt::Display for ClickTypeEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            ClickTypeEnum::Left => f.write_str("LEFT"),
            ClickTypeEnum::ShiftLeft => f.write_str("SHIFT_LEFT"),
            ClickTypeEnum::Right => f.write_str("RIGHT"),
            ClickTypeEnum::ShiftRight => f.write_str("SHIFT_RIGHT"),
            ClickTypeEnum::WindowBorderLeft => f.write_str("WINDOW_BORDER_LEFT"),
            ClickTypeEnum::WindowBorderRight => f.write_str("WINDOW_BORDER_RIGHT"),
            ClickTypeEnum::Middle => f.write_str("MIDDLE"),
            ClickTypeEnum::NumberKey => f.write_str("NUMBER_KEY"),
            ClickTypeEnum::DoubleClick => f.write_str("DOUBLE_CLICK"),
            ClickTypeEnum::Drop => f.write_str("DROP"),
            ClickTypeEnum::ControlDrop => f.write_str("CONTROL_DROP"),
            ClickTypeEnum::Creative => f.write_str("CREATIVE"),
            ClickTypeEnum::SwapOffhand => f.write_str("SWAP_OFFHAND"),
            ClickTypeEnum::Unknown => f.write_str("UNKNOWN"),
        }
    }
}
pub struct ClickType<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
    pub ClickTypeEnum,
);
impl<'mc> std::ops::Deref for ClickType<'mc> {
    type Target = ClickTypeEnum;
    fn deref(&self) -> &Self::Target {
        return &self.2;
    }
}
impl<'mc> JNIRaw<'mc> for ClickType<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> ClickType<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
        e: ClickTypeEnum,
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
            Ok(Self(env.clone(), obj, e))
        }
    }
    pub const LEFT: ClickTypeEnum = ClickTypeEnum::Left;
    pub const SHIFT_LEFT: ClickTypeEnum = ClickTypeEnum::ShiftLeft;
    pub const RIGHT: ClickTypeEnum = ClickTypeEnum::Right;
    pub const SHIFT_RIGHT: ClickTypeEnum = ClickTypeEnum::ShiftRight;
    pub const WINDOW_BORDER_LEFT: ClickTypeEnum = ClickTypeEnum::WindowBorderLeft;
    pub const WINDOW_BORDER_RIGHT: ClickTypeEnum = ClickTypeEnum::WindowBorderRight;
    pub const MIDDLE: ClickTypeEnum = ClickTypeEnum::Middle;
    pub const NUMBER_KEY: ClickTypeEnum = ClickTypeEnum::NumberKey;
    pub const DOUBLE_CLICK: ClickTypeEnum = ClickTypeEnum::DoubleClick;
    pub const DROP: ClickTypeEnum = ClickTypeEnum::Drop;
    pub const CONTROL_DROP: ClickTypeEnum = ClickTypeEnum::ControlDrop;
    pub const CREATIVE: ClickTypeEnum = ClickTypeEnum::Creative;
    pub const SWAP_OFFHAND: ClickTypeEnum = ClickTypeEnum::SwapOffhand;
    pub const UNKNOWN: ClickTypeEnum = ClickTypeEnum::Unknown;
    pub fn from_string(str: String) -> std::option::Option<ClickTypeEnum> {
        match str.as_str() {
            "LEFT" => Some(ClickTypeEnum::Left),
            "SHIFT_LEFT" => Some(ClickTypeEnum::ShiftLeft),
            "RIGHT" => Some(ClickTypeEnum::Right),
            "SHIFT_RIGHT" => Some(ClickTypeEnum::ShiftRight),
            "WINDOW_BORDER_LEFT" => Some(ClickTypeEnum::WindowBorderLeft),
            "WINDOW_BORDER_RIGHT" => Some(ClickTypeEnum::WindowBorderRight),
            "MIDDLE" => Some(ClickTypeEnum::Middle),
            "NUMBER_KEY" => Some(ClickTypeEnum::NumberKey),
            "DOUBLE_CLICK" => Some(ClickTypeEnum::DoubleClick),
            "DROP" => Some(ClickTypeEnum::Drop),
            "CONTROL_DROP" => Some(ClickTypeEnum::ControlDrop),
            "CREATIVE" => Some(ClickTypeEnum::Creative),
            "SWAP_OFFHAND" => Some(ClickTypeEnum::SwapOffhand),
            "UNKNOWN" => Some(ClickTypeEnum::Unknown),
            _ => None,
        }
    }
}
/// Called when an ItemStack is about to increase the fuel level of a brewing stand.
pub struct BrewingStandFuelEvent<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for BrewingStandFuelEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> BrewingStandFuelEvent<'mc> {
    pub fn from_raw(
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
    pub fn new(
        jni: blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<&'mc crate::block::Block<'mc>>,
        arg1: impl Into<&'mc crate::inventory::ItemStack<'mc>>,
        arg2: i32,
    ) -> Result<crate::event::inventory::BrewingStandFuelEvent<'mc>, Box<dyn std::error::Error>>
    {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let val_2 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone()) };
        let val_3 = jni::objects::JValueGen::Int(arg2.into());
        let cls = &jni.find_class("org/bukkit/event/inventory/BrewingStandFuelEvent")?;
        let res = jni.new_object(
            cls,
            "(Lorg/bukkit/block/Block;Lorg/bukkit/inventory/ItemStack;I)V",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
                jni::objects::JValueGen::from(&val_3),
            ],
        )?;
        crate::event::inventory::BrewingStandFuelEvent::from_raw(&jni, res)
    }
    /// <span class="descfrm-type-label">Description copied from interface:&nbsp;<code><a href="../Cancellable.html#isCancelled()">Cancellable</a></code></span>
    /// Gets the cancellation state of this event. A cancelled event will not be executed in the server, but will still pass to other plugins
    pub fn is_cancelled(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isCancelled", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
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
    /// Gets the ItemStack of the fuel before the amount was subtracted.
    /// Gets the fuel power for this fuel. Each unit of power can fuel one brewing operation.
    pub fn fuel(&mut self) -> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getFuel",
            "()Lorg/bukkit/inventory/ItemStack;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::ItemStack::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// <span class="descfrm-type-label">Description copied from interface:&nbsp;<code><a href="../Cancellable.html#setCancelled(boolean)">Cancellable</a></code></span>
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

    pub fn handler_list(
        jni: blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>> {
        let cls = &jni.find_class("org/bukkit/event/HandlerList")?;
        let res = jni.call_static_method(
            cls,
            "getHandlerList",
            "()Lorg/bukkit/event/HandlerList;",
            &[],
        )?;
        let obj = res.l()?;
        crate::event::HandlerList::from_raw(&jni, obj)
    }
    /// Gets the fuel power for this fuel. Each unit of power can fuel one brewing operation.
    pub fn fuel_power(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getFuelPower", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }
    /// Sets the fuel power for this fuel. Each unit of power can fuel one brewing operation.
    pub fn set_fuel_power(&mut self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setFuelPower",
            "(I)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Gets whether the brewing stand's fuel will be reduced / consumed or not.
    pub fn is_consuming(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isConsuming", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    /// Sets whether the brewing stand's fuel will be reduced / consumed or not.
    pub fn set_consuming(&mut self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        // -2
        let val_1 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setConsuming",
            "(Z)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn block(&mut self) -> Result<crate::block::Block<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getBlock",
            "()Lorg/bukkit/block/Block;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::block::Block::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

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
impl<'mc> Into<crate::event::Cancellable<'mc>> for BrewingStandFuelEvent<'mc> {
    fn into(self) -> crate::event::Cancellable<'mc> {
        crate::event::Cancellable::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
impl<'mc> Into<crate::event::block::BlockEvent<'mc>> for BrewingStandFuelEvent<'mc> {
    fn into(self) -> crate::event::block::BlockEvent<'mc> {
        crate::event::block::BlockEvent::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
/// Called when an item is put in a slot for repair or unenchanting in a grindstone.
pub struct PrepareGrindstoneEvent<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for PrepareGrindstoneEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> PrepareGrindstoneEvent<'mc> {
    pub fn from_raw(
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
    pub fn new(
        jni: blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<&'mc crate::inventory::InventoryView<'mc>>,
        arg1: impl Into<&'mc crate::inventory::ItemStack<'mc>>,
    ) -> Result<crate::event::inventory::PrepareGrindstoneEvent<'mc>, Box<dyn std::error::Error>>
    {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let val_2 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone()) };
        let cls = &jni.find_class("org/bukkit/event/inventory/PrepareGrindstoneEvent")?;
        let res = jni.new_object(
            cls,
            "(Lorg/bukkit/inventory/InventoryView;Lorg/bukkit/inventory/ItemStack;)V",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        )?;
        crate::event::inventory::PrepareGrindstoneEvent::from_raw(&jni, res)
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
    /// <span class="descfrm-type-label">Description copied from class:&nbsp;<code><a href="InventoryEvent.html#getInventory()">InventoryEvent</a></code></span>
    /// Gets the primary Inventory involved in this transaction
    pub fn inventory(
        &mut self,
    ) -> Result<crate::inventory::GrindstoneInventory<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getInventory",
            "()Lorg/bukkit/inventory/GrindstoneInventory;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::GrindstoneInventory::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn handler_list(
        jni: blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>> {
        let cls = &jni.find_class("org/bukkit/event/HandlerList")?;
        let res = jni.call_static_method(
            cls,
            "getHandlerList",
            "()Lorg/bukkit/event/HandlerList;",
            &[],
        )?;
        let obj = res.l()?;
        crate::event::HandlerList::from_raw(&jni, obj)
    }

    pub fn result(
        &mut self,
    ) -> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getResult",
            "()Lorg/bukkit/inventory/ItemStack;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::ItemStack::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn set_result(
        &mut self,
        arg0: impl Into<&'mc crate::inventory::ItemStack<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setResult",
            "(Lorg/bukkit/inventory/ItemStack;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn viewers(
        &mut self,
    ) -> Result<Vec<crate::entity::HumanEntity<'mc>>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getViewers", "()Ljava/util/List;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let mut list = blackboxmc_java::JavaList::from_raw(&self.0, res.l()?)?;
        let size = list.size()?;
        for i in 0..=size {
            let obj = list.get(i)?;
            new_vec.push(crate::entity::HumanEntity::from_raw(&self.0, obj)?);
        }
        Ok(new_vec)
    }

    pub fn view(
        &mut self,
    ) -> Result<crate::inventory::InventoryView<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getView",
            "()Lorg/bukkit/inventory/InventoryView;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::InventoryView::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

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
impl<'mc> Into<crate::event::inventory::PrepareInventoryResultEvent<'mc>>
    for PrepareGrindstoneEvent<'mc>
{
    fn into(self) -> crate::event::inventory::PrepareInventoryResultEvent<'mc> {
        crate::event::inventory::PrepareInventoryResultEvent::from_raw(&self.jni_ref(), self.1)
            .unwrap()
    }
}
/// Event that gets called each time a Hopper attempts to find its source/attached containers.
pub struct HopperInventorySearchEvent<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
pub struct HopperInventorySearchEventContainerType<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for HopperInventorySearchEventContainerType<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> HopperInventorySearchEventContainerType<'mc> {
    pub fn from_raw(
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
            "org/bukkit/event/inventory/HopperInventorySearchEventContainerType",
        )?;
        if !valid {
            Err(eyre::eyre!(
        "Invalid argument passed. Expected a HopperInventorySearchEventContainerType object, got {}",
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
impl<'mc> blackboxmc_general::JNIRaw<'mc> for HopperInventorySearchEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> HopperInventorySearchEvent<'mc> {
    pub fn from_raw(
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
    pub fn new(
        jni: blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<&'mc crate::inventory::Inventory<'mc>>,
        arg1: impl Into<&'mc crate::event::inventory::HopperInventorySearchEventContainerType<'mc>>,
        arg2: impl Into<&'mc crate::block::Block<'mc>>,
        arg3: impl Into<&'mc crate::block::Block<'mc>>,
    ) -> Result<crate::event::inventory::HopperInventorySearchEvent<'mc>, Box<dyn std::error::Error>>
    {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let val_2 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone()) };
        let val_3 = unsafe { jni::objects::JObject::from_raw(arg2.into().jni_object().clone()) };
        let val_4 = unsafe { jni::objects::JObject::from_raw(arg3.into().jni_object().clone()) };
        let cls = &jni.find_class("org/bukkit/event/inventory/HopperInventorySearchEvent")?;
        let res = jni.new_object(cls,
"(Lorg/bukkit/inventory/Inventory;Lorg/bukkit/event/inventory/HopperInventorySearchEvent$ContainerType;Lorg/bukkit/block/Block;Lorg/bukkit/block/Block;)V",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2),jni::objects::JValueGen::from(&val_3),jni::objects::JValueGen::from(&val_4)])?;
        crate::event::inventory::HopperInventorySearchEvent::from_raw(&jni, res)
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
    /// Gets the <a title="interface in org.bukkit.inventory" href="../../inventory/Inventory.html"><code>Inventory</code></a> that the Hopper will use for its source/attached Container.
    pub fn inventory(
        &mut self,
    ) -> Result<crate::inventory::Inventory<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getInventory",
            "()Lorg/bukkit/inventory/Inventory;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::Inventory::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn handler_list(
        jni: blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>> {
        let cls = &jni.find_class("org/bukkit/event/HandlerList")?;
        let res = jni.call_static_method(
            cls,
            "getHandlerList",
            "()Lorg/bukkit/event/HandlerList;",
            &[],
        )?;
        let obj = res.l()?;
        crate::event::HandlerList::from_raw(&jni, obj)
    }
    /// Set the <a href="../../inventory/Inventory.html" title="interface in org.bukkit.inventory"><code>Inventory</code></a> that the Hopper will use for its source/attached Container.
    pub fn set_inventory(
        &mut self,
        arg0: impl Into<&'mc crate::inventory::Inventory<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setInventory",
            "(Lorg/bukkit/inventory/Inventory;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Gets the Container type the Hopper is searching for.
    pub fn container_type(
        &mut self,
    ) -> Result<
        crate::event::inventory::HopperInventorySearchEventContainerType<'mc>,
        Box<dyn std::error::Error>,
    > {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getContainerType",
            "()Lorg/bukkit/event/inventory/HopperInventorySearchEvent$ContainerType;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::event::inventory::HopperInventorySearchEventContainerType::from_raw(
            &self.jni_ref(),
            unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) },
        )
    }
    /// Gets the Block that is being searched for an inventory.
    pub fn search_block(&mut self) -> Result<crate::block::Block<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getSearchBlock",
            "()Lorg/bukkit/block/Block;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::block::Block::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn block(&mut self) -> Result<crate::block::Block<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getBlock",
            "()Lorg/bukkit/block/Block;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::block::Block::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

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
impl<'mc> Into<crate::event::block::BlockEvent<'mc>> for HopperInventorySearchEvent<'mc> {
    fn into(self) -> crate::event::block::BlockEvent<'mc> {
        crate::event::block::BlockEvent::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
/// Called when the recipe of an Item is completed inside a crafting matrix.
pub struct CraftItemEvent<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for CraftItemEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> CraftItemEvent<'mc> {
    pub fn from_raw(
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
    pub fn new_with_recipe(
        jni: blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<&'mc crate::inventory::Recipe<'mc>>,
        arg1: impl Into<&'mc crate::inventory::InventoryView<'mc>>,
        arg2: impl Into<&'mc crate::event::inventory::InventoryTypeSlotType<'mc>>,
        arg3: i32,
        arg4: impl Into<&'mc crate::event::inventory::ClickType<'mc>>,
        arg5: std::option::Option<impl Into<&'mc crate::event::inventory::InventoryAction<'mc>>>,
        arg6: std::option::Option<i32>,
    ) -> Result<crate::event::inventory::CraftItemEvent<'mc>, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let val_2 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone()) };
        let val_3 = unsafe { jni::objects::JObject::from_raw(arg2.into().jni_object().clone()) };
        let val_4 = jni::objects::JValueGen::Int(arg3.into());
        let val_5 = unsafe { jni::objects::JObject::from_raw(arg4.into().jni_object().clone()) };
        let val_6 =
            unsafe { jni::objects::JObject::from_raw(arg5.unwrap().into().jni_object().clone()) };
        let val_7 = jni::objects::JValueGen::Int(arg6.unwrap().into());
        let cls = &jni.find_class("org/bukkit/event/inventory/CraftItemEvent")?;
        let res = jni.new_object(cls,
"(Lorg/bukkit/inventory/Recipe;Lorg/bukkit/inventory/InventoryView;Lorg/bukkit/event/inventory/InventoryType$SlotType;ILorg/bukkit/event/inventory/ClickType;Lorg/bukkit/event/inventory/InventoryAction;I)V",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2),jni::objects::JValueGen::from(&val_3),jni::objects::JValueGen::from(&val_4),jni::objects::JValueGen::from(&val_5),jni::objects::JValueGen::from(&val_6),jni::objects::JValueGen::from(&val_7)])?;
        crate::event::inventory::CraftItemEvent::from_raw(&jni, res)
    }
    /// <span class="descfrm-type-label">Description copied from class:&nbsp;<code><a href="InventoryEvent.html#getInventory()">InventoryEvent</a></code></span>
    /// Gets the primary Inventory involved in this transaction
    pub fn inventory(
        &mut self,
    ) -> Result<crate::inventory::CraftingInventory<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getInventory",
            "()Lorg/bukkit/inventory/CraftingInventory;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::CraftingInventory::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn recipe(&mut self) -> Result<crate::inventory::Recipe<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getRecipe",
            "()Lorg/bukkit/inventory/Recipe;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::Recipe::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
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
    #[deprecated]

    pub fn set_cursor(
        &mut self,
        arg0: impl Into<&'mc crate::inventory::ItemStack<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setCursor",
            "(Lorg/bukkit/inventory/ItemStack;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn cursor(
        &mut self,
    ) -> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getCursor",
            "()Lorg/bukkit/inventory/ItemStack;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::ItemStack::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn slot_type(
        &mut self,
    ) -> Result<crate::event::inventory::InventoryTypeSlotType<'mc>, Box<dyn std::error::Error>>
    {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getSlotType",
            "()Lorg/bukkit/event/inventory/InventoryType$SlotType;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::event::inventory::InventoryTypeSlotType::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn handler_list(
        jni: blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>> {
        let cls = &jni.find_class("org/bukkit/event/HandlerList")?;
        let res = jni.call_static_method(
            cls,
            "getHandlerList",
            "()Lorg/bukkit/event/HandlerList;",
            &[],
        )?;
        let obj = res.l()?;
        crate::event::HandlerList::from_raw(&jni, obj)
    }

    pub fn action(
        &mut self,
    ) -> Result<crate::event::inventory::InventoryAction<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getAction",
            "()Lorg/bukkit/event/inventory/InventoryAction;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant = self
            .0
            .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
        let variant_str = self
            .0
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::event::inventory::InventoryAction::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::event::inventory::InventoryAction::from_string(variant_str).unwrap(),
        )
    }

    pub fn is_shift_click(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isShiftClick", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn current_item(
        &mut self,
    ) -> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getCurrentItem",
            "()Lorg/bukkit/inventory/ItemStack;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::ItemStack::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn is_right_click(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isRightClick", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn is_left_click(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isLeftClick", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn set_current_item(
        &mut self,
        arg0: impl Into<&'mc crate::inventory::ItemStack<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setCurrentItem",
            "(Lorg/bukkit/inventory/ItemStack;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn clicked_inventory(
        &mut self,
    ) -> Result<crate::inventory::Inventory<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getClickedInventory",
            "()Lorg/bukkit/inventory/Inventory;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::Inventory::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn raw_slot(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getRawSlot", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }

    pub fn hotbar_button(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getHotbarButton", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }

    pub fn click(
        &mut self,
    ) -> Result<crate::event::inventory::ClickType<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getClick",
            "()Lorg/bukkit/event/inventory/ClickType;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant = self
            .0
            .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
        let variant_str = self
            .0
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::event::inventory::ClickType::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::event::inventory::ClickType::from_string(variant_str).unwrap(),
        )
    }

    pub fn slot(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getSlot", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }

    pub fn result(&mut self) -> Result<crate::event::EventResult<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getResult",
            "()Lorg/bukkit/event/Event$Result;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::event::EventResult::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn set_result(
        &mut self,
        arg0: impl Into<&'mc crate::event::EventResult<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setResult",
            "(Lorg/bukkit/event/Event$Result;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn is_cancelled(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isCancelled", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

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

    pub fn who_clicked(
        &mut self,
    ) -> Result<crate::entity::HumanEntity<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getWhoClicked",
            "()Lorg/bukkit/entity/HumanEntity;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::entity::HumanEntity::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn viewers(
        &mut self,
    ) -> Result<Vec<crate::entity::HumanEntity<'mc>>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getViewers", "()Ljava/util/List;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let mut list = blackboxmc_java::JavaList::from_raw(&self.0, res.l()?)?;
        let size = list.size()?;
        for i in 0..=size {
            let obj = list.get(i)?;
            new_vec.push(crate::entity::HumanEntity::from_raw(&self.0, obj)?);
        }
        Ok(new_vec)
    }

    pub fn view(
        &mut self,
    ) -> Result<crate::inventory::InventoryView<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getView",
            "()Lorg/bukkit/inventory/InventoryView;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::InventoryView::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

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
impl<'mc> Into<crate::event::inventory::InventoryClickEvent<'mc>> for CraftItemEvent<'mc> {
    fn into(self) -> crate::event::inventory::InventoryClickEvent<'mc> {
        crate::event::inventory::InventoryClickEvent::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
/// Called when a Furnace starts smelting.
pub struct FurnaceStartSmeltEvent<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for FurnaceStartSmeltEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> FurnaceStartSmeltEvent<'mc> {
    pub fn from_raw(
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
    pub fn new(
        jni: blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<&'mc crate::block::Block<'mc>>,
        arg1: impl Into<&'mc crate::inventory::ItemStack<'mc>>,
        arg2: impl Into<&'mc crate::inventory::CookingRecipe<'mc>>,
    ) -> Result<crate::event::inventory::FurnaceStartSmeltEvent<'mc>, Box<dyn std::error::Error>>
    {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let val_2 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone()) };
        let val_3 = unsafe { jni::objects::JObject::from_raw(arg2.into().jni_object().clone()) };
        let cls = &jni.find_class("org/bukkit/event/inventory/FurnaceStartSmeltEvent")?;
        let res = jni.new_object(cls,
"(Lorg/bukkit/block/Block;Lorg/bukkit/inventory/ItemStack;Lorg/bukkit/inventory/CookingRecipe;)V",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2),jni::objects::JValueGen::from(&val_3)])?;
        crate::event::inventory::FurnaceStartSmeltEvent::from_raw(&jni, res)
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
    /// Gets the FurnaceRecipe associated with this event
    pub fn recipe(
        &mut self,
    ) -> Result<crate::inventory::CookingRecipe<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getRecipe",
            "()Lorg/bukkit/inventory/CookingRecipe;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::CookingRecipe::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn handler_list(
        jni: blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>> {
        let cls = &jni.find_class("org/bukkit/event/HandlerList")?;
        let res = jni.call_static_method(
            cls,
            "getHandlerList",
            "()Lorg/bukkit/event/HandlerList;",
            &[],
        )?;
        let obj = res.l()?;
        crate::event::HandlerList::from_raw(&jni, obj)
    }
    /// Gets the total cook time associated with this event
    pub fn total_cook_time(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getTotalCookTime", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }
    /// Sets the total cook time for this event
    pub fn set_total_cook_time(&mut self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setTotalCookTime",
            "(I)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn source(
        &mut self,
    ) -> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getSource",
            "()Lorg/bukkit/inventory/ItemStack;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::ItemStack::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn block(&mut self) -> Result<crate::block::Block<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getBlock",
            "()Lorg/bukkit/block/Block;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::block::Block::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

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
impl<'mc> Into<crate::event::block::InventoryBlockStartEvent<'mc>> for FurnaceStartSmeltEvent<'mc> {
    fn into(self) -> crate::event::block::InventoryBlockStartEvent<'mc> {
        crate::event::block::InventoryBlockStartEvent::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
/// Called when a hopper or hopper minecart picks up a dropped item.
pub struct InventoryPickupItemEvent<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for InventoryPickupItemEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> InventoryPickupItemEvent<'mc> {
    pub fn from_raw(
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
    pub fn new(
        jni: blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<&'mc crate::inventory::Inventory<'mc>>,
        arg1: impl Into<&'mc crate::entity::Item<'mc>>,
    ) -> Result<crate::event::inventory::InventoryPickupItemEvent<'mc>, Box<dyn std::error::Error>>
    {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let val_2 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone()) };
        let cls = &jni.find_class("org/bukkit/event/inventory/InventoryPickupItemEvent")?;
        let res = jni.new_object(
            cls,
            "(Lorg/bukkit/inventory/Inventory;Lorg/bukkit/entity/Item;)V",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        )?;
        crate::event::inventory::InventoryPickupItemEvent::from_raw(&jni, res)
    }
    /// Gets the Item entity that was picked up
    pub fn item(&mut self) -> Result<crate::entity::Item<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getItem",
            "()Lorg/bukkit/entity/Item;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::entity::Item::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// <span class="descfrm-type-label">Description copied from interface:&nbsp;<code><a href="../Cancellable.html#isCancelled()">Cancellable</a></code></span>
    /// Gets the cancellation state of this event. A cancelled event will not be executed in the server, but will still pass to other plugins
    pub fn is_cancelled(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isCancelled", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
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
    /// Gets the Inventory that picked up the item
    pub fn inventory(
        &mut self,
    ) -> Result<crate::inventory::Inventory<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getInventory",
            "()Lorg/bukkit/inventory/Inventory;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::Inventory::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// <span class="descfrm-type-label">Description copied from interface:&nbsp;<code><a href="../Cancellable.html#setCancelled(boolean)">Cancellable</a></code></span>
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

    pub fn handler_list(
        jni: blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>> {
        let cls = &jni.find_class("org/bukkit/event/HandlerList")?;
        let res = jni.call_static_method(
            cls,
            "getHandlerList",
            "()Lorg/bukkit/event/HandlerList;",
            &[],
        )?;
        let obj = res.l()?;
        crate::event::HandlerList::from_raw(&jni, obj)
    }

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
impl<'mc> Into<crate::event::Cancellable<'mc>> for InventoryPickupItemEvent<'mc> {
    fn into(self) -> crate::event::Cancellable<'mc> {
        crate::event::Cancellable::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
impl<'mc> Into<crate::event::Event<'mc>> for InventoryPickupItemEvent<'mc> {
    fn into(self) -> crate::event::Event<'mc> {
        crate::event::Event::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
/// An abstract base class for events that describe an interaction between a HumanEntity and the contents of an Inventory.
pub struct InventoryInteractEvent<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for InventoryInteractEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> InventoryInteractEvent<'mc> {
    pub fn from_raw(
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
    pub fn new(
        jni: blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<&'mc crate::inventory::InventoryView<'mc>>,
    ) -> Result<crate::event::inventory::InventoryInteractEvent<'mc>, Box<dyn std::error::Error>>
    {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let cls = &jni.find_class("org/bukkit/event/inventory/InventoryInteractEvent")?;
        let res = jni.new_object(
            cls,
            "(Lorg/bukkit/inventory/InventoryView;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        )?;
        crate::event::inventory::InventoryInteractEvent::from_raw(&jni, res)
    }
    /// Gets the <a href="../Event.Result.html" title="enum in org.bukkit.event"><code>Event.Result</code></a> of this event. The Result describes the behavior that will be applied to the inventory in relation to this event.
    pub fn result(&mut self) -> Result<crate::event::EventResult<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getResult",
            "()Lorg/bukkit/event/Event$Result;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::event::EventResult::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Sets the result of this event. This will change whether or not this event is considered cancelled.
    pub fn set_result(
        &mut self,
        arg0: impl Into<&'mc crate::event::EventResult<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setResult",
            "(Lorg/bukkit/event/Event$Result;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Gets whether or not this event is cancelled. This is based off of the Result value returned by <a href="#getResult()"><code>getResult()</code></a>. Result.ALLOW and Result.DEFAULT will result in a returned value of false, but Result.DENY will result in a returned value of true.
    /// <p>Gets the cancellation state of this event. A cancelled event will not be executed in the server, but will still pass to other plugins</p>
    pub fn is_cancelled(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isCancelled", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    /// Proxy method to <a href="#setResult(org.bukkit.event.Event.Result)"><code>setResult(org.bukkit.event.Event.Result)</code></a> for the Cancellable interface. <a href="#setResult(org.bukkit.event.Event.Result)"><code>setResult(org.bukkit.event.Event.Result)</code></a> is preferred, as it allows you to specify the Result beyond Result.DENY and Result.ALLOW.
    /// <p>Sets the cancellation state of this event. A cancelled event will not be executed in the server, but will still pass to other plugins.</p>
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
    /// Gets the player who performed the click.
    pub fn who_clicked(
        &mut self,
    ) -> Result<crate::entity::HumanEntity<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getWhoClicked",
            "()Lorg/bukkit/entity/HumanEntity;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::entity::HumanEntity::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
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

    pub fn inventory(
        &mut self,
    ) -> Result<crate::inventory::Inventory<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getInventory",
            "()Lorg/bukkit/inventory/Inventory;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::Inventory::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn viewers(
        &mut self,
    ) -> Result<Vec<crate::entity::HumanEntity<'mc>>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getViewers", "()Ljava/util/List;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let mut list = blackboxmc_java::JavaList::from_raw(&self.0, res.l()?)?;
        let size = list.size()?;
        for i in 0..=size {
            let obj = list.get(i)?;
            new_vec.push(crate::entity::HumanEntity::from_raw(&self.0, obj)?);
        }
        Ok(new_vec)
    }

    pub fn handler_list(
        jni: blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>> {
        let cls = &jni.find_class("org/bukkit/event/HandlerList")?;
        let res = jni.call_static_method(
            cls,
            "getHandlerList",
            "()Lorg/bukkit/event/HandlerList;",
            &[],
        )?;
        let obj = res.l()?;
        crate::event::HandlerList::from_raw(&jni, obj)
    }

    pub fn view(
        &mut self,
    ) -> Result<crate::inventory::InventoryView<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getView",
            "()Lorg/bukkit/inventory/InventoryView;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::InventoryView::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

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
impl<'mc> Into<crate::event::Cancellable<'mc>> for InventoryInteractEvent<'mc> {
    fn into(self) -> crate::event::Cancellable<'mc> {
        crate::event::Cancellable::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
impl<'mc> Into<crate::event::inventory::InventoryEvent<'mc>> for InventoryInteractEvent<'mc> {
    fn into(self) -> crate::event::inventory::InventoryEvent<'mc> {
        crate::event::inventory::InventoryEvent::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
/// Represents a player related inventory event
pub struct InventoryEvent<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for InventoryEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> InventoryEvent<'mc> {
    pub fn from_raw(
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
    pub fn new(
        jni: blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<&'mc crate::inventory::InventoryView<'mc>>,
    ) -> Result<crate::event::inventory::InventoryEvent<'mc>, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let cls = &jni.find_class("org/bukkit/event/inventory/InventoryEvent")?;
        let res = jni.new_object(
            cls,
            "(Lorg/bukkit/inventory/InventoryView;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        )?;
        crate::event::inventory::InventoryEvent::from_raw(&jni, res)
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
    /// Gets the primary Inventory involved in this transaction
    pub fn inventory(
        &mut self,
    ) -> Result<crate::inventory::Inventory<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getInventory",
            "()Lorg/bukkit/inventory/Inventory;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::Inventory::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Gets the list of players viewing the primary (upper) inventory involved in this event
    pub fn viewers(
        &mut self,
    ) -> Result<Vec<crate::entity::HumanEntity<'mc>>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getViewers", "()Ljava/util/List;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let mut list = blackboxmc_java::JavaList::from_raw(&self.0, res.l()?)?;
        let size = list.size()?;
        for i in 0..=size {
            let obj = list.get(i)?;
            new_vec.push(crate::entity::HumanEntity::from_raw(&self.0, obj)?);
        }
        Ok(new_vec)
    }

    pub fn handler_list(
        jni: blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>> {
        let cls = &jni.find_class("org/bukkit/event/HandlerList")?;
        let res = jni.call_static_method(
            cls,
            "getHandlerList",
            "()Lorg/bukkit/event/HandlerList;",
            &[],
        )?;
        let obj = res.l()?;
        crate::event::HandlerList::from_raw(&jni, obj)
    }
    /// Gets the list of players viewing the primary (upper) inventory involved in this event
    /// Gets the view object itself
    pub fn view(
        &mut self,
    ) -> Result<crate::inventory::InventoryView<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getView",
            "()Lorg/bukkit/inventory/InventoryView;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::InventoryView::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

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
impl<'mc> Into<crate::event::Event<'mc>> for InventoryEvent<'mc> {
    fn into(self) -> crate::event::Event<'mc> {
        crate::event::Event::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
/// This event is called when a player in creative mode puts down or picks up an item in their inventory / hotbar and when they drop items from their Inventory while in creative mode.
pub struct InventoryCreativeEvent<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for InventoryCreativeEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> InventoryCreativeEvent<'mc> {
    pub fn from_raw(
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
    pub fn new(
        jni: blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<&'mc crate::inventory::InventoryView<'mc>>,
        arg1: impl Into<&'mc crate::event::inventory::InventoryTypeSlotType<'mc>>,
        arg2: i32,
        arg3: impl Into<&'mc crate::inventory::ItemStack<'mc>>,
    ) -> Result<crate::event::inventory::InventoryCreativeEvent<'mc>, Box<dyn std::error::Error>>
    {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let val_2 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone()) };
        let val_3 = jni::objects::JValueGen::Int(arg2.into());
        let val_4 = unsafe { jni::objects::JObject::from_raw(arg3.into().jni_object().clone()) };
        let cls = &jni.find_class("org/bukkit/event/inventory/InventoryCreativeEvent")?;
        let res = jni.new_object(cls,
"(Lorg/bukkit/inventory/InventoryView;Lorg/bukkit/event/inventory/InventoryType$SlotType;ILorg/bukkit/inventory/ItemStack;)V",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2),jni::objects::JValueGen::from(&val_3),jni::objects::JValueGen::from(&val_4)])?;
        crate::event::inventory::InventoryCreativeEvent::from_raw(&jni, res)
    }
    /// <span class="descfrm-type-label">Description copied from class:&nbsp;<code><a href="InventoryClickEvent.html#setCursor(org.bukkit.inventory.ItemStack)">InventoryClickEvent</a></code></span>
    /// Sets the item on the cursor.
    pub fn set_cursor(
        &mut self,
        arg0: impl Into<&'mc crate::inventory::ItemStack<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setCursor",
            "(Lorg/bukkit/inventory/ItemStack;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// <span class="descfrm-type-label">Description copied from class:&nbsp;<code><a href="InventoryClickEvent.html#getCursor()">InventoryClickEvent</a></code></span>
    /// Gets the current ItemStack on the cursor.
    pub fn cursor(
        &mut self,
    ) -> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getCursor",
            "()Lorg/bukkit/inventory/ItemStack;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::ItemStack::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
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

    pub fn slot_type(
        &mut self,
    ) -> Result<crate::event::inventory::InventoryTypeSlotType<'mc>, Box<dyn std::error::Error>>
    {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getSlotType",
            "()Lorg/bukkit/event/inventory/InventoryType$SlotType;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::event::inventory::InventoryTypeSlotType::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn handler_list(
        jni: blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>> {
        let cls = &jni.find_class("org/bukkit/event/HandlerList")?;
        let res = jni.call_static_method(
            cls,
            "getHandlerList",
            "()Lorg/bukkit/event/HandlerList;",
            &[],
        )?;
        let obj = res.l()?;
        crate::event::HandlerList::from_raw(&jni, obj)
    }

    pub fn action(
        &mut self,
    ) -> Result<crate::event::inventory::InventoryAction<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getAction",
            "()Lorg/bukkit/event/inventory/InventoryAction;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant = self
            .0
            .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
        let variant_str = self
            .0
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::event::inventory::InventoryAction::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::event::inventory::InventoryAction::from_string(variant_str).unwrap(),
        )
    }

    pub fn is_shift_click(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isShiftClick", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn current_item(
        &mut self,
    ) -> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getCurrentItem",
            "()Lorg/bukkit/inventory/ItemStack;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::ItemStack::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn is_right_click(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isRightClick", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn is_left_click(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isLeftClick", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn set_current_item(
        &mut self,
        arg0: impl Into<&'mc crate::inventory::ItemStack<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setCurrentItem",
            "(Lorg/bukkit/inventory/ItemStack;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn clicked_inventory(
        &mut self,
    ) -> Result<crate::inventory::Inventory<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getClickedInventory",
            "()Lorg/bukkit/inventory/Inventory;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::Inventory::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn raw_slot(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getRawSlot", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }

    pub fn hotbar_button(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getHotbarButton", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }

    pub fn click(
        &mut self,
    ) -> Result<crate::event::inventory::ClickType<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getClick",
            "()Lorg/bukkit/event/inventory/ClickType;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant = self
            .0
            .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
        let variant_str = self
            .0
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::event::inventory::ClickType::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::event::inventory::ClickType::from_string(variant_str).unwrap(),
        )
    }

    pub fn slot(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getSlot", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }

    pub fn result(&mut self) -> Result<crate::event::EventResult<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getResult",
            "()Lorg/bukkit/event/Event$Result;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::event::EventResult::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn set_result(
        &mut self,
        arg0: impl Into<&'mc crate::event::EventResult<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setResult",
            "(Lorg/bukkit/event/Event$Result;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn is_cancelled(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isCancelled", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

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

    pub fn who_clicked(
        &mut self,
    ) -> Result<crate::entity::HumanEntity<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getWhoClicked",
            "()Lorg/bukkit/entity/HumanEntity;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::entity::HumanEntity::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn inventory(
        &mut self,
    ) -> Result<crate::inventory::Inventory<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getInventory",
            "()Lorg/bukkit/inventory/Inventory;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::Inventory::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn viewers(
        &mut self,
    ) -> Result<Vec<crate::entity::HumanEntity<'mc>>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getViewers", "()Ljava/util/List;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let mut list = blackboxmc_java::JavaList::from_raw(&self.0, res.l()?)?;
        let size = list.size()?;
        for i in 0..=size {
            let obj = list.get(i)?;
            new_vec.push(crate::entity::HumanEntity::from_raw(&self.0, obj)?);
        }
        Ok(new_vec)
    }

    pub fn view(
        &mut self,
    ) -> Result<crate::inventory::InventoryView<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getView",
            "()Lorg/bukkit/inventory/InventoryView;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::InventoryView::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

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
impl<'mc> Into<crate::event::inventory::InventoryClickEvent<'mc>> for InventoryCreativeEvent<'mc> {
    fn into(self) -> crate::event::inventory::InventoryClickEvent<'mc> {
        crate::event::inventory::InventoryClickEvent::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
/// This event is called when a player takes items out of the furnace
pub struct FurnaceExtractEvent<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for FurnaceExtractEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> FurnaceExtractEvent<'mc> {
    pub fn from_raw(
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
    pub fn new(
        jni: blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<&'mc crate::entity::Player<'mc>>,
        arg1: impl Into<&'mc crate::block::Block<'mc>>,
        arg2: impl Into<&'mc crate::Material<'mc>>,
        arg3: i32,
        arg4: i32,
    ) -> Result<crate::event::inventory::FurnaceExtractEvent<'mc>, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let val_2 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone()) };
        let val_3 = unsafe { jni::objects::JObject::from_raw(arg2.into().jni_object().clone()) };
        let val_4 = jni::objects::JValueGen::Int(arg3.into());
        let val_5 = jni::objects::JValueGen::Int(arg4.into());
        let cls = &jni.find_class("org/bukkit/event/inventory/FurnaceExtractEvent")?;
        let res = jni.new_object(
            cls,
            "(Lorg/bukkit/entity/Player;Lorg/bukkit/block/Block;Lorg/bukkit/Material;II)V",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
                jni::objects::JValueGen::from(&val_3),
                jni::objects::JValueGen::from(&val_4),
                jni::objects::JValueGen::from(&val_5),
            ],
        )?;
        crate::event::inventory::FurnaceExtractEvent::from_raw(&jni, res)
    }
    /// Get the player that triggered the event
    pub fn player(&mut self) -> Result<crate::entity::Player<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getPlayer",
            "()Lorg/bukkit/entity/Player;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::entity::Player::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Get the Material of the item being retrieved
    pub fn item_type(&mut self) -> Result<crate::Material<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getItemType",
            "()Lorg/bukkit/Material;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant = self
            .0
            .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
        let variant_str = self
            .0
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::Material::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::Material::from_string(variant_str).unwrap(),
        )
    }
    /// Get the item count being retrieved
    pub fn item_amount(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getItemAmount", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
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

    pub fn handler_list(
        jni: blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>> {
        let cls = &jni.find_class("org/bukkit/event/HandlerList")?;
        let res = jni.call_static_method(
            cls,
            "getHandlerList",
            "()Lorg/bukkit/event/HandlerList;",
            &[],
        )?;
        let obj = res.l()?;
        crate::event::HandlerList::from_raw(&jni, obj)
    }

    pub fn exp_to_drop(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getExpToDrop", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }

    pub fn set_exp_to_drop(&mut self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setExpToDrop",
            "(I)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn block(&mut self) -> Result<crate::block::Block<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getBlock",
            "()Lorg/bukkit/block/Block;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::block::Block::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

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
impl<'mc> Into<crate::event::block::BlockExpEvent<'mc>> for FurnaceExtractEvent<'mc> {
    fn into(self) -> crate::event::block::BlockExpEvent<'mc> {
        crate::event::block::BlockExpEvent::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
pub enum InventoryTypeEnum {
    Chest,
    Dispenser,
    Dropper,
    Furnace,
    Workbench,
    Crafting,
    Enchanting,
    Brewing,
    Player,
    Creative,
    Merchant,
    EnderChest,
    Anvil,
    Smithing,
    Beacon,
    Hopper,
    ShulkerBox,
    Barrel,
    BlastFurnace,
    Lectern,
    Smoker,
    Loom,
    Cartography,
    Grindstone,
    Stonecutter,
    Composter,
    ChiseledBookshelf,
    Jukebox,
    #[deprecated]
    SmithingNew,
}
impl std::fmt::Display for InventoryTypeEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            InventoryTypeEnum::Chest => f.write_str("CHEST"),
            InventoryTypeEnum::Dispenser => f.write_str("DISPENSER"),
            InventoryTypeEnum::Dropper => f.write_str("DROPPER"),
            InventoryTypeEnum::Furnace => f.write_str("FURNACE"),
            InventoryTypeEnum::Workbench => f.write_str("WORKBENCH"),
            InventoryTypeEnum::Crafting => f.write_str("CRAFTING"),
            InventoryTypeEnum::Enchanting => f.write_str("ENCHANTING"),
            InventoryTypeEnum::Brewing => f.write_str("BREWING"),
            InventoryTypeEnum::Player => f.write_str("PLAYER"),
            InventoryTypeEnum::Creative => f.write_str("CREATIVE"),
            InventoryTypeEnum::Merchant => f.write_str("MERCHANT"),
            InventoryTypeEnum::EnderChest => f.write_str("ENDER_CHEST"),
            InventoryTypeEnum::Anvil => f.write_str("ANVIL"),
            InventoryTypeEnum::Smithing => f.write_str("SMITHING"),
            InventoryTypeEnum::Beacon => f.write_str("BEACON"),
            InventoryTypeEnum::Hopper => f.write_str("HOPPER"),
            InventoryTypeEnum::ShulkerBox => f.write_str("SHULKER_BOX"),
            InventoryTypeEnum::Barrel => f.write_str("BARREL"),
            InventoryTypeEnum::BlastFurnace => f.write_str("BLAST_FURNACE"),
            InventoryTypeEnum::Lectern => f.write_str("LECTERN"),
            InventoryTypeEnum::Smoker => f.write_str("SMOKER"),
            InventoryTypeEnum::Loom => f.write_str("LOOM"),
            InventoryTypeEnum::Cartography => f.write_str("CARTOGRAPHY"),
            InventoryTypeEnum::Grindstone => f.write_str("GRINDSTONE"),
            InventoryTypeEnum::Stonecutter => f.write_str("STONECUTTER"),
            InventoryTypeEnum::Composter => f.write_str("COMPOSTER"),
            InventoryTypeEnum::ChiseledBookshelf => f.write_str("CHISELED_BOOKSHELF"),
            InventoryTypeEnum::Jukebox => f.write_str("JUKEBOX"),
            InventoryTypeEnum::SmithingNew => f.write_str("SMITHING_NEW"),
        }
    }
}
pub struct InventoryType<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
    pub InventoryTypeEnum,
);
impl<'mc> std::ops::Deref for InventoryType<'mc> {
    type Target = InventoryTypeEnum;
    fn deref(&self) -> &Self::Target {
        return &self.2;
    }
}
impl<'mc> JNIRaw<'mc> for InventoryType<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
pub struct InventoryTypeSlotType<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for InventoryTypeSlotType<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> InventoryTypeSlotType<'mc> {
    pub fn from_raw(
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
            env.validate_name(&obj, "org/bukkit/event/inventory/InventoryTypeSlotType")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a InventoryTypeSlotType object, got {}",
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
impl<'mc> InventoryType<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
        e: InventoryTypeEnum,
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
            Ok(Self(env.clone(), obj, e))
        }
    }
    pub const CHEST: InventoryTypeEnum = InventoryTypeEnum::Chest;
    pub const DISPENSER: InventoryTypeEnum = InventoryTypeEnum::Dispenser;
    pub const DROPPER: InventoryTypeEnum = InventoryTypeEnum::Dropper;
    pub const FURNACE: InventoryTypeEnum = InventoryTypeEnum::Furnace;
    pub const WORKBENCH: InventoryTypeEnum = InventoryTypeEnum::Workbench;
    pub const CRAFTING: InventoryTypeEnum = InventoryTypeEnum::Crafting;
    pub const ENCHANTING: InventoryTypeEnum = InventoryTypeEnum::Enchanting;
    pub const BREWING: InventoryTypeEnum = InventoryTypeEnum::Brewing;
    pub const PLAYER: InventoryTypeEnum = InventoryTypeEnum::Player;
    pub const CREATIVE: InventoryTypeEnum = InventoryTypeEnum::Creative;
    pub const MERCHANT: InventoryTypeEnum = InventoryTypeEnum::Merchant;
    pub const ENDER_CHEST: InventoryTypeEnum = InventoryTypeEnum::EnderChest;
    pub const ANVIL: InventoryTypeEnum = InventoryTypeEnum::Anvil;
    pub const SMITHING: InventoryTypeEnum = InventoryTypeEnum::Smithing;
    pub const BEACON: InventoryTypeEnum = InventoryTypeEnum::Beacon;
    pub const HOPPER: InventoryTypeEnum = InventoryTypeEnum::Hopper;
    pub const SHULKER_BOX: InventoryTypeEnum = InventoryTypeEnum::ShulkerBox;
    pub const BARREL: InventoryTypeEnum = InventoryTypeEnum::Barrel;
    pub const BLAST_FURNACE: InventoryTypeEnum = InventoryTypeEnum::BlastFurnace;
    pub const LECTERN: InventoryTypeEnum = InventoryTypeEnum::Lectern;
    pub const SMOKER: InventoryTypeEnum = InventoryTypeEnum::Smoker;
    pub const LOOM: InventoryTypeEnum = InventoryTypeEnum::Loom;
    pub const CARTOGRAPHY: InventoryTypeEnum = InventoryTypeEnum::Cartography;
    pub const GRINDSTONE: InventoryTypeEnum = InventoryTypeEnum::Grindstone;
    pub const STONECUTTER: InventoryTypeEnum = InventoryTypeEnum::Stonecutter;
    pub const COMPOSTER: InventoryTypeEnum = InventoryTypeEnum::Composter;
    pub const CHISELED_BOOKSHELF: InventoryTypeEnum = InventoryTypeEnum::ChiseledBookshelf;
    pub const JUKEBOX: InventoryTypeEnum = InventoryTypeEnum::Jukebox;
    pub const SMITHING_NEW: InventoryTypeEnum = InventoryTypeEnum::SmithingNew;
    pub fn from_string(str: String) -> std::option::Option<InventoryTypeEnum> {
        match str.as_str() {
            "CHEST" => Some(InventoryTypeEnum::Chest),
            "DISPENSER" => Some(InventoryTypeEnum::Dispenser),
            "DROPPER" => Some(InventoryTypeEnum::Dropper),
            "FURNACE" => Some(InventoryTypeEnum::Furnace),
            "WORKBENCH" => Some(InventoryTypeEnum::Workbench),
            "CRAFTING" => Some(InventoryTypeEnum::Crafting),
            "ENCHANTING" => Some(InventoryTypeEnum::Enchanting),
            "BREWING" => Some(InventoryTypeEnum::Brewing),
            "PLAYER" => Some(InventoryTypeEnum::Player),
            "CREATIVE" => Some(InventoryTypeEnum::Creative),
            "MERCHANT" => Some(InventoryTypeEnum::Merchant),
            "ENDER_CHEST" => Some(InventoryTypeEnum::EnderChest),
            "ANVIL" => Some(InventoryTypeEnum::Anvil),
            "SMITHING" => Some(InventoryTypeEnum::Smithing),
            "BEACON" => Some(InventoryTypeEnum::Beacon),
            "HOPPER" => Some(InventoryTypeEnum::Hopper),
            "SHULKER_BOX" => Some(InventoryTypeEnum::ShulkerBox),
            "BARREL" => Some(InventoryTypeEnum::Barrel),
            "BLAST_FURNACE" => Some(InventoryTypeEnum::BlastFurnace),
            "LECTERN" => Some(InventoryTypeEnum::Lectern),
            "SMOKER" => Some(InventoryTypeEnum::Smoker),
            "LOOM" => Some(InventoryTypeEnum::Loom),
            "CARTOGRAPHY" => Some(InventoryTypeEnum::Cartography),
            "GRINDSTONE" => Some(InventoryTypeEnum::Grindstone),
            "STONECUTTER" => Some(InventoryTypeEnum::Stonecutter),
            "COMPOSTER" => Some(InventoryTypeEnum::Composter),
            "CHISELED_BOOKSHELF" => Some(InventoryTypeEnum::ChiseledBookshelf),
            "JUKEBOX" => Some(InventoryTypeEnum::Jukebox),
            "SMITHING_NEW" => Some(InventoryTypeEnum::SmithingNew),
            _ => None,
        }
    }
}
/// Called when an item is put in a slot for upgrade by a Smithing Table.
pub struct PrepareSmithingEvent<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for PrepareSmithingEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> PrepareSmithingEvent<'mc> {
    pub fn from_raw(
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
    pub fn new(
        jni: blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<&'mc crate::inventory::InventoryView<'mc>>,
        arg1: impl Into<&'mc crate::inventory::ItemStack<'mc>>,
    ) -> Result<crate::event::inventory::PrepareSmithingEvent<'mc>, Box<dyn std::error::Error>>
    {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let val_2 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone()) };
        let cls = &jni.find_class("org/bukkit/event/inventory/PrepareSmithingEvent")?;
        let res = jni.new_object(
            cls,
            "(Lorg/bukkit/inventory/InventoryView;Lorg/bukkit/inventory/ItemStack;)V",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        )?;
        crate::event::inventory::PrepareSmithingEvent::from_raw(&jni, res)
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
    /// <span class="descfrm-type-label">Description copied from class:&nbsp;<code><a href="InventoryEvent.html#getInventory()">InventoryEvent</a></code></span>
    /// Gets the primary Inventory involved in this transaction
    pub fn inventory(
        &mut self,
    ) -> Result<crate::inventory::SmithingInventory<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getInventory",
            "()Lorg/bukkit/inventory/SmithingInventory;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::SmithingInventory::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn handler_list(
        jni: blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>> {
        let cls = &jni.find_class("org/bukkit/event/HandlerList")?;
        let res = jni.call_static_method(
            cls,
            "getHandlerList",
            "()Lorg/bukkit/event/HandlerList;",
            &[],
        )?;
        let obj = res.l()?;
        crate::event::HandlerList::from_raw(&jni, obj)
    }

    pub fn result(
        &mut self,
    ) -> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getResult",
            "()Lorg/bukkit/inventory/ItemStack;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::ItemStack::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn set_result(
        &mut self,
        arg0: impl Into<&'mc crate::inventory::ItemStack<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setResult",
            "(Lorg/bukkit/inventory/ItemStack;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn viewers(
        &mut self,
    ) -> Result<Vec<crate::entity::HumanEntity<'mc>>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getViewers", "()Ljava/util/List;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let mut list = blackboxmc_java::JavaList::from_raw(&self.0, res.l()?)?;
        let size = list.size()?;
        for i in 0..=size {
            let obj = list.get(i)?;
            new_vec.push(crate::entity::HumanEntity::from_raw(&self.0, obj)?);
        }
        Ok(new_vec)
    }

    pub fn view(
        &mut self,
    ) -> Result<crate::inventory::InventoryView<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getView",
            "()Lorg/bukkit/inventory/InventoryView;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::InventoryView::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

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
impl<'mc> Into<crate::event::inventory::PrepareInventoryResultEvent<'mc>>
    for PrepareSmithingEvent<'mc>
{
    fn into(self) -> crate::event::inventory::PrepareInventoryResultEvent<'mc> {
        crate::event::inventory::PrepareInventoryResultEvent::from_raw(&self.jni_ref(), self.1)
            .unwrap()
    }
}
/// This event is called whenever a player clicks a new trade on the trades sidebar.
/// <p>This event allows the user to get the index of the trade, letting them get the MerchantRecipe via the Merchant.</p>
pub struct TradeSelectEvent<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for TradeSelectEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> TradeSelectEvent<'mc> {
    pub fn from_raw(
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
    pub fn new(
        jni: blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<&'mc crate::inventory::InventoryView<'mc>>,
        arg1: i32,
    ) -> Result<crate::event::inventory::TradeSelectEvent<'mc>, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let val_2 = jni::objects::JValueGen::Int(arg1.into());
        let cls = &jni.find_class("org/bukkit/event/inventory/TradeSelectEvent")?;
        let res = jni.new_object(
            cls,
            "(Lorg/bukkit/inventory/InventoryView;I)V",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        )?;
        crate::event::inventory::TradeSelectEvent::from_raw(&jni, res)
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
    /// <span class="descfrm-type-label">Description copied from class:&nbsp;<code><a href="InventoryEvent.html#getInventory()">InventoryEvent</a></code></span>
    /// Gets the primary Inventory involved in this transaction
    pub fn inventory(
        &mut self,
    ) -> Result<crate::inventory::Inventory<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getInventory",
            "()Lorg/bukkit/inventory/Inventory;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::Inventory::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Get the Merchant involved.
    pub fn merchant(
        &mut self,
    ) -> Result<crate::inventory::Merchant<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getMerchant",
            "()Lorg/bukkit/inventory/Merchant;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::Merchant::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn handler_list(
        jni: blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>> {
        let cls = &jni.find_class("org/bukkit/event/HandlerList")?;
        let res = jni.call_static_method(
            cls,
            "getHandlerList",
            "()Lorg/bukkit/event/HandlerList;",
            &[],
        )?;
        let obj = res.l()?;
        crate::event::HandlerList::from_raw(&jni, obj)
    }
    /// Used to get the index of the trade the player clicked on.
    pub fn index(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getIndex", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }

    pub fn result(&mut self) -> Result<crate::event::EventResult<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getResult",
            "()Lorg/bukkit/event/Event$Result;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::event::EventResult::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn set_result(
        &mut self,
        arg0: impl Into<&'mc crate::event::EventResult<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setResult",
            "(Lorg/bukkit/event/Event$Result;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn is_cancelled(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isCancelled", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

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

    pub fn who_clicked(
        &mut self,
    ) -> Result<crate::entity::HumanEntity<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getWhoClicked",
            "()Lorg/bukkit/entity/HumanEntity;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::entity::HumanEntity::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn viewers(
        &mut self,
    ) -> Result<Vec<crate::entity::HumanEntity<'mc>>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getViewers", "()Ljava/util/List;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let mut list = blackboxmc_java::JavaList::from_raw(&self.0, res.l()?)?;
        let size = list.size()?;
        for i in 0..=size {
            let obj = list.get(i)?;
            new_vec.push(crate::entity::HumanEntity::from_raw(&self.0, obj)?);
        }
        Ok(new_vec)
    }

    pub fn view(
        &mut self,
    ) -> Result<crate::inventory::InventoryView<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getView",
            "()Lorg/bukkit/inventory/InventoryView;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::InventoryView::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

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
impl<'mc> Into<crate::event::inventory::InventoryInteractEvent<'mc>> for TradeSelectEvent<'mc> {
    fn into(self) -> crate::event::inventory::InventoryInteractEvent<'mc> {
        crate::event::inventory::InventoryInteractEvent::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
pub enum InventoryActionEnum {
    Nothing,
    PickupAll,
    PickupSome,
    PickupHalf,
    PickupOne,
    PlaceAll,
    PlaceSome,
    PlaceOne,
    SwapWithCursor,
    DropAllCursor,
    DropOneCursor,
    DropAllSlot,
    DropOneSlot,
    MoveToOtherInventory,
    HotbarMoveAndReadd,
    HotbarSwap,
    CloneStack,
    CollectToCursor,
    Unknown,
}
impl std::fmt::Display for InventoryActionEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            InventoryActionEnum::Nothing => f.write_str("NOTHING"),
            InventoryActionEnum::PickupAll => f.write_str("PICKUP_ALL"),
            InventoryActionEnum::PickupSome => f.write_str("PICKUP_SOME"),
            InventoryActionEnum::PickupHalf => f.write_str("PICKUP_HALF"),
            InventoryActionEnum::PickupOne => f.write_str("PICKUP_ONE"),
            InventoryActionEnum::PlaceAll => f.write_str("PLACE_ALL"),
            InventoryActionEnum::PlaceSome => f.write_str("PLACE_SOME"),
            InventoryActionEnum::PlaceOne => f.write_str("PLACE_ONE"),
            InventoryActionEnum::SwapWithCursor => f.write_str("SWAP_WITH_CURSOR"),
            InventoryActionEnum::DropAllCursor => f.write_str("DROP_ALL_CURSOR"),
            InventoryActionEnum::DropOneCursor => f.write_str("DROP_ONE_CURSOR"),
            InventoryActionEnum::DropAllSlot => f.write_str("DROP_ALL_SLOT"),
            InventoryActionEnum::DropOneSlot => f.write_str("DROP_ONE_SLOT"),
            InventoryActionEnum::MoveToOtherInventory => f.write_str("MOVE_TO_OTHER_INVENTORY"),
            InventoryActionEnum::HotbarMoveAndReadd => f.write_str("HOTBAR_MOVE_AND_READD"),
            InventoryActionEnum::HotbarSwap => f.write_str("HOTBAR_SWAP"),
            InventoryActionEnum::CloneStack => f.write_str("CLONE_STACK"),
            InventoryActionEnum::CollectToCursor => f.write_str("COLLECT_TO_CURSOR"),
            InventoryActionEnum::Unknown => f.write_str("UNKNOWN"),
        }
    }
}
pub struct InventoryAction<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
    pub InventoryActionEnum,
);
impl<'mc> std::ops::Deref for InventoryAction<'mc> {
    type Target = InventoryActionEnum;
    fn deref(&self) -> &Self::Target {
        return &self.2;
    }
}
impl<'mc> JNIRaw<'mc> for InventoryAction<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> InventoryAction<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
        e: InventoryActionEnum,
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
            Ok(Self(env.clone(), obj, e))
        }
    }
    pub const NOTHING: InventoryActionEnum = InventoryActionEnum::Nothing;
    pub const PICKUP_ALL: InventoryActionEnum = InventoryActionEnum::PickupAll;
    pub const PICKUP_SOME: InventoryActionEnum = InventoryActionEnum::PickupSome;
    pub const PICKUP_HALF: InventoryActionEnum = InventoryActionEnum::PickupHalf;
    pub const PICKUP_ONE: InventoryActionEnum = InventoryActionEnum::PickupOne;
    pub const PLACE_ALL: InventoryActionEnum = InventoryActionEnum::PlaceAll;
    pub const PLACE_SOME: InventoryActionEnum = InventoryActionEnum::PlaceSome;
    pub const PLACE_ONE: InventoryActionEnum = InventoryActionEnum::PlaceOne;
    pub const SWAP_WITH_CURSOR: InventoryActionEnum = InventoryActionEnum::SwapWithCursor;
    pub const DROP_ALL_CURSOR: InventoryActionEnum = InventoryActionEnum::DropAllCursor;
    pub const DROP_ONE_CURSOR: InventoryActionEnum = InventoryActionEnum::DropOneCursor;
    pub const DROP_ALL_SLOT: InventoryActionEnum = InventoryActionEnum::DropAllSlot;
    pub const DROP_ONE_SLOT: InventoryActionEnum = InventoryActionEnum::DropOneSlot;
    pub const MOVE_TO_OTHER_INVENTORY: InventoryActionEnum =
        InventoryActionEnum::MoveToOtherInventory;
    pub const HOTBAR_MOVE_AND_READD: InventoryActionEnum = InventoryActionEnum::HotbarMoveAndReadd;
    pub const HOTBAR_SWAP: InventoryActionEnum = InventoryActionEnum::HotbarSwap;
    pub const CLONE_STACK: InventoryActionEnum = InventoryActionEnum::CloneStack;
    pub const COLLECT_TO_CURSOR: InventoryActionEnum = InventoryActionEnum::CollectToCursor;
    pub const UNKNOWN: InventoryActionEnum = InventoryActionEnum::Unknown;
    pub fn from_string(str: String) -> std::option::Option<InventoryActionEnum> {
        match str.as_str() {
            "NOTHING" => Some(InventoryActionEnum::Nothing),
            "PICKUP_ALL" => Some(InventoryActionEnum::PickupAll),
            "PICKUP_SOME" => Some(InventoryActionEnum::PickupSome),
            "PICKUP_HALF" => Some(InventoryActionEnum::PickupHalf),
            "PICKUP_ONE" => Some(InventoryActionEnum::PickupOne),
            "PLACE_ALL" => Some(InventoryActionEnum::PlaceAll),
            "PLACE_SOME" => Some(InventoryActionEnum::PlaceSome),
            "PLACE_ONE" => Some(InventoryActionEnum::PlaceOne),
            "SWAP_WITH_CURSOR" => Some(InventoryActionEnum::SwapWithCursor),
            "DROP_ALL_CURSOR" => Some(InventoryActionEnum::DropAllCursor),
            "DROP_ONE_CURSOR" => Some(InventoryActionEnum::DropOneCursor),
            "DROP_ALL_SLOT" => Some(InventoryActionEnum::DropAllSlot),
            "DROP_ONE_SLOT" => Some(InventoryActionEnum::DropOneSlot),
            "MOVE_TO_OTHER_INVENTORY" => Some(InventoryActionEnum::MoveToOtherInventory),
            "HOTBAR_MOVE_AND_READD" => Some(InventoryActionEnum::HotbarMoveAndReadd),
            "HOTBAR_SWAP" => Some(InventoryActionEnum::HotbarSwap),
            "CLONE_STACK" => Some(InventoryActionEnum::CloneStack),
            "COLLECT_TO_CURSOR" => Some(InventoryActionEnum::CollectToCursor),
            "UNKNOWN" => Some(InventoryActionEnum::Unknown),
            _ => None,
        }
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
pub struct InventoryClickEvent<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for InventoryClickEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> InventoryClickEvent<'mc> {
    pub fn from_raw(
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
    pub fn new_with_inventory_view(
        jni: blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<&'mc crate::inventory::InventoryView<'mc>>,
        arg1: impl Into<&'mc crate::event::inventory::InventoryTypeSlotType<'mc>>,
        arg2: i32,
        arg3: impl Into<&'mc crate::event::inventory::ClickType<'mc>>,
        arg4: std::option::Option<impl Into<&'mc crate::event::inventory::InventoryAction<'mc>>>,
        arg5: std::option::Option<i32>,
    ) -> Result<crate::event::inventory::InventoryClickEvent<'mc>, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let val_2 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone()) };
        let val_3 = jni::objects::JValueGen::Int(arg2.into());
        let val_4 = unsafe { jni::objects::JObject::from_raw(arg3.into().jni_object().clone()) };
        let val_5 =
            unsafe { jni::objects::JObject::from_raw(arg4.unwrap().into().jni_object().clone()) };
        let val_6 = jni::objects::JValueGen::Int(arg5.unwrap().into());
        let cls = &jni.find_class("org/bukkit/event/inventory/InventoryClickEvent")?;
        let res = jni.new_object(cls,
"(Lorg/bukkit/inventory/InventoryView;Lorg/bukkit/event/inventory/InventoryType$SlotType;ILorg/bukkit/event/inventory/ClickType;Lorg/bukkit/event/inventory/InventoryAction;I)V",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2),jni::objects::JValueGen::from(&val_3),jni::objects::JValueGen::from(&val_4),jni::objects::JValueGen::from(&val_5),jni::objects::JValueGen::from(&val_6)])?;
        crate::event::inventory::InventoryClickEvent::from_raw(&jni, res)
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
    #[deprecated]
    /// <span class="deprecated-label">Deprecated.</span>
    /// <div class="deprecation-comment">
    /// This changes the ItemStack in their hand before any calculations are applied to the Inventory, which has a tendency to create inconsistencies between the Player and the server, and to make unexpected changes in the behavior of the clicked Inventory.
    /// </div>
    /// This changes the ItemStack in their hand before any calculations are applied to the Inventory, which has a tendency to create inconsistencies between the Player and the server, and to make unexpected changes in the behavior of the clicked Inventory.
    ///
    /// Sets the item on the cursor.
    pub fn set_cursor(
        &mut self,
        arg0: impl Into<&'mc crate::inventory::ItemStack<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setCursor",
            "(Lorg/bukkit/inventory/ItemStack;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Gets the current ItemStack on the cursor.
    pub fn cursor(
        &mut self,
    ) -> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getCursor",
            "()Lorg/bukkit/inventory/ItemStack;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::ItemStack::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Gets the type of slot that was clicked.
    pub fn slot_type(
        &mut self,
    ) -> Result<crate::event::inventory::InventoryTypeSlotType<'mc>, Box<dyn std::error::Error>>
    {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getSlotType",
            "()Lorg/bukkit/event/inventory/InventoryType$SlotType;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::event::inventory::InventoryTypeSlotType::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn handler_list(
        jni: blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>> {
        let cls = &jni.find_class("org/bukkit/event/HandlerList")?;
        let res = jni.call_static_method(
            cls,
            "getHandlerList",
            "()Lorg/bukkit/event/HandlerList;",
            &[],
        )?;
        let obj = res.l()?;
        crate::event::HandlerList::from_raw(&jni, obj)
    }
    /// Gets the InventoryAction that triggered this event.
    /// <p>This action cannot be changed, and represents what the normal outcome of the event will be. To change the behavior of this InventoryClickEvent, changes must be manually applied.</p>
    pub fn action(
        &mut self,
    ) -> Result<crate::event::inventory::InventoryAction<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getAction",
            "()Lorg/bukkit/event/inventory/InventoryAction;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant = self
            .0
            .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
        let variant_str = self
            .0
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::event::inventory::InventoryAction::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::event::inventory::InventoryAction::from_string(variant_str).unwrap(),
        )
    }
    /// Gets whether the ClickType for this event indicates that the key was pressed down when the click was made.
    pub fn is_shift_click(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isShiftClick", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    /// Gets the ItemStack currently in the clicked slot.
    pub fn current_item(
        &mut self,
    ) -> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getCurrentItem",
            "()Lorg/bukkit/inventory/ItemStack;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::ItemStack::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Gets whether or not the ClickType for this event represents a right click.
    pub fn is_right_click(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isRightClick", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    /// Gets whether or not the ClickType for this event represents a left click.
    pub fn is_left_click(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isLeftClick", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    /// Sets the ItemStack currently in the clicked slot.
    pub fn set_current_item(
        &mut self,
        arg0: impl Into<&'mc crate::inventory::ItemStack<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setCurrentItem",
            "(Lorg/bukkit/inventory/ItemStack;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Gets the inventory corresponding to the clicked slot.
    pub fn clicked_inventory(
        &mut self,
    ) -> Result<crate::inventory::Inventory<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getClickedInventory",
            "()Lorg/bukkit/inventory/Inventory;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::Inventory::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// The raw slot number clicked, ready for passing to <a href="../../inventory/InventoryView.html" title="class in org.bukkit.inventory"><code>#getItem(int)</code></a> This slot number is unique for the view.
    pub fn raw_slot(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getRawSlot", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }
    /// If the ClickType is NUMBER_KEY, this method will return the index of the pressed key (0-8).
    pub fn hotbar_button(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getHotbarButton", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }
    /// Gets the inventory corresponding to the clicked slot.
    /// Gets the ClickType for this event.
    /// <p>This is insulated against changes to the inventory by other plugins.</p>
    pub fn click(
        &mut self,
    ) -> Result<crate::event::inventory::ClickType<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getClick",
            "()Lorg/bukkit/event/inventory/ClickType;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant = self
            .0
            .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
        let variant_str = self
            .0
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::event::inventory::ClickType::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::event::inventory::ClickType::from_string(variant_str).unwrap(),
        )
    }
    /// Gets the type of slot that was clicked.
    /// The slot number that was clicked, ready for passing to <a href="../../inventory/Inventory.html#getItem(int)"><code>Inventory.getItem(int)</code></a>. Note that there may be two slots with the same slot number, since a view links two different inventories.
    pub fn slot(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getSlot", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }

    pub fn result(&mut self) -> Result<crate::event::EventResult<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getResult",
            "()Lorg/bukkit/event/Event$Result;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::event::EventResult::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn set_result(
        &mut self,
        arg0: impl Into<&'mc crate::event::EventResult<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setResult",
            "(Lorg/bukkit/event/Event$Result;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn is_cancelled(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isCancelled", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

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

    pub fn who_clicked(
        &mut self,
    ) -> Result<crate::entity::HumanEntity<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getWhoClicked",
            "()Lorg/bukkit/entity/HumanEntity;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::entity::HumanEntity::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn inventory(
        &mut self,
    ) -> Result<crate::inventory::Inventory<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getInventory",
            "()Lorg/bukkit/inventory/Inventory;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::Inventory::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn viewers(
        &mut self,
    ) -> Result<Vec<crate::entity::HumanEntity<'mc>>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getViewers", "()Ljava/util/List;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let mut list = blackboxmc_java::JavaList::from_raw(&self.0, res.l()?)?;
        let size = list.size()?;
        for i in 0..=size {
            let obj = list.get(i)?;
            new_vec.push(crate::entity::HumanEntity::from_raw(&self.0, obj)?);
        }
        Ok(new_vec)
    }

    pub fn view(
        &mut self,
    ) -> Result<crate::inventory::InventoryView<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getView",
            "()Lorg/bukkit/inventory/InventoryView;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::InventoryView::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

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
impl<'mc> Into<crate::event::inventory::InventoryInteractEvent<'mc>> for InventoryClickEvent<'mc> {
    fn into(self) -> crate::event::inventory::InventoryInteractEvent<'mc> {
        crate::event::inventory::InventoryInteractEvent::from_raw(&self.jni_ref(), self.1).unwrap()
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
pub struct InventoryDragEvent<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for InventoryDragEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> InventoryDragEvent<'mc> {
    pub fn from_raw(
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
    pub fn new(
        jni: blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<&'mc crate::inventory::InventoryView<'mc>>,
        arg1: impl Into<&'mc crate::inventory::ItemStack<'mc>>,
        arg2: impl Into<&'mc crate::inventory::ItemStack<'mc>>,
        arg3: bool,
        arg4: impl Into<&'mc blackboxmc_java::JavaMap<'mc>>,
    ) -> Result<crate::event::inventory::InventoryDragEvent<'mc>, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let val_2 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone()) };
        let val_3 = unsafe { jni::objects::JObject::from_raw(arg2.into().jni_object().clone()) };
        // -2
        let val_4 = jni::objects::JValueGen::Bool(arg3.into());
        let val_5 = unsafe { jni::objects::JObject::from_raw(arg4.into().jni_object().clone()) };
        let cls = &jni.find_class("org/bukkit/event/inventory/InventoryDragEvent")?;
        let res = jni.new_object(cls,
"(Lorg/bukkit/inventory/InventoryView;Lorg/bukkit/inventory/ItemStack;Lorg/bukkit/inventory/ItemStack;ZLjava/util/Map;)V",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2),jni::objects::JValueGen::from(&val_3),jni::objects::JValueGen::from(&val_4),jni::objects::JValueGen::from(&val_5)])?;
        crate::event::inventory::InventoryDragEvent::from_raw(&jni, res)
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
    /// Sets the result cursor after the drag is done.
    /// <p>Changing this item stack changes the cursor item. Note that changing the affected "dragged" slots does not change this ItemStack, nor does changing this ItemStack affect the "dragged" slots.</p>
    pub fn set_cursor(
        &mut self,
        arg0: impl Into<&'mc crate::inventory::ItemStack<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setCursor",
            "(Lorg/bukkit/inventory/ItemStack;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Gets the result cursor after the drag is done. The returned value is mutable.
    pub fn cursor(
        &mut self,
    ) -> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getCursor",
            "()Lorg/bukkit/inventory/ItemStack;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::ItemStack::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn handler_list(
        jni: blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>> {
        let cls = &jni.find_class("org/bukkit/event/HandlerList")?;
        let res = jni.call_static_method(
            cls,
            "getHandlerList",
            "()Lorg/bukkit/event/HandlerList;",
            &[],
        )?;
        let obj = res.l()?;
        crate::event::HandlerList::from_raw(&jni, obj)
    }
    /// Gets all items to be added to the inventory in this drag.
    pub fn new_items(
        &mut self,
    ) -> Result<blackboxmc_java::JavaMap<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getNewItems", "()Ljava/util/Map;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::JavaMap::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Gets the raw slot ids to be changed in this drag.
    pub fn raw_slots(
        &mut self,
    ) -> Result<blackboxmc_java::JavaSet<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getRawSlots", "()Ljava/util/Set;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::JavaSet::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Gets the slots to be changed in this drag.
    pub fn inventory_slots(
        &mut self,
    ) -> Result<blackboxmc_java::JavaSet<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getInventorySlots",
            "()Ljava/util/Set;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::JavaSet::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Gets an ItemStack representing the cursor prior to any modifications as a result of this drag.
    pub fn old_cursor(
        &mut self,
    ) -> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getOldCursor",
            "()Lorg/bukkit/inventory/ItemStack;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::ItemStack::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Gets the DragType that describes the behavior of ItemStacks placed after this InventoryDragEvent.
    /// <p>The ItemStacks and the raw slots that they're being applied to can be found using <a href="#getNewItems()"><code>getNewItems()</code></a>.</p>
    pub fn get_type(
        &mut self,
    ) -> Result<crate::event::inventory::DragType<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getType",
            "()Lorg/bukkit/event/inventory/DragType;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant = self
            .0
            .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
        let variant_str = self
            .0
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::event::inventory::DragType::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::event::inventory::DragType::from_string(variant_str).unwrap(),
        )
    }

    pub fn result(&mut self) -> Result<crate::event::EventResult<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getResult",
            "()Lorg/bukkit/event/Event$Result;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::event::EventResult::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn set_result(
        &mut self,
        arg0: impl Into<&'mc crate::event::EventResult<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setResult",
            "(Lorg/bukkit/event/Event$Result;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn is_cancelled(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isCancelled", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

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

    pub fn who_clicked(
        &mut self,
    ) -> Result<crate::entity::HumanEntity<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getWhoClicked",
            "()Lorg/bukkit/entity/HumanEntity;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::entity::HumanEntity::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Gets the slots to be changed in this drag.
    pub fn inventory(
        &mut self,
    ) -> Result<crate::inventory::Inventory<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getInventory",
            "()Lorg/bukkit/inventory/Inventory;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::Inventory::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn viewers(
        &mut self,
    ) -> Result<Vec<crate::entity::HumanEntity<'mc>>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getViewers", "()Ljava/util/List;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let mut list = blackboxmc_java::JavaList::from_raw(&self.0, res.l()?)?;
        let size = list.size()?;
        for i in 0..=size {
            let obj = list.get(i)?;
            new_vec.push(crate::entity::HumanEntity::from_raw(&self.0, obj)?);
        }
        Ok(new_vec)
    }

    pub fn view(
        &mut self,
    ) -> Result<crate::inventory::InventoryView<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getView",
            "()Lorg/bukkit/inventory/InventoryView;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::InventoryView::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

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
impl<'mc> Into<crate::event::inventory::InventoryInteractEvent<'mc>> for InventoryDragEvent<'mc> {
    fn into(self) -> crate::event::inventory::InventoryInteractEvent<'mc> {
        crate::event::inventory::InventoryInteractEvent::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
/// Called when an ItemStack is successfully burned as fuel in a furnace.
pub struct FurnaceBurnEvent<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for FurnaceBurnEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> FurnaceBurnEvent<'mc> {
    pub fn from_raw(
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
    pub fn new(
        jni: blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<&'mc crate::block::Block<'mc>>,
        arg1: impl Into<&'mc crate::inventory::ItemStack<'mc>>,
        arg2: i32,
    ) -> Result<crate::event::inventory::FurnaceBurnEvent<'mc>, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let val_2 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone()) };
        let val_3 = jni::objects::JValueGen::Int(arg2.into());
        let cls = &jni.find_class("org/bukkit/event/inventory/FurnaceBurnEvent")?;
        let res = jni.new_object(
            cls,
            "(Lorg/bukkit/block/Block;Lorg/bukkit/inventory/ItemStack;I)V",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
                jni::objects::JValueGen::from(&val_3),
            ],
        )?;
        crate::event::inventory::FurnaceBurnEvent::from_raw(&jni, res)
    }
    /// <span class="descfrm-type-label">Description copied from interface:&nbsp;<code><a href="../Cancellable.html#isCancelled()">Cancellable</a></code></span>
    /// Gets the cancellation state of this event. A cancelled event will not be executed in the server, but will still pass to other plugins
    pub fn is_cancelled(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isCancelled", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
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
    /// Gets the fuel ItemStack for this event
    pub fn fuel(&mut self) -> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getFuel",
            "()Lorg/bukkit/inventory/ItemStack;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::ItemStack::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Gets the burn time for this fuel
    pub fn burn_time(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getBurnTime", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }
    /// Sets the burn time for this fuel
    pub fn set_burn_time(&mut self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setBurnTime",
            "(I)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// <span class="descfrm-type-label">Description copied from interface:&nbsp;<code><a href="../Cancellable.html#setCancelled(boolean)">Cancellable</a></code></span>
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

    pub fn handler_list(
        jni: blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>> {
        let cls = &jni.find_class("org/bukkit/event/HandlerList")?;
        let res = jni.call_static_method(
            cls,
            "getHandlerList",
            "()Lorg/bukkit/event/HandlerList;",
            &[],
        )?;
        let obj = res.l()?;
        crate::event::HandlerList::from_raw(&jni, obj)
    }
    /// Gets whether the furnace's fuel is burning or not.
    pub fn is_burning(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isBurning", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    /// Sets whether the furnace's fuel is burning or not.
    pub fn set_burning(&mut self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        // -2
        let val_1 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setBurning",
            "(Z)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn block(&mut self) -> Result<crate::block::Block<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getBlock",
            "()Lorg/bukkit/block/Block;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::block::Block::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

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
impl<'mc> Into<crate::event::Cancellable<'mc>> for FurnaceBurnEvent<'mc> {
    fn into(self) -> crate::event::Cancellable<'mc> {
        crate::event::Cancellable::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
impl<'mc> Into<crate::event::block::BlockEvent<'mc>> for FurnaceBurnEvent<'mc> {
    fn into(self) -> crate::event::block::BlockEvent<'mc> {
        crate::event::block::BlockEvent::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
/// Represents a player related inventory event
pub struct InventoryCloseEvent<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for InventoryCloseEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> InventoryCloseEvent<'mc> {
    pub fn from_raw(
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
    pub fn new(
        jni: blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<&'mc crate::inventory::InventoryView<'mc>>,
    ) -> Result<crate::event::inventory::InventoryCloseEvent<'mc>, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let cls = &jni.find_class("org/bukkit/event/inventory/InventoryCloseEvent")?;
        let res = jni.new_object(
            cls,
            "(Lorg/bukkit/inventory/InventoryView;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        )?;
        crate::event::inventory::InventoryCloseEvent::from_raw(&jni, res)
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
    /// Returns the player involved in this event
    pub fn player(
        &mut self,
    ) -> Result<crate::entity::HumanEntity<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getPlayer",
            "()Lorg/bukkit/entity/HumanEntity;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::entity::HumanEntity::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn handler_list(
        jni: blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>> {
        let cls = &jni.find_class("org/bukkit/event/HandlerList")?;
        let res = jni.call_static_method(
            cls,
            "getHandlerList",
            "()Lorg/bukkit/event/HandlerList;",
            &[],
        )?;
        let obj = res.l()?;
        crate::event::HandlerList::from_raw(&jni, obj)
    }

    pub fn inventory(
        &mut self,
    ) -> Result<crate::inventory::Inventory<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getInventory",
            "()Lorg/bukkit/inventory/Inventory;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::Inventory::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn viewers(
        &mut self,
    ) -> Result<Vec<crate::entity::HumanEntity<'mc>>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getViewers", "()Ljava/util/List;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let mut list = blackboxmc_java::JavaList::from_raw(&self.0, res.l()?)?;
        let size = list.size()?;
        for i in 0..=size {
            let obj = list.get(i)?;
            new_vec.push(crate::entity::HumanEntity::from_raw(&self.0, obj)?);
        }
        Ok(new_vec)
    }

    pub fn view(
        &mut self,
    ) -> Result<crate::inventory::InventoryView<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getView",
            "()Lorg/bukkit/inventory/InventoryView;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::InventoryView::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

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
impl<'mc> Into<crate::event::inventory::InventoryEvent<'mc>> for InventoryCloseEvent<'mc> {
    fn into(self) -> crate::event::inventory::InventoryEvent<'mc> {
        crate::event::inventory::InventoryEvent::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
/// Called when an item is put in a slot and the result is calculated.
pub struct PrepareInventoryResultEvent<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for PrepareInventoryResultEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> PrepareInventoryResultEvent<'mc> {
    pub fn from_raw(
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
    pub fn new(
        jni: blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<&'mc crate::inventory::InventoryView<'mc>>,
        arg1: impl Into<&'mc crate::inventory::ItemStack<'mc>>,
    ) -> Result<crate::event::inventory::PrepareInventoryResultEvent<'mc>, Box<dyn std::error::Error>>
    {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let val_2 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone()) };
        let cls = &jni.find_class("org/bukkit/event/inventory/PrepareInventoryResultEvent")?;
        let res = jni.new_object(
            cls,
            "(Lorg/bukkit/inventory/InventoryView;Lorg/bukkit/inventory/ItemStack;)V",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        )?;
        crate::event::inventory::PrepareInventoryResultEvent::from_raw(&jni, res)
    }
    /// Get result item, may be null.
    pub fn result(
        &mut self,
    ) -> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getResult",
            "()Lorg/bukkit/inventory/ItemStack;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::ItemStack::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Set result item, may be null.
    pub fn set_result(
        &mut self,
        arg0: impl Into<&'mc crate::inventory::ItemStack<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setResult",
            "(Lorg/bukkit/inventory/ItemStack;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
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

    pub fn handler_list(
        jni: blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>> {
        let cls = &jni.find_class("org/bukkit/event/HandlerList")?;
        let res = jni.call_static_method(
            cls,
            "getHandlerList",
            "()Lorg/bukkit/event/HandlerList;",
            &[],
        )?;
        let obj = res.l()?;
        crate::event::HandlerList::from_raw(&jni, obj)
    }

    pub fn inventory(
        &mut self,
    ) -> Result<crate::inventory::Inventory<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getInventory",
            "()Lorg/bukkit/inventory/Inventory;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::Inventory::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn viewers(
        &mut self,
    ) -> Result<Vec<crate::entity::HumanEntity<'mc>>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getViewers", "()Ljava/util/List;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let mut list = blackboxmc_java::JavaList::from_raw(&self.0, res.l()?)?;
        let size = list.size()?;
        for i in 0..=size {
            let obj = list.get(i)?;
            new_vec.push(crate::entity::HumanEntity::from_raw(&self.0, obj)?);
        }
        Ok(new_vec)
    }

    pub fn view(
        &mut self,
    ) -> Result<crate::inventory::InventoryView<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getView",
            "()Lorg/bukkit/inventory/InventoryView;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::InventoryView::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

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
impl<'mc> Into<crate::event::inventory::InventoryEvent<'mc>> for PrepareInventoryResultEvent<'mc> {
    fn into(self) -> crate::event::inventory::InventoryEvent<'mc> {
        crate::event::inventory::InventoryEvent::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
