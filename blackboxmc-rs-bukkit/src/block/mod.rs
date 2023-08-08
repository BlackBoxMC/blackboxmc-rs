#![allow(deprecated)]
use blackboxmc_general::JNIRaw;
use color_eyre::eyre::Result;
/// Represents a captured state of a chest.
///
/// This is a representation of an abstract class.
pub struct Chest<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> Chest<'mc> {
    pub fn from_raw(
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
    /// Gets the inventory of the chest block represented by this block state.
    /// <p>If the chest is a double chest, it returns just the portion of the inventory linked to the half of the chest corresponding to this block state.</p>
    /// <p>If the block was changed to a different type in the meantime, the returned inventory might no longer be valid.</p>
    /// <p>If this block state is not placed this will return the captured inventory snapshot instead.</p>
    pub fn block_inventory(
        &mut self,
    ) -> Result<crate::inventory::Inventory<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getBlockInventory",
            "()Lorg/bukkit/inventory/Inventory;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::Inventory::from_raw(&self.jni_ref(), unsafe {
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

    pub fn snapshot_inventory(
        &mut self,
    ) -> Result<crate::inventory::Inventory<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getSnapshotInventory",
            "()Lorg/bukkit/inventory/Inventory;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::Inventory::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn persistent_data_container(
        &mut self,
    ) -> Result<crate::persistence::PersistentDataContainer<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getPersistentDataContainer",
            "()Lorg/bukkit/persistence/PersistentDataContainer;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::persistence::PersistentDataContainer::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn set_type(
        &mut self,
        arg0: impl Into<&'mc crate::Material<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setType",
            "(Lorg/bukkit/Material;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn world(&mut self) -> Result<crate::World<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getWorld", "()Lorg/bukkit/World;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        crate::World::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn data(
        &mut self,
    ) -> Result<crate::material::MaterialData<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getData",
            "()Lorg/bukkit/material/MaterialData;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::material::MaterialData::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Gets the inventory of the chest block represented by this block state.
    /// <p>If the chest is a double chest, it returns just the portion of the inventory linked to the half of the chest corresponding to this block state.</p>
    /// <p>If the block was changed to a different type in the meantime, the returned inventory might no longer be valid.</p>
    /// <p>If this block state is not placed this will return the captured inventory snapshot instead.</p>
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

    pub fn block_data(
        &mut self,
    ) -> Result<crate::block::data::BlockData<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getBlockData",
            "()Lorg/bukkit/block/data/BlockData;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::block::data::BlockData::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn light_level(&mut self) -> Result<i8, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getLightLevel", "()B", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.b().unwrap())
    }

    pub fn x(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getX", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }

    pub fn y(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getY", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }

    pub fn z(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getZ", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }

    pub fn chunk(&mut self) -> Result<crate::Chunk<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getChunk", "()Lorg/bukkit/Chunk;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        crate::Chunk::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn set_data(
        &mut self,
        arg0: impl Into<&'mc crate::material::MaterialData<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setData",
            "(Lorg/bukkit/material/MaterialData;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn set_block_data(
        &mut self,
        arg0: impl Into<&'mc crate::block::data::BlockData<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setBlockData",
            "(Lorg/bukkit/block/data/BlockData;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    #[deprecated]

    pub fn raw_data(&mut self) -> Result<i8, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getRawData", "()B", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.b().unwrap())
    }
    #[deprecated]

    pub fn set_raw_data(&mut self, arg0: i8) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Byte(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setRawData",
            "(B)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn is_placed(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isPlaced", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn update(
        &mut self,
        arg0: std::option::Option<bool>,
        arg1: std::option::Option<bool>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        // -1
        let val_1 = jni::objects::JValueGen::Bool(arg0.unwrap().into());
        // -1
        let val_2 = jni::objects::JValueGen::Bool(arg1.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "update",
            "(ZZ)Z",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn location(&mut self) -> Result<crate::Location<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getLocation",
            "()Lorg/bukkit/Location;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::Location::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn get_type(&mut self) -> Result<crate::Material<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getType",
            "()Lorg/bukkit/Material;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant =
            self.jni_ref()
                .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
        let variant_str = self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::Material::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::Material::from_string(variant_str).unwrap(),
        )
    }

    pub fn set_metadata(
        &mut self,
        arg0: impl Into<&'mc String>,
        arg1: impl Into<&'mc crate::metadata::MetadataValue<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let val_2 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setMetadata",
            "(Ljava/lang/String;Lorg/bukkit/metadata/MetadataValue;)V",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn get_metadata(
        &mut self,
        arg0: impl Into<&'mc String>,
    ) -> Result<Vec<crate::metadata::MetadataValue<'mc>>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getMetadata",
            "(Ljava/lang/String;)Ljava/util/List;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let mut list = blackboxmc_java::JavaList::from_raw(&self.jni_ref(), res.l()?)?;
        let size = list.size()?;
        for i in 0..=size {
            let obj = list.get(i)?;
            new_vec.push(crate::metadata::MetadataValue::from_raw(
                &self.jni_ref(),
                obj,
            )?);
        }
        Ok(new_vec)
    }

    pub fn has_metadata(
        &mut self,
        arg0: impl Into<&'mc String>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "hasMetadata",
            "(Ljava/lang/String;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn remove_metadata(
        &mut self,
        arg0: impl Into<&'mc String>,
        arg1: impl Into<&'mc crate::plugin::Plugin<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let val_2 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeMetadata",
            "(Ljava/lang/String;Lorg/bukkit/plugin/Plugin;)V",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn is_locked(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isLocked", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn lock(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getLock", "()Ljava/lang/String;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }

    pub fn set_lock(
        &mut self,
        arg0: impl Into<&'mc String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setLock",
            "(Ljava/lang/String;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn custom_name(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getCustomName",
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

    pub fn set_custom_name(
        &mut self,
        arg0: impl Into<&'mc String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setCustomName",
            "(Ljava/lang/String;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn seed(&mut self) -> Result<i64, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getSeed", "()J", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.j().unwrap())
    }

    pub fn set_seed(&mut self, arg0: i64) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Long(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setSeed",
            "(J)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn set_loot_table(
        &mut self,
        arg0: impl Into<&'mc crate::loot::LootTable<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setLootTable",
            "(Lorg/bukkit/loot/LootTable;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn loot_table(
        &mut self,
    ) -> Result<crate::loot::LootTable<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getLootTable",
            "()Lorg/bukkit/loot/LootTable;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::loot::LootTable::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn close(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "close", "()V", &[]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn open(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "open", "()V", &[]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
}
impl<'mc> JNIRaw<'mc> for Chest<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Into<crate::block::Container<'mc>> for Chest<'mc> {
    fn into(self) -> crate::block::Container<'mc> {
        crate::block::Container::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
impl<'mc> Into<crate::loot::Lootable<'mc>> for Chest<'mc> {
    fn into(self) -> crate::loot::Lootable<'mc> {
        crate::loot::Lootable::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
impl<'mc> Into<crate::block::Lidded<'mc>> for Chest<'mc> {
    fn into(self) -> crate::block::Lidded<'mc> {
        crate::block::Lidded::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
pub enum BlockFaceEnum {
    North,
    East,
    South,
    West,
    Up,
    Down,
    NorthEast,
    NorthWest,
    SouthEast,
    SouthWest,
    WestNorthWest,
    NorthNorthWest,
    NorthNorthEast,
    EastNorthEast,
    EastSouthEast,
    SouthSouthEast,
    SouthSouthWest,
    WestSouthWest,
    VariantSelf,
}
impl std::fmt::Display for BlockFaceEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            BlockFaceEnum::North => f.write_str("NORTH"),
            BlockFaceEnum::East => f.write_str("EAST"),
            BlockFaceEnum::South => f.write_str("SOUTH"),
            BlockFaceEnum::West => f.write_str("WEST"),
            BlockFaceEnum::Up => f.write_str("UP"),
            BlockFaceEnum::Down => f.write_str("DOWN"),
            BlockFaceEnum::NorthEast => f.write_str("NORTH_EAST"),
            BlockFaceEnum::NorthWest => f.write_str("NORTH_WEST"),
            BlockFaceEnum::SouthEast => f.write_str("SOUTH_EAST"),
            BlockFaceEnum::SouthWest => f.write_str("SOUTH_WEST"),
            BlockFaceEnum::WestNorthWest => f.write_str("WEST_NORTH_WEST"),
            BlockFaceEnum::NorthNorthWest => f.write_str("NORTH_NORTH_WEST"),
            BlockFaceEnum::NorthNorthEast => f.write_str("NORTH_NORTH_EAST"),
            BlockFaceEnum::EastNorthEast => f.write_str("EAST_NORTH_EAST"),
            BlockFaceEnum::EastSouthEast => f.write_str("EAST_SOUTH_EAST"),
            BlockFaceEnum::SouthSouthEast => f.write_str("SOUTH_SOUTH_EAST"),
            BlockFaceEnum::SouthSouthWest => f.write_str("SOUTH_SOUTH_WEST"),
            BlockFaceEnum::WestSouthWest => f.write_str("WEST_SOUTH_WEST"),
            BlockFaceEnum::VariantSelf => f.write_str("SELF"),
        }
    }
}
pub struct BlockFace<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
    pub BlockFaceEnum,
);
impl<'mc> std::ops::Deref for BlockFace<'mc> {
    type Target = BlockFaceEnum;
    fn deref(&self) -> &Self::Target {
        return &self.2;
    }
}
impl<'mc> JNIRaw<'mc> for BlockFace<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> BlockFace<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
        e: BlockFaceEnum,
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
            Ok(Self(env.clone(), obj, e))
        }
    }
    pub const NORTH: BlockFaceEnum = BlockFaceEnum::North;
    pub const EAST: BlockFaceEnum = BlockFaceEnum::East;
    pub const SOUTH: BlockFaceEnum = BlockFaceEnum::South;
    pub const WEST: BlockFaceEnum = BlockFaceEnum::West;
    pub const UP: BlockFaceEnum = BlockFaceEnum::Up;
    pub const DOWN: BlockFaceEnum = BlockFaceEnum::Down;
    pub const NORTH_EAST: BlockFaceEnum = BlockFaceEnum::NorthEast;
    pub const NORTH_WEST: BlockFaceEnum = BlockFaceEnum::NorthWest;
    pub const SOUTH_EAST: BlockFaceEnum = BlockFaceEnum::SouthEast;
    pub const SOUTH_WEST: BlockFaceEnum = BlockFaceEnum::SouthWest;
    pub const WEST_NORTH_WEST: BlockFaceEnum = BlockFaceEnum::WestNorthWest;
    pub const NORTH_NORTH_WEST: BlockFaceEnum = BlockFaceEnum::NorthNorthWest;
    pub const NORTH_NORTH_EAST: BlockFaceEnum = BlockFaceEnum::NorthNorthEast;
    pub const EAST_NORTH_EAST: BlockFaceEnum = BlockFaceEnum::EastNorthEast;
    pub const EAST_SOUTH_EAST: BlockFaceEnum = BlockFaceEnum::EastSouthEast;
    pub const SOUTH_SOUTH_EAST: BlockFaceEnum = BlockFaceEnum::SouthSouthEast;
    pub const SOUTH_SOUTH_WEST: BlockFaceEnum = BlockFaceEnum::SouthSouthWest;
    pub const WEST_SOUTH_WEST: BlockFaceEnum = BlockFaceEnum::WestSouthWest;
    pub const SELF: BlockFaceEnum = BlockFaceEnum::VariantSelf;
    pub fn from_string(str: String) -> std::option::Option<BlockFaceEnum> {
        match str.as_str() {
            "NORTH" => Some(BlockFaceEnum::North),
            "EAST" => Some(BlockFaceEnum::East),
            "SOUTH" => Some(BlockFaceEnum::South),
            "WEST" => Some(BlockFaceEnum::West),
            "UP" => Some(BlockFaceEnum::Up),
            "DOWN" => Some(BlockFaceEnum::Down),
            "NORTH_EAST" => Some(BlockFaceEnum::NorthEast),
            "NORTH_WEST" => Some(BlockFaceEnum::NorthWest),
            "SOUTH_EAST" => Some(BlockFaceEnum::SouthEast),
            "SOUTH_WEST" => Some(BlockFaceEnum::SouthWest),
            "WEST_NORTH_WEST" => Some(BlockFaceEnum::WestNorthWest),
            "NORTH_NORTH_WEST" => Some(BlockFaceEnum::NorthNorthWest),
            "NORTH_NORTH_EAST" => Some(BlockFaceEnum::NorthNorthEast),
            "EAST_NORTH_EAST" => Some(BlockFaceEnum::EastNorthEast),
            "EAST_SOUTH_EAST" => Some(BlockFaceEnum::EastSouthEast),
            "SOUTH_SOUTH_EAST" => Some(BlockFaceEnum::SouthSouthEast),
            "SOUTH_SOUTH_WEST" => Some(BlockFaceEnum::SouthSouthWest),
            "WEST_SOUTH_WEST" => Some(BlockFaceEnum::WestSouthWest),
            "SELF" => Some(BlockFaceEnum::VariantSelf),
            _ => None,
        }
    }
}
/// Represents a captured state of a (possibly inverted) daylight detector.
///
/// This is a representation of an abstract class.
pub struct DaylightDetector<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> DaylightDetector<'mc> {
    pub fn from_raw(
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

    pub fn persistent_data_container(
        &mut self,
    ) -> Result<crate::persistence::PersistentDataContainer<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getPersistentDataContainer",
            "()Lorg/bukkit/persistence/PersistentDataContainer;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::persistence::PersistentDataContainer::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn set_type(
        &mut self,
        arg0: impl Into<&'mc crate::Material<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setType",
            "(Lorg/bukkit/Material;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn world(&mut self) -> Result<crate::World<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getWorld", "()Lorg/bukkit/World;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        crate::World::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn data(
        &mut self,
    ) -> Result<crate::material::MaterialData<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getData",
            "()Lorg/bukkit/material/MaterialData;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::material::MaterialData::from_raw(&self.jni_ref(), unsafe {
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

    pub fn block_data(
        &mut self,
    ) -> Result<crate::block::data::BlockData<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getBlockData",
            "()Lorg/bukkit/block/data/BlockData;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::block::data::BlockData::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn light_level(&mut self) -> Result<i8, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getLightLevel", "()B", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.b().unwrap())
    }

    pub fn x(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getX", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }

    pub fn y(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getY", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }

    pub fn z(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getZ", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }

    pub fn chunk(&mut self) -> Result<crate::Chunk<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getChunk", "()Lorg/bukkit/Chunk;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        crate::Chunk::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn set_data(
        &mut self,
        arg0: impl Into<&'mc crate::material::MaterialData<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setData",
            "(Lorg/bukkit/material/MaterialData;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn set_block_data(
        &mut self,
        arg0: impl Into<&'mc crate::block::data::BlockData<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setBlockData",
            "(Lorg/bukkit/block/data/BlockData;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    #[deprecated]

    pub fn raw_data(&mut self) -> Result<i8, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getRawData", "()B", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.b().unwrap())
    }
    #[deprecated]

    pub fn set_raw_data(&mut self, arg0: i8) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Byte(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setRawData",
            "(B)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn is_placed(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isPlaced", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn update(
        &mut self,
        arg0: std::option::Option<bool>,
        arg1: std::option::Option<bool>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        // -1
        let val_1 = jni::objects::JValueGen::Bool(arg0.unwrap().into());
        // -1
        let val_2 = jni::objects::JValueGen::Bool(arg1.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "update",
            "(ZZ)Z",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn location(&mut self) -> Result<crate::Location<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getLocation",
            "()Lorg/bukkit/Location;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::Location::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn get_type(&mut self) -> Result<crate::Material<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getType",
            "()Lorg/bukkit/Material;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant =
            self.jni_ref()
                .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
        let variant_str = self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::Material::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::Material::from_string(variant_str).unwrap(),
        )
    }

    pub fn set_metadata(
        &mut self,
        arg0: impl Into<&'mc String>,
        arg1: impl Into<&'mc crate::metadata::MetadataValue<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let val_2 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setMetadata",
            "(Ljava/lang/String;Lorg/bukkit/metadata/MetadataValue;)V",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn get_metadata(
        &mut self,
        arg0: impl Into<&'mc String>,
    ) -> Result<Vec<crate::metadata::MetadataValue<'mc>>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getMetadata",
            "(Ljava/lang/String;)Ljava/util/List;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let mut list = blackboxmc_java::JavaList::from_raw(&self.jni_ref(), res.l()?)?;
        let size = list.size()?;
        for i in 0..=size {
            let obj = list.get(i)?;
            new_vec.push(crate::metadata::MetadataValue::from_raw(
                &self.jni_ref(),
                obj,
            )?);
        }
        Ok(new_vec)
    }

    pub fn has_metadata(
        &mut self,
        arg0: impl Into<&'mc String>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "hasMetadata",
            "(Ljava/lang/String;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn remove_metadata(
        &mut self,
        arg0: impl Into<&'mc String>,
        arg1: impl Into<&'mc crate::plugin::Plugin<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let val_2 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeMetadata",
            "(Ljava/lang/String;Lorg/bukkit/plugin/Plugin;)V",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
}
impl<'mc> JNIRaw<'mc> for DaylightDetector<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Into<crate::block::TileState<'mc>> for DaylightDetector<'mc> {
    fn into(self) -> crate::block::TileState<'mc> {
        crate::block::TileState::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
/// Represents a block (usually a container) that may be locked. When a lock is active an item with a name corresponding to the key will be required to open this block.
///
/// This is a representation of an abstract class.
pub struct Lockable<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> Lockable<'mc> {
    pub fn from_raw(
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
    /// Checks if the container has a valid (non empty) key.
    pub fn is_locked(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isLocked", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    /// Gets the key needed to access the container.
    pub fn lock(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getLock", "()Ljava/lang/String;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    /// Sets the key required to access this container. Set to null (or empty string) to remove key.
    pub fn set_lock(
        &mut self,
        arg0: impl Into<&'mc String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setLock",
            "(Ljava/lang/String;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
}
impl<'mc> JNIRaw<'mc> for Lockable<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
/// Represents a captured state of a lectern.
///
/// This is a representation of an abstract class.
pub struct Lectern<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> Lectern<'mc> {
    pub fn from_raw(
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
    /// <span class="descfrm-type-label">Description copied from interface:&nbsp;<code><a href="../inventory/InventoryHolder.html#getInventory()">InventoryHolder</a></code></span>
    /// Get the object's inventory.
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

    pub fn snapshot_inventory(
        &mut self,
    ) -> Result<crate::inventory::Inventory<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getSnapshotInventory",
            "()Lorg/bukkit/inventory/Inventory;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::Inventory::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Get the current lectern page.
    pub fn page(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getPage", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }
    /// Set the current lectern page. If the page is greater than the number of pages of the book currently in the inventory, then behavior is undefined.
    pub fn set_page(&mut self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setPage",
            "(I)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn persistent_data_container(
        &mut self,
    ) -> Result<crate::persistence::PersistentDataContainer<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getPersistentDataContainer",
            "()Lorg/bukkit/persistence/PersistentDataContainer;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::persistence::PersistentDataContainer::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn set_type(
        &mut self,
        arg0: impl Into<&'mc crate::Material<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setType",
            "(Lorg/bukkit/Material;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn world(&mut self) -> Result<crate::World<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getWorld", "()Lorg/bukkit/World;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        crate::World::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn data(
        &mut self,
    ) -> Result<crate::material::MaterialData<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getData",
            "()Lorg/bukkit/material/MaterialData;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::material::MaterialData::from_raw(&self.jni_ref(), unsafe {
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

    pub fn block_data(
        &mut self,
    ) -> Result<crate::block::data::BlockData<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getBlockData",
            "()Lorg/bukkit/block/data/BlockData;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::block::data::BlockData::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn light_level(&mut self) -> Result<i8, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getLightLevel", "()B", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.b().unwrap())
    }

    pub fn x(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getX", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }

    pub fn y(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getY", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }

    pub fn z(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getZ", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }

    pub fn chunk(&mut self) -> Result<crate::Chunk<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getChunk", "()Lorg/bukkit/Chunk;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        crate::Chunk::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn set_data(
        &mut self,
        arg0: impl Into<&'mc crate::material::MaterialData<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setData",
            "(Lorg/bukkit/material/MaterialData;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn set_block_data(
        &mut self,
        arg0: impl Into<&'mc crate::block::data::BlockData<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setBlockData",
            "(Lorg/bukkit/block/data/BlockData;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    #[deprecated]

    pub fn raw_data(&mut self) -> Result<i8, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getRawData", "()B", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.b().unwrap())
    }
    #[deprecated]

    pub fn set_raw_data(&mut self, arg0: i8) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Byte(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setRawData",
            "(B)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn is_placed(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isPlaced", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn update(
        &mut self,
        arg0: std::option::Option<bool>,
        arg1: std::option::Option<bool>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        // -1
        let val_1 = jni::objects::JValueGen::Bool(arg0.unwrap().into());
        // -1
        let val_2 = jni::objects::JValueGen::Bool(arg1.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "update",
            "(ZZ)Z",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn location(&mut self) -> Result<crate::Location<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getLocation",
            "()Lorg/bukkit/Location;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::Location::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn get_type(&mut self) -> Result<crate::Material<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getType",
            "()Lorg/bukkit/Material;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant =
            self.jni_ref()
                .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
        let variant_str = self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::Material::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::Material::from_string(variant_str).unwrap(),
        )
    }

    pub fn set_metadata(
        &mut self,
        arg0: impl Into<&'mc String>,
        arg1: impl Into<&'mc crate::metadata::MetadataValue<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let val_2 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setMetadata",
            "(Ljava/lang/String;Lorg/bukkit/metadata/MetadataValue;)V",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn get_metadata(
        &mut self,
        arg0: impl Into<&'mc String>,
    ) -> Result<Vec<crate::metadata::MetadataValue<'mc>>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getMetadata",
            "(Ljava/lang/String;)Ljava/util/List;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let mut list = blackboxmc_java::JavaList::from_raw(&self.jni_ref(), res.l()?)?;
        let size = list.size()?;
        for i in 0..=size {
            let obj = list.get(i)?;
            new_vec.push(crate::metadata::MetadataValue::from_raw(
                &self.jni_ref(),
                obj,
            )?);
        }
        Ok(new_vec)
    }

    pub fn has_metadata(
        &mut self,
        arg0: impl Into<&'mc String>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "hasMetadata",
            "(Ljava/lang/String;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn remove_metadata(
        &mut self,
        arg0: impl Into<&'mc String>,
        arg1: impl Into<&'mc crate::plugin::Plugin<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let val_2 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeMetadata",
            "(Ljava/lang/String;Lorg/bukkit/plugin/Plugin;)V",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
}
impl<'mc> JNIRaw<'mc> for Lectern<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Into<crate::block::TileState<'mc>> for Lectern<'mc> {
    fn into(self) -> crate::block::TileState<'mc> {
        crate::block::TileState::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
impl<'mc> Into<crate::inventory::BlockInventoryHolder<'mc>> for Lectern<'mc> {
    fn into(self) -> crate::inventory::BlockInventoryHolder<'mc> {
        crate::inventory::BlockInventoryHolder::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
/// Represents a captured state of a creature spawner.
///
/// This is a representation of an abstract class.
pub struct CreatureSpawner<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> CreatureSpawner<'mc> {
    pub fn from_raw(
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
    /// Get the spawner's creature type.
    pub fn spawned_type(
        &mut self,
    ) -> Result<crate::entity::EntityType<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getSpawnedType",
            "()Lorg/bukkit/entity/EntityType;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant =
            self.jni_ref()
                .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
        let variant_str = self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::entity::EntityType::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::entity::EntityType::from_string(variant_str).unwrap(),
        )
    }
    /// Set the spawner's creature type.
    pub fn set_spawned_type(
        &mut self,
        arg0: impl Into<&'mc crate::entity::EntityType<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setSpawnedType",
            "(Lorg/bukkit/entity/EntityType;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Get the spawner's delay.
    ///
    /// This is the delay, in ticks, until the spawner will spawn its next mob.
    pub fn delay(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getDelay", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }
    /// Set the spawner's delay.
    ///
    /// If set to -1, the spawn delay will be reset to a random value between <a href="#getMinSpawnDelay()"><code>getMinSpawnDelay()</code></a> and <a href="#getMaxSpawnDelay()"><code>getMaxSpawnDelay()</code></a>.
    pub fn set_delay(&mut self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setDelay",
            "(I)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    #[deprecated]
    /// <span class="deprecated-label">Deprecated.</span>
    /// <div class="deprecation-comment">
    /// magic value, use <a href="#setSpawnedType(org.bukkit.entity.EntityType)"><code>setSpawnedType(org.bukkit.entity.EntityType)</code></a>.
    /// </div>
    /// magic value, use <a href="#setSpawnedType(org.bukkit.entity.EntityType)"><code>setSpawnedType(org.bukkit.entity.EntityType)</code></a>.
    ///
    /// Set the spawner mob type.
    pub fn set_creature_type_by_name(
        &mut self,
        arg0: impl Into<&'mc String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setCreatureTypeByName",
            "(Ljava/lang/String;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    #[deprecated]
    /// <span class="deprecated-label">Deprecated.</span>
    /// <div class="deprecation-comment">
    /// magic value, use <a href="#getSpawnedType()"><code>getSpawnedType()</code></a>.
    /// </div>
    /// magic value, use <a href="#getSpawnedType()"><code>getSpawnedType()</code></a>.
    ///
    /// Get the spawner's creature type.
    pub fn creature_type_name(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getCreatureTypeName",
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
    /// The minimum spawn delay amount (in ticks).
    ///
    /// This value is used when the spawner resets its delay (for any reason). It will choose a random number between <a href="#getMinSpawnDelay()"><code>getMinSpawnDelay()</code></a> and <a href="#getMaxSpawnDelay()"><code>getMaxSpawnDelay()</code></a> for its next <a href="#getDelay()"><code>getDelay()</code></a>.
    ///
    /// Default value is 200 ticks.
    pub fn min_spawn_delay(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getMinSpawnDelay", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }
    /// Set the minimum spawn delay amount (in ticks).
    pub fn set_min_spawn_delay(&mut self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setMinSpawnDelay",
            "(I)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// The maximum spawn delay amount (in ticks).
    ///
    /// This value is used when the spawner resets its delay (for any reason). It will choose a random number between <a href="#getMinSpawnDelay()"><code>getMinSpawnDelay()</code></a> and <a href="#getMaxSpawnDelay()"><code>getMaxSpawnDelay()</code></a> for its next <a href="#getDelay()"><code>getDelay()</code></a>.
    ///
    /// This value <b>must</b> be greater than 0 and less than or equal to <a href="#getMaxSpawnDelay()"><code>getMaxSpawnDelay()</code></a>.
    ///
    /// Default value is 800 ticks.
    pub fn max_spawn_delay(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getMaxSpawnDelay", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }
    /// Set the maximum spawn delay amount (in ticks).
    ///
    /// This value <b>must</b> be greater than 0, as well as greater than or equal to <a href="#getMinSpawnDelay()"><code>getMinSpawnDelay()</code></a>
    pub fn set_max_spawn_delay(&mut self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setMaxSpawnDelay",
            "(I)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Get how many mobs attempt to spawn.
    ///
    /// Default value is 4.
    pub fn spawn_count(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getSpawnCount", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }
    /// Set how many mobs attempt to spawn.
    pub fn set_spawn_count(&mut self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setSpawnCount",
            "(I)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Set the new maximum amount of similar entities that are allowed to be within spawning range of this spawner.
    ///
    /// If more than the maximum number of entities are within range, the spawner will not spawn and try again with a new <a href="#getDelay()"><code>getDelay()</code></a>.
    ///
    /// Default value is 16.
    pub fn max_nearby_entities(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getMaxNearbyEntities", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }
    /// Set the maximum number of similar entities that are allowed to be within spawning range of this spawner.
    ///
    /// Similar entities are entities that are of the same <a href="../entity/EntityType.html" title="enum in org.bukkit.entity"><code>EntityType</code></a>
    pub fn set_max_nearby_entities(&mut self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setMaxNearbyEntities",
            "(I)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Get the maximum distance(squared) a player can be in order for this spawner to be active.
    ///
    /// If this value is less than or equal to 0, this spawner is always active (given that there are players online).
    ///
    /// Default value is 16.
    pub fn required_player_range(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getRequiredPlayerRange", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }
    /// Set the maximum distance (squared) a player can be in order for this spawner to be active.
    ///
    /// Setting this value to less than or equal to 0 will make this spawner always active (given that there are players online).
    pub fn set_required_player_range(
        &mut self,
        arg0: i32,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setRequiredPlayerRange",
            "(I)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Get the radius around which the spawner will attempt to spawn mobs in.
    ///
    /// This area is square, includes the block the spawner is in, and is centered on the spawner's x,z coordinates - not the spawner itself.
    ///
    /// It is 2 blocks high, centered on the spawner's y-coordinate (its bottom); thus allowing mobs to spawn as high as its top surface and as low as 1 block below its bottom surface.
    ///
    /// Default value is 4.
    pub fn spawn_range(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getSpawnRange", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }
    /// Set the new spawn range.
    ///
    pub fn set_spawn_range(&mut self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setSpawnRange",
            "(I)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn persistent_data_container(
        &mut self,
    ) -> Result<crate::persistence::PersistentDataContainer<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getPersistentDataContainer",
            "()Lorg/bukkit/persistence/PersistentDataContainer;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::persistence::PersistentDataContainer::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn set_type(
        &mut self,
        arg0: impl Into<&'mc crate::Material<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setType",
            "(Lorg/bukkit/Material;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn world(&mut self) -> Result<crate::World<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getWorld", "()Lorg/bukkit/World;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        crate::World::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn data(
        &mut self,
    ) -> Result<crate::material::MaterialData<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getData",
            "()Lorg/bukkit/material/MaterialData;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::material::MaterialData::from_raw(&self.jni_ref(), unsafe {
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

    pub fn block_data(
        &mut self,
    ) -> Result<crate::block::data::BlockData<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getBlockData",
            "()Lorg/bukkit/block/data/BlockData;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::block::data::BlockData::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn light_level(&mut self) -> Result<i8, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getLightLevel", "()B", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.b().unwrap())
    }

    pub fn x(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getX", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }

    pub fn y(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getY", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }

    pub fn z(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getZ", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }

    pub fn chunk(&mut self) -> Result<crate::Chunk<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getChunk", "()Lorg/bukkit/Chunk;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        crate::Chunk::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn set_data(
        &mut self,
        arg0: impl Into<&'mc crate::material::MaterialData<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setData",
            "(Lorg/bukkit/material/MaterialData;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn set_block_data(
        &mut self,
        arg0: impl Into<&'mc crate::block::data::BlockData<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setBlockData",
            "(Lorg/bukkit/block/data/BlockData;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    #[deprecated]

    pub fn raw_data(&mut self) -> Result<i8, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getRawData", "()B", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.b().unwrap())
    }
    #[deprecated]

    pub fn set_raw_data(&mut self, arg0: i8) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Byte(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setRawData",
            "(B)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn is_placed(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isPlaced", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn update(
        &mut self,
        arg0: std::option::Option<bool>,
        arg1: std::option::Option<bool>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        // -1
        let val_1 = jni::objects::JValueGen::Bool(arg0.unwrap().into());
        // -1
        let val_2 = jni::objects::JValueGen::Bool(arg1.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "update",
            "(ZZ)Z",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn location(&mut self) -> Result<crate::Location<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getLocation",
            "()Lorg/bukkit/Location;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::Location::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn get_type(&mut self) -> Result<crate::Material<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getType",
            "()Lorg/bukkit/Material;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant =
            self.jni_ref()
                .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
        let variant_str = self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::Material::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::Material::from_string(variant_str).unwrap(),
        )
    }

    pub fn set_metadata(
        &mut self,
        arg0: impl Into<&'mc String>,
        arg1: impl Into<&'mc crate::metadata::MetadataValue<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let val_2 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setMetadata",
            "(Ljava/lang/String;Lorg/bukkit/metadata/MetadataValue;)V",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn get_metadata(
        &mut self,
        arg0: impl Into<&'mc String>,
    ) -> Result<Vec<crate::metadata::MetadataValue<'mc>>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getMetadata",
            "(Ljava/lang/String;)Ljava/util/List;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let mut list = blackboxmc_java::JavaList::from_raw(&self.jni_ref(), res.l()?)?;
        let size = list.size()?;
        for i in 0..=size {
            let obj = list.get(i)?;
            new_vec.push(crate::metadata::MetadataValue::from_raw(
                &self.jni_ref(),
                obj,
            )?);
        }
        Ok(new_vec)
    }

    pub fn has_metadata(
        &mut self,
        arg0: impl Into<&'mc String>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "hasMetadata",
            "(Ljava/lang/String;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn remove_metadata(
        &mut self,
        arg0: impl Into<&'mc String>,
        arg1: impl Into<&'mc crate::plugin::Plugin<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let val_2 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeMetadata",
            "(Ljava/lang/String;Lorg/bukkit/plugin/Plugin;)V",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
}
impl<'mc> JNIRaw<'mc> for CreatureSpawner<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Into<crate::block::TileState<'mc>> for CreatureSpawner<'mc> {
    fn into(self) -> crate::block::TileState<'mc> {
        crate::block::TileState::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
/// Represents a captured state of an on / off comparator.
///
/// This is a representation of an abstract class.
pub struct Comparator<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> Comparator<'mc> {
    pub fn from_raw(
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

    pub fn persistent_data_container(
        &mut self,
    ) -> Result<crate::persistence::PersistentDataContainer<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getPersistentDataContainer",
            "()Lorg/bukkit/persistence/PersistentDataContainer;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::persistence::PersistentDataContainer::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn set_type(
        &mut self,
        arg0: impl Into<&'mc crate::Material<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setType",
            "(Lorg/bukkit/Material;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn world(&mut self) -> Result<crate::World<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getWorld", "()Lorg/bukkit/World;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        crate::World::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn data(
        &mut self,
    ) -> Result<crate::material::MaterialData<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getData",
            "()Lorg/bukkit/material/MaterialData;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::material::MaterialData::from_raw(&self.jni_ref(), unsafe {
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

    pub fn block_data(
        &mut self,
    ) -> Result<crate::block::data::BlockData<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getBlockData",
            "()Lorg/bukkit/block/data/BlockData;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::block::data::BlockData::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn light_level(&mut self) -> Result<i8, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getLightLevel", "()B", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.b().unwrap())
    }

    pub fn x(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getX", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }

    pub fn y(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getY", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }

    pub fn z(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getZ", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }

    pub fn chunk(&mut self) -> Result<crate::Chunk<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getChunk", "()Lorg/bukkit/Chunk;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        crate::Chunk::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn set_data(
        &mut self,
        arg0: impl Into<&'mc crate::material::MaterialData<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setData",
            "(Lorg/bukkit/material/MaterialData;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn set_block_data(
        &mut self,
        arg0: impl Into<&'mc crate::block::data::BlockData<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setBlockData",
            "(Lorg/bukkit/block/data/BlockData;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    #[deprecated]

    pub fn raw_data(&mut self) -> Result<i8, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getRawData", "()B", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.b().unwrap())
    }
    #[deprecated]

    pub fn set_raw_data(&mut self, arg0: i8) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Byte(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setRawData",
            "(B)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn is_placed(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isPlaced", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn update(
        &mut self,
        arg0: std::option::Option<bool>,
        arg1: std::option::Option<bool>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        // -1
        let val_1 = jni::objects::JValueGen::Bool(arg0.unwrap().into());
        // -1
        let val_2 = jni::objects::JValueGen::Bool(arg1.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "update",
            "(ZZ)Z",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn location(&mut self) -> Result<crate::Location<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getLocation",
            "()Lorg/bukkit/Location;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::Location::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn get_type(&mut self) -> Result<crate::Material<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getType",
            "()Lorg/bukkit/Material;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant =
            self.jni_ref()
                .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
        let variant_str = self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::Material::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::Material::from_string(variant_str).unwrap(),
        )
    }

    pub fn set_metadata(
        &mut self,
        arg0: impl Into<&'mc String>,
        arg1: impl Into<&'mc crate::metadata::MetadataValue<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let val_2 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setMetadata",
            "(Ljava/lang/String;Lorg/bukkit/metadata/MetadataValue;)V",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn get_metadata(
        &mut self,
        arg0: impl Into<&'mc String>,
    ) -> Result<Vec<crate::metadata::MetadataValue<'mc>>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getMetadata",
            "(Ljava/lang/String;)Ljava/util/List;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let mut list = blackboxmc_java::JavaList::from_raw(&self.jni_ref(), res.l()?)?;
        let size = list.size()?;
        for i in 0..=size {
            let obj = list.get(i)?;
            new_vec.push(crate::metadata::MetadataValue::from_raw(
                &self.jni_ref(),
                obj,
            )?);
        }
        Ok(new_vec)
    }

    pub fn has_metadata(
        &mut self,
        arg0: impl Into<&'mc String>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "hasMetadata",
            "(Ljava/lang/String;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn remove_metadata(
        &mut self,
        arg0: impl Into<&'mc String>,
        arg1: impl Into<&'mc crate::plugin::Plugin<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let val_2 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeMetadata",
            "(Ljava/lang/String;Lorg/bukkit/plugin/Plugin;)V",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
}
impl<'mc> JNIRaw<'mc> for Comparator<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Into<crate::block::TileState<'mc>> for Comparator<'mc> {
    fn into(self) -> crate::block::TileState<'mc> {
        crate::block::TileState::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
/// Represents a captured state of an end gateway.
///
/// This is a representation of an abstract class.
pub struct EndGateway<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> EndGateway<'mc> {
    pub fn from_raw(
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
    /// Gets the location that entities are teleported to when entering the gateway portal.
    /// <p>If this block state is not placed the location's world will be null.</p>
    pub fn exit_location(&mut self) -> Result<crate::Location<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getExitLocation",
            "()Lorg/bukkit/Location;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::Location::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Sets the exit location that entities are teleported to when they enter the gateway portal.
    /// <p>If this block state is not placed the location's world has to be null.</p>
    pub fn set_exit_location(
        &mut self,
        arg0: impl Into<&'mc crate::Location<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setExitLocation",
            "(Lorg/bukkit/Location;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Gets whether this gateway will teleport entities directly to the exit location instead of finding a nearby location.
    pub fn is_exact_teleport(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isExactTeleport", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    /// Sets whether this gateway will teleport entities directly to the exit location instead of finding a nearby location.
    pub fn set_exact_teleport(&mut self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        // -2
        let val_1 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setExactTeleport",
            "(Z)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Gets the age in ticks of the gateway.
    ///
    /// If the age is less than 200 ticks a magenta beam will be emitted, whilst if it is a multiple of 2400 ticks a purple beam will be emitted.
    pub fn age(&mut self) -> Result<i64, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getAge", "()J", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.j().unwrap())
    }
    /// Sets the age in ticks of the gateway.
    ///
    /// If the age is less than 200 ticks a magenta beam will be emitted, whilst if it is a multiple of 2400 ticks a purple beam will be emitted.
    pub fn set_age(&mut self, arg0: i64) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Long(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setAge",
            "(J)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn persistent_data_container(
        &mut self,
    ) -> Result<crate::persistence::PersistentDataContainer<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getPersistentDataContainer",
            "()Lorg/bukkit/persistence/PersistentDataContainer;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::persistence::PersistentDataContainer::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn set_type(
        &mut self,
        arg0: impl Into<&'mc crate::Material<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setType",
            "(Lorg/bukkit/Material;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn world(&mut self) -> Result<crate::World<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getWorld", "()Lorg/bukkit/World;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        crate::World::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn data(
        &mut self,
    ) -> Result<crate::material::MaterialData<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getData",
            "()Lorg/bukkit/material/MaterialData;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::material::MaterialData::from_raw(&self.jni_ref(), unsafe {
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

    pub fn block_data(
        &mut self,
    ) -> Result<crate::block::data::BlockData<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getBlockData",
            "()Lorg/bukkit/block/data/BlockData;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::block::data::BlockData::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn light_level(&mut self) -> Result<i8, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getLightLevel", "()B", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.b().unwrap())
    }

    pub fn x(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getX", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }

    pub fn y(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getY", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }

    pub fn z(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getZ", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }

    pub fn chunk(&mut self) -> Result<crate::Chunk<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getChunk", "()Lorg/bukkit/Chunk;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        crate::Chunk::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn set_data(
        &mut self,
        arg0: impl Into<&'mc crate::material::MaterialData<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setData",
            "(Lorg/bukkit/material/MaterialData;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn set_block_data(
        &mut self,
        arg0: impl Into<&'mc crate::block::data::BlockData<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setBlockData",
            "(Lorg/bukkit/block/data/BlockData;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    #[deprecated]

    pub fn raw_data(&mut self) -> Result<i8, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getRawData", "()B", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.b().unwrap())
    }
    #[deprecated]

    pub fn set_raw_data(&mut self, arg0: i8) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Byte(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setRawData",
            "(B)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn is_placed(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isPlaced", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn update(
        &mut self,
        arg0: std::option::Option<bool>,
        arg1: std::option::Option<bool>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        // -1
        let val_1 = jni::objects::JValueGen::Bool(arg0.unwrap().into());
        // -1
        let val_2 = jni::objects::JValueGen::Bool(arg1.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "update",
            "(ZZ)Z",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn location(&mut self) -> Result<crate::Location<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getLocation",
            "()Lorg/bukkit/Location;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::Location::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn get_type(&mut self) -> Result<crate::Material<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getType",
            "()Lorg/bukkit/Material;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant =
            self.jni_ref()
                .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
        let variant_str = self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::Material::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::Material::from_string(variant_str).unwrap(),
        )
    }

    pub fn set_metadata(
        &mut self,
        arg0: impl Into<&'mc String>,
        arg1: impl Into<&'mc crate::metadata::MetadataValue<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let val_2 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setMetadata",
            "(Ljava/lang/String;Lorg/bukkit/metadata/MetadataValue;)V",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn get_metadata(
        &mut self,
        arg0: impl Into<&'mc String>,
    ) -> Result<Vec<crate::metadata::MetadataValue<'mc>>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getMetadata",
            "(Ljava/lang/String;)Ljava/util/List;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let mut list = blackboxmc_java::JavaList::from_raw(&self.jni_ref(), res.l()?)?;
        let size = list.size()?;
        for i in 0..=size {
            let obj = list.get(i)?;
            new_vec.push(crate::metadata::MetadataValue::from_raw(
                &self.jni_ref(),
                obj,
            )?);
        }
        Ok(new_vec)
    }

    pub fn has_metadata(
        &mut self,
        arg0: impl Into<&'mc String>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "hasMetadata",
            "(Ljava/lang/String;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn remove_metadata(
        &mut self,
        arg0: impl Into<&'mc String>,
        arg1: impl Into<&'mc crate::plugin::Plugin<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let val_2 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeMetadata",
            "(Ljava/lang/String;Lorg/bukkit/plugin/Plugin;)V",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
}
impl<'mc> JNIRaw<'mc> for EndGateway<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Into<crate::block::TileState<'mc>> for EndGateway<'mc> {
    fn into(self) -> crate::block::TileState<'mc> {
        crate::block::TileState::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
/// Represents a captured state of a container block.
///
/// This is a representation of an abstract class.
pub struct Container<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> Container<'mc> {
    pub fn from_raw(
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
    /// Gets the inventory of the block represented by this block state.
    /// <p>If the block was changed to a different type in the meantime, the returned inventory might no longer be valid.</p>
    /// <p>If this block state is not placed this will return the captured inventory snapshot instead.</p>
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
    /// Gets the captured inventory snapshot of this container.
    /// <p>The returned inventory is not linked to any block. Any modifications to the returned inventory will not be applied to the block represented by this block state up until <a href="BlockState.html#update(boolean,boolean)"><code>BlockState.update(boolean, boolean)</code></a> has been called.</p>
    pub fn snapshot_inventory(
        &mut self,
    ) -> Result<crate::inventory::Inventory<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getSnapshotInventory",
            "()Lorg/bukkit/inventory/Inventory;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::Inventory::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn persistent_data_container(
        &mut self,
    ) -> Result<crate::persistence::PersistentDataContainer<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getPersistentDataContainer",
            "()Lorg/bukkit/persistence/PersistentDataContainer;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::persistence::PersistentDataContainer::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn set_type(
        &mut self,
        arg0: impl Into<&'mc crate::Material<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setType",
            "(Lorg/bukkit/Material;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn world(&mut self) -> Result<crate::World<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getWorld", "()Lorg/bukkit/World;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        crate::World::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn data(
        &mut self,
    ) -> Result<crate::material::MaterialData<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getData",
            "()Lorg/bukkit/material/MaterialData;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::material::MaterialData::from_raw(&self.jni_ref(), unsafe {
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

    pub fn block_data(
        &mut self,
    ) -> Result<crate::block::data::BlockData<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getBlockData",
            "()Lorg/bukkit/block/data/BlockData;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::block::data::BlockData::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn light_level(&mut self) -> Result<i8, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getLightLevel", "()B", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.b().unwrap())
    }

    pub fn x(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getX", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }

    pub fn y(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getY", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }

    pub fn z(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getZ", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }

    pub fn chunk(&mut self) -> Result<crate::Chunk<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getChunk", "()Lorg/bukkit/Chunk;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        crate::Chunk::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn set_data(
        &mut self,
        arg0: impl Into<&'mc crate::material::MaterialData<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setData",
            "(Lorg/bukkit/material/MaterialData;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn set_block_data(
        &mut self,
        arg0: impl Into<&'mc crate::block::data::BlockData<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setBlockData",
            "(Lorg/bukkit/block/data/BlockData;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    #[deprecated]

    pub fn raw_data(&mut self) -> Result<i8, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getRawData", "()B", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.b().unwrap())
    }
    #[deprecated]

    pub fn set_raw_data(&mut self, arg0: i8) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Byte(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setRawData",
            "(B)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn is_placed(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isPlaced", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn update(
        &mut self,
        arg0: std::option::Option<bool>,
        arg1: std::option::Option<bool>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        // -1
        let val_1 = jni::objects::JValueGen::Bool(arg0.unwrap().into());
        // -1
        let val_2 = jni::objects::JValueGen::Bool(arg1.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "update",
            "(ZZ)Z",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn location(&mut self) -> Result<crate::Location<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getLocation",
            "()Lorg/bukkit/Location;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::Location::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn get_type(&mut self) -> Result<crate::Material<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getType",
            "()Lorg/bukkit/Material;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant =
            self.jni_ref()
                .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
        let variant_str = self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::Material::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::Material::from_string(variant_str).unwrap(),
        )
    }

    pub fn set_metadata(
        &mut self,
        arg0: impl Into<&'mc String>,
        arg1: impl Into<&'mc crate::metadata::MetadataValue<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let val_2 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setMetadata",
            "(Ljava/lang/String;Lorg/bukkit/metadata/MetadataValue;)V",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn get_metadata(
        &mut self,
        arg0: impl Into<&'mc String>,
    ) -> Result<Vec<crate::metadata::MetadataValue<'mc>>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getMetadata",
            "(Ljava/lang/String;)Ljava/util/List;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let mut list = blackboxmc_java::JavaList::from_raw(&self.jni_ref(), res.l()?)?;
        let size = list.size()?;
        for i in 0..=size {
            let obj = list.get(i)?;
            new_vec.push(crate::metadata::MetadataValue::from_raw(
                &self.jni_ref(),
                obj,
            )?);
        }
        Ok(new_vec)
    }

    pub fn has_metadata(
        &mut self,
        arg0: impl Into<&'mc String>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "hasMetadata",
            "(Ljava/lang/String;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn remove_metadata(
        &mut self,
        arg0: impl Into<&'mc String>,
        arg1: impl Into<&'mc crate::plugin::Plugin<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let val_2 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeMetadata",
            "(Ljava/lang/String;Lorg/bukkit/plugin/Plugin;)V",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn is_locked(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isLocked", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn lock(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getLock", "()Ljava/lang/String;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }

    pub fn set_lock(
        &mut self,
        arg0: impl Into<&'mc String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setLock",
            "(Ljava/lang/String;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn custom_name(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getCustomName",
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

    pub fn set_custom_name(
        &mut self,
        arg0: impl Into<&'mc String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setCustomName",
            "(Ljava/lang/String;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
}
impl<'mc> JNIRaw<'mc> for Container<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Into<crate::block::TileState<'mc>> for Container<'mc> {
    fn into(self) -> crate::block::TileState<'mc> {
        crate::block::TileState::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
impl<'mc> Into<crate::inventory::BlockInventoryHolder<'mc>> for Container<'mc> {
    fn into(self) -> crate::inventory::BlockInventoryHolder<'mc> {
        crate::inventory::BlockInventoryHolder::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
impl<'mc> Into<crate::block::Lockable<'mc>> for Container<'mc> {
    fn into(self) -> crate::block::Lockable<'mc> {
        crate::block::Lockable::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
impl<'mc> Into<crate::Nameable<'mc>> for Container<'mc> {
    fn into(self) -> crate::Nameable<'mc> {
        crate::Nameable::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
/// Represents a captured state of a blast furnace.
///
/// This is a representation of an abstract class.
pub struct BlastFurnace<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> BlastFurnace<'mc> {
    pub fn from_raw(
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

    pub fn snapshot_inventory(
        &mut self,
    ) -> Result<crate::inventory::FurnaceInventory<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getSnapshotInventory",
            "()Lorg/bukkit/inventory/FurnaceInventory;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::FurnaceInventory::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn cook_time(&mut self) -> Result<i16, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getCookTime", "()S", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.s().unwrap())
    }

    pub fn set_cook_time(&mut self, arg0: i16) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Short(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setCookTime",
            "(S)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn cook_time_total(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getCookTimeTotal", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }

    pub fn set_cook_time_total(&mut self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setCookTimeTotal",
            "(I)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn burn_time(&mut self) -> Result<i16, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getBurnTime", "()S", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.s().unwrap())
    }

    pub fn set_burn_time(&mut self, arg0: i16) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Short(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setBurnTime",
            "(S)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn recipes_used(
        &mut self,
    ) -> Result<blackboxmc_java::JavaMap<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getRecipesUsed",
            "()Ljava/util/Map;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::JavaMap::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn persistent_data_container(
        &mut self,
    ) -> Result<crate::persistence::PersistentDataContainer<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getPersistentDataContainer",
            "()Lorg/bukkit/persistence/PersistentDataContainer;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::persistence::PersistentDataContainer::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn set_type(
        &mut self,
        arg0: impl Into<&'mc crate::Material<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setType",
            "(Lorg/bukkit/Material;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn world(&mut self) -> Result<crate::World<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getWorld", "()Lorg/bukkit/World;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        crate::World::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn data(
        &mut self,
    ) -> Result<crate::material::MaterialData<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getData",
            "()Lorg/bukkit/material/MaterialData;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::material::MaterialData::from_raw(&self.jni_ref(), unsafe {
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

    pub fn block_data(
        &mut self,
    ) -> Result<crate::block::data::BlockData<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getBlockData",
            "()Lorg/bukkit/block/data/BlockData;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::block::data::BlockData::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn light_level(&mut self) -> Result<i8, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getLightLevel", "()B", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.b().unwrap())
    }

    pub fn x(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getX", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }

    pub fn y(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getY", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }

    pub fn z(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getZ", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }

    pub fn chunk(&mut self) -> Result<crate::Chunk<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getChunk", "()Lorg/bukkit/Chunk;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        crate::Chunk::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn set_data(
        &mut self,
        arg0: impl Into<&'mc crate::material::MaterialData<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setData",
            "(Lorg/bukkit/material/MaterialData;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn set_block_data(
        &mut self,
        arg0: impl Into<&'mc crate::block::data::BlockData<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setBlockData",
            "(Lorg/bukkit/block/data/BlockData;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    #[deprecated]

    pub fn raw_data(&mut self) -> Result<i8, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getRawData", "()B", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.b().unwrap())
    }
    #[deprecated]

    pub fn set_raw_data(&mut self, arg0: i8) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Byte(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setRawData",
            "(B)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn is_placed(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isPlaced", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn update(
        &mut self,
        arg0: std::option::Option<bool>,
        arg1: std::option::Option<bool>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        // -1
        let val_1 = jni::objects::JValueGen::Bool(arg0.unwrap().into());
        // -1
        let val_2 = jni::objects::JValueGen::Bool(arg1.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "update",
            "(ZZ)Z",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn location(&mut self) -> Result<crate::Location<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getLocation",
            "()Lorg/bukkit/Location;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::Location::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn get_type(&mut self) -> Result<crate::Material<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getType",
            "()Lorg/bukkit/Material;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant =
            self.jni_ref()
                .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
        let variant_str = self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::Material::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::Material::from_string(variant_str).unwrap(),
        )
    }

    pub fn set_metadata(
        &mut self,
        arg0: impl Into<&'mc String>,
        arg1: impl Into<&'mc crate::metadata::MetadataValue<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let val_2 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setMetadata",
            "(Ljava/lang/String;Lorg/bukkit/metadata/MetadataValue;)V",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn get_metadata(
        &mut self,
        arg0: impl Into<&'mc String>,
    ) -> Result<Vec<crate::metadata::MetadataValue<'mc>>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getMetadata",
            "(Ljava/lang/String;)Ljava/util/List;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let mut list = blackboxmc_java::JavaList::from_raw(&self.jni_ref(), res.l()?)?;
        let size = list.size()?;
        for i in 0..=size {
            let obj = list.get(i)?;
            new_vec.push(crate::metadata::MetadataValue::from_raw(
                &self.jni_ref(),
                obj,
            )?);
        }
        Ok(new_vec)
    }

    pub fn has_metadata(
        &mut self,
        arg0: impl Into<&'mc String>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "hasMetadata",
            "(Ljava/lang/String;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn remove_metadata(
        &mut self,
        arg0: impl Into<&'mc String>,
        arg1: impl Into<&'mc crate::plugin::Plugin<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let val_2 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeMetadata",
            "(Ljava/lang/String;Lorg/bukkit/plugin/Plugin;)V",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn is_locked(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isLocked", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn lock(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getLock", "()Ljava/lang/String;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }

    pub fn set_lock(
        &mut self,
        arg0: impl Into<&'mc String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setLock",
            "(Ljava/lang/String;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn custom_name(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getCustomName",
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

    pub fn set_custom_name(
        &mut self,
        arg0: impl Into<&'mc String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setCustomName",
            "(Ljava/lang/String;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
}
impl<'mc> JNIRaw<'mc> for BlastFurnace<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Into<crate::block::Furnace<'mc>> for BlastFurnace<'mc> {
    fn into(self) -> crate::block::Furnace<'mc> {
        crate::block::Furnace::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
pub enum PistonMoveReactionEnum {
    VariantMove,
    VariantBreak,
    Block,
    Ignore,
    PushOnly,
}
impl std::fmt::Display for PistonMoveReactionEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            PistonMoveReactionEnum::VariantMove => f.write_str("MOVE"),
            PistonMoveReactionEnum::VariantBreak => f.write_str("BREAK"),
            PistonMoveReactionEnum::Block => f.write_str("BLOCK"),
            PistonMoveReactionEnum::Ignore => f.write_str("IGNORE"),
            PistonMoveReactionEnum::PushOnly => f.write_str("PUSH_ONLY"),
        }
    }
}
pub struct PistonMoveReaction<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
    pub PistonMoveReactionEnum,
);
impl<'mc> std::ops::Deref for PistonMoveReaction<'mc> {
    type Target = PistonMoveReactionEnum;
    fn deref(&self) -> &Self::Target {
        return &self.2;
    }
}
impl<'mc> JNIRaw<'mc> for PistonMoveReaction<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> PistonMoveReaction<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
        e: PistonMoveReactionEnum,
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
            Ok(Self(env.clone(), obj, e))
        }
    }
    pub const MOVE: PistonMoveReactionEnum = PistonMoveReactionEnum::VariantMove;
    pub const BREAK: PistonMoveReactionEnum = PistonMoveReactionEnum::VariantBreak;
    pub const BLOCK: PistonMoveReactionEnum = PistonMoveReactionEnum::Block;
    pub const IGNORE: PistonMoveReactionEnum = PistonMoveReactionEnum::Ignore;
    pub const PUSH_ONLY: PistonMoveReactionEnum = PistonMoveReactionEnum::PushOnly;
    pub fn from_string(str: String) -> std::option::Option<PistonMoveReactionEnum> {
        match str.as_str() {
            "MOVE" => Some(PistonMoveReactionEnum::VariantMove),
            "BREAK" => Some(PistonMoveReactionEnum::VariantBreak),
            "BLOCK" => Some(PistonMoveReactionEnum::Block),
            "IGNORE" => Some(PistonMoveReactionEnum::Ignore),
            "PUSH_ONLY" => Some(PistonMoveReactionEnum::PushOnly),
            _ => None,
        }
    }
}

///
/// This is a representation of an abstract class.
pub struct Lidded<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> Lidded<'mc> {
    pub fn from_raw(
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
    /// Sets the block's animated state to closed even if a player is currently viewing this block.
    pub fn close(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "close", "()V", &[]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Sets the block's animated state to open and prevents it from being closed until <a href="#close()"><code>close()</code></a> is called.
    pub fn open(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "open", "()V", &[]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
}
impl<'mc> JNIRaw<'mc> for Lidded<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
/// Represents a captured state of an enchanting table.
///
/// This is a representation of an abstract class.
pub struct EnchantingTable<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> EnchantingTable<'mc> {
    pub fn from_raw(
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

    pub fn persistent_data_container(
        &mut self,
    ) -> Result<crate::persistence::PersistentDataContainer<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getPersistentDataContainer",
            "()Lorg/bukkit/persistence/PersistentDataContainer;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::persistence::PersistentDataContainer::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn set_type(
        &mut self,
        arg0: impl Into<&'mc crate::Material<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setType",
            "(Lorg/bukkit/Material;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn world(&mut self) -> Result<crate::World<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getWorld", "()Lorg/bukkit/World;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        crate::World::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn data(
        &mut self,
    ) -> Result<crate::material::MaterialData<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getData",
            "()Lorg/bukkit/material/MaterialData;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::material::MaterialData::from_raw(&self.jni_ref(), unsafe {
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

    pub fn block_data(
        &mut self,
    ) -> Result<crate::block::data::BlockData<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getBlockData",
            "()Lorg/bukkit/block/data/BlockData;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::block::data::BlockData::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn light_level(&mut self) -> Result<i8, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getLightLevel", "()B", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.b().unwrap())
    }

    pub fn x(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getX", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }

    pub fn y(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getY", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }

    pub fn z(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getZ", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }

    pub fn chunk(&mut self) -> Result<crate::Chunk<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getChunk", "()Lorg/bukkit/Chunk;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        crate::Chunk::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn set_data(
        &mut self,
        arg0: impl Into<&'mc crate::material::MaterialData<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setData",
            "(Lorg/bukkit/material/MaterialData;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn set_block_data(
        &mut self,
        arg0: impl Into<&'mc crate::block::data::BlockData<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setBlockData",
            "(Lorg/bukkit/block/data/BlockData;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    #[deprecated]

    pub fn raw_data(&mut self) -> Result<i8, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getRawData", "()B", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.b().unwrap())
    }
    #[deprecated]

    pub fn set_raw_data(&mut self, arg0: i8) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Byte(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setRawData",
            "(B)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn is_placed(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isPlaced", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn update(
        &mut self,
        arg0: std::option::Option<bool>,
        arg1: std::option::Option<bool>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        // -1
        let val_1 = jni::objects::JValueGen::Bool(arg0.unwrap().into());
        // -1
        let val_2 = jni::objects::JValueGen::Bool(arg1.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "update",
            "(ZZ)Z",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn location(&mut self) -> Result<crate::Location<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getLocation",
            "()Lorg/bukkit/Location;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::Location::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn get_type(&mut self) -> Result<crate::Material<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getType",
            "()Lorg/bukkit/Material;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant =
            self.jni_ref()
                .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
        let variant_str = self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::Material::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::Material::from_string(variant_str).unwrap(),
        )
    }

    pub fn set_metadata(
        &mut self,
        arg0: impl Into<&'mc String>,
        arg1: impl Into<&'mc crate::metadata::MetadataValue<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let val_2 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setMetadata",
            "(Ljava/lang/String;Lorg/bukkit/metadata/MetadataValue;)V",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn get_metadata(
        &mut self,
        arg0: impl Into<&'mc String>,
    ) -> Result<Vec<crate::metadata::MetadataValue<'mc>>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getMetadata",
            "(Ljava/lang/String;)Ljava/util/List;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let mut list = blackboxmc_java::JavaList::from_raw(&self.jni_ref(), res.l()?)?;
        let size = list.size()?;
        for i in 0..=size {
            let obj = list.get(i)?;
            new_vec.push(crate::metadata::MetadataValue::from_raw(
                &self.jni_ref(),
                obj,
            )?);
        }
        Ok(new_vec)
    }

    pub fn has_metadata(
        &mut self,
        arg0: impl Into<&'mc String>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "hasMetadata",
            "(Ljava/lang/String;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn remove_metadata(
        &mut self,
        arg0: impl Into<&'mc String>,
        arg1: impl Into<&'mc crate::plugin::Plugin<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let val_2 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeMetadata",
            "(Ljava/lang/String;Lorg/bukkit/plugin/Plugin;)V",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn custom_name(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getCustomName",
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

    pub fn set_custom_name(
        &mut self,
        arg0: impl Into<&'mc String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setCustomName",
            "(Ljava/lang/String;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
}
impl<'mc> JNIRaw<'mc> for EnchantingTable<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Into<crate::block::TileState<'mc>> for EnchantingTable<'mc> {
    fn into(self) -> crate::block::TileState<'mc> {
        crate::block::TileState::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
impl<'mc> Into<crate::Nameable<'mc>> for EnchantingTable<'mc> {
    fn into(self) -> crate::Nameable<'mc> {
        crate::Nameable::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
/// Represents a captured state of a ShulkerBox.
///
/// This is a representation of an abstract class.
pub struct ShulkerBox<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> ShulkerBox<'mc> {
    pub fn from_raw(
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
    /// Get the <a href="../DyeColor.html" title="enum in org.bukkit"><code>DyeColor</code></a> corresponding to this ShulkerBox
    pub fn color(&mut self) -> Result<crate::DyeColor<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getColor",
            "()Lorg/bukkit/DyeColor;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant =
            self.jni_ref()
                .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
        let variant_str = self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::DyeColor::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::DyeColor::from_string(variant_str).unwrap(),
        )
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

    pub fn snapshot_inventory(
        &mut self,
    ) -> Result<crate::inventory::Inventory<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getSnapshotInventory",
            "()Lorg/bukkit/inventory/Inventory;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::Inventory::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn persistent_data_container(
        &mut self,
    ) -> Result<crate::persistence::PersistentDataContainer<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getPersistentDataContainer",
            "()Lorg/bukkit/persistence/PersistentDataContainer;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::persistence::PersistentDataContainer::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn set_type(
        &mut self,
        arg0: impl Into<&'mc crate::Material<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setType",
            "(Lorg/bukkit/Material;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn world(&mut self) -> Result<crate::World<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getWorld", "()Lorg/bukkit/World;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        crate::World::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn data(
        &mut self,
    ) -> Result<crate::material::MaterialData<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getData",
            "()Lorg/bukkit/material/MaterialData;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::material::MaterialData::from_raw(&self.jni_ref(), unsafe {
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

    pub fn block_data(
        &mut self,
    ) -> Result<crate::block::data::BlockData<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getBlockData",
            "()Lorg/bukkit/block/data/BlockData;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::block::data::BlockData::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn light_level(&mut self) -> Result<i8, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getLightLevel", "()B", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.b().unwrap())
    }

    pub fn x(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getX", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }

    pub fn y(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getY", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }

    pub fn z(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getZ", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }

    pub fn chunk(&mut self) -> Result<crate::Chunk<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getChunk", "()Lorg/bukkit/Chunk;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        crate::Chunk::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn set_data(
        &mut self,
        arg0: impl Into<&'mc crate::material::MaterialData<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setData",
            "(Lorg/bukkit/material/MaterialData;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn set_block_data(
        &mut self,
        arg0: impl Into<&'mc crate::block::data::BlockData<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setBlockData",
            "(Lorg/bukkit/block/data/BlockData;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    #[deprecated]

    pub fn raw_data(&mut self) -> Result<i8, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getRawData", "()B", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.b().unwrap())
    }
    #[deprecated]

    pub fn set_raw_data(&mut self, arg0: i8) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Byte(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setRawData",
            "(B)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn is_placed(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isPlaced", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn update(
        &mut self,
        arg0: std::option::Option<bool>,
        arg1: std::option::Option<bool>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        // -1
        let val_1 = jni::objects::JValueGen::Bool(arg0.unwrap().into());
        // -1
        let val_2 = jni::objects::JValueGen::Bool(arg1.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "update",
            "(ZZ)Z",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn location(&mut self) -> Result<crate::Location<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getLocation",
            "()Lorg/bukkit/Location;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::Location::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn get_type(&mut self) -> Result<crate::Material<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getType",
            "()Lorg/bukkit/Material;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant =
            self.jni_ref()
                .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
        let variant_str = self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::Material::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::Material::from_string(variant_str).unwrap(),
        )
    }

    pub fn set_metadata(
        &mut self,
        arg0: impl Into<&'mc String>,
        arg1: impl Into<&'mc crate::metadata::MetadataValue<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let val_2 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setMetadata",
            "(Ljava/lang/String;Lorg/bukkit/metadata/MetadataValue;)V",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn get_metadata(
        &mut self,
        arg0: impl Into<&'mc String>,
    ) -> Result<Vec<crate::metadata::MetadataValue<'mc>>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getMetadata",
            "(Ljava/lang/String;)Ljava/util/List;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let mut list = blackboxmc_java::JavaList::from_raw(&self.jni_ref(), res.l()?)?;
        let size = list.size()?;
        for i in 0..=size {
            let obj = list.get(i)?;
            new_vec.push(crate::metadata::MetadataValue::from_raw(
                &self.jni_ref(),
                obj,
            )?);
        }
        Ok(new_vec)
    }

    pub fn has_metadata(
        &mut self,
        arg0: impl Into<&'mc String>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "hasMetadata",
            "(Ljava/lang/String;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn remove_metadata(
        &mut self,
        arg0: impl Into<&'mc String>,
        arg1: impl Into<&'mc crate::plugin::Plugin<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let val_2 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeMetadata",
            "(Ljava/lang/String;Lorg/bukkit/plugin/Plugin;)V",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn is_locked(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isLocked", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn lock(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getLock", "()Ljava/lang/String;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }

    pub fn set_lock(
        &mut self,
        arg0: impl Into<&'mc String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setLock",
            "(Ljava/lang/String;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn custom_name(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getCustomName",
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

    pub fn set_custom_name(
        &mut self,
        arg0: impl Into<&'mc String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setCustomName",
            "(Ljava/lang/String;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn seed(&mut self) -> Result<i64, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getSeed", "()J", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.j().unwrap())
    }

    pub fn set_seed(&mut self, arg0: i64) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Long(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setSeed",
            "(J)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn set_loot_table(
        &mut self,
        arg0: impl Into<&'mc crate::loot::LootTable<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setLootTable",
            "(Lorg/bukkit/loot/LootTable;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn loot_table(
        &mut self,
    ) -> Result<crate::loot::LootTable<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getLootTable",
            "()Lorg/bukkit/loot/LootTable;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::loot::LootTable::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn close(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "close", "()V", &[]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn open(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "open", "()V", &[]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
}
impl<'mc> JNIRaw<'mc> for ShulkerBox<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Into<crate::block::Container<'mc>> for ShulkerBox<'mc> {
    fn into(self) -> crate::block::Container<'mc> {
        crate::block::Container::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
impl<'mc> Into<crate::loot::Lootable<'mc>> for ShulkerBox<'mc> {
    fn into(self) -> crate::loot::Lootable<'mc> {
        crate::loot::Lootable::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
impl<'mc> Into<crate::block::Lidded<'mc>> for ShulkerBox<'mc> {
    fn into(self) -> crate::block::Lidded<'mc> {
        crate::block::Lidded::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
/// Represents a captured state of a block which stores entities.
///
/// This is a representation of an abstract class.
pub struct EntityBlockStorage<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> EntityBlockStorage<'mc> {
    pub fn from_raw(
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
    /// Check if the block is completely full of entities.
    pub fn is_full(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isFull", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    /// Get the amount of entities currently in this block.
    pub fn entity_count(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getEntityCount", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }
    /// Get the maximum amount of entities this block can hold.
    pub fn max_entities(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getMaxEntities", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }
    /// Set the maximum amount of entities this block can hold.
    pub fn set_max_entities(&mut self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setMaxEntities",
            "(I)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Release all the entities currently stored in the block.
    pub fn release_entities(
        &mut self,
    ) -> Result<Vec<jni::objects::JObject<'mc>>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "releaseEntities",
            "()Ljava/util/List;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let mut list = blackboxmc_java::JavaList::from_raw(&self.jni_ref(), res.l()?)?;
        let size = list.size()?;
        for i in 0..=size {
            let obj = list.get(i)?;
            new_vec.push(obj);
        }
        Ok(new_vec)
    }
    /// Add an entity to the block.
    pub fn add_entity(
        &mut self,
        arg0: impl Into<&'mc crate::entity::Entity<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "addEntity",
            "(Lorg/bukkit/entity/Entity;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn persistent_data_container(
        &mut self,
    ) -> Result<crate::persistence::PersistentDataContainer<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getPersistentDataContainer",
            "()Lorg/bukkit/persistence/PersistentDataContainer;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::persistence::PersistentDataContainer::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn set_type(
        &mut self,
        arg0: impl Into<&'mc crate::Material<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setType",
            "(Lorg/bukkit/Material;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn world(&mut self) -> Result<crate::World<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getWorld", "()Lorg/bukkit/World;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        crate::World::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn data(
        &mut self,
    ) -> Result<crate::material::MaterialData<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getData",
            "()Lorg/bukkit/material/MaterialData;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::material::MaterialData::from_raw(&self.jni_ref(), unsafe {
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

    pub fn block_data(
        &mut self,
    ) -> Result<crate::block::data::BlockData<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getBlockData",
            "()Lorg/bukkit/block/data/BlockData;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::block::data::BlockData::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn light_level(&mut self) -> Result<i8, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getLightLevel", "()B", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.b().unwrap())
    }

    pub fn x(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getX", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }

    pub fn y(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getY", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }

    pub fn z(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getZ", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }

    pub fn chunk(&mut self) -> Result<crate::Chunk<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getChunk", "()Lorg/bukkit/Chunk;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        crate::Chunk::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn set_data(
        &mut self,
        arg0: impl Into<&'mc crate::material::MaterialData<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setData",
            "(Lorg/bukkit/material/MaterialData;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn set_block_data(
        &mut self,
        arg0: impl Into<&'mc crate::block::data::BlockData<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setBlockData",
            "(Lorg/bukkit/block/data/BlockData;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    #[deprecated]

    pub fn raw_data(&mut self) -> Result<i8, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getRawData", "()B", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.b().unwrap())
    }
    #[deprecated]

    pub fn set_raw_data(&mut self, arg0: i8) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Byte(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setRawData",
            "(B)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn is_placed(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isPlaced", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn update(
        &mut self,
        arg0: std::option::Option<bool>,
        arg1: std::option::Option<bool>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        // -1
        let val_1 = jni::objects::JValueGen::Bool(arg0.unwrap().into());
        // -1
        let val_2 = jni::objects::JValueGen::Bool(arg1.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "update",
            "(ZZ)Z",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn location(&mut self) -> Result<crate::Location<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getLocation",
            "()Lorg/bukkit/Location;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::Location::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn get_type(&mut self) -> Result<crate::Material<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getType",
            "()Lorg/bukkit/Material;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant =
            self.jni_ref()
                .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
        let variant_str = self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::Material::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::Material::from_string(variant_str).unwrap(),
        )
    }

    pub fn set_metadata(
        &mut self,
        arg0: impl Into<&'mc String>,
        arg1: impl Into<&'mc crate::metadata::MetadataValue<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let val_2 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setMetadata",
            "(Ljava/lang/String;Lorg/bukkit/metadata/MetadataValue;)V",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn get_metadata(
        &mut self,
        arg0: impl Into<&'mc String>,
    ) -> Result<Vec<crate::metadata::MetadataValue<'mc>>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getMetadata",
            "(Ljava/lang/String;)Ljava/util/List;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let mut list = blackboxmc_java::JavaList::from_raw(&self.jni_ref(), res.l()?)?;
        let size = list.size()?;
        for i in 0..=size {
            let obj = list.get(i)?;
            new_vec.push(crate::metadata::MetadataValue::from_raw(
                &self.jni_ref(),
                obj,
            )?);
        }
        Ok(new_vec)
    }

    pub fn has_metadata(
        &mut self,
        arg0: impl Into<&'mc String>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "hasMetadata",
            "(Ljava/lang/String;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn remove_metadata(
        &mut self,
        arg0: impl Into<&'mc String>,
        arg1: impl Into<&'mc crate::plugin::Plugin<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let val_2 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeMetadata",
            "(Ljava/lang/String;Lorg/bukkit/plugin/Plugin;)V",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
}
impl<'mc> JNIRaw<'mc> for EntityBlockStorage<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Into<crate::block::TileState<'mc>> for EntityBlockStorage<'mc> {
    fn into(self) -> crate::block::TileState<'mc> {
        crate::block::TileState::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
/// Represents a captured state of a bee hive.
///
/// This is a representation of an abstract class.
pub struct Beehive<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> Beehive<'mc> {
    pub fn from_raw(
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
    /// Get the hive's flower location.
    pub fn flower(&mut self) -> Result<crate::Location<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getFlower",
            "()Lorg/bukkit/Location;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::Location::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Set the hive's flower location.
    pub fn set_flower(
        &mut self,
        arg0: impl Into<&'mc crate::Location<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setFlower",
            "(Lorg/bukkit/Location;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Check if the hive is sedated due to smoke from a nearby campfire.
    pub fn is_sedated(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isSedated", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn is_full(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isFull", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn entity_count(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getEntityCount", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }

    pub fn max_entities(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getMaxEntities", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }

    pub fn set_max_entities(&mut self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setMaxEntities",
            "(I)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn release_entities(
        &mut self,
    ) -> Result<Vec<jni::objects::JObject<'mc>>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "releaseEntities",
            "()Ljava/util/List;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let mut list = blackboxmc_java::JavaList::from_raw(&self.jni_ref(), res.l()?)?;
        let size = list.size()?;
        for i in 0..=size {
            let obj = list.get(i)?;
            new_vec.push(obj);
        }
        Ok(new_vec)
    }

    pub fn add_entity(
        &mut self,
        arg0: impl Into<&'mc crate::entity::Entity<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "addEntity",
            "(Lorg/bukkit/entity/Entity;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn persistent_data_container(
        &mut self,
    ) -> Result<crate::persistence::PersistentDataContainer<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getPersistentDataContainer",
            "()Lorg/bukkit/persistence/PersistentDataContainer;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::persistence::PersistentDataContainer::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn set_type(
        &mut self,
        arg0: impl Into<&'mc crate::Material<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setType",
            "(Lorg/bukkit/Material;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn world(&mut self) -> Result<crate::World<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getWorld", "()Lorg/bukkit/World;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        crate::World::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn data(
        &mut self,
    ) -> Result<crate::material::MaterialData<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getData",
            "()Lorg/bukkit/material/MaterialData;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::material::MaterialData::from_raw(&self.jni_ref(), unsafe {
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

    pub fn block_data(
        &mut self,
    ) -> Result<crate::block::data::BlockData<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getBlockData",
            "()Lorg/bukkit/block/data/BlockData;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::block::data::BlockData::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn light_level(&mut self) -> Result<i8, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getLightLevel", "()B", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.b().unwrap())
    }

    pub fn x(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getX", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }

    pub fn y(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getY", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }

    pub fn z(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getZ", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }

    pub fn chunk(&mut self) -> Result<crate::Chunk<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getChunk", "()Lorg/bukkit/Chunk;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        crate::Chunk::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn set_data(
        &mut self,
        arg0: impl Into<&'mc crate::material::MaterialData<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setData",
            "(Lorg/bukkit/material/MaterialData;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn set_block_data(
        &mut self,
        arg0: impl Into<&'mc crate::block::data::BlockData<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setBlockData",
            "(Lorg/bukkit/block/data/BlockData;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    #[deprecated]

    pub fn raw_data(&mut self) -> Result<i8, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getRawData", "()B", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.b().unwrap())
    }
    #[deprecated]

    pub fn set_raw_data(&mut self, arg0: i8) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Byte(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setRawData",
            "(B)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn is_placed(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isPlaced", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn update(
        &mut self,
        arg0: std::option::Option<bool>,
        arg1: std::option::Option<bool>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        // -1
        let val_1 = jni::objects::JValueGen::Bool(arg0.unwrap().into());
        // -1
        let val_2 = jni::objects::JValueGen::Bool(arg1.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "update",
            "(ZZ)Z",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn location(&mut self) -> Result<crate::Location<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getLocation",
            "()Lorg/bukkit/Location;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::Location::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn get_type(&mut self) -> Result<crate::Material<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getType",
            "()Lorg/bukkit/Material;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant =
            self.jni_ref()
                .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
        let variant_str = self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::Material::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::Material::from_string(variant_str).unwrap(),
        )
    }

    pub fn set_metadata(
        &mut self,
        arg0: impl Into<&'mc String>,
        arg1: impl Into<&'mc crate::metadata::MetadataValue<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let val_2 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setMetadata",
            "(Ljava/lang/String;Lorg/bukkit/metadata/MetadataValue;)V",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn get_metadata(
        &mut self,
        arg0: impl Into<&'mc String>,
    ) -> Result<Vec<crate::metadata::MetadataValue<'mc>>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getMetadata",
            "(Ljava/lang/String;)Ljava/util/List;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let mut list = blackboxmc_java::JavaList::from_raw(&self.jni_ref(), res.l()?)?;
        let size = list.size()?;
        for i in 0..=size {
            let obj = list.get(i)?;
            new_vec.push(crate::metadata::MetadataValue::from_raw(
                &self.jni_ref(),
                obj,
            )?);
        }
        Ok(new_vec)
    }

    pub fn has_metadata(
        &mut self,
        arg0: impl Into<&'mc String>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "hasMetadata",
            "(Ljava/lang/String;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn remove_metadata(
        &mut self,
        arg0: impl Into<&'mc String>,
        arg1: impl Into<&'mc crate::plugin::Plugin<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let val_2 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeMetadata",
            "(Ljava/lang/String;Lorg/bukkit/plugin/Plugin;)V",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
}
impl<'mc> JNIRaw<'mc> for Beehive<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Into<crate::block::EntityBlockStorage<'mc>> for Beehive<'mc> {
    fn into(self) -> crate::block::EntityBlockStorage<'mc> {
        crate::block::EntityBlockStorage::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
/// A side on a decorated pot. Sides are relative to the facing state of a <a href="data/type/DecoratedPot.html" title="interface in org.bukkit.block.data.type"><code>DecoratedPot</code></a>.
pub struct DecoratedPotSide<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for DecoratedPotSide<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> DecoratedPotSide<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate DecoratedPotSide from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/block/DecoratedPotSide")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a DecoratedPotSide object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    /// Returns the enum constant of this type with the specified name. The string must match <i>exactly</i> an identifier used to declare an enum constant in this type. (Extraneous whitespace characters are not permitted.)
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
/// Represents a captured state of a sculk catalyst.
///
/// This is a representation of an abstract class.
pub struct SculkCatalyst<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> SculkCatalyst<'mc> {
    pub fn from_raw(
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

    pub fn persistent_data_container(
        &mut self,
    ) -> Result<crate::persistence::PersistentDataContainer<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getPersistentDataContainer",
            "()Lorg/bukkit/persistence/PersistentDataContainer;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::persistence::PersistentDataContainer::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn set_type(
        &mut self,
        arg0: impl Into<&'mc crate::Material<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setType",
            "(Lorg/bukkit/Material;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn world(&mut self) -> Result<crate::World<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getWorld", "()Lorg/bukkit/World;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        crate::World::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn data(
        &mut self,
    ) -> Result<crate::material::MaterialData<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getData",
            "()Lorg/bukkit/material/MaterialData;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::material::MaterialData::from_raw(&self.jni_ref(), unsafe {
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

    pub fn block_data(
        &mut self,
    ) -> Result<crate::block::data::BlockData<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getBlockData",
            "()Lorg/bukkit/block/data/BlockData;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::block::data::BlockData::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn light_level(&mut self) -> Result<i8, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getLightLevel", "()B", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.b().unwrap())
    }

    pub fn x(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getX", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }

    pub fn y(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getY", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }

    pub fn z(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getZ", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }

    pub fn chunk(&mut self) -> Result<crate::Chunk<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getChunk", "()Lorg/bukkit/Chunk;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        crate::Chunk::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn set_data(
        &mut self,
        arg0: impl Into<&'mc crate::material::MaterialData<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setData",
            "(Lorg/bukkit/material/MaterialData;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn set_block_data(
        &mut self,
        arg0: impl Into<&'mc crate::block::data::BlockData<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setBlockData",
            "(Lorg/bukkit/block/data/BlockData;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    #[deprecated]

    pub fn raw_data(&mut self) -> Result<i8, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getRawData", "()B", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.b().unwrap())
    }
    #[deprecated]

    pub fn set_raw_data(&mut self, arg0: i8) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Byte(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setRawData",
            "(B)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn is_placed(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isPlaced", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn update(
        &mut self,
        arg0: std::option::Option<bool>,
        arg1: std::option::Option<bool>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        // -1
        let val_1 = jni::objects::JValueGen::Bool(arg0.unwrap().into());
        // -1
        let val_2 = jni::objects::JValueGen::Bool(arg1.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "update",
            "(ZZ)Z",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn location(&mut self) -> Result<crate::Location<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getLocation",
            "()Lorg/bukkit/Location;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::Location::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn get_type(&mut self) -> Result<crate::Material<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getType",
            "()Lorg/bukkit/Material;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant =
            self.jni_ref()
                .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
        let variant_str = self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::Material::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::Material::from_string(variant_str).unwrap(),
        )
    }

    pub fn set_metadata(
        &mut self,
        arg0: impl Into<&'mc String>,
        arg1: impl Into<&'mc crate::metadata::MetadataValue<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let val_2 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setMetadata",
            "(Ljava/lang/String;Lorg/bukkit/metadata/MetadataValue;)V",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn get_metadata(
        &mut self,
        arg0: impl Into<&'mc String>,
    ) -> Result<Vec<crate::metadata::MetadataValue<'mc>>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getMetadata",
            "(Ljava/lang/String;)Ljava/util/List;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let mut list = blackboxmc_java::JavaList::from_raw(&self.jni_ref(), res.l()?)?;
        let size = list.size()?;
        for i in 0..=size {
            let obj = list.get(i)?;
            new_vec.push(crate::metadata::MetadataValue::from_raw(
                &self.jni_ref(),
                obj,
            )?);
        }
        Ok(new_vec)
    }

    pub fn has_metadata(
        &mut self,
        arg0: impl Into<&'mc String>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "hasMetadata",
            "(Ljava/lang/String;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn remove_metadata(
        &mut self,
        arg0: impl Into<&'mc String>,
        arg1: impl Into<&'mc crate::plugin::Plugin<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let val_2 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeMetadata",
            "(Ljava/lang/String;Lorg/bukkit/plugin/Plugin;)V",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
}
impl<'mc> JNIRaw<'mc> for SculkCatalyst<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Into<crate::block::TileState<'mc>> for SculkCatalyst<'mc> {
    fn into(self) -> crate::block::TileState<'mc> {
        crate::block::TileState::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
/// Represents a captured state of a furnace.
///
/// This is a representation of an abstract class.
pub struct Furnace<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> Furnace<'mc> {
    pub fn from_raw(
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
    /// <span class="descfrm-type-label">Description copied from interface:&nbsp;<code><a href="Container.html#getInventory()">Container</a></code></span>
    /// Gets the inventory of the block represented by this block state.
    /// <p>If the block was changed to a different type in the meantime, the returned inventory might no longer be valid.</p>
    /// <p>If this block state is not placed this will return the captured inventory snapshot instead.</p>
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
    /// <span class="descfrm-type-label">Description copied from interface:&nbsp;<code><a href="Container.html#getSnapshotInventory()">Container</a></code></span>
    /// Gets the captured inventory snapshot of this container.
    /// <p>The returned inventory is not linked to any block. Any modifications to the returned inventory will not be applied to the block represented by this block state up until <a href="BlockState.html#update(boolean,boolean)"><code>BlockState.update(boolean, boolean)</code></a> has been called.</p>
    pub fn snapshot_inventory(
        &mut self,
    ) -> Result<crate::inventory::FurnaceInventory<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getSnapshotInventory",
            "()Lorg/bukkit/inventory/FurnaceInventory;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::FurnaceInventory::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Get cook time. This is the amount of time the item has been cooking for.
    /// Get cook time total. This is the amount of time the item is required to cook for.
    pub fn cook_time(&mut self) -> Result<i16, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getCookTime", "()S", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.s().unwrap())
    }
    /// Set cook time. This is the amount of time the item has been cooking for.
    /// Set cook time. This is the amount of time the item is required to cook for.
    pub fn set_cook_time(&mut self, arg0: i16) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Short(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setCookTime",
            "(S)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Get cook time total. This is the amount of time the item is required to cook for.
    pub fn cook_time_total(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getCookTimeTotal", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }
    /// Set cook time. This is the amount of time the item is required to cook for.
    pub fn set_cook_time_total(&mut self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setCookTimeTotal",
            "(I)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Get burn time.
    pub fn burn_time(&mut self) -> Result<i16, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getBurnTime", "()S", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.s().unwrap())
    }
    /// Set burn time. A burn time greater than 0 will cause this block to be lit, whilst a time less than 0 will extinguish it.
    pub fn set_burn_time(&mut self, arg0: i16) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Short(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setBurnTime",
            "(S)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Get the recipes used in this furnace. <b>Note:</b> These recipes used are reset when the result item is manually taken from the furnace.
    pub fn recipes_used(
        &mut self,
    ) -> Result<blackboxmc_java::JavaMap<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getRecipesUsed",
            "()Ljava/util/Map;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::JavaMap::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn persistent_data_container(
        &mut self,
    ) -> Result<crate::persistence::PersistentDataContainer<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getPersistentDataContainer",
            "()Lorg/bukkit/persistence/PersistentDataContainer;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::persistence::PersistentDataContainer::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn set_type(
        &mut self,
        arg0: impl Into<&'mc crate::Material<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setType",
            "(Lorg/bukkit/Material;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn world(&mut self) -> Result<crate::World<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getWorld", "()Lorg/bukkit/World;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        crate::World::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn data(
        &mut self,
    ) -> Result<crate::material::MaterialData<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getData",
            "()Lorg/bukkit/material/MaterialData;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::material::MaterialData::from_raw(&self.jni_ref(), unsafe {
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

    pub fn block_data(
        &mut self,
    ) -> Result<crate::block::data::BlockData<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getBlockData",
            "()Lorg/bukkit/block/data/BlockData;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::block::data::BlockData::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn light_level(&mut self) -> Result<i8, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getLightLevel", "()B", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.b().unwrap())
    }

    pub fn x(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getX", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }

    pub fn y(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getY", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }

    pub fn z(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getZ", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }

    pub fn chunk(&mut self) -> Result<crate::Chunk<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getChunk", "()Lorg/bukkit/Chunk;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        crate::Chunk::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn set_data(
        &mut self,
        arg0: impl Into<&'mc crate::material::MaterialData<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setData",
            "(Lorg/bukkit/material/MaterialData;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn set_block_data(
        &mut self,
        arg0: impl Into<&'mc crate::block::data::BlockData<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setBlockData",
            "(Lorg/bukkit/block/data/BlockData;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    #[deprecated]

    pub fn raw_data(&mut self) -> Result<i8, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getRawData", "()B", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.b().unwrap())
    }
    #[deprecated]

    pub fn set_raw_data(&mut self, arg0: i8) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Byte(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setRawData",
            "(B)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn is_placed(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isPlaced", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn update(
        &mut self,
        arg0: std::option::Option<bool>,
        arg1: std::option::Option<bool>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        // -1
        let val_1 = jni::objects::JValueGen::Bool(arg0.unwrap().into());
        // -1
        let val_2 = jni::objects::JValueGen::Bool(arg1.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "update",
            "(ZZ)Z",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn location(&mut self) -> Result<crate::Location<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getLocation",
            "()Lorg/bukkit/Location;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::Location::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn get_type(&mut self) -> Result<crate::Material<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getType",
            "()Lorg/bukkit/Material;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant =
            self.jni_ref()
                .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
        let variant_str = self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::Material::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::Material::from_string(variant_str).unwrap(),
        )
    }

    pub fn set_metadata(
        &mut self,
        arg0: impl Into<&'mc String>,
        arg1: impl Into<&'mc crate::metadata::MetadataValue<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let val_2 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setMetadata",
            "(Ljava/lang/String;Lorg/bukkit/metadata/MetadataValue;)V",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn get_metadata(
        &mut self,
        arg0: impl Into<&'mc String>,
    ) -> Result<Vec<crate::metadata::MetadataValue<'mc>>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getMetadata",
            "(Ljava/lang/String;)Ljava/util/List;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let mut list = blackboxmc_java::JavaList::from_raw(&self.jni_ref(), res.l()?)?;
        let size = list.size()?;
        for i in 0..=size {
            let obj = list.get(i)?;
            new_vec.push(crate::metadata::MetadataValue::from_raw(
                &self.jni_ref(),
                obj,
            )?);
        }
        Ok(new_vec)
    }

    pub fn has_metadata(
        &mut self,
        arg0: impl Into<&'mc String>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "hasMetadata",
            "(Ljava/lang/String;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn remove_metadata(
        &mut self,
        arg0: impl Into<&'mc String>,
        arg1: impl Into<&'mc crate::plugin::Plugin<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let val_2 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeMetadata",
            "(Ljava/lang/String;Lorg/bukkit/plugin/Plugin;)V",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn is_locked(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isLocked", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn lock(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getLock", "()Ljava/lang/String;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }

    pub fn set_lock(
        &mut self,
        arg0: impl Into<&'mc String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setLock",
            "(Ljava/lang/String;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn custom_name(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getCustomName",
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

    pub fn set_custom_name(
        &mut self,
        arg0: impl Into<&'mc String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setCustomName",
            "(Ljava/lang/String;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
}
impl<'mc> JNIRaw<'mc> for Furnace<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Into<crate::block::Container<'mc>> for Furnace<'mc> {
    fn into(self) -> crate::block::Container<'mc> {
        crate::block::Container::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
/// Represents a captured state of a dropper.
///
/// This is a representation of an abstract class.
pub struct Dropper<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> Dropper<'mc> {
    pub fn from_raw(
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
    /// Tries to drop a randomly selected item from the dropper's inventory, following the normal behavior of a dropper.
    /// <p>Normal behavior of a dropper is as follows:</p>
    /// <p>If the block that the dropper is facing is an InventoryHolder, the randomly selected ItemStack is placed within that Inventory in the first slot that's available, starting with 0 and counting up. If the inventory is full, nothing happens.</p>
    /// <p>If the block that the dropper is facing is not an InventoryHolder, the randomly selected ItemStack is dropped on the ground in the form of an <a title="interface in org.bukkit.entity" href="../entity/Item.html"><code>Item</code></a>.</p>
    /// <p>If the block represented by this state is no longer a dropper, this will do nothing.</p>
    pub fn drop(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "drop", "()V", &[]);
        self.jni_ref().translate_error(res)?;
        Ok(())
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

    pub fn snapshot_inventory(
        &mut self,
    ) -> Result<crate::inventory::Inventory<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getSnapshotInventory",
            "()Lorg/bukkit/inventory/Inventory;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::Inventory::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn persistent_data_container(
        &mut self,
    ) -> Result<crate::persistence::PersistentDataContainer<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getPersistentDataContainer",
            "()Lorg/bukkit/persistence/PersistentDataContainer;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::persistence::PersistentDataContainer::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn set_type(
        &mut self,
        arg0: impl Into<&'mc crate::Material<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setType",
            "(Lorg/bukkit/Material;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn world(&mut self) -> Result<crate::World<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getWorld", "()Lorg/bukkit/World;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        crate::World::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn data(
        &mut self,
    ) -> Result<crate::material::MaterialData<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getData",
            "()Lorg/bukkit/material/MaterialData;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::material::MaterialData::from_raw(&self.jni_ref(), unsafe {
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

    pub fn block_data(
        &mut self,
    ) -> Result<crate::block::data::BlockData<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getBlockData",
            "()Lorg/bukkit/block/data/BlockData;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::block::data::BlockData::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn light_level(&mut self) -> Result<i8, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getLightLevel", "()B", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.b().unwrap())
    }

    pub fn x(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getX", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }

    pub fn y(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getY", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }

    pub fn z(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getZ", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }

    pub fn chunk(&mut self) -> Result<crate::Chunk<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getChunk", "()Lorg/bukkit/Chunk;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        crate::Chunk::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn set_data(
        &mut self,
        arg0: impl Into<&'mc crate::material::MaterialData<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setData",
            "(Lorg/bukkit/material/MaterialData;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn set_block_data(
        &mut self,
        arg0: impl Into<&'mc crate::block::data::BlockData<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setBlockData",
            "(Lorg/bukkit/block/data/BlockData;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    #[deprecated]

    pub fn raw_data(&mut self) -> Result<i8, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getRawData", "()B", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.b().unwrap())
    }
    #[deprecated]

    pub fn set_raw_data(&mut self, arg0: i8) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Byte(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setRawData",
            "(B)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn is_placed(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isPlaced", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn update(
        &mut self,
        arg0: std::option::Option<bool>,
        arg1: std::option::Option<bool>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        // -1
        let val_1 = jni::objects::JValueGen::Bool(arg0.unwrap().into());
        // -1
        let val_2 = jni::objects::JValueGen::Bool(arg1.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "update",
            "(ZZ)Z",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn location(&mut self) -> Result<crate::Location<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getLocation",
            "()Lorg/bukkit/Location;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::Location::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn get_type(&mut self) -> Result<crate::Material<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getType",
            "()Lorg/bukkit/Material;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant =
            self.jni_ref()
                .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
        let variant_str = self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::Material::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::Material::from_string(variant_str).unwrap(),
        )
    }

    pub fn set_metadata(
        &mut self,
        arg0: impl Into<&'mc String>,
        arg1: impl Into<&'mc crate::metadata::MetadataValue<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let val_2 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setMetadata",
            "(Ljava/lang/String;Lorg/bukkit/metadata/MetadataValue;)V",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn get_metadata(
        &mut self,
        arg0: impl Into<&'mc String>,
    ) -> Result<Vec<crate::metadata::MetadataValue<'mc>>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getMetadata",
            "(Ljava/lang/String;)Ljava/util/List;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let mut list = blackboxmc_java::JavaList::from_raw(&self.jni_ref(), res.l()?)?;
        let size = list.size()?;
        for i in 0..=size {
            let obj = list.get(i)?;
            new_vec.push(crate::metadata::MetadataValue::from_raw(
                &self.jni_ref(),
                obj,
            )?);
        }
        Ok(new_vec)
    }

    pub fn has_metadata(
        &mut self,
        arg0: impl Into<&'mc String>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "hasMetadata",
            "(Ljava/lang/String;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn remove_metadata(
        &mut self,
        arg0: impl Into<&'mc String>,
        arg1: impl Into<&'mc crate::plugin::Plugin<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let val_2 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeMetadata",
            "(Ljava/lang/String;Lorg/bukkit/plugin/Plugin;)V",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn is_locked(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isLocked", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn lock(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getLock", "()Ljava/lang/String;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }

    pub fn set_lock(
        &mut self,
        arg0: impl Into<&'mc String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setLock",
            "(Ljava/lang/String;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn custom_name(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getCustomName",
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

    pub fn set_custom_name(
        &mut self,
        arg0: impl Into<&'mc String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setCustomName",
            "(Ljava/lang/String;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn seed(&mut self) -> Result<i64, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getSeed", "()J", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.j().unwrap())
    }

    pub fn set_seed(&mut self, arg0: i64) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Long(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setSeed",
            "(J)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn set_loot_table(
        &mut self,
        arg0: impl Into<&'mc crate::loot::LootTable<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setLootTable",
            "(Lorg/bukkit/loot/LootTable;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn loot_table(
        &mut self,
    ) -> Result<crate::loot::LootTable<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getLootTable",
            "()Lorg/bukkit/loot/LootTable;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::loot::LootTable::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
}
impl<'mc> JNIRaw<'mc> for Dropper<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Into<crate::block::Container<'mc>> for Dropper<'mc> {
    fn into(self) -> crate::block::Container<'mc> {
        crate::block::Container::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
impl<'mc> Into<crate::loot::Lootable<'mc>> for Dropper<'mc> {
    fn into(self) -> crate::loot::Lootable<'mc> {
        crate::loot::Lootable::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
/// Represents a captured state of suspicious sand.
///
/// This is a representation of an abstract class.
pub struct SuspiciousSand<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> SuspiciousSand<'mc> {
    pub fn from_raw(
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

    pub fn seed(&mut self) -> Result<i64, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getSeed", "()J", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.j().unwrap())
    }

    pub fn set_seed(&mut self, arg0: i64) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Long(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setSeed",
            "(J)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn set_loot_table(
        &mut self,
        arg0: impl Into<&'mc crate::loot::LootTable<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setLootTable",
            "(Lorg/bukkit/loot/LootTable;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn loot_table(
        &mut self,
    ) -> Result<crate::loot::LootTable<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getLootTable",
            "()Lorg/bukkit/loot/LootTable;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::loot::LootTable::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn persistent_data_container(
        &mut self,
    ) -> Result<crate::persistence::PersistentDataContainer<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getPersistentDataContainer",
            "()Lorg/bukkit/persistence/PersistentDataContainer;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::persistence::PersistentDataContainer::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn set_type(
        &mut self,
        arg0: impl Into<&'mc crate::Material<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setType",
            "(Lorg/bukkit/Material;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn world(&mut self) -> Result<crate::World<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getWorld", "()Lorg/bukkit/World;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        crate::World::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn data(
        &mut self,
    ) -> Result<crate::material::MaterialData<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getData",
            "()Lorg/bukkit/material/MaterialData;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::material::MaterialData::from_raw(&self.jni_ref(), unsafe {
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

    pub fn block_data(
        &mut self,
    ) -> Result<crate::block::data::BlockData<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getBlockData",
            "()Lorg/bukkit/block/data/BlockData;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::block::data::BlockData::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn light_level(&mut self) -> Result<i8, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getLightLevel", "()B", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.b().unwrap())
    }

    pub fn x(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getX", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }

    pub fn y(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getY", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }

    pub fn z(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getZ", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }

    pub fn chunk(&mut self) -> Result<crate::Chunk<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getChunk", "()Lorg/bukkit/Chunk;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        crate::Chunk::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn set_data(
        &mut self,
        arg0: impl Into<&'mc crate::material::MaterialData<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setData",
            "(Lorg/bukkit/material/MaterialData;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn set_block_data(
        &mut self,
        arg0: impl Into<&'mc crate::block::data::BlockData<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setBlockData",
            "(Lorg/bukkit/block/data/BlockData;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    #[deprecated]

    pub fn raw_data(&mut self) -> Result<i8, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getRawData", "()B", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.b().unwrap())
    }
    #[deprecated]

    pub fn set_raw_data(&mut self, arg0: i8) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Byte(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setRawData",
            "(B)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn is_placed(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isPlaced", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn update(
        &mut self,
        arg0: std::option::Option<bool>,
        arg1: std::option::Option<bool>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        // -1
        let val_1 = jni::objects::JValueGen::Bool(arg0.unwrap().into());
        // -1
        let val_2 = jni::objects::JValueGen::Bool(arg1.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "update",
            "(ZZ)Z",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn location(&mut self) -> Result<crate::Location<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getLocation",
            "()Lorg/bukkit/Location;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::Location::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn get_type(&mut self) -> Result<crate::Material<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getType",
            "()Lorg/bukkit/Material;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant =
            self.jni_ref()
                .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
        let variant_str = self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::Material::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::Material::from_string(variant_str).unwrap(),
        )
    }

    pub fn set_metadata(
        &mut self,
        arg0: impl Into<&'mc String>,
        arg1: impl Into<&'mc crate::metadata::MetadataValue<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let val_2 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setMetadata",
            "(Ljava/lang/String;Lorg/bukkit/metadata/MetadataValue;)V",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn get_metadata(
        &mut self,
        arg0: impl Into<&'mc String>,
    ) -> Result<Vec<crate::metadata::MetadataValue<'mc>>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getMetadata",
            "(Ljava/lang/String;)Ljava/util/List;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let mut list = blackboxmc_java::JavaList::from_raw(&self.jni_ref(), res.l()?)?;
        let size = list.size()?;
        for i in 0..=size {
            let obj = list.get(i)?;
            new_vec.push(crate::metadata::MetadataValue::from_raw(
                &self.jni_ref(),
                obj,
            )?);
        }
        Ok(new_vec)
    }

    pub fn has_metadata(
        &mut self,
        arg0: impl Into<&'mc String>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "hasMetadata",
            "(Ljava/lang/String;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn remove_metadata(
        &mut self,
        arg0: impl Into<&'mc String>,
        arg1: impl Into<&'mc crate::plugin::Plugin<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let val_2 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeMetadata",
            "(Ljava/lang/String;Lorg/bukkit/plugin/Plugin;)V",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
}
impl<'mc> JNIRaw<'mc> for SuspiciousSand<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Into<crate::block::BrushableBlock<'mc>> for SuspiciousSand<'mc> {
    fn into(self) -> crate::block::BrushableBlock<'mc> {
        crate::block::BrushableBlock::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
/// Represents a captured state of a conduit.
///
/// This is a representation of an abstract class.
pub struct Conduit<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> Conduit<'mc> {
    pub fn from_raw(
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

    pub fn persistent_data_container(
        &mut self,
    ) -> Result<crate::persistence::PersistentDataContainer<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getPersistentDataContainer",
            "()Lorg/bukkit/persistence/PersistentDataContainer;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::persistence::PersistentDataContainer::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn set_type(
        &mut self,
        arg0: impl Into<&'mc crate::Material<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setType",
            "(Lorg/bukkit/Material;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn world(&mut self) -> Result<crate::World<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getWorld", "()Lorg/bukkit/World;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        crate::World::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn data(
        &mut self,
    ) -> Result<crate::material::MaterialData<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getData",
            "()Lorg/bukkit/material/MaterialData;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::material::MaterialData::from_raw(&self.jni_ref(), unsafe {
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

    pub fn block_data(
        &mut self,
    ) -> Result<crate::block::data::BlockData<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getBlockData",
            "()Lorg/bukkit/block/data/BlockData;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::block::data::BlockData::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn light_level(&mut self) -> Result<i8, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getLightLevel", "()B", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.b().unwrap())
    }

    pub fn x(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getX", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }

    pub fn y(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getY", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }

    pub fn z(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getZ", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }

    pub fn chunk(&mut self) -> Result<crate::Chunk<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getChunk", "()Lorg/bukkit/Chunk;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        crate::Chunk::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn set_data(
        &mut self,
        arg0: impl Into<&'mc crate::material::MaterialData<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setData",
            "(Lorg/bukkit/material/MaterialData;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn set_block_data(
        &mut self,
        arg0: impl Into<&'mc crate::block::data::BlockData<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setBlockData",
            "(Lorg/bukkit/block/data/BlockData;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    #[deprecated]

    pub fn raw_data(&mut self) -> Result<i8, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getRawData", "()B", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.b().unwrap())
    }
    #[deprecated]

    pub fn set_raw_data(&mut self, arg0: i8) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Byte(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setRawData",
            "(B)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn is_placed(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isPlaced", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn update(
        &mut self,
        arg0: std::option::Option<bool>,
        arg1: std::option::Option<bool>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        // -1
        let val_1 = jni::objects::JValueGen::Bool(arg0.unwrap().into());
        // -1
        let val_2 = jni::objects::JValueGen::Bool(arg1.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "update",
            "(ZZ)Z",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn location(&mut self) -> Result<crate::Location<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getLocation",
            "()Lorg/bukkit/Location;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::Location::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn get_type(&mut self) -> Result<crate::Material<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getType",
            "()Lorg/bukkit/Material;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant =
            self.jni_ref()
                .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
        let variant_str = self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::Material::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::Material::from_string(variant_str).unwrap(),
        )
    }

    pub fn set_metadata(
        &mut self,
        arg0: impl Into<&'mc String>,
        arg1: impl Into<&'mc crate::metadata::MetadataValue<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let val_2 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setMetadata",
            "(Ljava/lang/String;Lorg/bukkit/metadata/MetadataValue;)V",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn get_metadata(
        &mut self,
        arg0: impl Into<&'mc String>,
    ) -> Result<Vec<crate::metadata::MetadataValue<'mc>>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getMetadata",
            "(Ljava/lang/String;)Ljava/util/List;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let mut list = blackboxmc_java::JavaList::from_raw(&self.jni_ref(), res.l()?)?;
        let size = list.size()?;
        for i in 0..=size {
            let obj = list.get(i)?;
            new_vec.push(crate::metadata::MetadataValue::from_raw(
                &self.jni_ref(),
                obj,
            )?);
        }
        Ok(new_vec)
    }

    pub fn has_metadata(
        &mut self,
        arg0: impl Into<&'mc String>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "hasMetadata",
            "(Ljava/lang/String;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn remove_metadata(
        &mut self,
        arg0: impl Into<&'mc String>,
        arg1: impl Into<&'mc crate::plugin::Plugin<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let val_2 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeMetadata",
            "(Ljava/lang/String;Lorg/bukkit/plugin/Plugin;)V",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
}
impl<'mc> JNIRaw<'mc> for Conduit<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Into<crate::block::TileState<'mc>> for Conduit<'mc> {
    fn into(self) -> crate::block::TileState<'mc> {
        crate::block::TileState::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
/// Represents a captured state of a beacon.
///
/// This is a representation of an abstract class.
pub struct Beacon<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> Beacon<'mc> {
    pub fn from_raw(
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
    /// Returns the list of players within the beacon's range of effect.
    /// <p>This will return an empty list if the block represented by this state is no longer a beacon.</p>
    pub fn entities_in_range(
        &mut self,
    ) -> Result<Vec<crate::entity::LivingEntity<'mc>>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getEntitiesInRange",
            "()Ljava/util/Collection;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let mut col = blackboxmc_java::JavaCollection::from_raw(&self.jni_ref(), res.l()?)?;
        let mut iter = blackboxmc_java::JavaIterator::from_raw(&self.jni_ref(), col.iterator()?)?;
        while iter.has_next()? {
            let obj = iter.next()?;
            new_vec.push(crate::entity::LivingEntity::from_raw(&self.jni_ref(), obj)?);
        }
        Ok(new_vec)
    }
    /// Returns the tier of the beacon pyramid (0-4). The tier refers to the beacon's power level, based on how many layers of blocks are in the pyramid. Tier 1 refers to a beacon with one layer of 9 blocks under it.
    pub fn tier(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getTier", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }
    /// Returns the primary effect set on the beacon
    pub fn primary_effect(
        &mut self,
    ) -> Result<crate::potion::PotionEffect<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getPrimaryEffect",
            "()Lorg/bukkit/potion/PotionEffect;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::potion::PotionEffect::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Set the primary effect on this beacon, or null to clear.
    pub fn set_primary_effect(
        &mut self,
        arg0: impl Into<&'mc crate::potion::PotionEffectType<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setPrimaryEffect",
            "(Lorg/bukkit/potion/PotionEffectType;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Returns the secondary effect set on the beacon.
    pub fn secondary_effect(
        &mut self,
    ) -> Result<crate::potion::PotionEffect<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getSecondaryEffect",
            "()Lorg/bukkit/potion/PotionEffect;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::potion::PotionEffect::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Set the secondary effect on this beacon, or null to clear. Note that tier must be &gt;= 4 for this effect to be active.
    pub fn set_secondary_effect(
        &mut self,
        arg0: impl Into<&'mc crate::potion::PotionEffectType<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setSecondaryEffect",
            "(Lorg/bukkit/potion/PotionEffectType;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn persistent_data_container(
        &mut self,
    ) -> Result<crate::persistence::PersistentDataContainer<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getPersistentDataContainer",
            "()Lorg/bukkit/persistence/PersistentDataContainer;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::persistence::PersistentDataContainer::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn set_type(
        &mut self,
        arg0: impl Into<&'mc crate::Material<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setType",
            "(Lorg/bukkit/Material;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn world(&mut self) -> Result<crate::World<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getWorld", "()Lorg/bukkit/World;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        crate::World::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn data(
        &mut self,
    ) -> Result<crate::material::MaterialData<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getData",
            "()Lorg/bukkit/material/MaterialData;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::material::MaterialData::from_raw(&self.jni_ref(), unsafe {
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

    pub fn block_data(
        &mut self,
    ) -> Result<crate::block::data::BlockData<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getBlockData",
            "()Lorg/bukkit/block/data/BlockData;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::block::data::BlockData::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn light_level(&mut self) -> Result<i8, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getLightLevel", "()B", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.b().unwrap())
    }

    pub fn x(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getX", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }

    pub fn y(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getY", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }

    pub fn z(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getZ", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }

    pub fn chunk(&mut self) -> Result<crate::Chunk<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getChunk", "()Lorg/bukkit/Chunk;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        crate::Chunk::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn set_data(
        &mut self,
        arg0: impl Into<&'mc crate::material::MaterialData<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setData",
            "(Lorg/bukkit/material/MaterialData;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn set_block_data(
        &mut self,
        arg0: impl Into<&'mc crate::block::data::BlockData<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setBlockData",
            "(Lorg/bukkit/block/data/BlockData;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    #[deprecated]

    pub fn raw_data(&mut self) -> Result<i8, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getRawData", "()B", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.b().unwrap())
    }
    #[deprecated]

    pub fn set_raw_data(&mut self, arg0: i8) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Byte(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setRawData",
            "(B)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn is_placed(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isPlaced", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn update(
        &mut self,
        arg0: std::option::Option<bool>,
        arg1: std::option::Option<bool>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        // -1
        let val_1 = jni::objects::JValueGen::Bool(arg0.unwrap().into());
        // -1
        let val_2 = jni::objects::JValueGen::Bool(arg1.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "update",
            "(ZZ)Z",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn location(&mut self) -> Result<crate::Location<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getLocation",
            "()Lorg/bukkit/Location;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::Location::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn get_type(&mut self) -> Result<crate::Material<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getType",
            "()Lorg/bukkit/Material;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant =
            self.jni_ref()
                .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
        let variant_str = self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::Material::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::Material::from_string(variant_str).unwrap(),
        )
    }

    pub fn set_metadata(
        &mut self,
        arg0: impl Into<&'mc String>,
        arg1: impl Into<&'mc crate::metadata::MetadataValue<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let val_2 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setMetadata",
            "(Ljava/lang/String;Lorg/bukkit/metadata/MetadataValue;)V",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn get_metadata(
        &mut self,
        arg0: impl Into<&'mc String>,
    ) -> Result<Vec<crate::metadata::MetadataValue<'mc>>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getMetadata",
            "(Ljava/lang/String;)Ljava/util/List;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let mut list = blackboxmc_java::JavaList::from_raw(&self.jni_ref(), res.l()?)?;
        let size = list.size()?;
        for i in 0..=size {
            let obj = list.get(i)?;
            new_vec.push(crate::metadata::MetadataValue::from_raw(
                &self.jni_ref(),
                obj,
            )?);
        }
        Ok(new_vec)
    }

    pub fn has_metadata(
        &mut self,
        arg0: impl Into<&'mc String>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "hasMetadata",
            "(Ljava/lang/String;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn remove_metadata(
        &mut self,
        arg0: impl Into<&'mc String>,
        arg1: impl Into<&'mc crate::plugin::Plugin<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let val_2 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeMetadata",
            "(Ljava/lang/String;Lorg/bukkit/plugin/Plugin;)V",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn is_locked(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isLocked", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn lock(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getLock", "()Ljava/lang/String;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }

    pub fn set_lock(
        &mut self,
        arg0: impl Into<&'mc String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setLock",
            "(Ljava/lang/String;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn custom_name(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getCustomName",
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

    pub fn set_custom_name(
        &mut self,
        arg0: impl Into<&'mc String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setCustomName",
            "(Ljava/lang/String;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
}
impl<'mc> JNIRaw<'mc> for Beacon<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Into<crate::block::TileState<'mc>> for Beacon<'mc> {
    fn into(self) -> crate::block::TileState<'mc> {
        crate::block::TileState::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
impl<'mc> Into<crate::block::Lockable<'mc>> for Beacon<'mc> {
    fn into(self) -> crate::block::Lockable<'mc> {
        crate::block::Lockable::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
impl<'mc> Into<crate::Nameable<'mc>> for Beacon<'mc> {
    fn into(self) -> crate::Nameable<'mc> {
        crate::Nameable::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
/// Represents a block. This is a live object, and only one Block may exist for any given location in a world. The state of the block may change concurrently to your own handling of it; use block.getState() to get a snapshot state of a block which will not be modified.
///
/// Note that parts of this class which require access to the world at large (i.e. lighting and power) may not be able to be safely accessed during world generation when used in cases like BlockPhysicsEvent!!!!
///
/// This is a representation of an abstract class.
pub struct Block<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> Block<'mc> {
    pub fn from_raw(
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
    /// Sets the type of this block
    /// Sets the type of this block
    ///
    /// Note that applyPhysics = false is not in general safe. It should only be used when you need to avoid triggering a physics update of neighboring blocks, for example when creating a <a href="data/Bisected.html" title="interface in org.bukkit.block.data"><code>Bisected</code></a> block. If you are using a custom populator, then this parameter may also be required to prevent triggering infinite chunk loads on border blocks. This method should NOT be used to "hack" physics by placing blocks in impossible locations. Such blocks are liable to be removed on various events such as world upgrades. Furthermore setting large amounts of such blocks in close proximity may overload the server physics engine if an update is triggered at a later point. If this occurs, the resulting behavior is undefined.
    pub fn set_type_with_material(
        &mut self,
        arg0: std::option::Option<impl Into<&'mc crate::Material<'mc>>>,
        arg1: std::option::Option<bool>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 =
            unsafe { jni::objects::JObject::from_raw(arg0.unwrap().into().jni_object().clone()) };
        // 0
        let val_2 = jni::objects::JValueGen::Bool(arg1.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setType",
            "(Lorg/bukkit/Material;Z)V",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Gets the approximate bounding box for this block.
    /// <p>This isn't exact as some blocks <a href="data/type/Stairs.html" title="interface in org.bukkit.block.data.type"><code>Stairs</code></a> contain many bounding boxes to establish their complete form. Also, the box may not be exactly the same as the collision shape (such as cactus, which is 16/16 of a block with 15/16 collisional bounds). This method will return an empty bounding box if the geometric shape of the block is empty (such as air blocks).</p>
    pub fn bounding_box(
        &mut self,
    ) -> Result<crate::util::BoundingBox<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getBoundingBox",
            "()Lorg/bukkit/util/BoundingBox;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::util::BoundingBox::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Gets the world which contains this Block
    pub fn world(&mut self) -> Result<crate::World<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getWorld", "()Lorg/bukkit/World;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        crate::World::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    #[deprecated]
    /// <span class="deprecated-label">Deprecated.</span>
    /// <div class="deprecation-comment">
    /// Magic value
    /// </div>
    /// Magic value
    ///
    /// Gets the metadata for this block
    pub fn data(&mut self) -> Result<i8, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getData", "()B", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.b().unwrap())
    }
    /// Gets the complete block data for this block
    pub fn block_data(
        &mut self,
    ) -> Result<crate::block::data::BlockData<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getBlockData",
            "()Lorg/bukkit/block/data/BlockData;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::block::data::BlockData::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Gets the light level between 0-15
    pub fn light_level(&mut self) -> Result<i8, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getLightLevel", "()B", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.b().unwrap())
    }
    /// Gets the x-coordinate of this block
    pub fn x(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getX", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }
    /// Gets the y-coordinate of this block
    pub fn y(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getY", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }
    /// Gets the z-coordinate of this block
    pub fn z(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getZ", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }
    /// Gets the chunk which contains this block
    pub fn chunk(&mut self) -> Result<crate::Chunk<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getChunk", "()Lorg/bukkit/Chunk;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        crate::Chunk::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Sets the complete data for this block
    /// Sets the complete data for this block
    ///
    /// Note that applyPhysics = false is not in general safe. It should only be used when you need to avoid triggering a physics update of neighboring blocks, for example when creating a <a title="interface in org.bukkit.block.data" href="data/Bisected.html"><code>Bisected</code></a> block. If you are using a custom populator, then this parameter may also be required to prevent triggering infinite chunk loads on border blocks. This method should NOT be used to "hack" physics by placing blocks in impossible locations. Such blocks are liable to be removed on various events such as world upgrades. Furthermore setting large amounts of such blocks in close proximity may overload the server physics engine if an update is triggered at a later point. If this occurs, the resulting behavior is undefined.
    pub fn set_block_data_with_block_data(
        &mut self,
        arg0: std::option::Option<impl Into<&'mc crate::block::data::BlockData<'mc>>>,
        arg1: std::option::Option<bool>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 =
            unsafe { jni::objects::JObject::from_raw(arg0.unwrap().into().jni_object().clone()) };
        // 0
        let val_2 = jni::objects::JValueGen::Bool(arg1.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setBlockData",
            "(Lorg/bukkit/block/data/BlockData;Z)V",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Returns if the given item is a preferred choice to break this Block. In some cases this determines if a block will drop anything or extra loot.
    pub fn is_preferred_tool(
        &mut self,
        arg0: impl Into<&'mc crate::inventory::ItemStack<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "isPreferredTool",
            "(Lorg/bukkit/inventory/ItemStack;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    /// Returns the reaction of the block when moved by a piston
    pub fn piston_move_reaction(
        &mut self,
    ) -> Result<crate::block::PistonMoveReaction<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getPistonMoveReaction",
            "()Lorg/bukkit/block/PistonMoveReaction;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant =
            self.jni_ref()
                .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
        let variant_str = self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::block::PistonMoveReaction::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::block::PistonMoveReaction::from_string(variant_str).unwrap(),
        )
    }
    /// Gets the face relation of this block compared to the given block.
    /// <p>For example:</p>
    /// <pre><code>
    /// Block current = world.getBlockAt(100, 100, 100);
    /// Block target = world.getBlockAt(100, 101, 100);
    /// current.getFace(target) == BlockFace.Up;
    /// </code></pre>
    ///
    /// If the given block is not connected to this block, null may be returned
    pub fn get_face(
        &mut self,
        arg0: impl Into<&'mc crate::block::Block<'mc>>,
    ) -> Result<crate::block::BlockFace<'mc>, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getFace",
            "(Lorg/bukkit/block/Block;)Lorg/bukkit/block/BlockFace;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant =
            self.jni_ref()
                .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
        let variant_str = self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::block::BlockFace::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::block::BlockFace::from_string(variant_str).unwrap(),
        )
    }
    /// Performs a ray trace that checks for collision with this specific block in its current state using its precise collision shape.
    pub fn ray_trace(
        &mut self,
        arg0: impl Into<&'mc crate::Location<'mc>>,
        arg1: impl Into<&'mc crate::util::Vector<'mc>>,
        arg2: f64,
        arg3: impl Into<&'mc crate::FluidCollisionMode<'mc>>,
    ) -> Result<crate::util::RayTraceResult<'mc>, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let val_2 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone()) };
        let val_3 = jni::objects::JValueGen::Double(arg2.into());
        let val_4 = unsafe { jni::objects::JObject::from_raw(arg3.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(&self.jni_object(),"rayTrace","(Lorg/bukkit/Location;Lorg/bukkit/util/Vector;DLorg/bukkit/FluidCollisionMode;)Lorg/bukkit/util/RayTraceResult;",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2),jni::objects::JValueGen::from(&val_3),jni::objects::JValueGen::from(&val_4)]);
        let res = self.jni_ref().translate_error(res)?;
        crate::util::RayTraceResult::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Returns the biome that this block resides in
    pub fn biome(&mut self) -> Result<crate::block::Biome<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getBiome",
            "()Lorg/bukkit/block/Biome;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant =
            self.jni_ref()
                .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
        let variant_str = self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::block::Biome::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::block::Biome::from_string(variant_str).unwrap(),
        )
    }
    /// Sets the biome that this block resides in
    pub fn set_biome(
        &mut self,
        arg0: impl Into<&'mc crate::block::Biome<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setBiome",
            "(Lorg/bukkit/block/Biome;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Gets the block at the given offsets
    /// Gets the block at the given face
    /// <p>This method is equal to getRelative(face, 1)</p>
    /// Gets the block at the given distance of the given face
    /// <p>For example, the following method places water at 100,102,100; two blocks above 100,100,100.</p>
    /// <pre> Block block = world.getBlockAt(100, 100, 100);
    /// Block shower = block.getRelative(BlockFace.UP, 2);
    /// shower.setType(Material.WATER);
    /// </pre>
    pub fn get_relative_with_block_face(
        &mut self,
        arg0: std::option::Option<i32>,
        arg1: std::option::Option<i32>,
        arg2: std::option::Option<i32>,
    ) -> Result<crate::block::Block<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.unwrap().into());
        let val_2 = jni::objects::JValueGen::Int(arg1.unwrap().into());
        let val_3 = jni::objects::JValueGen::Int(arg2.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getRelative",
            "(III)Lorg/bukkit/block/Block;",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
                jni::objects::JValueGen::from(&val_3),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::block::Block::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Gets the temperature of this block.
    /// <p>If the raw biome temperature without adjusting for height effects is required then please use <a href="../World.html#getTemperature(int,int)"><code>World.getTemperature(int, int)</code></a>.</p>
    pub fn temperature(&mut self) -> Result<f64, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getTemperature", "()D", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.d().unwrap())
    }
    /// Gets the humidity of the biome of this block
    pub fn humidity(&mut self) -> Result<f64, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getHumidity", "()D", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.d().unwrap())
    }
    /// Get the amount of light at this block from the sky.
    /// <p>Any light given from other sources (such as blocks like torches) will be ignored.</p>
    pub fn light_from_sky(&mut self) -> Result<i8, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getLightFromSky", "()B", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.b().unwrap())
    }
    /// Get the amount of light at this block from nearby blocks.
    /// <p>Any light given from other sources (such as the sun) will be ignored.</p>
    pub fn light_from_blocks(&mut self) -> Result<i8, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getLightFromBlocks", "()B", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.b().unwrap())
    }
    /// Returns true if the block is being powered by Redstone.
    pub fn is_block_powered(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isBlockPowered", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    /// Returns true if the block is being indirectly powered by Redstone.
    pub fn is_block_indirectly_powered(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isBlockIndirectlyPowered", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    /// Returns true if the block face is being powered by Redstone.
    pub fn is_block_face_powered(
        &mut self,
        arg0: impl Into<&'mc crate::block::BlockFace<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "isBlockFacePowered",
            "(Lorg/bukkit/block/BlockFace;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    /// Returns true if the block face is being indirectly powered by Redstone.
    pub fn is_block_face_indirectly_powered(
        &mut self,
        arg0: impl Into<&'mc crate::block::BlockFace<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "isBlockFaceIndirectlyPowered",
            "(Lorg/bukkit/block/BlockFace;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    /// Returns the redstone power being provided to this block face
    /// Returns the redstone power being provided to this block
    pub fn block_power(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getBlockPower", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }
    /// Checks if this block is liquid.
    /// <p>A block is considered liquid when <a href="#getType()"><code>getType()</code></a> returns <a href="../Material.html#WATER"><code>Material.WATER</code></a> or <a href="../Material.html#LAVA"><code>Material.LAVA</code></a>.</p>
    pub fn is_liquid(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isLiquid", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    /// Breaks the block and spawns items as if a player had digged it regardless of the tool.
    /// Breaks the block and spawns items as if a player had digged it with a specific tool
    pub fn break_naturally(
        &mut self,
        arg0: std::option::Option<impl Into<&'mc crate::inventory::ItemStack<'mc>>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 =
            unsafe { jni::objects::JObject::from_raw(arg0.unwrap().into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "breakNaturally",
            "(Lorg/bukkit/inventory/ItemStack;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    /// Simulate bone meal application to this block (if possible).
    pub fn apply_bone_meal(
        &mut self,
        arg0: impl Into<&'mc crate::block::BlockFace<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "applyBoneMeal",
            "(Lorg/bukkit/block/BlockFace;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    /// Returns a list of items which would drop by destroying this block
    /// Returns a list of items which would drop by destroying this block with a specific tool
    /// Returns a list of items which would drop by the entity destroying this block with a specific tool
    pub fn get_drops_with_item_stack(
        &mut self,
        arg0: std::option::Option<impl Into<&'mc crate::inventory::ItemStack<'mc>>>,
        arg1: std::option::Option<impl Into<&'mc crate::entity::Entity<'mc>>>,
    ) -> Result<Vec<crate::inventory::ItemStack<'mc>>, Box<dyn std::error::Error>> {
        let val_1 =
            unsafe { jni::objects::JObject::from_raw(arg0.unwrap().into().jni_object().clone()) };
        let val_2 =
            unsafe { jni::objects::JObject::from_raw(arg1.unwrap().into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getDrops",
            "(Lorg/bukkit/inventory/ItemStack;Lorg/bukkit/entity/Entity;)Ljava/util/Collection;",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let mut col = blackboxmc_java::JavaCollection::from_raw(&self.jni_ref(), res.l()?)?;
        let mut iter = blackboxmc_java::JavaIterator::from_raw(&self.jni_ref(), col.iterator()?)?;
        while iter.has_next()? {
            let obj = iter.next()?;
            new_vec.push(crate::inventory::ItemStack::from_raw(&self.jni_ref(), obj)?);
        }
        Ok(new_vec)
    }
    /// Gets the speed at which the given player would break this block, taking into account tools, potion effects, whether or not the player is in water, enchantments, etc. The returned value is the amount of progress made in breaking the block each tick. When the total breaking progress reaches <code>1.0f</code>, the block is broken. Note that the break speed can change in the course of breaking a block, e.g. if a potion effect is applied or expires, or the player jumps/enters water.
    pub fn get_break_speed(
        &mut self,
        arg0: impl Into<&'mc crate::entity::Player<'mc>>,
    ) -> Result<f32, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getBreakSpeed",
            "(Lorg/bukkit/entity/Player;)F",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.f().unwrap())
    }
    /// Checks if this block is passable.
    /// <p>A block is passable if it has no colliding parts that would prevent players from moving through it.</p>
    /// <p>Examples: Tall grass, flowers, signs, etc. are passable, but open doors, fence gates, trap doors, etc. are not because they still have parts that can be collided with.</p>
    pub fn is_passable(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isPassable", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    /// Gets the collision shape of this block.
    pub fn collision_shape(
        &mut self,
    ) -> Result<crate::util::VoxelShape<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getCollisionShape",
            "()Lorg/bukkit/util/VoxelShape;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::util::VoxelShape::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Checks if this block is a valid placement location for the specified block data.
    pub fn can_place(
        &mut self,
        arg0: impl Into<&'mc crate::block::data::BlockData<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "canPlace",
            "(Lorg/bukkit/block/data/BlockData;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    /// Checks if this block is empty.
    /// <p>A block is considered empty when <a href="#getType()"><code>getType()</code></a> returns <a href="../Material.html#AIR"><code>Material.AIR</code></a>.</p>
    pub fn is_empty(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isEmpty", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    /// Gets the Location of the block
    /// Stores the location of the block in the provided Location object.
    /// <p>If the provided Location is null this method does nothing and returns null.</p>
    pub fn get_location(
        &mut self,
        arg0: impl Into<&'mc crate::Location<'mc>>,
    ) -> Result<crate::Location<'mc>, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getLocation",
            "(Lorg/bukkit/Location;)Lorg/bukkit/Location;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::Location::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Captures the current state of this block. You may then cast that state into any accepted type, such as Furnace or Sign.
    /// <p>The returned object will never be updated, and you are not guaranteed that (for example) a sign is still a sign after you capture its state.</p>
    pub fn state(&mut self) -> Result<crate::block::BlockState<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getState",
            "()Lorg/bukkit/block/BlockState;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::block::BlockState::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Gets the type of this block
    pub fn get_type(&mut self) -> Result<crate::Material<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getType",
            "()Lorg/bukkit/Material;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant =
            self.jni_ref()
                .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
        let variant_str = self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::Material::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::Material::from_string(variant_str).unwrap(),
        )
    }

    pub fn set_metadata(
        &mut self,
        arg0: impl Into<&'mc String>,
        arg1: impl Into<&'mc crate::metadata::MetadataValue<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let val_2 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setMetadata",
            "(Ljava/lang/String;Lorg/bukkit/metadata/MetadataValue;)V",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn get_metadata(
        &mut self,
        arg0: impl Into<&'mc String>,
    ) -> Result<Vec<crate::metadata::MetadataValue<'mc>>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getMetadata",
            "(Ljava/lang/String;)Ljava/util/List;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let mut list = blackboxmc_java::JavaList::from_raw(&self.jni_ref(), res.l()?)?;
        let size = list.size()?;
        for i in 0..=size {
            let obj = list.get(i)?;
            new_vec.push(crate::metadata::MetadataValue::from_raw(
                &self.jni_ref(),
                obj,
            )?);
        }
        Ok(new_vec)
    }

    pub fn has_metadata(
        &mut self,
        arg0: impl Into<&'mc String>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "hasMetadata",
            "(Ljava/lang/String;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn remove_metadata(
        &mut self,
        arg0: impl Into<&'mc String>,
        arg1: impl Into<&'mc crate::plugin::Plugin<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let val_2 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeMetadata",
            "(Ljava/lang/String;Lorg/bukkit/plugin/Plugin;)V",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn translation_key(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getTranslationKey",
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
}
impl<'mc> JNIRaw<'mc> for Block<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Into<crate::metadata::Metadatable<'mc>> for Block<'mc> {
    fn into(self) -> crate::metadata::Metadatable<'mc> {
        crate::metadata::Metadatable::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
impl<'mc> Into<crate::Translatable<'mc>> for Block<'mc> {
    fn into(self) -> crate::Translatable<'mc> {
        crate::Translatable::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
/// Represents a captured state of Bell.
///
/// This is a representation of an abstract class.
pub struct Bell<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> Bell<'mc> {
    pub fn from_raw(
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
    /// Ring this bell. This will call a <a title="class in org.bukkit.event.block" href="../event/block/BellRingEvent.html"><code>BellRingEvent</code></a>.
    /// Ring this bell in the direction that the bell is facing. This will call a <a title="class in org.bukkit.event.block" href="../event/block/BellRingEvent.html"><code>BellRingEvent</code></a>.
    /// Ring this bell. This will call a <a href="../event/block/BellRingEvent.html" title="class in org.bukkit.event.block"><code>BellRingEvent</code></a>.
    /// Ring this bell in the direction that the bell is facing. This will call a <a href="../event/block/BellRingEvent.html" title="class in org.bukkit.event.block"><code>BellRingEvent</code></a>.
    pub fn ring(
        &mut self,
        arg0: std::option::Option<impl Into<&'mc crate::entity::Entity<'mc>>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 =
            unsafe { jni::objects::JObject::from_raw(arg0.unwrap().into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "ring",
            "(Lorg/bukkit/entity/Entity;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    /// Ring this bell. This will call a <a title="class in org.bukkit.event.block" href="../event/block/BellRingEvent.html"><code>BellRingEvent</code></a>.
    /// Ring this bell in the direction that the bell is facing. This will call a <a href="../event/block/BellRingEvent.html" title="class in org.bukkit.event.block"><code>BellRingEvent</code></a>.
    /// Ring this bell. This will call a <a href="../event/block/BellRingEvent.html" title="class in org.bukkit.event.block"><code>BellRingEvent</code></a>.
    /// Ring this bell in the direction that the bell is facing. This will call a <a href="../event/block/BellRingEvent.html" title="class in org.bukkit.event.block"><code>BellRingEvent</code></a>.
    pub fn ring_with_entity(
        &mut self,
        arg0: impl Into<&'mc crate::entity::Entity<'mc>>,
        arg1: std::option::Option<impl Into<&'mc crate::block::BlockFace<'mc>>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let val_2 =
            unsafe { jni::objects::JObject::from_raw(arg1.unwrap().into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "ring",
            "(Lorg/bukkit/entity/Entity;Lorg/bukkit/block/BlockFace;)Z",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    /// Check whether or not this bell is shaking. A bell is considered to be shaking if it was recently rung.
    /// <p>A bell will typically shake for 50 ticks.</p>
    pub fn is_shaking(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isShaking", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    /// Get the amount of ticks since this bell has been shaking, or 0 if the bell is not currently shaking.
    /// <p>A bell will typically shake for 50 ticks.</p>
    pub fn shaking_ticks(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getShakingTicks", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }
    /// Check whether or not this bell is resonating. A bell is considered to be resonating if <a href="#isShaking()"><code>while shaking</code></a>, raiders were detected in the area and are ready to be highlighted to nearby players.
    /// <p>A bell will typically resonate for 40 ticks.</p>
    pub fn is_resonating(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isResonating", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    /// Get the amount of ticks since this bell has been resonating, or 0 if the bell is not currently resonating.
    /// <p>A bell will typically resonate for 40 ticks.</p>
    pub fn resonating_ticks(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getResonatingTicks", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }

    pub fn persistent_data_container(
        &mut self,
    ) -> Result<crate::persistence::PersistentDataContainer<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getPersistentDataContainer",
            "()Lorg/bukkit/persistence/PersistentDataContainer;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::persistence::PersistentDataContainer::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn set_type(
        &mut self,
        arg0: impl Into<&'mc crate::Material<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setType",
            "(Lorg/bukkit/Material;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn world(&mut self) -> Result<crate::World<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getWorld", "()Lorg/bukkit/World;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        crate::World::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn data(
        &mut self,
    ) -> Result<crate::material::MaterialData<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getData",
            "()Lorg/bukkit/material/MaterialData;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::material::MaterialData::from_raw(&self.jni_ref(), unsafe {
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

    pub fn block_data(
        &mut self,
    ) -> Result<crate::block::data::BlockData<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getBlockData",
            "()Lorg/bukkit/block/data/BlockData;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::block::data::BlockData::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn light_level(&mut self) -> Result<i8, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getLightLevel", "()B", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.b().unwrap())
    }

    pub fn x(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getX", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }

    pub fn y(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getY", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }

    pub fn z(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getZ", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }

    pub fn chunk(&mut self) -> Result<crate::Chunk<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getChunk", "()Lorg/bukkit/Chunk;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        crate::Chunk::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn set_data(
        &mut self,
        arg0: impl Into<&'mc crate::material::MaterialData<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setData",
            "(Lorg/bukkit/material/MaterialData;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn set_block_data(
        &mut self,
        arg0: impl Into<&'mc crate::block::data::BlockData<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setBlockData",
            "(Lorg/bukkit/block/data/BlockData;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    #[deprecated]

    pub fn raw_data(&mut self) -> Result<i8, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getRawData", "()B", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.b().unwrap())
    }
    #[deprecated]

    pub fn set_raw_data(&mut self, arg0: i8) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Byte(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setRawData",
            "(B)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn is_placed(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isPlaced", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn update(
        &mut self,
        arg0: std::option::Option<bool>,
        arg1: std::option::Option<bool>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        // -1
        let val_1 = jni::objects::JValueGen::Bool(arg0.unwrap().into());
        // -1
        let val_2 = jni::objects::JValueGen::Bool(arg1.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "update",
            "(ZZ)Z",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn location(&mut self) -> Result<crate::Location<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getLocation",
            "()Lorg/bukkit/Location;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::Location::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn get_type(&mut self) -> Result<crate::Material<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getType",
            "()Lorg/bukkit/Material;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant =
            self.jni_ref()
                .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
        let variant_str = self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::Material::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::Material::from_string(variant_str).unwrap(),
        )
    }

    pub fn set_metadata(
        &mut self,
        arg0: impl Into<&'mc String>,
        arg1: impl Into<&'mc crate::metadata::MetadataValue<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let val_2 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setMetadata",
            "(Ljava/lang/String;Lorg/bukkit/metadata/MetadataValue;)V",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn get_metadata(
        &mut self,
        arg0: impl Into<&'mc String>,
    ) -> Result<Vec<crate::metadata::MetadataValue<'mc>>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getMetadata",
            "(Ljava/lang/String;)Ljava/util/List;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let mut list = blackboxmc_java::JavaList::from_raw(&self.jni_ref(), res.l()?)?;
        let size = list.size()?;
        for i in 0..=size {
            let obj = list.get(i)?;
            new_vec.push(crate::metadata::MetadataValue::from_raw(
                &self.jni_ref(),
                obj,
            )?);
        }
        Ok(new_vec)
    }

    pub fn has_metadata(
        &mut self,
        arg0: impl Into<&'mc String>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "hasMetadata",
            "(Ljava/lang/String;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn remove_metadata(
        &mut self,
        arg0: impl Into<&'mc String>,
        arg1: impl Into<&'mc crate::plugin::Plugin<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let val_2 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeMetadata",
            "(Ljava/lang/String;Lorg/bukkit/plugin/Plugin;)V",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
}
impl<'mc> JNIRaw<'mc> for Bell<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Into<crate::block::TileState<'mc>> for Bell<'mc> {
    fn into(self) -> crate::block::TileState<'mc> {
        crate::block::TileState::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
/// Represents a captured state of a jigsaw.
///
/// This is a representation of an abstract class.
pub struct Jigsaw<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> Jigsaw<'mc> {
    pub fn from_raw(
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

    pub fn persistent_data_container(
        &mut self,
    ) -> Result<crate::persistence::PersistentDataContainer<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getPersistentDataContainer",
            "()Lorg/bukkit/persistence/PersistentDataContainer;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::persistence::PersistentDataContainer::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn set_type(
        &mut self,
        arg0: impl Into<&'mc crate::Material<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setType",
            "(Lorg/bukkit/Material;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn world(&mut self) -> Result<crate::World<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getWorld", "()Lorg/bukkit/World;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        crate::World::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn data(
        &mut self,
    ) -> Result<crate::material::MaterialData<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getData",
            "()Lorg/bukkit/material/MaterialData;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::material::MaterialData::from_raw(&self.jni_ref(), unsafe {
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

    pub fn block_data(
        &mut self,
    ) -> Result<crate::block::data::BlockData<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getBlockData",
            "()Lorg/bukkit/block/data/BlockData;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::block::data::BlockData::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn light_level(&mut self) -> Result<i8, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getLightLevel", "()B", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.b().unwrap())
    }

    pub fn x(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getX", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }

    pub fn y(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getY", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }

    pub fn z(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getZ", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }

    pub fn chunk(&mut self) -> Result<crate::Chunk<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getChunk", "()Lorg/bukkit/Chunk;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        crate::Chunk::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn set_data(
        &mut self,
        arg0: impl Into<&'mc crate::material::MaterialData<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setData",
            "(Lorg/bukkit/material/MaterialData;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn set_block_data(
        &mut self,
        arg0: impl Into<&'mc crate::block::data::BlockData<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setBlockData",
            "(Lorg/bukkit/block/data/BlockData;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    #[deprecated]

    pub fn raw_data(&mut self) -> Result<i8, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getRawData", "()B", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.b().unwrap())
    }
    #[deprecated]

    pub fn set_raw_data(&mut self, arg0: i8) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Byte(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setRawData",
            "(B)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn is_placed(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isPlaced", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn update(
        &mut self,
        arg0: std::option::Option<bool>,
        arg1: std::option::Option<bool>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        // -1
        let val_1 = jni::objects::JValueGen::Bool(arg0.unwrap().into());
        // -1
        let val_2 = jni::objects::JValueGen::Bool(arg1.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "update",
            "(ZZ)Z",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn location(&mut self) -> Result<crate::Location<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getLocation",
            "()Lorg/bukkit/Location;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::Location::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn get_type(&mut self) -> Result<crate::Material<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getType",
            "()Lorg/bukkit/Material;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant =
            self.jni_ref()
                .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
        let variant_str = self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::Material::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::Material::from_string(variant_str).unwrap(),
        )
    }

    pub fn set_metadata(
        &mut self,
        arg0: impl Into<&'mc String>,
        arg1: impl Into<&'mc crate::metadata::MetadataValue<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let val_2 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setMetadata",
            "(Ljava/lang/String;Lorg/bukkit/metadata/MetadataValue;)V",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn get_metadata(
        &mut self,
        arg0: impl Into<&'mc String>,
    ) -> Result<Vec<crate::metadata::MetadataValue<'mc>>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getMetadata",
            "(Ljava/lang/String;)Ljava/util/List;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let mut list = blackboxmc_java::JavaList::from_raw(&self.jni_ref(), res.l()?)?;
        let size = list.size()?;
        for i in 0..=size {
            let obj = list.get(i)?;
            new_vec.push(crate::metadata::MetadataValue::from_raw(
                &self.jni_ref(),
                obj,
            )?);
        }
        Ok(new_vec)
    }

    pub fn has_metadata(
        &mut self,
        arg0: impl Into<&'mc String>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "hasMetadata",
            "(Ljava/lang/String;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn remove_metadata(
        &mut self,
        arg0: impl Into<&'mc String>,
        arg1: impl Into<&'mc crate::plugin::Plugin<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let val_2 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeMetadata",
            "(Ljava/lang/String;Lorg/bukkit/plugin/Plugin;)V",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
}
impl<'mc> JNIRaw<'mc> for Jigsaw<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Into<crate::block::TileState<'mc>> for Jigsaw<'mc> {
    fn into(self) -> crate::block::TileState<'mc> {
        crate::block::TileState::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
/// Represents a captured state of a dispenser.
///
/// This is a representation of an abstract class.
pub struct Dispenser<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> Dispenser<'mc> {
    pub fn from_raw(
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
    /// Gets the BlockProjectileSource object for the dispenser.
    /// <p>If the block represented by this state is no longer a dispenser, this will return null.</p>
    pub fn block_projectile_source(
        &mut self,
    ) -> Result<crate::projectiles::BlockProjectileSource<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getBlockProjectileSource",
            "()Lorg/bukkit/projectiles/BlockProjectileSource;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::projectiles::BlockProjectileSource::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Attempts to dispense the contents of the dispenser.
    /// <p>If the block represented by this state is no longer a dispenser, this will return false.</p>
    pub fn dispense(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "dispense", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
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

    pub fn snapshot_inventory(
        &mut self,
    ) -> Result<crate::inventory::Inventory<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getSnapshotInventory",
            "()Lorg/bukkit/inventory/Inventory;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::Inventory::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn persistent_data_container(
        &mut self,
    ) -> Result<crate::persistence::PersistentDataContainer<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getPersistentDataContainer",
            "()Lorg/bukkit/persistence/PersistentDataContainer;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::persistence::PersistentDataContainer::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn set_type(
        &mut self,
        arg0: impl Into<&'mc crate::Material<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setType",
            "(Lorg/bukkit/Material;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn world(&mut self) -> Result<crate::World<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getWorld", "()Lorg/bukkit/World;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        crate::World::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn data(
        &mut self,
    ) -> Result<crate::material::MaterialData<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getData",
            "()Lorg/bukkit/material/MaterialData;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::material::MaterialData::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Gets the BlockProjectileSource object for the dispenser.
    /// <p>If the block represented by this state is no longer a dispenser, this will return null.</p>
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

    pub fn block_data(
        &mut self,
    ) -> Result<crate::block::data::BlockData<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getBlockData",
            "()Lorg/bukkit/block/data/BlockData;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::block::data::BlockData::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn light_level(&mut self) -> Result<i8, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getLightLevel", "()B", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.b().unwrap())
    }

    pub fn x(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getX", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }

    pub fn y(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getY", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }

    pub fn z(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getZ", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }

    pub fn chunk(&mut self) -> Result<crate::Chunk<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getChunk", "()Lorg/bukkit/Chunk;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        crate::Chunk::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn set_data(
        &mut self,
        arg0: impl Into<&'mc crate::material::MaterialData<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setData",
            "(Lorg/bukkit/material/MaterialData;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn set_block_data(
        &mut self,
        arg0: impl Into<&'mc crate::block::data::BlockData<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setBlockData",
            "(Lorg/bukkit/block/data/BlockData;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    #[deprecated]

    pub fn raw_data(&mut self) -> Result<i8, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getRawData", "()B", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.b().unwrap())
    }
    #[deprecated]

    pub fn set_raw_data(&mut self, arg0: i8) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Byte(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setRawData",
            "(B)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn is_placed(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isPlaced", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn update(
        &mut self,
        arg0: std::option::Option<bool>,
        arg1: std::option::Option<bool>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        // -1
        let val_1 = jni::objects::JValueGen::Bool(arg0.unwrap().into());
        // -1
        let val_2 = jni::objects::JValueGen::Bool(arg1.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "update",
            "(ZZ)Z",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn location(&mut self) -> Result<crate::Location<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getLocation",
            "()Lorg/bukkit/Location;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::Location::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn get_type(&mut self) -> Result<crate::Material<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getType",
            "()Lorg/bukkit/Material;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant =
            self.jni_ref()
                .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
        let variant_str = self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::Material::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::Material::from_string(variant_str).unwrap(),
        )
    }

    pub fn set_metadata(
        &mut self,
        arg0: impl Into<&'mc String>,
        arg1: impl Into<&'mc crate::metadata::MetadataValue<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let val_2 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setMetadata",
            "(Ljava/lang/String;Lorg/bukkit/metadata/MetadataValue;)V",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn get_metadata(
        &mut self,
        arg0: impl Into<&'mc String>,
    ) -> Result<Vec<crate::metadata::MetadataValue<'mc>>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getMetadata",
            "(Ljava/lang/String;)Ljava/util/List;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let mut list = blackboxmc_java::JavaList::from_raw(&self.jni_ref(), res.l()?)?;
        let size = list.size()?;
        for i in 0..=size {
            let obj = list.get(i)?;
            new_vec.push(crate::metadata::MetadataValue::from_raw(
                &self.jni_ref(),
                obj,
            )?);
        }
        Ok(new_vec)
    }

    pub fn has_metadata(
        &mut self,
        arg0: impl Into<&'mc String>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "hasMetadata",
            "(Ljava/lang/String;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn remove_metadata(
        &mut self,
        arg0: impl Into<&'mc String>,
        arg1: impl Into<&'mc crate::plugin::Plugin<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let val_2 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeMetadata",
            "(Ljava/lang/String;Lorg/bukkit/plugin/Plugin;)V",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn is_locked(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isLocked", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn lock(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getLock", "()Ljava/lang/String;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }

    pub fn set_lock(
        &mut self,
        arg0: impl Into<&'mc String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setLock",
            "(Ljava/lang/String;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn custom_name(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getCustomName",
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

    pub fn set_custom_name(
        &mut self,
        arg0: impl Into<&'mc String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setCustomName",
            "(Ljava/lang/String;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn seed(&mut self) -> Result<i64, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getSeed", "()J", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.j().unwrap())
    }

    pub fn set_seed(&mut self, arg0: i64) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Long(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setSeed",
            "(J)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn set_loot_table(
        &mut self,
        arg0: impl Into<&'mc crate::loot::LootTable<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setLootTable",
            "(Lorg/bukkit/loot/LootTable;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn loot_table(
        &mut self,
    ) -> Result<crate::loot::LootTable<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getLootTable",
            "()Lorg/bukkit/loot/LootTable;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::loot::LootTable::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
}
impl<'mc> JNIRaw<'mc> for Dispenser<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Into<crate::block::Container<'mc>> for Dispenser<'mc> {
    fn into(self) -> crate::block::Container<'mc> {
        crate::block::Container::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
impl<'mc> Into<crate::Nameable<'mc>> for Dispenser<'mc> {
    fn into(self) -> crate::Nameable<'mc> {
        crate::Nameable::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
impl<'mc> Into<crate::loot::Lootable<'mc>> for Dispenser<'mc> {
    fn into(self) -> crate::loot::Lootable<'mc> {
        crate::loot::Lootable::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
/// Represents a captured state of suspicious sand or gravel.
///
/// This is a representation of an abstract class.
pub struct BrushableBlock<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> BrushableBlock<'mc> {
    pub fn from_raw(
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
    /// Get the item which will be revealed when the sand is fully brushed away and uncovered.
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
    /// Sets the item which will be revealed when the sand is fully brushed away and uncovered.
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

    pub fn seed(&mut self) -> Result<i64, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getSeed", "()J", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.j().unwrap())
    }

    pub fn set_seed(&mut self, arg0: i64) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Long(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setSeed",
            "(J)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn set_loot_table(
        &mut self,
        arg0: impl Into<&'mc crate::loot::LootTable<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setLootTable",
            "(Lorg/bukkit/loot/LootTable;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn loot_table(
        &mut self,
    ) -> Result<crate::loot::LootTable<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getLootTable",
            "()Lorg/bukkit/loot/LootTable;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::loot::LootTable::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn persistent_data_container(
        &mut self,
    ) -> Result<crate::persistence::PersistentDataContainer<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getPersistentDataContainer",
            "()Lorg/bukkit/persistence/PersistentDataContainer;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::persistence::PersistentDataContainer::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn set_type(
        &mut self,
        arg0: impl Into<&'mc crate::Material<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setType",
            "(Lorg/bukkit/Material;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn world(&mut self) -> Result<crate::World<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getWorld", "()Lorg/bukkit/World;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        crate::World::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn data(
        &mut self,
    ) -> Result<crate::material::MaterialData<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getData",
            "()Lorg/bukkit/material/MaterialData;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::material::MaterialData::from_raw(&self.jni_ref(), unsafe {
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

    pub fn block_data(
        &mut self,
    ) -> Result<crate::block::data::BlockData<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getBlockData",
            "()Lorg/bukkit/block/data/BlockData;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::block::data::BlockData::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn light_level(&mut self) -> Result<i8, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getLightLevel", "()B", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.b().unwrap())
    }

    pub fn x(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getX", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }

    pub fn y(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getY", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }

    pub fn z(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getZ", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }

    pub fn chunk(&mut self) -> Result<crate::Chunk<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getChunk", "()Lorg/bukkit/Chunk;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        crate::Chunk::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn set_data(
        &mut self,
        arg0: impl Into<&'mc crate::material::MaterialData<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setData",
            "(Lorg/bukkit/material/MaterialData;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn set_block_data(
        &mut self,
        arg0: impl Into<&'mc crate::block::data::BlockData<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setBlockData",
            "(Lorg/bukkit/block/data/BlockData;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    #[deprecated]

    pub fn raw_data(&mut self) -> Result<i8, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getRawData", "()B", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.b().unwrap())
    }
    #[deprecated]

    pub fn set_raw_data(&mut self, arg0: i8) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Byte(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setRawData",
            "(B)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn is_placed(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isPlaced", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn update(
        &mut self,
        arg0: std::option::Option<bool>,
        arg1: std::option::Option<bool>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        // -1
        let val_1 = jni::objects::JValueGen::Bool(arg0.unwrap().into());
        // -1
        let val_2 = jni::objects::JValueGen::Bool(arg1.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "update",
            "(ZZ)Z",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn location(&mut self) -> Result<crate::Location<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getLocation",
            "()Lorg/bukkit/Location;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::Location::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn get_type(&mut self) -> Result<crate::Material<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getType",
            "()Lorg/bukkit/Material;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant =
            self.jni_ref()
                .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
        let variant_str = self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::Material::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::Material::from_string(variant_str).unwrap(),
        )
    }

    pub fn set_metadata(
        &mut self,
        arg0: impl Into<&'mc String>,
        arg1: impl Into<&'mc crate::metadata::MetadataValue<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let val_2 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setMetadata",
            "(Ljava/lang/String;Lorg/bukkit/metadata/MetadataValue;)V",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn get_metadata(
        &mut self,
        arg0: impl Into<&'mc String>,
    ) -> Result<Vec<crate::metadata::MetadataValue<'mc>>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getMetadata",
            "(Ljava/lang/String;)Ljava/util/List;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let mut list = blackboxmc_java::JavaList::from_raw(&self.jni_ref(), res.l()?)?;
        let size = list.size()?;
        for i in 0..=size {
            let obj = list.get(i)?;
            new_vec.push(crate::metadata::MetadataValue::from_raw(
                &self.jni_ref(),
                obj,
            )?);
        }
        Ok(new_vec)
    }

    pub fn has_metadata(
        &mut self,
        arg0: impl Into<&'mc String>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "hasMetadata",
            "(Ljava/lang/String;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn remove_metadata(
        &mut self,
        arg0: impl Into<&'mc String>,
        arg1: impl Into<&'mc crate::plugin::Plugin<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let val_2 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeMetadata",
            "(Ljava/lang/String;Lorg/bukkit/plugin/Plugin;)V",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
}
impl<'mc> JNIRaw<'mc> for BrushableBlock<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Into<crate::loot::Lootable<'mc>> for BrushableBlock<'mc> {
    fn into(self) -> crate::loot::Lootable<'mc> {
        crate::loot::Lootable::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
impl<'mc> Into<crate::block::TileState<'mc>> for BrushableBlock<'mc> {
    fn into(self) -> crate::block::TileState<'mc> {
        crate::block::TileState::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
/// Represents a captured state of a sculk shrieker.
///
/// This is a representation of an abstract class.
pub struct SculkShrieker<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> SculkShrieker<'mc> {
    pub fn from_raw(
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
    /// Gets the most recent warning level of this block. When the warning level reaches 4, the shrieker will attempt to spawn a Warden.
    pub fn warning_level(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getWarningLevel", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }
    /// Sets the most recent warning level of this block. When the warning level reaches 4, the shrieker will attempt to spawn a Warden.
    pub fn set_warning_level(&mut self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setWarningLevel",
            "(I)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn persistent_data_container(
        &mut self,
    ) -> Result<crate::persistence::PersistentDataContainer<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getPersistentDataContainer",
            "()Lorg/bukkit/persistence/PersistentDataContainer;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::persistence::PersistentDataContainer::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn set_type(
        &mut self,
        arg0: impl Into<&'mc crate::Material<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setType",
            "(Lorg/bukkit/Material;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn world(&mut self) -> Result<crate::World<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getWorld", "()Lorg/bukkit/World;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        crate::World::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn data(
        &mut self,
    ) -> Result<crate::material::MaterialData<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getData",
            "()Lorg/bukkit/material/MaterialData;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::material::MaterialData::from_raw(&self.jni_ref(), unsafe {
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

    pub fn block_data(
        &mut self,
    ) -> Result<crate::block::data::BlockData<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getBlockData",
            "()Lorg/bukkit/block/data/BlockData;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::block::data::BlockData::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn light_level(&mut self) -> Result<i8, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getLightLevel", "()B", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.b().unwrap())
    }

    pub fn x(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getX", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }

    pub fn y(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getY", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }

    pub fn z(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getZ", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }

    pub fn chunk(&mut self) -> Result<crate::Chunk<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getChunk", "()Lorg/bukkit/Chunk;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        crate::Chunk::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn set_data(
        &mut self,
        arg0: impl Into<&'mc crate::material::MaterialData<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setData",
            "(Lorg/bukkit/material/MaterialData;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn set_block_data(
        &mut self,
        arg0: impl Into<&'mc crate::block::data::BlockData<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setBlockData",
            "(Lorg/bukkit/block/data/BlockData;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    #[deprecated]

    pub fn raw_data(&mut self) -> Result<i8, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getRawData", "()B", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.b().unwrap())
    }
    #[deprecated]

    pub fn set_raw_data(&mut self, arg0: i8) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Byte(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setRawData",
            "(B)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn is_placed(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isPlaced", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn update(
        &mut self,
        arg0: std::option::Option<bool>,
        arg1: std::option::Option<bool>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        // -1
        let val_1 = jni::objects::JValueGen::Bool(arg0.unwrap().into());
        // -1
        let val_2 = jni::objects::JValueGen::Bool(arg1.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "update",
            "(ZZ)Z",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn location(&mut self) -> Result<crate::Location<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getLocation",
            "()Lorg/bukkit/Location;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::Location::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn get_type(&mut self) -> Result<crate::Material<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getType",
            "()Lorg/bukkit/Material;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant =
            self.jni_ref()
                .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
        let variant_str = self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::Material::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::Material::from_string(variant_str).unwrap(),
        )
    }

    pub fn set_metadata(
        &mut self,
        arg0: impl Into<&'mc String>,
        arg1: impl Into<&'mc crate::metadata::MetadataValue<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let val_2 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setMetadata",
            "(Ljava/lang/String;Lorg/bukkit/metadata/MetadataValue;)V",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn get_metadata(
        &mut self,
        arg0: impl Into<&'mc String>,
    ) -> Result<Vec<crate::metadata::MetadataValue<'mc>>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getMetadata",
            "(Ljava/lang/String;)Ljava/util/List;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let mut list = blackboxmc_java::JavaList::from_raw(&self.jni_ref(), res.l()?)?;
        let size = list.size()?;
        for i in 0..=size {
            let obj = list.get(i)?;
            new_vec.push(crate::metadata::MetadataValue::from_raw(
                &self.jni_ref(),
                obj,
            )?);
        }
        Ok(new_vec)
    }

    pub fn has_metadata(
        &mut self,
        arg0: impl Into<&'mc String>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "hasMetadata",
            "(Ljava/lang/String;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn remove_metadata(
        &mut self,
        arg0: impl Into<&'mc String>,
        arg1: impl Into<&'mc crate::plugin::Plugin<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let val_2 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeMetadata",
            "(Ljava/lang/String;Lorg/bukkit/plugin/Plugin;)V",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
}
impl<'mc> JNIRaw<'mc> for SculkShrieker<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Into<crate::block::TileState<'mc>> for SculkShrieker<'mc> {
    fn into(self) -> crate::block::TileState<'mc> {
        crate::block::TileState::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
/// Represents a captured state of a decorated pot.
///
/// This is a representation of an abstract class.
pub struct DecoratedPot<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> DecoratedPot<'mc> {
    pub fn from_raw(
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
    /// Set the sherd on the provided side.
    pub fn set_sherd(
        &mut self,
        arg0: impl Into<&'mc crate::block::DecoratedPotSide<'mc>>,
        arg1: impl Into<&'mc crate::Material<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let val_2 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setSherd",
            "(Lorg/bukkit/block/DecoratedPot$Side;Lorg/bukkit/Material;)V",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Get the sherd on the provided side.
    /// Gets a Map of all sides on this decorated pot and the sherds on them. If a side does not have a specific sherd on it, <a href="../Material.html#BRICK"><code>Material.BRICK</code></a> will be the value of that side.
    pub fn get_sherd(
        &mut self,
        arg0: impl Into<&'mc crate::block::DecoratedPotSide<'mc>>,
    ) -> Result<crate::Material<'mc>, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getSherd",
            "(Lorg/bukkit/block/DecoratedPot$Side;)Lorg/bukkit/Material;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant =
            self.jni_ref()
                .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
        let variant_str = self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::Material::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::Material::from_string(variant_str).unwrap(),
        )
    }
    /// Gets a Map of all sides on this decorated pot and the sherds on them. If a side does not have a specific sherd on it, <a href="../Material.html#BRICK"><code>Material.BRICK</code></a> will be the value of that side.
    pub fn sherds(&mut self) -> Result<blackboxmc_java::JavaMap<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getSherds", "()Ljava/util/Map;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::JavaMap::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    #[deprecated]
    /// <span class="deprecated-label">Deprecated.</span>
    /// <div class="deprecation-comment">
    /// in favor of <a href="#getSherds()"><code>getSherds()</code></a>
    /// </div>
    /// in favor of <a href="#getSherds()"><code>getSherds()</code></a>
    ///
    /// Gets the sherds on this decorated pot. For faces without a specific sherd, <a href="../Material.html#BRICK"><code>Material.BRICK</code></a> is used in its place.
    pub fn shards(&mut self) -> Result<Vec<crate::Material<'mc>>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getShards", "()Ljava/util/List;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let mut list = blackboxmc_java::JavaList::from_raw(&self.jni_ref(), res.l()?)?;
        let size = list.size()?;
        for i in 0..=size {
            let obj = list.get(i)?;
            let variant = self.jni_ref().call_method(
                list.get(i)?,
                "toString",
                "()Ljava/lang/String;",
                &[],
            )?;
            let variant_str = self
                .jni_ref()
                .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
                .to_string_lossy()
                .to_string();
            new_vec.push(crate::Material::from_raw(
                &self.jni_ref(),
                obj,
                crate::Material::from_string(variant_str).unwrap(),
            )?);
        }
        Ok(new_vec)
    }

    pub fn persistent_data_container(
        &mut self,
    ) -> Result<crate::persistence::PersistentDataContainer<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getPersistentDataContainer",
            "()Lorg/bukkit/persistence/PersistentDataContainer;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::persistence::PersistentDataContainer::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn set_type(
        &mut self,
        arg0: impl Into<&'mc crate::Material<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setType",
            "(Lorg/bukkit/Material;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn world(&mut self) -> Result<crate::World<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getWorld", "()Lorg/bukkit/World;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        crate::World::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn data(
        &mut self,
    ) -> Result<crate::material::MaterialData<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getData",
            "()Lorg/bukkit/material/MaterialData;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::material::MaterialData::from_raw(&self.jni_ref(), unsafe {
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

    pub fn block_data(
        &mut self,
    ) -> Result<crate::block::data::BlockData<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getBlockData",
            "()Lorg/bukkit/block/data/BlockData;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::block::data::BlockData::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn light_level(&mut self) -> Result<i8, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getLightLevel", "()B", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.b().unwrap())
    }

    pub fn x(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getX", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }

    pub fn y(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getY", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }

    pub fn z(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getZ", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }

    pub fn chunk(&mut self) -> Result<crate::Chunk<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getChunk", "()Lorg/bukkit/Chunk;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        crate::Chunk::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn set_data(
        &mut self,
        arg0: impl Into<&'mc crate::material::MaterialData<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setData",
            "(Lorg/bukkit/material/MaterialData;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn set_block_data(
        &mut self,
        arg0: impl Into<&'mc crate::block::data::BlockData<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setBlockData",
            "(Lorg/bukkit/block/data/BlockData;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    #[deprecated]

    pub fn raw_data(&mut self) -> Result<i8, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getRawData", "()B", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.b().unwrap())
    }
    #[deprecated]

    pub fn set_raw_data(&mut self, arg0: i8) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Byte(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setRawData",
            "(B)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn is_placed(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isPlaced", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn update(
        &mut self,
        arg0: std::option::Option<bool>,
        arg1: std::option::Option<bool>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        // -1
        let val_1 = jni::objects::JValueGen::Bool(arg0.unwrap().into());
        // -1
        let val_2 = jni::objects::JValueGen::Bool(arg1.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "update",
            "(ZZ)Z",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn location(&mut self) -> Result<crate::Location<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getLocation",
            "()Lorg/bukkit/Location;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::Location::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn get_type(&mut self) -> Result<crate::Material<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getType",
            "()Lorg/bukkit/Material;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant =
            self.jni_ref()
                .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
        let variant_str = self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::Material::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::Material::from_string(variant_str).unwrap(),
        )
    }

    pub fn set_metadata(
        &mut self,
        arg0: impl Into<&'mc String>,
        arg1: impl Into<&'mc crate::metadata::MetadataValue<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let val_2 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setMetadata",
            "(Ljava/lang/String;Lorg/bukkit/metadata/MetadataValue;)V",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn get_metadata(
        &mut self,
        arg0: impl Into<&'mc String>,
    ) -> Result<Vec<crate::metadata::MetadataValue<'mc>>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getMetadata",
            "(Ljava/lang/String;)Ljava/util/List;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let mut list = blackboxmc_java::JavaList::from_raw(&self.jni_ref(), res.l()?)?;
        let size = list.size()?;
        for i in 0..=size {
            let obj = list.get(i)?;
            new_vec.push(crate::metadata::MetadataValue::from_raw(
                &self.jni_ref(),
                obj,
            )?);
        }
        Ok(new_vec)
    }

    pub fn has_metadata(
        &mut self,
        arg0: impl Into<&'mc String>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "hasMetadata",
            "(Ljava/lang/String;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn remove_metadata(
        &mut self,
        arg0: impl Into<&'mc String>,
        arg1: impl Into<&'mc crate::plugin::Plugin<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let val_2 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeMetadata",
            "(Ljava/lang/String;Lorg/bukkit/plugin/Plugin;)V",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
}
impl<'mc> JNIRaw<'mc> for DecoratedPot<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Into<crate::block::TileState<'mc>> for DecoratedPot<'mc> {
    fn into(self) -> crate::block::TileState<'mc> {
        crate::block::TileState::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
/// Represents a captured state of a campfire.
///
/// This is a representation of an abstract class.
pub struct Campfire<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> Campfire<'mc> {
    pub fn from_raw(
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

    pub fn get_item(
        &mut self,
        arg0: i32,
    ) -> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getItem",
            "(I)Lorg/bukkit/inventory/ItemStack;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::ItemStack::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn set_item(
        &mut self,
        arg0: i32,
        arg1: impl Into<&'mc crate::inventory::ItemStack<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let val_2 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setItem",
            "(ILorg/bukkit/inventory/ItemStack;)V",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Get cook time. This is the amount of time the item has been cooking for.
    /// Get cook time total. This is the amount of time the item is required to cook for.
    pub fn get_cook_time(&mut self, arg0: i32) -> Result<i32, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getCookTime",
            "(I)I",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }
    /// Set cook time. This is the amount of time the item has been cooking for.
    /// Set cook time. This is the amount of time the item is required to cook for.
    pub fn set_cook_time(
        &mut self,
        arg0: i32,
        arg1: i32,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let val_2 = jni::objects::JValueGen::Int(arg1.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setCookTime",
            "(II)V",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Get cook time total. This is the amount of time the item is required to cook for.
    pub fn get_cook_time_total(&mut self, arg0: i32) -> Result<i32, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getCookTimeTotal",
            "(I)I",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }
    /// Set cook time. This is the amount of time the item is required to cook for.
    pub fn set_cook_time_total(
        &mut self,
        arg0: i32,
        arg1: i32,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let val_2 = jni::objects::JValueGen::Int(arg1.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setCookTimeTotal",
            "(II)V",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn size(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getSize", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }

    pub fn persistent_data_container(
        &mut self,
    ) -> Result<crate::persistence::PersistentDataContainer<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getPersistentDataContainer",
            "()Lorg/bukkit/persistence/PersistentDataContainer;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::persistence::PersistentDataContainer::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn set_type(
        &mut self,
        arg0: impl Into<&'mc crate::Material<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setType",
            "(Lorg/bukkit/Material;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn world(&mut self) -> Result<crate::World<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getWorld", "()Lorg/bukkit/World;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        crate::World::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn data(
        &mut self,
    ) -> Result<crate::material::MaterialData<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getData",
            "()Lorg/bukkit/material/MaterialData;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::material::MaterialData::from_raw(&self.jni_ref(), unsafe {
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

    pub fn block_data(
        &mut self,
    ) -> Result<crate::block::data::BlockData<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getBlockData",
            "()Lorg/bukkit/block/data/BlockData;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::block::data::BlockData::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn light_level(&mut self) -> Result<i8, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getLightLevel", "()B", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.b().unwrap())
    }

    pub fn x(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getX", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }

    pub fn y(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getY", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }

    pub fn z(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getZ", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }

    pub fn chunk(&mut self) -> Result<crate::Chunk<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getChunk", "()Lorg/bukkit/Chunk;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        crate::Chunk::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn set_data(
        &mut self,
        arg0: impl Into<&'mc crate::material::MaterialData<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setData",
            "(Lorg/bukkit/material/MaterialData;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn set_block_data(
        &mut self,
        arg0: impl Into<&'mc crate::block::data::BlockData<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setBlockData",
            "(Lorg/bukkit/block/data/BlockData;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    #[deprecated]

    pub fn raw_data(&mut self) -> Result<i8, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getRawData", "()B", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.b().unwrap())
    }
    #[deprecated]

    pub fn set_raw_data(&mut self, arg0: i8) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Byte(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setRawData",
            "(B)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn is_placed(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isPlaced", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn update(
        &mut self,
        arg0: std::option::Option<bool>,
        arg1: std::option::Option<bool>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        // -1
        let val_1 = jni::objects::JValueGen::Bool(arg0.unwrap().into());
        // -1
        let val_2 = jni::objects::JValueGen::Bool(arg1.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "update",
            "(ZZ)Z",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn location(&mut self) -> Result<crate::Location<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getLocation",
            "()Lorg/bukkit/Location;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::Location::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn get_type(&mut self) -> Result<crate::Material<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getType",
            "()Lorg/bukkit/Material;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant =
            self.jni_ref()
                .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
        let variant_str = self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::Material::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::Material::from_string(variant_str).unwrap(),
        )
    }

    pub fn set_metadata(
        &mut self,
        arg0: impl Into<&'mc String>,
        arg1: impl Into<&'mc crate::metadata::MetadataValue<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let val_2 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setMetadata",
            "(Ljava/lang/String;Lorg/bukkit/metadata/MetadataValue;)V",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn get_metadata(
        &mut self,
        arg0: impl Into<&'mc String>,
    ) -> Result<Vec<crate::metadata::MetadataValue<'mc>>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getMetadata",
            "(Ljava/lang/String;)Ljava/util/List;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let mut list = blackboxmc_java::JavaList::from_raw(&self.jni_ref(), res.l()?)?;
        let size = list.size()?;
        for i in 0..=size {
            let obj = list.get(i)?;
            new_vec.push(crate::metadata::MetadataValue::from_raw(
                &self.jni_ref(),
                obj,
            )?);
        }
        Ok(new_vec)
    }

    pub fn has_metadata(
        &mut self,
        arg0: impl Into<&'mc String>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "hasMetadata",
            "(Ljava/lang/String;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn remove_metadata(
        &mut self,
        arg0: impl Into<&'mc String>,
        arg1: impl Into<&'mc crate::plugin::Plugin<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let val_2 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeMetadata",
            "(Ljava/lang/String;Lorg/bukkit/plugin/Plugin;)V",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
}
impl<'mc> JNIRaw<'mc> for Campfire<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Into<crate::block::TileState<'mc>> for Campfire<'mc> {
    fn into(self) -> crate::block::TileState<'mc> {
        crate::block::TileState::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
pub enum BiomeEnum {
    Ocean,
    Plains,
    Desert,
    WindsweptHills,
    Forest,
    Taiga,
    Swamp,
    MangroveSwamp,
    River,
    NetherWastes,
    TheEnd,
    FrozenOcean,
    FrozenRiver,
    SnowyPlains,
    MushroomFields,
    Beach,
    Jungle,
    SparseJungle,
    DeepOcean,
    StonyShore,
    SnowyBeach,
    BirchForest,
    DarkForest,
    SnowyTaiga,
    OldGrowthPineTaiga,
    WindsweptForest,
    Savanna,
    SavannaPlateau,
    Badlands,
    WoodedBadlands,
    SmallEndIslands,
    EndMidlands,
    EndHighlands,
    EndBarrens,
    WarmOcean,
    LukewarmOcean,
    ColdOcean,
    DeepLukewarmOcean,
    DeepColdOcean,
    DeepFrozenOcean,
    TheVoid,
    SunflowerPlains,
    WindsweptGravellyHills,
    FlowerForest,
    IceSpikes,
    OldGrowthBirchForest,
    OldGrowthSpruceTaiga,
    WindsweptSavanna,
    ErodedBadlands,
    BambooJungle,
    SoulSandValley,
    CrimsonForest,
    WarpedForest,
    BasaltDeltas,
    DripstoneCaves,
    LushCaves,
    DeepDark,
    Meadow,
    Grove,
    SnowySlopes,
    FrozenPeaks,
    JaggedPeaks,
    StonyPeaks,
    CherryGrove,
    Custom,
}
impl std::fmt::Display for BiomeEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            BiomeEnum::Ocean => f.write_str("OCEAN"),
            BiomeEnum::Plains => f.write_str("PLAINS"),
            BiomeEnum::Desert => f.write_str("DESERT"),
            BiomeEnum::WindsweptHills => f.write_str("WINDSWEPT_HILLS"),
            BiomeEnum::Forest => f.write_str("FOREST"),
            BiomeEnum::Taiga => f.write_str("TAIGA"),
            BiomeEnum::Swamp => f.write_str("SWAMP"),
            BiomeEnum::MangroveSwamp => f.write_str("MANGROVE_SWAMP"),
            BiomeEnum::River => f.write_str("RIVER"),
            BiomeEnum::NetherWastes => f.write_str("NETHER_WASTES"),
            BiomeEnum::TheEnd => f.write_str("THE_END"),
            BiomeEnum::FrozenOcean => f.write_str("FROZEN_OCEAN"),
            BiomeEnum::FrozenRiver => f.write_str("FROZEN_RIVER"),
            BiomeEnum::SnowyPlains => f.write_str("SNOWY_PLAINS"),
            BiomeEnum::MushroomFields => f.write_str("MUSHROOM_FIELDS"),
            BiomeEnum::Beach => f.write_str("BEACH"),
            BiomeEnum::Jungle => f.write_str("JUNGLE"),
            BiomeEnum::SparseJungle => f.write_str("SPARSE_JUNGLE"),
            BiomeEnum::DeepOcean => f.write_str("DEEP_OCEAN"),
            BiomeEnum::StonyShore => f.write_str("STONY_SHORE"),
            BiomeEnum::SnowyBeach => f.write_str("SNOWY_BEACH"),
            BiomeEnum::BirchForest => f.write_str("BIRCH_FOREST"),
            BiomeEnum::DarkForest => f.write_str("DARK_FOREST"),
            BiomeEnum::SnowyTaiga => f.write_str("SNOWY_TAIGA"),
            BiomeEnum::OldGrowthPineTaiga => f.write_str("OLD_GROWTH_PINE_TAIGA"),
            BiomeEnum::WindsweptForest => f.write_str("WINDSWEPT_FOREST"),
            BiomeEnum::Savanna => f.write_str("SAVANNA"),
            BiomeEnum::SavannaPlateau => f.write_str("SAVANNA_PLATEAU"),
            BiomeEnum::Badlands => f.write_str("BADLANDS"),
            BiomeEnum::WoodedBadlands => f.write_str("WOODED_BADLANDS"),
            BiomeEnum::SmallEndIslands => f.write_str("SMALL_END_ISLANDS"),
            BiomeEnum::EndMidlands => f.write_str("END_MIDLANDS"),
            BiomeEnum::EndHighlands => f.write_str("END_HIGHLANDS"),
            BiomeEnum::EndBarrens => f.write_str("END_BARRENS"),
            BiomeEnum::WarmOcean => f.write_str("WARM_OCEAN"),
            BiomeEnum::LukewarmOcean => f.write_str("LUKEWARM_OCEAN"),
            BiomeEnum::ColdOcean => f.write_str("COLD_OCEAN"),
            BiomeEnum::DeepLukewarmOcean => f.write_str("DEEP_LUKEWARM_OCEAN"),
            BiomeEnum::DeepColdOcean => f.write_str("DEEP_COLD_OCEAN"),
            BiomeEnum::DeepFrozenOcean => f.write_str("DEEP_FROZEN_OCEAN"),
            BiomeEnum::TheVoid => f.write_str("THE_VOID"),
            BiomeEnum::SunflowerPlains => f.write_str("SUNFLOWER_PLAINS"),
            BiomeEnum::WindsweptGravellyHills => f.write_str("WINDSWEPT_GRAVELLY_HILLS"),
            BiomeEnum::FlowerForest => f.write_str("FLOWER_FOREST"),
            BiomeEnum::IceSpikes => f.write_str("ICE_SPIKES"),
            BiomeEnum::OldGrowthBirchForest => f.write_str("OLD_GROWTH_BIRCH_FOREST"),
            BiomeEnum::OldGrowthSpruceTaiga => f.write_str("OLD_GROWTH_SPRUCE_TAIGA"),
            BiomeEnum::WindsweptSavanna => f.write_str("WINDSWEPT_SAVANNA"),
            BiomeEnum::ErodedBadlands => f.write_str("ERODED_BADLANDS"),
            BiomeEnum::BambooJungle => f.write_str("BAMBOO_JUNGLE"),
            BiomeEnum::SoulSandValley => f.write_str("SOUL_SAND_VALLEY"),
            BiomeEnum::CrimsonForest => f.write_str("CRIMSON_FOREST"),
            BiomeEnum::WarpedForest => f.write_str("WARPED_FOREST"),
            BiomeEnum::BasaltDeltas => f.write_str("BASALT_DELTAS"),
            BiomeEnum::DripstoneCaves => f.write_str("DRIPSTONE_CAVES"),
            BiomeEnum::LushCaves => f.write_str("LUSH_CAVES"),
            BiomeEnum::DeepDark => f.write_str("DEEP_DARK"),
            BiomeEnum::Meadow => f.write_str("MEADOW"),
            BiomeEnum::Grove => f.write_str("GROVE"),
            BiomeEnum::SnowySlopes => f.write_str("SNOWY_SLOPES"),
            BiomeEnum::FrozenPeaks => f.write_str("FROZEN_PEAKS"),
            BiomeEnum::JaggedPeaks => f.write_str("JAGGED_PEAKS"),
            BiomeEnum::StonyPeaks => f.write_str("STONY_PEAKS"),
            BiomeEnum::CherryGrove => f.write_str("CHERRY_GROVE"),
            BiomeEnum::Custom => f.write_str("CUSTOM"),
        }
    }
}
pub struct Biome<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
    pub BiomeEnum,
);
impl<'mc> std::ops::Deref for Biome<'mc> {
    type Target = BiomeEnum;
    fn deref(&self) -> &Self::Target {
        return &self.2;
    }
}
impl<'mc> JNIRaw<'mc> for Biome<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Biome<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
        e: BiomeEnum,
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
            Ok(Self(env.clone(), obj, e))
        }
    }
    pub const OCEAN: BiomeEnum = BiomeEnum::Ocean;
    pub const PLAINS: BiomeEnum = BiomeEnum::Plains;
    pub const DESERT: BiomeEnum = BiomeEnum::Desert;
    pub const WINDSWEPT_HILLS: BiomeEnum = BiomeEnum::WindsweptHills;
    pub const FOREST: BiomeEnum = BiomeEnum::Forest;
    pub const TAIGA: BiomeEnum = BiomeEnum::Taiga;
    pub const SWAMP: BiomeEnum = BiomeEnum::Swamp;
    pub const MANGROVE_SWAMP: BiomeEnum = BiomeEnum::MangroveSwamp;
    pub const RIVER: BiomeEnum = BiomeEnum::River;
    pub const NETHER_WASTES: BiomeEnum = BiomeEnum::NetherWastes;
    pub const THE_END: BiomeEnum = BiomeEnum::TheEnd;
    pub const FROZEN_OCEAN: BiomeEnum = BiomeEnum::FrozenOcean;
    pub const FROZEN_RIVER: BiomeEnum = BiomeEnum::FrozenRiver;
    pub const SNOWY_PLAINS: BiomeEnum = BiomeEnum::SnowyPlains;
    pub const MUSHROOM_FIELDS: BiomeEnum = BiomeEnum::MushroomFields;
    pub const BEACH: BiomeEnum = BiomeEnum::Beach;
    pub const JUNGLE: BiomeEnum = BiomeEnum::Jungle;
    pub const SPARSE_JUNGLE: BiomeEnum = BiomeEnum::SparseJungle;
    pub const DEEP_OCEAN: BiomeEnum = BiomeEnum::DeepOcean;
    pub const STONY_SHORE: BiomeEnum = BiomeEnum::StonyShore;
    pub const SNOWY_BEACH: BiomeEnum = BiomeEnum::SnowyBeach;
    pub const BIRCH_FOREST: BiomeEnum = BiomeEnum::BirchForest;
    pub const DARK_FOREST: BiomeEnum = BiomeEnum::DarkForest;
    pub const SNOWY_TAIGA: BiomeEnum = BiomeEnum::SnowyTaiga;
    pub const OLD_GROWTH_PINE_TAIGA: BiomeEnum = BiomeEnum::OldGrowthPineTaiga;
    pub const WINDSWEPT_FOREST: BiomeEnum = BiomeEnum::WindsweptForest;
    pub const SAVANNA: BiomeEnum = BiomeEnum::Savanna;
    pub const SAVANNA_PLATEAU: BiomeEnum = BiomeEnum::SavannaPlateau;
    pub const BADLANDS: BiomeEnum = BiomeEnum::Badlands;
    pub const WOODED_BADLANDS: BiomeEnum = BiomeEnum::WoodedBadlands;
    pub const SMALL_END_ISLANDS: BiomeEnum = BiomeEnum::SmallEndIslands;
    pub const END_MIDLANDS: BiomeEnum = BiomeEnum::EndMidlands;
    pub const END_HIGHLANDS: BiomeEnum = BiomeEnum::EndHighlands;
    pub const END_BARRENS: BiomeEnum = BiomeEnum::EndBarrens;
    pub const WARM_OCEAN: BiomeEnum = BiomeEnum::WarmOcean;
    pub const LUKEWARM_OCEAN: BiomeEnum = BiomeEnum::LukewarmOcean;
    pub const COLD_OCEAN: BiomeEnum = BiomeEnum::ColdOcean;
    pub const DEEP_LUKEWARM_OCEAN: BiomeEnum = BiomeEnum::DeepLukewarmOcean;
    pub const DEEP_COLD_OCEAN: BiomeEnum = BiomeEnum::DeepColdOcean;
    pub const DEEP_FROZEN_OCEAN: BiomeEnum = BiomeEnum::DeepFrozenOcean;
    pub const THE_VOID: BiomeEnum = BiomeEnum::TheVoid;
    pub const SUNFLOWER_PLAINS: BiomeEnum = BiomeEnum::SunflowerPlains;
    pub const WINDSWEPT_GRAVELLY_HILLS: BiomeEnum = BiomeEnum::WindsweptGravellyHills;
    pub const FLOWER_FOREST: BiomeEnum = BiomeEnum::FlowerForest;
    pub const ICE_SPIKES: BiomeEnum = BiomeEnum::IceSpikes;
    pub const OLD_GROWTH_BIRCH_FOREST: BiomeEnum = BiomeEnum::OldGrowthBirchForest;
    pub const OLD_GROWTH_SPRUCE_TAIGA: BiomeEnum = BiomeEnum::OldGrowthSpruceTaiga;
    pub const WINDSWEPT_SAVANNA: BiomeEnum = BiomeEnum::WindsweptSavanna;
    pub const ERODED_BADLANDS: BiomeEnum = BiomeEnum::ErodedBadlands;
    pub const BAMBOO_JUNGLE: BiomeEnum = BiomeEnum::BambooJungle;
    pub const SOUL_SAND_VALLEY: BiomeEnum = BiomeEnum::SoulSandValley;
    pub const CRIMSON_FOREST: BiomeEnum = BiomeEnum::CrimsonForest;
    pub const WARPED_FOREST: BiomeEnum = BiomeEnum::WarpedForest;
    pub const BASALT_DELTAS: BiomeEnum = BiomeEnum::BasaltDeltas;
    pub const DRIPSTONE_CAVES: BiomeEnum = BiomeEnum::DripstoneCaves;
    pub const LUSH_CAVES: BiomeEnum = BiomeEnum::LushCaves;
    pub const DEEP_DARK: BiomeEnum = BiomeEnum::DeepDark;
    pub const MEADOW: BiomeEnum = BiomeEnum::Meadow;
    pub const GROVE: BiomeEnum = BiomeEnum::Grove;
    pub const SNOWY_SLOPES: BiomeEnum = BiomeEnum::SnowySlopes;
    pub const FROZEN_PEAKS: BiomeEnum = BiomeEnum::FrozenPeaks;
    pub const JAGGED_PEAKS: BiomeEnum = BiomeEnum::JaggedPeaks;
    pub const STONY_PEAKS: BiomeEnum = BiomeEnum::StonyPeaks;
    pub const CHERRY_GROVE: BiomeEnum = BiomeEnum::CherryGrove;
    pub const CUSTOM: BiomeEnum = BiomeEnum::Custom;
    pub fn from_string(str: String) -> std::option::Option<BiomeEnum> {
        match str.as_str() {
            "OCEAN" => Some(BiomeEnum::Ocean),
            "PLAINS" => Some(BiomeEnum::Plains),
            "DESERT" => Some(BiomeEnum::Desert),
            "WINDSWEPT_HILLS" => Some(BiomeEnum::WindsweptHills),
            "FOREST" => Some(BiomeEnum::Forest),
            "TAIGA" => Some(BiomeEnum::Taiga),
            "SWAMP" => Some(BiomeEnum::Swamp),
            "MANGROVE_SWAMP" => Some(BiomeEnum::MangroveSwamp),
            "RIVER" => Some(BiomeEnum::River),
            "NETHER_WASTES" => Some(BiomeEnum::NetherWastes),
            "THE_END" => Some(BiomeEnum::TheEnd),
            "FROZEN_OCEAN" => Some(BiomeEnum::FrozenOcean),
            "FROZEN_RIVER" => Some(BiomeEnum::FrozenRiver),
            "SNOWY_PLAINS" => Some(BiomeEnum::SnowyPlains),
            "MUSHROOM_FIELDS" => Some(BiomeEnum::MushroomFields),
            "BEACH" => Some(BiomeEnum::Beach),
            "JUNGLE" => Some(BiomeEnum::Jungle),
            "SPARSE_JUNGLE" => Some(BiomeEnum::SparseJungle),
            "DEEP_OCEAN" => Some(BiomeEnum::DeepOcean),
            "STONY_SHORE" => Some(BiomeEnum::StonyShore),
            "SNOWY_BEACH" => Some(BiomeEnum::SnowyBeach),
            "BIRCH_FOREST" => Some(BiomeEnum::BirchForest),
            "DARK_FOREST" => Some(BiomeEnum::DarkForest),
            "SNOWY_TAIGA" => Some(BiomeEnum::SnowyTaiga),
            "OLD_GROWTH_PINE_TAIGA" => Some(BiomeEnum::OldGrowthPineTaiga),
            "WINDSWEPT_FOREST" => Some(BiomeEnum::WindsweptForest),
            "SAVANNA" => Some(BiomeEnum::Savanna),
            "SAVANNA_PLATEAU" => Some(BiomeEnum::SavannaPlateau),
            "BADLANDS" => Some(BiomeEnum::Badlands),
            "WOODED_BADLANDS" => Some(BiomeEnum::WoodedBadlands),
            "SMALL_END_ISLANDS" => Some(BiomeEnum::SmallEndIslands),
            "END_MIDLANDS" => Some(BiomeEnum::EndMidlands),
            "END_HIGHLANDS" => Some(BiomeEnum::EndHighlands),
            "END_BARRENS" => Some(BiomeEnum::EndBarrens),
            "WARM_OCEAN" => Some(BiomeEnum::WarmOcean),
            "LUKEWARM_OCEAN" => Some(BiomeEnum::LukewarmOcean),
            "COLD_OCEAN" => Some(BiomeEnum::ColdOcean),
            "DEEP_LUKEWARM_OCEAN" => Some(BiomeEnum::DeepLukewarmOcean),
            "DEEP_COLD_OCEAN" => Some(BiomeEnum::DeepColdOcean),
            "DEEP_FROZEN_OCEAN" => Some(BiomeEnum::DeepFrozenOcean),
            "THE_VOID" => Some(BiomeEnum::TheVoid),
            "SUNFLOWER_PLAINS" => Some(BiomeEnum::SunflowerPlains),
            "WINDSWEPT_GRAVELLY_HILLS" => Some(BiomeEnum::WindsweptGravellyHills),
            "FLOWER_FOREST" => Some(BiomeEnum::FlowerForest),
            "ICE_SPIKES" => Some(BiomeEnum::IceSpikes),
            "OLD_GROWTH_BIRCH_FOREST" => Some(BiomeEnum::OldGrowthBirchForest),
            "OLD_GROWTH_SPRUCE_TAIGA" => Some(BiomeEnum::OldGrowthSpruceTaiga),
            "WINDSWEPT_SAVANNA" => Some(BiomeEnum::WindsweptSavanna),
            "ERODED_BADLANDS" => Some(BiomeEnum::ErodedBadlands),
            "BAMBOO_JUNGLE" => Some(BiomeEnum::BambooJungle),
            "SOUL_SAND_VALLEY" => Some(BiomeEnum::SoulSandValley),
            "CRIMSON_FOREST" => Some(BiomeEnum::CrimsonForest),
            "WARPED_FOREST" => Some(BiomeEnum::WarpedForest),
            "BASALT_DELTAS" => Some(BiomeEnum::BasaltDeltas),
            "DRIPSTONE_CAVES" => Some(BiomeEnum::DripstoneCaves),
            "LUSH_CAVES" => Some(BiomeEnum::LushCaves),
            "DEEP_DARK" => Some(BiomeEnum::DeepDark),
            "MEADOW" => Some(BiomeEnum::Meadow),
            "GROVE" => Some(BiomeEnum::Grove),
            "SNOWY_SLOPES" => Some(BiomeEnum::SnowySlopes),
            "FROZEN_PEAKS" => Some(BiomeEnum::FrozenPeaks),
            "JAGGED_PEAKS" => Some(BiomeEnum::JaggedPeaks),
            "STONY_PEAKS" => Some(BiomeEnum::StonyPeaks),
            "CHERRY_GROVE" => Some(BiomeEnum::CherryGrove),
            "CUSTOM" => Some(BiomeEnum::Custom),
            _ => None,
        }
    }
}
/// Represents a captured state of a smoker.
///
/// This is a representation of an abstract class.
pub struct Smoker<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> Smoker<'mc> {
    pub fn from_raw(
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

    pub fn snapshot_inventory(
        &mut self,
    ) -> Result<crate::inventory::FurnaceInventory<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getSnapshotInventory",
            "()Lorg/bukkit/inventory/FurnaceInventory;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::FurnaceInventory::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn cook_time(&mut self) -> Result<i16, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getCookTime", "()S", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.s().unwrap())
    }

    pub fn set_cook_time(&mut self, arg0: i16) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Short(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setCookTime",
            "(S)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn cook_time_total(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getCookTimeTotal", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }

    pub fn set_cook_time_total(&mut self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setCookTimeTotal",
            "(I)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn burn_time(&mut self) -> Result<i16, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getBurnTime", "()S", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.s().unwrap())
    }

    pub fn set_burn_time(&mut self, arg0: i16) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Short(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setBurnTime",
            "(S)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn recipes_used(
        &mut self,
    ) -> Result<blackboxmc_java::JavaMap<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getRecipesUsed",
            "()Ljava/util/Map;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::JavaMap::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn persistent_data_container(
        &mut self,
    ) -> Result<crate::persistence::PersistentDataContainer<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getPersistentDataContainer",
            "()Lorg/bukkit/persistence/PersistentDataContainer;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::persistence::PersistentDataContainer::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn set_type(
        &mut self,
        arg0: impl Into<&'mc crate::Material<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setType",
            "(Lorg/bukkit/Material;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn world(&mut self) -> Result<crate::World<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getWorld", "()Lorg/bukkit/World;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        crate::World::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn data(
        &mut self,
    ) -> Result<crate::material::MaterialData<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getData",
            "()Lorg/bukkit/material/MaterialData;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::material::MaterialData::from_raw(&self.jni_ref(), unsafe {
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

    pub fn block_data(
        &mut self,
    ) -> Result<crate::block::data::BlockData<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getBlockData",
            "()Lorg/bukkit/block/data/BlockData;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::block::data::BlockData::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn light_level(&mut self) -> Result<i8, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getLightLevel", "()B", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.b().unwrap())
    }

    pub fn x(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getX", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }

    pub fn y(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getY", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }

    pub fn z(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getZ", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }

    pub fn chunk(&mut self) -> Result<crate::Chunk<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getChunk", "()Lorg/bukkit/Chunk;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        crate::Chunk::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn set_data(
        &mut self,
        arg0: impl Into<&'mc crate::material::MaterialData<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setData",
            "(Lorg/bukkit/material/MaterialData;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn set_block_data(
        &mut self,
        arg0: impl Into<&'mc crate::block::data::BlockData<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setBlockData",
            "(Lorg/bukkit/block/data/BlockData;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    #[deprecated]

    pub fn raw_data(&mut self) -> Result<i8, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getRawData", "()B", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.b().unwrap())
    }
    #[deprecated]

    pub fn set_raw_data(&mut self, arg0: i8) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Byte(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setRawData",
            "(B)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn is_placed(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isPlaced", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn update(
        &mut self,
        arg0: std::option::Option<bool>,
        arg1: std::option::Option<bool>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        // -1
        let val_1 = jni::objects::JValueGen::Bool(arg0.unwrap().into());
        // -1
        let val_2 = jni::objects::JValueGen::Bool(arg1.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "update",
            "(ZZ)Z",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn location(&mut self) -> Result<crate::Location<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getLocation",
            "()Lorg/bukkit/Location;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::Location::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn get_type(&mut self) -> Result<crate::Material<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getType",
            "()Lorg/bukkit/Material;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant =
            self.jni_ref()
                .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
        let variant_str = self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::Material::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::Material::from_string(variant_str).unwrap(),
        )
    }

    pub fn set_metadata(
        &mut self,
        arg0: impl Into<&'mc String>,
        arg1: impl Into<&'mc crate::metadata::MetadataValue<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let val_2 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setMetadata",
            "(Ljava/lang/String;Lorg/bukkit/metadata/MetadataValue;)V",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn get_metadata(
        &mut self,
        arg0: impl Into<&'mc String>,
    ) -> Result<Vec<crate::metadata::MetadataValue<'mc>>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getMetadata",
            "(Ljava/lang/String;)Ljava/util/List;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let mut list = blackboxmc_java::JavaList::from_raw(&self.jni_ref(), res.l()?)?;
        let size = list.size()?;
        for i in 0..=size {
            let obj = list.get(i)?;
            new_vec.push(crate::metadata::MetadataValue::from_raw(
                &self.jni_ref(),
                obj,
            )?);
        }
        Ok(new_vec)
    }

    pub fn has_metadata(
        &mut self,
        arg0: impl Into<&'mc String>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "hasMetadata",
            "(Ljava/lang/String;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn remove_metadata(
        &mut self,
        arg0: impl Into<&'mc String>,
        arg1: impl Into<&'mc crate::plugin::Plugin<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let val_2 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeMetadata",
            "(Ljava/lang/String;Lorg/bukkit/plugin/Plugin;)V",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn is_locked(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isLocked", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn lock(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getLock", "()Ljava/lang/String;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }

    pub fn set_lock(
        &mut self,
        arg0: impl Into<&'mc String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setLock",
            "(Ljava/lang/String;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn custom_name(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getCustomName",
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

    pub fn set_custom_name(
        &mut self,
        arg0: impl Into<&'mc String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setCustomName",
            "(Ljava/lang/String;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
}
impl<'mc> JNIRaw<'mc> for Smoker<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Into<crate::block::Furnace<'mc>> for Smoker<'mc> {
    fn into(self) -> crate::block::Furnace<'mc> {
        crate::block::Furnace::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
/// Represents a captured state of a calibrated sculk sensor
///
/// This is a representation of an abstract class.
pub struct CalibratedSculkSensor<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> CalibratedSculkSensor<'mc> {
    pub fn from_raw(
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

    pub fn last_vibration_frequency(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getLastVibrationFrequency", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }

    pub fn set_last_vibration_frequency(
        &mut self,
        arg0: i32,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setLastVibrationFrequency",
            "(I)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn persistent_data_container(
        &mut self,
    ) -> Result<crate::persistence::PersistentDataContainer<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getPersistentDataContainer",
            "()Lorg/bukkit/persistence/PersistentDataContainer;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::persistence::PersistentDataContainer::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn set_type(
        &mut self,
        arg0: impl Into<&'mc crate::Material<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setType",
            "(Lorg/bukkit/Material;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn world(&mut self) -> Result<crate::World<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getWorld", "()Lorg/bukkit/World;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        crate::World::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn data(
        &mut self,
    ) -> Result<crate::material::MaterialData<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getData",
            "()Lorg/bukkit/material/MaterialData;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::material::MaterialData::from_raw(&self.jni_ref(), unsafe {
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

    pub fn block_data(
        &mut self,
    ) -> Result<crate::block::data::BlockData<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getBlockData",
            "()Lorg/bukkit/block/data/BlockData;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::block::data::BlockData::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn light_level(&mut self) -> Result<i8, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getLightLevel", "()B", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.b().unwrap())
    }

    pub fn x(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getX", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }

    pub fn y(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getY", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }

    pub fn z(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getZ", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }

    pub fn chunk(&mut self) -> Result<crate::Chunk<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getChunk", "()Lorg/bukkit/Chunk;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        crate::Chunk::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn set_data(
        &mut self,
        arg0: impl Into<&'mc crate::material::MaterialData<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setData",
            "(Lorg/bukkit/material/MaterialData;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn set_block_data(
        &mut self,
        arg0: impl Into<&'mc crate::block::data::BlockData<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setBlockData",
            "(Lorg/bukkit/block/data/BlockData;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    #[deprecated]

    pub fn raw_data(&mut self) -> Result<i8, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getRawData", "()B", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.b().unwrap())
    }
    #[deprecated]

    pub fn set_raw_data(&mut self, arg0: i8) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Byte(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setRawData",
            "(B)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn is_placed(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isPlaced", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn update(
        &mut self,
        arg0: std::option::Option<bool>,
        arg1: std::option::Option<bool>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        // -1
        let val_1 = jni::objects::JValueGen::Bool(arg0.unwrap().into());
        // -1
        let val_2 = jni::objects::JValueGen::Bool(arg1.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "update",
            "(ZZ)Z",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn location(&mut self) -> Result<crate::Location<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getLocation",
            "()Lorg/bukkit/Location;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::Location::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn get_type(&mut self) -> Result<crate::Material<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getType",
            "()Lorg/bukkit/Material;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant =
            self.jni_ref()
                .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
        let variant_str = self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::Material::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::Material::from_string(variant_str).unwrap(),
        )
    }

    pub fn set_metadata(
        &mut self,
        arg0: impl Into<&'mc String>,
        arg1: impl Into<&'mc crate::metadata::MetadataValue<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let val_2 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setMetadata",
            "(Ljava/lang/String;Lorg/bukkit/metadata/MetadataValue;)V",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn get_metadata(
        &mut self,
        arg0: impl Into<&'mc String>,
    ) -> Result<Vec<crate::metadata::MetadataValue<'mc>>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getMetadata",
            "(Ljava/lang/String;)Ljava/util/List;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let mut list = blackboxmc_java::JavaList::from_raw(&self.jni_ref(), res.l()?)?;
        let size = list.size()?;
        for i in 0..=size {
            let obj = list.get(i)?;
            new_vec.push(crate::metadata::MetadataValue::from_raw(
                &self.jni_ref(),
                obj,
            )?);
        }
        Ok(new_vec)
    }

    pub fn has_metadata(
        &mut self,
        arg0: impl Into<&'mc String>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "hasMetadata",
            "(Ljava/lang/String;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn remove_metadata(
        &mut self,
        arg0: impl Into<&'mc String>,
        arg1: impl Into<&'mc crate::plugin::Plugin<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let val_2 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeMetadata",
            "(Ljava/lang/String;Lorg/bukkit/plugin/Plugin;)V",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
}
impl<'mc> JNIRaw<'mc> for CalibratedSculkSensor<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Into<crate::block::SculkSensor<'mc>> for CalibratedSculkSensor<'mc> {
    fn into(self) -> crate::block::SculkSensor<'mc> {
        crate::block::SculkSensor::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
/// Represents a structure block that can save and load blocks from a file. They can only be used by OPs, and are not obtainable in survival.
///
/// This is a representation of an abstract class.
pub struct Structure<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> Structure<'mc> {
    pub fn from_raw(
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
    /// The seed used to determine how many blocks are removed upon loading of this structure.
    pub fn seed(&mut self) -> Result<i64, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getSeed", "()J", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.j().unwrap())
    }
    /// The seed used to determine which blocks will be removed upon loading. <a href="#getIntegrity()"><code>getIntegrity()</code></a> and seed are used together to determine which blocks are randomly removed to mimic "decay."
    pub fn set_seed(&mut self, arg0: i64) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Long(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setSeed",
            "(J)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Only applicable while in <a href="structure/UsageMode.html#DATA"><code>UsageMode.DATA</code></a>. Metadata are specific functions that can be applied to the structure location. Consult the <a href="https://minecraft.gamepedia.com/Structure_Block#Data">Minecraft wiki</a> for more information.
    pub fn set_metadata_with_string(
        &mut self,
        arg0: std::option::Option<impl Into<&'mc String>>,
        arg1: std::option::Option<impl Into<&'mc crate::metadata::MetadataValue<'mc>>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 =
            jni::objects::JObject::from(self.jni_ref().new_string(arg0.unwrap().into()).unwrap());
        let val_2 =
            unsafe { jni::objects::JObject::from_raw(arg1.unwrap().into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setMetadata",
            "(Ljava/lang/String;Lorg/bukkit/metadata/MetadataValue;)V",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Get the metadata function this structure block will perform when activated. Consult the <a href="https://minecraft.gamepedia.com/Structure_Block#Data">Minecraft Wiki</a> for more information.
    pub fn get_metadata(
        &mut self,
        arg0: impl Into<&'mc String>,
    ) -> Result<Vec<crate::metadata::MetadataValue<'mc>>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getMetadata",
            "(Ljava/lang/String;)Ljava/util/List;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let mut list = blackboxmc_java::JavaList::from_raw(&self.jni_ref(), res.l()?)?;
        let size = list.size()?;
        for i in 0..=size {
            let obj = list.get(i)?;
            new_vec.push(crate::metadata::MetadataValue::from_raw(
                &self.jni_ref(),
                obj,
            )?);
        }
        Ok(new_vec)
    }
    /// Get how this structure is rotated.
    pub fn rotation(
        &mut self,
    ) -> Result<crate::block::structure::StructureRotation<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getRotation",
            "()Lorg/bukkit/block/structure/StructureRotation;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant =
            self.jni_ref()
                .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
        let variant_str = self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::block::structure::StructureRotation::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::block::structure::StructureRotation::from_string(variant_str).unwrap(),
        )
    }
    /// Set how this structure is rotated.
    pub fn set_rotation(
        &mut self,
        arg0: impl Into<&'mc crate::block::structure::StructureRotation<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setRotation",
            "(Lorg/bukkit/block/structure/StructureRotation;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// The relative position of the structure outline based on the position of the structure block. Maximum allowed distance is 48 blocks in any direction.
    pub fn relative_position(
        &mut self,
    ) -> Result<crate::util::BlockVector<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getRelativePosition",
            "()Lorg/bukkit/util/BlockVector;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::util::BlockVector::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// The name of this structure.
    pub fn structure_name(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getStructureName",
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
    /// Set the name of this structure. This is case-sensitive. The name of the structure in the <a href="structure/UsageMode.html#SAVE"><code>UsageMode.SAVE</code></a> structure block MUST match the name within the <a href="structure/UsageMode.html#CORNER"><code>UsageMode.CORNER</code></a> block or the size calculation will fail.
    pub fn set_structure_name(
        &mut self,
        arg0: impl Into<&'mc String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setStructureName",
            "(Ljava/lang/String;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Get the name of who created this structure.
    pub fn author(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getAuthor",
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
    /// Set the name of whoever created this structure.
    /// Set the name of whoever created this structure using a <a href="../entity/LivingEntity.html" title="interface in org.bukkit.entity"><code>LivingEntity</code></a>.
    pub fn set_author_with_living_entity(
        &mut self,
        arg0: std::option::Option<impl Into<&'mc String>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 =
            jni::objects::JObject::from(self.jni_ref().new_string(arg0.unwrap().into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setAuthor",
            "(Ljava/lang/String;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Set the relative position from the structure block. Maximum allowed distance is 48 blocks in any direction.
    pub fn set_relative_position(
        &mut self,
        arg0: impl Into<&'mc crate::util::BlockVector<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setRelativePosition",
            "(Lorg/bukkit/util/BlockVector;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// The distance to the opposite corner of this structure. The maximum structure size is 48x48x48. When a structure has successfully been calculated (i.e. it is within the maximum allowed distance) a white border surrounds the structure.
    pub fn structure_size(
        &mut self,
    ) -> Result<crate::util::BlockVector<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getStructureSize",
            "()Lorg/bukkit/util/BlockVector;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::util::BlockVector::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Set the maximum size of this structure from the origin point. Maximum allowed size is 48x48x48.
    pub fn set_structure_size(
        &mut self,
        arg0: impl Into<&'mc crate::util::BlockVector<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setStructureSize",
            "(Lorg/bukkit/util/BlockVector;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Sets the mirroring of the structure.
    pub fn set_mirror(
        &mut self,
        arg0: impl Into<&'mc crate::block::structure::Mirror<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setMirror",
            "(Lorg/bukkit/block/structure/Mirror;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// How this structure is mirrored.
    pub fn mirror(
        &mut self,
    ) -> Result<crate::block::structure::Mirror<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getMirror",
            "()Lorg/bukkit/block/structure/Mirror;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant =
            self.jni_ref()
                .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
        let variant_str = self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::block::structure::Mirror::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::block::structure::Mirror::from_string(variant_str).unwrap(),
        )
    }
    /// Set the <a href="structure/UsageMode.html" title="enum in org.bukkit.block.structure"><code>UsageMode</code></a> of this structure block.
    pub fn set_usage_mode(
        &mut self,
        arg0: impl Into<&'mc crate::block::structure::UsageMode<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setUsageMode",
            "(Lorg/bukkit/block/structure/UsageMode;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Get the <a title="enum in org.bukkit.block.structure" href="structure/UsageMode.html"><code>UsageMode</code></a> of this structure block.
    pub fn usage_mode(
        &mut self,
    ) -> Result<crate::block::structure::UsageMode<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getUsageMode",
            "()Lorg/bukkit/block/structure/UsageMode;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant =
            self.jni_ref()
                .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
        let variant_str = self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::block::structure::UsageMode::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::block::structure::UsageMode::from_string(variant_str).unwrap(),
        )
    }
    /// While in <a href="structure/UsageMode.html#SAVE"><code>UsageMode.SAVE</code></a> mode, this will ignore any entities when saving the structure.
    ///
    /// While in <a href="structure/UsageMode.html#LOAD"><code>UsageMode.LOAD</code></a> mode this will ignore any entities that were saved to file.
    pub fn set_ignore_entities(&mut self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        // -2
        let val_1 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setIgnoreEntities",
            "(Z)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Get if this structure block should ignore entities.
    pub fn is_ignore_entities(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isIgnoreEntities", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    /// Set if the structure outline should show air blocks.
    pub fn set_show_air(&mut self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        // -2
        let val_1 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setShowAir",
            "(Z)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Check if this structure block is currently showing all air blocks
    pub fn is_show_air(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isShowAir", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    /// Set if this structure box should show the bounding box.
    pub fn set_bounding_box_visible(
        &mut self,
        arg0: bool,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // -2
        let val_1 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setBoundingBoxVisible",
            "(Z)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Get if this structure block is currently showing the bounding box.
    pub fn is_bounding_box_visible(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isBoundingBoxVisible", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    /// Set the integrity of the structure. Integrity must be between 0.0 and 1.0 Lower integrity values will result in more blocks being removed when loading a structure. Integrity and <a href="#getSeed()"><code>getSeed()</code></a> are used together to determine which blocks are randomly removed to mimic "decay."
    pub fn set_integrity(&mut self, arg0: f32) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Float(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setIntegrity",
            "(F)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Get the integrity of this structure.
    pub fn integrity(&mut self) -> Result<f32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getIntegrity", "()F", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.f().unwrap())
    }

    pub fn persistent_data_container(
        &mut self,
    ) -> Result<crate::persistence::PersistentDataContainer<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getPersistentDataContainer",
            "()Lorg/bukkit/persistence/PersistentDataContainer;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::persistence::PersistentDataContainer::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn set_type(
        &mut self,
        arg0: impl Into<&'mc crate::Material<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setType",
            "(Lorg/bukkit/Material;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn world(&mut self) -> Result<crate::World<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getWorld", "()Lorg/bukkit/World;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        crate::World::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn data(
        &mut self,
    ) -> Result<crate::material::MaterialData<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getData",
            "()Lorg/bukkit/material/MaterialData;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::material::MaterialData::from_raw(&self.jni_ref(), unsafe {
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

    pub fn block_data(
        &mut self,
    ) -> Result<crate::block::data::BlockData<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getBlockData",
            "()Lorg/bukkit/block/data/BlockData;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::block::data::BlockData::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn light_level(&mut self) -> Result<i8, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getLightLevel", "()B", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.b().unwrap())
    }

    pub fn x(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getX", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }

    pub fn y(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getY", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }

    pub fn z(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getZ", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }

    pub fn chunk(&mut self) -> Result<crate::Chunk<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getChunk", "()Lorg/bukkit/Chunk;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        crate::Chunk::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn set_data(
        &mut self,
        arg0: impl Into<&'mc crate::material::MaterialData<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setData",
            "(Lorg/bukkit/material/MaterialData;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn set_block_data(
        &mut self,
        arg0: impl Into<&'mc crate::block::data::BlockData<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setBlockData",
            "(Lorg/bukkit/block/data/BlockData;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    #[deprecated]

    pub fn raw_data(&mut self) -> Result<i8, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getRawData", "()B", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.b().unwrap())
    }
    #[deprecated]

    pub fn set_raw_data(&mut self, arg0: i8) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Byte(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setRawData",
            "(B)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn is_placed(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isPlaced", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn update(
        &mut self,
        arg0: std::option::Option<bool>,
        arg1: std::option::Option<bool>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        // -1
        let val_1 = jni::objects::JValueGen::Bool(arg0.unwrap().into());
        // -1
        let val_2 = jni::objects::JValueGen::Bool(arg1.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "update",
            "(ZZ)Z",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn location(&mut self) -> Result<crate::Location<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getLocation",
            "()Lorg/bukkit/Location;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::Location::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn get_type(&mut self) -> Result<crate::Material<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getType",
            "()Lorg/bukkit/Material;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant =
            self.jni_ref()
                .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
        let variant_str = self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::Material::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::Material::from_string(variant_str).unwrap(),
        )
    }

    pub fn has_metadata(
        &mut self,
        arg0: impl Into<&'mc String>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "hasMetadata",
            "(Ljava/lang/String;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn remove_metadata(
        &mut self,
        arg0: impl Into<&'mc String>,
        arg1: impl Into<&'mc crate::plugin::Plugin<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let val_2 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeMetadata",
            "(Ljava/lang/String;Lorg/bukkit/plugin/Plugin;)V",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
}
impl<'mc> JNIRaw<'mc> for Structure<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Into<crate::block::TileState<'mc>> for Structure<'mc> {
    fn into(self) -> crate::block::TileState<'mc> {
        crate::block::TileState::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
/// Represents a captured state of an ender chest.
///
/// This is a representation of an abstract class.
pub struct EnderChest<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> EnderChest<'mc> {
    pub fn from_raw(
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

    pub fn close(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "close", "()V", &[]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn open(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "open", "()V", &[]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn persistent_data_container(
        &mut self,
    ) -> Result<crate::persistence::PersistentDataContainer<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getPersistentDataContainer",
            "()Lorg/bukkit/persistence/PersistentDataContainer;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::persistence::PersistentDataContainer::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn set_type(
        &mut self,
        arg0: impl Into<&'mc crate::Material<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setType",
            "(Lorg/bukkit/Material;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn world(&mut self) -> Result<crate::World<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getWorld", "()Lorg/bukkit/World;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        crate::World::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn data(
        &mut self,
    ) -> Result<crate::material::MaterialData<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getData",
            "()Lorg/bukkit/material/MaterialData;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::material::MaterialData::from_raw(&self.jni_ref(), unsafe {
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

    pub fn block_data(
        &mut self,
    ) -> Result<crate::block::data::BlockData<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getBlockData",
            "()Lorg/bukkit/block/data/BlockData;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::block::data::BlockData::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn light_level(&mut self) -> Result<i8, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getLightLevel", "()B", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.b().unwrap())
    }

    pub fn x(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getX", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }

    pub fn y(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getY", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }

    pub fn z(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getZ", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }

    pub fn chunk(&mut self) -> Result<crate::Chunk<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getChunk", "()Lorg/bukkit/Chunk;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        crate::Chunk::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn set_data(
        &mut self,
        arg0: impl Into<&'mc crate::material::MaterialData<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setData",
            "(Lorg/bukkit/material/MaterialData;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn set_block_data(
        &mut self,
        arg0: impl Into<&'mc crate::block::data::BlockData<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setBlockData",
            "(Lorg/bukkit/block/data/BlockData;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    #[deprecated]

    pub fn raw_data(&mut self) -> Result<i8, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getRawData", "()B", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.b().unwrap())
    }
    #[deprecated]

    pub fn set_raw_data(&mut self, arg0: i8) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Byte(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setRawData",
            "(B)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn is_placed(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isPlaced", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn update(
        &mut self,
        arg0: std::option::Option<bool>,
        arg1: std::option::Option<bool>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        // -1
        let val_1 = jni::objects::JValueGen::Bool(arg0.unwrap().into());
        // -1
        let val_2 = jni::objects::JValueGen::Bool(arg1.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "update",
            "(ZZ)Z",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn location(&mut self) -> Result<crate::Location<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getLocation",
            "()Lorg/bukkit/Location;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::Location::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn get_type(&mut self) -> Result<crate::Material<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getType",
            "()Lorg/bukkit/Material;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant =
            self.jni_ref()
                .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
        let variant_str = self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::Material::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::Material::from_string(variant_str).unwrap(),
        )
    }

    pub fn set_metadata(
        &mut self,
        arg0: impl Into<&'mc String>,
        arg1: impl Into<&'mc crate::metadata::MetadataValue<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let val_2 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setMetadata",
            "(Ljava/lang/String;Lorg/bukkit/metadata/MetadataValue;)V",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn get_metadata(
        &mut self,
        arg0: impl Into<&'mc String>,
    ) -> Result<Vec<crate::metadata::MetadataValue<'mc>>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getMetadata",
            "(Ljava/lang/String;)Ljava/util/List;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let mut list = blackboxmc_java::JavaList::from_raw(&self.jni_ref(), res.l()?)?;
        let size = list.size()?;
        for i in 0..=size {
            let obj = list.get(i)?;
            new_vec.push(crate::metadata::MetadataValue::from_raw(
                &self.jni_ref(),
                obj,
            )?);
        }
        Ok(new_vec)
    }

    pub fn has_metadata(
        &mut self,
        arg0: impl Into<&'mc String>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "hasMetadata",
            "(Ljava/lang/String;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn remove_metadata(
        &mut self,
        arg0: impl Into<&'mc String>,
        arg1: impl Into<&'mc crate::plugin::Plugin<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let val_2 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeMetadata",
            "(Ljava/lang/String;Lorg/bukkit/plugin/Plugin;)V",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
}
impl<'mc> JNIRaw<'mc> for EnderChest<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Into<crate::block::Lidded<'mc>> for EnderChest<'mc> {
    fn into(self) -> crate::block::Lidded<'mc> {
        crate::block::Lidded::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
impl<'mc> Into<crate::block::TileState<'mc>> for EnderChest<'mc> {
    fn into(self) -> crate::block::TileState<'mc> {
        crate::block::TileState::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
/// Represents a captured state of a bed.
///
/// This is a representation of an abstract class.
pub struct Bed<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> Bed<'mc> {
    pub fn from_raw(
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

    pub fn persistent_data_container(
        &mut self,
    ) -> Result<crate::persistence::PersistentDataContainer<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getPersistentDataContainer",
            "()Lorg/bukkit/persistence/PersistentDataContainer;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::persistence::PersistentDataContainer::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn set_type(
        &mut self,
        arg0: impl Into<&'mc crate::Material<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setType",
            "(Lorg/bukkit/Material;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn world(&mut self) -> Result<crate::World<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getWorld", "()Lorg/bukkit/World;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        crate::World::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn data(
        &mut self,
    ) -> Result<crate::material::MaterialData<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getData",
            "()Lorg/bukkit/material/MaterialData;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::material::MaterialData::from_raw(&self.jni_ref(), unsafe {
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

    pub fn block_data(
        &mut self,
    ) -> Result<crate::block::data::BlockData<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getBlockData",
            "()Lorg/bukkit/block/data/BlockData;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::block::data::BlockData::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn light_level(&mut self) -> Result<i8, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getLightLevel", "()B", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.b().unwrap())
    }

    pub fn x(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getX", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }

    pub fn y(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getY", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }

    pub fn z(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getZ", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }

    pub fn chunk(&mut self) -> Result<crate::Chunk<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getChunk", "()Lorg/bukkit/Chunk;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        crate::Chunk::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn set_data(
        &mut self,
        arg0: impl Into<&'mc crate::material::MaterialData<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setData",
            "(Lorg/bukkit/material/MaterialData;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn set_block_data(
        &mut self,
        arg0: impl Into<&'mc crate::block::data::BlockData<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setBlockData",
            "(Lorg/bukkit/block/data/BlockData;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    #[deprecated]

    pub fn raw_data(&mut self) -> Result<i8, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getRawData", "()B", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.b().unwrap())
    }
    #[deprecated]

    pub fn set_raw_data(&mut self, arg0: i8) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Byte(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setRawData",
            "(B)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn is_placed(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isPlaced", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn update(
        &mut self,
        arg0: std::option::Option<bool>,
        arg1: std::option::Option<bool>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        // -1
        let val_1 = jni::objects::JValueGen::Bool(arg0.unwrap().into());
        // -1
        let val_2 = jni::objects::JValueGen::Bool(arg1.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "update",
            "(ZZ)Z",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn location(&mut self) -> Result<crate::Location<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getLocation",
            "()Lorg/bukkit/Location;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::Location::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn get_type(&mut self) -> Result<crate::Material<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getType",
            "()Lorg/bukkit/Material;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant =
            self.jni_ref()
                .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
        let variant_str = self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::Material::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::Material::from_string(variant_str).unwrap(),
        )
    }

    pub fn set_metadata(
        &mut self,
        arg0: impl Into<&'mc String>,
        arg1: impl Into<&'mc crate::metadata::MetadataValue<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let val_2 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setMetadata",
            "(Ljava/lang/String;Lorg/bukkit/metadata/MetadataValue;)V",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn get_metadata(
        &mut self,
        arg0: impl Into<&'mc String>,
    ) -> Result<Vec<crate::metadata::MetadataValue<'mc>>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getMetadata",
            "(Ljava/lang/String;)Ljava/util/List;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let mut list = blackboxmc_java::JavaList::from_raw(&self.jni_ref(), res.l()?)?;
        let size = list.size()?;
        for i in 0..=size {
            let obj = list.get(i)?;
            new_vec.push(crate::metadata::MetadataValue::from_raw(
                &self.jni_ref(),
                obj,
            )?);
        }
        Ok(new_vec)
    }

    pub fn has_metadata(
        &mut self,
        arg0: impl Into<&'mc String>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "hasMetadata",
            "(Ljava/lang/String;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn remove_metadata(
        &mut self,
        arg0: impl Into<&'mc String>,
        arg1: impl Into<&'mc crate::plugin::Plugin<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let val_2 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeMetadata",
            "(Ljava/lang/String;Lorg/bukkit/plugin/Plugin;)V",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn set_color(
        &mut self,
        arg0: impl Into<&'mc crate::DyeColor<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setColor",
            "(Lorg/bukkit/DyeColor;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn color(&mut self) -> Result<crate::DyeColor<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getColor",
            "()Lorg/bukkit/DyeColor;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant =
            self.jni_ref()
                .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
        let variant_str = self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::DyeColor::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::DyeColor::from_string(variant_str).unwrap(),
        )
    }
}
impl<'mc> JNIRaw<'mc> for Bed<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Into<crate::block::TileState<'mc>> for Bed<'mc> {
    fn into(self) -> crate::block::TileState<'mc> {
        crate::block::TileState::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
impl<'mc> Into<crate::material::Colorable<'mc>> for Bed<'mc> {
    fn into(self) -> crate::material::Colorable<'mc> {
        crate::material::Colorable::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
/// Represents a captured state of a jukebox.
///
/// This is a representation of an abstract class.
pub struct Jukebox<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> Jukebox<'mc> {
    pub fn from_raw(
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
    /// Stops the jukebox playing and ejects the current record.
    /// <p>If the block represented by this state is no longer a jukebox, this will do nothing and return false.</p>
    pub fn eject(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "eject", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    /// <span class="descfrm-type-label">Description copied from interface:&nbsp;<code><a href="../inventory/InventoryHolder.html#getInventory()">InventoryHolder</a></code></span>
    /// Get the object's inventory.
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

    pub fn snapshot_inventory(
        &mut self,
    ) -> Result<crate::inventory::JukeboxInventory<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getSnapshotInventory",
            "()Lorg/bukkit/inventory/JukeboxInventory;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::JukeboxInventory::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Sets the record being played. The jukebox will start playing automatically.
    pub fn set_record(
        &mut self,
        arg0: impl Into<&'mc crate::inventory::ItemStack<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setRecord",
            "(Lorg/bukkit/inventory/ItemStack;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Gets the record item inserted into the jukebox.
    pub fn record(
        &mut self,
    ) -> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getRecord",
            "()Lorg/bukkit/inventory/ItemStack;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::ItemStack::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Gets the record inserted into the jukebox.
    pub fn playing(&mut self) -> Result<crate::Material<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getPlaying",
            "()Lorg/bukkit/Material;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant =
            self.jni_ref()
                .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
        let variant_str = self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::Material::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::Material::from_string(variant_str).unwrap(),
        )
    }
    /// Sets the record being played.
    pub fn set_playing(
        &mut self,
        arg0: impl Into<&'mc crate::Material<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setPlaying",
            "(Lorg/bukkit/Material;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Gets whether or not this jukebox has a record.
    /// <p>A jukebox can have a record but not <a href="#isPlaying()"><code>be playing</code></a> if it was stopped with <a href="#stopPlaying()"><code>stopPlaying()</code></a> or if a record has finished playing.</p>
    pub fn has_record(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasRecord", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    /// Checks if the jukebox is playing a record.
    pub fn is_playing(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isPlaying", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    /// Starts the jukebox playing if there is a record.
    pub fn start_playing(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "startPlaying", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    /// Stops the jukebox playing without ejecting the record.
    pub fn stop_playing(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "stopPlaying", "()V", &[]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn persistent_data_container(
        &mut self,
    ) -> Result<crate::persistence::PersistentDataContainer<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getPersistentDataContainer",
            "()Lorg/bukkit/persistence/PersistentDataContainer;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::persistence::PersistentDataContainer::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn set_type(
        &mut self,
        arg0: impl Into<&'mc crate::Material<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setType",
            "(Lorg/bukkit/Material;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn world(&mut self) -> Result<crate::World<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getWorld", "()Lorg/bukkit/World;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        crate::World::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn data(
        &mut self,
    ) -> Result<crate::material::MaterialData<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getData",
            "()Lorg/bukkit/material/MaterialData;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::material::MaterialData::from_raw(&self.jni_ref(), unsafe {
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

    pub fn block_data(
        &mut self,
    ) -> Result<crate::block::data::BlockData<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getBlockData",
            "()Lorg/bukkit/block/data/BlockData;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::block::data::BlockData::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn light_level(&mut self) -> Result<i8, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getLightLevel", "()B", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.b().unwrap())
    }

    pub fn x(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getX", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }

    pub fn y(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getY", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }

    pub fn z(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getZ", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }

    pub fn chunk(&mut self) -> Result<crate::Chunk<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getChunk", "()Lorg/bukkit/Chunk;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        crate::Chunk::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn set_data(
        &mut self,
        arg0: impl Into<&'mc crate::material::MaterialData<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setData",
            "(Lorg/bukkit/material/MaterialData;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn set_block_data(
        &mut self,
        arg0: impl Into<&'mc crate::block::data::BlockData<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setBlockData",
            "(Lorg/bukkit/block/data/BlockData;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    #[deprecated]

    pub fn raw_data(&mut self) -> Result<i8, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getRawData", "()B", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.b().unwrap())
    }
    #[deprecated]

    pub fn set_raw_data(&mut self, arg0: i8) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Byte(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setRawData",
            "(B)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn is_placed(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isPlaced", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn update(
        &mut self,
        arg0: std::option::Option<bool>,
        arg1: std::option::Option<bool>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        // -1
        let val_1 = jni::objects::JValueGen::Bool(arg0.unwrap().into());
        // -1
        let val_2 = jni::objects::JValueGen::Bool(arg1.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "update",
            "(ZZ)Z",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn location(&mut self) -> Result<crate::Location<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getLocation",
            "()Lorg/bukkit/Location;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::Location::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn get_type(&mut self) -> Result<crate::Material<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getType",
            "()Lorg/bukkit/Material;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant =
            self.jni_ref()
                .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
        let variant_str = self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::Material::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::Material::from_string(variant_str).unwrap(),
        )
    }

    pub fn set_metadata(
        &mut self,
        arg0: impl Into<&'mc String>,
        arg1: impl Into<&'mc crate::metadata::MetadataValue<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let val_2 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setMetadata",
            "(Ljava/lang/String;Lorg/bukkit/metadata/MetadataValue;)V",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn get_metadata(
        &mut self,
        arg0: impl Into<&'mc String>,
    ) -> Result<Vec<crate::metadata::MetadataValue<'mc>>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getMetadata",
            "(Ljava/lang/String;)Ljava/util/List;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let mut list = blackboxmc_java::JavaList::from_raw(&self.jni_ref(), res.l()?)?;
        let size = list.size()?;
        for i in 0..=size {
            let obj = list.get(i)?;
            new_vec.push(crate::metadata::MetadataValue::from_raw(
                &self.jni_ref(),
                obj,
            )?);
        }
        Ok(new_vec)
    }

    pub fn has_metadata(
        &mut self,
        arg0: impl Into<&'mc String>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "hasMetadata",
            "(Ljava/lang/String;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn remove_metadata(
        &mut self,
        arg0: impl Into<&'mc String>,
        arg1: impl Into<&'mc crate::plugin::Plugin<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let val_2 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeMetadata",
            "(Ljava/lang/String;Lorg/bukkit/plugin/Plugin;)V",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
}
impl<'mc> JNIRaw<'mc> for Jukebox<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Into<crate::block::TileState<'mc>> for Jukebox<'mc> {
    fn into(self) -> crate::block::TileState<'mc> {
        crate::block::TileState::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
impl<'mc> Into<crate::inventory::BlockInventoryHolder<'mc>> for Jukebox<'mc> {
    fn into(self) -> crate::inventory::BlockInventoryHolder<'mc> {
        crate::inventory::BlockInventoryHolder::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
/// Represents a captured state of a banner.
///
/// This is a representation of an abstract class.
pub struct Banner<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> Banner<'mc> {
    pub fn from_raw(
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
    /// Returns the base color for this banner
    pub fn base_color(&mut self) -> Result<crate::DyeColor<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getBaseColor",
            "()Lorg/bukkit/DyeColor;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant =
            self.jni_ref()
                .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
        let variant_str = self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::DyeColor::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::DyeColor::from_string(variant_str).unwrap(),
        )
    }
    /// Sets the base color for this banner. <b>Only valid for shield pseudo banners, otherwise base depends on block type</b>
    pub fn set_base_color(
        &mut self,
        arg0: impl Into<&'mc crate::DyeColor<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setBaseColor",
            "(Lorg/bukkit/DyeColor;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Returns a list of patterns on this banner
    pub fn patterns(
        &mut self,
    ) -> Result<Vec<crate::block::banner::Pattern<'mc>>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getPatterns",
            "()Ljava/util/List;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let mut list = blackboxmc_java::JavaList::from_raw(&self.jni_ref(), res.l()?)?;
        let size = list.size()?;
        for i in 0..=size {
            let obj = list.get(i)?;
            new_vec.push(crate::block::banner::Pattern::from_raw(
                &self.jni_ref(),
                obj,
            )?);
        }
        Ok(new_vec)
    }
    /// Sets the patterns used on this banner
    pub fn set_patterns(
        &mut self,
        arg0: Vec<impl Into<crate::block::banner::Pattern<'mc>>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let raw_val_1 = self
            .jni_ref()
            .new_object("java/util/ArrayList", "()V", &[])
            .unwrap();
        for v in arg0 {
            let map_val_0 =
                unsafe { jni::objects::JObject::from_raw(v.into().jni_object().clone()) };
            self.jni_ref().call_method(
                &raw_val_1,
                "add",
                "(Ljava/Lang/Object)V",
                &[jni::objects::JValueGen::from(&map_val_0)],
            )?;
        }
        let val_1 = jni::objects::JValueGen::Object(raw_val_1);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setPatterns",
            "(Ljava/util/List;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Adds a new pattern on top of the existing patterns
    pub fn add_pattern(
        &mut self,
        arg0: impl Into<&'mc crate::block::banner::Pattern<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "addPattern",
            "(Lorg/bukkit/block/banner/Pattern;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Returns a list of patterns on this banner
    /// Returns the pattern at the specified index
    pub fn get_pattern(
        &mut self,
        arg0: i32,
    ) -> Result<crate::block::banner::Pattern<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getPattern",
            "(I)Lorg/bukkit/block/banner/Pattern;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::block::banner::Pattern::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Removes the pattern at the specified index
    pub fn remove_pattern(
        &mut self,
        arg0: i32,
    ) -> Result<crate::block::banner::Pattern<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removePattern",
            "(I)Lorg/bukkit/block/banner/Pattern;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::block::banner::Pattern::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Sets the patterns used on this banner
    /// Sets the pattern at the specified index
    pub fn set_pattern(
        &mut self,
        arg0: i32,
        arg1: impl Into<&'mc crate::block::banner::Pattern<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let val_2 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setPattern",
            "(ILorg/bukkit/block/banner/Pattern;)V",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Returns the number of patterns on this banner
    pub fn number_of_patterns(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "numberOfPatterns", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }

    pub fn persistent_data_container(
        &mut self,
    ) -> Result<crate::persistence::PersistentDataContainer<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getPersistentDataContainer",
            "()Lorg/bukkit/persistence/PersistentDataContainer;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::persistence::PersistentDataContainer::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn set_type(
        &mut self,
        arg0: impl Into<&'mc crate::Material<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setType",
            "(Lorg/bukkit/Material;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn world(&mut self) -> Result<crate::World<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getWorld", "()Lorg/bukkit/World;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        crate::World::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn data(
        &mut self,
    ) -> Result<crate::material::MaterialData<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getData",
            "()Lorg/bukkit/material/MaterialData;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::material::MaterialData::from_raw(&self.jni_ref(), unsafe {
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

    pub fn block_data(
        &mut self,
    ) -> Result<crate::block::data::BlockData<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getBlockData",
            "()Lorg/bukkit/block/data/BlockData;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::block::data::BlockData::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn light_level(&mut self) -> Result<i8, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getLightLevel", "()B", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.b().unwrap())
    }

    pub fn x(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getX", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }

    pub fn y(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getY", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }

    pub fn z(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getZ", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }

    pub fn chunk(&mut self) -> Result<crate::Chunk<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getChunk", "()Lorg/bukkit/Chunk;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        crate::Chunk::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn set_data(
        &mut self,
        arg0: impl Into<&'mc crate::material::MaterialData<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setData",
            "(Lorg/bukkit/material/MaterialData;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn set_block_data(
        &mut self,
        arg0: impl Into<&'mc crate::block::data::BlockData<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setBlockData",
            "(Lorg/bukkit/block/data/BlockData;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    #[deprecated]

    pub fn raw_data(&mut self) -> Result<i8, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getRawData", "()B", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.b().unwrap())
    }
    #[deprecated]

    pub fn set_raw_data(&mut self, arg0: i8) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Byte(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setRawData",
            "(B)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn is_placed(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isPlaced", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn update(
        &mut self,
        arg0: std::option::Option<bool>,
        arg1: std::option::Option<bool>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        // -1
        let val_1 = jni::objects::JValueGen::Bool(arg0.unwrap().into());
        // -1
        let val_2 = jni::objects::JValueGen::Bool(arg1.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "update",
            "(ZZ)Z",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn location(&mut self) -> Result<crate::Location<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getLocation",
            "()Lorg/bukkit/Location;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::Location::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn get_type(&mut self) -> Result<crate::Material<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getType",
            "()Lorg/bukkit/Material;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant =
            self.jni_ref()
                .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
        let variant_str = self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::Material::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::Material::from_string(variant_str).unwrap(),
        )
    }

    pub fn set_metadata(
        &mut self,
        arg0: impl Into<&'mc String>,
        arg1: impl Into<&'mc crate::metadata::MetadataValue<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let val_2 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setMetadata",
            "(Ljava/lang/String;Lorg/bukkit/metadata/MetadataValue;)V",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn get_metadata(
        &mut self,
        arg0: impl Into<&'mc String>,
    ) -> Result<Vec<crate::metadata::MetadataValue<'mc>>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getMetadata",
            "(Ljava/lang/String;)Ljava/util/List;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let mut list = blackboxmc_java::JavaList::from_raw(&self.jni_ref(), res.l()?)?;
        let size = list.size()?;
        for i in 0..=size {
            let obj = list.get(i)?;
            new_vec.push(crate::metadata::MetadataValue::from_raw(
                &self.jni_ref(),
                obj,
            )?);
        }
        Ok(new_vec)
    }

    pub fn has_metadata(
        &mut self,
        arg0: impl Into<&'mc String>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "hasMetadata",
            "(Ljava/lang/String;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn remove_metadata(
        &mut self,
        arg0: impl Into<&'mc String>,
        arg1: impl Into<&'mc crate::plugin::Plugin<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let val_2 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeMetadata",
            "(Ljava/lang/String;Lorg/bukkit/plugin/Plugin;)V",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
}
impl<'mc> JNIRaw<'mc> for Banner<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Into<crate::block::TileState<'mc>> for Banner<'mc> {
    fn into(self) -> crate::block::TileState<'mc> {
        crate::block::TileState::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
/// Represents a captured state of a Barrel.
///
/// This is a representation of an abstract class.
pub struct Barrel<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> Barrel<'mc> {
    pub fn from_raw(
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

    pub fn snapshot_inventory(
        &mut self,
    ) -> Result<crate::inventory::Inventory<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getSnapshotInventory",
            "()Lorg/bukkit/inventory/Inventory;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::Inventory::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn persistent_data_container(
        &mut self,
    ) -> Result<crate::persistence::PersistentDataContainer<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getPersistentDataContainer",
            "()Lorg/bukkit/persistence/PersistentDataContainer;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::persistence::PersistentDataContainer::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn set_type(
        &mut self,
        arg0: impl Into<&'mc crate::Material<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setType",
            "(Lorg/bukkit/Material;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn world(&mut self) -> Result<crate::World<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getWorld", "()Lorg/bukkit/World;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        crate::World::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn data(
        &mut self,
    ) -> Result<crate::material::MaterialData<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getData",
            "()Lorg/bukkit/material/MaterialData;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::material::MaterialData::from_raw(&self.jni_ref(), unsafe {
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

    pub fn block_data(
        &mut self,
    ) -> Result<crate::block::data::BlockData<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getBlockData",
            "()Lorg/bukkit/block/data/BlockData;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::block::data::BlockData::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn light_level(&mut self) -> Result<i8, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getLightLevel", "()B", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.b().unwrap())
    }

    pub fn x(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getX", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }

    pub fn y(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getY", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }

    pub fn z(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getZ", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }

    pub fn chunk(&mut self) -> Result<crate::Chunk<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getChunk", "()Lorg/bukkit/Chunk;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        crate::Chunk::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn set_data(
        &mut self,
        arg0: impl Into<&'mc crate::material::MaterialData<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setData",
            "(Lorg/bukkit/material/MaterialData;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn set_block_data(
        &mut self,
        arg0: impl Into<&'mc crate::block::data::BlockData<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setBlockData",
            "(Lorg/bukkit/block/data/BlockData;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    #[deprecated]

    pub fn raw_data(&mut self) -> Result<i8, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getRawData", "()B", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.b().unwrap())
    }
    #[deprecated]

    pub fn set_raw_data(&mut self, arg0: i8) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Byte(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setRawData",
            "(B)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn is_placed(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isPlaced", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn update(
        &mut self,
        arg0: std::option::Option<bool>,
        arg1: std::option::Option<bool>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        // -1
        let val_1 = jni::objects::JValueGen::Bool(arg0.unwrap().into());
        // -1
        let val_2 = jni::objects::JValueGen::Bool(arg1.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "update",
            "(ZZ)Z",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn location(&mut self) -> Result<crate::Location<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getLocation",
            "()Lorg/bukkit/Location;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::Location::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn get_type(&mut self) -> Result<crate::Material<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getType",
            "()Lorg/bukkit/Material;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant =
            self.jni_ref()
                .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
        let variant_str = self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::Material::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::Material::from_string(variant_str).unwrap(),
        )
    }

    pub fn set_metadata(
        &mut self,
        arg0: impl Into<&'mc String>,
        arg1: impl Into<&'mc crate::metadata::MetadataValue<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let val_2 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setMetadata",
            "(Ljava/lang/String;Lorg/bukkit/metadata/MetadataValue;)V",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn get_metadata(
        &mut self,
        arg0: impl Into<&'mc String>,
    ) -> Result<Vec<crate::metadata::MetadataValue<'mc>>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getMetadata",
            "(Ljava/lang/String;)Ljava/util/List;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let mut list = blackboxmc_java::JavaList::from_raw(&self.jni_ref(), res.l()?)?;
        let size = list.size()?;
        for i in 0..=size {
            let obj = list.get(i)?;
            new_vec.push(crate::metadata::MetadataValue::from_raw(
                &self.jni_ref(),
                obj,
            )?);
        }
        Ok(new_vec)
    }

    pub fn has_metadata(
        &mut self,
        arg0: impl Into<&'mc String>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "hasMetadata",
            "(Ljava/lang/String;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn remove_metadata(
        &mut self,
        arg0: impl Into<&'mc String>,
        arg1: impl Into<&'mc crate::plugin::Plugin<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let val_2 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeMetadata",
            "(Ljava/lang/String;Lorg/bukkit/plugin/Plugin;)V",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn is_locked(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isLocked", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn lock(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getLock", "()Ljava/lang/String;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }

    pub fn set_lock(
        &mut self,
        arg0: impl Into<&'mc String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setLock",
            "(Ljava/lang/String;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn custom_name(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getCustomName",
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

    pub fn set_custom_name(
        &mut self,
        arg0: impl Into<&'mc String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setCustomName",
            "(Ljava/lang/String;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn seed(&mut self) -> Result<i64, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getSeed", "()J", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.j().unwrap())
    }

    pub fn set_seed(&mut self, arg0: i64) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Long(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setSeed",
            "(J)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn set_loot_table(
        &mut self,
        arg0: impl Into<&'mc crate::loot::LootTable<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setLootTable",
            "(Lorg/bukkit/loot/LootTable;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn loot_table(
        &mut self,
    ) -> Result<crate::loot::LootTable<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getLootTable",
            "()Lorg/bukkit/loot/LootTable;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::loot::LootTable::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn close(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "close", "()V", &[]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn open(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "open", "()V", &[]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
}
impl<'mc> JNIRaw<'mc> for Barrel<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Into<crate::block::Container<'mc>> for Barrel<'mc> {
    fn into(self) -> crate::block::Container<'mc> {
        crate::block::Container::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
impl<'mc> Into<crate::loot::Lootable<'mc>> for Barrel<'mc> {
    fn into(self) -> crate::loot::Lootable<'mc> {
        crate::loot::Lootable::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
impl<'mc> Into<crate::block::Lidded<'mc>> for Barrel<'mc> {
    fn into(self) -> crate::block::Lidded<'mc> {
        crate::block::Lidded::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
/// Represents a captured state of a command block.
///
/// This is a representation of an abstract class.
pub struct CommandBlock<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> CommandBlock<'mc> {
    pub fn from_raw(
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
    /// Gets the command that this CommandBlock will run when powered. This will never return null. If the CommandBlock does not have a command, an empty String will be returned instead.
    pub fn command(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getCommand",
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
    /// Sets the command that this CommandBlock will run when powered. Setting the command to null is the same as setting it to an empty String.
    pub fn set_command(
        &mut self,
        arg0: impl Into<&'mc String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setCommand",
            "(Ljava/lang/String;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Gets the name of this CommandBlock. The name is used with commands that this CommandBlock executes. This name will never be null, and by default is "@".
    pub fn name(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getName", "()Ljava/lang/String;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    /// Sets the name of this CommandBlock. The name is used with commands that this CommandBlock executes. Setting the name to null is the same as setting it to "@".
    pub fn set_name(
        &mut self,
        arg0: impl Into<&'mc String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setName",
            "(Ljava/lang/String;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn persistent_data_container(
        &mut self,
    ) -> Result<crate::persistence::PersistentDataContainer<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getPersistentDataContainer",
            "()Lorg/bukkit/persistence/PersistentDataContainer;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::persistence::PersistentDataContainer::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn set_type(
        &mut self,
        arg0: impl Into<&'mc crate::Material<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setType",
            "(Lorg/bukkit/Material;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn world(&mut self) -> Result<crate::World<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getWorld", "()Lorg/bukkit/World;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        crate::World::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn data(
        &mut self,
    ) -> Result<crate::material::MaterialData<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getData",
            "()Lorg/bukkit/material/MaterialData;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::material::MaterialData::from_raw(&self.jni_ref(), unsafe {
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

    pub fn block_data(
        &mut self,
    ) -> Result<crate::block::data::BlockData<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getBlockData",
            "()Lorg/bukkit/block/data/BlockData;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::block::data::BlockData::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn light_level(&mut self) -> Result<i8, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getLightLevel", "()B", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.b().unwrap())
    }

    pub fn x(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getX", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }

    pub fn y(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getY", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }

    pub fn z(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getZ", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }

    pub fn chunk(&mut self) -> Result<crate::Chunk<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getChunk", "()Lorg/bukkit/Chunk;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        crate::Chunk::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn set_data(
        &mut self,
        arg0: impl Into<&'mc crate::material::MaterialData<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setData",
            "(Lorg/bukkit/material/MaterialData;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn set_block_data(
        &mut self,
        arg0: impl Into<&'mc crate::block::data::BlockData<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setBlockData",
            "(Lorg/bukkit/block/data/BlockData;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    #[deprecated]

    pub fn raw_data(&mut self) -> Result<i8, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getRawData", "()B", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.b().unwrap())
    }
    #[deprecated]

    pub fn set_raw_data(&mut self, arg0: i8) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Byte(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setRawData",
            "(B)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn is_placed(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isPlaced", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn update(
        &mut self,
        arg0: std::option::Option<bool>,
        arg1: std::option::Option<bool>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        // -1
        let val_1 = jni::objects::JValueGen::Bool(arg0.unwrap().into());
        // -1
        let val_2 = jni::objects::JValueGen::Bool(arg1.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "update",
            "(ZZ)Z",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn location(&mut self) -> Result<crate::Location<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getLocation",
            "()Lorg/bukkit/Location;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::Location::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn get_type(&mut self) -> Result<crate::Material<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getType",
            "()Lorg/bukkit/Material;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant =
            self.jni_ref()
                .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
        let variant_str = self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::Material::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::Material::from_string(variant_str).unwrap(),
        )
    }

    pub fn set_metadata(
        &mut self,
        arg0: impl Into<&'mc String>,
        arg1: impl Into<&'mc crate::metadata::MetadataValue<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let val_2 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setMetadata",
            "(Ljava/lang/String;Lorg/bukkit/metadata/MetadataValue;)V",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn get_metadata(
        &mut self,
        arg0: impl Into<&'mc String>,
    ) -> Result<Vec<crate::metadata::MetadataValue<'mc>>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getMetadata",
            "(Ljava/lang/String;)Ljava/util/List;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let mut list = blackboxmc_java::JavaList::from_raw(&self.jni_ref(), res.l()?)?;
        let size = list.size()?;
        for i in 0..=size {
            let obj = list.get(i)?;
            new_vec.push(crate::metadata::MetadataValue::from_raw(
                &self.jni_ref(),
                obj,
            )?);
        }
        Ok(new_vec)
    }

    pub fn has_metadata(
        &mut self,
        arg0: impl Into<&'mc String>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "hasMetadata",
            "(Ljava/lang/String;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn remove_metadata(
        &mut self,
        arg0: impl Into<&'mc String>,
        arg1: impl Into<&'mc crate::plugin::Plugin<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let val_2 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeMetadata",
            "(Ljava/lang/String;Lorg/bukkit/plugin/Plugin;)V",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
}
impl<'mc> JNIRaw<'mc> for CommandBlock<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Into<crate::block::TileState<'mc>> for CommandBlock<'mc> {
    fn into(self) -> crate::block::TileState<'mc> {
        crate::block::TileState::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
/// Represents a captured state of a sculk sensor
///
/// This is a representation of an abstract class.
pub struct SculkSensor<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> SculkSensor<'mc> {
    pub fn from_raw(
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
    /// Gets the last vibration frequency of this sensor. Different activities detected by the sensor will produce different frequencies and dictate the output of connected comparators.
    pub fn last_vibration_frequency(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getLastVibrationFrequency", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }
    /// Sets the last vibration frequency of this sensor. Different activities detected by the sensor will produce different frequencies and dictate the output of connected comparators.
    pub fn set_last_vibration_frequency(
        &mut self,
        arg0: i32,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setLastVibrationFrequency",
            "(I)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn persistent_data_container(
        &mut self,
    ) -> Result<crate::persistence::PersistentDataContainer<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getPersistentDataContainer",
            "()Lorg/bukkit/persistence/PersistentDataContainer;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::persistence::PersistentDataContainer::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn set_type(
        &mut self,
        arg0: impl Into<&'mc crate::Material<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setType",
            "(Lorg/bukkit/Material;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn world(&mut self) -> Result<crate::World<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getWorld", "()Lorg/bukkit/World;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        crate::World::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn data(
        &mut self,
    ) -> Result<crate::material::MaterialData<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getData",
            "()Lorg/bukkit/material/MaterialData;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::material::MaterialData::from_raw(&self.jni_ref(), unsafe {
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

    pub fn block_data(
        &mut self,
    ) -> Result<crate::block::data::BlockData<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getBlockData",
            "()Lorg/bukkit/block/data/BlockData;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::block::data::BlockData::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn light_level(&mut self) -> Result<i8, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getLightLevel", "()B", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.b().unwrap())
    }

    pub fn x(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getX", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }

    pub fn y(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getY", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }

    pub fn z(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getZ", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }

    pub fn chunk(&mut self) -> Result<crate::Chunk<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getChunk", "()Lorg/bukkit/Chunk;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        crate::Chunk::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn set_data(
        &mut self,
        arg0: impl Into<&'mc crate::material::MaterialData<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setData",
            "(Lorg/bukkit/material/MaterialData;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn set_block_data(
        &mut self,
        arg0: impl Into<&'mc crate::block::data::BlockData<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setBlockData",
            "(Lorg/bukkit/block/data/BlockData;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    #[deprecated]

    pub fn raw_data(&mut self) -> Result<i8, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getRawData", "()B", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.b().unwrap())
    }
    #[deprecated]

    pub fn set_raw_data(&mut self, arg0: i8) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Byte(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setRawData",
            "(B)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn is_placed(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isPlaced", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn update(
        &mut self,
        arg0: std::option::Option<bool>,
        arg1: std::option::Option<bool>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        // -1
        let val_1 = jni::objects::JValueGen::Bool(arg0.unwrap().into());
        // -1
        let val_2 = jni::objects::JValueGen::Bool(arg1.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "update",
            "(ZZ)Z",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn location(&mut self) -> Result<crate::Location<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getLocation",
            "()Lorg/bukkit/Location;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::Location::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn get_type(&mut self) -> Result<crate::Material<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getType",
            "()Lorg/bukkit/Material;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant =
            self.jni_ref()
                .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
        let variant_str = self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::Material::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::Material::from_string(variant_str).unwrap(),
        )
    }

    pub fn set_metadata(
        &mut self,
        arg0: impl Into<&'mc String>,
        arg1: impl Into<&'mc crate::metadata::MetadataValue<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let val_2 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setMetadata",
            "(Ljava/lang/String;Lorg/bukkit/metadata/MetadataValue;)V",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn get_metadata(
        &mut self,
        arg0: impl Into<&'mc String>,
    ) -> Result<Vec<crate::metadata::MetadataValue<'mc>>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getMetadata",
            "(Ljava/lang/String;)Ljava/util/List;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let mut list = blackboxmc_java::JavaList::from_raw(&self.jni_ref(), res.l()?)?;
        let size = list.size()?;
        for i in 0..=size {
            let obj = list.get(i)?;
            new_vec.push(crate::metadata::MetadataValue::from_raw(
                &self.jni_ref(),
                obj,
            )?);
        }
        Ok(new_vec)
    }

    pub fn has_metadata(
        &mut self,
        arg0: impl Into<&'mc String>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "hasMetadata",
            "(Ljava/lang/String;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn remove_metadata(
        &mut self,
        arg0: impl Into<&'mc String>,
        arg1: impl Into<&'mc crate::plugin::Plugin<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let val_2 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeMetadata",
            "(Ljava/lang/String;Lorg/bukkit/plugin/Plugin;)V",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
}
impl<'mc> JNIRaw<'mc> for SculkSensor<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Into<crate::block::TileState<'mc>> for SculkSensor<'mc> {
    fn into(self) -> crate::block::TileState<'mc> {
        crate::block::TileState::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
/// Represents a captured state of a chiseled bookshelf.
///
/// This is a representation of an abstract class.
pub struct ChiseledBookshelf<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> ChiseledBookshelf<'mc> {
    pub fn from_raw(
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
    /// <span class="descfrm-type-label">Description copied from interface:&nbsp;<code><a href="../inventory/InventoryHolder.html#getInventory()">InventoryHolder</a></code></span>
    /// Get the object's inventory.
    pub fn inventory(
        &mut self,
    ) -> Result<crate::inventory::ChiseledBookshelfInventory<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getInventory",
            "()Lorg/bukkit/inventory/ChiseledBookshelfInventory;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::ChiseledBookshelfInventory::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn snapshot_inventory(
        &mut self,
    ) -> Result<crate::inventory::ChiseledBookshelfInventory<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getSnapshotInventory",
            "()Lorg/bukkit/inventory/ChiseledBookshelfInventory;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::ChiseledBookshelfInventory::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Gets the last interacted inventory slot.
    pub fn last_interacted_slot(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getLastInteractedSlot", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }
    /// Sets the last interacted inventory slot.
    pub fn set_last_interacted_slot(
        &mut self,
        arg0: i32,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setLastInteractedSlot",
            "(I)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Gets the appropriate slot based on a vector relative to this block.
    ///
    /// Will return -1 if the given vector is not within the bounds of any slot.
    /// <p>The supplied vector should only contain components with values between 0.0 and 1.0, inclusive.</p>
    pub fn get_slot(
        &mut self,
        arg0: impl Into<&'mc crate::util::Vector<'mc>>,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getSlot",
            "(Lorg/bukkit/util/Vector;)I",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }

    pub fn persistent_data_container(
        &mut self,
    ) -> Result<crate::persistence::PersistentDataContainer<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getPersistentDataContainer",
            "()Lorg/bukkit/persistence/PersistentDataContainer;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::persistence::PersistentDataContainer::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn set_type(
        &mut self,
        arg0: impl Into<&'mc crate::Material<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setType",
            "(Lorg/bukkit/Material;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn world(&mut self) -> Result<crate::World<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getWorld", "()Lorg/bukkit/World;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        crate::World::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn data(
        &mut self,
    ) -> Result<crate::material::MaterialData<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getData",
            "()Lorg/bukkit/material/MaterialData;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::material::MaterialData::from_raw(&self.jni_ref(), unsafe {
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

    pub fn block_data(
        &mut self,
    ) -> Result<crate::block::data::BlockData<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getBlockData",
            "()Lorg/bukkit/block/data/BlockData;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::block::data::BlockData::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn light_level(&mut self) -> Result<i8, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getLightLevel", "()B", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.b().unwrap())
    }

    pub fn x(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getX", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }

    pub fn y(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getY", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }

    pub fn z(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getZ", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }

    pub fn chunk(&mut self) -> Result<crate::Chunk<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getChunk", "()Lorg/bukkit/Chunk;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        crate::Chunk::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn set_data(
        &mut self,
        arg0: impl Into<&'mc crate::material::MaterialData<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setData",
            "(Lorg/bukkit/material/MaterialData;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn set_block_data(
        &mut self,
        arg0: impl Into<&'mc crate::block::data::BlockData<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setBlockData",
            "(Lorg/bukkit/block/data/BlockData;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    #[deprecated]

    pub fn raw_data(&mut self) -> Result<i8, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getRawData", "()B", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.b().unwrap())
    }
    #[deprecated]

    pub fn set_raw_data(&mut self, arg0: i8) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Byte(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setRawData",
            "(B)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn is_placed(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isPlaced", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn update(
        &mut self,
        arg0: std::option::Option<bool>,
        arg1: std::option::Option<bool>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        // -1
        let val_1 = jni::objects::JValueGen::Bool(arg0.unwrap().into());
        // -1
        let val_2 = jni::objects::JValueGen::Bool(arg1.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "update",
            "(ZZ)Z",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn location(&mut self) -> Result<crate::Location<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getLocation",
            "()Lorg/bukkit/Location;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::Location::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn get_type(&mut self) -> Result<crate::Material<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getType",
            "()Lorg/bukkit/Material;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant =
            self.jni_ref()
                .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
        let variant_str = self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::Material::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::Material::from_string(variant_str).unwrap(),
        )
    }

    pub fn set_metadata(
        &mut self,
        arg0: impl Into<&'mc String>,
        arg1: impl Into<&'mc crate::metadata::MetadataValue<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let val_2 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setMetadata",
            "(Ljava/lang/String;Lorg/bukkit/metadata/MetadataValue;)V",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn get_metadata(
        &mut self,
        arg0: impl Into<&'mc String>,
    ) -> Result<Vec<crate::metadata::MetadataValue<'mc>>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getMetadata",
            "(Ljava/lang/String;)Ljava/util/List;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let mut list = blackboxmc_java::JavaList::from_raw(&self.jni_ref(), res.l()?)?;
        let size = list.size()?;
        for i in 0..=size {
            let obj = list.get(i)?;
            new_vec.push(crate::metadata::MetadataValue::from_raw(
                &self.jni_ref(),
                obj,
            )?);
        }
        Ok(new_vec)
    }

    pub fn has_metadata(
        &mut self,
        arg0: impl Into<&'mc String>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "hasMetadata",
            "(Ljava/lang/String;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn remove_metadata(
        &mut self,
        arg0: impl Into<&'mc String>,
        arg1: impl Into<&'mc crate::plugin::Plugin<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let val_2 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeMetadata",
            "(Ljava/lang/String;Lorg/bukkit/plugin/Plugin;)V",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
}
impl<'mc> JNIRaw<'mc> for ChiseledBookshelf<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Into<crate::block::TileState<'mc>> for ChiseledBookshelf<'mc> {
    fn into(self) -> crate::block::TileState<'mc> {
        crate::block::TileState::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
impl<'mc> Into<crate::inventory::BlockInventoryHolder<'mc>> for ChiseledBookshelf<'mc> {
    fn into(self) -> crate::inventory::BlockInventoryHolder<'mc> {
        crate::inventory::BlockInventoryHolder::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
/// Represents a captured state of a hanging sign.
///
/// This is a representation of an abstract class.
pub struct HangingSign<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> HangingSign<'mc> {
    pub fn from_raw(
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
    #[deprecated]

    pub fn set_color(
        &mut self,
        arg0: impl Into<&'mc crate::DyeColor<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setColor",
            "(Lorg/bukkit/DyeColor;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    #[deprecated]

    pub fn is_glowing_text(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isGlowingText", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    #[deprecated]

    pub fn get_line(&mut self, arg0: i32) -> Result<String, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getLine",
            "(I)Ljava/lang/String;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    #[deprecated]

    pub fn set_line(
        &mut self,
        arg0: i32,
        arg1: impl Into<&'mc String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let val_2 = jni::objects::JObject::from(self.jni_ref().new_string(arg1.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setLine",
            "(ILjava/lang/String;)V",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    #[deprecated]

    pub fn is_editable(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isEditable", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    #[deprecated]

    pub fn set_editable(&mut self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        // -2
        let val_1 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setEditable",
            "(Z)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn is_waxed(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isWaxed", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn set_waxed(&mut self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        // -2
        let val_1 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setWaxed",
            "(Z)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    #[deprecated]

    pub fn set_glowing_text(&mut self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        // -2
        let val_1 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setGlowingText",
            "(Z)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    #[deprecated]

    pub fn color(&mut self) -> Result<crate::DyeColor<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getColor",
            "()Lorg/bukkit/DyeColor;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant =
            self.jni_ref()
                .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
        let variant_str = self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::DyeColor::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::DyeColor::from_string(variant_str).unwrap(),
        )
    }

    pub fn get_side(
        &mut self,
        arg0: impl Into<&'mc crate::block::sign::Side<'mc>>,
    ) -> Result<crate::block::sign::SignSide<'mc>, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getSide",
            "(Lorg/bukkit/block/sign/Side;)Lorg/bukkit/block/sign/SignSide;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::block::sign::SignSide::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn persistent_data_container(
        &mut self,
    ) -> Result<crate::persistence::PersistentDataContainer<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getPersistentDataContainer",
            "()Lorg/bukkit/persistence/PersistentDataContainer;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::persistence::PersistentDataContainer::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn set_type(
        &mut self,
        arg0: impl Into<&'mc crate::Material<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setType",
            "(Lorg/bukkit/Material;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn world(&mut self) -> Result<crate::World<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getWorld", "()Lorg/bukkit/World;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        crate::World::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn data(
        &mut self,
    ) -> Result<crate::material::MaterialData<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getData",
            "()Lorg/bukkit/material/MaterialData;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::material::MaterialData::from_raw(&self.jni_ref(), unsafe {
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

    pub fn block_data(
        &mut self,
    ) -> Result<crate::block::data::BlockData<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getBlockData",
            "()Lorg/bukkit/block/data/BlockData;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::block::data::BlockData::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn light_level(&mut self) -> Result<i8, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getLightLevel", "()B", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.b().unwrap())
    }

    pub fn x(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getX", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }

    pub fn y(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getY", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }

    pub fn z(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getZ", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }

    pub fn chunk(&mut self) -> Result<crate::Chunk<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getChunk", "()Lorg/bukkit/Chunk;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        crate::Chunk::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn set_data(
        &mut self,
        arg0: impl Into<&'mc crate::material::MaterialData<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setData",
            "(Lorg/bukkit/material/MaterialData;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn set_block_data(
        &mut self,
        arg0: impl Into<&'mc crate::block::data::BlockData<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setBlockData",
            "(Lorg/bukkit/block/data/BlockData;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    #[deprecated]

    pub fn raw_data(&mut self) -> Result<i8, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getRawData", "()B", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.b().unwrap())
    }
    #[deprecated]

    pub fn set_raw_data(&mut self, arg0: i8) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Byte(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setRawData",
            "(B)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn is_placed(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isPlaced", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn update(
        &mut self,
        arg0: std::option::Option<bool>,
        arg1: std::option::Option<bool>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        // -1
        let val_1 = jni::objects::JValueGen::Bool(arg0.unwrap().into());
        // -1
        let val_2 = jni::objects::JValueGen::Bool(arg1.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "update",
            "(ZZ)Z",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn location(&mut self) -> Result<crate::Location<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getLocation",
            "()Lorg/bukkit/Location;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::Location::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn get_type(&mut self) -> Result<crate::Material<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getType",
            "()Lorg/bukkit/Material;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant =
            self.jni_ref()
                .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
        let variant_str = self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::Material::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::Material::from_string(variant_str).unwrap(),
        )
    }

    pub fn set_metadata(
        &mut self,
        arg0: impl Into<&'mc String>,
        arg1: impl Into<&'mc crate::metadata::MetadataValue<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let val_2 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setMetadata",
            "(Ljava/lang/String;Lorg/bukkit/metadata/MetadataValue;)V",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn get_metadata(
        &mut self,
        arg0: impl Into<&'mc String>,
    ) -> Result<Vec<crate::metadata::MetadataValue<'mc>>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getMetadata",
            "(Ljava/lang/String;)Ljava/util/List;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let mut list = blackboxmc_java::JavaList::from_raw(&self.jni_ref(), res.l()?)?;
        let size = list.size()?;
        for i in 0..=size {
            let obj = list.get(i)?;
            new_vec.push(crate::metadata::MetadataValue::from_raw(
                &self.jni_ref(),
                obj,
            )?);
        }
        Ok(new_vec)
    }

    pub fn has_metadata(
        &mut self,
        arg0: impl Into<&'mc String>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "hasMetadata",
            "(Ljava/lang/String;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn remove_metadata(
        &mut self,
        arg0: impl Into<&'mc String>,
        arg1: impl Into<&'mc crate::plugin::Plugin<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let val_2 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeMetadata",
            "(Ljava/lang/String;Lorg/bukkit/plugin/Plugin;)V",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
}
impl<'mc> JNIRaw<'mc> for HangingSign<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Into<crate::block::Sign<'mc>> for HangingSign<'mc> {
    fn into(self) -> crate::block::Sign<'mc> {
        crate::block::Sign::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
/// Represents a block state that also hosts a tile entity at the given location. This interface alone is merely a marker that does not provide any data. Data about the tile entities is provided by the respective interface for each tile entity type. After modifying the data provided by a TileState, <a href="BlockState.html#update()"><code>BlockState.update()</code></a> needs to be called to store the data.
///
/// This is a representation of an abstract class.
pub struct TileState<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> TileState<'mc> {
    pub fn from_raw(
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
    /// Returns a custom tag container capable of storing tags on the object. Note that the tags stored on this container are all stored under their own custom namespace therefore modifying default tags using this <a title="interface in org.bukkit.persistence" href="../persistence/PersistentDataHolder.html"><code>PersistentDataHolder</code></a> is impossible.
    /// <p>This <a href="../persistence/PersistentDataHolder.html" title="interface in org.bukkit.persistence"><code>PersistentDataHolder</code></a> is only linked to the snapshot instance stored by the <a title="interface in org.bukkit.block" href="BlockState.html"><code>BlockState</code></a>. When storing changes on the <a title="interface in org.bukkit.persistence" href="../persistence/PersistentDataHolder.html"><code>PersistentDataHolder</code></a>, the updated content will only be applied to the actual tile entity after one of the <a href="BlockState.html#update()"><code>BlockState.update()</code></a> methods is called.</p>
    pub fn persistent_data_container(
        &mut self,
    ) -> Result<crate::persistence::PersistentDataContainer<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getPersistentDataContainer",
            "()Lorg/bukkit/persistence/PersistentDataContainer;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::persistence::PersistentDataContainer::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn set_type(
        &mut self,
        arg0: impl Into<&'mc crate::Material<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setType",
            "(Lorg/bukkit/Material;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn world(&mut self) -> Result<crate::World<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getWorld", "()Lorg/bukkit/World;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        crate::World::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn data(
        &mut self,
    ) -> Result<crate::material::MaterialData<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getData",
            "()Lorg/bukkit/material/MaterialData;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::material::MaterialData::from_raw(&self.jni_ref(), unsafe {
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

    pub fn block_data(
        &mut self,
    ) -> Result<crate::block::data::BlockData<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getBlockData",
            "()Lorg/bukkit/block/data/BlockData;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::block::data::BlockData::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn light_level(&mut self) -> Result<i8, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getLightLevel", "()B", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.b().unwrap())
    }

    pub fn x(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getX", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }

    pub fn y(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getY", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }

    pub fn z(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getZ", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }

    pub fn chunk(&mut self) -> Result<crate::Chunk<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getChunk", "()Lorg/bukkit/Chunk;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        crate::Chunk::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn set_data(
        &mut self,
        arg0: impl Into<&'mc crate::material::MaterialData<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setData",
            "(Lorg/bukkit/material/MaterialData;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn set_block_data(
        &mut self,
        arg0: impl Into<&'mc crate::block::data::BlockData<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setBlockData",
            "(Lorg/bukkit/block/data/BlockData;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    #[deprecated]

    pub fn raw_data(&mut self) -> Result<i8, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getRawData", "()B", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.b().unwrap())
    }
    #[deprecated]

    pub fn set_raw_data(&mut self, arg0: i8) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Byte(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setRawData",
            "(B)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn is_placed(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isPlaced", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn update(
        &mut self,
        arg0: std::option::Option<bool>,
        arg1: std::option::Option<bool>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        // -1
        let val_1 = jni::objects::JValueGen::Bool(arg0.unwrap().into());
        // -1
        let val_2 = jni::objects::JValueGen::Bool(arg1.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "update",
            "(ZZ)Z",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn location(&mut self) -> Result<crate::Location<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getLocation",
            "()Lorg/bukkit/Location;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::Location::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn get_type(&mut self) -> Result<crate::Material<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getType",
            "()Lorg/bukkit/Material;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant =
            self.jni_ref()
                .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
        let variant_str = self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::Material::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::Material::from_string(variant_str).unwrap(),
        )
    }

    pub fn set_metadata(
        &mut self,
        arg0: impl Into<&'mc String>,
        arg1: impl Into<&'mc crate::metadata::MetadataValue<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let val_2 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setMetadata",
            "(Ljava/lang/String;Lorg/bukkit/metadata/MetadataValue;)V",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn get_metadata(
        &mut self,
        arg0: impl Into<&'mc String>,
    ) -> Result<Vec<crate::metadata::MetadataValue<'mc>>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getMetadata",
            "(Ljava/lang/String;)Ljava/util/List;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let mut list = blackboxmc_java::JavaList::from_raw(&self.jni_ref(), res.l()?)?;
        let size = list.size()?;
        for i in 0..=size {
            let obj = list.get(i)?;
            new_vec.push(crate::metadata::MetadataValue::from_raw(
                &self.jni_ref(),
                obj,
            )?);
        }
        Ok(new_vec)
    }

    pub fn has_metadata(
        &mut self,
        arg0: impl Into<&'mc String>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "hasMetadata",
            "(Ljava/lang/String;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn remove_metadata(
        &mut self,
        arg0: impl Into<&'mc String>,
        arg1: impl Into<&'mc crate::plugin::Plugin<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let val_2 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeMetadata",
            "(Ljava/lang/String;Lorg/bukkit/plugin/Plugin;)V",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
}
impl<'mc> JNIRaw<'mc> for TileState<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Into<crate::block::BlockState<'mc>> for TileState<'mc> {
    fn into(self) -> crate::block::BlockState<'mc> {
        crate::block::BlockState::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
impl<'mc> Into<crate::persistence::PersistentDataHolder<'mc>> for TileState<'mc> {
    fn into(self) -> crate::persistence::PersistentDataHolder<'mc> {
        crate::persistence::PersistentDataHolder::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
/// Represents a captured state of a block, which will not change automatically.
/// <p>Unlike Block, which only one object can exist per coordinate, BlockState can exist multiple times for any given Block. Note that another plugin may change the state of the block and you will not know, or they may change the block to another type entirely, causing your BlockState to become invalid.</p>
///
/// This is a representation of an abstract class.
pub struct BlockState<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> BlockState<'mc> {
    pub fn from_raw(
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
    /// Sets the type of this block state.
    pub fn set_type(
        &mut self,
        arg0: impl Into<&'mc crate::Material<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setType",
            "(Lorg/bukkit/Material;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Gets the world which contains the block represented by this block state.
    pub fn world(&mut self) -> Result<crate::World<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getWorld", "()Lorg/bukkit/World;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        crate::World::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Gets the metadata for this block state.
    pub fn data(
        &mut self,
    ) -> Result<crate::material::MaterialData<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getData",
            "()Lorg/bukkit/material/MaterialData;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::material::MaterialData::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Gets the block represented by this block state.
    /// Gets the data for this block state.
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
    /// Gets the data for this block state.
    pub fn block_data(
        &mut self,
    ) -> Result<crate::block::data::BlockData<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getBlockData",
            "()Lorg/bukkit/block/data/BlockData;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::block::data::BlockData::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Gets the current light level of the block represented by this block state.
    pub fn light_level(&mut self) -> Result<i8, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getLightLevel", "()B", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.b().unwrap())
    }
    /// Gets the x-coordinate of this block state.
    pub fn x(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getX", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }
    /// Gets the y-coordinate of this block state.
    pub fn y(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getY", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }
    /// Gets the z-coordinate of this block state.
    pub fn z(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getZ", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }
    /// Gets the chunk which contains the block represented by this block state.
    pub fn chunk(&mut self) -> Result<crate::Chunk<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getChunk", "()Lorg/bukkit/Chunk;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        crate::Chunk::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Sets the metadata for this block state.
    pub fn set_data(
        &mut self,
        arg0: impl Into<&'mc crate::material::MaterialData<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setData",
            "(Lorg/bukkit/material/MaterialData;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Sets the data for this block state.
    pub fn set_block_data(
        &mut self,
        arg0: impl Into<&'mc crate::block::data::BlockData<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setBlockData",
            "(Lorg/bukkit/block/data/BlockData;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    #[deprecated]
    /// <span class="deprecated-label">Deprecated.</span>
    /// <div class="deprecation-comment">
    /// Magic value
    /// </div>
    /// Magic value
    ///
    pub fn raw_data(&mut self) -> Result<i8, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getRawData", "()B", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.b().unwrap())
    }
    #[deprecated]
    /// <span class="deprecated-label">Deprecated.</span>
    /// <div class="deprecation-comment">
    /// Magic value
    /// </div>
    /// Magic value
    ///
    pub fn set_raw_data(&mut self, arg0: i8) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Byte(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setRawData",
            "(B)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Returns whether this state is placed in the world.
    /// <p>Some methods will not work if the block state isn't placed in the world.</p>
    pub fn is_placed(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isPlaced", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    /// Attempts to update the block represented by this state, setting it to the new values as defined by this state.
    /// <p>This has the same effect as calling update(false). That is to say, this will not modify the state of a block if it is no longer the same type as it was when this state was taken. It will return false in this eventuality.</p>
    /// Attempts to update the block represented by this state, setting it to the new values as defined by this state.
    /// <p>This has the same effect as calling update(force, true). That is to say, this will trigger a physics update to surrounding blocks.</p>
    /// Attempts to update the block represented by this state, setting it to the new values as defined by this state.
    /// <p>If this state is not placed, this will have no effect and return true.</p>
    /// <p>Unless force is true, this will not modify the state of a block if it is no longer the same type as it was when this state was taken. It will return false in this eventuality.</p>
    /// <p>If force is true, it will set the type of the block to match the new state, set the state data and then return true.</p>
    /// <p>If applyPhysics is true, it will trigger a physics update on surrounding blocks which could cause them to update or disappear.</p>
    pub fn update(
        &mut self,
        arg0: std::option::Option<bool>,
        arg1: std::option::Option<bool>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        // -1
        let val_1 = jni::objects::JValueGen::Bool(arg0.unwrap().into());
        // -1
        let val_2 = jni::objects::JValueGen::Bool(arg1.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "update",
            "(ZZ)Z",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    /// Gets the location of this block state.
    /// <p>If this block state is not placed the location's world will be null!</p>
    /// Stores the location of this block state in the provided Location object.
    /// <p>If the provided Location is null this method does nothing and returns null.</p>
    /// <p>If this block state is not placed the location's world will be null!</p>
    pub fn location(&mut self) -> Result<crate::Location<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getLocation",
            "()Lorg/bukkit/Location;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::Location::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Gets the type of this block state.
    pub fn get_type(&mut self) -> Result<crate::Material<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getType",
            "()Lorg/bukkit/Material;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant =
            self.jni_ref()
                .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
        let variant_str = self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::Material::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::Material::from_string(variant_str).unwrap(),
        )
    }

    pub fn set_metadata(
        &mut self,
        arg0: impl Into<&'mc String>,
        arg1: impl Into<&'mc crate::metadata::MetadataValue<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let val_2 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setMetadata",
            "(Ljava/lang/String;Lorg/bukkit/metadata/MetadataValue;)V",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn get_metadata(
        &mut self,
        arg0: impl Into<&'mc String>,
    ) -> Result<Vec<crate::metadata::MetadataValue<'mc>>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getMetadata",
            "(Ljava/lang/String;)Ljava/util/List;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let mut list = blackboxmc_java::JavaList::from_raw(&self.jni_ref(), res.l()?)?;
        let size = list.size()?;
        for i in 0..=size {
            let obj = list.get(i)?;
            new_vec.push(crate::metadata::MetadataValue::from_raw(
                &self.jni_ref(),
                obj,
            )?);
        }
        Ok(new_vec)
    }

    pub fn has_metadata(
        &mut self,
        arg0: impl Into<&'mc String>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "hasMetadata",
            "(Ljava/lang/String;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn remove_metadata(
        &mut self,
        arg0: impl Into<&'mc String>,
        arg1: impl Into<&'mc crate::plugin::Plugin<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let val_2 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeMetadata",
            "(Ljava/lang/String;Lorg/bukkit/plugin/Plugin;)V",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
}
impl<'mc> JNIRaw<'mc> for BlockState<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Into<crate::metadata::Metadatable<'mc>> for BlockState<'mc> {
    fn into(self) -> crate::metadata::Metadatable<'mc> {
        crate::metadata::Metadatable::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
/// Represents a captured state of a brewing stand.
///
/// This is a representation of an abstract class.
pub struct BrewingStand<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> BrewingStand<'mc> {
    pub fn from_raw(
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
    /// <span class="descfrm-type-label">Description copied from interface:&nbsp;<code><a href="Container.html#getInventory()">Container</a></code></span>
    /// Gets the inventory of the block represented by this block state.
    /// <p>If the block was changed to a different type in the meantime, the returned inventory might no longer be valid.</p>
    /// <p>If this block state is not placed this will return the captured inventory snapshot instead.</p>
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
    /// <span class="descfrm-type-label">Description copied from interface:&nbsp;<code><a href="Container.html#getSnapshotInventory()">Container</a></code></span>
    /// Gets the captured inventory snapshot of this container.
    /// <p>The returned inventory is not linked to any block. Any modifications to the returned inventory will not be applied to the block represented by this block state up until <a href="BlockState.html#update(boolean,boolean)"><code>BlockState.update(boolean, boolean)</code></a> has been called.</p>
    pub fn snapshot_inventory(
        &mut self,
    ) -> Result<crate::inventory::BrewerInventory<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getSnapshotInventory",
            "()Lorg/bukkit/inventory/BrewerInventory;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::BrewerInventory::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// How much time is left in the brewing cycle.
    pub fn brewing_time(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getBrewingTime", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }
    /// Set the time left before brewing completes.
    pub fn set_brewing_time(&mut self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setBrewingTime",
            "(I)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Get the level of current fuel for brewing.
    pub fn fuel_level(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getFuelLevel", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }
    /// Set the level of current fuel for brewing.
    pub fn set_fuel_level(&mut self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setFuelLevel",
            "(I)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn persistent_data_container(
        &mut self,
    ) -> Result<crate::persistence::PersistentDataContainer<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getPersistentDataContainer",
            "()Lorg/bukkit/persistence/PersistentDataContainer;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::persistence::PersistentDataContainer::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn set_type(
        &mut self,
        arg0: impl Into<&'mc crate::Material<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setType",
            "(Lorg/bukkit/Material;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn world(&mut self) -> Result<crate::World<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getWorld", "()Lorg/bukkit/World;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        crate::World::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn data(
        &mut self,
    ) -> Result<crate::material::MaterialData<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getData",
            "()Lorg/bukkit/material/MaterialData;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::material::MaterialData::from_raw(&self.jni_ref(), unsafe {
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

    pub fn block_data(
        &mut self,
    ) -> Result<crate::block::data::BlockData<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getBlockData",
            "()Lorg/bukkit/block/data/BlockData;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::block::data::BlockData::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn light_level(&mut self) -> Result<i8, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getLightLevel", "()B", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.b().unwrap())
    }

    pub fn x(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getX", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }

    pub fn y(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getY", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }

    pub fn z(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getZ", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }

    pub fn chunk(&mut self) -> Result<crate::Chunk<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getChunk", "()Lorg/bukkit/Chunk;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        crate::Chunk::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn set_data(
        &mut self,
        arg0: impl Into<&'mc crate::material::MaterialData<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setData",
            "(Lorg/bukkit/material/MaterialData;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn set_block_data(
        &mut self,
        arg0: impl Into<&'mc crate::block::data::BlockData<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setBlockData",
            "(Lorg/bukkit/block/data/BlockData;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    #[deprecated]

    pub fn raw_data(&mut self) -> Result<i8, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getRawData", "()B", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.b().unwrap())
    }
    #[deprecated]

    pub fn set_raw_data(&mut self, arg0: i8) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Byte(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setRawData",
            "(B)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn is_placed(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isPlaced", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn update(
        &mut self,
        arg0: std::option::Option<bool>,
        arg1: std::option::Option<bool>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        // -1
        let val_1 = jni::objects::JValueGen::Bool(arg0.unwrap().into());
        // -1
        let val_2 = jni::objects::JValueGen::Bool(arg1.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "update",
            "(ZZ)Z",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn location(&mut self) -> Result<crate::Location<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getLocation",
            "()Lorg/bukkit/Location;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::Location::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn get_type(&mut self) -> Result<crate::Material<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getType",
            "()Lorg/bukkit/Material;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant =
            self.jni_ref()
                .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
        let variant_str = self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::Material::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::Material::from_string(variant_str).unwrap(),
        )
    }

    pub fn set_metadata(
        &mut self,
        arg0: impl Into<&'mc String>,
        arg1: impl Into<&'mc crate::metadata::MetadataValue<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let val_2 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setMetadata",
            "(Ljava/lang/String;Lorg/bukkit/metadata/MetadataValue;)V",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn get_metadata(
        &mut self,
        arg0: impl Into<&'mc String>,
    ) -> Result<Vec<crate::metadata::MetadataValue<'mc>>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getMetadata",
            "(Ljava/lang/String;)Ljava/util/List;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let mut list = blackboxmc_java::JavaList::from_raw(&self.jni_ref(), res.l()?)?;
        let size = list.size()?;
        for i in 0..=size {
            let obj = list.get(i)?;
            new_vec.push(crate::metadata::MetadataValue::from_raw(
                &self.jni_ref(),
                obj,
            )?);
        }
        Ok(new_vec)
    }

    pub fn has_metadata(
        &mut self,
        arg0: impl Into<&'mc String>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "hasMetadata",
            "(Ljava/lang/String;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn remove_metadata(
        &mut self,
        arg0: impl Into<&'mc String>,
        arg1: impl Into<&'mc crate::plugin::Plugin<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let val_2 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeMetadata",
            "(Ljava/lang/String;Lorg/bukkit/plugin/Plugin;)V",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn is_locked(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isLocked", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn lock(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getLock", "()Ljava/lang/String;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }

    pub fn set_lock(
        &mut self,
        arg0: impl Into<&'mc String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setLock",
            "(Ljava/lang/String;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn custom_name(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getCustomName",
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

    pub fn set_custom_name(
        &mut self,
        arg0: impl Into<&'mc String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setCustomName",
            "(Ljava/lang/String;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
}
impl<'mc> JNIRaw<'mc> for BrewingStand<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Into<crate::block::Container<'mc>> for BrewingStand<'mc> {
    fn into(self) -> crate::block::Container<'mc> {
        crate::block::Container::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
/// Represents a captured state of a skull block.
///
/// This is a representation of an abstract class.
pub struct Skull<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> Skull<'mc> {
    pub fn from_raw(
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
    #[deprecated]
    /// <span class="deprecated-label">Deprecated.</span>
    /// <div class="deprecation-comment">
    /// See <a href="#getOwningPlayer()"><code>getOwningPlayer()</code></a>.
    /// </div>
    /// See <a href="#getOwningPlayer()"><code>getOwningPlayer()</code></a>.
    ///
    /// Gets the owner of the skull, if one exists
    /// Gets the profile of the player who owns the skull. This player profile may appear as the texture depending on skull type.
    pub fn owner(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getOwner", "()Ljava/lang/String;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    #[deprecated]
    /// <span class="deprecated-label">Deprecated.</span>
    /// <div class="deprecation-comment">
    /// see <a href="#setOwningPlayer(org.bukkit.OfflinePlayer)"><code>setOwningPlayer(org.bukkit.OfflinePlayer)</code></a>.
    /// </div>
    /// see <a href="#setOwningPlayer(org.bukkit.OfflinePlayer)"><code>setOwningPlayer(org.bukkit.OfflinePlayer)</code></a>.
    ///
    /// Sets the owner of the skull
    /// <p>Involves a potentially blocking web request to acquire the profile data for the provided name.</p>
    /// Sets the profile of the player who owns the skull. This player profile may appear as the texture depending on skull type.
    /// <p>The profile must contain both a unique id and a skin texture. If either of these is missing, the profile must contain a name by which the server will then attempt to look up the unique id and skin texture.</p>
    pub fn set_owner(
        &mut self,
        arg0: impl Into<&'mc String>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setOwner",
            "(Ljava/lang/String;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    #[deprecated]
    /// <span class="deprecated-label">Deprecated.</span>
    /// <div class="deprecation-comment">
    /// use <a href="data/BlockData.html" title="interface in org.bukkit.block.data"><code>BlockData</code></a>
    /// </div>
    /// use <a href="data/BlockData.html" title="interface in org.bukkit.block.data"><code>BlockData</code></a>
    ///
    /// Gets the rotation of the skull in the world (or facing direction if this is a wall mounted skull).
    pub fn rotation(&mut self) -> Result<crate::block::BlockFace<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getRotation",
            "()Lorg/bukkit/block/BlockFace;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant =
            self.jni_ref()
                .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
        let variant_str = self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::block::BlockFace::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::block::BlockFace::from_string(variant_str).unwrap(),
        )
    }
    #[deprecated]
    /// <span class="deprecated-label">Deprecated.</span>
    /// <div class="deprecation-comment">
    /// use <a href="data/BlockData.html" title="interface in org.bukkit.block.data"><code>BlockData</code></a>
    /// </div>
    /// use <a href="data/BlockData.html" title="interface in org.bukkit.block.data"><code>BlockData</code></a>
    ///
    /// Sets the rotation of the skull in the world (or facing direction if this is a wall mounted skull).
    pub fn set_rotation(
        &mut self,
        arg0: impl Into<&'mc crate::block::BlockFace<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setRotation",
            "(Lorg/bukkit/block/BlockFace;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Get the player which owns the skull. This player may appear as the texture depending on skull type.
    pub fn owning_player(
        &mut self,
    ) -> Result<crate::OfflinePlayer<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getOwningPlayer",
            "()Lorg/bukkit/OfflinePlayer;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::OfflinePlayer::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Set the player which owns the skull. This player may appear as the texture depending on skull type.
    pub fn set_owning_player(
        &mut self,
        arg0: impl Into<&'mc crate::OfflinePlayer<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setOwningPlayer",
            "(Lorg/bukkit/OfflinePlayer;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Checks to see if the skull has an owner
    pub fn has_owner(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasOwner", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    /// Gets the profile of the player who owns the skull. This player profile may appear as the texture depending on skull type.
    pub fn owner_profile(
        &mut self,
    ) -> Result<crate::profile::PlayerProfile<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getOwnerProfile",
            "()Lorg/bukkit/profile/PlayerProfile;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::profile::PlayerProfile::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Sets the profile of the player who owns the skull. This player profile may appear as the texture depending on skull type.
    /// <p>The profile must contain both a unique id and a skin texture. If either of these is missing, the profile must contain a name by which the server will then attempt to look up the unique id and skin texture.</p>
    pub fn set_owner_profile(
        &mut self,
        arg0: impl Into<&'mc crate::profile::PlayerProfile<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setOwnerProfile",
            "(Lorg/bukkit/profile/PlayerProfile;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Gets the sound to play if the skull is placed on a note block.
    ///
    /// <strong>Note:</strong> This only works for player heads. For other heads, see <a title="enum in org.bukkit" href="../Instrument.html"><code>Instrument</code></a>.
    pub fn note_block_sound(
        &mut self,
    ) -> Result<crate::NamespacedKey<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getNoteBlockSound",
            "()Lorg/bukkit/NamespacedKey;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::NamespacedKey::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Sets the sound to play if the skull is placed on a note block.
    ///
    /// <strong>Note:</strong> This only works for player heads. For other heads, see <a href="../Instrument.html" title="enum in org.bukkit"><code>Instrument</code></a>.
    pub fn set_note_block_sound(
        &mut self,
        arg0: impl Into<&'mc crate::NamespacedKey<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setNoteBlockSound",
            "(Lorg/bukkit/NamespacedKey;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    #[deprecated]
    /// <span class="deprecated-label">Deprecated.</span>
    /// <div class="deprecation-comment">
    /// check <a href="../Material.html" title="enum in org.bukkit"><code>Material</code></a> instead
    /// </div>
    /// check <a href="../Material.html" title="enum in org.bukkit"><code>Material</code></a> instead
    ///
    /// Gets the type of skull
    pub fn skull_type(&mut self) -> Result<crate::SkullType<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getSkullType",
            "()Lorg/bukkit/SkullType;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant =
            self.jni_ref()
                .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
        let variant_str = self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::SkullType::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::SkullType::from_string(variant_str).unwrap(),
        )
    }
    #[deprecated]
    /// <span class="deprecated-label">Deprecated.</span>
    /// <div class="deprecation-comment">
    /// check <a title="enum in org.bukkit" href="../Material.html"><code>Material</code></a> instead
    /// </div>
    /// check <a title="enum in org.bukkit" href="../Material.html"><code>Material</code></a> instead
    ///
    /// Sets the type of skull
    pub fn set_skull_type(
        &mut self,
        arg0: impl Into<&'mc crate::SkullType<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setSkullType",
            "(Lorg/bukkit/SkullType;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn persistent_data_container(
        &mut self,
    ) -> Result<crate::persistence::PersistentDataContainer<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getPersistentDataContainer",
            "()Lorg/bukkit/persistence/PersistentDataContainer;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::persistence::PersistentDataContainer::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn set_type(
        &mut self,
        arg0: impl Into<&'mc crate::Material<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setType",
            "(Lorg/bukkit/Material;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn world(&mut self) -> Result<crate::World<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getWorld", "()Lorg/bukkit/World;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        crate::World::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn data(
        &mut self,
    ) -> Result<crate::material::MaterialData<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getData",
            "()Lorg/bukkit/material/MaterialData;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::material::MaterialData::from_raw(&self.jni_ref(), unsafe {
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

    pub fn block_data(
        &mut self,
    ) -> Result<crate::block::data::BlockData<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getBlockData",
            "()Lorg/bukkit/block/data/BlockData;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::block::data::BlockData::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn light_level(&mut self) -> Result<i8, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getLightLevel", "()B", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.b().unwrap())
    }

    pub fn x(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getX", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }

    pub fn y(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getY", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }

    pub fn z(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getZ", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }

    pub fn chunk(&mut self) -> Result<crate::Chunk<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getChunk", "()Lorg/bukkit/Chunk;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        crate::Chunk::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn set_data(
        &mut self,
        arg0: impl Into<&'mc crate::material::MaterialData<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setData",
            "(Lorg/bukkit/material/MaterialData;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn set_block_data(
        &mut self,
        arg0: impl Into<&'mc crate::block::data::BlockData<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setBlockData",
            "(Lorg/bukkit/block/data/BlockData;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    #[deprecated]

    pub fn raw_data(&mut self) -> Result<i8, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getRawData", "()B", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.b().unwrap())
    }
    #[deprecated]

    pub fn set_raw_data(&mut self, arg0: i8) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Byte(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setRawData",
            "(B)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn is_placed(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isPlaced", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn update(
        &mut self,
        arg0: std::option::Option<bool>,
        arg1: std::option::Option<bool>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        // -1
        let val_1 = jni::objects::JValueGen::Bool(arg0.unwrap().into());
        // -1
        let val_2 = jni::objects::JValueGen::Bool(arg1.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "update",
            "(ZZ)Z",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn location(&mut self) -> Result<crate::Location<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getLocation",
            "()Lorg/bukkit/Location;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::Location::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn get_type(&mut self) -> Result<crate::Material<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getType",
            "()Lorg/bukkit/Material;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant =
            self.jni_ref()
                .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
        let variant_str = self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::Material::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::Material::from_string(variant_str).unwrap(),
        )
    }

    pub fn set_metadata(
        &mut self,
        arg0: impl Into<&'mc String>,
        arg1: impl Into<&'mc crate::metadata::MetadataValue<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let val_2 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setMetadata",
            "(Ljava/lang/String;Lorg/bukkit/metadata/MetadataValue;)V",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn get_metadata(
        &mut self,
        arg0: impl Into<&'mc String>,
    ) -> Result<Vec<crate::metadata::MetadataValue<'mc>>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getMetadata",
            "(Ljava/lang/String;)Ljava/util/List;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let mut list = blackboxmc_java::JavaList::from_raw(&self.jni_ref(), res.l()?)?;
        let size = list.size()?;
        for i in 0..=size {
            let obj = list.get(i)?;
            new_vec.push(crate::metadata::MetadataValue::from_raw(
                &self.jni_ref(),
                obj,
            )?);
        }
        Ok(new_vec)
    }

    pub fn has_metadata(
        &mut self,
        arg0: impl Into<&'mc String>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "hasMetadata",
            "(Ljava/lang/String;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn remove_metadata(
        &mut self,
        arg0: impl Into<&'mc String>,
        arg1: impl Into<&'mc crate::plugin::Plugin<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let val_2 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeMetadata",
            "(Ljava/lang/String;Lorg/bukkit/plugin/Plugin;)V",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
}
impl<'mc> JNIRaw<'mc> for Skull<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Into<crate::block::TileState<'mc>> for Skull<'mc> {
    fn into(self) -> crate::block::TileState<'mc> {
        crate::block::TileState::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
/// Represents a captured state of a hopper.
///
/// This is a representation of an abstract class.
pub struct Hopper<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> Hopper<'mc> {
    pub fn from_raw(
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

    pub fn snapshot_inventory(
        &mut self,
    ) -> Result<crate::inventory::Inventory<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getSnapshotInventory",
            "()Lorg/bukkit/inventory/Inventory;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::Inventory::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn persistent_data_container(
        &mut self,
    ) -> Result<crate::persistence::PersistentDataContainer<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getPersistentDataContainer",
            "()Lorg/bukkit/persistence/PersistentDataContainer;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::persistence::PersistentDataContainer::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn set_type(
        &mut self,
        arg0: impl Into<&'mc crate::Material<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setType",
            "(Lorg/bukkit/Material;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn world(&mut self) -> Result<crate::World<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getWorld", "()Lorg/bukkit/World;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        crate::World::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn data(
        &mut self,
    ) -> Result<crate::material::MaterialData<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getData",
            "()Lorg/bukkit/material/MaterialData;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::material::MaterialData::from_raw(&self.jni_ref(), unsafe {
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

    pub fn block_data(
        &mut self,
    ) -> Result<crate::block::data::BlockData<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getBlockData",
            "()Lorg/bukkit/block/data/BlockData;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::block::data::BlockData::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn light_level(&mut self) -> Result<i8, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getLightLevel", "()B", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.b().unwrap())
    }

    pub fn x(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getX", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }

    pub fn y(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getY", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }

    pub fn z(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getZ", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }

    pub fn chunk(&mut self) -> Result<crate::Chunk<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getChunk", "()Lorg/bukkit/Chunk;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        crate::Chunk::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn set_data(
        &mut self,
        arg0: impl Into<&'mc crate::material::MaterialData<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setData",
            "(Lorg/bukkit/material/MaterialData;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn set_block_data(
        &mut self,
        arg0: impl Into<&'mc crate::block::data::BlockData<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setBlockData",
            "(Lorg/bukkit/block/data/BlockData;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    #[deprecated]

    pub fn raw_data(&mut self) -> Result<i8, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getRawData", "()B", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.b().unwrap())
    }
    #[deprecated]

    pub fn set_raw_data(&mut self, arg0: i8) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Byte(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setRawData",
            "(B)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn is_placed(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isPlaced", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn update(
        &mut self,
        arg0: std::option::Option<bool>,
        arg1: std::option::Option<bool>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        // -1
        let val_1 = jni::objects::JValueGen::Bool(arg0.unwrap().into());
        // -1
        let val_2 = jni::objects::JValueGen::Bool(arg1.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "update",
            "(ZZ)Z",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn location(&mut self) -> Result<crate::Location<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getLocation",
            "()Lorg/bukkit/Location;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::Location::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn get_type(&mut self) -> Result<crate::Material<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getType",
            "()Lorg/bukkit/Material;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant =
            self.jni_ref()
                .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
        let variant_str = self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::Material::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::Material::from_string(variant_str).unwrap(),
        )
    }

    pub fn set_metadata(
        &mut self,
        arg0: impl Into<&'mc String>,
        arg1: impl Into<&'mc crate::metadata::MetadataValue<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let val_2 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setMetadata",
            "(Ljava/lang/String;Lorg/bukkit/metadata/MetadataValue;)V",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn get_metadata(
        &mut self,
        arg0: impl Into<&'mc String>,
    ) -> Result<Vec<crate::metadata::MetadataValue<'mc>>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getMetadata",
            "(Ljava/lang/String;)Ljava/util/List;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let mut list = blackboxmc_java::JavaList::from_raw(&self.jni_ref(), res.l()?)?;
        let size = list.size()?;
        for i in 0..=size {
            let obj = list.get(i)?;
            new_vec.push(crate::metadata::MetadataValue::from_raw(
                &self.jni_ref(),
                obj,
            )?);
        }
        Ok(new_vec)
    }

    pub fn has_metadata(
        &mut self,
        arg0: impl Into<&'mc String>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "hasMetadata",
            "(Ljava/lang/String;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn remove_metadata(
        &mut self,
        arg0: impl Into<&'mc String>,
        arg1: impl Into<&'mc crate::plugin::Plugin<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let val_2 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeMetadata",
            "(Ljava/lang/String;Lorg/bukkit/plugin/Plugin;)V",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn is_locked(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isLocked", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn lock(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getLock", "()Ljava/lang/String;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }

    pub fn set_lock(
        &mut self,
        arg0: impl Into<&'mc String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setLock",
            "(Ljava/lang/String;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn custom_name(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getCustomName",
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

    pub fn set_custom_name(
        &mut self,
        arg0: impl Into<&'mc String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setCustomName",
            "(Ljava/lang/String;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn seed(&mut self) -> Result<i64, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getSeed", "()J", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.j().unwrap())
    }

    pub fn set_seed(&mut self, arg0: i64) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Long(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setSeed",
            "(J)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn set_loot_table(
        &mut self,
        arg0: impl Into<&'mc crate::loot::LootTable<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setLootTable",
            "(Lorg/bukkit/loot/LootTable;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn loot_table(
        &mut self,
    ) -> Result<crate::loot::LootTable<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getLootTable",
            "()Lorg/bukkit/loot/LootTable;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::loot::LootTable::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
}
impl<'mc> JNIRaw<'mc> for Hopper<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Into<crate::block::Container<'mc>> for Hopper<'mc> {
    fn into(self) -> crate::block::Container<'mc> {
        crate::block::Container::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
impl<'mc> Into<crate::loot::Lootable<'mc>> for Hopper<'mc> {
    fn into(self) -> crate::loot::Lootable<'mc> {
        crate::loot::Lootable::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
/// Represents a captured state of either a SignPost or a WallSign.
///
/// This is a representation of an abstract class.
pub struct Sign<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> Sign<'mc> {
    pub fn from_raw(
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
    #[deprecated]
    /// <span class="deprecated-label">Deprecated.</span>
    /// <div class="deprecation-comment">
    /// A sign may have multiple writable sides now. Use <a href="#getSide(org.bukkit.block.sign.Side)"><code>getSide(Side)</code></a> and <a href="../material/Colorable.html#setColor(org.bukkit.DyeColor)"><code>Colorable.setColor(org.bukkit.DyeColor)</code></a>.
    /// </div>
    /// A sign may have multiple writable sides now. Use <a href="#getSide(org.bukkit.block.sign.Side)"><code>getSide(Side)</code></a> and <a href="../material/Colorable.html#setColor(org.bukkit.DyeColor)"><code>Colorable.setColor(org.bukkit.DyeColor)</code></a>.
    ///
    /// Sets the color of this object to the specified DyeColor.
    ///
    /// This may be null to represent the default color of an object, if the object has a special default color (e.g Shulkers).
    pub fn set_color(
        &mut self,
        arg0: impl Into<&'mc crate::DyeColor<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setColor",
            "(Lorg/bukkit/DyeColor;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    #[deprecated]
    /// <span class="deprecated-label">Deprecated.</span>
    /// <div class="deprecation-comment">
    /// A sign may have multiple writable sides now. Use <a href="#getSide(org.bukkit.block.sign.Side)"><code>getSide(Side)</code></a> and <a href="sign/SignSide.html#isGlowingText()"><code>SignSide.isGlowingText()</code></a>.
    /// </div>
    /// A sign may have multiple writable sides now. Use <a href="#getSide(org.bukkit.block.sign.Side)"><code>getSide(Side)</code></a> and <a href="sign/SignSide.html#isGlowingText()"><code>SignSide.isGlowingText()</code></a>.
    ///
    /// Gets whether this sign has glowing text. Only affects the <a href="sign/Side.html#FRONT"><code>Side.FRONT</code></a>.
    pub fn is_glowing_text(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isGlowingText", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    #[deprecated]
    /// <span class="deprecated-label">Deprecated.</span>
    /// <div class="deprecation-comment">
    /// A sign may have multiple writable sides now. Use <a href="#getSide(org.bukkit.block.sign.Side)"><code>getSide(Side)</code></a> and <a href="sign/SignSide.html#getLines()"><code>SignSide.getLines()</code></a>.
    /// </div>
    /// A sign may have multiple writable sides now. Use <a href="#getSide(org.bukkit.block.sign.Side)"><code>getSide(Side)</code></a> and <a href="sign/SignSide.html#getLines()"><code>SignSide.getLines()</code></a>.
    ///
    /// Gets all the lines of text currently on the <a href="sign/Side.html#FRONT"><code>Side.FRONT</code></a> of this sign.
    /// <span class="deprecated-label">Deprecated.</span>
    /// <div class="deprecation-comment">
    /// A sign may have multiple writable sides now. Use <a href="#getSide(org.bukkit.block.sign.Side)"><code>getSide(Side)</code></a> and <a href="sign/SignSide.html#getLine(int)"><code>SignSide.getLine(int)</code></a>.
    /// </div>
    /// A sign may have multiple writable sides now. Use <a href="#getSide(org.bukkit.block.sign.Side)"><code>getSide(Side)</code></a> and <a href="sign/SignSide.html#getLine(int)"><code>SignSide.getLine(int)</code></a>.
    ///
    /// Gets the line of text at the specified index.
    /// <p>For example, getLine(0) will return the first line of text on the <a href="sign/Side.html#FRONT"><code>Side.FRONT</code></a>.</p>
    pub fn get_line(&mut self, arg0: i32) -> Result<String, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getLine",
            "(I)Ljava/lang/String;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    #[deprecated]
    /// <span class="deprecated-label">Deprecated.</span>
    /// <div class="deprecation-comment">
    /// A sign may have multiple writable sides now. Use <a href="#getSide(org.bukkit.block.sign.Side)"><code>getSide(Side)</code></a> and <a href="sign/SignSide.html#setLine(int,java.lang.String)"><code>SignSide.setLine(int, String)</code></a>.
    /// </div>
    /// A sign may have multiple writable sides now. Use <a href="#getSide(org.bukkit.block.sign.Side)"><code>getSide(Side)</code></a> and <a href="sign/SignSide.html#setLine(int,java.lang.String)"><code>SignSide.setLine(int, String)</code></a>.
    ///
    /// Sets the line of text at the specified index.
    /// <p>For example, setLine(0, "Line One") will set the first line of text to "Line One".</p>
    pub fn set_line(
        &mut self,
        arg0: i32,
        arg1: impl Into<&'mc String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let val_2 = jni::objects::JObject::from(self.jni_ref().new_string(arg1.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setLine",
            "(ILjava/lang/String;)V",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    #[deprecated]
    /// <span class="deprecated-label">Deprecated.</span>
    /// <div class="deprecation-comment">
    /// use <a href="#isWaxed()"><code>isWaxed()</code></a> instead
    /// </div>
    /// use <a href="#isWaxed()"><code>isWaxed()</code></a> instead
    ///
    /// Marks whether this sign can be edited by players.
    pub fn is_editable(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isEditable", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    #[deprecated]
    /// <span class="deprecated-label">Deprecated.</span>
    /// <div class="deprecation-comment">
    /// use <a href="#setWaxed(boolean)"><code>setWaxed(boolean)</code></a> instead
    /// </div>
    /// use <a href="#setWaxed(boolean)"><code>setWaxed(boolean)</code></a> instead
    ///
    /// Marks whether this sign can be edited by players.
    pub fn set_editable(&mut self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        // -2
        let val_1 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setEditable",
            "(Z)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Gets whether or not this sign has been waxed. If a sign has been waxed, it cannot be edited by a player.
    pub fn is_waxed(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isWaxed", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    /// Sets whether or not this sign has been waxed. If a sign has been waxed, it cannot be edited by a player.
    pub fn set_waxed(&mut self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        // -2
        let val_1 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setWaxed",
            "(Z)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    #[deprecated]
    /// <span class="deprecated-label">Deprecated.</span>
    /// <div class="deprecation-comment">
    /// A sign may have multiple writable sides now. Use <a href="#getSide(org.bukkit.block.sign.Side)"><code>getSide(Side)</code></a> and <a href="sign/SignSide.html#setGlowingText(boolean)"><code>SignSide.setGlowingText(boolean)</code></a>.
    /// </div>
    /// A sign may have multiple writable sides now. Use <a href="#getSide(org.bukkit.block.sign.Side)"><code>getSide(Side)</code></a> and <a href="sign/SignSide.html#setGlowingText(boolean)"><code>SignSide.setGlowingText(boolean)</code></a>.
    ///
    /// Sets whether this sign has glowing text. Only affects the <a href="sign/Side.html#FRONT"><code>Side.FRONT</code></a>.
    pub fn set_glowing_text(&mut self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        // -2
        let val_1 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setGlowingText",
            "(Z)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    #[deprecated]
    /// <span class="deprecated-label">Deprecated.</span>
    /// <div class="deprecation-comment">
    /// A sign may have multiple writable sides now. Use <a href="#getSide(org.bukkit.block.sign.Side)"><code>getSide(Side)</code></a> and <a href="../material/Colorable.html#getColor()"><code>Colorable.getColor()</code></a>.
    /// </div>
    /// A sign may have multiple writable sides now. Use <a href="#getSide(org.bukkit.block.sign.Side)"><code>getSide(Side)</code></a> and <a href="../material/Colorable.html#getColor()"><code>Colorable.getColor()</code></a>.
    ///
    /// Gets the color of this object.
    ///
    /// This may be null to represent the default color of an object, if the object has a special default color (e.g Shulkers).
    pub fn color(&mut self) -> Result<crate::DyeColor<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getColor",
            "()Lorg/bukkit/DyeColor;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant =
            self.jni_ref()
                .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
        let variant_str = self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::DyeColor::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::DyeColor::from_string(variant_str).unwrap(),
        )
    }
    /// Return the side of the sign.
    pub fn get_side(
        &mut self,
        arg0: impl Into<&'mc crate::block::sign::Side<'mc>>,
    ) -> Result<crate::block::sign::SignSide<'mc>, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getSide",
            "(Lorg/bukkit/block/sign/Side;)Lorg/bukkit/block/sign/SignSide;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::block::sign::SignSide::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn persistent_data_container(
        &mut self,
    ) -> Result<crate::persistence::PersistentDataContainer<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getPersistentDataContainer",
            "()Lorg/bukkit/persistence/PersistentDataContainer;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::persistence::PersistentDataContainer::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn set_type(
        &mut self,
        arg0: impl Into<&'mc crate::Material<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setType",
            "(Lorg/bukkit/Material;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn world(&mut self) -> Result<crate::World<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getWorld", "()Lorg/bukkit/World;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        crate::World::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn data(
        &mut self,
    ) -> Result<crate::material::MaterialData<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getData",
            "()Lorg/bukkit/material/MaterialData;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::material::MaterialData::from_raw(&self.jni_ref(), unsafe {
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

    pub fn block_data(
        &mut self,
    ) -> Result<crate::block::data::BlockData<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getBlockData",
            "()Lorg/bukkit/block/data/BlockData;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::block::data::BlockData::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn light_level(&mut self) -> Result<i8, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getLightLevel", "()B", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.b().unwrap())
    }

    pub fn x(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getX", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }

    pub fn y(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getY", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }

    pub fn z(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getZ", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }

    pub fn chunk(&mut self) -> Result<crate::Chunk<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getChunk", "()Lorg/bukkit/Chunk;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        crate::Chunk::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn set_data(
        &mut self,
        arg0: impl Into<&'mc crate::material::MaterialData<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setData",
            "(Lorg/bukkit/material/MaterialData;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn set_block_data(
        &mut self,
        arg0: impl Into<&'mc crate::block::data::BlockData<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setBlockData",
            "(Lorg/bukkit/block/data/BlockData;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    #[deprecated]

    pub fn raw_data(&mut self) -> Result<i8, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getRawData", "()B", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.b().unwrap())
    }
    #[deprecated]

    pub fn set_raw_data(&mut self, arg0: i8) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Byte(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setRawData",
            "(B)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn is_placed(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isPlaced", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn update(
        &mut self,
        arg0: std::option::Option<bool>,
        arg1: std::option::Option<bool>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        // -1
        let val_1 = jni::objects::JValueGen::Bool(arg0.unwrap().into());
        // -1
        let val_2 = jni::objects::JValueGen::Bool(arg1.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "update",
            "(ZZ)Z",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn location(&mut self) -> Result<crate::Location<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getLocation",
            "()Lorg/bukkit/Location;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::Location::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn get_type(&mut self) -> Result<crate::Material<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getType",
            "()Lorg/bukkit/Material;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant =
            self.jni_ref()
                .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
        let variant_str = self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::Material::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::Material::from_string(variant_str).unwrap(),
        )
    }

    pub fn set_metadata(
        &mut self,
        arg0: impl Into<&'mc String>,
        arg1: impl Into<&'mc crate::metadata::MetadataValue<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let val_2 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setMetadata",
            "(Ljava/lang/String;Lorg/bukkit/metadata/MetadataValue;)V",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn get_metadata(
        &mut self,
        arg0: impl Into<&'mc String>,
    ) -> Result<Vec<crate::metadata::MetadataValue<'mc>>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getMetadata",
            "(Ljava/lang/String;)Ljava/util/List;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let mut list = blackboxmc_java::JavaList::from_raw(&self.jni_ref(), res.l()?)?;
        let size = list.size()?;
        for i in 0..=size {
            let obj = list.get(i)?;
            new_vec.push(crate::metadata::MetadataValue::from_raw(
                &self.jni_ref(),
                obj,
            )?);
        }
        Ok(new_vec)
    }

    pub fn has_metadata(
        &mut self,
        arg0: impl Into<&'mc String>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "hasMetadata",
            "(Ljava/lang/String;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn remove_metadata(
        &mut self,
        arg0: impl Into<&'mc String>,
        arg1: impl Into<&'mc crate::plugin::Plugin<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let val_2 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeMetadata",
            "(Ljava/lang/String;Lorg/bukkit/plugin/Plugin;)V",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
}
impl<'mc> JNIRaw<'mc> for Sign<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Into<crate::block::TileState<'mc>> for Sign<'mc> {
    fn into(self) -> crate::block::TileState<'mc> {
        crate::block::TileState::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
impl<'mc> Into<crate::material::Colorable<'mc>> for Sign<'mc> {
    fn into(self) -> crate::material::Colorable<'mc> {
        crate::material::Colorable::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
/// Represents a double chest.
pub struct DoubleChest<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for DoubleChest<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> DoubleChest<'mc> {
    pub fn from_raw(
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
    pub fn new(
        jni: blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<&'mc crate::inventory::DoubleChestInventory<'mc>>,
    ) -> Result<crate::block::DoubleChest<'mc>, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let cls = &jni.find_class("org/bukkit/block/DoubleChest")?;
        let res = jni.new_object(
            cls,
            "(Lorg/bukkit/inventory/DoubleChestInventory;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        )?;
        crate::block::DoubleChest::from_raw(&jni, res)
    }

    pub fn world(&mut self) -> Result<crate::World<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getWorld", "()Lorg/bukkit/World;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        crate::World::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn x(&mut self) -> Result<f64, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getX", "()D", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.d().unwrap())
    }

    pub fn y(&mut self) -> Result<f64, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getY", "()D", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.d().unwrap())
    }

    pub fn z(&mut self) -> Result<f64, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getZ", "()D", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.d().unwrap())
    }
    /// <span class="descfrm-type-label">Description copied from interface:&nbsp;<code><a href="../inventory/InventoryHolder.html#getInventory()">InventoryHolder</a></code></span>
    /// Get the object's inventory.
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

    pub fn left_side(
        &mut self,
    ) -> Result<crate::inventory::InventoryHolder<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getLeftSide",
            "()Lorg/bukkit/inventory/InventoryHolder;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::InventoryHolder::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn right_side(
        &mut self,
    ) -> Result<crate::inventory::InventoryHolder<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getRightSide",
            "()Lorg/bukkit/inventory/InventoryHolder;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::InventoryHolder::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn location(&mut self) -> Result<crate::Location<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getLocation",
            "()Lorg/bukkit/Location;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::Location::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
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
impl<'mc> Into<crate::inventory::InventoryHolder<'mc>> for DoubleChest<'mc> {
    fn into(self) -> crate::inventory::InventoryHolder<'mc> {
        crate::inventory::InventoryHolder::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
pub enum BlockSupportEnum {
    Full,
    Center,
    Rigid,
}
impl std::fmt::Display for BlockSupportEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            BlockSupportEnum::Full => f.write_str("FULL"),
            BlockSupportEnum::Center => f.write_str("CENTER"),
            BlockSupportEnum::Rigid => f.write_str("RIGID"),
        }
    }
}
pub struct BlockSupport<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
    pub BlockSupportEnum,
);
impl<'mc> std::ops::Deref for BlockSupport<'mc> {
    type Target = BlockSupportEnum;
    fn deref(&self) -> &Self::Target {
        return &self.2;
    }
}
impl<'mc> JNIRaw<'mc> for BlockSupport<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> BlockSupport<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
        e: BlockSupportEnum,
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
            Ok(Self(env.clone(), obj, e))
        }
    }
    pub const FULL: BlockSupportEnum = BlockSupportEnum::Full;
    pub const CENTER: BlockSupportEnum = BlockSupportEnum::Center;
    pub const RIGID: BlockSupportEnum = BlockSupportEnum::Rigid;
    pub fn from_string(str: String) -> std::option::Option<BlockSupportEnum> {
        match str.as_str() {
            "FULL" => Some(BlockSupportEnum::Full),
            "CENTER" => Some(BlockSupportEnum::Center),
            "RIGID" => Some(BlockSupportEnum::Rigid),
            _ => None,
        }
    }
}
pub mod banner;
pub mod data;
pub mod sign;
pub mod structure;
