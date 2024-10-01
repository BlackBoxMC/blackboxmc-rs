#![allow(deprecated)]
use blackboxmc_general::JNIInstantiatable;
use blackboxmc_general::JNIRaw;
use color_eyre::eyre::Result;
#[repr(C)]
pub struct PrepareItemEnchantEvent<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for PrepareItemEnchantEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for PrepareItemEnchantEvent<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate PrepareItemEnchantEvent from null object."
            )
            .into());
        }
        let (valid, name) =
            env.validate_name(&obj, "org/bukkit/event/enchantment/PrepareItemEnchantEvent")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a PrepareItemEnchantEvent object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> PrepareItemEnchantEvent<'mc> {
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        enchanter: impl Into<crate::entity::Player<'mc>>,
        view: impl Into<crate::inventory::InventoryView<'mc>>,
        table: impl Into<crate::block::Block<'mc>>,
        item: impl Into<crate::inventory::ItemStack<'mc>>,
        offers: impl Into<crate::enchantments::EnchantmentOffer<'mc>>,
        bonus: i32,
    ) -> Result<crate::event::enchantment::PrepareItemEnchantEvent<'mc>, Box<dyn std::error::Error>>
    {
        let sig = String::from("(Lorg/bukkit/entity/Player;Lorg/bukkit/inventory/InventoryView;Lorg/bukkit/block/Block;Lorg/bukkit/inventory/ItemStack;Lorg/bukkit/enchantments/EnchantmentOffer;I)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(enchanter.into().jni_object().clone())
        });
        let val_2 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(view.into().jni_object().clone())
        });
        let val_3 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(table.into().jni_object().clone())
        });
        let val_4 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(item.into().jni_object().clone())
        });
        let val_5 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(offers.into().jni_object().clone())
        });
        let val_6 = jni::objects::JValueGen::Int(bonus);
        let cls = jni.find_class("org/bukkit/event/enchantment/PrepareItemEnchantEvent");
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
                jni::objects::JValueGen::from(val_6),
            ],
        );
        let res = jni.translate_error_no_gen(res)?;
        crate::event::enchantment::PrepareItemEnchantEvent::from_raw(&jni, res)
    }
    /// Gets the player enchanting the item
    pub fn enchanter(&self) -> Result<crate::entity::Player<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/entity/Player;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getEnchanter", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::entity::Player::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Gets the block being used to enchant the item
    pub fn enchant_block(&self) -> Result<crate::block::Block<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/block/Block;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getEnchantBlock", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::block::Block::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Gets the item to be enchanted.
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
    #[deprecated]
    /// Get a list of offered experience level costs of the enchantment.
    pub fn exp_level_costs_offered(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getExpLevelCostsOffered",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    /// Get a list of available {@link EnchantmentOffer} for the player. You can
    /// modify the values to change the available offers for the player. An offer
    /// may be null, if there isn't a enchantment offer at a specific slot. There
    /// are 3 slots in the enchantment table available to modify.
    pub fn offers(
        &self,
    ) -> Result<crate::enchantments::EnchantmentOffer<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/enchantments/EnchantmentOffer;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getOffers", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::enchantments::EnchantmentOffer::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Get enchantment bonus in effect - corresponds to number of bookshelves
    pub fn enchantment_bonus(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getEnchantmentBonus",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }

    pub fn is_cancelled(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isCancelled", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn set_cancelled(&self, cancel: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        let val_1 = jni::objects::JValueGen::Bool(cancel.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setCancelled",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
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

    pub fn handler_list(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/event/HandlerList;");
        let cls = jni.find_class("org/bukkit/event/enchantment/PrepareItemEnchantEvent");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "getHandlerList", sig.as_str(), vec![]);
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        crate::event::HandlerList::from_raw(&jni, obj)
    }
    // SUPER CLASS: org.bukkit.event.inventory.InventoryEvent ( ['getEnchanter', 'getEnchantBlock', 'getItem', 'getExpLevelCostsOffered', 'getOffers', 'getEnchantmentBonus', 'isCancelled', 'setCancelled', 'getHandlers', 'getHandlerList'])
    /// Gets the primary Inventory involved in this transaction
    pub fn inventory(
        &self,
    ) -> Result<crate::inventory::Inventory<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::inventory::InventoryEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::inventory::InventoryEvent = temp_clone.into();
        real.inventory()
    }
    /// Gets the list of players viewing the primary (upper) inventory involved
    /// in this event
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
    /// Gets the view object itself
    pub fn view(&self) -> Result<crate::inventory::InventoryView<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::inventory::InventoryEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::inventory::InventoryEvent = temp_clone.into();
        real.view()
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::event::Cancellable<'mc>> for PrepareItemEnchantEvent<'mc> {
    fn into(self) -> crate::event::Cancellable<'mc> {
        crate::event::Cancellable::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting PrepareItemEnchantEvent into crate::event::Cancellable")
    }
}
impl<'mc> Into<crate::event::inventory::InventoryEvent<'mc>> for PrepareItemEnchantEvent<'mc> {
    fn into(self) -> crate::event::inventory::InventoryEvent<'mc> {
        crate::event::inventory::InventoryEvent::from_raw(&self.jni_ref(), self.1).expect(
            "Error converting PrepareItemEnchantEvent into crate::event::inventory::InventoryEvent",
        )
    }
}
#[repr(C)]
pub struct EnchantItemEvent<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for EnchantItemEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for EnchantItemEvent<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate EnchantItemEvent from null object.").into(),
            );
        }
        let (valid, name) =
            env.validate_name(&obj, "org/bukkit/event/enchantment/EnchantItemEvent")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a EnchantItemEvent object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> EnchantItemEvent<'mc> {
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        enchanter: impl Into<crate::entity::Player<'mc>>,
        view: impl Into<crate::inventory::InventoryView<'mc>>,
        table: impl Into<crate::block::Block<'mc>>,
        item: impl Into<crate::inventory::ItemStack<'mc>>,
        level: i32,
        enchants: impl Into<blackboxmc_java::util::JavaMap<'mc>>,
        enchantment_hint: impl Into<crate::enchantments::Enchantment<'mc>>,
        level_hint: i32,
        i: i32,
    ) -> Result<crate::event::enchantment::EnchantItemEvent<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/entity/Player;Lorg/bukkit/inventory/InventoryView;Lorg/bukkit/block/Block;Lorg/bukkit/inventory/ItemStack;ILjava/util/Map;Lorg/bukkit/enchantments/Enchantment;II)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(enchanter.into().jni_object().clone())
        });
        let val_2 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(view.into().jni_object().clone())
        });
        let val_3 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(table.into().jni_object().clone())
        });
        let val_4 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(item.into().jni_object().clone())
        });
        let val_5 = jni::objects::JValueGen::Int(level);
        let val_6 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(enchants.into().jni_object().clone())
        });
        let val_7 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(enchantment_hint.into().jni_object().clone())
        });
        let val_8 = jni::objects::JValueGen::Int(level_hint);
        let val_9 = jni::objects::JValueGen::Int(i);
        let cls = jni.find_class("org/bukkit/event/enchantment/EnchantItemEvent");
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
                jni::objects::JValueGen::from(val_6),
                jni::objects::JValueGen::from(val_7),
                jni::objects::JValueGen::from(val_8),
                jni::objects::JValueGen::from(val_9),
            ],
        );
        let res = jni.translate_error_no_gen(res)?;
        crate::event::enchantment::EnchantItemEvent::from_raw(&jni, res)
    }
    /// Gets the player enchanting the item
    pub fn enchanter(&self) -> Result<crate::entity::Player<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/entity/Player;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getEnchanter", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::entity::Player::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Gets the block being used to enchant the item
    pub fn enchant_block(&self) -> Result<crate::block::Block<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/block/Block;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getEnchantBlock", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::block::Block::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Gets the item to be enchanted (can be modified)
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
    /// Gets the cost (minimum level) which is displayed as a number on the right
    /// hand side of the enchantment offer.
    pub fn exp_level_cost(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getExpLevelCost", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    /// Sets the cost (minimum level) which is displayed as a number on the right
    /// hand side of the enchantment offer.
    pub fn set_exp_level_cost(&self, level: i32) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(I)V");
        let val_1 = jni::objects::JValueGen::Int(level);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setExpLevelCost",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Get map of enchantment (levels, keyed by type) to be added to item
    /// (modify map returned to change values). Note: Any enchantments not
    /// allowed for the item will be ignored
    pub fn enchants_to_add(
        &self,
    ) -> Result<blackboxmc_java::util::JavaMap<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/Map;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getEnchantsToAdd",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::util::JavaMap::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Get the {@link Enchantment} that was displayed as a hint to the player
    /// on the selected enchantment offer.
    pub fn enchantment_hint(
        &self,
    ) -> Result<crate::enchantments::Enchantment<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/enchantments/Enchantment;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getEnchantmentHint",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::enchantments::Enchantment::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Get the level of the enchantment that was displayed as a hint to the
    /// player on the selected enchantment offer.
    pub fn level_hint(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getLevelHint", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    /// Which button was pressed to initiate the enchanting.
    pub fn which_button(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "whichButton", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }

    pub fn is_cancelled(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isCancelled", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn set_cancelled(&self, cancel: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        let val_1 = jni::objects::JValueGen::Bool(cancel.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setCancelled",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
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

    pub fn handler_list(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/event/HandlerList;");
        let cls = jni.find_class("org/bukkit/event/enchantment/EnchantItemEvent");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "getHandlerList", sig.as_str(), vec![]);
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        crate::event::HandlerList::from_raw(&jni, obj)
    }
    // SUPER CLASS: org.bukkit.event.inventory.InventoryEvent ( ['getEnchanter', 'getEnchantBlock', 'getItem', 'getExpLevelCost', 'setExpLevelCost', 'getEnchantsToAdd', 'getEnchantmentHint', 'getLevelHint', 'whichButton', 'isCancelled', 'setCancelled', 'getHandlers', 'getHandlerList'])
    /// Gets the primary Inventory involved in this transaction
    pub fn inventory(
        &self,
    ) -> Result<crate::inventory::Inventory<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::inventory::InventoryEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::inventory::InventoryEvent = temp_clone.into();
        real.inventory()
    }
    /// Gets the list of players viewing the primary (upper) inventory involved
    /// in this event
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
    /// Gets the view object itself
    pub fn view(&self) -> Result<crate::inventory::InventoryView<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::inventory::InventoryEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::inventory::InventoryEvent = temp_clone.into();
        real.view()
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::event::Cancellable<'mc>> for EnchantItemEvent<'mc> {
    fn into(self) -> crate::event::Cancellable<'mc> {
        crate::event::Cancellable::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting EnchantItemEvent into crate::event::Cancellable")
    }
}
impl<'mc> Into<crate::event::inventory::InventoryEvent<'mc>> for EnchantItemEvent<'mc> {
    fn into(self) -> crate::event::inventory::InventoryEvent<'mc> {
        crate::event::inventory::InventoryEvent::from_raw(&self.jni_ref(), self.1).expect(
            "Error converting EnchantItemEvent into crate::event::inventory::InventoryEvent",
        )
    }
}
