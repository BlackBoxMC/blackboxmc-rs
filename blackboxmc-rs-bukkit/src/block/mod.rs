#![allow(deprecated)]
use blackboxmc_general::JNIInstantiatable;
use blackboxmc_general::JNIRaw;
use color_eyre::eyre::Result;
/// Represents a captured state of a chest.
///
/// This is a representation of an abstract class.
#[repr(C)]
pub struct Chest<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for Chest<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for Chest<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Chest from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/block/Chest")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Chest object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> Chest<'mc> {
    pub fn block_inventory(
        &self,
    ) -> Result<crate::inventory::Inventory<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/inventory/Inventory;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getBlockInventory",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::Inventory::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    pub fn inventory(
        &self,
    ) -> Result<crate::inventory::Inventory<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = Chest::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::Container = temp_clone.into();
        real.inventory()
    }
    pub fn snapshot_inventory(
        &self,
    ) -> Result<crate::inventory::Inventory<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = Chest::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::Container = temp_clone.into();
        real.snapshot_inventory()
    }
    // SUPER CLASS: TileState
    pub fn data(&self) -> Result<crate::material::MaterialData<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::TileState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.data()
    }
    pub fn world(&self) -> Result<crate::World<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::TileState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.world()
    }
    pub fn set_type(
        &self,
        arg0: impl Into<crate::Material<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::block::TileState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.set_type(arg0)
    }
    pub fn block(&self) -> Result<crate::block::Block<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::TileState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.block()
    }
    pub fn block_data(
        &self,
    ) -> Result<crate::block::data::BlockData<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::TileState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.block_data()
    }
    pub fn light_level(&self) -> Result<i8, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::TileState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.light_level()
    }
    pub fn x(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::TileState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.x()
    }
    pub fn y(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::TileState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.y()
    }
    pub fn z(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::TileState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.z()
    }
    pub fn chunk(&self) -> Result<crate::Chunk<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::TileState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.chunk()
    }
    pub fn set_data(
        &self,
        arg0: impl Into<crate::material::MaterialData<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::block::TileState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.set_data(arg0)
    }
    pub fn set_block_data(
        &self,
        arg0: impl Into<crate::block::data::BlockData<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::block::TileState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.set_block_data(arg0)
    }

    pub fn raw_data(&self) -> Result<i8, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::TileState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.raw_data()
    }

    pub fn set_raw_data(&self, arg0: i8) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::block::TileState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.set_raw_data(arg0)
    }
    pub fn is_placed(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::TileState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.is_placed()
    }
    pub fn update_with_boolean(
        &self,
        arg0: std::option::Option<bool>,
        arg1: std::option::Option<bool>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "Z";
            let val_1 = jni::objects::JValueGen::Bool(a.into());
            args.push(val_1);
        }
        if let Some(a) = arg1 {
            sig += "Z";
            let val_2 = jni::objects::JValueGen::Bool(a.into());
            args.push(val_2);
        }
        sig += ")Z";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "update", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    pub fn location(&self) -> Result<crate::Location<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::TileState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.location()
    }
    pub fn get_type(&self) -> Result<crate::Material<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::TileState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.get_type()
    }
    // SUPER CLASS: Metadatable
    pub fn set_metadata(
        &self,
        arg0: impl Into<String>,
        arg1: impl Into<crate::metadata::MetadataValue<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::metadata::Metadatable::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::metadata::Metadatable = temp_clone.into();
        real.set_metadata(arg0, arg1)
    }
    pub fn get_metadata(
        &self,
        arg0: impl Into<String>,
    ) -> Result<Vec<crate::metadata::MetadataValue<'mc>>, Box<dyn std::error::Error>> {
        let temp_clone = crate::metadata::Metadatable::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::metadata::Metadatable = temp_clone.into();
        real.get_metadata(arg0)
    }
    pub fn has_metadata(
        &self,
        arg0: impl Into<String>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::metadata::Metadatable::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::metadata::Metadatable = temp_clone.into();
        real.has_metadata(arg0)
    }
    pub fn remove_metadata(
        &self,
        arg0: impl Into<String>,
        arg1: impl Into<crate::plugin::Plugin<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::metadata::Metadatable::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::metadata::Metadatable = temp_clone.into();
        real.remove_metadata(arg0, arg1)
    }
    pub fn persistent_data_container(
        &self,
    ) -> Result<crate::persistence::PersistentDataContainer<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::TileState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::persistence::PersistentDataHolder = temp_clone.into();
        real.persistent_data_container()
    }
    // SUPER CLASS: BlockInventoryHolder
    // SUPER CLASS: Lockable
    pub fn is_locked(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::Lockable::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::Lockable = temp_clone.into();
        real.is_locked()
    }
    pub fn lock(&self) -> Result<String, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::Lockable::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::Lockable = temp_clone.into();
        real.lock()
    }
    pub fn set_lock(&self, arg0: impl Into<String>) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::block::Lockable::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::Lockable = temp_clone.into();
        real.set_lock(arg0)
    }
    // SUPER CLASS: Nameable
    pub fn custom_name(&self) -> Result<Option<String>, Box<dyn std::error::Error>> {
        let temp_clone = crate::Nameable::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::Nameable = temp_clone.into();
        real.custom_name()
    }
    pub fn set_custom_name(
        &self,
        arg0: impl Into<String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::Nameable::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::Nameable = temp_clone.into();
        real.set_custom_name(arg0)
    }
    pub fn seed(&self) -> Result<i64, Box<dyn std::error::Error>> {
        let temp_clone = Chest::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::loot::Lootable = temp_clone.into();
        real.seed()
    }
    pub fn set_seed(&self, arg0: i64) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = Chest::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::loot::Lootable = temp_clone.into();
        real.set_seed(arg0)
    }
    pub fn set_loot_table(
        &self,
        arg0: impl Into<crate::loot::LootTable<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = Chest::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::loot::Lootable = temp_clone.into();
        real.set_loot_table(arg0)
    }
    pub fn loot_table(
        &self,
    ) -> Result<Option<crate::loot::LootTable<'mc>>, Box<dyn std::error::Error>> {
        let temp_clone = Chest::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::loot::Lootable = temp_clone.into();
        real.loot_table()
    }
    pub fn close(&self) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = Chest::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::Lidded = temp_clone.into();
        real.close()
    }
    pub fn open(&self) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = Chest::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::Lidded = temp_clone.into();
        real.open()
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::block::Container<'mc>> for Chest<'mc> {
    fn into(self) -> crate::block::Container<'mc> {
        crate::block::Container::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting Chest into crate::block::Container")
    }
}
impl<'mc> Into<crate::loot::Lootable<'mc>> for Chest<'mc> {
    fn into(self) -> crate::loot::Lootable<'mc> {
        crate::loot::Lootable::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting Chest into crate::loot::Lootable")
    }
}
impl<'mc> Into<crate::block::Lidded<'mc>> for Chest<'mc> {
    fn into(self) -> crate::block::Lidded<'mc> {
        crate::block::Lidded::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting Chest into crate::block::Lidded")
    }
}
pub enum BlockFace<'mc> {
    North { inner: BlockFaceStruct<'mc> },
    East { inner: BlockFaceStruct<'mc> },
    South { inner: BlockFaceStruct<'mc> },
    West { inner: BlockFaceStruct<'mc> },
    Up { inner: BlockFaceStruct<'mc> },
    Down { inner: BlockFaceStruct<'mc> },
    NorthEast { inner: BlockFaceStruct<'mc> },
    NorthWest { inner: BlockFaceStruct<'mc> },
    SouthEast { inner: BlockFaceStruct<'mc> },
    SouthWest { inner: BlockFaceStruct<'mc> },
    WestNorthWest { inner: BlockFaceStruct<'mc> },
    NorthNorthWest { inner: BlockFaceStruct<'mc> },
    NorthNorthEast { inner: BlockFaceStruct<'mc> },
    EastNorthEast { inner: BlockFaceStruct<'mc> },
    EastSouthEast { inner: BlockFaceStruct<'mc> },
    SouthSouthEast { inner: BlockFaceStruct<'mc> },
    SouthSouthWest { inner: BlockFaceStruct<'mc> },
    WestSouthWest { inner: BlockFaceStruct<'mc> },
    VariantSelf { inner: BlockFaceStruct<'mc> },
}
impl<'mc> std::fmt::Display for BlockFace<'mc> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BlockFace::North { .. } => f.write_str("NORTH"),
            BlockFace::East { .. } => f.write_str("EAST"),
            BlockFace::South { .. } => f.write_str("SOUTH"),
            BlockFace::West { .. } => f.write_str("WEST"),
            BlockFace::Up { .. } => f.write_str("UP"),
            BlockFace::Down { .. } => f.write_str("DOWN"),
            BlockFace::NorthEast { .. } => f.write_str("NORTH_EAST"),
            BlockFace::NorthWest { .. } => f.write_str("NORTH_WEST"),
            BlockFace::SouthEast { .. } => f.write_str("SOUTH_EAST"),
            BlockFace::SouthWest { .. } => f.write_str("SOUTH_WEST"),
            BlockFace::WestNorthWest { .. } => f.write_str("WEST_NORTH_WEST"),
            BlockFace::NorthNorthWest { .. } => f.write_str("NORTH_NORTH_WEST"),
            BlockFace::NorthNorthEast { .. } => f.write_str("NORTH_NORTH_EAST"),
            BlockFace::EastNorthEast { .. } => f.write_str("EAST_NORTH_EAST"),
            BlockFace::EastSouthEast { .. } => f.write_str("EAST_SOUTH_EAST"),
            BlockFace::SouthSouthEast { .. } => f.write_str("SOUTH_SOUTH_EAST"),
            BlockFace::SouthSouthWest { .. } => f.write_str("SOUTH_SOUTH_WEST"),
            BlockFace::WestSouthWest { .. } => f.write_str("WEST_SOUTH_WEST"),
            BlockFace::VariantSelf { .. } => f.write_str("SELF"),
        }
    }
}

impl<'mc> BlockFace<'mc> {
    pub fn value_of(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<String>,
    ) -> Result<BlockFace<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(env.new_string(arg0.into())?);
        let cls = env.find_class("org/bukkit/block/BlockFace");
        let cls = env.translate_error_with_class(cls)?;
        let res = env.call_static_method(
            cls,
            "valueOf",
            "(Ljava/lang/String;)Lorg/bukkit/block/BlockFace;",
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
            "NORTH" => Ok(BlockFace::North {
                inner: BlockFaceStruct::from_raw(env, obj)?,
            }),
            "EAST" => Ok(BlockFace::East {
                inner: BlockFaceStruct::from_raw(env, obj)?,
            }),
            "SOUTH" => Ok(BlockFace::South {
                inner: BlockFaceStruct::from_raw(env, obj)?,
            }),
            "WEST" => Ok(BlockFace::West {
                inner: BlockFaceStruct::from_raw(env, obj)?,
            }),
            "UP" => Ok(BlockFace::Up {
                inner: BlockFaceStruct::from_raw(env, obj)?,
            }),
            "DOWN" => Ok(BlockFace::Down {
                inner: BlockFaceStruct::from_raw(env, obj)?,
            }),
            "NORTH_EAST" => Ok(BlockFace::NorthEast {
                inner: BlockFaceStruct::from_raw(env, obj)?,
            }),
            "NORTH_WEST" => Ok(BlockFace::NorthWest {
                inner: BlockFaceStruct::from_raw(env, obj)?,
            }),
            "SOUTH_EAST" => Ok(BlockFace::SouthEast {
                inner: BlockFaceStruct::from_raw(env, obj)?,
            }),
            "SOUTH_WEST" => Ok(BlockFace::SouthWest {
                inner: BlockFaceStruct::from_raw(env, obj)?,
            }),
            "WEST_NORTH_WEST" => Ok(BlockFace::WestNorthWest {
                inner: BlockFaceStruct::from_raw(env, obj)?,
            }),
            "NORTH_NORTH_WEST" => Ok(BlockFace::NorthNorthWest {
                inner: BlockFaceStruct::from_raw(env, obj)?,
            }),
            "NORTH_NORTH_EAST" => Ok(BlockFace::NorthNorthEast {
                inner: BlockFaceStruct::from_raw(env, obj)?,
            }),
            "EAST_NORTH_EAST" => Ok(BlockFace::EastNorthEast {
                inner: BlockFaceStruct::from_raw(env, obj)?,
            }),
            "EAST_SOUTH_EAST" => Ok(BlockFace::EastSouthEast {
                inner: BlockFaceStruct::from_raw(env, obj)?,
            }),
            "SOUTH_SOUTH_EAST" => Ok(BlockFace::SouthSouthEast {
                inner: BlockFaceStruct::from_raw(env, obj)?,
            }),
            "SOUTH_SOUTH_WEST" => Ok(BlockFace::SouthSouthWest {
                inner: BlockFaceStruct::from_raw(env, obj)?,
            }),
            "WEST_SOUTH_WEST" => Ok(BlockFace::WestSouthWest {
                inner: BlockFaceStruct::from_raw(env, obj)?,
            }),
            "SELF" => Ok(BlockFace::VariantSelf {
                inner: BlockFaceStruct::from_raw(env, obj)?,
            }),

            _ => Err(eyre::eyre!("String gaven for variant was invalid").into()),
        }
    }
}

#[repr(C)]
pub struct BlockFaceStruct<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for BlockFace<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        match self {
            Self::North { inner } => inner.0.clone(),
            Self::East { inner } => inner.0.clone(),
            Self::South { inner } => inner.0.clone(),
            Self::West { inner } => inner.0.clone(),
            Self::Up { inner } => inner.0.clone(),
            Self::Down { inner } => inner.0.clone(),
            Self::NorthEast { inner } => inner.0.clone(),
            Self::NorthWest { inner } => inner.0.clone(),
            Self::SouthEast { inner } => inner.0.clone(),
            Self::SouthWest { inner } => inner.0.clone(),
            Self::WestNorthWest { inner } => inner.0.clone(),
            Self::NorthNorthWest { inner } => inner.0.clone(),
            Self::NorthNorthEast { inner } => inner.0.clone(),
            Self::EastNorthEast { inner } => inner.0.clone(),
            Self::EastSouthEast { inner } => inner.0.clone(),
            Self::SouthSouthEast { inner } => inner.0.clone(),
            Self::SouthSouthWest { inner } => inner.0.clone(),
            Self::WestSouthWest { inner } => inner.0.clone(),
            Self::VariantSelf { inner } => inner.0.clone(),
        }
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        match self {
            Self::North { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::East { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::South { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::West { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Up { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Down { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::NorthEast { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::NorthWest { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::SouthEast { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::SouthWest { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::WestNorthWest { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::NorthNorthWest { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::NorthNorthEast { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::EastNorthEast { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::EastSouthEast { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::SouthSouthEast { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::SouthSouthWest { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::WestSouthWest { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::VariantSelf { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
        }
    }
}
impl<'mc> JNIInstantiatable<'mc> for BlockFace<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate BlockFace from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/block/BlockFace")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a BlockFace object, got {}",
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
                "NORTH" => Ok(BlockFace::North {
                    inner: BlockFaceStruct::from_raw(env, obj)?,
                }),
                "EAST" => Ok(BlockFace::East {
                    inner: BlockFaceStruct::from_raw(env, obj)?,
                }),
                "SOUTH" => Ok(BlockFace::South {
                    inner: BlockFaceStruct::from_raw(env, obj)?,
                }),
                "WEST" => Ok(BlockFace::West {
                    inner: BlockFaceStruct::from_raw(env, obj)?,
                }),
                "UP" => Ok(BlockFace::Up {
                    inner: BlockFaceStruct::from_raw(env, obj)?,
                }),
                "DOWN" => Ok(BlockFace::Down {
                    inner: BlockFaceStruct::from_raw(env, obj)?,
                }),
                "NORTH_EAST" => Ok(BlockFace::NorthEast {
                    inner: BlockFaceStruct::from_raw(env, obj)?,
                }),
                "NORTH_WEST" => Ok(BlockFace::NorthWest {
                    inner: BlockFaceStruct::from_raw(env, obj)?,
                }),
                "SOUTH_EAST" => Ok(BlockFace::SouthEast {
                    inner: BlockFaceStruct::from_raw(env, obj)?,
                }),
                "SOUTH_WEST" => Ok(BlockFace::SouthWest {
                    inner: BlockFaceStruct::from_raw(env, obj)?,
                }),
                "WEST_NORTH_WEST" => Ok(BlockFace::WestNorthWest {
                    inner: BlockFaceStruct::from_raw(env, obj)?,
                }),
                "NORTH_NORTH_WEST" => Ok(BlockFace::NorthNorthWest {
                    inner: BlockFaceStruct::from_raw(env, obj)?,
                }),
                "NORTH_NORTH_EAST" => Ok(BlockFace::NorthNorthEast {
                    inner: BlockFaceStruct::from_raw(env, obj)?,
                }),
                "EAST_NORTH_EAST" => Ok(BlockFace::EastNorthEast {
                    inner: BlockFaceStruct::from_raw(env, obj)?,
                }),
                "EAST_SOUTH_EAST" => Ok(BlockFace::EastSouthEast {
                    inner: BlockFaceStruct::from_raw(env, obj)?,
                }),
                "SOUTH_SOUTH_EAST" => Ok(BlockFace::SouthSouthEast {
                    inner: BlockFaceStruct::from_raw(env, obj)?,
                }),
                "SOUTH_SOUTH_WEST" => Ok(BlockFace::SouthSouthWest {
                    inner: BlockFaceStruct::from_raw(env, obj)?,
                }),
                "WEST_SOUTH_WEST" => Ok(BlockFace::WestSouthWest {
                    inner: BlockFaceStruct::from_raw(env, obj)?,
                }),
                "SELF" => Ok(BlockFace::VariantSelf {
                    inner: BlockFaceStruct::from_raw(env, obj)?,
                }),
                _ => Err(eyre::eyre!("String gaven for variant was invalid").into()),
            }
        }
    }
}

impl<'mc> JNIRaw<'mc> for BlockFaceStruct<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for BlockFaceStruct<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate BlockFaceStruct from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/block/BlockFace")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a BlockFaceStruct object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> BlockFaceStruct<'mc> {
    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
/// Represents a captured state of a (possibly inverted) daylight detector.
///
/// This is a representation of an abstract class.
#[repr(C)]
pub struct DaylightDetector<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for DaylightDetector<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for DaylightDetector<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate DaylightDetector from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/block/DaylightDetector")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a DaylightDetector object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> DaylightDetector<'mc> {
    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::block::TileState<'mc>> for DaylightDetector<'mc> {
    fn into(self) -> crate::block::TileState<'mc> {
        crate::block::TileState::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting DaylightDetector into crate::block::TileState")
    }
}
/// Represents a block (usually a container) that may be locked. When a lock is active an item with a name corresponding to the key will be required to open this block.
///
/// This is a representation of an abstract class.
#[repr(C)]
pub struct Lockable<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for Lockable<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for Lockable<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Lockable from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/block/Lockable")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Lockable object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> Lockable<'mc> {
    pub fn is_locked(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isLocked", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn lock(&self) -> Result<String, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/String;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getLock", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }

    pub fn set_lock(&self, arg0: impl Into<String>) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/String;)V");
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(arg0.into())?,
        ));
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setLock",
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
/// Represents a captured state of a lectern.
///
/// This is a representation of an abstract class.
#[repr(C)]
pub struct Lectern<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for Lectern<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for Lectern<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Lectern from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/block/Lectern")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Lectern object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> Lectern<'mc> {
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

    pub fn snapshot_inventory(
        &self,
    ) -> Result<crate::inventory::Inventory<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/inventory/Inventory;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getSnapshotInventory",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::Inventory::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn page(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getPage", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    /// Set the current lectern page. If the page is greater than the number of pages of the book currently in the inventory, then behavior is undefined.
    pub fn set_page(&self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(I)V");
        let val_1 = jni::objects::JValueGen::Int(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setPage",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub fn persistent_data_container(
        &self,
    ) -> Result<crate::persistence::PersistentDataContainer<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = Lectern::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::TileState = temp_clone.into();
        real.persistent_data_container()
    }
    // SUPER CLASS: BlockState
    pub fn set_metadata(
        &self,
        arg0: impl Into<String>,
        arg1: impl Into<crate::metadata::MetadataValue<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::metadata::Metadatable = temp_clone.into();
        real.set_metadata(arg0, arg1)
    }
    pub fn get_metadata(
        &self,
        arg0: impl Into<String>,
    ) -> Result<Vec<crate::metadata::MetadataValue<'mc>>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::metadata::Metadatable = temp_clone.into();
        real.get_metadata(arg0)
    }
    pub fn has_metadata(
        &self,
        arg0: impl Into<String>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::metadata::Metadatable = temp_clone.into();
        real.has_metadata(arg0)
    }
    pub fn remove_metadata(
        &self,
        arg0: impl Into<String>,
        arg1: impl Into<crate::plugin::Plugin<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::metadata::Metadatable = temp_clone.into();
        real.remove_metadata(arg0, arg1)
    }
    pub fn data(&self) -> Result<crate::material::MaterialData<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.data()
    }
    pub fn world(&self) -> Result<crate::World<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.world()
    }
    pub fn set_type(
        &self,
        arg0: impl Into<crate::Material<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.set_type(arg0)
    }
    pub fn block(&self) -> Result<crate::block::Block<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.block()
    }
    pub fn block_data(
        &self,
    ) -> Result<crate::block::data::BlockData<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.block_data()
    }
    pub fn light_level(&self) -> Result<i8, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.light_level()
    }
    pub fn x(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.x()
    }
    pub fn y(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.y()
    }
    pub fn z(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.z()
    }
    pub fn chunk(&self) -> Result<crate::Chunk<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.chunk()
    }
    pub fn set_data(
        &self,
        arg0: impl Into<crate::material::MaterialData<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.set_data(arg0)
    }
    pub fn set_block_data(
        &self,
        arg0: impl Into<crate::block::data::BlockData<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.set_block_data(arg0)
    }

    pub fn raw_data(&self) -> Result<i8, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.raw_data()
    }

    pub fn set_raw_data(&self, arg0: i8) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.set_raw_data(arg0)
    }
    pub fn is_placed(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.is_placed()
    }
    pub fn update_with_boolean(
        &self,
        arg0: std::option::Option<bool>,
        arg1: std::option::Option<bool>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "Z";
            let val_1 = jni::objects::JValueGen::Bool(a.into());
            args.push(val_1);
        }
        if let Some(a) = arg1 {
            sig += "Z";
            let val_2 = jni::objects::JValueGen::Bool(a.into());
            args.push(val_2);
        }
        sig += ")Z";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "update", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    pub fn location(&self) -> Result<crate::Location<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.location()
    }
    pub fn get_type(&self) -> Result<crate::Material<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.get_type()
    }
    // SUPER CLASS: PersistentDataHolder
    // SUPER CLASS: InventoryHolder

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::block::TileState<'mc>> for Lectern<'mc> {
    fn into(self) -> crate::block::TileState<'mc> {
        crate::block::TileState::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting Lectern into crate::block::TileState")
    }
}
impl<'mc> Into<crate::inventory::BlockInventoryHolder<'mc>> for Lectern<'mc> {
    fn into(self) -> crate::inventory::BlockInventoryHolder<'mc> {
        crate::inventory::BlockInventoryHolder::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting Lectern into crate::inventory::BlockInventoryHolder")
    }
}
/// Represents a captured state of a creature spawner.
///
/// This is a representation of an abstract class.
#[repr(C)]
pub struct CreatureSpawner<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for CreatureSpawner<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for CreatureSpawner<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate CreatureSpawner from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/block/CreatureSpawner")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a CreatureSpawner object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> CreatureSpawner<'mc> {
    pub fn spawned_type(
        &self,
    ) -> Result<crate::entity::EntityType<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/entity/EntityType;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getSpawnedType", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::entity::EntityType::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn set_spawned_type(
        &self,
        arg0: impl Into<crate::entity::EntityType<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/entity/EntityType;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setSpawnedType",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn delay(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getDelay", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    /// Set the spawner's delay.
    ///
    /// If set to -1, the spawn delay will be reset to a random value between <a href="#getMinSpawnDelay()"><code>getMinSpawnDelay()</code></a> and <a href="#getMaxSpawnDelay()"><code>getMaxSpawnDelay()</code></a>.
    pub fn set_delay(&self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(I)V");
        let val_1 = jni::objects::JValueGen::Int(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setDelay",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn set_creature_type_by_name(
        &self,
        arg0: impl Into<String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/String;)V");
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(arg0.into())?,
        ));
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setCreatureTypeByName",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    #[deprecated]

    pub fn creature_type_name(&self) -> Result<Option<String>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/String;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getCreatureTypeName",
            sig.as_str(),
            vec![],
        );
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

    pub fn min_spawn_delay(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getMinSpawnDelay",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    /// Set the minimum spawn delay amount (in ticks).
    pub fn set_min_spawn_delay(&self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(I)V");
        let val_1 = jni::objects::JValueGen::Int(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setMinSpawnDelay",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn max_spawn_delay(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getMaxSpawnDelay",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    /// Set the maximum spawn delay amount (in ticks).
    ///
    /// This value <b>must</b> be greater than 0, as well as greater than or equal to <a href="#getMinSpawnDelay()"><code>getMinSpawnDelay()</code></a>
    pub fn set_max_spawn_delay(&self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(I)V");
        let val_1 = jni::objects::JValueGen::Int(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setMaxSpawnDelay",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn spawn_count(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getSpawnCount", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    /// Set how many mobs attempt to spawn.
    pub fn set_spawn_count(&self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(I)V");
        let val_1 = jni::objects::JValueGen::Int(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setSpawnCount",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn max_nearby_entities(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getMaxNearbyEntities",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    /// Set the maximum number of similar entities that are allowed to be within spawning range of this spawner.
    ///
    /// Similar entities are entities that are of the same <a title="enum in org.bukkit.entity" href="../entity/EntityType.html"><code>EntityType</code></a>
    pub fn set_max_nearby_entities(&self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(I)V");
        let val_1 = jni::objects::JValueGen::Int(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setMaxNearbyEntities",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn required_player_range(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getRequiredPlayerRange",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    /// Set the maximum distance (squared) a player can be in order for this spawner to be active.
    ///
    /// Setting this value to less than or equal to 0 will make this spawner always active (given that there are players online).
    pub fn set_required_player_range(&self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(I)V");
        let val_1 = jni::objects::JValueGen::Int(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setRequiredPlayerRange",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn spawn_range(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getSpawnRange", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    /// Set the new spawn range.
    ///
    pub fn set_spawn_range(&self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(I)V");
        let val_1 = jni::objects::JValueGen::Int(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setSpawnRange",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub fn persistent_data_container(
        &self,
    ) -> Result<crate::persistence::PersistentDataContainer<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = CreatureSpawner::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::TileState = temp_clone.into();
        real.persistent_data_container()
    }
    // SUPER CLASS: BlockState
    pub fn set_metadata(
        &self,
        arg0: impl Into<String>,
        arg1: impl Into<crate::metadata::MetadataValue<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::metadata::Metadatable = temp_clone.into();
        real.set_metadata(arg0, arg1)
    }
    pub fn get_metadata(
        &self,
        arg0: impl Into<String>,
    ) -> Result<Vec<crate::metadata::MetadataValue<'mc>>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::metadata::Metadatable = temp_clone.into();
        real.get_metadata(arg0)
    }
    pub fn has_metadata(
        &self,
        arg0: impl Into<String>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::metadata::Metadatable = temp_clone.into();
        real.has_metadata(arg0)
    }
    pub fn remove_metadata(
        &self,
        arg0: impl Into<String>,
        arg1: impl Into<crate::plugin::Plugin<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::metadata::Metadatable = temp_clone.into();
        real.remove_metadata(arg0, arg1)
    }
    pub fn data(&self) -> Result<crate::material::MaterialData<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.data()
    }
    pub fn world(&self) -> Result<crate::World<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.world()
    }
    pub fn set_type(
        &self,
        arg0: impl Into<crate::Material<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.set_type(arg0)
    }
    pub fn block(&self) -> Result<crate::block::Block<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.block()
    }
    pub fn block_data(
        &self,
    ) -> Result<crate::block::data::BlockData<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.block_data()
    }
    pub fn light_level(&self) -> Result<i8, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.light_level()
    }
    pub fn x(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.x()
    }
    pub fn y(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.y()
    }
    pub fn z(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.z()
    }
    pub fn chunk(&self) -> Result<crate::Chunk<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.chunk()
    }
    pub fn set_data(
        &self,
        arg0: impl Into<crate::material::MaterialData<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.set_data(arg0)
    }
    pub fn set_block_data(
        &self,
        arg0: impl Into<crate::block::data::BlockData<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.set_block_data(arg0)
    }

    pub fn raw_data(&self) -> Result<i8, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.raw_data()
    }

    pub fn set_raw_data(&self, arg0: i8) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.set_raw_data(arg0)
    }
    pub fn is_placed(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.is_placed()
    }
    pub fn update_with_boolean(
        &self,
        arg0: std::option::Option<bool>,
        arg1: std::option::Option<bool>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "Z";
            let val_1 = jni::objects::JValueGen::Bool(a.into());
            args.push(val_1);
        }
        if let Some(a) = arg1 {
            sig += "Z";
            let val_2 = jni::objects::JValueGen::Bool(a.into());
            args.push(val_2);
        }
        sig += ")Z";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "update", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    pub fn location(&self) -> Result<crate::Location<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.location()
    }
    pub fn get_type(&self) -> Result<crate::Material<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.get_type()
    }
    // SUPER CLASS: PersistentDataHolder

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::block::TileState<'mc>> for CreatureSpawner<'mc> {
    fn into(self) -> crate::block::TileState<'mc> {
        crate::block::TileState::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting CreatureSpawner into crate::block::TileState")
    }
}
/// Represents a captured state of an on / off comparator.
///
/// This is a representation of an abstract class.
#[repr(C)]
pub struct Comparator<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for Comparator<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for Comparator<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Comparator from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/block/Comparator")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Comparator object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> Comparator<'mc> {
    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::block::TileState<'mc>> for Comparator<'mc> {
    fn into(self) -> crate::block::TileState<'mc> {
        crate::block::TileState::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting Comparator into crate::block::TileState")
    }
}
/// Represents a captured state of a container block.
///
/// This is a representation of an abstract class.
#[repr(C)]
pub struct Container<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for Container<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for Container<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Container from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/block/Container")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Container object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> Container<'mc> {
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

    pub fn snapshot_inventory(
        &self,
    ) -> Result<crate::inventory::Inventory<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/inventory/Inventory;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getSnapshotInventory",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::Inventory::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    pub fn persistent_data_container(
        &self,
    ) -> Result<crate::persistence::PersistentDataContainer<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = Container::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::TileState = temp_clone.into();
        real.persistent_data_container()
    }
    // SUPER CLASS: BlockState
    pub fn set_metadata(
        &self,
        arg0: impl Into<String>,
        arg1: impl Into<crate::metadata::MetadataValue<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::metadata::Metadatable = temp_clone.into();
        real.set_metadata(arg0, arg1)
    }
    pub fn get_metadata(
        &self,
        arg0: impl Into<String>,
    ) -> Result<Vec<crate::metadata::MetadataValue<'mc>>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::metadata::Metadatable = temp_clone.into();
        real.get_metadata(arg0)
    }
    pub fn has_metadata(
        &self,
        arg0: impl Into<String>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::metadata::Metadatable = temp_clone.into();
        real.has_metadata(arg0)
    }
    pub fn remove_metadata(
        &self,
        arg0: impl Into<String>,
        arg1: impl Into<crate::plugin::Plugin<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::metadata::Metadatable = temp_clone.into();
        real.remove_metadata(arg0, arg1)
    }
    pub fn data(&self) -> Result<crate::material::MaterialData<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.data()
    }
    pub fn world(&self) -> Result<crate::World<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.world()
    }
    pub fn set_type(
        &self,
        arg0: impl Into<crate::Material<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.set_type(arg0)
    }
    pub fn block(&self) -> Result<crate::block::Block<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.block()
    }
    pub fn block_data(
        &self,
    ) -> Result<crate::block::data::BlockData<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.block_data()
    }
    pub fn light_level(&self) -> Result<i8, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.light_level()
    }
    pub fn x(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.x()
    }
    pub fn y(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.y()
    }
    pub fn z(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.z()
    }
    pub fn chunk(&self) -> Result<crate::Chunk<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.chunk()
    }
    pub fn set_data(
        &self,
        arg0: impl Into<crate::material::MaterialData<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.set_data(arg0)
    }
    pub fn set_block_data(
        &self,
        arg0: impl Into<crate::block::data::BlockData<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.set_block_data(arg0)
    }

    pub fn raw_data(&self) -> Result<i8, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.raw_data()
    }

    pub fn set_raw_data(&self, arg0: i8) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.set_raw_data(arg0)
    }
    pub fn is_placed(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.is_placed()
    }
    pub fn update_with_boolean(
        &self,
        arg0: std::option::Option<bool>,
        arg1: std::option::Option<bool>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "Z";
            let val_1 = jni::objects::JValueGen::Bool(a.into());
            args.push(val_1);
        }
        if let Some(a) = arg1 {
            sig += "Z";
            let val_2 = jni::objects::JValueGen::Bool(a.into());
            args.push(val_2);
        }
        sig += ")Z";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "update", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    pub fn location(&self) -> Result<crate::Location<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.location()
    }
    pub fn get_type(&self) -> Result<crate::Material<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.get_type()
    }
    // SUPER CLASS: PersistentDataHolder
    // SUPER CLASS: InventoryHolder
    pub fn is_locked(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = Container::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::Lockable = temp_clone.into();
        real.is_locked()
    }
    pub fn lock(&self) -> Result<String, Box<dyn std::error::Error>> {
        let temp_clone = Container::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::Lockable = temp_clone.into();
        real.lock()
    }
    pub fn set_lock(&self, arg0: impl Into<String>) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = Container::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::Lockable = temp_clone.into();
        real.set_lock(arg0)
    }
    pub fn custom_name(&self) -> Result<Option<String>, Box<dyn std::error::Error>> {
        let temp_clone = Container::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::Nameable = temp_clone.into();
        real.custom_name()
    }
    pub fn set_custom_name(
        &self,
        arg0: impl Into<String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = Container::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::Nameable = temp_clone.into();
        real.set_custom_name(arg0)
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::block::TileState<'mc>> for Container<'mc> {
    fn into(self) -> crate::block::TileState<'mc> {
        crate::block::TileState::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting Container into crate::block::TileState")
    }
}
impl<'mc> Into<crate::inventory::BlockInventoryHolder<'mc>> for Container<'mc> {
    fn into(self) -> crate::inventory::BlockInventoryHolder<'mc> {
        crate::inventory::BlockInventoryHolder::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting Container into crate::inventory::BlockInventoryHolder")
    }
}
impl<'mc> Into<crate::block::Lockable<'mc>> for Container<'mc> {
    fn into(self) -> crate::block::Lockable<'mc> {
        crate::block::Lockable::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting Container into crate::block::Lockable")
    }
}
impl<'mc> Into<crate::Nameable<'mc>> for Container<'mc> {
    fn into(self) -> crate::Nameable<'mc> {
        crate::Nameable::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting Container into crate::Nameable")
    }
}
/// Represents a captured state of an end gateway.
///
/// This is a representation of an abstract class.
#[repr(C)]
pub struct EndGateway<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for EndGateway<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for EndGateway<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate EndGateway from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/block/EndGateway")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a EndGateway object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> EndGateway<'mc> {
    pub fn exit_location(
        &self,
    ) -> Result<Option<crate::Location<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/Location;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getExitLocation", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {
            return Ok(None);
        }
        Ok(Some(crate::Location::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })?))
    }

    pub fn set_exit_location(
        &self,
        arg0: impl Into<crate::Location<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/Location;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setExitLocation",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn is_exact_teleport(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isExactTeleport", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Sets whether this gateway will teleport entities directly to the exit location instead of finding a nearby location.
    pub fn set_exact_teleport(&self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        let val_1 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setExactTeleport",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn age(&self) -> Result<i64, Box<dyn std::error::Error>> {
        let sig = String::from("()J");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getAge", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.j()?)
    }
    /// Sets the age in ticks of the gateway.
    ///
    /// If the age is less than 200 ticks a magenta beam will be emitted, whilst if it is a multiple of 2400 ticks a purple beam will be emitted.
    pub fn set_age(&self, arg0: i64) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(J)V");
        let val_1 = jni::objects::JValueGen::Long(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setAge",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub fn persistent_data_container(
        &self,
    ) -> Result<crate::persistence::PersistentDataContainer<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = EndGateway::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::TileState = temp_clone.into();
        real.persistent_data_container()
    }
    // SUPER CLASS: BlockState
    pub fn set_metadata(
        &self,
        arg0: impl Into<String>,
        arg1: impl Into<crate::metadata::MetadataValue<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::metadata::Metadatable = temp_clone.into();
        real.set_metadata(arg0, arg1)
    }
    pub fn get_metadata(
        &self,
        arg0: impl Into<String>,
    ) -> Result<Vec<crate::metadata::MetadataValue<'mc>>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::metadata::Metadatable = temp_clone.into();
        real.get_metadata(arg0)
    }
    pub fn has_metadata(
        &self,
        arg0: impl Into<String>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::metadata::Metadatable = temp_clone.into();
        real.has_metadata(arg0)
    }
    pub fn remove_metadata(
        &self,
        arg0: impl Into<String>,
        arg1: impl Into<crate::plugin::Plugin<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::metadata::Metadatable = temp_clone.into();
        real.remove_metadata(arg0, arg1)
    }
    pub fn data(&self) -> Result<crate::material::MaterialData<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.data()
    }
    pub fn world(&self) -> Result<crate::World<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.world()
    }
    pub fn set_type(
        &self,
        arg0: impl Into<crate::Material<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.set_type(arg0)
    }
    pub fn block(&self) -> Result<crate::block::Block<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.block()
    }
    pub fn block_data(
        &self,
    ) -> Result<crate::block::data::BlockData<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.block_data()
    }
    pub fn light_level(&self) -> Result<i8, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.light_level()
    }
    pub fn x(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.x()
    }
    pub fn y(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.y()
    }
    pub fn z(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.z()
    }
    pub fn chunk(&self) -> Result<crate::Chunk<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.chunk()
    }
    pub fn set_data(
        &self,
        arg0: impl Into<crate::material::MaterialData<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.set_data(arg0)
    }
    pub fn set_block_data(
        &self,
        arg0: impl Into<crate::block::data::BlockData<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.set_block_data(arg0)
    }

    pub fn raw_data(&self) -> Result<i8, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.raw_data()
    }

    pub fn set_raw_data(&self, arg0: i8) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.set_raw_data(arg0)
    }
    pub fn is_placed(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.is_placed()
    }
    pub fn update_with_boolean(
        &self,
        arg0: std::option::Option<bool>,
        arg1: std::option::Option<bool>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "Z";
            let val_1 = jni::objects::JValueGen::Bool(a.into());
            args.push(val_1);
        }
        if let Some(a) = arg1 {
            sig += "Z";
            let val_2 = jni::objects::JValueGen::Bool(a.into());
            args.push(val_2);
        }
        sig += ")Z";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "update", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    pub fn location(&self) -> Result<crate::Location<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.location()
    }
    pub fn get_type(&self) -> Result<crate::Material<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.get_type()
    }
    // SUPER CLASS: PersistentDataHolder

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::block::TileState<'mc>> for EndGateway<'mc> {
    fn into(self) -> crate::block::TileState<'mc> {
        crate::block::TileState::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting EndGateway into crate::block::TileState")
    }
}
/// Represents a captured state of a blast furnace.
///
/// This is a representation of an abstract class.
#[repr(C)]
pub struct BlastFurnace<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for BlastFurnace<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for BlastFurnace<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate BlastFurnace from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/block/BlastFurnace")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a BlastFurnace object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> BlastFurnace<'mc> {
    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::block::Furnace<'mc>> for BlastFurnace<'mc> {
    fn into(self) -> crate::block::Furnace<'mc> {
        crate::block::Furnace::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting BlastFurnace into crate::block::Furnace")
    }
}
pub enum PistonMoveReaction<'mc> {
    VariantMove {
        inner: PistonMoveReactionStruct<'mc>,
    },
    VariantBreak {
        inner: PistonMoveReactionStruct<'mc>,
    },
    Block {
        inner: PistonMoveReactionStruct<'mc>,
    },
    Ignore {
        inner: PistonMoveReactionStruct<'mc>,
    },
    PushOnly {
        inner: PistonMoveReactionStruct<'mc>,
    },
}
impl<'mc> std::fmt::Display for PistonMoveReaction<'mc> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PistonMoveReaction::VariantMove { .. } => f.write_str("MOVE"),
            PistonMoveReaction::VariantBreak { .. } => f.write_str("BREAK"),
            PistonMoveReaction::Block { .. } => f.write_str("BLOCK"),
            PistonMoveReaction::Ignore { .. } => f.write_str("IGNORE"),
            PistonMoveReaction::PushOnly { .. } => f.write_str("PUSH_ONLY"),
        }
    }
}

impl<'mc> PistonMoveReaction<'mc> {
    pub fn value_of(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<String>,
    ) -> Result<PistonMoveReaction<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(env.new_string(arg0.into())?);
        let cls = env.find_class("org/bukkit/block/PistonMoveReaction");
        let cls = env.translate_error_with_class(cls)?;
        let res = env.call_static_method(
            cls,
            "valueOf",
            "(Ljava/lang/String;)Lorg/bukkit/block/PistonMoveReaction;",
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
            "MOVE" => Ok(PistonMoveReaction::VariantMove {
                inner: PistonMoveReactionStruct::from_raw(env, obj)?,
            }),
            "BREAK" => Ok(PistonMoveReaction::VariantBreak {
                inner: PistonMoveReactionStruct::from_raw(env, obj)?,
            }),
            "BLOCK" => Ok(PistonMoveReaction::Block {
                inner: PistonMoveReactionStruct::from_raw(env, obj)?,
            }),
            "IGNORE" => Ok(PistonMoveReaction::Ignore {
                inner: PistonMoveReactionStruct::from_raw(env, obj)?,
            }),
            "PUSH_ONLY" => Ok(PistonMoveReaction::PushOnly {
                inner: PistonMoveReactionStruct::from_raw(env, obj)?,
            }),

            _ => Err(eyre::eyre!("String gaven for variant was invalid").into()),
        }
    }
}

#[repr(C)]
pub struct PistonMoveReactionStruct<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for PistonMoveReaction<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        match self {
            Self::VariantMove { inner } => inner.0.clone(),
            Self::VariantBreak { inner } => inner.0.clone(),
            Self::Block { inner } => inner.0.clone(),
            Self::Ignore { inner } => inner.0.clone(),
            Self::PushOnly { inner } => inner.0.clone(),
        }
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        match self {
            Self::VariantMove { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::VariantBreak { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::Block { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Ignore { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::PushOnly { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
        }
    }
}
impl<'mc> JNIInstantiatable<'mc> for PistonMoveReaction<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate PistonMoveReaction from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/block/PistonMoveReaction")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a PistonMoveReaction object, got {}",
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
                "MOVE" => Ok(PistonMoveReaction::VariantMove {
                    inner: PistonMoveReactionStruct::from_raw(env, obj)?,
                }),
                "BREAK" => Ok(PistonMoveReaction::VariantBreak {
                    inner: PistonMoveReactionStruct::from_raw(env, obj)?,
                }),
                "BLOCK" => Ok(PistonMoveReaction::Block {
                    inner: PistonMoveReactionStruct::from_raw(env, obj)?,
                }),
                "IGNORE" => Ok(PistonMoveReaction::Ignore {
                    inner: PistonMoveReactionStruct::from_raw(env, obj)?,
                }),
                "PUSH_ONLY" => Ok(PistonMoveReaction::PushOnly {
                    inner: PistonMoveReactionStruct::from_raw(env, obj)?,
                }),
                _ => Err(eyre::eyre!("String gaven for variant was invalid").into()),
            }
        }
    }
}

impl<'mc> JNIRaw<'mc> for PistonMoveReactionStruct<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for PistonMoveReactionStruct<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate PistonMoveReactionStruct from null object."
            )
            .into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/block/PistonMoveReaction")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a PistonMoveReactionStruct object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> PistonMoveReactionStruct<'mc> {
    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}

///
/// This is a representation of an abstract class.
#[repr(C)]
pub struct Lidded<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for Lidded<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for Lidded<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Lidded from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/block/Lidded")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Lidded object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> Lidded<'mc> {
    pub fn close(&self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "close", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn open(&self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "open", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
/// Represents a captured state of an enchanting table.
///
/// This is a representation of an abstract class.
#[repr(C)]
pub struct EnchantingTable<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for EnchantingTable<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for EnchantingTable<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate EnchantingTable from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/block/EnchantingTable")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a EnchantingTable object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> EnchantingTable<'mc> {
    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::block::TileState<'mc>> for EnchantingTable<'mc> {
    fn into(self) -> crate::block::TileState<'mc> {
        crate::block::TileState::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting EnchantingTable into crate::block::TileState")
    }
}
impl<'mc> Into<crate::Nameable<'mc>> for EnchantingTable<'mc> {
    fn into(self) -> crate::Nameable<'mc> {
        crate::Nameable::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting EnchantingTable into crate::Nameable")
    }
}
/// Represents a captured state of a ShulkerBox.
///
/// This is a representation of an abstract class.
#[repr(C)]
pub struct ShulkerBox<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for ShulkerBox<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for ShulkerBox<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate ShulkerBox from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/block/ShulkerBox")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a ShulkerBox object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> ShulkerBox<'mc> {
    pub fn color(&self) -> Result<Option<crate::DyeColor<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/DyeColor;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getColor", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {
            return Ok(None);
        }
        Ok(Some(crate::DyeColor::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })?))
    }
    pub fn inventory(
        &self,
    ) -> Result<crate::inventory::Inventory<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = ShulkerBox::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::Container = temp_clone.into();
        real.inventory()
    }
    pub fn snapshot_inventory(
        &self,
    ) -> Result<crate::inventory::Inventory<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = ShulkerBox::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::Container = temp_clone.into();
        real.snapshot_inventory()
    }
    // SUPER CLASS: TileState
    pub fn data(&self) -> Result<crate::material::MaterialData<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::TileState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.data()
    }
    pub fn world(&self) -> Result<crate::World<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::TileState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.world()
    }
    pub fn set_type(
        &self,
        arg0: impl Into<crate::Material<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::block::TileState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.set_type(arg0)
    }
    pub fn block(&self) -> Result<crate::block::Block<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::TileState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.block()
    }
    pub fn block_data(
        &self,
    ) -> Result<crate::block::data::BlockData<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::TileState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.block_data()
    }
    pub fn light_level(&self) -> Result<i8, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::TileState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.light_level()
    }
    pub fn x(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::TileState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.x()
    }
    pub fn y(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::TileState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.y()
    }
    pub fn z(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::TileState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.z()
    }
    pub fn chunk(&self) -> Result<crate::Chunk<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::TileState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.chunk()
    }
    pub fn set_data(
        &self,
        arg0: impl Into<crate::material::MaterialData<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::block::TileState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.set_data(arg0)
    }
    pub fn set_block_data(
        &self,
        arg0: impl Into<crate::block::data::BlockData<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::block::TileState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.set_block_data(arg0)
    }

    pub fn raw_data(&self) -> Result<i8, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::TileState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.raw_data()
    }

    pub fn set_raw_data(&self, arg0: i8) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::block::TileState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.set_raw_data(arg0)
    }
    pub fn is_placed(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::TileState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.is_placed()
    }
    pub fn update_with_boolean(
        &self,
        arg0: std::option::Option<bool>,
        arg1: std::option::Option<bool>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "Z";
            let val_1 = jni::objects::JValueGen::Bool(a.into());
            args.push(val_1);
        }
        if let Some(a) = arg1 {
            sig += "Z";
            let val_2 = jni::objects::JValueGen::Bool(a.into());
            args.push(val_2);
        }
        sig += ")Z";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "update", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    pub fn location(&self) -> Result<crate::Location<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::TileState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.location()
    }
    pub fn get_type(&self) -> Result<crate::Material<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::TileState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.get_type()
    }
    // SUPER CLASS: Metadatable
    pub fn set_metadata(
        &self,
        arg0: impl Into<String>,
        arg1: impl Into<crate::metadata::MetadataValue<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::metadata::Metadatable::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::metadata::Metadatable = temp_clone.into();
        real.set_metadata(arg0, arg1)
    }
    pub fn get_metadata(
        &self,
        arg0: impl Into<String>,
    ) -> Result<Vec<crate::metadata::MetadataValue<'mc>>, Box<dyn std::error::Error>> {
        let temp_clone = crate::metadata::Metadatable::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::metadata::Metadatable = temp_clone.into();
        real.get_metadata(arg0)
    }
    pub fn has_metadata(
        &self,
        arg0: impl Into<String>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::metadata::Metadatable::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::metadata::Metadatable = temp_clone.into();
        real.has_metadata(arg0)
    }
    pub fn remove_metadata(
        &self,
        arg0: impl Into<String>,
        arg1: impl Into<crate::plugin::Plugin<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::metadata::Metadatable::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::metadata::Metadatable = temp_clone.into();
        real.remove_metadata(arg0, arg1)
    }
    pub fn persistent_data_container(
        &self,
    ) -> Result<crate::persistence::PersistentDataContainer<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::TileState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::persistence::PersistentDataHolder = temp_clone.into();
        real.persistent_data_container()
    }
    // SUPER CLASS: BlockInventoryHolder
    // SUPER CLASS: Lockable
    pub fn is_locked(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::Lockable::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::Lockable = temp_clone.into();
        real.is_locked()
    }
    pub fn lock(&self) -> Result<String, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::Lockable::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::Lockable = temp_clone.into();
        real.lock()
    }
    pub fn set_lock(&self, arg0: impl Into<String>) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::block::Lockable::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::Lockable = temp_clone.into();
        real.set_lock(arg0)
    }
    // SUPER CLASS: Nameable
    pub fn custom_name(&self) -> Result<Option<String>, Box<dyn std::error::Error>> {
        let temp_clone = crate::Nameable::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::Nameable = temp_clone.into();
        real.custom_name()
    }
    pub fn set_custom_name(
        &self,
        arg0: impl Into<String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::Nameable::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::Nameable = temp_clone.into();
        real.set_custom_name(arg0)
    }
    pub fn seed(&self) -> Result<i64, Box<dyn std::error::Error>> {
        let temp_clone = ShulkerBox::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::loot::Lootable = temp_clone.into();
        real.seed()
    }
    pub fn set_seed(&self, arg0: i64) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = ShulkerBox::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::loot::Lootable = temp_clone.into();
        real.set_seed(arg0)
    }
    pub fn set_loot_table(
        &self,
        arg0: impl Into<crate::loot::LootTable<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = ShulkerBox::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::loot::Lootable = temp_clone.into();
        real.set_loot_table(arg0)
    }
    pub fn loot_table(
        &self,
    ) -> Result<Option<crate::loot::LootTable<'mc>>, Box<dyn std::error::Error>> {
        let temp_clone = ShulkerBox::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::loot::Lootable = temp_clone.into();
        real.loot_table()
    }
    pub fn close(&self) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = ShulkerBox::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::Lidded = temp_clone.into();
        real.close()
    }
    pub fn open(&self) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = ShulkerBox::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::Lidded = temp_clone.into();
        real.open()
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::block::Container<'mc>> for ShulkerBox<'mc> {
    fn into(self) -> crate::block::Container<'mc> {
        crate::block::Container::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting ShulkerBox into crate::block::Container")
    }
}
impl<'mc> Into<crate::loot::Lootable<'mc>> for ShulkerBox<'mc> {
    fn into(self) -> crate::loot::Lootable<'mc> {
        crate::loot::Lootable::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting ShulkerBox into crate::loot::Lootable")
    }
}
impl<'mc> Into<crate::block::Lidded<'mc>> for ShulkerBox<'mc> {
    fn into(self) -> crate::block::Lidded<'mc> {
        crate::block::Lidded::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting ShulkerBox into crate::block::Lidded")
    }
}
/// Represents a captured state of a block which stores entities.
///
/// This is a representation of an abstract class.
#[repr(C)]
pub struct EntityBlockStorage<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for EntityBlockStorage<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for EntityBlockStorage<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate EntityBlockStorage from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/block/EntityBlockStorage")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a EntityBlockStorage object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> EntityBlockStorage<'mc> {
    pub fn is_full(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isFull", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn entity_count(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getEntityCount", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }

    pub fn max_entities(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getMaxEntities", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    /// Set the maximum amount of entities this block can hold.
    pub fn set_max_entities(&self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(I)V");
        let val_1 = jni::objects::JValueGen::Int(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setMaxEntities",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn release_entities(
        &self,
    ) -> Result<Vec<jni::objects::JObject<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/List;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "releaseEntities", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let list = blackboxmc_java::util::JavaList::from_raw(&self.jni_ref(), res.l()?)?;
        let iter = list.iterator()?;
        while iter.has_next()? {
            let obj = iter.next()?;
            new_vec.push(obj);
        }
        Ok(new_vec)
    }

    pub fn add_entity(
        &self,
        arg0: impl Into<crate::entity::Entity<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/entity/Entity;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "addEntity",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub fn persistent_data_container(
        &self,
    ) -> Result<crate::persistence::PersistentDataContainer<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = EntityBlockStorage::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::TileState = temp_clone.into();
        real.persistent_data_container()
    }
    // SUPER CLASS: BlockState
    pub fn set_metadata(
        &self,
        arg0: impl Into<String>,
        arg1: impl Into<crate::metadata::MetadataValue<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::metadata::Metadatable = temp_clone.into();
        real.set_metadata(arg0, arg1)
    }
    pub fn get_metadata(
        &self,
        arg0: impl Into<String>,
    ) -> Result<Vec<crate::metadata::MetadataValue<'mc>>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::metadata::Metadatable = temp_clone.into();
        real.get_metadata(arg0)
    }
    pub fn has_metadata(
        &self,
        arg0: impl Into<String>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::metadata::Metadatable = temp_clone.into();
        real.has_metadata(arg0)
    }
    pub fn remove_metadata(
        &self,
        arg0: impl Into<String>,
        arg1: impl Into<crate::plugin::Plugin<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::metadata::Metadatable = temp_clone.into();
        real.remove_metadata(arg0, arg1)
    }
    pub fn data(&self) -> Result<crate::material::MaterialData<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.data()
    }
    pub fn world(&self) -> Result<crate::World<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.world()
    }
    pub fn set_type(
        &self,
        arg0: impl Into<crate::Material<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.set_type(arg0)
    }
    pub fn block(&self) -> Result<crate::block::Block<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.block()
    }
    pub fn block_data(
        &self,
    ) -> Result<crate::block::data::BlockData<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.block_data()
    }
    pub fn light_level(&self) -> Result<i8, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.light_level()
    }
    pub fn x(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.x()
    }
    pub fn y(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.y()
    }
    pub fn z(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.z()
    }
    pub fn chunk(&self) -> Result<crate::Chunk<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.chunk()
    }
    pub fn set_data(
        &self,
        arg0: impl Into<crate::material::MaterialData<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.set_data(arg0)
    }
    pub fn set_block_data(
        &self,
        arg0: impl Into<crate::block::data::BlockData<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.set_block_data(arg0)
    }

    pub fn raw_data(&self) -> Result<i8, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.raw_data()
    }

    pub fn set_raw_data(&self, arg0: i8) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.set_raw_data(arg0)
    }
    pub fn is_placed(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.is_placed()
    }
    pub fn update_with_boolean(
        &self,
        arg0: std::option::Option<bool>,
        arg1: std::option::Option<bool>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "Z";
            let val_1 = jni::objects::JValueGen::Bool(a.into());
            args.push(val_1);
        }
        if let Some(a) = arg1 {
            sig += "Z";
            let val_2 = jni::objects::JValueGen::Bool(a.into());
            args.push(val_2);
        }
        sig += ")Z";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "update", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    pub fn location(&self) -> Result<crate::Location<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.location()
    }
    pub fn get_type(&self) -> Result<crate::Material<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.get_type()
    }
    // SUPER CLASS: PersistentDataHolder

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::block::TileState<'mc>> for EntityBlockStorage<'mc> {
    fn into(self) -> crate::block::TileState<'mc> {
        crate::block::TileState::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting EntityBlockStorage into crate::block::TileState")
    }
}
/// Represents a captured state of a bee hive.
///
/// This is a representation of an abstract class.
#[repr(C)]
pub struct Beehive<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for Beehive<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for Beehive<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Beehive from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/block/Beehive")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Beehive object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> Beehive<'mc> {
    pub fn flower(&self) -> Result<Option<crate::Location<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/Location;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getFlower", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {
            return Ok(None);
        }
        Ok(Some(crate::Location::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })?))
    }

    pub fn set_flower(
        &self,
        arg0: impl Into<crate::Location<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/Location;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setFlower",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn is_sedated(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isSedated", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    pub fn is_full(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = Beehive::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::EntityBlockStorage = temp_clone.into();
        real.is_full()
    }
    pub fn entity_count(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let temp_clone = Beehive::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::EntityBlockStorage = temp_clone.into();
        real.entity_count()
    }
    pub fn max_entities(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getMaxEntities", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    pub fn set_max_entities(&self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(I)V");
        let val_1 = jni::objects::JValueGen::Int(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setMaxEntities",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub fn release_entities(
        &self,
    ) -> Result<Vec<jni::objects::JObject<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/List;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "releaseEntities", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let list = blackboxmc_java::util::JavaList::from_raw(&self.jni_ref(), res.l()?)?;
        let iter = list.iterator()?;
        while iter.has_next()? {
            let obj = iter.next()?;
            new_vec.push(obj);
        }
        Ok(new_vec)
    }
    pub fn add_entity(
        &self,
        arg0: impl Into<crate::entity::Entity<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = Beehive::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::EntityBlockStorage = temp_clone.into();
        real.add_entity(arg0)
    }
    // SUPER CLASS: TileState
    pub fn data(&self) -> Result<crate::material::MaterialData<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::TileState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.data()
    }
    pub fn world(&self) -> Result<crate::World<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::TileState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.world()
    }
    pub fn set_type(
        &self,
        arg0: impl Into<crate::Material<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::block::TileState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.set_type(arg0)
    }
    pub fn block(&self) -> Result<crate::block::Block<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::TileState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.block()
    }
    pub fn block_data(
        &self,
    ) -> Result<crate::block::data::BlockData<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::TileState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.block_data()
    }
    pub fn light_level(&self) -> Result<i8, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::TileState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.light_level()
    }
    pub fn x(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::TileState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.x()
    }
    pub fn y(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::TileState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.y()
    }
    pub fn z(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::TileState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.z()
    }
    pub fn chunk(&self) -> Result<crate::Chunk<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::TileState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.chunk()
    }
    pub fn set_data(
        &self,
        arg0: impl Into<crate::material::MaterialData<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::block::TileState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.set_data(arg0)
    }
    pub fn set_block_data(
        &self,
        arg0: impl Into<crate::block::data::BlockData<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::block::TileState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.set_block_data(arg0)
    }

    pub fn raw_data(&self) -> Result<i8, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::TileState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.raw_data()
    }

    pub fn set_raw_data(&self, arg0: i8) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::block::TileState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.set_raw_data(arg0)
    }
    pub fn is_placed(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::TileState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.is_placed()
    }
    pub fn update_with_boolean(
        &self,
        arg0: std::option::Option<bool>,
        arg1: std::option::Option<bool>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "Z";
            let val_1 = jni::objects::JValueGen::Bool(a.into());
            args.push(val_1);
        }
        if let Some(a) = arg1 {
            sig += "Z";
            let val_2 = jni::objects::JValueGen::Bool(a.into());
            args.push(val_2);
        }
        sig += ")Z";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "update", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    pub fn location(&self) -> Result<crate::Location<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::TileState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.location()
    }
    pub fn get_type(&self) -> Result<crate::Material<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::TileState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.get_type()
    }
    // SUPER CLASS: Metadatable
    pub fn set_metadata(
        &self,
        arg0: impl Into<String>,
        arg1: impl Into<crate::metadata::MetadataValue<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::metadata::Metadatable::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::metadata::Metadatable = temp_clone.into();
        real.set_metadata(arg0, arg1)
    }
    pub fn get_metadata(
        &self,
        arg0: impl Into<String>,
    ) -> Result<Vec<crate::metadata::MetadataValue<'mc>>, Box<dyn std::error::Error>> {
        let temp_clone = crate::metadata::Metadatable::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::metadata::Metadatable = temp_clone.into();
        real.get_metadata(arg0)
    }
    pub fn has_metadata(
        &self,
        arg0: impl Into<String>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::metadata::Metadatable::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::metadata::Metadatable = temp_clone.into();
        real.has_metadata(arg0)
    }
    pub fn remove_metadata(
        &self,
        arg0: impl Into<String>,
        arg1: impl Into<crate::plugin::Plugin<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::metadata::Metadatable::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::metadata::Metadatable = temp_clone.into();
        real.remove_metadata(arg0, arg1)
    }
    pub fn persistent_data_container(
        &self,
    ) -> Result<crate::persistence::PersistentDataContainer<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::TileState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::persistence::PersistentDataHolder = temp_clone.into();
        real.persistent_data_container()
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::block::EntityBlockStorage<'mc>> for Beehive<'mc> {
    fn into(self) -> crate::block::EntityBlockStorage<'mc> {
        crate::block::EntityBlockStorage::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting Beehive into crate::block::EntityBlockStorage")
    }
}
/// A side on a decorated pot. Sides are relative to the facing state of a <a title="interface in org.bukkit.block.data.type" href="data/type/DecoratedPot.html"><code>DecoratedPot</code></a>.
pub enum DecoratedPotSide<'mc> {
    Back { inner: DecoratedPotSideStruct<'mc> },
    Left { inner: DecoratedPotSideStruct<'mc> },
    Right { inner: DecoratedPotSideStruct<'mc> },
    Front { inner: DecoratedPotSideStruct<'mc> },
}
impl<'mc> std::fmt::Display for DecoratedPotSide<'mc> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DecoratedPotSide::Back { .. } => f.write_str("BACK"),
            DecoratedPotSide::Left { .. } => f.write_str("LEFT"),
            DecoratedPotSide::Right { .. } => f.write_str("RIGHT"),
            DecoratedPotSide::Front { .. } => f.write_str("FRONT"),
        }
    }
}

impl<'mc> DecoratedPotSide<'mc> {
    pub fn value_of(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<String>,
    ) -> Result<DecoratedPotSide<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(env.new_string(arg0.into())?);
        let cls = env.find_class("org/bukkit/block/DecoratedPot$Side");
        let cls = env.translate_error_with_class(cls)?;
        let res = env.call_static_method(
            cls,
            "valueOf",
            "(Ljava/lang/String;)Lorg/bukkit/block/DecoratedPot$Side;",
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
            "BACK" => Ok(DecoratedPotSide::Back {
                inner: DecoratedPotSideStruct::from_raw(env, obj)?,
            }),
            "LEFT" => Ok(DecoratedPotSide::Left {
                inner: DecoratedPotSideStruct::from_raw(env, obj)?,
            }),
            "RIGHT" => Ok(DecoratedPotSide::Right {
                inner: DecoratedPotSideStruct::from_raw(env, obj)?,
            }),
            "FRONT" => Ok(DecoratedPotSide::Front {
                inner: DecoratedPotSideStruct::from_raw(env, obj)?,
            }),

            _ => Err(eyre::eyre!("String gaven for variant was invalid").into()),
        }
    }
}

#[repr(C)]
pub struct DecoratedPotSideStruct<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for DecoratedPotSide<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        match self {
            Self::Back { inner } => inner.0.clone(),
            Self::Left { inner } => inner.0.clone(),
            Self::Right { inner } => inner.0.clone(),
            Self::Front { inner } => inner.0.clone(),
        }
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        match self {
            Self::Back { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Left { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Right { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Front { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
        }
    }
}
impl<'mc> JNIInstantiatable<'mc> for DecoratedPotSide<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate DecoratedPotSide from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/block/DecoratedPot$Side")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a DecoratedPotSide object, got {}",
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
                "BACK" => Ok(DecoratedPotSide::Back {
                    inner: DecoratedPotSideStruct::from_raw(env, obj)?,
                }),
                "LEFT" => Ok(DecoratedPotSide::Left {
                    inner: DecoratedPotSideStruct::from_raw(env, obj)?,
                }),
                "RIGHT" => Ok(DecoratedPotSide::Right {
                    inner: DecoratedPotSideStruct::from_raw(env, obj)?,
                }),
                "FRONT" => Ok(DecoratedPotSide::Front {
                    inner: DecoratedPotSideStruct::from_raw(env, obj)?,
                }),
                _ => Err(eyre::eyre!("String gaven for variant was invalid").into()),
            }
        }
    }
}

impl<'mc> JNIRaw<'mc> for DecoratedPotSideStruct<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for DecoratedPotSideStruct<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate DecoratedPotSideStruct from null object."
            )
            .into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/block/DecoratedPot$Side")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a DecoratedPotSideStruct object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> DecoratedPotSideStruct<'mc> {
    pub fn values(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<Vec<crate::block::DecoratedPotSide<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/block/DecoratedPot$Side;");
        let cls = jni.find_class("org/bukkit/block/DecoratedPot$Side");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "values", sig.as_str(), vec![]);
        let res = jni.translate_error(res)?;
        let arr = Into::<jni::objects::JObjectArray>::into(res.l()?);
        let len = jni.get_array_length(&arr)?;
        let mut vec = Vec::new();
        for i in 0..len {
            let res = jni.get_object_array_element(&arr, i)?;
            vec.push({ crate::block::DecoratedPotSide::from_raw(&jni, res)? });
        }
        Ok(vec)
    }
    // SUPER CLASS: Enum

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
/// Represents a captured state of a sculk catalyst.
///
/// This is a representation of an abstract class.
#[repr(C)]
pub struct SculkCatalyst<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for SculkCatalyst<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for SculkCatalyst<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate SculkCatalyst from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/block/SculkCatalyst")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a SculkCatalyst object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> SculkCatalyst<'mc> {
    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::block::TileState<'mc>> for SculkCatalyst<'mc> {
    fn into(self) -> crate::block::TileState<'mc> {
        crate::block::TileState::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting SculkCatalyst into crate::block::TileState")
    }
}
/// Represents a captured state of a furnace.
///
/// This is a representation of an abstract class.
#[repr(C)]
pub struct Furnace<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for Furnace<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for Furnace<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Furnace from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/block/Furnace")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Furnace object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> Furnace<'mc> {
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

    pub fn snapshot_inventory(
        &self,
    ) -> Result<crate::inventory::FurnaceInventory<'mc>, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Lorg/bukkit/inventory/FurnaceInventory;";
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getSnapshotInventory",
            sig.as_str(),
            args,
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::FurnaceInventory::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn cook_time(&self) -> Result<i16, Box<dyn std::error::Error>> {
        let sig = String::from("()S");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getCookTime", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.s()?)
    }
    /// Set cook time. This is the amount of time the item has been cooking for.
    pub fn set_cook_time(&self, arg0: i16) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(S)V");
        let val_1 = jni::objects::JValueGen::Short(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setCookTime",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn cook_time_total(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getCookTimeTotal",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    /// Set cook time. This is the amount of time the item is required to cook for.
    pub fn set_cook_time_total(&self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(I)V");
        let val_1 = jni::objects::JValueGen::Int(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setCookTimeTotal",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn burn_time(&self) -> Result<i16, Box<dyn std::error::Error>> {
        let sig = String::from("()S");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getBurnTime", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.s()?)
    }
    /// Set burn time. A burn time greater than 0 will cause this block to be lit, whilst a time less than 0 will extinguish it.
    pub fn set_burn_time(&self, arg0: i16) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(S)V");
        let val_1 = jni::objects::JValueGen::Short(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setBurnTime",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn recipes_used(
        &self,
    ) -> Result<blackboxmc_java::util::JavaMap<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/Map;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getRecipesUsed", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::util::JavaMap::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    // SUPER CLASS: TileState
    pub fn data(&self) -> Result<crate::material::MaterialData<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::TileState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.data()
    }
    pub fn world(&self) -> Result<crate::World<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::TileState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.world()
    }
    pub fn set_type(
        &self,
        arg0: impl Into<crate::Material<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::block::TileState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.set_type(arg0)
    }
    pub fn block(&self) -> Result<crate::block::Block<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::TileState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.block()
    }
    pub fn block_data(
        &self,
    ) -> Result<crate::block::data::BlockData<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::TileState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.block_data()
    }
    pub fn light_level(&self) -> Result<i8, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::TileState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.light_level()
    }
    pub fn x(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::TileState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.x()
    }
    pub fn y(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::TileState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.y()
    }
    pub fn z(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::TileState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.z()
    }
    pub fn chunk(&self) -> Result<crate::Chunk<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::TileState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.chunk()
    }
    pub fn set_data(
        &self,
        arg0: impl Into<crate::material::MaterialData<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::block::TileState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.set_data(arg0)
    }
    pub fn set_block_data(
        &self,
        arg0: impl Into<crate::block::data::BlockData<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::block::TileState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.set_block_data(arg0)
    }

    pub fn raw_data(&self) -> Result<i8, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::TileState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.raw_data()
    }

    pub fn set_raw_data(&self, arg0: i8) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::block::TileState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.set_raw_data(arg0)
    }
    pub fn is_placed(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::TileState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.is_placed()
    }
    pub fn update_with_boolean(
        &self,
        arg0: std::option::Option<bool>,
        arg1: std::option::Option<bool>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "Z";
            let val_1 = jni::objects::JValueGen::Bool(a.into());
            args.push(val_1);
        }
        if let Some(a) = arg1 {
            sig += "Z";
            let val_2 = jni::objects::JValueGen::Bool(a.into());
            args.push(val_2);
        }
        sig += ")Z";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "update", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    pub fn location(&self) -> Result<crate::Location<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::TileState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.location()
    }
    pub fn get_type(&self) -> Result<crate::Material<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::TileState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.get_type()
    }
    // SUPER CLASS: Metadatable
    pub fn set_metadata(
        &self,
        arg0: impl Into<String>,
        arg1: impl Into<crate::metadata::MetadataValue<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::metadata::Metadatable::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::metadata::Metadatable = temp_clone.into();
        real.set_metadata(arg0, arg1)
    }
    pub fn get_metadata(
        &self,
        arg0: impl Into<String>,
    ) -> Result<Vec<crate::metadata::MetadataValue<'mc>>, Box<dyn std::error::Error>> {
        let temp_clone = crate::metadata::Metadatable::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::metadata::Metadatable = temp_clone.into();
        real.get_metadata(arg0)
    }
    pub fn has_metadata(
        &self,
        arg0: impl Into<String>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::metadata::Metadatable::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::metadata::Metadatable = temp_clone.into();
        real.has_metadata(arg0)
    }
    pub fn remove_metadata(
        &self,
        arg0: impl Into<String>,
        arg1: impl Into<crate::plugin::Plugin<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::metadata::Metadatable::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::metadata::Metadatable = temp_clone.into();
        real.remove_metadata(arg0, arg1)
    }
    pub fn persistent_data_container(
        &self,
    ) -> Result<crate::persistence::PersistentDataContainer<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::TileState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::persistence::PersistentDataHolder = temp_clone.into();
        real.persistent_data_container()
    }
    // SUPER CLASS: BlockInventoryHolder
    // SUPER CLASS: Lockable
    pub fn is_locked(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::Lockable::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::Lockable = temp_clone.into();
        real.is_locked()
    }
    pub fn lock(&self) -> Result<String, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::Lockable::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::Lockable = temp_clone.into();
        real.lock()
    }
    pub fn set_lock(&self, arg0: impl Into<String>) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::block::Lockable::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::Lockable = temp_clone.into();
        real.set_lock(arg0)
    }
    // SUPER CLASS: Nameable
    pub fn custom_name(&self) -> Result<Option<String>, Box<dyn std::error::Error>> {
        let temp_clone = crate::Nameable::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::Nameable = temp_clone.into();
        real.custom_name()
    }
    pub fn set_custom_name(
        &self,
        arg0: impl Into<String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::Nameable::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::Nameable = temp_clone.into();
        real.set_custom_name(arg0)
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::block::Container<'mc>> for Furnace<'mc> {
    fn into(self) -> crate::block::Container<'mc> {
        crate::block::Container::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting Furnace into crate::block::Container")
    }
}
/// Represents a captured state of a dropper.
///
/// This is a representation of an abstract class.
#[repr(C)]
pub struct Dropper<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for Dropper<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for Dropper<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Dropper from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/block/Dropper")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Dropper object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> Dropper<'mc> {
    pub fn drop(&self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "drop", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub fn inventory(
        &self,
    ) -> Result<crate::inventory::Inventory<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = Dropper::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::Container = temp_clone.into();
        real.inventory()
    }
    pub fn snapshot_inventory(
        &self,
    ) -> Result<crate::inventory::Inventory<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = Dropper::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::Container = temp_clone.into();
        real.snapshot_inventory()
    }
    // SUPER CLASS: TileState
    pub fn data(&self) -> Result<crate::material::MaterialData<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::TileState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.data()
    }
    pub fn world(&self) -> Result<crate::World<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::TileState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.world()
    }
    pub fn set_type(
        &self,
        arg0: impl Into<crate::Material<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::block::TileState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.set_type(arg0)
    }
    pub fn block(&self) -> Result<crate::block::Block<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::TileState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.block()
    }
    pub fn block_data(
        &self,
    ) -> Result<crate::block::data::BlockData<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::TileState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.block_data()
    }
    pub fn light_level(&self) -> Result<i8, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::TileState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.light_level()
    }
    pub fn x(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::TileState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.x()
    }
    pub fn y(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::TileState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.y()
    }
    pub fn z(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::TileState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.z()
    }
    pub fn chunk(&self) -> Result<crate::Chunk<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::TileState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.chunk()
    }
    pub fn set_data(
        &self,
        arg0: impl Into<crate::material::MaterialData<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::block::TileState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.set_data(arg0)
    }
    pub fn set_block_data(
        &self,
        arg0: impl Into<crate::block::data::BlockData<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::block::TileState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.set_block_data(arg0)
    }

    pub fn raw_data(&self) -> Result<i8, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::TileState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.raw_data()
    }

    pub fn set_raw_data(&self, arg0: i8) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::block::TileState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.set_raw_data(arg0)
    }
    pub fn is_placed(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::TileState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.is_placed()
    }
    pub fn update_with_boolean(
        &self,
        arg0: std::option::Option<bool>,
        arg1: std::option::Option<bool>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "Z";
            let val_1 = jni::objects::JValueGen::Bool(a.into());
            args.push(val_1);
        }
        if let Some(a) = arg1 {
            sig += "Z";
            let val_2 = jni::objects::JValueGen::Bool(a.into());
            args.push(val_2);
        }
        sig += ")Z";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "update", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    pub fn location(&self) -> Result<crate::Location<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::TileState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.location()
    }
    pub fn get_type(&self) -> Result<crate::Material<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::TileState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.get_type()
    }
    // SUPER CLASS: Metadatable
    pub fn set_metadata(
        &self,
        arg0: impl Into<String>,
        arg1: impl Into<crate::metadata::MetadataValue<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::metadata::Metadatable::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::metadata::Metadatable = temp_clone.into();
        real.set_metadata(arg0, arg1)
    }
    pub fn get_metadata(
        &self,
        arg0: impl Into<String>,
    ) -> Result<Vec<crate::metadata::MetadataValue<'mc>>, Box<dyn std::error::Error>> {
        let temp_clone = crate::metadata::Metadatable::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::metadata::Metadatable = temp_clone.into();
        real.get_metadata(arg0)
    }
    pub fn has_metadata(
        &self,
        arg0: impl Into<String>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::metadata::Metadatable::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::metadata::Metadatable = temp_clone.into();
        real.has_metadata(arg0)
    }
    pub fn remove_metadata(
        &self,
        arg0: impl Into<String>,
        arg1: impl Into<crate::plugin::Plugin<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::metadata::Metadatable::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::metadata::Metadatable = temp_clone.into();
        real.remove_metadata(arg0, arg1)
    }
    pub fn persistent_data_container(
        &self,
    ) -> Result<crate::persistence::PersistentDataContainer<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::TileState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::persistence::PersistentDataHolder = temp_clone.into();
        real.persistent_data_container()
    }
    // SUPER CLASS: BlockInventoryHolder
    // SUPER CLASS: Lockable
    pub fn is_locked(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::Lockable::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::Lockable = temp_clone.into();
        real.is_locked()
    }
    pub fn lock(&self) -> Result<String, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::Lockable::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::Lockable = temp_clone.into();
        real.lock()
    }
    pub fn set_lock(&self, arg0: impl Into<String>) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::block::Lockable::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::Lockable = temp_clone.into();
        real.set_lock(arg0)
    }
    // SUPER CLASS: Nameable
    pub fn custom_name(&self) -> Result<Option<String>, Box<dyn std::error::Error>> {
        let temp_clone = crate::Nameable::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::Nameable = temp_clone.into();
        real.custom_name()
    }
    pub fn set_custom_name(
        &self,
        arg0: impl Into<String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::Nameable::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::Nameable = temp_clone.into();
        real.set_custom_name(arg0)
    }
    pub fn seed(&self) -> Result<i64, Box<dyn std::error::Error>> {
        let temp_clone = Dropper::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::loot::Lootable = temp_clone.into();
        real.seed()
    }
    pub fn set_seed(&self, arg0: i64) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = Dropper::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::loot::Lootable = temp_clone.into();
        real.set_seed(arg0)
    }
    pub fn set_loot_table(
        &self,
        arg0: impl Into<crate::loot::LootTable<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = Dropper::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::loot::Lootable = temp_clone.into();
        real.set_loot_table(arg0)
    }
    pub fn loot_table(
        &self,
    ) -> Result<Option<crate::loot::LootTable<'mc>>, Box<dyn std::error::Error>> {
        let temp_clone = Dropper::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::loot::Lootable = temp_clone.into();
        real.loot_table()
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::block::Container<'mc>> for Dropper<'mc> {
    fn into(self) -> crate::block::Container<'mc> {
        crate::block::Container::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting Dropper into crate::block::Container")
    }
}
impl<'mc> Into<crate::loot::Lootable<'mc>> for Dropper<'mc> {
    fn into(self) -> crate::loot::Lootable<'mc> {
        crate::loot::Lootable::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting Dropper into crate::loot::Lootable")
    }
}
/// Represents a captured state of suspicious sand.
///
/// This is a representation of an abstract class.
#[repr(C)]
pub struct SuspiciousSand<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for SuspiciousSand<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for SuspiciousSand<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate SuspiciousSand from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/block/SuspiciousSand")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a SuspiciousSand object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> SuspiciousSand<'mc> {
    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::block::BrushableBlock<'mc>> for SuspiciousSand<'mc> {
    fn into(self) -> crate::block::BrushableBlock<'mc> {
        crate::block::BrushableBlock::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting SuspiciousSand into crate::block::BrushableBlock")
    }
}
/// Represents a captured state of a conduit.
///
/// This is a representation of an abstract class.
#[repr(C)]
pub struct Conduit<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for Conduit<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for Conduit<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Conduit from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/block/Conduit")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Conduit object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> Conduit<'mc> {
    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::block::TileState<'mc>> for Conduit<'mc> {
    fn into(self) -> crate::block::TileState<'mc> {
        crate::block::TileState::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting Conduit into crate::block::TileState")
    }
}
/// Represents a captured state of a beacon.
///
/// This is a representation of an abstract class.
#[repr(C)]
pub struct Beacon<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for Beacon<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for Beacon<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Beacon from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/block/Beacon")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Beacon object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> Beacon<'mc> {
    pub fn entities_in_range(
        &self,
    ) -> Result<Vec<crate::entity::LivingEntity<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/Collection;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getEntitiesInRange",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let col = blackboxmc_java::util::JavaCollection::from_raw(&self.jni_ref(), res.l()?)?;
        let iter = col.iterator()?;
        while iter.has_next()? {
            let obj = iter.next()?;
            new_vec.push(crate::entity::LivingEntity::from_raw(&self.jni_ref(), obj)?);
        }
        Ok(new_vec)
    }

    pub fn tier(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getTier", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }

    pub fn primary_effect(
        &self,
    ) -> Result<Option<crate::potion::PotionEffect<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/potion/PotionEffect;");
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
        Ok(Some(crate::potion::PotionEffect::from_raw(
            &self.jni_ref(),
            unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) },
        )?))
    }

    pub fn set_primary_effect(
        &self,
        arg0: impl Into<crate::potion::PotionEffectType<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/potion/PotionEffectType;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
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

    pub fn secondary_effect(
        &self,
    ) -> Result<Option<crate::potion::PotionEffect<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/potion/PotionEffect;");
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
        Ok(Some(crate::potion::PotionEffect::from_raw(
            &self.jni_ref(),
            unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) },
        )?))
    }

    pub fn set_secondary_effect(
        &self,
        arg0: impl Into<crate::potion::PotionEffectType<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/potion/PotionEffectType;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
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
    pub fn persistent_data_container(
        &self,
    ) -> Result<crate::persistence::PersistentDataContainer<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = Beacon::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::TileState = temp_clone.into();
        real.persistent_data_container()
    }
    // SUPER CLASS: BlockState
    pub fn set_metadata(
        &self,
        arg0: impl Into<String>,
        arg1: impl Into<crate::metadata::MetadataValue<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::metadata::Metadatable = temp_clone.into();
        real.set_metadata(arg0, arg1)
    }
    pub fn get_metadata(
        &self,
        arg0: impl Into<String>,
    ) -> Result<Vec<crate::metadata::MetadataValue<'mc>>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::metadata::Metadatable = temp_clone.into();
        real.get_metadata(arg0)
    }
    pub fn has_metadata(
        &self,
        arg0: impl Into<String>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::metadata::Metadatable = temp_clone.into();
        real.has_metadata(arg0)
    }
    pub fn remove_metadata(
        &self,
        arg0: impl Into<String>,
        arg1: impl Into<crate::plugin::Plugin<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::metadata::Metadatable = temp_clone.into();
        real.remove_metadata(arg0, arg1)
    }
    pub fn data(&self) -> Result<crate::material::MaterialData<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.data()
    }
    pub fn world(&self) -> Result<crate::World<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.world()
    }
    pub fn set_type(
        &self,
        arg0: impl Into<crate::Material<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.set_type(arg0)
    }
    pub fn block(&self) -> Result<crate::block::Block<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.block()
    }
    pub fn block_data(
        &self,
    ) -> Result<crate::block::data::BlockData<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.block_data()
    }
    pub fn light_level(&self) -> Result<i8, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.light_level()
    }
    pub fn x(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.x()
    }
    pub fn y(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.y()
    }
    pub fn z(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.z()
    }
    pub fn chunk(&self) -> Result<crate::Chunk<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.chunk()
    }
    pub fn set_data(
        &self,
        arg0: impl Into<crate::material::MaterialData<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.set_data(arg0)
    }
    pub fn set_block_data(
        &self,
        arg0: impl Into<crate::block::data::BlockData<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.set_block_data(arg0)
    }

    pub fn raw_data(&self) -> Result<i8, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.raw_data()
    }

    pub fn set_raw_data(&self, arg0: i8) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.set_raw_data(arg0)
    }
    pub fn is_placed(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.is_placed()
    }
    pub fn update_with_boolean(
        &self,
        arg0: std::option::Option<bool>,
        arg1: std::option::Option<bool>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "Z";
            let val_1 = jni::objects::JValueGen::Bool(a.into());
            args.push(val_1);
        }
        if let Some(a) = arg1 {
            sig += "Z";
            let val_2 = jni::objects::JValueGen::Bool(a.into());
            args.push(val_2);
        }
        sig += ")Z";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "update", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    pub fn location(&self) -> Result<crate::Location<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.location()
    }
    pub fn get_type(&self) -> Result<crate::Material<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.get_type()
    }
    // SUPER CLASS: PersistentDataHolder
    pub fn is_locked(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = Beacon::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::Lockable = temp_clone.into();
        real.is_locked()
    }
    pub fn lock(&self) -> Result<String, Box<dyn std::error::Error>> {
        let temp_clone = Beacon::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::Lockable = temp_clone.into();
        real.lock()
    }
    pub fn set_lock(&self, arg0: impl Into<String>) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = Beacon::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::Lockable = temp_clone.into();
        real.set_lock(arg0)
    }
    pub fn custom_name(&self) -> Result<Option<String>, Box<dyn std::error::Error>> {
        let temp_clone = Beacon::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::Nameable = temp_clone.into();
        real.custom_name()
    }
    pub fn set_custom_name(
        &self,
        arg0: impl Into<String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = Beacon::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::Nameable = temp_clone.into();
        real.set_custom_name(arg0)
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::block::TileState<'mc>> for Beacon<'mc> {
    fn into(self) -> crate::block::TileState<'mc> {
        crate::block::TileState::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting Beacon into crate::block::TileState")
    }
}
impl<'mc> Into<crate::block::Lockable<'mc>> for Beacon<'mc> {
    fn into(self) -> crate::block::Lockable<'mc> {
        crate::block::Lockable::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting Beacon into crate::block::Lockable")
    }
}
impl<'mc> Into<crate::Nameable<'mc>> for Beacon<'mc> {
    fn into(self) -> crate::Nameable<'mc> {
        crate::Nameable::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting Beacon into crate::Nameable")
    }
}
/// Represents a block. This is a live object, and only one Block may exist for any given location in a world. The state of the block may change concurrently to your own handling of it; use block.getState() to get a snapshot state of a block which will not be modified.
///
/// Note that parts of this class which require access to the world at large (i.e. lighting and power) may not be able to be safely accessed during world generation when used in cases like BlockPhysicsEvent!!!!
///
/// This is a representation of an abstract class.
#[repr(C)]
pub struct Block<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for Block<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for Block<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Block from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/block/Block")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Block object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> Block<'mc> {
    pub fn data(&self) -> Result<i8, Box<dyn std::error::Error>> {
        let sig = String::from("()B");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getData", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.b()?)
    }

    pub fn world(&self) -> Result<crate::World<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/World;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getWorld", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::World::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn is_preferred_tool(
        &self,
        arg0: impl Into<crate::inventory::ItemStack<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/inventory/ItemStack;)Z");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "isPreferredTool",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn piston_move_reaction(
        &self,
    ) -> Result<crate::block::PistonMoveReaction<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/block/PistonMoveReaction;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getPistonMoveReaction",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::block::PistonMoveReaction::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn set_type_with_material(
        &self,
        arg0: impl Into<crate::Material<'mc>>,
        arg1: std::option::Option<bool>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/Material;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        if let Some(a) = arg1 {
            sig += "Z";
            let val_2 = jni::objects::JValueGen::Bool(a.into());
            args.push(val_2);
        }
        sig += ")V";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "setType", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn bounding_box(
        &self,
    ) -> Result<crate::util::BoundingBox<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/util/BoundingBox;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getBoundingBox", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::util::BoundingBox::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn block_data(
        &self,
    ) -> Result<crate::block::data::BlockData<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/block/data/BlockData;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getBlockData", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::block::data::BlockData::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn light_level(&self) -> Result<i8, Box<dyn std::error::Error>> {
        let sig = String::from("()B");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getLightLevel", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.b()?)
    }

    pub fn x(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getX", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }

    pub fn y(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getY", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }

    pub fn z(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getZ", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }

    pub fn chunk(&self) -> Result<crate::Chunk<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/Chunk;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getChunk", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::Chunk::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn set_block_data_with_block_data(
        &self,
        arg0: impl Into<crate::block::data::BlockData<'mc>>,
        arg1: std::option::Option<bool>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/block/data/BlockData;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        if let Some(a) = arg1 {
            sig += "Z";
            let val_2 = jni::objects::JValueGen::Bool(a.into());
            args.push(val_2);
        }
        sig += ")V";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "setBlockData", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn get_face(
        &self,
        arg0: impl Into<crate::block::Block<'mc>>,
    ) -> Result<crate::block::BlockFace<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/Block;)Lorg/bukkit/block/BlockFace;");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getFace",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::block::BlockFace::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn ray_trace(
        &self,
        arg0: impl Into<crate::Location<'mc>>,
        arg1: impl Into<crate::util::Vector<'mc>>,
        arg2: f64,
        arg3: impl Into<crate::FluidCollisionMode<'mc>>,
    ) -> Result<crate::util::RayTraceResult<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/Location;Lorg/bukkit/util/Vector;DLorg/bukkit/FluidCollisionMode;)Lorg/bukkit/util/RayTraceResult;");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let val_2 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg1.into().jni_object().clone())
        });
        let val_3 = jni::objects::JValueGen::Double(arg2);
        let val_4 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg3.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "rayTrace",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
                jni::objects::JValueGen::from(val_3),
                jni::objects::JValueGen::from(val_4),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::util::RayTraceResult::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn biome(&self) -> Result<crate::block::Biome<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/block/Biome;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getBiome", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::block::Biome::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn set_biome(
        &self,
        arg0: impl Into<crate::block::Biome<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/Biome;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setBiome",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Gets the block at the given offsets
    pub fn get_relative_with_int(
        &self,
        arg0: i32,
        arg1: std::option::Option<i32>,
        arg2: std::option::Option<i32>,
    ) -> Result<crate::block::Block<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "I";
        let val_1 = jni::objects::JValueGen::Int(arg0);
        args.push(val_1);
        if let Some(a) = arg1 {
            sig += "I";
            let val_2 = jni::objects::JValueGen::Int(a);
            args.push(val_2);
        }
        if let Some(a) = arg2 {
            sig += "I";
            let val_3 = jni::objects::JValueGen::Int(a);
            args.push(val_3);
        }
        sig += ")Lorg/bukkit/block/Block;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getRelative", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        crate::block::Block::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn temperature(&self) -> Result<f64, Box<dyn std::error::Error>> {
        let sig = String::from("()D");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getTemperature", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.d()?)
    }

    pub fn humidity(&self) -> Result<f64, Box<dyn std::error::Error>> {
        let sig = String::from("()D");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getHumidity", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.d()?)
    }

    pub fn light_from_sky(&self) -> Result<i8, Box<dyn std::error::Error>> {
        let sig = String::from("()B");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getLightFromSky", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.b()?)
    }

    pub fn light_from_blocks(&self) -> Result<i8, Box<dyn std::error::Error>> {
        let sig = String::from("()B");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getLightFromBlocks",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.b()?)
    }

    pub fn is_block_powered(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isBlockPowered", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn is_block_indirectly_powered(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "isBlockIndirectlyPowered",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn is_block_face_powered(
        &self,
        arg0: impl Into<crate::block::BlockFace<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/BlockFace;)Z");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "isBlockFacePowered",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn is_block_face_indirectly_powered(
        &self,
        arg0: impl Into<crate::block::BlockFace<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/BlockFace;)Z");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "isBlockFaceIndirectlyPowered",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn block_power(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getBlockPower", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }

    pub fn is_liquid(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isLiquid", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn break_naturally_with_item_stack(
        &self,
        arg0: std::option::Option<impl Into<crate::inventory::ItemStack<'mc>>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "Lorg/bukkit/inventory/ItemStack;";
            let val_1 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(a.into().jni_object().clone())
            });
            args.push(val_1);
        }
        sig += ")Z";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "breakNaturally", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn apply_bone_meal(
        &self,
        arg0: impl Into<crate::block::BlockFace<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/BlockFace;)Z");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "applyBoneMeal",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn get_drops_with_item_stack(
        &self,
        arg0: impl Into<crate::inventory::ItemStack<'mc>>,
        arg1: std::option::Option<impl Into<crate::entity::Entity<'mc>>>,
    ) -> Result<Vec<crate::inventory::ItemStack<'mc>>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/inventory/ItemStack;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        if let Some(a) = arg1 {
            sig += "Lorg/bukkit/entity/Entity;";
            let val_2 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(a.into().jni_object().clone())
            });
            args.push(val_2);
        }
        sig += ")Ljava/util/Collection;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getDrops", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let col = blackboxmc_java::util::JavaCollection::from_raw(&self.jni_ref(), res.l()?)?;
        let iter = col.iterator()?;
        while iter.has_next()? {
            let obj = iter.next()?;
            new_vec.push(crate::inventory::ItemStack::from_raw(&self.jni_ref(), obj)?);
        }
        Ok(new_vec)
    }

    pub fn get_break_speed(
        &self,
        arg0: impl Into<crate::entity::Player<'mc>>,
    ) -> Result<f32, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/entity/Player;)F");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getBreakSpeed",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.f()?)
    }

    pub fn is_passable(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isPassable", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn collision_shape(
        &self,
    ) -> Result<crate::util::VoxelShape<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/util/VoxelShape;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getCollisionShape",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::util::VoxelShape::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn can_place(
        &self,
        arg0: impl Into<crate::block::data::BlockData<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/data/BlockData;)Z");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "canPlace",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn is_empty(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isEmpty", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn get_location(
        &self,
        arg0: impl Into<crate::Location<'mc>>,
    ) -> Result<crate::Location<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/Location;)Lorg/bukkit/Location;");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getLocation",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::Location::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn state(&self) -> Result<crate::block::BlockState<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/block/BlockState;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getState", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::block::BlockState::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn get_type(&self) -> Result<crate::Material<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/Material;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getType", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::Material::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    pub fn set_metadata(
        &self,
        arg0: impl Into<String>,
        arg1: impl Into<crate::metadata::MetadataValue<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = Block::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::metadata::Metadatable = temp_clone.into();
        real.set_metadata(arg0, arg1)
    }
    pub fn get_metadata(
        &self,
        arg0: impl Into<String>,
    ) -> Result<Vec<crate::metadata::MetadataValue<'mc>>, Box<dyn std::error::Error>> {
        let temp_clone = Block::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::metadata::Metadatable = temp_clone.into();
        real.get_metadata(arg0)
    }
    pub fn has_metadata(
        &self,
        arg0: impl Into<String>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = Block::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::metadata::Metadatable = temp_clone.into();
        real.has_metadata(arg0)
    }
    pub fn remove_metadata(
        &self,
        arg0: impl Into<String>,
        arg1: impl Into<crate::plugin::Plugin<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = Block::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::metadata::Metadatable = temp_clone.into();
        real.remove_metadata(arg0, arg1)
    }
    pub fn translation_key(&self) -> Result<String, Box<dyn std::error::Error>> {
        let temp_clone = Block::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::Translatable = temp_clone.into();
        real.translation_key()
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::metadata::Metadatable<'mc>> for Block<'mc> {
    fn into(self) -> crate::metadata::Metadatable<'mc> {
        crate::metadata::Metadatable::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting Block into crate::metadata::Metadatable")
    }
}
impl<'mc> Into<crate::Translatable<'mc>> for Block<'mc> {
    fn into(self) -> crate::Translatable<'mc> {
        crate::Translatable::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting Block into crate::Translatable")
    }
}
/// Represents a captured state of Bell.
///
/// This is a representation of an abstract class.
#[repr(C)]
pub struct Bell<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for Bell<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for Bell<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Bell from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/block/Bell")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Bell object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> Bell<'mc> {
    pub fn ring_with_entity(
        &self,
        arg0: std::option::Option<impl Into<crate::entity::Entity<'mc>>>,
        arg1: std::option::Option<impl Into<crate::block::BlockFace<'mc>>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "Lorg/bukkit/entity/Entity;";
            let val_1 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(a.into().jni_object().clone())
            });
            args.push(val_1);
        }
        if let Some(a) = arg1 {
            sig += "Lorg/bukkit/block/BlockFace;";
            let val_2 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(a.into().jni_object().clone())
            });
            args.push(val_2);
        }
        sig += ")Z";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "ring", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn is_shaking(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isShaking", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn shaking_ticks(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getShakingTicks", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }

    pub fn is_resonating(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isResonating", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn resonating_ticks(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getResonatingTicks",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    pub fn persistent_data_container(
        &self,
    ) -> Result<crate::persistence::PersistentDataContainer<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = Bell::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::TileState = temp_clone.into();
        real.persistent_data_container()
    }
    // SUPER CLASS: BlockState
    pub fn set_metadata(
        &self,
        arg0: impl Into<String>,
        arg1: impl Into<crate::metadata::MetadataValue<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::metadata::Metadatable = temp_clone.into();
        real.set_metadata(arg0, arg1)
    }
    pub fn get_metadata(
        &self,
        arg0: impl Into<String>,
    ) -> Result<Vec<crate::metadata::MetadataValue<'mc>>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::metadata::Metadatable = temp_clone.into();
        real.get_metadata(arg0)
    }
    pub fn has_metadata(
        &self,
        arg0: impl Into<String>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::metadata::Metadatable = temp_clone.into();
        real.has_metadata(arg0)
    }
    pub fn remove_metadata(
        &self,
        arg0: impl Into<String>,
        arg1: impl Into<crate::plugin::Plugin<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::metadata::Metadatable = temp_clone.into();
        real.remove_metadata(arg0, arg1)
    }
    pub fn data(&self) -> Result<crate::material::MaterialData<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.data()
    }
    pub fn world(&self) -> Result<crate::World<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.world()
    }
    pub fn set_type(
        &self,
        arg0: impl Into<crate::Material<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.set_type(arg0)
    }
    pub fn block(&self) -> Result<crate::block::Block<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.block()
    }
    pub fn block_data(
        &self,
    ) -> Result<crate::block::data::BlockData<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.block_data()
    }
    pub fn light_level(&self) -> Result<i8, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.light_level()
    }
    pub fn x(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.x()
    }
    pub fn y(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.y()
    }
    pub fn z(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.z()
    }
    pub fn chunk(&self) -> Result<crate::Chunk<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.chunk()
    }
    pub fn set_data(
        &self,
        arg0: impl Into<crate::material::MaterialData<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.set_data(arg0)
    }
    pub fn set_block_data(
        &self,
        arg0: impl Into<crate::block::data::BlockData<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.set_block_data(arg0)
    }

    pub fn raw_data(&self) -> Result<i8, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.raw_data()
    }

    pub fn set_raw_data(&self, arg0: i8) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.set_raw_data(arg0)
    }
    pub fn is_placed(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.is_placed()
    }
    pub fn update_with_boolean(
        &self,
        arg0: std::option::Option<bool>,
        arg1: std::option::Option<bool>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "Z";
            let val_1 = jni::objects::JValueGen::Bool(a.into());
            args.push(val_1);
        }
        if let Some(a) = arg1 {
            sig += "Z";
            let val_2 = jni::objects::JValueGen::Bool(a.into());
            args.push(val_2);
        }
        sig += ")Z";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "update", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    pub fn location(&self) -> Result<crate::Location<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.location()
    }
    pub fn get_type(&self) -> Result<crate::Material<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.get_type()
    }
    // SUPER CLASS: PersistentDataHolder

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::block::TileState<'mc>> for Bell<'mc> {
    fn into(self) -> crate::block::TileState<'mc> {
        crate::block::TileState::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting Bell into crate::block::TileState")
    }
}
/// Represents a captured state of a jigsaw.
///
/// This is a representation of an abstract class.
#[repr(C)]
pub struct Jigsaw<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for Jigsaw<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for Jigsaw<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Jigsaw from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/block/Jigsaw")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Jigsaw object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> Jigsaw<'mc> {
    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::block::TileState<'mc>> for Jigsaw<'mc> {
    fn into(self) -> crate::block::TileState<'mc> {
        crate::block::TileState::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting Jigsaw into crate::block::TileState")
    }
}
/// Represents a captured state of a dispenser.
///
/// This is a representation of an abstract class.
#[repr(C)]
pub struct Dispenser<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for Dispenser<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for Dispenser<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Dispenser from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/block/Dispenser")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Dispenser object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> Dispenser<'mc> {
    pub fn block_projectile_source(
        &self,
    ) -> Result<Option<crate::projectiles::BlockProjectileSource<'mc>>, Box<dyn std::error::Error>>
    {
        let sig = String::from("()Lorg/bukkit/projectiles/BlockProjectileSource;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getBlockProjectileSource",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {
            return Ok(None);
        }
        Ok(Some(crate::projectiles::BlockProjectileSource::from_raw(
            &self.jni_ref(),
            unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) },
        )?))
    }

    pub fn dispense(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "dispense", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    pub fn inventory(
        &self,
    ) -> Result<crate::inventory::Inventory<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = Dispenser::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::Container = temp_clone.into();
        real.inventory()
    }
    pub fn snapshot_inventory(
        &self,
    ) -> Result<crate::inventory::Inventory<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = Dispenser::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::Container = temp_clone.into();
        real.snapshot_inventory()
    }
    // SUPER CLASS: TileState
    pub fn data(&self) -> Result<crate::material::MaterialData<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::TileState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.data()
    }
    pub fn world(&self) -> Result<crate::World<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::TileState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.world()
    }
    pub fn set_type(
        &self,
        arg0: impl Into<crate::Material<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::block::TileState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.set_type(arg0)
    }
    pub fn block(&self) -> Result<crate::block::Block<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::TileState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.block()
    }
    pub fn block_data(
        &self,
    ) -> Result<crate::block::data::BlockData<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::TileState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.block_data()
    }
    pub fn light_level(&self) -> Result<i8, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::TileState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.light_level()
    }
    pub fn x(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::TileState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.x()
    }
    pub fn y(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::TileState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.y()
    }
    pub fn z(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::TileState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.z()
    }
    pub fn chunk(&self) -> Result<crate::Chunk<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::TileState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.chunk()
    }
    pub fn set_data(
        &self,
        arg0: impl Into<crate::material::MaterialData<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::block::TileState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.set_data(arg0)
    }
    pub fn set_block_data(
        &self,
        arg0: impl Into<crate::block::data::BlockData<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::block::TileState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.set_block_data(arg0)
    }

    pub fn raw_data(&self) -> Result<i8, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::TileState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.raw_data()
    }

    pub fn set_raw_data(&self, arg0: i8) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::block::TileState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.set_raw_data(arg0)
    }
    pub fn is_placed(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::TileState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.is_placed()
    }
    pub fn update_with_boolean(
        &self,
        arg0: std::option::Option<bool>,
        arg1: std::option::Option<bool>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "Z";
            let val_1 = jni::objects::JValueGen::Bool(a.into());
            args.push(val_1);
        }
        if let Some(a) = arg1 {
            sig += "Z";
            let val_2 = jni::objects::JValueGen::Bool(a.into());
            args.push(val_2);
        }
        sig += ")Z";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "update", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    pub fn location(&self) -> Result<crate::Location<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::TileState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.location()
    }
    pub fn get_type(&self) -> Result<crate::Material<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::TileState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.get_type()
    }
    // SUPER CLASS: Metadatable
    pub fn set_metadata(
        &self,
        arg0: impl Into<String>,
        arg1: impl Into<crate::metadata::MetadataValue<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::metadata::Metadatable::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::metadata::Metadatable = temp_clone.into();
        real.set_metadata(arg0, arg1)
    }
    pub fn get_metadata(
        &self,
        arg0: impl Into<String>,
    ) -> Result<Vec<crate::metadata::MetadataValue<'mc>>, Box<dyn std::error::Error>> {
        let temp_clone = crate::metadata::Metadatable::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::metadata::Metadatable = temp_clone.into();
        real.get_metadata(arg0)
    }
    pub fn has_metadata(
        &self,
        arg0: impl Into<String>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::metadata::Metadatable::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::metadata::Metadatable = temp_clone.into();
        real.has_metadata(arg0)
    }
    pub fn remove_metadata(
        &self,
        arg0: impl Into<String>,
        arg1: impl Into<crate::plugin::Plugin<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::metadata::Metadatable::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::metadata::Metadatable = temp_clone.into();
        real.remove_metadata(arg0, arg1)
    }
    pub fn persistent_data_container(
        &self,
    ) -> Result<crate::persistence::PersistentDataContainer<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::TileState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::persistence::PersistentDataHolder = temp_clone.into();
        real.persistent_data_container()
    }
    // SUPER CLASS: BlockInventoryHolder
    // SUPER CLASS: Lockable
    pub fn is_locked(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::Lockable::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::Lockable = temp_clone.into();
        real.is_locked()
    }
    pub fn lock(&self) -> Result<String, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::Lockable::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::Lockable = temp_clone.into();
        real.lock()
    }
    pub fn set_lock(&self, arg0: impl Into<String>) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::block::Lockable::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::Lockable = temp_clone.into();
        real.set_lock(arg0)
    }
    // SUPER CLASS: Nameable
    pub fn custom_name(&self) -> Result<Option<String>, Box<dyn std::error::Error>> {
        let temp_clone = crate::Nameable::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::Nameable = temp_clone.into();
        real.custom_name()
    }
    pub fn set_custom_name(
        &self,
        arg0: impl Into<String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::Nameable::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::Nameable = temp_clone.into();
        real.set_custom_name(arg0)
    }
    pub fn seed(&self) -> Result<i64, Box<dyn std::error::Error>> {
        let temp_clone = Dispenser::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::loot::Lootable = temp_clone.into();
        real.seed()
    }
    pub fn set_seed(&self, arg0: i64) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = Dispenser::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::loot::Lootable = temp_clone.into();
        real.set_seed(arg0)
    }
    pub fn set_loot_table(
        &self,
        arg0: impl Into<crate::loot::LootTable<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = Dispenser::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::loot::Lootable = temp_clone.into();
        real.set_loot_table(arg0)
    }
    pub fn loot_table(
        &self,
    ) -> Result<Option<crate::loot::LootTable<'mc>>, Box<dyn std::error::Error>> {
        let temp_clone = Dispenser::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::loot::Lootable = temp_clone.into();
        real.loot_table()
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::block::Container<'mc>> for Dispenser<'mc> {
    fn into(self) -> crate::block::Container<'mc> {
        crate::block::Container::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting Dispenser into crate::block::Container")
    }
}
impl<'mc> Into<crate::Nameable<'mc>> for Dispenser<'mc> {
    fn into(self) -> crate::Nameable<'mc> {
        crate::Nameable::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting Dispenser into crate::Nameable")
    }
}
impl<'mc> Into<crate::loot::Lootable<'mc>> for Dispenser<'mc> {
    fn into(self) -> crate::loot::Lootable<'mc> {
        crate::loot::Lootable::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting Dispenser into crate::loot::Lootable")
    }
}
/// Represents a captured state of suspicious sand or gravel.
///
/// This is a representation of an abstract class.
#[repr(C)]
pub struct BrushableBlock<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for BrushableBlock<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for BrushableBlock<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate BrushableBlock from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/block/BrushableBlock")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a BrushableBlock object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> BrushableBlock<'mc> {
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
    pub fn seed(&self) -> Result<i64, Box<dyn std::error::Error>> {
        let temp_clone = BrushableBlock::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::loot::Lootable = temp_clone.into();
        real.seed()
    }
    pub fn set_seed(&self, arg0: i64) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = BrushableBlock::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::loot::Lootable = temp_clone.into();
        real.set_seed(arg0)
    }
    pub fn set_loot_table(
        &self,
        arg0: impl Into<crate::loot::LootTable<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = BrushableBlock::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::loot::Lootable = temp_clone.into();
        real.set_loot_table(arg0)
    }
    pub fn loot_table(
        &self,
    ) -> Result<Option<crate::loot::LootTable<'mc>>, Box<dyn std::error::Error>> {
        let temp_clone = BrushableBlock::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::loot::Lootable = temp_clone.into();
        real.loot_table()
    }
    pub fn persistent_data_container(
        &self,
    ) -> Result<crate::persistence::PersistentDataContainer<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = BrushableBlock::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::TileState = temp_clone.into();
        real.persistent_data_container()
    }
    // SUPER CLASS: BlockState
    pub fn set_metadata(
        &self,
        arg0: impl Into<String>,
        arg1: impl Into<crate::metadata::MetadataValue<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::metadata::Metadatable = temp_clone.into();
        real.set_metadata(arg0, arg1)
    }
    pub fn get_metadata(
        &self,
        arg0: impl Into<String>,
    ) -> Result<Vec<crate::metadata::MetadataValue<'mc>>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::metadata::Metadatable = temp_clone.into();
        real.get_metadata(arg0)
    }
    pub fn has_metadata(
        &self,
        arg0: impl Into<String>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::metadata::Metadatable = temp_clone.into();
        real.has_metadata(arg0)
    }
    pub fn remove_metadata(
        &self,
        arg0: impl Into<String>,
        arg1: impl Into<crate::plugin::Plugin<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::metadata::Metadatable = temp_clone.into();
        real.remove_metadata(arg0, arg1)
    }
    pub fn data(&self) -> Result<crate::material::MaterialData<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.data()
    }
    pub fn world(&self) -> Result<crate::World<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.world()
    }
    pub fn set_type(
        &self,
        arg0: impl Into<crate::Material<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.set_type(arg0)
    }
    pub fn block(&self) -> Result<crate::block::Block<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.block()
    }
    pub fn block_data(
        &self,
    ) -> Result<crate::block::data::BlockData<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.block_data()
    }
    pub fn light_level(&self) -> Result<i8, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.light_level()
    }
    pub fn x(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.x()
    }
    pub fn y(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.y()
    }
    pub fn z(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.z()
    }
    pub fn chunk(&self) -> Result<crate::Chunk<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.chunk()
    }
    pub fn set_data(
        &self,
        arg0: impl Into<crate::material::MaterialData<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.set_data(arg0)
    }
    pub fn set_block_data(
        &self,
        arg0: impl Into<crate::block::data::BlockData<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.set_block_data(arg0)
    }

    pub fn raw_data(&self) -> Result<i8, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.raw_data()
    }

    pub fn set_raw_data(&self, arg0: i8) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.set_raw_data(arg0)
    }
    pub fn is_placed(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.is_placed()
    }
    pub fn update_with_boolean(
        &self,
        arg0: std::option::Option<bool>,
        arg1: std::option::Option<bool>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "Z";
            let val_1 = jni::objects::JValueGen::Bool(a.into());
            args.push(val_1);
        }
        if let Some(a) = arg1 {
            sig += "Z";
            let val_2 = jni::objects::JValueGen::Bool(a.into());
            args.push(val_2);
        }
        sig += ")Z";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "update", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    pub fn location(&self) -> Result<crate::Location<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.location()
    }
    pub fn get_type(&self) -> Result<crate::Material<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.get_type()
    }
    // SUPER CLASS: PersistentDataHolder

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::loot::Lootable<'mc>> for BrushableBlock<'mc> {
    fn into(self) -> crate::loot::Lootable<'mc> {
        crate::loot::Lootable::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting BrushableBlock into crate::loot::Lootable")
    }
}
impl<'mc> Into<crate::block::TileState<'mc>> for BrushableBlock<'mc> {
    fn into(self) -> crate::block::TileState<'mc> {
        crate::block::TileState::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting BrushableBlock into crate::block::TileState")
    }
}
/// Represents a captured state of a sculk shrieker.
///
/// This is a representation of an abstract class.
#[repr(C)]
pub struct SculkShrieker<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for SculkShrieker<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for SculkShrieker<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate SculkShrieker from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/block/SculkShrieker")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a SculkShrieker object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> SculkShrieker<'mc> {
    pub fn warning_level(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getWarningLevel", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    /// Sets the most recent warning level of this block. When the warning level reaches 4, the shrieker will attempt to spawn a Warden.
    pub fn set_warning_level(&self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(I)V");
        let val_1 = jni::objects::JValueGen::Int(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setWarningLevel",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub fn persistent_data_container(
        &self,
    ) -> Result<crate::persistence::PersistentDataContainer<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = SculkShrieker::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::TileState = temp_clone.into();
        real.persistent_data_container()
    }
    // SUPER CLASS: BlockState
    pub fn set_metadata(
        &self,
        arg0: impl Into<String>,
        arg1: impl Into<crate::metadata::MetadataValue<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::metadata::Metadatable = temp_clone.into();
        real.set_metadata(arg0, arg1)
    }
    pub fn get_metadata(
        &self,
        arg0: impl Into<String>,
    ) -> Result<Vec<crate::metadata::MetadataValue<'mc>>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::metadata::Metadatable = temp_clone.into();
        real.get_metadata(arg0)
    }
    pub fn has_metadata(
        &self,
        arg0: impl Into<String>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::metadata::Metadatable = temp_clone.into();
        real.has_metadata(arg0)
    }
    pub fn remove_metadata(
        &self,
        arg0: impl Into<String>,
        arg1: impl Into<crate::plugin::Plugin<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::metadata::Metadatable = temp_clone.into();
        real.remove_metadata(arg0, arg1)
    }
    pub fn data(&self) -> Result<crate::material::MaterialData<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.data()
    }
    pub fn world(&self) -> Result<crate::World<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.world()
    }
    pub fn set_type(
        &self,
        arg0: impl Into<crate::Material<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.set_type(arg0)
    }
    pub fn block(&self) -> Result<crate::block::Block<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.block()
    }
    pub fn block_data(
        &self,
    ) -> Result<crate::block::data::BlockData<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.block_data()
    }
    pub fn light_level(&self) -> Result<i8, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.light_level()
    }
    pub fn x(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.x()
    }
    pub fn y(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.y()
    }
    pub fn z(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.z()
    }
    pub fn chunk(&self) -> Result<crate::Chunk<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.chunk()
    }
    pub fn set_data(
        &self,
        arg0: impl Into<crate::material::MaterialData<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.set_data(arg0)
    }
    pub fn set_block_data(
        &self,
        arg0: impl Into<crate::block::data::BlockData<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.set_block_data(arg0)
    }

    pub fn raw_data(&self) -> Result<i8, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.raw_data()
    }

    pub fn set_raw_data(&self, arg0: i8) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.set_raw_data(arg0)
    }
    pub fn is_placed(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.is_placed()
    }
    pub fn update_with_boolean(
        &self,
        arg0: std::option::Option<bool>,
        arg1: std::option::Option<bool>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "Z";
            let val_1 = jni::objects::JValueGen::Bool(a.into());
            args.push(val_1);
        }
        if let Some(a) = arg1 {
            sig += "Z";
            let val_2 = jni::objects::JValueGen::Bool(a.into());
            args.push(val_2);
        }
        sig += ")Z";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "update", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    pub fn location(&self) -> Result<crate::Location<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.location()
    }
    pub fn get_type(&self) -> Result<crate::Material<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.get_type()
    }
    // SUPER CLASS: PersistentDataHolder

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::block::TileState<'mc>> for SculkShrieker<'mc> {
    fn into(self) -> crate::block::TileState<'mc> {
        crate::block::TileState::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting SculkShrieker into crate::block::TileState")
    }
}
/// Represents a captured state of a decorated pot.
///
/// This is a representation of an abstract class.
#[repr(C)]
pub struct DecoratedPot<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for DecoratedPot<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for DecoratedPot<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate DecoratedPot from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/block/DecoratedPot")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a DecoratedPot object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> DecoratedPot<'mc> {
    pub fn set_sherd(
        &self,
        arg0: impl Into<crate::block::DecoratedPotSide<'mc>>,
        arg1: impl Into<crate::Material<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/DecoratedPot$Side;Lorg/bukkit/Material;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let val_2 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg1.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setSherd",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
            ],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn get_sherd(
        &self,
        arg0: impl Into<crate::block::DecoratedPotSide<'mc>>,
    ) -> Result<crate::Material<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/DecoratedPot$Side;)Lorg/bukkit/Material;");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getSherd",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::Material::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn sherds(
        &self,
    ) -> Result<blackboxmc_java::util::JavaMap<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/Map;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getSherds", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::util::JavaMap::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    #[deprecated]

    pub fn shards(&self) -> Result<Vec<crate::Material<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/List;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getShards", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let list = blackboxmc_java::util::JavaList::from_raw(&self.jni_ref(), res.l()?)?;
        let iter = list.iterator()?;
        while iter.has_next()? {
            let obj = iter.next()?;
            new_vec.push(crate::Material::from_raw(&self.jni_ref(), obj)?);
        }
        Ok(new_vec)
    }
    pub fn persistent_data_container(
        &self,
    ) -> Result<crate::persistence::PersistentDataContainer<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = DecoratedPot::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::TileState = temp_clone.into();
        real.persistent_data_container()
    }
    // SUPER CLASS: BlockState
    pub fn set_metadata(
        &self,
        arg0: impl Into<String>,
        arg1: impl Into<crate::metadata::MetadataValue<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::metadata::Metadatable = temp_clone.into();
        real.set_metadata(arg0, arg1)
    }
    pub fn get_metadata(
        &self,
        arg0: impl Into<String>,
    ) -> Result<Vec<crate::metadata::MetadataValue<'mc>>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::metadata::Metadatable = temp_clone.into();
        real.get_metadata(arg0)
    }
    pub fn has_metadata(
        &self,
        arg0: impl Into<String>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::metadata::Metadatable = temp_clone.into();
        real.has_metadata(arg0)
    }
    pub fn remove_metadata(
        &self,
        arg0: impl Into<String>,
        arg1: impl Into<crate::plugin::Plugin<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::metadata::Metadatable = temp_clone.into();
        real.remove_metadata(arg0, arg1)
    }
    pub fn data(&self) -> Result<crate::material::MaterialData<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.data()
    }
    pub fn world(&self) -> Result<crate::World<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.world()
    }
    pub fn set_type(
        &self,
        arg0: impl Into<crate::Material<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.set_type(arg0)
    }
    pub fn block(&self) -> Result<crate::block::Block<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.block()
    }
    pub fn block_data(
        &self,
    ) -> Result<crate::block::data::BlockData<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.block_data()
    }
    pub fn light_level(&self) -> Result<i8, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.light_level()
    }
    pub fn x(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.x()
    }
    pub fn y(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.y()
    }
    pub fn z(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.z()
    }
    pub fn chunk(&self) -> Result<crate::Chunk<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.chunk()
    }
    pub fn set_data(
        &self,
        arg0: impl Into<crate::material::MaterialData<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.set_data(arg0)
    }
    pub fn set_block_data(
        &self,
        arg0: impl Into<crate::block::data::BlockData<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.set_block_data(arg0)
    }

    pub fn raw_data(&self) -> Result<i8, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.raw_data()
    }

    pub fn set_raw_data(&self, arg0: i8) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.set_raw_data(arg0)
    }
    pub fn is_placed(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.is_placed()
    }
    pub fn update_with_boolean(
        &self,
        arg0: std::option::Option<bool>,
        arg1: std::option::Option<bool>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "Z";
            let val_1 = jni::objects::JValueGen::Bool(a.into());
            args.push(val_1);
        }
        if let Some(a) = arg1 {
            sig += "Z";
            let val_2 = jni::objects::JValueGen::Bool(a.into());
            args.push(val_2);
        }
        sig += ")Z";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "update", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    pub fn location(&self) -> Result<crate::Location<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.location()
    }
    pub fn get_type(&self) -> Result<crate::Material<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.get_type()
    }
    // SUPER CLASS: PersistentDataHolder

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::block::TileState<'mc>> for DecoratedPot<'mc> {
    fn into(self) -> crate::block::TileState<'mc> {
        crate::block::TileState::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting DecoratedPot into crate::block::TileState")
    }
}
/// Represents a captured state of a campfire.
///
/// This is a representation of an abstract class.
#[repr(C)]
pub struct Campfire<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for Campfire<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for Campfire<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Campfire from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/block/Campfire")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Campfire object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> Campfire<'mc> {
    pub fn get_item(
        &self,
        arg0: i32,
    ) -> Result<Option<crate::inventory::ItemStack<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("(I)Lorg/bukkit/inventory/ItemStack;");
        let val_1 = jni::objects::JValueGen::Int(arg0);
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

    pub fn set_item(
        &self,
        arg0: i32,
        arg1: impl Into<crate::inventory::ItemStack<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(ILorg/bukkit/inventory/ItemStack;)V");
        let val_1 = jni::objects::JValueGen::Int(arg0);
        let val_2 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg1.into().jni_object().clone())
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
    /// Get cook time. This is the amount of time the item has been cooking for.
    /// Get cook time total. This is the amount of time the item is required to cook for.
    pub fn get_cook_time(&self, arg0: i32) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("(I)I");
        let val_1 = jni::objects::JValueGen::Int(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getCookTime",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    /// Set cook time. This is the amount of time the item has been cooking for.
    /// Set cook time. This is the amount of time the item is required to cook for.
    pub fn set_cook_time(&self, arg0: i32, arg1: i32) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(II)V");
        let val_1 = jni::objects::JValueGen::Int(arg0);
        let val_2 = jni::objects::JValueGen::Int(arg1);
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
    /// Get cook time total. This is the amount of time the item is required to cook for.
    pub fn get_cook_time_total(&self, arg0: i32) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("(I)I");
        let val_1 = jni::objects::JValueGen::Int(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getCookTimeTotal",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    /// Set cook time. This is the amount of time the item is required to cook for.
    pub fn set_cook_time_total(
        &self,
        arg0: i32,
        arg1: i32,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(II)V");
        let val_1 = jni::objects::JValueGen::Int(arg0);
        let val_2 = jni::objects::JValueGen::Int(arg1);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setCookTimeTotal",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
            ],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn size(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getSize", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    pub fn persistent_data_container(
        &self,
    ) -> Result<crate::persistence::PersistentDataContainer<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = Campfire::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::TileState = temp_clone.into();
        real.persistent_data_container()
    }
    // SUPER CLASS: BlockState
    pub fn set_metadata(
        &self,
        arg0: impl Into<String>,
        arg1: impl Into<crate::metadata::MetadataValue<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::metadata::Metadatable = temp_clone.into();
        real.set_metadata(arg0, arg1)
    }
    pub fn get_metadata(
        &self,
        arg0: impl Into<String>,
    ) -> Result<Vec<crate::metadata::MetadataValue<'mc>>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::metadata::Metadatable = temp_clone.into();
        real.get_metadata(arg0)
    }
    pub fn has_metadata(
        &self,
        arg0: impl Into<String>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::metadata::Metadatable = temp_clone.into();
        real.has_metadata(arg0)
    }
    pub fn remove_metadata(
        &self,
        arg0: impl Into<String>,
        arg1: impl Into<crate::plugin::Plugin<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::metadata::Metadatable = temp_clone.into();
        real.remove_metadata(arg0, arg1)
    }
    pub fn data(&self) -> Result<crate::material::MaterialData<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.data()
    }
    pub fn world(&self) -> Result<crate::World<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.world()
    }
    pub fn set_type(
        &self,
        arg0: impl Into<crate::Material<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.set_type(arg0)
    }
    pub fn block(&self) -> Result<crate::block::Block<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.block()
    }
    pub fn block_data(
        &self,
    ) -> Result<crate::block::data::BlockData<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.block_data()
    }
    pub fn light_level(&self) -> Result<i8, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.light_level()
    }
    pub fn x(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.x()
    }
    pub fn y(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.y()
    }
    pub fn z(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.z()
    }
    pub fn chunk(&self) -> Result<crate::Chunk<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.chunk()
    }
    pub fn set_data(
        &self,
        arg0: impl Into<crate::material::MaterialData<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.set_data(arg0)
    }
    pub fn set_block_data(
        &self,
        arg0: impl Into<crate::block::data::BlockData<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.set_block_data(arg0)
    }

    pub fn raw_data(&self) -> Result<i8, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.raw_data()
    }

    pub fn set_raw_data(&self, arg0: i8) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.set_raw_data(arg0)
    }
    pub fn is_placed(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.is_placed()
    }
    pub fn update_with_boolean(
        &self,
        arg0: std::option::Option<bool>,
        arg1: std::option::Option<bool>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "Z";
            let val_1 = jni::objects::JValueGen::Bool(a.into());
            args.push(val_1);
        }
        if let Some(a) = arg1 {
            sig += "Z";
            let val_2 = jni::objects::JValueGen::Bool(a.into());
            args.push(val_2);
        }
        sig += ")Z";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "update", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    pub fn location(&self) -> Result<crate::Location<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.location()
    }
    pub fn get_type(&self) -> Result<crate::Material<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.get_type()
    }
    // SUPER CLASS: PersistentDataHolder

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::block::TileState<'mc>> for Campfire<'mc> {
    fn into(self) -> crate::block::TileState<'mc> {
        crate::block::TileState::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting Campfire into crate::block::TileState")
    }
}
pub enum Biome<'mc> {
    Ocean { inner: BiomeStruct<'mc> },
    Plains { inner: BiomeStruct<'mc> },
    Desert { inner: BiomeStruct<'mc> },
    WindsweptHills { inner: BiomeStruct<'mc> },
    Forest { inner: BiomeStruct<'mc> },
    Taiga { inner: BiomeStruct<'mc> },
    Swamp { inner: BiomeStruct<'mc> },
    MangroveSwamp { inner: BiomeStruct<'mc> },
    River { inner: BiomeStruct<'mc> },
    NetherWastes { inner: BiomeStruct<'mc> },
    TheEnd { inner: BiomeStruct<'mc> },
    FrozenOcean { inner: BiomeStruct<'mc> },
    FrozenRiver { inner: BiomeStruct<'mc> },
    SnowyPlains { inner: BiomeStruct<'mc> },
    MushroomFields { inner: BiomeStruct<'mc> },
    Beach { inner: BiomeStruct<'mc> },
    Jungle { inner: BiomeStruct<'mc> },
    SparseJungle { inner: BiomeStruct<'mc> },
    DeepOcean { inner: BiomeStruct<'mc> },
    StonyShore { inner: BiomeStruct<'mc> },
    SnowyBeach { inner: BiomeStruct<'mc> },
    BirchForest { inner: BiomeStruct<'mc> },
    DarkForest { inner: BiomeStruct<'mc> },
    SnowyTaiga { inner: BiomeStruct<'mc> },
    OldGrowthPineTaiga { inner: BiomeStruct<'mc> },
    WindsweptForest { inner: BiomeStruct<'mc> },
    Savanna { inner: BiomeStruct<'mc> },
    SavannaPlateau { inner: BiomeStruct<'mc> },
    Badlands { inner: BiomeStruct<'mc> },
    WoodedBadlands { inner: BiomeStruct<'mc> },
    SmallEndIslands { inner: BiomeStruct<'mc> },
    EndMidlands { inner: BiomeStruct<'mc> },
    EndHighlands { inner: BiomeStruct<'mc> },
    EndBarrens { inner: BiomeStruct<'mc> },
    WarmOcean { inner: BiomeStruct<'mc> },
    LukewarmOcean { inner: BiomeStruct<'mc> },
    ColdOcean { inner: BiomeStruct<'mc> },
    DeepLukewarmOcean { inner: BiomeStruct<'mc> },
    DeepColdOcean { inner: BiomeStruct<'mc> },
    DeepFrozenOcean { inner: BiomeStruct<'mc> },
    TheVoid { inner: BiomeStruct<'mc> },
    SunflowerPlains { inner: BiomeStruct<'mc> },
    WindsweptGravellyHills { inner: BiomeStruct<'mc> },
    FlowerForest { inner: BiomeStruct<'mc> },
    IceSpikes { inner: BiomeStruct<'mc> },
    OldGrowthBirchForest { inner: BiomeStruct<'mc> },
    OldGrowthSpruceTaiga { inner: BiomeStruct<'mc> },
    WindsweptSavanna { inner: BiomeStruct<'mc> },
    ErodedBadlands { inner: BiomeStruct<'mc> },
    BambooJungle { inner: BiomeStruct<'mc> },
    SoulSandValley { inner: BiomeStruct<'mc> },
    CrimsonForest { inner: BiomeStruct<'mc> },
    WarpedForest { inner: BiomeStruct<'mc> },
    BasaltDeltas { inner: BiomeStruct<'mc> },
    DripstoneCaves { inner: BiomeStruct<'mc> },
    LushCaves { inner: BiomeStruct<'mc> },
    DeepDark { inner: BiomeStruct<'mc> },
    Meadow { inner: BiomeStruct<'mc> },
    Grove { inner: BiomeStruct<'mc> },
    SnowySlopes { inner: BiomeStruct<'mc> },
    FrozenPeaks { inner: BiomeStruct<'mc> },
    JaggedPeaks { inner: BiomeStruct<'mc> },
    StonyPeaks { inner: BiomeStruct<'mc> },
    CherryGrove { inner: BiomeStruct<'mc> },
    Custom { inner: BiomeStruct<'mc> },
}
impl<'mc> std::fmt::Display for Biome<'mc> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Biome::Ocean { .. } => f.write_str("OCEAN"),
            Biome::Plains { .. } => f.write_str("PLAINS"),
            Biome::Desert { .. } => f.write_str("DESERT"),
            Biome::WindsweptHills { .. } => f.write_str("WINDSWEPT_HILLS"),
            Biome::Forest { .. } => f.write_str("FOREST"),
            Biome::Taiga { .. } => f.write_str("TAIGA"),
            Biome::Swamp { .. } => f.write_str("SWAMP"),
            Biome::MangroveSwamp { .. } => f.write_str("MANGROVE_SWAMP"),
            Biome::River { .. } => f.write_str("RIVER"),
            Biome::NetherWastes { .. } => f.write_str("NETHER_WASTES"),
            Biome::TheEnd { .. } => f.write_str("THE_END"),
            Biome::FrozenOcean { .. } => f.write_str("FROZEN_OCEAN"),
            Biome::FrozenRiver { .. } => f.write_str("FROZEN_RIVER"),
            Biome::SnowyPlains { .. } => f.write_str("SNOWY_PLAINS"),
            Biome::MushroomFields { .. } => f.write_str("MUSHROOM_FIELDS"),
            Biome::Beach { .. } => f.write_str("BEACH"),
            Biome::Jungle { .. } => f.write_str("JUNGLE"),
            Biome::SparseJungle { .. } => f.write_str("SPARSE_JUNGLE"),
            Biome::DeepOcean { .. } => f.write_str("DEEP_OCEAN"),
            Biome::StonyShore { .. } => f.write_str("STONY_SHORE"),
            Biome::SnowyBeach { .. } => f.write_str("SNOWY_BEACH"),
            Biome::BirchForest { .. } => f.write_str("BIRCH_FOREST"),
            Biome::DarkForest { .. } => f.write_str("DARK_FOREST"),
            Biome::SnowyTaiga { .. } => f.write_str("SNOWY_TAIGA"),
            Biome::OldGrowthPineTaiga { .. } => f.write_str("OLD_GROWTH_PINE_TAIGA"),
            Biome::WindsweptForest { .. } => f.write_str("WINDSWEPT_FOREST"),
            Biome::Savanna { .. } => f.write_str("SAVANNA"),
            Biome::SavannaPlateau { .. } => f.write_str("SAVANNA_PLATEAU"),
            Biome::Badlands { .. } => f.write_str("BADLANDS"),
            Biome::WoodedBadlands { .. } => f.write_str("WOODED_BADLANDS"),
            Biome::SmallEndIslands { .. } => f.write_str("SMALL_END_ISLANDS"),
            Biome::EndMidlands { .. } => f.write_str("END_MIDLANDS"),
            Biome::EndHighlands { .. } => f.write_str("END_HIGHLANDS"),
            Biome::EndBarrens { .. } => f.write_str("END_BARRENS"),
            Biome::WarmOcean { .. } => f.write_str("WARM_OCEAN"),
            Biome::LukewarmOcean { .. } => f.write_str("LUKEWARM_OCEAN"),
            Biome::ColdOcean { .. } => f.write_str("COLD_OCEAN"),
            Biome::DeepLukewarmOcean { .. } => f.write_str("DEEP_LUKEWARM_OCEAN"),
            Biome::DeepColdOcean { .. } => f.write_str("DEEP_COLD_OCEAN"),
            Biome::DeepFrozenOcean { .. } => f.write_str("DEEP_FROZEN_OCEAN"),
            Biome::TheVoid { .. } => f.write_str("THE_VOID"),
            Biome::SunflowerPlains { .. } => f.write_str("SUNFLOWER_PLAINS"),
            Biome::WindsweptGravellyHills { .. } => f.write_str("WINDSWEPT_GRAVELLY_HILLS"),
            Biome::FlowerForest { .. } => f.write_str("FLOWER_FOREST"),
            Biome::IceSpikes { .. } => f.write_str("ICE_SPIKES"),
            Biome::OldGrowthBirchForest { .. } => f.write_str("OLD_GROWTH_BIRCH_FOREST"),
            Biome::OldGrowthSpruceTaiga { .. } => f.write_str("OLD_GROWTH_SPRUCE_TAIGA"),
            Biome::WindsweptSavanna { .. } => f.write_str("WINDSWEPT_SAVANNA"),
            Biome::ErodedBadlands { .. } => f.write_str("ERODED_BADLANDS"),
            Biome::BambooJungle { .. } => f.write_str("BAMBOO_JUNGLE"),
            Biome::SoulSandValley { .. } => f.write_str("SOUL_SAND_VALLEY"),
            Biome::CrimsonForest { .. } => f.write_str("CRIMSON_FOREST"),
            Biome::WarpedForest { .. } => f.write_str("WARPED_FOREST"),
            Biome::BasaltDeltas { .. } => f.write_str("BASALT_DELTAS"),
            Biome::DripstoneCaves { .. } => f.write_str("DRIPSTONE_CAVES"),
            Biome::LushCaves { .. } => f.write_str("LUSH_CAVES"),
            Biome::DeepDark { .. } => f.write_str("DEEP_DARK"),
            Biome::Meadow { .. } => f.write_str("MEADOW"),
            Biome::Grove { .. } => f.write_str("GROVE"),
            Biome::SnowySlopes { .. } => f.write_str("SNOWY_SLOPES"),
            Biome::FrozenPeaks { .. } => f.write_str("FROZEN_PEAKS"),
            Biome::JaggedPeaks { .. } => f.write_str("JAGGED_PEAKS"),
            Biome::StonyPeaks { .. } => f.write_str("STONY_PEAKS"),
            Biome::CherryGrove { .. } => f.write_str("CHERRY_GROVE"),
            Biome::Custom { .. } => f.write_str("CUSTOM"),
        }
    }
}

impl<'mc> Biome<'mc> {
    pub fn value_of(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<String>,
    ) -> Result<Biome<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(env.new_string(arg0.into())?);
        let cls = env.find_class("org/bukkit/block/Biome");
        let cls = env.translate_error_with_class(cls)?;
        let res = env.call_static_method(
            cls,
            "valueOf",
            "(Ljava/lang/String;)Lorg/bukkit/block/Biome;",
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
            "OCEAN" => Ok(Biome::Ocean {
                inner: BiomeStruct::from_raw(env, obj)?,
            }),
            "PLAINS" => Ok(Biome::Plains {
                inner: BiomeStruct::from_raw(env, obj)?,
            }),
            "DESERT" => Ok(Biome::Desert {
                inner: BiomeStruct::from_raw(env, obj)?,
            }),
            "WINDSWEPT_HILLS" => Ok(Biome::WindsweptHills {
                inner: BiomeStruct::from_raw(env, obj)?,
            }),
            "FOREST" => Ok(Biome::Forest {
                inner: BiomeStruct::from_raw(env, obj)?,
            }),
            "TAIGA" => Ok(Biome::Taiga {
                inner: BiomeStruct::from_raw(env, obj)?,
            }),
            "SWAMP" => Ok(Biome::Swamp {
                inner: BiomeStruct::from_raw(env, obj)?,
            }),
            "MANGROVE_SWAMP" => Ok(Biome::MangroveSwamp {
                inner: BiomeStruct::from_raw(env, obj)?,
            }),
            "RIVER" => Ok(Biome::River {
                inner: BiomeStruct::from_raw(env, obj)?,
            }),
            "NETHER_WASTES" => Ok(Biome::NetherWastes {
                inner: BiomeStruct::from_raw(env, obj)?,
            }),
            "THE_END" => Ok(Biome::TheEnd {
                inner: BiomeStruct::from_raw(env, obj)?,
            }),
            "FROZEN_OCEAN" => Ok(Biome::FrozenOcean {
                inner: BiomeStruct::from_raw(env, obj)?,
            }),
            "FROZEN_RIVER" => Ok(Biome::FrozenRiver {
                inner: BiomeStruct::from_raw(env, obj)?,
            }),
            "SNOWY_PLAINS" => Ok(Biome::SnowyPlains {
                inner: BiomeStruct::from_raw(env, obj)?,
            }),
            "MUSHROOM_FIELDS" => Ok(Biome::MushroomFields {
                inner: BiomeStruct::from_raw(env, obj)?,
            }),
            "BEACH" => Ok(Biome::Beach {
                inner: BiomeStruct::from_raw(env, obj)?,
            }),
            "JUNGLE" => Ok(Biome::Jungle {
                inner: BiomeStruct::from_raw(env, obj)?,
            }),
            "SPARSE_JUNGLE" => Ok(Biome::SparseJungle {
                inner: BiomeStruct::from_raw(env, obj)?,
            }),
            "DEEP_OCEAN" => Ok(Biome::DeepOcean {
                inner: BiomeStruct::from_raw(env, obj)?,
            }),
            "STONY_SHORE" => Ok(Biome::StonyShore {
                inner: BiomeStruct::from_raw(env, obj)?,
            }),
            "SNOWY_BEACH" => Ok(Biome::SnowyBeach {
                inner: BiomeStruct::from_raw(env, obj)?,
            }),
            "BIRCH_FOREST" => Ok(Biome::BirchForest {
                inner: BiomeStruct::from_raw(env, obj)?,
            }),
            "DARK_FOREST" => Ok(Biome::DarkForest {
                inner: BiomeStruct::from_raw(env, obj)?,
            }),
            "SNOWY_TAIGA" => Ok(Biome::SnowyTaiga {
                inner: BiomeStruct::from_raw(env, obj)?,
            }),
            "OLD_GROWTH_PINE_TAIGA" => Ok(Biome::OldGrowthPineTaiga {
                inner: BiomeStruct::from_raw(env, obj)?,
            }),
            "WINDSWEPT_FOREST" => Ok(Biome::WindsweptForest {
                inner: BiomeStruct::from_raw(env, obj)?,
            }),
            "SAVANNA" => Ok(Biome::Savanna {
                inner: BiomeStruct::from_raw(env, obj)?,
            }),
            "SAVANNA_PLATEAU" => Ok(Biome::SavannaPlateau {
                inner: BiomeStruct::from_raw(env, obj)?,
            }),
            "BADLANDS" => Ok(Biome::Badlands {
                inner: BiomeStruct::from_raw(env, obj)?,
            }),
            "WOODED_BADLANDS" => Ok(Biome::WoodedBadlands {
                inner: BiomeStruct::from_raw(env, obj)?,
            }),
            "SMALL_END_ISLANDS" => Ok(Biome::SmallEndIslands {
                inner: BiomeStruct::from_raw(env, obj)?,
            }),
            "END_MIDLANDS" => Ok(Biome::EndMidlands {
                inner: BiomeStruct::from_raw(env, obj)?,
            }),
            "END_HIGHLANDS" => Ok(Biome::EndHighlands {
                inner: BiomeStruct::from_raw(env, obj)?,
            }),
            "END_BARRENS" => Ok(Biome::EndBarrens {
                inner: BiomeStruct::from_raw(env, obj)?,
            }),
            "WARM_OCEAN" => Ok(Biome::WarmOcean {
                inner: BiomeStruct::from_raw(env, obj)?,
            }),
            "LUKEWARM_OCEAN" => Ok(Biome::LukewarmOcean {
                inner: BiomeStruct::from_raw(env, obj)?,
            }),
            "COLD_OCEAN" => Ok(Biome::ColdOcean {
                inner: BiomeStruct::from_raw(env, obj)?,
            }),
            "DEEP_LUKEWARM_OCEAN" => Ok(Biome::DeepLukewarmOcean {
                inner: BiomeStruct::from_raw(env, obj)?,
            }),
            "DEEP_COLD_OCEAN" => Ok(Biome::DeepColdOcean {
                inner: BiomeStruct::from_raw(env, obj)?,
            }),
            "DEEP_FROZEN_OCEAN" => Ok(Biome::DeepFrozenOcean {
                inner: BiomeStruct::from_raw(env, obj)?,
            }),
            "THE_VOID" => Ok(Biome::TheVoid {
                inner: BiomeStruct::from_raw(env, obj)?,
            }),
            "SUNFLOWER_PLAINS" => Ok(Biome::SunflowerPlains {
                inner: BiomeStruct::from_raw(env, obj)?,
            }),
            "WINDSWEPT_GRAVELLY_HILLS" => Ok(Biome::WindsweptGravellyHills {
                inner: BiomeStruct::from_raw(env, obj)?,
            }),
            "FLOWER_FOREST" => Ok(Biome::FlowerForest {
                inner: BiomeStruct::from_raw(env, obj)?,
            }),
            "ICE_SPIKES" => Ok(Biome::IceSpikes {
                inner: BiomeStruct::from_raw(env, obj)?,
            }),
            "OLD_GROWTH_BIRCH_FOREST" => Ok(Biome::OldGrowthBirchForest {
                inner: BiomeStruct::from_raw(env, obj)?,
            }),
            "OLD_GROWTH_SPRUCE_TAIGA" => Ok(Biome::OldGrowthSpruceTaiga {
                inner: BiomeStruct::from_raw(env, obj)?,
            }),
            "WINDSWEPT_SAVANNA" => Ok(Biome::WindsweptSavanna {
                inner: BiomeStruct::from_raw(env, obj)?,
            }),
            "ERODED_BADLANDS" => Ok(Biome::ErodedBadlands {
                inner: BiomeStruct::from_raw(env, obj)?,
            }),
            "BAMBOO_JUNGLE" => Ok(Biome::BambooJungle {
                inner: BiomeStruct::from_raw(env, obj)?,
            }),
            "SOUL_SAND_VALLEY" => Ok(Biome::SoulSandValley {
                inner: BiomeStruct::from_raw(env, obj)?,
            }),
            "CRIMSON_FOREST" => Ok(Biome::CrimsonForest {
                inner: BiomeStruct::from_raw(env, obj)?,
            }),
            "WARPED_FOREST" => Ok(Biome::WarpedForest {
                inner: BiomeStruct::from_raw(env, obj)?,
            }),
            "BASALT_DELTAS" => Ok(Biome::BasaltDeltas {
                inner: BiomeStruct::from_raw(env, obj)?,
            }),
            "DRIPSTONE_CAVES" => Ok(Biome::DripstoneCaves {
                inner: BiomeStruct::from_raw(env, obj)?,
            }),
            "LUSH_CAVES" => Ok(Biome::LushCaves {
                inner: BiomeStruct::from_raw(env, obj)?,
            }),
            "DEEP_DARK" => Ok(Biome::DeepDark {
                inner: BiomeStruct::from_raw(env, obj)?,
            }),
            "MEADOW" => Ok(Biome::Meadow {
                inner: BiomeStruct::from_raw(env, obj)?,
            }),
            "GROVE" => Ok(Biome::Grove {
                inner: BiomeStruct::from_raw(env, obj)?,
            }),
            "SNOWY_SLOPES" => Ok(Biome::SnowySlopes {
                inner: BiomeStruct::from_raw(env, obj)?,
            }),
            "FROZEN_PEAKS" => Ok(Biome::FrozenPeaks {
                inner: BiomeStruct::from_raw(env, obj)?,
            }),
            "JAGGED_PEAKS" => Ok(Biome::JaggedPeaks {
                inner: BiomeStruct::from_raw(env, obj)?,
            }),
            "STONY_PEAKS" => Ok(Biome::StonyPeaks {
                inner: BiomeStruct::from_raw(env, obj)?,
            }),
            "CHERRY_GROVE" => Ok(Biome::CherryGrove {
                inner: BiomeStruct::from_raw(env, obj)?,
            }),
            "CUSTOM" => Ok(Biome::Custom {
                inner: BiomeStruct::from_raw(env, obj)?,
            }),

            _ => Err(eyre::eyre!("String gaven for variant was invalid").into()),
        }
    }
}

#[repr(C)]
pub struct BiomeStruct<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for Biome<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        match self {
            Self::Ocean { inner } => inner.0.clone(),
            Self::Plains { inner } => inner.0.clone(),
            Self::Desert { inner } => inner.0.clone(),
            Self::WindsweptHills { inner } => inner.0.clone(),
            Self::Forest { inner } => inner.0.clone(),
            Self::Taiga { inner } => inner.0.clone(),
            Self::Swamp { inner } => inner.0.clone(),
            Self::MangroveSwamp { inner } => inner.0.clone(),
            Self::River { inner } => inner.0.clone(),
            Self::NetherWastes { inner } => inner.0.clone(),
            Self::TheEnd { inner } => inner.0.clone(),
            Self::FrozenOcean { inner } => inner.0.clone(),
            Self::FrozenRiver { inner } => inner.0.clone(),
            Self::SnowyPlains { inner } => inner.0.clone(),
            Self::MushroomFields { inner } => inner.0.clone(),
            Self::Beach { inner } => inner.0.clone(),
            Self::Jungle { inner } => inner.0.clone(),
            Self::SparseJungle { inner } => inner.0.clone(),
            Self::DeepOcean { inner } => inner.0.clone(),
            Self::StonyShore { inner } => inner.0.clone(),
            Self::SnowyBeach { inner } => inner.0.clone(),
            Self::BirchForest { inner } => inner.0.clone(),
            Self::DarkForest { inner } => inner.0.clone(),
            Self::SnowyTaiga { inner } => inner.0.clone(),
            Self::OldGrowthPineTaiga { inner } => inner.0.clone(),
            Self::WindsweptForest { inner } => inner.0.clone(),
            Self::Savanna { inner } => inner.0.clone(),
            Self::SavannaPlateau { inner } => inner.0.clone(),
            Self::Badlands { inner } => inner.0.clone(),
            Self::WoodedBadlands { inner } => inner.0.clone(),
            Self::SmallEndIslands { inner } => inner.0.clone(),
            Self::EndMidlands { inner } => inner.0.clone(),
            Self::EndHighlands { inner } => inner.0.clone(),
            Self::EndBarrens { inner } => inner.0.clone(),
            Self::WarmOcean { inner } => inner.0.clone(),
            Self::LukewarmOcean { inner } => inner.0.clone(),
            Self::ColdOcean { inner } => inner.0.clone(),
            Self::DeepLukewarmOcean { inner } => inner.0.clone(),
            Self::DeepColdOcean { inner } => inner.0.clone(),
            Self::DeepFrozenOcean { inner } => inner.0.clone(),
            Self::TheVoid { inner } => inner.0.clone(),
            Self::SunflowerPlains { inner } => inner.0.clone(),
            Self::WindsweptGravellyHills { inner } => inner.0.clone(),
            Self::FlowerForest { inner } => inner.0.clone(),
            Self::IceSpikes { inner } => inner.0.clone(),
            Self::OldGrowthBirchForest { inner } => inner.0.clone(),
            Self::OldGrowthSpruceTaiga { inner } => inner.0.clone(),
            Self::WindsweptSavanna { inner } => inner.0.clone(),
            Self::ErodedBadlands { inner } => inner.0.clone(),
            Self::BambooJungle { inner } => inner.0.clone(),
            Self::SoulSandValley { inner } => inner.0.clone(),
            Self::CrimsonForest { inner } => inner.0.clone(),
            Self::WarpedForest { inner } => inner.0.clone(),
            Self::BasaltDeltas { inner } => inner.0.clone(),
            Self::DripstoneCaves { inner } => inner.0.clone(),
            Self::LushCaves { inner } => inner.0.clone(),
            Self::DeepDark { inner } => inner.0.clone(),
            Self::Meadow { inner } => inner.0.clone(),
            Self::Grove { inner } => inner.0.clone(),
            Self::SnowySlopes { inner } => inner.0.clone(),
            Self::FrozenPeaks { inner } => inner.0.clone(),
            Self::JaggedPeaks { inner } => inner.0.clone(),
            Self::StonyPeaks { inner } => inner.0.clone(),
            Self::CherryGrove { inner } => inner.0.clone(),
            Self::Custom { inner } => inner.0.clone(),
        }
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        match self {
            Self::Ocean { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Plains { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Desert { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::WindsweptHills { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::Forest { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Taiga { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Swamp { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::MangroveSwamp { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::River { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::NetherWastes { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::TheEnd { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::FrozenOcean { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::FrozenRiver { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::SnowyPlains { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::MushroomFields { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::Beach { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Jungle { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::SparseJungle { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::DeepOcean { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::StonyShore { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::SnowyBeach { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::BirchForest { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::DarkForest { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::SnowyTaiga { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::OldGrowthPineTaiga { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::WindsweptForest { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::Savanna { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::SavannaPlateau { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::Badlands { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::WoodedBadlands { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::SmallEndIslands { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::EndMidlands { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::EndHighlands { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::EndBarrens { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::WarmOcean { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::LukewarmOcean { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::ColdOcean { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::DeepLukewarmOcean { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::DeepColdOcean { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::DeepFrozenOcean { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::TheVoid { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::SunflowerPlains { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::WindsweptGravellyHills { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::FlowerForest { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::IceSpikes { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::OldGrowthBirchForest { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::OldGrowthSpruceTaiga { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::WindsweptSavanna { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::ErodedBadlands { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::BambooJungle { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::SoulSandValley { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::CrimsonForest { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::WarpedForest { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::BasaltDeltas { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::DripstoneCaves { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::LushCaves { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::DeepDark { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Meadow { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Grove { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::SnowySlopes { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::FrozenPeaks { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::JaggedPeaks { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::StonyPeaks { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::CherryGrove { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::Custom { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
        }
    }
}
impl<'mc> JNIInstantiatable<'mc> for Biome<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Biome from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/block/Biome")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Biome object, got {}",
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
                "OCEAN" => Ok(Biome::Ocean {
                    inner: BiomeStruct::from_raw(env, obj)?,
                }),
                "PLAINS" => Ok(Biome::Plains {
                    inner: BiomeStruct::from_raw(env, obj)?,
                }),
                "DESERT" => Ok(Biome::Desert {
                    inner: BiomeStruct::from_raw(env, obj)?,
                }),
                "WINDSWEPT_HILLS" => Ok(Biome::WindsweptHills {
                    inner: BiomeStruct::from_raw(env, obj)?,
                }),
                "FOREST" => Ok(Biome::Forest {
                    inner: BiomeStruct::from_raw(env, obj)?,
                }),
                "TAIGA" => Ok(Biome::Taiga {
                    inner: BiomeStruct::from_raw(env, obj)?,
                }),
                "SWAMP" => Ok(Biome::Swamp {
                    inner: BiomeStruct::from_raw(env, obj)?,
                }),
                "MANGROVE_SWAMP" => Ok(Biome::MangroveSwamp {
                    inner: BiomeStruct::from_raw(env, obj)?,
                }),
                "RIVER" => Ok(Biome::River {
                    inner: BiomeStruct::from_raw(env, obj)?,
                }),
                "NETHER_WASTES" => Ok(Biome::NetherWastes {
                    inner: BiomeStruct::from_raw(env, obj)?,
                }),
                "THE_END" => Ok(Biome::TheEnd {
                    inner: BiomeStruct::from_raw(env, obj)?,
                }),
                "FROZEN_OCEAN" => Ok(Biome::FrozenOcean {
                    inner: BiomeStruct::from_raw(env, obj)?,
                }),
                "FROZEN_RIVER" => Ok(Biome::FrozenRiver {
                    inner: BiomeStruct::from_raw(env, obj)?,
                }),
                "SNOWY_PLAINS" => Ok(Biome::SnowyPlains {
                    inner: BiomeStruct::from_raw(env, obj)?,
                }),
                "MUSHROOM_FIELDS" => Ok(Biome::MushroomFields {
                    inner: BiomeStruct::from_raw(env, obj)?,
                }),
                "BEACH" => Ok(Biome::Beach {
                    inner: BiomeStruct::from_raw(env, obj)?,
                }),
                "JUNGLE" => Ok(Biome::Jungle {
                    inner: BiomeStruct::from_raw(env, obj)?,
                }),
                "SPARSE_JUNGLE" => Ok(Biome::SparseJungle {
                    inner: BiomeStruct::from_raw(env, obj)?,
                }),
                "DEEP_OCEAN" => Ok(Biome::DeepOcean {
                    inner: BiomeStruct::from_raw(env, obj)?,
                }),
                "STONY_SHORE" => Ok(Biome::StonyShore {
                    inner: BiomeStruct::from_raw(env, obj)?,
                }),
                "SNOWY_BEACH" => Ok(Biome::SnowyBeach {
                    inner: BiomeStruct::from_raw(env, obj)?,
                }),
                "BIRCH_FOREST" => Ok(Biome::BirchForest {
                    inner: BiomeStruct::from_raw(env, obj)?,
                }),
                "DARK_FOREST" => Ok(Biome::DarkForest {
                    inner: BiomeStruct::from_raw(env, obj)?,
                }),
                "SNOWY_TAIGA" => Ok(Biome::SnowyTaiga {
                    inner: BiomeStruct::from_raw(env, obj)?,
                }),
                "OLD_GROWTH_PINE_TAIGA" => Ok(Biome::OldGrowthPineTaiga {
                    inner: BiomeStruct::from_raw(env, obj)?,
                }),
                "WINDSWEPT_FOREST" => Ok(Biome::WindsweptForest {
                    inner: BiomeStruct::from_raw(env, obj)?,
                }),
                "SAVANNA" => Ok(Biome::Savanna {
                    inner: BiomeStruct::from_raw(env, obj)?,
                }),
                "SAVANNA_PLATEAU" => Ok(Biome::SavannaPlateau {
                    inner: BiomeStruct::from_raw(env, obj)?,
                }),
                "BADLANDS" => Ok(Biome::Badlands {
                    inner: BiomeStruct::from_raw(env, obj)?,
                }),
                "WOODED_BADLANDS" => Ok(Biome::WoodedBadlands {
                    inner: BiomeStruct::from_raw(env, obj)?,
                }),
                "SMALL_END_ISLANDS" => Ok(Biome::SmallEndIslands {
                    inner: BiomeStruct::from_raw(env, obj)?,
                }),
                "END_MIDLANDS" => Ok(Biome::EndMidlands {
                    inner: BiomeStruct::from_raw(env, obj)?,
                }),
                "END_HIGHLANDS" => Ok(Biome::EndHighlands {
                    inner: BiomeStruct::from_raw(env, obj)?,
                }),
                "END_BARRENS" => Ok(Biome::EndBarrens {
                    inner: BiomeStruct::from_raw(env, obj)?,
                }),
                "WARM_OCEAN" => Ok(Biome::WarmOcean {
                    inner: BiomeStruct::from_raw(env, obj)?,
                }),
                "LUKEWARM_OCEAN" => Ok(Biome::LukewarmOcean {
                    inner: BiomeStruct::from_raw(env, obj)?,
                }),
                "COLD_OCEAN" => Ok(Biome::ColdOcean {
                    inner: BiomeStruct::from_raw(env, obj)?,
                }),
                "DEEP_LUKEWARM_OCEAN" => Ok(Biome::DeepLukewarmOcean {
                    inner: BiomeStruct::from_raw(env, obj)?,
                }),
                "DEEP_COLD_OCEAN" => Ok(Biome::DeepColdOcean {
                    inner: BiomeStruct::from_raw(env, obj)?,
                }),
                "DEEP_FROZEN_OCEAN" => Ok(Biome::DeepFrozenOcean {
                    inner: BiomeStruct::from_raw(env, obj)?,
                }),
                "THE_VOID" => Ok(Biome::TheVoid {
                    inner: BiomeStruct::from_raw(env, obj)?,
                }),
                "SUNFLOWER_PLAINS" => Ok(Biome::SunflowerPlains {
                    inner: BiomeStruct::from_raw(env, obj)?,
                }),
                "WINDSWEPT_GRAVELLY_HILLS" => Ok(Biome::WindsweptGravellyHills {
                    inner: BiomeStruct::from_raw(env, obj)?,
                }),
                "FLOWER_FOREST" => Ok(Biome::FlowerForest {
                    inner: BiomeStruct::from_raw(env, obj)?,
                }),
                "ICE_SPIKES" => Ok(Biome::IceSpikes {
                    inner: BiomeStruct::from_raw(env, obj)?,
                }),
                "OLD_GROWTH_BIRCH_FOREST" => Ok(Biome::OldGrowthBirchForest {
                    inner: BiomeStruct::from_raw(env, obj)?,
                }),
                "OLD_GROWTH_SPRUCE_TAIGA" => Ok(Biome::OldGrowthSpruceTaiga {
                    inner: BiomeStruct::from_raw(env, obj)?,
                }),
                "WINDSWEPT_SAVANNA" => Ok(Biome::WindsweptSavanna {
                    inner: BiomeStruct::from_raw(env, obj)?,
                }),
                "ERODED_BADLANDS" => Ok(Biome::ErodedBadlands {
                    inner: BiomeStruct::from_raw(env, obj)?,
                }),
                "BAMBOO_JUNGLE" => Ok(Biome::BambooJungle {
                    inner: BiomeStruct::from_raw(env, obj)?,
                }),
                "SOUL_SAND_VALLEY" => Ok(Biome::SoulSandValley {
                    inner: BiomeStruct::from_raw(env, obj)?,
                }),
                "CRIMSON_FOREST" => Ok(Biome::CrimsonForest {
                    inner: BiomeStruct::from_raw(env, obj)?,
                }),
                "WARPED_FOREST" => Ok(Biome::WarpedForest {
                    inner: BiomeStruct::from_raw(env, obj)?,
                }),
                "BASALT_DELTAS" => Ok(Biome::BasaltDeltas {
                    inner: BiomeStruct::from_raw(env, obj)?,
                }),
                "DRIPSTONE_CAVES" => Ok(Biome::DripstoneCaves {
                    inner: BiomeStruct::from_raw(env, obj)?,
                }),
                "LUSH_CAVES" => Ok(Biome::LushCaves {
                    inner: BiomeStruct::from_raw(env, obj)?,
                }),
                "DEEP_DARK" => Ok(Biome::DeepDark {
                    inner: BiomeStruct::from_raw(env, obj)?,
                }),
                "MEADOW" => Ok(Biome::Meadow {
                    inner: BiomeStruct::from_raw(env, obj)?,
                }),
                "GROVE" => Ok(Biome::Grove {
                    inner: BiomeStruct::from_raw(env, obj)?,
                }),
                "SNOWY_SLOPES" => Ok(Biome::SnowySlopes {
                    inner: BiomeStruct::from_raw(env, obj)?,
                }),
                "FROZEN_PEAKS" => Ok(Biome::FrozenPeaks {
                    inner: BiomeStruct::from_raw(env, obj)?,
                }),
                "JAGGED_PEAKS" => Ok(Biome::JaggedPeaks {
                    inner: BiomeStruct::from_raw(env, obj)?,
                }),
                "STONY_PEAKS" => Ok(Biome::StonyPeaks {
                    inner: BiomeStruct::from_raw(env, obj)?,
                }),
                "CHERRY_GROVE" => Ok(Biome::CherryGrove {
                    inner: BiomeStruct::from_raw(env, obj)?,
                }),
                "CUSTOM" => Ok(Biome::Custom {
                    inner: BiomeStruct::from_raw(env, obj)?,
                }),
                _ => Err(eyre::eyre!("String gaven for variant was invalid").into()),
            }
        }
    }
}

impl<'mc> JNIRaw<'mc> for BiomeStruct<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for BiomeStruct<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate BiomeStruct from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/block/Biome")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a BiomeStruct object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> BiomeStruct<'mc> {
    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
/// Represents a captured state of a smoker.
///
/// This is a representation of an abstract class.
#[repr(C)]
pub struct Smoker<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for Smoker<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for Smoker<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Smoker from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/block/Smoker")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Smoker object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> Smoker<'mc> {
    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::block::Furnace<'mc>> for Smoker<'mc> {
    fn into(self) -> crate::block::Furnace<'mc> {
        crate::block::Furnace::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting Smoker into crate::block::Furnace")
    }
}
/// Represents a captured state of a calibrated sculk sensor
///
/// This is a representation of an abstract class.
#[repr(C)]
pub struct CalibratedSculkSensor<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for CalibratedSculkSensor<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for CalibratedSculkSensor<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate CalibratedSculkSensor from null object."
            )
            .into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/block/CalibratedSculkSensor")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a CalibratedSculkSensor object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> CalibratedSculkSensor<'mc> {
    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::block::SculkSensor<'mc>> for CalibratedSculkSensor<'mc> {
    fn into(self) -> crate::block::SculkSensor<'mc> {
        crate::block::SculkSensor::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting CalibratedSculkSensor into crate::block::SculkSensor")
    }
}
/// Represents a structure block that can save and load blocks from a file. They can only be used by OPs, and are not obtainable in survival.
///
/// This is a representation of an abstract class.
#[repr(C)]
pub struct Structure<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for Structure<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for Structure<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Structure from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/block/Structure")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Structure object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> Structure<'mc> {
    pub fn seed(&self) -> Result<i64, Box<dyn std::error::Error>> {
        let sig = String::from("()J");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getSeed", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.j()?)
    }
    /// The seed used to determine which blocks will be removed upon loading. <a href="#getIntegrity()"><code>getIntegrity()</code></a> and seed are used together to determine which blocks are randomly removed to mimic "decay."
    pub fn set_seed(&self, arg0: i64) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(J)V");
        let val_1 = jni::objects::JValueGen::Long(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setSeed",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn rotation(
        &self,
    ) -> Result<crate::block::structure::StructureRotation<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/block/structure/StructureRotation;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getRotation", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::block::structure::StructureRotation::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn set_rotation(
        &self,
        arg0: impl Into<crate::block::structure::StructureRotation<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/structure/StructureRotation;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setRotation",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn set_metadata(&self, arg0: impl Into<String>) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/String;)V");
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(arg0.into())?,
        ));
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setMetadata",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn metadata(&self) -> Result<String, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/String;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getMetadata", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }

    pub fn relative_position(
        &self,
    ) -> Result<crate::util::BlockVector<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/util/BlockVector;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getRelativePosition",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::util::BlockVector::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn structure_name(&self) -> Result<String, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/String;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getStructureName",
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

    pub fn set_structure_name(
        &self,
        arg0: impl Into<String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/String;)V");
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(arg0.into())?,
        ));
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setStructureName",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn author(&self) -> Result<String, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/String;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getAuthor", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }

    pub fn set_author_with_string(
        &self,
        arg0: impl Into<String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Ljava/lang/String;";
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(arg0.into())?,
        ));
        args.push(val_1);
        sig += ")V";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "setAuthor", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn set_relative_position(
        &self,
        arg0: impl Into<crate::util::BlockVector<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/util/BlockVector;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setRelativePosition",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn structure_size(
        &self,
    ) -> Result<crate::util::BlockVector<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/util/BlockVector;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getStructureSize",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::util::BlockVector::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn set_structure_size(
        &self,
        arg0: impl Into<crate::util::BlockVector<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/util/BlockVector;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setStructureSize",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn set_mirror(
        &self,
        arg0: impl Into<crate::block::structure::Mirror<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/structure/Mirror;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setMirror",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn mirror(
        &self,
    ) -> Result<crate::block::structure::Mirror<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/block/structure/Mirror;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getMirror", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::block::structure::Mirror::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn set_usage_mode(
        &self,
        arg0: impl Into<crate::block::structure::UsageMode<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/structure/UsageMode;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setUsageMode",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn usage_mode(
        &self,
    ) -> Result<crate::block::structure::UsageMode<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/block/structure/UsageMode;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getUsageMode", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::block::structure::UsageMode::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// While in <a href="structure/UsageMode.html#SAVE"><code>UsageMode.SAVE</code></a> mode, this will ignore any entities when saving the structure.
    ///
    /// While in <a href="structure/UsageMode.html#LOAD"><code>UsageMode.LOAD</code></a> mode this will ignore any entities that were saved to file.
    pub fn set_ignore_entities(&self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        let val_1 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setIgnoreEntities",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn is_ignore_entities(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "isIgnoreEntities",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Set if the structure outline should show air blocks.
    pub fn set_show_air(&self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        let val_1 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setShowAir",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn is_show_air(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isShowAir", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Set if this structure box should show the bounding box.
    pub fn set_bounding_box_visible(&self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        let val_1 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setBoundingBoxVisible",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn is_bounding_box_visible(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "isBoundingBoxVisible",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Set the integrity of the structure. Integrity must be between 0.0 and 1.0 Lower integrity values will result in more blocks being removed when loading a structure. Integrity and <a href="#getSeed()"><code>getSeed()</code></a> are used together to determine which blocks are randomly removed to mimic "decay."
    pub fn set_integrity(&self, arg0: f32) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(F)V");
        let val_1 = jni::objects::JValueGen::Float(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setIntegrity",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn integrity(&self) -> Result<f32, Box<dyn std::error::Error>> {
        let sig = String::from("()F");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getIntegrity", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.f()?)
    }
    pub fn persistent_data_container(
        &self,
    ) -> Result<crate::persistence::PersistentDataContainer<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = Structure::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::TileState = temp_clone.into();
        real.persistent_data_container()
    }
    // SUPER CLASS: BlockState
    pub fn has_metadata(
        &self,
        arg0: impl Into<String>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::metadata::Metadatable = temp_clone.into();
        real.has_metadata(arg0)
    }
    pub fn remove_metadata(
        &self,
        arg0: impl Into<String>,
        arg1: impl Into<crate::plugin::Plugin<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::metadata::Metadatable = temp_clone.into();
        real.remove_metadata(arg0, arg1)
    }
    pub fn data(&self) -> Result<crate::material::MaterialData<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.data()
    }
    pub fn world(&self) -> Result<crate::World<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.world()
    }
    pub fn set_type(
        &self,
        arg0: impl Into<crate::Material<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.set_type(arg0)
    }
    pub fn block(&self) -> Result<crate::block::Block<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.block()
    }
    pub fn block_data(
        &self,
    ) -> Result<crate::block::data::BlockData<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.block_data()
    }
    pub fn light_level(&self) -> Result<i8, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.light_level()
    }
    pub fn x(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.x()
    }
    pub fn y(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.y()
    }
    pub fn z(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.z()
    }
    pub fn chunk(&self) -> Result<crate::Chunk<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.chunk()
    }
    pub fn set_data(
        &self,
        arg0: impl Into<crate::material::MaterialData<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.set_data(arg0)
    }
    pub fn set_block_data(
        &self,
        arg0: impl Into<crate::block::data::BlockData<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.set_block_data(arg0)
    }

    pub fn raw_data(&self) -> Result<i8, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.raw_data()
    }

    pub fn set_raw_data(&self, arg0: i8) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.set_raw_data(arg0)
    }
    pub fn is_placed(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.is_placed()
    }
    pub fn update_with_boolean(
        &self,
        arg0: std::option::Option<bool>,
        arg1: std::option::Option<bool>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "Z";
            let val_1 = jni::objects::JValueGen::Bool(a.into());
            args.push(val_1);
        }
        if let Some(a) = arg1 {
            sig += "Z";
            let val_2 = jni::objects::JValueGen::Bool(a.into());
            args.push(val_2);
        }
        sig += ")Z";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "update", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    pub fn location(&self) -> Result<crate::Location<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.location()
    }
    pub fn get_type(&self) -> Result<crate::Material<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.get_type()
    }
    // SUPER CLASS: PersistentDataHolder

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::block::TileState<'mc>> for Structure<'mc> {
    fn into(self) -> crate::block::TileState<'mc> {
        crate::block::TileState::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting Structure into crate::block::TileState")
    }
}
/// Represents a captured state of an ender chest.
///
/// This is a representation of an abstract class.
#[repr(C)]
pub struct EnderChest<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for EnderChest<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for EnderChest<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate EnderChest from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/block/EnderChest")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a EnderChest object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> EnderChest<'mc> {
    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::block::Lidded<'mc>> for EnderChest<'mc> {
    fn into(self) -> crate::block::Lidded<'mc> {
        crate::block::Lidded::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting EnderChest into crate::block::Lidded")
    }
}
impl<'mc> Into<crate::block::TileState<'mc>> for EnderChest<'mc> {
    fn into(self) -> crate::block::TileState<'mc> {
        crate::block::TileState::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting EnderChest into crate::block::TileState")
    }
}
/// Represents a captured state of a bed.
///
/// This is a representation of an abstract class.
#[repr(C)]
pub struct Bed<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for Bed<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for Bed<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Bed from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/block/Bed")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Bed object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> Bed<'mc> {
    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::block::TileState<'mc>> for Bed<'mc> {
    fn into(self) -> crate::block::TileState<'mc> {
        crate::block::TileState::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting Bed into crate::block::TileState")
    }
}
impl<'mc> Into<crate::material::Colorable<'mc>> for Bed<'mc> {
    fn into(self) -> crate::material::Colorable<'mc> {
        crate::material::Colorable::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting Bed into crate::material::Colorable")
    }
}
/// Represents a captured state of a jukebox.
///
/// This is a representation of an abstract class.
#[repr(C)]
pub struct Jukebox<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for Jukebox<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for Jukebox<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Jukebox from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/block/Jukebox")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Jukebox object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> Jukebox<'mc> {
    pub fn eject(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "eject", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
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

    pub fn snapshot_inventory(
        &self,
    ) -> Result<crate::inventory::JukeboxInventory<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/inventory/JukeboxInventory;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getSnapshotInventory",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::JukeboxInventory::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn set_record(
        &self,
        arg0: impl Into<crate::inventory::ItemStack<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/inventory/ItemStack;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setRecord",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn record(
        &self,
    ) -> Result<Option<crate::inventory::ItemStack<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/inventory/ItemStack;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getRecord", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {
            return Ok(None);
        }
        Ok(Some(crate::inventory::ItemStack::from_raw(
            &self.jni_ref(),
            unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) },
        )?))
    }

    pub fn playing(&self) -> Result<crate::Material<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/Material;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getPlaying", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::Material::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn set_playing(
        &self,
        arg0: impl Into<crate::Material<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/Material;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setPlaying",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn has_record(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasRecord", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn is_playing(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isPlaying", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn start_playing(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "startPlaying", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn stop_playing(&self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "stopPlaying", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub fn persistent_data_container(
        &self,
    ) -> Result<crate::persistence::PersistentDataContainer<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = Jukebox::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::TileState = temp_clone.into();
        real.persistent_data_container()
    }
    // SUPER CLASS: BlockState
    pub fn set_metadata(
        &self,
        arg0: impl Into<String>,
        arg1: impl Into<crate::metadata::MetadataValue<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::metadata::Metadatable = temp_clone.into();
        real.set_metadata(arg0, arg1)
    }
    pub fn get_metadata(
        &self,
        arg0: impl Into<String>,
    ) -> Result<Vec<crate::metadata::MetadataValue<'mc>>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::metadata::Metadatable = temp_clone.into();
        real.get_metadata(arg0)
    }
    pub fn has_metadata(
        &self,
        arg0: impl Into<String>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::metadata::Metadatable = temp_clone.into();
        real.has_metadata(arg0)
    }
    pub fn remove_metadata(
        &self,
        arg0: impl Into<String>,
        arg1: impl Into<crate::plugin::Plugin<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::metadata::Metadatable = temp_clone.into();
        real.remove_metadata(arg0, arg1)
    }
    pub fn data(&self) -> Result<crate::material::MaterialData<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.data()
    }
    pub fn world(&self) -> Result<crate::World<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.world()
    }
    pub fn set_type(
        &self,
        arg0: impl Into<crate::Material<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.set_type(arg0)
    }
    pub fn block(&self) -> Result<crate::block::Block<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.block()
    }
    pub fn block_data(
        &self,
    ) -> Result<crate::block::data::BlockData<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.block_data()
    }
    pub fn light_level(&self) -> Result<i8, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.light_level()
    }
    pub fn x(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.x()
    }
    pub fn y(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.y()
    }
    pub fn z(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.z()
    }
    pub fn chunk(&self) -> Result<crate::Chunk<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.chunk()
    }
    pub fn set_data(
        &self,
        arg0: impl Into<crate::material::MaterialData<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.set_data(arg0)
    }
    pub fn set_block_data(
        &self,
        arg0: impl Into<crate::block::data::BlockData<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.set_block_data(arg0)
    }

    pub fn raw_data(&self) -> Result<i8, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.raw_data()
    }

    pub fn set_raw_data(&self, arg0: i8) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.set_raw_data(arg0)
    }
    pub fn is_placed(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.is_placed()
    }
    pub fn update_with_boolean(
        &self,
        arg0: std::option::Option<bool>,
        arg1: std::option::Option<bool>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "Z";
            let val_1 = jni::objects::JValueGen::Bool(a.into());
            args.push(val_1);
        }
        if let Some(a) = arg1 {
            sig += "Z";
            let val_2 = jni::objects::JValueGen::Bool(a.into());
            args.push(val_2);
        }
        sig += ")Z";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "update", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    pub fn location(&self) -> Result<crate::Location<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.location()
    }
    pub fn get_type(&self) -> Result<crate::Material<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.get_type()
    }
    // SUPER CLASS: PersistentDataHolder
    // SUPER CLASS: InventoryHolder

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::block::TileState<'mc>> for Jukebox<'mc> {
    fn into(self) -> crate::block::TileState<'mc> {
        crate::block::TileState::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting Jukebox into crate::block::TileState")
    }
}
impl<'mc> Into<crate::inventory::BlockInventoryHolder<'mc>> for Jukebox<'mc> {
    fn into(self) -> crate::inventory::BlockInventoryHolder<'mc> {
        crate::inventory::BlockInventoryHolder::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting Jukebox into crate::inventory::BlockInventoryHolder")
    }
}
pub enum BiomeBiome<'mc> {
    Ocean { inner: BiomeBiomeStruct<'mc> },
    Plains { inner: BiomeBiomeStruct<'mc> },
    Desert { inner: BiomeBiomeStruct<'mc> },
    WindsweptHills { inner: BiomeBiomeStruct<'mc> },
    Forest { inner: BiomeBiomeStruct<'mc> },
    Taiga { inner: BiomeBiomeStruct<'mc> },
    Swamp { inner: BiomeBiomeStruct<'mc> },
    MangroveSwamp { inner: BiomeBiomeStruct<'mc> },
    River { inner: BiomeBiomeStruct<'mc> },
    NetherWastes { inner: BiomeBiomeStruct<'mc> },
    TheEnd { inner: BiomeBiomeStruct<'mc> },
    FrozenOcean { inner: BiomeBiomeStruct<'mc> },
    FrozenRiver { inner: BiomeBiomeStruct<'mc> },
    SnowyPlains { inner: BiomeBiomeStruct<'mc> },
    MushroomFields { inner: BiomeBiomeStruct<'mc> },
    Beach { inner: BiomeBiomeStruct<'mc> },
    Jungle { inner: BiomeBiomeStruct<'mc> },
    SparseJungle { inner: BiomeBiomeStruct<'mc> },
    DeepOcean { inner: BiomeBiomeStruct<'mc> },
    StonyShore { inner: BiomeBiomeStruct<'mc> },
    SnowyBeach { inner: BiomeBiomeStruct<'mc> },
    BirchForest { inner: BiomeBiomeStruct<'mc> },
    DarkForest { inner: BiomeBiomeStruct<'mc> },
    SnowyTaiga { inner: BiomeBiomeStruct<'mc> },
    OldGrowthPineTaiga { inner: BiomeBiomeStruct<'mc> },
    WindsweptForest { inner: BiomeBiomeStruct<'mc> },
    Savanna { inner: BiomeBiomeStruct<'mc> },
    SavannaPlateau { inner: BiomeBiomeStruct<'mc> },
    Badlands { inner: BiomeBiomeStruct<'mc> },
    WoodedBadlands { inner: BiomeBiomeStruct<'mc> },
    SmallEndIslands { inner: BiomeBiomeStruct<'mc> },
    EndMidlands { inner: BiomeBiomeStruct<'mc> },
    EndHighlands { inner: BiomeBiomeStruct<'mc> },
    EndBarrens { inner: BiomeBiomeStruct<'mc> },
    WarmOcean { inner: BiomeBiomeStruct<'mc> },
    LukewarmOcean { inner: BiomeBiomeStruct<'mc> },
    ColdOcean { inner: BiomeBiomeStruct<'mc> },
    DeepLukewarmOcean { inner: BiomeBiomeStruct<'mc> },
    DeepColdOcean { inner: BiomeBiomeStruct<'mc> },
    DeepFrozenOcean { inner: BiomeBiomeStruct<'mc> },
    TheVoid { inner: BiomeBiomeStruct<'mc> },
    SunflowerPlains { inner: BiomeBiomeStruct<'mc> },
    WindsweptGravellyHills { inner: BiomeBiomeStruct<'mc> },
    FlowerForest { inner: BiomeBiomeStruct<'mc> },
    IceSpikes { inner: BiomeBiomeStruct<'mc> },
    OldGrowthBirchForest { inner: BiomeBiomeStruct<'mc> },
    OldGrowthSpruceTaiga { inner: BiomeBiomeStruct<'mc> },
    WindsweptSavanna { inner: BiomeBiomeStruct<'mc> },
    ErodedBadlands { inner: BiomeBiomeStruct<'mc> },
    BambooJungle { inner: BiomeBiomeStruct<'mc> },
    SoulSandValley { inner: BiomeBiomeStruct<'mc> },
    CrimsonForest { inner: BiomeBiomeStruct<'mc> },
    WarpedForest { inner: BiomeBiomeStruct<'mc> },
    BasaltDeltas { inner: BiomeBiomeStruct<'mc> },
    DripstoneCaves { inner: BiomeBiomeStruct<'mc> },
    LushCaves { inner: BiomeBiomeStruct<'mc> },
    DeepDark { inner: BiomeBiomeStruct<'mc> },
    Meadow { inner: BiomeBiomeStruct<'mc> },
    Grove { inner: BiomeBiomeStruct<'mc> },
    SnowySlopes { inner: BiomeBiomeStruct<'mc> },
    FrozenPeaks { inner: BiomeBiomeStruct<'mc> },
    JaggedPeaks { inner: BiomeBiomeStruct<'mc> },
    StonyPeaks { inner: BiomeBiomeStruct<'mc> },
    CherryGrove { inner: BiomeBiomeStruct<'mc> },
    Custom { inner: BiomeBiomeStruct<'mc> },
}
impl<'mc> std::fmt::Display for BiomeBiome<'mc> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BiomeBiome::Ocean { .. } => f.write_str("OCEAN"),
            BiomeBiome::Plains { .. } => f.write_str("PLAINS"),
            BiomeBiome::Desert { .. } => f.write_str("DESERT"),
            BiomeBiome::WindsweptHills { .. } => f.write_str("WINDSWEPT_HILLS"),
            BiomeBiome::Forest { .. } => f.write_str("FOREST"),
            BiomeBiome::Taiga { .. } => f.write_str("TAIGA"),
            BiomeBiome::Swamp { .. } => f.write_str("SWAMP"),
            BiomeBiome::MangroveSwamp { .. } => f.write_str("MANGROVE_SWAMP"),
            BiomeBiome::River { .. } => f.write_str("RIVER"),
            BiomeBiome::NetherWastes { .. } => f.write_str("NETHER_WASTES"),
            BiomeBiome::TheEnd { .. } => f.write_str("THE_END"),
            BiomeBiome::FrozenOcean { .. } => f.write_str("FROZEN_OCEAN"),
            BiomeBiome::FrozenRiver { .. } => f.write_str("FROZEN_RIVER"),
            BiomeBiome::SnowyPlains { .. } => f.write_str("SNOWY_PLAINS"),
            BiomeBiome::MushroomFields { .. } => f.write_str("MUSHROOM_FIELDS"),
            BiomeBiome::Beach { .. } => f.write_str("BEACH"),
            BiomeBiome::Jungle { .. } => f.write_str("JUNGLE"),
            BiomeBiome::SparseJungle { .. } => f.write_str("SPARSE_JUNGLE"),
            BiomeBiome::DeepOcean { .. } => f.write_str("DEEP_OCEAN"),
            BiomeBiome::StonyShore { .. } => f.write_str("STONY_SHORE"),
            BiomeBiome::SnowyBeach { .. } => f.write_str("SNOWY_BEACH"),
            BiomeBiome::BirchForest { .. } => f.write_str("BIRCH_FOREST"),
            BiomeBiome::DarkForest { .. } => f.write_str("DARK_FOREST"),
            BiomeBiome::SnowyTaiga { .. } => f.write_str("SNOWY_TAIGA"),
            BiomeBiome::OldGrowthPineTaiga { .. } => f.write_str("OLD_GROWTH_PINE_TAIGA"),
            BiomeBiome::WindsweptForest { .. } => f.write_str("WINDSWEPT_FOREST"),
            BiomeBiome::Savanna { .. } => f.write_str("SAVANNA"),
            BiomeBiome::SavannaPlateau { .. } => f.write_str("SAVANNA_PLATEAU"),
            BiomeBiome::Badlands { .. } => f.write_str("BADLANDS"),
            BiomeBiome::WoodedBadlands { .. } => f.write_str("WOODED_BADLANDS"),
            BiomeBiome::SmallEndIslands { .. } => f.write_str("SMALL_END_ISLANDS"),
            BiomeBiome::EndMidlands { .. } => f.write_str("END_MIDLANDS"),
            BiomeBiome::EndHighlands { .. } => f.write_str("END_HIGHLANDS"),
            BiomeBiome::EndBarrens { .. } => f.write_str("END_BARRENS"),
            BiomeBiome::WarmOcean { .. } => f.write_str("WARM_OCEAN"),
            BiomeBiome::LukewarmOcean { .. } => f.write_str("LUKEWARM_OCEAN"),
            BiomeBiome::ColdOcean { .. } => f.write_str("COLD_OCEAN"),
            BiomeBiome::DeepLukewarmOcean { .. } => f.write_str("DEEP_LUKEWARM_OCEAN"),
            BiomeBiome::DeepColdOcean { .. } => f.write_str("DEEP_COLD_OCEAN"),
            BiomeBiome::DeepFrozenOcean { .. } => f.write_str("DEEP_FROZEN_OCEAN"),
            BiomeBiome::TheVoid { .. } => f.write_str("THE_VOID"),
            BiomeBiome::SunflowerPlains { .. } => f.write_str("SUNFLOWER_PLAINS"),
            BiomeBiome::WindsweptGravellyHills { .. } => f.write_str("WINDSWEPT_GRAVELLY_HILLS"),
            BiomeBiome::FlowerForest { .. } => f.write_str("FLOWER_FOREST"),
            BiomeBiome::IceSpikes { .. } => f.write_str("ICE_SPIKES"),
            BiomeBiome::OldGrowthBirchForest { .. } => f.write_str("OLD_GROWTH_BIRCH_FOREST"),
            BiomeBiome::OldGrowthSpruceTaiga { .. } => f.write_str("OLD_GROWTH_SPRUCE_TAIGA"),
            BiomeBiome::WindsweptSavanna { .. } => f.write_str("WINDSWEPT_SAVANNA"),
            BiomeBiome::ErodedBadlands { .. } => f.write_str("ERODED_BADLANDS"),
            BiomeBiome::BambooJungle { .. } => f.write_str("BAMBOO_JUNGLE"),
            BiomeBiome::SoulSandValley { .. } => f.write_str("SOUL_SAND_VALLEY"),
            BiomeBiome::CrimsonForest { .. } => f.write_str("CRIMSON_FOREST"),
            BiomeBiome::WarpedForest { .. } => f.write_str("WARPED_FOREST"),
            BiomeBiome::BasaltDeltas { .. } => f.write_str("BASALT_DELTAS"),
            BiomeBiome::DripstoneCaves { .. } => f.write_str("DRIPSTONE_CAVES"),
            BiomeBiome::LushCaves { .. } => f.write_str("LUSH_CAVES"),
            BiomeBiome::DeepDark { .. } => f.write_str("DEEP_DARK"),
            BiomeBiome::Meadow { .. } => f.write_str("MEADOW"),
            BiomeBiome::Grove { .. } => f.write_str("GROVE"),
            BiomeBiome::SnowySlopes { .. } => f.write_str("SNOWY_SLOPES"),
            BiomeBiome::FrozenPeaks { .. } => f.write_str("FROZEN_PEAKS"),
            BiomeBiome::JaggedPeaks { .. } => f.write_str("JAGGED_PEAKS"),
            BiomeBiome::StonyPeaks { .. } => f.write_str("STONY_PEAKS"),
            BiomeBiome::CherryGrove { .. } => f.write_str("CHERRY_GROVE"),
            BiomeBiome::Custom { .. } => f.write_str("CUSTOM"),
        }
    }
}

impl<'mc> BiomeBiome<'mc> {
    pub fn value_of(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<String>,
    ) -> Result<BiomeBiome<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(env.new_string(arg0.into())?);
        let cls = env.find_class("org/bukkit/block/Biome$Biome");
        let cls = env.translate_error_with_class(cls)?;
        let res = env.call_static_method(
            cls,
            "valueOf",
            "(Ljava/lang/String;)Lorg/bukkit/block/Biome$Biome;",
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
            "OCEAN" => Ok(BiomeBiome::Ocean {
                inner: BiomeBiomeStruct::from_raw(env, obj)?,
            }),
            "PLAINS" => Ok(BiomeBiome::Plains {
                inner: BiomeBiomeStruct::from_raw(env, obj)?,
            }),
            "DESERT" => Ok(BiomeBiome::Desert {
                inner: BiomeBiomeStruct::from_raw(env, obj)?,
            }),
            "WINDSWEPT_HILLS" => Ok(BiomeBiome::WindsweptHills {
                inner: BiomeBiomeStruct::from_raw(env, obj)?,
            }),
            "FOREST" => Ok(BiomeBiome::Forest {
                inner: BiomeBiomeStruct::from_raw(env, obj)?,
            }),
            "TAIGA" => Ok(BiomeBiome::Taiga {
                inner: BiomeBiomeStruct::from_raw(env, obj)?,
            }),
            "SWAMP" => Ok(BiomeBiome::Swamp {
                inner: BiomeBiomeStruct::from_raw(env, obj)?,
            }),
            "MANGROVE_SWAMP" => Ok(BiomeBiome::MangroveSwamp {
                inner: BiomeBiomeStruct::from_raw(env, obj)?,
            }),
            "RIVER" => Ok(BiomeBiome::River {
                inner: BiomeBiomeStruct::from_raw(env, obj)?,
            }),
            "NETHER_WASTES" => Ok(BiomeBiome::NetherWastes {
                inner: BiomeBiomeStruct::from_raw(env, obj)?,
            }),
            "THE_END" => Ok(BiomeBiome::TheEnd {
                inner: BiomeBiomeStruct::from_raw(env, obj)?,
            }),
            "FROZEN_OCEAN" => Ok(BiomeBiome::FrozenOcean {
                inner: BiomeBiomeStruct::from_raw(env, obj)?,
            }),
            "FROZEN_RIVER" => Ok(BiomeBiome::FrozenRiver {
                inner: BiomeBiomeStruct::from_raw(env, obj)?,
            }),
            "SNOWY_PLAINS" => Ok(BiomeBiome::SnowyPlains {
                inner: BiomeBiomeStruct::from_raw(env, obj)?,
            }),
            "MUSHROOM_FIELDS" => Ok(BiomeBiome::MushroomFields {
                inner: BiomeBiomeStruct::from_raw(env, obj)?,
            }),
            "BEACH" => Ok(BiomeBiome::Beach {
                inner: BiomeBiomeStruct::from_raw(env, obj)?,
            }),
            "JUNGLE" => Ok(BiomeBiome::Jungle {
                inner: BiomeBiomeStruct::from_raw(env, obj)?,
            }),
            "SPARSE_JUNGLE" => Ok(BiomeBiome::SparseJungle {
                inner: BiomeBiomeStruct::from_raw(env, obj)?,
            }),
            "DEEP_OCEAN" => Ok(BiomeBiome::DeepOcean {
                inner: BiomeBiomeStruct::from_raw(env, obj)?,
            }),
            "STONY_SHORE" => Ok(BiomeBiome::StonyShore {
                inner: BiomeBiomeStruct::from_raw(env, obj)?,
            }),
            "SNOWY_BEACH" => Ok(BiomeBiome::SnowyBeach {
                inner: BiomeBiomeStruct::from_raw(env, obj)?,
            }),
            "BIRCH_FOREST" => Ok(BiomeBiome::BirchForest {
                inner: BiomeBiomeStruct::from_raw(env, obj)?,
            }),
            "DARK_FOREST" => Ok(BiomeBiome::DarkForest {
                inner: BiomeBiomeStruct::from_raw(env, obj)?,
            }),
            "SNOWY_TAIGA" => Ok(BiomeBiome::SnowyTaiga {
                inner: BiomeBiomeStruct::from_raw(env, obj)?,
            }),
            "OLD_GROWTH_PINE_TAIGA" => Ok(BiomeBiome::OldGrowthPineTaiga {
                inner: BiomeBiomeStruct::from_raw(env, obj)?,
            }),
            "WINDSWEPT_FOREST" => Ok(BiomeBiome::WindsweptForest {
                inner: BiomeBiomeStruct::from_raw(env, obj)?,
            }),
            "SAVANNA" => Ok(BiomeBiome::Savanna {
                inner: BiomeBiomeStruct::from_raw(env, obj)?,
            }),
            "SAVANNA_PLATEAU" => Ok(BiomeBiome::SavannaPlateau {
                inner: BiomeBiomeStruct::from_raw(env, obj)?,
            }),
            "BADLANDS" => Ok(BiomeBiome::Badlands {
                inner: BiomeBiomeStruct::from_raw(env, obj)?,
            }),
            "WOODED_BADLANDS" => Ok(BiomeBiome::WoodedBadlands {
                inner: BiomeBiomeStruct::from_raw(env, obj)?,
            }),
            "SMALL_END_ISLANDS" => Ok(BiomeBiome::SmallEndIslands {
                inner: BiomeBiomeStruct::from_raw(env, obj)?,
            }),
            "END_MIDLANDS" => Ok(BiomeBiome::EndMidlands {
                inner: BiomeBiomeStruct::from_raw(env, obj)?,
            }),
            "END_HIGHLANDS" => Ok(BiomeBiome::EndHighlands {
                inner: BiomeBiomeStruct::from_raw(env, obj)?,
            }),
            "END_BARRENS" => Ok(BiomeBiome::EndBarrens {
                inner: BiomeBiomeStruct::from_raw(env, obj)?,
            }),
            "WARM_OCEAN" => Ok(BiomeBiome::WarmOcean {
                inner: BiomeBiomeStruct::from_raw(env, obj)?,
            }),
            "LUKEWARM_OCEAN" => Ok(BiomeBiome::LukewarmOcean {
                inner: BiomeBiomeStruct::from_raw(env, obj)?,
            }),
            "COLD_OCEAN" => Ok(BiomeBiome::ColdOcean {
                inner: BiomeBiomeStruct::from_raw(env, obj)?,
            }),
            "DEEP_LUKEWARM_OCEAN" => Ok(BiomeBiome::DeepLukewarmOcean {
                inner: BiomeBiomeStruct::from_raw(env, obj)?,
            }),
            "DEEP_COLD_OCEAN" => Ok(BiomeBiome::DeepColdOcean {
                inner: BiomeBiomeStruct::from_raw(env, obj)?,
            }),
            "DEEP_FROZEN_OCEAN" => Ok(BiomeBiome::DeepFrozenOcean {
                inner: BiomeBiomeStruct::from_raw(env, obj)?,
            }),
            "THE_VOID" => Ok(BiomeBiome::TheVoid {
                inner: BiomeBiomeStruct::from_raw(env, obj)?,
            }),
            "SUNFLOWER_PLAINS" => Ok(BiomeBiome::SunflowerPlains {
                inner: BiomeBiomeStruct::from_raw(env, obj)?,
            }),
            "WINDSWEPT_GRAVELLY_HILLS" => Ok(BiomeBiome::WindsweptGravellyHills {
                inner: BiomeBiomeStruct::from_raw(env, obj)?,
            }),
            "FLOWER_FOREST" => Ok(BiomeBiome::FlowerForest {
                inner: BiomeBiomeStruct::from_raw(env, obj)?,
            }),
            "ICE_SPIKES" => Ok(BiomeBiome::IceSpikes {
                inner: BiomeBiomeStruct::from_raw(env, obj)?,
            }),
            "OLD_GROWTH_BIRCH_FOREST" => Ok(BiomeBiome::OldGrowthBirchForest {
                inner: BiomeBiomeStruct::from_raw(env, obj)?,
            }),
            "OLD_GROWTH_SPRUCE_TAIGA" => Ok(BiomeBiome::OldGrowthSpruceTaiga {
                inner: BiomeBiomeStruct::from_raw(env, obj)?,
            }),
            "WINDSWEPT_SAVANNA" => Ok(BiomeBiome::WindsweptSavanna {
                inner: BiomeBiomeStruct::from_raw(env, obj)?,
            }),
            "ERODED_BADLANDS" => Ok(BiomeBiome::ErodedBadlands {
                inner: BiomeBiomeStruct::from_raw(env, obj)?,
            }),
            "BAMBOO_JUNGLE" => Ok(BiomeBiome::BambooJungle {
                inner: BiomeBiomeStruct::from_raw(env, obj)?,
            }),
            "SOUL_SAND_VALLEY" => Ok(BiomeBiome::SoulSandValley {
                inner: BiomeBiomeStruct::from_raw(env, obj)?,
            }),
            "CRIMSON_FOREST" => Ok(BiomeBiome::CrimsonForest {
                inner: BiomeBiomeStruct::from_raw(env, obj)?,
            }),
            "WARPED_FOREST" => Ok(BiomeBiome::WarpedForest {
                inner: BiomeBiomeStruct::from_raw(env, obj)?,
            }),
            "BASALT_DELTAS" => Ok(BiomeBiome::BasaltDeltas {
                inner: BiomeBiomeStruct::from_raw(env, obj)?,
            }),
            "DRIPSTONE_CAVES" => Ok(BiomeBiome::DripstoneCaves {
                inner: BiomeBiomeStruct::from_raw(env, obj)?,
            }),
            "LUSH_CAVES" => Ok(BiomeBiome::LushCaves {
                inner: BiomeBiomeStruct::from_raw(env, obj)?,
            }),
            "DEEP_DARK" => Ok(BiomeBiome::DeepDark {
                inner: BiomeBiomeStruct::from_raw(env, obj)?,
            }),
            "MEADOW" => Ok(BiomeBiome::Meadow {
                inner: BiomeBiomeStruct::from_raw(env, obj)?,
            }),
            "GROVE" => Ok(BiomeBiome::Grove {
                inner: BiomeBiomeStruct::from_raw(env, obj)?,
            }),
            "SNOWY_SLOPES" => Ok(BiomeBiome::SnowySlopes {
                inner: BiomeBiomeStruct::from_raw(env, obj)?,
            }),
            "FROZEN_PEAKS" => Ok(BiomeBiome::FrozenPeaks {
                inner: BiomeBiomeStruct::from_raw(env, obj)?,
            }),
            "JAGGED_PEAKS" => Ok(BiomeBiome::JaggedPeaks {
                inner: BiomeBiomeStruct::from_raw(env, obj)?,
            }),
            "STONY_PEAKS" => Ok(BiomeBiome::StonyPeaks {
                inner: BiomeBiomeStruct::from_raw(env, obj)?,
            }),
            "CHERRY_GROVE" => Ok(BiomeBiome::CherryGrove {
                inner: BiomeBiomeStruct::from_raw(env, obj)?,
            }),
            "CUSTOM" => Ok(BiomeBiome::Custom {
                inner: BiomeBiomeStruct::from_raw(env, obj)?,
            }),

            _ => Err(eyre::eyre!("String gaven for variant was invalid").into()),
        }
    }
}

#[repr(C)]
pub struct BiomeBiomeStruct<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for BiomeBiome<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        match self {
            Self::Ocean { inner } => inner.0.clone(),
            Self::Plains { inner } => inner.0.clone(),
            Self::Desert { inner } => inner.0.clone(),
            Self::WindsweptHills { inner } => inner.0.clone(),
            Self::Forest { inner } => inner.0.clone(),
            Self::Taiga { inner } => inner.0.clone(),
            Self::Swamp { inner } => inner.0.clone(),
            Self::MangroveSwamp { inner } => inner.0.clone(),
            Self::River { inner } => inner.0.clone(),
            Self::NetherWastes { inner } => inner.0.clone(),
            Self::TheEnd { inner } => inner.0.clone(),
            Self::FrozenOcean { inner } => inner.0.clone(),
            Self::FrozenRiver { inner } => inner.0.clone(),
            Self::SnowyPlains { inner } => inner.0.clone(),
            Self::MushroomFields { inner } => inner.0.clone(),
            Self::Beach { inner } => inner.0.clone(),
            Self::Jungle { inner } => inner.0.clone(),
            Self::SparseJungle { inner } => inner.0.clone(),
            Self::DeepOcean { inner } => inner.0.clone(),
            Self::StonyShore { inner } => inner.0.clone(),
            Self::SnowyBeach { inner } => inner.0.clone(),
            Self::BirchForest { inner } => inner.0.clone(),
            Self::DarkForest { inner } => inner.0.clone(),
            Self::SnowyTaiga { inner } => inner.0.clone(),
            Self::OldGrowthPineTaiga { inner } => inner.0.clone(),
            Self::WindsweptForest { inner } => inner.0.clone(),
            Self::Savanna { inner } => inner.0.clone(),
            Self::SavannaPlateau { inner } => inner.0.clone(),
            Self::Badlands { inner } => inner.0.clone(),
            Self::WoodedBadlands { inner } => inner.0.clone(),
            Self::SmallEndIslands { inner } => inner.0.clone(),
            Self::EndMidlands { inner } => inner.0.clone(),
            Self::EndHighlands { inner } => inner.0.clone(),
            Self::EndBarrens { inner } => inner.0.clone(),
            Self::WarmOcean { inner } => inner.0.clone(),
            Self::LukewarmOcean { inner } => inner.0.clone(),
            Self::ColdOcean { inner } => inner.0.clone(),
            Self::DeepLukewarmOcean { inner } => inner.0.clone(),
            Self::DeepColdOcean { inner } => inner.0.clone(),
            Self::DeepFrozenOcean { inner } => inner.0.clone(),
            Self::TheVoid { inner } => inner.0.clone(),
            Self::SunflowerPlains { inner } => inner.0.clone(),
            Self::WindsweptGravellyHills { inner } => inner.0.clone(),
            Self::FlowerForest { inner } => inner.0.clone(),
            Self::IceSpikes { inner } => inner.0.clone(),
            Self::OldGrowthBirchForest { inner } => inner.0.clone(),
            Self::OldGrowthSpruceTaiga { inner } => inner.0.clone(),
            Self::WindsweptSavanna { inner } => inner.0.clone(),
            Self::ErodedBadlands { inner } => inner.0.clone(),
            Self::BambooJungle { inner } => inner.0.clone(),
            Self::SoulSandValley { inner } => inner.0.clone(),
            Self::CrimsonForest { inner } => inner.0.clone(),
            Self::WarpedForest { inner } => inner.0.clone(),
            Self::BasaltDeltas { inner } => inner.0.clone(),
            Self::DripstoneCaves { inner } => inner.0.clone(),
            Self::LushCaves { inner } => inner.0.clone(),
            Self::DeepDark { inner } => inner.0.clone(),
            Self::Meadow { inner } => inner.0.clone(),
            Self::Grove { inner } => inner.0.clone(),
            Self::SnowySlopes { inner } => inner.0.clone(),
            Self::FrozenPeaks { inner } => inner.0.clone(),
            Self::JaggedPeaks { inner } => inner.0.clone(),
            Self::StonyPeaks { inner } => inner.0.clone(),
            Self::CherryGrove { inner } => inner.0.clone(),
            Self::Custom { inner } => inner.0.clone(),
        }
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        match self {
            Self::Ocean { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Plains { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Desert { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::WindsweptHills { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::Forest { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Taiga { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Swamp { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::MangroveSwamp { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::River { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::NetherWastes { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::TheEnd { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::FrozenOcean { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::FrozenRiver { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::SnowyPlains { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::MushroomFields { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::Beach { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Jungle { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::SparseJungle { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::DeepOcean { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::StonyShore { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::SnowyBeach { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::BirchForest { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::DarkForest { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::SnowyTaiga { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::OldGrowthPineTaiga { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::WindsweptForest { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::Savanna { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::SavannaPlateau { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::Badlands { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::WoodedBadlands { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::SmallEndIslands { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::EndMidlands { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::EndHighlands { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::EndBarrens { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::WarmOcean { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::LukewarmOcean { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::ColdOcean { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::DeepLukewarmOcean { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::DeepColdOcean { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::DeepFrozenOcean { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::TheVoid { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::SunflowerPlains { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::WindsweptGravellyHills { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::FlowerForest { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::IceSpikes { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::OldGrowthBirchForest { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::OldGrowthSpruceTaiga { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::WindsweptSavanna { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::ErodedBadlands { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::BambooJungle { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::SoulSandValley { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::CrimsonForest { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::WarpedForest { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::BasaltDeltas { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::DripstoneCaves { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::LushCaves { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::DeepDark { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Meadow { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Grove { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::SnowySlopes { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::FrozenPeaks { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::JaggedPeaks { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::StonyPeaks { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::CherryGrove { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::Custom { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
        }
    }
}
impl<'mc> JNIInstantiatable<'mc> for BiomeBiome<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate BiomeBiome from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/block/Biome$Biome")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a BiomeBiome object, got {}",
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
                "OCEAN" => Ok(BiomeBiome::Ocean {
                    inner: BiomeBiomeStruct::from_raw(env, obj)?,
                }),
                "PLAINS" => Ok(BiomeBiome::Plains {
                    inner: BiomeBiomeStruct::from_raw(env, obj)?,
                }),
                "DESERT" => Ok(BiomeBiome::Desert {
                    inner: BiomeBiomeStruct::from_raw(env, obj)?,
                }),
                "WINDSWEPT_HILLS" => Ok(BiomeBiome::WindsweptHills {
                    inner: BiomeBiomeStruct::from_raw(env, obj)?,
                }),
                "FOREST" => Ok(BiomeBiome::Forest {
                    inner: BiomeBiomeStruct::from_raw(env, obj)?,
                }),
                "TAIGA" => Ok(BiomeBiome::Taiga {
                    inner: BiomeBiomeStruct::from_raw(env, obj)?,
                }),
                "SWAMP" => Ok(BiomeBiome::Swamp {
                    inner: BiomeBiomeStruct::from_raw(env, obj)?,
                }),
                "MANGROVE_SWAMP" => Ok(BiomeBiome::MangroveSwamp {
                    inner: BiomeBiomeStruct::from_raw(env, obj)?,
                }),
                "RIVER" => Ok(BiomeBiome::River {
                    inner: BiomeBiomeStruct::from_raw(env, obj)?,
                }),
                "NETHER_WASTES" => Ok(BiomeBiome::NetherWastes {
                    inner: BiomeBiomeStruct::from_raw(env, obj)?,
                }),
                "THE_END" => Ok(BiomeBiome::TheEnd {
                    inner: BiomeBiomeStruct::from_raw(env, obj)?,
                }),
                "FROZEN_OCEAN" => Ok(BiomeBiome::FrozenOcean {
                    inner: BiomeBiomeStruct::from_raw(env, obj)?,
                }),
                "FROZEN_RIVER" => Ok(BiomeBiome::FrozenRiver {
                    inner: BiomeBiomeStruct::from_raw(env, obj)?,
                }),
                "SNOWY_PLAINS" => Ok(BiomeBiome::SnowyPlains {
                    inner: BiomeBiomeStruct::from_raw(env, obj)?,
                }),
                "MUSHROOM_FIELDS" => Ok(BiomeBiome::MushroomFields {
                    inner: BiomeBiomeStruct::from_raw(env, obj)?,
                }),
                "BEACH" => Ok(BiomeBiome::Beach {
                    inner: BiomeBiomeStruct::from_raw(env, obj)?,
                }),
                "JUNGLE" => Ok(BiomeBiome::Jungle {
                    inner: BiomeBiomeStruct::from_raw(env, obj)?,
                }),
                "SPARSE_JUNGLE" => Ok(BiomeBiome::SparseJungle {
                    inner: BiomeBiomeStruct::from_raw(env, obj)?,
                }),
                "DEEP_OCEAN" => Ok(BiomeBiome::DeepOcean {
                    inner: BiomeBiomeStruct::from_raw(env, obj)?,
                }),
                "STONY_SHORE" => Ok(BiomeBiome::StonyShore {
                    inner: BiomeBiomeStruct::from_raw(env, obj)?,
                }),
                "SNOWY_BEACH" => Ok(BiomeBiome::SnowyBeach {
                    inner: BiomeBiomeStruct::from_raw(env, obj)?,
                }),
                "BIRCH_FOREST" => Ok(BiomeBiome::BirchForest {
                    inner: BiomeBiomeStruct::from_raw(env, obj)?,
                }),
                "DARK_FOREST" => Ok(BiomeBiome::DarkForest {
                    inner: BiomeBiomeStruct::from_raw(env, obj)?,
                }),
                "SNOWY_TAIGA" => Ok(BiomeBiome::SnowyTaiga {
                    inner: BiomeBiomeStruct::from_raw(env, obj)?,
                }),
                "OLD_GROWTH_PINE_TAIGA" => Ok(BiomeBiome::OldGrowthPineTaiga {
                    inner: BiomeBiomeStruct::from_raw(env, obj)?,
                }),
                "WINDSWEPT_FOREST" => Ok(BiomeBiome::WindsweptForest {
                    inner: BiomeBiomeStruct::from_raw(env, obj)?,
                }),
                "SAVANNA" => Ok(BiomeBiome::Savanna {
                    inner: BiomeBiomeStruct::from_raw(env, obj)?,
                }),
                "SAVANNA_PLATEAU" => Ok(BiomeBiome::SavannaPlateau {
                    inner: BiomeBiomeStruct::from_raw(env, obj)?,
                }),
                "BADLANDS" => Ok(BiomeBiome::Badlands {
                    inner: BiomeBiomeStruct::from_raw(env, obj)?,
                }),
                "WOODED_BADLANDS" => Ok(BiomeBiome::WoodedBadlands {
                    inner: BiomeBiomeStruct::from_raw(env, obj)?,
                }),
                "SMALL_END_ISLANDS" => Ok(BiomeBiome::SmallEndIslands {
                    inner: BiomeBiomeStruct::from_raw(env, obj)?,
                }),
                "END_MIDLANDS" => Ok(BiomeBiome::EndMidlands {
                    inner: BiomeBiomeStruct::from_raw(env, obj)?,
                }),
                "END_HIGHLANDS" => Ok(BiomeBiome::EndHighlands {
                    inner: BiomeBiomeStruct::from_raw(env, obj)?,
                }),
                "END_BARRENS" => Ok(BiomeBiome::EndBarrens {
                    inner: BiomeBiomeStruct::from_raw(env, obj)?,
                }),
                "WARM_OCEAN" => Ok(BiomeBiome::WarmOcean {
                    inner: BiomeBiomeStruct::from_raw(env, obj)?,
                }),
                "LUKEWARM_OCEAN" => Ok(BiomeBiome::LukewarmOcean {
                    inner: BiomeBiomeStruct::from_raw(env, obj)?,
                }),
                "COLD_OCEAN" => Ok(BiomeBiome::ColdOcean {
                    inner: BiomeBiomeStruct::from_raw(env, obj)?,
                }),
                "DEEP_LUKEWARM_OCEAN" => Ok(BiomeBiome::DeepLukewarmOcean {
                    inner: BiomeBiomeStruct::from_raw(env, obj)?,
                }),
                "DEEP_COLD_OCEAN" => Ok(BiomeBiome::DeepColdOcean {
                    inner: BiomeBiomeStruct::from_raw(env, obj)?,
                }),
                "DEEP_FROZEN_OCEAN" => Ok(BiomeBiome::DeepFrozenOcean {
                    inner: BiomeBiomeStruct::from_raw(env, obj)?,
                }),
                "THE_VOID" => Ok(BiomeBiome::TheVoid {
                    inner: BiomeBiomeStruct::from_raw(env, obj)?,
                }),
                "SUNFLOWER_PLAINS" => Ok(BiomeBiome::SunflowerPlains {
                    inner: BiomeBiomeStruct::from_raw(env, obj)?,
                }),
                "WINDSWEPT_GRAVELLY_HILLS" => Ok(BiomeBiome::WindsweptGravellyHills {
                    inner: BiomeBiomeStruct::from_raw(env, obj)?,
                }),
                "FLOWER_FOREST" => Ok(BiomeBiome::FlowerForest {
                    inner: BiomeBiomeStruct::from_raw(env, obj)?,
                }),
                "ICE_SPIKES" => Ok(BiomeBiome::IceSpikes {
                    inner: BiomeBiomeStruct::from_raw(env, obj)?,
                }),
                "OLD_GROWTH_BIRCH_FOREST" => Ok(BiomeBiome::OldGrowthBirchForest {
                    inner: BiomeBiomeStruct::from_raw(env, obj)?,
                }),
                "OLD_GROWTH_SPRUCE_TAIGA" => Ok(BiomeBiome::OldGrowthSpruceTaiga {
                    inner: BiomeBiomeStruct::from_raw(env, obj)?,
                }),
                "WINDSWEPT_SAVANNA" => Ok(BiomeBiome::WindsweptSavanna {
                    inner: BiomeBiomeStruct::from_raw(env, obj)?,
                }),
                "ERODED_BADLANDS" => Ok(BiomeBiome::ErodedBadlands {
                    inner: BiomeBiomeStruct::from_raw(env, obj)?,
                }),
                "BAMBOO_JUNGLE" => Ok(BiomeBiome::BambooJungle {
                    inner: BiomeBiomeStruct::from_raw(env, obj)?,
                }),
                "SOUL_SAND_VALLEY" => Ok(BiomeBiome::SoulSandValley {
                    inner: BiomeBiomeStruct::from_raw(env, obj)?,
                }),
                "CRIMSON_FOREST" => Ok(BiomeBiome::CrimsonForest {
                    inner: BiomeBiomeStruct::from_raw(env, obj)?,
                }),
                "WARPED_FOREST" => Ok(BiomeBiome::WarpedForest {
                    inner: BiomeBiomeStruct::from_raw(env, obj)?,
                }),
                "BASALT_DELTAS" => Ok(BiomeBiome::BasaltDeltas {
                    inner: BiomeBiomeStruct::from_raw(env, obj)?,
                }),
                "DRIPSTONE_CAVES" => Ok(BiomeBiome::DripstoneCaves {
                    inner: BiomeBiomeStruct::from_raw(env, obj)?,
                }),
                "LUSH_CAVES" => Ok(BiomeBiome::LushCaves {
                    inner: BiomeBiomeStruct::from_raw(env, obj)?,
                }),
                "DEEP_DARK" => Ok(BiomeBiome::DeepDark {
                    inner: BiomeBiomeStruct::from_raw(env, obj)?,
                }),
                "MEADOW" => Ok(BiomeBiome::Meadow {
                    inner: BiomeBiomeStruct::from_raw(env, obj)?,
                }),
                "GROVE" => Ok(BiomeBiome::Grove {
                    inner: BiomeBiomeStruct::from_raw(env, obj)?,
                }),
                "SNOWY_SLOPES" => Ok(BiomeBiome::SnowySlopes {
                    inner: BiomeBiomeStruct::from_raw(env, obj)?,
                }),
                "FROZEN_PEAKS" => Ok(BiomeBiome::FrozenPeaks {
                    inner: BiomeBiomeStruct::from_raw(env, obj)?,
                }),
                "JAGGED_PEAKS" => Ok(BiomeBiome::JaggedPeaks {
                    inner: BiomeBiomeStruct::from_raw(env, obj)?,
                }),
                "STONY_PEAKS" => Ok(BiomeBiome::StonyPeaks {
                    inner: BiomeBiomeStruct::from_raw(env, obj)?,
                }),
                "CHERRY_GROVE" => Ok(BiomeBiome::CherryGrove {
                    inner: BiomeBiomeStruct::from_raw(env, obj)?,
                }),
                "CUSTOM" => Ok(BiomeBiome::Custom {
                    inner: BiomeBiomeStruct::from_raw(env, obj)?,
                }),
                _ => Err(eyre::eyre!("String gaven for variant was invalid").into()),
            }
        }
    }
}

impl<'mc> JNIRaw<'mc> for BiomeBiomeStruct<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for BiomeBiomeStruct<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate BiomeBiomeStruct from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/block/Biome$Biome")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a BiomeBiomeStruct object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> BiomeBiomeStruct<'mc> {
    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
/// Represents a captured state of a banner.
///
/// This is a representation of an abstract class.
#[repr(C)]
pub struct Banner<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for Banner<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for Banner<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Banner from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/block/Banner")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Banner object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> Banner<'mc> {
    #[deprecated]

    pub fn base_color(&self) -> Result<Option<crate::DyeColor<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/DyeColor;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getBaseColor", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {
            return Ok(None);
        }
        Ok(Some(crate::DyeColor::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })?))
    }

    pub fn set_base_color(
        &self,
        arg0: impl Into<crate::DyeColor<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/DyeColor;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setBaseColor",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn patterns(
        &self,
    ) -> Result<Vec<crate::block::banner::Pattern<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/List;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getPatterns", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let list = blackboxmc_java::util::JavaList::from_raw(&self.jni_ref(), res.l()?)?;
        let iter = list.iterator()?;
        while iter.has_next()? {
            let obj = iter.next()?;
            new_vec.push(crate::block::banner::Pattern::from_raw(
                &self.jni_ref(),
                obj,
            )?);
        }
        Ok(new_vec)
    }

    pub fn set_patterns(
        &self,
        arg0: Vec<impl Into<crate::block::banner::Pattern<'mc>>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/util/List;)V");
        let raw_val_1 = self
            .jni_ref()
            .new_object("java/util/ArrayList", "()V", vec![])?;
        for v in arg0 {
            let map_val_0 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(v.into().jni_object().clone())
            });
            self.jni_ref().call_method(
                &raw_val_1,
                "add",
                "(Ljava/lang/Object;)Z",
                vec![jni::objects::JValueGen::from(map_val_0)],
            )?;
        }
        let val_1 = jni::objects::JValueGen::Object(raw_val_1);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setPatterns",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn add_pattern(
        &self,
        arg0: impl Into<crate::block::banner::Pattern<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/banner/Pattern;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "addPattern",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Returns the pattern at the specified index
    pub fn get_pattern(
        &self,
        arg0: i32,
    ) -> Result<crate::block::banner::Pattern<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(I)Lorg/bukkit/block/banner/Pattern;");
        let val_1 = jni::objects::JValueGen::Int(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getPattern",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::block::banner::Pattern::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Removes the pattern at the specified index
    pub fn remove_pattern(
        &self,
        arg0: i32,
    ) -> Result<crate::block::banner::Pattern<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(I)Lorg/bukkit/block/banner/Pattern;");
        let val_1 = jni::objects::JValueGen::Int(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removePattern",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::block::banner::Pattern::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn set_pattern(
        &self,
        arg0: i32,
        arg1: impl Into<crate::block::banner::Pattern<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(ILorg/bukkit/block/banner/Pattern;)V");
        let val_1 = jni::objects::JValueGen::Int(arg0);
        let val_2 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg1.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setPattern",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
            ],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn number_of_patterns(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "numberOfPatterns",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    pub fn persistent_data_container(
        &self,
    ) -> Result<crate::persistence::PersistentDataContainer<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = Banner::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::TileState = temp_clone.into();
        real.persistent_data_container()
    }
    // SUPER CLASS: BlockState
    pub fn set_metadata(
        &self,
        arg0: impl Into<String>,
        arg1: impl Into<crate::metadata::MetadataValue<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::metadata::Metadatable = temp_clone.into();
        real.set_metadata(arg0, arg1)
    }
    pub fn get_metadata(
        &self,
        arg0: impl Into<String>,
    ) -> Result<Vec<crate::metadata::MetadataValue<'mc>>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::metadata::Metadatable = temp_clone.into();
        real.get_metadata(arg0)
    }
    pub fn has_metadata(
        &self,
        arg0: impl Into<String>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::metadata::Metadatable = temp_clone.into();
        real.has_metadata(arg0)
    }
    pub fn remove_metadata(
        &self,
        arg0: impl Into<String>,
        arg1: impl Into<crate::plugin::Plugin<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::metadata::Metadatable = temp_clone.into();
        real.remove_metadata(arg0, arg1)
    }
    pub fn data(&self) -> Result<crate::material::MaterialData<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.data()
    }
    pub fn world(&self) -> Result<crate::World<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.world()
    }
    pub fn set_type(
        &self,
        arg0: impl Into<crate::Material<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.set_type(arg0)
    }
    pub fn block(&self) -> Result<crate::block::Block<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.block()
    }
    pub fn block_data(
        &self,
    ) -> Result<crate::block::data::BlockData<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.block_data()
    }
    pub fn light_level(&self) -> Result<i8, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.light_level()
    }
    pub fn x(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.x()
    }
    pub fn y(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.y()
    }
    pub fn z(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.z()
    }
    pub fn chunk(&self) -> Result<crate::Chunk<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.chunk()
    }
    pub fn set_data(
        &self,
        arg0: impl Into<crate::material::MaterialData<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.set_data(arg0)
    }
    pub fn set_block_data(
        &self,
        arg0: impl Into<crate::block::data::BlockData<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.set_block_data(arg0)
    }

    pub fn raw_data(&self) -> Result<i8, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.raw_data()
    }

    pub fn set_raw_data(&self, arg0: i8) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.set_raw_data(arg0)
    }
    pub fn is_placed(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.is_placed()
    }
    pub fn update_with_boolean(
        &self,
        arg0: std::option::Option<bool>,
        arg1: std::option::Option<bool>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "Z";
            let val_1 = jni::objects::JValueGen::Bool(a.into());
            args.push(val_1);
        }
        if let Some(a) = arg1 {
            sig += "Z";
            let val_2 = jni::objects::JValueGen::Bool(a.into());
            args.push(val_2);
        }
        sig += ")Z";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "update", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    pub fn location(&self) -> Result<crate::Location<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.location()
    }
    pub fn get_type(&self) -> Result<crate::Material<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.get_type()
    }
    // SUPER CLASS: PersistentDataHolder

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::block::TileState<'mc>> for Banner<'mc> {
    fn into(self) -> crate::block::TileState<'mc> {
        crate::block::TileState::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting Banner into crate::block::TileState")
    }
}
/// Represents a captured state of a Barrel.
///
/// This is a representation of an abstract class.
#[repr(C)]
pub struct Barrel<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for Barrel<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for Barrel<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Barrel from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/block/Barrel")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Barrel object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> Barrel<'mc> {
    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::block::Container<'mc>> for Barrel<'mc> {
    fn into(self) -> crate::block::Container<'mc> {
        crate::block::Container::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting Barrel into crate::block::Container")
    }
}
impl<'mc> Into<crate::loot::Lootable<'mc>> for Barrel<'mc> {
    fn into(self) -> crate::loot::Lootable<'mc> {
        crate::loot::Lootable::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting Barrel into crate::loot::Lootable")
    }
}
impl<'mc> Into<crate::block::Lidded<'mc>> for Barrel<'mc> {
    fn into(self) -> crate::block::Lidded<'mc> {
        crate::block::Lidded::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting Barrel into crate::block::Lidded")
    }
}
/// Represents a captured state of a command block.
///
/// This is a representation of an abstract class.
#[repr(C)]
pub struct CommandBlock<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for CommandBlock<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for CommandBlock<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate CommandBlock from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/block/CommandBlock")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a CommandBlock object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> CommandBlock<'mc> {
    pub fn command(&self) -> Result<String, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/String;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getCommand", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }

    pub fn set_command(&self, arg0: impl Into<String>) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/String;)V");
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(arg0.into())?,
        ));
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setCommand",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn name(&self) -> Result<String, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/String;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getName", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }

    pub fn set_name(&self, arg0: impl Into<String>) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/String;)V");
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(arg0.into())?,
        ));
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setName",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub fn persistent_data_container(
        &self,
    ) -> Result<crate::persistence::PersistentDataContainer<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = CommandBlock::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::TileState = temp_clone.into();
        real.persistent_data_container()
    }
    // SUPER CLASS: BlockState
    pub fn set_metadata(
        &self,
        arg0: impl Into<String>,
        arg1: impl Into<crate::metadata::MetadataValue<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::metadata::Metadatable = temp_clone.into();
        real.set_metadata(arg0, arg1)
    }
    pub fn get_metadata(
        &self,
        arg0: impl Into<String>,
    ) -> Result<Vec<crate::metadata::MetadataValue<'mc>>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::metadata::Metadatable = temp_clone.into();
        real.get_metadata(arg0)
    }
    pub fn has_metadata(
        &self,
        arg0: impl Into<String>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::metadata::Metadatable = temp_clone.into();
        real.has_metadata(arg0)
    }
    pub fn remove_metadata(
        &self,
        arg0: impl Into<String>,
        arg1: impl Into<crate::plugin::Plugin<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::metadata::Metadatable = temp_clone.into();
        real.remove_metadata(arg0, arg1)
    }
    pub fn data(&self) -> Result<crate::material::MaterialData<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.data()
    }
    pub fn world(&self) -> Result<crate::World<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.world()
    }
    pub fn set_type(
        &self,
        arg0: impl Into<crate::Material<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.set_type(arg0)
    }
    pub fn block(&self) -> Result<crate::block::Block<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.block()
    }
    pub fn block_data(
        &self,
    ) -> Result<crate::block::data::BlockData<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.block_data()
    }
    pub fn light_level(&self) -> Result<i8, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.light_level()
    }
    pub fn x(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.x()
    }
    pub fn y(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.y()
    }
    pub fn z(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.z()
    }
    pub fn chunk(&self) -> Result<crate::Chunk<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.chunk()
    }
    pub fn set_data(
        &self,
        arg0: impl Into<crate::material::MaterialData<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.set_data(arg0)
    }
    pub fn set_block_data(
        &self,
        arg0: impl Into<crate::block::data::BlockData<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.set_block_data(arg0)
    }

    pub fn raw_data(&self) -> Result<i8, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.raw_data()
    }

    pub fn set_raw_data(&self, arg0: i8) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.set_raw_data(arg0)
    }
    pub fn is_placed(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.is_placed()
    }
    pub fn update_with_boolean(
        &self,
        arg0: std::option::Option<bool>,
        arg1: std::option::Option<bool>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "Z";
            let val_1 = jni::objects::JValueGen::Bool(a.into());
            args.push(val_1);
        }
        if let Some(a) = arg1 {
            sig += "Z";
            let val_2 = jni::objects::JValueGen::Bool(a.into());
            args.push(val_2);
        }
        sig += ")Z";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "update", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    pub fn location(&self) -> Result<crate::Location<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.location()
    }
    pub fn get_type(&self) -> Result<crate::Material<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.get_type()
    }
    // SUPER CLASS: PersistentDataHolder

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::block::TileState<'mc>> for CommandBlock<'mc> {
    fn into(self) -> crate::block::TileState<'mc> {
        crate::block::TileState::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting CommandBlock into crate::block::TileState")
    }
}
/// Represents a captured state of a sculk sensor
///
/// This is a representation of an abstract class.
#[repr(C)]
pub struct SculkSensor<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for SculkSensor<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for SculkSensor<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate SculkSensor from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/block/SculkSensor")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a SculkSensor object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> SculkSensor<'mc> {
    pub fn last_vibration_frequency(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getLastVibrationFrequency",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    /// Sets the last vibration frequency of this sensor. Different activities detected by the sensor will produce different frequencies and dictate the output of connected comparators.
    pub fn set_last_vibration_frequency(
        &self,
        arg0: i32,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(I)V");
        let val_1 = jni::objects::JValueGen::Int(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setLastVibrationFrequency",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub fn persistent_data_container(
        &self,
    ) -> Result<crate::persistence::PersistentDataContainer<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = SculkSensor::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::TileState = temp_clone.into();
        real.persistent_data_container()
    }
    // SUPER CLASS: BlockState
    pub fn set_metadata(
        &self,
        arg0: impl Into<String>,
        arg1: impl Into<crate::metadata::MetadataValue<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::metadata::Metadatable = temp_clone.into();
        real.set_metadata(arg0, arg1)
    }
    pub fn get_metadata(
        &self,
        arg0: impl Into<String>,
    ) -> Result<Vec<crate::metadata::MetadataValue<'mc>>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::metadata::Metadatable = temp_clone.into();
        real.get_metadata(arg0)
    }
    pub fn has_metadata(
        &self,
        arg0: impl Into<String>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::metadata::Metadatable = temp_clone.into();
        real.has_metadata(arg0)
    }
    pub fn remove_metadata(
        &self,
        arg0: impl Into<String>,
        arg1: impl Into<crate::plugin::Plugin<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::metadata::Metadatable = temp_clone.into();
        real.remove_metadata(arg0, arg1)
    }
    pub fn data(&self) -> Result<crate::material::MaterialData<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.data()
    }
    pub fn world(&self) -> Result<crate::World<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.world()
    }
    pub fn set_type(
        &self,
        arg0: impl Into<crate::Material<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.set_type(arg0)
    }
    pub fn block(&self) -> Result<crate::block::Block<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.block()
    }
    pub fn block_data(
        &self,
    ) -> Result<crate::block::data::BlockData<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.block_data()
    }
    pub fn light_level(&self) -> Result<i8, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.light_level()
    }
    pub fn x(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.x()
    }
    pub fn y(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.y()
    }
    pub fn z(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.z()
    }
    pub fn chunk(&self) -> Result<crate::Chunk<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.chunk()
    }
    pub fn set_data(
        &self,
        arg0: impl Into<crate::material::MaterialData<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.set_data(arg0)
    }
    pub fn set_block_data(
        &self,
        arg0: impl Into<crate::block::data::BlockData<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.set_block_data(arg0)
    }

    pub fn raw_data(&self) -> Result<i8, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.raw_data()
    }

    pub fn set_raw_data(&self, arg0: i8) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.set_raw_data(arg0)
    }
    pub fn is_placed(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.is_placed()
    }
    pub fn update_with_boolean(
        &self,
        arg0: std::option::Option<bool>,
        arg1: std::option::Option<bool>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "Z";
            let val_1 = jni::objects::JValueGen::Bool(a.into());
            args.push(val_1);
        }
        if let Some(a) = arg1 {
            sig += "Z";
            let val_2 = jni::objects::JValueGen::Bool(a.into());
            args.push(val_2);
        }
        sig += ")Z";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "update", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    pub fn location(&self) -> Result<crate::Location<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.location()
    }
    pub fn get_type(&self) -> Result<crate::Material<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.get_type()
    }
    // SUPER CLASS: PersistentDataHolder

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::block::TileState<'mc>> for SculkSensor<'mc> {
    fn into(self) -> crate::block::TileState<'mc> {
        crate::block::TileState::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting SculkSensor into crate::block::TileState")
    }
}
/// Represents a captured state of a hanging sign.
///
/// This is a representation of an abstract class.
#[repr(C)]
pub struct HangingSign<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for HangingSign<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for HangingSign<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate HangingSign from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/block/HangingSign")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a HangingSign object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> HangingSign<'mc> {
    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::block::Sign<'mc>> for HangingSign<'mc> {
    fn into(self) -> crate::block::Sign<'mc> {
        crate::block::Sign::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting HangingSign into crate::block::Sign")
    }
}
/// Represents a captured state of a chiseled bookshelf.
///
/// This is a representation of an abstract class.
#[repr(C)]
pub struct ChiseledBookshelf<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for ChiseledBookshelf<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for ChiseledBookshelf<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate ChiseledBookshelf from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/block/ChiseledBookshelf")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a ChiseledBookshelf object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> ChiseledBookshelf<'mc> {
    pub fn inventory(
        &self,
    ) -> Result<crate::inventory::ChiseledBookshelfInventory<'mc>, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Lorg/bukkit/inventory/ChiseledBookshelfInventory;";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getInventory", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::ChiseledBookshelfInventory::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn snapshot_inventory(
        &self,
    ) -> Result<crate::inventory::ChiseledBookshelfInventory<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/inventory/ChiseledBookshelfInventory;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getSnapshotInventory",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::ChiseledBookshelfInventory::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn last_interacted_slot(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getLastInteractedSlot",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    /// Sets the last interacted inventory slot.
    pub fn set_last_interacted_slot(&self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(I)V");
        let val_1 = jni::objects::JValueGen::Int(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setLastInteractedSlot",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn get_slot(
        &self,
        arg0: impl Into<crate::util::Vector<'mc>>,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/util/Vector;)I");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getSlot",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    pub fn persistent_data_container(
        &self,
    ) -> Result<crate::persistence::PersistentDataContainer<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = ChiseledBookshelf::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::TileState = temp_clone.into();
        real.persistent_data_container()
    }
    // SUPER CLASS: BlockState
    pub fn set_metadata(
        &self,
        arg0: impl Into<String>,
        arg1: impl Into<crate::metadata::MetadataValue<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::metadata::Metadatable = temp_clone.into();
        real.set_metadata(arg0, arg1)
    }
    pub fn get_metadata(
        &self,
        arg0: impl Into<String>,
    ) -> Result<Vec<crate::metadata::MetadataValue<'mc>>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::metadata::Metadatable = temp_clone.into();
        real.get_metadata(arg0)
    }
    pub fn has_metadata(
        &self,
        arg0: impl Into<String>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::metadata::Metadatable = temp_clone.into();
        real.has_metadata(arg0)
    }
    pub fn remove_metadata(
        &self,
        arg0: impl Into<String>,
        arg1: impl Into<crate::plugin::Plugin<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::metadata::Metadatable = temp_clone.into();
        real.remove_metadata(arg0, arg1)
    }
    pub fn data(&self) -> Result<crate::material::MaterialData<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.data()
    }
    pub fn world(&self) -> Result<crate::World<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.world()
    }
    pub fn set_type(
        &self,
        arg0: impl Into<crate::Material<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.set_type(arg0)
    }
    pub fn block(&self) -> Result<crate::block::Block<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.block()
    }
    pub fn block_data(
        &self,
    ) -> Result<crate::block::data::BlockData<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.block_data()
    }
    pub fn light_level(&self) -> Result<i8, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.light_level()
    }
    pub fn x(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.x()
    }
    pub fn y(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.y()
    }
    pub fn z(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.z()
    }
    pub fn chunk(&self) -> Result<crate::Chunk<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.chunk()
    }
    pub fn set_data(
        &self,
        arg0: impl Into<crate::material::MaterialData<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.set_data(arg0)
    }
    pub fn set_block_data(
        &self,
        arg0: impl Into<crate::block::data::BlockData<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.set_block_data(arg0)
    }

    pub fn raw_data(&self) -> Result<i8, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.raw_data()
    }

    pub fn set_raw_data(&self, arg0: i8) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.set_raw_data(arg0)
    }
    pub fn is_placed(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.is_placed()
    }
    pub fn update_with_boolean(
        &self,
        arg0: std::option::Option<bool>,
        arg1: std::option::Option<bool>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "Z";
            let val_1 = jni::objects::JValueGen::Bool(a.into());
            args.push(val_1);
        }
        if let Some(a) = arg1 {
            sig += "Z";
            let val_2 = jni::objects::JValueGen::Bool(a.into());
            args.push(val_2);
        }
        sig += ")Z";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "update", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    pub fn location(&self) -> Result<crate::Location<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.location()
    }
    pub fn get_type(&self) -> Result<crate::Material<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.get_type()
    }
    // SUPER CLASS: PersistentDataHolder
    // SUPER CLASS: InventoryHolder

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::block::TileState<'mc>> for ChiseledBookshelf<'mc> {
    fn into(self) -> crate::block::TileState<'mc> {
        crate::block::TileState::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting ChiseledBookshelf into crate::block::TileState")
    }
}
impl<'mc> Into<crate::inventory::BlockInventoryHolder<'mc>> for ChiseledBookshelf<'mc> {
    fn into(self) -> crate::inventory::BlockInventoryHolder<'mc> {
        crate::inventory::BlockInventoryHolder::from_raw(&self.jni_ref(), self.1).expect(
            "Error converting ChiseledBookshelf into crate::inventory::BlockInventoryHolder",
        )
    }
}
/// Represents a block state that also hosts a tile entity at the given location. This interface alone is merely a marker that does not provide any data. Data about the tile entities is provided by the respective interface for each tile entity type. After modifying the data provided by a TileState, <a href="BlockState.html#update()"><code>BlockState.update()</code></a> needs to be called to store the data.
///
/// This is a representation of an abstract class.
#[repr(C)]
pub struct TileState<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for TileState<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for TileState<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate TileState from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/block/TileState")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a TileState object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> TileState<'mc> {
    pub fn persistent_data_container(
        &self,
    ) -> Result<crate::persistence::PersistentDataContainer<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/persistence/PersistentDataContainer;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getPersistentDataContainer",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::persistence::PersistentDataContainer::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    pub fn data(&self) -> Result<crate::material::MaterialData<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = TileState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.data()
    }
    pub fn world(&self) -> Result<crate::World<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = TileState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.world()
    }
    pub fn set_type(
        &self,
        arg0: impl Into<crate::Material<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = TileState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.set_type(arg0)
    }
    pub fn block(&self) -> Result<crate::block::Block<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = TileState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.block()
    }
    pub fn block_data(
        &self,
    ) -> Result<crate::block::data::BlockData<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = TileState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.block_data()
    }
    pub fn light_level(&self) -> Result<i8, Box<dyn std::error::Error>> {
        let temp_clone = TileState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.light_level()
    }
    pub fn x(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let temp_clone = TileState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.x()
    }
    pub fn y(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let temp_clone = TileState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.y()
    }
    pub fn z(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let temp_clone = TileState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.z()
    }
    pub fn chunk(&self) -> Result<crate::Chunk<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = TileState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.chunk()
    }
    pub fn set_data(
        &self,
        arg0: impl Into<crate::material::MaterialData<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = TileState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.set_data(arg0)
    }
    pub fn set_block_data(
        &self,
        arg0: impl Into<crate::block::data::BlockData<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = TileState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.set_block_data(arg0)
    }

    pub fn raw_data(&self) -> Result<i8, Box<dyn std::error::Error>> {
        let temp_clone = TileState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.raw_data()
    }

    pub fn set_raw_data(&self, arg0: i8) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = TileState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.set_raw_data(arg0)
    }
    pub fn is_placed(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = TileState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.is_placed()
    }
    pub fn update_with_boolean(
        &self,
        arg0: std::option::Option<bool>,
        arg1: std::option::Option<bool>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "Z";
            let val_1 = jni::objects::JValueGen::Bool(a.into());
            args.push(val_1);
        }
        if let Some(a) = arg1 {
            sig += "Z";
            let val_2 = jni::objects::JValueGen::Bool(a.into());
            args.push(val_2);
        }
        sig += ")Z";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "update", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    pub fn location(&self) -> Result<crate::Location<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = TileState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.location()
    }
    pub fn get_type(&self) -> Result<crate::Material<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = TileState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.get_type()
    }
    // SUPER CLASS: Metadatable
    pub fn set_metadata(
        &self,
        arg0: impl Into<String>,
        arg1: impl Into<crate::metadata::MetadataValue<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::metadata::Metadatable::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::metadata::Metadatable = temp_clone.into();
        real.set_metadata(arg0, arg1)
    }
    pub fn get_metadata(
        &self,
        arg0: impl Into<String>,
    ) -> Result<Vec<crate::metadata::MetadataValue<'mc>>, Box<dyn std::error::Error>> {
        let temp_clone = crate::metadata::Metadatable::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::metadata::Metadatable = temp_clone.into();
        real.get_metadata(arg0)
    }
    pub fn has_metadata(
        &self,
        arg0: impl Into<String>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::metadata::Metadatable::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::metadata::Metadatable = temp_clone.into();
        real.has_metadata(arg0)
    }
    pub fn remove_metadata(
        &self,
        arg0: impl Into<String>,
        arg1: impl Into<crate::plugin::Plugin<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::metadata::Metadatable::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::metadata::Metadatable = temp_clone.into();
        real.remove_metadata(arg0, arg1)
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::block::BlockState<'mc>> for TileState<'mc> {
    fn into(self) -> crate::block::BlockState<'mc> {
        crate::block::BlockState::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting TileState into crate::block::BlockState")
    }
}
impl<'mc> Into<crate::persistence::PersistentDataHolder<'mc>> for TileState<'mc> {
    fn into(self) -> crate::persistence::PersistentDataHolder<'mc> {
        crate::persistence::PersistentDataHolder::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting TileState into crate::persistence::PersistentDataHolder")
    }
}
/// Represents a captured state of a block, which will not change automatically.
/// <p>Unlike Block, which only one object can exist per coordinate, BlockState can exist multiple times for any given Block. Note that another plugin may change the state of the block and you will not know, or they may change the block to another type entirely, causing your BlockState to become invalid.</p>
///
/// This is a representation of an abstract class.
#[repr(C)]
pub struct BlockState<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for BlockState<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for BlockState<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate BlockState from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/block/BlockState")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a BlockState object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> BlockState<'mc> {
    pub fn data(&self) -> Result<crate::material::MaterialData<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/material/MaterialData;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getData", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::material::MaterialData::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn world(&self) -> Result<crate::World<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/World;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getWorld", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::World::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn set_type(
        &self,
        arg0: impl Into<crate::Material<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/Material;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setType",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn block(&self) -> Result<crate::block::Block<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/block/Block;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getBlock", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::block::Block::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn block_data(
        &self,
    ) -> Result<crate::block::data::BlockData<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/block/data/BlockData;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getBlockData", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::block::data::BlockData::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn light_level(&self) -> Result<i8, Box<dyn std::error::Error>> {
        let sig = String::from("()B");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getLightLevel", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.b()?)
    }

    pub fn x(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getX", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }

    pub fn y(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getY", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }

    pub fn z(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getZ", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }

    pub fn chunk(&self) -> Result<crate::Chunk<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/Chunk;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getChunk", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::Chunk::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn set_data(
        &self,
        arg0: impl Into<crate::material::MaterialData<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/material/MaterialData;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setData",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn set_block_data(
        &self,
        arg0: impl Into<crate::block::data::BlockData<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/data/BlockData;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setBlockData",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    #[deprecated]

    pub fn raw_data(&self) -> Result<i8, Box<dyn std::error::Error>> {
        let sig = String::from("()B");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getRawData", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.b()?)
    }
    #[deprecated = "Magic value "]

    pub fn set_raw_data(&self, arg0: i8) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(B)V");
        let val_1 = jni::objects::JValueGen::Byte(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setRawData",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn is_placed(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isPlaced", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Attempts to update the block represented by this state, setting it to the new values as defined by this state.
    /// <p>If this state is not placed, this will have no effect and return true.</p>
    /// <p>Unless force is true, this will not modify the state of a block if it is no longer the same type as it was when this state was taken. It will return false in this eventuality.</p>
    /// <p>If force is true, it will set the type of the block to match the new state, set the state data and then return true.</p>
    /// <p>If applyPhysics is true, it will trigger a physics update on surrounding blocks which could cause them to update or disappear.</p>
    pub fn update_with_boolean(
        &self,
        arg0: std::option::Option<bool>,
        arg1: std::option::Option<bool>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "Z";
            let val_1 = jni::objects::JValueGen::Bool(a.into());
            args.push(val_1);
        }
        if let Some(a) = arg1 {
            sig += "Z";
            let val_2 = jni::objects::JValueGen::Bool(a.into());
            args.push(val_2);
        }
        sig += ")Z";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "update", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn location(&self) -> Result<crate::Location<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/Location;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getLocation", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::Location::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn get_type(&self) -> Result<crate::Material<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/Material;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getType", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::Material::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    pub fn set_metadata(
        &self,
        arg0: impl Into<String>,
        arg1: impl Into<crate::metadata::MetadataValue<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::metadata::Metadatable = temp_clone.into();
        real.set_metadata(arg0, arg1)
    }
    pub fn get_metadata(
        &self,
        arg0: impl Into<String>,
    ) -> Result<Vec<crate::metadata::MetadataValue<'mc>>, Box<dyn std::error::Error>> {
        let temp_clone = BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::metadata::Metadatable = temp_clone.into();
        real.get_metadata(arg0)
    }
    pub fn has_metadata(
        &self,
        arg0: impl Into<String>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::metadata::Metadatable = temp_clone.into();
        real.has_metadata(arg0)
    }
    pub fn remove_metadata(
        &self,
        arg0: impl Into<String>,
        arg1: impl Into<crate::plugin::Plugin<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::metadata::Metadatable = temp_clone.into();
        real.remove_metadata(arg0, arg1)
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::metadata::Metadatable<'mc>> for BlockState<'mc> {
    fn into(self) -> crate::metadata::Metadatable<'mc> {
        crate::metadata::Metadatable::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting BlockState into crate::metadata::Metadatable")
    }
}
/// Represents a captured state of a brewing stand.
///
/// This is a representation of an abstract class.
#[repr(C)]
pub struct BrewingStand<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for BrewingStand<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for BrewingStand<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate BrewingStand from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/block/BrewingStand")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a BrewingStand object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> BrewingStand<'mc> {
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

    pub fn snapshot_inventory(
        &self,
    ) -> Result<crate::inventory::BrewerInventory<'mc>, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Lorg/bukkit/inventory/BrewerInventory;";
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getSnapshotInventory",
            sig.as_str(),
            args,
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::BrewerInventory::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn brewing_time(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getBrewingTime", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    /// Set the time left before brewing completes.
    pub fn set_brewing_time(&self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(I)V");
        let val_1 = jni::objects::JValueGen::Int(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setBrewingTime",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn fuel_level(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getFuelLevel", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    /// Set the level of current fuel for brewing.
    pub fn set_fuel_level(&self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(I)V");
        let val_1 = jni::objects::JValueGen::Int(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setFuelLevel",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    // SUPER CLASS: TileState
    pub fn data(&self) -> Result<crate::material::MaterialData<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::TileState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.data()
    }
    pub fn world(&self) -> Result<crate::World<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::TileState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.world()
    }
    pub fn set_type(
        &self,
        arg0: impl Into<crate::Material<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::block::TileState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.set_type(arg0)
    }
    pub fn block(&self) -> Result<crate::block::Block<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::TileState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.block()
    }
    pub fn block_data(
        &self,
    ) -> Result<crate::block::data::BlockData<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::TileState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.block_data()
    }
    pub fn light_level(&self) -> Result<i8, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::TileState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.light_level()
    }
    pub fn x(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::TileState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.x()
    }
    pub fn y(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::TileState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.y()
    }
    pub fn z(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::TileState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.z()
    }
    pub fn chunk(&self) -> Result<crate::Chunk<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::TileState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.chunk()
    }
    pub fn set_data(
        &self,
        arg0: impl Into<crate::material::MaterialData<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::block::TileState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.set_data(arg0)
    }
    pub fn set_block_data(
        &self,
        arg0: impl Into<crate::block::data::BlockData<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::block::TileState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.set_block_data(arg0)
    }

    pub fn raw_data(&self) -> Result<i8, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::TileState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.raw_data()
    }

    pub fn set_raw_data(&self, arg0: i8) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::block::TileState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.set_raw_data(arg0)
    }
    pub fn is_placed(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::TileState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.is_placed()
    }
    pub fn update_with_boolean(
        &self,
        arg0: std::option::Option<bool>,
        arg1: std::option::Option<bool>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "Z";
            let val_1 = jni::objects::JValueGen::Bool(a.into());
            args.push(val_1);
        }
        if let Some(a) = arg1 {
            sig += "Z";
            let val_2 = jni::objects::JValueGen::Bool(a.into());
            args.push(val_2);
        }
        sig += ")Z";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "update", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    pub fn location(&self) -> Result<crate::Location<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::TileState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.location()
    }
    pub fn get_type(&self) -> Result<crate::Material<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::TileState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.get_type()
    }
    // SUPER CLASS: Metadatable
    pub fn set_metadata(
        &self,
        arg0: impl Into<String>,
        arg1: impl Into<crate::metadata::MetadataValue<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::metadata::Metadatable::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::metadata::Metadatable = temp_clone.into();
        real.set_metadata(arg0, arg1)
    }
    pub fn get_metadata(
        &self,
        arg0: impl Into<String>,
    ) -> Result<Vec<crate::metadata::MetadataValue<'mc>>, Box<dyn std::error::Error>> {
        let temp_clone = crate::metadata::Metadatable::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::metadata::Metadatable = temp_clone.into();
        real.get_metadata(arg0)
    }
    pub fn has_metadata(
        &self,
        arg0: impl Into<String>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::metadata::Metadatable::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::metadata::Metadatable = temp_clone.into();
        real.has_metadata(arg0)
    }
    pub fn remove_metadata(
        &self,
        arg0: impl Into<String>,
        arg1: impl Into<crate::plugin::Plugin<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::metadata::Metadatable::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::metadata::Metadatable = temp_clone.into();
        real.remove_metadata(arg0, arg1)
    }
    pub fn persistent_data_container(
        &self,
    ) -> Result<crate::persistence::PersistentDataContainer<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::TileState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::persistence::PersistentDataHolder = temp_clone.into();
        real.persistent_data_container()
    }
    // SUPER CLASS: BlockInventoryHolder
    // SUPER CLASS: Lockable
    pub fn is_locked(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::Lockable::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::Lockable = temp_clone.into();
        real.is_locked()
    }
    pub fn lock(&self) -> Result<String, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::Lockable::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::Lockable = temp_clone.into();
        real.lock()
    }
    pub fn set_lock(&self, arg0: impl Into<String>) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::block::Lockable::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::Lockable = temp_clone.into();
        real.set_lock(arg0)
    }
    // SUPER CLASS: Nameable
    pub fn custom_name(&self) -> Result<Option<String>, Box<dyn std::error::Error>> {
        let temp_clone = crate::Nameable::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::Nameable = temp_clone.into();
        real.custom_name()
    }
    pub fn set_custom_name(
        &self,
        arg0: impl Into<String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::Nameable::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::Nameable = temp_clone.into();
        real.set_custom_name(arg0)
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::block::Container<'mc>> for BrewingStand<'mc> {
    fn into(self) -> crate::block::Container<'mc> {
        crate::block::Container::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting BrewingStand into crate::block::Container")
    }
}
/// Represents a captured state of a skull block.
///
/// This is a representation of an abstract class.
#[repr(C)]
pub struct Skull<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for Skull<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for Skull<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Skull from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/block/Skull")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Skull object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> Skull<'mc> {
    pub fn owner(&self) -> Result<Option<String>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/String;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getOwner", sig.as_str(), vec![]);
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

    pub fn set_owner(&self, arg0: impl Into<String>) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/String;)Z");
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(arg0.into())?,
        ));
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setOwner",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn rotation(&self) -> Result<crate::block::BlockFace<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/block/BlockFace;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getRotation", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::block::BlockFace::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn set_rotation(
        &self,
        arg0: impl Into<crate::block::BlockFace<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/BlockFace;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setRotation",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn has_owner(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasOwner", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn owning_player(
        &self,
    ) -> Result<Option<crate::OfflinePlayer<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/OfflinePlayer;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getOwningPlayer", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {
            return Ok(None);
        }
        Ok(Some(crate::OfflinePlayer::from_raw(
            &self.jni_ref(),
            unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) },
        )?))
    }

    pub fn set_owning_player(
        &self,
        arg0: impl Into<crate::OfflinePlayer<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/OfflinePlayer;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setOwningPlayer",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn owner_profile(
        &self,
    ) -> Result<Option<crate::profile::PlayerProfile<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/profile/PlayerProfile;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getOwnerProfile", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {
            return Ok(None);
        }
        Ok(Some(crate::profile::PlayerProfile::from_raw(
            &self.jni_ref(),
            unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) },
        )?))
    }

    pub fn set_owner_profile(
        &self,
        arg0: impl Into<crate::profile::PlayerProfile<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/profile/PlayerProfile;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setOwnerProfile",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn note_block_sound(
        &self,
    ) -> Result<Option<crate::NamespacedKey<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/NamespacedKey;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getNoteBlockSound",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {
            return Ok(None);
        }
        Ok(Some(crate::NamespacedKey::from_raw(
            &self.jni_ref(),
            unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) },
        )?))
    }

    pub fn set_note_block_sound(
        &self,
        arg0: impl Into<crate::NamespacedKey<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/NamespacedKey;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setNoteBlockSound",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    #[deprecated]

    pub fn skull_type(&self) -> Result<crate::SkullType<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/SkullType;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getSkullType", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::SkullType::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn set_skull_type(
        &self,
        arg0: impl Into<crate::SkullType<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/SkullType;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setSkullType",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub fn persistent_data_container(
        &self,
    ) -> Result<crate::persistence::PersistentDataContainer<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = Skull::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::TileState = temp_clone.into();
        real.persistent_data_container()
    }
    // SUPER CLASS: BlockState
    pub fn set_metadata(
        &self,
        arg0: impl Into<String>,
        arg1: impl Into<crate::metadata::MetadataValue<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::metadata::Metadatable = temp_clone.into();
        real.set_metadata(arg0, arg1)
    }
    pub fn get_metadata(
        &self,
        arg0: impl Into<String>,
    ) -> Result<Vec<crate::metadata::MetadataValue<'mc>>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::metadata::Metadatable = temp_clone.into();
        real.get_metadata(arg0)
    }
    pub fn has_metadata(
        &self,
        arg0: impl Into<String>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::metadata::Metadatable = temp_clone.into();
        real.has_metadata(arg0)
    }
    pub fn remove_metadata(
        &self,
        arg0: impl Into<String>,
        arg1: impl Into<crate::plugin::Plugin<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::metadata::Metadatable = temp_clone.into();
        real.remove_metadata(arg0, arg1)
    }
    pub fn data(&self) -> Result<crate::material::MaterialData<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.data()
    }
    pub fn world(&self) -> Result<crate::World<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.world()
    }
    pub fn set_type(
        &self,
        arg0: impl Into<crate::Material<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.set_type(arg0)
    }
    pub fn block(&self) -> Result<crate::block::Block<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.block()
    }
    pub fn block_data(
        &self,
    ) -> Result<crate::block::data::BlockData<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.block_data()
    }
    pub fn light_level(&self) -> Result<i8, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.light_level()
    }
    pub fn x(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.x()
    }
    pub fn y(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.y()
    }
    pub fn z(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.z()
    }
    pub fn chunk(&self) -> Result<crate::Chunk<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.chunk()
    }
    pub fn set_data(
        &self,
        arg0: impl Into<crate::material::MaterialData<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.set_data(arg0)
    }
    pub fn set_block_data(
        &self,
        arg0: impl Into<crate::block::data::BlockData<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.set_block_data(arg0)
    }

    pub fn raw_data(&self) -> Result<i8, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.raw_data()
    }

    pub fn set_raw_data(&self, arg0: i8) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.set_raw_data(arg0)
    }
    pub fn is_placed(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.is_placed()
    }
    pub fn update_with_boolean(
        &self,
        arg0: std::option::Option<bool>,
        arg1: std::option::Option<bool>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "Z";
            let val_1 = jni::objects::JValueGen::Bool(a.into());
            args.push(val_1);
        }
        if let Some(a) = arg1 {
            sig += "Z";
            let val_2 = jni::objects::JValueGen::Bool(a.into());
            args.push(val_2);
        }
        sig += ")Z";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "update", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    pub fn location(&self) -> Result<crate::Location<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.location()
    }
    pub fn get_type(&self) -> Result<crate::Material<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.get_type()
    }
    // SUPER CLASS: PersistentDataHolder

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::block::TileState<'mc>> for Skull<'mc> {
    fn into(self) -> crate::block::TileState<'mc> {
        crate::block::TileState::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting Skull into crate::block::TileState")
    }
}
/// Represents a captured state of a hopper.
///
/// This is a representation of an abstract class.
#[repr(C)]
pub struct Hopper<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for Hopper<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for Hopper<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Hopper from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/block/Hopper")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Hopper object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> Hopper<'mc> {
    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::block::Container<'mc>> for Hopper<'mc> {
    fn into(self) -> crate::block::Container<'mc> {
        crate::block::Container::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting Hopper into crate::block::Container")
    }
}
impl<'mc> Into<crate::loot::Lootable<'mc>> for Hopper<'mc> {
    fn into(self) -> crate::loot::Lootable<'mc> {
        crate::loot::Lootable::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting Hopper into crate::loot::Lootable")
    }
}
/// Represents a captured state of either a SignPost or a WallSign.
///
/// This is a representation of an abstract class.
#[repr(C)]
pub struct Sign<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for Sign<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for Sign<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Sign from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/block/Sign")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Sign object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> Sign<'mc> {
    pub fn set_color(
        &self,
        arg0: impl Into<crate::DyeColor<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/DyeColor;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setColor",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    #[deprecated = "use <a href='#setWaxed(boolean)'><code>setWaxed(boolean)</code></a> instead "]
    /// Marks whether this sign can be edited by players.
    pub fn set_editable(&self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        let val_1 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setEditable",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    #[deprecated]

    pub fn is_glowing_text(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isGlowingText", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    #[deprecated]

    pub fn lines(&self) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/String;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getLines", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        let arr = Into::<jni::objects::JObjectArray>::into(res.l()?);
        let len = self.jni_ref().get_array_length(&arr)?;
        let mut vec = Vec::new();
        for i in 0..len {
            let res = self.jni_ref().get_object_array_element(&arr, i)?;
            vec.push({
                self.jni_ref()
                    .get_string(unsafe { &jni::objects::JString::from_raw(*res) })
                    .unwrap()
                    .to_string_lossy()
                    .to_string()
            });
        }
        Ok(vec)
    }
    #[deprecated = "A sign may have multiple writable sides now. Use <a href='#getSide(org.bukkit.block.sign.Side)'><code>getSide(Side)</code></a> and <a href='sign/SignSide.html#getLine(int)'><code>SignSide.getLine(int)</code></a>. "]
    /// Gets the line of text at the specified index.<p>For example, getLine(0) will return the first line of text on the <a href="sign/Side.html#FRONT"><code>Side.FRONT</code></a>.</p>
    pub fn get_line(&self, arg0: i32) -> Result<String, Box<dyn std::error::Error>> {
        let sig = String::from("(I)Ljava/lang/String;");
        let val_1 = jni::objects::JValueGen::Int(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getLine",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }

    pub fn set_line(
        &self,
        arg0: i32,
        arg1: impl Into<String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(ILjava/lang/String;)V");
        let val_1 = jni::objects::JValueGen::Int(arg0);
        let val_2 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(arg1.into())?,
        ));
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setLine",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
            ],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    #[deprecated]

    pub fn is_editable(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isEditable", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn is_waxed(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isWaxed", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Sets whether or not this sign has been waxed. If a sign has been waxed, it cannot be edited by a player.
    pub fn set_waxed(&self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        let val_1 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setWaxed",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    #[deprecated = "A sign may have multiple writable sides now. Use <a href='#getSide(org.bukkit.block.sign.Side)'><code>getSide(Side)</code></a> and <a href='sign/SignSide.html#setGlowingText(boolean)'><code>SignSide.setGlowingText(boolean)</code></a>. "]
    /// Sets whether this sign has glowing text. Only affects the <a href="sign/Side.html#FRONT"><code>Side.FRONT</code></a>.
    pub fn set_glowing_text(&self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        let val_1 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setGlowingText",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn color(&self) -> Result<Option<crate::DyeColor<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/DyeColor;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getColor", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {
            return Ok(None);
        }
        Ok(Some(crate::DyeColor::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })?))
    }

    pub fn get_side(
        &self,
        arg0: impl Into<crate::block::sign::Side<'mc>>,
    ) -> Result<crate::block::sign::SignSide<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/sign/Side;)Lorg/bukkit/block/sign/SignSide;");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getSide",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::block::sign::SignSide::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    pub fn persistent_data_container(
        &self,
    ) -> Result<crate::persistence::PersistentDataContainer<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = Sign::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::TileState = temp_clone.into();
        real.persistent_data_container()
    }
    // SUPER CLASS: BlockState
    pub fn set_metadata(
        &self,
        arg0: impl Into<String>,
        arg1: impl Into<crate::metadata::MetadataValue<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::metadata::Metadatable = temp_clone.into();
        real.set_metadata(arg0, arg1)
    }
    pub fn get_metadata(
        &self,
        arg0: impl Into<String>,
    ) -> Result<Vec<crate::metadata::MetadataValue<'mc>>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::metadata::Metadatable = temp_clone.into();
        real.get_metadata(arg0)
    }
    pub fn has_metadata(
        &self,
        arg0: impl Into<String>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::metadata::Metadatable = temp_clone.into();
        real.has_metadata(arg0)
    }
    pub fn remove_metadata(
        &self,
        arg0: impl Into<String>,
        arg1: impl Into<crate::plugin::Plugin<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::metadata::Metadatable = temp_clone.into();
        real.remove_metadata(arg0, arg1)
    }
    pub fn data(&self) -> Result<crate::material::MaterialData<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.data()
    }
    pub fn world(&self) -> Result<crate::World<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.world()
    }
    pub fn set_type(
        &self,
        arg0: impl Into<crate::Material<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.set_type(arg0)
    }
    pub fn block(&self) -> Result<crate::block::Block<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.block()
    }
    pub fn block_data(
        &self,
    ) -> Result<crate::block::data::BlockData<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.block_data()
    }
    pub fn light_level(&self) -> Result<i8, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.light_level()
    }
    pub fn x(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.x()
    }
    pub fn y(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.y()
    }
    pub fn z(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.z()
    }
    pub fn chunk(&self) -> Result<crate::Chunk<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.chunk()
    }
    pub fn set_data(
        &self,
        arg0: impl Into<crate::material::MaterialData<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.set_data(arg0)
    }
    pub fn set_block_data(
        &self,
        arg0: impl Into<crate::block::data::BlockData<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.set_block_data(arg0)
    }

    pub fn raw_data(&self) -> Result<i8, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.raw_data()
    }

    pub fn set_raw_data(&self, arg0: i8) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.set_raw_data(arg0)
    }
    pub fn is_placed(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.is_placed()
    }
    pub fn update_with_boolean(
        &self,
        arg0: std::option::Option<bool>,
        arg1: std::option::Option<bool>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "Z";
            let val_1 = jni::objects::JValueGen::Bool(a.into());
            args.push(val_1);
        }
        if let Some(a) = arg1 {
            sig += "Z";
            let val_2 = jni::objects::JValueGen::Bool(a.into());
            args.push(val_2);
        }
        sig += ")Z";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "update", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    pub fn location(&self) -> Result<crate::Location<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.location()
    }
    pub fn get_type(&self) -> Result<crate::Material<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::BlockState::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::BlockState = temp_clone.into();
        real.get_type()
    }
    // SUPER CLASS: PersistentDataHolder

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::block::TileState<'mc>> for Sign<'mc> {
    fn into(self) -> crate::block::TileState<'mc> {
        crate::block::TileState::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting Sign into crate::block::TileState")
    }
}
impl<'mc> Into<crate::material::Colorable<'mc>> for Sign<'mc> {
    fn into(self) -> crate::material::Colorable<'mc> {
        crate::material::Colorable::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting Sign into crate::material::Colorable")
    }
}
/// Represents a double chest.
#[repr(C)]
pub struct DoubleChest<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for DoubleChest<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for DoubleChest<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate DoubleChest from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/block/DoubleChest")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a DoubleChest object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> DoubleChest<'mc> {
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<crate::inventory::DoubleChestInventory<'mc>>,
    ) -> Result<crate::block::DoubleChest<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/inventory/DoubleChestInventory;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let cls = jni.find_class("org/bukkit/block/DoubleChest");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(
            cls,
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = jni.translate_error_no_gen(res)?;
        crate::block::DoubleChest::from_raw(&jni, res)
    }

    pub fn world(&self) -> Result<crate::World<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/World;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getWorld", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::World::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn x(&self) -> Result<f64, Box<dyn std::error::Error>> {
        let sig = String::from("()D");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getX", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.d()?)
    }

    pub fn y(&self) -> Result<f64, Box<dyn std::error::Error>> {
        let sig = String::from("()D");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getY", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.d()?)
    }

    pub fn z(&self) -> Result<f64, Box<dyn std::error::Error>> {
        let sig = String::from("()D");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getZ", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.d()?)
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

    pub fn left_side(
        &self,
    ) -> Result<crate::inventory::InventoryHolder<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/inventory/InventoryHolder;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getLeftSide", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::InventoryHolder::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn right_side(
        &self,
    ) -> Result<crate::inventory::InventoryHolder<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/inventory/InventoryHolder;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getRightSide", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::InventoryHolder::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn location(&self) -> Result<crate::Location<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/Location;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getLocation", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::Location::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    // SUPER CLASS: Object

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::inventory::InventoryHolder<'mc>> for DoubleChest<'mc> {
    fn into(self) -> crate::inventory::InventoryHolder<'mc> {
        crate::inventory::InventoryHolder::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting DoubleChest into crate::inventory::InventoryHolder")
    }
}
pub enum BlockSupport<'mc> {
    Full { inner: BlockSupportStruct<'mc> },
    Center { inner: BlockSupportStruct<'mc> },
    Rigid { inner: BlockSupportStruct<'mc> },
}
impl<'mc> std::fmt::Display for BlockSupport<'mc> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BlockSupport::Full { .. } => f.write_str("FULL"),
            BlockSupport::Center { .. } => f.write_str("CENTER"),
            BlockSupport::Rigid { .. } => f.write_str("RIGID"),
        }
    }
}

impl<'mc> BlockSupport<'mc> {
    pub fn value_of(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<String>,
    ) -> Result<BlockSupport<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(env.new_string(arg0.into())?);
        let cls = env.find_class("org/bukkit/block/BlockSupport");
        let cls = env.translate_error_with_class(cls)?;
        let res = env.call_static_method(
            cls,
            "valueOf",
            "(Ljava/lang/String;)Lorg/bukkit/block/BlockSupport;",
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
            "FULL" => Ok(BlockSupport::Full {
                inner: BlockSupportStruct::from_raw(env, obj)?,
            }),
            "CENTER" => Ok(BlockSupport::Center {
                inner: BlockSupportStruct::from_raw(env, obj)?,
            }),
            "RIGID" => Ok(BlockSupport::Rigid {
                inner: BlockSupportStruct::from_raw(env, obj)?,
            }),

            _ => Err(eyre::eyre!("String gaven for variant was invalid").into()),
        }
    }
}

#[repr(C)]
pub struct BlockSupportStruct<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for BlockSupport<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        match self {
            Self::Full { inner } => inner.0.clone(),
            Self::Center { inner } => inner.0.clone(),
            Self::Rigid { inner } => inner.0.clone(),
        }
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        match self {
            Self::Full { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Center { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Rigid { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
        }
    }
}
impl<'mc> JNIInstantiatable<'mc> for BlockSupport<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate BlockSupport from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/block/BlockSupport")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a BlockSupport object, got {}",
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
                "FULL" => Ok(BlockSupport::Full {
                    inner: BlockSupportStruct::from_raw(env, obj)?,
                }),
                "CENTER" => Ok(BlockSupport::Center {
                    inner: BlockSupportStruct::from_raw(env, obj)?,
                }),
                "RIGID" => Ok(BlockSupport::Rigid {
                    inner: BlockSupportStruct::from_raw(env, obj)?,
                }),
                _ => Err(eyre::eyre!("String gaven for variant was invalid").into()),
            }
        }
    }
}

impl<'mc> JNIRaw<'mc> for BlockSupportStruct<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for BlockSupportStruct<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate BlockSupportStruct from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/block/BlockSupport")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a BlockSupportStruct object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> BlockSupportStruct<'mc> {
    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
pub mod banner;
pub mod data;
pub mod sign;
pub mod structure;
