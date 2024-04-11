#![allow(deprecated)]
use blackboxmc_general::JNIRaw;
use blackboxmc_general::JNIInstantiatable;
use color_eyre::eyre::Result;
#[repr(C)]
pub struct HopperInventorySearchEvent<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

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
                    "Tried to instantiate HopperInventorySearchEvent from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/event/inventory/HopperInventorySearchEvent")?;
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
	pub fn new(jni: &blackboxmc_general::SharedJNIEnv<'mc>,inventory: impl Into<crate::inventory::Inventory<'mc>>,container_type: impl Into<crate::event::inventory::HopperInventorySearchEventContainerType<'mc>>,hopper: impl Into<crate::block::Block<'mc>>,search_block: impl Into<crate::block::Block<'mc>>) 
-> Result<crate::event::inventory::HopperInventorySearchEvent<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/inventory/Inventory;Lorg/bukkit/event/inventory/HopperInventorySearchEvent/ContainerType;Lorg/bukkit/block/Block;Lorg/bukkit/block/Block;)V");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(inventory.into().jni_object().clone())});
let val_2 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(container_type.into().jni_object().clone())});
let val_3 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(hopper.into().jni_object().clone())});
let val_4 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(search_block.into().jni_object().clone())});
let cls = jni.find_class("org/bukkit/event/inventory/HopperInventorySearchEvent"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.new_object(cls,
sig.as_str(),vec![jni::objects::JValueGen::from(val_1),jni::objects::JValueGen::from(val_2),jni::objects::JValueGen::from(val_3),jni::objects::JValueGen::from(val_4)]);
let res = 
jni.translate_error_no_gen(res)?;
crate::event::inventory::HopperInventorySearchEvent::from_raw(&jni,res
)}
	pub fn set_inventory(&self,inventory: impl Into<crate::inventory::Inventory<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/inventory/Inventory;)L();");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(inventory.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"setInventory",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
	pub fn inventory(&self) 
-> Result<Option<crate::inventory::Inventory<'mc>>, Box<dyn std::error::Error>>

{let sig = String::from("()Lcrate::inventory::Inventory;");
let res = self.jni_ref().call_method(&self.jni_object(),"getInventory",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {return Ok(None);}
Ok(
Some(
crate::inventory::Inventory::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)?
)
)}
	pub fn container_type(&self) 
-> Result<crate::event::inventory::HopperInventorySearchEventContainerType<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lcrate::event::inventory::HopperInventorySearchEventContainerType;");
let res = self.jni_ref().call_method(&self.jni_object(),"getContainerType",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::event::inventory::HopperInventorySearchEventContainerType::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn search_block(&self) 
-> Result<crate::block::Block<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lcrate::block::Block;");
let res = self.jni_ref().call_method(&self.jni_object(),"getSearchBlock",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::block::Block::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn handlers(&self) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lcrate::event::HandlerList;");
let res = self.jni_ref().call_method(&self.jni_object(),"getHandlers",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::event::HandlerList::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn handler_list(jni: &blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lcrate::event::HandlerList;");
let cls = jni.find_class("org/bukkit/event/inventory/HopperInventorySearchEvent"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.call_static_method(cls,"getHandlerList",
sig.as_str(),vec![]);
let res = 
jni.translate_error(res)?;
let obj = res.l()?;
crate::event::HandlerList::from_raw(&jni,obj
)}
// SUPER CLASS: BlockEvent

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
impl<'mc> Into<crate::event::block::BlockEvent<'mc>> for HopperInventorySearchEvent<'mc>{

fn into(self) -> crate::event::block::BlockEvent<'mc> {

crate::event::block::BlockEvent::from_raw(&self.jni_ref(), self.1).expect("Error converting HopperInventorySearchEvent into crate::event::block::BlockEvent")

   }
}
#[repr(C)]
pub struct FurnaceBurnEvent<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

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
                return Err(eyre::eyre!(
                    "Tried to instantiate FurnaceBurnEvent from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/event/inventory/FurnaceBurnEvent")?;
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
	pub fn new(jni: &blackboxmc_general::SharedJNIEnv<'mc>,furnace: impl Into<crate::block::Block<'mc>>,fuel: impl Into<crate::inventory::ItemStack<'mc>>,burn_time: i32) 
-> Result<crate::event::inventory::FurnaceBurnEvent<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/block/Block;Lorg/bukkit/inventory/ItemStack;I)V");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(furnace.into().jni_object().clone())});
let val_2 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(fuel.into().jni_object().clone())});
let val_3 = jni::objects::JValueGen::Int(burn_time);
let cls = jni.find_class("org/bukkit/event/inventory/FurnaceBurnEvent"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.new_object(cls,
sig.as_str(),vec![jni::objects::JValueGen::from(val_1),jni::objects::JValueGen::from(val_2),jni::objects::JValueGen::from(val_3)]);
let res = 
jni.translate_error_no_gen(res)?;
crate::event::inventory::FurnaceBurnEvent::from_raw(&jni,res
)}
	pub fn fuel(&self) 
-> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lcrate::inventory::ItemStack;");
let res = self.jni_ref().call_method(&self.jni_object(),"getFuel",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::inventory::ItemStack::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn burn_time(&self) 
-> Result<i32, Box<dyn std::error::Error>>

{let sig = String::from("()Li32;");
let res = self.jni_ref().call_method(&self.jni_object(),"getBurnTime",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.i()?
)}
	pub fn set_burn_time(&self,burn_time: i32) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(I)L();");
let val_1 = jni::objects::JValueGen::Int(burn_time);
let res = self.jni_ref().call_method(&self.jni_object(),"setBurnTime",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
	pub fn is_burning(&self) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("()Lbool;");
let res = self.jni_ref().call_method(&self.jni_object(),"isBurning",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
	pub fn set_burning(&self,burning: bool) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Z)L();");
let val_1 = jni::objects::JValueGen::Bool(burning.into());
let res = self.jni_ref().call_method(&self.jni_object(),"setBurning",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
	pub fn is_cancelled(&self) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("()Lbool;");
let res = self.jni_ref().call_method(&self.jni_object(),"isCancelled",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
	pub fn set_cancelled(&self,cancel: bool) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Z)L();");
let val_1 = jni::objects::JValueGen::Bool(cancel.into());
let res = self.jni_ref().call_method(&self.jni_object(),"setCancelled",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
	pub fn handlers(&self) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lcrate::event::HandlerList;");
let res = self.jni_ref().call_method(&self.jni_object(),"getHandlers",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::event::HandlerList::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn handler_list(jni: &blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lcrate::event::HandlerList;");
let cls = jni.find_class("org/bukkit/event/inventory/FurnaceBurnEvent"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.call_static_method(cls,"getHandlerList",
sig.as_str(),vec![]);
let res = 
jni.translate_error(res)?;
let obj = res.l()?;
crate::event::HandlerList::from_raw(&jni,obj
)}
// SUPER CLASS: BlockEvent

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
impl<'mc> Into<crate::event::Cancellable<'mc>> for FurnaceBurnEvent<'mc>{

fn into(self) -> crate::event::Cancellable<'mc> {

crate::event::Cancellable::from_raw(&self.jni_ref(), self.1).expect("Error converting FurnaceBurnEvent into crate::event::Cancellable")

   }
}
impl<'mc> Into<crate::event::block::BlockEvent<'mc>> for FurnaceBurnEvent<'mc>{

fn into(self) -> crate::event::block::BlockEvent<'mc> {

crate::event::block::BlockEvent::from_raw(&self.jni_ref(), self.1).expect("Error converting FurnaceBurnEvent into crate::event::block::BlockEvent")

   }
}
#[repr(C)]
pub struct InventoryOpenEvent<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

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
                return Err(eyre::eyre!(
                    "Tried to instantiate InventoryOpenEvent from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/event/inventory/InventoryOpenEvent")?;
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
	pub fn new(jni: &blackboxmc_general::SharedJNIEnv<'mc>,transaction: impl Into<crate::inventory::InventoryView<'mc>>) 
-> Result<crate::event::inventory::InventoryOpenEvent<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/inventory/InventoryView;)V");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(transaction.into().jni_object().clone())});
let cls = jni.find_class("org/bukkit/event/inventory/InventoryOpenEvent"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.new_object(cls,
sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
jni.translate_error_no_gen(res)?;
crate::event::inventory::InventoryOpenEvent::from_raw(&jni,res
)}
	pub fn player(&self) 
-> Result<crate::entity::HumanEntity<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lcrate::entity::HumanEntity;");
let res = self.jni_ref().call_method(&self.jni_object(),"getPlayer",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::entity::HumanEntity::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn is_cancelled(&self) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("()Lbool;");
let res = self.jni_ref().call_method(&self.jni_object(),"isCancelled",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
	pub fn set_cancelled(&self,cancel: bool) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Z)L();");
let val_1 = jni::objects::JValueGen::Bool(cancel.into());
let res = self.jni_ref().call_method(&self.jni_object(),"setCancelled",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
	pub fn handlers(&self) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lcrate::event::HandlerList;");
let res = self.jni_ref().call_method(&self.jni_object(),"getHandlers",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::event::HandlerList::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn handler_list(jni: &blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lcrate::event::HandlerList;");
let cls = jni.find_class("org/bukkit/event/inventory/InventoryOpenEvent"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.call_static_method(cls,"getHandlerList",
sig.as_str(),vec![]);
let res = 
jni.translate_error(res)?;
let obj = res.l()?;
crate::event::HandlerList::from_raw(&jni,obj
)}
// SUPER CLASS: InventoryEvent

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
impl<'mc> Into<crate::event::Cancellable<'mc>> for InventoryOpenEvent<'mc>{

fn into(self) -> crate::event::Cancellable<'mc> {

crate::event::Cancellable::from_raw(&self.jni_ref(), self.1).expect("Error converting InventoryOpenEvent into crate::event::Cancellable")

   }
}
impl<'mc> Into<crate::event::inventory::InventoryEvent<'mc>> for InventoryOpenEvent<'mc>{

fn into(self) -> crate::event::inventory::InventoryEvent<'mc> {

crate::event::inventory::InventoryEvent::from_raw(&self.jni_ref(), self.1).expect("Error converting InventoryOpenEvent into crate::event::inventory::InventoryEvent")

   }
}
pub enum ClickType<'mc> {
	Left {inner: ClickTypeStruct<'mc>},
	ShiftLeft {inner: ClickTypeStruct<'mc>},
	Right {inner: ClickTypeStruct<'mc>},
	ShiftRight {inner: ClickTypeStruct<'mc>},
	WindowBorderLeft {inner: ClickTypeStruct<'mc>},
	WindowBorderRight {inner: ClickTypeStruct<'mc>},
	Middle {inner: ClickTypeStruct<'mc>},
	NumberKey {inner: ClickTypeStruct<'mc>},
	DoubleClick {inner: ClickTypeStruct<'mc>},
	Drop {inner: ClickTypeStruct<'mc>},
	ControlDrop {inner: ClickTypeStruct<'mc>},
	Creative {inner: ClickTypeStruct<'mc>},
	SwapOffhand {inner: ClickTypeStruct<'mc>},
	Unknown {inner: ClickTypeStruct<'mc>},
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
                    
"LEFT" => Ok(ClickType::Left { inner: ClickTypeStruct::from_raw(env,obj)?}),
"SHIFT_LEFT" => Ok(ClickType::ShiftLeft { inner: ClickTypeStruct::from_raw(env,obj)?}),
"RIGHT" => Ok(ClickType::Right { inner: ClickTypeStruct::from_raw(env,obj)?}),
"SHIFT_RIGHT" => Ok(ClickType::ShiftRight { inner: ClickTypeStruct::from_raw(env,obj)?}),
"WINDOW_BORDER_LEFT" => Ok(ClickType::WindowBorderLeft { inner: ClickTypeStruct::from_raw(env,obj)?}),
"WINDOW_BORDER_RIGHT" => Ok(ClickType::WindowBorderRight { inner: ClickTypeStruct::from_raw(env,obj)?}),
"MIDDLE" => Ok(ClickType::Middle { inner: ClickTypeStruct::from_raw(env,obj)?}),
"NUMBER_KEY" => Ok(ClickType::NumberKey { inner: ClickTypeStruct::from_raw(env,obj)?}),
"DOUBLE_CLICK" => Ok(ClickType::DoubleClick { inner: ClickTypeStruct::from_raw(env,obj)?}),
"DROP" => Ok(ClickType::Drop { inner: ClickTypeStruct::from_raw(env,obj)?}),
"CONTROL_DROP" => Ok(ClickType::ControlDrop { inner: ClickTypeStruct::from_raw(env,obj)?}),
"CREATIVE" => Ok(ClickType::Creative { inner: ClickTypeStruct::from_raw(env,obj)?}),
"SWAP_OFFHAND" => Ok(ClickType::SwapOffhand { inner: ClickTypeStruct::from_raw(env,obj)?}),
"UNKNOWN" => Ok(ClickType::Unknown { inner: ClickTypeStruct::from_raw(env,obj)?}),

                    _ => Err(eyre::eyre!("String gaven for variant was invalid").into())
                }
            }
        }
        
#[repr(C)]
pub struct ClickTypeStruct<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

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
Self::ShiftLeft { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::Right { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::ShiftRight { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::WindowBorderLeft { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::WindowBorderRight { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::Middle { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::NumberKey { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::DoubleClick { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::Drop { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::ControlDrop { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::Creative { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::SwapOffhand { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
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
                return Err(eyre::eyre!(
                    "Tried to instantiate ClickType from null object.")
                .into());
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
                    "LEFT" => Ok(ClickType::Left { inner: ClickTypeStruct::from_raw(env,obj)?}),"SHIFT_LEFT" => Ok(ClickType::ShiftLeft { inner: ClickTypeStruct::from_raw(env,obj)?}),"RIGHT" => Ok(ClickType::Right { inner: ClickTypeStruct::from_raw(env,obj)?}),"SHIFT_RIGHT" => Ok(ClickType::ShiftRight { inner: ClickTypeStruct::from_raw(env,obj)?}),"WINDOW_BORDER_LEFT" => Ok(ClickType::WindowBorderLeft { inner: ClickTypeStruct::from_raw(env,obj)?}),"WINDOW_BORDER_RIGHT" => Ok(ClickType::WindowBorderRight { inner: ClickTypeStruct::from_raw(env,obj)?}),"MIDDLE" => Ok(ClickType::Middle { inner: ClickTypeStruct::from_raw(env,obj)?}),"NUMBER_KEY" => Ok(ClickType::NumberKey { inner: ClickTypeStruct::from_raw(env,obj)?}),"DOUBLE_CLICK" => Ok(ClickType::DoubleClick { inner: ClickTypeStruct::from_raw(env,obj)?}),"DROP" => Ok(ClickType::Drop { inner: ClickTypeStruct::from_raw(env,obj)?}),"CONTROL_DROP" => Ok(ClickType::ControlDrop { inner: ClickTypeStruct::from_raw(env,obj)?}),"CREATIVE" => Ok(ClickType::Creative { inner: ClickTypeStruct::from_raw(env,obj)?}),"SWAP_OFFHAND" => Ok(ClickType::SwapOffhand { inner: ClickTypeStruct::from_raw(env,obj)?}),"UNKNOWN" => Ok(ClickType::Unknown { inner: ClickTypeStruct::from_raw(env,obj)?}),_ => Err(eyre::eyre!("String gaven for variant was invalid").into())}
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
                return Err(eyre::eyre!(
                    "Tried to instantiate ClickTypeStruct from null object.")
                .into());
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
	pub fn values(jni: &blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::event::inventory::ClickType<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lcrate::event::inventory::ClickType;");
let cls = jni.find_class("org/bukkit/event/inventory/ClickType"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.call_static_method(cls,"values",
sig.as_str(),vec![]);
let res = 
jni.translate_error(res)?;
let obj = res.l()?;
crate::event::inventory::ClickType::from_raw(&jni,obj
)}
	pub fn is_keyboard_click(&self) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("()Lbool;");
let res = self.jni_ref().call_method(&self.jni_object(),"isKeyboardClick",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
	pub fn is_mouse_click(&self) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("()Lbool;");
let res = self.jni_ref().call_method(&self.jni_object(),"isMouseClick",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
	pub fn is_creative_action(&self) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("()Lbool;");
let res = self.jni_ref().call_method(&self.jni_object(),"isCreativeAction",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
	pub fn is_right_click(&self) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("()Lbool;");
let res = self.jni_ref().call_method(&self.jni_object(),"isRightClick",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
	pub fn is_left_click(&self) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("()Lbool;");
let res = self.jni_ref().call_method(&self.jni_object(),"isLeftClick",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
	pub fn is_shift_click(&self) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("()Lbool;");
let res = self.jni_ref().call_method(&self.jni_object(),"isShiftClick",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
// SUPER CLASS: Enum

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
#[repr(C)]
pub struct TradeSelectEvent<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

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
                return Err(eyre::eyre!(
                    "Tried to instantiate TradeSelectEvent from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/event/inventory/TradeSelectEvent")?;
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
	pub fn new(jni: &blackboxmc_general::SharedJNIEnv<'mc>,transaction: impl Into<crate::inventory::InventoryView<'mc>>,new_index: i32) 
-> Result<crate::event::inventory::TradeSelectEvent<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/inventory/InventoryView;I)V");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(transaction.into().jni_object().clone())});
let val_2 = jni::objects::JValueGen::Int(new_index);
let cls = jni.find_class("org/bukkit/event/inventory/TradeSelectEvent"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.new_object(cls,
sig.as_str(),vec![jni::objects::JValueGen::from(val_1),jni::objects::JValueGen::from(val_2)]);
let res = 
jni.translate_error_no_gen(res)?;
crate::event::inventory::TradeSelectEvent::from_raw(&jni,res
)}
	pub fn index(&self) 
-> Result<i32, Box<dyn std::error::Error>>

{let sig = String::from("()Li32;");
let res = self.jni_ref().call_method(&self.jni_object(),"getIndex",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.i()?
)}
	pub fn inventory(&self) 
-> Result<crate::inventory::MerchantInventory<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lcrate::inventory::MerchantInventory;");
let res = self.jni_ref().call_method(&self.jni_object(),"getInventory",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::inventory::MerchantInventory::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn merchant(&self) 
-> Result<crate::inventory::Merchant<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lcrate::inventory::Merchant;");
let res = self.jni_ref().call_method(&self.jni_object(),"getMerchant",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::inventory::Merchant::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn handlers(&self) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lcrate::event::HandlerList;");
let res = self.jni_ref().call_method(&self.jni_object(),"getHandlers",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::event::HandlerList::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn handler_list(jni: &blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lcrate::event::HandlerList;");
let cls = jni.find_class("org/bukkit/event/inventory/TradeSelectEvent"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.call_static_method(cls,"getHandlerList",
sig.as_str(),vec![]);
let res = 
jni.translate_error(res)?;
let obj = res.l()?;
crate::event::HandlerList::from_raw(&jni,obj
)}
// SUPER CLASS: InventoryInteractEvent

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
impl<'mc> Into<crate::event::inventory::InventoryInteractEvent<'mc>> for TradeSelectEvent<'mc>{

fn into(self) -> crate::event::inventory::InventoryInteractEvent<'mc> {

crate::event::inventory::InventoryInteractEvent::from_raw(&self.jni_ref(), self.1).expect("Error converting TradeSelectEvent into crate::event::inventory::InventoryInteractEvent")

   }
}
#[repr(C)]
pub struct InventoryCloseEvent<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

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
                return Err(eyre::eyre!(
                    "Tried to instantiate InventoryCloseEvent from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/event/inventory/InventoryCloseEvent")?;
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
	pub fn new(jni: &blackboxmc_general::SharedJNIEnv<'mc>,transaction: impl Into<crate::inventory::InventoryView<'mc>>) 
-> Result<crate::event::inventory::InventoryCloseEvent<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/inventory/InventoryView;)V");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(transaction.into().jni_object().clone())});
let cls = jni.find_class("org/bukkit/event/inventory/InventoryCloseEvent"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.new_object(cls,
sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
jni.translate_error_no_gen(res)?;
crate::event::inventory::InventoryCloseEvent::from_raw(&jni,res
)}
	pub fn player(&self) 
-> Result<crate::entity::HumanEntity<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lcrate::entity::HumanEntity;");
let res = self.jni_ref().call_method(&self.jni_object(),"getPlayer",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::entity::HumanEntity::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn handlers(&self) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lcrate::event::HandlerList;");
let res = self.jni_ref().call_method(&self.jni_object(),"getHandlers",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::event::HandlerList::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn handler_list(jni: &blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lcrate::event::HandlerList;");
let cls = jni.find_class("org/bukkit/event/inventory/InventoryCloseEvent"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.call_static_method(cls,"getHandlerList",
sig.as_str(),vec![]);
let res = 
jni.translate_error(res)?;
let obj = res.l()?;
crate::event::HandlerList::from_raw(&jni,obj
)}
// SUPER CLASS: InventoryEvent

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
impl<'mc> Into<crate::event::inventory::InventoryEvent<'mc>> for InventoryCloseEvent<'mc>{

fn into(self) -> crate::event::inventory::InventoryEvent<'mc> {

crate::event::inventory::InventoryEvent::from_raw(&self.jni_ref(), self.1).expect("Error converting InventoryCloseEvent into crate::event::inventory::InventoryEvent")

   }
}
#[repr(C)]
pub struct InventoryCreativeEvent<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

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
                    "Tried to instantiate InventoryCreativeEvent from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/event/inventory/InventoryCreativeEvent")?;
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
	pub fn new(jni: &blackboxmc_general::SharedJNIEnv<'mc>,what: impl Into<crate::inventory::InventoryView<'mc>>,val_type: impl Into<crate::event::inventory::InventoryTypeSlotType<'mc>>,slot: i32,new_item: impl Into<crate::inventory::ItemStack<'mc>>) 
-> Result<crate::event::inventory::InventoryCreativeEvent<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/inventory/InventoryView;Lorg/bukkit/event/inventory/InventoryType/SlotType;ILorg/bukkit/inventory/ItemStack;)V");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(what.into().jni_object().clone())});
let val_2 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(val_type.into().jni_object().clone())});
let val_3 = jni::objects::JValueGen::Int(slot);
let val_4 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(new_item.into().jni_object().clone())});
let cls = jni.find_class("org/bukkit/event/inventory/InventoryCreativeEvent"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.new_object(cls,
sig.as_str(),vec![jni::objects::JValueGen::from(val_1),jni::objects::JValueGen::from(val_2),jni::objects::JValueGen::from(val_3),jni::objects::JValueGen::from(val_4)]);
let res = 
jni.translate_error_no_gen(res)?;
crate::event::inventory::InventoryCreativeEvent::from_raw(&jni,res
)}
	pub fn cursor(&self) 
-> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lcrate::inventory::ItemStack;");
let res = self.jni_ref().call_method(&self.jni_object(),"getCursor",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::inventory::ItemStack::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn set_cursor(&self,item: impl Into<crate::inventory::ItemStack<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/inventory/ItemStack;)L();");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(item.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"setCursor",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
// SUPER CLASS: InventoryClickEvent

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
impl<'mc> Into<crate::event::inventory::InventoryClickEvent<'mc>> for InventoryCreativeEvent<'mc>{

fn into(self) -> crate::event::inventory::InventoryClickEvent<'mc> {

crate::event::inventory::InventoryClickEvent::from_raw(&self.jni_ref(), self.1).expect("Error converting InventoryCreativeEvent into crate::event::inventory::InventoryClickEvent")

   }
}
#[repr(C)]
pub struct PrepareSmithingEvent<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

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
                return Err(eyre::eyre!(
                    "Tried to instantiate PrepareSmithingEvent from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/event/inventory/PrepareSmithingEvent")?;
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
	pub fn new(jni: &blackboxmc_general::SharedJNIEnv<'mc>,inventory: impl Into<crate::inventory::InventoryView<'mc>>,result: impl Into<crate::inventory::ItemStack<'mc>>) 
-> Result<crate::event::inventory::PrepareSmithingEvent<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/inventory/InventoryView;Lorg/bukkit/inventory/ItemStack;)V");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(inventory.into().jni_object().clone())});
let val_2 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(result.into().jni_object().clone())});
let cls = jni.find_class("org/bukkit/event/inventory/PrepareSmithingEvent"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.new_object(cls,
sig.as_str(),vec![jni::objects::JValueGen::from(val_1),jni::objects::JValueGen::from(val_2)]);
let res = 
jni.translate_error_no_gen(res)?;
crate::event::inventory::PrepareSmithingEvent::from_raw(&jni,res
)}
	pub fn inventory(&self) 
-> Result<crate::inventory::SmithingInventory<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lcrate::inventory::SmithingInventory;");
let res = self.jni_ref().call_method(&self.jni_object(),"getInventory",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::inventory::SmithingInventory::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn handlers(&self) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lcrate::event::HandlerList;");
let res = self.jni_ref().call_method(&self.jni_object(),"getHandlers",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::event::HandlerList::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn handler_list(jni: &blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lcrate::event::HandlerList;");
let cls = jni.find_class("org/bukkit/event/inventory/PrepareSmithingEvent"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.call_static_method(cls,"getHandlerList",
sig.as_str(),vec![]);
let res = 
jni.translate_error(res)?;
let obj = res.l()?;
crate::event::HandlerList::from_raw(&jni,obj
)}
// SUPER CLASS: PrepareInventoryResultEvent

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
impl<'mc> Into<crate::event::inventory::PrepareInventoryResultEvent<'mc>> for PrepareSmithingEvent<'mc>{

fn into(self) -> crate::event::inventory::PrepareInventoryResultEvent<'mc> {

crate::event::inventory::PrepareInventoryResultEvent::from_raw(&self.jni_ref(), self.1).expect("Error converting PrepareSmithingEvent into crate::event::inventory::PrepareInventoryResultEvent")

   }
}
pub enum DragType<'mc> {
	Single {inner: DragTypeStruct<'mc>},
	Even {inner: DragTypeStruct<'mc>},
}
impl<'mc> std::fmt::Display for DragType<'mc> {
   fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
       match self {
           DragType::Single { .. } => f.write_str("SINGLE"),
           DragType::Even { .. } => f.write_str("EVEN"),
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
                    
"SINGLE" => Ok(DragType::Single { inner: DragTypeStruct::from_raw(env,obj)?}),
"EVEN" => Ok(DragType::Even { inner: DragTypeStruct::from_raw(env,obj)?}),

                    _ => Err(eyre::eyre!("String gaven for variant was invalid").into())
                }
            }
        }
        
#[repr(C)]
pub struct DragTypeStruct<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

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
                return Err(eyre::eyre!(
                    "Tried to instantiate DragType from null object.")
                .into());
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
                    "SINGLE" => Ok(DragType::Single { inner: DragTypeStruct::from_raw(env,obj)?}),"EVEN" => Ok(DragType::Even { inner: DragTypeStruct::from_raw(env,obj)?}),_ => Err(eyre::eyre!("String gaven for variant was invalid").into())}
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
                return Err(eyre::eyre!(
                    "Tried to instantiate DragTypeStruct from null object.")
                .into());
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
	pub fn values(jni: &blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::event::inventory::DragType<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lcrate::event::inventory::DragType;");
let cls = jni.find_class("org/bukkit/event/inventory/DragType"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.call_static_method(cls,"values",
sig.as_str(),vec![]);
let res = 
jni.translate_error(res)?;
let obj = res.l()?;
crate::event::inventory::DragType::from_raw(&jni,obj
)}
// SUPER CLASS: Enum

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
#[repr(C)]
pub struct BrewingStandFuelEvent<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

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
                    "Tried to instantiate BrewingStandFuelEvent from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/event/inventory/BrewingStandFuelEvent")?;
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
	pub fn new(jni: &blackboxmc_general::SharedJNIEnv<'mc>,brewing_stand: impl Into<crate::block::Block<'mc>>,fuel: impl Into<crate::inventory::ItemStack<'mc>>,fuel_power: i32) 
-> Result<crate::event::inventory::BrewingStandFuelEvent<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/block/Block;Lorg/bukkit/inventory/ItemStack;I)V");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(brewing_stand.into().jni_object().clone())});
let val_2 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(fuel.into().jni_object().clone())});
let val_3 = jni::objects::JValueGen::Int(fuel_power);
let cls = jni.find_class("org/bukkit/event/inventory/BrewingStandFuelEvent"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.new_object(cls,
sig.as_str(),vec![jni::objects::JValueGen::from(val_1),jni::objects::JValueGen::from(val_2),jni::objects::JValueGen::from(val_3)]);
let res = 
jni.translate_error_no_gen(res)?;
crate::event::inventory::BrewingStandFuelEvent::from_raw(&jni,res
)}
	pub fn fuel(&self) 
-> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lcrate::inventory::ItemStack;");
let res = self.jni_ref().call_method(&self.jni_object(),"getFuel",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::inventory::ItemStack::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn fuel_power(&self) 
-> Result<i32, Box<dyn std::error::Error>>

{let sig = String::from("()Li32;");
let res = self.jni_ref().call_method(&self.jni_object(),"getFuelPower",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.i()?
)}
	pub fn set_fuel_power(&self,fuel_power: i32) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(I)L();");
let val_1 = jni::objects::JValueGen::Int(fuel_power);
let res = self.jni_ref().call_method(&self.jni_object(),"setFuelPower",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
	pub fn is_consuming(&self) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("()Lbool;");
let res = self.jni_ref().call_method(&self.jni_object(),"isConsuming",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
	pub fn set_consuming(&self,consuming: bool) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Z)L();");
let val_1 = jni::objects::JValueGen::Bool(consuming.into());
let res = self.jni_ref().call_method(&self.jni_object(),"setConsuming",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
	pub fn is_cancelled(&self) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("()Lbool;");
let res = self.jni_ref().call_method(&self.jni_object(),"isCancelled",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
	pub fn set_cancelled(&self,cancel: bool) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Z)L();");
let val_1 = jni::objects::JValueGen::Bool(cancel.into());
let res = self.jni_ref().call_method(&self.jni_object(),"setCancelled",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
	pub fn handlers(&self) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lcrate::event::HandlerList;");
let res = self.jni_ref().call_method(&self.jni_object(),"getHandlers",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::event::HandlerList::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn handler_list(jni: &blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lcrate::event::HandlerList;");
let cls = jni.find_class("org/bukkit/event/inventory/BrewingStandFuelEvent"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.call_static_method(cls,"getHandlerList",
sig.as_str(),vec![]);
let res = 
jni.translate_error(res)?;
let obj = res.l()?;
crate::event::HandlerList::from_raw(&jni,obj
)}
// SUPER CLASS: BlockEvent

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
impl<'mc> Into<crate::event::Cancellable<'mc>> for BrewingStandFuelEvent<'mc>{

fn into(self) -> crate::event::Cancellable<'mc> {

crate::event::Cancellable::from_raw(&self.jni_ref(), self.1).expect("Error converting BrewingStandFuelEvent into crate::event::Cancellable")

   }
}
impl<'mc> Into<crate::event::block::BlockEvent<'mc>> for BrewingStandFuelEvent<'mc>{

fn into(self) -> crate::event::block::BlockEvent<'mc> {

crate::event::block::BlockEvent::from_raw(&self.jni_ref(), self.1).expect("Error converting BrewingStandFuelEvent into crate::event::block::BlockEvent")

   }
}
#[repr(C)]
pub struct PrepareGrindstoneEvent<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

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
                    "Tried to instantiate PrepareGrindstoneEvent from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/event/inventory/PrepareGrindstoneEvent")?;
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
	pub fn new(jni: &blackboxmc_general::SharedJNIEnv<'mc>,inventory: impl Into<crate::inventory::InventoryView<'mc>>,result: impl Into<crate::inventory::ItemStack<'mc>>) 
-> Result<crate::event::inventory::PrepareGrindstoneEvent<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/inventory/InventoryView;Lorg/bukkit/inventory/ItemStack;)V");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(inventory.into().jni_object().clone())});
let val_2 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(result.into().jni_object().clone())});
let cls = jni.find_class("org/bukkit/event/inventory/PrepareGrindstoneEvent"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.new_object(cls,
sig.as_str(),vec![jni::objects::JValueGen::from(val_1),jni::objects::JValueGen::from(val_2)]);
let res = 
jni.translate_error_no_gen(res)?;
crate::event::inventory::PrepareGrindstoneEvent::from_raw(&jni,res
)}
	pub fn inventory(&self) 
-> Result<crate::inventory::GrindstoneInventory<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lcrate::inventory::GrindstoneInventory;");
let res = self.jni_ref().call_method(&self.jni_object(),"getInventory",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::inventory::GrindstoneInventory::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn handlers(&self) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lcrate::event::HandlerList;");
let res = self.jni_ref().call_method(&self.jni_object(),"getHandlers",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::event::HandlerList::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn handler_list(jni: &blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lcrate::event::HandlerList;");
let cls = jni.find_class("org/bukkit/event/inventory/PrepareGrindstoneEvent"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.call_static_method(cls,"getHandlerList",
sig.as_str(),vec![]);
let res = 
jni.translate_error(res)?;
let obj = res.l()?;
crate::event::HandlerList::from_raw(&jni,obj
)}
// SUPER CLASS: PrepareInventoryResultEvent

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
impl<'mc> Into<crate::event::inventory::PrepareInventoryResultEvent<'mc>> for PrepareGrindstoneEvent<'mc>{

fn into(self) -> crate::event::inventory::PrepareInventoryResultEvent<'mc> {

crate::event::inventory::PrepareInventoryResultEvent::from_raw(&self.jni_ref(), self.1).expect("Error converting PrepareGrindstoneEvent into crate::event::inventory::PrepareInventoryResultEvent")

   }
}
#[repr(C)]
pub struct FurnaceSmeltEvent<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

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
                return Err(eyre::eyre!(
                    "Tried to instantiate FurnaceSmeltEvent from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/event/inventory/FurnaceSmeltEvent")?;
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
	pub fn new(jni: &blackboxmc_general::SharedJNIEnv<'mc>,furnace: impl Into<crate::block::Block<'mc>>,source: impl Into<crate::inventory::ItemStack<'mc>>,result: impl Into<crate::inventory::ItemStack<'mc>>) 
-> Result<crate::event::inventory::FurnaceSmeltEvent<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/block/Block;Lorg/bukkit/inventory/ItemStack;Lorg/bukkit/inventory/ItemStack;)V");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(furnace.into().jni_object().clone())});
let val_2 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(source.into().jni_object().clone())});
let val_3 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(result.into().jni_object().clone())});
let cls = jni.find_class("org/bukkit/event/inventory/FurnaceSmeltEvent"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.new_object(cls,
sig.as_str(),vec![jni::objects::JValueGen::from(val_1),jni::objects::JValueGen::from(val_2),jni::objects::JValueGen::from(val_3)]);
let res = 
jni.translate_error_no_gen(res)?;
crate::event::inventory::FurnaceSmeltEvent::from_raw(&jni,res
)}
// SUPER CLASS: BlockCookEvent

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
impl<'mc> Into<crate::event::block::BlockCookEvent<'mc>> for FurnaceSmeltEvent<'mc>{

fn into(self) -> crate::event::block::BlockCookEvent<'mc> {

crate::event::block::BlockCookEvent::from_raw(&self.jni_ref(), self.1).expect("Error converting FurnaceSmeltEvent into crate::event::block::BlockCookEvent")

   }
}
#[repr(C)]
pub struct InventoryEvent<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

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
                return Err(eyre::eyre!(
                    "Tried to instantiate InventoryEvent from null object.")
                .into());
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
	pub fn new(jni: &blackboxmc_general::SharedJNIEnv<'mc>,transaction: impl Into<crate::inventory::InventoryView<'mc>>) 
-> Result<crate::event::inventory::InventoryEvent<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/inventory/InventoryView;)V");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(transaction.into().jni_object().clone())});
let cls = jni.find_class("org/bukkit/event/inventory/InventoryEvent"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.new_object(cls,
sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
jni.translate_error_no_gen(res)?;
crate::event::inventory::InventoryEvent::from_raw(&jni,res
)}
	pub fn inventory(&self) 
-> Result<crate::inventory::Inventory<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lcrate::inventory::Inventory;");
let res = self.jni_ref().call_method(&self.jni_object(),"getInventory",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::inventory::Inventory::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn viewers(&self) 
-> Result<Vec<jni::objects::JObject<'mc>>, Box<dyn std::error::Error>>

{let sig = String::from("()LVec;");
let res = self.jni_ref().call_method(&self.jni_object(),"getViewers",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
let mut new_vec = Vec::new();
let list = blackboxmc_java::util::JavaList::from_raw(&self.0, res.l()?)?;let iter = list.iterator()?;
while iter.has_next()? {            let obj = iter.next()?;
new_vec.push(obj);
};
Ok(
new_vec
)}
	pub fn view(&self) 
-> Result<crate::inventory::InventoryView<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lcrate::inventory::InventoryView;");
let res = self.jni_ref().call_method(&self.jni_object(),"getView",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::inventory::InventoryView::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn handlers(&self) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lcrate::event::HandlerList;");
let res = self.jni_ref().call_method(&self.jni_object(),"getHandlers",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::event::HandlerList::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn handler_list(jni: &blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lcrate::event::HandlerList;");
let cls = jni.find_class("org/bukkit/event/inventory/InventoryEvent"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.call_static_method(cls,"getHandlerList",
sig.as_str(),vec![]);
let res = 
jni.translate_error(res)?;
let obj = res.l()?;
crate::event::HandlerList::from_raw(&jni,obj
)}
// SUPER CLASS: Event

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
impl<'mc> Into<crate::event::Event<'mc>> for InventoryEvent<'mc>{

fn into(self) -> crate::event::Event<'mc> {

crate::event::Event::from_raw(&self.jni_ref(), self.1).expect("Error converting InventoryEvent into crate::event::Event")

   }
}
#[repr(C)]
pub struct InventoryMoveItemEvent<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

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
                    "Tried to instantiate InventoryMoveItemEvent from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/event/inventory/InventoryMoveItemEvent")?;
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
	pub fn new(jni: &blackboxmc_general::SharedJNIEnv<'mc>,source_inventory: impl Into<crate::inventory::Inventory<'mc>>,item_stack: impl Into<crate::inventory::ItemStack<'mc>>,destination_inventory: impl Into<crate::inventory::Inventory<'mc>>,did_source_initiate: bool) 
-> Result<crate::event::inventory::InventoryMoveItemEvent<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/inventory/Inventory;Lorg/bukkit/inventory/ItemStack;Lorg/bukkit/inventory/Inventory;Z)V");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(source_inventory.into().jni_object().clone())});
let val_2 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(item_stack.into().jni_object().clone())});
let val_3 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(destination_inventory.into().jni_object().clone())});
let val_4 = jni::objects::JValueGen::Bool(did_source_initiate.into());
let cls = jni.find_class("org/bukkit/event/inventory/InventoryMoveItemEvent"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.new_object(cls,
sig.as_str(),vec![jni::objects::JValueGen::from(val_1),jni::objects::JValueGen::from(val_2),jni::objects::JValueGen::from(val_3),jni::objects::JValueGen::from(val_4)]);
let res = 
jni.translate_error_no_gen(res)?;
crate::event::inventory::InventoryMoveItemEvent::from_raw(&jni,res
)}
	pub fn source(&self) 
-> Result<crate::inventory::Inventory<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lcrate::inventory::Inventory;");
let res = self.jni_ref().call_method(&self.jni_object(),"getSource",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::inventory::Inventory::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn item(&self) 
-> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lcrate::inventory::ItemStack;");
let res = self.jni_ref().call_method(&self.jni_object(),"getItem",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::inventory::ItemStack::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn set_item(&self,item_stack: impl Into<crate::inventory::ItemStack<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/inventory/ItemStack;)L();");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(item_stack.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"setItem",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
	pub fn destination(&self) 
-> Result<crate::inventory::Inventory<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lcrate::inventory::Inventory;");
let res = self.jni_ref().call_method(&self.jni_object(),"getDestination",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::inventory::Inventory::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn initiator(&self) 
-> Result<crate::inventory::Inventory<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lcrate::inventory::Inventory;");
let res = self.jni_ref().call_method(&self.jni_object(),"getInitiator",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::inventory::Inventory::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn is_cancelled(&self) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("()Lbool;");
let res = self.jni_ref().call_method(&self.jni_object(),"isCancelled",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
	pub fn set_cancelled(&self,cancel: bool) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Z)L();");
let val_1 = jni::objects::JValueGen::Bool(cancel.into());
let res = self.jni_ref().call_method(&self.jni_object(),"setCancelled",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
	pub fn handlers(&self) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lcrate::event::HandlerList;");
let res = self.jni_ref().call_method(&self.jni_object(),"getHandlers",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::event::HandlerList::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn handler_list(jni: &blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lcrate::event::HandlerList;");
let cls = jni.find_class("org/bukkit/event/inventory/InventoryMoveItemEvent"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.call_static_method(cls,"getHandlerList",
sig.as_str(),vec![]);
let res = 
jni.translate_error(res)?;
let obj = res.l()?;
crate::event::HandlerList::from_raw(&jni,obj
)}
// SUPER CLASS: Event

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
impl<'mc> Into<crate::event::Cancellable<'mc>> for InventoryMoveItemEvent<'mc>{

fn into(self) -> crate::event::Cancellable<'mc> {

crate::event::Cancellable::from_raw(&self.jni_ref(), self.1).expect("Error converting InventoryMoveItemEvent into crate::event::Cancellable")

   }
}
impl<'mc> Into<crate::event::Event<'mc>> for InventoryMoveItemEvent<'mc>{

fn into(self) -> crate::event::Event<'mc> {

crate::event::Event::from_raw(&self.jni_ref(), self.1).expect("Error converting InventoryMoveItemEvent into crate::event::Event")

   }
}
pub enum InventoryTypeSlotType<'mc> {
	Result {inner: InventoryTypeSlotTypeStruct<'mc>},
	Crafting {inner: InventoryTypeSlotTypeStruct<'mc>},
	Armor {inner: InventoryTypeSlotTypeStruct<'mc>},
	Container {inner: InventoryTypeSlotTypeStruct<'mc>},
	Quickbar {inner: InventoryTypeSlotTypeStruct<'mc>},
	Outside {inner: InventoryTypeSlotTypeStruct<'mc>},
	Fuel {inner: InventoryTypeSlotTypeStruct<'mc>},
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
                    
"RESULT" => Ok(InventoryTypeSlotType::Result { inner: InventoryTypeSlotTypeStruct::from_raw(env,obj)?}),
"CRAFTING" => Ok(InventoryTypeSlotType::Crafting { inner: InventoryTypeSlotTypeStruct::from_raw(env,obj)?}),
"ARMOR" => Ok(InventoryTypeSlotType::Armor { inner: InventoryTypeSlotTypeStruct::from_raw(env,obj)?}),
"CONTAINER" => Ok(InventoryTypeSlotType::Container { inner: InventoryTypeSlotTypeStruct::from_raw(env,obj)?}),
"QUICKBAR" => Ok(InventoryTypeSlotType::Quickbar { inner: InventoryTypeSlotTypeStruct::from_raw(env,obj)?}),
"OUTSIDE" => Ok(InventoryTypeSlotType::Outside { inner: InventoryTypeSlotTypeStruct::from_raw(env,obj)?}),
"FUEL" => Ok(InventoryTypeSlotType::Fuel { inner: InventoryTypeSlotTypeStruct::from_raw(env,obj)?}),

                    _ => Err(eyre::eyre!("String gaven for variant was invalid").into())
                }
            }
        }
        
#[repr(C)]
pub struct InventoryTypeSlotTypeStruct<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

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
Self::Container { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
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
                    "Tried to instantiate InventoryTypeSlotType from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/event/inventory/InventoryType/SlotType")?;
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
                    "RESULT" => Ok(InventoryTypeSlotType::Result { inner: InventoryTypeSlotTypeStruct::from_raw(env,obj)?}),"CRAFTING" => Ok(InventoryTypeSlotType::Crafting { inner: InventoryTypeSlotTypeStruct::from_raw(env,obj)?}),"ARMOR" => Ok(InventoryTypeSlotType::Armor { inner: InventoryTypeSlotTypeStruct::from_raw(env,obj)?}),"CONTAINER" => Ok(InventoryTypeSlotType::Container { inner: InventoryTypeSlotTypeStruct::from_raw(env,obj)?}),"QUICKBAR" => Ok(InventoryTypeSlotType::Quickbar { inner: InventoryTypeSlotTypeStruct::from_raw(env,obj)?}),"OUTSIDE" => Ok(InventoryTypeSlotType::Outside { inner: InventoryTypeSlotTypeStruct::from_raw(env,obj)?}),"FUEL" => Ok(InventoryTypeSlotType::Fuel { inner: InventoryTypeSlotTypeStruct::from_raw(env,obj)?}),_ => Err(eyre::eyre!("String gaven for variant was invalid").into())}
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
                    "Tried to instantiate InventoryTypeSlotTypeStruct from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/event/inventory/InventoryType/SlotType")?;
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
	pub fn values(jni: &blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::event::inventory::InventoryTypeSlotType<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lcrate::event::inventory::InventoryTypeSlotType;");
let cls = jni.find_class("org/bukkit/event/inventory/InventoryType/SlotType"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.call_static_method(cls,"values",
sig.as_str(),vec![]);
let res = 
jni.translate_error(res)?;
let obj = res.l()?;
crate::event::inventory::InventoryTypeSlotType::from_raw(&jni,obj
)}
// SUPER CLASS: Enum

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
#[repr(C)]
pub struct InventoryInteractEvent<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

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
                    "Tried to instantiate InventoryInteractEvent from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/event/inventory/InventoryInteractEvent")?;
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
	pub fn new(jni: &blackboxmc_general::SharedJNIEnv<'mc>,transaction: impl Into<crate::inventory::InventoryView<'mc>>) 
-> Result<crate::event::inventory::InventoryInteractEvent<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/inventory/InventoryView;)V");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(transaction.into().jni_object().clone())});
let cls = jni.find_class("org/bukkit/event/inventory/InventoryInteractEvent"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.new_object(cls,
sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
jni.translate_error_no_gen(res)?;
crate::event::inventory::InventoryInteractEvent::from_raw(&jni,res
)}
	pub fn who_clicked(&self) 
-> Result<crate::entity::HumanEntity<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lcrate::entity::HumanEntity;");
let res = self.jni_ref().call_method(&self.jni_object(),"getWhoClicked",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::entity::HumanEntity::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn set_result(&self,new_result: impl Into<crate::event::EventResult<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/event/Event/Result;)L();");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(new_result.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"setResult",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
	pub fn result(&self) 
-> Result<crate::event::EventResult<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lcrate::event::EventResult;");
let res = self.jni_ref().call_method(&self.jni_object(),"getResult",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::event::EventResult::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn is_cancelled(&self) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("()Lbool;");
let res = self.jni_ref().call_method(&self.jni_object(),"isCancelled",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
	pub fn set_cancelled(&self,to_cancel: bool) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Z)L();");
let val_1 = jni::objects::JValueGen::Bool(to_cancel.into());
let res = self.jni_ref().call_method(&self.jni_object(),"setCancelled",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
// SUPER CLASS: InventoryEvent

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
impl<'mc> Into<crate::event::Cancellable<'mc>> for InventoryInteractEvent<'mc>{

fn into(self) -> crate::event::Cancellable<'mc> {

crate::event::Cancellable::from_raw(&self.jni_ref(), self.1).expect("Error converting InventoryInteractEvent into crate::event::Cancellable")

   }
}
impl<'mc> Into<crate::event::inventory::InventoryEvent<'mc>> for InventoryInteractEvent<'mc>{

fn into(self) -> crate::event::inventory::InventoryEvent<'mc> {

crate::event::inventory::InventoryEvent::from_raw(&self.jni_ref(), self.1).expect("Error converting InventoryInteractEvent into crate::event::inventory::InventoryEvent")

   }
}
#[repr(C)]
pub struct InventoryDragEvent<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

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
                return Err(eyre::eyre!(
                    "Tried to instantiate InventoryDragEvent from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/event/inventory/InventoryDragEvent")?;
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
	pub fn new(jni: &blackboxmc_general::SharedJNIEnv<'mc>,what: impl Into<crate::inventory::InventoryView<'mc>>,new_cursor: impl Into<crate::inventory::ItemStack<'mc>>,old_cursor: impl Into<crate::inventory::ItemStack<'mc>>,right: bool,slots: impl Into<blackboxmc_java::util::JavaMap<'mc>>) 
-> Result<crate::event::inventory::InventoryDragEvent<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/inventory/InventoryView;Lorg/bukkit/inventory/ItemStack;Lorg/bukkit/inventory/ItemStack;ZLjava/util/Map;)V");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(what.into().jni_object().clone())});
let val_2 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(new_cursor.into().jni_object().clone())});
let val_3 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(old_cursor.into().jni_object().clone())});
let val_4 = jni::objects::JValueGen::Bool(right.into());
let val_5 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(slots.into().jni_object().clone())});
let cls = jni.find_class("org/bukkit/event/inventory/InventoryDragEvent"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.new_object(cls,
sig.as_str(),vec![jni::objects::JValueGen::from(val_1),jni::objects::JValueGen::from(val_2),jni::objects::JValueGen::from(val_3),jni::objects::JValueGen::from(val_4),jni::objects::JValueGen::from(val_5)]);
let res = 
jni.translate_error_no_gen(res)?;
crate::event::inventory::InventoryDragEvent::from_raw(&jni,res
)}
	pub fn new_items(&self) 
-> Result<blackboxmc_java::util::JavaMap<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lblackboxmc_java::util::Map;");
let res = self.jni_ref().call_method(&self.jni_object(),"getNewItems",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
blackboxmc_java::util::JavaMap::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn raw_slots(&self) 
-> Result<blackboxmc_java::util::JavaSet<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lblackboxmc_java::util::Set;");
let res = self.jni_ref().call_method(&self.jni_object(),"getRawSlots",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
blackboxmc_java::util::JavaSet::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn inventory_slots(&self) 
-> Result<blackboxmc_java::util::JavaSet<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lblackboxmc_java::util::Set;");
let res = self.jni_ref().call_method(&self.jni_object(),"getInventorySlots",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
blackboxmc_java::util::JavaSet::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn cursor(&self) 
-> Result<Option<crate::inventory::ItemStack<'mc>>, Box<dyn std::error::Error>>

{let sig = String::from("()Lcrate::inventory::ItemStack;");
let res = self.jni_ref().call_method(&self.jni_object(),"getCursor",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {return Ok(None);}
Ok(
Some(
crate::inventory::ItemStack::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)?
)
)}
	pub fn set_cursor(&self,new_cursor: impl Into<crate::inventory::ItemStack<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/inventory/ItemStack;)L();");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(new_cursor.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"setCursor",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
	pub fn old_cursor(&self) 
-> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lcrate::inventory::ItemStack;");
let res = self.jni_ref().call_method(&self.jni_object(),"getOldCursor",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::inventory::ItemStack::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn get_type(&self) 
-> Result<crate::event::inventory::DragType<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lcrate::event::inventory::DragType;");
let res = self.jni_ref().call_method(&self.jni_object(),"getType",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::event::inventory::DragType::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn handlers(&self) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lcrate::event::HandlerList;");
let res = self.jni_ref().call_method(&self.jni_object(),"getHandlers",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::event::HandlerList::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn handler_list(jni: &blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lcrate::event::HandlerList;");
let cls = jni.find_class("org/bukkit/event/inventory/InventoryDragEvent"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.call_static_method(cls,"getHandlerList",
sig.as_str(),vec![]);
let res = 
jni.translate_error(res)?;
let obj = res.l()?;
crate::event::HandlerList::from_raw(&jni,obj
)}
// SUPER CLASS: InventoryInteractEvent

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
impl<'mc> Into<crate::event::inventory::InventoryInteractEvent<'mc>> for InventoryDragEvent<'mc>{

fn into(self) -> crate::event::inventory::InventoryInteractEvent<'mc> {

crate::event::inventory::InventoryInteractEvent::from_raw(&self.jni_ref(), self.1).expect("Error converting InventoryDragEvent into crate::event::inventory::InventoryInteractEvent")

   }
}
pub enum InventoryAction<'mc> {
	Nothing {inner: InventoryActionStruct<'mc>},
	PickupAll {inner: InventoryActionStruct<'mc>},
	PickupSome {inner: InventoryActionStruct<'mc>},
	PickupHalf {inner: InventoryActionStruct<'mc>},
	PickupOne {inner: InventoryActionStruct<'mc>},
	PlaceAll {inner: InventoryActionStruct<'mc>},
	PlaceSome {inner: InventoryActionStruct<'mc>},
	PlaceOne {inner: InventoryActionStruct<'mc>},
	SwapWithCursor {inner: InventoryActionStruct<'mc>},
	DropAllCursor {inner: InventoryActionStruct<'mc>},
	DropOneCursor {inner: InventoryActionStruct<'mc>},
	DropAllSlot {inner: InventoryActionStruct<'mc>},
	DropOneSlot {inner: InventoryActionStruct<'mc>},
	MoveToOtherInventory {inner: InventoryActionStruct<'mc>},
	HotbarMoveAndReadd {inner: InventoryActionStruct<'mc>},
	HotbarSwap {inner: InventoryActionStruct<'mc>},
	CloneStack {inner: InventoryActionStruct<'mc>},
	CollectToCursor {inner: InventoryActionStruct<'mc>},
	Unknown {inner: InventoryActionStruct<'mc>},
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
                    
"NOTHING" => Ok(InventoryAction::Nothing { inner: InventoryActionStruct::from_raw(env,obj)?}),
"PICKUP_ALL" => Ok(InventoryAction::PickupAll { inner: InventoryActionStruct::from_raw(env,obj)?}),
"PICKUP_SOME" => Ok(InventoryAction::PickupSome { inner: InventoryActionStruct::from_raw(env,obj)?}),
"PICKUP_HALF" => Ok(InventoryAction::PickupHalf { inner: InventoryActionStruct::from_raw(env,obj)?}),
"PICKUP_ONE" => Ok(InventoryAction::PickupOne { inner: InventoryActionStruct::from_raw(env,obj)?}),
"PLACE_ALL" => Ok(InventoryAction::PlaceAll { inner: InventoryActionStruct::from_raw(env,obj)?}),
"PLACE_SOME" => Ok(InventoryAction::PlaceSome { inner: InventoryActionStruct::from_raw(env,obj)?}),
"PLACE_ONE" => Ok(InventoryAction::PlaceOne { inner: InventoryActionStruct::from_raw(env,obj)?}),
"SWAP_WITH_CURSOR" => Ok(InventoryAction::SwapWithCursor { inner: InventoryActionStruct::from_raw(env,obj)?}),
"DROP_ALL_CURSOR" => Ok(InventoryAction::DropAllCursor { inner: InventoryActionStruct::from_raw(env,obj)?}),
"DROP_ONE_CURSOR" => Ok(InventoryAction::DropOneCursor { inner: InventoryActionStruct::from_raw(env,obj)?}),
"DROP_ALL_SLOT" => Ok(InventoryAction::DropAllSlot { inner: InventoryActionStruct::from_raw(env,obj)?}),
"DROP_ONE_SLOT" => Ok(InventoryAction::DropOneSlot { inner: InventoryActionStruct::from_raw(env,obj)?}),
"MOVE_TO_OTHER_INVENTORY" => Ok(InventoryAction::MoveToOtherInventory { inner: InventoryActionStruct::from_raw(env,obj)?}),
"HOTBAR_MOVE_AND_READD" => Ok(InventoryAction::HotbarMoveAndReadd { inner: InventoryActionStruct::from_raw(env,obj)?}),
"HOTBAR_SWAP" => Ok(InventoryAction::HotbarSwap { inner: InventoryActionStruct::from_raw(env,obj)?}),
"CLONE_STACK" => Ok(InventoryAction::CloneStack { inner: InventoryActionStruct::from_raw(env,obj)?}),
"COLLECT_TO_CURSOR" => Ok(InventoryAction::CollectToCursor { inner: InventoryActionStruct::from_raw(env,obj)?}),
"UNKNOWN" => Ok(InventoryAction::Unknown { inner: InventoryActionStruct::from_raw(env,obj)?}),

                    _ => Err(eyre::eyre!("String gaven for variant was invalid").into())
                }
            }
        }
        
#[repr(C)]
pub struct InventoryActionStruct<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

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
Self::PickupAll { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::PickupSome { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::PickupHalf { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::PickupOne { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::PlaceAll { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::PlaceSome { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::PlaceOne { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::SwapWithCursor { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::DropAllCursor { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::DropOneCursor { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::DropAllSlot { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::DropOneSlot { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::MoveToOtherInventory { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::HotbarMoveAndReadd { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::HotbarSwap { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::CloneStack { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::CollectToCursor { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
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
                return Err(eyre::eyre!(
                    "Tried to instantiate InventoryAction from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/event/inventory/InventoryAction")?;
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
                    "NOTHING" => Ok(InventoryAction::Nothing { inner: InventoryActionStruct::from_raw(env,obj)?}),"PICKUP_ALL" => Ok(InventoryAction::PickupAll { inner: InventoryActionStruct::from_raw(env,obj)?}),"PICKUP_SOME" => Ok(InventoryAction::PickupSome { inner: InventoryActionStruct::from_raw(env,obj)?}),"PICKUP_HALF" => Ok(InventoryAction::PickupHalf { inner: InventoryActionStruct::from_raw(env,obj)?}),"PICKUP_ONE" => Ok(InventoryAction::PickupOne { inner: InventoryActionStruct::from_raw(env,obj)?}),"PLACE_ALL" => Ok(InventoryAction::PlaceAll { inner: InventoryActionStruct::from_raw(env,obj)?}),"PLACE_SOME" => Ok(InventoryAction::PlaceSome { inner: InventoryActionStruct::from_raw(env,obj)?}),"PLACE_ONE" => Ok(InventoryAction::PlaceOne { inner: InventoryActionStruct::from_raw(env,obj)?}),"SWAP_WITH_CURSOR" => Ok(InventoryAction::SwapWithCursor { inner: InventoryActionStruct::from_raw(env,obj)?}),"DROP_ALL_CURSOR" => Ok(InventoryAction::DropAllCursor { inner: InventoryActionStruct::from_raw(env,obj)?}),"DROP_ONE_CURSOR" => Ok(InventoryAction::DropOneCursor { inner: InventoryActionStruct::from_raw(env,obj)?}),"DROP_ALL_SLOT" => Ok(InventoryAction::DropAllSlot { inner: InventoryActionStruct::from_raw(env,obj)?}),"DROP_ONE_SLOT" => Ok(InventoryAction::DropOneSlot { inner: InventoryActionStruct::from_raw(env,obj)?}),"MOVE_TO_OTHER_INVENTORY" => Ok(InventoryAction::MoveToOtherInventory { inner: InventoryActionStruct::from_raw(env,obj)?}),"HOTBAR_MOVE_AND_READD" => Ok(InventoryAction::HotbarMoveAndReadd { inner: InventoryActionStruct::from_raw(env,obj)?}),"HOTBAR_SWAP" => Ok(InventoryAction::HotbarSwap { inner: InventoryActionStruct::from_raw(env,obj)?}),"CLONE_STACK" => Ok(InventoryAction::CloneStack { inner: InventoryActionStruct::from_raw(env,obj)?}),"COLLECT_TO_CURSOR" => Ok(InventoryAction::CollectToCursor { inner: InventoryActionStruct::from_raw(env,obj)?}),"UNKNOWN" => Ok(InventoryAction::Unknown { inner: InventoryActionStruct::from_raw(env,obj)?}),_ => Err(eyre::eyre!("String gaven for variant was invalid").into())}
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
                    "Tried to instantiate InventoryActionStruct from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/event/inventory/InventoryAction")?;
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
	pub fn values(jni: &blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::event::inventory::InventoryAction<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lcrate::event::inventory::InventoryAction;");
let cls = jni.find_class("org/bukkit/event/inventory/InventoryAction"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.call_static_method(cls,"values",
sig.as_str(),vec![]);
let res = 
jni.translate_error(res)?;
let obj = res.l()?;
crate::event::inventory::InventoryAction::from_raw(&jni,obj
)}
// SUPER CLASS: Enum

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
#[repr(C)]
pub struct PrepareInventoryResultEvent<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

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
                    "Tried to instantiate PrepareInventoryResultEvent from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/event/inventory/PrepareInventoryResultEvent")?;
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
	pub fn new(jni: &blackboxmc_general::SharedJNIEnv<'mc>,inventory: impl Into<crate::inventory::InventoryView<'mc>>,result: impl Into<crate::inventory::ItemStack<'mc>>) 
-> Result<crate::event::inventory::PrepareInventoryResultEvent<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/inventory/InventoryView;Lorg/bukkit/inventory/ItemStack;)V");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(inventory.into().jni_object().clone())});
let val_2 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(result.into().jni_object().clone())});
let cls = jni.find_class("org/bukkit/event/inventory/PrepareInventoryResultEvent"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.new_object(cls,
sig.as_str(),vec![jni::objects::JValueGen::from(val_1),jni::objects::JValueGen::from(val_2)]);
let res = 
jni.translate_error_no_gen(res)?;
crate::event::inventory::PrepareInventoryResultEvent::from_raw(&jni,res
)}
	pub fn result(&self) 
-> Result<Option<crate::inventory::ItemStack<'mc>>, Box<dyn std::error::Error>>

{let sig = String::from("()Lcrate::inventory::ItemStack;");
let res = self.jni_ref().call_method(&self.jni_object(),"getResult",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {return Ok(None);}
Ok(
Some(
crate::inventory::ItemStack::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)?
)
)}
	pub fn set_result(&self,result: impl Into<crate::inventory::ItemStack<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/inventory/ItemStack;)L();");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(result.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"setResult",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
	pub fn handlers(&self) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lcrate::event::HandlerList;");
let res = self.jni_ref().call_method(&self.jni_object(),"getHandlers",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::event::HandlerList::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn handler_list(jni: &blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lcrate::event::HandlerList;");
let cls = jni.find_class("org/bukkit/event/inventory/PrepareInventoryResultEvent"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.call_static_method(cls,"getHandlerList",
sig.as_str(),vec![]);
let res = 
jni.translate_error(res)?;
let obj = res.l()?;
crate::event::HandlerList::from_raw(&jni,obj
)}
// SUPER CLASS: InventoryEvent

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
impl<'mc> Into<crate::event::inventory::InventoryEvent<'mc>> for PrepareInventoryResultEvent<'mc>{

fn into(self) -> crate::event::inventory::InventoryEvent<'mc> {

crate::event::inventory::InventoryEvent::from_raw(&self.jni_ref(), self.1).expect("Error converting PrepareInventoryResultEvent into crate::event::inventory::InventoryEvent")

   }
}
#[repr(C)]
pub struct FurnaceStartSmeltEvent<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

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
                    "Tried to instantiate FurnaceStartSmeltEvent from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/event/inventory/FurnaceStartSmeltEvent")?;
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
	pub fn new(jni: &blackboxmc_general::SharedJNIEnv<'mc>,furnace: impl Into<crate::block::Block<'mc>>,source: impl Into<crate::inventory::ItemStack<'mc>>,recipe: impl Into<crate::inventory::CookingRecipe<'mc>>) 
-> Result<crate::event::inventory::FurnaceStartSmeltEvent<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/block/Block;Lorg/bukkit/inventory/ItemStack;Lorg/bukkit/inventory/CookingRecipe;)V");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(furnace.into().jni_object().clone())});
let val_2 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(source.into().jni_object().clone())});
let val_3 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(recipe.into().jni_object().clone())});
let cls = jni.find_class("org/bukkit/event/inventory/FurnaceStartSmeltEvent"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.new_object(cls,
sig.as_str(),vec![jni::objects::JValueGen::from(val_1),jni::objects::JValueGen::from(val_2),jni::objects::JValueGen::from(val_3)]);
let res = 
jni.translate_error_no_gen(res)?;
crate::event::inventory::FurnaceStartSmeltEvent::from_raw(&jni,res
)}
	pub fn recipe(&self) 
-> Result<crate::inventory::CookingRecipe<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lcrate::inventory::CookingRecipe;");
let res = self.jni_ref().call_method(&self.jni_object(),"getRecipe",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::inventory::CookingRecipe::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn total_cook_time(&self) 
-> Result<i32, Box<dyn std::error::Error>>

{let sig = String::from("()Li32;");
let res = self.jni_ref().call_method(&self.jni_object(),"getTotalCookTime",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.i()?
)}
	pub fn set_total_cook_time(&self,cook_time: i32) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(I)L();");
let val_1 = jni::objects::JValueGen::Int(cook_time);
let res = self.jni_ref().call_method(&self.jni_object(),"setTotalCookTime",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
	pub fn handlers(&self) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lcrate::event::HandlerList;");
let res = self.jni_ref().call_method(&self.jni_object(),"getHandlers",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::event::HandlerList::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn handler_list(jni: &blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lcrate::event::HandlerList;");
let cls = jni.find_class("org/bukkit/event/inventory/FurnaceStartSmeltEvent"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.call_static_method(cls,"getHandlerList",
sig.as_str(),vec![]);
let res = 
jni.translate_error(res)?;
let obj = res.l()?;
crate::event::HandlerList::from_raw(&jni,obj
)}
// SUPER CLASS: InventoryBlockStartEvent

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
impl<'mc> Into<crate::event::block::InventoryBlockStartEvent<'mc>> for FurnaceStartSmeltEvent<'mc>{

fn into(self) -> crate::event::block::InventoryBlockStartEvent<'mc> {

crate::event::block::InventoryBlockStartEvent::from_raw(&self.jni_ref(), self.1).expect("Error converting FurnaceStartSmeltEvent into crate::event::block::InventoryBlockStartEvent")

   }
}
#[repr(C)]
pub struct FurnaceExtractEvent<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

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
                return Err(eyre::eyre!(
                    "Tried to instantiate FurnaceExtractEvent from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/event/inventory/FurnaceExtractEvent")?;
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
	pub fn new(jni: &blackboxmc_general::SharedJNIEnv<'mc>,player: impl Into<crate::entity::Player<'mc>>,block: impl Into<crate::block::Block<'mc>>,item_type: impl Into<crate::Material<'mc>>,item_amount: i32,exp: i32) 
-> Result<crate::event::inventory::FurnaceExtractEvent<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/entity/Player;Lorg/bukkit/block/Block;Lorg/bukkit/Material;II)V");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(player.into().jni_object().clone())});
let val_2 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(block.into().jni_object().clone())});
let val_3 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(item_type.into().jni_object().clone())});
let val_4 = jni::objects::JValueGen::Int(item_amount);
let val_5 = jni::objects::JValueGen::Int(exp);
let cls = jni.find_class("org/bukkit/event/inventory/FurnaceExtractEvent"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.new_object(cls,
sig.as_str(),vec![jni::objects::JValueGen::from(val_1),jni::objects::JValueGen::from(val_2),jni::objects::JValueGen::from(val_3),jni::objects::JValueGen::from(val_4),jni::objects::JValueGen::from(val_5)]);
let res = 
jni.translate_error_no_gen(res)?;
crate::event::inventory::FurnaceExtractEvent::from_raw(&jni,res
)}
	pub fn player(&self) 
-> Result<crate::entity::Player<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lcrate::entity::Player;");
let res = self.jni_ref().call_method(&self.jni_object(),"getPlayer",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::entity::Player::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn item_type(&self) 
-> Result<crate::Material<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lcrate::Material;");
let res = self.jni_ref().call_method(&self.jni_object(),"getItemType",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::Material::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn item_amount(&self) 
-> Result<i32, Box<dyn std::error::Error>>

{let sig = String::from("()Li32;");
let res = self.jni_ref().call_method(&self.jni_object(),"getItemAmount",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.i()?
)}
// SUPER CLASS: BlockExpEvent

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
impl<'mc> Into<crate::event::block::BlockExpEvent<'mc>> for FurnaceExtractEvent<'mc>{

fn into(self) -> crate::event::block::BlockExpEvent<'mc> {

crate::event::block::BlockExpEvent::from_raw(&self.jni_ref(), self.1).expect("Error converting FurnaceExtractEvent into crate::event::block::BlockExpEvent")

   }
}
#[repr(C)]
pub struct InventoryPickupItemEvent<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

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
                    "Tried to instantiate InventoryPickupItemEvent from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/event/inventory/InventoryPickupItemEvent")?;
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
	pub fn new(jni: &blackboxmc_general::SharedJNIEnv<'mc>,inventory: impl Into<crate::inventory::Inventory<'mc>>,item: impl Into<crate::entity::Item<'mc>>) 
-> Result<crate::event::inventory::InventoryPickupItemEvent<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/inventory/Inventory;Lorg/bukkit/entity/Item;)V");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(inventory.into().jni_object().clone())});
let val_2 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(item.into().jni_object().clone())});
let cls = jni.find_class("org/bukkit/event/inventory/InventoryPickupItemEvent"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.new_object(cls,
sig.as_str(),vec![jni::objects::JValueGen::from(val_1),jni::objects::JValueGen::from(val_2)]);
let res = 
jni.translate_error_no_gen(res)?;
crate::event::inventory::InventoryPickupItemEvent::from_raw(&jni,res
)}
	pub fn inventory(&self) 
-> Result<crate::inventory::Inventory<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lcrate::inventory::Inventory;");
let res = self.jni_ref().call_method(&self.jni_object(),"getInventory",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::inventory::Inventory::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn item(&self) 
-> Result<crate::entity::Item<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lcrate::entity::Item;");
let res = self.jni_ref().call_method(&self.jni_object(),"getItem",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::entity::Item::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn is_cancelled(&self) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("()Lbool;");
let res = self.jni_ref().call_method(&self.jni_object(),"isCancelled",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
	pub fn set_cancelled(&self,cancel: bool) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Z)L();");
let val_1 = jni::objects::JValueGen::Bool(cancel.into());
let res = self.jni_ref().call_method(&self.jni_object(),"setCancelled",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
	pub fn handlers(&self) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lcrate::event::HandlerList;");
let res = self.jni_ref().call_method(&self.jni_object(),"getHandlers",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::event::HandlerList::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn handler_list(jni: &blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lcrate::event::HandlerList;");
let cls = jni.find_class("org/bukkit/event/inventory/InventoryPickupItemEvent"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.call_static_method(cls,"getHandlerList",
sig.as_str(),vec![]);
let res = 
jni.translate_error(res)?;
let obj = res.l()?;
crate::event::HandlerList::from_raw(&jni,obj
)}
// SUPER CLASS: Event

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
impl<'mc> Into<crate::event::Cancellable<'mc>> for InventoryPickupItemEvent<'mc>{

fn into(self) -> crate::event::Cancellable<'mc> {

crate::event::Cancellable::from_raw(&self.jni_ref(), self.1).expect("Error converting InventoryPickupItemEvent into crate::event::Cancellable")

   }
}
impl<'mc> Into<crate::event::Event<'mc>> for InventoryPickupItemEvent<'mc>{

fn into(self) -> crate::event::Event<'mc> {

crate::event::Event::from_raw(&self.jni_ref(), self.1).expect("Error converting InventoryPickupItemEvent into crate::event::Event")

   }
}
#[repr(C)]
pub struct BrewEvent<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

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
                return Err(eyre::eyre!(
                    "Tried to instantiate BrewEvent from null object.")
                .into());
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
	pub fn new(jni: &blackboxmc_general::SharedJNIEnv<'mc>,brewer: impl Into<crate::block::Block<'mc>>,contents: impl Into<crate::inventory::BrewerInventory<'mc>>,results: Vec<jni::objects::JObject<'mc>>,fuel_level: i32) 
-> Result<crate::event::inventory::BrewEvent<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/block/Block;Lorg/bukkit/inventory/BrewerInventory;Ljava/util/List;I)V");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(brewer.into().jni_object().clone())});
let val_2 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(contents.into().jni_object().clone())});
let raw_val_3 = jni.new_object("java/util/ArrayList", "()V", vec![])?;
for v in results{
let map_val_0 = jni::objects::JValueGen::Object(v);
jni.call_method(&raw_val_3,"add","(Ljava/lang/Object;)Z",vec![jni::objects::JValueGen::from(map_val_0)])?;
};
let val_3 = jni::objects::JValueGen::Object(raw_val_3);
let val_4 = jni::objects::JValueGen::Int(fuel_level);
let cls = jni.find_class("org/bukkit/event/inventory/BrewEvent"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.new_object(cls,
sig.as_str(),vec![jni::objects::JValueGen::from(val_1),jni::objects::JValueGen::from(val_2),jni::objects::JValueGen::from(val_3),jni::objects::JValueGen::from(val_4)]);
let res = 
jni.translate_error_no_gen(res)?;
crate::event::inventory::BrewEvent::from_raw(&jni,res
)}
	pub fn contents(&self) 
-> Result<crate::inventory::BrewerInventory<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lcrate::inventory::BrewerInventory;");
let res = self.jni_ref().call_method(&self.jni_object(),"getContents",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::inventory::BrewerInventory::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn fuel_level(&self) 
-> Result<i32, Box<dyn std::error::Error>>

{let sig = String::from("()Li32;");
let res = self.jni_ref().call_method(&self.jni_object(),"getFuelLevel",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.i()?
)}
	pub fn results(&self) 
-> Result<Vec<jni::objects::JObject<'mc>>, Box<dyn std::error::Error>>

{let sig = String::from("()LVec;");
let res = self.jni_ref().call_method(&self.jni_object(),"getResults",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
let mut new_vec = Vec::new();
let list = blackboxmc_java::util::JavaList::from_raw(&self.0, res.l()?)?;let iter = list.iterator()?;
while iter.has_next()? {            let obj = iter.next()?;
new_vec.push(obj);
};
Ok(
new_vec
)}
	pub fn is_cancelled(&self) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("()Lbool;");
let res = self.jni_ref().call_method(&self.jni_object(),"isCancelled",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
	pub fn set_cancelled(&self,cancel: bool) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Z)L();");
let val_1 = jni::objects::JValueGen::Bool(cancel.into());
let res = self.jni_ref().call_method(&self.jni_object(),"setCancelled",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
	pub fn handlers(&self) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lcrate::event::HandlerList;");
let res = self.jni_ref().call_method(&self.jni_object(),"getHandlers",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::event::HandlerList::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn handler_list(jni: &blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lcrate::event::HandlerList;");
let cls = jni.find_class("org/bukkit/event/inventory/BrewEvent"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.call_static_method(cls,"getHandlerList",
sig.as_str(),vec![]);
let res = 
jni.translate_error(res)?;
let obj = res.l()?;
crate::event::HandlerList::from_raw(&jni,obj
)}
// SUPER CLASS: BlockEvent

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
impl<'mc> Into<crate::event::Cancellable<'mc>> for BrewEvent<'mc>{

fn into(self) -> crate::event::Cancellable<'mc> {

crate::event::Cancellable::from_raw(&self.jni_ref(), self.1).expect("Error converting BrewEvent into crate::event::Cancellable")

   }
}
impl<'mc> Into<crate::event::block::BlockEvent<'mc>> for BrewEvent<'mc>{

fn into(self) -> crate::event::block::BlockEvent<'mc> {

crate::event::block::BlockEvent::from_raw(&self.jni_ref(), self.1).expect("Error converting BrewEvent into crate::event::block::BlockEvent")

   }
}
#[repr(C)]
pub struct SmithItemEvent<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

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
                return Err(eyre::eyre!(
                    "Tried to instantiate SmithItemEvent from null object.")
                .into());
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
	pub fn new_with_view(jni: &blackboxmc_general::SharedJNIEnv<'mc>,view: impl Into<crate::inventory::InventoryView<'mc>>,val_type: impl Into<crate::event::inventory::InventoryTypeSlotType<'mc>>,slot: i32,click: impl Into<crate::event::inventory::ClickType<'mc>>,action: impl Into<crate::event::inventory::InventoryAction<'mc>>,key: std::option::Option<i32>) 
-> Result<crate::event::inventory::SmithItemEvent<'mc>, Box<dyn std::error::Error>>

{let mut args = Vec::new();
let mut sig = String::from("(");
sig += "Lorg/bukkit/inventory/InventoryView;";
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(view.into().jni_object().clone())});
args.push(val_1);
sig += "Lorg/bukkit/event/inventory/InventoryType/SlotType;";
let val_2 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(val_type.into().jni_object().clone())});
args.push(val_2);
sig += "I";
let val_3 = jni::objects::JValueGen::Int(slot);
args.push(val_3);
sig += "Lorg/bukkit/event/inventory/ClickType;";
let val_4 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(click.into().jni_object().clone())});
args.push(val_4);
sig += "Lorg/bukkit/event/inventory/InventoryAction;";
let val_5 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(action.into().jni_object().clone())});
args.push(val_5);
if let Some(a) = key {
sig += "I";
let val_6 = jni::objects::JValueGen::Int(a);
args.push(val_6);
}
sig += ")V";
let cls = jni.find_class("org/bukkit/event/inventory/SmithItemEvent"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.new_object(cls,
sig.as_str(),args);
let res = 
jni.translate_error_no_gen(res)?;
crate::event::inventory::SmithItemEvent::from_raw(&jni,res
)}
	pub fn inventory(&self) 
-> Result<crate::inventory::SmithingInventory<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lcrate::inventory::SmithingInventory;");
let res = self.jni_ref().call_method(&self.jni_object(),"getInventory",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::inventory::SmithingInventory::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
// SUPER CLASS: InventoryClickEvent

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
impl<'mc> Into<crate::event::inventory::InventoryClickEvent<'mc>> for SmithItemEvent<'mc>{

fn into(self) -> crate::event::inventory::InventoryClickEvent<'mc> {

crate::event::inventory::InventoryClickEvent::from_raw(&self.jni_ref(), self.1).expect("Error converting SmithItemEvent into crate::event::inventory::InventoryClickEvent")

   }
}
pub enum InventoryType<'mc> {
	Chest {inner: InventoryTypeStruct<'mc>},
	Dispenser {inner: InventoryTypeStruct<'mc>},
	Dropper {inner: InventoryTypeStruct<'mc>},
	Furnace {inner: InventoryTypeStruct<'mc>},
	Workbench {inner: InventoryTypeStruct<'mc>},
	Crafting {inner: InventoryTypeStruct<'mc>},
	Enchanting {inner: InventoryTypeStruct<'mc>},
	Brewing {inner: InventoryTypeStruct<'mc>},
	Player {inner: InventoryTypeStruct<'mc>},
	Creative {inner: InventoryTypeStruct<'mc>},
	Merchant {inner: InventoryTypeStruct<'mc>},
	EnderChest {inner: InventoryTypeStruct<'mc>},
	Anvil {inner: InventoryTypeStruct<'mc>},
	Smithing {inner: InventoryTypeStruct<'mc>},
	Beacon {inner: InventoryTypeStruct<'mc>},
	Hopper {inner: InventoryTypeStruct<'mc>},
	ShulkerBox {inner: InventoryTypeStruct<'mc>},
	Barrel {inner: InventoryTypeStruct<'mc>},
	BlastFurnace {inner: InventoryTypeStruct<'mc>},
	Lectern {inner: InventoryTypeStruct<'mc>},
	Smoker {inner: InventoryTypeStruct<'mc>},
	Loom {inner: InventoryTypeStruct<'mc>},
	Cartography {inner: InventoryTypeStruct<'mc>},
	Grindstone {inner: InventoryTypeStruct<'mc>},
	Stonecutter {inner: InventoryTypeStruct<'mc>},
	Composter {inner: InventoryTypeStruct<'mc>},
	ChiseledBookshelf {inner: InventoryTypeStruct<'mc>},
	Jukebox {inner: InventoryTypeStruct<'mc>},
	Crafter {inner: InventoryTypeStruct<'mc>},
	SmithingNew {inner: InventoryTypeStruct<'mc>},
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
                    
"CHEST" => Ok(InventoryType::Chest { inner: InventoryTypeStruct::from_raw(env,obj)?}),
"DISPENSER" => Ok(InventoryType::Dispenser { inner: InventoryTypeStruct::from_raw(env,obj)?}),
"DROPPER" => Ok(InventoryType::Dropper { inner: InventoryTypeStruct::from_raw(env,obj)?}),
"FURNACE" => Ok(InventoryType::Furnace { inner: InventoryTypeStruct::from_raw(env,obj)?}),
"WORKBENCH" => Ok(InventoryType::Workbench { inner: InventoryTypeStruct::from_raw(env,obj)?}),
"CRAFTING" => Ok(InventoryType::Crafting { inner: InventoryTypeStruct::from_raw(env,obj)?}),
"ENCHANTING" => Ok(InventoryType::Enchanting { inner: InventoryTypeStruct::from_raw(env,obj)?}),
"BREWING" => Ok(InventoryType::Brewing { inner: InventoryTypeStruct::from_raw(env,obj)?}),
"PLAYER" => Ok(InventoryType::Player { inner: InventoryTypeStruct::from_raw(env,obj)?}),
"CREATIVE" => Ok(InventoryType::Creative { inner: InventoryTypeStruct::from_raw(env,obj)?}),
"MERCHANT" => Ok(InventoryType::Merchant { inner: InventoryTypeStruct::from_raw(env,obj)?}),
"ENDER_CHEST" => Ok(InventoryType::EnderChest { inner: InventoryTypeStruct::from_raw(env,obj)?}),
"ANVIL" => Ok(InventoryType::Anvil { inner: InventoryTypeStruct::from_raw(env,obj)?}),
"SMITHING" => Ok(InventoryType::Smithing { inner: InventoryTypeStruct::from_raw(env,obj)?}),
"BEACON" => Ok(InventoryType::Beacon { inner: InventoryTypeStruct::from_raw(env,obj)?}),
"HOPPER" => Ok(InventoryType::Hopper { inner: InventoryTypeStruct::from_raw(env,obj)?}),
"SHULKER_BOX" => Ok(InventoryType::ShulkerBox { inner: InventoryTypeStruct::from_raw(env,obj)?}),
"BARREL" => Ok(InventoryType::Barrel { inner: InventoryTypeStruct::from_raw(env,obj)?}),
"BLAST_FURNACE" => Ok(InventoryType::BlastFurnace { inner: InventoryTypeStruct::from_raw(env,obj)?}),
"LECTERN" => Ok(InventoryType::Lectern { inner: InventoryTypeStruct::from_raw(env,obj)?}),
"SMOKER" => Ok(InventoryType::Smoker { inner: InventoryTypeStruct::from_raw(env,obj)?}),
"LOOM" => Ok(InventoryType::Loom { inner: InventoryTypeStruct::from_raw(env,obj)?}),
"CARTOGRAPHY" => Ok(InventoryType::Cartography { inner: InventoryTypeStruct::from_raw(env,obj)?}),
"GRINDSTONE" => Ok(InventoryType::Grindstone { inner: InventoryTypeStruct::from_raw(env,obj)?}),
"STONECUTTER" => Ok(InventoryType::Stonecutter { inner: InventoryTypeStruct::from_raw(env,obj)?}),
"COMPOSTER" => Ok(InventoryType::Composter { inner: InventoryTypeStruct::from_raw(env,obj)?}),
"CHISELED_BOOKSHELF" => Ok(InventoryType::ChiseledBookshelf { inner: InventoryTypeStruct::from_raw(env,obj)?}),
"JUKEBOX" => Ok(InventoryType::Jukebox { inner: InventoryTypeStruct::from_raw(env,obj)?}),
"CRAFTER" => Ok(InventoryType::Crafter { inner: InventoryTypeStruct::from_raw(env,obj)?}),
"SMITHING_NEW" => Ok(InventoryType::SmithingNew { inner: InventoryTypeStruct::from_raw(env,obj)?}),

                    _ => Err(eyre::eyre!("String gaven for variant was invalid").into())
                }
            }
        }
        
#[repr(C)]
pub struct InventoryTypeStruct<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

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
Self::Dispenser { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::Dropper { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::Furnace { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::Workbench { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::Crafting { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::Enchanting { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::Brewing { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::Player { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::Creative { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::Merchant { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::EnderChest { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::Anvil { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::Smithing { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::Beacon { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::Hopper { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::ShulkerBox { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::Barrel { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::BlastFurnace { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::Lectern { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::Smoker { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::Loom { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::Cartography { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::Grindstone { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::Stonecutter { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::Composter { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::ChiseledBookshelf { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::Jukebox { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::Crafter { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
Self::SmithingNew { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
}
}
}
impl<'mc> JNIInstantiatable<'mc> for InventoryType<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate InventoryType from null object.")
                .into());
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
                    "CHEST" => Ok(InventoryType::Chest { inner: InventoryTypeStruct::from_raw(env,obj)?}),"DISPENSER" => Ok(InventoryType::Dispenser { inner: InventoryTypeStruct::from_raw(env,obj)?}),"DROPPER" => Ok(InventoryType::Dropper { inner: InventoryTypeStruct::from_raw(env,obj)?}),"FURNACE" => Ok(InventoryType::Furnace { inner: InventoryTypeStruct::from_raw(env,obj)?}),"WORKBENCH" => Ok(InventoryType::Workbench { inner: InventoryTypeStruct::from_raw(env,obj)?}),"CRAFTING" => Ok(InventoryType::Crafting { inner: InventoryTypeStruct::from_raw(env,obj)?}),"ENCHANTING" => Ok(InventoryType::Enchanting { inner: InventoryTypeStruct::from_raw(env,obj)?}),"BREWING" => Ok(InventoryType::Brewing { inner: InventoryTypeStruct::from_raw(env,obj)?}),"PLAYER" => Ok(InventoryType::Player { inner: InventoryTypeStruct::from_raw(env,obj)?}),"CREATIVE" => Ok(InventoryType::Creative { inner: InventoryTypeStruct::from_raw(env,obj)?}),"MERCHANT" => Ok(InventoryType::Merchant { inner: InventoryTypeStruct::from_raw(env,obj)?}),"ENDER_CHEST" => Ok(InventoryType::EnderChest { inner: InventoryTypeStruct::from_raw(env,obj)?}),"ANVIL" => Ok(InventoryType::Anvil { inner: InventoryTypeStruct::from_raw(env,obj)?}),"SMITHING" => Ok(InventoryType::Smithing { inner: InventoryTypeStruct::from_raw(env,obj)?}),"BEACON" => Ok(InventoryType::Beacon { inner: InventoryTypeStruct::from_raw(env,obj)?}),"HOPPER" => Ok(InventoryType::Hopper { inner: InventoryTypeStruct::from_raw(env,obj)?}),"SHULKER_BOX" => Ok(InventoryType::ShulkerBox { inner: InventoryTypeStruct::from_raw(env,obj)?}),"BARREL" => Ok(InventoryType::Barrel { inner: InventoryTypeStruct::from_raw(env,obj)?}),"BLAST_FURNACE" => Ok(InventoryType::BlastFurnace { inner: InventoryTypeStruct::from_raw(env,obj)?}),"LECTERN" => Ok(InventoryType::Lectern { inner: InventoryTypeStruct::from_raw(env,obj)?}),"SMOKER" => Ok(InventoryType::Smoker { inner: InventoryTypeStruct::from_raw(env,obj)?}),"LOOM" => Ok(InventoryType::Loom { inner: InventoryTypeStruct::from_raw(env,obj)?}),"CARTOGRAPHY" => Ok(InventoryType::Cartography { inner: InventoryTypeStruct::from_raw(env,obj)?}),"GRINDSTONE" => Ok(InventoryType::Grindstone { inner: InventoryTypeStruct::from_raw(env,obj)?}),"STONECUTTER" => Ok(InventoryType::Stonecutter { inner: InventoryTypeStruct::from_raw(env,obj)?}),"COMPOSTER" => Ok(InventoryType::Composter { inner: InventoryTypeStruct::from_raw(env,obj)?}),"CHISELED_BOOKSHELF" => Ok(InventoryType::ChiseledBookshelf { inner: InventoryTypeStruct::from_raw(env,obj)?}),"JUKEBOX" => Ok(InventoryType::Jukebox { inner: InventoryTypeStruct::from_raw(env,obj)?}),"CRAFTER" => Ok(InventoryType::Crafter { inner: InventoryTypeStruct::from_raw(env,obj)?}),"SMITHING_NEW" => Ok(InventoryType::SmithingNew { inner: InventoryTypeStruct::from_raw(env,obj)?}),_ => Err(eyre::eyre!("String gaven for variant was invalid").into())}
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
                return Err(eyre::eyre!(
                    "Tried to instantiate InventoryTypeStruct from null object.")
                .into());
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
	pub fn values(jni: &blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::event::inventory::InventoryType<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lcrate::event::inventory::InventoryType;");
let cls = jni.find_class("org/bukkit/event/inventory/InventoryType"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.call_static_method(cls,"values",
sig.as_str(),vec![]);
let res = 
jni.translate_error(res)?;
let obj = res.l()?;
crate::event::inventory::InventoryType::from_raw(&jni,obj
)}
	pub fn default_size(&self) 
-> Result<i32, Box<dyn std::error::Error>>

{let sig = String::from("()Li32;");
let res = self.jni_ref().call_method(&self.jni_object(),"getDefaultSize",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.i()?
)}
	pub fn default_title(&self) 
-> Result<String, Box<dyn std::error::Error>>

{let sig = String::from("()LString;");
let res = self.jni_ref().call_method(&self.jni_object(),"getDefaultTitle",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string()
)}
	pub fn is_creatable(&self) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("()Lbool;");
let res = self.jni_ref().call_method(&self.jni_object(),"isCreatable",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
// SUPER CLASS: Enum

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
pub enum HopperInventorySearchEventContainerType<'mc> {
	Source {inner: HopperInventorySearchEventContainerTypeStruct<'mc>},
	Destination {inner: HopperInventorySearchEventContainerTypeStruct<'mc>},
}
impl<'mc> std::fmt::Display for HopperInventorySearchEventContainerType<'mc> {
   fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
       match self {
           HopperInventorySearchEventContainerType::Source { .. } => f.write_str("SOURCE"),
           HopperInventorySearchEventContainerType::Destination { .. } => f.write_str("DESTINATION"),
       }
   }
}

        impl<'mc> HopperInventorySearchEventContainerType<'mc> {
            pub fn value_of(
                env: &blackboxmc_general::SharedJNIEnv<'mc>,
                arg0: impl Into<String>,
            ) -> Result<HopperInventorySearchEventContainerType<'mc>, Box<dyn std::error::Error>> {
                let val_1 = jni::objects::JObject::from(env.new_string(arg0.into())?);
                let cls = env.find_class("org/bukkit/event/inventory/HopperInventorySearchEvent/ContainerType");
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
                    
"SOURCE" => Ok(HopperInventorySearchEventContainerType::Source { inner: HopperInventorySearchEventContainerTypeStruct::from_raw(env,obj)?}),
"DESTINATION" => Ok(HopperInventorySearchEventContainerType::Destination { inner: HopperInventorySearchEventContainerTypeStruct::from_raw(env,obj)?}),

                    _ => Err(eyre::eyre!("String gaven for variant was invalid").into())
                }
            }
        }
        
#[repr(C)]
pub struct HopperInventorySearchEventContainerTypeStruct<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

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
Self::Destination { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
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
                    "Tried to instantiate HopperInventorySearchEventContainerType from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/event/inventory/HopperInventorySearchEvent/ContainerType")?;
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
                    "SOURCE" => Ok(HopperInventorySearchEventContainerType::Source { inner: HopperInventorySearchEventContainerTypeStruct::from_raw(env,obj)?}),"DESTINATION" => Ok(HopperInventorySearchEventContainerType::Destination { inner: HopperInventorySearchEventContainerTypeStruct::from_raw(env,obj)?}),_ => Err(eyre::eyre!("String gaven for variant was invalid").into())}
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
            let (valid, name) = env.validate_name(&obj, "org/bukkit/event/inventory/HopperInventorySearchEvent/ContainerType")?;
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
	pub fn values(jni: &blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::event::inventory::HopperInventorySearchEventContainerType<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lcrate::event::inventory::HopperInventorySearchEventContainerType;");
let cls = jni.find_class("org/bukkit/event/inventory/HopperInventorySearchEvent/ContainerType"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.call_static_method(cls,"values",
sig.as_str(),vec![]);
let res = 
jni.translate_error(res)?;
let obj = res.l()?;
crate::event::inventory::HopperInventorySearchEventContainerType::from_raw(&jni,obj
)}
// SUPER CLASS: Enum

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
#[repr(C)]
pub struct PrepareItemCraftEvent<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

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
                    "Tried to instantiate PrepareItemCraftEvent from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/event/inventory/PrepareItemCraftEvent")?;
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
	pub fn new(jni: &blackboxmc_general::SharedJNIEnv<'mc>,what: impl Into<crate::inventory::CraftingInventory<'mc>>,view: impl Into<crate::inventory::InventoryView<'mc>>,is_repair: bool) 
-> Result<crate::event::inventory::PrepareItemCraftEvent<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/inventory/CraftingInventory;Lorg/bukkit/inventory/InventoryView;Z)V");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(what.into().jni_object().clone())});
let val_2 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(view.into().jni_object().clone())});
let val_3 = jni::objects::JValueGen::Bool(is_repair.into());
let cls = jni.find_class("org/bukkit/event/inventory/PrepareItemCraftEvent"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.new_object(cls,
sig.as_str(),vec![jni::objects::JValueGen::from(val_1),jni::objects::JValueGen::from(val_2),jni::objects::JValueGen::from(val_3)]);
let res = 
jni.translate_error_no_gen(res)?;
crate::event::inventory::PrepareItemCraftEvent::from_raw(&jni,res
)}
	pub fn recipe(&self) 
-> Result<Option<crate::inventory::Recipe<'mc>>, Box<dyn std::error::Error>>

{let sig = String::from("()Lcrate::inventory::Recipe;");
let res = self.jni_ref().call_method(&self.jni_object(),"getRecipe",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {return Ok(None);}
Ok(
Some(
crate::inventory::Recipe::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)?
)
)}
	pub fn inventory(&self) 
-> Result<crate::inventory::CraftingInventory<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lcrate::inventory::CraftingInventory;");
let res = self.jni_ref().call_method(&self.jni_object(),"getInventory",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::inventory::CraftingInventory::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn is_repair(&self) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("()Lbool;");
let res = self.jni_ref().call_method(&self.jni_object(),"isRepair",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
	pub fn handlers(&self) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lcrate::event::HandlerList;");
let res = self.jni_ref().call_method(&self.jni_object(),"getHandlers",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::event::HandlerList::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn handler_list(jni: &blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lcrate::event::HandlerList;");
let cls = jni.find_class("org/bukkit/event/inventory/PrepareItemCraftEvent"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.call_static_method(cls,"getHandlerList",
sig.as_str(),vec![]);
let res = 
jni.translate_error(res)?;
let obj = res.l()?;
crate::event::HandlerList::from_raw(&jni,obj
)}
// SUPER CLASS: InventoryEvent

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
impl<'mc> Into<crate::event::inventory::InventoryEvent<'mc>> for PrepareItemCraftEvent<'mc>{

fn into(self) -> crate::event::inventory::InventoryEvent<'mc> {

crate::event::inventory::InventoryEvent::from_raw(&self.jni_ref(), self.1).expect("Error converting PrepareItemCraftEvent into crate::event::inventory::InventoryEvent")

   }
}
#[repr(C)]
pub struct CraftItemEvent<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

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
                return Err(eyre::eyre!(
                    "Tried to instantiate CraftItemEvent from null object.")
                .into());
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
	pub fn new_with_recipe(jni: &blackboxmc_general::SharedJNIEnv<'mc>,recipe: impl Into<crate::inventory::Recipe<'mc>>,what: impl Into<crate::inventory::InventoryView<'mc>>,val_type: impl Into<crate::event::inventory::InventoryTypeSlotType<'mc>>,slot: i32,click: impl Into<crate::event::inventory::ClickType<'mc>>,action: impl Into<crate::event::inventory::InventoryAction<'mc>>,key: std::option::Option<i32>) 
-> Result<crate::event::inventory::CraftItemEvent<'mc>, Box<dyn std::error::Error>>

{let mut args = Vec::new();
let mut sig = String::from("(");
sig += "Lorg/bukkit/inventory/Recipe;";
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(recipe.into().jni_object().clone())});
args.push(val_1);
sig += "Lorg/bukkit/inventory/InventoryView;";
let val_2 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(what.into().jni_object().clone())});
args.push(val_2);
sig += "Lorg/bukkit/event/inventory/InventoryType/SlotType;";
let val_3 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(val_type.into().jni_object().clone())});
args.push(val_3);
sig += "I";
let val_4 = jni::objects::JValueGen::Int(slot);
args.push(val_4);
sig += "Lorg/bukkit/event/inventory/ClickType;";
let val_5 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(click.into().jni_object().clone())});
args.push(val_5);
sig += "Lorg/bukkit/event/inventory/InventoryAction;";
let val_6 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(action.into().jni_object().clone())});
args.push(val_6);
if let Some(a) = key {
sig += "I";
let val_7 = jni::objects::JValueGen::Int(a);
args.push(val_7);
}
sig += ")V";
let cls = jni.find_class("org/bukkit/event/inventory/CraftItemEvent"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.new_object(cls,
sig.as_str(),args);
let res = 
jni.translate_error_no_gen(res)?;
crate::event::inventory::CraftItemEvent::from_raw(&jni,res
)}
	pub fn recipe(&self) 
-> Result<crate::inventory::Recipe<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lcrate::inventory::Recipe;");
let res = self.jni_ref().call_method(&self.jni_object(),"getRecipe",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::inventory::Recipe::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn inventory(&self) 
-> Result<crate::inventory::CraftingInventory<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lcrate::inventory::CraftingInventory;");
let res = self.jni_ref().call_method(&self.jni_object(),"getInventory",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::inventory::CraftingInventory::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
// SUPER CLASS: InventoryClickEvent

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
impl<'mc> Into<crate::event::inventory::InventoryClickEvent<'mc>> for CraftItemEvent<'mc>{

fn into(self) -> crate::event::inventory::InventoryClickEvent<'mc> {

crate::event::inventory::InventoryClickEvent::from_raw(&self.jni_ref(), self.1).expect("Error converting CraftItemEvent into crate::event::inventory::InventoryClickEvent")

   }
}
#[repr(C)]
pub struct InventoryClickEvent<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

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
                return Err(eyre::eyre!(
                    "Tried to instantiate InventoryClickEvent from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/event/inventory/InventoryClickEvent")?;
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
	pub fn new_with_view(jni: &blackboxmc_general::SharedJNIEnv<'mc>,view: impl Into<crate::inventory::InventoryView<'mc>>,val_type: impl Into<crate::event::inventory::InventoryTypeSlotType<'mc>>,slot: i32,click: impl Into<crate::event::inventory::ClickType<'mc>>,action: impl Into<crate::event::inventory::InventoryAction<'mc>>,key: std::option::Option<i32>) 
-> Result<crate::event::inventory::InventoryClickEvent<'mc>, Box<dyn std::error::Error>>

{let mut args = Vec::new();
let mut sig = String::from("(");
sig += "Lorg/bukkit/inventory/InventoryView;";
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(view.into().jni_object().clone())});
args.push(val_1);
sig += "Lorg/bukkit/event/inventory/InventoryType/SlotType;";
let val_2 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(val_type.into().jni_object().clone())});
args.push(val_2);
sig += "I";
let val_3 = jni::objects::JValueGen::Int(slot);
args.push(val_3);
sig += "Lorg/bukkit/event/inventory/ClickType;";
let val_4 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(click.into().jni_object().clone())});
args.push(val_4);
sig += "Lorg/bukkit/event/inventory/InventoryAction;";
let val_5 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(action.into().jni_object().clone())});
args.push(val_5);
if let Some(a) = key {
sig += "I";
let val_6 = jni::objects::JValueGen::Int(a);
args.push(val_6);
}
sig += ")V";
let cls = jni.find_class("org/bukkit/event/inventory/InventoryClickEvent"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.new_object(cls,
sig.as_str(),args);
let res = 
jni.translate_error_no_gen(res)?;
crate::event::inventory::InventoryClickEvent::from_raw(&jni,res
)}
	pub fn slot_type(&self) 
-> Result<crate::event::inventory::InventoryTypeSlotType<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lcrate::event::inventory::InventoryTypeSlotType;");
let res = self.jni_ref().call_method(&self.jni_object(),"getSlotType",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::event::inventory::InventoryTypeSlotType::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn cursor(&self) 
-> Result<Option<crate::inventory::ItemStack<'mc>>, Box<dyn std::error::Error>>

{let sig = String::from("()Lcrate::inventory::ItemStack;");
let res = self.jni_ref().call_method(&self.jni_object(),"getCursor",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {return Ok(None);}
Ok(
Some(
crate::inventory::ItemStack::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)?
)
)}
	pub fn current_item(&self) 
-> Result<Option<crate::inventory::ItemStack<'mc>>, Box<dyn std::error::Error>>

{let sig = String::from("()Lcrate::inventory::ItemStack;");
let res = self.jni_ref().call_method(&self.jni_object(),"getCurrentItem",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {return Ok(None);}
Ok(
Some(
crate::inventory::ItemStack::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)?
)
)}
	pub fn is_right_click(&self) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("()Lbool;");
let res = self.jni_ref().call_method(&self.jni_object(),"isRightClick",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
	pub fn is_left_click(&self) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("()Lbool;");
let res = self.jni_ref().call_method(&self.jni_object(),"isLeftClick",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
	pub fn is_shift_click(&self) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("()Lbool;");
let res = self.jni_ref().call_method(&self.jni_object(),"isShiftClick",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
#[deprecated]

	pub fn set_cursor(&self,stack: impl Into<crate::inventory::ItemStack<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/inventory/ItemStack;)L();");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(stack.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"setCursor",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
	pub fn set_current_item(&self,stack: impl Into<crate::inventory::ItemStack<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/inventory/ItemStack;)L();");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(stack.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"setCurrentItem",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
	pub fn clicked_inventory(&self) 
-> Result<Option<crate::inventory::Inventory<'mc>>, Box<dyn std::error::Error>>

{let sig = String::from("()Lcrate::inventory::Inventory;");
let res = self.jni_ref().call_method(&self.jni_object(),"getClickedInventory",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {return Ok(None);}
Ok(
Some(
crate::inventory::Inventory::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)?
)
)}
	pub fn slot(&self) 
-> Result<i32, Box<dyn std::error::Error>>

{let sig = String::from("()Li32;");
let res = self.jni_ref().call_method(&self.jni_object(),"getSlot",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.i()?
)}
	pub fn raw_slot(&self) 
-> Result<i32, Box<dyn std::error::Error>>

{let sig = String::from("()Li32;");
let res = self.jni_ref().call_method(&self.jni_object(),"getRawSlot",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.i()?
)}
	pub fn hotbar_button(&self) 
-> Result<i32, Box<dyn std::error::Error>>

{let sig = String::from("()Li32;");
let res = self.jni_ref().call_method(&self.jni_object(),"getHotbarButton",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.i()?
)}
	pub fn action(&self) 
-> Result<crate::event::inventory::InventoryAction<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lcrate::event::inventory::InventoryAction;");
let res = self.jni_ref().call_method(&self.jni_object(),"getAction",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::event::inventory::InventoryAction::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn click(&self) 
-> Result<crate::event::inventory::ClickType<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lcrate::event::inventory::ClickType;");
let res = self.jni_ref().call_method(&self.jni_object(),"getClick",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::event::inventory::ClickType::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn handlers(&self) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lcrate::event::HandlerList;");
let res = self.jni_ref().call_method(&self.jni_object(),"getHandlers",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::event::HandlerList::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn handler_list(jni: &blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lcrate::event::HandlerList;");
let cls = jni.find_class("org/bukkit/event/inventory/InventoryClickEvent"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.call_static_method(cls,"getHandlerList",
sig.as_str(),vec![]);
let res = 
jni.translate_error(res)?;
let obj = res.l()?;
crate::event::HandlerList::from_raw(&jni,obj
)}
// SUPER CLASS: InventoryInteractEvent

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
impl<'mc> Into<crate::event::inventory::InventoryInteractEvent<'mc>> for InventoryClickEvent<'mc>{

fn into(self) -> crate::event::inventory::InventoryInteractEvent<'mc> {

crate::event::inventory::InventoryInteractEvent::from_raw(&self.jni_ref(), self.1).expect("Error converting InventoryClickEvent into crate::event::inventory::InventoryInteractEvent")

   }
}
#[repr(C)]
pub struct PrepareAnvilEvent<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

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
                return Err(eyre::eyre!(
                    "Tried to instantiate PrepareAnvilEvent from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/event/inventory/PrepareAnvilEvent")?;
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
	pub fn new(jni: &blackboxmc_general::SharedJNIEnv<'mc>,inventory: impl Into<crate::inventory::InventoryView<'mc>>,result: impl Into<crate::inventory::ItemStack<'mc>>) 
-> Result<crate::event::inventory::PrepareAnvilEvent<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/inventory/InventoryView;Lorg/bukkit/inventory/ItemStack;)V");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(inventory.into().jni_object().clone())});
let val_2 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(result.into().jni_object().clone())});
let cls = jni.find_class("org/bukkit/event/inventory/PrepareAnvilEvent"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.new_object(cls,
sig.as_str(),vec![jni::objects::JValueGen::from(val_1),jni::objects::JValueGen::from(val_2)]);
let res = 
jni.translate_error_no_gen(res)?;
crate::event::inventory::PrepareAnvilEvent::from_raw(&jni,res
)}
	pub fn inventory(&self) 
-> Result<crate::inventory::AnvilInventory<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lcrate::inventory::AnvilInventory;");
let res = self.jni_ref().call_method(&self.jni_object(),"getInventory",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::inventory::AnvilInventory::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn handlers(&self) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lcrate::event::HandlerList;");
let res = self.jni_ref().call_method(&self.jni_object(),"getHandlers",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::event::HandlerList::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn handler_list(jni: &blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lcrate::event::HandlerList;");
let cls = jni.find_class("org/bukkit/event/inventory/PrepareAnvilEvent"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.call_static_method(cls,"getHandlerList",
sig.as_str(),vec![]);
let res = 
jni.translate_error(res)?;
let obj = res.l()?;
crate::event::HandlerList::from_raw(&jni,obj
)}
// SUPER CLASS: PrepareInventoryResultEvent

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
impl<'mc> Into<crate::event::inventory::PrepareInventoryResultEvent<'mc>> for PrepareAnvilEvent<'mc>{

fn into(self) -> crate::event::inventory::PrepareInventoryResultEvent<'mc> {

crate::event::inventory::PrepareInventoryResultEvent::from_raw(&self.jni_ref(), self.1).expect("Error converting PrepareAnvilEvent into crate::event::inventory::PrepareInventoryResultEvent")

   }
}
