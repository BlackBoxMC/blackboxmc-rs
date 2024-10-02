#![allow(deprecated)]
use blackboxmc_general::JNIInstantiatable;
use blackboxmc_general::JNIRaw;
use color_eyre::eyre::Result;
pub enum InventoryType<'mc> {
    Chest { inner: InventoryTypeStruct<'mc> },
    Dispenser { inner: InventoryTypeStruct<'mc> },
    Dropper { inner: InventoryTypeStruct<'mc> },
    Furnace { inner: InventoryTypeStruct<'mc> },
    Workbench { inner: InventoryTypeStruct<'mc> },
    Crafting { inner: InventoryTypeStruct<'mc> },
    Enchanting { inner: InventoryTypeStruct<'mc> },
    Brewing { inner: InventoryTypeStruct<'mc> },
    Player { inner: InventoryTypeStruct<'mc> },
    Creative { inner: InventoryTypeStruct<'mc> },
    Merchant { inner: InventoryTypeStruct<'mc> },
    EnderChest { inner: InventoryTypeStruct<'mc> },
    Anvil { inner: InventoryTypeStruct<'mc> },
    Smithing { inner: InventoryTypeStruct<'mc> },
    Beacon { inner: InventoryTypeStruct<'mc> },
    Hopper { inner: InventoryTypeStruct<'mc> },
    ShulkerBox { inner: InventoryTypeStruct<'mc> },
    Barrel { inner: InventoryTypeStruct<'mc> },
    BlastFurnace { inner: InventoryTypeStruct<'mc> },
    Lectern { inner: InventoryTypeStruct<'mc> },
    Smoker { inner: InventoryTypeStruct<'mc> },
    Loom { inner: InventoryTypeStruct<'mc> },
    Cartography { inner: InventoryTypeStruct<'mc> },
    Grindstone { inner: InventoryTypeStruct<'mc> },
    Stonecutter { inner: InventoryTypeStruct<'mc> },
    Composter { inner: InventoryTypeStruct<'mc> },
    ChiseledBookshelf { inner: InventoryTypeStruct<'mc> },
    Jukebox { inner: InventoryTypeStruct<'mc> },
    Crafter { inner: InventoryTypeStruct<'mc> },
    SmithingNew { inner: InventoryTypeStruct<'mc> },
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
            InventoryType::Crafter { .. } => f.write_str("CRAFTER"),
            InventoryType::SmithingNew { .. } => f.write_str("SMITHING_NEW"),
        }
    }
}
impl<'mc> std::ops::Deref for InventoryType<'mc> {
    type Target = InventoryTypeStruct<'mc>;
    fn deref(&self) -> &<InventoryType<'mc> as std::ops::Deref>::Target {
        match self {
            InventoryType::Chest { inner } => inner,
            InventoryType::Dispenser { inner } => inner,
            InventoryType::Dropper { inner } => inner,
            InventoryType::Furnace { inner } => inner,
            InventoryType::Workbench { inner } => inner,
            InventoryType::Crafting { inner } => inner,
            InventoryType::Enchanting { inner } => inner,
            InventoryType::Brewing { inner } => inner,
            InventoryType::Player { inner } => inner,
            InventoryType::Creative { inner } => inner,
            InventoryType::Merchant { inner } => inner,
            InventoryType::EnderChest { inner } => inner,
            InventoryType::Anvil { inner } => inner,
            InventoryType::Smithing { inner } => inner,
            InventoryType::Beacon { inner } => inner,
            InventoryType::Hopper { inner } => inner,
            InventoryType::ShulkerBox { inner } => inner,
            InventoryType::Barrel { inner } => inner,
            InventoryType::BlastFurnace { inner } => inner,
            InventoryType::Lectern { inner } => inner,
            InventoryType::Smoker { inner } => inner,
            InventoryType::Loom { inner } => inner,
            InventoryType::Cartography { inner } => inner,
            InventoryType::Grindstone { inner } => inner,
            InventoryType::Stonecutter { inner } => inner,
            InventoryType::Composter { inner } => inner,
            InventoryType::ChiseledBookshelf { inner } => inner,
            InventoryType::Jukebox { inner } => inner,
            InventoryType::Crafter { inner } => inner,
            InventoryType::SmithingNew { inner } => inner,
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
            "CRAFTER" => Ok(InventoryType::Crafter {
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
            Self::Crafter { inner } => inner.0.clone(),
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
            Self::Crafter { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
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
                "CRAFTER" => Ok(InventoryType::Crafter {
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
    pub fn values(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<crate::event::inventory::InventoryType<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/event/inventory/InventoryType;");
        let cls = jni.find_class("org/bukkit/event/inventory/InventoryType");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "values", sig.as_str(), vec![]);
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        crate::event::inventory::InventoryType::from_raw(&jni, obj)
    }

    pub fn default_size(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getDefaultSize", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }

    pub fn default_title(&self) -> Result<String, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/String;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getDefaultTitle", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    /// Gets the corresponding {@link MenuType} of this InventoryType.
    ///
    /// Not all InventoryType correspond to a {@link MenuType}. These
    /// InventoryTypes are also not creatable. If this method returns null,
    /// {@link #isCreatable()} will return false, with the exception of
    /// {@link #MERCHANT}.
    ///
    /// As well as not necessarily corresponding to a {@link MenuType} some
    /// InventoryType correspond to the same {@link MenuType}, including:
    /// <ul>
    /// <li>Dropper, Dispenser
    /// <li>ShulkerBox, Barrel, Chest
    /// </ul>
    pub fn menu_type(
        &self,
    ) -> Result<Option<crate::inventory::MenuType<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/inventory/MenuType;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getMenuType", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {
            return Ok(None);
        }
        Ok(Some(crate::inventory::MenuType::from_raw(
            &self.jni_ref(),
            unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) },
        )?))
    }
    /// Denotes that this InventoryType can be created via the normal
    /// {@link org.bukkit.Bukkit#createInventory} methods.
    pub fn is_creatable(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isCreatable", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
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
impl<'mc> std::ops::Deref for InventoryTypeSlotType<'mc> {
    type Target = InventoryTypeSlotTypeStruct<'mc>;
    fn deref(&self) -> &<InventoryTypeSlotType<'mc> as std::ops::Deref>::Target {
        match self {
            InventoryTypeSlotType::Result { inner } => inner,
            InventoryTypeSlotType::Crafting { inner } => inner,
            InventoryTypeSlotType::Armor { inner } => inner,
            InventoryTypeSlotType::Container { inner } => inner,
            InventoryTypeSlotType::Quickbar { inner } => inner,
            InventoryTypeSlotType::Outside { inner } => inner,
            InventoryTypeSlotType::Fuel { inner } => inner,
        }
    }
}

impl<'mc> InventoryTypeSlotType<'mc> {
    pub fn value_of(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<String>,
    ) -> Result<InventoryTypeSlotType<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(env.new_string(arg0.into())?);
        let cls = env.find_class("org/bukkit/event/inventory/InventoryType/SlotType");
        let cls = env.translate_error_with_class(cls)?;
        let res = env.call_static_method(
            cls,
            "valueOf",
            "(Ljava/lang/String;)Lorg/bukkit/event/inventory/InventoryType/SlotType;",
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
            env.validate_name(&obj, "org/bukkit/event/inventory/InventoryType/SlotType")?;
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
            env.validate_name(&obj, "org/bukkit/event/inventory/InventoryType/SlotType")?;
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
    ) -> Result<crate::event::inventory::InventoryTypeSlotType<'mc>, Box<dyn std::error::Error>>
    {
        let sig = String::from("()Lorg/bukkit/event/inventory/InventoryType/SlotType;");
        let cls = jni.find_class("org/bukkit/event/inventory/InventoryType/SlotType");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "values", sig.as_str(), vec![]);
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        crate::event::inventory::InventoryTypeSlotType::from_raw(&jni, obj)
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
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
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        view: impl Into<crate::inventory::InventoryView<'mc>>,
        val_type: impl Into<crate::event::inventory::InventoryTypeSlotType<'mc>>,
        slot: i32,
        click: impl Into<crate::event::inventory::ClickType<'mc>>,
        action: impl Into<crate::event::inventory::InventoryAction<'mc>>,
        key: std::option::Option<i32>,
    ) -> Result<crate::event::inventory::InventoryClickEvent<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/inventory/InventoryView;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(view.into().jni_object().clone())
        });
        args.push(val_1);
        sig += "Lorg/bukkit/event/inventory/InventoryType/SlotType;";
        let val_2 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(val_type.into().jni_object().clone())
        });
        args.push(val_2);
        sig += "I";
        let val_3 = jni::objects::JValueGen::Int(slot);
        args.push(val_3);
        sig += "Lorg/bukkit/event/inventory/ClickType;";
        let val_4 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(click.into().jni_object().clone())
        });
        args.push(val_4);
        sig += "Lorg/bukkit/event/inventory/InventoryAction;";
        let val_5 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(action.into().jni_object().clone())
        });
        args.push(val_5);
        if let Some(a) = key {
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
    /// Gets the type of slot that was clicked.
    pub fn slot_type(
        &self,
    ) -> Result<crate::event::inventory::InventoryTypeSlotType<'mc>, Box<dyn std::error::Error>>
    {
        let sig = String::from("()Lorg/bukkit/event/inventory/InventoryType/SlotType;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getSlotType", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::event::inventory::InventoryTypeSlotType::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Gets the current ItemStack on the cursor.
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
    /// Gets the ItemStack currently in the clicked slot.
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
    /// Gets whether or not the ClickType for this event represents a right
    /// click.
    pub fn is_right_click(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isRightClick", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Gets whether or not the ClickType for this event represents a left
    /// click.
    pub fn is_left_click(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isLeftClick", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Gets whether the ClickType for this event indicates that the key was
    /// pressed down when the click was made.
    pub fn is_shift_click(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isShiftClick", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    #[deprecated]
    /// Sets the item on the cursor.
    pub fn set_cursor(
        &self,
        stack: impl Into<crate::inventory::ItemStack<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/inventory/ItemStack;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(stack.into().jni_object().clone())
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
    /// Sets the ItemStack currently in the clicked slot.
    pub fn set_current_item(
        &self,
        stack: impl Into<crate::inventory::ItemStack<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/inventory/ItemStack;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(stack.into().jni_object().clone())
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
    /// Gets the inventory corresponding to the clicked slot.
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
    /// The slot number that was clicked, ready for passing to
    /// {@link Inventory#getItem(int)}. Note that there may be two slots with
    /// the same slot number, since a view links two different inventories.
    pub fn slot(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getSlot", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    /// The raw slot number clicked, ready for passing to {@link InventoryView
    /// #getItem(int)} This slot number is unique for the view.
    pub fn raw_slot(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getRawSlot", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    /// If the ClickType is NUMBER_KEY, this method will return the index of
    /// the pressed key (0-8).
    pub fn hotbar_button(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getHotbarButton", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    /// Gets the InventoryAction that triggered this event.
    ///
    /// This action cannot be changed, and represents what the normal outcome
    /// of the event will be. To change the behavior of this
    /// InventoryClickEvent, changes must be manually applied.
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
    /// Gets the ClickType for this event.
    ///
    /// This is insulated against changes to the inventory by other plugins.
    pub fn click(
        &self,
    ) -> Result<crate::event::inventory::ClickType<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/event/inventory/ClickType;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getClick", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::event::inventory::ClickType::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
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
        let cls = jni.find_class("org/bukkit/event/inventory/InventoryClickEvent");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "getHandlerList", sig.as_str(), vec![]);
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        crate::event::HandlerList::from_raw(&jni, obj)
    }
    // SUPER CLASS: org.bukkit.event.inventory.InventoryInteractEvent ( ['getSlotType', 'getCursor', 'getCurrentItem', 'isRightClick', 'isLeftClick', 'isShiftClick', 'setCursor', 'setCurrentItem', 'getClickedInventory', 'getSlot', 'getRawSlot', 'getHotbarButton', 'getAction', 'getClick', 'getHandlers', 'getHandlerList'])
    /// Gets the player who performed the click.
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
    /// Sets the result of this event. This will change whether or not this
    /// event is considered cancelled.
    pub fn set_result(
        &self,
        new_result: impl Into<crate::event::EventResult<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone =
            crate::event::inventory::InventoryInteractEvent::from_raw(&self.0, unsafe {
                jni::objects::JObject::from_raw(self.1.clone())
            })?;
        let real: crate::event::inventory::InventoryInteractEvent = temp_clone.into();
        real.set_result(new_result)
    }
    /// Gets the {@link org.bukkit.event.Event.Result} of this event. The Result describes the
    /// behavior that will be applied to the inventory in relation to this
    /// event.
    pub fn result(&self) -> Result<crate::event::EventResult<'mc>, Box<dyn std::error::Error>> {
        let temp_clone =
            crate::event::inventory::InventoryInteractEvent::from_raw(&self.0, unsafe {
                jni::objects::JObject::from_raw(self.1.clone())
            })?;
        let real: crate::event::inventory::InventoryInteractEvent = temp_clone.into();
        real.result()
    }
    /// Gets whether or not this event is cancelled. This is based off of the
    /// Result value returned by {@link #getResult()}.Result.ALLOW and
    /// Result.DEFAULT will result in a returned value of false, but
    /// Result.DENY will result in a returned value of true.
    ///
    /// {@inheritDoc}
    pub fn is_cancelled(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone =
            crate::event::inventory::InventoryInteractEvent::from_raw(&self.0, unsafe {
                jni::objects::JObject::from_raw(self.1.clone())
            })?;
        let real: crate::event::inventory::InventoryInteractEvent = temp_clone.into();
        real.is_cancelled()
    }
    /// Proxy method to {@link #setResult(org.bukkit.event.Event.Result)} for the Cancellable
    /// interface. {@link #setResult(org.bukkit.event.Event.Result)} is preferred, as it allows
    /// you to specify the Result beyond Result.DENY and Result.ALLOW.
    ///
    /// {@inheritDoc}
    pub fn set_cancelled(&self, to_cancel: bool) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone =
            crate::event::inventory::InventoryInteractEvent::from_raw(&self.0, unsafe {
                jni::objects::JObject::from_raw(self.1.clone())
            })?;
        let real: crate::event::inventory::InventoryInteractEvent = temp_clone.into();
        real.set_cancelled(to_cancel)
    }

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
        player: impl Into<crate::entity::Player<'mc>>,
        block: impl Into<crate::block::Block<'mc>>,
        item_type: impl Into<crate::Material<'mc>>,
        item_amount: i32,
        exp: i32,
    ) -> Result<crate::event::inventory::FurnaceExtractEvent<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from(
            "(Lorg/bukkit/entity/Player;Lorg/bukkit/block/Block;Lorg/bukkit/Material;II)V",
        );
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(player.into().jni_object().clone())
        });
        let val_2 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(block.into().jni_object().clone())
        });
        let val_3 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(item_type.into().jni_object().clone())
        });
        let val_4 = jni::objects::JValueGen::Int(item_amount);
        let val_5 = jni::objects::JValueGen::Int(exp);
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
    /// Get the player that triggered the event
    pub fn player(&self) -> Result<crate::entity::Player<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/entity/Player;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getPlayer", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::entity::Player::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Get the Material of the item being retrieved
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
    /// Get the item count being retrieved
    pub fn item_amount(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getItemAmount", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    // SUPER CLASS: org.bukkit.event.block.BlockExpEvent ( ['getPlayer', 'getItemType', 'getItemAmount'])
    /// Get the experience dropped by the block after the event has processed
    pub fn exp_to_drop(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::block::BlockExpEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::block::BlockExpEvent = temp_clone.into();
        real.exp_to_drop()
    }
    /// Set the amount of experience dropped by the block after the event has
    /// processed
    pub fn set_exp_to_drop(&self, exp: i32) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::event::block::BlockExpEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::block::BlockExpEvent = temp_clone.into();
        real.set_exp_to_drop(exp)
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
        crate::event::block::BlockExpEvent::handler_list(jni)
    }

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
        furnace: impl Into<crate::block::Block<'mc>>,
        source: impl Into<crate::inventory::ItemStack<'mc>>,
        result: impl Into<crate::inventory::ItemStack<'mc>>,
    ) -> Result<crate::event::inventory::FurnaceSmeltEvent<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/Block;Lorg/bukkit/inventory/ItemStack;Lorg/bukkit/inventory/ItemStack;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(furnace.into().jni_object().clone())
        });
        let val_2 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(source.into().jni_object().clone())
        });
        let val_3 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(result.into().jni_object().clone())
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
    // SUPER CLASS: org.bukkit.event.block.BlockCookEvent ( [])
    /// Gets the smelted ItemStack for this event
    pub fn source(&self) -> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::block::BlockCookEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::block::BlockCookEvent = temp_clone.into();
        real.source()
    }
    /// Gets the resultant ItemStack for this event
    pub fn result(&self) -> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::block::BlockCookEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::block::BlockCookEvent = temp_clone.into();
        real.result()
    }
    /// Sets the resultant ItemStack for this event
    pub fn set_result(
        &self,
        result: impl Into<crate::inventory::ItemStack<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::event::block::BlockCookEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::block::BlockCookEvent = temp_clone.into();
        real.set_result(result)
    }

    pub fn is_cancelled(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::block::BlockCookEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::block::BlockCookEvent = temp_clone.into();
        real.is_cancelled()
    }

    pub fn set_cancelled(&self, cancel: bool) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::event::block::BlockCookEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::block::BlockCookEvent = temp_clone.into();
        real.set_cancelled(cancel)
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
        crate::event::block::BlockCookEvent::handler_list(jni)
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
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        recipe: impl Into<crate::inventory::Recipe<'mc>>,
        what: impl Into<crate::inventory::InventoryView<'mc>>,
        val_type: impl Into<crate::event::inventory::InventoryTypeSlotType<'mc>>,
        slot: i32,
        click: impl Into<crate::event::inventory::ClickType<'mc>>,
        action: impl Into<crate::event::inventory::InventoryAction<'mc>>,
        key: std::option::Option<i32>,
    ) -> Result<crate::event::inventory::CraftItemEvent<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/inventory/Recipe;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(recipe.into().jni_object().clone())
        });
        args.push(val_1);
        sig += "Lorg/bukkit/inventory/InventoryView;";
        let val_2 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(what.into().jni_object().clone())
        });
        args.push(val_2);
        sig += "Lorg/bukkit/event/inventory/InventoryType/SlotType;";
        let val_3 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(val_type.into().jni_object().clone())
        });
        args.push(val_3);
        sig += "I";
        let val_4 = jni::objects::JValueGen::Int(slot);
        args.push(val_4);
        sig += "Lorg/bukkit/event/inventory/ClickType;";
        let val_5 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(click.into().jni_object().clone())
        });
        args.push(val_5);
        sig += "Lorg/bukkit/event/inventory/InventoryAction;";
        let val_6 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(action.into().jni_object().clone())
        });
        args.push(val_6);
        if let Some(a) = key {
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

    pub fn recipe(&self) -> Result<crate::inventory::Recipe<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/inventory/Recipe;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getRecipe", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::Recipe::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn inventory(
        &self,
    ) -> Result<crate::inventory::CraftingInventory<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/inventory/CraftingInventory;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getInventory", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::CraftingInventory::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    // SUPER CLASS: org.bukkit.event.inventory.InventoryClickEvent ( ['getRecipe', 'getInventory'])
    /// Gets the type of slot that was clicked.
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
    /// Gets the current ItemStack on the cursor.
    pub fn cursor(
        &self,
    ) -> Result<Option<crate::inventory::ItemStack<'mc>>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::inventory::InventoryClickEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::inventory::InventoryClickEvent = temp_clone.into();
        real.cursor()
    }
    /// Gets the ItemStack currently in the clicked slot.
    pub fn current_item(
        &self,
    ) -> Result<Option<crate::inventory::ItemStack<'mc>>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::inventory::InventoryClickEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::inventory::InventoryClickEvent = temp_clone.into();
        real.current_item()
    }
    /// Gets whether or not the ClickType for this event represents a right
    /// click.
    pub fn is_right_click(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::inventory::InventoryClickEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::inventory::InventoryClickEvent = temp_clone.into();
        real.is_right_click()
    }
    /// Gets whether or not the ClickType for this event represents a left
    /// click.
    pub fn is_left_click(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::inventory::InventoryClickEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::inventory::InventoryClickEvent = temp_clone.into();
        real.is_left_click()
    }
    /// Gets whether the ClickType for this event indicates that the key was
    /// pressed down when the click was made.
    pub fn is_shift_click(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::inventory::InventoryClickEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::inventory::InventoryClickEvent = temp_clone.into();
        real.is_shift_click()
    }
    /// Sets the item on the cursor.
    pub fn set_cursor(
        &self,
        stack: impl Into<crate::inventory::ItemStack<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::event::inventory::InventoryClickEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::inventory::InventoryClickEvent = temp_clone.into();
        real.set_cursor(stack)
    }
    /// Sets the ItemStack currently in the clicked slot.
    pub fn set_current_item(
        &self,
        stack: impl Into<crate::inventory::ItemStack<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::event::inventory::InventoryClickEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::inventory::InventoryClickEvent = temp_clone.into();
        real.set_current_item(stack)
    }
    /// Gets the inventory corresponding to the clicked slot.
    pub fn clicked_inventory(
        &self,
    ) -> Result<Option<crate::inventory::Inventory<'mc>>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::inventory::InventoryClickEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::inventory::InventoryClickEvent = temp_clone.into();
        real.clicked_inventory()
    }
    /// The slot number that was clicked, ready for passing to
    /// {@link Inventory#getItem(int)}. Note that there may be two slots with
    /// the same slot number, since a view links two different inventories.
    pub fn slot(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::inventory::InventoryClickEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::inventory::InventoryClickEvent = temp_clone.into();
        real.slot()
    }
    /// The raw slot number clicked, ready for passing to {@link InventoryView
    /// #getItem(int)} This slot number is unique for the view.
    pub fn raw_slot(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::inventory::InventoryClickEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::inventory::InventoryClickEvent = temp_clone.into();
        real.raw_slot()
    }
    /// If the ClickType is NUMBER_KEY, this method will return the index of
    /// the pressed key (0-8).
    pub fn hotbar_button(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::inventory::InventoryClickEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::inventory::InventoryClickEvent = temp_clone.into();
        real.hotbar_button()
    }
    /// Gets the InventoryAction that triggered this event.
    ///
    /// This action cannot be changed, and represents what the normal outcome
    /// of the event will be. To change the behavior of this
    /// InventoryClickEvent, changes must be manually applied.
    pub fn action(
        &self,
    ) -> Result<crate::event::inventory::InventoryAction<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::inventory::InventoryClickEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::inventory::InventoryClickEvent = temp_clone.into();
        real.action()
    }
    /// Gets the ClickType for this event.
    ///
    /// This is insulated against changes to the inventory by other plugins.
    pub fn click(
        &self,
    ) -> Result<crate::event::inventory::ClickType<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::inventory::InventoryClickEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::inventory::InventoryClickEvent = temp_clone.into();
        real.click()
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
        crate::event::inventory::InventoryClickEvent::handler_list(jni)
    }

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
        transaction: impl Into<crate::inventory::InventoryView<'mc>>,
    ) -> Result<crate::event::inventory::InventoryEvent<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/inventory/InventoryView;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(transaction.into().jni_object().clone())
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
    /// Gets the primary Inventory involved in this transaction
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
        let sig = String::from("()Lorg/bukkit/inventory/InventoryView;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getView", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::InventoryView::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
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
        let cls = jni.find_class("org/bukkit/event/inventory/InventoryEvent");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "getHandlerList", sig.as_str(), vec![]);
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        crate::event::HandlerList::from_raw(&jni, obj)
    }
    // SUPER CLASS: org.bukkit.event.Event ( ['getInventory', 'getViewers', 'getView', 'getHandlers', 'getHandlerList'])
    /// Convenience method for providing a user-friendly identifier. By
    /// default, it is the event's class's {@linkplain Class#getSimpleName()
    /// simple name}.
    pub fn event_name(&self) -> Result<String, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::Event::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::Event = temp_clone.into();
        real.event_name()
    }
    /// Any custom event that should not by synchronized with other events must
    /// use the specific constructor. These are the caveats of using an
    /// asynchronous event:
    /// <ul>
    /// <li>The event is never fired from inside code triggered by a
    /// synchronous event. Attempting to do so results in an {@link
    /// java.lang.IllegalStateException}.
    /// <li>However, asynchronous event handlers may fire synchronous or
    /// asynchronous events
    /// <li>The event may be fired multiple times simultaneously and in any
    /// order.
    /// <li>Any newly registered or unregistered handler is ignored after an
    /// event starts execution.
    /// <li>The handlers for this event may block for any length of time.
    /// <li>Some implementations may selectively declare a specific event use
    /// as asynchronous. This behavior should be clearly defined.
    /// <li>Asynchronous calls are not calculated in the plugin timing system.
    /// </ul>
    pub fn is_asynchronous(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isAsynchronous", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

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
        brewer: impl Into<crate::block::Block<'mc>>,
        contents: impl Into<crate::inventory::BrewerInventory<'mc>>,
        results: Vec<jni::objects::JObject<'mc>>,
        fuel_level: i32,
    ) -> Result<crate::event::inventory::BrewEvent<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from(
            "(Lorg/bukkit/block/Block;Lorg/bukkit/inventory/BrewerInventory;Ljava/util/List;I)V",
        );
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(brewer.into().jni_object().clone())
        });
        let val_2 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(contents.into().jni_object().clone())
        });
        let raw_val_3 = jni.new_object("java/util/ArrayList", "()V", vec![])?;
        for v in results {
            let map_val_0 = jni::objects::JValueGen::Object(v);
            jni.call_method(
                &raw_val_3,
                "add",
                "(Ljava/lang/Object;)Z",
                vec![jni::objects::JValueGen::from(map_val_0)],
            )?;
        }
        let val_3 = jni::objects::JValueGen::Object(raw_val_3);
        let val_4 = jni::objects::JValueGen::Int(fuel_level);
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
    /// Gets the contents of the Brewing Stand.
    /// <b>Note:</b> The brewer inventory still holds the items found prior to
    /// the finalization of the brewing process, e.g. the plain water bottles.
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
    /// Gets the remaining fuel level.
    pub fn fuel_level(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getFuelLevel", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    /// Gets the resulting items in the Brewing Stand.
    /// The returned list, in case of a server-created event instance, is
    /// mutable. Any changes in the returned list will reflect in the brewing
    /// result if the event is not cancelled. If the size of the list is reduced,
    /// remaining items will be set to air.
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
        let cls = jni.find_class("org/bukkit/event/inventory/BrewEvent");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "getHandlerList", sig.as_str(), vec![]);
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        crate::event::HandlerList::from_raw(&jni, obj)
    }
    // SUPER CLASS: org.bukkit.event.block.BlockEvent ( ['getContents', 'getFuelLevel', 'getResults', 'isCancelled', 'setCancelled', 'getHandlers', 'getHandlerList'])
    /// Gets the block involved in this event.
    pub fn block(&self) -> Result<crate::block::Block<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::block::BlockEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::block::BlockEvent = temp_clone.into();
        real.block()
    }

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
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        view: impl Into<crate::inventory::InventoryView<'mc>>,
        val_type: impl Into<crate::event::inventory::InventoryTypeSlotType<'mc>>,
        slot: i32,
        click: impl Into<crate::event::inventory::ClickType<'mc>>,
        action: impl Into<crate::event::inventory::InventoryAction<'mc>>,
        key: std::option::Option<i32>,
    ) -> Result<crate::event::inventory::SmithItemEvent<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/inventory/InventoryView;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(view.into().jni_object().clone())
        });
        args.push(val_1);
        sig += "Lorg/bukkit/event/inventory/InventoryType/SlotType;";
        let val_2 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(val_type.into().jni_object().clone())
        });
        args.push(val_2);
        sig += "I";
        let val_3 = jni::objects::JValueGen::Int(slot);
        args.push(val_3);
        sig += "Lorg/bukkit/event/inventory/ClickType;";
        let val_4 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(click.into().jni_object().clone())
        });
        args.push(val_4);
        sig += "Lorg/bukkit/event/inventory/InventoryAction;";
        let val_5 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(action.into().jni_object().clone())
        });
        args.push(val_5);
        if let Some(a) = key {
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
        let sig = String::from("()Lorg/bukkit/inventory/SmithingInventory;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getInventory", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::SmithingInventory::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    // SUPER CLASS: org.bukkit.event.inventory.InventoryClickEvent ( ['getInventory'])
    /// Gets the type of slot that was clicked.
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
    /// Gets the current ItemStack on the cursor.
    pub fn cursor(
        &self,
    ) -> Result<Option<crate::inventory::ItemStack<'mc>>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::inventory::InventoryClickEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::inventory::InventoryClickEvent = temp_clone.into();
        real.cursor()
    }
    /// Gets the ItemStack currently in the clicked slot.
    pub fn current_item(
        &self,
    ) -> Result<Option<crate::inventory::ItemStack<'mc>>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::inventory::InventoryClickEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::inventory::InventoryClickEvent = temp_clone.into();
        real.current_item()
    }
    /// Gets whether or not the ClickType for this event represents a right
    /// click.
    pub fn is_right_click(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::inventory::InventoryClickEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::inventory::InventoryClickEvent = temp_clone.into();
        real.is_right_click()
    }
    /// Gets whether or not the ClickType for this event represents a left
    /// click.
    pub fn is_left_click(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::inventory::InventoryClickEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::inventory::InventoryClickEvent = temp_clone.into();
        real.is_left_click()
    }
    /// Gets whether the ClickType for this event indicates that the key was
    /// pressed down when the click was made.
    pub fn is_shift_click(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::inventory::InventoryClickEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::inventory::InventoryClickEvent = temp_clone.into();
        real.is_shift_click()
    }
    /// Sets the item on the cursor.
    pub fn set_cursor(
        &self,
        stack: impl Into<crate::inventory::ItemStack<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::event::inventory::InventoryClickEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::inventory::InventoryClickEvent = temp_clone.into();
        real.set_cursor(stack)
    }
    /// Sets the ItemStack currently in the clicked slot.
    pub fn set_current_item(
        &self,
        stack: impl Into<crate::inventory::ItemStack<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::event::inventory::InventoryClickEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::inventory::InventoryClickEvent = temp_clone.into();
        real.set_current_item(stack)
    }
    /// Gets the inventory corresponding to the clicked slot.
    pub fn clicked_inventory(
        &self,
    ) -> Result<Option<crate::inventory::Inventory<'mc>>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::inventory::InventoryClickEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::inventory::InventoryClickEvent = temp_clone.into();
        real.clicked_inventory()
    }
    /// The slot number that was clicked, ready for passing to
    /// {@link Inventory#getItem(int)}. Note that there may be two slots with
    /// the same slot number, since a view links two different inventories.
    pub fn slot(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::inventory::InventoryClickEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::inventory::InventoryClickEvent = temp_clone.into();
        real.slot()
    }
    /// The raw slot number clicked, ready for passing to {@link InventoryView
    /// #getItem(int)} This slot number is unique for the view.
    pub fn raw_slot(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::inventory::InventoryClickEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::inventory::InventoryClickEvent = temp_clone.into();
        real.raw_slot()
    }
    /// If the ClickType is NUMBER_KEY, this method will return the index of
    /// the pressed key (0-8).
    pub fn hotbar_button(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::inventory::InventoryClickEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::inventory::InventoryClickEvent = temp_clone.into();
        real.hotbar_button()
    }
    /// Gets the InventoryAction that triggered this event.
    ///
    /// This action cannot be changed, and represents what the normal outcome
    /// of the event will be. To change the behavior of this
    /// InventoryClickEvent, changes must be manually applied.
    pub fn action(
        &self,
    ) -> Result<crate::event::inventory::InventoryAction<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::inventory::InventoryClickEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::inventory::InventoryClickEvent = temp_clone.into();
        real.action()
    }
    /// Gets the ClickType for this event.
    ///
    /// This is insulated against changes to the inventory by other plugins.
    pub fn click(
        &self,
    ) -> Result<crate::event::inventory::ClickType<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::inventory::InventoryClickEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::inventory::InventoryClickEvent = temp_clone.into();
        real.click()
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
        crate::event::inventory::InventoryClickEvent::handler_list(jni)
    }

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
        brewing_stand: impl Into<crate::block::Block<'mc>>,
        fuel: impl Into<crate::inventory::ItemStack<'mc>>,
        fuel_power: i32,
    ) -> Result<crate::event::inventory::BrewingStandFuelEvent<'mc>, Box<dyn std::error::Error>>
    {
        let sig = String::from("(Lorg/bukkit/block/Block;Lorg/bukkit/inventory/ItemStack;I)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(brewing_stand.into().jni_object().clone())
        });
        let val_2 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(fuel.into().jni_object().clone())
        });
        let val_3 = jni::objects::JValueGen::Int(fuel_power);
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
    /// Gets the ItemStack of the fuel before the amount was subtracted.
    pub fn fuel(&self) -> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/inventory/ItemStack;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getFuel", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::ItemStack::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Gets the fuel power for this fuel. Each unit of power can fuel one
    /// brewing operation.
    pub fn fuel_power(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getFuelPower", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    /// Sets the fuel power for this fuel. Each unit of power can fuel one
    /// brewing operation.
    pub fn set_fuel_power(&self, fuel_power: i32) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(I)V");
        let val_1 = jni::objects::JValueGen::Int(fuel_power);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setFuelPower",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Gets whether the brewing stand's fuel will be reduced / consumed or not.
    pub fn is_consuming(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isConsuming", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Sets whether the brewing stand's fuel will be reduced / consumed or not.
    pub fn set_consuming(&self, consuming: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        let val_1 = jni::objects::JValueGen::Bool(consuming.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setConsuming",
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
        let cls = jni.find_class("org/bukkit/event/inventory/BrewingStandFuelEvent");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "getHandlerList", sig.as_str(), vec![]);
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        crate::event::HandlerList::from_raw(&jni, obj)
    }
    // SUPER CLASS: org.bukkit.event.block.BlockEvent ( ['getFuel', 'getFuelPower', 'setFuelPower', 'isConsuming', 'setConsuming', 'isCancelled', 'setCancelled', 'getHandlers', 'getHandlerList'])
    /// Gets the block involved in this event.
    pub fn block(&self) -> Result<crate::block::Block<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::block::BlockEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::block::BlockEvent = temp_clone.into();
        real.block()
    }

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
impl<'mc> std::ops::Deref for DragType<'mc> {
    type Target = DragTypeStruct<'mc>;
    fn deref(&self) -> &<DragType<'mc> as std::ops::Deref>::Target {
        match self {
            DragType::Single { inner } => inner,
            DragType::Even { inner } => inner,
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
    pub fn values(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<crate::event::inventory::DragType<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/event/inventory/DragType;");
        let cls = jni.find_class("org/bukkit/event/inventory/DragType");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "values", sig.as_str(), vec![]);
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        crate::event::inventory::DragType::from_raw(&jni, obj)
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
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
        inventory: impl Into<crate::inventory::InventoryView<'mc>>,
        result: impl Into<crate::inventory::ItemStack<'mc>>,
    ) -> Result<crate::event::inventory::PrepareInventoryResultEvent<'mc>, Box<dyn std::error::Error>>
    {
        let sig =
            String::from("(Lorg/bukkit/inventory/InventoryView;Lorg/bukkit/inventory/ItemStack;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(inventory.into().jni_object().clone())
        });
        let val_2 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(result.into().jni_object().clone())
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
    /// Get result item, may be null.
    pub fn result(
        &self,
    ) -> Result<Option<crate::inventory::ItemStack<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/inventory/ItemStack;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getResult", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {
            return Ok(None);
        }
        Ok(Some(crate::inventory::ItemStack::from_raw(
            &self.jni_ref(),
            unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) },
        )?))
    }
    /// Set result item, may be null.
    pub fn set_result(
        &self,
        result: impl Into<crate::inventory::ItemStack<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/inventory/ItemStack;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(result.into().jni_object().clone())
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
        let cls = jni.find_class("org/bukkit/event/inventory/PrepareInventoryResultEvent");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "getHandlerList", sig.as_str(), vec![]);
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        crate::event::HandlerList::from_raw(&jni, obj)
    }
    // SUPER CLASS: org.bukkit.event.inventory.InventoryEvent ( ['getResult', 'setResult', 'getHandlers', 'getHandlerList'])
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
impl<'mc> Into<crate::event::inventory::InventoryEvent<'mc>> for PrepareInventoryResultEvent<'mc> {
    fn into(self) -> crate::event::inventory::InventoryEvent<'mc> {
        crate::event::inventory::InventoryEvent::from_raw(&self.jni_ref(), self.1).expect("Error converting PrepareInventoryResultEvent into crate::event::inventory::InventoryEvent")
    }
}
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
        what: impl Into<crate::inventory::InventoryView<'mc>>,
        new_cursor: impl Into<crate::inventory::ItemStack<'mc>>,
        old_cursor: impl Into<crate::inventory::ItemStack<'mc>>,
        right: bool,
        slots: impl Into<blackboxmc_java::util::JavaMap<'mc>>,
    ) -> Result<crate::event::inventory::InventoryDragEvent<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/inventory/InventoryView;Lorg/bukkit/inventory/ItemStack;Lorg/bukkit/inventory/ItemStack;ZLjava/util/Map;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(what.into().jni_object().clone())
        });
        let val_2 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(new_cursor.into().jni_object().clone())
        });
        let val_3 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(old_cursor.into().jni_object().clone())
        });
        let val_4 = jni::objects::JValueGen::Bool(right.into());
        let val_5 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(slots.into().jni_object().clone())
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
    /// Gets all items to be added to the inventory in this drag.
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
    /// Gets the raw slot ids to be changed in this drag.
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
    /// Gets the slots to be changed in this drag.
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
    /// Gets the result cursor after the drag is done. The returned value is
    /// mutable.
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
    /// Sets the result cursor after the drag is done.
    ///
    /// Changing this item stack changes the cursor item. Note that changing
    /// the affected "dragged" slots does not change this ItemStack, nor does
    /// changing this ItemStack affect the "dragged" slots.
    pub fn set_cursor(
        &self,
        new_cursor: impl Into<crate::inventory::ItemStack<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/inventory/ItemStack;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(new_cursor.into().jni_object().clone())
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
    /// Gets an ItemStack representing the cursor prior to any modifications
    /// as a result of this drag.
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
    /// Gets the DragType that describes the behavior of ItemStacks placed
    /// after this InventoryDragEvent.
    ///
    /// The ItemStacks and the raw slots that they're being applied to can be
    /// found using {@link #getNewItems()}.
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
        let cls = jni.find_class("org/bukkit/event/inventory/InventoryDragEvent");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "getHandlerList", sig.as_str(), vec![]);
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        crate::event::HandlerList::from_raw(&jni, obj)
    }
    // SUPER CLASS: org.bukkit.event.inventory.InventoryInteractEvent ( ['getNewItems', 'getRawSlots', 'getInventorySlots', 'getCursor', 'setCursor', 'getOldCursor', 'getType', 'getHandlers', 'getHandlerList'])
    /// Gets the player who performed the click.
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
    /// Sets the result of this event. This will change whether or not this
    /// event is considered cancelled.
    pub fn set_result(
        &self,
        new_result: impl Into<crate::event::EventResult<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone =
            crate::event::inventory::InventoryInteractEvent::from_raw(&self.0, unsafe {
                jni::objects::JObject::from_raw(self.1.clone())
            })?;
        let real: crate::event::inventory::InventoryInteractEvent = temp_clone.into();
        real.set_result(new_result)
    }
    /// Gets the {@link org.bukkit.event.Event.Result} of this event. The Result describes the
    /// behavior that will be applied to the inventory in relation to this
    /// event.
    pub fn result(&self) -> Result<crate::event::EventResult<'mc>, Box<dyn std::error::Error>> {
        let temp_clone =
            crate::event::inventory::InventoryInteractEvent::from_raw(&self.0, unsafe {
                jni::objects::JObject::from_raw(self.1.clone())
            })?;
        let real: crate::event::inventory::InventoryInteractEvent = temp_clone.into();
        real.result()
    }
    /// Gets whether or not this event is cancelled. This is based off of the
    /// Result value returned by {@link #getResult()}.Result.ALLOW and
    /// Result.DEFAULT will result in a returned value of false, but
    /// Result.DENY will result in a returned value of true.
    ///
    /// {@inheritDoc}
    pub fn is_cancelled(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone =
            crate::event::inventory::InventoryInteractEvent::from_raw(&self.0, unsafe {
                jni::objects::JObject::from_raw(self.1.clone())
            })?;
        let real: crate::event::inventory::InventoryInteractEvent = temp_clone.into();
        real.is_cancelled()
    }
    /// Proxy method to {@link #setResult(org.bukkit.event.Event.Result)} for the Cancellable
    /// interface. {@link #setResult(org.bukkit.event.Event.Result)} is preferred, as it allows
    /// you to specify the Result beyond Result.DENY and Result.ALLOW.
    ///
    /// {@inheritDoc}
    pub fn set_cancelled(&self, to_cancel: bool) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone =
            crate::event::inventory::InventoryInteractEvent::from_raw(&self.0, unsafe {
                jni::objects::JObject::from_raw(self.1.clone())
            })?;
        let real: crate::event::inventory::InventoryInteractEvent = temp_clone.into();
        real.set_cancelled(to_cancel)
    }

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
        furnace: impl Into<crate::block::Block<'mc>>,
        source: impl Into<crate::inventory::ItemStack<'mc>>,
        recipe: impl Into<crate::inventory::CookingRecipe<'mc>>,
    ) -> Result<crate::event::inventory::FurnaceStartSmeltEvent<'mc>, Box<dyn std::error::Error>>
    {
        let sig = String::from("(Lorg/bukkit/block/Block;Lorg/bukkit/inventory/ItemStack;Lorg/bukkit/inventory/CookingRecipe;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(furnace.into().jni_object().clone())
        });
        let val_2 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(source.into().jni_object().clone())
        });
        let val_3 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(recipe.into().jni_object().clone())
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
    /// Gets the total cook time associated with this event
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
    pub fn set_total_cook_time(&self, cook_time: i32) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(I)V");
        let val_1 = jni::objects::JValueGen::Int(cook_time);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setTotalCookTime",
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
        let cls = jni.find_class("org/bukkit/event/inventory/FurnaceStartSmeltEvent");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "getHandlerList", sig.as_str(), vec![]);
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        crate::event::HandlerList::from_raw(&jni, obj)
    }
    // SUPER CLASS: org.bukkit.event.block.InventoryBlockStartEvent ( ['getRecipe', 'getTotalCookTime', 'setTotalCookTime', 'getHandlers', 'getHandlerList'])
    /// Gets the source ItemStack for this event.
    pub fn source(&self) -> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>> {
        let temp_clone =
            crate::event::block::InventoryBlockStartEvent::from_raw(&self.0, unsafe {
                jni::objects::JObject::from_raw(self.1.clone())
            })?;
        let real: crate::event::block::InventoryBlockStartEvent = temp_clone.into();
        real.source()
    }

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
#[repr(C)]
pub struct HopperInventorySearchEvent<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

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
        inventory: impl Into<crate::inventory::Inventory<'mc>>,
        container_type: impl Into<crate::event::inventory::HopperInventorySearchEventContainerType<'mc>>,
        hopper: impl Into<crate::block::Block<'mc>>,
        search_block: impl Into<crate::block::Block<'mc>>,
    ) -> Result<crate::event::inventory::HopperInventorySearchEvent<'mc>, Box<dyn std::error::Error>>
    {
        let sig = String::from("(Lorg/bukkit/inventory/Inventory;Lorg/bukkit/event/inventory/HopperInventorySearchEvent/ContainerType;Lorg/bukkit/block/Block;Lorg/bukkit/block/Block;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(inventory.into().jni_object().clone())
        });
        let val_2 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(container_type.into().jni_object().clone())
        });
        let val_3 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(hopper.into().jni_object().clone())
        });
        let val_4 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(search_block.into().jni_object().clone())
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
    /// Set the {@link Inventory} that the Hopper will use for its
    /// source/attached Container.
    pub fn set_inventory(
        &self,
        inventory: impl Into<crate::inventory::Inventory<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/inventory/Inventory;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(inventory.into().jni_object().clone())
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
    /// Gets the {@link Inventory} that the Hopper will use for its
    /// source/attached Container.
    pub fn inventory(
        &self,
    ) -> Result<Option<crate::inventory::Inventory<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/inventory/Inventory;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getInventory", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {
            return Ok(None);
        }
        Ok(Some(crate::inventory::Inventory::from_raw(
            &self.jni_ref(),
            unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) },
        )?))
    }
    /// Gets the Container type the Hopper is searching for.
    pub fn container_type(
        &self,
    ) -> Result<
        crate::event::inventory::HopperInventorySearchEventContainerType<'mc>,
        Box<dyn std::error::Error>,
    > {
        let sig =
            String::from("()Lorg/bukkit/event/inventory/HopperInventorySearchEvent/ContainerType;");
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
    /// Gets the Block that is being searched for an inventory.
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
        let cls = jni.find_class("org/bukkit/event/inventory/HopperInventorySearchEvent");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "getHandlerList", sig.as_str(), vec![]);
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        crate::event::HandlerList::from_raw(&jni, obj)
    }
    // SUPER CLASS: org.bukkit.event.block.BlockEvent ( ['setInventory', 'getInventory', 'getContainerType', 'getSearchBlock', 'getHandlers', 'getHandlerList'])
    /// Gets the block involved in this event.
    pub fn block(&self) -> Result<crate::block::Block<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::block::BlockEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::block::BlockEvent = temp_clone.into();
        real.block()
    }

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
impl<'mc> std::ops::Deref for HopperInventorySearchEventContainerType<'mc> {
    type Target = HopperInventorySearchEventContainerTypeStruct<'mc>;
    fn deref(&self) -> &<HopperInventorySearchEventContainerType<'mc> as std::ops::Deref>::Target {
        match self {
            HopperInventorySearchEventContainerType::Source { inner } => inner,
            HopperInventorySearchEventContainerType::Destination { inner } => inner,
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
            env.find_class("org/bukkit/event/inventory/HopperInventorySearchEvent/ContainerType");
        let cls = env.translate_error_with_class(cls)?;
        let res = env.call_static_method(
                    cls,
                    "valueOf",
                    "(Ljava/lang/String;)Lorg/bukkit/event/inventory/HopperInventorySearchEvent/ContainerType;",
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
            "org/bukkit/event/inventory/HopperInventorySearchEvent/ContainerType",
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
            "org/bukkit/event/inventory/HopperInventorySearchEvent/ContainerType",
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
        crate::event::inventory::HopperInventorySearchEventContainerType<'mc>,
        Box<dyn std::error::Error>,
    > {
        let sig =
            String::from("()Lorg/bukkit/event/inventory/HopperInventorySearchEvent/ContainerType;");
        let cls =
            jni.find_class("org/bukkit/event/inventory/HopperInventorySearchEvent/ContainerType");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "values", sig.as_str(), vec![]);
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        crate::event::inventory::HopperInventorySearchEventContainerType::from_raw(&jni, obj)
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
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
        source_inventory: impl Into<crate::inventory::Inventory<'mc>>,
        item_stack: impl Into<crate::inventory::ItemStack<'mc>>,
        destination_inventory: impl Into<crate::inventory::Inventory<'mc>>,
        did_source_initiate: bool,
    ) -> Result<crate::event::inventory::InventoryMoveItemEvent<'mc>, Box<dyn std::error::Error>>
    {
        let sig = String::from("(Lorg/bukkit/inventory/Inventory;Lorg/bukkit/inventory/ItemStack;Lorg/bukkit/inventory/Inventory;Z)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(source_inventory.into().jni_object().clone())
        });
        let val_2 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(item_stack.into().jni_object().clone())
        });
        let val_3 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(destination_inventory.into().jni_object().clone())
        });
        let val_4 = jni::objects::JValueGen::Bool(did_source_initiate.into());
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
    /// Gets the Inventory that the ItemStack is being taken from
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
    /// Gets the ItemStack being moved; if modified, the original item will not
    /// be removed from the source inventory.
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
    /// Sets the ItemStack being moved; if this is different from the original
    /// ItemStack, the original item will not be removed from the source
    /// inventory.
    pub fn set_item(
        &self,
        item_stack: impl Into<crate::inventory::ItemStack<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/inventory/ItemStack;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(item_stack.into().jni_object().clone())
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
    /// Gets the Inventory that the ItemStack is being put into
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
    /// Gets the Inventory that initiated the transfer. This will always be
    /// either the destination or source Inventory.
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
        let cls = jni.find_class("org/bukkit/event/inventory/InventoryMoveItemEvent");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "getHandlerList", sig.as_str(), vec![]);
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        crate::event::HandlerList::from_raw(&jni, obj)
    }
    // SUPER CLASS: org.bukkit.event.Event ( ['getSource', 'getItem', 'setItem', 'getDestination', 'getInitiator', 'isCancelled', 'setCancelled', 'getHandlers', 'getHandlerList'])
    /// Convenience method for providing a user-friendly identifier. By
    /// default, it is the event's class's {@linkplain Class#getSimpleName()
    /// simple name}.
    pub fn event_name(&self) -> Result<String, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::Event::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::Event = temp_clone.into();
        real.event_name()
    }
    /// Any custom event that should not by synchronized with other events must
    /// use the specific constructor. These are the caveats of using an
    /// asynchronous event:
    /// <ul>
    /// <li>The event is never fired from inside code triggered by a
    /// synchronous event. Attempting to do so results in an {@link
    /// java.lang.IllegalStateException}.
    /// <li>However, asynchronous event handlers may fire synchronous or
    /// asynchronous events
    /// <li>The event may be fired multiple times simultaneously and in any
    /// order.
    /// <li>Any newly registered or unregistered handler is ignored after an
    /// event starts execution.
    /// <li>The handlers for this event may block for any length of time.
    /// <li>Some implementations may selectively declare a specific event use
    /// as asynchronous. This behavior should be clearly defined.
    /// <li>Asynchronous calls are not calculated in the plugin timing system.
    /// </ul>
    pub fn is_asynchronous(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isAsynchronous", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

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
        what: impl Into<crate::inventory::CraftingInventory<'mc>>,
        view: impl Into<crate::inventory::InventoryView<'mc>>,
        is_repair: bool,
    ) -> Result<crate::event::inventory::PrepareItemCraftEvent<'mc>, Box<dyn std::error::Error>>
    {
        let sig = String::from(
            "(Lorg/bukkit/inventory/CraftingInventory;Lorg/bukkit/inventory/InventoryView;Z)V",
        );
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(what.into().jni_object().clone())
        });
        let val_2 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(view.into().jni_object().clone())
        });
        let val_3 = jni::objects::JValueGen::Bool(is_repair.into());
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
    /// Get the recipe that has been formed. If this event was triggered by a
    /// tool repair, this will be a temporary shapeless recipe representing the
    /// repair.
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

    pub fn inventory(
        &self,
    ) -> Result<crate::inventory::CraftingInventory<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/inventory/CraftingInventory;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getInventory", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::CraftingInventory::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Check if this event was triggered by a tool repair operation rather
    /// than a crafting recipe.
    pub fn is_repair(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isRepair", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
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
        let cls = jni.find_class("org/bukkit/event/inventory/PrepareItemCraftEvent");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "getHandlerList", sig.as_str(), vec![]);
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        crate::event::HandlerList::from_raw(&jni, obj)
    }
    // SUPER CLASS: org.bukkit.event.inventory.InventoryEvent ( ['getRecipe', 'getInventory', 'isRepair', 'getHandlers', 'getHandlerList'])
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
impl<'mc> Into<crate::event::inventory::InventoryEvent<'mc>> for PrepareItemCraftEvent<'mc> {
    fn into(self) -> crate::event::inventory::InventoryEvent<'mc> {
        crate::event::inventory::InventoryEvent::from_raw(&self.jni_ref(), self.1).expect(
            "Error converting PrepareItemCraftEvent into crate::event::inventory::InventoryEvent",
        )
    }
}
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
        inventory: impl Into<crate::inventory::InventoryView<'mc>>,
        result: impl Into<crate::inventory::ItemStack<'mc>>,
    ) -> Result<crate::event::inventory::PrepareGrindstoneEvent<'mc>, Box<dyn std::error::Error>>
    {
        let sig =
            String::from("(Lorg/bukkit/inventory/InventoryView;Lorg/bukkit/inventory/ItemStack;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(inventory.into().jni_object().clone())
        });
        let val_2 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(result.into().jni_object().clone())
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

    pub fn inventory(
        &self,
    ) -> Result<crate::inventory::GrindstoneInventory<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/inventory/GrindstoneInventory;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getInventory", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::GrindstoneInventory::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
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
        let cls = jni.find_class("org/bukkit/event/inventory/PrepareGrindstoneEvent");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "getHandlerList", sig.as_str(), vec![]);
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        crate::event::HandlerList::from_raw(&jni, obj)
    }
    // SUPER CLASS: org.bukkit.event.inventory.PrepareInventoryResultEvent ( ['getInventory', 'getHandlers', 'getHandlerList'])
    /// Get result item, may be null.
    pub fn result(
        &self,
    ) -> Result<Option<crate::inventory::ItemStack<'mc>>, Box<dyn std::error::Error>> {
        let temp_clone =
            crate::event::inventory::PrepareInventoryResultEvent::from_raw(&self.0, unsafe {
                jni::objects::JObject::from_raw(self.1.clone())
            })?;
        let real: crate::event::inventory::PrepareInventoryResultEvent = temp_clone.into();
        real.result()
    }
    /// Set result item, may be null.
    pub fn set_result(
        &self,
        result: impl Into<crate::inventory::ItemStack<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone =
            crate::event::inventory::PrepareInventoryResultEvent::from_raw(&self.0, unsafe {
                jni::objects::JObject::from_raw(self.1.clone())
            })?;
        let real: crate::event::inventory::PrepareInventoryResultEvent = temp_clone.into();
        real.set_result(result)
    }

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
impl<'mc> std::ops::Deref for InventoryAction<'mc> {
    type Target = InventoryActionStruct<'mc>;
    fn deref(&self) -> &<InventoryAction<'mc> as std::ops::Deref>::Target {
        match self {
            InventoryAction::Nothing { inner } => inner,
            InventoryAction::PickupAll { inner } => inner,
            InventoryAction::PickupSome { inner } => inner,
            InventoryAction::PickupHalf { inner } => inner,
            InventoryAction::PickupOne { inner } => inner,
            InventoryAction::PlaceAll { inner } => inner,
            InventoryAction::PlaceSome { inner } => inner,
            InventoryAction::PlaceOne { inner } => inner,
            InventoryAction::SwapWithCursor { inner } => inner,
            InventoryAction::DropAllCursor { inner } => inner,
            InventoryAction::DropOneCursor { inner } => inner,
            InventoryAction::DropAllSlot { inner } => inner,
            InventoryAction::DropOneSlot { inner } => inner,
            InventoryAction::MoveToOtherInventory { inner } => inner,
            InventoryAction::HotbarMoveAndReadd { inner } => inner,
            InventoryAction::HotbarSwap { inner } => inner,
            InventoryAction::CloneStack { inner } => inner,
            InventoryAction::CollectToCursor { inner } => inner,
            InventoryAction::Unknown { inner } => inner,
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
    pub fn values(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<crate::event::inventory::InventoryAction<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/event/inventory/InventoryAction;");
        let cls = jni.find_class("org/bukkit/event/inventory/InventoryAction");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "values", sig.as_str(), vec![]);
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        crate::event::inventory::InventoryAction::from_raw(&jni, obj)
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
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
        inventory: impl Into<crate::inventory::Inventory<'mc>>,
        item: impl Into<crate::entity::Item<'mc>>,
    ) -> Result<crate::event::inventory::InventoryPickupItemEvent<'mc>, Box<dyn std::error::Error>>
    {
        let sig = String::from("(Lorg/bukkit/inventory/Inventory;Lorg/bukkit/entity/Item;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(inventory.into().jni_object().clone())
        });
        let val_2 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(item.into().jni_object().clone())
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
    /// Gets the Inventory that picked up the item
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
    /// Gets the Item entity that was picked up
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
        let cls = jni.find_class("org/bukkit/event/inventory/InventoryPickupItemEvent");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "getHandlerList", sig.as_str(), vec![]);
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        crate::event::HandlerList::from_raw(&jni, obj)
    }
    // SUPER CLASS: org.bukkit.event.Event ( ['getInventory', 'getItem', 'isCancelled', 'setCancelled', 'getHandlers', 'getHandlerList'])
    /// Convenience method for providing a user-friendly identifier. By
    /// default, it is the event's class's {@linkplain Class#getSimpleName()
    /// simple name}.
    pub fn event_name(&self) -> Result<String, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::Event::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::Event = temp_clone.into();
        real.event_name()
    }
    /// Any custom event that should not by synchronized with other events must
    /// use the specific constructor. These are the caveats of using an
    /// asynchronous event:
    /// <ul>
    /// <li>The event is never fired from inside code triggered by a
    /// synchronous event. Attempting to do so results in an {@link
    /// java.lang.IllegalStateException}.
    /// <li>However, asynchronous event handlers may fire synchronous or
    /// asynchronous events
    /// <li>The event may be fired multiple times simultaneously and in any
    /// order.
    /// <li>Any newly registered or unregistered handler is ignored after an
    /// event starts execution.
    /// <li>The handlers for this event may block for any length of time.
    /// <li>Some implementations may selectively declare a specific event use
    /// as asynchronous. This behavior should be clearly defined.
    /// <li>Asynchronous calls are not calculated in the plugin timing system.
    /// </ul>
    pub fn is_asynchronous(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isAsynchronous", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

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
        transaction: impl Into<crate::inventory::InventoryView<'mc>>,
    ) -> Result<crate::event::inventory::InventoryCloseEvent<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/inventory/InventoryView;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(transaction.into().jni_object().clone())
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
    /// Returns the player involved in this event
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
        let cls = jni.find_class("org/bukkit/event/inventory/InventoryCloseEvent");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "getHandlerList", sig.as_str(), vec![]);
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        crate::event::HandlerList::from_raw(&jni, obj)
    }
    // SUPER CLASS: org.bukkit.event.inventory.InventoryEvent ( ['getPlayer', 'getHandlers', 'getHandlerList'])
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
impl<'mc> Into<crate::event::inventory::InventoryEvent<'mc>> for InventoryCloseEvent<'mc> {
    fn into(self) -> crate::event::inventory::InventoryEvent<'mc> {
        crate::event::inventory::InventoryEvent::from_raw(&self.jni_ref(), self.1).expect(
            "Error converting InventoryCloseEvent into crate::event::inventory::InventoryEvent",
        )
    }
}
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
        transaction: impl Into<crate::inventory::InventoryView<'mc>>,
    ) -> Result<crate::event::inventory::InventoryInteractEvent<'mc>, Box<dyn std::error::Error>>
    {
        let sig = String::from("(Lorg/bukkit/inventory/InventoryView;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(transaction.into().jni_object().clone())
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
    /// Gets the player who performed the click.
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
    /// Sets the result of this event. This will change whether or not this
    /// event is considered cancelled.
    pub fn set_result(
        &self,
        new_result: impl Into<crate::event::EventResult<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/event/Event/Result;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(new_result.into().jni_object().clone())
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
    /// Gets the {@link org.bukkit.event.Event.Result} of this event. The Result describes the
    /// behavior that will be applied to the inventory in relation to this
    /// event.
    pub fn result(&self) -> Result<crate::event::EventResult<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/event/Event/Result;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getResult", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::event::EventResult::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Gets whether or not this event is cancelled. This is based off of the
    /// Result value returned by {@link #getResult()}.Result.ALLOW and
    /// Result.DEFAULT will result in a returned value of false, but
    /// Result.DENY will result in a returned value of true.
    ///
    /// {@inheritDoc}
    pub fn is_cancelled(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isCancelled", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Proxy method to {@link #setResult(org.bukkit.event.Event.Result)} for the Cancellable
    /// interface. {@link #setResult(org.bukkit.event.Event.Result)} is preferred, as it allows
    /// you to specify the Result beyond Result.DENY and Result.ALLOW.
    ///
    /// {@inheritDoc}
    pub fn set_cancelled(&self, to_cancel: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        let val_1 = jni::objects::JValueGen::Bool(to_cancel.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setCancelled",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    // SUPER CLASS: org.bukkit.event.inventory.InventoryEvent ( ['getWhoClicked', 'setResult', 'getResult', 'isCancelled', 'setCancelled'])
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
        crate::event::inventory::InventoryEvent::handler_list(jni)
    }

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
        inventory: impl Into<crate::inventory::view::AnvilView<'mc>>,
        result: impl Into<crate::inventory::ItemStack<'mc>>,
    ) -> Result<crate::event::inventory::PrepareAnvilEvent<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from(
            "(Lorg/bukkit/inventory/view/AnvilView;Lorg/bukkit/inventory/ItemStack;)V",
        );
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(inventory.into().jni_object().clone())
        });
        let val_2 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(result.into().jni_object().clone())
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

    pub fn inventory(
        &self,
    ) -> Result<crate::inventory::AnvilInventory<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/inventory/AnvilInventory;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getInventory", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::AnvilInventory::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn view(
        &self,
    ) -> Result<crate::inventory::view::AnvilView<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/inventory/view/AnvilView;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getView", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::view::AnvilView::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
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
        let cls = jni.find_class("org/bukkit/event/inventory/PrepareAnvilEvent");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "getHandlerList", sig.as_str(), vec![]);
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        crate::event::HandlerList::from_raw(&jni, obj)
    }
    // SUPER CLASS: org.bukkit.event.inventory.PrepareInventoryResultEvent ( ['getInventory', 'getView', 'getHandlers', 'getHandlerList'])
    /// Get result item, may be null.
    pub fn result(
        &self,
    ) -> Result<Option<crate::inventory::ItemStack<'mc>>, Box<dyn std::error::Error>> {
        let temp_clone =
            crate::event::inventory::PrepareInventoryResultEvent::from_raw(&self.0, unsafe {
                jni::objects::JObject::from_raw(self.1.clone())
            })?;
        let real: crate::event::inventory::PrepareInventoryResultEvent = temp_clone.into();
        real.result()
    }
    /// Set result item, may be null.
    pub fn set_result(
        &self,
        result: impl Into<crate::inventory::ItemStack<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone =
            crate::event::inventory::PrepareInventoryResultEvent::from_raw(&self.0, unsafe {
                jni::objects::JObject::from_raw(self.1.clone())
            })?;
        let real: crate::event::inventory::PrepareInventoryResultEvent = temp_clone.into();
        real.set_result(result)
    }

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
        what: impl Into<crate::inventory::InventoryView<'mc>>,
        val_type: impl Into<crate::event::inventory::InventoryTypeSlotType<'mc>>,
        slot: i32,
        new_item: impl Into<crate::inventory::ItemStack<'mc>>,
    ) -> Result<crate::event::inventory::InventoryCreativeEvent<'mc>, Box<dyn std::error::Error>>
    {
        let sig = String::from("(Lorg/bukkit/inventory/InventoryView;Lorg/bukkit/event/inventory/InventoryType/SlotType;ILorg/bukkit/inventory/ItemStack;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(what.into().jni_object().clone())
        });
        let val_2 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(val_type.into().jni_object().clone())
        });
        let val_3 = jni::objects::JValueGen::Int(slot);
        let val_4 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(new_item.into().jni_object().clone())
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

    pub fn cursor(&self) -> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/inventory/ItemStack;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getCursor", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::ItemStack::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

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
    // SUPER CLASS: org.bukkit.event.inventory.InventoryClickEvent ( ['getCursor', 'setCursor'])
    /// Gets the type of slot that was clicked.
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
    /// Gets the ItemStack currently in the clicked slot.
    pub fn current_item(
        &self,
    ) -> Result<Option<crate::inventory::ItemStack<'mc>>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::inventory::InventoryClickEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::inventory::InventoryClickEvent = temp_clone.into();
        real.current_item()
    }
    /// Gets whether or not the ClickType for this event represents a right
    /// click.
    pub fn is_right_click(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::inventory::InventoryClickEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::inventory::InventoryClickEvent = temp_clone.into();
        real.is_right_click()
    }
    /// Gets whether or not the ClickType for this event represents a left
    /// click.
    pub fn is_left_click(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::inventory::InventoryClickEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::inventory::InventoryClickEvent = temp_clone.into();
        real.is_left_click()
    }
    /// Gets whether the ClickType for this event indicates that the key was
    /// pressed down when the click was made.
    pub fn is_shift_click(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::inventory::InventoryClickEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::inventory::InventoryClickEvent = temp_clone.into();
        real.is_shift_click()
    }
    /// Sets the ItemStack currently in the clicked slot.
    pub fn set_current_item(
        &self,
        stack: impl Into<crate::inventory::ItemStack<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::event::inventory::InventoryClickEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::inventory::InventoryClickEvent = temp_clone.into();
        real.set_current_item(stack)
    }
    /// Gets the inventory corresponding to the clicked slot.
    pub fn clicked_inventory(
        &self,
    ) -> Result<Option<crate::inventory::Inventory<'mc>>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::inventory::InventoryClickEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::inventory::InventoryClickEvent = temp_clone.into();
        real.clicked_inventory()
    }
    /// The slot number that was clicked, ready for passing to
    /// {@link Inventory#getItem(int)}. Note that there may be two slots with
    /// the same slot number, since a view links two different inventories.
    pub fn slot(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::inventory::InventoryClickEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::inventory::InventoryClickEvent = temp_clone.into();
        real.slot()
    }
    /// The raw slot number clicked, ready for passing to {@link InventoryView
    /// #getItem(int)} This slot number is unique for the view.
    pub fn raw_slot(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::inventory::InventoryClickEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::inventory::InventoryClickEvent = temp_clone.into();
        real.raw_slot()
    }
    /// If the ClickType is NUMBER_KEY, this method will return the index of
    /// the pressed key (0-8).
    pub fn hotbar_button(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::inventory::InventoryClickEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::inventory::InventoryClickEvent = temp_clone.into();
        real.hotbar_button()
    }
    /// Gets the InventoryAction that triggered this event.
    ///
    /// This action cannot be changed, and represents what the normal outcome
    /// of the event will be. To change the behavior of this
    /// InventoryClickEvent, changes must be manually applied.
    pub fn action(
        &self,
    ) -> Result<crate::event::inventory::InventoryAction<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::inventory::InventoryClickEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::inventory::InventoryClickEvent = temp_clone.into();
        real.action()
    }
    /// Gets the ClickType for this event.
    ///
    /// This is insulated against changes to the inventory by other plugins.
    pub fn click(
        &self,
    ) -> Result<crate::event::inventory::ClickType<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::inventory::InventoryClickEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::inventory::InventoryClickEvent = temp_clone.into();
        real.click()
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
        crate::event::inventory::InventoryClickEvent::handler_list(jni)
    }

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
        transaction: impl Into<crate::inventory::InventoryView<'mc>>,
    ) -> Result<crate::event::inventory::InventoryOpenEvent<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/inventory/InventoryView;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(transaction.into().jni_object().clone())
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
    /// Returns the player involved in this event
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
    /// Gets the cancellation state of this event. A cancelled event will not
    /// be executed in the server, but will still pass to other plugins.
    ///
    /// If an inventory open event is cancelled, the inventory screen will not
    /// show.
    pub fn is_cancelled(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isCancelled", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Sets the cancellation state of this event. A cancelled event will not
    /// be executed in the server, but will still pass to other plugins.
    ///
    /// If an inventory open event is cancelled, the inventory screen will not
    /// show.
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
        let cls = jni.find_class("org/bukkit/event/inventory/InventoryOpenEvent");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "getHandlerList", sig.as_str(), vec![]);
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        crate::event::HandlerList::from_raw(&jni, obj)
    }
    // SUPER CLASS: org.bukkit.event.inventory.InventoryEvent ( ['getPlayer', 'isCancelled', 'setCancelled', 'getHandlers', 'getHandlerList'])
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
impl<'mc> std::ops::Deref for ClickType<'mc> {
    type Target = ClickTypeStruct<'mc>;
    fn deref(&self) -> &<ClickType<'mc> as std::ops::Deref>::Target {
        match self {
            ClickType::Left { inner } => inner,
            ClickType::ShiftLeft { inner } => inner,
            ClickType::Right { inner } => inner,
            ClickType::ShiftRight { inner } => inner,
            ClickType::WindowBorderLeft { inner } => inner,
            ClickType::WindowBorderRight { inner } => inner,
            ClickType::Middle { inner } => inner,
            ClickType::NumberKey { inner } => inner,
            ClickType::DoubleClick { inner } => inner,
            ClickType::Drop { inner } => inner,
            ClickType::ControlDrop { inner } => inner,
            ClickType::Creative { inner } => inner,
            ClickType::SwapOffhand { inner } => inner,
            ClickType::Unknown { inner } => inner,
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
    pub fn values(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<crate::event::inventory::ClickType<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/event/inventory/ClickType;");
        let cls = jni.find_class("org/bukkit/event/inventory/ClickType");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "values", sig.as_str(), vec![]);
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        crate::event::inventory::ClickType::from_raw(&jni, obj)
    }
    /// Gets whether this ClickType represents the pressing of a key on a
    /// keyboard.
    pub fn is_keyboard_click(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isKeyboardClick", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Gets whether this ClickType represents the pressing of a mouse button
    pub fn is_mouse_click(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isMouseClick", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Gets whether this ClickType represents an action that can only be
    /// performed by a Player in creative mode.
    pub fn is_creative_action(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "isCreativeAction",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Gets whether this ClickType represents a right click.
    pub fn is_right_click(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isRightClick", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Gets whether this ClickType represents a left click.
    pub fn is_left_click(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isLeftClick", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Gets whether this ClickType indicates that the shift key was pressed
    /// down when the click was made.
    pub fn is_shift_click(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isShiftClick", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
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
        furnace: impl Into<crate::block::Block<'mc>>,
        fuel: impl Into<crate::inventory::ItemStack<'mc>>,
        burn_time: i32,
    ) -> Result<crate::event::inventory::FurnaceBurnEvent<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/Block;Lorg/bukkit/inventory/ItemStack;I)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(furnace.into().jni_object().clone())
        });
        let val_2 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(fuel.into().jni_object().clone())
        });
        let val_3 = jni::objects::JValueGen::Int(burn_time);
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
    /// Gets the fuel ItemStack for this event
    pub fn fuel(&self) -> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/inventory/ItemStack;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getFuel", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::ItemStack::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Gets the burn time for this fuel
    pub fn burn_time(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getBurnTime", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    /// Sets the burn time for this fuel
    pub fn set_burn_time(&self, burn_time: i32) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(I)V");
        let val_1 = jni::objects::JValueGen::Int(burn_time);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setBurnTime",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Gets whether the furnace's fuel is burning or not.
    pub fn is_burning(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isBurning", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Sets whether the furnace's fuel is burning or not.
    pub fn set_burning(&self, burning: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        let val_1 = jni::objects::JValueGen::Bool(burning.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setBurning",
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
        let cls = jni.find_class("org/bukkit/event/inventory/FurnaceBurnEvent");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "getHandlerList", sig.as_str(), vec![]);
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        crate::event::HandlerList::from_raw(&jni, obj)
    }
    // SUPER CLASS: org.bukkit.event.block.BlockEvent ( ['getFuel', 'getBurnTime', 'setBurnTime', 'isBurning', 'setBurning', 'isCancelled', 'setCancelled', 'getHandlers', 'getHandlerList'])
    /// Gets the block involved in this event.
    pub fn block(&self) -> Result<crate::block::Block<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::event::block::BlockEvent::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::event::block::BlockEvent = temp_clone.into();
        real.block()
    }

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
        inventory: impl Into<crate::inventory::InventoryView<'mc>>,
        result: impl Into<crate::inventory::ItemStack<'mc>>,
    ) -> Result<crate::event::inventory::PrepareSmithingEvent<'mc>, Box<dyn std::error::Error>>
    {
        let sig =
            String::from("(Lorg/bukkit/inventory/InventoryView;Lorg/bukkit/inventory/ItemStack;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(inventory.into().jni_object().clone())
        });
        let val_2 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(result.into().jni_object().clone())
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

    pub fn inventory(
        &self,
    ) -> Result<crate::inventory::SmithingInventory<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/inventory/SmithingInventory;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getInventory", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::SmithingInventory::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
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
        let cls = jni.find_class("org/bukkit/event/inventory/PrepareSmithingEvent");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "getHandlerList", sig.as_str(), vec![]);
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        crate::event::HandlerList::from_raw(&jni, obj)
    }
    // SUPER CLASS: org.bukkit.event.inventory.PrepareInventoryResultEvent ( ['getInventory', 'getHandlers', 'getHandlerList'])
    /// Get result item, may be null.
    pub fn result(
        &self,
    ) -> Result<Option<crate::inventory::ItemStack<'mc>>, Box<dyn std::error::Error>> {
        let temp_clone =
            crate::event::inventory::PrepareInventoryResultEvent::from_raw(&self.0, unsafe {
                jni::objects::JObject::from_raw(self.1.clone())
            })?;
        let real: crate::event::inventory::PrepareInventoryResultEvent = temp_clone.into();
        real.result()
    }
    /// Set result item, may be null.
    pub fn set_result(
        &self,
        result: impl Into<crate::inventory::ItemStack<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone =
            crate::event::inventory::PrepareInventoryResultEvent::from_raw(&self.0, unsafe {
                jni::objects::JObject::from_raw(self.1.clone())
            })?;
        let real: crate::event::inventory::PrepareInventoryResultEvent = temp_clone.into();
        real.set_result(result)
    }

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
        transaction: impl Into<crate::inventory::view::MerchantView<'mc>>,
        new_index: i32,
    ) -> Result<crate::event::inventory::TradeSelectEvent<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/inventory/view/MerchantView;I)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(transaction.into().jni_object().clone())
        });
        let val_2 = jni::objects::JValueGen::Int(new_index);
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
    /// Used to get the index of the trade the player clicked on.
    pub fn index(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getIndex", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }

    pub fn inventory(
        &self,
    ) -> Result<crate::inventory::MerchantInventory<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/inventory/MerchantInventory;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getInventory", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::MerchantInventory::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Get the Merchant involved.
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

    pub fn view(
        &self,
    ) -> Result<crate::inventory::view::MerchantView<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/inventory/view/MerchantView;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getView", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::view::MerchantView::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
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
        let cls = jni.find_class("org/bukkit/event/inventory/TradeSelectEvent");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "getHandlerList", sig.as_str(), vec![]);
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        crate::event::HandlerList::from_raw(&jni, obj)
    }
    // SUPER CLASS: org.bukkit.event.inventory.InventoryInteractEvent ( ['getIndex', 'getInventory', 'getMerchant', 'getView', 'getHandlers', 'getHandlerList'])
    /// Gets the player who performed the click.
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
    /// Sets the result of this event. This will change whether or not this
    /// event is considered cancelled.
    pub fn set_result(
        &self,
        new_result: impl Into<crate::event::EventResult<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone =
            crate::event::inventory::InventoryInteractEvent::from_raw(&self.0, unsafe {
                jni::objects::JObject::from_raw(self.1.clone())
            })?;
        let real: crate::event::inventory::InventoryInteractEvent = temp_clone.into();
        real.set_result(new_result)
    }
    /// Gets the {@link org.bukkit.event.Event.Result} of this event. The Result describes the
    /// behavior that will be applied to the inventory in relation to this
    /// event.
    pub fn result(&self) -> Result<crate::event::EventResult<'mc>, Box<dyn std::error::Error>> {
        let temp_clone =
            crate::event::inventory::InventoryInteractEvent::from_raw(&self.0, unsafe {
                jni::objects::JObject::from_raw(self.1.clone())
            })?;
        let real: crate::event::inventory::InventoryInteractEvent = temp_clone.into();
        real.result()
    }
    /// Gets whether or not this event is cancelled. This is based off of the
    /// Result value returned by {@link #getResult()}.Result.ALLOW and
    /// Result.DEFAULT will result in a returned value of false, but
    /// Result.DENY will result in a returned value of true.
    ///
    /// {@inheritDoc}
    pub fn is_cancelled(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone =
            crate::event::inventory::InventoryInteractEvent::from_raw(&self.0, unsafe {
                jni::objects::JObject::from_raw(self.1.clone())
            })?;
        let real: crate::event::inventory::InventoryInteractEvent = temp_clone.into();
        real.is_cancelled()
    }
    /// Proxy method to {@link #setResult(org.bukkit.event.Event.Result)} for the Cancellable
    /// interface. {@link #setResult(org.bukkit.event.Event.Result)} is preferred, as it allows
    /// you to specify the Result beyond Result.DENY and Result.ALLOW.
    ///
    /// {@inheritDoc}
    pub fn set_cancelled(&self, to_cancel: bool) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone =
            crate::event::inventory::InventoryInteractEvent::from_raw(&self.0, unsafe {
                jni::objects::JObject::from_raw(self.1.clone())
            })?;
        let real: crate::event::inventory::InventoryInteractEvent = temp_clone.into();
        real.set_cancelled(to_cancel)
    }

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
