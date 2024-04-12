#![allow(deprecated)]
use blackboxmc_general::JNIRaw;
use blackboxmc_general::JNIInstantiatable;
use color_eyre::eyre::Result;/*org/bukkit/inventory/meta/mod.rs*/

#[repr(C)]
pub struct MusicInstrumentMeta<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for MusicInstrumentMeta<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for MusicInstrumentMeta<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate MusicInstrumentMeta from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/inventory/meta/MusicInstrumentMeta")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a MusicInstrumentMeta object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> MusicInstrumentMetaTrait<'mc> for MusicInstrumentMeta<'mc> {}
pub trait MusicInstrumentMetaTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {
/// Sets the goat horn's instrument.
	fn set_instrument(&self,instrument: impl Into<crate::MusicInstrument<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/MusicInstrument;)V");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(instrument.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"setInstrument",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
/// Gets the instrument of the goat horn.
	fn instrument(&self) 
-> Result<Option<crate::MusicInstrument<'mc>>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/MusicInstrument;");
let res = self.jni_ref().call_method(&self.jni_object(),"getInstrument",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {return Ok(None);}
Ok(
Some(
crate::MusicInstrument::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)?
)
)}

	fn clone(&self) 
-> Result<crate::inventory::meta::MusicInstrumentMeta<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/inventory/meta/MusicInstrumentMeta;");
let res = self.jni_ref().call_method(&self.jni_object(),"clone",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::inventory::meta::MusicInstrumentMeta::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
impl<'mc> Into<crate::inventory::meta::ItemMeta<'mc>> for MusicInstrumentMeta<'mc>{

fn into(self) -> crate::inventory::meta::ItemMeta<'mc> {

crate::inventory::meta::ItemMeta::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting MusicInstrumentMeta into crate::inventory::meta::ItemMeta")

   }
}
impl<'mc> crate::inventory::meta::ItemMetaTrait<'mc> for MusicInstrumentMeta<'mc> {}
#[repr(C)]
pub struct SpawnEggMeta<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for SpawnEggMeta<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for SpawnEggMeta<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate SpawnEggMeta from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/inventory/meta/SpawnEggMeta")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a SpawnEggMeta object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> SpawnEggMetaTrait<'mc> for SpawnEggMeta<'mc> {}
pub trait SpawnEggMetaTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {
#[deprecated]
/// Get the type of entity this egg will spawn.
	fn spawned_type(&self) 
-> Result<crate::entity::EntityType<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/entity/EntityType;");
let res = self.jni_ref().call_method(&self.jni_object(),"getSpawnedType",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::entity::EntityType::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
#[deprecated]
/// Set the type of entity this egg will spawn.
	fn set_spawned_type(&self,val_type: impl Into<crate::entity::EntityType<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/entity/EntityType;)V");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(val_type.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"setSpawnedType",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
/// Gets the {@link EntitySnapshot} that will be spawned by this spawn egg or null if no entity
/// has been set.
/// 
/// 
/// All applicable data from the egg will be copied, such as custom name, health,
/// and velocity.
/// 
	fn spawned_entity(&self) 
-> Result<Option<crate::entity::EntitySnapshot<'mc>>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/entity/EntitySnapshot;");
let res = self.jni_ref().call_method(&self.jni_object(),"getSpawnedEntity",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {return Ok(None);}
Ok(
Some(
crate::entity::EntitySnapshot::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)?
)
)}
/// Sets the {@link EntitySnapshot} that will be spawned by this spawn egg.
/// 
/// 
/// All applicable data from the entity will be copied, such as custom name,
/// health, and velocity.
/// 
	fn set_spawned_entity(&self,snapshot: impl Into<crate::entity::EntitySnapshot<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/entity/EntitySnapshot;)V");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(snapshot.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"setSpawnedEntity",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}

	fn clone(&self) 
-> Result<crate::inventory::meta::SpawnEggMeta<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/inventory/meta/SpawnEggMeta;");
let res = self.jni_ref().call_method(&self.jni_object(),"clone",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::inventory::meta::SpawnEggMeta::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
impl<'mc> Into<crate::inventory::meta::ItemMeta<'mc>> for SpawnEggMeta<'mc>{

fn into(self) -> crate::inventory::meta::ItemMeta<'mc> {

crate::inventory::meta::ItemMeta::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting SpawnEggMeta into crate::inventory::meta::ItemMeta")

   }
}
impl<'mc> crate::inventory::meta::ItemMetaTrait<'mc> for SpawnEggMeta<'mc> {}
#[repr(C)]
pub struct ColorableArmorMeta<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for ColorableArmorMeta<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for ColorableArmorMeta<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate ColorableArmorMeta from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/inventory/meta/ColorableArmorMeta")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a ColorableArmorMeta object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> ColorableArmorMetaTrait<'mc> for ColorableArmorMeta<'mc> {}
pub trait ColorableArmorMetaTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {

	fn clone(&self) 
-> Result<crate::inventory::meta::ColorableArmorMeta<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/inventory/meta/ColorableArmorMeta;");
let res = self.jni_ref().call_method(&self.jni_object(),"clone",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::inventory::meta::ColorableArmorMeta::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
impl<'mc> Into<crate::inventory::meta::ArmorMeta<'mc>> for ColorableArmorMeta<'mc>{

fn into(self) -> crate::inventory::meta::ArmorMeta<'mc> {

crate::inventory::meta::ArmorMeta::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting ColorableArmorMeta into crate::inventory::meta::ArmorMeta")

   }
}
impl<'mc> crate::inventory::meta::ArmorMetaTrait<'mc> for ColorableArmorMeta<'mc> {}
impl<'mc> Into<crate::inventory::meta::LeatherArmorMeta<'mc>> for ColorableArmorMeta<'mc>{

fn into(self) -> crate::inventory::meta::LeatherArmorMeta<'mc> {

crate::inventory::meta::LeatherArmorMeta::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting ColorableArmorMeta into crate::inventory::meta::LeatherArmorMeta")

   }
}
impl<'mc> crate::inventory::meta::LeatherArmorMetaTrait<'mc> for ColorableArmorMeta<'mc> {}
pub enum BookMetaGeneration<'mc> {
	Original {inner: BookMetaGenerationStruct<'mc>},
	CopyOfOriginal {inner: BookMetaGenerationStruct<'mc>},
	CopyOfCopy {inner: BookMetaGenerationStruct<'mc>},
	Tattered {inner: BookMetaGenerationStruct<'mc>},
}
impl<'mc> std::fmt::Display for BookMetaGeneration<'mc> {
   fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
       match self {
           BookMetaGeneration::Original { .. } => f.write_str("ORIGINAL"),
           BookMetaGeneration::CopyOfOriginal { .. } => f.write_str("COPY_OF_ORIGINAL"),
           BookMetaGeneration::CopyOfCopy { .. } => f.write_str("COPY_OF_COPY"),
           BookMetaGeneration::Tattered { .. } => f.write_str("TATTERED"),
       }
   }
}

        impl<'mc> BookMetaGenerationTrait<'mc> for BookMetaGeneration<'mc> {}
        
        pub trait BookMetaGenerationTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc>  {
            fn value_of(
                env: &blackboxmc_general::SharedJNIEnv<'mc>,
                arg0: impl Into<String>,
            ) -> Result<BookMetaGeneration<'mc>, Box<dyn std::error::Error>> {
                let val_1 = jni::objects::JObject::from(env.new_string(arg0.into())?);
                let cls = env.find_class("org/bukkit/inventory/meta/BookMeta/Generation");
                let cls = env.translate_error_with_class(cls)?;
                let res = env.call_static_method(
                    cls,
                    "valueOf",
                    "(Ljava/lang/String;)Lorg/bukkit/inventory/meta/BookMeta/Generation;",
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
                    
"ORIGINAL" => Ok(BookMetaGeneration::Original { inner: BookMetaGenerationStruct::from_raw(env,obj)?}),
"COPY_OF_ORIGINAL" => Ok(BookMetaGeneration::CopyOfOriginal { inner: BookMetaGenerationStruct::from_raw(env,obj)?}),
"COPY_OF_COPY" => Ok(BookMetaGeneration::CopyOfCopy { inner: BookMetaGenerationStruct::from_raw(env,obj)?}),
"TATTERED" => Ok(BookMetaGeneration::Tattered { inner: BookMetaGenerationStruct::from_raw(env,obj)?}),

                    _ => Err(eyre::eyre!("String gaven for variant was invalid").into())
                }
            }
        }
        
#[repr(C)]
pub struct BookMetaGenerationStruct<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for BookMetaGeneration<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
match self {
Self::Original { inner } => inner.0.clone(),
Self::CopyOfOriginal { inner } => inner.0.clone(),
Self::CopyOfCopy { inner } => inner.0.clone(),
Self::Tattered { inner } => inner.0.clone(),
}
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
match self {
Self::Original { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::CopyOfOriginal { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::CopyOfCopy { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::Tattered { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
}
}
}
impl<'mc> JNIInstantiatable<'mc> for BookMetaGeneration<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate BookMetaGeneration from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/inventory/meta/BookMeta/Generation")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a BookMetaGeneration object, got {}",
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
                    "ORIGINAL" => Ok(BookMetaGeneration::Original { inner: BookMetaGenerationStruct::from_raw(env,obj)?}),"COPY_OF_ORIGINAL" => Ok(BookMetaGeneration::CopyOfOriginal { inner: BookMetaGenerationStruct::from_raw(env,obj)?}),"COPY_OF_COPY" => Ok(BookMetaGeneration::CopyOfCopy { inner: BookMetaGenerationStruct::from_raw(env,obj)?}),"TATTERED" => Ok(BookMetaGeneration::Tattered { inner: BookMetaGenerationStruct::from_raw(env,obj)?}),_ => Err(eyre::eyre!("String gaven for variant was invalid").into())}
            }
        }
    }
    

    impl<'mc> JNIRaw<'mc> for BookMetaGenerationStruct<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for BookMetaGenerationStruct<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate BookMetaGenerationStruct from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/inventory/meta/BookMeta/Generation")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a BookMetaGenerationStruct object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> BookMetaGenerationStruct<'mc> {

	fn values(jni: &blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::inventory::meta::BookMetaGeneration<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/inventory/meta/BookMeta/Generation;");
let cls = jni.find_class("org/bukkit/inventory/meta/BookMeta/Generation"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.call_static_method(cls,"values",
sig.as_str(),vec![]);
let res = 
jni.translate_error(res)?;
let obj = res.l()?;
crate::inventory::meta::BookMetaGeneration::from_raw(&jni,obj
)}

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
#[repr(C)]
pub struct BlockStateMeta<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for BlockStateMeta<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for BlockStateMeta<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate BlockStateMeta from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/inventory/meta/BlockStateMeta")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a BlockStateMeta object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> BlockStateMetaTrait<'mc> for BlockStateMeta<'mc> {}
pub trait BlockStateMetaTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {
/// Returns whether the item has a block state currently
/// attached to it.
	fn has_block_state(&self) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("()Z");
let res = self.jni_ref().call_method(&self.jni_object(),"hasBlockState",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
/// Returns the currently attached block state for this
/// item or creates a new one if one doesn't exist.
/// The state is a copy, it must be set back (or to another
/// item) with {@link #setBlockState(org.bukkit.block.BlockState)}
	fn block_state(&self) 
-> Result<crate::block::BlockState<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/block/BlockState;");
let res = self.jni_ref().call_method(&self.jni_object(),"getBlockState",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::block::BlockState::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
/// Attaches a copy of the passed block state to the item.
	fn set_block_state(&self,block_state: impl Into<crate::block::BlockState<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/block/BlockState;)V");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(block_state.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"setBlockState",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
impl<'mc> Into<crate::inventory::meta::ItemMeta<'mc>> for BlockStateMeta<'mc>{

fn into(self) -> crate::inventory::meta::ItemMeta<'mc> {

crate::inventory::meta::ItemMeta::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting BlockStateMeta into crate::inventory::meta::ItemMeta")

   }
}
impl<'mc> crate::inventory::meta::ItemMetaTrait<'mc> for BlockStateMeta<'mc> {}
#[repr(C)]
pub struct BlockDataMeta<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for BlockDataMeta<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for BlockDataMeta<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate BlockDataMeta from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/inventory/meta/BlockDataMeta")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a BlockDataMeta object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> BlockDataMetaTrait<'mc> for BlockDataMeta<'mc> {}
pub trait BlockDataMetaTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {
/// Returns whether the item has block data currently attached to it.
	fn has_block_data(&self) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("()Z");
let res = self.jni_ref().call_method(&self.jni_object(),"hasBlockData",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
/// Returns the currently attached block data for this item or creates a new
/// one if one doesn't exist.
/// The state is a copy, it must be set back (or to another item) with
/// {@link #setBlockData(org.bukkit.block.data.BlockData)}
	fn get_block_data(&self,material: impl Into<crate::Material<'mc>>) 
-> Result<crate::block::data::BlockData<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/Material;)Lorg/bukkit/block/data/BlockData;");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(material.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"getBlockData",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
crate::block::data::BlockData::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
/// Attaches a copy of the passed block data to the item.
	fn set_block_data(&self,block_data: impl Into<crate::block::data::BlockData<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/block/data/BlockData;)V");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(block_data.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"setBlockData",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
impl<'mc> Into<crate::inventory::meta::ItemMeta<'mc>> for BlockDataMeta<'mc>{

fn into(self) -> crate::inventory::meta::ItemMeta<'mc> {

crate::inventory::meta::ItemMeta::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting BlockDataMeta into crate::inventory::meta::ItemMeta")

   }
}
impl<'mc> crate::inventory::meta::ItemMetaTrait<'mc> for BlockDataMeta<'mc> {}
#[repr(C)]
pub struct PotionMeta<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for PotionMeta<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for PotionMeta<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate PotionMeta from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/inventory/meta/PotionMeta")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a PotionMeta object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> PotionMetaTrait<'mc> for PotionMeta<'mc> {}
pub trait PotionMetaTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {
#[deprecated]
/// Sets the underlying potion data
	fn set_base_potion_data(&self,data: impl Into<crate::potion::PotionData<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/potion/PotionData;)V");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(data.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"setBasePotionData",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
#[deprecated]
/// Returns the potion data about the base potion
	fn base_potion_data(&self) 
-> Result<crate::potion::PotionData<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/potion/PotionData;");
let res = self.jni_ref().call_method(&self.jni_object(),"getBasePotionData",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::potion::PotionData::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
/// Sets the underlying potion type
	fn set_base_potion_type(&self,val_type: impl Into<crate::potion::PotionType<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/potion/PotionType;)V");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(val_type.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"setBasePotionType",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
/// Returns the potion type about the base potion
	fn base_potion_type(&self) 
-> Result<crate::potion::PotionType<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/potion/PotionType;");
let res = self.jni_ref().call_method(&self.jni_object(),"getBasePotionType",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::potion::PotionType::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
/// Checks for the presence of custom potion effects.
	fn has_custom_effects(&self) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("()Z");
let res = self.jni_ref().call_method(&self.jni_object(),"hasCustomEffects",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
/// Gets an immutable list containing all custom potion effects applied to
/// this potion.
/// 
/// Plugins should check that hasCustomEffects() returns true before calling
/// this method.
	fn custom_effects(&self) 
-> Result<Vec<crate::potion::PotionEffect<'mc>>, Box<dyn std::error::Error>>

{let sig = String::from("()Ljava/util/List;");
let res = self.jni_ref().call_method(&self.jni_object(),"getCustomEffects",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
let mut new_vec = Vec::new();
let list = blackboxmc_java::util::JavaList::from_raw(&self.jni_ref(), res.l()?)?;let iter = list.iterator()?;
while iter.has_next()? {            let obj = iter.next()?;
new_vec.push(crate::potion::PotionEffect::from_raw(&self.jni_ref(),obj,)?);
};
Ok(
new_vec
)}
/// Adds a custom potion effect to this potion.
	fn add_custom_effect(&self,effect: impl Into<crate::potion::PotionEffect<'mc>>,overwrite: bool) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/potion/PotionEffect;Z)Z");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(effect.into().jni_object().clone())});
let val_2 = jni::objects::JValueGen::Bool(overwrite.into());
let res = self.jni_ref().call_method(&self.jni_object(),"addCustomEffect",sig.as_str(),vec![jni::objects::JValueGen::from(val_1),jni::objects::JValueGen::from(val_2)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
/// Removes a custom potion effect from this potion.
	fn remove_custom_effect(&self,val_type: impl Into<crate::potion::PotionEffectType<'mc>>) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/potion/PotionEffectType;)Z");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(val_type.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"removeCustomEffect",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
/// Checks for a specific custom potion effect type on this potion.
	fn has_custom_effect(&self,val_type: impl Into<crate::potion::PotionEffectType<'mc>>) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/potion/PotionEffectType;)Z");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(val_type.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"hasCustomEffect",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
#[deprecated]
/// Moves a potion effect to the top of the potion effect list.This causes the client to display the potion effect in the potion's name.
	fn set_main_effect(&self,val_type: impl Into<crate::potion::PotionEffectType<'mc>>) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/potion/PotionEffectType;)Z");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(val_type.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"setMainEffect",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
/// Removes all custom potion effects from this potion.
	fn clear_custom_effects(&self) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("()Z");
let res = self.jni_ref().call_method(&self.jni_object(),"clearCustomEffects",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
/// Checks for existence of a potion color.
	fn has_color(&self) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("()Z");
let res = self.jni_ref().call_method(&self.jni_object(),"hasColor",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
/// Gets the potion color that is set. A custom potion color will alter the
/// display of the potion in an inventory slot.
/// 
/// Plugins should check that hasColor() returns <code>true</code> before
/// calling this method.
	fn color(&self) 
-> Result<Option<crate::Color<'mc>>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/Color;");
let res = self.jni_ref().call_method(&self.jni_object(),"getColor",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {return Ok(None);}
Ok(
Some(
crate::Color::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)?
)
)}
/// Sets the potion color. A custom potion color will alter the display of
/// the potion in an inventory slot.
	fn set_color(&self,color: impl Into<crate::Color<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/Color;)V");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(color.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"setColor",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}

	fn clone(&self) 
-> Result<crate::inventory::meta::PotionMeta<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/inventory/meta/PotionMeta;");
let res = self.jni_ref().call_method(&self.jni_object(),"clone",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::inventory::meta::PotionMeta::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
impl<'mc> Into<crate::inventory::meta::ItemMeta<'mc>> for PotionMeta<'mc>{

fn into(self) -> crate::inventory::meta::ItemMeta<'mc> {

crate::inventory::meta::ItemMeta::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting PotionMeta into crate::inventory::meta::ItemMeta")

   }
}
impl<'mc> crate::inventory::meta::ItemMetaTrait<'mc> for PotionMeta<'mc> {}
#[repr(C)]
pub struct Repairable<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for Repairable<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for Repairable<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate Repairable from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/inventory/meta/Repairable")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a Repairable object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> RepairableTrait<'mc> for Repairable<'mc> {}
pub trait RepairableTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {
/// Checks to see if this has a repair penalty
	fn has_repair_cost(&self) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("()Z");
let res = self.jni_ref().call_method(&self.jni_object(),"hasRepairCost",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
/// Gets the repair penalty
	fn repair_cost(&self) 
-> Result<i32, Box<dyn std::error::Error>>

{let sig = String::from("()I");
let res = self.jni_ref().call_method(&self.jni_object(),"getRepairCost",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.i()?
)}
/// Sets the repair penalty
	fn set_repair_cost(&self,cost: i32) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(I)V");
let val_1 = jni::objects::JValueGen::Int(cost);
let res = self.jni_ref().call_method(&self.jni_object(),"setRepairCost",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}

	fn clone(&self) 
-> Result<crate::inventory::meta::Repairable<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/inventory/meta/Repairable;");
let res = self.jni_ref().call_method(&self.jni_object(),"clone",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::inventory::meta::Repairable::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
impl<'mc> Into<crate::inventory::meta::ItemMeta<'mc>> for Repairable<'mc>{

fn into(self) -> crate::inventory::meta::ItemMeta<'mc> {

crate::inventory::meta::ItemMeta::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting Repairable into crate::inventory::meta::ItemMeta")

   }
}
impl<'mc> crate::inventory::meta::ItemMetaTrait<'mc> for Repairable<'mc> {}
#[repr(C)]
pub struct CrossbowMeta<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for CrossbowMeta<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for CrossbowMeta<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate CrossbowMeta from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/inventory/meta/CrossbowMeta")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a CrossbowMeta object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> CrossbowMetaTrait<'mc> for CrossbowMeta<'mc> {}
pub trait CrossbowMetaTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {
/// Returns whether the item has any charged projectiles.
	fn has_charged_projectiles(&self) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("()Z");
let res = self.jni_ref().call_method(&self.jni_object(),"hasChargedProjectiles",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
/// Returns an immutable list of the projectiles charged on this item.
	fn charged_projectiles(&self) 
-> Result<Vec<crate::inventory::ItemStack<'mc>>, Box<dyn std::error::Error>>

{let sig = String::from("()Ljava/util/List;");
let res = self.jni_ref().call_method(&self.jni_object(),"getChargedProjectiles",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
let mut new_vec = Vec::new();
let list = blackboxmc_java::util::JavaList::from_raw(&self.jni_ref(), res.l()?)?;let iter = list.iterator()?;
while iter.has_next()? {            let obj = iter.next()?;
new_vec.push(crate::inventory::ItemStack::from_raw(&self.jni_ref(),obj,)?);
};
Ok(
new_vec
)}
/// Sets the projectiles charged on this item.
/// Removes all projectiles when given null.
	fn set_charged_projectiles(&self,projectiles: Vec<jni::objects::JObject<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Ljava/util/List;)V");
let raw_val_1 = self.jni_ref().new_object("java/util/ArrayList", "()V", vec![])?;
for v in projectiles{
let map_val_0 = jni::objects::JValueGen::Object(v);
self.jni_ref().call_method(&raw_val_1,"add","(Ljava/lang/Object;)Z",vec![jni::objects::JValueGen::from(map_val_0)])?;
};
let val_1 = jni::objects::JValueGen::Object(raw_val_1);
let res = self.jni_ref().call_method(&self.jni_object(),"setChargedProjectiles",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
/// Adds a charged projectile to this item.
	fn add_charged_projectile(&self,item: impl Into<crate::inventory::ItemStack<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/inventory/ItemStack;)V");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(item.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"addChargedProjectile",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
impl<'mc> Into<crate::inventory::meta::ItemMeta<'mc>> for CrossbowMeta<'mc>{

fn into(self) -> crate::inventory::meta::ItemMeta<'mc> {

crate::inventory::meta::ItemMeta::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting CrossbowMeta into crate::inventory::meta::ItemMeta")

   }
}
impl<'mc> crate::inventory::meta::ItemMetaTrait<'mc> for CrossbowMeta<'mc> {}
#[repr(C)]
pub struct ArmorMeta<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for ArmorMeta<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for ArmorMeta<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate ArmorMeta from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/inventory/meta/ArmorMeta")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a ArmorMeta object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> ArmorMetaTrait<'mc> for ArmorMeta<'mc> {}
pub trait ArmorMetaTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {
/// Check whether or not this item has an armor trim.
	fn has_trim(&self) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("()Z");
let res = self.jni_ref().call_method(&self.jni_object(),"hasTrim",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
/// Set the {@link ArmorTrim}.
	fn set_trim(&self,trim: impl Into<crate::inventory::meta::trim::ArmorTrim<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/inventory/meta/trim/ArmorTrim;)V");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(trim.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"setTrim",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
/// Get the {@link ArmorTrim}.
	fn trim(&self) 
-> Result<Option<crate::inventory::meta::trim::ArmorTrim<'mc>>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/inventory/meta/trim/ArmorTrim;");
let res = self.jni_ref().call_method(&self.jni_object(),"getTrim",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {return Ok(None);}
Ok(
Some(
crate::inventory::meta::trim::ArmorTrim::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)?
)
)}

	fn clone(&self) 
-> Result<crate::inventory::meta::ArmorMeta<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/inventory/meta/ArmorMeta;");
let res = self.jni_ref().call_method(&self.jni_object(),"clone",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::inventory::meta::ArmorMeta::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
impl<'mc> Into<crate::inventory::meta::ItemMeta<'mc>> for ArmorMeta<'mc>{

fn into(self) -> crate::inventory::meta::ItemMeta<'mc> {

crate::inventory::meta::ItemMeta::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting ArmorMeta into crate::inventory::meta::ItemMeta")

   }
}
impl<'mc> crate::inventory::meta::ItemMetaTrait<'mc> for ArmorMeta<'mc> {}
#[repr(C)]
pub struct FireworkMeta<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for FireworkMeta<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for FireworkMeta<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate FireworkMeta from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/inventory/meta/FireworkMeta")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a FireworkMeta object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> FireworkMetaTrait<'mc> for FireworkMeta<'mc> {}
pub trait FireworkMetaTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {
/// Add another effect to this firework.
	fn add_effect(&self,effect: impl Into<crate::FireworkEffect<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/FireworkEffect;)V");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(effect.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"addEffect",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
/// Get the effects in this firework.
	fn effects(&self) 
-> Result<Vec<crate::FireworkEffect<'mc>>, Box<dyn std::error::Error>>

{let sig = String::from("()Ljava/util/List;");
let res = self.jni_ref().call_method(&self.jni_object(),"getEffects",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
let mut new_vec = Vec::new();
let list = blackboxmc_java::util::JavaList::from_raw(&self.jni_ref(), res.l()?)?;let iter = list.iterator()?;
while iter.has_next()? {            let obj = iter.next()?;
new_vec.push(crate::FireworkEffect::from_raw(&self.jni_ref(),obj,)?);
};
Ok(
new_vec
)}
/// Get the number of effects in this firework.
	fn effects_size(&self) 
-> Result<i32, Box<dyn std::error::Error>>

{let sig = String::from("()I");
let res = self.jni_ref().call_method(&self.jni_object(),"getEffectsSize",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.i()?
)}
/// Remove an effect from this firework.
	fn remove_effect(&self,index: i32) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(I)V");
let val_1 = jni::objects::JValueGen::Int(index);
let res = self.jni_ref().call_method(&self.jni_object(),"removeEffect",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
/// Remove all effects from this firework.
	fn clear_effects(&self) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("()V");
let res = self.jni_ref().call_method(&self.jni_object(),"clearEffects",sig.as_str(),vec![]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
/// Get whether this firework has any effects.
	fn has_effects(&self) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("()Z");
let res = self.jni_ref().call_method(&self.jni_object(),"hasEffects",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
/// Gets the approximate height the firework will fly.
	fn power(&self) 
-> Result<i32, Box<dyn std::error::Error>>

{let sig = String::from("()I");
let res = self.jni_ref().call_method(&self.jni_object(),"getPower",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.i()?
)}
/// Sets the approximate power of the firework. Each level of power is half
/// a second of flight time.
	fn set_power(&self,power: i32) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(I)V");
let val_1 = jni::objects::JValueGen::Int(power);
let res = self.jni_ref().call_method(&self.jni_object(),"setPower",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}

	fn clone(&self) 
-> Result<crate::inventory::meta::FireworkMeta<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/inventory/meta/FireworkMeta;");
let res = self.jni_ref().call_method(&self.jni_object(),"clone",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::inventory::meta::FireworkMeta::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
impl<'mc> Into<crate::inventory::meta::ItemMeta<'mc>> for FireworkMeta<'mc>{

fn into(self) -> crate::inventory::meta::ItemMeta<'mc> {

crate::inventory::meta::ItemMeta::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting FireworkMeta into crate::inventory::meta::ItemMeta")

   }
}
impl<'mc> crate::inventory::meta::ItemMetaTrait<'mc> for FireworkMeta<'mc> {}
#[repr(C)]
pub struct MapMeta<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for MapMeta<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for MapMeta<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate MapMeta from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/inventory/meta/MapMeta")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a MapMeta object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> MapMetaTrait<'mc> for MapMeta<'mc> {}
pub trait MapMetaTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {
#[deprecated]
/// Checks for existence of a map ID number.
	fn has_map_id(&self) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("()Z");
let res = self.jni_ref().call_method(&self.jni_object(),"hasMapId",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
#[deprecated]
/// Gets the map ID that is set. This is used to determine what map is displayed.Plugins should check that hasMapId() returns <code>true</code> before calling this method.
	fn map_id(&self) 
-> Result<i32, Box<dyn std::error::Error>>

{let sig = String::from("()I");
let res = self.jni_ref().call_method(&self.jni_object(),"getMapId",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.i()?
)}
#[deprecated]
/// Sets the map ID. This is used to determine what map is displayed.
	fn set_map_id(&self,id: i32) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(I)V");
let val_1 = jni::objects::JValueGen::Int(id);
let res = self.jni_ref().call_method(&self.jni_object(),"setMapId",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
/// Checks for existence of an associated map.
	fn has_map_view(&self) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("()Z");
let res = self.jni_ref().call_method(&self.jni_object(),"hasMapView",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
/// Gets the map view that is associated with this map item.
/// 
/// Plugins should check that hasMapView() returns <code>true</code> before
/// calling this method.
	fn map_view(&self) 
-> Result<Option<crate::map::MapView<'mc>>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/map/MapView;");
let res = self.jni_ref().call_method(&self.jni_object(),"getMapView",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {return Ok(None);}
Ok(
Some(
crate::map::MapView::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)?
)
)}
/// Sets the associated map. This is used to determine what map is displayed.
/// 
/// The implementation <b>may</b> allow null to clear the associated map, but
/// this is not required and is liable to generate a new (undefined) map when
/// the item is first used.
	fn set_map_view(&self,map: impl Into<crate::map::MapView<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/map/MapView;)V");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(map.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"setMapView",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
/// Checks to see if this map is scaling.
	fn is_scaling(&self) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("()Z");
let res = self.jni_ref().call_method(&self.jni_object(),"isScaling",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
/// Sets if this map is scaling or not.
	fn set_scaling(&self,value: bool) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Z)V");
let val_1 = jni::objects::JValueGen::Bool(value.into());
let res = self.jni_ref().call_method(&self.jni_object(),"setScaling",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
#[deprecated]
/// Checks for existence of a location name.
	fn has_location_name(&self) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("()Z");
let res = self.jni_ref().call_method(&self.jni_object(),"hasLocationName",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
#[deprecated]
/// Gets the location name that is set.Plugins should check that hasLocationName() returns <code>true</code> before calling this method.
	fn location_name(&self) 
-> Result<Option<String>, Box<dyn std::error::Error>>

{let sig = String::from("()Ljava/lang/String;");
let res = self.jni_ref().call_method(&self.jni_object(),"getLocationName",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {return Ok(None);}
Ok(
Some(
self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string()
)
)}
#[deprecated]
/// Sets the location name.
	fn set_location_name(&self,name: impl Into<String>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Ljava/lang/String;)V");
let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(name.into())?));
let res = self.jni_ref().call_method(&self.jni_object(),"setLocationName",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
/// Checks for existence of a map color.
	fn has_color(&self) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("()Z");
let res = self.jni_ref().call_method(&self.jni_object(),"hasColor",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
/// Gets the map color that is set. A custom map color will alter the display
/// of the map in an inventory slot.
/// 
/// Plugins should check that hasColor() returns <code>true</code> before
/// calling this method.
	fn color(&self) 
-> Result<Option<crate::Color<'mc>>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/Color;");
let res = self.jni_ref().call_method(&self.jni_object(),"getColor",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {return Ok(None);}
Ok(
Some(
crate::Color::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)?
)
)}
/// Sets the map color. A custom map color will alter the display of the map
/// in an inventory slot.
	fn set_color(&self,color: impl Into<crate::Color<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/Color;)V");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(color.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"setColor",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}

	fn clone(&self) 
-> Result<crate::inventory::meta::MapMeta<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/inventory/meta/MapMeta;");
let res = self.jni_ref().call_method(&self.jni_object(),"clone",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::inventory::meta::MapMeta::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
impl<'mc> Into<crate::inventory::meta::ItemMeta<'mc>> for MapMeta<'mc>{

fn into(self) -> crate::inventory::meta::ItemMeta<'mc> {

crate::inventory::meta::ItemMeta::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting MapMeta into crate::inventory::meta::ItemMeta")

   }
}
impl<'mc> crate::inventory::meta::ItemMetaTrait<'mc> for MapMeta<'mc> {}
#[repr(C)]
pub struct Damageable<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for Damageable<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for Damageable<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate Damageable from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/inventory/meta/Damageable")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a Damageable object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> DamageableTrait<'mc> for Damageable<'mc> {}
pub trait DamageableTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {
/// Checks to see if this item has damage
	fn has_damage(&self) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("()Z");
let res = self.jni_ref().call_method(&self.jni_object(),"hasDamage",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
/// Gets the damage
	fn damage(&self) 
-> Result<i32, Box<dyn std::error::Error>>

{let sig = String::from("()I");
let res = self.jni_ref().call_method(&self.jni_object(),"getDamage",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.i()?
)}
/// Sets the damage
	fn set_damage(&self,damage: i32) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(I)V");
let val_1 = jni::objects::JValueGen::Int(damage);
let res = self.jni_ref().call_method(&self.jni_object(),"setDamage",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}

	fn clone(&self) 
-> Result<crate::inventory::meta::Damageable<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/inventory/meta/Damageable;");
let res = self.jni_ref().call_method(&self.jni_object(),"clone",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::inventory::meta::Damageable::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
impl<'mc> Into<crate::inventory::meta::ItemMeta<'mc>> for Damageable<'mc>{

fn into(self) -> crate::inventory::meta::ItemMeta<'mc> {

crate::inventory::meta::ItemMeta::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting Damageable into crate::inventory::meta::ItemMeta")

   }
}
impl<'mc> crate::inventory::meta::ItemMetaTrait<'mc> for Damageable<'mc> {}
#[repr(C)]
pub struct BannerMeta<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for BannerMeta<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for BannerMeta<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate BannerMeta from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/inventory/meta/BannerMeta")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a BannerMeta object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> BannerMetaTrait<'mc> for BannerMeta<'mc> {}
pub trait BannerMetaTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {
#[deprecated]
/// Returns the base color for this banner
	fn base_color(&self) 
-> Result<Option<crate::DyeColor<'mc>>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/DyeColor;");
let res = self.jni_ref().call_method(&self.jni_object(),"getBaseColor",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {return Ok(None);}
Ok(
Some(
crate::DyeColor::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)?
)
)}
#[deprecated]
/// Sets the base color for this banner
	fn set_base_color(&self,color: impl Into<crate::DyeColor<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/DyeColor;)V");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(color.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"setBaseColor",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
/// Returns a list of patterns on this banner
	fn patterns(&self) 
-> Result<Vec<crate::block::banner::Pattern<'mc>>, Box<dyn std::error::Error>>

{let sig = String::from("()Ljava/util/List;");
let res = self.jni_ref().call_method(&self.jni_object(),"getPatterns",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
let mut new_vec = Vec::new();
let list = blackboxmc_java::util::JavaList::from_raw(&self.jni_ref(), res.l()?)?;let iter = list.iterator()?;
while iter.has_next()? {            let obj = iter.next()?;
new_vec.push(crate::block::banner::Pattern::from_raw(&self.jni_ref(),obj,)?);
};
Ok(
new_vec
)}
/// Sets the patterns used on this banner
	fn set_patterns(&self,patterns: Vec<jni::objects::JObject<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Ljava/util/List;)V");
let raw_val_1 = self.jni_ref().new_object("java/util/ArrayList", "()V", vec![])?;
for v in patterns{
let map_val_0 = jni::objects::JValueGen::Object(v);
self.jni_ref().call_method(&raw_val_1,"add","(Ljava/lang/Object;)Z",vec![jni::objects::JValueGen::from(map_val_0)])?;
};
let val_1 = jni::objects::JValueGen::Object(raw_val_1);
let res = self.jni_ref().call_method(&self.jni_object(),"setPatterns",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
/// Adds a new pattern on top of the existing
/// patterns
	fn add_pattern(&self,pattern: impl Into<crate::block::banner::Pattern<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/block/banner/Pattern;)V");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(pattern.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"addPattern",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
/// Returns the pattern at the specified index
	fn get_pattern(&self,i: i32) 
-> Result<crate::block::banner::Pattern<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("(I)Lorg/bukkit/block/banner/Pattern;");
let val_1 = jni::objects::JValueGen::Int(i);
let res = self.jni_ref().call_method(&self.jni_object(),"getPattern",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
crate::block::banner::Pattern::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
/// Removes the pattern at the specified index
	fn remove_pattern(&self,i: i32) 
-> Result<crate::block::banner::Pattern<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("(I)Lorg/bukkit/block/banner/Pattern;");
let val_1 = jni::objects::JValueGen::Int(i);
let res = self.jni_ref().call_method(&self.jni_object(),"removePattern",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
crate::block::banner::Pattern::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
/// Sets the pattern at the specified index
	fn set_pattern(&self,i: i32,pattern: impl Into<crate::block::banner::Pattern<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(ILorg/bukkit/block/banner/Pattern;)V");
let val_1 = jni::objects::JValueGen::Int(i);
let val_2 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(pattern.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"setPattern",sig.as_str(),vec![jni::objects::JValueGen::from(val_1),jni::objects::JValueGen::from(val_2)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
/// Returns the number of patterns on this
/// banner
	fn number_of_patterns(&self) 
-> Result<i32, Box<dyn std::error::Error>>

{let sig = String::from("()I");
let res = self.jni_ref().call_method(&self.jni_object(),"numberOfPatterns",sig.as_str(),vec![]);
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
impl<'mc> Into<crate::inventory::meta::ItemMeta<'mc>> for BannerMeta<'mc>{

fn into(self) -> crate::inventory::meta::ItemMeta<'mc> {

crate::inventory::meta::ItemMeta::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting BannerMeta into crate::inventory::meta::ItemMeta")

   }
}
impl<'mc> crate::inventory::meta::ItemMetaTrait<'mc> for BannerMeta<'mc> {}
#[repr(C)]
pub struct SkullMeta<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for SkullMeta<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for SkullMeta<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate SkullMeta from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/inventory/meta/SkullMeta")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a SkullMeta object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> SkullMetaTrait<'mc> for SkullMeta<'mc> {}
pub trait SkullMetaTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {
#[deprecated]
/// Gets the owner of the skull.
	fn owner(&self) 
-> Result<Option<String>, Box<dyn std::error::Error>>

{let sig = String::from("()Ljava/lang/String;");
let res = self.jni_ref().call_method(&self.jni_object(),"getOwner",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {return Ok(None);}
Ok(
Some(
self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string()
)
)}
/// Checks to see if the skull has an owner.
	fn has_owner(&self) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("()Z");
let res = self.jni_ref().call_method(&self.jni_object(),"hasOwner",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
#[deprecated]
/// Sets the owner of the skull.
	fn set_owner(&self,owner: impl Into<String>) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("(Ljava/lang/String;)Z");
let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(owner.into())?));
let res = self.jni_ref().call_method(&self.jni_object(),"setOwner",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
/// Gets the owner of the skull.
	fn owning_player(&self) 
-> Result<Option<crate::OfflinePlayer<'mc>>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/OfflinePlayer;");
let res = self.jni_ref().call_method(&self.jni_object(),"getOwningPlayer",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {return Ok(None);}
Ok(
Some(
crate::OfflinePlayer::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)?
)
)}
/// Sets the owner of the skull.
/// 
/// Plugins should check that hasOwner() returns true before calling this
/// plugin.
	fn set_owning_player(&self,owner: impl Into<crate::OfflinePlayer<'mc>>) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/OfflinePlayer;)Z");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(owner.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"setOwningPlayer",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
/// Gets the profile of the player who owns the skull. This player profile
/// may appear as the texture depending on skull type.
	fn owner_profile(&self) 
-> Result<Option<crate::profile::PlayerProfile<'mc>>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/profile/PlayerProfile;");
let res = self.jni_ref().call_method(&self.jni_object(),"getOwnerProfile",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {return Ok(None);}
Ok(
Some(
crate::profile::PlayerProfile::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)?
)
)}
/// Sets the profile of the player who owns the skull. This player profile
/// may appear as the texture depending on skull type.
/// 
/// The profile must contain both a unique id and a skin texture. If either
/// of these is missing, the profile must contain a name by which the server
/// will then attempt to look up the unique id and skin texture.
	fn set_owner_profile(&self,profile: impl Into<crate::profile::PlayerProfile<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/profile/PlayerProfile;)V");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(profile.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"setOwnerProfile",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
/// Sets the sound to play if the skull is placed on a note block.
/// 
/// <strong>Note:</strong> This only works for player heads. For other heads,
/// see {@link org.bukkit.Instrument}.
	fn set_note_block_sound(&self,note_block_sound: impl Into<crate::NamespacedKey<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/NamespacedKey;)V");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(note_block_sound.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"setNoteBlockSound",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
/// Gets the sound to play if the skull is placed on a note block.
/// 
/// <strong>Note:</strong> This only works for player heads. For other heads,
/// see {@link org.bukkit.Instrument}.
	fn note_block_sound(&self) 
-> Result<Option<crate::NamespacedKey<'mc>>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/NamespacedKey;");
let res = self.jni_ref().call_method(&self.jni_object(),"getNoteBlockSound",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {return Ok(None);}
Ok(
Some(
crate::NamespacedKey::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)?
)
)}

	fn clone(&self) 
-> Result<crate::inventory::meta::SkullMeta<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/inventory/meta/SkullMeta;");
let res = self.jni_ref().call_method(&self.jni_object(),"clone",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::inventory::meta::SkullMeta::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
impl<'mc> Into<crate::inventory::meta::ItemMeta<'mc>> for SkullMeta<'mc>{

fn into(self) -> crate::inventory::meta::ItemMeta<'mc> {

crate::inventory::meta::ItemMeta::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting SkullMeta into crate::inventory::meta::ItemMeta")

   }
}
impl<'mc> crate::inventory::meta::ItemMetaTrait<'mc> for SkullMeta<'mc> {}
#[repr(C)]
pub struct BookMeta<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for BookMeta<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for BookMeta<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate BookMeta from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/inventory/meta/BookMeta")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a BookMeta object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> BookMetaTrait<'mc> for BookMeta<'mc> {}
pub trait BookMetaTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {
/// Checks for the existence of a title in the book.
	fn has_title(&self) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("()Z");
let res = self.jni_ref().call_method(&self.jni_object(),"hasTitle",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
/// Gets the title of the book.
/// 
/// Plugins should check that hasTitle() returns true before calling this
/// method.
	fn title(&self) 
-> Result<Option<String>, Box<dyn std::error::Error>>

{let sig = String::from("()Ljava/lang/String;");
let res = self.jni_ref().call_method(&self.jni_object(),"getTitle",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {return Ok(None);}
Ok(
Some(
self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string()
)
)}
/// Sets the title of the book.
/// 
/// Limited to 32 characters. Removes title when given null.
	fn set_title(&self,title: impl Into<String>) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("(Ljava/lang/String;)Z");
let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(title.into())?));
let res = self.jni_ref().call_method(&self.jni_object(),"setTitle",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
/// Checks for the existence of an author in the book.
	fn has_author(&self) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("()Z");
let res = self.jni_ref().call_method(&self.jni_object(),"hasAuthor",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
/// Gets the author of the book.
/// 
/// Plugins should check that hasAuthor() returns true before calling this
/// method.
	fn author(&self) 
-> Result<Option<String>, Box<dyn std::error::Error>>

{let sig = String::from("()Ljava/lang/String;");
let res = self.jni_ref().call_method(&self.jni_object(),"getAuthor",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {return Ok(None);}
Ok(
Some(
self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string()
)
)}
/// Sets the author of the book. Removes author when given null.
	fn set_author(&self,author: impl Into<String>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Ljava/lang/String;)V");
let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(author.into())?));
let res = self.jni_ref().call_method(&self.jni_object(),"setAuthor",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
/// Checks for the existence of generation level in the book.
	fn has_generation(&self) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("()Z");
let res = self.jni_ref().call_method(&self.jni_object(),"hasGeneration",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
/// Gets the generation of the book.
/// 
/// Plugins should check that hasGeneration() returns true before calling
/// this method.
	fn generation(&self) 
-> Result<Option<crate::inventory::meta::BookMetaGeneration<'mc>>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/inventory/meta/BookMeta/Generation;");
let res = self.jni_ref().call_method(&self.jni_object(),"getGeneration",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {return Ok(None);}
Ok(
Some(
crate::inventory::meta::BookMetaGeneration::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)?
)
)}
/// Sets the generation of the book. Removes generation when given null.
	fn set_generation(&self,generation: impl Into<crate::inventory::meta::BookMetaGeneration<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/inventory/meta/BookMeta/Generation;)V");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(generation.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"setGeneration",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
/// Checks for the existence of pages in the book.
	fn has_pages(&self) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("()Z");
let res = self.jni_ref().call_method(&self.jni_object(),"hasPages",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
/// Gets the specified page in the book. The given page must exist.
/// 
/// Pages are 1-indexed.
	fn get_page(&self,page: i32) 
-> Result<String, Box<dyn std::error::Error>>

{let sig = String::from("(I)Ljava/lang/String;");
let val_1 = jni::objects::JValueGen::Int(page);
let res = self.jni_ref().call_method(&self.jni_object(),"getPage",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string()
)}
/// Sets the specified page in the book. Pages of the book must be
/// contiguous.
/// 
/// The data can be up to 1024 characters in length, additional characters
/// are truncated.
/// 
/// Pages are 1-indexed.
	fn set_page(&self,page: i32,data: impl Into<String>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(ILjava/lang/String;)V");
let val_1 = jni::objects::JValueGen::Int(page);
let val_2 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(data.into())?));
let res = self.jni_ref().call_method(&self.jni_object(),"setPage",sig.as_str(),vec![jni::objects::JValueGen::from(val_1),jni::objects::JValueGen::from(val_2)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
/// Gets all the pages in the book.
	fn pages(&self) 
-> Result<Vec<String>, Box<dyn std::error::Error>>

{let sig = String::from("()Ljava/util/List;");
let res = self.jni_ref().call_method(&self.jni_object(),"getPages",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
let mut new_vec = Vec::new();
let list = blackboxmc_java::util::JavaList::from_raw(&self.jni_ref(), res.l()?)?;let iter = list.iterator()?;
while iter.has_next()? {            let obj = iter.next()?;
new_vec.push(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(*obj) })?.to_string_lossy().to_string());
};
Ok(
new_vec
)}
/// Clears the existing book pages, and sets the book to use the provided
/// pages. Maximum 100 pages with 1024 characters per page.
	fn set_pages(&self,pages: impl Into<String>) 
-> Result<(), Box<dyn std::error::Error>>

{let mut args = Vec::new();
let mut sig = String::from("(");
sig += "Ljava/lang/String;";
let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(pages.into())?));
args.push(val_1);
sig += ")V";
let res = self.jni_ref().call_method(&self.jni_object(),"setPages",sig.as_str(),args);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
/// Adds new pages to the end of the book. Up to a maximum of 100 pages with
/// 1024 characters per page.
	fn add_page(&self,pages: impl Into<String>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Ljava/lang/String;)V");
let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(pages.into())?));
let res = self.jni_ref().call_method(&self.jni_object(),"addPage",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
/// Gets the number of pages in the book.
	fn page_count(&self) 
-> Result<i32, Box<dyn std::error::Error>>

{let sig = String::from("()I");
let res = self.jni_ref().call_method(&self.jni_object(),"getPageCount",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.i()?
)}

	fn clone(&self) 
-> Result<crate::inventory::meta::BookMeta<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/inventory/meta/BookMeta;");
let res = self.jni_ref().call_method(&self.jni_object(),"clone",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::inventory::meta::BookMeta::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
impl<'mc> Into<crate::inventory::meta::ItemMeta<'mc>> for BookMeta<'mc>{

fn into(self) -> crate::inventory::meta::ItemMeta<'mc> {

crate::inventory::meta::ItemMeta::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting BookMeta into crate::inventory::meta::ItemMeta")

   }
}
impl<'mc> crate::inventory::meta::ItemMetaTrait<'mc> for BookMeta<'mc> {}
#[repr(C)]
pub struct BundleMeta<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for BundleMeta<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for BundleMeta<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate BundleMeta from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/inventory/meta/BundleMeta")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a BundleMeta object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> BundleMetaTrait<'mc> for BundleMeta<'mc> {}
pub trait BundleMetaTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {
/// Returns whether the item has any items.
	fn has_items(&self) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("()Z");
let res = self.jni_ref().call_method(&self.jni_object(),"hasItems",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
/// Returns an immutable list of the items stored in this item.
	fn items(&self) 
-> Result<Vec<crate::inventory::ItemStack<'mc>>, Box<dyn std::error::Error>>

{let sig = String::from("()Ljava/util/List;");
let res = self.jni_ref().call_method(&self.jni_object(),"getItems",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
let mut new_vec = Vec::new();
let list = blackboxmc_java::util::JavaList::from_raw(&self.jni_ref(), res.l()?)?;let iter = list.iterator()?;
while iter.has_next()? {            let obj = iter.next()?;
new_vec.push(crate::inventory::ItemStack::from_raw(&self.jni_ref(),obj,)?);
};
Ok(
new_vec
)}
/// Sets the items stored in this item.
/// 
/// Removes all items when given null.
	fn set_items(&self,items: Vec<jni::objects::JObject<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Ljava/util/List;)V");
let raw_val_1 = self.jni_ref().new_object("java/util/ArrayList", "()V", vec![])?;
for v in items{
let map_val_0 = jni::objects::JValueGen::Object(v);
self.jni_ref().call_method(&raw_val_1,"add","(Ljava/lang/Object;)Z",vec![jni::objects::JValueGen::from(map_val_0)])?;
};
let val_1 = jni::objects::JValueGen::Object(raw_val_1);
let res = self.jni_ref().call_method(&self.jni_object(),"setItems",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
/// Adds an item to this item.
	fn add_item(&self,item: impl Into<crate::inventory::ItemStack<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/inventory/ItemStack;)V");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(item.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"addItem",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
impl<'mc> Into<crate::inventory::meta::ItemMeta<'mc>> for BundleMeta<'mc>{

fn into(self) -> crate::inventory::meta::ItemMeta<'mc> {

crate::inventory::meta::ItemMeta::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting BundleMeta into crate::inventory::meta::ItemMeta")

   }
}
impl<'mc> crate::inventory::meta::ItemMetaTrait<'mc> for BundleMeta<'mc> {}
#[repr(C)]
pub struct SuspiciousStewMeta<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for SuspiciousStewMeta<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for SuspiciousStewMeta<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate SuspiciousStewMeta from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/inventory/meta/SuspiciousStewMeta")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a SuspiciousStewMeta object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> SuspiciousStewMetaTrait<'mc> for SuspiciousStewMeta<'mc> {}
pub trait SuspiciousStewMetaTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {
/// Checks for the presence of custom potion effects.
	fn has_custom_effects(&self) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("()Z");
let res = self.jni_ref().call_method(&self.jni_object(),"hasCustomEffects",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
/// Gets an immutable list containing all custom potion effects applied to
/// this suspicious stew.
/// 
/// Plugins should check that hasCustomEffects() returns true before calling
/// this method.
	fn custom_effects(&self) 
-> Result<Vec<crate::potion::PotionEffect<'mc>>, Box<dyn std::error::Error>>

{let sig = String::from("()Ljava/util/List;");
let res = self.jni_ref().call_method(&self.jni_object(),"getCustomEffects",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
let mut new_vec = Vec::new();
let list = blackboxmc_java::util::JavaList::from_raw(&self.jni_ref(), res.l()?)?;let iter = list.iterator()?;
while iter.has_next()? {            let obj = iter.next()?;
new_vec.push(crate::potion::PotionEffect::from_raw(&self.jni_ref(),obj,)?);
};
Ok(
new_vec
)}
/// Adds a custom potion effect to this suspicious stew.
	fn add_custom_effect(&self,effect: impl Into<crate::potion::PotionEffect<'mc>>,overwrite: bool) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/potion/PotionEffect;Z)Z");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(effect.into().jni_object().clone())});
let val_2 = jni::objects::JValueGen::Bool(overwrite.into());
let res = self.jni_ref().call_method(&self.jni_object(),"addCustomEffect",sig.as_str(),vec![jni::objects::JValueGen::from(val_1),jni::objects::JValueGen::from(val_2)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
/// Removes a custom potion effect from this suspicious stew.
	fn remove_custom_effect(&self,val_type: impl Into<crate::potion::PotionEffectType<'mc>>) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/potion/PotionEffectType;)Z");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(val_type.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"removeCustomEffect",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
/// Checks for a specific custom potion effect type on this suspicious stew.
	fn has_custom_effect(&self,val_type: impl Into<crate::potion::PotionEffectType<'mc>>) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/potion/PotionEffectType;)Z");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(val_type.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"hasCustomEffect",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
/// Removes all custom potion effects from this suspicious stew.
	fn clear_custom_effects(&self) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("()Z");
let res = self.jni_ref().call_method(&self.jni_object(),"clearCustomEffects",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}

	fn clone(&self) 
-> Result<crate::inventory::meta::SuspiciousStewMeta<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/inventory/meta/SuspiciousStewMeta;");
let res = self.jni_ref().call_method(&self.jni_object(),"clone",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::inventory::meta::SuspiciousStewMeta::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
impl<'mc> Into<crate::inventory::meta::ItemMeta<'mc>> for SuspiciousStewMeta<'mc>{

fn into(self) -> crate::inventory::meta::ItemMeta<'mc> {

crate::inventory::meta::ItemMeta::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting SuspiciousStewMeta into crate::inventory::meta::ItemMeta")

   }
}
impl<'mc> crate::inventory::meta::ItemMetaTrait<'mc> for SuspiciousStewMeta<'mc> {}
#[repr(C)]
pub struct FireworkEffectMeta<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for FireworkEffectMeta<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for FireworkEffectMeta<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate FireworkEffectMeta from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/inventory/meta/FireworkEffectMeta")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a FireworkEffectMeta object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> FireworkEffectMetaTrait<'mc> for FireworkEffectMeta<'mc> {}
pub trait FireworkEffectMetaTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {
/// Sets the firework effect for this meta.
	fn set_effect(&self,effect: impl Into<crate::FireworkEffect<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/FireworkEffect;)V");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(effect.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"setEffect",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
/// Checks if this meta has an effect.
	fn has_effect(&self) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("()Z");
let res = self.jni_ref().call_method(&self.jni_object(),"hasEffect",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
/// Gets the firework effect for this meta.
	fn effect(&self) 
-> Result<Option<crate::FireworkEffect<'mc>>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/FireworkEffect;");
let res = self.jni_ref().call_method(&self.jni_object(),"getEffect",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {return Ok(None);}
Ok(
Some(
crate::FireworkEffect::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)?
)
)}

	fn clone(&self) 
-> Result<crate::inventory::meta::FireworkEffectMeta<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/inventory/meta/FireworkEffectMeta;");
let res = self.jni_ref().call_method(&self.jni_object(),"clone",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::inventory::meta::FireworkEffectMeta::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
impl<'mc> Into<crate::inventory::meta::ItemMeta<'mc>> for FireworkEffectMeta<'mc>{

fn into(self) -> crate::inventory::meta::ItemMeta<'mc> {

crate::inventory::meta::ItemMeta::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting FireworkEffectMeta into crate::inventory::meta::ItemMeta")

   }
}
impl<'mc> crate::inventory::meta::ItemMetaTrait<'mc> for FireworkEffectMeta<'mc> {}
#[repr(C)]
pub struct ItemMeta<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for ItemMeta<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for ItemMeta<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate ItemMeta from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/inventory/meta/ItemMeta")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a ItemMeta object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> ItemMetaTrait<'mc> for ItemMeta<'mc> {}
pub trait ItemMetaTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {
/// Checks for existence of a display name.
	fn has_display_name(&self) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("()Z");
let res = self.jni_ref().call_method(&self.jni_object(),"hasDisplayName",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
/// Gets the display name that is set.
/// 
/// Plugins should check that hasDisplayName() returns <code>true</code>
/// before calling this method.
	fn display_name(&self) 
-> Result<String, Box<dyn std::error::Error>>

{let sig = String::from("()Ljava/lang/String;");
let res = self.jni_ref().call_method(&self.jni_object(),"getDisplayName",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string()
)}
/// Sets the display name.
	fn set_display_name(&self,name: impl Into<String>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Ljava/lang/String;)V");
let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(name.into())?));
let res = self.jni_ref().call_method(&self.jni_object(),"setDisplayName",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
/// Checks for existence of a localized name.
	fn has_localized_name(&self) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("()Z");
let res = self.jni_ref().call_method(&self.jni_object(),"hasLocalizedName",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
/// Gets the localized display name that is set.
/// 
/// Plugins should check that hasLocalizedName() returns <code>true</code>
/// before calling this method.
	fn localized_name(&self) 
-> Result<String, Box<dyn std::error::Error>>

{let sig = String::from("()Ljava/lang/String;");
let res = self.jni_ref().call_method(&self.jni_object(),"getLocalizedName",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string()
)}
/// Sets the localized name.
	fn set_localized_name(&self,name: impl Into<String>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Ljava/lang/String;)V");
let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(name.into())?));
let res = self.jni_ref().call_method(&self.jni_object(),"setLocalizedName",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
/// Checks for existence of lore.
	fn has_lore(&self) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("()Z");
let res = self.jni_ref().call_method(&self.jni_object(),"hasLore",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
/// Gets the lore that is set.
/// 
/// Plugins should check if hasLore() returns <code>true</code> before
/// calling this method.
	fn lore(&self) 
-> Result<Option<Vec<String>>, Box<dyn std::error::Error>>

{let sig = String::from("()Ljava/util/List;");
let res = self.jni_ref().call_method(&self.jni_object(),"getLore",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {return Ok(None);}
let mut new_vec = Vec::new();
let list = blackboxmc_java::util::JavaList::from_raw(&self.jni_ref(), res.l()?)?;let iter = list.iterator()?;
while iter.has_next()? {            let obj = iter.next()?;
new_vec.push(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(*obj) })?.to_string_lossy().to_string());
};
Ok(
Some(
new_vec
)
)}
/// Sets the lore for this item.
/// Removes lore when given null.
	fn set_lore(&self,lore: Vec<jni::objects::JObject<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Ljava/util/List;)V");
let raw_val_1 = self.jni_ref().new_object("java/util/ArrayList", "()V", vec![])?;
for v in lore{
let map_val_0 = jni::objects::JValueGen::Object(v);
self.jni_ref().call_method(&raw_val_1,"add","(Ljava/lang/Object;)Z",vec![jni::objects::JValueGen::from(map_val_0)])?;
};
let val_1 = jni::objects::JValueGen::Object(raw_val_1);
let res = self.jni_ref().call_method(&self.jni_object(),"setLore",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
/// Checks for existence of custom model data.
/// 
/// CustomModelData is an integer that may be associated client side with a
/// custom item model.
	fn has_custom_model_data(&self) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("()Z");
let res = self.jni_ref().call_method(&self.jni_object(),"hasCustomModelData",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
/// Gets the custom model data that is set.
/// 
/// CustomModelData is an integer that may be associated client side with a
/// custom item model.
/// 
/// Plugins should check that hasCustomModelData() returns <code>true</code>
/// before calling this method.
	fn custom_model_data(&self) 
-> Result<i32, Box<dyn std::error::Error>>

{let sig = String::from("()I");
let res = self.jni_ref().call_method(&self.jni_object(),"getCustomModelData",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.i()?
)}
/// Sets the custom model data.
/// 
/// CustomModelData is an integer that may be associated client side with a
/// custom item model.
	fn set_custom_model_data(&self,data: i32) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Ljava/lang/Integer;)V");
let val_1 = jni::objects::JValueGen::Object(self.jni_ref().new_object("java/lang/Integer", "(Ljava/Lang/Object;)V", vec![data.into()])?);
let res = self.jni_ref().call_method(&self.jni_object(),"setCustomModelData",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
/// Checks for the existence of any enchantments.
	fn has_enchants(&self) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("()Z");
let res = self.jni_ref().call_method(&self.jni_object(),"hasEnchants",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
/// Checks for existence of the specified enchantment.
	fn has_enchant(&self,ench: impl Into<crate::enchantments::Enchantment<'mc>>) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/enchantments/Enchantment;)Z");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(ench.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"hasEnchant",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
/// Checks for the level of the specified enchantment.
	fn get_enchant_level(&self,ench: impl Into<crate::enchantments::Enchantment<'mc>>) 
-> Result<i32, Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/enchantments/Enchantment;)I");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(ench.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"getEnchantLevel",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.i()?
)}
/// Returns a copy the enchantments in this ItemMeta.
/// 
/// Returns an empty map if none.
	fn enchants(&self) 
-> Result<blackboxmc_java::util::JavaMap<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Ljava/util/Map;");
let res = self.jni_ref().call_method(&self.jni_object(),"getEnchants",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
blackboxmc_java::util::JavaMap::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
/// Adds the specified enchantment to this item meta.
	fn add_enchant(&self,ench: impl Into<crate::enchantments::Enchantment<'mc>>,level: i32,ignore_level_restriction: bool) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/enchantments/Enchantment;IZ)Z");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(ench.into().jni_object().clone())});
let val_2 = jni::objects::JValueGen::Int(level);
let val_3 = jni::objects::JValueGen::Bool(ignore_level_restriction.into());
let res = self.jni_ref().call_method(&self.jni_object(),"addEnchant",sig.as_str(),vec![jni::objects::JValueGen::from(val_1),jni::objects::JValueGen::from(val_2),jni::objects::JValueGen::from(val_3)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
/// Removes the specified enchantment from this item meta.
	fn remove_enchant(&self,ench: impl Into<crate::enchantments::Enchantment<'mc>>) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/enchantments/Enchantment;)Z");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(ench.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"removeEnchant",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
/// Removes all enchantments from this item meta.
	fn remove_enchantments(&self) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("()V");
let res = self.jni_ref().call_method(&self.jni_object(),"removeEnchantments",sig.as_str(),vec![]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
/// Checks if the specified enchantment conflicts with any enchantments in
/// this ItemMeta.
	fn has_conflicting_enchant(&self,ench: impl Into<crate::enchantments::Enchantment<'mc>>) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/enchantments/Enchantment;)Z");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(ench.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"hasConflictingEnchant",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
/// Set itemflags which should be ignored when rendering a ItemStack in the Client. This Method does silently ignore double set itemFlags.
	fn add_item_flags(&self,item_flags: impl Into<crate::inventory::ItemFlag<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/inventory/ItemFlag;)V");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(item_flags.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"addItemFlags",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
/// Remove specific set of itemFlags. This tells the Client it should render it again. This Method does silently ignore double removed itemFlags.
	fn remove_item_flags(&self,item_flags: impl Into<crate::inventory::ItemFlag<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/inventory/ItemFlag;)V");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(item_flags.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"removeItemFlags",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
/// Get current set itemFlags. The collection returned is unmodifiable.
	fn item_flags(&self) 
-> Result<blackboxmc_java::util::JavaSet<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Ljava/util/Set;");
let res = self.jni_ref().call_method(&self.jni_object(),"getItemFlags",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
blackboxmc_java::util::JavaSet::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
/// Check if the specified flag is present on this item.
	fn has_item_flag(&self,flag: impl Into<crate::inventory::ItemFlag<'mc>>) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/inventory/ItemFlag;)Z");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(flag.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"hasItemFlag",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
/// Return if the unbreakable tag is true. An unbreakable item will not lose
/// durability.
	fn is_unbreakable(&self) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("()Z");
let res = self.jni_ref().call_method(&self.jni_object(),"isUnbreakable",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
/// Sets the unbreakable tag. An unbreakable item will not lose durability.
	fn set_unbreakable(&self,unbreakable: bool) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Z)V");
let val_1 = jni::objects::JValueGen::Bool(unbreakable.into());
let res = self.jni_ref().call_method(&self.jni_object(),"setUnbreakable",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
/// Checks for the existence of any AttributeModifiers.
	fn has_attribute_modifiers(&self) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("()Z");
let res = self.jni_ref().call_method(&self.jni_object(),"hasAttributeModifiers",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
/// Return an immutable copy of all {@link AttributeModifier}s
/// for a given {@link Attribute}
	fn get_attribute_modifiers(&self,attribute: impl Into<crate::attribute::Attribute<'mc>>) 
-> Result<Option<Vec<crate::attribute::AttributeModifier<'mc>>>, Box<dyn std::error::Error>>

{let mut args = Vec::new();
let mut sig = String::from("(");
sig += "Lorg/bukkit/attribute/Attribute;";
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(attribute.into().jni_object().clone())});
args.push(val_1);
sig += ")Ljava/util/Collection;";
let res = self.jni_ref().call_method(&self.jni_object(),"getAttributeModifiers",sig.as_str(),args);
let res = 
self.jni_ref().translate_error(res)?;
if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {return Ok(None);}
let mut new_vec = Vec::new();
let col = blackboxmc_java::util::JavaCollection::from_raw(&self.jni_ref(),res.l()?)?;let iter = col.iterator()?;
while iter.has_next()? {            let obj = iter.next()?;
new_vec.push(crate::attribute::AttributeModifier::from_raw(&self.jni_ref(),obj,)?);
};
Ok(
Some(
new_vec
)
)}
/// Add an Attribute and it's Modifier.
/// AttributeModifiers can now support {@link EquipmentSlot}s.
/// If not set, the {@link AttributeModifier} will be active in ALL slots.
/// 
/// Two {@link AttributeModifier}s that have the same {@link java.util.UUID}
/// cannot exist on the same Attribute.
	fn add_attribute_modifier(&self,attribute: impl Into<crate::attribute::Attribute<'mc>>,modifier: impl Into<crate::attribute::AttributeModifier<'mc>>) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/attribute/Attribute;Lorg/bukkit/attribute/AttributeModifier;)Z");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(attribute.into().jni_object().clone())});
let val_2 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(modifier.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"addAttributeModifier",sig.as_str(),vec![jni::objects::JValueGen::from(val_1),jni::objects::JValueGen::from(val_2)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
/// Set all {@link Attribute}s and their {@link AttributeModifier}s.
/// To clear all currently set Attributes and AttributeModifiers use
/// null or an empty Multimap.
/// If not null nor empty, this will filter all entries that are not-null
/// and add them to the ItemStack.
	fn set_attribute_modifiers(&self,attribute_modifiers: jni::objects::JObject<'mc>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Lcom/google/common/collect/Multimap;)V");
let val_1 = jni::objects::JValueGen::Object(attribute_modifiers);
let res = self.jni_ref().call_method(&self.jni_object(),"setAttributeModifiers",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
/// Remove a specific {@link Attribute} and {@link AttributeModifier}.
/// AttributeModifiers are matched according to their {@link java.util.UUID}.
	fn remove_attribute_modifier(&self,attribute: impl Into<crate::attribute::Attribute<'mc>>,modifier: std::option::Option<impl Into<crate::attribute::AttributeModifier<'mc>>>) 
-> Result<bool, Box<dyn std::error::Error>>

{let mut args = Vec::new();
let mut sig = String::from("(");
sig += "Lorg/bukkit/attribute/Attribute;";
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(attribute.into().jni_object().clone())});
args.push(val_1);
if let Some(a) = modifier {
sig += "Lorg/bukkit/attribute/AttributeModifier;";
let val_2 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(a.into().jni_object().clone())});
args.push(val_2);
}
sig += ")Z";
let res = self.jni_ref().call_method(&self.jni_object(),"removeAttributeModifier",sig.as_str(),args);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
/// Get this ItemMeta as an NBT string.
/// 
/// This string should not be relied upon as a serializable value. If
/// serialization is desired, the {@link ConfigurationSerializable} API
/// should be used instead.
	fn as_string(&self) 
-> Result<String, Box<dyn std::error::Error>>

{let sig = String::from("()Ljava/lang/String;");
let res = self.jni_ref().call_method(&self.jni_object(),"getAsString",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string()
)}
#[deprecated]
/// Returns a public custom tag container capable of storing tags on the item. Those tags will be sent to the client with all of their content, so the client is capable of reading them. This will result in the player seeing a NBT Tag notification on the item. These tags can also be modified by the client once in creative mode
	fn custom_tag_container(&self) 
-> Result<crate::inventory::meta::tags::CustomItemTagContainer<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/inventory/meta/tags/CustomItemTagContainer;");
let res = self.jni_ref().call_method(&self.jni_object(),"getCustomTagContainer",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::inventory::meta::tags::CustomItemTagContainer::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
/// Internal use only! Do not use under any circumstances!
	fn set_version(&self,version: i32) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(I)V");
let val_1 = jni::objects::JValueGen::Int(version);
let res = self.jni_ref().call_method(&self.jni_object(),"setVersion",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}

	fn clone(&self) 
-> Result<crate::inventory::meta::ItemMeta<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/inventory/meta/ItemMeta;");
let res = self.jni_ref().call_method(&self.jni_object(),"clone",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::inventory::meta::ItemMeta::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
impl<'mc> Into<crate::configuration::serialization::ConfigurationSerializable<'mc>> for ItemMeta<'mc>{

fn into(self) -> crate::configuration::serialization::ConfigurationSerializable<'mc> {

crate::configuration::serialization::ConfigurationSerializable::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting ItemMeta into crate::configuration::serialization::ConfigurationSerializable")

   }
}
impl<'mc> crate::configuration::serialization::ConfigurationSerializableTrait<'mc> for ItemMeta<'mc> {}
impl<'mc> Into<crate::persistence::PersistentDataHolder<'mc>> for ItemMeta<'mc>{

fn into(self) -> crate::persistence::PersistentDataHolder<'mc> {

crate::persistence::PersistentDataHolder::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting ItemMeta into crate::persistence::PersistentDataHolder")

   }
}
impl<'mc> crate::persistence::PersistentDataHolderTrait<'mc> for ItemMeta<'mc> {}
#[repr(C)]
pub struct CompassMeta<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for CompassMeta<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for CompassMeta<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate CompassMeta from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/inventory/meta/CompassMeta")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a CompassMeta object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> CompassMetaTrait<'mc> for CompassMeta<'mc> {}
pub trait CompassMetaTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {
/// Checks if this compass has been paired to a lodestone.
	fn has_lodestone(&self) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("()Z");
let res = self.jni_ref().call_method(&self.jni_object(),"hasLodestone",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
/// Gets the location that this compass will point to.
/// Check {@link #hasLodestone()} first!
	fn lodestone(&self) 
-> Result<Option<crate::Location<'mc>>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/Location;");
let res = self.jni_ref().call_method(&self.jni_object(),"getLodestone",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {return Ok(None);}
Ok(
Some(
crate::Location::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)?
)
)}
/// Sets the location this lodestone compass will point to.
	fn set_lodestone(&self,lodestone: impl Into<crate::Location<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/Location;)V");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(lodestone.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"setLodestone",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
/// Gets if this compass is tracking a specific lodestone.
/// If true the compass will only work if there is a lodestone at the tracked
/// location.
	fn is_lodestone_tracked(&self) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("()Z");
let res = self.jni_ref().call_method(&self.jni_object(),"isLodestoneTracked",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
/// Sets if this compass is tracking a specific lodestone.
/// If true the compass will only work if there is a lodestone at the tracked
/// location.
	fn set_lodestone_tracked(&self,tracked: bool) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Z)V");
let val_1 = jni::objects::JValueGen::Bool(tracked.into());
let res = self.jni_ref().call_method(&self.jni_object(),"setLodestoneTracked",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}

	fn clone(&self) 
-> Result<crate::inventory::meta::CompassMeta<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/inventory/meta/CompassMeta;");
let res = self.jni_ref().call_method(&self.jni_object(),"clone",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::inventory::meta::CompassMeta::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
impl<'mc> Into<crate::inventory::meta::ItemMeta<'mc>> for CompassMeta<'mc>{

fn into(self) -> crate::inventory::meta::ItemMeta<'mc> {

crate::inventory::meta::ItemMeta::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting CompassMeta into crate::inventory::meta::ItemMeta")

   }
}
impl<'mc> crate::inventory::meta::ItemMetaTrait<'mc> for CompassMeta<'mc> {}
#[repr(C)]
pub struct LeatherArmorMeta<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for LeatherArmorMeta<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for LeatherArmorMeta<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate LeatherArmorMeta from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/inventory/meta/LeatherArmorMeta")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a LeatherArmorMeta object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> LeatherArmorMetaTrait<'mc> for LeatherArmorMeta<'mc> {}
pub trait LeatherArmorMetaTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {
/// Gets the color of the armor. If it has not been set otherwise, it will
/// be {@link ItemFactory#getDefaultLeatherColor()}.
	fn color(&self) 
-> Result<crate::Color<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/Color;");
let res = self.jni_ref().call_method(&self.jni_object(),"getColor",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::Color::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
/// Sets the color of the armor.
	fn set_color(&self,color: impl Into<crate::Color<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/Color;)V");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(color.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"setColor",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}

	fn clone(&self) 
-> Result<crate::inventory::meta::LeatherArmorMeta<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/inventory/meta/LeatherArmorMeta;");
let res = self.jni_ref().call_method(&self.jni_object(),"clone",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::inventory::meta::LeatherArmorMeta::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
impl<'mc> Into<crate::inventory::meta::ItemMeta<'mc>> for LeatherArmorMeta<'mc>{

fn into(self) -> crate::inventory::meta::ItemMeta<'mc> {

crate::inventory::meta::ItemMeta::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting LeatherArmorMeta into crate::inventory::meta::ItemMeta")

   }
}
impl<'mc> crate::inventory::meta::ItemMetaTrait<'mc> for LeatherArmorMeta<'mc> {}
#[repr(C)]
pub struct EnchantmentStorageMeta<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for EnchantmentStorageMeta<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for EnchantmentStorageMeta<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate EnchantmentStorageMeta from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/inventory/meta/EnchantmentStorageMeta")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a EnchantmentStorageMeta object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> EnchantmentStorageMetaTrait<'mc> for EnchantmentStorageMeta<'mc> {}
pub trait EnchantmentStorageMetaTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {
/// Checks for the existence of any stored enchantments.
	fn has_stored_enchants(&self) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("()Z");
let res = self.jni_ref().call_method(&self.jni_object(),"hasStoredEnchants",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
/// Checks for storage of the specified enchantment.
	fn has_stored_enchant(&self,ench: impl Into<crate::enchantments::Enchantment<'mc>>) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/enchantments/Enchantment;)Z");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(ench.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"hasStoredEnchant",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
/// Checks for the level of the stored enchantment.
	fn get_stored_enchant_level(&self,ench: impl Into<crate::enchantments::Enchantment<'mc>>) 
-> Result<i32, Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/enchantments/Enchantment;)I");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(ench.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"getStoredEnchantLevel",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.i()?
)}
/// Gets a copy the stored enchantments in this ItemMeta.
	fn stored_enchants(&self) 
-> Result<blackboxmc_java::util::JavaMap<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Ljava/util/Map;");
let res = self.jni_ref().call_method(&self.jni_object(),"getStoredEnchants",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
blackboxmc_java::util::JavaMap::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
/// Stores the specified enchantment in this item meta.
	fn add_stored_enchant(&self,ench: impl Into<crate::enchantments::Enchantment<'mc>>,level: i32,ignore_level_restriction: bool) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/enchantments/Enchantment;IZ)Z");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(ench.into().jni_object().clone())});
let val_2 = jni::objects::JValueGen::Int(level);
let val_3 = jni::objects::JValueGen::Bool(ignore_level_restriction.into());
let res = self.jni_ref().call_method(&self.jni_object(),"addStoredEnchant",sig.as_str(),vec![jni::objects::JValueGen::from(val_1),jni::objects::JValueGen::from(val_2),jni::objects::JValueGen::from(val_3)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
/// Remove the specified stored enchantment from this item meta.
	fn remove_stored_enchant(&self,ench: impl Into<crate::enchantments::Enchantment<'mc>>) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/enchantments/Enchantment;)Z");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(ench.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"removeStoredEnchant",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
/// Checks if the specified enchantment conflicts with any enchantments in
/// this ItemMeta.
	fn has_conflicting_stored_enchant(&self,ench: impl Into<crate::enchantments::Enchantment<'mc>>) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/enchantments/Enchantment;)Z");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(ench.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"hasConflictingStoredEnchant",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}

	fn clone(&self) 
-> Result<crate::inventory::meta::EnchantmentStorageMeta<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/inventory/meta/EnchantmentStorageMeta;");
let res = self.jni_ref().call_method(&self.jni_object(),"clone",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::inventory::meta::EnchantmentStorageMeta::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
impl<'mc> Into<crate::inventory::meta::ItemMeta<'mc>> for EnchantmentStorageMeta<'mc>{

fn into(self) -> crate::inventory::meta::ItemMeta<'mc> {

crate::inventory::meta::ItemMeta::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting EnchantmentStorageMeta into crate::inventory::meta::ItemMeta")

   }
}
impl<'mc> crate::inventory::meta::ItemMetaTrait<'mc> for EnchantmentStorageMeta<'mc> {}
#[repr(C)]
pub struct AxolotlBucketMeta<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for AxolotlBucketMeta<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for AxolotlBucketMeta<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate AxolotlBucketMeta from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/inventory/meta/AxolotlBucketMeta")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a AxolotlBucketMeta object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> AxolotlBucketMetaTrait<'mc> for AxolotlBucketMeta<'mc> {}
pub trait AxolotlBucketMetaTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {
/// Get the variant of the axolotl in the bucket.
/// 
/// Plugins should check that hasVariant() returns <code>true</code> before
/// calling this method.
	fn variant(&self) 
-> Result<crate::entity::AxolotlVariant<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/entity/Axolotl/Variant;");
let res = self.jni_ref().call_method(&self.jni_object(),"getVariant",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::entity::AxolotlVariant::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
/// Set the variant of this axolotl in the bucket.
	fn set_variant(&self,variant: impl Into<crate::entity::AxolotlVariant<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/entity/Axolotl/Variant;)V");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(variant.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"setVariant",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
/// Checks for existence of a variant tag indicating a specific axolotl will be
/// spawned.
	fn has_variant(&self) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("()Z");
let res = self.jni_ref().call_method(&self.jni_object(),"hasVariant",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}

	fn clone(&self) 
-> Result<crate::inventory::meta::AxolotlBucketMeta<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/inventory/meta/AxolotlBucketMeta;");
let res = self.jni_ref().call_method(&self.jni_object(),"clone",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::inventory::meta::AxolotlBucketMeta::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
impl<'mc> Into<crate::inventory::meta::ItemMeta<'mc>> for AxolotlBucketMeta<'mc>{

fn into(self) -> crate::inventory::meta::ItemMeta<'mc> {

crate::inventory::meta::ItemMeta::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting AxolotlBucketMeta into crate::inventory::meta::ItemMeta")

   }
}
impl<'mc> crate::inventory::meta::ItemMetaTrait<'mc> for AxolotlBucketMeta<'mc> {}
#[repr(C)]
pub struct KnowledgeBookMeta<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for KnowledgeBookMeta<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for KnowledgeBookMeta<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate KnowledgeBookMeta from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/inventory/meta/KnowledgeBookMeta")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a KnowledgeBookMeta object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> KnowledgeBookMetaTrait<'mc> for KnowledgeBookMeta<'mc> {}
pub trait KnowledgeBookMetaTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {
/// Checks for the existence of recipes in the book.
	fn has_recipes(&self) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("()Z");
let res = self.jni_ref().call_method(&self.jni_object(),"hasRecipes",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
/// Gets all the recipes in the book.
	fn recipes(&self) 
-> Result<Vec<crate::NamespacedKey<'mc>>, Box<dyn std::error::Error>>

{let sig = String::from("()Ljava/util/List;");
let res = self.jni_ref().call_method(&self.jni_object(),"getRecipes",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
let mut new_vec = Vec::new();
let list = blackboxmc_java::util::JavaList::from_raw(&self.jni_ref(), res.l()?)?;let iter = list.iterator()?;
while iter.has_next()? {            let obj = iter.next()?;
new_vec.push(crate::NamespacedKey::from_raw(&self.jni_ref(),obj,)?);
};
Ok(
new_vec
)}
/// Clears the existing book recipes, and sets the book to use the provided
/// recipes.
	fn set_recipes(&self,recipes: Vec<jni::objects::JObject<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Ljava/util/List;)V");
let raw_val_1 = self.jni_ref().new_object("java/util/ArrayList", "()V", vec![])?;
for v in recipes{
let map_val_0 = jni::objects::JValueGen::Object(v);
self.jni_ref().call_method(&raw_val_1,"add","(Ljava/lang/Object;)Z",vec![jni::objects::JValueGen::from(map_val_0)])?;
};
let val_1 = jni::objects::JValueGen::Object(raw_val_1);
let res = self.jni_ref().call_method(&self.jni_object(),"setRecipes",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
/// Adds new recipe to the end of the book.
	fn add_recipe(&self,recipes: impl Into<crate::NamespacedKey<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/NamespacedKey;)V");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(recipes.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"addRecipe",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}

	fn clone(&self) 
-> Result<crate::inventory::meta::KnowledgeBookMeta<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/inventory/meta/KnowledgeBookMeta;");
let res = self.jni_ref().call_method(&self.jni_object(),"clone",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::inventory::meta::KnowledgeBookMeta::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
impl<'mc> Into<crate::inventory::meta::ItemMeta<'mc>> for KnowledgeBookMeta<'mc>{

fn into(self) -> crate::inventory::meta::ItemMeta<'mc> {

crate::inventory::meta::ItemMeta::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting KnowledgeBookMeta into crate::inventory::meta::ItemMeta")

   }
}
impl<'mc> crate::inventory::meta::ItemMetaTrait<'mc> for KnowledgeBookMeta<'mc> {}
#[repr(C)]
pub struct TropicalFishBucketMeta<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for TropicalFishBucketMeta<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for TropicalFishBucketMeta<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate TropicalFishBucketMeta from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/inventory/meta/TropicalFishBucketMeta")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a TropicalFishBucketMeta object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> TropicalFishBucketMetaTrait<'mc> for TropicalFishBucketMeta<'mc> {}
pub trait TropicalFishBucketMetaTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {
/// Gets the color of the fish's pattern.
/// 
/// Plugins should check that hasVariant() returns <code>true</code> before
/// calling this method.
	fn pattern_color(&self) 
-> Result<crate::DyeColor<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/DyeColor;");
let res = self.jni_ref().call_method(&self.jni_object(),"getPatternColor",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::DyeColor::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
/// Sets the color of the fish's pattern.
/// 
/// Setting this when hasVariant() returns <code>false</code> will initialize
/// all other values to unspecified defaults.
	fn set_pattern_color(&self,color: impl Into<crate::DyeColor<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/DyeColor;)V");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(color.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"setPatternColor",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
/// Gets the color of the fish's body.
/// 
/// Plugins should check that hasVariant() returns <code>true</code> before
/// calling this method.
	fn body_color(&self) 
-> Result<crate::DyeColor<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/DyeColor;");
let res = self.jni_ref().call_method(&self.jni_object(),"getBodyColor",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::DyeColor::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
/// Sets the color of the fish's body.
/// 
/// Setting this when hasVariant() returns <code>false</code> will initialize
/// all other values to unspecified defaults.
	fn set_body_color(&self,color: impl Into<crate::DyeColor<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/DyeColor;)V");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(color.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"setBodyColor",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
/// Gets the fish's pattern.
/// 
/// Plugins should check that hasVariant() returns <code>true</code> before
/// calling this method.
	fn pattern(&self) 
-> Result<crate::entity::TropicalFishPattern<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/entity/TropicalFish/Pattern;");
let res = self.jni_ref().call_method(&self.jni_object(),"getPattern",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::entity::TropicalFishPattern::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
/// Sets the fish's pattern.
/// 
/// Setting this when hasVariant() returns <code>false</code> will initialize
/// all other values to unspecified defaults.
	fn set_pattern(&self,pattern: impl Into<crate::entity::TropicalFishPattern<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/entity/TropicalFish/Pattern;)V");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(pattern.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"setPattern",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
/// Checks for existence of a variant tag indicating a specific fish will be
/// spawned.
	fn has_variant(&self) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("()Z");
let res = self.jni_ref().call_method(&self.jni_object(),"hasVariant",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}

	fn clone(&self) 
-> Result<crate::inventory::meta::TropicalFishBucketMeta<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/inventory/meta/TropicalFishBucketMeta;");
let res = self.jni_ref().call_method(&self.jni_object(),"clone",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::inventory::meta::TropicalFishBucketMeta::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
impl<'mc> Into<crate::inventory::meta::ItemMeta<'mc>> for TropicalFishBucketMeta<'mc>{

fn into(self) -> crate::inventory::meta::ItemMeta<'mc> {

crate::inventory::meta::ItemMeta::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting TropicalFishBucketMeta into crate::inventory::meta::ItemMeta")

   }
}
impl<'mc> crate::inventory::meta::ItemMetaTrait<'mc> for TropicalFishBucketMeta<'mc> {}
pub mod tags;
pub mod trim;
