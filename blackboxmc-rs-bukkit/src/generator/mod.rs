#![allow(deprecated)]
use blackboxmc_general::JNIRaw;
use color_eyre::eyre::Result;
/// Data for a Chunk.
///
/// This is a representation of an abstract class.
pub struct ChunkGeneratorChunkData<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> ChunkGeneratorChunkData<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate ChunkGeneratorChunkData from null object."
            )
            .into());
        }
        let (valid, name) =
            env.validate_name(&obj, "org/bukkit/generator/ChunkGenerator$ChunkData")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a ChunkGeneratorChunkData object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    //@Deprecated

    #[deprecated]
    /// <span class="deprecated-label">Deprecated.</span>
    /// <div class="deprecation-comment">
    /// Uses magic values
    /// </div>
    /// Uses magic values
    ///
    /// Get the block data at x,y,z in the chunk data. Getting blocks outside the chunk's bounds returns 0.
    pub fn get_data(
        &mut self,
        arg0: i32,
        arg1: i32,
        arg2: i32,
    ) -> Result<i8, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let val_2 = jni::objects::JValueGen::Int(arg1.into());
        let val_3 = jni::objects::JValueGen::Int(arg2.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getData",
            "(III)B",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
                jni::objects::JValueGen::from(&val_3),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.b()?)
    }
    //@NotNull

    /// Get the type and data of the block at x, y, z. Getting blocks outside the chunk's bounds returns air.
    pub fn get_block_data(
        &mut self,
        arg0: i32,
        arg1: i32,
        arg2: i32,
    ) -> Result<crate::block::data::BlockData<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let val_2 = jni::objects::JValueGen::Int(arg1.into());
        let val_3 = jni::objects::JValueGen::Int(arg2.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getBlockData",
            "(III)Lorg/bukkit/block/data/BlockData;",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
                jni::objects::JValueGen::from(&val_3),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::block::data::BlockData::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //@NotNull

    /// Get the biome at x, y, z within chunk being generated
    pub fn get_biome(
        &mut self,
        arg0: i32,
        arg1: i32,
        arg2: i32,
    ) -> Result<crate::block::Biome<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let val_2 = jni::objects::JValueGen::Int(arg1.into());
        let val_3 = jni::objects::JValueGen::Int(arg2.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getBiome",
            "(III)Lorg/bukkit/block/Biome;",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
                jni::objects::JValueGen::from(&val_3),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant = self
            .jni_ref()
            .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[]);
        let variant = self.jni_ref().translate_error(variant)?;
        let variant_str = self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::block::Biome::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::block::Biome::from_string(variant_str)
                .ok_or(eyre::eyre!("String gaven for variant was invalid"))?,
        )
    }
    //

    pub fn set_block_with_int(
        &mut self,
        arg0: i32,
        arg1: i32,
        arg2: i32,
        arg3: std::option::Option<impl Into<crate::Material<'mc>>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let val_2 = jni::objects::JValueGen::Int(arg1.into());
        let val_3 = jni::objects::JValueGen::Int(arg2.into());
        let val_4 = unsafe {
            jni::objects::JObject::from_raw(
                arg3.ok_or(eyre::eyre!("None arguments aren't actually supported yet"))?
                    .into()
                    .jni_object()
                    .clone(),
            )
        };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setBlock",
            "(IIILorg/bukkit/Material;)V",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
                jni::objects::JValueGen::from(&val_3),
                jni::objects::JValueGen::from(&val_4),
            ],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn set_region_with_int(
        &mut self,
        arg0: i32,
        arg1: i32,
        arg2: i32,
        arg3: i32,
        arg4: i32,
        arg5: i32,
        arg6: std::option::Option<impl Into<crate::Material<'mc>>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let val_2 = jni::objects::JValueGen::Int(arg1.into());
        let val_3 = jni::objects::JValueGen::Int(arg2.into());
        let val_4 = jni::objects::JValueGen::Int(arg3.into());
        let val_5 = jni::objects::JValueGen::Int(arg4.into());
        let val_6 = jni::objects::JValueGen::Int(arg5.into());
        let val_7 = unsafe {
            jni::objects::JObject::from_raw(
                arg6.ok_or(eyre::eyre!("None arguments aren't actually supported yet"))?
                    .into()
                    .jni_object()
                    .clone(),
            )
        };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setRegion",
            "(IIIIIILorg/bukkit/Material;)V",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
                jni::objects::JValueGen::from(&val_3),
                jni::objects::JValueGen::from(&val_4),
                jni::objects::JValueGen::from(&val_5),
                jni::objects::JValueGen::from(&val_6),
                jni::objects::JValueGen::from(&val_7),
            ],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //@NotNull

    /// Get the type and data of the block at x, y, z. Getting blocks outside the chunk's bounds returns air.
    pub fn get_type_and_data(
        &mut self,
        arg0: i32,
        arg1: i32,
        arg2: i32,
    ) -> Result<crate::material::MaterialData<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let val_2 = jni::objects::JValueGen::Int(arg1.into());
        let val_3 = jni::objects::JValueGen::Int(arg2.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getTypeAndData",
            "(III)Lorg/bukkit/material/MaterialData;",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
                jni::objects::JValueGen::from(&val_3),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::material::MaterialData::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn min_height(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getMinHeight", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    //

    pub fn max_height(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getMaxHeight", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    //@NotNull

    //@NotNull

    /// Get the type of the block at x, y, z. Getting blocks outside the chunk's bounds returns air.
    /// Get the type and data of the block at x, y, z. Getting blocks outside the chunk's bounds returns air.
    pub fn get_type(
        &mut self,
        arg0: i32,
        arg1: i32,
        arg2: i32,
    ) -> Result<crate::Material<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let val_2 = jni::objects::JValueGen::Int(arg1.into());
        let val_3 = jni::objects::JValueGen::Int(arg2.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getType",
            "(III)Lorg/bukkit/Material;",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
                jni::objects::JValueGen::from(&val_3),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant = self
            .jni_ref()
            .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[]);
        let variant = self.jni_ref().translate_error(variant)?;
        let variant_str = self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::Material::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::Material::from_string(variant_str)
                .ok_or(eyre::eyre!("String gaven for variant was invalid"))?,
        )
    }
}
impl<'mc> JNIRaw<'mc> for ChunkGeneratorChunkData<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
/// Class for providing biomes.
pub struct BiomeProvider<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for BiomeProvider<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> BiomeProvider<'mc> {
    pub fn from_extendable(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        plugin: &'mc crate::plugin::Plugin,
        address: i32,
        lib_name: String,
        name: String,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let obj = unsafe { plugin.new_extendable(address, "BiomeProvider", name, lib_name) }?;
        Self::from_raw(env, obj)
    }
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate BiomeProvider from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/generator/BiomeProvider")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a BiomeProvider object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<crate::generator::BiomeProvider<'mc>, Box<dyn std::error::Error>> {
        let cls = jni.find_class("org/bukkit/generator/BiomeProvider");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(cls, "()V", &[]);
        let res = jni.translate_error_no_gen(res)?;
        crate::generator::BiomeProvider::from_raw(&jni, res)
    }
    //

    pub fn get_biome_with_world_info(
        &mut self,
        arg0: impl Into<crate::generator::WorldInfo<'mc>>,
        arg1: i32,
        arg2: i32,
        arg3: std::option::Option<i32>,
        arg4: std::option::Option<impl Into<crate::generator::BiomeParameterPoint<'mc>>>,
    ) -> Result<crate::block::Biome<'mc>, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let val_2 = jni::objects::JValueGen::Int(arg1.into());
        let val_3 = jni::objects::JValueGen::Int(arg2.into());
        let val_4 = jni::objects::JValueGen::Int(
            arg3.ok_or(eyre::eyre!("None arguments aren't actually supported yet"))?
                .into(),
        );
        let val_5 = unsafe {
            jni::objects::JObject::from_raw(
                arg4.ok_or(eyre::eyre!("None arguments aren't actually supported yet"))?
                    .into()
                    .jni_object()
                    .clone(),
            )
        };
        let res = self.jni_ref().call_method(&self.jni_object(),"getBiome","(Lorg/bukkit/generator/WorldInfo;IIILorg/bukkit/generator/BiomeParameterPoint;)Lorg/bukkit/block/Biome;",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2),jni::objects::JValueGen::from(&val_3),jni::objects::JValueGen::from(&val_4),jni::objects::JValueGen::from(&val_5)]);
        let res = self.jni_ref().translate_error(res)?;
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant = self
            .0
            .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[]);
        let variant = self.jni_ref().translate_error(variant)?;
        let variant_str = self
            .0
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::block::Biome::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::block::Biome::from_string(variant_str)
                .ok_or(eyre::eyre!("String gaven for variant was invalid"))?,
        )
    }
    //

    pub fn get_biomes(
        &mut self,
        arg0: impl Into<crate::generator::WorldInfo<'mc>>,
    ) -> Result<Vec<crate::block::Biome<'mc>>, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getBiomes",
            "(Lorg/bukkit/generator/WorldInfo;)Ljava/util/List;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let mut list = blackboxmc_java::JavaList::from_raw(&self.0, res.l()?)?;
        let size = list.size()?;
        for i in 0..=size {
            let obj = list.get(i)?;
            let variant = self
                .0
                .call_method(list.get(i)?, "toString", "()Ljava/lang/String;", &[]);
            let variant = self.jni_ref().translate_error(variant)?;
            let variant_str = self
                .0
                .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
                .to_string_lossy()
                .to_string();
            new_vec.push(crate::block::Biome::from_raw(
                &self.0,
                obj,
                crate::block::Biome::from_string(variant_str)
                    .ok_or(eyre::eyre!("String gaven for variant was invalid"))?,
            )?);
        }
        Ok(new_vec)
    }
    //

    pub fn wait(
        &mut self,
        arg0: std::option::Option<i64>,
        arg1: std::option::Option<i32>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Long(
            arg0.ok_or(eyre::eyre!("None arguments aren't actually supported yet"))?
                .into(),
        );
        let val_2 = jni::objects::JValueGen::Int(
            arg1.ok_or(eyre::eyre!("None arguments aren't actually supported yet"))?
                .into(),
        );
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
    //

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
        Ok(res.z()?)
    }
    //

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
    //

    pub fn hash_code(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hashCode", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    //

    pub fn class(&mut self) -> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getClass", "()Ljava/lang/Class;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(unsafe { jni::objects::JClass::from_raw(res.as_jni().l) })
    }
    //

    pub fn notify(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notify", "()V", &[]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn notify_all(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notifyAll", "()V", &[]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
}
/// Interface to biome section for chunk to be generated: initialized with default values for world type and seed.
/// <p>Custom generator is free to access and tailor values during generateBlockSections() or generateExtBlockSections().</p>
///
/// This is a representation of an abstract class.
pub struct ChunkGeneratorBiomeGrid<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> ChunkGeneratorBiomeGrid<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate ChunkGeneratorBiomeGrid from null object."
            )
            .into());
        }
        let (valid, name) =
            env.validate_name(&obj, "org/bukkit/generator/ChunkGenerator$BiomeGrid")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a ChunkGeneratorBiomeGrid object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    //@NotNull

    /// <span class="deprecated-label">Deprecated.</span>
    /// Get biome at x, z within chunk being generated
    pub fn get_biome_with_int(
        &mut self,
        arg0: i32,
        arg1: std::option::Option<i32>,
        arg2: std::option::Option<i32>,
    ) -> Result<crate::block::Biome<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let val_2 = jni::objects::JValueGen::Int(
            arg1.ok_or(eyre::eyre!("None arguments aren't actually supported yet"))?
                .into(),
        );
        let val_3 = jni::objects::JValueGen::Int(
            arg2.ok_or(eyre::eyre!("None arguments aren't actually supported yet"))?
                .into(),
        );
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getBiome",
            "(III)Lorg/bukkit/block/Biome;",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
                jni::objects::JValueGen::from(&val_3),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant = self
            .jni_ref()
            .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[]);
        let variant = self.jni_ref().translate_error(variant)?;
        let variant_str = self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::block::Biome::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::block::Biome::from_string(variant_str)
                .ok_or(eyre::eyre!("String gaven for variant was invalid"))?,
        )
    }
    //

    pub fn set_biome_with_int(
        &mut self,
        arg0: i32,
        arg1: i32,
        arg2: std::option::Option<i32>,
        arg3: std::option::Option<impl Into<crate::block::Biome<'mc>>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let val_2 = jni::objects::JValueGen::Int(arg1.into());
        let val_3 = jni::objects::JValueGen::Int(
            arg2.ok_or(eyre::eyre!("None arguments aren't actually supported yet"))?
                .into(),
        );
        let val_4 = unsafe {
            jni::objects::JObject::from_raw(
                arg3.ok_or(eyre::eyre!("None arguments aren't actually supported yet"))?
                    .into()
                    .jni_object()
                    .clone(),
            )
        };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setBiome",
            "(IIILorg/bukkit/block/Biome;)V",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
                jni::objects::JValueGen::from(&val_3),
                jni::objects::JValueGen::from(&val_4),
            ],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
}
impl<'mc> JNIRaw<'mc> for ChunkGeneratorBiomeGrid<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
/// Represents the biome noise parameters which may be passed to a world generator.
///
/// This is a representation of an abstract class.
pub struct BiomeParameterPoint<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> BiomeParameterPoint<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate BiomeParameterPoint from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/generator/BiomeParameterPoint")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a BiomeParameterPoint object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    //

    pub fn min_erosion(&mut self) -> Result<f64, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getMinErosion", "()D", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.d()?)
    }
    //

    pub fn temperature(&mut self) -> Result<f64, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getTemperature", "()D", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.d()?)
    }
    //

    pub fn max_temperature(&mut self) -> Result<f64, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getMaxTemperature", "()D", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.d()?)
    }
    //

    pub fn min_temperature(&mut self) -> Result<f64, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getMinTemperature", "()D", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.d()?)
    }
    //

    pub fn humidity(&mut self) -> Result<f64, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getHumidity", "()D", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.d()?)
    }
    //

    pub fn max_humidity(&mut self) -> Result<f64, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getMaxHumidity", "()D", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.d()?)
    }
    //

    pub fn min_humidity(&mut self) -> Result<f64, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getMinHumidity", "()D", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.d()?)
    }
    //

    pub fn continentalness(&mut self) -> Result<f64, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getContinentalness", "()D", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.d()?)
    }
    //

    pub fn max_continentalness(&mut self) -> Result<f64, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getMaxContinentalness", "()D", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.d()?)
    }
    //

    pub fn min_continentalness(&mut self) -> Result<f64, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getMinContinentalness", "()D", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.d()?)
    }
    //

    pub fn erosion(&mut self) -> Result<f64, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getErosion", "()D", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.d()?)
    }
    //

    pub fn max_erosion(&mut self) -> Result<f64, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getMaxErosion", "()D", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.d()?)
    }
    //

    pub fn depth(&mut self) -> Result<f64, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getDepth", "()D", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.d()?)
    }
    //

    pub fn max_depth(&mut self) -> Result<f64, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getMaxDepth", "()D", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.d()?)
    }
    //

    pub fn min_depth(&mut self) -> Result<f64, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getMinDepth", "()D", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.d()?)
    }
    //

    pub fn weirdness(&mut self) -> Result<f64, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getWeirdness", "()D", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.d()?)
    }
    //

    pub fn max_weirdness(&mut self) -> Result<f64, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getMaxWeirdness", "()D", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.d()?)
    }
    //

    pub fn min_weirdness(&mut self) -> Result<f64, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getMinWeirdness", "()D", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.d()?)
    }
}
impl<'mc> JNIRaw<'mc> for BiomeParameterPoint<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
/// A block populator is responsible for generating a small area of blocks.
/// <p>For example, generating glowstone inside the nether or generating dungeons full of treasure</p>
/// <p>A BlockPopulator can be used in combination with a custom <a href="ChunkGenerator.html" title="class in org.bukkit.generator"><code>ChunkGenerator</code></a> by returning it in the method <a href="ChunkGenerator.html#getDefaultPopulators(org.bukkit.World)"><code>ChunkGenerator.getDefaultPopulators(World)</code></a> or by adding it manually to the worlds populator list returned by <a href="../World.html#getPopulators()"><code>World.getPopulators()</code></a>.</p>
/// <p>When adding a BlockPopulator manually to a world it is recommended to do so during the <a href="../event/world/WorldInitEvent.html" title="class in org.bukkit.event.world"><code>WorldInitEvent</code></a>.</p>
pub struct BlockPopulator<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for BlockPopulator<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> BlockPopulator<'mc> {
    pub fn from_extendable(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        plugin: &'mc crate::plugin::Plugin,
        address: i32,
        lib_name: String,
        name: String,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let obj = unsafe { plugin.new_extendable(address, "BlockPopulator", name, lib_name) }?;
        Self::from_raw(env, obj)
    }
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate BlockPopulator from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/generator/BlockPopulator")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a BlockPopulator object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<crate::generator::BlockPopulator<'mc>, Box<dyn std::error::Error>> {
        let cls = jni.find_class("org/bukkit/generator/BlockPopulator");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(cls, "()V", &[]);
        let res = jni.translate_error_no_gen(res)?;
        crate::generator::BlockPopulator::from_raw(&jni, res)
    }
    //

    pub fn populate_with_world(
        &mut self,
        arg0: impl Into<crate::generator::WorldInfo<'mc>>,
        arg1: impl Into<blackboxmc_java::JavaRandom<'mc>>,
        arg2: std::option::Option<i32>,
        arg3: std::option::Option<i32>,
        arg4: std::option::Option<impl Into<crate::generator::LimitedRegion<'mc>>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let val_2 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone()) };
        let val_3 = jni::objects::JValueGen::Int(
            arg2.ok_or(eyre::eyre!("None arguments aren't actually supported yet"))?
                .into(),
        );
        let val_4 = jni::objects::JValueGen::Int(
            arg3.ok_or(eyre::eyre!("None arguments aren't actually supported yet"))?
                .into(),
        );
        let val_5 = unsafe {
            jni::objects::JObject::from_raw(
                arg4.ok_or(eyre::eyre!("None arguments aren't actually supported yet"))?
                    .into()
                    .jni_object()
                    .clone(),
            )
        };
        let res = self.jni_ref().call_method(&self.jni_object(),"populate","(Lorg/bukkit/generator/WorldInfo;Ljava/util/Random;IILorg/bukkit/generator/LimitedRegion;)V",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2),jni::objects::JValueGen::from(&val_3),jni::objects::JValueGen::from(&val_4),jni::objects::JValueGen::from(&val_5)]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn wait(
        &mut self,
        arg0: std::option::Option<i64>,
        arg1: std::option::Option<i32>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Long(
            arg0.ok_or(eyre::eyre!("None arguments aren't actually supported yet"))?
                .into(),
        );
        let val_2 = jni::objects::JValueGen::Int(
            arg1.ok_or(eyre::eyre!("None arguments aren't actually supported yet"))?
                .into(),
        );
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
    //

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
        Ok(res.z()?)
    }
    //

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
    //

    pub fn hash_code(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hashCode", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    //

    pub fn class(&mut self) -> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getClass", "()Ljava/lang/Class;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(unsafe { jni::objects::JClass::from_raw(res.as_jni().l) })
    }
    //

    pub fn notify(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notify", "()V", &[]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn notify_all(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notifyAll", "()V", &[]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
}
/// A limited region is used in world generation for features which are going over a chunk. For example, trees or ores. Use <a href="#getBuffer()"><code>getBuffer()</code></a> to know how much you can go beyond the central chunk. The buffer zone may or may not be already populated. The coordinates are <b>absolute</b> from the world origin.
///
/// This is a representation of an abstract class.
pub struct LimitedRegion<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> LimitedRegion<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate LimitedRegion from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/generator/LimitedRegion")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a LimitedRegion object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    //

    pub fn buffer(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getBuffer", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    //

    /// Checks if the given coordinates are in the region.
    pub fn is_in_region_with_location(
        &mut self,
        arg0: std::option::Option<i32>,
        arg1: std::option::Option<i32>,
        arg2: std::option::Option<i32>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(
            arg0.ok_or(eyre::eyre!("None arguments aren't actually supported yet"))?
                .into(),
        );
        let val_2 = jni::objects::JValueGen::Int(
            arg1.ok_or(eyre::eyre!("None arguments aren't actually supported yet"))?
                .into(),
        );
        let val_3 = jni::objects::JValueGen::Int(
            arg2.ok_or(eyre::eyre!("None arguments aren't actually supported yet"))?
                .into(),
        );
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "isInRegion",
            "(III)Z",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
                jni::objects::JValueGen::from(&val_3),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn tile_entities(
        &mut self,
    ) -> Result<Vec<crate::block::BlockState<'mc>>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getTileEntities",
            "()Ljava/util/List;",
            &[],
        );
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
    //

    pub fn set_type_with_location(
        &mut self,
        arg0: i32,
        arg1: std::option::Option<i32>,
        arg2: std::option::Option<i32>,
        arg3: std::option::Option<impl Into<crate::Material<'mc>>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let val_2 = jni::objects::JValueGen::Int(
            arg1.ok_or(eyre::eyre!("None arguments aren't actually supported yet"))?
                .into(),
        );
        let val_3 = jni::objects::JValueGen::Int(
            arg2.ok_or(eyre::eyre!("None arguments aren't actually supported yet"))?
                .into(),
        );
        let val_4 = unsafe {
            jni::objects::JObject::from_raw(
                arg3.ok_or(eyre::eyre!("None arguments aren't actually supported yet"))?
                    .into()
                    .jni_object()
                    .clone(),
            )
        };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setType",
            "(IIILorg/bukkit/Material;)V",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
                jni::objects::JValueGen::from(&val_3),
                jni::objects::JValueGen::from(&val_4),
            ],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //@NotNull

    pub fn get_block_data_with_location(
        &mut self,
        arg0: std::option::Option<i32>,
        arg1: std::option::Option<i32>,
        arg2: std::option::Option<i32>,
    ) -> Result<crate::block::data::BlockData<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(
            arg0.ok_or(eyre::eyre!("None arguments aren't actually supported yet"))?
                .into(),
        );
        let val_2 = jni::objects::JValueGen::Int(
            arg1.ok_or(eyre::eyre!("None arguments aren't actually supported yet"))?
                .into(),
        );
        let val_3 = jni::objects::JValueGen::Int(
            arg2.ok_or(eyre::eyre!("None arguments aren't actually supported yet"))?
                .into(),
        );
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getBlockData",
            "(III)Lorg/bukkit/block/data/BlockData;",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
                jni::objects::JValueGen::from(&val_3),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::block::data::BlockData::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn set_block_data_with_location(
        &mut self,
        arg0: i32,
        arg1: std::option::Option<i32>,
        arg2: std::option::Option<i32>,
        arg3: std::option::Option<impl Into<crate::block::data::BlockData<'mc>>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let val_2 = jni::objects::JValueGen::Int(
            arg1.ok_or(eyre::eyre!("None arguments aren't actually supported yet"))?
                .into(),
        );
        let val_3 = jni::objects::JValueGen::Int(
            arg2.ok_or(eyre::eyre!("None arguments aren't actually supported yet"))?
                .into(),
        );
        let val_4 = unsafe {
            jni::objects::JObject::from_raw(
                arg3.ok_or(eyre::eyre!("None arguments aren't actually supported yet"))?
                    .into()
                    .jni_object()
                    .clone(),
            )
        };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setBlockData",
            "(IIILorg/bukkit/block/data/BlockData;)V",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
                jni::objects::JValueGen::from(&val_3),
                jni::objects::JValueGen::from(&val_4),
            ],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn get_highest_block_yat_with_location(
        &mut self,
        arg0: std::option::Option<i32>,
        arg1: std::option::Option<i32>,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(
            arg0.ok_or(eyre::eyre!("None arguments aren't actually supported yet"))?
                .into(),
        );
        let val_2 = jni::objects::JValueGen::Int(
            arg1.ok_or(eyre::eyre!("None arguments aren't actually supported yet"))?
                .into(),
        );
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getHighestBlockYAt",
            "(II)I",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    //

    pub fn get_highest_block_yat_with_int(
        &mut self,
        arg0: i32,
        arg1: i32,
        arg2: std::option::Option<impl Into<crate::HeightMap<'mc>>>,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let val_2 = jni::objects::JValueGen::Int(arg1.into());
        let val_3 = unsafe {
            jni::objects::JObject::from_raw(
                arg2.ok_or(eyre::eyre!("None arguments aren't actually supported yet"))?
                    .into()
                    .jni_object()
                    .clone(),
            )
        };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getHighestBlockYAt",
            "(IILorg/bukkit/HeightMap;)I",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
                jni::objects::JValueGen::from(&val_3),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    //@NotNull

    pub fn get_biome_with_location(
        &mut self,
        arg0: std::option::Option<i32>,
        arg1: std::option::Option<i32>,
        arg2: std::option::Option<i32>,
    ) -> Result<crate::block::Biome<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(
            arg0.ok_or(eyre::eyre!("None arguments aren't actually supported yet"))?
                .into(),
        );
        let val_2 = jni::objects::JValueGen::Int(
            arg1.ok_or(eyre::eyre!("None arguments aren't actually supported yet"))?
                .into(),
        );
        let val_3 = jni::objects::JValueGen::Int(
            arg2.ok_or(eyre::eyre!("None arguments aren't actually supported yet"))?
                .into(),
        );
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getBiome",
            "(III)Lorg/bukkit/block/Biome;",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
                jni::objects::JValueGen::from(&val_3),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant = self
            .jni_ref()
            .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[]);
        let variant = self.jni_ref().translate_error(variant)?;
        let variant_str = self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::block::Biome::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::block::Biome::from_string(variant_str)
                .ok_or(eyre::eyre!("String gaven for variant was invalid"))?,
        )
    }
    //

    pub fn set_biome_with_location(
        &mut self,
        arg0: i32,
        arg1: std::option::Option<i32>,
        arg2: std::option::Option<i32>,
        arg3: std::option::Option<impl Into<crate::block::Biome<'mc>>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let val_2 = jni::objects::JValueGen::Int(
            arg1.ok_or(eyre::eyre!("None arguments aren't actually supported yet"))?
                .into(),
        );
        let val_3 = jni::objects::JValueGen::Int(
            arg2.ok_or(eyre::eyre!("None arguments aren't actually supported yet"))?
                .into(),
        );
        let val_4 = unsafe {
            jni::objects::JObject::from_raw(
                arg3.ok_or(eyre::eyre!("None arguments aren't actually supported yet"))?
                    .into()
                    .jni_object()
                    .clone(),
            )
        };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setBiome",
            "(IIILorg/bukkit/block/Biome;)V",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
                jni::objects::JValueGen::from(&val_3),
                jni::objects::JValueGen::from(&val_4),
            ],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //@NotNull

    pub fn get_block_state_with_location(
        &mut self,
        arg0: std::option::Option<i32>,
        arg1: std::option::Option<i32>,
        arg2: std::option::Option<i32>,
    ) -> Result<crate::block::BlockState<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(
            arg0.ok_or(eyre::eyre!("None arguments aren't actually supported yet"))?
                .into(),
        );
        let val_2 = jni::objects::JValueGen::Int(
            arg1.ok_or(eyre::eyre!("None arguments aren't actually supported yet"))?
                .into(),
        );
        let val_3 = jni::objects::JValueGen::Int(
            arg2.ok_or(eyre::eyre!("None arguments aren't actually supported yet"))?
                .into(),
        );
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getBlockState",
            "(III)Lorg/bukkit/block/BlockState;",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
                jni::objects::JValueGen::from(&val_3),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::block::BlockState::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn spawn_entity_with_location(
        &mut self,
        arg0: impl Into<crate::Location<'mc>>,
        arg1: std::option::Option<impl Into<crate::entity::EntityType<'mc>>>,
        arg2: std::option::Option<bool>,
    ) -> Result<crate::entity::Entity<'mc>, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let val_2 = unsafe {
            jni::objects::JObject::from_raw(
                arg1.ok_or(eyre::eyre!("None arguments aren't actually supported yet"))?
                    .into()
                    .jni_object()
                    .clone(),
            )
        };
        // 1
        let val_3 = jni::objects::JValueGen::Bool(
            arg2.ok_or(eyre::eyre!("None arguments aren't actually supported yet"))?
                .into(),
        );
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "spawnEntity",
            "(Lorg/bukkit/Location;Lorg/bukkit/entity/EntityType;Z)Lorg/bukkit/entity/Entity;",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
                jni::objects::JValueGen::from(&val_3),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::entity::Entity::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

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
    //

    pub fn living_entities(
        &mut self,
    ) -> Result<Vec<crate::entity::LivingEntity<'mc>>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getLivingEntities",
            "()Ljava/util/List;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let mut list = blackboxmc_java::JavaList::from_raw(&self.jni_ref(), res.l()?)?;
        let size = list.size()?;
        for i in 0..=size {
            let obj = list.get(i)?;
            new_vec.push(crate::entity::LivingEntity::from_raw(&self.jni_ref(), obj)?);
        }
        Ok(new_vec)
    }
    //

    pub fn get_entities_by_class(
        &mut self,
        arg0: jni::objects::JClass<'mc>,
    ) -> Result<Vec<jni::objects::JObject<'mc>>, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getEntitiesByClass",
            "(Ljava/lang/Class;)Ljava/util/Collection;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let mut col = blackboxmc_java::JavaCollection::from_raw(&self.jni_ref(), res.l()?)?;
        let mut iter = blackboxmc_java::JavaIterator::from_raw(&self.jni_ref(), col.iterator()?)?;
        while iter.has_next()? {
            let obj = iter.next()?;
            new_vec.push(obj);
        }
        Ok(new_vec)
    }
    //

    pub fn get_entities_by_classes(
        &mut self,
        arg0: Vec<jni::objects::JClass<'mc>>,
    ) -> Result<Vec<crate::entity::Entity<'mc>>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getEntitiesByClasses",
            "(Ljava/lang/Class;)Ljava/util/Collection;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let mut col = blackboxmc_java::JavaCollection::from_raw(&self.jni_ref(), res.l()?)?;
        let mut iter = blackboxmc_java::JavaIterator::from_raw(&self.jni_ref(), col.iterator()?)?;
        while iter.has_next()? {
            let obj = iter.next()?;
            new_vec.push(crate::entity::Entity::from_raw(&self.jni_ref(), obj)?);
        }
        Ok(new_vec)
    }
    //

    pub fn spawn_with_location(
        &mut self,
        arg0: impl Into<crate::Location<'mc>>,
        arg1: std::option::Option<jni::objects::JClass<'mc>>,
        arg2: std::option::Option<bool>,
        arg3: std::option::Option<impl Into<crate::util::Consumer<'mc>>>,
    ) -> Result<crate::entity::Entity<'mc>, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let val_2 = arg1.ok_or(eyre::eyre!("None arguments aren't actually supported yet"))?;
        // 1
        let val_3 = jni::objects::JValueGen::Bool(
            arg2.ok_or(eyre::eyre!("None arguments aren't actually supported yet"))?
                .into(),
        );
        let val_4 = unsafe {
            jni::objects::JObject::from_raw(
                arg3.ok_or(eyre::eyre!("None arguments aren't actually supported yet"))?
                    .into()
                    .jni_object()
                    .clone(),
            )
        };
        let res = self.jni_ref().call_method(&self.jni_object(),"spawn","(Lorg/bukkit/Location;Ljava/lang/Class;ZLorg/bukkit/util/Consumer;)Lorg/bukkit/entity/Entity;",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2),jni::objects::JValueGen::from(&val_3),jni::objects::JValueGen::from(&val_4)]);
        let res = self.jni_ref().translate_error(res)?;
        crate::entity::Entity::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //@NotNull

    //@NotNull

    pub fn get_type_with_location(
        &mut self,
        arg0: std::option::Option<i32>,
        arg1: std::option::Option<i32>,
        arg2: std::option::Option<i32>,
    ) -> Result<crate::Material<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(
            arg0.ok_or(eyre::eyre!("None arguments aren't actually supported yet"))?
                .into(),
        );
        let val_2 = jni::objects::JValueGen::Int(
            arg1.ok_or(eyre::eyre!("None arguments aren't actually supported yet"))?
                .into(),
        );
        let val_3 = jni::objects::JValueGen::Int(
            arg2.ok_or(eyre::eyre!("None arguments aren't actually supported yet"))?
                .into(),
        );
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getType",
            "(III)Lorg/bukkit/Material;",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
                jni::objects::JValueGen::from(&val_3),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant = self
            .jni_ref()
            .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[]);
        let variant = self.jni_ref().translate_error(variant)?;
        let variant_str = self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::Material::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::Material::from_string(variant_str)
                .ok_or(eyre::eyre!("String gaven for variant was invalid"))?,
        )
    }
}
impl<'mc> JNIRaw<'mc> for LimitedRegion<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Into<crate::RegionAccessor<'mc>> for LimitedRegion<'mc> {
    fn into(self) -> crate::RegionAccessor<'mc> {
        crate::RegionAccessor::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting LimitedRegion into crate::RegionAccessor")
    }
}
/// Holds various information of a World
///
/// This is a representation of an abstract class.
pub struct WorldInfo<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> WorldInfo<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate WorldInfo from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/generator/WorldInfo")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a WorldInfo object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    //

    pub fn seed(&mut self) -> Result<i64, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getSeed", "()J", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.j()?)
    }
    //

    //

    pub fn environment(
        &mut self,
    ) -> Result<crate::WorldEnvironment<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getEnvironment",
            "()Lorg/bukkit/World$Environment;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant = self
            .jni_ref()
            .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[]);
        let variant = self.jni_ref().translate_error(variant)?;
        let variant_str = self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::WorldEnvironment::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::WorldEnvironment::from_string(variant_str)
                .ok_or(eyre::eyre!("String gaven for variant was invalid"))?,
        )
    }
    //

    pub fn min_height(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getMinHeight", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    //

    pub fn max_height(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getMaxHeight", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    //

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
}
impl<'mc> JNIRaw<'mc> for WorldInfo<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
/// A chunk generator is responsible for the initial shaping of an entire chunk. For example, the nether chunk generator should shape netherrack and soulsand. A chunk is generated in multiple steps, those steps are always in the same order. Between those steps however an unlimited time may pass. This means, a chunk may generated until the surface step and continue with the bedrock step after one or multiple server restarts or even after multiple Minecraft versions. The order of generation is as follows
/// <ol>
/// <li><a href="#generateNoise(org.bukkit.generator.WorldInfo,java.util.Random,int,int,org.bukkit.generator.ChunkGenerator.ChunkData)"><code>generateNoise(WorldInfo, Random, int, int, ChunkData)</code></a></li>
/// <li><a href="#generateSurface(org.bukkit.generator.WorldInfo,java.util.Random,int,int,org.bukkit.generator.ChunkGenerator.ChunkData)"><code>generateSurface(WorldInfo, Random, int, int, ChunkData)</code></a></li>
/// <li><a href="#generateBedrock(org.bukkit.generator.WorldInfo,java.util.Random,int,int,org.bukkit.generator.ChunkGenerator.ChunkData)"><code>generateBedrock(WorldInfo, Random, int, int, ChunkData)</code></a></li>
/// <li><a href="#generateCaves(org.bukkit.generator.WorldInfo,java.util.Random,int,int,org.bukkit.generator.ChunkGenerator.ChunkData)"><code>generateCaves(WorldInfo, Random, int, int, ChunkData)</code></a></li>
/// </ol> Every method listed above as well as <a href="#getBaseHeight(org.bukkit.generator.WorldInfo,java.util.Random,int,int,org.bukkit.HeightMap)"><code>getBaseHeight(WorldInfo, Random, int, int, HeightMap)</code></a> <b>must</b> be completely thread safe and able to handle multiple concurrent callers. Some aspects of world generation can be delegated to the Vanilla generator. The following methods can be overridden to enable this:
/// <ul>
/// <li><a href="#shouldGenerateNoise()"><code>shouldGenerateNoise()</code></a> or <a href="#shouldGenerateNoise(org.bukkit.generator.WorldInfo,java.util.Random,int,int)"><code>shouldGenerateNoise(WorldInfo, Random, int, int)</code></a></li>
/// <li><a href="#shouldGenerateSurface()"><code>shouldGenerateSurface()</code></a> or <a href="#shouldGenerateSurface(org.bukkit.generator.WorldInfo,java.util.Random,int,int)"><code>shouldGenerateSurface(WorldInfo, Random, int, int)</code></a></li>
/// <li><a href="#shouldGenerateCaves()"><code>shouldGenerateCaves()</code></a> or <a href="#shouldGenerateCaves(org.bukkit.generator.WorldInfo,java.util.Random,int,int)"><code>shouldGenerateCaves(WorldInfo, Random, int, int)</code></a></li>
/// <li><a href="#shouldGenerateDecorations()"><code>shouldGenerateDecorations()</code></a> or <a href="#shouldGenerateDecorations(org.bukkit.generator.WorldInfo,java.util.Random,int,int)"><code>shouldGenerateDecorations(WorldInfo, Random, int, int)</code></a></li>
/// <li><a href="#shouldGenerateMobs()"><code>shouldGenerateMobs()</code></a> or <a href="#shouldGenerateMobs(org.bukkit.generator.WorldInfo,java.util.Random,int,int)"><code>shouldGenerateMobs(WorldInfo, Random, int, int)</code></a></li>
/// <li><a href="#shouldGenerateStructures()"><code>shouldGenerateStructures()</code></a> or <a href="#shouldGenerateStructures(org.bukkit.generator.WorldInfo,java.util.Random,int,int)"><code>shouldGenerateStructures(WorldInfo, Random, int, int)</code></a></li>
/// </ul>
pub struct ChunkGenerator<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for ChunkGenerator<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> ChunkGenerator<'mc> {
    pub fn from_extendable(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        plugin: &'mc crate::plugin::Plugin,
        address: i32,
        lib_name: String,
        name: String,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let obj = unsafe { plugin.new_extendable(address, "ChunkGenerator", name, lib_name) }?;
        Self::from_raw(env, obj)
    }
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate ChunkGenerator from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/generator/ChunkGenerator")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a ChunkGenerator object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<crate::generator::ChunkGenerator<'mc>, Box<dyn std::error::Error>> {
        let cls = jni.find_class("org/bukkit/generator/ChunkGenerator");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(cls, "()V", &[]);
        let res = jni.translate_error_no_gen(res)?;
        crate::generator::ChunkGenerator::from_raw(&jni, res)
    }
    //

    pub fn generate_noise(
        &mut self,
        arg0: impl Into<crate::generator::WorldInfo<'mc>>,
        arg1: impl Into<blackboxmc_java::JavaRandom<'mc>>,
        arg2: i32,
        arg3: i32,
        arg4: impl Into<crate::generator::ChunkGeneratorChunkData<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let val_2 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone()) };
        let val_3 = jni::objects::JValueGen::Int(arg2.into());
        let val_4 = jni::objects::JValueGen::Int(arg3.into());
        let val_5 = unsafe { jni::objects::JObject::from_raw(arg4.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(&self.jni_object(),"generateNoise","(Lorg/bukkit/generator/WorldInfo;Ljava/util/Random;IILorg/bukkit/generator/ChunkGenerator$ChunkData;)V",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2),jni::objects::JValueGen::from(&val_3),jni::objects::JValueGen::from(&val_4),jni::objects::JValueGen::from(&val_5)]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn generate_surface(
        &mut self,
        arg0: impl Into<crate::generator::WorldInfo<'mc>>,
        arg1: impl Into<blackboxmc_java::JavaRandom<'mc>>,
        arg2: i32,
        arg3: i32,
        arg4: impl Into<crate::generator::ChunkGeneratorChunkData<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let val_2 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone()) };
        let val_3 = jni::objects::JValueGen::Int(arg2.into());
        let val_4 = jni::objects::JValueGen::Int(arg3.into());
        let val_5 = unsafe { jni::objects::JObject::from_raw(arg4.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(&self.jni_object(),"generateSurface","(Lorg/bukkit/generator/WorldInfo;Ljava/util/Random;IILorg/bukkit/generator/ChunkGenerator$ChunkData;)V",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2),jni::objects::JValueGen::from(&val_3),jni::objects::JValueGen::from(&val_4),jni::objects::JValueGen::from(&val_5)]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn generate_bedrock(
        &mut self,
        arg0: impl Into<crate::generator::WorldInfo<'mc>>,
        arg1: impl Into<blackboxmc_java::JavaRandom<'mc>>,
        arg2: i32,
        arg3: i32,
        arg4: impl Into<crate::generator::ChunkGeneratorChunkData<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let val_2 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone()) };
        let val_3 = jni::objects::JValueGen::Int(arg2.into());
        let val_4 = jni::objects::JValueGen::Int(arg3.into());
        let val_5 = unsafe { jni::objects::JObject::from_raw(arg4.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(&self.jni_object(),"generateBedrock","(Lorg/bukkit/generator/WorldInfo;Ljava/util/Random;IILorg/bukkit/generator/ChunkGenerator$ChunkData;)V",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2),jni::objects::JValueGen::from(&val_3),jni::objects::JValueGen::from(&val_4),jni::objects::JValueGen::from(&val_5)]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn generate_caves(
        &mut self,
        arg0: impl Into<crate::generator::WorldInfo<'mc>>,
        arg1: impl Into<blackboxmc_java::JavaRandom<'mc>>,
        arg2: i32,
        arg3: i32,
        arg4: impl Into<crate::generator::ChunkGeneratorChunkData<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let val_2 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone()) };
        let val_3 = jni::objects::JValueGen::Int(arg2.into());
        let val_4 = jni::objects::JValueGen::Int(arg3.into());
        let val_5 = unsafe { jni::objects::JObject::from_raw(arg4.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(&self.jni_object(),"generateCaves","(Lorg/bukkit/generator/WorldInfo;Ljava/util/Random;IILorg/bukkit/generator/ChunkGenerator$ChunkData;)V",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2),jni::objects::JValueGen::from(&val_3),jni::objects::JValueGen::from(&val_4),jni::objects::JValueGen::from(&val_5)]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn get_default_biome_provider(
        &mut self,
        arg0: impl Into<crate::generator::WorldInfo<'mc>>,
    ) -> Result<crate::generator::BiomeProvider<'mc>, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getDefaultBiomeProvider",
            "(Lorg/bukkit/generator/WorldInfo;)Lorg/bukkit/generator/BiomeProvider;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::generator::BiomeProvider::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn get_base_height(
        &mut self,
        arg0: impl Into<crate::generator::WorldInfo<'mc>>,
        arg1: impl Into<blackboxmc_java::JavaRandom<'mc>>,
        arg2: i32,
        arg3: i32,
        arg4: impl Into<crate::HeightMap<'mc>>,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let val_2 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone()) };
        let val_3 = jni::objects::JValueGen::Int(arg2.into());
        let val_4 = jni::objects::JValueGen::Int(arg3.into());
        let val_5 = unsafe { jni::objects::JObject::from_raw(arg4.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getBaseHeight",
            "(Lorg/bukkit/generator/WorldInfo;Ljava/util/Random;IILorg/bukkit/HeightMap;)I",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
                jni::objects::JValueGen::from(&val_3),
                jni::objects::JValueGen::from(&val_4),
                jni::objects::JValueGen::from(&val_5),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    //

    pub fn generate_chunk_data(
        &mut self,
        arg0: impl Into<crate::World<'mc>>,
        arg1: impl Into<blackboxmc_java::JavaRandom<'mc>>,
        arg2: i32,
        arg3: i32,
        arg4: impl Into<crate::generator::ChunkGeneratorBiomeGrid<'mc>>,
    ) -> Result<crate::generator::ChunkGeneratorChunkData<'mc>, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let val_2 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone()) };
        let val_3 = jni::objects::JValueGen::Int(arg2.into());
        let val_4 = jni::objects::JValueGen::Int(arg3.into());
        let val_5 = unsafe { jni::objects::JObject::from_raw(arg4.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(&self.jni_object(),"generateChunkData","(Lorg/bukkit/World;Ljava/util/Random;IILorg/bukkit/generator/ChunkGenerator$BiomeGrid;)Lorg/bukkit/generator/ChunkGenerator$ChunkData;",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2),jni::objects::JValueGen::from(&val_3),jni::objects::JValueGen::from(&val_4),jni::objects::JValueGen::from(&val_5)]);
        let res = self.jni_ref().translate_error(res)?;
        crate::generator::ChunkGeneratorChunkData::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn can_spawn(
        &mut self,
        arg0: impl Into<crate::World<'mc>>,
        arg1: i32,
        arg2: i32,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let val_2 = jni::objects::JValueGen::Int(arg1.into());
        let val_3 = jni::objects::JValueGen::Int(arg2.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "canSpawn",
            "(Lorg/bukkit/World;II)Z",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
                jni::objects::JValueGen::from(&val_3),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn get_default_populators(
        &mut self,
        arg0: impl Into<crate::World<'mc>>,
    ) -> Result<Vec<crate::generator::BlockPopulator<'mc>>, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getDefaultPopulators",
            "(Lorg/bukkit/World;)Ljava/util/List;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let mut list = blackboxmc_java::JavaList::from_raw(&self.0, res.l()?)?;
        let size = list.size()?;
        for i in 0..=size {
            let obj = list.get(i)?;
            new_vec.push(crate::generator::BlockPopulator::from_raw(&self.0, obj)?);
        }
        Ok(new_vec)
    }
    //

    pub fn get_fixed_spawn_location(
        &mut self,
        arg0: impl Into<crate::World<'mc>>,
        arg1: impl Into<blackboxmc_java::JavaRandom<'mc>>,
    ) -> Result<crate::Location<'mc>, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let val_2 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getFixedSpawnLocation",
            "(Lorg/bukkit/World;Ljava/util/Random;)Lorg/bukkit/Location;",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::Location::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn is_parallel_capable(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isParallelCapable", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn should_generate_noise(
        &mut self,
        arg0: std::option::Option<impl Into<crate::generator::WorldInfo<'mc>>>,
        arg1: std::option::Option<impl Into<blackboxmc_java::JavaRandom<'mc>>>,
        arg2: std::option::Option<i32>,
        arg3: std::option::Option<i32>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = unsafe {
            jni::objects::JObject::from_raw(
                arg0.ok_or(eyre::eyre!("None arguments aren't actually supported yet"))?
                    .into()
                    .jni_object()
                    .clone(),
            )
        };
        let val_2 = unsafe {
            jni::objects::JObject::from_raw(
                arg1.ok_or(eyre::eyre!("None arguments aren't actually supported yet"))?
                    .into()
                    .jni_object()
                    .clone(),
            )
        };
        let val_3 = jni::objects::JValueGen::Int(
            arg2.ok_or(eyre::eyre!("None arguments aren't actually supported yet"))?
                .into(),
        );
        let val_4 = jni::objects::JValueGen::Int(
            arg3.ok_or(eyre::eyre!("None arguments aren't actually supported yet"))?
                .into(),
        );
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "shouldGenerateNoise",
            "(Lorg/bukkit/generator/WorldInfo;Ljava/util/Random;II)Z",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
                jni::objects::JValueGen::from(&val_3),
                jni::objects::JValueGen::from(&val_4),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn should_generate_surface(
        &mut self,
        arg0: std::option::Option<impl Into<crate::generator::WorldInfo<'mc>>>,
        arg1: std::option::Option<impl Into<blackboxmc_java::JavaRandom<'mc>>>,
        arg2: std::option::Option<i32>,
        arg3: std::option::Option<i32>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = unsafe {
            jni::objects::JObject::from_raw(
                arg0.ok_or(eyre::eyre!("None arguments aren't actually supported yet"))?
                    .into()
                    .jni_object()
                    .clone(),
            )
        };
        let val_2 = unsafe {
            jni::objects::JObject::from_raw(
                arg1.ok_or(eyre::eyre!("None arguments aren't actually supported yet"))?
                    .into()
                    .jni_object()
                    .clone(),
            )
        };
        let val_3 = jni::objects::JValueGen::Int(
            arg2.ok_or(eyre::eyre!("None arguments aren't actually supported yet"))?
                .into(),
        );
        let val_4 = jni::objects::JValueGen::Int(
            arg3.ok_or(eyre::eyre!("None arguments aren't actually supported yet"))?
                .into(),
        );
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "shouldGenerateSurface",
            "(Lorg/bukkit/generator/WorldInfo;Ljava/util/Random;II)Z",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
                jni::objects::JValueGen::from(&val_3),
                jni::objects::JValueGen::from(&val_4),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn should_generate_bedrock(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "shouldGenerateBedrock", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn should_generate_caves(
        &mut self,
        arg0: std::option::Option<impl Into<crate::generator::WorldInfo<'mc>>>,
        arg1: std::option::Option<impl Into<blackboxmc_java::JavaRandom<'mc>>>,
        arg2: std::option::Option<i32>,
        arg3: std::option::Option<i32>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = unsafe {
            jni::objects::JObject::from_raw(
                arg0.ok_or(eyre::eyre!("None arguments aren't actually supported yet"))?
                    .into()
                    .jni_object()
                    .clone(),
            )
        };
        let val_2 = unsafe {
            jni::objects::JObject::from_raw(
                arg1.ok_or(eyre::eyre!("None arguments aren't actually supported yet"))?
                    .into()
                    .jni_object()
                    .clone(),
            )
        };
        let val_3 = jni::objects::JValueGen::Int(
            arg2.ok_or(eyre::eyre!("None arguments aren't actually supported yet"))?
                .into(),
        );
        let val_4 = jni::objects::JValueGen::Int(
            arg3.ok_or(eyre::eyre!("None arguments aren't actually supported yet"))?
                .into(),
        );
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "shouldGenerateCaves",
            "(Lorg/bukkit/generator/WorldInfo;Ljava/util/Random;II)Z",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
                jni::objects::JValueGen::from(&val_3),
                jni::objects::JValueGen::from(&val_4),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn should_generate_decorations(
        &mut self,
        arg0: std::option::Option<impl Into<crate::generator::WorldInfo<'mc>>>,
        arg1: std::option::Option<impl Into<blackboxmc_java::JavaRandom<'mc>>>,
        arg2: std::option::Option<i32>,
        arg3: std::option::Option<i32>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = unsafe {
            jni::objects::JObject::from_raw(
                arg0.ok_or(eyre::eyre!("None arguments aren't actually supported yet"))?
                    .into()
                    .jni_object()
                    .clone(),
            )
        };
        let val_2 = unsafe {
            jni::objects::JObject::from_raw(
                arg1.ok_or(eyre::eyre!("None arguments aren't actually supported yet"))?
                    .into()
                    .jni_object()
                    .clone(),
            )
        };
        let val_3 = jni::objects::JValueGen::Int(
            arg2.ok_or(eyre::eyre!("None arguments aren't actually supported yet"))?
                .into(),
        );
        let val_4 = jni::objects::JValueGen::Int(
            arg3.ok_or(eyre::eyre!("None arguments aren't actually supported yet"))?
                .into(),
        );
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "shouldGenerateDecorations",
            "(Lorg/bukkit/generator/WorldInfo;Ljava/util/Random;II)Z",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
                jni::objects::JValueGen::from(&val_3),
                jni::objects::JValueGen::from(&val_4),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn should_generate_mobs(
        &mut self,
        arg0: std::option::Option<impl Into<crate::generator::WorldInfo<'mc>>>,
        arg1: std::option::Option<impl Into<blackboxmc_java::JavaRandom<'mc>>>,
        arg2: std::option::Option<i32>,
        arg3: std::option::Option<i32>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = unsafe {
            jni::objects::JObject::from_raw(
                arg0.ok_or(eyre::eyre!("None arguments aren't actually supported yet"))?
                    .into()
                    .jni_object()
                    .clone(),
            )
        };
        let val_2 = unsafe {
            jni::objects::JObject::from_raw(
                arg1.ok_or(eyre::eyre!("None arguments aren't actually supported yet"))?
                    .into()
                    .jni_object()
                    .clone(),
            )
        };
        let val_3 = jni::objects::JValueGen::Int(
            arg2.ok_or(eyre::eyre!("None arguments aren't actually supported yet"))?
                .into(),
        );
        let val_4 = jni::objects::JValueGen::Int(
            arg3.ok_or(eyre::eyre!("None arguments aren't actually supported yet"))?
                .into(),
        );
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "shouldGenerateMobs",
            "(Lorg/bukkit/generator/WorldInfo;Ljava/util/Random;II)Z",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
                jni::objects::JValueGen::from(&val_3),
                jni::objects::JValueGen::from(&val_4),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn should_generate_structures(
        &mut self,
        arg0: std::option::Option<impl Into<crate::generator::WorldInfo<'mc>>>,
        arg1: std::option::Option<impl Into<blackboxmc_java::JavaRandom<'mc>>>,
        arg2: std::option::Option<i32>,
        arg3: std::option::Option<i32>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = unsafe {
            jni::objects::JObject::from_raw(
                arg0.ok_or(eyre::eyre!("None arguments aren't actually supported yet"))?
                    .into()
                    .jni_object()
                    .clone(),
            )
        };
        let val_2 = unsafe {
            jni::objects::JObject::from_raw(
                arg1.ok_or(eyre::eyre!("None arguments aren't actually supported yet"))?
                    .into()
                    .jni_object()
                    .clone(),
            )
        };
        let val_3 = jni::objects::JValueGen::Int(
            arg2.ok_or(eyre::eyre!("None arguments aren't actually supported yet"))?
                .into(),
        );
        let val_4 = jni::objects::JValueGen::Int(
            arg3.ok_or(eyre::eyre!("None arguments aren't actually supported yet"))?
                .into(),
        );
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "shouldGenerateStructures",
            "(Lorg/bukkit/generator/WorldInfo;Ljava/util/Random;II)Z",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
                jni::objects::JValueGen::from(&val_3),
                jni::objects::JValueGen::from(&val_4),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn wait(
        &mut self,
        arg0: std::option::Option<i64>,
        arg1: std::option::Option<i32>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Long(
            arg0.ok_or(eyre::eyre!("None arguments aren't actually supported yet"))?
                .into(),
        );
        let val_2 = jni::objects::JValueGen::Int(
            arg1.ok_or(eyre::eyre!("None arguments aren't actually supported yet"))?
                .into(),
        );
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
    //

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
        Ok(res.z()?)
    }
    //

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
    //

    pub fn hash_code(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hashCode", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    //

    pub fn class(&mut self) -> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getClass", "()Ljava/lang/Class;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(unsafe { jni::objects::JClass::from_raw(res.as_jni().l) })
    }
    //

    pub fn notify(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notify", "()V", &[]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn notify_all(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notifyAll", "()V", &[]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
}
pub mod structure;
