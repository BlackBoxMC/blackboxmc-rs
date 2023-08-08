#![allow(deprecated)]
use blackboxmc_general::JNIRaw;
use color_eyre::eyre::Result;
/// Represent a variation of a structure. Most structures, like the ones generated with structure blocks, only have a single variant.
///
/// This is a representation of an abstract class.
pub struct Palette<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> Palette<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Palette from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/structure/Palette")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Palette object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    /// Gets a copy of the blocks this Palette is made of. The <a href="../block/BlockState.html#getLocation()"><code>positions</code></a> of the returned block states are offsets relative to the structure's position that is provided once the structure is placed into the world.
    pub fn blocks(
        &mut self,
    ) -> Result<Vec<crate::block::BlockState<'mc>>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getBlocks", "()Ljava/util/List;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let mut list = blackboxmc_java::JavaList::from_raw(&self.jni_ref(), res.l()?)?;
        let size = list.size()?;
        for i in 0..=size {
            let obj = list.get(i)?;
            new_vec.push(crate::block::BlockState::from_raw(&self.jni_ref(), obj)?);
        }
        Ok(new_vec)
    }
    /// Gets the number of blocks stored in this palette.
    pub fn block_count(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getBlockCount", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }
}
impl<'mc> JNIRaw<'mc> for Palette<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}

///
/// This is a representation of an abstract class.
pub struct StructureManager<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> StructureManager<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate StructureManager from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/structure/StructureManager")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a StructureManager object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    /// Gets the currently registered structures.
    /// <p>These are the currently loaded structures that the StructureManager is aware of. When a structure block refers to a structure, these structures are checked first. If the specified structure is not found among the currently registered structures, the StructureManager may dynamically read the structure from the primary world folder, DataPacks, or the server's own resources. Structures can be registered via <a href="#registerStructure(org.bukkit.NamespacedKey,org.bukkit.structure.Structure)"><code>registerStructure(NamespacedKey, Structure)</code></a></p>
    /// Gets a registered Structure.
    /// Gets the location where a structure file would exist in the primary world directory based on the NamespacedKey using the format world/generated/{NAMESPACE}/structures/{KEY}.nbt. This method will always return a file, even if none exists at the moment.
    pub fn get_structure(
        &mut self,
        arg0: impl Into<&'mc crate::NamespacedKey<'mc>>,
    ) -> Result<crate::structure::Structure<'mc>, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getStructure",
            "(Lorg/bukkit/NamespacedKey;)Lorg/bukkit/structure/Structure;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::structure::Structure::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Gets the currently registered structures.
    /// <p>These are the currently loaded structures that the StructureManager is aware of. When a structure block refers to a structure, these structures are checked first. If the specified structure is not found among the currently registered structures, the StructureManager may dynamically read the structure from the primary world folder, DataPacks, or the server's own resources. Structures can be registered via <a href="#registerStructure(org.bukkit.NamespacedKey,org.bukkit.structure.Structure)"><code>registerStructure(NamespacedKey, Structure)</code></a></p>
    pub fn structures(
        &mut self,
    ) -> Result<blackboxmc_java::JavaMap<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getStructures",
            "()Ljava/util/Map;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::JavaMap::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Registers the given structure. See <a href="#getStructures()"><code>getStructures()</code></a>.
    pub fn register_structure(
        &mut self,
        arg0: impl Into<&'mc crate::NamespacedKey<'mc>>,
        arg1: impl Into<&'mc crate::structure::Structure<'mc>>,
    ) -> Result<crate::structure::Structure<'mc>, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let val_2 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(&self.jni_object(),"registerStructure","(Lorg/bukkit/NamespacedKey;Lorg/bukkit/structure/Structure;)Lorg/bukkit/structure/Structure;",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)]);
        let res = self.jni_ref().translate_error(res)?;
        crate::structure::Structure::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Unregisters a structure. Unregisters the specified structure. If the structure still exists in the primary world folder, a DataPack, or is part of the server's own resources, it may be loaded and registered again when it is requested by a plugin or the server itself.
    pub fn unregister_structure(
        &mut self,
        arg0: impl Into<&'mc crate::NamespacedKey<'mc>>,
    ) -> Result<crate::structure::Structure<'mc>, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "unregisterStructure",
            "(Lorg/bukkit/NamespacedKey;)Lorg/bukkit/structure/Structure;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::structure::Structure::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Loads a structure for the specified key and optionally <a href="#registerStructure(org.bukkit.NamespacedKey,org.bukkit.structure.Structure)"><code>registers</code></a> it.
    /// <p>This will first check the already loaded <a href="#getStructures()"><code>registered structures</code></a>, and otherwise load the structure from the primary world folder, DataPacks, and the server's own resources (in this order).</p>
    /// <p>When loading the structure from the primary world folder, the given key is translated to a file as specified by <a href="#getStructureFile(org.bukkit.NamespacedKey)"><code>getStructureFile(NamespacedKey)</code></a>.</p>
    /// Loads the structure for the specified key and automatically registers it. See <a href="#loadStructure(org.bukkit.NamespacedKey,boolean)"><code>loadStructure(NamespacedKey, boolean)</code></a>.
    /// Reads a Structure from disk.
    /// Reads a Structure from a stream.
    pub unsafe fn load_structure_with_input_stream(
        &mut self,
        arg0: std::option::Option<jni::objects::JObject<'mc>>,
    ) -> Result<crate::structure::Structure<'mc>, Box<dyn std::error::Error>> {
        let val_1 = arg0.unwrap();
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "loadStructure",
            "(Ljava/io/File;)Lorg/bukkit/structure/Structure;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::structure::Structure::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Loads a structure for the specified key and optionally <a href="#registerStructure(org.bukkit.NamespacedKey,org.bukkit.structure.Structure)"><code>registers</code></a> it.
    /// <p>This will first check the already loaded <a href="#getStructures()"><code>registered structures</code></a>, and otherwise load the structure from the primary world folder, DataPacks, and the server's own resources (in this order).</p>
    /// <p>When loading the structure from the primary world folder, the given key is translated to a file as specified by <a href="#getStructureFile(org.bukkit.NamespacedKey)"><code>getStructureFile(NamespacedKey)</code></a>.</p>
    /// Loads the structure for the specified key and automatically registers it. See <a href="#loadStructure(org.bukkit.NamespacedKey,boolean)"><code>loadStructure(NamespacedKey, boolean)</code></a>.
    /// Reads a Structure from disk.
    /// Reads a Structure from a stream.
    pub fn load_structure_with_namespaced_key(
        &mut self,
        arg0: std::option::Option<impl Into<&'mc crate::NamespacedKey<'mc>>>,
        arg1: std::option::Option<bool>,
    ) -> Result<crate::structure::Structure<'mc>, Box<dyn std::error::Error>> {
        let val_1 =
            unsafe { jni::objects::JObject::from_raw(arg0.unwrap().into().jni_object().clone()) };
        // 0
        let val_2 = jni::objects::JValueGen::Bool(arg1.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "loadStructure",
            "(Lorg/bukkit/NamespacedKey;Z)Lorg/bukkit/structure/Structure;",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::structure::Structure::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Saves the currently <a href="#getStructures()"><code>registered structure</code></a> for the specified <a title="class in org.bukkit" href="../NamespacedKey.html"><code>key</code></a> to the primary world folder as specified by {#getStructureFile(NamespacedKey}.
    /// Saves a structure with a given key to the primary world folder.
    /// Save a structure to a file. This will overwrite a file if it already exists.
    /// Save a structure to a stream.
    pub unsafe fn save_structure_with_namespaced_key(
        &mut self,
        arg0: std::option::Option<jni::objects::JObject<'mc>>,
        arg1: std::option::Option<impl Into<&'mc crate::structure::Structure<'mc>>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = arg0.unwrap();
        let val_2 =
            unsafe { jni::objects::JObject::from_raw(arg1.unwrap().into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "saveStructure",
            "(Ljava/io/File;Lorg/bukkit/structure/Structure;)V",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Saves the currently <a href="#getStructures()"><code>registered structure</code></a> for the specified <a href="../NamespacedKey.html" title="class in org.bukkit"><code>key</code></a> to the primary world folder as specified by {#getStructureFile(NamespacedKey}.
    /// Saves a structure with a given key to the primary world folder.
    /// Save a structure to a file. This will overwrite a file if it already exists.
    /// Save a structure to a stream.
    pub fn save_structure_with_output_stream(
        &mut self,
        arg0: impl Into<&'mc crate::NamespacedKey<'mc>>,
        arg1: std::option::Option<impl Into<&'mc crate::structure::Structure<'mc>>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let val_2 =
            unsafe { jni::objects::JObject::from_raw(arg1.unwrap().into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "saveStructure",
            "(Lorg/bukkit/NamespacedKey;Lorg/bukkit/structure/Structure;)V",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Unregisters the specified structure and deletes its <a href="#getStructureFile(org.bukkit.NamespacedKey)"><code>structure file</code></a> from the primary world folder. Note that this method cannot be used to delete vanilla Minecraft structures, or structures from DataPacks. Unregistering these structures will however work fine.
    /// Deletes the <a href="#getStructureFile(org.bukkit.NamespacedKey)"><code>structure file</code></a> for the specified structure from the primary world folder. Note that this method cannot be used to delete vanilla Minecraft structures, or structures from DataPacks. Unregistering these structures will however work fine.
    pub fn delete_structure_with_namespaced_key(
        &mut self,
        arg0: std::option::Option<impl Into<&'mc crate::NamespacedKey<'mc>>>,
        arg1: std::option::Option<bool>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 =
            unsafe { jni::objects::JObject::from_raw(arg0.unwrap().into().jni_object().clone()) };
        // 0
        let val_2 = jni::objects::JValueGen::Bool(arg1.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "deleteStructure",
            "(Lorg/bukkit/NamespacedKey;Z)V",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Gets the location where a structure file would exist in the primary world directory based on the NamespacedKey using the format world/generated/{NAMESPACE}/structures/{KEY}.nbt. This method will always return a file, even if none exists at the moment.
    pub fn get_structure_file(
        &mut self,
        arg0: impl Into<&'mc crate::NamespacedKey<'mc>>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getStructureFile",
            "(Lorg/bukkit/NamespacedKey;)Ljava/io/File;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
    /// Creates a new empty structure.
    pub fn create_structure(
        &mut self,
    ) -> Result<crate::structure::Structure<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "createStructure",
            "()Lorg/bukkit/structure/Structure;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::structure::Structure::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Creates a copy of this structure.
    pub fn copy(
        &mut self,
        arg0: impl Into<&'mc crate::structure::Structure<'mc>>,
    ) -> Result<crate::structure::Structure<'mc>, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "copy",
            "(Lorg/bukkit/structure/Structure;)Lorg/bukkit/structure/Structure;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::structure::Structure::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
}
impl<'mc> JNIRaw<'mc> for StructureManager<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
/// Represents a structure.
/// <p>A structure is a mutable template of captured blocks and entities that can be copied back into the world. The <a href="StructureManager.html" title="interface in org.bukkit.structure"><code>StructureManager</code></a>, retrieved via <a href="../Server.html#getStructureManager()"><code>Server.getStructureManager()</code></a>, allows you to create new structures, load existing structures, and save structures.</p>
/// <p>In order for a structure to be usable by structure blocks, it needs to be null <a href="StructureManager.html#registerStructure(org.bukkit.NamespacedKey,org.bukkit.structure.Structure)"><code>registered</code></a> with the <a title="interface in org.bukkit.structure" href="StructureManager.html"><code>StructureManager</code></a>, or located in the primary world folder, a DataPack, or the server's own default resources, so that the StructureManager can find it.</p>
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
        let (valid, name) = env.validate_name(&obj, "org/bukkit/structure/Structure")?;
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
    /// Gets a list of entities that have been included in the Structure. The entity positions are offsets relative to the structure's position that is provided once the structure is placed into the world.
    pub fn entities(
        &mut self,
    ) -> Result<Vec<crate::entity::Entity<'mc>>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getEntities",
            "()Ljava/util/List;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let mut list = blackboxmc_java::JavaList::from_raw(&self.jni_ref(), res.l()?)?;
        let size = list.size()?;
        for i in 0..=size {
            let obj = list.get(i)?;
            new_vec.push(crate::entity::Entity::from_raw(&self.jni_ref(), obj)?);
        }
        Ok(new_vec)
    }
    /// Gets the number of entities in this structure.
    pub fn entity_count(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getEntityCount", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }
    /// Gets a list of available block palettes.
    pub fn palettes(
        &mut self,
    ) -> Result<Vec<crate::structure::Palette<'mc>>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getPalettes",
            "()Ljava/util/List;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let mut list = blackboxmc_java::JavaList::from_raw(&self.jni_ref(), res.l()?)?;
        let size = list.size()?;
        for i in 0..=size {
            let obj = list.get(i)?;
            new_vec.push(crate::structure::Palette::from_raw(&self.jni_ref(), obj)?);
        }
        Ok(new_vec)
    }
    /// Gets the number of palettes in this structure.
    pub fn palette_count(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getPaletteCount", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }
    /// Place a structure in the world.
    /// Place a structure in the world.
    pub fn place_with_location(
        &mut self,
        arg0: impl Into<&'mc crate::RegionAccessor<'mc>>,
        arg1: impl Into<&'mc crate::util::BlockVector<'mc>>,
        arg2: bool,
        arg3: impl Into<&'mc crate::block::structure::StructureRotation<'mc>>,
        arg4: impl Into<&'mc crate::block::structure::Mirror<'mc>>,
        arg5: i32,
        arg6: std::option::Option<f32>,
        arg7: std::option::Option<impl Into<&'mc blackboxmc_java::JavaRandom<'mc>>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let val_2 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone()) };
        // 6
        let val_3 = jni::objects::JValueGen::Bool(arg2.into());
        let val_4 = unsafe { jni::objects::JObject::from_raw(arg3.into().jni_object().clone()) };
        let val_5 = unsafe { jni::objects::JObject::from_raw(arg4.into().jni_object().clone()) };
        let val_6 = jni::objects::JValueGen::Int(arg5.into());
        let val_7 = jni::objects::JValueGen::Float(arg6.unwrap().into());
        let val_8 =
            unsafe { jni::objects::JObject::from_raw(arg7.unwrap().into().jni_object().clone()) };
        let res = self.jni_ref().call_method(&self.jni_object(),"place","(Lorg/bukkit/RegionAccessor;Lorg/bukkit/util/BlockVector;ZLorg/bukkit/block/structure/StructureRotation;Lorg/bukkit/block/structure/Mirror;IFLjava/util/Random;)V",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2),jni::objects::JValueGen::from(&val_3),jni::objects::JValueGen::from(&val_4),jni::objects::JValueGen::from(&val_5),jni::objects::JValueGen::from(&val_6),jni::objects::JValueGen::from(&val_7),jni::objects::JValueGen::from(&val_8)]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Fills the structure from an area in a world. The origin and size will be calculated automatically from the two corners provided.
    /// <p>Be careful as this will override the current data of the structure.</p>
    /// <p>Be aware that this method allows for creating structures larger than the 48x48x48 size that Minecraft's Structure blocks support. Any structures saved this way can not be loaded by using a structure block. Using the API however will still work.</p>
    /// Fills the Structure from an area in a world, starting at the specified origin and extending in each axis according to the specified size vector.
    /// <p>Be careful as this will override the current data of the structure.</p>
    /// <p>Be aware that this method allows for saving structures larger than the 48x48x48 size that Minecraft's Structure blocks support. Any structures saved this way can not be loaded by using a structure block. Using the API however will still work.</p>
    pub fn fill_with_location(
        &mut self,
        arg0: impl Into<&'mc crate::Location<'mc>>,
        arg1: impl Into<&'mc crate::util::BlockVector<'mc>>,
        arg2: std::option::Option<bool>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let val_2 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone()) };
        // 2
        let val_3 = jni::objects::JValueGen::Bool(arg2.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "fill",
            "(Lorg/bukkit/Location;Lorg/bukkit/util/BlockVector;Z)V",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
                jni::objects::JValueGen::from(&val_3),
            ],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Gets the current size of the structure.
    /// <p>The size of the structure may not be fixed.</p>
    pub fn size(&mut self) -> Result<crate::util::BlockVector<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getSize",
            "()Lorg/bukkit/util/BlockVector;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::util::BlockVector::from_raw(&self.jni_ref(), unsafe {
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
}
impl<'mc> JNIRaw<'mc> for Structure<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Into<crate::persistence::PersistentDataHolder<'mc>> for Structure<'mc> {
    fn into(self) -> crate::persistence::PersistentDataHolder<'mc> {
        crate::persistence::PersistentDataHolder::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
