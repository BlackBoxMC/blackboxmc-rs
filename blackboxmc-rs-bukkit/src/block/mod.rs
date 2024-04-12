#![allow(deprecated)]
use blackboxmc_general::JNIRaw;
use blackboxmc_general::JNIInstantiatable;
use color_eyre::eyre::Result;/*org/bukkit/block/mod.rs*/

#[repr(C)]
pub struct Block<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

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
                return Err(eyre::eyre!(
                    "Tried to instantiate Block from null object.")
                .into());
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
    
impl<'mc> BlockTrait<'mc> for Block<'mc> {}
pub trait BlockTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {
#[deprecated]
/// Gets the metadata for this block
	fn data(&self) 
-> Result<i8, Box<dyn std::error::Error>>

{let sig = String::from("()B");
let res = self.jni_ref().call_method(&self.jni_object(),"getData",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.b()?
)}
/// Gets the complete block data for this block
	fn block_data(&self) 
-> Result<crate::block::data::BlockData<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/block/data/BlockData;");
let res = self.jni_ref().call_method(&self.jni_object(),"getBlockData",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::block::data::BlockData::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
/// Gets the block at the given offsets
	fn get_relative(&self,mod_x: i32,mod_y: std::option::Option<i32>,mod_z: std::option::Option<i32>) 
-> Result<crate::block::Block<'mc>, Box<dyn std::error::Error>>

{let mut args = Vec::new();
let mut sig = String::from("(");
sig += "I";
let val_1 = jni::objects::JValueGen::Int(mod_x);
args.push(val_1);
if let Some(a) = mod_y {
sig += "I";
let val_2 = jni::objects::JValueGen::Int(a);
args.push(val_2);
}
if let Some(a) = mod_z {
sig += "I";
let val_3 = jni::objects::JValueGen::Int(a);
args.push(val_3);
}
sig += ")Lorg/bukkit/block/Block;";
let res = self.jni_ref().call_method(&self.jni_object(),"getRelative",sig.as_str(),args);
let res = 
self.jni_ref().translate_error(res)?;
crate::block::Block::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
/// Gets the type of this block
	fn get_type(&self) 
-> Result<crate::Material<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/Material;");
let res = self.jni_ref().call_method(&self.jni_object(),"getType",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::Material::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
/// Gets the light level between 0-15
	fn light_level(&self) 
-> Result<i8, Box<dyn std::error::Error>>

{let sig = String::from("()B");
let res = self.jni_ref().call_method(&self.jni_object(),"getLightLevel",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.b()?
)}
/// Get the amount of light at this block from the sky.
/// 
/// Any light given from other sources (such as blocks like torches) will
/// be ignored.
	fn light_from_sky(&self) 
-> Result<i8, Box<dyn std::error::Error>>

{let sig = String::from("()B");
let res = self.jni_ref().call_method(&self.jni_object(),"getLightFromSky",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.b()?
)}
/// Get the amount of light at this block from nearby blocks.
/// 
/// Any light given from other sources (such as the sun) will be ignored.
	fn light_from_blocks(&self) 
-> Result<i8, Box<dyn std::error::Error>>

{let sig = String::from("()B");
let res = self.jni_ref().call_method(&self.jni_object(),"getLightFromBlocks",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.b()?
)}
/// Gets the world which contains this Block
	fn world(&self) 
-> Result<crate::World<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/World;");
let res = self.jni_ref().call_method(&self.jni_object(),"getWorld",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::World::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
/// Gets the x-coordinate of this block
	fn x(&self) 
-> Result<i32, Box<dyn std::error::Error>>

{let sig = String::from("()I");
let res = self.jni_ref().call_method(&self.jni_object(),"getX",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.i()?
)}
/// Gets the y-coordinate of this block
	fn y(&self) 
-> Result<i32, Box<dyn std::error::Error>>

{let sig = String::from("()I");
let res = self.jni_ref().call_method(&self.jni_object(),"getY",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.i()?
)}
/// Gets the z-coordinate of this block
	fn z(&self) 
-> Result<i32, Box<dyn std::error::Error>>

{let sig = String::from("()I");
let res = self.jni_ref().call_method(&self.jni_object(),"getZ",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.i()?
)}
/// Stores the location of the block in the provided Location object.
/// 
/// If the provided Location is null this method does nothing and returns
/// null.
	fn get_location(&self,loc: impl Into<crate::Location<'mc>>) 
-> Result<Option<crate::Location<'mc>>, Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/Location;)Lorg/bukkit/Location;");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(loc.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"getLocation",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {return Ok(None);}
Ok(
Some(
crate::Location::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)?
)
)}
/// Gets the chunk which contains this block
	fn chunk(&self) 
-> Result<crate::Chunk<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/Chunk;");
let res = self.jni_ref().call_method(&self.jni_object(),"getChunk",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::Chunk::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
/// Sets the complete data for this block
/// 
/// Note that applyPhysics = false is not in general safe. It should only be
/// used when you need to avoid triggering a physics update of neighboring
/// blocks, for example when creating a {@link Bisected} block. If you are
/// using a custom populator, then this parameter may also be required to
/// prevent triggering infinite chunk loads on border blocks. This method
/// should NOT be used to "hack" physics by placing blocks in impossible
/// locations. Such blocks are liable to be removed on various events such as
/// world upgrades. Furthermore setting large amounts of such blocks in close
/// proximity may overload the server physics engine if an update is
/// triggered at a later point. If this occurs, the resulting behavior is
/// undefined.
	fn set_block_data(&self,data: impl Into<crate::block::data::BlockData<'mc>>,apply_physics: std::option::Option<bool>) 
-> Result<(), Box<dyn std::error::Error>>

{let mut args = Vec::new();
let mut sig = String::from("(");
sig += "Lorg/bukkit/block/data/BlockData;";
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(data.into().jni_object().clone())});
args.push(val_1);
if let Some(a) = apply_physics {
sig += "Z";
let val_2 = jni::objects::JValueGen::Bool(a.into());
args.push(val_2);
}
sig += ")V";
let res = self.jni_ref().call_method(&self.jni_object(),"setBlockData",sig.as_str(),args);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
/// Sets the type of this block
/// 
/// Note that applyPhysics = false is not in general safe. It should only be
/// used when you need to avoid triggering a physics update of neighboring
/// blocks, for example when creating a {@link Bisected} block. If you are
/// using a custom populator, then this parameter may also be required to
/// prevent triggering infinite chunk loads on border blocks. This method
/// should NOT be used to "hack" physics by placing blocks in impossible
/// locations. Such blocks are liable to be removed on various events such as
/// world upgrades. Furthermore setting large amounts of such blocks in close
/// proximity may overload the server physics engine if an update is
/// triggered at a later point. If this occurs, the resulting behavior is
/// undefined.
	fn set_type(&self,val_type: impl Into<crate::Material<'mc>>,apply_physics: std::option::Option<bool>) 
-> Result<(), Box<dyn std::error::Error>>

{let mut args = Vec::new();
let mut sig = String::from("(");
sig += "Lorg/bukkit/Material;";
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(val_type.into().jni_object().clone())});
args.push(val_1);
if let Some(a) = apply_physics {
sig += "Z";
let val_2 = jni::objects::JValueGen::Bool(a.into());
args.push(val_2);
}
sig += ")V";
let res = self.jni_ref().call_method(&self.jni_object(),"setType",sig.as_str(),args);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
/// Gets the face relation of this block compared to the given block.
/// 
/// For example:
/// <pre>{@code
/// Block current = world.getBlockAt(100, 100, 100);
/// Block target = world.getBlockAt(100, 101, 100);
/// current.getFace(target) == BlockFace.Up;
/// }</pre>
/// 
/// If the given block is not connected to this block, null may be returned
	fn get_face(&self,block: impl Into<crate::block::Block<'mc>>) 
-> Result<Option<crate::block::BlockFace<'mc>>, Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/block/Block;)Lorg/bukkit/block/BlockFace;");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(block.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"getFace",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {return Ok(None);}
Ok(
Some(
crate::block::BlockFace::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)?
)
)}
/// Captures the current state of this block. You may then cast that state
/// into any accepted type, such as Furnace or Sign.
/// 
/// The returned object will never be updated, and you are not guaranteed
/// that (for example) a sign is still a sign after you capture its state.
	fn state(&self) 
-> Result<crate::block::BlockState<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/block/BlockState;");
let res = self.jni_ref().call_method(&self.jni_object(),"getState",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::block::BlockState::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
/// Returns the biome that this block resides in
	fn biome(&self) 
-> Result<crate::block::Biome<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/block/Biome;");
let res = self.jni_ref().call_method(&self.jni_object(),"getBiome",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::block::Biome::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
/// Sets the biome that this block resides in
	fn set_biome(&self,bio: impl Into<crate::block::Biome<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/block/Biome;)V");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(bio.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"setBiome",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
/// Returns true if the block is being powered by Redstone.
	fn is_block_powered(&self) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("()Z");
let res = self.jni_ref().call_method(&self.jni_object(),"isBlockPowered",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
/// Returns true if the block is being indirectly powered by Redstone.
	fn is_block_indirectly_powered(&self) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("()Z");
let res = self.jni_ref().call_method(&self.jni_object(),"isBlockIndirectlyPowered",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
/// Returns true if the block face is being powered by Redstone.
	fn is_block_face_powered(&self,face: impl Into<crate::block::BlockFace<'mc>>) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/block/BlockFace;)Z");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(face.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"isBlockFacePowered",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
/// Returns true if the block face is being indirectly powered by Redstone.
	fn is_block_face_indirectly_powered(&self,face: impl Into<crate::block::BlockFace<'mc>>) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/block/BlockFace;)Z");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(face.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"isBlockFaceIndirectlyPowered",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
/// Returns the redstone power being provided to this block
	fn block_power(&self) 
-> Result<i32, Box<dyn std::error::Error>>

{let sig = String::from("()I");
let res = self.jni_ref().call_method(&self.jni_object(),"getBlockPower",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.i()?
)}
/// Checks if this block is empty.
/// 
/// A block is considered empty when {@link #getType()} returns {@link
/// Material#AIR}.
	fn is_empty(&self) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("()Z");
let res = self.jni_ref().call_method(&self.jni_object(),"isEmpty",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
/// Checks if this block is liquid.
/// 
/// A block is considered liquid when {@link #getType()} returns {@link
/// Material#WATER} or {@link Material#LAVA}.
	fn is_liquid(&self) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("()Z");
let res = self.jni_ref().call_method(&self.jni_object(),"isLiquid",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
/// Gets the temperature of this block.
/// 
/// If the raw biome temperature without adjusting for height effects is
/// required then please use {@link World#getTemperature(int, int)}.
	fn temperature(&self) 
-> Result<f64, Box<dyn std::error::Error>>

{let sig = String::from("()D");
let res = self.jni_ref().call_method(&self.jni_object(),"getTemperature",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.d()?
)}
/// Gets the humidity of the biome of this block
	fn humidity(&self) 
-> Result<f64, Box<dyn std::error::Error>>

{let sig = String::from("()D");
let res = self.jni_ref().call_method(&self.jni_object(),"getHumidity",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.d()?
)}
/// Returns the reaction of the block when moved by a piston
	fn piston_move_reaction(&self) 
-> Result<crate::block::PistonMoveReaction<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/block/PistonMoveReaction;");
let res = self.jni_ref().call_method(&self.jni_object(),"getPistonMoveReaction",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::block::PistonMoveReaction::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
/// Breaks the block and spawns items as if a player had digged it with a
/// specific tool
	fn break_naturally(&self,tool: std::option::Option<impl Into<crate::inventory::ItemStack<'mc>>>) 
-> Result<bool, Box<dyn std::error::Error>>

{let mut args = Vec::new();
let mut sig = String::from("(");
if let Some(a) = tool {
sig += "Lorg/bukkit/inventory/ItemStack;";
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(a.into().jni_object().clone())});
args.push(val_1);
}
sig += ")Z";
let res = self.jni_ref().call_method(&self.jni_object(),"breakNaturally",sig.as_str(),args);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
/// Simulate bone meal application to this block (if possible).
	fn apply_bone_meal(&self,face: impl Into<crate::block::BlockFace<'mc>>) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/block/BlockFace;)Z");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(face.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"applyBoneMeal",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
/// Returns a list of items which would drop by the entity destroying this
/// block with a specific tool
	fn get_drops(&self,tool: impl Into<crate::inventory::ItemStack<'mc>>,entity: std::option::Option<impl Into<crate::entity::Entity<'mc>>>) 
-> Result<Vec<crate::inventory::ItemStack<'mc>>, Box<dyn std::error::Error>>

{let mut args = Vec::new();
let mut sig = String::from("(");
sig += "Lorg/bukkit/inventory/ItemStack;";
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(tool.into().jni_object().clone())});
args.push(val_1);
if let Some(a) = entity {
sig += "Lorg/bukkit/entity/Entity;";
let val_2 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(a.into().jni_object().clone())});
args.push(val_2);
}
sig += ")Ljava/util/Collection;";
let res = self.jni_ref().call_method(&self.jni_object(),"getDrops",sig.as_str(),args);
let res = 
self.jni_ref().translate_error(res)?;
let mut new_vec = Vec::new();
let col = blackboxmc_java::util::JavaCollection::from_raw(&self.jni_ref(),res.l()?)?;let iter = col.iterator()?;
while iter.has_next()? {            let obj = iter.next()?;
new_vec.push(crate::inventory::ItemStack::from_raw(&self.jni_ref(),obj,)?);
};
Ok(
new_vec
)}
/// Returns if the given item is a preferred choice to break this Block.
/// In some cases this determines if a block will drop anything or extra
/// loot.
	fn is_preferred_tool(&self,tool: impl Into<crate::inventory::ItemStack<'mc>>) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/inventory/ItemStack;)Z");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(tool.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"isPreferredTool",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
/// Gets the speed at which the given player would break this block, taking
/// into account tools, potion effects, whether or not the player is in
/// water, enchantments, etc.
/// The returned value is the amount of progress made in breaking the block
/// each tick. When the total breaking progress reaches {@code 1.0f}, the
/// block is broken. Note that the break speed can change in the course of
/// breaking a block, e.g. if a potion effect is applied or expires, or the
/// player jumps/enters water.
	fn get_break_speed(&self,player: impl Into<crate::entity::Player<'mc>>) 
-> Result<f32, Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/entity/Player;)F");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(player.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"getBreakSpeed",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.f()?
)}
/// Checks if this block is passable.
/// 
/// A block is passable if it has no colliding parts that would prevent
/// players from moving through it.
/// 
/// Examples: Tall grass, flowers, signs, etc. are passable, but open doors,
/// fence gates, trap doors, etc. are not because they still have parts that
/// can be collided with.
	fn is_passable(&self) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("()Z");
let res = self.jni_ref().call_method(&self.jni_object(),"isPassable",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
/// Performs a ray trace that checks for collision with this specific block
/// in its current state using its precise collision shape.
	fn ray_trace(&self,start: impl Into<crate::Location<'mc>>,direction: impl Into<crate::util::Vector<'mc>>,max_distance: f64,fluid_collision_mode: impl Into<crate::FluidCollisionMode<'mc>>) 
-> Result<Option<crate::util::RayTraceResult<'mc>>, Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/Location;Lorg/bukkit/util/Vector;DLorg/bukkit/FluidCollisionMode;)Lorg/bukkit/util/RayTraceResult;");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(start.into().jni_object().clone())});
let val_2 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(direction.into().jni_object().clone())});
let val_3 = jni::objects::JValueGen::Double(max_distance);
let val_4 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(fluid_collision_mode.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"rayTrace",sig.as_str(),vec![jni::objects::JValueGen::from(val_1),jni::objects::JValueGen::from(val_2),jni::objects::JValueGen::from(val_3),jni::objects::JValueGen::from(val_4)]);
let res = 
self.jni_ref().translate_error(res)?;
if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {return Ok(None);}
Ok(
Some(
crate::util::RayTraceResult::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)?
)
)}
/// Gets the approximate bounding box for this block.
/// 
/// This isn't exact as some blocks {@link org.bukkit.block.data.type.Stairs}
/// contain many bounding boxes to establish their complete form.
/// Also, the box may not be exactly the same as the collision shape (such as
/// cactus, which is 16/16 of a block with 15/16 collisional bounds).
/// This method will return an empty bounding box if the geometric shape of
/// the block is empty (such as air blocks).
	fn bounding_box(&self) 
-> Result<crate::util::BoundingBox<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/util/BoundingBox;");
let res = self.jni_ref().call_method(&self.jni_object(),"getBoundingBox",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::util::BoundingBox::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
/// Gets the collision shape of this block.
	fn collision_shape(&self) 
-> Result<crate::util::VoxelShape<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/util/VoxelShape;");
let res = self.jni_ref().call_method(&self.jni_object(),"getCollisionShape",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::util::VoxelShape::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
/// Checks if this block is a valid placement location for the specified
/// block data.
	fn can_place(&self,data: impl Into<crate::block::data::BlockData<'mc>>) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/block/data/BlockData;)Z");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(data.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"canPlace",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
impl<'mc> Into<crate::metadata::Metadatable<'mc>> for Block<'mc>{

fn into(self) -> crate::metadata::Metadatable<'mc> {

crate::metadata::Metadatable::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting Block into crate::metadata::Metadatable")

   }
}
impl<'mc> crate::metadata::MetadatableTrait<'mc> for Block<'mc> {}
impl<'mc> Into<crate::Translatable<'mc>> for Block<'mc>{

fn into(self) -> crate::Translatable<'mc> {

crate::Translatable::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting Block into crate::Translatable")

   }
}
impl<'mc> crate::TranslatableTrait<'mc> for Block<'mc> {}
#[repr(C)]
pub struct BrewingStand<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

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
                return Err(eyre::eyre!(
                    "Tried to instantiate BrewingStand from null object.")
                .into());
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
    
impl<'mc> BrewingStandTrait<'mc> for BrewingStand<'mc> {}
pub trait BrewingStandTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {
/// How much time is left in the brewing cycle.
	fn brewing_time(&self) 
-> Result<i32, Box<dyn std::error::Error>>

{let sig = String::from("()I");
let res = self.jni_ref().call_method(&self.jni_object(),"getBrewingTime",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.i()?
)}
/// Set the time left before brewing completes.
	fn set_brewing_time(&self,brew_time: i32) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(I)V");
let val_1 = jni::objects::JValueGen::Int(brew_time);
let res = self.jni_ref().call_method(&self.jni_object(),"setBrewingTime",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
/// Get the level of current fuel for brewing.
	fn fuel_level(&self) 
-> Result<i32, Box<dyn std::error::Error>>

{let sig = String::from("()I");
let res = self.jni_ref().call_method(&self.jni_object(),"getFuelLevel",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.i()?
)}
/// Set the level of current fuel for brewing.
	fn set_fuel_level(&self,level: i32) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(I)V");
let val_1 = jni::objects::JValueGen::Int(level);
let res = self.jni_ref().call_method(&self.jni_object(),"setFuelLevel",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}

	fn inventory(&self) 
-> Result<crate::inventory::BrewerInventory<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/inventory/BrewerInventory;");
let res = self.jni_ref().call_method(&self.jni_object(),"getInventory",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::inventory::BrewerInventory::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}

	fn snapshot_inventory(&self) 
-> Result<crate::inventory::BrewerInventory<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/inventory/BrewerInventory;");
let res = self.jni_ref().call_method(&self.jni_object(),"getSnapshotInventory",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::inventory::BrewerInventory::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
impl<'mc> Into<crate::block::Container<'mc>> for BrewingStand<'mc>{

fn into(self) -> crate::block::Container<'mc> {

crate::block::Container::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting BrewingStand into crate::block::Container")

   }
}
impl<'mc> crate::block::ContainerTrait<'mc> for BrewingStand<'mc> {}
#[repr(C)]
pub struct Beacon<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

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
                return Err(eyre::eyre!(
                    "Tried to instantiate Beacon from null object.")
                .into());
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
    
impl<'mc> BeaconTrait<'mc> for Beacon<'mc> {}
pub trait BeaconTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {
/// Returns the list of players within the beacon's range of effect.
/// 
/// This will return an empty list if the block represented by this state is
/// no longer a beacon.
	fn entities_in_range(&self) 
-> Result<Vec<crate::entity::LivingEntity<'mc>>, Box<dyn std::error::Error>>

{let sig = String::from("()Ljava/util/Collection;");
let res = self.jni_ref().call_method(&self.jni_object(),"getEntitiesInRange",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
let mut new_vec = Vec::new();
let col = blackboxmc_java::util::JavaCollection::from_raw(&self.jni_ref(),res.l()?)?;let iter = col.iterator()?;
while iter.has_next()? {            let obj = iter.next()?;
new_vec.push(crate::entity::LivingEntity::from_raw(&self.jni_ref(),obj,)?);
};
Ok(
new_vec
)}
/// Returns the tier of the beacon pyramid (0-4). The tier refers to the
/// beacon's power level, based on how many layers of blocks are in the
/// pyramid. Tier 1 refers to a beacon with one layer of 9 blocks under it.
	fn tier(&self) 
-> Result<i32, Box<dyn std::error::Error>>

{let sig = String::from("()I");
let res = self.jni_ref().call_method(&self.jni_object(),"getTier",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.i()?
)}
/// Returns the primary effect set on the beacon
	fn primary_effect(&self) 
-> Result<Option<crate::potion::PotionEffect<'mc>>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/potion/PotionEffect;");
let res = self.jni_ref().call_method(&self.jni_object(),"getPrimaryEffect",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {return Ok(None);}
Ok(
Some(
crate::potion::PotionEffect::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)?
)
)}
/// Set the primary effect on this beacon, or null to clear.
	fn set_primary_effect(&self,effect: impl Into<crate::potion::PotionEffectType<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/potion/PotionEffectType;)V");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(effect.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"setPrimaryEffect",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
/// Returns the secondary effect set on the beacon.
	fn secondary_effect(&self) 
-> Result<Option<crate::potion::PotionEffect<'mc>>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/potion/PotionEffect;");
let res = self.jni_ref().call_method(&self.jni_object(),"getSecondaryEffect",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {return Ok(None);}
Ok(
Some(
crate::potion::PotionEffect::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)?
)
)}
/// Set the secondary effect on this beacon, or null to clear. Note that tier
/// must be &gt;= 4 for this effect to be active.
	fn set_secondary_effect(&self,effect: impl Into<crate::potion::PotionEffectType<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/potion/PotionEffectType;)V");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(effect.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"setSecondaryEffect",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
impl<'mc> Into<crate::block::TileState<'mc>> for Beacon<'mc>{

fn into(self) -> crate::block::TileState<'mc> {

crate::block::TileState::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting Beacon into crate::block::TileState")

   }
}
impl<'mc> crate::block::TileStateTrait<'mc> for Beacon<'mc> {}
impl<'mc> Into<crate::block::Lockable<'mc>> for Beacon<'mc>{

fn into(self) -> crate::block::Lockable<'mc> {

crate::block::Lockable::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting Beacon into crate::block::Lockable")

   }
}
impl<'mc> crate::block::LockableTrait<'mc> for Beacon<'mc> {}
impl<'mc> Into<crate::Nameable<'mc>> for Beacon<'mc>{

fn into(self) -> crate::Nameable<'mc> {

crate::Nameable::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting Beacon into crate::Nameable")

   }
}
impl<'mc> crate::NameableTrait<'mc> for Beacon<'mc> {}
#[repr(C)]
pub struct Banner<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

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
                return Err(eyre::eyre!(
                    "Tried to instantiate Banner from null object.")
                .into());
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
    
impl<'mc> BannerTrait<'mc> for Banner<'mc> {}
pub trait BannerTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {
/// Returns the base color for this banner
	fn base_color(&self) 
-> Result<crate::DyeColor<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/DyeColor;");
let res = self.jni_ref().call_method(&self.jni_object(),"getBaseColor",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::DyeColor::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
/// Sets the base color for this banner.
/// <b>Only valid for shield pseudo banners, otherwise base depends on block
/// type</b>
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
impl<'mc> Into<crate::block::TileState<'mc>> for Banner<'mc>{

fn into(self) -> crate::block::TileState<'mc> {

crate::block::TileState::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting Banner into crate::block::TileState")

   }
}
impl<'mc> crate::block::TileStateTrait<'mc> for Banner<'mc> {}
#[repr(C)]
pub struct Dropper<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

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
                return Err(eyre::eyre!(
                    "Tried to instantiate Dropper from null object.")
                .into());
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
    
impl<'mc> DropperTrait<'mc> for Dropper<'mc> {}
pub trait DropperTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {
/// Tries to drop a randomly selected item from the dropper's inventory,
/// following the normal behavior of a dropper.
/// 
/// Normal behavior of a dropper is as follows:
/// 
/// If the block that the dropper is facing is an InventoryHolder,
/// the randomly selected ItemStack is placed within that
/// Inventory in the first slot that's available, starting with 0 and
/// counting up.If the inventory is full, nothing happens.
/// 
/// If the block that the dropper is facing is not an InventoryHolder,
/// the randomly selected ItemStack is dropped on
/// the ground in the form of an {@link org.bukkit.entity.Item Item}.
/// 
/// If the block represented by this state is no longer a dropper, this will
/// do nothing.
	fn drop(&self) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("()V");
let res = self.jni_ref().call_method(&self.jni_object(),"drop",sig.as_str(),vec![]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
impl<'mc> Into<crate::block::Container<'mc>> for Dropper<'mc>{

fn into(self) -> crate::block::Container<'mc> {

crate::block::Container::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting Dropper into crate::block::Container")

   }
}
impl<'mc> crate::block::ContainerTrait<'mc> for Dropper<'mc> {}
impl<'mc> Into<crate::loot::Lootable<'mc>> for Dropper<'mc>{

fn into(self) -> crate::loot::Lootable<'mc> {

crate::loot::Lootable::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting Dropper into crate::loot::Lootable")

   }
}
impl<'mc> crate::loot::LootableTrait<'mc> for Dropper<'mc> {}
#[repr(C)]
pub struct Furnace<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

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
                return Err(eyre::eyre!(
                    "Tried to instantiate Furnace from null object.")
                .into());
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
    
impl<'mc> FurnaceTrait<'mc> for Furnace<'mc> {}
pub trait FurnaceTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {
/// Get burn time.
	fn burn_time(&self) 
-> Result<i16, Box<dyn std::error::Error>>

{let sig = String::from("()S");
let res = self.jni_ref().call_method(&self.jni_object(),"getBurnTime",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.s()?
)}
/// Set burn time.
/// A burn time greater than 0 will cause this block to be lit, whilst a time
/// less than 0 will extinguish it.
	fn set_burn_time(&self,burn_time: i16) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(S)V");
let val_1 = jni::objects::JValueGen::Short(burn_time);
let res = self.jni_ref().call_method(&self.jni_object(),"setBurnTime",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
/// Get cook time.
/// This is the amount of time the item has been cooking for.
	fn cook_time(&self) 
-> Result<i16, Box<dyn std::error::Error>>

{let sig = String::from("()S");
let res = self.jni_ref().call_method(&self.jni_object(),"getCookTime",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.s()?
)}
/// Set cook time.
/// This is the amount of time the item has been cooking for.
	fn set_cook_time(&self,cook_time: i16) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(S)V");
let val_1 = jni::objects::JValueGen::Short(cook_time);
let res = self.jni_ref().call_method(&self.jni_object(),"setCookTime",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
/// Get cook time total.
/// This is the amount of time the item is required to cook for.
	fn cook_time_total(&self) 
-> Result<i32, Box<dyn std::error::Error>>

{let sig = String::from("()I");
let res = self.jni_ref().call_method(&self.jni_object(),"getCookTimeTotal",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.i()?
)}
/// Set cook time.
/// This is the amount of time the item is required to cook for.
	fn set_cook_time_total(&self,cook_time_total: i32) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(I)V");
let val_1 = jni::objects::JValueGen::Int(cook_time_total);
let res = self.jni_ref().call_method(&self.jni_object(),"setCookTimeTotal",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
/// Get the recipes used in this furnace.
/// <b>Note:</b> These recipes used are reset when the result item is
/// manually taken from the furnace.
	fn recipes_used(&self) 
-> Result<blackboxmc_java::util::JavaMap<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Ljava/util/Map;");
let res = self.jni_ref().call_method(&self.jni_object(),"getRecipesUsed",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
blackboxmc_java::util::JavaMap::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}

	fn inventory(&self) 
-> Result<crate::inventory::FurnaceInventory<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/inventory/FurnaceInventory;");
let res = self.jni_ref().call_method(&self.jni_object(),"getInventory",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::inventory::FurnaceInventory::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}

	fn snapshot_inventory(&self) 
-> Result<crate::inventory::FurnaceInventory<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/inventory/FurnaceInventory;");
let res = self.jni_ref().call_method(&self.jni_object(),"getSnapshotInventory",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::inventory::FurnaceInventory::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
impl<'mc> Into<crate::block::Container<'mc>> for Furnace<'mc>{

fn into(self) -> crate::block::Container<'mc> {

crate::block::Container::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting Furnace into crate::block::Container")

   }
}
impl<'mc> crate::block::ContainerTrait<'mc> for Furnace<'mc> {}
#[repr(C)]
pub struct DecoratedPot<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

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
                return Err(eyre::eyre!(
                    "Tried to instantiate DecoratedPot from null object.")
                .into());
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
    
impl<'mc> DecoratedPotTrait<'mc> for DecoratedPot<'mc> {}
pub trait DecoratedPotTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {
/// Set the sherd on the provided side.
	fn set_sherd(&self,side: impl Into<crate::block::DecoratedPotSide<'mc>>,sherd: impl Into<crate::Material<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/block/DecoratedPot/Side;Lorg/bukkit/Material;)V");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(side.into().jni_object().clone())});
let val_2 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(sherd.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"setSherd",sig.as_str(),vec![jni::objects::JValueGen::from(val_1),jni::objects::JValueGen::from(val_2)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
/// Get the sherd on the provided side.
	fn get_sherd(&self,side: impl Into<crate::block::DecoratedPotSide<'mc>>) 
-> Result<crate::Material<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/block/DecoratedPot/Side;)Lorg/bukkit/Material;");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(side.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"getSherd",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
crate::Material::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
/// Gets a Map of all sides on this decorated pot and the sherds on them.
/// If a side does not have a specific sherd on it, {@link Material#BRICK}
/// will be the value of that side.
	fn sherds(&self) 
-> Result<blackboxmc_java::util::JavaMap<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Ljava/util/Map;");
let res = self.jni_ref().call_method(&self.jni_object(),"getSherds",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
blackboxmc_java::util::JavaMap::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
#[deprecated]
/// Gets the sherds on this decorated pot. For faces without a specific sherd, {@link Material#BRICK} is used in its place.
	fn shards(&self) 
-> Result<Vec<crate::Material<'mc>>, Box<dyn std::error::Error>>

{let sig = String::from("()Ljava/util/List;");
let res = self.jni_ref().call_method(&self.jni_object(),"getShards",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
let mut new_vec = Vec::new();
let list = blackboxmc_java::util::JavaList::from_raw(&self.jni_ref(), res.l()?)?;let iter = list.iterator()?;
while iter.has_next()? {            let obj = iter.next()?;
new_vec.push(crate::Material::from_raw(&self.jni_ref(),obj,)?);
};
Ok(
new_vec
)}

	fn inventory(&self) 
-> Result<crate::inventory::DecoratedPotInventory<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/inventory/DecoratedPotInventory;");
let res = self.jni_ref().call_method(&self.jni_object(),"getInventory",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::inventory::DecoratedPotInventory::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}

	fn snapshot_inventory(&self) 
-> Result<crate::inventory::DecoratedPotInventory<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/inventory/DecoratedPotInventory;");
let res = self.jni_ref().call_method(&self.jni_object(),"getSnapshotInventory",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::inventory::DecoratedPotInventory::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
impl<'mc> Into<crate::block::TileState<'mc>> for DecoratedPot<'mc>{

fn into(self) -> crate::block::TileState<'mc> {

crate::block::TileState::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting DecoratedPot into crate::block::TileState")

   }
}
impl<'mc> crate::block::TileStateTrait<'mc> for DecoratedPot<'mc> {}
impl<'mc> Into<crate::inventory::BlockInventoryHolder<'mc>> for DecoratedPot<'mc>{

fn into(self) -> crate::inventory::BlockInventoryHolder<'mc> {

crate::inventory::BlockInventoryHolder::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting DecoratedPot into crate::inventory::BlockInventoryHolder")

   }
}
impl<'mc> crate::inventory::BlockInventoryHolderTrait<'mc> for DecoratedPot<'mc> {}
#[repr(C)]
pub struct Lectern<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

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
                return Err(eyre::eyre!(
                    "Tried to instantiate Lectern from null object.")
                .into());
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
    
impl<'mc> LecternTrait<'mc> for Lectern<'mc> {}
pub trait LecternTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {
/// Get the current lectern page.
	fn page(&self) 
-> Result<i32, Box<dyn std::error::Error>>

{let sig = String::from("()I");
let res = self.jni_ref().call_method(&self.jni_object(),"getPage",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.i()?
)}
/// Set the current lectern page.
/// If the page is greater than the number of pages of the book currently in
/// the inventory, then behavior is undefined.
	fn set_page(&self,page: i32) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(I)V");
let val_1 = jni::objects::JValueGen::Int(page);
let res = self.jni_ref().call_method(&self.jni_object(),"setPage",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}

	fn inventory(&self) 
-> Result<crate::inventory::Inventory<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/inventory/Inventory;");
let res = self.jni_ref().call_method(&self.jni_object(),"getInventory",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::inventory::Inventory::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}

	fn snapshot_inventory(&self) 
-> Result<crate::inventory::Inventory<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/inventory/Inventory;");
let res = self.jni_ref().call_method(&self.jni_object(),"getSnapshotInventory",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::inventory::Inventory::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
impl<'mc> Into<crate::block::TileState<'mc>> for Lectern<'mc>{

fn into(self) -> crate::block::TileState<'mc> {

crate::block::TileState::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting Lectern into crate::block::TileState")

   }
}
impl<'mc> crate::block::TileStateTrait<'mc> for Lectern<'mc> {}
impl<'mc> Into<crate::inventory::BlockInventoryHolder<'mc>> for Lectern<'mc>{

fn into(self) -> crate::inventory::BlockInventoryHolder<'mc> {

crate::inventory::BlockInventoryHolder::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting Lectern into crate::inventory::BlockInventoryHolder")

   }
}
impl<'mc> crate::inventory::BlockInventoryHolderTrait<'mc> for Lectern<'mc> {}
pub enum BlockSupport<'mc> {
	Full {inner: BlockSupportStruct<'mc>},
	Center {inner: BlockSupportStruct<'mc>},
	Rigid {inner: BlockSupportStruct<'mc>},
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

        impl<'mc> BlockSupportTrait<'mc> for BlockSupport<'mc> {}
        
        pub trait BlockSupportTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc>  {
            fn value_of(
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
                    
"FULL" => Ok(BlockSupport::Full { inner: BlockSupportStruct::from_raw(env,obj)?}),
"CENTER" => Ok(BlockSupport::Center { inner: BlockSupportStruct::from_raw(env,obj)?}),
"RIGID" => Ok(BlockSupport::Rigid { inner: BlockSupportStruct::from_raw(env,obj)?}),

                    _ => Err(eyre::eyre!("String gaven for variant was invalid").into())
                }
            }
        }
        
#[repr(C)]
pub struct BlockSupportStruct<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

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
                return Err(eyre::eyre!(
                    "Tried to instantiate BlockSupport from null object.")
                .into());
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
                    "FULL" => Ok(BlockSupport::Full { inner: BlockSupportStruct::from_raw(env,obj)?}),"CENTER" => Ok(BlockSupport::Center { inner: BlockSupportStruct::from_raw(env,obj)?}),"RIGID" => Ok(BlockSupport::Rigid { inner: BlockSupportStruct::from_raw(env,obj)?}),_ => Err(eyre::eyre!("String gaven for variant was invalid").into())}
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
                return Err(eyre::eyre!(
                    "Tried to instantiate BlockSupportStruct from null object.")
                .into());
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

	fn values(jni: &blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::block::BlockSupport<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/block/BlockSupport;");
let cls = jni.find_class("org/bukkit/block/BlockSupport"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.call_static_method(cls,"values",
sig.as_str(),vec![]);
let res = 
jni.translate_error(res)?;
let obj = res.l()?;
crate::block::BlockSupport::from_raw(&jni,obj
)}

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
#[repr(C)]
pub struct Barrel<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

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
                return Err(eyre::eyre!(
                    "Tried to instantiate Barrel from null object.")
                .into());
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
    
impl<'mc> BarrelTrait<'mc> for Barrel<'mc> {}
pub trait BarrelTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
impl<'mc> Into<crate::block::Container<'mc>> for Barrel<'mc>{

fn into(self) -> crate::block::Container<'mc> {

crate::block::Container::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting Barrel into crate::block::Container")

   }
}
impl<'mc> crate::block::ContainerTrait<'mc> for Barrel<'mc> {}
impl<'mc> Into<crate::loot::Lootable<'mc>> for Barrel<'mc>{

fn into(self) -> crate::loot::Lootable<'mc> {

crate::loot::Lootable::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting Barrel into crate::loot::Lootable")

   }
}
impl<'mc> crate::loot::LootableTrait<'mc> for Barrel<'mc> {}
impl<'mc> Into<crate::block::Lidded<'mc>> for Barrel<'mc>{

fn into(self) -> crate::block::Lidded<'mc> {

crate::block::Lidded::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting Barrel into crate::block::Lidded")

   }
}
impl<'mc> crate::block::LiddedTrait<'mc> for Barrel<'mc> {}
#[repr(C)]
pub struct Jigsaw<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

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
                return Err(eyre::eyre!(
                    "Tried to instantiate Jigsaw from null object.")
                .into());
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
    
impl<'mc> JigsawTrait<'mc> for Jigsaw<'mc> {}
pub trait JigsawTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
impl<'mc> Into<crate::block::TileState<'mc>> for Jigsaw<'mc>{

fn into(self) -> crate::block::TileState<'mc> {

crate::block::TileState::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting Jigsaw into crate::block::TileState")

   }
}
impl<'mc> crate::block::TileStateTrait<'mc> for Jigsaw<'mc> {}
#[repr(C)]
pub struct SculkSensor<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

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
                return Err(eyre::eyre!(
                    "Tried to instantiate SculkSensor from null object.")
                .into());
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
    
impl<'mc> SculkSensorTrait<'mc> for SculkSensor<'mc> {}
pub trait SculkSensorTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {
/// Gets the last vibration frequency of this sensor.
/// Different activities detected by the sensor will produce different
/// frequencies and dictate the output of connected comparators.
	fn last_vibration_frequency(&self) 
-> Result<i32, Box<dyn std::error::Error>>

{let sig = String::from("()I");
let res = self.jni_ref().call_method(&self.jni_object(),"getLastVibrationFrequency",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.i()?
)}
/// Sets the last vibration frequency of this sensor.
/// Different activities detected by the sensor will produce different
/// frequencies and dictate the output of connected comparators.
	fn set_last_vibration_frequency(&self,last_vibration_frequency: i32) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(I)V");
let val_1 = jni::objects::JValueGen::Int(last_vibration_frequency);
let res = self.jni_ref().call_method(&self.jni_object(),"setLastVibrationFrequency",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
impl<'mc> Into<crate::block::TileState<'mc>> for SculkSensor<'mc>{

fn into(self) -> crate::block::TileState<'mc> {

crate::block::TileState::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting SculkSensor into crate::block::TileState")

   }
}
impl<'mc> crate::block::TileStateTrait<'mc> for SculkSensor<'mc> {}
#[repr(C)]
pub struct Conduit<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

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
                return Err(eyre::eyre!(
                    "Tried to instantiate Conduit from null object.")
                .into());
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
    
impl<'mc> ConduitTrait<'mc> for Conduit<'mc> {}
pub trait ConduitTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {
/// Checks whether or not this conduit is active.
/// 
/// A conduit is considered active if there are at least 16 valid frame
/// blocks surrounding it and the conduit is surrounded by a 3x3x3 area of
/// water source blocks (or waterlogged blocks), at which point its animation
/// will activate, start spinning, and apply effects to nearby players.
	fn is_active(&self) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("()Z");
let res = self.jni_ref().call_method(&self.jni_object(),"isActive",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
/// Get whether or not this conduit is actively hunting for nearby hostile
/// creatures.
/// 
/// A conduit will hunt if it is active (see {@link #isActive()}) and its
/// frame is complete (it is surrounded by at least 42 valid frame blocks).
/// While hunting, the {@link #getTarget()
/// conduit's target}, if within its {@link #getHuntingArea() hunting area},
/// will be damaged every 2 seconds.
	fn is_hunting(&self) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("()Z");
let res = self.jni_ref().call_method(&self.jni_object(),"isHunting",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
/// Get a {@link Collection} of all {@link Block Blocks} that make up the
/// frame of this conduit. The returned collection will contain only blocks
/// that match the types required by the conduit to make up a valid frame,
/// <strong>not</strong> the blocks at which the conduit is searching,
/// meaning it will be of variable size depending on how many valid frames
/// are surrounding the conduit at the time of invocation.
	fn frame_blocks(&self) 
-> Result<Vec<crate::block::Block<'mc>>, Box<dyn std::error::Error>>

{let sig = String::from("()Ljava/util/Collection;");
let res = self.jni_ref().call_method(&self.jni_object(),"getFrameBlocks",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
let mut new_vec = Vec::new();
let col = blackboxmc_java::util::JavaCollection::from_raw(&self.jni_ref(),res.l()?)?;let iter = col.iterator()?;
while iter.has_next()? {            let obj = iter.next()?;
new_vec.push(crate::block::Block::from_raw(&self.jni_ref(),obj,)?);
};
Ok(
new_vec
)}
/// Get the amount of valid frame blocks that are currently surrounding the
/// conduit.
	fn frame_block_count(&self) 
-> Result<i32, Box<dyn std::error::Error>>

{let sig = String::from("()I");
let res = self.jni_ref().call_method(&self.jni_object(),"getFrameBlockCount",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.i()?
)}
/// Get the range (measured in blocks) within which players will receive the
/// conduit's benefits.
	fn range(&self) 
-> Result<i32, Box<dyn std::error::Error>>

{let sig = String::from("()I");
let res = self.jni_ref().call_method(&self.jni_object(),"getRange",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.i()?
)}
/// Set the conduit's hunting target.
/// 
/// Note that the value set by this method may be overwritten by the
/// conduit's periodic hunting logic. If the target is ever set to
/// {@code null}, the conduit will continue to look for a new target.
/// Additionally, if the target is set to an entity that does not meet a
/// conduit's hunting conditions (e.g. the entity is not within the
/// {@link #getHuntingArea() hunting area}, has already been killed, etc.)
/// then the passed entity will be ignored and the conduit will also continue
/// to look for a new target.
	fn set_target(&self,target: impl Into<crate::entity::LivingEntity<'mc>>) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/entity/LivingEntity;)Z");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(target.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"setTarget",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
/// Get the conduit's hunting target.
	fn target(&self) 
-> Result<Option<crate::entity::LivingEntity<'mc>>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/entity/LivingEntity;");
let res = self.jni_ref().call_method(&self.jni_object(),"getTarget",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {return Ok(None);}
Ok(
Some(
crate::entity::LivingEntity::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)?
)
)}
/// Check whether or not this conduit has an active (alive) hunting target.
	fn has_target(&self) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("()Z");
let res = self.jni_ref().call_method(&self.jni_object(),"hasTarget",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
/// Get a {@link BoundingBox} (relative to real-world coordinates) in which
/// the conduit will search for hostile entities to target.
	fn hunting_area(&self) 
-> Result<crate::util::BoundingBox<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/util/BoundingBox;");
let res = self.jni_ref().call_method(&self.jni_object(),"getHuntingArea",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::util::BoundingBox::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
impl<'mc> Into<crate::block::TileState<'mc>> for Conduit<'mc>{

fn into(self) -> crate::block::TileState<'mc> {

crate::block::TileState::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting Conduit into crate::block::TileState")

   }
}
impl<'mc> crate::block::TileStateTrait<'mc> for Conduit<'mc> {}
#[repr(C)]
pub struct DoubleChest<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

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
                return Err(eyre::eyre!(
                    "Tried to instantiate DoubleChest from null object.")
                .into());
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
    
impl<'mc> DoubleChestTrait<'mc> for DoubleChest<'mc> {}
pub trait DoubleChestTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {

	fn new(jni: &blackboxmc_general::SharedJNIEnv<'mc>,chest: impl Into<crate::inventory::DoubleChestInventory<'mc>>) 
-> Result<crate::block::DoubleChest<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/inventory/DoubleChestInventory;)V");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(chest.into().jni_object().clone())});
let cls = jni.find_class("org/bukkit/block/DoubleChest"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.new_object(cls,
sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
jni.translate_error_no_gen(res)?;
crate::block::DoubleChest::from_raw(&jni,res
)}

	fn inventory(&self) 
-> Result<crate::inventory::Inventory<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/inventory/Inventory;");
let res = self.jni_ref().call_method(&self.jni_object(),"getInventory",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::inventory::Inventory::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}

	fn left_side(&self) 
-> Result<Option<crate::inventory::InventoryHolder<'mc>>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/inventory/InventoryHolder;");
let res = self.jni_ref().call_method(&self.jni_object(),"getLeftSide",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {return Ok(None);}
Ok(
Some(
crate::inventory::InventoryHolder::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)?
)
)}

	fn right_side(&self) 
-> Result<Option<crate::inventory::InventoryHolder<'mc>>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/inventory/InventoryHolder;");
let res = self.jni_ref().call_method(&self.jni_object(),"getRightSide",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {return Ok(None);}
Ok(
Some(
crate::inventory::InventoryHolder::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)?
)
)}

	fn location(&self) 
-> Result<crate::Location<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/Location;");
let res = self.jni_ref().call_method(&self.jni_object(),"getLocation",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::Location::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}

	fn world(&self) 
-> Result<Option<crate::World<'mc>>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/World;");
let res = self.jni_ref().call_method(&self.jni_object(),"getWorld",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {return Ok(None);}
Ok(
Some(
crate::World::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)?
)
)}

	fn x(&self) 
-> Result<f64, Box<dyn std::error::Error>>

{let sig = String::from("()D");
let res = self.jni_ref().call_method(&self.jni_object(),"getX",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.d()?
)}

	fn y(&self) 
-> Result<f64, Box<dyn std::error::Error>>

{let sig = String::from("()D");
let res = self.jni_ref().call_method(&self.jni_object(),"getY",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.d()?
)}

	fn z(&self) 
-> Result<f64, Box<dyn std::error::Error>>

{let sig = String::from("()D");
let res = self.jni_ref().call_method(&self.jni_object(),"getZ",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.d()?
)}

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
impl<'mc> Into<crate::inventory::InventoryHolder<'mc>> for DoubleChest<'mc>{

fn into(self) -> crate::inventory::InventoryHolder<'mc> {

crate::inventory::InventoryHolder::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting DoubleChest into crate::inventory::InventoryHolder")

   }
}
impl<'mc> crate::inventory::InventoryHolderTrait<'mc> for DoubleChest<'mc> {}
#[repr(C)]
pub struct BlockState<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

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
                return Err(eyre::eyre!(
                    "Tried to instantiate BlockState from null object.")
                .into());
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
    
impl<'mc> BlockStateTrait<'mc> for BlockState<'mc> {}
pub trait BlockStateTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {
/// Gets the block represented by this block state.
	fn block(&self) 
-> Result<crate::block::Block<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/block/Block;");
let res = self.jni_ref().call_method(&self.jni_object(),"getBlock",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::block::Block::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
/// Gets the metadata for this block state.
	fn data(&self) 
-> Result<crate::material::MaterialData<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/material/MaterialData;");
let res = self.jni_ref().call_method(&self.jni_object(),"getData",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::material::MaterialData::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
/// Gets the data for this block state.
	fn block_data(&self) 
-> Result<crate::block::data::BlockData<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/block/data/BlockData;");
let res = self.jni_ref().call_method(&self.jni_object(),"getBlockData",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::block::data::BlockData::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
/// Copies the state to another block as an unplaced BlockState.
	fn copy(&self,location: std::option::Option<impl Into<crate::Location<'mc>>>) 
-> Result<crate::block::BlockState<'mc>, Box<dyn std::error::Error>>

{let mut args = Vec::new();
let mut sig = String::from("(");
if let Some(a) = location {
sig += "Lorg/bukkit/Location;";
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(a.into().jni_object().clone())});
args.push(val_1);
}
sig += ")Lorg/bukkit/block/BlockState;";
let res = self.jni_ref().call_method(&self.jni_object(),"copy",sig.as_str(),args);
let res = 
self.jni_ref().translate_error(res)?;
crate::block::BlockState::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
/// Gets the type of this block state.
	fn get_type(&self) 
-> Result<crate::Material<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/Material;");
let res = self.jni_ref().call_method(&self.jni_object(),"getType",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::Material::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
/// Gets the current light level of the block represented by this block state.
	fn light_level(&self) 
-> Result<i8, Box<dyn std::error::Error>>

{let sig = String::from("()B");
let res = self.jni_ref().call_method(&self.jni_object(),"getLightLevel",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.b()?
)}
/// Gets the world which contains the block represented by this block state.
	fn world(&self) 
-> Result<crate::World<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/World;");
let res = self.jni_ref().call_method(&self.jni_object(),"getWorld",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::World::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
/// Gets the x-coordinate of this block state.
	fn x(&self) 
-> Result<i32, Box<dyn std::error::Error>>

{let sig = String::from("()I");
let res = self.jni_ref().call_method(&self.jni_object(),"getX",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.i()?
)}
/// Gets the y-coordinate of this block state.
	fn y(&self) 
-> Result<i32, Box<dyn std::error::Error>>

{let sig = String::from("()I");
let res = self.jni_ref().call_method(&self.jni_object(),"getY",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.i()?
)}
/// Gets the z-coordinate of this block state.
	fn z(&self) 
-> Result<i32, Box<dyn std::error::Error>>

{let sig = String::from("()I");
let res = self.jni_ref().call_method(&self.jni_object(),"getZ",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.i()?
)}
/// Stores the location of this block state in the provided Location object.
/// 
/// If the provided Location is null this method does nothing and returns
/// null.
/// 
/// If this block state is not placed the location's world will be null!
	fn get_location(&self,loc: impl Into<crate::Location<'mc>>) 
-> Result<Option<crate::Location<'mc>>, Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/Location;)Lorg/bukkit/Location;");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(loc.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"getLocation",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {return Ok(None);}
Ok(
Some(
crate::Location::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)?
)
)}
/// Gets the chunk which contains the block represented by this block state.
	fn chunk(&self) 
-> Result<crate::Chunk<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/Chunk;");
let res = self.jni_ref().call_method(&self.jni_object(),"getChunk",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::Chunk::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
/// Sets the metadata for this block state.
	fn set_data(&self,data: impl Into<crate::material::MaterialData<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/material/MaterialData;)V");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(data.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"setData",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
/// Sets the data for this block state.
	fn set_block_data(&self,data: impl Into<crate::block::data::BlockData<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/block/data/BlockData;)V");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(data.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"setBlockData",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
/// Sets the type of this block state.
	fn set_type(&self,val_type: impl Into<crate::Material<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/Material;)V");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(val_type.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"setType",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
/// Attempts to update the block represented by this state, setting it to
/// the new values as defined by this state.
/// 
/// If this state is not placed, this will have no effect and return true.
/// 
/// Unless force is true, this will not modify the state of a block if it
/// is no longer the same type as it was when this state was taken. It will
/// return false in this eventuality.
/// 
/// If force is true, it will set the type of the block to match the new
/// state, set the state data and then return true.
/// 
/// If applyPhysics is true, it will trigger a physics update on
/// surrounding blocks which could cause them to update or disappear.
	fn update(&self,force: std::option::Option<bool>,apply_physics: std::option::Option<bool>) 
-> Result<bool, Box<dyn std::error::Error>>

{let mut args = Vec::new();
let mut sig = String::from("(");
if let Some(a) = force {
sig += "Z";
let val_1 = jni::objects::JValueGen::Bool(a.into());
args.push(val_1);
}
if let Some(a) = apply_physics {
sig += "Z";
let val_2 = jni::objects::JValueGen::Bool(a.into());
args.push(val_2);
}
sig += ")Z";
let res = self.jni_ref().call_method(&self.jni_object(),"update",sig.as_str(),args);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
#[deprecated]

	fn raw_data(&self) 
-> Result<i8, Box<dyn std::error::Error>>

{let sig = String::from("()B");
let res = self.jni_ref().call_method(&self.jni_object(),"getRawData",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.b()?
)}
#[deprecated]

	fn set_raw_data(&self,data: i8) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(B)V");
let val_1 = jni::objects::JValueGen::Byte(data);
let res = self.jni_ref().call_method(&self.jni_object(),"setRawData",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
/// Returns whether this state is placed in the world.
/// 
/// Some methods will not work if the block state isn't
/// placed in the world.
	fn is_placed(&self) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("()Z");
let res = self.jni_ref().call_method(&self.jni_object(),"isPlaced",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
impl<'mc> Into<crate::metadata::Metadatable<'mc>> for BlockState<'mc>{

fn into(self) -> crate::metadata::Metadatable<'mc> {

crate::metadata::Metadatable::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting BlockState into crate::metadata::Metadatable")

   }
}
impl<'mc> crate::metadata::MetadatableTrait<'mc> for BlockState<'mc> {}
pub enum Biome<'mc> {
	Ocean {inner: BiomeStruct<'mc>},
	Plains {inner: BiomeStruct<'mc>},
	Desert {inner: BiomeStruct<'mc>},
	WindsweptHills {inner: BiomeStruct<'mc>},
	Forest {inner: BiomeStruct<'mc>},
	Taiga {inner: BiomeStruct<'mc>},
	Swamp {inner: BiomeStruct<'mc>},
	MangroveSwamp {inner: BiomeStruct<'mc>},
	River {inner: BiomeStruct<'mc>},
	NetherWastes {inner: BiomeStruct<'mc>},
	TheEnd {inner: BiomeStruct<'mc>},
	FrozenOcean {inner: BiomeStruct<'mc>},
	FrozenRiver {inner: BiomeStruct<'mc>},
	SnowyPlains {inner: BiomeStruct<'mc>},
	MushroomFields {inner: BiomeStruct<'mc>},
	Beach {inner: BiomeStruct<'mc>},
	Jungle {inner: BiomeStruct<'mc>},
	SparseJungle {inner: BiomeStruct<'mc>},
	DeepOcean {inner: BiomeStruct<'mc>},
	StonyShore {inner: BiomeStruct<'mc>},
	SnowyBeach {inner: BiomeStruct<'mc>},
	BirchForest {inner: BiomeStruct<'mc>},
	DarkForest {inner: BiomeStruct<'mc>},
	SnowyTaiga {inner: BiomeStruct<'mc>},
	OldGrowthPineTaiga {inner: BiomeStruct<'mc>},
	WindsweptForest {inner: BiomeStruct<'mc>},
	Savanna {inner: BiomeStruct<'mc>},
	SavannaPlateau {inner: BiomeStruct<'mc>},
	Badlands {inner: BiomeStruct<'mc>},
	WoodedBadlands {inner: BiomeStruct<'mc>},
	SmallEndIslands {inner: BiomeStruct<'mc>},
	EndMidlands {inner: BiomeStruct<'mc>},
	EndHighlands {inner: BiomeStruct<'mc>},
	EndBarrens {inner: BiomeStruct<'mc>},
	WarmOcean {inner: BiomeStruct<'mc>},
	LukewarmOcean {inner: BiomeStruct<'mc>},
	ColdOcean {inner: BiomeStruct<'mc>},
	DeepLukewarmOcean {inner: BiomeStruct<'mc>},
	DeepColdOcean {inner: BiomeStruct<'mc>},
	DeepFrozenOcean {inner: BiomeStruct<'mc>},
	TheVoid {inner: BiomeStruct<'mc>},
	SunflowerPlains {inner: BiomeStruct<'mc>},
	WindsweptGravellyHills {inner: BiomeStruct<'mc>},
	FlowerForest {inner: BiomeStruct<'mc>},
	IceSpikes {inner: BiomeStruct<'mc>},
	OldGrowthBirchForest {inner: BiomeStruct<'mc>},
	OldGrowthSpruceTaiga {inner: BiomeStruct<'mc>},
	WindsweptSavanna {inner: BiomeStruct<'mc>},
	ErodedBadlands {inner: BiomeStruct<'mc>},
	BambooJungle {inner: BiomeStruct<'mc>},
	SoulSandValley {inner: BiomeStruct<'mc>},
	CrimsonForest {inner: BiomeStruct<'mc>},
	WarpedForest {inner: BiomeStruct<'mc>},
	BasaltDeltas {inner: BiomeStruct<'mc>},
	DripstoneCaves {inner: BiomeStruct<'mc>},
	LushCaves {inner: BiomeStruct<'mc>},
	DeepDark {inner: BiomeStruct<'mc>},
	Meadow {inner: BiomeStruct<'mc>},
	Grove {inner: BiomeStruct<'mc>},
	SnowySlopes {inner: BiomeStruct<'mc>},
	FrozenPeaks {inner: BiomeStruct<'mc>},
	JaggedPeaks {inner: BiomeStruct<'mc>},
	StonyPeaks {inner: BiomeStruct<'mc>},
	CherryGrove {inner: BiomeStruct<'mc>},
	Custom {inner: BiomeStruct<'mc>},
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

        impl<'mc> BiomeTrait<'mc> for Biome<'mc> {}
        
        pub trait BiomeTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc>  {
            fn value_of(
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
                    
"OCEAN" => Ok(Biome::Ocean { inner: BiomeStruct::from_raw(env,obj)?}),
"PLAINS" => Ok(Biome::Plains { inner: BiomeStruct::from_raw(env,obj)?}),
"DESERT" => Ok(Biome::Desert { inner: BiomeStruct::from_raw(env,obj)?}),
"WINDSWEPT_HILLS" => Ok(Biome::WindsweptHills { inner: BiomeStruct::from_raw(env,obj)?}),
"FOREST" => Ok(Biome::Forest { inner: BiomeStruct::from_raw(env,obj)?}),
"TAIGA" => Ok(Biome::Taiga { inner: BiomeStruct::from_raw(env,obj)?}),
"SWAMP" => Ok(Biome::Swamp { inner: BiomeStruct::from_raw(env,obj)?}),
"MANGROVE_SWAMP" => Ok(Biome::MangroveSwamp { inner: BiomeStruct::from_raw(env,obj)?}),
"RIVER" => Ok(Biome::River { inner: BiomeStruct::from_raw(env,obj)?}),
"NETHER_WASTES" => Ok(Biome::NetherWastes { inner: BiomeStruct::from_raw(env,obj)?}),
"THE_END" => Ok(Biome::TheEnd { inner: BiomeStruct::from_raw(env,obj)?}),
"FROZEN_OCEAN" => Ok(Biome::FrozenOcean { inner: BiomeStruct::from_raw(env,obj)?}),
"FROZEN_RIVER" => Ok(Biome::FrozenRiver { inner: BiomeStruct::from_raw(env,obj)?}),
"SNOWY_PLAINS" => Ok(Biome::SnowyPlains { inner: BiomeStruct::from_raw(env,obj)?}),
"MUSHROOM_FIELDS" => Ok(Biome::MushroomFields { inner: BiomeStruct::from_raw(env,obj)?}),
"BEACH" => Ok(Biome::Beach { inner: BiomeStruct::from_raw(env,obj)?}),
"JUNGLE" => Ok(Biome::Jungle { inner: BiomeStruct::from_raw(env,obj)?}),
"SPARSE_JUNGLE" => Ok(Biome::SparseJungle { inner: BiomeStruct::from_raw(env,obj)?}),
"DEEP_OCEAN" => Ok(Biome::DeepOcean { inner: BiomeStruct::from_raw(env,obj)?}),
"STONY_SHORE" => Ok(Biome::StonyShore { inner: BiomeStruct::from_raw(env,obj)?}),
"SNOWY_BEACH" => Ok(Biome::SnowyBeach { inner: BiomeStruct::from_raw(env,obj)?}),
"BIRCH_FOREST" => Ok(Biome::BirchForest { inner: BiomeStruct::from_raw(env,obj)?}),
"DARK_FOREST" => Ok(Biome::DarkForest { inner: BiomeStruct::from_raw(env,obj)?}),
"SNOWY_TAIGA" => Ok(Biome::SnowyTaiga { inner: BiomeStruct::from_raw(env,obj)?}),
"OLD_GROWTH_PINE_TAIGA" => Ok(Biome::OldGrowthPineTaiga { inner: BiomeStruct::from_raw(env,obj)?}),
"WINDSWEPT_FOREST" => Ok(Biome::WindsweptForest { inner: BiomeStruct::from_raw(env,obj)?}),
"SAVANNA" => Ok(Biome::Savanna { inner: BiomeStruct::from_raw(env,obj)?}),
"SAVANNA_PLATEAU" => Ok(Biome::SavannaPlateau { inner: BiomeStruct::from_raw(env,obj)?}),
"BADLANDS" => Ok(Biome::Badlands { inner: BiomeStruct::from_raw(env,obj)?}),
"WOODED_BADLANDS" => Ok(Biome::WoodedBadlands { inner: BiomeStruct::from_raw(env,obj)?}),
"SMALL_END_ISLANDS" => Ok(Biome::SmallEndIslands { inner: BiomeStruct::from_raw(env,obj)?}),
"END_MIDLANDS" => Ok(Biome::EndMidlands { inner: BiomeStruct::from_raw(env,obj)?}),
"END_HIGHLANDS" => Ok(Biome::EndHighlands { inner: BiomeStruct::from_raw(env,obj)?}),
"END_BARRENS" => Ok(Biome::EndBarrens { inner: BiomeStruct::from_raw(env,obj)?}),
"WARM_OCEAN" => Ok(Biome::WarmOcean { inner: BiomeStruct::from_raw(env,obj)?}),
"LUKEWARM_OCEAN" => Ok(Biome::LukewarmOcean { inner: BiomeStruct::from_raw(env,obj)?}),
"COLD_OCEAN" => Ok(Biome::ColdOcean { inner: BiomeStruct::from_raw(env,obj)?}),
"DEEP_LUKEWARM_OCEAN" => Ok(Biome::DeepLukewarmOcean { inner: BiomeStruct::from_raw(env,obj)?}),
"DEEP_COLD_OCEAN" => Ok(Biome::DeepColdOcean { inner: BiomeStruct::from_raw(env,obj)?}),
"DEEP_FROZEN_OCEAN" => Ok(Biome::DeepFrozenOcean { inner: BiomeStruct::from_raw(env,obj)?}),
"THE_VOID" => Ok(Biome::TheVoid { inner: BiomeStruct::from_raw(env,obj)?}),
"SUNFLOWER_PLAINS" => Ok(Biome::SunflowerPlains { inner: BiomeStruct::from_raw(env,obj)?}),
"WINDSWEPT_GRAVELLY_HILLS" => Ok(Biome::WindsweptGravellyHills { inner: BiomeStruct::from_raw(env,obj)?}),
"FLOWER_FOREST" => Ok(Biome::FlowerForest { inner: BiomeStruct::from_raw(env,obj)?}),
"ICE_SPIKES" => Ok(Biome::IceSpikes { inner: BiomeStruct::from_raw(env,obj)?}),
"OLD_GROWTH_BIRCH_FOREST" => Ok(Biome::OldGrowthBirchForest { inner: BiomeStruct::from_raw(env,obj)?}),
"OLD_GROWTH_SPRUCE_TAIGA" => Ok(Biome::OldGrowthSpruceTaiga { inner: BiomeStruct::from_raw(env,obj)?}),
"WINDSWEPT_SAVANNA" => Ok(Biome::WindsweptSavanna { inner: BiomeStruct::from_raw(env,obj)?}),
"ERODED_BADLANDS" => Ok(Biome::ErodedBadlands { inner: BiomeStruct::from_raw(env,obj)?}),
"BAMBOO_JUNGLE" => Ok(Biome::BambooJungle { inner: BiomeStruct::from_raw(env,obj)?}),
"SOUL_SAND_VALLEY" => Ok(Biome::SoulSandValley { inner: BiomeStruct::from_raw(env,obj)?}),
"CRIMSON_FOREST" => Ok(Biome::CrimsonForest { inner: BiomeStruct::from_raw(env,obj)?}),
"WARPED_FOREST" => Ok(Biome::WarpedForest { inner: BiomeStruct::from_raw(env,obj)?}),
"BASALT_DELTAS" => Ok(Biome::BasaltDeltas { inner: BiomeStruct::from_raw(env,obj)?}),
"DRIPSTONE_CAVES" => Ok(Biome::DripstoneCaves { inner: BiomeStruct::from_raw(env,obj)?}),
"LUSH_CAVES" => Ok(Biome::LushCaves { inner: BiomeStruct::from_raw(env,obj)?}),
"DEEP_DARK" => Ok(Biome::DeepDark { inner: BiomeStruct::from_raw(env,obj)?}),
"MEADOW" => Ok(Biome::Meadow { inner: BiomeStruct::from_raw(env,obj)?}),
"GROVE" => Ok(Biome::Grove { inner: BiomeStruct::from_raw(env,obj)?}),
"SNOWY_SLOPES" => Ok(Biome::SnowySlopes { inner: BiomeStruct::from_raw(env,obj)?}),
"FROZEN_PEAKS" => Ok(Biome::FrozenPeaks { inner: BiomeStruct::from_raw(env,obj)?}),
"JAGGED_PEAKS" => Ok(Biome::JaggedPeaks { inner: BiomeStruct::from_raw(env,obj)?}),
"STONY_PEAKS" => Ok(Biome::StonyPeaks { inner: BiomeStruct::from_raw(env,obj)?}),
"CHERRY_GROVE" => Ok(Biome::CherryGrove { inner: BiomeStruct::from_raw(env,obj)?}),
"CUSTOM" => Ok(Biome::Custom { inner: BiomeStruct::from_raw(env,obj)?}),

                    _ => Err(eyre::eyre!("String gaven for variant was invalid").into())
                }
            }
        }
        
#[repr(C)]
pub struct BiomeStruct<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

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
Self::WindsweptHills { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::Forest { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::Taiga { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::Swamp { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::MangroveSwamp { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::River { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::NetherWastes { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::TheEnd { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::FrozenOcean { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::FrozenRiver { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::SnowyPlains { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::MushroomFields { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::Beach { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::Jungle { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::SparseJungle { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::DeepOcean { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::StonyShore { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::SnowyBeach { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::BirchForest { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::DarkForest { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::SnowyTaiga { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::OldGrowthPineTaiga { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::WindsweptForest { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::Savanna { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::SavannaPlateau { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::Badlands { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::WoodedBadlands { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::SmallEndIslands { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::EndMidlands { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::EndHighlands { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::EndBarrens { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::WarmOcean { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::LukewarmOcean { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::ColdOcean { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::DeepLukewarmOcean { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::DeepColdOcean { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::DeepFrozenOcean { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::TheVoid { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::SunflowerPlains { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::WindsweptGravellyHills { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::FlowerForest { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::IceSpikes { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::OldGrowthBirchForest { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::OldGrowthSpruceTaiga { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::WindsweptSavanna { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::ErodedBadlands { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::BambooJungle { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::SoulSandValley { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::CrimsonForest { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::WarpedForest { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::BasaltDeltas { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::DripstoneCaves { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::LushCaves { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::DeepDark { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::Meadow { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::Grove { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::SnowySlopes { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::FrozenPeaks { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::JaggedPeaks { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::StonyPeaks { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::CherryGrove { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
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
                return Err(eyre::eyre!(
                    "Tried to instantiate Biome from null object.")
                .into());
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
                    "OCEAN" => Ok(Biome::Ocean { inner: BiomeStruct::from_raw(env,obj)?}),"PLAINS" => Ok(Biome::Plains { inner: BiomeStruct::from_raw(env,obj)?}),"DESERT" => Ok(Biome::Desert { inner: BiomeStruct::from_raw(env,obj)?}),"WINDSWEPT_HILLS" => Ok(Biome::WindsweptHills { inner: BiomeStruct::from_raw(env,obj)?}),"FOREST" => Ok(Biome::Forest { inner: BiomeStruct::from_raw(env,obj)?}),"TAIGA" => Ok(Biome::Taiga { inner: BiomeStruct::from_raw(env,obj)?}),"SWAMP" => Ok(Biome::Swamp { inner: BiomeStruct::from_raw(env,obj)?}),"MANGROVE_SWAMP" => Ok(Biome::MangroveSwamp { inner: BiomeStruct::from_raw(env,obj)?}),"RIVER" => Ok(Biome::River { inner: BiomeStruct::from_raw(env,obj)?}),"NETHER_WASTES" => Ok(Biome::NetherWastes { inner: BiomeStruct::from_raw(env,obj)?}),"THE_END" => Ok(Biome::TheEnd { inner: BiomeStruct::from_raw(env,obj)?}),"FROZEN_OCEAN" => Ok(Biome::FrozenOcean { inner: BiomeStruct::from_raw(env,obj)?}),"FROZEN_RIVER" => Ok(Biome::FrozenRiver { inner: BiomeStruct::from_raw(env,obj)?}),"SNOWY_PLAINS" => Ok(Biome::SnowyPlains { inner: BiomeStruct::from_raw(env,obj)?}),"MUSHROOM_FIELDS" => Ok(Biome::MushroomFields { inner: BiomeStruct::from_raw(env,obj)?}),"BEACH" => Ok(Biome::Beach { inner: BiomeStruct::from_raw(env,obj)?}),"JUNGLE" => Ok(Biome::Jungle { inner: BiomeStruct::from_raw(env,obj)?}),"SPARSE_JUNGLE" => Ok(Biome::SparseJungle { inner: BiomeStruct::from_raw(env,obj)?}),"DEEP_OCEAN" => Ok(Biome::DeepOcean { inner: BiomeStruct::from_raw(env,obj)?}),"STONY_SHORE" => Ok(Biome::StonyShore { inner: BiomeStruct::from_raw(env,obj)?}),"SNOWY_BEACH" => Ok(Biome::SnowyBeach { inner: BiomeStruct::from_raw(env,obj)?}),"BIRCH_FOREST" => Ok(Biome::BirchForest { inner: BiomeStruct::from_raw(env,obj)?}),"DARK_FOREST" => Ok(Biome::DarkForest { inner: BiomeStruct::from_raw(env,obj)?}),"SNOWY_TAIGA" => Ok(Biome::SnowyTaiga { inner: BiomeStruct::from_raw(env,obj)?}),"OLD_GROWTH_PINE_TAIGA" => Ok(Biome::OldGrowthPineTaiga { inner: BiomeStruct::from_raw(env,obj)?}),"WINDSWEPT_FOREST" => Ok(Biome::WindsweptForest { inner: BiomeStruct::from_raw(env,obj)?}),"SAVANNA" => Ok(Biome::Savanna { inner: BiomeStruct::from_raw(env,obj)?}),"SAVANNA_PLATEAU" => Ok(Biome::SavannaPlateau { inner: BiomeStruct::from_raw(env,obj)?}),"BADLANDS" => Ok(Biome::Badlands { inner: BiomeStruct::from_raw(env,obj)?}),"WOODED_BADLANDS" => Ok(Biome::WoodedBadlands { inner: BiomeStruct::from_raw(env,obj)?}),"SMALL_END_ISLANDS" => Ok(Biome::SmallEndIslands { inner: BiomeStruct::from_raw(env,obj)?}),"END_MIDLANDS" => Ok(Biome::EndMidlands { inner: BiomeStruct::from_raw(env,obj)?}),"END_HIGHLANDS" => Ok(Biome::EndHighlands { inner: BiomeStruct::from_raw(env,obj)?}),"END_BARRENS" => Ok(Biome::EndBarrens { inner: BiomeStruct::from_raw(env,obj)?}),"WARM_OCEAN" => Ok(Biome::WarmOcean { inner: BiomeStruct::from_raw(env,obj)?}),"LUKEWARM_OCEAN" => Ok(Biome::LukewarmOcean { inner: BiomeStruct::from_raw(env,obj)?}),"COLD_OCEAN" => Ok(Biome::ColdOcean { inner: BiomeStruct::from_raw(env,obj)?}),"DEEP_LUKEWARM_OCEAN" => Ok(Biome::DeepLukewarmOcean { inner: BiomeStruct::from_raw(env,obj)?}),"DEEP_COLD_OCEAN" => Ok(Biome::DeepColdOcean { inner: BiomeStruct::from_raw(env,obj)?}),"DEEP_FROZEN_OCEAN" => Ok(Biome::DeepFrozenOcean { inner: BiomeStruct::from_raw(env,obj)?}),"THE_VOID" => Ok(Biome::TheVoid { inner: BiomeStruct::from_raw(env,obj)?}),"SUNFLOWER_PLAINS" => Ok(Biome::SunflowerPlains { inner: BiomeStruct::from_raw(env,obj)?}),"WINDSWEPT_GRAVELLY_HILLS" => Ok(Biome::WindsweptGravellyHills { inner: BiomeStruct::from_raw(env,obj)?}),"FLOWER_FOREST" => Ok(Biome::FlowerForest { inner: BiomeStruct::from_raw(env,obj)?}),"ICE_SPIKES" => Ok(Biome::IceSpikes { inner: BiomeStruct::from_raw(env,obj)?}),"OLD_GROWTH_BIRCH_FOREST" => Ok(Biome::OldGrowthBirchForest { inner: BiomeStruct::from_raw(env,obj)?}),"OLD_GROWTH_SPRUCE_TAIGA" => Ok(Biome::OldGrowthSpruceTaiga { inner: BiomeStruct::from_raw(env,obj)?}),"WINDSWEPT_SAVANNA" => Ok(Biome::WindsweptSavanna { inner: BiomeStruct::from_raw(env,obj)?}),"ERODED_BADLANDS" => Ok(Biome::ErodedBadlands { inner: BiomeStruct::from_raw(env,obj)?}),"BAMBOO_JUNGLE" => Ok(Biome::BambooJungle { inner: BiomeStruct::from_raw(env,obj)?}),"SOUL_SAND_VALLEY" => Ok(Biome::SoulSandValley { inner: BiomeStruct::from_raw(env,obj)?}),"CRIMSON_FOREST" => Ok(Biome::CrimsonForest { inner: BiomeStruct::from_raw(env,obj)?}),"WARPED_FOREST" => Ok(Biome::WarpedForest { inner: BiomeStruct::from_raw(env,obj)?}),"BASALT_DELTAS" => Ok(Biome::BasaltDeltas { inner: BiomeStruct::from_raw(env,obj)?}),"DRIPSTONE_CAVES" => Ok(Biome::DripstoneCaves { inner: BiomeStruct::from_raw(env,obj)?}),"LUSH_CAVES" => Ok(Biome::LushCaves { inner: BiomeStruct::from_raw(env,obj)?}),"DEEP_DARK" => Ok(Biome::DeepDark { inner: BiomeStruct::from_raw(env,obj)?}),"MEADOW" => Ok(Biome::Meadow { inner: BiomeStruct::from_raw(env,obj)?}),"GROVE" => Ok(Biome::Grove { inner: BiomeStruct::from_raw(env,obj)?}),"SNOWY_SLOPES" => Ok(Biome::SnowySlopes { inner: BiomeStruct::from_raw(env,obj)?}),"FROZEN_PEAKS" => Ok(Biome::FrozenPeaks { inner: BiomeStruct::from_raw(env,obj)?}),"JAGGED_PEAKS" => Ok(Biome::JaggedPeaks { inner: BiomeStruct::from_raw(env,obj)?}),"STONY_PEAKS" => Ok(Biome::StonyPeaks { inner: BiomeStruct::from_raw(env,obj)?}),"CHERRY_GROVE" => Ok(Biome::CherryGrove { inner: BiomeStruct::from_raw(env,obj)?}),"CUSTOM" => Ok(Biome::Custom { inner: BiomeStruct::from_raw(env,obj)?}),_ => Err(eyre::eyre!("String gaven for variant was invalid").into())}
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
                return Err(eyre::eyre!(
                    "Tried to instantiate BiomeStruct from null object.")
                .into());
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

	fn values(jni: &blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::block::Biome<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/block/Biome;");
let cls = jni.find_class("org/bukkit/block/Biome"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.call_static_method(cls,"values",
sig.as_str(),vec![]);
let res = 
jni.translate_error(res)?;
let obj = res.l()?;
crate::block::Biome::from_raw(&jni,obj
)}

	fn key(&self) 
-> Result<crate::NamespacedKey<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/NamespacedKey;");
let res = self.jni_ref().call_method(&self.jni_object(),"getKey",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::NamespacedKey::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
#[repr(C)]
pub struct TrialSpawner<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for TrialSpawner<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for TrialSpawner<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate TrialSpawner from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/block/TrialSpawner")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a TrialSpawner object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> TrialSpawnerTrait<'mc> for TrialSpawner<'mc> {}
pub trait TrialSpawnerTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
impl<'mc> Into<crate::block::TileState<'mc>> for TrialSpawner<'mc>{

fn into(self) -> crate::block::TileState<'mc> {

crate::block::TileState::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting TrialSpawner into crate::block::TileState")

   }
}
impl<'mc> crate::block::TileStateTrait<'mc> for TrialSpawner<'mc> {}
#[repr(C)]
pub struct Bell<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

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
                return Err(eyre::eyre!(
                    "Tried to instantiate Bell from null object.")
                .into());
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
    
impl<'mc> BellTrait<'mc> for Bell<'mc> {}
pub trait BellTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {
/// Ring this bell. This will call a {@link BellRingEvent}.
	fn ring(&self,entity: std::option::Option<impl Into<crate::entity::Entity<'mc>>>,direction: std::option::Option<impl Into<crate::block::BlockFace<'mc>>>) 
-> Result<bool, Box<dyn std::error::Error>>

{let mut args = Vec::new();
let mut sig = String::from("(");
if let Some(a) = entity {
sig += "Lorg/bukkit/entity/Entity;";
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(a.into().jni_object().clone())});
args.push(val_1);
}
if let Some(a) = direction {
sig += "Lorg/bukkit/block/BlockFace;";
let val_2 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(a.into().jni_object().clone())});
args.push(val_2);
}
sig += ")Z";
let res = self.jni_ref().call_method(&self.jni_object(),"ring",sig.as_str(),args);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
/// Check whether or not this bell is shaking. A bell is considered to be
/// shaking if it was recently rung.
/// 
/// A bell will typically shake for 50 ticks.
	fn is_shaking(&self) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("()Z");
let res = self.jni_ref().call_method(&self.jni_object(),"isShaking",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
/// Get the amount of ticks since this bell has been shaking, or 0 if the
/// bell is not currently shaking.
/// 
/// A bell will typically shake for 50 ticks.
	fn shaking_ticks(&self) 
-> Result<i32, Box<dyn std::error::Error>>

{let sig = String::from("()I");
let res = self.jni_ref().call_method(&self.jni_object(),"getShakingTicks",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.i()?
)}
/// Check whether or not this bell is resonating. A bell is considered to be
/// resonating if {@link #isShaking() while shaking}, raiders were detected
/// in the area and are ready to be highlighted to nearby players.
/// 
/// A bell will typically resonate for 40 ticks.
	fn is_resonating(&self) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("()Z");
let res = self.jni_ref().call_method(&self.jni_object(),"isResonating",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
/// Get the amount of ticks since this bell has been resonating, or 0 if the
/// bell is not currently resonating.
/// 
/// A bell will typically resonate for 40 ticks.
	fn resonating_ticks(&self) 
-> Result<i32, Box<dyn std::error::Error>>

{let sig = String::from("()I");
let res = self.jni_ref().call_method(&self.jni_object(),"getResonatingTicks",sig.as_str(),vec![]);
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
impl<'mc> Into<crate::block::TileState<'mc>> for Bell<'mc>{

fn into(self) -> crate::block::TileState<'mc> {

crate::block::TileState::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting Bell into crate::block::TileState")

   }
}
impl<'mc> crate::block::TileStateTrait<'mc> for Bell<'mc> {}
#[repr(C)]
pub struct Lockable<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

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
                return Err(eyre::eyre!(
                    "Tried to instantiate Lockable from null object.")
                .into());
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
    
impl<'mc> LockableTrait<'mc> for Lockable<'mc> {}
pub trait LockableTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {
/// Checks if the container has a valid (non empty) key.
	fn is_locked(&self) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("()Z");
let res = self.jni_ref().call_method(&self.jni_object(),"isLocked",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
/// Gets the key needed to access the container.
	fn lock(&self) 
-> Result<String, Box<dyn std::error::Error>>

{let sig = String::from("()Ljava/lang/String;");
let res = self.jni_ref().call_method(&self.jni_object(),"getLock",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string()
)}
/// Sets the key required to access this container. Set to null (or empty
/// string) to remove key.
	fn set_lock(&self,key: impl Into<String>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Ljava/lang/String;)V");
let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(key.into())?));
let res = self.jni_ref().call_method(&self.jni_object(),"setLock",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
#[repr(C)]
pub struct ChiseledBookshelf<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

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
                return Err(eyre::eyre!(
                    "Tried to instantiate ChiseledBookshelf from null object.")
                .into());
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
    
impl<'mc> ChiseledBookshelfTrait<'mc> for ChiseledBookshelf<'mc> {}
pub trait ChiseledBookshelfTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {
/// Gets the last interacted inventory slot.
	fn last_interacted_slot(&self) 
-> Result<i32, Box<dyn std::error::Error>>

{let sig = String::from("()I");
let res = self.jni_ref().call_method(&self.jni_object(),"getLastInteractedSlot",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.i()?
)}
/// Sets the last interacted inventory slot.
	fn set_last_interacted_slot(&self,last_interacted_slot: i32) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(I)V");
let val_1 = jni::objects::JValueGen::Int(last_interacted_slot);
let res = self.jni_ref().call_method(&self.jni_object(),"setLastInteractedSlot",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}

	fn inventory(&self) 
-> Result<crate::inventory::ChiseledBookshelfInventory<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/inventory/ChiseledBookshelfInventory;");
let res = self.jni_ref().call_method(&self.jni_object(),"getInventory",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::inventory::ChiseledBookshelfInventory::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}

	fn snapshot_inventory(&self) 
-> Result<crate::inventory::ChiseledBookshelfInventory<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/inventory/ChiseledBookshelfInventory;");
let res = self.jni_ref().call_method(&self.jni_object(),"getSnapshotInventory",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::inventory::ChiseledBookshelfInventory::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
/// Gets the appropriate slot based on a vector relative to this block. 
/// 
/// Will return -1 if the given vector is not within the bounds of any slot.
/// 
/// The supplied vector should only contain components with values between 0.0
/// and 1.0, inclusive.
	fn get_slot(&self,position: impl Into<crate::util::Vector<'mc>>) 
-> Result<i32, Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/util/Vector;)I");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(position.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"getSlot",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
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
impl<'mc> Into<crate::block::TileState<'mc>> for ChiseledBookshelf<'mc>{

fn into(self) -> crate::block::TileState<'mc> {

crate::block::TileState::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting ChiseledBookshelf into crate::block::TileState")

   }
}
impl<'mc> crate::block::TileStateTrait<'mc> for ChiseledBookshelf<'mc> {}
impl<'mc> Into<crate::inventory::BlockInventoryHolder<'mc>> for ChiseledBookshelf<'mc>{

fn into(self) -> crate::inventory::BlockInventoryHolder<'mc> {

crate::inventory::BlockInventoryHolder::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting ChiseledBookshelf into crate::inventory::BlockInventoryHolder")

   }
}
impl<'mc> crate::inventory::BlockInventoryHolderTrait<'mc> for ChiseledBookshelf<'mc> {}
#[repr(C)]
pub struct EnchantingTable<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

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
                return Err(eyre::eyre!(
                    "Tried to instantiate EnchantingTable from null object.")
                .into());
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
    
impl<'mc> EnchantingTableTrait<'mc> for EnchantingTable<'mc> {}
pub trait EnchantingTableTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
impl<'mc> Into<crate::block::TileState<'mc>> for EnchantingTable<'mc>{

fn into(self) -> crate::block::TileState<'mc> {

crate::block::TileState::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting EnchantingTable into crate::block::TileState")

   }
}
impl<'mc> crate::block::TileStateTrait<'mc> for EnchantingTable<'mc> {}
impl<'mc> Into<crate::Nameable<'mc>> for EnchantingTable<'mc>{

fn into(self) -> crate::Nameable<'mc> {

crate::Nameable::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting EnchantingTable into crate::Nameable")

   }
}
impl<'mc> crate::NameableTrait<'mc> for EnchantingTable<'mc> {}
#[repr(C)]
pub struct Crafter<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for Crafter<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for Crafter<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate Crafter from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/block/Crafter")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a Crafter object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> CrafterTrait<'mc> for Crafter<'mc> {}
pub trait CrafterTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {
/// Gets the number of ticks which this block will remain in the crafting
/// state for.
	fn crafting_ticks(&self) 
-> Result<i32, Box<dyn std::error::Error>>

{let sig = String::from("()I");
let res = self.jni_ref().call_method(&self.jni_object(),"getCraftingTicks",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.i()?
)}
/// Sets the number of ticks which this block will remain in the crafting
/// state for.
	fn set_crafting_ticks(&self,ticks: i32) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(I)V");
let val_1 = jni::objects::JValueGen::Int(ticks);
let res = self.jni_ref().call_method(&self.jni_object(),"setCraftingTicks",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
/// Gets whether the slot at the specified index is disabled and will not
/// have items placed in it.
	fn is_slot_disabled(&self,slot: i32) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("(I)Z");
let val_1 = jni::objects::JValueGen::Int(slot);
let res = self.jni_ref().call_method(&self.jni_object(),"isSlotDisabled",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
/// Sets whether the slot at the specified index is disabled and will not
/// have items placed in it.
	fn set_slot_disabled(&self,slot: i32,disabled: bool) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(IZ)V");
let val_1 = jni::objects::JValueGen::Int(slot);
let val_2 = jni::objects::JValueGen::Bool(disabled.into());
let res = self.jni_ref().call_method(&self.jni_object(),"setSlotDisabled",sig.as_str(),vec![jni::objects::JValueGen::from(val_1),jni::objects::JValueGen::from(val_2)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
/// Gets whether this Crafter is powered.
	fn is_triggered(&self) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("()Z");
let res = self.jni_ref().call_method(&self.jni_object(),"isTriggered",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
/// Sets whether this Crafter is powered.
	fn set_triggered(&self,triggered: bool) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Z)V");
let val_1 = jni::objects::JValueGen::Bool(triggered.into());
let res = self.jni_ref().call_method(&self.jni_object(),"setTriggered",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
impl<'mc> Into<crate::block::Container<'mc>> for Crafter<'mc>{

fn into(self) -> crate::block::Container<'mc> {

crate::block::Container::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting Crafter into crate::block::Container")

   }
}
impl<'mc> crate::block::ContainerTrait<'mc> for Crafter<'mc> {}
impl<'mc> Into<crate::loot::Lootable<'mc>> for Crafter<'mc>{

fn into(self) -> crate::loot::Lootable<'mc> {

crate::loot::Lootable::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting Crafter into crate::loot::Lootable")

   }
}
impl<'mc> crate::loot::LootableTrait<'mc> for Crafter<'mc> {}
pub enum DecoratedPotSide<'mc> {
	Back {inner: DecoratedPotSideStruct<'mc>},
	Left {inner: DecoratedPotSideStruct<'mc>},
	Right {inner: DecoratedPotSideStruct<'mc>},
	Front {inner: DecoratedPotSideStruct<'mc>},
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

        impl<'mc> DecoratedPotSideTrait<'mc> for DecoratedPotSide<'mc> {}
        
        pub trait DecoratedPotSideTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc>  {
            fn value_of(
                env: &blackboxmc_general::SharedJNIEnv<'mc>,
                arg0: impl Into<String>,
            ) -> Result<DecoratedPotSide<'mc>, Box<dyn std::error::Error>> {
                let val_1 = jni::objects::JObject::from(env.new_string(arg0.into())?);
                let cls = env.find_class("org/bukkit/block/DecoratedPot/Side");
                let cls = env.translate_error_with_class(cls)?;
                let res = env.call_static_method(
                    cls,
                    "valueOf",
                    "(Ljava/lang/String;)Lorg/bukkit/block/DecoratedPot/Side;",
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
                    
"BACK" => Ok(DecoratedPotSide::Back { inner: DecoratedPotSideStruct::from_raw(env,obj)?}),
"LEFT" => Ok(DecoratedPotSide::Left { inner: DecoratedPotSideStruct::from_raw(env,obj)?}),
"RIGHT" => Ok(DecoratedPotSide::Right { inner: DecoratedPotSideStruct::from_raw(env,obj)?}),
"FRONT" => Ok(DecoratedPotSide::Front { inner: DecoratedPotSideStruct::from_raw(env,obj)?}),

                    _ => Err(eyre::eyre!("String gaven for variant was invalid").into())
                }
            }
        }
        
#[repr(C)]
pub struct DecoratedPotSideStruct<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

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
                return Err(eyre::eyre!(
                    "Tried to instantiate DecoratedPotSide from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/block/DecoratedPot/Side")?;
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
                    "BACK" => Ok(DecoratedPotSide::Back { inner: DecoratedPotSideStruct::from_raw(env,obj)?}),"LEFT" => Ok(DecoratedPotSide::Left { inner: DecoratedPotSideStruct::from_raw(env,obj)?}),"RIGHT" => Ok(DecoratedPotSide::Right { inner: DecoratedPotSideStruct::from_raw(env,obj)?}),"FRONT" => Ok(DecoratedPotSide::Front { inner: DecoratedPotSideStruct::from_raw(env,obj)?}),_ => Err(eyre::eyre!("String gaven for variant was invalid").into())}
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
                    "Tried to instantiate DecoratedPotSideStruct from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/block/DecoratedPot/Side")?;
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

	fn values(jni: &blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::block::DecoratedPotSide<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/block/DecoratedPot/Side;");
let cls = jni.find_class("org/bukkit/block/DecoratedPot/Side"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.call_static_method(cls,"values",
sig.as_str(),vec![]);
let res = 
jni.translate_error(res)?;
let obj = res.l()?;
crate::block::DecoratedPotSide::from_raw(&jni,obj
)}

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
#[repr(C)]
pub struct SculkShrieker<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

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
                return Err(eyre::eyre!(
                    "Tried to instantiate SculkShrieker from null object.")
                .into());
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
    
impl<'mc> SculkShriekerTrait<'mc> for SculkShrieker<'mc> {}
pub trait SculkShriekerTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {
/// Gets the most recent warning level of this block.
/// When the warning level reaches 4, the shrieker will attempt to spawn a
/// Warden.
	fn warning_level(&self) 
-> Result<i32, Box<dyn std::error::Error>>

{let sig = String::from("()I");
let res = self.jni_ref().call_method(&self.jni_object(),"getWarningLevel",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.i()?
)}
/// Sets the most recent warning level of this block.
/// When the warning level reaches 4, the shrieker will attempt to spawn a
/// Warden.
	fn set_warning_level(&self,level: i32) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(I)V");
let val_1 = jni::objects::JValueGen::Int(level);
let res = self.jni_ref().call_method(&self.jni_object(),"setWarningLevel",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
/// Simulates a player causing a vibration.
	fn try_shriek(&self,player: impl Into<crate::entity::Player<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/entity/Player;)V");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(player.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"tryShriek",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
impl<'mc> Into<crate::block::TileState<'mc>> for SculkShrieker<'mc>{

fn into(self) -> crate::block::TileState<'mc> {

crate::block::TileState::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting SculkShrieker into crate::block::TileState")

   }
}
impl<'mc> crate::block::TileStateTrait<'mc> for SculkShrieker<'mc> {}
#[repr(C)]
pub struct DaylightDetector<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

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
                return Err(eyre::eyre!(
                    "Tried to instantiate DaylightDetector from null object.")
                .into());
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
    
impl<'mc> DaylightDetectorTrait<'mc> for DaylightDetector<'mc> {}
pub trait DaylightDetectorTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
impl<'mc> Into<crate::block::TileState<'mc>> for DaylightDetector<'mc>{

fn into(self) -> crate::block::TileState<'mc> {

crate::block::TileState::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting DaylightDetector into crate::block::TileState")

   }
}
impl<'mc> crate::block::TileStateTrait<'mc> for DaylightDetector<'mc> {}
#[repr(C)]
pub struct Comparator<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

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
                return Err(eyre::eyre!(
                    "Tried to instantiate Comparator from null object.")
                .into());
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
    
impl<'mc> ComparatorTrait<'mc> for Comparator<'mc> {}
pub trait ComparatorTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
impl<'mc> Into<crate::block::TileState<'mc>> for Comparator<'mc>{

fn into(self) -> crate::block::TileState<'mc> {

crate::block::TileState::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting Comparator into crate::block::TileState")

   }
}
impl<'mc> crate::block::TileStateTrait<'mc> for Comparator<'mc> {}
#[repr(C)]
pub struct Campfire<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

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
                return Err(eyre::eyre!(
                    "Tried to instantiate Campfire from null object.")
                .into());
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
    
impl<'mc> CampfireTrait<'mc> for Campfire<'mc> {}
pub trait CampfireTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {

	fn size(&self) 
-> Result<i32, Box<dyn std::error::Error>>

{let sig = String::from("()I");
let res = self.jni_ref().call_method(&self.jni_object(),"getSize",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.i()?
)}

	fn get_item(&self,index: i32) 
-> Result<Option<crate::inventory::ItemStack<'mc>>, Box<dyn std::error::Error>>

{let sig = String::from("(I)Lorg/bukkit/inventory/ItemStack;");
let val_1 = jni::objects::JValueGen::Int(index);
let res = self.jni_ref().call_method(&self.jni_object(),"getItem",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {return Ok(None);}
Ok(
Some(
crate::inventory::ItemStack::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)?
)
)}

	fn set_item(&self,index: i32,item: impl Into<crate::inventory::ItemStack<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(ILorg/bukkit/inventory/ItemStack;)V");
let val_1 = jni::objects::JValueGen::Int(index);
let val_2 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(item.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"setItem",sig.as_str(),vec![jni::objects::JValueGen::from(val_1),jni::objects::JValueGen::from(val_2)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
/// Get cook time.
/// This is the amount of time the item has been cooking for.
	fn get_cook_time(&self,index: i32) 
-> Result<i32, Box<dyn std::error::Error>>

{let sig = String::from("(I)I");
let val_1 = jni::objects::JValueGen::Int(index);
let res = self.jni_ref().call_method(&self.jni_object(),"getCookTime",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.i()?
)}
/// Set cook time.
/// This is the amount of time the item has been cooking for.
	fn set_cook_time(&self,index: i32,cook_time: i32) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(II)V");
let val_1 = jni::objects::JValueGen::Int(index);
let val_2 = jni::objects::JValueGen::Int(cook_time);
let res = self.jni_ref().call_method(&self.jni_object(),"setCookTime",sig.as_str(),vec![jni::objects::JValueGen::from(val_1),jni::objects::JValueGen::from(val_2)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
/// Get cook time total.
/// This is the amount of time the item is required to cook for.
	fn get_cook_time_total(&self,index: i32) 
-> Result<i32, Box<dyn std::error::Error>>

{let sig = String::from("(I)I");
let val_1 = jni::objects::JValueGen::Int(index);
let res = self.jni_ref().call_method(&self.jni_object(),"getCookTimeTotal",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.i()?
)}
/// Set cook time.
/// This is the amount of time the item is required to cook for.
	fn set_cook_time_total(&self,index: i32,cook_time_total: i32) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(II)V");
let val_1 = jni::objects::JValueGen::Int(index);
let val_2 = jni::objects::JValueGen::Int(cook_time_total);
let res = self.jni_ref().call_method(&self.jni_object(),"setCookTimeTotal",sig.as_str(),vec![jni::objects::JValueGen::from(val_1),jni::objects::JValueGen::from(val_2)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
impl<'mc> Into<crate::block::TileState<'mc>> for Campfire<'mc>{

fn into(self) -> crate::block::TileState<'mc> {

crate::block::TileState::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting Campfire into crate::block::TileState")

   }
}
impl<'mc> crate::block::TileStateTrait<'mc> for Campfire<'mc> {}
#[repr(C)]
pub struct BrushableBlock<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

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
                return Err(eyre::eyre!(
                    "Tried to instantiate BrushableBlock from null object.")
                .into());
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
    
impl<'mc> BrushableBlockTrait<'mc> for BrushableBlock<'mc> {}
pub trait BrushableBlockTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {
/// Get the item which will be revealed when the sand is fully brushed away
/// and uncovered.
	fn item(&self) 
-> Result<Option<crate::inventory::ItemStack<'mc>>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/inventory/ItemStack;");
let res = self.jni_ref().call_method(&self.jni_object(),"getItem",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {return Ok(None);}
Ok(
Some(
crate::inventory::ItemStack::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)?
)
)}
/// Sets the item which will be revealed when the sand is fully brushed away
/// and uncovered.
	fn set_item(&self,item: impl Into<crate::inventory::ItemStack<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/inventory/ItemStack;)V");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(item.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"setItem",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
impl<'mc> Into<crate::loot::Lootable<'mc>> for BrushableBlock<'mc>{

fn into(self) -> crate::loot::Lootable<'mc> {

crate::loot::Lootable::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting BrushableBlock into crate::loot::Lootable")

   }
}
impl<'mc> crate::loot::LootableTrait<'mc> for BrushableBlock<'mc> {}
impl<'mc> Into<crate::block::TileState<'mc>> for BrushableBlock<'mc>{

fn into(self) -> crate::block::TileState<'mc> {

crate::block::TileState::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting BrushableBlock into crate::block::TileState")

   }
}
impl<'mc> crate::block::TileStateTrait<'mc> for BrushableBlock<'mc> {}
#[repr(C)]
pub struct Sign<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

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
                return Err(eyre::eyre!(
                    "Tried to instantiate Sign from null object.")
                .into());
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
    
impl<'mc> SignTrait<'mc> for Sign<'mc> {}
pub trait SignTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {
#[deprecated]
/// Gets all the lines of text currently on the {@link Side#FRONT} of this sign.
	fn lines(&self) 
-> Result<String, Box<dyn std::error::Error>>

{let sig = String::from("()Ljava/lang/String;");
let res = self.jni_ref().call_method(&self.jni_object(),"getLines",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string()
)}
#[deprecated]
/// Gets the line of text at the specified index.For example, getLine(0) will return the first line of text on the {@link Side#FRONT}.
	fn get_line(&self,index: i32) 
-> Result<String, Box<dyn std::error::Error>>

{let sig = String::from("(I)Ljava/lang/String;");
let val_1 = jni::objects::JValueGen::Int(index);
let res = self.jni_ref().call_method(&self.jni_object(),"getLine",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string()
)}
#[deprecated]
/// Sets the line of text at the specified index.For example, setLine(0, "Line One") will set the first line of text to "Line One".
	fn set_line(&self,index: i32,line: impl Into<String>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(ILjava/lang/String;)V");
let val_1 = jni::objects::JValueGen::Int(index);
let val_2 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(line.into())?));
let res = self.jni_ref().call_method(&self.jni_object(),"setLine",sig.as_str(),vec![jni::objects::JValueGen::from(val_1),jni::objects::JValueGen::from(val_2)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
#[deprecated]
/// Marks whether this sign can be edited by players.
	fn is_editable(&self) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("()Z");
let res = self.jni_ref().call_method(&self.jni_object(),"isEditable",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
#[deprecated]
/// Marks whether this sign can be edited by players.
	fn set_editable(&self,editable: bool) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Z)V");
let val_1 = jni::objects::JValueGen::Bool(editable.into());
let res = self.jni_ref().call_method(&self.jni_object(),"setEditable",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
/// Gets whether or not this sign has been waxed. If a sign has been waxed, it
/// cannot be edited by a player.
	fn is_waxed(&self) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("()Z");
let res = self.jni_ref().call_method(&self.jni_object(),"isWaxed",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
/// Sets whether or not this sign has been waxed. If a sign has been waxed, it
/// cannot be edited by a player.
	fn set_waxed(&self,waxed: bool) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Z)V");
let val_1 = jni::objects::JValueGen::Bool(waxed.into());
let res = self.jni_ref().call_method(&self.jni_object(),"setWaxed",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
#[deprecated]
/// Gets whether this sign has glowing text. Only affects the {@link Side#FRONT}.
	fn is_glowing_text(&self) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("()Z");
let res = self.jni_ref().call_method(&self.jni_object(),"isGlowingText",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
#[deprecated]
/// Sets whether this sign has glowing text. Only affects the {@link Side#FRONT}.
	fn set_glowing_text(&self,glowing: bool) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Z)V");
let val_1 = jni::objects::JValueGen::Bool(glowing.into());
let res = self.jni_ref().call_method(&self.jni_object(),"setGlowingText",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
#[deprecated]
/// {@inheritDoc}
	fn color(&self) 
-> Result<crate::DyeColor<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/DyeColor;");
let res = self.jni_ref().call_method(&self.jni_object(),"getColor",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::DyeColor::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
#[deprecated]
/// {@inheritDoc}
	fn set_color(&self,color: impl Into<crate::DyeColor<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/DyeColor;)V");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(color.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"setColor",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
/// Return the side of the sign.
	fn get_side(&self,side: impl Into<crate::block::sign::Side<'mc>>) 
-> Result<crate::block::sign::SignSide<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/block/sign/Side;)Lorg/bukkit/block/sign/SignSide;");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(side.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"getSide",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
crate::block::sign::SignSide::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
/// Gets the side of this sign the given player is currently standing on.
	fn get_target_side(&self,player: impl Into<crate::entity::Player<'mc>>) 
-> Result<crate::block::sign::SignSide<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/entity/Player;)Lorg/bukkit/block/sign/SignSide;");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(player.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"getTargetSide",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
crate::block::sign::SignSide::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
/// Gets the player that is currently allowed to edit this sign.
/// 
/// Edits from other players will be rejected if this value is not null.
	fn allowed_editor(&self) 
-> Result<Option<crate::entity::Player<'mc>>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/entity/Player;");
let res = self.jni_ref().call_method(&self.jni_object(),"getAllowedEditor",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {return Ok(None);}
Ok(
Some(
crate::entity::Player::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)?
)
)}

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
impl<'mc> Into<crate::block::TileState<'mc>> for Sign<'mc>{

fn into(self) -> crate::block::TileState<'mc> {

crate::block::TileState::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting Sign into crate::block::TileState")

   }
}
impl<'mc> crate::block::TileStateTrait<'mc> for Sign<'mc> {}
impl<'mc> Into<crate::material::Colorable<'mc>> for Sign<'mc>{

fn into(self) -> crate::material::Colorable<'mc> {

crate::material::Colorable::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting Sign into crate::material::Colorable")

   }
}
impl<'mc> crate::material::ColorableTrait<'mc> for Sign<'mc> {}
#[repr(C)]
pub struct EntityBlockStorage<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

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
                return Err(eyre::eyre!(
                    "Tried to instantiate EntityBlockStorage from null object.")
                .into());
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
    
impl<'mc> EntityBlockStorageTrait<'mc> for EntityBlockStorage<'mc> {}
pub trait EntityBlockStorageTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {
/// Check if the block is completely full of entities.
	fn is_full(&self) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("()Z");
let res = self.jni_ref().call_method(&self.jni_object(),"isFull",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
/// Get the amount of entities currently in this block.
	fn entity_count(&self) 
-> Result<i32, Box<dyn std::error::Error>>

{let sig = String::from("()I");
let res = self.jni_ref().call_method(&self.jni_object(),"getEntityCount",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.i()?
)}
/// Get the maximum amount of entities this block can hold.
	fn max_entities(&self) 
-> Result<i32, Box<dyn std::error::Error>>

{let sig = String::from("()I");
let res = self.jni_ref().call_method(&self.jni_object(),"getMaxEntities",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.i()?
)}
/// Set the maximum amount of entities this block can hold.
	fn set_max_entities(&self,max: i32) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(I)V");
let val_1 = jni::objects::JValueGen::Int(max);
let res = self.jni_ref().call_method(&self.jni_object(),"setMaxEntities",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
/// Release all the entities currently stored in the block.
	fn release_entities(&self) 
-> Result<Vec<jni::objects::JObject<'mc>>, Box<dyn std::error::Error>>

{let sig = String::from("()Ljava/util/List;");
let res = self.jni_ref().call_method(&self.jni_object(),"releaseEntities",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
let mut new_vec = Vec::new();
let list = blackboxmc_java::util::JavaList::from_raw(&self.jni_ref(), res.l()?)?;let iter = list.iterator()?;
while iter.has_next()? {            let obj = iter.next()?;
new_vec.push(obj);
};
Ok(
new_vec
)}
/// Add an entity to the block.
	fn add_entity(&self,entity: jni::objects::JObject<'mc>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(LT;)V");
let val_1 = jni::objects::JValueGen::Object(entity);
let res = self.jni_ref().call_method(&self.jni_object(),"addEntity",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
impl<'mc> Into<crate::block::TileState<'mc>> for EntityBlockStorage<'mc>{

fn into(self) -> crate::block::TileState<'mc> {

crate::block::TileState::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting EntityBlockStorage into crate::block::TileState")

   }
}
impl<'mc> crate::block::TileStateTrait<'mc> for EntityBlockStorage<'mc> {}
#[repr(C)]
pub struct Container<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

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
                return Err(eyre::eyre!(
                    "Tried to instantiate Container from null object.")
                .into());
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
    
impl<'mc> ContainerTrait<'mc> for Container<'mc> {}
pub trait ContainerTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {
/// Gets the inventory of the block represented by this block state.
/// 
/// If the block was changed to a different type in the meantime, the
/// returned inventory might no longer be valid.
/// 
/// If this block state is not placed this will return the captured inventory
/// snapshot instead.
	fn inventory(&self) 
-> Result<crate::inventory::Inventory<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/inventory/Inventory;");
let res = self.jni_ref().call_method(&self.jni_object(),"getInventory",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::inventory::Inventory::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
/// Gets the captured inventory snapshot of this container.
/// 
/// The returned inventory is not linked to any block. Any modifications to
/// the returned inventory will not be applied to the block represented by
/// this block state up until {@link #update(boolean, boolean)} has been
/// called.
	fn snapshot_inventory(&self) 
-> Result<crate::inventory::Inventory<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/inventory/Inventory;");
let res = self.jni_ref().call_method(&self.jni_object(),"getSnapshotInventory",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::inventory::Inventory::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
impl<'mc> Into<crate::block::TileState<'mc>> for Container<'mc>{

fn into(self) -> crate::block::TileState<'mc> {

crate::block::TileState::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting Container into crate::block::TileState")

   }
}
impl<'mc> crate::block::TileStateTrait<'mc> for Container<'mc> {}
impl<'mc> Into<crate::inventory::BlockInventoryHolder<'mc>> for Container<'mc>{

fn into(self) -> crate::inventory::BlockInventoryHolder<'mc> {

crate::inventory::BlockInventoryHolder::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting Container into crate::inventory::BlockInventoryHolder")

   }
}
impl<'mc> crate::inventory::BlockInventoryHolderTrait<'mc> for Container<'mc> {}
impl<'mc> Into<crate::block::Lockable<'mc>> for Container<'mc>{

fn into(self) -> crate::block::Lockable<'mc> {

crate::block::Lockable::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting Container into crate::block::Lockable")

   }
}
impl<'mc> crate::block::LockableTrait<'mc> for Container<'mc> {}
impl<'mc> Into<crate::Nameable<'mc>> for Container<'mc>{

fn into(self) -> crate::Nameable<'mc> {

crate::Nameable::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting Container into crate::Nameable")

   }
}
impl<'mc> crate::NameableTrait<'mc> for Container<'mc> {}
#[repr(C)]
pub struct Smoker<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

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
                return Err(eyre::eyre!(
                    "Tried to instantiate Smoker from null object.")
                .into());
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
    
impl<'mc> SmokerTrait<'mc> for Smoker<'mc> {}
pub trait SmokerTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
impl<'mc> Into<crate::block::Furnace<'mc>> for Smoker<'mc>{

fn into(self) -> crate::block::Furnace<'mc> {

crate::block::Furnace::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting Smoker into crate::block::Furnace")

   }
}
impl<'mc> crate::block::FurnaceTrait<'mc> for Smoker<'mc> {}
#[repr(C)]
pub struct BlastFurnace<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

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
                return Err(eyre::eyre!(
                    "Tried to instantiate BlastFurnace from null object.")
                .into());
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
    
impl<'mc> BlastFurnaceTrait<'mc> for BlastFurnace<'mc> {}
pub trait BlastFurnaceTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
impl<'mc> Into<crate::block::Furnace<'mc>> for BlastFurnace<'mc>{

fn into(self) -> crate::block::Furnace<'mc> {

crate::block::Furnace::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting BlastFurnace into crate::block::Furnace")

   }
}
impl<'mc> crate::block::FurnaceTrait<'mc> for BlastFurnace<'mc> {}
pub enum PistonMoveReaction<'mc> {
	VariantMove {inner: PistonMoveReactionStruct<'mc>},
	VariantBreak {inner: PistonMoveReactionStruct<'mc>},
	Block {inner: PistonMoveReactionStruct<'mc>},
	Ignore {inner: PistonMoveReactionStruct<'mc>},
	PushOnly {inner: PistonMoveReactionStruct<'mc>},
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

        impl<'mc> PistonMoveReactionTrait<'mc> for PistonMoveReaction<'mc> {}
        
        pub trait PistonMoveReactionTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc>  {
            fn value_of(
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
                    
"MOVE" => Ok(PistonMoveReaction::VariantMove { inner: PistonMoveReactionStruct::from_raw(env,obj)?}),
"BREAK" => Ok(PistonMoveReaction::VariantBreak { inner: PistonMoveReactionStruct::from_raw(env,obj)?}),
"BLOCK" => Ok(PistonMoveReaction::Block { inner: PistonMoveReactionStruct::from_raw(env,obj)?}),
"IGNORE" => Ok(PistonMoveReaction::Ignore { inner: PistonMoveReactionStruct::from_raw(env,obj)?}),
"PUSH_ONLY" => Ok(PistonMoveReaction::PushOnly { inner: PistonMoveReactionStruct::from_raw(env,obj)?}),

                    _ => Err(eyre::eyre!("String gaven for variant was invalid").into())
                }
            }
        }
        
#[repr(C)]
pub struct PistonMoveReactionStruct<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

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
Self::VariantMove { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::VariantBreak { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
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
                return Err(eyre::eyre!(
                    "Tried to instantiate PistonMoveReaction from null object.")
                .into());
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
                    "MOVE" => Ok(PistonMoveReaction::VariantMove { inner: PistonMoveReactionStruct::from_raw(env,obj)?}),"BREAK" => Ok(PistonMoveReaction::VariantBreak { inner: PistonMoveReactionStruct::from_raw(env,obj)?}),"BLOCK" => Ok(PistonMoveReaction::Block { inner: PistonMoveReactionStruct::from_raw(env,obj)?}),"IGNORE" => Ok(PistonMoveReaction::Ignore { inner: PistonMoveReactionStruct::from_raw(env,obj)?}),"PUSH_ONLY" => Ok(PistonMoveReaction::PushOnly { inner: PistonMoveReactionStruct::from_raw(env,obj)?}),_ => Err(eyre::eyre!("String gaven for variant was invalid").into())}
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
                    "Tried to instantiate PistonMoveReactionStruct from null object.")
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

	fn values(jni: &blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::block::PistonMoveReaction<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/block/PistonMoveReaction;");
let cls = jni.find_class("org/bukkit/block/PistonMoveReaction"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.call_static_method(cls,"values",
sig.as_str(),vec![]);
let res = 
jni.translate_error(res)?;
let obj = res.l()?;
crate::block::PistonMoveReaction::from_raw(&jni,obj
)}
#[deprecated]

	fn id(&self) 
-> Result<i32, Box<dyn std::error::Error>>

{let sig = String::from("()I");
let res = self.jni_ref().call_method(&self.jni_object(),"getId",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.i()?
)}
#[deprecated]

	fn get_by_id(jni: &blackboxmc_general::SharedJNIEnv<'mc>,id: i32) 
-> Result<Option<crate::block::PistonMoveReaction<'mc>>, Box<dyn std::error::Error>>

{let sig = String::from("(I)Lorg/bukkit/block/PistonMoveReaction;");
let val_1 = jni::objects::JValueGen::Int(id);
let cls = jni.find_class("org/bukkit/block/PistonMoveReaction"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.call_static_method(cls,"getById",
sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
jni.translate_error(res)?;
if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {return Ok(None);}
let obj = res.l()?;
Ok(
Some(
crate::block::PistonMoveReaction::from_raw(&jni,obj
)?
)
)}

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
#[repr(C)]
pub struct Hopper<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

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
                return Err(eyre::eyre!(
                    "Tried to instantiate Hopper from null object.")
                .into());
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
    
impl<'mc> HopperTrait<'mc> for Hopper<'mc> {}
pub trait HopperTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
impl<'mc> Into<crate::block::Container<'mc>> for Hopper<'mc>{

fn into(self) -> crate::block::Container<'mc> {

crate::block::Container::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting Hopper into crate::block::Container")

   }
}
impl<'mc> crate::block::ContainerTrait<'mc> for Hopper<'mc> {}
impl<'mc> Into<crate::loot::Lootable<'mc>> for Hopper<'mc>{

fn into(self) -> crate::loot::Lootable<'mc> {

crate::loot::Lootable::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting Hopper into crate::loot::Lootable")

   }
}
impl<'mc> crate::loot::LootableTrait<'mc> for Hopper<'mc> {}
#[repr(C)]
pub struct CalibratedSculkSensor<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

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
                    "Tried to instantiate CalibratedSculkSensor from null object.")
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
    
impl<'mc> CalibratedSculkSensorTrait<'mc> for CalibratedSculkSensor<'mc> {}
pub trait CalibratedSculkSensorTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
impl<'mc> Into<crate::block::SculkSensor<'mc>> for CalibratedSculkSensor<'mc>{

fn into(self) -> crate::block::SculkSensor<'mc> {

crate::block::SculkSensor::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting CalibratedSculkSensor into crate::block::SculkSensor")

   }
}
impl<'mc> crate::block::SculkSensorTrait<'mc> for CalibratedSculkSensor<'mc> {}
#[repr(C)]
pub struct EndGateway<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

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
                return Err(eyre::eyre!(
                    "Tried to instantiate EndGateway from null object.")
                .into());
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
    
impl<'mc> EndGatewayTrait<'mc> for EndGateway<'mc> {}
pub trait EndGatewayTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {
/// Gets the location that entities are teleported to when
/// entering the gateway portal.
/// 
/// If this block state is not placed the location's world will be null.
	fn exit_location(&self) 
-> Result<Option<crate::Location<'mc>>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/Location;");
let res = self.jni_ref().call_method(&self.jni_object(),"getExitLocation",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {return Ok(None);}
Ok(
Some(
crate::Location::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)?
)
)}
/// Sets the exit location that entities are teleported to when
/// they enter the gateway portal.
/// 
/// If this block state is not placed the location's world has to be null.
	fn set_exit_location(&self,location: impl Into<crate::Location<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/Location;)V");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(location.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"setExitLocation",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
/// Gets whether this gateway will teleport entities directly to
/// the exit location instead of finding a nearby location.
	fn is_exact_teleport(&self) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("()Z");
let res = self.jni_ref().call_method(&self.jni_object(),"isExactTeleport",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
/// Sets whether this gateway will teleport entities directly to
/// the exit location instead of finding a nearby location.
	fn set_exact_teleport(&self,exact: bool) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Z)V");
let val_1 = jni::objects::JValueGen::Bool(exact.into());
let res = self.jni_ref().call_method(&self.jni_object(),"setExactTeleport",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
/// Gets the age in ticks of the gateway.
/// 
/// If the age is less than 200 ticks a magenta beam will be emitted, whilst
/// if it is a multiple of 2400 ticks a purple beam will be emitted.
	fn age(&self) 
-> Result<i64, Box<dyn std::error::Error>>

{let sig = String::from("()J");
let res = self.jni_ref().call_method(&self.jni_object(),"getAge",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.j()?
)}
/// Sets the age in ticks of the gateway.
/// 
/// If the age is less than 200 ticks a magenta beam will be emitted, whilst
/// if it is a multiple of 2400 ticks a purple beam will be emitted.
	fn set_age(&self,age: i64) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(J)V");
let val_1 = jni::objects::JValueGen::Long(age);
let res = self.jni_ref().call_method(&self.jni_object(),"setAge",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
impl<'mc> Into<crate::block::TileState<'mc>> for EndGateway<'mc>{

fn into(self) -> crate::block::TileState<'mc> {

crate::block::TileState::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting EndGateway into crate::block::TileState")

   }
}
impl<'mc> crate::block::TileStateTrait<'mc> for EndGateway<'mc> {}
#[repr(C)]
pub struct Lidded<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

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
                return Err(eyre::eyre!(
                    "Tried to instantiate Lidded from null object.")
                .into());
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
    
impl<'mc> LiddedTrait<'mc> for Lidded<'mc> {}
pub trait LiddedTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {
/// Sets the block's animated state to open and prevents it from being closed
/// until {@link #close()} is called.
	fn open(&self) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("()V");
let res = self.jni_ref().call_method(&self.jni_object(),"open",sig.as_str(),vec![]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
/// Sets the block's animated state to closed even if a player is currently
/// viewing this block.
	fn close(&self) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("()V");
let res = self.jni_ref().call_method(&self.jni_object(),"close",sig.as_str(),vec![]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
#[repr(C)]
pub struct SculkCatalyst<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

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
                return Err(eyre::eyre!(
                    "Tried to instantiate SculkCatalyst from null object.")
                .into());
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
    
impl<'mc> SculkCatalystTrait<'mc> for SculkCatalyst<'mc> {}
pub trait SculkCatalystTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {
/// Causes a new sculk bloom, as if an entity just died around this catalyst.
/// 
/// Typically, charges should be set to the exp reward of a mob
/// ({@link EntityDeathEvent#getDroppedExp()}), which is usually
/// 3-5 for animals, and 5-10 for the average mob (up to 50 for
/// wither skeletons). Roughly speaking, for each charge, 1 more
/// sculk block will be placed.
/// 
/// If <code>charges > 1000</code>, multiple cursors will be spawned in the
/// block.
	fn bloom(&self,block: impl Into<crate::block::Block<'mc>>,charges: i32) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/block/Block;I)V");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(block.into().jni_object().clone())});
let val_2 = jni::objects::JValueGen::Int(charges);
let res = self.jni_ref().call_method(&self.jni_object(),"bloom",sig.as_str(),vec![jni::objects::JValueGen::from(val_1),jni::objects::JValueGen::from(val_2)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
impl<'mc> Into<crate::block::TileState<'mc>> for SculkCatalyst<'mc>{

fn into(self) -> crate::block::TileState<'mc> {

crate::block::TileState::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting SculkCatalyst into crate::block::TileState")

   }
}
impl<'mc> crate::block::TileStateTrait<'mc> for SculkCatalyst<'mc> {}
#[repr(C)]
pub struct Jukebox<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

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
                return Err(eyre::eyre!(
                    "Tried to instantiate Jukebox from null object.")
                .into());
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
    
impl<'mc> JukeboxTrait<'mc> for Jukebox<'mc> {}
pub trait JukeboxTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {
/// Gets the record inserted into the jukebox.
	fn playing(&self) 
-> Result<crate::Material<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/Material;");
let res = self.jni_ref().call_method(&self.jni_object(),"getPlaying",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::Material::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
/// Sets the record being played.
	fn set_playing(&self,record: impl Into<crate::Material<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/Material;)V");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(record.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"setPlaying",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
/// Gets whether or not this jukebox has a record.
/// 
/// A jukebox can have a record but not {@link #isPlaying() be playing}
/// if it was stopped with {@link #stopPlaying()} or if a record has
/// finished playing.
	fn has_record(&self) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("()Z");
let res = self.jni_ref().call_method(&self.jni_object(),"hasRecord",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
/// Gets the record item inserted into the jukebox.
	fn record(&self) 
-> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/inventory/ItemStack;");
let res = self.jni_ref().call_method(&self.jni_object(),"getRecord",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::inventory::ItemStack::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
/// Sets the record being played. The jukebox will start playing automatically.
	fn set_record(&self,record: impl Into<crate::inventory::ItemStack<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/inventory/ItemStack;)V");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(record.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"setRecord",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
/// Checks if the jukebox is playing a record.
	fn is_playing(&self) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("()Z");
let res = self.jni_ref().call_method(&self.jni_object(),"isPlaying",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
/// Starts the jukebox playing if there is a record.
	fn start_playing(&self) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("()Z");
let res = self.jni_ref().call_method(&self.jni_object(),"startPlaying",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
/// Stops the jukebox playing without ejecting the record.
	fn stop_playing(&self) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("()V");
let res = self.jni_ref().call_method(&self.jni_object(),"stopPlaying",sig.as_str(),vec![]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
/// Stops the jukebox playing and ejects the current record.
/// 
/// If the block represented by this state is no longer a jukebox, this will
/// do nothing and return false.
	fn eject(&self) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("()Z");
let res = self.jni_ref().call_method(&self.jni_object(),"eject",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}

	fn inventory(&self) 
-> Result<crate::inventory::JukeboxInventory<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/inventory/JukeboxInventory;");
let res = self.jni_ref().call_method(&self.jni_object(),"getInventory",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::inventory::JukeboxInventory::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}

	fn snapshot_inventory(&self) 
-> Result<crate::inventory::JukeboxInventory<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/inventory/JukeboxInventory;");
let res = self.jni_ref().call_method(&self.jni_object(),"getSnapshotInventory",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::inventory::JukeboxInventory::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
impl<'mc> Into<crate::block::TileState<'mc>> for Jukebox<'mc>{

fn into(self) -> crate::block::TileState<'mc> {

crate::block::TileState::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting Jukebox into crate::block::TileState")

   }
}
impl<'mc> crate::block::TileStateTrait<'mc> for Jukebox<'mc> {}
impl<'mc> Into<crate::inventory::BlockInventoryHolder<'mc>> for Jukebox<'mc>{

fn into(self) -> crate::inventory::BlockInventoryHolder<'mc> {

crate::inventory::BlockInventoryHolder::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting Jukebox into crate::inventory::BlockInventoryHolder")

   }
}
impl<'mc> crate::inventory::BlockInventoryHolderTrait<'mc> for Jukebox<'mc> {}
pub enum BlockFace<'mc> {
	North {inner: BlockFaceStruct<'mc>},
	East {inner: BlockFaceStruct<'mc>},
	South {inner: BlockFaceStruct<'mc>},
	West {inner: BlockFaceStruct<'mc>},
	Up {inner: BlockFaceStruct<'mc>},
	Down {inner: BlockFaceStruct<'mc>},
	NorthEast {inner: BlockFaceStruct<'mc>},
	NorthWest {inner: BlockFaceStruct<'mc>},
	SouthEast {inner: BlockFaceStruct<'mc>},
	SouthWest {inner: BlockFaceStruct<'mc>},
	WestNorthWest {inner: BlockFaceStruct<'mc>},
	NorthNorthWest {inner: BlockFaceStruct<'mc>},
	NorthNorthEast {inner: BlockFaceStruct<'mc>},
	EastNorthEast {inner: BlockFaceStruct<'mc>},
	EastSouthEast {inner: BlockFaceStruct<'mc>},
	SouthSouthEast {inner: BlockFaceStruct<'mc>},
	SouthSouthWest {inner: BlockFaceStruct<'mc>},
	WestSouthWest {inner: BlockFaceStruct<'mc>},
	VariantSelf {inner: BlockFaceStruct<'mc>},
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

        impl<'mc> BlockFaceTrait<'mc> for BlockFace<'mc> {}
        
        pub trait BlockFaceTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc>  {
            fn value_of(
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
                    
"NORTH" => Ok(BlockFace::North { inner: BlockFaceStruct::from_raw(env,obj)?}),
"EAST" => Ok(BlockFace::East { inner: BlockFaceStruct::from_raw(env,obj)?}),
"SOUTH" => Ok(BlockFace::South { inner: BlockFaceStruct::from_raw(env,obj)?}),
"WEST" => Ok(BlockFace::West { inner: BlockFaceStruct::from_raw(env,obj)?}),
"UP" => Ok(BlockFace::Up { inner: BlockFaceStruct::from_raw(env,obj)?}),
"DOWN" => Ok(BlockFace::Down { inner: BlockFaceStruct::from_raw(env,obj)?}),
"NORTH_EAST" => Ok(BlockFace::NorthEast { inner: BlockFaceStruct::from_raw(env,obj)?}),
"NORTH_WEST" => Ok(BlockFace::NorthWest { inner: BlockFaceStruct::from_raw(env,obj)?}),
"SOUTH_EAST" => Ok(BlockFace::SouthEast { inner: BlockFaceStruct::from_raw(env,obj)?}),
"SOUTH_WEST" => Ok(BlockFace::SouthWest { inner: BlockFaceStruct::from_raw(env,obj)?}),
"WEST_NORTH_WEST" => Ok(BlockFace::WestNorthWest { inner: BlockFaceStruct::from_raw(env,obj)?}),
"NORTH_NORTH_WEST" => Ok(BlockFace::NorthNorthWest { inner: BlockFaceStruct::from_raw(env,obj)?}),
"NORTH_NORTH_EAST" => Ok(BlockFace::NorthNorthEast { inner: BlockFaceStruct::from_raw(env,obj)?}),
"EAST_NORTH_EAST" => Ok(BlockFace::EastNorthEast { inner: BlockFaceStruct::from_raw(env,obj)?}),
"EAST_SOUTH_EAST" => Ok(BlockFace::EastSouthEast { inner: BlockFaceStruct::from_raw(env,obj)?}),
"SOUTH_SOUTH_EAST" => Ok(BlockFace::SouthSouthEast { inner: BlockFaceStruct::from_raw(env,obj)?}),
"SOUTH_SOUTH_WEST" => Ok(BlockFace::SouthSouthWest { inner: BlockFaceStruct::from_raw(env,obj)?}),
"WEST_SOUTH_WEST" => Ok(BlockFace::WestSouthWest { inner: BlockFaceStruct::from_raw(env,obj)?}),
"SELF" => Ok(BlockFace::VariantSelf { inner: BlockFaceStruct::from_raw(env,obj)?}),

                    _ => Err(eyre::eyre!("String gaven for variant was invalid").into())
                }
            }
        }
        
#[repr(C)]
pub struct BlockFaceStruct<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

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
Self::NorthEast { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::NorthWest { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::SouthEast { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::SouthWest { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::WestNorthWest { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::NorthNorthWest { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::NorthNorthEast { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::EastNorthEast { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::EastSouthEast { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::SouthSouthEast { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::SouthSouthWest { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::WestSouthWest { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::VariantSelf { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
}
}
}
impl<'mc> JNIInstantiatable<'mc> for BlockFace<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate BlockFace from null object.")
                .into());
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
                    "NORTH" => Ok(BlockFace::North { inner: BlockFaceStruct::from_raw(env,obj)?}),"EAST" => Ok(BlockFace::East { inner: BlockFaceStruct::from_raw(env,obj)?}),"SOUTH" => Ok(BlockFace::South { inner: BlockFaceStruct::from_raw(env,obj)?}),"WEST" => Ok(BlockFace::West { inner: BlockFaceStruct::from_raw(env,obj)?}),"UP" => Ok(BlockFace::Up { inner: BlockFaceStruct::from_raw(env,obj)?}),"DOWN" => Ok(BlockFace::Down { inner: BlockFaceStruct::from_raw(env,obj)?}),"NORTH_EAST" => Ok(BlockFace::NorthEast { inner: BlockFaceStruct::from_raw(env,obj)?}),"NORTH_WEST" => Ok(BlockFace::NorthWest { inner: BlockFaceStruct::from_raw(env,obj)?}),"SOUTH_EAST" => Ok(BlockFace::SouthEast { inner: BlockFaceStruct::from_raw(env,obj)?}),"SOUTH_WEST" => Ok(BlockFace::SouthWest { inner: BlockFaceStruct::from_raw(env,obj)?}),"WEST_NORTH_WEST" => Ok(BlockFace::WestNorthWest { inner: BlockFaceStruct::from_raw(env,obj)?}),"NORTH_NORTH_WEST" => Ok(BlockFace::NorthNorthWest { inner: BlockFaceStruct::from_raw(env,obj)?}),"NORTH_NORTH_EAST" => Ok(BlockFace::NorthNorthEast { inner: BlockFaceStruct::from_raw(env,obj)?}),"EAST_NORTH_EAST" => Ok(BlockFace::EastNorthEast { inner: BlockFaceStruct::from_raw(env,obj)?}),"EAST_SOUTH_EAST" => Ok(BlockFace::EastSouthEast { inner: BlockFaceStruct::from_raw(env,obj)?}),"SOUTH_SOUTH_EAST" => Ok(BlockFace::SouthSouthEast { inner: BlockFaceStruct::from_raw(env,obj)?}),"SOUTH_SOUTH_WEST" => Ok(BlockFace::SouthSouthWest { inner: BlockFaceStruct::from_raw(env,obj)?}),"WEST_SOUTH_WEST" => Ok(BlockFace::WestSouthWest { inner: BlockFaceStruct::from_raw(env,obj)?}),"SELF" => Ok(BlockFace::VariantSelf { inner: BlockFaceStruct::from_raw(env,obj)?}),_ => Err(eyre::eyre!("String gaven for variant was invalid").into())}
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
                return Err(eyre::eyre!(
                    "Tried to instantiate BlockFaceStruct from null object.")
                .into());
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

	fn values(jni: &blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::block::BlockFace<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/block/BlockFace;");
let cls = jni.find_class("org/bukkit/block/BlockFace"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.call_static_method(cls,"values",
sig.as_str(),vec![]);
let res = 
jni.translate_error(res)?;
let obj = res.l()?;
crate::block::BlockFace::from_raw(&jni,obj
)}
/// Get the amount of X-coordinates to modify to get the represented block
	fn mod_x(&self) 
-> Result<i32, Box<dyn std::error::Error>>

{let sig = String::from("()I");
let res = self.jni_ref().call_method(&self.jni_object(),"getModX",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.i()?
)}
/// Get the amount of Y-coordinates to modify to get the represented block
	fn mod_y(&self) 
-> Result<i32, Box<dyn std::error::Error>>

{let sig = String::from("()I");
let res = self.jni_ref().call_method(&self.jni_object(),"getModY",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.i()?
)}
/// Get the amount of Z-coordinates to modify to get the represented block
	fn mod_z(&self) 
-> Result<i32, Box<dyn std::error::Error>>

{let sig = String::from("()I");
let res = self.jni_ref().call_method(&self.jni_object(),"getModZ",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.i()?
)}
/// Gets the normal vector corresponding to this block face.
	fn direction(&self) 
-> Result<crate::util::Vector<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/util/Vector;");
let res = self.jni_ref().call_method(&self.jni_object(),"getDirection",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::util::Vector::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
/// Returns true if this face is aligned with one of the unit axes in 3D
/// Cartesian space (ie NORTH, SOUTH, EAST, WEST, UP, DOWN).
	fn is_cartesian(&self) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("()Z");
let res = self.jni_ref().call_method(&self.jni_object(),"isCartesian",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}

	fn opposite_face(&self) 
-> Result<crate::block::BlockFace<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/block/BlockFace;");
let res = self.jni_ref().call_method(&self.jni_object(),"getOppositeFace",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::block::BlockFace::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
#[repr(C)]
pub struct HangingSign<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

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
                return Err(eyre::eyre!(
                    "Tried to instantiate HangingSign from null object.")
                .into());
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
    
impl<'mc> HangingSignTrait<'mc> for HangingSign<'mc> {}
pub trait HangingSignTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
impl<'mc> Into<crate::block::Sign<'mc>> for HangingSign<'mc>{

fn into(self) -> crate::block::Sign<'mc> {

crate::block::Sign::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting HangingSign into crate::block::Sign")

   }
}
impl<'mc> crate::block::SignTrait<'mc> for HangingSign<'mc> {}
#[repr(C)]
pub struct CommandBlock<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

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
                return Err(eyre::eyre!(
                    "Tried to instantiate CommandBlock from null object.")
                .into());
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
    
impl<'mc> CommandBlockTrait<'mc> for CommandBlock<'mc> {}
pub trait CommandBlockTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {
/// Gets the command that this CommandBlock will run when powered.
/// This will never return null.If the CommandBlock does not have a
/// command, an empty String will be returned instead.
	fn command(&self) 
-> Result<String, Box<dyn std::error::Error>>

{let sig = String::from("()Ljava/lang/String;");
let res = self.jni_ref().call_method(&self.jni_object(),"getCommand",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string()
)}
/// Sets the command that this CommandBlock will run when powered.
/// Setting the command to null is the same as setting it to an empty
/// String.
	fn set_command(&self,command: impl Into<String>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Ljava/lang/String;)V");
let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(command.into())?));
let res = self.jni_ref().call_method(&self.jni_object(),"setCommand",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
/// Gets the name of this CommandBlock.The name is used with commands
/// that this CommandBlock executes.This name will never be null, and
/// by default is "@".
	fn name(&self) 
-> Result<String, Box<dyn std::error::Error>>

{let sig = String::from("()Ljava/lang/String;");
let res = self.jni_ref().call_method(&self.jni_object(),"getName",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string()
)}
/// Sets the name of this CommandBlock.The name is used with commands
/// that this CommandBlock executes.Setting the name to null is the
/// same as setting it to "@".
	fn set_name(&self,name: impl Into<String>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Ljava/lang/String;)V");
let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(name.into())?));
let res = self.jni_ref().call_method(&self.jni_object(),"setName",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
impl<'mc> Into<crate::block::TileState<'mc>> for CommandBlock<'mc>{

fn into(self) -> crate::block::TileState<'mc> {

crate::block::TileState::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting CommandBlock into crate::block::TileState")

   }
}
impl<'mc> crate::block::TileStateTrait<'mc> for CommandBlock<'mc> {}
#[repr(C)]
pub struct TileState<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

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
                return Err(eyre::eyre!(
                    "Tried to instantiate TileState from null object.")
                .into());
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
    
impl<'mc> TileStateTrait<'mc> for TileState<'mc> {}
pub trait TileStateTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {
/// Returns a custom tag container capable of storing tags on the object.
/// Note that the tags stored on this container are all stored under their
/// own custom namespace therefore modifying default tags using this
/// {@link PersistentDataHolder} is impossible.
/// 
/// This {@link PersistentDataHolder} is only linked to the snapshot instance
/// stored by the {@link BlockState}.
/// When storing changes on the {@link PersistentDataHolder}, the updated
/// content will only be applied to the actual tile entity after one of the
/// {@link #update()} methods is called.
	fn persistent_data_container(&self) 
-> Result<crate::persistence::PersistentDataContainer<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/persistence/PersistentDataContainer;");
let res = self.jni_ref().call_method(&self.jni_object(),"getPersistentDataContainer",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::persistence::PersistentDataContainer::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
impl<'mc> Into<crate::block::BlockState<'mc>> for TileState<'mc>{

fn into(self) -> crate::block::BlockState<'mc> {

crate::block::BlockState::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting TileState into crate::block::BlockState")

   }
}
impl<'mc> crate::block::BlockStateTrait<'mc> for TileState<'mc> {}
impl<'mc> Into<crate::persistence::PersistentDataHolder<'mc>> for TileState<'mc>{

fn into(self) -> crate::persistence::PersistentDataHolder<'mc> {

crate::persistence::PersistentDataHolder::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting TileState into crate::persistence::PersistentDataHolder")

   }
}
impl<'mc> crate::persistence::PersistentDataHolderTrait<'mc> for TileState<'mc> {}
#[repr(C)]
pub struct Beehive<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

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
                return Err(eyre::eyre!(
                    "Tried to instantiate Beehive from null object.")
                .into());
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
    
impl<'mc> BeehiveTrait<'mc> for Beehive<'mc> {}
pub trait BeehiveTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {
/// Get the hive's flower location.
	fn flower(&self) 
-> Result<Option<crate::Location<'mc>>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/Location;");
let res = self.jni_ref().call_method(&self.jni_object(),"getFlower",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {return Ok(None);}
Ok(
Some(
crate::Location::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)?
)
)}
/// Set the hive's flower location.
	fn set_flower(&self,location: impl Into<crate::Location<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/Location;)V");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(location.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"setFlower",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
/// Check if the hive is sedated due to smoke from a nearby campfire.
	fn is_sedated(&self) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("()Z");
let res = self.jni_ref().call_method(&self.jni_object(),"isSedated",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
impl<'mc> Into<crate::block::EntityBlockStorage<'mc>> for Beehive<'mc>{

fn into(self) -> crate::block::EntityBlockStorage<'mc> {

crate::block::EntityBlockStorage::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting Beehive into crate::block::EntityBlockStorage")

   }
}
impl<'mc> crate::block::EntityBlockStorageTrait<'mc> for Beehive<'mc> {}
#[repr(C)]
pub struct Structure<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

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
                return Err(eyre::eyre!(
                    "Tried to instantiate Structure from null object.")
                .into());
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
    
impl<'mc> StructureTrait<'mc> for Structure<'mc> {}
pub trait StructureTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {
/// The name of this structure.
	fn structure_name(&self) 
-> Result<String, Box<dyn std::error::Error>>

{let sig = String::from("()Ljava/lang/String;");
let res = self.jni_ref().call_method(&self.jni_object(),"getStructureName",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string()
)}
/// Set the name of this structure. This is case-sensitive. The name of the
/// structure in the {@link UsageMode#SAVE} structure block MUST match the
/// name within the {@link UsageMode#CORNER} block or the size calculation
/// will fail.
	fn set_structure_name(&self,name: impl Into<String>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Ljava/lang/String;)V");
let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(name.into())?));
let res = self.jni_ref().call_method(&self.jni_object(),"setStructureName",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
/// Get the name of who created this structure.
	fn author(&self) 
-> Result<String, Box<dyn std::error::Error>>

{let sig = String::from("()Ljava/lang/String;");
let res = self.jni_ref().call_method(&self.jni_object(),"getAuthor",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string()
)}
/// Set the name of whoever created this structure using a
/// {@link LivingEntity}.
	fn set_author(&self,living_entity: impl Into<crate::entity::LivingEntity<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let mut args = Vec::new();
let mut sig = String::from("(");
sig += "Lorg/bukkit/entity/LivingEntity;";
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(living_entity.into().jni_object().clone())});
args.push(val_1);
sig += ")V";
let res = self.jni_ref().call_method(&self.jni_object(),"setAuthor",sig.as_str(),args);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
/// The relative position of the structure outline based on the position of
/// the structure block. Maximum allowed distance is 48 blocks in any
/// direction.
	fn relative_position(&self) 
-> Result<crate::util::BlockVector<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/util/BlockVector;");
let res = self.jni_ref().call_method(&self.jni_object(),"getRelativePosition",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::util::BlockVector::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
/// Set the relative position from the structure block. Maximum allowed
/// distance is 48 blocks in any direction.
	fn set_relative_position(&self,vector: impl Into<crate::util::BlockVector<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/util/BlockVector;)V");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(vector.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"setRelativePosition",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
/// The distance to the opposite corner of this structure. The maximum
/// structure size is 48x48x48. When a structure has successfully been
/// calculated (i.e. it is within the maximum allowed distance) a white
/// border surrounds the structure.
	fn structure_size(&self) 
-> Result<crate::util::BlockVector<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/util/BlockVector;");
let res = self.jni_ref().call_method(&self.jni_object(),"getStructureSize",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::util::BlockVector::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
/// Set the maximum size of this structure from the origin point. Maximum
/// allowed size is 48x48x48.
	fn set_structure_size(&self,vector: impl Into<crate::util::BlockVector<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/util/BlockVector;)V");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(vector.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"setStructureSize",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
/// Sets the mirroring of the structure.
	fn set_mirror(&self,mirror: impl Into<crate::block::structure::Mirror<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/block/structure/Mirror;)V");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(mirror.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"setMirror",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
/// How this structure is mirrored.
	fn mirror(&self) 
-> Result<crate::block::structure::Mirror<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/block/structure/Mirror;");
let res = self.jni_ref().call_method(&self.jni_object(),"getMirror",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::block::structure::Mirror::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
/// Set how this structure is rotated.
	fn set_rotation(&self,rotation: impl Into<crate::block::structure::StructureRotation<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/block/structure/StructureRotation;)V");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(rotation.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"setRotation",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
/// Get how this structure is rotated.
	fn rotation(&self) 
-> Result<crate::block::structure::StructureRotation<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/block/structure/StructureRotation;");
let res = self.jni_ref().call_method(&self.jni_object(),"getRotation",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::block::structure::StructureRotation::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
/// Set the {@link UsageMode} of this structure block.
	fn set_usage_mode(&self,mode: impl Into<crate::block::structure::UsageMode<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/block/structure/UsageMode;)V");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(mode.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"setUsageMode",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
/// Get the {@link UsageMode} of this structure block.
	fn usage_mode(&self) 
-> Result<crate::block::structure::UsageMode<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/block/structure/UsageMode;");
let res = self.jni_ref().call_method(&self.jni_object(),"getUsageMode",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::block::structure::UsageMode::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
/// While in {@link UsageMode#SAVE} mode, this will ignore any entities when
/// saving the structure.
/// 
/// While in {@link UsageMode#LOAD} mode this will ignore any entities that
/// were saved to file.
	fn set_ignore_entities(&self,ignore_entities: bool) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Z)V");
let val_1 = jni::objects::JValueGen::Bool(ignore_entities.into());
let res = self.jni_ref().call_method(&self.jni_object(),"setIgnoreEntities",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
/// Get if this structure block should ignore entities.
	fn is_ignore_entities(&self) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("()Z");
let res = self.jni_ref().call_method(&self.jni_object(),"isIgnoreEntities",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
/// Set if the structure outline should show air blocks.
	fn set_show_air(&self,show_air: bool) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Z)V");
let val_1 = jni::objects::JValueGen::Bool(show_air.into());
let res = self.jni_ref().call_method(&self.jni_object(),"setShowAir",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
/// Check if this structure block is currently showing all air blocks
	fn is_show_air(&self) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("()Z");
let res = self.jni_ref().call_method(&self.jni_object(),"isShowAir",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
/// Set if this structure box should show the bounding box.
	fn set_bounding_box_visible(&self,show_bounding_box: bool) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Z)V");
let val_1 = jni::objects::JValueGen::Bool(show_bounding_box.into());
let res = self.jni_ref().call_method(&self.jni_object(),"setBoundingBoxVisible",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
/// Get if this structure block is currently showing the bounding box.
	fn is_bounding_box_visible(&self) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("()Z");
let res = self.jni_ref().call_method(&self.jni_object(),"isBoundingBoxVisible",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
/// Set the integrity of the structure. Integrity must be between 0.0 and 1.0
/// Lower integrity values will result in more blocks being removed when
/// loading a structure. Integrity and {@link #getSeed()} are used together
/// to determine which blocks are randomly removed to mimic "decay."
	fn set_integrity(&self,integrity: f32) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(F)V");
let val_1 = jni::objects::JValueGen::Float(integrity);
let res = self.jni_ref().call_method(&self.jni_object(),"setIntegrity",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
/// Get the integrity of this structure.
	fn integrity(&self) 
-> Result<f32, Box<dyn std::error::Error>>

{let sig = String::from("()F");
let res = self.jni_ref().call_method(&self.jni_object(),"getIntegrity",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.f()?
)}
/// The seed used to determine which blocks will be removed upon loading.
/// {@link #getIntegrity()} and seed are used together to determine which
/// blocks are randomly removed to mimic "decay."
	fn set_seed(&self,seed: i64) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(J)V");
let val_1 = jni::objects::JValueGen::Long(seed);
let res = self.jni_ref().call_method(&self.jni_object(),"setSeed",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
/// The seed used to determine how many blocks are removed upon loading of
/// this structure.
	fn seed(&self) 
-> Result<i64, Box<dyn std::error::Error>>

{let sig = String::from("()J");
let res = self.jni_ref().call_method(&self.jni_object(),"getSeed",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.j()?
)}
/// Only applicable while in {@link UsageMode#DATA}. Metadata are specific
/// functions that can be applied to the structure location. Consult the
/// <a href="https://minecraft.wiki/w/Structure_Block#Data">Minecraft
/// wiki</a> for more information.
	fn set_metadata(&self,metadata: impl Into<String>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Ljava/lang/String;)V");
let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(metadata.into())?));
let res = self.jni_ref().call_method(&self.jni_object(),"setMetadata",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
/// Get the metadata function this structure block will perform when
/// activated. Consult the
/// <a href="https://minecraft.wiki/w/Structure_Block#Data">Minecraft
/// Wiki</a> for more information.
	fn metadata(&self) 
-> Result<String, Box<dyn std::error::Error>>

{let sig = String::from("()Ljava/lang/String;");
let res = self.jni_ref().call_method(&self.jni_object(),"getMetadata",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string()
)}

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
impl<'mc> Into<crate::block::TileState<'mc>> for Structure<'mc>{

fn into(self) -> crate::block::TileState<'mc> {

crate::block::TileState::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting Structure into crate::block::TileState")

   }
}
impl<'mc> crate::block::TileStateTrait<'mc> for Structure<'mc> {}
#[repr(C)]
pub struct ShulkerBox<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

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
                return Err(eyre::eyre!(
                    "Tried to instantiate ShulkerBox from null object.")
                .into());
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
    
impl<'mc> ShulkerBoxTrait<'mc> for ShulkerBox<'mc> {}
pub trait ShulkerBoxTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {
/// Get the {@link DyeColor} corresponding to this ShulkerBox
	fn color(&self) 
-> Result<Option<crate::DyeColor<'mc>>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/DyeColor;");
let res = self.jni_ref().call_method(&self.jni_object(),"getColor",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {return Ok(None);}
Ok(
Some(
crate::DyeColor::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)?
)
)}

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
impl<'mc> Into<crate::block::Container<'mc>> for ShulkerBox<'mc>{

fn into(self) -> crate::block::Container<'mc> {

crate::block::Container::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting ShulkerBox into crate::block::Container")

   }
}
impl<'mc> crate::block::ContainerTrait<'mc> for ShulkerBox<'mc> {}
impl<'mc> Into<crate::loot::Lootable<'mc>> for ShulkerBox<'mc>{

fn into(self) -> crate::loot::Lootable<'mc> {

crate::loot::Lootable::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting ShulkerBox into crate::loot::Lootable")

   }
}
impl<'mc> crate::loot::LootableTrait<'mc> for ShulkerBox<'mc> {}
impl<'mc> Into<crate::block::Lidded<'mc>> for ShulkerBox<'mc>{

fn into(self) -> crate::block::Lidded<'mc> {

crate::block::Lidded::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting ShulkerBox into crate::block::Lidded")

   }
}
impl<'mc> crate::block::LiddedTrait<'mc> for ShulkerBox<'mc> {}
#[repr(C)]
pub struct CreatureSpawner<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

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
                return Err(eyre::eyre!(
                    "Tried to instantiate CreatureSpawner from null object.")
                .into());
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
    
impl<'mc> CreatureSpawnerTrait<'mc> for CreatureSpawner<'mc> {}
pub trait CreatureSpawnerTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {
/// Get the spawner's creature type.
	fn spawned_type(&self) 
-> Result<Option<crate::entity::EntityType<'mc>>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/entity/EntityType;");
let res = self.jni_ref().call_method(&self.jni_object(),"getSpawnedType",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {return Ok(None);}
Ok(
Some(
crate::entity::EntityType::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)?
)
)}
/// Set the spawner's creature type.
/// 
/// This will override any entities that have been added with {@link #addPotentialSpawn}
	fn set_spawned_type(&self,creature_type: impl Into<crate::entity::EntityType<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/entity/EntityType;)V");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(creature_type.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"setSpawnedType",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
#[deprecated]
/// Set the spawner mob type.
	fn set_creature_type_by_name(&self,creature_type: impl Into<String>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Ljava/lang/String;)V");
let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(creature_type.into())?));
let res = self.jni_ref().call_method(&self.jni_object(),"setCreatureTypeByName",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
#[deprecated]
/// Get the spawner's creature type.
	fn creature_type_name(&self) 
-> Result<Option<String>, Box<dyn std::error::Error>>

{let sig = String::from("()Ljava/lang/String;");
let res = self.jni_ref().call_method(&self.jni_object(),"getCreatureTypeName",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {return Ok(None);}
Ok(
Some(
self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string()
)
)}
/// Get the spawner's delay.
/// 
/// This is the delay, in ticks, until the spawner will spawn its next mob.
	fn delay(&self) 
-> Result<i32, Box<dyn std::error::Error>>

{let sig = String::from("()I");
let res = self.jni_ref().call_method(&self.jni_object(),"getDelay",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.i()?
)}
/// Set the spawner's delay.
/// 
/// If set to -1, the spawn delay will be reset to a random value between
/// {@link #getMinSpawnDelay} and {@link #getMaxSpawnDelay()}.
	fn set_delay(&self,delay: i32) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(I)V");
let val_1 = jni::objects::JValueGen::Int(delay);
let res = self.jni_ref().call_method(&self.jni_object(),"setDelay",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
/// The minimum spawn delay amount (in ticks).
/// 
/// This value is used when the spawner resets its delay (for any reason).
/// It will choose a random number between {@link #getMinSpawnDelay()}
/// and {@link #getMaxSpawnDelay()} for its next {@link #getDelay()}.
/// 
/// Default value is 200 ticks.
	fn min_spawn_delay(&self) 
-> Result<i32, Box<dyn std::error::Error>>

{let sig = String::from("()I");
let res = self.jni_ref().call_method(&self.jni_object(),"getMinSpawnDelay",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.i()?
)}
/// Set the minimum spawn delay amount (in ticks).
	fn set_min_spawn_delay(&self,delay: i32) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(I)V");
let val_1 = jni::objects::JValueGen::Int(delay);
let res = self.jni_ref().call_method(&self.jni_object(),"setMinSpawnDelay",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
/// The maximum spawn delay amount (in ticks).
/// 
/// This value is used when the spawner resets its delay (for any reason).
/// It will choose a random number between {@link #getMinSpawnDelay()}
/// and {@link #getMaxSpawnDelay()} for its next {@link #getDelay()}.
/// 
/// This value <b>must</b> be greater than 0 and less than or equal to
/// {@link #getMaxSpawnDelay()}.
/// 
/// Default value is 800 ticks.
	fn max_spawn_delay(&self) 
-> Result<i32, Box<dyn std::error::Error>>

{let sig = String::from("()I");
let res = self.jni_ref().call_method(&self.jni_object(),"getMaxSpawnDelay",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.i()?
)}
/// Set the maximum spawn delay amount (in ticks).
/// 
/// This value <b>must</b> be greater than 0, as well as greater than or
/// equal to {@link #getMinSpawnDelay()}
	fn set_max_spawn_delay(&self,delay: i32) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(I)V");
let val_1 = jni::objects::JValueGen::Int(delay);
let res = self.jni_ref().call_method(&self.jni_object(),"setMaxSpawnDelay",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
/// Get how many mobs attempt to spawn.
/// 
/// Default value is 4.
	fn spawn_count(&self) 
-> Result<i32, Box<dyn std::error::Error>>

{let sig = String::from("()I");
let res = self.jni_ref().call_method(&self.jni_object(),"getSpawnCount",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.i()?
)}
/// Set how many mobs attempt to spawn.
	fn set_spawn_count(&self,spawn_count: i32) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(I)V");
let val_1 = jni::objects::JValueGen::Int(spawn_count);
let res = self.jni_ref().call_method(&self.jni_object(),"setSpawnCount",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
/// Set the new maximum amount of similar entities that are allowed to be
/// within spawning range of this spawner.
/// 
/// If more than the maximum number of entities are within range, the spawner
/// will not spawn and try again with a new {@link #getDelay()}.
/// 
/// Default value is 16.
	fn max_nearby_entities(&self) 
-> Result<i32, Box<dyn std::error::Error>>

{let sig = String::from("()I");
let res = self.jni_ref().call_method(&self.jni_object(),"getMaxNearbyEntities",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.i()?
)}
/// Set the maximum number of similar entities that are allowed to be within
/// spawning range of this spawner.
/// 
/// Similar entities are entities that are of the same {@link EntityType}
	fn set_max_nearby_entities(&self,max_nearby_entities: i32) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(I)V");
let val_1 = jni::objects::JValueGen::Int(max_nearby_entities);
let res = self.jni_ref().call_method(&self.jni_object(),"setMaxNearbyEntities",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
/// Get the maximum distance(squared) a player can be in order for this
/// spawner to be active.
/// 
/// If this value is less than or equal to 0, this spawner is always active
/// (given that there are players online).
/// 
/// Default value is 16.
	fn required_player_range(&self) 
-> Result<i32, Box<dyn std::error::Error>>

{let sig = String::from("()I");
let res = self.jni_ref().call_method(&self.jni_object(),"getRequiredPlayerRange",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.i()?
)}
/// Set the maximum distance (squared) a player can be in order for this
/// spawner to be active.
/// 
/// Setting this value to less than or equal to 0 will make this spawner
/// always active (given that there are players online).
	fn set_required_player_range(&self,required_player_range: i32) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(I)V");
let val_1 = jni::objects::JValueGen::Int(required_player_range);
let res = self.jni_ref().call_method(&self.jni_object(),"setRequiredPlayerRange",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
/// Get the radius around which the spawner will attempt to spawn mobs in.
/// 
/// This area is square, includes the block the spawner is in, and is
/// centered on the spawner's x,z coordinates - not the spawner itself.
/// 
/// It is 2 blocks high, centered on the spawner's y-coordinate (its bottom);
/// thus allowing mobs to spawn as high as its top surface and as low
/// as 1 block below its bottom surface.
/// 
/// Default value is 4.
	fn spawn_range(&self) 
-> Result<i32, Box<dyn std::error::Error>>

{let sig = String::from("()I");
let res = self.jni_ref().call_method(&self.jni_object(),"getSpawnRange",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.i()?
)}
/// Set the new spawn range.
/// 
	fn set_spawn_range(&self,spawn_range: i32) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(I)V");
let val_1 = jni::objects::JValueGen::Int(spawn_range);
let res = self.jni_ref().call_method(&self.jni_object(),"setSpawnRange",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
/// Gets the {@link EntitySnapshot} that will be spawned by this spawner or null
/// if no entities have been assigned to this spawner.
/// 
/// 
/// All applicable data from the spawner will be copied, such as custom name,
/// health, and velocity.
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
/// Sets the entity that will be spawned by this spawner.
/// 
/// This will override any previous entries that have been added with
/// {@link #addPotentialSpawn}
/// 
/// All applicable data from the snapshot will be copied, such as custom name,
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
/// Adds a new {@link EntitySnapshot} to the list of entities this spawner can
/// spawn.
/// 
/// The weight will determine how often this entry is chosen to spawn, higher
/// weighted entries will spawn more often than lower weighted ones.
/// 
/// The {@link SpawnRule} will determine under what conditions this entry can
/// spawn, passing null will use the default conditions for the given entity.
	fn add_potential_spawn(&self,snapshot: impl Into<crate::entity::EntitySnapshot<'mc>>,weight: std::option::Option<i32>,spawn_rule: std::option::Option<impl Into<crate::block::spawner::SpawnRule<'mc>>>) 
-> Result<(), Box<dyn std::error::Error>>

{let mut args = Vec::new();
let mut sig = String::from("(");
sig += "Lorg/bukkit/entity/EntitySnapshot;";
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(snapshot.into().jni_object().clone())});
args.push(val_1);
if let Some(a) = weight {
sig += "I";
let val_2 = jni::objects::JValueGen::Int(a);
args.push(val_2);
}
if let Some(a) = spawn_rule {
sig += "Lorg/bukkit/block/spawner/SpawnRule;";
let val_3 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(a.into().jni_object().clone())});
args.push(val_3);
}
sig += ")V";
let res = self.jni_ref().call_method(&self.jni_object(),"addPotentialSpawn",sig.as_str(),args);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
/// Gets a list of potential spawns from this spawner or an empty list if no
/// entities have been assigned to this spawner.
/// 
/// Changes made to the returned list will not be reflected in the spawner unless
/// applied with {@link #setPotentialSpawns}
	fn potential_spawns(&self) 
-> Result<Vec<crate::block::spawner::SpawnerEntry<'mc>>, Box<dyn std::error::Error>>

{let sig = String::from("()Ljava/util/List;");
let res = self.jni_ref().call_method(&self.jni_object(),"getPotentialSpawns",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
let mut new_vec = Vec::new();
let list = blackboxmc_java::util::JavaList::from_raw(&self.jni_ref(), res.l()?)?;let iter = list.iterator()?;
while iter.has_next()? {            let obj = iter.next()?;
new_vec.push(crate::block::spawner::SpawnerEntry::from_raw(&self.jni_ref(),obj,)?);
};
Ok(
new_vec
)}

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
impl<'mc> Into<crate::block::TileState<'mc>> for CreatureSpawner<'mc>{

fn into(self) -> crate::block::TileState<'mc> {

crate::block::TileState::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting CreatureSpawner into crate::block::TileState")

   }
}
impl<'mc> crate::block::TileStateTrait<'mc> for CreatureSpawner<'mc> {}
#[repr(C)]
pub struct Skull<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

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
                return Err(eyre::eyre!(
                    "Tried to instantiate Skull from null object.")
                .into());
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
    
impl<'mc> SkullTrait<'mc> for Skull<'mc> {}
pub trait SkullTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {
/// Checks to see if the skull has an owner
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
/// Gets the owner of the skull, if one exists
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
#[deprecated]
/// Sets the owner of the skullInvolves a potentially blocking web request to acquire the profile data for the provided name.
	fn set_owner(&self,name: impl Into<String>) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("(Ljava/lang/String;)Z");
let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(self.jni_ref().new_string(name.into())?));
let res = self.jni_ref().call_method(&self.jni_object(),"setOwner",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
/// Get the player which owns the skull. This player may appear as the
/// texture depending on skull type.
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
/// Set the player which owns the skull. This player may appear as the
/// texture depending on skull type.
	fn set_owning_player(&self,player: impl Into<crate::OfflinePlayer<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/OfflinePlayer;)V");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(player.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"setOwningPlayer",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
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
#[deprecated]
/// Gets the rotation of the skull in the world (or facing direction if this is a wall mounted skull).
	fn rotation(&self) 
-> Result<crate::block::BlockFace<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/block/BlockFace;");
let res = self.jni_ref().call_method(&self.jni_object(),"getRotation",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::block::BlockFace::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
#[deprecated]
/// Sets the rotation of the skull in the world (or facing direction if this is a wall mounted skull).
	fn set_rotation(&self,rotation: impl Into<crate::block::BlockFace<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/block/BlockFace;)V");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(rotation.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"setRotation",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
#[deprecated]
/// Gets the type of skull
	fn skull_type(&self) 
-> Result<crate::SkullType<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/SkullType;");
let res = self.jni_ref().call_method(&self.jni_object(),"getSkullType",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::SkullType::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
#[deprecated]
/// Sets the type of skull
	fn set_skull_type(&self,skull_type: impl Into<crate::SkullType<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/SkullType;)V");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(skull_type.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"setSkullType",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
impl<'mc> Into<crate::block::TileState<'mc>> for Skull<'mc>{

fn into(self) -> crate::block::TileState<'mc> {

crate::block::TileState::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting Skull into crate::block::TileState")

   }
}
impl<'mc> crate::block::TileStateTrait<'mc> for Skull<'mc> {}
#[repr(C)]
pub struct Bed<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

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
                return Err(eyre::eyre!(
                    "Tried to instantiate Bed from null object.")
                .into());
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
    
impl<'mc> BedTrait<'mc> for Bed<'mc> {}
pub trait BedTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
impl<'mc> Into<crate::block::TileState<'mc>> for Bed<'mc>{

fn into(self) -> crate::block::TileState<'mc> {

crate::block::TileState::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting Bed into crate::block::TileState")

   }
}
impl<'mc> crate::block::TileStateTrait<'mc> for Bed<'mc> {}
impl<'mc> Into<crate::material::Colorable<'mc>> for Bed<'mc>{

fn into(self) -> crate::material::Colorable<'mc> {

crate::material::Colorable::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting Bed into crate::material::Colorable")

   }
}
impl<'mc> crate::material::ColorableTrait<'mc> for Bed<'mc> {}
#[repr(C)]
pub struct Dispenser<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

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
                return Err(eyre::eyre!(
                    "Tried to instantiate Dispenser from null object.")
                .into());
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
    
impl<'mc> DispenserTrait<'mc> for Dispenser<'mc> {}
pub trait DispenserTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {
/// Gets the BlockProjectileSource object for the dispenser.
/// 
/// If the block represented by this state is no longer a dispenser, this
/// will return null.
	fn block_projectile_source(&self) 
-> Result<Option<crate::projectiles::BlockProjectileSource<'mc>>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/projectiles/BlockProjectileSource;");
let res = self.jni_ref().call_method(&self.jni_object(),"getBlockProjectileSource",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {return Ok(None);}
Ok(
Some(
crate::projectiles::BlockProjectileSource::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)?
)
)}
/// Attempts to dispense the contents of the dispenser.
/// 
/// If the block represented by this state is no longer a dispenser, this
/// will return false.
	fn dispense(&self) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("()Z");
let res = self.jni_ref().call_method(&self.jni_object(),"dispense",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
impl<'mc> Into<crate::block::Container<'mc>> for Dispenser<'mc>{

fn into(self) -> crate::block::Container<'mc> {

crate::block::Container::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting Dispenser into crate::block::Container")

   }
}
impl<'mc> crate::block::ContainerTrait<'mc> for Dispenser<'mc> {}
impl<'mc> Into<crate::Nameable<'mc>> for Dispenser<'mc>{

fn into(self) -> crate::Nameable<'mc> {

crate::Nameable::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting Dispenser into crate::Nameable")

   }
}
impl<'mc> crate::NameableTrait<'mc> for Dispenser<'mc> {}
impl<'mc> Into<crate::loot::Lootable<'mc>> for Dispenser<'mc>{

fn into(self) -> crate::loot::Lootable<'mc> {

crate::loot::Lootable::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting Dispenser into crate::loot::Lootable")

   }
}
impl<'mc> crate::loot::LootableTrait<'mc> for Dispenser<'mc> {}
#[repr(C)]
pub struct SuspiciousSand<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

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
                return Err(eyre::eyre!(
                    "Tried to instantiate SuspiciousSand from null object.")
                .into());
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
    
impl<'mc> SuspiciousSandTrait<'mc> for SuspiciousSand<'mc> {}
pub trait SuspiciousSandTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
impl<'mc> Into<crate::block::BrushableBlock<'mc>> for SuspiciousSand<'mc>{

fn into(self) -> crate::block::BrushableBlock<'mc> {

crate::block::BrushableBlock::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting SuspiciousSand into crate::block::BrushableBlock")

   }
}
impl<'mc> crate::block::BrushableBlockTrait<'mc> for SuspiciousSand<'mc> {}
#[repr(C)]
pub struct EnderChest<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

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
                return Err(eyre::eyre!(
                    "Tried to instantiate EnderChest from null object.")
                .into());
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
    
impl<'mc> EnderChestTrait<'mc> for EnderChest<'mc> {}
pub trait EnderChestTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
impl<'mc> Into<crate::block::Lidded<'mc>> for EnderChest<'mc>{

fn into(self) -> crate::block::Lidded<'mc> {

crate::block::Lidded::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting EnderChest into crate::block::Lidded")

   }
}
impl<'mc> crate::block::LiddedTrait<'mc> for EnderChest<'mc> {}
impl<'mc> Into<crate::block::TileState<'mc>> for EnderChest<'mc>{

fn into(self) -> crate::block::TileState<'mc> {

crate::block::TileState::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting EnderChest into crate::block::TileState")

   }
}
impl<'mc> crate::block::TileStateTrait<'mc> for EnderChest<'mc> {}
#[repr(C)]
pub struct Chest<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

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
                return Err(eyre::eyre!(
                    "Tried to instantiate Chest from null object.")
                .into());
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
    
impl<'mc> ChestTrait<'mc> for Chest<'mc> {}
pub trait ChestTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {
/// Gets the inventory of the chest block represented by this block state.
/// 
/// If the chest is a double chest, it returns just the portion of the
/// inventory linked to the half of the chest corresponding to this block state.
/// 
/// If the block was changed to a different type in the meantime, the
/// returned inventory might no longer be valid.
/// 
/// If this block state is not placed this will return the captured
/// inventory snapshot instead.
	fn block_inventory(&self) 
-> Result<crate::inventory::Inventory<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/inventory/Inventory;");
let res = self.jni_ref().call_method(&self.jni_object(),"getBlockInventory",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::inventory::Inventory::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
impl<'mc> Into<crate::block::Container<'mc>> for Chest<'mc>{

fn into(self) -> crate::block::Container<'mc> {

crate::block::Container::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting Chest into crate::block::Container")

   }
}
impl<'mc> crate::block::ContainerTrait<'mc> for Chest<'mc> {}
impl<'mc> Into<crate::loot::Lootable<'mc>> for Chest<'mc>{

fn into(self) -> crate::loot::Lootable<'mc> {

crate::loot::Lootable::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting Chest into crate::loot::Lootable")

   }
}
impl<'mc> crate::loot::LootableTrait<'mc> for Chest<'mc> {}
impl<'mc> Into<crate::block::Lidded<'mc>> for Chest<'mc>{

fn into(self) -> crate::block::Lidded<'mc> {

crate::block::Lidded::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting Chest into crate::block::Lidded")

   }
}
impl<'mc> crate::block::LiddedTrait<'mc> for Chest<'mc> {}
pub mod banner;
pub mod spawner;
pub mod data;
pub mod structure;
pub mod sign;
