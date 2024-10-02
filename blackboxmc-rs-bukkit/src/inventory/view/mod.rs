#![allow(deprecated)]
use blackboxmc_general::JNIInstantiatable;
use blackboxmc_general::JNIRaw;
use color_eyre::eyre::Result;
#[repr(C)]
pub struct LoomView<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for LoomView<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for LoomView<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate LoomView from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/inventory/view/LoomView")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a LoomView object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> LoomView<'mc> {
    /// Gets a list of all selectable to the player.
    pub fn selectable_patterns(
        &self,
    ) -> Result<Vec<crate::block::banner::PatternType<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/List;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getSelectablePatterns",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let list = blackboxmc_java::util::JavaList::from_raw(&self.jni_ref(), res.l()?)?;
        let iter = list.iterator()?;
        while iter.has_next()? {
            let obj = iter.next()?;
            new_vec.push(crate::block::banner::PatternType::from_raw(
                &self.jni_ref(),
                obj,
            )?);
        }
        Ok(new_vec)
    }
    /// Gets an index of the selected pattern.
    pub fn selected_pattern_index(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getSelectedPatternIndex",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    /// Get the upper inventory involved in this transaction.
    pub fn top_inventory(
        &self,
    ) -> Result<crate::inventory::Inventory<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/inventory/Inventory;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getTopInventory", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::Inventory::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Get the lower inventory involved in this transaction.
    pub fn bottom_inventory(
        &self,
    ) -> Result<crate::inventory::Inventory<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/inventory/Inventory;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getBottomInventory",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::Inventory::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Get the player viewing.
    pub fn player(&self) -> Result<crate::entity::HumanEntity<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/entity/HumanEntity;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getPlayer", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::entity::HumanEntity::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Determine the type of inventory involved in the transaction. This
    /// indicates the window style being shown. It will never return PLAYER,
    /// since that is common to all windows.
    pub fn get_type(
        &self,
    ) -> Result<crate::event::inventory::InventoryType<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/event/inventory/InventoryType;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getType", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::event::inventory::InventoryType::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Sets one item in this inventory view by its raw slot ID.
    ///
    /// Note: If slot ID -999 is chosen, it may be expected that the item is
    /// dropped on the ground. This is not required behaviour, however.
    pub fn set_item(
        &self,
        slot: i32,
        item: impl Into<crate::inventory::ItemStack<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(ILorg/bukkit/inventory/ItemStack;)V");
        let val_1 = jni::objects::JValueGen::Int(slot);
        let val_2 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(item.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setItem",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
            ],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Gets one item in this inventory view by its raw slot ID.
    pub fn get_item(
        &self,
        slot: i32,
    ) -> Result<Option<crate::inventory::ItemStack<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("(I)Lorg/bukkit/inventory/ItemStack;");
        let val_1 = jni::objects::JValueGen::Int(slot);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getItem",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {
            return Ok(None);
        }
        Ok(Some(crate::inventory::ItemStack::from_raw(
            &self.jni_ref(),
            unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) },
        )?))
    }
    /// Sets the item on the cursor of one of the viewing players.
    pub fn set_cursor(
        &self,
        item: impl Into<crate::inventory::ItemStack<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/inventory/ItemStack;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(item.into().jni_object().clone())
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
    /// Get the item on the cursor of one of the viewing players.
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
    /// Gets the inventory corresponding to the given raw slot ID.
    /// If the slot ID is {@link #OUTSIDE} null will be returned, otherwise
    /// behaviour for illegal and negative slot IDs is undefined.
    /// May be used with {@link #convertSlot(int)} to directly index an
    /// underlying inventory.
    pub fn get_inventory(
        &self,
        raw_slot: i32,
    ) -> Result<Option<crate::inventory::Inventory<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("(I)Lorg/bukkit/inventory/Inventory;");
        let val_1 = jni::objects::JValueGen::Int(raw_slot);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getInventory",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
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
    /// Converts a raw slot ID into its local slot ID into whichever of the two
    /// inventories the slot points to.
    ///
    /// If the raw slot refers to the upper inventory, it will be returned
    /// unchanged and thus be suitable for getTopInventory().getItem(); if it
    /// refers to the lower inventory, the output will differ from the input
    /// and be suitable for getBottomInventory().getItem().
    pub fn convert_slot(&self, raw_slot: i32) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("(I)I");
        let val_1 = jni::objects::JValueGen::Int(raw_slot);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "convertSlot",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    /// Determine the type of the slot by its raw slot ID.
    ///
    /// If the type of the slot is unknown, then
    /// {@link InventoryType.SlotType#CONTAINER} will be returned.
    pub fn get_slot_type(
        &self,
        slot: i32,
    ) -> Result<crate::event::inventory::InventoryTypeSlotType<'mc>, Box<dyn std::error::Error>>
    {
        let sig = String::from("(I)Lorg/bukkit/event/inventory/InventoryType/SlotType;");
        let val_1 = jni::objects::JValueGen::Int(slot);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getSlotType",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::event::inventory::InventoryTypeSlotType::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Closes the inventory view.
    pub fn close(&self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "close", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Check the total number of slots in this view, combining the upper and
    /// lower inventories.
    ///
    /// Note though that it's possible for this to be greater than the sum of
    /// the two inventories if for example some slots are not being used.
    pub fn count_slots(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "countSlots", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    /// Sets an extra property of this inventory if supported by that
    /// inventory, for example the state of a progress bar.
    pub fn set_property(
        &self,
        prop: impl Into<crate::inventory::InventoryViewProperty<'mc>>,
        value: i32,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/inventory/InventoryView/Property;I)Z");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(prop.into().jni_object().clone())
        });
        let val_2 = jni::objects::JValueGen::Int(value);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setProperty",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Get the title of this inventory window.
    pub fn title(&self) -> Result<String, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/String;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getTitle", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    /// Get the original title of this inventory window, before any changes were
    /// made using {@link #setTitle(String)}.
    pub fn original_title(&self) -> Result<String, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/String;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getOriginalTitle",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    /// Sets the title of this inventory window to the specified title if the
    /// inventory window supports it.
    ///
    /// Note if the inventory does not support titles that can be changed (ie, it
    /// is not creatable or viewed by a player), then this method will throw an
    /// exception.
    pub fn set_title(&self, title: impl Into<String>) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/String;)V");
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(title.into())?,
        ));
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setTitle",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::inventory::InventoryView<'mc>> for LoomView<'mc> {
    fn into(self) -> crate::inventory::InventoryView<'mc> {
        crate::inventory::InventoryView::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting LoomView into crate::inventory::InventoryView")
    }
}
#[repr(C)]
pub struct EnchantmentView<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for EnchantmentView<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for EnchantmentView<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate EnchantmentView from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/inventory/view/EnchantmentView")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a EnchantmentView object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> EnchantmentView<'mc> {
    /// Gets the random enchantment seed used in this view
    pub fn enchantment_seed(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getEnchantmentSeed",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    /// Gets the offers of this EnchantmentView
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
    /// Sets the offers to provide to the player.
    pub fn set_offers(
        &self,
        offers: impl Into<crate::enchantments::EnchantmentOffer<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/enchantments/EnchantmentOffer;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(offers.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setOffers",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Get the upper inventory involved in this transaction.
    pub fn top_inventory(
        &self,
    ) -> Result<crate::inventory::Inventory<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/inventory/Inventory;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getTopInventory", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::Inventory::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Get the lower inventory involved in this transaction.
    pub fn bottom_inventory(
        &self,
    ) -> Result<crate::inventory::Inventory<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/inventory/Inventory;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getBottomInventory",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::Inventory::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Get the player viewing.
    pub fn player(&self) -> Result<crate::entity::HumanEntity<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/entity/HumanEntity;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getPlayer", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::entity::HumanEntity::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Determine the type of inventory involved in the transaction. This
    /// indicates the window style being shown. It will never return PLAYER,
    /// since that is common to all windows.
    pub fn get_type(
        &self,
    ) -> Result<crate::event::inventory::InventoryType<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/event/inventory/InventoryType;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getType", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::event::inventory::InventoryType::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Sets one item in this inventory view by its raw slot ID.
    ///
    /// Note: If slot ID -999 is chosen, it may be expected that the item is
    /// dropped on the ground. This is not required behaviour, however.
    pub fn set_item(
        &self,
        slot: i32,
        item: impl Into<crate::inventory::ItemStack<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(ILorg/bukkit/inventory/ItemStack;)V");
        let val_1 = jni::objects::JValueGen::Int(slot);
        let val_2 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(item.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setItem",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
            ],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Gets one item in this inventory view by its raw slot ID.
    pub fn get_item(
        &self,
        slot: i32,
    ) -> Result<Option<crate::inventory::ItemStack<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("(I)Lorg/bukkit/inventory/ItemStack;");
        let val_1 = jni::objects::JValueGen::Int(slot);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getItem",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {
            return Ok(None);
        }
        Ok(Some(crate::inventory::ItemStack::from_raw(
            &self.jni_ref(),
            unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) },
        )?))
    }
    /// Sets the item on the cursor of one of the viewing players.
    pub fn set_cursor(
        &self,
        item: impl Into<crate::inventory::ItemStack<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/inventory/ItemStack;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(item.into().jni_object().clone())
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
    /// Get the item on the cursor of one of the viewing players.
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
    /// Gets the inventory corresponding to the given raw slot ID.
    /// If the slot ID is {@link #OUTSIDE} null will be returned, otherwise
    /// behaviour for illegal and negative slot IDs is undefined.
    /// May be used with {@link #convertSlot(int)} to directly index an
    /// underlying inventory.
    pub fn get_inventory(
        &self,
        raw_slot: i32,
    ) -> Result<Option<crate::inventory::Inventory<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("(I)Lorg/bukkit/inventory/Inventory;");
        let val_1 = jni::objects::JValueGen::Int(raw_slot);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getInventory",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
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
    /// Converts a raw slot ID into its local slot ID into whichever of the two
    /// inventories the slot points to.
    ///
    /// If the raw slot refers to the upper inventory, it will be returned
    /// unchanged and thus be suitable for getTopInventory().getItem(); if it
    /// refers to the lower inventory, the output will differ from the input
    /// and be suitable for getBottomInventory().getItem().
    pub fn convert_slot(&self, raw_slot: i32) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("(I)I");
        let val_1 = jni::objects::JValueGen::Int(raw_slot);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "convertSlot",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    /// Determine the type of the slot by its raw slot ID.
    ///
    /// If the type of the slot is unknown, then
    /// {@link InventoryType.SlotType#CONTAINER} will be returned.
    pub fn get_slot_type(
        &self,
        slot: i32,
    ) -> Result<crate::event::inventory::InventoryTypeSlotType<'mc>, Box<dyn std::error::Error>>
    {
        let sig = String::from("(I)Lorg/bukkit/event/inventory/InventoryType/SlotType;");
        let val_1 = jni::objects::JValueGen::Int(slot);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getSlotType",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::event::inventory::InventoryTypeSlotType::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Closes the inventory view.
    pub fn close(&self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "close", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Check the total number of slots in this view, combining the upper and
    /// lower inventories.
    ///
    /// Note though that it's possible for this to be greater than the sum of
    /// the two inventories if for example some slots are not being used.
    pub fn count_slots(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "countSlots", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    /// Sets an extra property of this inventory if supported by that
    /// inventory, for example the state of a progress bar.
    pub fn set_property(
        &self,
        prop: impl Into<crate::inventory::InventoryViewProperty<'mc>>,
        value: i32,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/inventory/InventoryView/Property;I)Z");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(prop.into().jni_object().clone())
        });
        let val_2 = jni::objects::JValueGen::Int(value);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setProperty",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Get the title of this inventory window.
    pub fn title(&self) -> Result<String, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/String;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getTitle", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    /// Get the original title of this inventory window, before any changes were
    /// made using {@link #setTitle(String)}.
    pub fn original_title(&self) -> Result<String, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/String;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getOriginalTitle",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    /// Sets the title of this inventory window to the specified title if the
    /// inventory window supports it.
    ///
    /// Note if the inventory does not support titles that can be changed (ie, it
    /// is not creatable or viewed by a player), then this method will throw an
    /// exception.
    pub fn set_title(&self, title: impl Into<String>) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/String;)V");
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(title.into())?,
        ));
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setTitle",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::inventory::InventoryView<'mc>> for EnchantmentView<'mc> {
    fn into(self) -> crate::inventory::InventoryView<'mc> {
        crate::inventory::InventoryView::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting EnchantmentView into crate::inventory::InventoryView")
    }
}
#[repr(C)]
pub struct CrafterView<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for CrafterView<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for CrafterView<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate CrafterView from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/inventory/view/CrafterView")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a CrafterView object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> CrafterView<'mc> {
    /// Checks if the given crafter slot is disabled.
    pub fn is_slot_disabled(&self, slot: i32) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(I)Z");
        let val_1 = jni::objects::JValueGen::Int(slot);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "isSlotDisabled",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Checks whether or not this crafter view is powered.
    pub fn is_powered(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isPowered", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Sets the status of the crafter slot.
    pub fn set_slot_disabled(
        &self,
        slot: i32,
        disabled: bool,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(IZ)V");
        let val_1 = jni::objects::JValueGen::Int(slot);
        let val_2 = jni::objects::JValueGen::Bool(disabled.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setSlotDisabled",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
            ],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Get the upper inventory involved in this transaction.
    pub fn top_inventory(
        &self,
    ) -> Result<crate::inventory::Inventory<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/inventory/Inventory;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getTopInventory", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::Inventory::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Get the lower inventory involved in this transaction.
    pub fn bottom_inventory(
        &self,
    ) -> Result<crate::inventory::Inventory<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/inventory/Inventory;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getBottomInventory",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::Inventory::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Get the player viewing.
    pub fn player(&self) -> Result<crate::entity::HumanEntity<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/entity/HumanEntity;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getPlayer", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::entity::HumanEntity::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Determine the type of inventory involved in the transaction. This
    /// indicates the window style being shown. It will never return PLAYER,
    /// since that is common to all windows.
    pub fn get_type(
        &self,
    ) -> Result<crate::event::inventory::InventoryType<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/event/inventory/InventoryType;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getType", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::event::inventory::InventoryType::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Sets one item in this inventory view by its raw slot ID.
    ///
    /// Note: If slot ID -999 is chosen, it may be expected that the item is
    /// dropped on the ground. This is not required behaviour, however.
    pub fn set_item(
        &self,
        slot: i32,
        item: impl Into<crate::inventory::ItemStack<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(ILorg/bukkit/inventory/ItemStack;)V");
        let val_1 = jni::objects::JValueGen::Int(slot);
        let val_2 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(item.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setItem",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
            ],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Gets one item in this inventory view by its raw slot ID.
    pub fn get_item(
        &self,
        slot: i32,
    ) -> Result<Option<crate::inventory::ItemStack<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("(I)Lorg/bukkit/inventory/ItemStack;");
        let val_1 = jni::objects::JValueGen::Int(slot);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getItem",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {
            return Ok(None);
        }
        Ok(Some(crate::inventory::ItemStack::from_raw(
            &self.jni_ref(),
            unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) },
        )?))
    }
    /// Sets the item on the cursor of one of the viewing players.
    pub fn set_cursor(
        &self,
        item: impl Into<crate::inventory::ItemStack<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/inventory/ItemStack;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(item.into().jni_object().clone())
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
    /// Get the item on the cursor of one of the viewing players.
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
    /// Gets the inventory corresponding to the given raw slot ID.
    /// If the slot ID is {@link #OUTSIDE} null will be returned, otherwise
    /// behaviour for illegal and negative slot IDs is undefined.
    /// May be used with {@link #convertSlot(int)} to directly index an
    /// underlying inventory.
    pub fn get_inventory(
        &self,
        raw_slot: i32,
    ) -> Result<Option<crate::inventory::Inventory<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("(I)Lorg/bukkit/inventory/Inventory;");
        let val_1 = jni::objects::JValueGen::Int(raw_slot);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getInventory",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
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
    /// Converts a raw slot ID into its local slot ID into whichever of the two
    /// inventories the slot points to.
    ///
    /// If the raw slot refers to the upper inventory, it will be returned
    /// unchanged and thus be suitable for getTopInventory().getItem(); if it
    /// refers to the lower inventory, the output will differ from the input
    /// and be suitable for getBottomInventory().getItem().
    pub fn convert_slot(&self, raw_slot: i32) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("(I)I");
        let val_1 = jni::objects::JValueGen::Int(raw_slot);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "convertSlot",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    /// Determine the type of the slot by its raw slot ID.
    ///
    /// If the type of the slot is unknown, then
    /// {@link InventoryType.SlotType#CONTAINER} will be returned.
    pub fn get_slot_type(
        &self,
        slot: i32,
    ) -> Result<crate::event::inventory::InventoryTypeSlotType<'mc>, Box<dyn std::error::Error>>
    {
        let sig = String::from("(I)Lorg/bukkit/event/inventory/InventoryType/SlotType;");
        let val_1 = jni::objects::JValueGen::Int(slot);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getSlotType",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::event::inventory::InventoryTypeSlotType::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Closes the inventory view.
    pub fn close(&self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "close", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Check the total number of slots in this view, combining the upper and
    /// lower inventories.
    ///
    /// Note though that it's possible for this to be greater than the sum of
    /// the two inventories if for example some slots are not being used.
    pub fn count_slots(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "countSlots", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    /// Sets an extra property of this inventory if supported by that
    /// inventory, for example the state of a progress bar.
    pub fn set_property(
        &self,
        prop: impl Into<crate::inventory::InventoryViewProperty<'mc>>,
        value: i32,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/inventory/InventoryView/Property;I)Z");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(prop.into().jni_object().clone())
        });
        let val_2 = jni::objects::JValueGen::Int(value);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setProperty",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Get the title of this inventory window.
    pub fn title(&self) -> Result<String, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/String;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getTitle", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    /// Get the original title of this inventory window, before any changes were
    /// made using {@link #setTitle(String)}.
    pub fn original_title(&self) -> Result<String, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/String;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getOriginalTitle",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    /// Sets the title of this inventory window to the specified title if the
    /// inventory window supports it.
    ///
    /// Note if the inventory does not support titles that can be changed (ie, it
    /// is not creatable or viewed by a player), then this method will throw an
    /// exception.
    pub fn set_title(&self, title: impl Into<String>) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/String;)V");
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(title.into())?,
        ));
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setTitle",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::inventory::InventoryView<'mc>> for CrafterView<'mc> {
    fn into(self) -> crate::inventory::InventoryView<'mc> {
        crate::inventory::InventoryView::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting CrafterView into crate::inventory::InventoryView")
    }
}
#[repr(C)]
pub struct MerchantView<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for MerchantView<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for MerchantView<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate MerchantView from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/inventory/view/MerchantView")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a MerchantView object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> MerchantView<'mc> {
    /// Gets the merchant that this view is for.
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
    /// Get the upper inventory involved in this transaction.
    pub fn top_inventory(
        &self,
    ) -> Result<crate::inventory::Inventory<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/inventory/Inventory;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getTopInventory", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::Inventory::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Get the lower inventory involved in this transaction.
    pub fn bottom_inventory(
        &self,
    ) -> Result<crate::inventory::Inventory<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/inventory/Inventory;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getBottomInventory",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::Inventory::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Get the player viewing.
    pub fn player(&self) -> Result<crate::entity::HumanEntity<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/entity/HumanEntity;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getPlayer", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::entity::HumanEntity::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Determine the type of inventory involved in the transaction. This
    /// indicates the window style being shown. It will never return PLAYER,
    /// since that is common to all windows.
    pub fn get_type(
        &self,
    ) -> Result<crate::event::inventory::InventoryType<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/event/inventory/InventoryType;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getType", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::event::inventory::InventoryType::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Sets one item in this inventory view by its raw slot ID.
    ///
    /// Note: If slot ID -999 is chosen, it may be expected that the item is
    /// dropped on the ground. This is not required behaviour, however.
    pub fn set_item(
        &self,
        slot: i32,
        item: impl Into<crate::inventory::ItemStack<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(ILorg/bukkit/inventory/ItemStack;)V");
        let val_1 = jni::objects::JValueGen::Int(slot);
        let val_2 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(item.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setItem",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
            ],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Gets one item in this inventory view by its raw slot ID.
    pub fn get_item(
        &self,
        slot: i32,
    ) -> Result<Option<crate::inventory::ItemStack<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("(I)Lorg/bukkit/inventory/ItemStack;");
        let val_1 = jni::objects::JValueGen::Int(slot);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getItem",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {
            return Ok(None);
        }
        Ok(Some(crate::inventory::ItemStack::from_raw(
            &self.jni_ref(),
            unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) },
        )?))
    }
    /// Sets the item on the cursor of one of the viewing players.
    pub fn set_cursor(
        &self,
        item: impl Into<crate::inventory::ItemStack<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/inventory/ItemStack;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(item.into().jni_object().clone())
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
    /// Get the item on the cursor of one of the viewing players.
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
    /// Gets the inventory corresponding to the given raw slot ID.
    /// If the slot ID is {@link #OUTSIDE} null will be returned, otherwise
    /// behaviour for illegal and negative slot IDs is undefined.
    /// May be used with {@link #convertSlot(int)} to directly index an
    /// underlying inventory.
    pub fn get_inventory(
        &self,
        raw_slot: i32,
    ) -> Result<Option<crate::inventory::Inventory<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("(I)Lorg/bukkit/inventory/Inventory;");
        let val_1 = jni::objects::JValueGen::Int(raw_slot);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getInventory",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
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
    /// Converts a raw slot ID into its local slot ID into whichever of the two
    /// inventories the slot points to.
    ///
    /// If the raw slot refers to the upper inventory, it will be returned
    /// unchanged and thus be suitable for getTopInventory().getItem(); if it
    /// refers to the lower inventory, the output will differ from the input
    /// and be suitable for getBottomInventory().getItem().
    pub fn convert_slot(&self, raw_slot: i32) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("(I)I");
        let val_1 = jni::objects::JValueGen::Int(raw_slot);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "convertSlot",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    /// Determine the type of the slot by its raw slot ID.
    ///
    /// If the type of the slot is unknown, then
    /// {@link InventoryType.SlotType#CONTAINER} will be returned.
    pub fn get_slot_type(
        &self,
        slot: i32,
    ) -> Result<crate::event::inventory::InventoryTypeSlotType<'mc>, Box<dyn std::error::Error>>
    {
        let sig = String::from("(I)Lorg/bukkit/event/inventory/InventoryType/SlotType;");
        let val_1 = jni::objects::JValueGen::Int(slot);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getSlotType",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::event::inventory::InventoryTypeSlotType::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Closes the inventory view.
    pub fn close(&self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "close", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Check the total number of slots in this view, combining the upper and
    /// lower inventories.
    ///
    /// Note though that it's possible for this to be greater than the sum of
    /// the two inventories if for example some slots are not being used.
    pub fn count_slots(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "countSlots", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    /// Sets an extra property of this inventory if supported by that
    /// inventory, for example the state of a progress bar.
    pub fn set_property(
        &self,
        prop: impl Into<crate::inventory::InventoryViewProperty<'mc>>,
        value: i32,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/inventory/InventoryView/Property;I)Z");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(prop.into().jni_object().clone())
        });
        let val_2 = jni::objects::JValueGen::Int(value);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setProperty",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Get the title of this inventory window.
    pub fn title(&self) -> Result<String, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/String;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getTitle", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    /// Get the original title of this inventory window, before any changes were
    /// made using {@link #setTitle(String)}.
    pub fn original_title(&self) -> Result<String, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/String;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getOriginalTitle",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    /// Sets the title of this inventory window to the specified title if the
    /// inventory window supports it.
    ///
    /// Note if the inventory does not support titles that can be changed (ie, it
    /// is not creatable or viewed by a player), then this method will throw an
    /// exception.
    pub fn set_title(&self, title: impl Into<String>) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/String;)V");
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(title.into())?,
        ));
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setTitle",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::inventory::InventoryView<'mc>> for MerchantView<'mc> {
    fn into(self) -> crate::inventory::InventoryView<'mc> {
        crate::inventory::InventoryView::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting MerchantView into crate::inventory::InventoryView")
    }
}
#[repr(C)]
pub struct BeaconView<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for BeaconView<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for BeaconView<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate BeaconView from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/inventory/view/BeaconView")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a BeaconView object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> BeaconView<'mc> {
    /// Gets the tier of the beacon
    ///
    /// Beacon tier is deduced by the height of the pyramid the beacon is
    /// standing on. The level of the beacon is 0 unless the beacon is activated.
    pub fn tier(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getTier", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    /// Gets the primary effect of the beacon.
    ///
    /// If the beacon level is high enough where the primary effect can be
    /// upgraded to level two, e.g. Speed 2. Instead of
    /// {@link #getSecondaryEffect()} being null it {@link #getSecondaryEffect()}
    /// returns the same {@link PotionEffectType} as this method.
    pub fn primary_effect(
        &self,
    ) -> Result<Option<crate::potion::PotionEffectType<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/potion/PotionEffectType;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getPrimaryEffect",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {
            return Ok(None);
        }
        Ok(Some(crate::potion::PotionEffectType::from_raw(
            &self.jni_ref(),
            unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) },
        )?))
    }
    /// Gets the secondary effect of the beacon.
    ///
    /// If the beacon level is high enough where the primary effect can be
    /// upgraded to level two, e.g. Speed 2. The secondary effect will return the
    /// same effect as {@link #getPrimaryEffect()}.
    pub fn secondary_effect(
        &self,
    ) -> Result<Option<crate::potion::PotionEffectType<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/potion/PotionEffectType;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getSecondaryEffect",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {
            return Ok(None);
        }
        Ok(Some(crate::potion::PotionEffectType::from_raw(
            &self.jni_ref(),
            unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) },
        )?))
    }
    /// Sets the primary effect of the beacon, or null to clear
    ///
    /// The {@link PotionEffectType} provided must be one that is already within
    /// the beacon as a valid option.
    /// <ol>
    /// <li>{@link PotionEffectType#SPEED}
    /// <li>{@link PotionEffectType#HASTE}
    /// <li>{@link PotionEffectType#RESISTANCE}
    /// <li>{@link PotionEffectType#JUMP_BOOST}
    /// <li>{@link PotionEffectType#STRENGTH}
    /// <li>{@link PotionEffectType#REGENERATION}
    /// </ol>
    pub fn set_primary_effect(
        &self,
        effect: impl Into<crate::potion::PotionEffectType<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/potion/PotionEffectType;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(effect.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setPrimaryEffect",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Sets the secondary effect on this beacon, or null to clear. Note that
    /// tier must be &gt;= 4 and a primary effect must be set in order for this
    /// effect to be active.
    ///
    /// The {@link PotionEffectType} provided must be one that is already within
    /// the beacon as a valid option.
    /// <ol>
    /// <li>{@link PotionEffectType#SPEED}
    /// <li>{@link PotionEffectType#HASTE}
    /// <li>{@link PotionEffectType#RESISTANCE}
    /// <li>{@link PotionEffectType#JUMP_BOOST}
    /// <li>{@link PotionEffectType#STRENGTH}
    /// <li>{@link PotionEffectType#REGENERATION}
    /// </ol>
    pub fn set_secondary_effect(
        &self,
        effect: impl Into<crate::potion::PotionEffectType<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/potion/PotionEffectType;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(effect.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setSecondaryEffect",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Get the upper inventory involved in this transaction.
    pub fn top_inventory(
        &self,
    ) -> Result<crate::inventory::Inventory<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/inventory/Inventory;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getTopInventory", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::Inventory::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Get the lower inventory involved in this transaction.
    pub fn bottom_inventory(
        &self,
    ) -> Result<crate::inventory::Inventory<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/inventory/Inventory;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getBottomInventory",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::Inventory::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Get the player viewing.
    pub fn player(&self) -> Result<crate::entity::HumanEntity<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/entity/HumanEntity;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getPlayer", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::entity::HumanEntity::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Determine the type of inventory involved in the transaction. This
    /// indicates the window style being shown. It will never return PLAYER,
    /// since that is common to all windows.
    pub fn get_type(
        &self,
    ) -> Result<crate::event::inventory::InventoryType<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/event/inventory/InventoryType;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getType", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::event::inventory::InventoryType::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Sets one item in this inventory view by its raw slot ID.
    ///
    /// Note: If slot ID -999 is chosen, it may be expected that the item is
    /// dropped on the ground. This is not required behaviour, however.
    pub fn set_item(
        &self,
        slot: i32,
        item: impl Into<crate::inventory::ItemStack<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(ILorg/bukkit/inventory/ItemStack;)V");
        let val_1 = jni::objects::JValueGen::Int(slot);
        let val_2 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(item.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setItem",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
            ],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Gets one item in this inventory view by its raw slot ID.
    pub fn get_item(
        &self,
        slot: i32,
    ) -> Result<Option<crate::inventory::ItemStack<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("(I)Lorg/bukkit/inventory/ItemStack;");
        let val_1 = jni::objects::JValueGen::Int(slot);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getItem",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {
            return Ok(None);
        }
        Ok(Some(crate::inventory::ItemStack::from_raw(
            &self.jni_ref(),
            unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) },
        )?))
    }
    /// Sets the item on the cursor of one of the viewing players.
    pub fn set_cursor(
        &self,
        item: impl Into<crate::inventory::ItemStack<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/inventory/ItemStack;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(item.into().jni_object().clone())
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
    /// Get the item on the cursor of one of the viewing players.
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
    /// Gets the inventory corresponding to the given raw slot ID.
    /// If the slot ID is {@link #OUTSIDE} null will be returned, otherwise
    /// behaviour for illegal and negative slot IDs is undefined.
    /// May be used with {@link #convertSlot(int)} to directly index an
    /// underlying inventory.
    pub fn get_inventory(
        &self,
        raw_slot: i32,
    ) -> Result<Option<crate::inventory::Inventory<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("(I)Lorg/bukkit/inventory/Inventory;");
        let val_1 = jni::objects::JValueGen::Int(raw_slot);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getInventory",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
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
    /// Converts a raw slot ID into its local slot ID into whichever of the two
    /// inventories the slot points to.
    ///
    /// If the raw slot refers to the upper inventory, it will be returned
    /// unchanged and thus be suitable for getTopInventory().getItem(); if it
    /// refers to the lower inventory, the output will differ from the input
    /// and be suitable for getBottomInventory().getItem().
    pub fn convert_slot(&self, raw_slot: i32) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("(I)I");
        let val_1 = jni::objects::JValueGen::Int(raw_slot);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "convertSlot",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    /// Determine the type of the slot by its raw slot ID.
    ///
    /// If the type of the slot is unknown, then
    /// {@link InventoryType.SlotType#CONTAINER} will be returned.
    pub fn get_slot_type(
        &self,
        slot: i32,
    ) -> Result<crate::event::inventory::InventoryTypeSlotType<'mc>, Box<dyn std::error::Error>>
    {
        let sig = String::from("(I)Lorg/bukkit/event/inventory/InventoryType/SlotType;");
        let val_1 = jni::objects::JValueGen::Int(slot);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getSlotType",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::event::inventory::InventoryTypeSlotType::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Closes the inventory view.
    pub fn close(&self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "close", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Check the total number of slots in this view, combining the upper and
    /// lower inventories.
    ///
    /// Note though that it's possible for this to be greater than the sum of
    /// the two inventories if for example some slots are not being used.
    pub fn count_slots(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "countSlots", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    /// Sets an extra property of this inventory if supported by that
    /// inventory, for example the state of a progress bar.
    pub fn set_property(
        &self,
        prop: impl Into<crate::inventory::InventoryViewProperty<'mc>>,
        value: i32,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/inventory/InventoryView/Property;I)Z");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(prop.into().jni_object().clone())
        });
        let val_2 = jni::objects::JValueGen::Int(value);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setProperty",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Get the title of this inventory window.
    pub fn title(&self) -> Result<String, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/String;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getTitle", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    /// Get the original title of this inventory window, before any changes were
    /// made using {@link #setTitle(String)}.
    pub fn original_title(&self) -> Result<String, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/String;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getOriginalTitle",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    /// Sets the title of this inventory window to the specified title if the
    /// inventory window supports it.
    ///
    /// Note if the inventory does not support titles that can be changed (ie, it
    /// is not creatable or viewed by a player), then this method will throw an
    /// exception.
    pub fn set_title(&self, title: impl Into<String>) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/String;)V");
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(title.into())?,
        ));
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setTitle",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::inventory::InventoryView<'mc>> for BeaconView<'mc> {
    fn into(self) -> crate::inventory::InventoryView<'mc> {
        crate::inventory::InventoryView::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting BeaconView into crate::inventory::InventoryView")
    }
}
#[repr(C)]
pub struct AnvilView<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for AnvilView<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for AnvilView<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate AnvilView from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/inventory/view/AnvilView")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a AnvilView object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> AnvilView<'mc> {
    /// Gets the rename text specified within the anvil's text field.
    pub fn rename_text(&self) -> Result<Option<String>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/String;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getRenameText", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {
            return Ok(None);
        }
        Ok(Some(
            self.jni_ref()
                .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
                .to_string_lossy()
                .to_string(),
        ))
    }
    /// Gets the amount of items needed to repair.
    pub fn repair_item_count_cost(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getRepairItemCountCost",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    /// Gets the experience cost needed to repair.
    pub fn repair_cost(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getRepairCost", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    /// Gets the maximum repair cost needed to repair.
    pub fn maximum_repair_cost(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getMaximumRepairCost",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    /// Sets the amount of repair materials required to repair the item.
    pub fn set_repair_item_count_cost(
        &self,
        amount: i32,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(I)V");
        let val_1 = jni::objects::JValueGen::Int(amount);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setRepairItemCountCost",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Sets the repair cost in experience.
    pub fn set_repair_cost(&self, cost: i32) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(I)V");
        let val_1 = jni::objects::JValueGen::Int(cost);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setRepairCost",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Sets maximum repair cost in experience.
    pub fn set_maximum_repair_cost(&self, levels: i32) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(I)V");
        let val_1 = jni::objects::JValueGen::Int(levels);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setMaximumRepairCost",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Get the upper inventory involved in this transaction.
    pub fn top_inventory(
        &self,
    ) -> Result<crate::inventory::Inventory<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/inventory/Inventory;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getTopInventory", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::Inventory::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Get the lower inventory involved in this transaction.
    pub fn bottom_inventory(
        &self,
    ) -> Result<crate::inventory::Inventory<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/inventory/Inventory;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getBottomInventory",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::Inventory::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Get the player viewing.
    pub fn player(&self) -> Result<crate::entity::HumanEntity<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/entity/HumanEntity;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getPlayer", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::entity::HumanEntity::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Determine the type of inventory involved in the transaction. This
    /// indicates the window style being shown. It will never return PLAYER,
    /// since that is common to all windows.
    pub fn get_type(
        &self,
    ) -> Result<crate::event::inventory::InventoryType<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/event/inventory/InventoryType;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getType", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::event::inventory::InventoryType::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Sets one item in this inventory view by its raw slot ID.
    ///
    /// Note: If slot ID -999 is chosen, it may be expected that the item is
    /// dropped on the ground. This is not required behaviour, however.
    pub fn set_item(
        &self,
        slot: i32,
        item: impl Into<crate::inventory::ItemStack<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(ILorg/bukkit/inventory/ItemStack;)V");
        let val_1 = jni::objects::JValueGen::Int(slot);
        let val_2 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(item.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setItem",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
            ],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Gets one item in this inventory view by its raw slot ID.
    pub fn get_item(
        &self,
        slot: i32,
    ) -> Result<Option<crate::inventory::ItemStack<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("(I)Lorg/bukkit/inventory/ItemStack;");
        let val_1 = jni::objects::JValueGen::Int(slot);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getItem",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {
            return Ok(None);
        }
        Ok(Some(crate::inventory::ItemStack::from_raw(
            &self.jni_ref(),
            unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) },
        )?))
    }
    /// Sets the item on the cursor of one of the viewing players.
    pub fn set_cursor(
        &self,
        item: impl Into<crate::inventory::ItemStack<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/inventory/ItemStack;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(item.into().jni_object().clone())
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
    /// Get the item on the cursor of one of the viewing players.
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
    /// Gets the inventory corresponding to the given raw slot ID.
    /// If the slot ID is {@link #OUTSIDE} null will be returned, otherwise
    /// behaviour for illegal and negative slot IDs is undefined.
    /// May be used with {@link #convertSlot(int)} to directly index an
    /// underlying inventory.
    pub fn get_inventory(
        &self,
        raw_slot: i32,
    ) -> Result<Option<crate::inventory::Inventory<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("(I)Lorg/bukkit/inventory/Inventory;");
        let val_1 = jni::objects::JValueGen::Int(raw_slot);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getInventory",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
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
    /// Converts a raw slot ID into its local slot ID into whichever of the two
    /// inventories the slot points to.
    ///
    /// If the raw slot refers to the upper inventory, it will be returned
    /// unchanged and thus be suitable for getTopInventory().getItem(); if it
    /// refers to the lower inventory, the output will differ from the input
    /// and be suitable for getBottomInventory().getItem().
    pub fn convert_slot(&self, raw_slot: i32) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("(I)I");
        let val_1 = jni::objects::JValueGen::Int(raw_slot);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "convertSlot",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    /// Determine the type of the slot by its raw slot ID.
    ///
    /// If the type of the slot is unknown, then
    /// {@link InventoryType.SlotType#CONTAINER} will be returned.
    pub fn get_slot_type(
        &self,
        slot: i32,
    ) -> Result<crate::event::inventory::InventoryTypeSlotType<'mc>, Box<dyn std::error::Error>>
    {
        let sig = String::from("(I)Lorg/bukkit/event/inventory/InventoryType/SlotType;");
        let val_1 = jni::objects::JValueGen::Int(slot);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getSlotType",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::event::inventory::InventoryTypeSlotType::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Closes the inventory view.
    pub fn close(&self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "close", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Check the total number of slots in this view, combining the upper and
    /// lower inventories.
    ///
    /// Note though that it's possible for this to be greater than the sum of
    /// the two inventories if for example some slots are not being used.
    pub fn count_slots(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "countSlots", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    /// Sets an extra property of this inventory if supported by that
    /// inventory, for example the state of a progress bar.
    pub fn set_property(
        &self,
        prop: impl Into<crate::inventory::InventoryViewProperty<'mc>>,
        value: i32,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/inventory/InventoryView/Property;I)Z");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(prop.into().jni_object().clone())
        });
        let val_2 = jni::objects::JValueGen::Int(value);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setProperty",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Get the title of this inventory window.
    pub fn title(&self) -> Result<String, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/String;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getTitle", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    /// Get the original title of this inventory window, before any changes were
    /// made using {@link #setTitle(String)}.
    pub fn original_title(&self) -> Result<String, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/String;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getOriginalTitle",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    /// Sets the title of this inventory window to the specified title if the
    /// inventory window supports it.
    ///
    /// Note if the inventory does not support titles that can be changed (ie, it
    /// is not creatable or viewed by a player), then this method will throw an
    /// exception.
    pub fn set_title(&self, title: impl Into<String>) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/String;)V");
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(title.into())?,
        ));
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setTitle",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::inventory::InventoryView<'mc>> for AnvilView<'mc> {
    fn into(self) -> crate::inventory::InventoryView<'mc> {
        crate::inventory::InventoryView::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting AnvilView into crate::inventory::InventoryView")
    }
}
#[repr(C)]
pub struct StonecutterView<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for StonecutterView<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for StonecutterView<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate StonecutterView from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/inventory/view/StonecutterView")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a StonecutterView object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> StonecutterView<'mc> {
    /// Gets the current index of the selected recipe.
    pub fn selected_recipe_index(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getSelectedRecipeIndex",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    /// Gets a copy of all recipes currently available to the player.
    pub fn recipes(
        &self,
    ) -> Result<Vec<crate::inventory::StonecuttingRecipe<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/List;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getRecipes", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let list = blackboxmc_java::util::JavaList::from_raw(&self.jni_ref(), res.l()?)?;
        let iter = list.iterator()?;
        while iter.has_next()? {
            let obj = iter.next()?;
            new_vec.push(crate::inventory::StonecuttingRecipe::from_raw(
                &self.jni_ref(),
                obj,
            )?);
        }
        Ok(new_vec)
    }
    /// Gets the amount of recipes currently available.
    pub fn recipe_amount(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getRecipeAmount", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    /// Get the upper inventory involved in this transaction.
    pub fn top_inventory(
        &self,
    ) -> Result<crate::inventory::Inventory<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/inventory/Inventory;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getTopInventory", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::Inventory::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Get the lower inventory involved in this transaction.
    pub fn bottom_inventory(
        &self,
    ) -> Result<crate::inventory::Inventory<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/inventory/Inventory;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getBottomInventory",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::Inventory::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Get the player viewing.
    pub fn player(&self) -> Result<crate::entity::HumanEntity<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/entity/HumanEntity;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getPlayer", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::entity::HumanEntity::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Determine the type of inventory involved in the transaction. This
    /// indicates the window style being shown. It will never return PLAYER,
    /// since that is common to all windows.
    pub fn get_type(
        &self,
    ) -> Result<crate::event::inventory::InventoryType<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/event/inventory/InventoryType;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getType", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::event::inventory::InventoryType::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Sets one item in this inventory view by its raw slot ID.
    ///
    /// Note: If slot ID -999 is chosen, it may be expected that the item is
    /// dropped on the ground. This is not required behaviour, however.
    pub fn set_item(
        &self,
        slot: i32,
        item: impl Into<crate::inventory::ItemStack<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(ILorg/bukkit/inventory/ItemStack;)V");
        let val_1 = jni::objects::JValueGen::Int(slot);
        let val_2 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(item.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setItem",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
            ],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Gets one item in this inventory view by its raw slot ID.
    pub fn get_item(
        &self,
        slot: i32,
    ) -> Result<Option<crate::inventory::ItemStack<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("(I)Lorg/bukkit/inventory/ItemStack;");
        let val_1 = jni::objects::JValueGen::Int(slot);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getItem",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {
            return Ok(None);
        }
        Ok(Some(crate::inventory::ItemStack::from_raw(
            &self.jni_ref(),
            unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) },
        )?))
    }
    /// Sets the item on the cursor of one of the viewing players.
    pub fn set_cursor(
        &self,
        item: impl Into<crate::inventory::ItemStack<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/inventory/ItemStack;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(item.into().jni_object().clone())
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
    /// Get the item on the cursor of one of the viewing players.
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
    /// Gets the inventory corresponding to the given raw slot ID.
    /// If the slot ID is {@link #OUTSIDE} null will be returned, otherwise
    /// behaviour for illegal and negative slot IDs is undefined.
    /// May be used with {@link #convertSlot(int)} to directly index an
    /// underlying inventory.
    pub fn get_inventory(
        &self,
        raw_slot: i32,
    ) -> Result<Option<crate::inventory::Inventory<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("(I)Lorg/bukkit/inventory/Inventory;");
        let val_1 = jni::objects::JValueGen::Int(raw_slot);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getInventory",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
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
    /// Converts a raw slot ID into its local slot ID into whichever of the two
    /// inventories the slot points to.
    ///
    /// If the raw slot refers to the upper inventory, it will be returned
    /// unchanged and thus be suitable for getTopInventory().getItem(); if it
    /// refers to the lower inventory, the output will differ from the input
    /// and be suitable for getBottomInventory().getItem().
    pub fn convert_slot(&self, raw_slot: i32) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("(I)I");
        let val_1 = jni::objects::JValueGen::Int(raw_slot);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "convertSlot",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    /// Determine the type of the slot by its raw slot ID.
    ///
    /// If the type of the slot is unknown, then
    /// {@link InventoryType.SlotType#CONTAINER} will be returned.
    pub fn get_slot_type(
        &self,
        slot: i32,
    ) -> Result<crate::event::inventory::InventoryTypeSlotType<'mc>, Box<dyn std::error::Error>>
    {
        let sig = String::from("(I)Lorg/bukkit/event/inventory/InventoryType/SlotType;");
        let val_1 = jni::objects::JValueGen::Int(slot);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getSlotType",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::event::inventory::InventoryTypeSlotType::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Closes the inventory view.
    pub fn close(&self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "close", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Check the total number of slots in this view, combining the upper and
    /// lower inventories.
    ///
    /// Note though that it's possible for this to be greater than the sum of
    /// the two inventories if for example some slots are not being used.
    pub fn count_slots(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "countSlots", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    /// Sets an extra property of this inventory if supported by that
    /// inventory, for example the state of a progress bar.
    pub fn set_property(
        &self,
        prop: impl Into<crate::inventory::InventoryViewProperty<'mc>>,
        value: i32,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/inventory/InventoryView/Property;I)Z");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(prop.into().jni_object().clone())
        });
        let val_2 = jni::objects::JValueGen::Int(value);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setProperty",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Get the title of this inventory window.
    pub fn title(&self) -> Result<String, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/String;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getTitle", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    /// Get the original title of this inventory window, before any changes were
    /// made using {@link #setTitle(String)}.
    pub fn original_title(&self) -> Result<String, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/String;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getOriginalTitle",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    /// Sets the title of this inventory window to the specified title if the
    /// inventory window supports it.
    ///
    /// Note if the inventory does not support titles that can be changed (ie, it
    /// is not creatable or viewed by a player), then this method will throw an
    /// exception.
    pub fn set_title(&self, title: impl Into<String>) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/String;)V");
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(title.into())?,
        ));
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setTitle",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::inventory::InventoryView<'mc>> for StonecutterView<'mc> {
    fn into(self) -> crate::inventory::InventoryView<'mc> {
        crate::inventory::InventoryView::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting StonecutterView into crate::inventory::InventoryView")
    }
}
#[repr(C)]
pub struct BrewingStandView<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for BrewingStandView<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for BrewingStandView<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate BrewingStandView from null object.").into(),
            );
        }
        let (valid, name) =
            env.validate_name(&obj, "org/bukkit/inventory/view/BrewingStandView")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a BrewingStandView object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> BrewingStandView<'mc> {
    /// Gets the fuel level of this brewing stand.
    ///
    /// The default maximum fuel level in minecraft is 20.
    pub fn fuel_level(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getFuelLevel", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    /// Gets the amount of brewing ticks left.
    pub fn brewing_ticks(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getBrewingTicks", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    /// Sets the fuel level left.
    pub fn set_fuel_level(&self, level: i32) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(I)V");
        let val_1 = jni::objects::JValueGen::Int(level);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setFuelLevel",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Sets the brewing ticks left.
    pub fn set_brewing_ticks(&self, ticks: i32) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(I)V");
        let val_1 = jni::objects::JValueGen::Int(ticks);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setBrewingTicks",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Get the upper inventory involved in this transaction.
    pub fn top_inventory(
        &self,
    ) -> Result<crate::inventory::Inventory<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/inventory/Inventory;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getTopInventory", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::Inventory::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Get the lower inventory involved in this transaction.
    pub fn bottom_inventory(
        &self,
    ) -> Result<crate::inventory::Inventory<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/inventory/Inventory;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getBottomInventory",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::Inventory::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Get the player viewing.
    pub fn player(&self) -> Result<crate::entity::HumanEntity<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/entity/HumanEntity;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getPlayer", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::entity::HumanEntity::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Determine the type of inventory involved in the transaction. This
    /// indicates the window style being shown. It will never return PLAYER,
    /// since that is common to all windows.
    pub fn get_type(
        &self,
    ) -> Result<crate::event::inventory::InventoryType<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/event/inventory/InventoryType;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getType", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::event::inventory::InventoryType::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Sets one item in this inventory view by its raw slot ID.
    ///
    /// Note: If slot ID -999 is chosen, it may be expected that the item is
    /// dropped on the ground. This is not required behaviour, however.
    pub fn set_item(
        &self,
        slot: i32,
        item: impl Into<crate::inventory::ItemStack<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(ILorg/bukkit/inventory/ItemStack;)V");
        let val_1 = jni::objects::JValueGen::Int(slot);
        let val_2 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(item.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setItem",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
            ],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Gets one item in this inventory view by its raw slot ID.
    pub fn get_item(
        &self,
        slot: i32,
    ) -> Result<Option<crate::inventory::ItemStack<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("(I)Lorg/bukkit/inventory/ItemStack;");
        let val_1 = jni::objects::JValueGen::Int(slot);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getItem",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {
            return Ok(None);
        }
        Ok(Some(crate::inventory::ItemStack::from_raw(
            &self.jni_ref(),
            unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) },
        )?))
    }
    /// Sets the item on the cursor of one of the viewing players.
    pub fn set_cursor(
        &self,
        item: impl Into<crate::inventory::ItemStack<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/inventory/ItemStack;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(item.into().jni_object().clone())
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
    /// Get the item on the cursor of one of the viewing players.
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
    /// Gets the inventory corresponding to the given raw slot ID.
    /// If the slot ID is {@link #OUTSIDE} null will be returned, otherwise
    /// behaviour for illegal and negative slot IDs is undefined.
    /// May be used with {@link #convertSlot(int)} to directly index an
    /// underlying inventory.
    pub fn get_inventory(
        &self,
        raw_slot: i32,
    ) -> Result<Option<crate::inventory::Inventory<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("(I)Lorg/bukkit/inventory/Inventory;");
        let val_1 = jni::objects::JValueGen::Int(raw_slot);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getInventory",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
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
    /// Converts a raw slot ID into its local slot ID into whichever of the two
    /// inventories the slot points to.
    ///
    /// If the raw slot refers to the upper inventory, it will be returned
    /// unchanged and thus be suitable for getTopInventory().getItem(); if it
    /// refers to the lower inventory, the output will differ from the input
    /// and be suitable for getBottomInventory().getItem().
    pub fn convert_slot(&self, raw_slot: i32) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("(I)I");
        let val_1 = jni::objects::JValueGen::Int(raw_slot);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "convertSlot",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    /// Determine the type of the slot by its raw slot ID.
    ///
    /// If the type of the slot is unknown, then
    /// {@link InventoryType.SlotType#CONTAINER} will be returned.
    pub fn get_slot_type(
        &self,
        slot: i32,
    ) -> Result<crate::event::inventory::InventoryTypeSlotType<'mc>, Box<dyn std::error::Error>>
    {
        let sig = String::from("(I)Lorg/bukkit/event/inventory/InventoryType/SlotType;");
        let val_1 = jni::objects::JValueGen::Int(slot);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getSlotType",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::event::inventory::InventoryTypeSlotType::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Closes the inventory view.
    pub fn close(&self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "close", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Check the total number of slots in this view, combining the upper and
    /// lower inventories.
    ///
    /// Note though that it's possible for this to be greater than the sum of
    /// the two inventories if for example some slots are not being used.
    pub fn count_slots(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "countSlots", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    /// Sets an extra property of this inventory if supported by that
    /// inventory, for example the state of a progress bar.
    pub fn set_property(
        &self,
        prop: impl Into<crate::inventory::InventoryViewProperty<'mc>>,
        value: i32,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/inventory/InventoryView/Property;I)Z");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(prop.into().jni_object().clone())
        });
        let val_2 = jni::objects::JValueGen::Int(value);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setProperty",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Get the title of this inventory window.
    pub fn title(&self) -> Result<String, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/String;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getTitle", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    /// Get the original title of this inventory window, before any changes were
    /// made using {@link #setTitle(String)}.
    pub fn original_title(&self) -> Result<String, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/String;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getOriginalTitle",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    /// Sets the title of this inventory window to the specified title if the
    /// inventory window supports it.
    ///
    /// Note if the inventory does not support titles that can be changed (ie, it
    /// is not creatable or viewed by a player), then this method will throw an
    /// exception.
    pub fn set_title(&self, title: impl Into<String>) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/String;)V");
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(title.into())?,
        ));
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setTitle",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::inventory::InventoryView<'mc>> for BrewingStandView<'mc> {
    fn into(self) -> crate::inventory::InventoryView<'mc> {
        crate::inventory::InventoryView::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting BrewingStandView into crate::inventory::InventoryView")
    }
}
#[repr(C)]
pub struct LecternView<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for LecternView<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for LecternView<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate LecternView from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/inventory/view/LecternView")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a LecternView object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> LecternView<'mc> {
    /// Gets the page that the LecternView is on.
    pub fn page(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getPage", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    /// Sets the page of the lectern book.
    pub fn set_page(&self, page: i32) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(I)V");
        let val_1 = jni::objects::JValueGen::Int(page);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setPage",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Get the upper inventory involved in this transaction.
    pub fn top_inventory(
        &self,
    ) -> Result<crate::inventory::Inventory<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/inventory/Inventory;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getTopInventory", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::Inventory::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Get the lower inventory involved in this transaction.
    pub fn bottom_inventory(
        &self,
    ) -> Result<crate::inventory::Inventory<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/inventory/Inventory;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getBottomInventory",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::Inventory::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Get the player viewing.
    pub fn player(&self) -> Result<crate::entity::HumanEntity<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/entity/HumanEntity;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getPlayer", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::entity::HumanEntity::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Determine the type of inventory involved in the transaction. This
    /// indicates the window style being shown. It will never return PLAYER,
    /// since that is common to all windows.
    pub fn get_type(
        &self,
    ) -> Result<crate::event::inventory::InventoryType<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/event/inventory/InventoryType;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getType", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::event::inventory::InventoryType::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Sets one item in this inventory view by its raw slot ID.
    ///
    /// Note: If slot ID -999 is chosen, it may be expected that the item is
    /// dropped on the ground. This is not required behaviour, however.
    pub fn set_item(
        &self,
        slot: i32,
        item: impl Into<crate::inventory::ItemStack<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(ILorg/bukkit/inventory/ItemStack;)V");
        let val_1 = jni::objects::JValueGen::Int(slot);
        let val_2 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(item.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setItem",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
            ],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Gets one item in this inventory view by its raw slot ID.
    pub fn get_item(
        &self,
        slot: i32,
    ) -> Result<Option<crate::inventory::ItemStack<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("(I)Lorg/bukkit/inventory/ItemStack;");
        let val_1 = jni::objects::JValueGen::Int(slot);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getItem",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {
            return Ok(None);
        }
        Ok(Some(crate::inventory::ItemStack::from_raw(
            &self.jni_ref(),
            unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) },
        )?))
    }
    /// Sets the item on the cursor of one of the viewing players.
    pub fn set_cursor(
        &self,
        item: impl Into<crate::inventory::ItemStack<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/inventory/ItemStack;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(item.into().jni_object().clone())
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
    /// Get the item on the cursor of one of the viewing players.
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
    /// Gets the inventory corresponding to the given raw slot ID.
    /// If the slot ID is {@link #OUTSIDE} null will be returned, otherwise
    /// behaviour for illegal and negative slot IDs is undefined.
    /// May be used with {@link #convertSlot(int)} to directly index an
    /// underlying inventory.
    pub fn get_inventory(
        &self,
        raw_slot: i32,
    ) -> Result<Option<crate::inventory::Inventory<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("(I)Lorg/bukkit/inventory/Inventory;");
        let val_1 = jni::objects::JValueGen::Int(raw_slot);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getInventory",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
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
    /// Converts a raw slot ID into its local slot ID into whichever of the two
    /// inventories the slot points to.
    ///
    /// If the raw slot refers to the upper inventory, it will be returned
    /// unchanged and thus be suitable for getTopInventory().getItem(); if it
    /// refers to the lower inventory, the output will differ from the input
    /// and be suitable for getBottomInventory().getItem().
    pub fn convert_slot(&self, raw_slot: i32) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("(I)I");
        let val_1 = jni::objects::JValueGen::Int(raw_slot);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "convertSlot",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    /// Determine the type of the slot by its raw slot ID.
    ///
    /// If the type of the slot is unknown, then
    /// {@link InventoryType.SlotType#CONTAINER} will be returned.
    pub fn get_slot_type(
        &self,
        slot: i32,
    ) -> Result<crate::event::inventory::InventoryTypeSlotType<'mc>, Box<dyn std::error::Error>>
    {
        let sig = String::from("(I)Lorg/bukkit/event/inventory/InventoryType/SlotType;");
        let val_1 = jni::objects::JValueGen::Int(slot);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getSlotType",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::event::inventory::InventoryTypeSlotType::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Closes the inventory view.
    pub fn close(&self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "close", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Check the total number of slots in this view, combining the upper and
    /// lower inventories.
    ///
    /// Note though that it's possible for this to be greater than the sum of
    /// the two inventories if for example some slots are not being used.
    pub fn count_slots(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "countSlots", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    /// Sets an extra property of this inventory if supported by that
    /// inventory, for example the state of a progress bar.
    pub fn set_property(
        &self,
        prop: impl Into<crate::inventory::InventoryViewProperty<'mc>>,
        value: i32,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/inventory/InventoryView/Property;I)Z");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(prop.into().jni_object().clone())
        });
        let val_2 = jni::objects::JValueGen::Int(value);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setProperty",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Get the title of this inventory window.
    pub fn title(&self) -> Result<String, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/String;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getTitle", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    /// Get the original title of this inventory window, before any changes were
    /// made using {@link #setTitle(String)}.
    pub fn original_title(&self) -> Result<String, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/String;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getOriginalTitle",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    /// Sets the title of this inventory window to the specified title if the
    /// inventory window supports it.
    ///
    /// Note if the inventory does not support titles that can be changed (ie, it
    /// is not creatable or viewed by a player), then this method will throw an
    /// exception.
    pub fn set_title(&self, title: impl Into<String>) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/String;)V");
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(title.into())?,
        ));
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setTitle",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::inventory::InventoryView<'mc>> for LecternView<'mc> {
    fn into(self) -> crate::inventory::InventoryView<'mc> {
        crate::inventory::InventoryView::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting LecternView into crate::inventory::InventoryView")
    }
}
#[repr(C)]
pub struct FurnaceView<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for FurnaceView<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for FurnaceView<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate FurnaceView from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/inventory/view/FurnaceView")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a FurnaceView object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> FurnaceView<'mc> {
    /// The cook time for this view.
    ///
    /// See {@link Furnace#getCookTime()} for more information.
    pub fn cook_time(&self) -> Result<f32, Box<dyn std::error::Error>> {
        let sig = String::from("()F");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getCookTime", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.f()?)
    }
    /// The total burn time for this view.
    ///
    /// See {@link Furnace#getBurnTime()} for more information.
    pub fn burn_time(&self) -> Result<f32, Box<dyn std::error::Error>> {
        let sig = String::from("()F");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getBurnTime", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.f()?)
    }
    /// Checks whether or not the furnace is burning
    pub fn is_burning(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isBurning", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Sets the cook time
    ///
    /// Setting cook time requires manipulation of both cookProgress and
    /// cookDuration. This method does a simple division to get total progress
    /// within the furnaces visual duration bar. For a clear visual effect
    /// (cookProgress / cookDuration) should return a number between 0 and 1
    /// inclusively.
    pub fn set_cook_time(
        &self,
        cook_progress: i32,
        cook_duration: i32,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(II)V");
        let val_1 = jni::objects::JValueGen::Int(cook_progress);
        let val_2 = jni::objects::JValueGen::Int(cook_duration);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setCookTime",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
            ],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Sets the burn time
    ///
    /// Setting burn time requires manipulation of both burnProgress and
    /// burnDuration. This method does a simple division to get total progress
    /// within the furnaces visual burning bar. For a clear visual effect
    /// (burnProgress / burnDuration) should return a number between 0 and 1
    /// inclusively.
    pub fn set_burn_time(
        &self,
        burn_progress: i32,
        burn_duration: i32,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(II)V");
        let val_1 = jni::objects::JValueGen::Int(burn_progress);
        let val_2 = jni::objects::JValueGen::Int(burn_duration);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setBurnTime",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
            ],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Get the upper inventory involved in this transaction.
    pub fn top_inventory(
        &self,
    ) -> Result<crate::inventory::Inventory<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/inventory/Inventory;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getTopInventory", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::Inventory::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Get the lower inventory involved in this transaction.
    pub fn bottom_inventory(
        &self,
    ) -> Result<crate::inventory::Inventory<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/inventory/Inventory;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getBottomInventory",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::Inventory::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Get the player viewing.
    pub fn player(&self) -> Result<crate::entity::HumanEntity<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/entity/HumanEntity;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getPlayer", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::entity::HumanEntity::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Determine the type of inventory involved in the transaction. This
    /// indicates the window style being shown. It will never return PLAYER,
    /// since that is common to all windows.
    pub fn get_type(
        &self,
    ) -> Result<crate::event::inventory::InventoryType<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/event/inventory/InventoryType;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getType", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::event::inventory::InventoryType::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Sets one item in this inventory view by its raw slot ID.
    ///
    /// Note: If slot ID -999 is chosen, it may be expected that the item is
    /// dropped on the ground. This is not required behaviour, however.
    pub fn set_item(
        &self,
        slot: i32,
        item: impl Into<crate::inventory::ItemStack<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(ILorg/bukkit/inventory/ItemStack;)V");
        let val_1 = jni::objects::JValueGen::Int(slot);
        let val_2 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(item.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setItem",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
            ],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Gets one item in this inventory view by its raw slot ID.
    pub fn get_item(
        &self,
        slot: i32,
    ) -> Result<Option<crate::inventory::ItemStack<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("(I)Lorg/bukkit/inventory/ItemStack;");
        let val_1 = jni::objects::JValueGen::Int(slot);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getItem",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {
            return Ok(None);
        }
        Ok(Some(crate::inventory::ItemStack::from_raw(
            &self.jni_ref(),
            unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) },
        )?))
    }
    /// Sets the item on the cursor of one of the viewing players.
    pub fn set_cursor(
        &self,
        item: impl Into<crate::inventory::ItemStack<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/inventory/ItemStack;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(item.into().jni_object().clone())
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
    /// Get the item on the cursor of one of the viewing players.
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
    /// Gets the inventory corresponding to the given raw slot ID.
    /// If the slot ID is {@link #OUTSIDE} null will be returned, otherwise
    /// behaviour for illegal and negative slot IDs is undefined.
    /// May be used with {@link #convertSlot(int)} to directly index an
    /// underlying inventory.
    pub fn get_inventory(
        &self,
        raw_slot: i32,
    ) -> Result<Option<crate::inventory::Inventory<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("(I)Lorg/bukkit/inventory/Inventory;");
        let val_1 = jni::objects::JValueGen::Int(raw_slot);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getInventory",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
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
    /// Converts a raw slot ID into its local slot ID into whichever of the two
    /// inventories the slot points to.
    ///
    /// If the raw slot refers to the upper inventory, it will be returned
    /// unchanged and thus be suitable for getTopInventory().getItem(); if it
    /// refers to the lower inventory, the output will differ from the input
    /// and be suitable for getBottomInventory().getItem().
    pub fn convert_slot(&self, raw_slot: i32) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("(I)I");
        let val_1 = jni::objects::JValueGen::Int(raw_slot);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "convertSlot",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    /// Determine the type of the slot by its raw slot ID.
    ///
    /// If the type of the slot is unknown, then
    /// {@link InventoryType.SlotType#CONTAINER} will be returned.
    pub fn get_slot_type(
        &self,
        slot: i32,
    ) -> Result<crate::event::inventory::InventoryTypeSlotType<'mc>, Box<dyn std::error::Error>>
    {
        let sig = String::from("(I)Lorg/bukkit/event/inventory/InventoryType/SlotType;");
        let val_1 = jni::objects::JValueGen::Int(slot);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getSlotType",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::event::inventory::InventoryTypeSlotType::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Closes the inventory view.
    pub fn close(&self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "close", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Check the total number of slots in this view, combining the upper and
    /// lower inventories.
    ///
    /// Note though that it's possible for this to be greater than the sum of
    /// the two inventories if for example some slots are not being used.
    pub fn count_slots(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "countSlots", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    /// Sets an extra property of this inventory if supported by that
    /// inventory, for example the state of a progress bar.
    pub fn set_property(
        &self,
        prop: impl Into<crate::inventory::InventoryViewProperty<'mc>>,
        value: i32,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/inventory/InventoryView/Property;I)Z");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(prop.into().jni_object().clone())
        });
        let val_2 = jni::objects::JValueGen::Int(value);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setProperty",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Get the title of this inventory window.
    pub fn title(&self) -> Result<String, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/String;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getTitle", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    /// Get the original title of this inventory window, before any changes were
    /// made using {@link #setTitle(String)}.
    pub fn original_title(&self) -> Result<String, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/String;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getOriginalTitle",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    /// Sets the title of this inventory window to the specified title if the
    /// inventory window supports it.
    ///
    /// Note if the inventory does not support titles that can be changed (ie, it
    /// is not creatable or viewed by a player), then this method will throw an
    /// exception.
    pub fn set_title(&self, title: impl Into<String>) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/String;)V");
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(title.into())?,
        ));
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setTitle",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::inventory::InventoryView<'mc>> for FurnaceView<'mc> {
    fn into(self) -> crate::inventory::InventoryView<'mc> {
        crate::inventory::InventoryView::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting FurnaceView into crate::inventory::InventoryView")
    }
}
