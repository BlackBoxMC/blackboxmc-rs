#![allow(deprecated)]
use blackboxmc_general::JNIRaw;
use color_eyre::eyre::Result;
pub struct InventoryOpenEvent<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for InventoryOpenEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> InventoryOpenEvent<'mc> {
pub fn from_raw(env: &blackboxmc_general::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>
) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate InventoryOpenEvent from null object.")
    .into());
}
let (valid, name) = env.validate_name(&obj, "InventoryOpenEvent")?;
if !valid {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a InventoryOpenEvent object, got {}",
        name
    )
    .into())
} else {
    Ok(Self(env.clone(), obj
))
}
}
	pub fn new(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: impl Into<&'mc crate::inventory::InventoryView<'mc>>) 
-> Result<crate::event::inventory::InventoryOpenEvent<'mc>, Box<dyn std::error::Error>>

{let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let cls = &jni.find_class("org/bukkit/event/inventory/InventoryOpenEvent")?;
let res = jni.new_object(cls,
"(Lorg/bukkit/inventory/InventoryView;)V",&[jni::objects::JValueGen::from(&val_0)])?;
crate::event::inventory::InventoryOpenEvent::from_raw(&jni,res
)}
	pub fn handlers(&mut self) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getHandlers","()Lorg/bukkit/event/HandlerList;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::event::HandlerList::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn is_cancelled(&mut self) 
-> Result<bool, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"isCancelled","()Z",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn player(&mut self) 
-> Result<crate::entity::HumanEntity<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getPlayer","()Lorg/bukkit/entity/HumanEntity;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::entity::HumanEntity::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn set_cancelled(&mut self,arg0: bool) 
-> Result<(), Box<dyn std::error::Error>>

{// -2
let val_0 = jni::objects::JValueGen::Bool(arg0.into());
let res = self.jni_ref().call_method(&self.jni_object(),"setCancelled","(Z)V",&[jni::objects::JValueGen::from(&val_0)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn handler_list(jni: blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let cls = &jni.find_class("org/bukkit/event/HandlerList")?;
let res = jni.call_static_method(cls,"getHandlerList",
"()Lorg/bukkit/event/HandlerList;",&[])?;
let mut obj = res.l()?;
crate::event::HandlerList::from_raw(&jni,obj
)}
	pub fn inventory(&mut self) 
-> Result<crate::inventory::Inventory<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getInventory","()Lorg/bukkit/inventory/Inventory;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::inventory::Inventory::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn viewers(&mut self) 
-> Result<blackboxmc_java::JavaList<'mc, orgHumanEntity>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getViewers","()Ljava/util/List;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
blackboxmc_java::JavaList::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn view(&mut self) 
-> Result<crate::inventory::InventoryView<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getView","()Lorg/bukkit/inventory/InventoryView;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::inventory::InventoryView::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn event_name(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getEventName","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn is_asynchronous(&mut self) 
-> Result<bool, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"isAsynchronous","()Z",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn wait(&mut self,arg0: std::option::Option<i64>,arg1: std::option::Option<i32>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_0 = jni::objects::JValueGen::Long(arg0.unwrap().into());
let val_1 = jni::objects::JValueGen::Int(arg1.unwrap().into());
let res = self.jni_ref().call_method(&self.jni_object(),"wait","(JI)V",&[jni::objects::JValueGen::from(&val_0),jni::objects::JValueGen::from(&val_1)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn equals(&mut self,arg0: jni::objects::JObject<'mc>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_0 = arg0;
let res = self.jni_ref().call_method(&self.jni_object(),"equals","(Ljava/lang/Object;)Z",&[jni::objects::JValueGen::from(&val_0)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn to_string(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"toString","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn hash_code(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"hashCode","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn class(&mut self) 
-> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getClass","()Ljava/lang/Class;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(unsafe {jni::objects::JClass::from_raw(res.as_jni().l)})}
	pub fn notify(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notify","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn notify_all(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notifyAll","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
}
impl<'mc> Into<crate::event::Cancellable<'mc>> for InventoryOpenEvent<'mc>{
   fn into(self) -> crate::event::Cancellable<'mc> {
       crate::event::Cancellable::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
impl<'mc> Into<crate::event::inventory::InventoryEvent<'mc>> for InventoryOpenEvent<'mc>{
   fn into(self) -> crate::event::inventory::InventoryEvent<'mc> {
       crate::event::inventory::InventoryEvent::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
pub struct PrepareItemCraftEvent<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for PrepareItemCraftEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> PrepareItemCraftEvent<'mc> {
pub fn from_raw(env: &blackboxmc_general::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>
) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate PrepareItemCraftEvent from null object.")
    .into());
}
let (valid, name) = env.validate_name(&obj, "PrepareItemCraftEvent")?;
if !valid {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a PrepareItemCraftEvent object, got {}",
        name
    )
    .into())
} else {
    Ok(Self(env.clone(), obj
))
}
}
	pub fn new(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: impl Into<&'mc crate::inventory::CraftingInventory<'mc>>,arg1: impl Into<&'mc crate::inventory::InventoryView<'mc>>,arg2: bool) 
-> Result<crate::event::inventory::PrepareItemCraftEvent<'mc>, Box<dyn std::error::Error>>

{let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let val_1 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone())};
// -2
let val_2 = jni::objects::JValueGen::Bool(arg2.into());
let cls = &jni.find_class("org/bukkit/event/inventory/PrepareItemCraftEvent")?;
let res = jni.new_object(cls,
"(Lorg/bukkit/inventory/CraftingInventory;Lorg/bukkit/inventory/InventoryView;Z)V",&[jni::objects::JValueGen::from(&val_0),jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)])?;
crate::event::inventory::PrepareItemCraftEvent::from_raw(&jni,res
)}
	pub fn handlers(&mut self) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getHandlers","()Lorg/bukkit/event/HandlerList;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::event::HandlerList::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn inventory(&mut self) 
-> Result<crate::inventory::CraftingInventory<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getInventory","()Lorg/bukkit/inventory/CraftingInventory;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::inventory::CraftingInventory::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn recipe(&mut self) 
-> Result<crate::inventory::Recipe<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getRecipe","()Lorg/bukkit/inventory/Recipe;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::inventory::Recipe::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn handler_list(jni: blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let cls = &jni.find_class("org/bukkit/event/HandlerList")?;
let res = jni.call_static_method(cls,"getHandlerList",
"()Lorg/bukkit/event/HandlerList;",&[])?;
let mut obj = res.l()?;
crate::event::HandlerList::from_raw(&jni,obj
)}
	pub fn is_repair(&mut self) 
-> Result<bool, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"isRepair","()Z",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn viewers(&mut self) 
-> Result<blackboxmc_java::JavaList<'mc, orgHumanEntity>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getViewers","()Ljava/util/List;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
blackboxmc_java::JavaList::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn view(&mut self) 
-> Result<crate::inventory::InventoryView<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getView","()Lorg/bukkit/inventory/InventoryView;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::inventory::InventoryView::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn event_name(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getEventName","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn is_asynchronous(&mut self) 
-> Result<bool, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"isAsynchronous","()Z",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn wait(&mut self,arg0: std::option::Option<i64>,arg1: std::option::Option<i32>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_0 = jni::objects::JValueGen::Long(arg0.unwrap().into());
let val_1 = jni::objects::JValueGen::Int(arg1.unwrap().into());
let res = self.jni_ref().call_method(&self.jni_object(),"wait","(JI)V",&[jni::objects::JValueGen::from(&val_0),jni::objects::JValueGen::from(&val_1)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn equals(&mut self,arg0: jni::objects::JObject<'mc>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_0 = arg0;
let res = self.jni_ref().call_method(&self.jni_object(),"equals","(Ljava/lang/Object;)Z",&[jni::objects::JValueGen::from(&val_0)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn to_string(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"toString","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn hash_code(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"hashCode","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn class(&mut self) 
-> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getClass","()Ljava/lang/Class;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(unsafe {jni::objects::JClass::from_raw(res.as_jni().l)})}
	pub fn notify(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notify","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn notify_all(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notifyAll","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
}
impl<'mc> Into<crate::event::inventory::InventoryEvent<'mc>> for PrepareItemCraftEvent<'mc>{
   fn into(self) -> crate::event::inventory::InventoryEvent<'mc> {
       crate::event::inventory::InventoryEvent::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
pub struct PrepareAnvilEvent<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for PrepareAnvilEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> PrepareAnvilEvent<'mc> {
pub fn from_raw(env: &blackboxmc_general::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>
) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate PrepareAnvilEvent from null object.")
    .into());
}
let (valid, name) = env.validate_name(&obj, "PrepareAnvilEvent")?;
if !valid {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a PrepareAnvilEvent object, got {}",
        name
    )
    .into())
} else {
    Ok(Self(env.clone(), obj
))
}
}
	pub fn new(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: impl Into<&'mc crate::inventory::InventoryView<'mc>>,arg1: impl Into<&'mc crate::inventory::ItemStack<'mc>>) 
-> Result<crate::event::inventory::PrepareAnvilEvent<'mc>, Box<dyn std::error::Error>>

{let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let val_1 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone())};
let cls = &jni.find_class("org/bukkit/event/inventory/PrepareAnvilEvent")?;
let res = jni.new_object(cls,
"(Lorg/bukkit/inventory/InventoryView;Lorg/bukkit/inventory/ItemStack;)V",&[jni::objects::JValueGen::from(&val_0),jni::objects::JValueGen::from(&val_1)])?;
crate::event::inventory::PrepareAnvilEvent::from_raw(&jni,res
)}
	pub fn handlers(&mut self) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getHandlers","()Lorg/bukkit/event/HandlerList;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::event::HandlerList::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn inventory(&mut self) 
-> Result<crate::inventory::Inventory<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getInventory","()Lorg/bukkit/inventory/Inventory;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::inventory::Inventory::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn handler_list(jni: blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let cls = &jni.find_class("org/bukkit/event/HandlerList")?;
let res = jni.call_static_method(cls,"getHandlerList",
"()Lorg/bukkit/event/HandlerList;",&[])?;
let mut obj = res.l()?;
crate::event::HandlerList::from_raw(&jni,obj
)}
	pub fn set_result(&mut self,arg0: impl Into<&'mc crate::inventory::ItemStack<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let res = self.jni_ref().call_method(&self.jni_object(),"setResult","(Lorg/bukkit/inventory/ItemStack;)V",&[jni::objects::JValueGen::from(&val_0)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn result(&mut self) 
-> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getResult","()Lorg/bukkit/inventory/ItemStack;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::inventory::ItemStack::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn viewers(&mut self) 
-> Result<blackboxmc_java::JavaList<'mc, orgHumanEntity>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getViewers","()Ljava/util/List;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
blackboxmc_java::JavaList::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn view(&mut self) 
-> Result<crate::inventory::InventoryView<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getView","()Lorg/bukkit/inventory/InventoryView;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::inventory::InventoryView::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn event_name(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getEventName","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn is_asynchronous(&mut self) 
-> Result<bool, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"isAsynchronous","()Z",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn wait(&mut self,arg0: std::option::Option<i64>,arg1: std::option::Option<i32>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_0 = jni::objects::JValueGen::Long(arg0.unwrap().into());
let val_1 = jni::objects::JValueGen::Int(arg1.unwrap().into());
let res = self.jni_ref().call_method(&self.jni_object(),"wait","(JI)V",&[jni::objects::JValueGen::from(&val_0),jni::objects::JValueGen::from(&val_1)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn equals(&mut self,arg0: jni::objects::JObject<'mc>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_0 = arg0;
let res = self.jni_ref().call_method(&self.jni_object(),"equals","(Ljava/lang/Object;)Z",&[jni::objects::JValueGen::from(&val_0)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn to_string(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"toString","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn hash_code(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"hashCode","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn class(&mut self) 
-> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getClass","()Ljava/lang/Class;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(unsafe {jni::objects::JClass::from_raw(res.as_jni().l)})}
	pub fn notify(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notify","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn notify_all(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notifyAll","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
}
impl<'mc> Into<crate::event::inventory::PrepareInventoryResultEvent<'mc>> for PrepareAnvilEvent<'mc>{
   fn into(self) -> crate::event::inventory::PrepareInventoryResultEvent<'mc> {
       crate::event::inventory::PrepareInventoryResultEvent::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
pub enum DragTypeEnum {
	Single,
	Even,
}
impl std::fmt::Display for DragTypeEnum {
   fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
       match &self {
           DragTypeEnum::Single => f.write_str("SINGLE"),
           DragTypeEnum::Even => f.write_str("EVEN"),
       }
   }
}
pub struct DragType<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>, pub DragTypeEnum);
impl<'mc> std::ops::Deref for DragType<'mc> {
   type Target = DragTypeEnum;
   fn deref(&self) -> &Self::Target {
       return &self.2;
   }
}
impl<'mc> JNIRaw<'mc> for DragType<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> DragType<'mc> {
pub fn from_raw(env: &blackboxmc_general::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>
, e: DragTypeEnum
) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate DragType from null object.")
    .into());
}
let (valid, name) = env.validate_name(&obj, "DragType")?;
if !valid {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a DragType object, got {}",
        name
    )
    .into())
} else {
    Ok(Self(env.clone(), obj
, e
))
}
}
pub const SINGLE: DragTypeEnum = DragTypeEnum::Single;
pub const EVEN: DragTypeEnum = DragTypeEnum::Even;
pub fn from_string(str: String) -> std::option::Option<DragTypeEnum> {
match str.as_str() {
"SINGLE" => Some(DragTypeEnum::Single),
"EVEN" => Some(DragTypeEnum::Even),
_ => None}}
	pub fn value_of(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: impl Into<&'mc String>) 
-> Result<crate::event::inventory::DragType<'mc>, Box<dyn std::error::Error>>

{let val_0 = jni::objects::JObject::from(jni.new_string(arg0.into()).unwrap());
let cls = &jni.find_class("org/bukkit/event/inventory/DragType")?;
let res = jni.call_static_method(cls,"valueOf",
"(Ljava/lang/String;)Lorg/bukkit/event/inventory/DragType;",&[jni::objects::JValueGen::from(&val_0)])?;
let mut obj = res.l()?;
let raw_obj = obj;let variant = jni.call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;let variant_str = jni    .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?    .to_string_lossy()    .to_string();
crate::event::inventory::DragType::from_raw(&jni,raw_obj
, crate::event::inventory::DragType::from_string(variant_str).unwrap()
)}
}
pub struct BrewEvent<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for BrewEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> BrewEvent<'mc> {
pub fn from_raw(env: &blackboxmc_general::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>
) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate BrewEvent from null object.")
    .into());
}
let (valid, name) = env.validate_name(&obj, "BrewEvent")?;
if !valid {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a BrewEvent object, got {}",
        name
    )
    .into())
} else {
    Ok(Self(env.clone(), obj
))
}
}
	pub fn new(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: impl Into<&'mc crate::block::Block<'mc>>,arg1: impl Into<&'mc crate::inventory::BrewerInventory<'mc>>,arg2: impl Into<&'mc blackboxmc_java::JavaList<'mc, orgItemStack>>,arg3: i32) 
-> Result<crate::event::inventory::BrewEvent<'mc>, Box<dyn std::error::Error>>

{let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let val_1 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone())};
let val_2 = unsafe { jni::objects::JObject::from_raw(arg2.into().jni_object().clone())};
let val_3 = jni::objects::JValueGen::Int(arg3.into());
let cls = &jni.find_class("org/bukkit/event/inventory/BrewEvent")?;
let res = jni.new_object(cls,
"(Lorg/bukkit/block/Block;Lorg/bukkit/inventory/BrewerInventory;Ljava/util/List;I)V",&[jni::objects::JValueGen::from(&val_0),jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2),jni::objects::JValueGen::from(&val_3)])?;
crate::event::inventory::BrewEvent::from_raw(&jni,res
)}
	pub fn handlers(&mut self) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getHandlers","()Lorg/bukkit/event/HandlerList;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::event::HandlerList::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn is_cancelled(&mut self) 
-> Result<bool, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"isCancelled","()Z",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn contents(&mut self) 
-> Result<crate::inventory::BrewerInventory<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getContents","()Lorg/bukkit/inventory/BrewerInventory;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::inventory::BrewerInventory::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn fuel_level(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getFuelLevel","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn set_cancelled(&mut self,arg0: bool) 
-> Result<(), Box<dyn std::error::Error>>

{// -2
let val_0 = jni::objects::JValueGen::Bool(arg0.into());
let res = self.jni_ref().call_method(&self.jni_object(),"setCancelled","(Z)V",&[jni::objects::JValueGen::from(&val_0)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn handler_list(jni: blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let cls = &jni.find_class("org/bukkit/event/HandlerList")?;
let res = jni.call_static_method(cls,"getHandlerList",
"()Lorg/bukkit/event/HandlerList;",&[])?;
let mut obj = res.l()?;
crate::event::HandlerList::from_raw(&jni,obj
)}
	pub fn results(&mut self) 
-> Result<blackboxmc_java::JavaList<'mc, orgItemStack>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getResults","()Ljava/util/List;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
blackboxmc_java::JavaList::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn block(&mut self) 
-> Result<crate::block::Block<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getBlock","()Lorg/bukkit/block/Block;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::block::Block::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn event_name(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getEventName","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn is_asynchronous(&mut self) 
-> Result<bool, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"isAsynchronous","()Z",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn wait(&mut self,arg0: std::option::Option<i64>,arg1: std::option::Option<i32>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_0 = jni::objects::JValueGen::Long(arg0.unwrap().into());
let val_1 = jni::objects::JValueGen::Int(arg1.unwrap().into());
let res = self.jni_ref().call_method(&self.jni_object(),"wait","(JI)V",&[jni::objects::JValueGen::from(&val_0),jni::objects::JValueGen::from(&val_1)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn equals(&mut self,arg0: jni::objects::JObject<'mc>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_0 = arg0;
let res = self.jni_ref().call_method(&self.jni_object(),"equals","(Ljava/lang/Object;)Z",&[jni::objects::JValueGen::from(&val_0)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn to_string(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"toString","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn hash_code(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"hashCode","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn class(&mut self) 
-> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getClass","()Ljava/lang/Class;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(unsafe {jni::objects::JClass::from_raw(res.as_jni().l)})}
	pub fn notify(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notify","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn notify_all(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notifyAll","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
}
impl<'mc> Into<crate::event::Cancellable<'mc>> for BrewEvent<'mc>{
   fn into(self) -> crate::event::Cancellable<'mc> {
       crate::event::Cancellable::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
impl<'mc> Into<crate::event::block::BlockEvent<'mc>> for BrewEvent<'mc>{
   fn into(self) -> crate::event::block::BlockEvent<'mc> {
       crate::event::block::BlockEvent::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
pub struct InventoryMoveItemEvent<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for InventoryMoveItemEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> InventoryMoveItemEvent<'mc> {
pub fn from_raw(env: &blackboxmc_general::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>
) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate InventoryMoveItemEvent from null object.")
    .into());
}
let (valid, name) = env.validate_name(&obj, "InventoryMoveItemEvent")?;
if !valid {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a InventoryMoveItemEvent object, got {}",
        name
    )
    .into())
} else {
    Ok(Self(env.clone(), obj
))
}
}
	pub fn new(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: impl Into<&'mc crate::inventory::Inventory<'mc>>,arg1: impl Into<&'mc crate::inventory::ItemStack<'mc>>,arg2: impl Into<&'mc crate::inventory::Inventory<'mc>>,arg3: bool) 
-> Result<crate::event::inventory::InventoryMoveItemEvent<'mc>, Box<dyn std::error::Error>>

{let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let val_1 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone())};
let val_2 = unsafe { jni::objects::JObject::from_raw(arg2.into().jni_object().clone())};
// -2
let val_3 = jni::objects::JValueGen::Bool(arg3.into());
let cls = &jni.find_class("org/bukkit/event/inventory/InventoryMoveItemEvent")?;
let res = jni.new_object(cls,
"(Lorg/bukkit/inventory/Inventory;Lorg/bukkit/inventory/ItemStack;Lorg/bukkit/inventory/Inventory;Z)V",&[jni::objects::JValueGen::from(&val_0),jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2),jni::objects::JValueGen::from(&val_3)])?;
crate::event::inventory::InventoryMoveItemEvent::from_raw(&jni,res
)}
	pub fn handlers(&mut self) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getHandlers","()Lorg/bukkit/event/HandlerList;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::event::HandlerList::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn item(&mut self) 
-> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getItem","()Lorg/bukkit/inventory/ItemStack;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::inventory::ItemStack::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn source(&mut self) 
-> Result<crate::inventory::Inventory<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getSource","()Lorg/bukkit/inventory/Inventory;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::inventory::Inventory::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn is_cancelled(&mut self) 
-> Result<bool, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"isCancelled","()Z",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn set_item(&mut self,arg0: impl Into<&'mc crate::inventory::ItemStack<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let res = self.jni_ref().call_method(&self.jni_object(),"setItem","(Lorg/bukkit/inventory/ItemStack;)V",&[jni::objects::JValueGen::from(&val_0)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn destination(&mut self) 
-> Result<crate::inventory::Inventory<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getDestination","()Lorg/bukkit/inventory/Inventory;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::inventory::Inventory::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn set_cancelled(&mut self,arg0: bool) 
-> Result<(), Box<dyn std::error::Error>>

{// -2
let val_0 = jni::objects::JValueGen::Bool(arg0.into());
let res = self.jni_ref().call_method(&self.jni_object(),"setCancelled","(Z)V",&[jni::objects::JValueGen::from(&val_0)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn handler_list(jni: blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let cls = &jni.find_class("org/bukkit/event/HandlerList")?;
let res = jni.call_static_method(cls,"getHandlerList",
"()Lorg/bukkit/event/HandlerList;",&[])?;
let mut obj = res.l()?;
crate::event::HandlerList::from_raw(&jni,obj
)}
	pub fn initiator(&mut self) 
-> Result<crate::inventory::Inventory<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getInitiator","()Lorg/bukkit/inventory/Inventory;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::inventory::Inventory::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn event_name(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getEventName","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn is_asynchronous(&mut self) 
-> Result<bool, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"isAsynchronous","()Z",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn wait(&mut self,arg0: std::option::Option<i64>,arg1: std::option::Option<i32>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_0 = jni::objects::JValueGen::Long(arg0.unwrap().into());
let val_1 = jni::objects::JValueGen::Int(arg1.unwrap().into());
let res = self.jni_ref().call_method(&self.jni_object(),"wait","(JI)V",&[jni::objects::JValueGen::from(&val_0),jni::objects::JValueGen::from(&val_1)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn equals(&mut self,arg0: jni::objects::JObject<'mc>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_0 = arg0;
let res = self.jni_ref().call_method(&self.jni_object(),"equals","(Ljava/lang/Object;)Z",&[jni::objects::JValueGen::from(&val_0)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn to_string(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"toString","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn hash_code(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"hashCode","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn class(&mut self) 
-> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getClass","()Ljava/lang/Class;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(unsafe {jni::objects::JClass::from_raw(res.as_jni().l)})}
	pub fn notify(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notify","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn notify_all(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notifyAll","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
}
impl<'mc> Into<crate::event::Cancellable<'mc>> for InventoryMoveItemEvent<'mc>{
   fn into(self) -> crate::event::Cancellable<'mc> {
       crate::event::Cancellable::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
impl<'mc> Into<crate::event::Event<'mc>> for InventoryMoveItemEvent<'mc>{
   fn into(self) -> crate::event::Event<'mc> {
       crate::event::Event::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
pub struct FurnaceSmeltEvent<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for FurnaceSmeltEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> FurnaceSmeltEvent<'mc> {
pub fn from_raw(env: &blackboxmc_general::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>
) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate FurnaceSmeltEvent from null object.")
    .into());
}
let (valid, name) = env.validate_name(&obj, "FurnaceSmeltEvent")?;
if !valid {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a FurnaceSmeltEvent object, got {}",
        name
    )
    .into())
} else {
    Ok(Self(env.clone(), obj
))
}
}
	pub fn new(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: impl Into<&'mc crate::block::Block<'mc>>,arg1: impl Into<&'mc crate::inventory::ItemStack<'mc>>,arg2: impl Into<&'mc crate::inventory::ItemStack<'mc>>) 
-> Result<crate::event::inventory::FurnaceSmeltEvent<'mc>, Box<dyn std::error::Error>>

{let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let val_1 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone())};
let val_2 = unsafe { jni::objects::JObject::from_raw(arg2.into().jni_object().clone())};
let cls = &jni.find_class("org/bukkit/event/inventory/FurnaceSmeltEvent")?;
let res = jni.new_object(cls,
"(Lorg/bukkit/block/Block;Lorg/bukkit/inventory/ItemStack;Lorg/bukkit/inventory/ItemStack;)V",&[jni::objects::JValueGen::from(&val_0),jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)])?;
crate::event::inventory::FurnaceSmeltEvent::from_raw(&jni,res
)}
	pub fn handlers(&mut self) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getHandlers","()Lorg/bukkit/event/HandlerList;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::event::HandlerList::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn set_result(&mut self,arg0: impl Into<&'mc crate::inventory::ItemStack<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let res = self.jni_ref().call_method(&self.jni_object(),"setResult","(Lorg/bukkit/inventory/ItemStack;)V",&[jni::objects::JValueGen::from(&val_0)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn result(&mut self) 
-> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getResult","()Lorg/bukkit/inventory/ItemStack;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::inventory::ItemStack::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn source(&mut self) 
-> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getSource","()Lorg/bukkit/inventory/ItemStack;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::inventory::ItemStack::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn is_cancelled(&mut self) 
-> Result<bool, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"isCancelled","()Z",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn set_cancelled(&mut self,arg0: bool) 
-> Result<(), Box<dyn std::error::Error>>

{// -2
let val_0 = jni::objects::JValueGen::Bool(arg0.into());
let res = self.jni_ref().call_method(&self.jni_object(),"setCancelled","(Z)V",&[jni::objects::JValueGen::from(&val_0)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn handler_list(jni: blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let cls = &jni.find_class("org/bukkit/event/HandlerList")?;
let res = jni.call_static_method(cls,"getHandlerList",
"()Lorg/bukkit/event/HandlerList;",&[])?;
let mut obj = res.l()?;
crate::event::HandlerList::from_raw(&jni,obj
)}
	pub fn block(&mut self) 
-> Result<crate::block::Block<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getBlock","()Lorg/bukkit/block/Block;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::block::Block::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn event_name(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getEventName","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn is_asynchronous(&mut self) 
-> Result<bool, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"isAsynchronous","()Z",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn wait(&mut self,arg0: std::option::Option<i64>,arg1: std::option::Option<i32>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_0 = jni::objects::JValueGen::Long(arg0.unwrap().into());
let val_1 = jni::objects::JValueGen::Int(arg1.unwrap().into());
let res = self.jni_ref().call_method(&self.jni_object(),"wait","(JI)V",&[jni::objects::JValueGen::from(&val_0),jni::objects::JValueGen::from(&val_1)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn equals(&mut self,arg0: jni::objects::JObject<'mc>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_0 = arg0;
let res = self.jni_ref().call_method(&self.jni_object(),"equals","(Ljava/lang/Object;)Z",&[jni::objects::JValueGen::from(&val_0)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn to_string(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"toString","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn hash_code(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"hashCode","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn class(&mut self) 
-> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getClass","()Ljava/lang/Class;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(unsafe {jni::objects::JClass::from_raw(res.as_jni().l)})}
	pub fn notify(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notify","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn notify_all(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notifyAll","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
}
impl<'mc> Into<crate::event::block::BlockCookEvent<'mc>> for FurnaceSmeltEvent<'mc>{
   fn into(self) -> crate::event::block::BlockCookEvent<'mc> {
       crate::event::block::BlockCookEvent::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
pub struct SmithItemEvent<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for SmithItemEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> SmithItemEvent<'mc> {
pub fn from_raw(env: &blackboxmc_general::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>
) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate SmithItemEvent from null object.")
    .into());
}
let (valid, name) = env.validate_name(&obj, "SmithItemEvent")?;
if !valid {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a SmithItemEvent object, got {}",
        name
    )
    .into())
} else {
    Ok(Self(env.clone(), obj
))
}
}
	pub fn new_with_inventory_view(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: impl Into<&'mc crate::inventory::InventoryView<'mc>>,arg1: impl Into<&'mc crate::event::inventory::InventoryTypeSlotType<'mc>>,arg2: i32,arg3: impl Into<&'mc crate::event::inventory::ClickType<'mc>>,arg4: std::option::Option<impl Into<&'mc crate::event::inventory::InventoryAction<'mc>>>,arg5: std::option::Option<i32>) 
-> Result<crate::event::inventory::SmithItemEvent<'mc>, Box<dyn std::error::Error>>

{let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let val_1 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone())};
let val_2 = jni::objects::JValueGen::Int(arg2.into());
let val_3 = unsafe { jni::objects::JObject::from_raw(arg3.into().jni_object().clone())};
let val_4 = unsafe { jni::objects::JObject::from_raw(arg4.unwrap().into().jni_object().clone())};
let val_5 = jni::objects::JValueGen::Int(arg5.unwrap().into());
let cls = &jni.find_class("org/bukkit/event/inventory/SmithItemEvent")?;
let res = jni.new_object(cls,
"(Lorg/bukkit/inventory/InventoryView;Lorg/bukkit/event/inventory/InventoryType$SlotType;ILorg/bukkit/event/inventory/ClickType;Lorg/bukkit/event/inventory/InventoryAction;I)V",&[jni::objects::JValueGen::from(&val_0),jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2),jni::objects::JValueGen::from(&val_3),jni::objects::JValueGen::from(&val_4),jni::objects::JValueGen::from(&val_5)])?;
crate::event::inventory::SmithItemEvent::from_raw(&jni,res
)}
	pub fn inventory(&mut self) 
-> Result<crate::inventory::SmithingInventory<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getInventory","()Lorg/bukkit/inventory/SmithingInventory;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::inventory::SmithingInventory::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn handlers(&mut self) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getHandlers","()Lorg/bukkit/event/HandlerList;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::event::HandlerList::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn slot(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getSlot","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
#[deprecated]
	pub fn set_cursor(&mut self,arg0: impl Into<&'mc crate::inventory::ItemStack<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let res = self.jni_ref().call_method(&self.jni_object(),"setCursor","(Lorg/bukkit/inventory/ItemStack;)V",&[jni::objects::JValueGen::from(&val_0)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn cursor(&mut self) 
-> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getCursor","()Lorg/bukkit/inventory/ItemStack;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::inventory::ItemStack::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn slot_type(&mut self) 
-> Result<crate::event::inventory::InventoryTypeSlotType<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getSlotType","()Lorg/bukkit/event/inventory/InventoryType$SlotType;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::event::inventory::InventoryTypeSlotType::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn handler_list(jni: blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let cls = &jni.find_class("org/bukkit/event/HandlerList")?;
let res = jni.call_static_method(cls,"getHandlerList",
"()Lorg/bukkit/event/HandlerList;",&[])?;
let mut obj = res.l()?;
crate::event::HandlerList::from_raw(&jni,obj
)}
	pub fn action(&mut self) 
-> Result<crate::event::inventory::InventoryAction<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getAction","()Lorg/bukkit/event/inventory/InventoryAction;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };let variant = self.0.call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;let variant_str = self.0    .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?    .to_string_lossy()    .to_string();
crate::event::inventory::InventoryAction::from_raw(&self.jni_ref(),raw_obj
, crate::event::inventory::InventoryAction::from_string(variant_str).unwrap()
)}
	pub fn current_item(&mut self) 
-> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getCurrentItem","()Lorg/bukkit/inventory/ItemStack;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::inventory::ItemStack::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn is_right_click(&mut self) 
-> Result<bool, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"isRightClick","()Z",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn is_left_click(&mut self) 
-> Result<bool, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"isLeftClick","()Z",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn is_shift_click(&mut self) 
-> Result<bool, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"isShiftClick","()Z",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn set_current_item(&mut self,arg0: impl Into<&'mc crate::inventory::ItemStack<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let res = self.jni_ref().call_method(&self.jni_object(),"setCurrentItem","(Lorg/bukkit/inventory/ItemStack;)V",&[jni::objects::JValueGen::from(&val_0)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn clicked_inventory(&mut self) 
-> Result<crate::inventory::Inventory<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getClickedInventory","()Lorg/bukkit/inventory/Inventory;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::inventory::Inventory::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn raw_slot(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getRawSlot","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn hotbar_button(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getHotbarButton","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn click(&mut self) 
-> Result<crate::event::inventory::ClickType<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getClick","()Lorg/bukkit/event/inventory/ClickType;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };let variant = self.0.call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;let variant_str = self.0    .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?    .to_string_lossy()    .to_string();
crate::event::inventory::ClickType::from_raw(&self.jni_ref(),raw_obj
, crate::event::inventory::ClickType::from_string(variant_str).unwrap()
)}
	pub fn set_result(&mut self,arg0: impl Into<&'mc crate::event::EventResult<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let res = self.jni_ref().call_method(&self.jni_object(),"setResult","(Lorg/bukkit/event/Event$Result;)V",&[jni::objects::JValueGen::from(&val_0)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn result(&mut self) 
-> Result<crate::event::EventResult<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getResult","()Lorg/bukkit/event/Event$Result;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::event::EventResult::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn is_cancelled(&mut self) 
-> Result<bool, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"isCancelled","()Z",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn set_cancelled(&mut self,arg0: bool) 
-> Result<(), Box<dyn std::error::Error>>

{// -2
let val_0 = jni::objects::JValueGen::Bool(arg0.into());
let res = self.jni_ref().call_method(&self.jni_object(),"setCancelled","(Z)V",&[jni::objects::JValueGen::from(&val_0)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn who_clicked(&mut self) 
-> Result<crate::entity::HumanEntity<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getWhoClicked","()Lorg/bukkit/entity/HumanEntity;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::entity::HumanEntity::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn viewers(&mut self) 
-> Result<blackboxmc_java::JavaList<'mc, orgHumanEntity>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getViewers","()Ljava/util/List;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
blackboxmc_java::JavaList::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn view(&mut self) 
-> Result<crate::inventory::InventoryView<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getView","()Lorg/bukkit/inventory/InventoryView;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::inventory::InventoryView::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn event_name(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getEventName","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn is_asynchronous(&mut self) 
-> Result<bool, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"isAsynchronous","()Z",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn wait(&mut self,arg0: std::option::Option<i64>,arg1: std::option::Option<i32>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_0 = jni::objects::JValueGen::Long(arg0.unwrap().into());
let val_1 = jni::objects::JValueGen::Int(arg1.unwrap().into());
let res = self.jni_ref().call_method(&self.jni_object(),"wait","(JI)V",&[jni::objects::JValueGen::from(&val_0),jni::objects::JValueGen::from(&val_1)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn equals(&mut self,arg0: jni::objects::JObject<'mc>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_0 = arg0;
let res = self.jni_ref().call_method(&self.jni_object(),"equals","(Ljava/lang/Object;)Z",&[jni::objects::JValueGen::from(&val_0)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn to_string(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"toString","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn hash_code(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"hashCode","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn class(&mut self) 
-> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getClass","()Ljava/lang/Class;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(unsafe {jni::objects::JClass::from_raw(res.as_jni().l)})}
	pub fn notify(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notify","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn notify_all(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notifyAll","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
}
impl<'mc> Into<crate::event::inventory::InventoryClickEvent<'mc>> for SmithItemEvent<'mc>{
   fn into(self) -> crate::event::inventory::InventoryClickEvent<'mc> {
       crate::event::inventory::InventoryClickEvent::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
pub enum ClickTypeEnum {
	Left,
	ShiftLeft,
	Right,
	ShiftRight,
	WindowBorderLeft,
	WindowBorderRight,
	Middle,
	NumberKey,
	DoubleClick,
	Drop,
	ControlDrop,
	Creative,
	SwapOffhand,
	Unknown,
}
impl std::fmt::Display for ClickTypeEnum {
   fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
       match &self {
           ClickTypeEnum::Left => f.write_str("LEFT"),
           ClickTypeEnum::ShiftLeft => f.write_str("SHIFT_LEFT"),
           ClickTypeEnum::Right => f.write_str("RIGHT"),
           ClickTypeEnum::ShiftRight => f.write_str("SHIFT_RIGHT"),
           ClickTypeEnum::WindowBorderLeft => f.write_str("WINDOW_BORDER_LEFT"),
           ClickTypeEnum::WindowBorderRight => f.write_str("WINDOW_BORDER_RIGHT"),
           ClickTypeEnum::Middle => f.write_str("MIDDLE"),
           ClickTypeEnum::NumberKey => f.write_str("NUMBER_KEY"),
           ClickTypeEnum::DoubleClick => f.write_str("DOUBLE_CLICK"),
           ClickTypeEnum::Drop => f.write_str("DROP"),
           ClickTypeEnum::ControlDrop => f.write_str("CONTROL_DROP"),
           ClickTypeEnum::Creative => f.write_str("CREATIVE"),
           ClickTypeEnum::SwapOffhand => f.write_str("SWAP_OFFHAND"),
           ClickTypeEnum::Unknown => f.write_str("UNKNOWN"),
       }
   }
}
pub struct ClickType<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>, pub ClickTypeEnum);
impl<'mc> std::ops::Deref for ClickType<'mc> {
   type Target = ClickTypeEnum;
   fn deref(&self) -> &Self::Target {
       return &self.2;
   }
}
impl<'mc> JNIRaw<'mc> for ClickType<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> ClickType<'mc> {
pub fn from_raw(env: &blackboxmc_general::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>
, e: ClickTypeEnum
) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate ClickType from null object.")
    .into());
}
let (valid, name) = env.validate_name(&obj, "ClickType")?;
if !valid {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a ClickType object, got {}",
        name
    )
    .into())
} else {
    Ok(Self(env.clone(), obj
, e
))
}
}
pub const LEFT: ClickTypeEnum = ClickTypeEnum::Left;
pub const SHIFT_LEFT: ClickTypeEnum = ClickTypeEnum::ShiftLeft;
pub const RIGHT: ClickTypeEnum = ClickTypeEnum::Right;
pub const SHIFT_RIGHT: ClickTypeEnum = ClickTypeEnum::ShiftRight;
pub const WINDOW_BORDER_LEFT: ClickTypeEnum = ClickTypeEnum::WindowBorderLeft;
pub const WINDOW_BORDER_RIGHT: ClickTypeEnum = ClickTypeEnum::WindowBorderRight;
pub const MIDDLE: ClickTypeEnum = ClickTypeEnum::Middle;
pub const NUMBER_KEY: ClickTypeEnum = ClickTypeEnum::NumberKey;
pub const DOUBLE_CLICK: ClickTypeEnum = ClickTypeEnum::DoubleClick;
pub const DROP: ClickTypeEnum = ClickTypeEnum::Drop;
pub const CONTROL_DROP: ClickTypeEnum = ClickTypeEnum::ControlDrop;
pub const CREATIVE: ClickTypeEnum = ClickTypeEnum::Creative;
pub const SWAP_OFFHAND: ClickTypeEnum = ClickTypeEnum::SwapOffhand;
pub const UNKNOWN: ClickTypeEnum = ClickTypeEnum::Unknown;
pub fn from_string(str: String) -> std::option::Option<ClickTypeEnum> {
match str.as_str() {
"LEFT" => Some(ClickTypeEnum::Left),
"SHIFT_LEFT" => Some(ClickTypeEnum::ShiftLeft),
"RIGHT" => Some(ClickTypeEnum::Right),
"SHIFT_RIGHT" => Some(ClickTypeEnum::ShiftRight),
"WINDOW_BORDER_LEFT" => Some(ClickTypeEnum::WindowBorderLeft),
"WINDOW_BORDER_RIGHT" => Some(ClickTypeEnum::WindowBorderRight),
"MIDDLE" => Some(ClickTypeEnum::Middle),
"NUMBER_KEY" => Some(ClickTypeEnum::NumberKey),
"DOUBLE_CLICK" => Some(ClickTypeEnum::DoubleClick),
"DROP" => Some(ClickTypeEnum::Drop),
"CONTROL_DROP" => Some(ClickTypeEnum::ControlDrop),
"CREATIVE" => Some(ClickTypeEnum::Creative),
"SWAP_OFFHAND" => Some(ClickTypeEnum::SwapOffhand),
"UNKNOWN" => Some(ClickTypeEnum::Unknown),
_ => None}}
	pub fn value_of(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: impl Into<&'mc String>) 
-> Result<crate::event::inventory::ClickType<'mc>, Box<dyn std::error::Error>>

{let val_0 = jni::objects::JObject::from(jni.new_string(arg0.into()).unwrap());
let cls = &jni.find_class("org/bukkit/event/inventory/ClickType")?;
let res = jni.call_static_method(cls,"valueOf",
"(Ljava/lang/String;)Lorg/bukkit/event/inventory/ClickType;",&[jni::objects::JValueGen::from(&val_0)])?;
let mut obj = res.l()?;
let raw_obj = obj;let variant = jni.call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;let variant_str = jni    .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?    .to_string_lossy()    .to_string();
crate::event::inventory::ClickType::from_raw(&jni,raw_obj
, crate::event::inventory::ClickType::from_string(variant_str).unwrap()
)}
	pub fn is_right_click(&mut self) 
-> Result<bool, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"isRightClick","()Z",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn is_left_click(&mut self) 
-> Result<bool, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"isLeftClick","()Z",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn is_shift_click(&mut self) 
-> Result<bool, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"isShiftClick","()Z",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn is_keyboard_click(&mut self) 
-> Result<bool, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"isKeyboardClick","()Z",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn is_mouse_click(&mut self) 
-> Result<bool, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"isMouseClick","()Z",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn is_creative_action(&mut self) 
-> Result<bool, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"isCreativeAction","()Z",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
}
pub struct BrewingStandFuelEvent<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for BrewingStandFuelEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> BrewingStandFuelEvent<'mc> {
pub fn from_raw(env: &blackboxmc_general::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>
) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate BrewingStandFuelEvent from null object.")
    .into());
}
let (valid, name) = env.validate_name(&obj, "BrewingStandFuelEvent")?;
if !valid {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a BrewingStandFuelEvent object, got {}",
        name
    )
    .into())
} else {
    Ok(Self(env.clone(), obj
))
}
}
	pub fn new(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: impl Into<&'mc crate::block::Block<'mc>>,arg1: impl Into<&'mc crate::inventory::ItemStack<'mc>>,arg2: i32) 
-> Result<crate::event::inventory::BrewingStandFuelEvent<'mc>, Box<dyn std::error::Error>>

{let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let val_1 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone())};
let val_2 = jni::objects::JValueGen::Int(arg2.into());
let cls = &jni.find_class("org/bukkit/event/inventory/BrewingStandFuelEvent")?;
let res = jni.new_object(cls,
"(Lorg/bukkit/block/Block;Lorg/bukkit/inventory/ItemStack;I)V",&[jni::objects::JValueGen::from(&val_0),jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)])?;
crate::event::inventory::BrewingStandFuelEvent::from_raw(&jni,res
)}
	pub fn handlers(&mut self) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getHandlers","()Lorg/bukkit/event/HandlerList;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::event::HandlerList::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn is_cancelled(&mut self) 
-> Result<bool, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"isCancelled","()Z",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn fuel(&mut self) 
-> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getFuel","()Lorg/bukkit/inventory/ItemStack;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::inventory::ItemStack::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn set_cancelled(&mut self,arg0: bool) 
-> Result<(), Box<dyn std::error::Error>>

{// -2
let val_0 = jni::objects::JValueGen::Bool(arg0.into());
let res = self.jni_ref().call_method(&self.jni_object(),"setCancelled","(Z)V",&[jni::objects::JValueGen::from(&val_0)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn handler_list(jni: blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let cls = &jni.find_class("org/bukkit/event/HandlerList")?;
let res = jni.call_static_method(cls,"getHandlerList",
"()Lorg/bukkit/event/HandlerList;",&[])?;
let mut obj = res.l()?;
crate::event::HandlerList::from_raw(&jni,obj
)}
	pub fn fuel_power(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getFuelPower","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn set_fuel_power(&mut self,arg0: i32) 
-> Result<(), Box<dyn std::error::Error>>

{let val_0 = jni::objects::JValueGen::Int(arg0.into());
let res = self.jni_ref().call_method(&self.jni_object(),"setFuelPower","(I)V",&[jni::objects::JValueGen::from(&val_0)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn is_consuming(&mut self) 
-> Result<bool, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"isConsuming","()Z",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn set_consuming(&mut self,arg0: bool) 
-> Result<(), Box<dyn std::error::Error>>

{// -2
let val_0 = jni::objects::JValueGen::Bool(arg0.into());
let res = self.jni_ref().call_method(&self.jni_object(),"setConsuming","(Z)V",&[jni::objects::JValueGen::from(&val_0)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn block(&mut self) 
-> Result<crate::block::Block<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getBlock","()Lorg/bukkit/block/Block;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::block::Block::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn event_name(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getEventName","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn is_asynchronous(&mut self) 
-> Result<bool, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"isAsynchronous","()Z",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn wait(&mut self,arg0: std::option::Option<i64>,arg1: std::option::Option<i32>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_0 = jni::objects::JValueGen::Long(arg0.unwrap().into());
let val_1 = jni::objects::JValueGen::Int(arg1.unwrap().into());
let res = self.jni_ref().call_method(&self.jni_object(),"wait","(JI)V",&[jni::objects::JValueGen::from(&val_0),jni::objects::JValueGen::from(&val_1)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn equals(&mut self,arg0: jni::objects::JObject<'mc>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_0 = arg0;
let res = self.jni_ref().call_method(&self.jni_object(),"equals","(Ljava/lang/Object;)Z",&[jni::objects::JValueGen::from(&val_0)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn to_string(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"toString","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn hash_code(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"hashCode","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn class(&mut self) 
-> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getClass","()Ljava/lang/Class;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(unsafe {jni::objects::JClass::from_raw(res.as_jni().l)})}
	pub fn notify(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notify","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn notify_all(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notifyAll","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
}
impl<'mc> Into<crate::event::Cancellable<'mc>> for BrewingStandFuelEvent<'mc>{
   fn into(self) -> crate::event::Cancellable<'mc> {
       crate::event::Cancellable::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
impl<'mc> Into<crate::event::block::BlockEvent<'mc>> for BrewingStandFuelEvent<'mc>{
   fn into(self) -> crate::event::block::BlockEvent<'mc> {
       crate::event::block::BlockEvent::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
pub struct PrepareGrindstoneEvent<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for PrepareGrindstoneEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> PrepareGrindstoneEvent<'mc> {
pub fn from_raw(env: &blackboxmc_general::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>
) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate PrepareGrindstoneEvent from null object.")
    .into());
}
let (valid, name) = env.validate_name(&obj, "PrepareGrindstoneEvent")?;
if !valid {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a PrepareGrindstoneEvent object, got {}",
        name
    )
    .into())
} else {
    Ok(Self(env.clone(), obj
))
}
}
	pub fn new(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: impl Into<&'mc crate::inventory::InventoryView<'mc>>,arg1: impl Into<&'mc crate::inventory::ItemStack<'mc>>) 
-> Result<crate::event::inventory::PrepareGrindstoneEvent<'mc>, Box<dyn std::error::Error>>

{let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let val_1 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone())};
let cls = &jni.find_class("org/bukkit/event/inventory/PrepareGrindstoneEvent")?;
let res = jni.new_object(cls,
"(Lorg/bukkit/inventory/InventoryView;Lorg/bukkit/inventory/ItemStack;)V",&[jni::objects::JValueGen::from(&val_0),jni::objects::JValueGen::from(&val_1)])?;
crate::event::inventory::PrepareGrindstoneEvent::from_raw(&jni,res
)}
	pub fn handlers(&mut self) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getHandlers","()Lorg/bukkit/event/HandlerList;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::event::HandlerList::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn inventory(&mut self) 
-> Result<crate::inventory::Inventory<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getInventory","()Lorg/bukkit/inventory/Inventory;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::inventory::Inventory::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn handler_list(jni: blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let cls = &jni.find_class("org/bukkit/event/HandlerList")?;
let res = jni.call_static_method(cls,"getHandlerList",
"()Lorg/bukkit/event/HandlerList;",&[])?;
let mut obj = res.l()?;
crate::event::HandlerList::from_raw(&jni,obj
)}
	pub fn set_result(&mut self,arg0: impl Into<&'mc crate::inventory::ItemStack<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let res = self.jni_ref().call_method(&self.jni_object(),"setResult","(Lorg/bukkit/inventory/ItemStack;)V",&[jni::objects::JValueGen::from(&val_0)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn result(&mut self) 
-> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getResult","()Lorg/bukkit/inventory/ItemStack;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::inventory::ItemStack::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn viewers(&mut self) 
-> Result<blackboxmc_java::JavaList<'mc, orgHumanEntity>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getViewers","()Ljava/util/List;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
blackboxmc_java::JavaList::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn view(&mut self) 
-> Result<crate::inventory::InventoryView<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getView","()Lorg/bukkit/inventory/InventoryView;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::inventory::InventoryView::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn event_name(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getEventName","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn is_asynchronous(&mut self) 
-> Result<bool, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"isAsynchronous","()Z",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn wait(&mut self,arg0: std::option::Option<i64>,arg1: std::option::Option<i32>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_0 = jni::objects::JValueGen::Long(arg0.unwrap().into());
let val_1 = jni::objects::JValueGen::Int(arg1.unwrap().into());
let res = self.jni_ref().call_method(&self.jni_object(),"wait","(JI)V",&[jni::objects::JValueGen::from(&val_0),jni::objects::JValueGen::from(&val_1)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn equals(&mut self,arg0: jni::objects::JObject<'mc>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_0 = arg0;
let res = self.jni_ref().call_method(&self.jni_object(),"equals","(Ljava/lang/Object;)Z",&[jni::objects::JValueGen::from(&val_0)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn to_string(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"toString","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn hash_code(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"hashCode","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn class(&mut self) 
-> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getClass","()Ljava/lang/Class;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(unsafe {jni::objects::JClass::from_raw(res.as_jni().l)})}
	pub fn notify(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notify","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn notify_all(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notifyAll","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
}
impl<'mc> Into<crate::event::inventory::PrepareInventoryResultEvent<'mc>> for PrepareGrindstoneEvent<'mc>{
   fn into(self) -> crate::event::inventory::PrepareInventoryResultEvent<'mc> {
       crate::event::inventory::PrepareInventoryResultEvent::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
pub struct HopperInventorySearchEvent<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);
pub struct HopperInventorySearchEventContainerType<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for HopperInventorySearchEventContainerType<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> HopperInventorySearchEventContainerType<'mc> {
pub fn from_raw(env: &blackboxmc_general::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>
) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate HopperInventorySearchEventContainerType from null object.")
    .into());
}
let (valid, name) = env.validate_name(&obj, "HopperInventorySearchEventContainerType")?;
if !valid {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a HopperInventorySearchEventContainerType object, got {}",
        name
    )
    .into())
} else {
    Ok(Self(env.clone(), obj
))
}
}
	pub fn value_of_with_string(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: std::option::Option<jni::objects::JClass<'mc>>,arg1: std::option::Option<impl Into<&'mc String>>) 
-> Result<blackboxmc_java::JavaEnum<'mc>, Box<dyn std::error::Error>>

{let val_0 = arg0.unwrap();
let val_1 = jni::objects::JObject::from(jni.new_string(arg1.unwrap().into()).unwrap());
let cls = &jni.find_class("java/lang/Enum")?;
let res = jni.call_static_method(cls,"valueOf",
"(Ljava/lang/Class;Ljava/lang/String;)Ljava/lang/Enum;",&[jni::objects::JValueGen::from(&val_0),jni::objects::JValueGen::from(&val_1)])?;
let mut obj = res.l()?;
blackboxmc_java::JavaEnum::from_raw(&jni,obj
)}
	pub fn name(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"name","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn equals(&mut self,arg0: jni::objects::JObject<'mc>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_0 = arg0;
let res = self.jni_ref().call_method(&self.jni_object(),"equals","(Ljava/lang/Object;)Z",&[jni::objects::JValueGen::from(&val_0)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn to_string(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"toString","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn hash_code(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"hashCode","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn describe_constable(&mut self) 
-> Result<blackboxmc_java::JavaOptional<'mc, javaEnumEnumDesc<E>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"describeConstable","()Ljava/util/Optional;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
blackboxmc_java::JavaOptional::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn declaring_class(&mut self) 
-> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getDeclaringClass","()Ljava/lang/Class;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(unsafe {jni::objects::JClass::from_raw(res.as_jni().l)})}
	pub fn ordinal(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"ordinal","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn wait(&mut self,arg0: std::option::Option<i64>,arg1: std::option::Option<i32>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_0 = jni::objects::JValueGen::Long(arg0.unwrap().into());
let val_1 = jni::objects::JValueGen::Int(arg1.unwrap().into());
let res = self.jni_ref().call_method(&self.jni_object(),"wait","(JI)V",&[jni::objects::JValueGen::from(&val_0),jni::objects::JValueGen::from(&val_1)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn class(&mut self) 
-> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getClass","()Ljava/lang/Class;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(unsafe {jni::objects::JClass::from_raw(res.as_jni().l)})}
	pub fn notify(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notify","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn notify_all(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notifyAll","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
}
impl<'mc> blackboxmc_general::JNIRaw<'mc> for HopperInventorySearchEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> HopperInventorySearchEvent<'mc> {
pub fn from_raw(env: &blackboxmc_general::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>
) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate HopperInventorySearchEvent from null object.")
    .into());
}
let (valid, name) = env.validate_name(&obj, "HopperInventorySearchEvent")?;
if !valid {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a HopperInventorySearchEvent object, got {}",
        name
    )
    .into())
} else {
    Ok(Self(env.clone(), obj
))
}
}
	pub fn new(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: impl Into<&'mc crate::inventory::Inventory<'mc>>,arg1: impl Into<&'mc crate::event::inventory::HopperInventorySearchEventContainerType<'mc>>,arg2: impl Into<&'mc crate::block::Block<'mc>>,arg3: impl Into<&'mc crate::block::Block<'mc>>) 
-> Result<crate::event::inventory::HopperInventorySearchEvent<'mc>, Box<dyn std::error::Error>>

{let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let val_1 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone())};
let val_2 = unsafe { jni::objects::JObject::from_raw(arg2.into().jni_object().clone())};
let val_3 = unsafe { jni::objects::JObject::from_raw(arg3.into().jni_object().clone())};
let cls = &jni.find_class("org/bukkit/event/inventory/HopperInventorySearchEvent")?;
let res = jni.new_object(cls,
"(Lorg/bukkit/inventory/Inventory;Lorg/bukkit/event/inventory/HopperInventorySearchEvent$ContainerType;Lorg/bukkit/block/Block;Lorg/bukkit/block/Block;)V",&[jni::objects::JValueGen::from(&val_0),jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2),jni::objects::JValueGen::from(&val_3)])?;
crate::event::inventory::HopperInventorySearchEvent::from_raw(&jni,res
)}
	pub fn handlers(&mut self) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getHandlers","()Lorg/bukkit/event/HandlerList;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::event::HandlerList::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn inventory(&mut self) 
-> Result<crate::inventory::Inventory<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getInventory","()Lorg/bukkit/inventory/Inventory;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::inventory::Inventory::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn handler_list(jni: blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let cls = &jni.find_class("org/bukkit/event/HandlerList")?;
let res = jni.call_static_method(cls,"getHandlerList",
"()Lorg/bukkit/event/HandlerList;",&[])?;
let mut obj = res.l()?;
crate::event::HandlerList::from_raw(&jni,obj
)}
	pub fn set_inventory(&mut self,arg0: impl Into<&'mc crate::inventory::Inventory<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let res = self.jni_ref().call_method(&self.jni_object(),"setInventory","(Lorg/bukkit/inventory/Inventory;)V",&[jni::objects::JValueGen::from(&val_0)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn container_type(&mut self) 
-> Result<crate::event::inventory::HopperInventorySearchEventContainerType<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getContainerType","()Lorg/bukkit/event/inventory/HopperInventorySearchEvent$ContainerType;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::event::inventory::HopperInventorySearchEventContainerType::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn search_block(&mut self) 
-> Result<crate::block::Block<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getSearchBlock","()Lorg/bukkit/block/Block;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::block::Block::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn block(&mut self) 
-> Result<crate::block::Block<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getBlock","()Lorg/bukkit/block/Block;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::block::Block::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn event_name(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getEventName","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn is_asynchronous(&mut self) 
-> Result<bool, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"isAsynchronous","()Z",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn wait(&mut self,arg0: std::option::Option<i64>,arg1: std::option::Option<i32>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_0 = jni::objects::JValueGen::Long(arg0.unwrap().into());
let val_1 = jni::objects::JValueGen::Int(arg1.unwrap().into());
let res = self.jni_ref().call_method(&self.jni_object(),"wait","(JI)V",&[jni::objects::JValueGen::from(&val_0),jni::objects::JValueGen::from(&val_1)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn equals(&mut self,arg0: jni::objects::JObject<'mc>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_0 = arg0;
let res = self.jni_ref().call_method(&self.jni_object(),"equals","(Ljava/lang/Object;)Z",&[jni::objects::JValueGen::from(&val_0)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn to_string(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"toString","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn hash_code(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"hashCode","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn class(&mut self) 
-> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getClass","()Ljava/lang/Class;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(unsafe {jni::objects::JClass::from_raw(res.as_jni().l)})}
	pub fn notify(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notify","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn notify_all(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notifyAll","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
}
impl<'mc> Into<crate::event::block::BlockEvent<'mc>> for HopperInventorySearchEvent<'mc>{
   fn into(self) -> crate::event::block::BlockEvent<'mc> {
       crate::event::block::BlockEvent::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
pub struct CraftItemEvent<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for CraftItemEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> CraftItemEvent<'mc> {
pub fn from_raw(env: &blackboxmc_general::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>
) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate CraftItemEvent from null object.")
    .into());
}
let (valid, name) = env.validate_name(&obj, "CraftItemEvent")?;
if !valid {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a CraftItemEvent object, got {}",
        name
    )
    .into())
} else {
    Ok(Self(env.clone(), obj
))
}
}
	pub fn new_with_recipe(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: impl Into<&'mc crate::inventory::Recipe<'mc>>,arg1: impl Into<&'mc crate::inventory::InventoryView<'mc>>,arg2: impl Into<&'mc crate::event::inventory::InventoryTypeSlotType<'mc>>,arg3: i32,arg4: impl Into<&'mc crate::event::inventory::ClickType<'mc>>,arg5: std::option::Option<impl Into<&'mc crate::event::inventory::InventoryAction<'mc>>>,arg6: std::option::Option<i32>) 
-> Result<crate::event::inventory::CraftItemEvent<'mc>, Box<dyn std::error::Error>>

{let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let val_1 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone())};
let val_2 = unsafe { jni::objects::JObject::from_raw(arg2.into().jni_object().clone())};
let val_3 = jni::objects::JValueGen::Int(arg3.into());
let val_4 = unsafe { jni::objects::JObject::from_raw(arg4.into().jni_object().clone())};
let val_5 = unsafe { jni::objects::JObject::from_raw(arg5.unwrap().into().jni_object().clone())};
let val_6 = jni::objects::JValueGen::Int(arg6.unwrap().into());
let cls = &jni.find_class("org/bukkit/event/inventory/CraftItemEvent")?;
let res = jni.new_object(cls,
"(Lorg/bukkit/inventory/Recipe;Lorg/bukkit/inventory/InventoryView;Lorg/bukkit/event/inventory/InventoryType$SlotType;ILorg/bukkit/event/inventory/ClickType;Lorg/bukkit/event/inventory/InventoryAction;I)V",&[jni::objects::JValueGen::from(&val_0),jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2),jni::objects::JValueGen::from(&val_3),jni::objects::JValueGen::from(&val_4),jni::objects::JValueGen::from(&val_5),jni::objects::JValueGen::from(&val_6)])?;
crate::event::inventory::CraftItemEvent::from_raw(&jni,res
)}
	pub fn inventory(&mut self) 
-> Result<crate::inventory::Inventory<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getInventory","()Lorg/bukkit/inventory/Inventory;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::inventory::Inventory::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn recipe(&mut self) 
-> Result<crate::inventory::Recipe<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getRecipe","()Lorg/bukkit/inventory/Recipe;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::inventory::Recipe::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn handlers(&mut self) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getHandlers","()Lorg/bukkit/event/HandlerList;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::event::HandlerList::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn slot(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getSlot","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
#[deprecated]
	pub fn set_cursor(&mut self,arg0: impl Into<&'mc crate::inventory::ItemStack<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let res = self.jni_ref().call_method(&self.jni_object(),"setCursor","(Lorg/bukkit/inventory/ItemStack;)V",&[jni::objects::JValueGen::from(&val_0)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn cursor(&mut self) 
-> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getCursor","()Lorg/bukkit/inventory/ItemStack;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::inventory::ItemStack::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn slot_type(&mut self) 
-> Result<crate::event::inventory::InventoryTypeSlotType<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getSlotType","()Lorg/bukkit/event/inventory/InventoryType$SlotType;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::event::inventory::InventoryTypeSlotType::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn handler_list(jni: blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let cls = &jni.find_class("org/bukkit/event/HandlerList")?;
let res = jni.call_static_method(cls,"getHandlerList",
"()Lorg/bukkit/event/HandlerList;",&[])?;
let mut obj = res.l()?;
crate::event::HandlerList::from_raw(&jni,obj
)}
	pub fn action(&mut self) 
-> Result<crate::event::inventory::InventoryAction<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getAction","()Lorg/bukkit/event/inventory/InventoryAction;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };let variant = self.0.call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;let variant_str = self.0    .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?    .to_string_lossy()    .to_string();
crate::event::inventory::InventoryAction::from_raw(&self.jni_ref(),raw_obj
, crate::event::inventory::InventoryAction::from_string(variant_str).unwrap()
)}
	pub fn current_item(&mut self) 
-> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getCurrentItem","()Lorg/bukkit/inventory/ItemStack;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::inventory::ItemStack::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn is_right_click(&mut self) 
-> Result<bool, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"isRightClick","()Z",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn is_left_click(&mut self) 
-> Result<bool, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"isLeftClick","()Z",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn is_shift_click(&mut self) 
-> Result<bool, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"isShiftClick","()Z",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn set_current_item(&mut self,arg0: impl Into<&'mc crate::inventory::ItemStack<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let res = self.jni_ref().call_method(&self.jni_object(),"setCurrentItem","(Lorg/bukkit/inventory/ItemStack;)V",&[jni::objects::JValueGen::from(&val_0)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn clicked_inventory(&mut self) 
-> Result<crate::inventory::Inventory<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getClickedInventory","()Lorg/bukkit/inventory/Inventory;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::inventory::Inventory::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn raw_slot(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getRawSlot","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn hotbar_button(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getHotbarButton","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn click(&mut self) 
-> Result<crate::event::inventory::ClickType<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getClick","()Lorg/bukkit/event/inventory/ClickType;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };let variant = self.0.call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;let variant_str = self.0    .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?    .to_string_lossy()    .to_string();
crate::event::inventory::ClickType::from_raw(&self.jni_ref(),raw_obj
, crate::event::inventory::ClickType::from_string(variant_str).unwrap()
)}
	pub fn set_result(&mut self,arg0: impl Into<&'mc crate::event::EventResult<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let res = self.jni_ref().call_method(&self.jni_object(),"setResult","(Lorg/bukkit/event/Event$Result;)V",&[jni::objects::JValueGen::from(&val_0)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn result(&mut self) 
-> Result<crate::event::EventResult<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getResult","()Lorg/bukkit/event/Event$Result;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::event::EventResult::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn is_cancelled(&mut self) 
-> Result<bool, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"isCancelled","()Z",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn set_cancelled(&mut self,arg0: bool) 
-> Result<(), Box<dyn std::error::Error>>

{// -2
let val_0 = jni::objects::JValueGen::Bool(arg0.into());
let res = self.jni_ref().call_method(&self.jni_object(),"setCancelled","(Z)V",&[jni::objects::JValueGen::from(&val_0)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn who_clicked(&mut self) 
-> Result<crate::entity::HumanEntity<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getWhoClicked","()Lorg/bukkit/entity/HumanEntity;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::entity::HumanEntity::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn viewers(&mut self) 
-> Result<blackboxmc_java::JavaList<'mc, orgHumanEntity>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getViewers","()Ljava/util/List;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
blackboxmc_java::JavaList::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn view(&mut self) 
-> Result<crate::inventory::InventoryView<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getView","()Lorg/bukkit/inventory/InventoryView;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::inventory::InventoryView::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn event_name(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getEventName","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn is_asynchronous(&mut self) 
-> Result<bool, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"isAsynchronous","()Z",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn wait(&mut self,arg0: std::option::Option<i64>,arg1: std::option::Option<i32>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_0 = jni::objects::JValueGen::Long(arg0.unwrap().into());
let val_1 = jni::objects::JValueGen::Int(arg1.unwrap().into());
let res = self.jni_ref().call_method(&self.jni_object(),"wait","(JI)V",&[jni::objects::JValueGen::from(&val_0),jni::objects::JValueGen::from(&val_1)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn equals(&mut self,arg0: jni::objects::JObject<'mc>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_0 = arg0;
let res = self.jni_ref().call_method(&self.jni_object(),"equals","(Ljava/lang/Object;)Z",&[jni::objects::JValueGen::from(&val_0)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn to_string(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"toString","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn hash_code(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"hashCode","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn class(&mut self) 
-> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getClass","()Ljava/lang/Class;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(unsafe {jni::objects::JClass::from_raw(res.as_jni().l)})}
	pub fn notify(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notify","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn notify_all(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notifyAll","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
}
impl<'mc> Into<crate::event::inventory::InventoryClickEvent<'mc>> for CraftItemEvent<'mc>{
   fn into(self) -> crate::event::inventory::InventoryClickEvent<'mc> {
       crate::event::inventory::InventoryClickEvent::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
pub struct FurnaceStartSmeltEvent<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for FurnaceStartSmeltEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> FurnaceStartSmeltEvent<'mc> {
pub fn from_raw(env: &blackboxmc_general::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>
) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate FurnaceStartSmeltEvent from null object.")
    .into());
}
let (valid, name) = env.validate_name(&obj, "FurnaceStartSmeltEvent")?;
if !valid {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a FurnaceStartSmeltEvent object, got {}",
        name
    )
    .into())
} else {
    Ok(Self(env.clone(), obj
))
}
}
	pub fn new(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: impl Into<&'mc crate::block::Block<'mc>>,arg1: impl Into<&'mc crate::inventory::ItemStack<'mc>>,arg2: impl Into<&'mc crate::inventory::CookingRecipe<'mc, dyn JNIRaw<'mc>>>) 
-> Result<crate::event::inventory::FurnaceStartSmeltEvent<'mc>, Box<dyn std::error::Error>>

{let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let val_1 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone())};
let val_2 = unsafe { jni::objects::JObject::from_raw(arg2.into().jni_object().clone())};
let cls = &jni.find_class("org/bukkit/event/inventory/FurnaceStartSmeltEvent")?;
let res = jni.new_object(cls,
"(Lorg/bukkit/block/Block;Lorg/bukkit/inventory/ItemStack;Lorg/bukkit/inventory/CookingRecipe;)V",&[jni::objects::JValueGen::from(&val_0),jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)])?;
crate::event::inventory::FurnaceStartSmeltEvent::from_raw(&jni,res
)}
	pub fn handlers(&mut self) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getHandlers","()Lorg/bukkit/event/HandlerList;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::event::HandlerList::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn recipe(&mut self) 
-> Result<crate::inventory::CookingRecipe<'mc, dyn JNIRaw<'mc>>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getRecipe","()Lorg/bukkit/inventory/CookingRecipe;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::inventory::CookingRecipe::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn handler_list(jni: blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let cls = &jni.find_class("org/bukkit/event/HandlerList")?;
let res = jni.call_static_method(cls,"getHandlerList",
"()Lorg/bukkit/event/HandlerList;",&[])?;
let mut obj = res.l()?;
crate::event::HandlerList::from_raw(&jni,obj
)}
	pub fn total_cook_time(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getTotalCookTime","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn set_total_cook_time(&mut self,arg0: i32) 
-> Result<(), Box<dyn std::error::Error>>

{let val_0 = jni::objects::JValueGen::Int(arg0.into());
let res = self.jni_ref().call_method(&self.jni_object(),"setTotalCookTime","(I)V",&[jni::objects::JValueGen::from(&val_0)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn source(&mut self) 
-> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getSource","()Lorg/bukkit/inventory/ItemStack;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::inventory::ItemStack::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn block(&mut self) 
-> Result<crate::block::Block<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getBlock","()Lorg/bukkit/block/Block;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::block::Block::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn event_name(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getEventName","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn is_asynchronous(&mut self) 
-> Result<bool, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"isAsynchronous","()Z",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn wait(&mut self,arg0: std::option::Option<i64>,arg1: std::option::Option<i32>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_0 = jni::objects::JValueGen::Long(arg0.unwrap().into());
let val_1 = jni::objects::JValueGen::Int(arg1.unwrap().into());
let res = self.jni_ref().call_method(&self.jni_object(),"wait","(JI)V",&[jni::objects::JValueGen::from(&val_0),jni::objects::JValueGen::from(&val_1)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn equals(&mut self,arg0: jni::objects::JObject<'mc>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_0 = arg0;
let res = self.jni_ref().call_method(&self.jni_object(),"equals","(Ljava/lang/Object;)Z",&[jni::objects::JValueGen::from(&val_0)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn to_string(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"toString","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn hash_code(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"hashCode","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn class(&mut self) 
-> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getClass","()Ljava/lang/Class;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(unsafe {jni::objects::JClass::from_raw(res.as_jni().l)})}
	pub fn notify(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notify","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn notify_all(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notifyAll","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
}
impl<'mc> Into<crate::event::block::InventoryBlockStartEvent<'mc>> for FurnaceStartSmeltEvent<'mc>{
   fn into(self) -> crate::event::block::InventoryBlockStartEvent<'mc> {
       crate::event::block::InventoryBlockStartEvent::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
pub struct InventoryPickupItemEvent<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for InventoryPickupItemEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> InventoryPickupItemEvent<'mc> {
pub fn from_raw(env: &blackboxmc_general::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>
) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate InventoryPickupItemEvent from null object.")
    .into());
}
let (valid, name) = env.validate_name(&obj, "InventoryPickupItemEvent")?;
if !valid {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a InventoryPickupItemEvent object, got {}",
        name
    )
    .into())
} else {
    Ok(Self(env.clone(), obj
))
}
}
	pub fn new(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: impl Into<&'mc crate::inventory::Inventory<'mc>>,arg1: impl Into<&'mc crate::entity::Item<'mc>>) 
-> Result<crate::event::inventory::InventoryPickupItemEvent<'mc>, Box<dyn std::error::Error>>

{let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let val_1 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone())};
let cls = &jni.find_class("org/bukkit/event/inventory/InventoryPickupItemEvent")?;
let res = jni.new_object(cls,
"(Lorg/bukkit/inventory/Inventory;Lorg/bukkit/entity/Item;)V",&[jni::objects::JValueGen::from(&val_0),jni::objects::JValueGen::from(&val_1)])?;
crate::event::inventory::InventoryPickupItemEvent::from_raw(&jni,res
)}
	pub fn handlers(&mut self) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getHandlers","()Lorg/bukkit/event/HandlerList;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::event::HandlerList::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn item(&mut self) 
-> Result<crate::entity::Item<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getItem","()Lorg/bukkit/entity/Item;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::entity::Item::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn is_cancelled(&mut self) 
-> Result<bool, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"isCancelled","()Z",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn inventory(&mut self) 
-> Result<crate::inventory::Inventory<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getInventory","()Lorg/bukkit/inventory/Inventory;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::inventory::Inventory::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn set_cancelled(&mut self,arg0: bool) 
-> Result<(), Box<dyn std::error::Error>>

{// -2
let val_0 = jni::objects::JValueGen::Bool(arg0.into());
let res = self.jni_ref().call_method(&self.jni_object(),"setCancelled","(Z)V",&[jni::objects::JValueGen::from(&val_0)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn handler_list(jni: blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let cls = &jni.find_class("org/bukkit/event/HandlerList")?;
let res = jni.call_static_method(cls,"getHandlerList",
"()Lorg/bukkit/event/HandlerList;",&[])?;
let mut obj = res.l()?;
crate::event::HandlerList::from_raw(&jni,obj
)}
	pub fn event_name(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getEventName","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn is_asynchronous(&mut self) 
-> Result<bool, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"isAsynchronous","()Z",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn wait(&mut self,arg0: std::option::Option<i64>,arg1: std::option::Option<i32>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_0 = jni::objects::JValueGen::Long(arg0.unwrap().into());
let val_1 = jni::objects::JValueGen::Int(arg1.unwrap().into());
let res = self.jni_ref().call_method(&self.jni_object(),"wait","(JI)V",&[jni::objects::JValueGen::from(&val_0),jni::objects::JValueGen::from(&val_1)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn equals(&mut self,arg0: jni::objects::JObject<'mc>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_0 = arg0;
let res = self.jni_ref().call_method(&self.jni_object(),"equals","(Ljava/lang/Object;)Z",&[jni::objects::JValueGen::from(&val_0)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn to_string(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"toString","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn hash_code(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"hashCode","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn class(&mut self) 
-> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getClass","()Ljava/lang/Class;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(unsafe {jni::objects::JClass::from_raw(res.as_jni().l)})}
	pub fn notify(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notify","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn notify_all(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notifyAll","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
}
impl<'mc> Into<crate::event::Cancellable<'mc>> for InventoryPickupItemEvent<'mc>{
   fn into(self) -> crate::event::Cancellable<'mc> {
       crate::event::Cancellable::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
impl<'mc> Into<crate::event::Event<'mc>> for InventoryPickupItemEvent<'mc>{
   fn into(self) -> crate::event::Event<'mc> {
       crate::event::Event::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
pub struct InventoryInteractEvent<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for InventoryInteractEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> InventoryInteractEvent<'mc> {
pub fn from_raw(env: &blackboxmc_general::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>
) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate InventoryInteractEvent from null object.")
    .into());
}
let (valid, name) = env.validate_name(&obj, "InventoryInteractEvent")?;
if !valid {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a InventoryInteractEvent object, got {}",
        name
    )
    .into())
} else {
    Ok(Self(env.clone(), obj
))
}
}
	pub fn new(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: impl Into<&'mc crate::inventory::InventoryView<'mc>>) 
-> Result<crate::event::inventory::InventoryInteractEvent<'mc>, Box<dyn std::error::Error>>

{let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let cls = &jni.find_class("org/bukkit/event/inventory/InventoryInteractEvent")?;
let res = jni.new_object(cls,
"(Lorg/bukkit/inventory/InventoryView;)V",&[jni::objects::JValueGen::from(&val_0)])?;
crate::event::inventory::InventoryInteractEvent::from_raw(&jni,res
)}
	pub fn set_result(&mut self,arg0: impl Into<&'mc crate::event::EventResult<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let res = self.jni_ref().call_method(&self.jni_object(),"setResult","(Lorg/bukkit/event/Event$Result;)V",&[jni::objects::JValueGen::from(&val_0)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn result(&mut self) 
-> Result<crate::event::EventResult<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getResult","()Lorg/bukkit/event/Event$Result;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::event::EventResult::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn is_cancelled(&mut self) 
-> Result<bool, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"isCancelled","()Z",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn set_cancelled(&mut self,arg0: bool) 
-> Result<(), Box<dyn std::error::Error>>

{// -2
let val_0 = jni::objects::JValueGen::Bool(arg0.into());
let res = self.jni_ref().call_method(&self.jni_object(),"setCancelled","(Z)V",&[jni::objects::JValueGen::from(&val_0)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn who_clicked(&mut self) 
-> Result<crate::entity::HumanEntity<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getWhoClicked","()Lorg/bukkit/entity/HumanEntity;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::entity::HumanEntity::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn handlers(&mut self) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getHandlers","()Lorg/bukkit/event/HandlerList;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::event::HandlerList::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn inventory(&mut self) 
-> Result<crate::inventory::Inventory<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getInventory","()Lorg/bukkit/inventory/Inventory;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::inventory::Inventory::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn viewers(&mut self) 
-> Result<blackboxmc_java::JavaList<'mc, orgHumanEntity>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getViewers","()Ljava/util/List;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
blackboxmc_java::JavaList::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn handler_list(jni: blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let cls = &jni.find_class("org/bukkit/event/HandlerList")?;
let res = jni.call_static_method(cls,"getHandlerList",
"()Lorg/bukkit/event/HandlerList;",&[])?;
let mut obj = res.l()?;
crate::event::HandlerList::from_raw(&jni,obj
)}
	pub fn view(&mut self) 
-> Result<crate::inventory::InventoryView<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getView","()Lorg/bukkit/inventory/InventoryView;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::inventory::InventoryView::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn event_name(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getEventName","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn is_asynchronous(&mut self) 
-> Result<bool, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"isAsynchronous","()Z",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn wait(&mut self,arg0: std::option::Option<i64>,arg1: std::option::Option<i32>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_0 = jni::objects::JValueGen::Long(arg0.unwrap().into());
let val_1 = jni::objects::JValueGen::Int(arg1.unwrap().into());
let res = self.jni_ref().call_method(&self.jni_object(),"wait","(JI)V",&[jni::objects::JValueGen::from(&val_0),jni::objects::JValueGen::from(&val_1)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn equals(&mut self,arg0: jni::objects::JObject<'mc>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_0 = arg0;
let res = self.jni_ref().call_method(&self.jni_object(),"equals","(Ljava/lang/Object;)Z",&[jni::objects::JValueGen::from(&val_0)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn to_string(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"toString","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn hash_code(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"hashCode","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn class(&mut self) 
-> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getClass","()Ljava/lang/Class;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(unsafe {jni::objects::JClass::from_raw(res.as_jni().l)})}
	pub fn notify(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notify","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn notify_all(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notifyAll","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
}
impl<'mc> Into<crate::event::Cancellable<'mc>> for InventoryInteractEvent<'mc>{
   fn into(self) -> crate::event::Cancellable<'mc> {
       crate::event::Cancellable::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
impl<'mc> Into<crate::event::inventory::InventoryEvent<'mc>> for InventoryInteractEvent<'mc>{
   fn into(self) -> crate::event::inventory::InventoryEvent<'mc> {
       crate::event::inventory::InventoryEvent::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
pub struct InventoryEvent<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for InventoryEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> InventoryEvent<'mc> {
pub fn from_raw(env: &blackboxmc_general::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>
) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate InventoryEvent from null object.")
    .into());
}
let (valid, name) = env.validate_name(&obj, "InventoryEvent")?;
if !valid {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a InventoryEvent object, got {}",
        name
    )
    .into())
} else {
    Ok(Self(env.clone(), obj
))
}
}
	pub fn new(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: impl Into<&'mc crate::inventory::InventoryView<'mc>>) 
-> Result<crate::event::inventory::InventoryEvent<'mc>, Box<dyn std::error::Error>>

{let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let cls = &jni.find_class("org/bukkit/event/inventory/InventoryEvent")?;
let res = jni.new_object(cls,
"(Lorg/bukkit/inventory/InventoryView;)V",&[jni::objects::JValueGen::from(&val_0)])?;
crate::event::inventory::InventoryEvent::from_raw(&jni,res
)}
	pub fn handlers(&mut self) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getHandlers","()Lorg/bukkit/event/HandlerList;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::event::HandlerList::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn inventory(&mut self) 
-> Result<crate::inventory::Inventory<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getInventory","()Lorg/bukkit/inventory/Inventory;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::inventory::Inventory::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn viewers(&mut self) 
-> Result<blackboxmc_java::JavaList<'mc, orgHumanEntity>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getViewers","()Ljava/util/List;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
blackboxmc_java::JavaList::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn handler_list(jni: blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let cls = &jni.find_class("org/bukkit/event/HandlerList")?;
let res = jni.call_static_method(cls,"getHandlerList",
"()Lorg/bukkit/event/HandlerList;",&[])?;
let mut obj = res.l()?;
crate::event::HandlerList::from_raw(&jni,obj
)}
	pub fn view(&mut self) 
-> Result<crate::inventory::InventoryView<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getView","()Lorg/bukkit/inventory/InventoryView;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::inventory::InventoryView::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn event_name(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getEventName","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn is_asynchronous(&mut self) 
-> Result<bool, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"isAsynchronous","()Z",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn wait(&mut self,arg0: std::option::Option<i64>,arg1: std::option::Option<i32>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_0 = jni::objects::JValueGen::Long(arg0.unwrap().into());
let val_1 = jni::objects::JValueGen::Int(arg1.unwrap().into());
let res = self.jni_ref().call_method(&self.jni_object(),"wait","(JI)V",&[jni::objects::JValueGen::from(&val_0),jni::objects::JValueGen::from(&val_1)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn equals(&mut self,arg0: jni::objects::JObject<'mc>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_0 = arg0;
let res = self.jni_ref().call_method(&self.jni_object(),"equals","(Ljava/lang/Object;)Z",&[jni::objects::JValueGen::from(&val_0)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn to_string(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"toString","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn hash_code(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"hashCode","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn class(&mut self) 
-> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getClass","()Ljava/lang/Class;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(unsafe {jni::objects::JClass::from_raw(res.as_jni().l)})}
	pub fn notify(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notify","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn notify_all(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notifyAll","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
}
impl<'mc> Into<crate::event::Event<'mc>> for InventoryEvent<'mc>{
   fn into(self) -> crate::event::Event<'mc> {
       crate::event::Event::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
pub struct InventoryCreativeEvent<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for InventoryCreativeEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> InventoryCreativeEvent<'mc> {
pub fn from_raw(env: &blackboxmc_general::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>
) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate InventoryCreativeEvent from null object.")
    .into());
}
let (valid, name) = env.validate_name(&obj, "InventoryCreativeEvent")?;
if !valid {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a InventoryCreativeEvent object, got {}",
        name
    )
    .into())
} else {
    Ok(Self(env.clone(), obj
))
}
}
	pub fn new(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: impl Into<&'mc crate::inventory::InventoryView<'mc>>,arg1: impl Into<&'mc crate::event::inventory::InventoryTypeSlotType<'mc>>,arg2: i32,arg3: impl Into<&'mc crate::inventory::ItemStack<'mc>>) 
-> Result<crate::event::inventory::InventoryCreativeEvent<'mc>, Box<dyn std::error::Error>>

{let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let val_1 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone())};
let val_2 = jni::objects::JValueGen::Int(arg2.into());
let val_3 = unsafe { jni::objects::JObject::from_raw(arg3.into().jni_object().clone())};
let cls = &jni.find_class("org/bukkit/event/inventory/InventoryCreativeEvent")?;
let res = jni.new_object(cls,
"(Lorg/bukkit/inventory/InventoryView;Lorg/bukkit/event/inventory/InventoryType$SlotType;ILorg/bukkit/inventory/ItemStack;)V",&[jni::objects::JValueGen::from(&val_0),jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2),jni::objects::JValueGen::from(&val_3)])?;
crate::event::inventory::InventoryCreativeEvent::from_raw(&jni,res
)}
	pub fn set_cursor(&mut self,arg0: impl Into<&'mc crate::inventory::ItemStack<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let res = self.jni_ref().call_method(&self.jni_object(),"setCursor","(Lorg/bukkit/inventory/ItemStack;)V",&[jni::objects::JValueGen::from(&val_0)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn cursor(&mut self) 
-> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getCursor","()Lorg/bukkit/inventory/ItemStack;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::inventory::ItemStack::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn handlers(&mut self) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getHandlers","()Lorg/bukkit/event/HandlerList;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::event::HandlerList::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn slot(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getSlot","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn slot_type(&mut self) 
-> Result<crate::event::inventory::InventoryTypeSlotType<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getSlotType","()Lorg/bukkit/event/inventory/InventoryType$SlotType;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::event::inventory::InventoryTypeSlotType::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn handler_list(jni: blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let cls = &jni.find_class("org/bukkit/event/HandlerList")?;
let res = jni.call_static_method(cls,"getHandlerList",
"()Lorg/bukkit/event/HandlerList;",&[])?;
let mut obj = res.l()?;
crate::event::HandlerList::from_raw(&jni,obj
)}
	pub fn action(&mut self) 
-> Result<crate::event::inventory::InventoryAction<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getAction","()Lorg/bukkit/event/inventory/InventoryAction;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };let variant = self.0.call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;let variant_str = self.0    .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?    .to_string_lossy()    .to_string();
crate::event::inventory::InventoryAction::from_raw(&self.jni_ref(),raw_obj
, crate::event::inventory::InventoryAction::from_string(variant_str).unwrap()
)}
	pub fn current_item(&mut self) 
-> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getCurrentItem","()Lorg/bukkit/inventory/ItemStack;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::inventory::ItemStack::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn is_right_click(&mut self) 
-> Result<bool, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"isRightClick","()Z",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn is_left_click(&mut self) 
-> Result<bool, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"isLeftClick","()Z",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn is_shift_click(&mut self) 
-> Result<bool, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"isShiftClick","()Z",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn set_current_item(&mut self,arg0: impl Into<&'mc crate::inventory::ItemStack<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let res = self.jni_ref().call_method(&self.jni_object(),"setCurrentItem","(Lorg/bukkit/inventory/ItemStack;)V",&[jni::objects::JValueGen::from(&val_0)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn clicked_inventory(&mut self) 
-> Result<crate::inventory::Inventory<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getClickedInventory","()Lorg/bukkit/inventory/Inventory;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::inventory::Inventory::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn raw_slot(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getRawSlot","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn hotbar_button(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getHotbarButton","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn click(&mut self) 
-> Result<crate::event::inventory::ClickType<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getClick","()Lorg/bukkit/event/inventory/ClickType;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };let variant = self.0.call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;let variant_str = self.0    .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?    .to_string_lossy()    .to_string();
crate::event::inventory::ClickType::from_raw(&self.jni_ref(),raw_obj
, crate::event::inventory::ClickType::from_string(variant_str).unwrap()
)}
	pub fn set_result(&mut self,arg0: impl Into<&'mc crate::event::EventResult<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let res = self.jni_ref().call_method(&self.jni_object(),"setResult","(Lorg/bukkit/event/Event$Result;)V",&[jni::objects::JValueGen::from(&val_0)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn result(&mut self) 
-> Result<crate::event::EventResult<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getResult","()Lorg/bukkit/event/Event$Result;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::event::EventResult::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn is_cancelled(&mut self) 
-> Result<bool, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"isCancelled","()Z",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn set_cancelled(&mut self,arg0: bool) 
-> Result<(), Box<dyn std::error::Error>>

{// -2
let val_0 = jni::objects::JValueGen::Bool(arg0.into());
let res = self.jni_ref().call_method(&self.jni_object(),"setCancelled","(Z)V",&[jni::objects::JValueGen::from(&val_0)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn who_clicked(&mut self) 
-> Result<crate::entity::HumanEntity<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getWhoClicked","()Lorg/bukkit/entity/HumanEntity;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::entity::HumanEntity::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn inventory(&mut self) 
-> Result<crate::inventory::Inventory<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getInventory","()Lorg/bukkit/inventory/Inventory;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::inventory::Inventory::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn viewers(&mut self) 
-> Result<blackboxmc_java::JavaList<'mc, orgHumanEntity>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getViewers","()Ljava/util/List;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
blackboxmc_java::JavaList::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn view(&mut self) 
-> Result<crate::inventory::InventoryView<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getView","()Lorg/bukkit/inventory/InventoryView;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::inventory::InventoryView::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn event_name(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getEventName","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn is_asynchronous(&mut self) 
-> Result<bool, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"isAsynchronous","()Z",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn wait(&mut self,arg0: std::option::Option<i64>,arg1: std::option::Option<i32>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_0 = jni::objects::JValueGen::Long(arg0.unwrap().into());
let val_1 = jni::objects::JValueGen::Int(arg1.unwrap().into());
let res = self.jni_ref().call_method(&self.jni_object(),"wait","(JI)V",&[jni::objects::JValueGen::from(&val_0),jni::objects::JValueGen::from(&val_1)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn equals(&mut self,arg0: jni::objects::JObject<'mc>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_0 = arg0;
let res = self.jni_ref().call_method(&self.jni_object(),"equals","(Ljava/lang/Object;)Z",&[jni::objects::JValueGen::from(&val_0)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn to_string(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"toString","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn hash_code(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"hashCode","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn class(&mut self) 
-> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getClass","()Ljava/lang/Class;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(unsafe {jni::objects::JClass::from_raw(res.as_jni().l)})}
	pub fn notify(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notify","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn notify_all(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notifyAll","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
}
impl<'mc> Into<crate::event::inventory::InventoryClickEvent<'mc>> for InventoryCreativeEvent<'mc>{
   fn into(self) -> crate::event::inventory::InventoryClickEvent<'mc> {
       crate::event::inventory::InventoryClickEvent::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
pub struct FurnaceExtractEvent<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for FurnaceExtractEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> FurnaceExtractEvent<'mc> {
pub fn from_raw(env: &blackboxmc_general::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>
) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate FurnaceExtractEvent from null object.")
    .into());
}
let (valid, name) = env.validate_name(&obj, "FurnaceExtractEvent")?;
if !valid {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a FurnaceExtractEvent object, got {}",
        name
    )
    .into())
} else {
    Ok(Self(env.clone(), obj
))
}
}
	pub fn new(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: impl Into<&'mc crate::entity::Player<'mc>>,arg1: impl Into<&'mc crate::block::Block<'mc>>,arg2: impl Into<&'mc crate::Material<'mc>>,arg3: i32,arg4: i32) 
-> Result<crate::event::inventory::FurnaceExtractEvent<'mc>, Box<dyn std::error::Error>>

{let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let val_1 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone())};
let val_2 = unsafe { jni::objects::JObject::from_raw(arg2.into().jni_object().clone())};
let val_3 = jni::objects::JValueGen::Int(arg3.into());
let val_4 = jni::objects::JValueGen::Int(arg4.into());
let cls = &jni.find_class("org/bukkit/event/inventory/FurnaceExtractEvent")?;
let res = jni.new_object(cls,
"(Lorg/bukkit/entity/Player;Lorg/bukkit/block/Block;Lorg/bukkit/Material;II)V",&[jni::objects::JValueGen::from(&val_0),jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2),jni::objects::JValueGen::from(&val_3),jni::objects::JValueGen::from(&val_4)])?;
crate::event::inventory::FurnaceExtractEvent::from_raw(&jni,res
)}
	pub fn player(&mut self) 
-> Result<crate::entity::Player<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getPlayer","()Lorg/bukkit/entity/Player;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::entity::Player::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn item_type(&mut self) 
-> Result<crate::Material<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getItemType","()Lorg/bukkit/Material;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };let variant = self.0.call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;let variant_str = self.0    .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?    .to_string_lossy()    .to_string();
crate::Material::from_raw(&self.jni_ref(),raw_obj
, crate::Material::from_string(variant_str).unwrap()
)}
	pub fn item_amount(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getItemAmount","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn handlers(&mut self) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getHandlers","()Lorg/bukkit/event/HandlerList;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::event::HandlerList::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn handler_list(jni: blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let cls = &jni.find_class("org/bukkit/event/HandlerList")?;
let res = jni.call_static_method(cls,"getHandlerList",
"()Lorg/bukkit/event/HandlerList;",&[])?;
let mut obj = res.l()?;
crate::event::HandlerList::from_raw(&jni,obj
)}
	pub fn exp_to_drop(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getExpToDrop","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn set_exp_to_drop(&mut self,arg0: i32) 
-> Result<(), Box<dyn std::error::Error>>

{let val_0 = jni::objects::JValueGen::Int(arg0.into());
let res = self.jni_ref().call_method(&self.jni_object(),"setExpToDrop","(I)V",&[jni::objects::JValueGen::from(&val_0)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn block(&mut self) 
-> Result<crate::block::Block<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getBlock","()Lorg/bukkit/block/Block;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::block::Block::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn event_name(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getEventName","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn is_asynchronous(&mut self) 
-> Result<bool, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"isAsynchronous","()Z",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn wait(&mut self,arg0: std::option::Option<i64>,arg1: std::option::Option<i32>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_0 = jni::objects::JValueGen::Long(arg0.unwrap().into());
let val_1 = jni::objects::JValueGen::Int(arg1.unwrap().into());
let res = self.jni_ref().call_method(&self.jni_object(),"wait","(JI)V",&[jni::objects::JValueGen::from(&val_0),jni::objects::JValueGen::from(&val_1)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn equals(&mut self,arg0: jni::objects::JObject<'mc>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_0 = arg0;
let res = self.jni_ref().call_method(&self.jni_object(),"equals","(Ljava/lang/Object;)Z",&[jni::objects::JValueGen::from(&val_0)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn to_string(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"toString","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn hash_code(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"hashCode","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn class(&mut self) 
-> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getClass","()Ljava/lang/Class;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(unsafe {jni::objects::JClass::from_raw(res.as_jni().l)})}
	pub fn notify(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notify","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn notify_all(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notifyAll","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
}
impl<'mc> Into<crate::event::block::BlockExpEvent<'mc>> for FurnaceExtractEvent<'mc>{
   fn into(self) -> crate::event::block::BlockExpEvent<'mc> {
       crate::event::block::BlockExpEvent::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
pub enum InventoryTypeEnum {
	Chest,
	Dispenser,
	Dropper,
	Furnace,
	Workbench,
	Crafting,
	Enchanting,
	Brewing,
	Player,
	Creative,
	Merchant,
	EnderChest,
	Anvil,
	Smithing,
	Beacon,
	Hopper,
	ShulkerBox,
	Barrel,
	BlastFurnace,
	Lectern,
	Smoker,
	Loom,
	Cartography,
	Grindstone,
	Stonecutter,
	Composter,
	ChiseledBookshelf,
	Jukebox,
#[deprecated]
	SmithingNew,
}
impl std::fmt::Display for InventoryTypeEnum {
   fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
       match &self {
           InventoryTypeEnum::Chest => f.write_str("CHEST"),
           InventoryTypeEnum::Dispenser => f.write_str("DISPENSER"),
           InventoryTypeEnum::Dropper => f.write_str("DROPPER"),
           InventoryTypeEnum::Furnace => f.write_str("FURNACE"),
           InventoryTypeEnum::Workbench => f.write_str("WORKBENCH"),
           InventoryTypeEnum::Crafting => f.write_str("CRAFTING"),
           InventoryTypeEnum::Enchanting => f.write_str("ENCHANTING"),
           InventoryTypeEnum::Brewing => f.write_str("BREWING"),
           InventoryTypeEnum::Player => f.write_str("PLAYER"),
           InventoryTypeEnum::Creative => f.write_str("CREATIVE"),
           InventoryTypeEnum::Merchant => f.write_str("MERCHANT"),
           InventoryTypeEnum::EnderChest => f.write_str("ENDER_CHEST"),
           InventoryTypeEnum::Anvil => f.write_str("ANVIL"),
           InventoryTypeEnum::Smithing => f.write_str("SMITHING"),
           InventoryTypeEnum::Beacon => f.write_str("BEACON"),
           InventoryTypeEnum::Hopper => f.write_str("HOPPER"),
           InventoryTypeEnum::ShulkerBox => f.write_str("SHULKER_BOX"),
           InventoryTypeEnum::Barrel => f.write_str("BARREL"),
           InventoryTypeEnum::BlastFurnace => f.write_str("BLAST_FURNACE"),
           InventoryTypeEnum::Lectern => f.write_str("LECTERN"),
           InventoryTypeEnum::Smoker => f.write_str("SMOKER"),
           InventoryTypeEnum::Loom => f.write_str("LOOM"),
           InventoryTypeEnum::Cartography => f.write_str("CARTOGRAPHY"),
           InventoryTypeEnum::Grindstone => f.write_str("GRINDSTONE"),
           InventoryTypeEnum::Stonecutter => f.write_str("STONECUTTER"),
           InventoryTypeEnum::Composter => f.write_str("COMPOSTER"),
           InventoryTypeEnum::ChiseledBookshelf => f.write_str("CHISELED_BOOKSHELF"),
           InventoryTypeEnum::Jukebox => f.write_str("JUKEBOX"),
           InventoryTypeEnum::SmithingNew => f.write_str("SMITHING_NEW"),
       }
   }
}
pub struct InventoryType<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>, pub InventoryTypeEnum);
impl<'mc> std::ops::Deref for InventoryType<'mc> {
   type Target = InventoryTypeEnum;
   fn deref(&self) -> &Self::Target {
       return &self.2;
   }
}
impl<'mc> JNIRaw<'mc> for InventoryType<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
pub struct InventoryTypeSlotType<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for InventoryTypeSlotType<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> InventoryTypeSlotType<'mc> {
pub fn from_raw(env: &blackboxmc_general::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>
) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate InventoryTypeSlotType from null object.")
    .into());
}
let (valid, name) = env.validate_name(&obj, "InventoryTypeSlotType")?;
if !valid {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a InventoryTypeSlotType object, got {}",
        name
    )
    .into())
} else {
    Ok(Self(env.clone(), obj
))
}
}
	pub fn value_of_with_string(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: std::option::Option<jni::objects::JClass<'mc>>,arg1: std::option::Option<impl Into<&'mc String>>) 
-> Result<blackboxmc_java::JavaEnum<'mc>, Box<dyn std::error::Error>>

{let val_0 = arg0.unwrap();
let val_1 = jni::objects::JObject::from(jni.new_string(arg1.unwrap().into()).unwrap());
let cls = &jni.find_class("java/lang/Enum")?;
let res = jni.call_static_method(cls,"valueOf",
"(Ljava/lang/Class;Ljava/lang/String;)Ljava/lang/Enum;",&[jni::objects::JValueGen::from(&val_0),jni::objects::JValueGen::from(&val_1)])?;
let mut obj = res.l()?;
blackboxmc_java::JavaEnum::from_raw(&jni,obj
)}
	pub fn name(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"name","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn equals(&mut self,arg0: jni::objects::JObject<'mc>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_0 = arg0;
let res = self.jni_ref().call_method(&self.jni_object(),"equals","(Ljava/lang/Object;)Z",&[jni::objects::JValueGen::from(&val_0)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn to_string(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"toString","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn hash_code(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"hashCode","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn describe_constable(&mut self) 
-> Result<blackboxmc_java::JavaOptional<'mc, javaEnumEnumDesc<E>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"describeConstable","()Ljava/util/Optional;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
blackboxmc_java::JavaOptional::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn declaring_class(&mut self) 
-> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getDeclaringClass","()Ljava/lang/Class;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(unsafe {jni::objects::JClass::from_raw(res.as_jni().l)})}
	pub fn ordinal(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"ordinal","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn wait(&mut self,arg0: std::option::Option<i64>,arg1: std::option::Option<i32>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_0 = jni::objects::JValueGen::Long(arg0.unwrap().into());
let val_1 = jni::objects::JValueGen::Int(arg1.unwrap().into());
let res = self.jni_ref().call_method(&self.jni_object(),"wait","(JI)V",&[jni::objects::JValueGen::from(&val_0),jni::objects::JValueGen::from(&val_1)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn class(&mut self) 
-> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getClass","()Ljava/lang/Class;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(unsafe {jni::objects::JClass::from_raw(res.as_jni().l)})}
	pub fn notify(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notify","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn notify_all(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notifyAll","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
}
impl<'mc> InventoryType<'mc> {
pub fn from_raw(env: &blackboxmc_general::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>
, e: InventoryTypeEnum
) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate InventoryType from null object.")
    .into());
}
let (valid, name) = env.validate_name(&obj, "InventoryType")?;
if !valid {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a InventoryType object, got {}",
        name
    )
    .into())
} else {
    Ok(Self(env.clone(), obj
, e
))
}
}
pub const CHEST: InventoryTypeEnum = InventoryTypeEnum::Chest;
pub const DISPENSER: InventoryTypeEnum = InventoryTypeEnum::Dispenser;
pub const DROPPER: InventoryTypeEnum = InventoryTypeEnum::Dropper;
pub const FURNACE: InventoryTypeEnum = InventoryTypeEnum::Furnace;
pub const WORKBENCH: InventoryTypeEnum = InventoryTypeEnum::Workbench;
pub const CRAFTING: InventoryTypeEnum = InventoryTypeEnum::Crafting;
pub const ENCHANTING: InventoryTypeEnum = InventoryTypeEnum::Enchanting;
pub const BREWING: InventoryTypeEnum = InventoryTypeEnum::Brewing;
pub const PLAYER: InventoryTypeEnum = InventoryTypeEnum::Player;
pub const CREATIVE: InventoryTypeEnum = InventoryTypeEnum::Creative;
pub const MERCHANT: InventoryTypeEnum = InventoryTypeEnum::Merchant;
pub const ENDER_CHEST: InventoryTypeEnum = InventoryTypeEnum::EnderChest;
pub const ANVIL: InventoryTypeEnum = InventoryTypeEnum::Anvil;
pub const SMITHING: InventoryTypeEnum = InventoryTypeEnum::Smithing;
pub const BEACON: InventoryTypeEnum = InventoryTypeEnum::Beacon;
pub const HOPPER: InventoryTypeEnum = InventoryTypeEnum::Hopper;
pub const SHULKER_BOX: InventoryTypeEnum = InventoryTypeEnum::ShulkerBox;
pub const BARREL: InventoryTypeEnum = InventoryTypeEnum::Barrel;
pub const BLAST_FURNACE: InventoryTypeEnum = InventoryTypeEnum::BlastFurnace;
pub const LECTERN: InventoryTypeEnum = InventoryTypeEnum::Lectern;
pub const SMOKER: InventoryTypeEnum = InventoryTypeEnum::Smoker;
pub const LOOM: InventoryTypeEnum = InventoryTypeEnum::Loom;
pub const CARTOGRAPHY: InventoryTypeEnum = InventoryTypeEnum::Cartography;
pub const GRINDSTONE: InventoryTypeEnum = InventoryTypeEnum::Grindstone;
pub const STONECUTTER: InventoryTypeEnum = InventoryTypeEnum::Stonecutter;
pub const COMPOSTER: InventoryTypeEnum = InventoryTypeEnum::Composter;
pub const CHISELED_BOOKSHELF: InventoryTypeEnum = InventoryTypeEnum::ChiseledBookshelf;
pub const JUKEBOX: InventoryTypeEnum = InventoryTypeEnum::Jukebox;
pub const SMITHING_NEW: InventoryTypeEnum = InventoryTypeEnum::SmithingNew;
pub fn from_string(str: String) -> std::option::Option<InventoryTypeEnum> {
match str.as_str() {
"CHEST" => Some(InventoryTypeEnum::Chest),
"DISPENSER" => Some(InventoryTypeEnum::Dispenser),
"DROPPER" => Some(InventoryTypeEnum::Dropper),
"FURNACE" => Some(InventoryTypeEnum::Furnace),
"WORKBENCH" => Some(InventoryTypeEnum::Workbench),
"CRAFTING" => Some(InventoryTypeEnum::Crafting),
"ENCHANTING" => Some(InventoryTypeEnum::Enchanting),
"BREWING" => Some(InventoryTypeEnum::Brewing),
"PLAYER" => Some(InventoryTypeEnum::Player),
"CREATIVE" => Some(InventoryTypeEnum::Creative),
"MERCHANT" => Some(InventoryTypeEnum::Merchant),
"ENDER_CHEST" => Some(InventoryTypeEnum::EnderChest),
"ANVIL" => Some(InventoryTypeEnum::Anvil),
"SMITHING" => Some(InventoryTypeEnum::Smithing),
"BEACON" => Some(InventoryTypeEnum::Beacon),
"HOPPER" => Some(InventoryTypeEnum::Hopper),
"SHULKER_BOX" => Some(InventoryTypeEnum::ShulkerBox),
"BARREL" => Some(InventoryTypeEnum::Barrel),
"BLAST_FURNACE" => Some(InventoryTypeEnum::BlastFurnace),
"LECTERN" => Some(InventoryTypeEnum::Lectern),
"SMOKER" => Some(InventoryTypeEnum::Smoker),
"LOOM" => Some(InventoryTypeEnum::Loom),
"CARTOGRAPHY" => Some(InventoryTypeEnum::Cartography),
"GRINDSTONE" => Some(InventoryTypeEnum::Grindstone),
"STONECUTTER" => Some(InventoryTypeEnum::Stonecutter),
"COMPOSTER" => Some(InventoryTypeEnum::Composter),
"CHISELED_BOOKSHELF" => Some(InventoryTypeEnum::ChiseledBookshelf),
"JUKEBOX" => Some(InventoryTypeEnum::Jukebox),
"SMITHING_NEW" => Some(InventoryTypeEnum::SmithingNew),
_ => None}}
	pub fn value_of(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: impl Into<&'mc String>) 
-> Result<crate::event::inventory::InventoryType<'mc>, Box<dyn std::error::Error>>

{let val_0 = jni::objects::JObject::from(jni.new_string(arg0.into()).unwrap());
let cls = &jni.find_class("org/bukkit/event/inventory/InventoryType")?;
let res = jni.call_static_method(cls,"valueOf",
"(Ljava/lang/String;)Lorg/bukkit/event/inventory/InventoryType;",&[jni::objects::JValueGen::from(&val_0)])?;
let mut obj = res.l()?;
let raw_obj = obj;let variant = jni.call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;let variant_str = jni    .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?    .to_string_lossy()    .to_string();
crate::event::inventory::InventoryType::from_raw(&jni,raw_obj
, crate::event::inventory::InventoryType::from_string(variant_str).unwrap()
)}
	pub fn default_size(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getDefaultSize","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn default_title(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getDefaultTitle","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn is_creatable(&mut self) 
-> Result<bool, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"isCreatable","()Z",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
}
pub struct PrepareSmithingEvent<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for PrepareSmithingEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> PrepareSmithingEvent<'mc> {
pub fn from_raw(env: &blackboxmc_general::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>
) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate PrepareSmithingEvent from null object.")
    .into());
}
let (valid, name) = env.validate_name(&obj, "PrepareSmithingEvent")?;
if !valid {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a PrepareSmithingEvent object, got {}",
        name
    )
    .into())
} else {
    Ok(Self(env.clone(), obj
))
}
}
	pub fn new(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: impl Into<&'mc crate::inventory::InventoryView<'mc>>,arg1: impl Into<&'mc crate::inventory::ItemStack<'mc>>) 
-> Result<crate::event::inventory::PrepareSmithingEvent<'mc>, Box<dyn std::error::Error>>

{let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let val_1 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone())};
let cls = &jni.find_class("org/bukkit/event/inventory/PrepareSmithingEvent")?;
let res = jni.new_object(cls,
"(Lorg/bukkit/inventory/InventoryView;Lorg/bukkit/inventory/ItemStack;)V",&[jni::objects::JValueGen::from(&val_0),jni::objects::JValueGen::from(&val_1)])?;
crate::event::inventory::PrepareSmithingEvent::from_raw(&jni,res
)}
	pub fn handlers(&mut self) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getHandlers","()Lorg/bukkit/event/HandlerList;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::event::HandlerList::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn inventory(&mut self) 
-> Result<crate::inventory::Inventory<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getInventory","()Lorg/bukkit/inventory/Inventory;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::inventory::Inventory::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn handler_list(jni: blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let cls = &jni.find_class("org/bukkit/event/HandlerList")?;
let res = jni.call_static_method(cls,"getHandlerList",
"()Lorg/bukkit/event/HandlerList;",&[])?;
let mut obj = res.l()?;
crate::event::HandlerList::from_raw(&jni,obj
)}
	pub fn set_result(&mut self,arg0: impl Into<&'mc crate::inventory::ItemStack<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let res = self.jni_ref().call_method(&self.jni_object(),"setResult","(Lorg/bukkit/inventory/ItemStack;)V",&[jni::objects::JValueGen::from(&val_0)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn result(&mut self) 
-> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getResult","()Lorg/bukkit/inventory/ItemStack;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::inventory::ItemStack::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn viewers(&mut self) 
-> Result<blackboxmc_java::JavaList<'mc, orgHumanEntity>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getViewers","()Ljava/util/List;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
blackboxmc_java::JavaList::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn view(&mut self) 
-> Result<crate::inventory::InventoryView<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getView","()Lorg/bukkit/inventory/InventoryView;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::inventory::InventoryView::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn event_name(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getEventName","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn is_asynchronous(&mut self) 
-> Result<bool, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"isAsynchronous","()Z",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn wait(&mut self,arg0: std::option::Option<i64>,arg1: std::option::Option<i32>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_0 = jni::objects::JValueGen::Long(arg0.unwrap().into());
let val_1 = jni::objects::JValueGen::Int(arg1.unwrap().into());
let res = self.jni_ref().call_method(&self.jni_object(),"wait","(JI)V",&[jni::objects::JValueGen::from(&val_0),jni::objects::JValueGen::from(&val_1)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn equals(&mut self,arg0: jni::objects::JObject<'mc>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_0 = arg0;
let res = self.jni_ref().call_method(&self.jni_object(),"equals","(Ljava/lang/Object;)Z",&[jni::objects::JValueGen::from(&val_0)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn to_string(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"toString","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn hash_code(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"hashCode","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn class(&mut self) 
-> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getClass","()Ljava/lang/Class;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(unsafe {jni::objects::JClass::from_raw(res.as_jni().l)})}
	pub fn notify(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notify","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn notify_all(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notifyAll","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
}
impl<'mc> Into<crate::event::inventory::PrepareInventoryResultEvent<'mc>> for PrepareSmithingEvent<'mc>{
   fn into(self) -> crate::event::inventory::PrepareInventoryResultEvent<'mc> {
       crate::event::inventory::PrepareInventoryResultEvent::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
pub struct TradeSelectEvent<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for TradeSelectEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> TradeSelectEvent<'mc> {
pub fn from_raw(env: &blackboxmc_general::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>
) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate TradeSelectEvent from null object.")
    .into());
}
let (valid, name) = env.validate_name(&obj, "TradeSelectEvent")?;
if !valid {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a TradeSelectEvent object, got {}",
        name
    )
    .into())
} else {
    Ok(Self(env.clone(), obj
))
}
}
	pub fn new(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: impl Into<&'mc crate::inventory::InventoryView<'mc>>,arg1: i32) 
-> Result<crate::event::inventory::TradeSelectEvent<'mc>, Box<dyn std::error::Error>>

{let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let val_1 = jni::objects::JValueGen::Int(arg1.into());
let cls = &jni.find_class("org/bukkit/event/inventory/TradeSelectEvent")?;
let res = jni.new_object(cls,
"(Lorg/bukkit/inventory/InventoryView;I)V",&[jni::objects::JValueGen::from(&val_0),jni::objects::JValueGen::from(&val_1)])?;
crate::event::inventory::TradeSelectEvent::from_raw(&jni,res
)}
	pub fn handlers(&mut self) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getHandlers","()Lorg/bukkit/event/HandlerList;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::event::HandlerList::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn index(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getIndex","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn inventory(&mut self) 
-> Result<crate::inventory::MerchantInventory<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getInventory","()Lorg/bukkit/inventory/MerchantInventory;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::inventory::MerchantInventory::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn merchant(&mut self) 
-> Result<crate::inventory::Merchant<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getMerchant","()Lorg/bukkit/inventory/Merchant;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::inventory::Merchant::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn handler_list(jni: blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let cls = &jni.find_class("org/bukkit/event/HandlerList")?;
let res = jni.call_static_method(cls,"getHandlerList",
"()Lorg/bukkit/event/HandlerList;",&[])?;
let mut obj = res.l()?;
crate::event::HandlerList::from_raw(&jni,obj
)}
	pub fn set_result(&mut self,arg0: impl Into<&'mc crate::event::EventResult<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let res = self.jni_ref().call_method(&self.jni_object(),"setResult","(Lorg/bukkit/event/Event$Result;)V",&[jni::objects::JValueGen::from(&val_0)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn result(&mut self) 
-> Result<crate::event::EventResult<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getResult","()Lorg/bukkit/event/Event$Result;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::event::EventResult::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn is_cancelled(&mut self) 
-> Result<bool, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"isCancelled","()Z",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn set_cancelled(&mut self,arg0: bool) 
-> Result<(), Box<dyn std::error::Error>>

{// -2
let val_0 = jni::objects::JValueGen::Bool(arg0.into());
let res = self.jni_ref().call_method(&self.jni_object(),"setCancelled","(Z)V",&[jni::objects::JValueGen::from(&val_0)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn who_clicked(&mut self) 
-> Result<crate::entity::HumanEntity<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getWhoClicked","()Lorg/bukkit/entity/HumanEntity;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::entity::HumanEntity::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn viewers(&mut self) 
-> Result<blackboxmc_java::JavaList<'mc, orgHumanEntity>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getViewers","()Ljava/util/List;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
blackboxmc_java::JavaList::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn view(&mut self) 
-> Result<crate::inventory::InventoryView<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getView","()Lorg/bukkit/inventory/InventoryView;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::inventory::InventoryView::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn event_name(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getEventName","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn is_asynchronous(&mut self) 
-> Result<bool, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"isAsynchronous","()Z",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn wait(&mut self,arg0: std::option::Option<i64>,arg1: std::option::Option<i32>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_0 = jni::objects::JValueGen::Long(arg0.unwrap().into());
let val_1 = jni::objects::JValueGen::Int(arg1.unwrap().into());
let res = self.jni_ref().call_method(&self.jni_object(),"wait","(JI)V",&[jni::objects::JValueGen::from(&val_0),jni::objects::JValueGen::from(&val_1)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn equals(&mut self,arg0: jni::objects::JObject<'mc>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_0 = arg0;
let res = self.jni_ref().call_method(&self.jni_object(),"equals","(Ljava/lang/Object;)Z",&[jni::objects::JValueGen::from(&val_0)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn to_string(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"toString","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn hash_code(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"hashCode","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn class(&mut self) 
-> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getClass","()Ljava/lang/Class;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(unsafe {jni::objects::JClass::from_raw(res.as_jni().l)})}
	pub fn notify(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notify","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn notify_all(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notifyAll","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
}
impl<'mc> Into<crate::event::inventory::InventoryInteractEvent<'mc>> for TradeSelectEvent<'mc>{
   fn into(self) -> crate::event::inventory::InventoryInteractEvent<'mc> {
       crate::event::inventory::InventoryInteractEvent::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
pub enum InventoryActionEnum {
	Nothing,
	PickupAll,
	PickupSome,
	PickupHalf,
	PickupOne,
	PlaceAll,
	PlaceSome,
	PlaceOne,
	SwapWithCursor,
	DropAllCursor,
	DropOneCursor,
	DropAllSlot,
	DropOneSlot,
	MoveToOtherInventory,
	HotbarMoveAndReadd,
	HotbarSwap,
	CloneStack,
	CollectToCursor,
	Unknown,
}
impl std::fmt::Display for InventoryActionEnum {
   fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
       match &self {
           InventoryActionEnum::Nothing => f.write_str("NOTHING"),
           InventoryActionEnum::PickupAll => f.write_str("PICKUP_ALL"),
           InventoryActionEnum::PickupSome => f.write_str("PICKUP_SOME"),
           InventoryActionEnum::PickupHalf => f.write_str("PICKUP_HALF"),
           InventoryActionEnum::PickupOne => f.write_str("PICKUP_ONE"),
           InventoryActionEnum::PlaceAll => f.write_str("PLACE_ALL"),
           InventoryActionEnum::PlaceSome => f.write_str("PLACE_SOME"),
           InventoryActionEnum::PlaceOne => f.write_str("PLACE_ONE"),
           InventoryActionEnum::SwapWithCursor => f.write_str("SWAP_WITH_CURSOR"),
           InventoryActionEnum::DropAllCursor => f.write_str("DROP_ALL_CURSOR"),
           InventoryActionEnum::DropOneCursor => f.write_str("DROP_ONE_CURSOR"),
           InventoryActionEnum::DropAllSlot => f.write_str("DROP_ALL_SLOT"),
           InventoryActionEnum::DropOneSlot => f.write_str("DROP_ONE_SLOT"),
           InventoryActionEnum::MoveToOtherInventory => f.write_str("MOVE_TO_OTHER_INVENTORY"),
           InventoryActionEnum::HotbarMoveAndReadd => f.write_str("HOTBAR_MOVE_AND_READD"),
           InventoryActionEnum::HotbarSwap => f.write_str("HOTBAR_SWAP"),
           InventoryActionEnum::CloneStack => f.write_str("CLONE_STACK"),
           InventoryActionEnum::CollectToCursor => f.write_str("COLLECT_TO_CURSOR"),
           InventoryActionEnum::Unknown => f.write_str("UNKNOWN"),
       }
   }
}
pub struct InventoryAction<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>, pub InventoryActionEnum);
impl<'mc> std::ops::Deref for InventoryAction<'mc> {
   type Target = InventoryActionEnum;
   fn deref(&self) -> &Self::Target {
       return &self.2;
   }
}
impl<'mc> JNIRaw<'mc> for InventoryAction<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> InventoryAction<'mc> {
pub fn from_raw(env: &blackboxmc_general::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>
, e: InventoryActionEnum
) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate InventoryAction from null object.")
    .into());
}
let (valid, name) = env.validate_name(&obj, "InventoryAction")?;
if !valid {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a InventoryAction object, got {}",
        name
    )
    .into())
} else {
    Ok(Self(env.clone(), obj
, e
))
}
}
pub const NOTHING: InventoryActionEnum = InventoryActionEnum::Nothing;
pub const PICKUP_ALL: InventoryActionEnum = InventoryActionEnum::PickupAll;
pub const PICKUP_SOME: InventoryActionEnum = InventoryActionEnum::PickupSome;
pub const PICKUP_HALF: InventoryActionEnum = InventoryActionEnum::PickupHalf;
pub const PICKUP_ONE: InventoryActionEnum = InventoryActionEnum::PickupOne;
pub const PLACE_ALL: InventoryActionEnum = InventoryActionEnum::PlaceAll;
pub const PLACE_SOME: InventoryActionEnum = InventoryActionEnum::PlaceSome;
pub const PLACE_ONE: InventoryActionEnum = InventoryActionEnum::PlaceOne;
pub const SWAP_WITH_CURSOR: InventoryActionEnum = InventoryActionEnum::SwapWithCursor;
pub const DROP_ALL_CURSOR: InventoryActionEnum = InventoryActionEnum::DropAllCursor;
pub const DROP_ONE_CURSOR: InventoryActionEnum = InventoryActionEnum::DropOneCursor;
pub const DROP_ALL_SLOT: InventoryActionEnum = InventoryActionEnum::DropAllSlot;
pub const DROP_ONE_SLOT: InventoryActionEnum = InventoryActionEnum::DropOneSlot;
pub const MOVE_TO_OTHER_INVENTORY: InventoryActionEnum = InventoryActionEnum::MoveToOtherInventory;
pub const HOTBAR_MOVE_AND_READD: InventoryActionEnum = InventoryActionEnum::HotbarMoveAndReadd;
pub const HOTBAR_SWAP: InventoryActionEnum = InventoryActionEnum::HotbarSwap;
pub const CLONE_STACK: InventoryActionEnum = InventoryActionEnum::CloneStack;
pub const COLLECT_TO_CURSOR: InventoryActionEnum = InventoryActionEnum::CollectToCursor;
pub const UNKNOWN: InventoryActionEnum = InventoryActionEnum::Unknown;
pub fn from_string(str: String) -> std::option::Option<InventoryActionEnum> {
match str.as_str() {
"NOTHING" => Some(InventoryActionEnum::Nothing),
"PICKUP_ALL" => Some(InventoryActionEnum::PickupAll),
"PICKUP_SOME" => Some(InventoryActionEnum::PickupSome),
"PICKUP_HALF" => Some(InventoryActionEnum::PickupHalf),
"PICKUP_ONE" => Some(InventoryActionEnum::PickupOne),
"PLACE_ALL" => Some(InventoryActionEnum::PlaceAll),
"PLACE_SOME" => Some(InventoryActionEnum::PlaceSome),
"PLACE_ONE" => Some(InventoryActionEnum::PlaceOne),
"SWAP_WITH_CURSOR" => Some(InventoryActionEnum::SwapWithCursor),
"DROP_ALL_CURSOR" => Some(InventoryActionEnum::DropAllCursor),
"DROP_ONE_CURSOR" => Some(InventoryActionEnum::DropOneCursor),
"DROP_ALL_SLOT" => Some(InventoryActionEnum::DropAllSlot),
"DROP_ONE_SLOT" => Some(InventoryActionEnum::DropOneSlot),
"MOVE_TO_OTHER_INVENTORY" => Some(InventoryActionEnum::MoveToOtherInventory),
"HOTBAR_MOVE_AND_READD" => Some(InventoryActionEnum::HotbarMoveAndReadd),
"HOTBAR_SWAP" => Some(InventoryActionEnum::HotbarSwap),
"CLONE_STACK" => Some(InventoryActionEnum::CloneStack),
"COLLECT_TO_CURSOR" => Some(InventoryActionEnum::CollectToCursor),
"UNKNOWN" => Some(InventoryActionEnum::Unknown),
_ => None}}
	pub fn value_of(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: impl Into<&'mc String>) 
-> Result<crate::event::inventory::InventoryAction<'mc>, Box<dyn std::error::Error>>

{let val_0 = jni::objects::JObject::from(jni.new_string(arg0.into()).unwrap());
let cls = &jni.find_class("org/bukkit/event/inventory/InventoryAction")?;
let res = jni.call_static_method(cls,"valueOf",
"(Ljava/lang/String;)Lorg/bukkit/event/inventory/InventoryAction;",&[jni::objects::JValueGen::from(&val_0)])?;
let mut obj = res.l()?;
let raw_obj = obj;let variant = jni.call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;let variant_str = jni    .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?    .to_string_lossy()    .to_string();
crate::event::inventory::InventoryAction::from_raw(&jni,raw_obj
, crate::event::inventory::InventoryAction::from_string(variant_str).unwrap()
)}
}
pub struct InventoryClickEvent<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for InventoryClickEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> InventoryClickEvent<'mc> {
pub fn from_raw(env: &blackboxmc_general::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>
) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate InventoryClickEvent from null object.")
    .into());
}
let (valid, name) = env.validate_name(&obj, "InventoryClickEvent")?;
if !valid {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a InventoryClickEvent object, got {}",
        name
    )
    .into())
} else {
    Ok(Self(env.clone(), obj
))
}
}
	pub fn new_with_inventory_view(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: impl Into<&'mc crate::inventory::InventoryView<'mc>>,arg1: impl Into<&'mc crate::event::inventory::InventoryTypeSlotType<'mc>>,arg2: i32,arg3: impl Into<&'mc crate::event::inventory::ClickType<'mc>>,arg4: std::option::Option<impl Into<&'mc crate::event::inventory::InventoryAction<'mc>>>,arg5: std::option::Option<i32>) 
-> Result<crate::event::inventory::InventoryClickEvent<'mc>, Box<dyn std::error::Error>>

{let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let val_1 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone())};
let val_2 = jni::objects::JValueGen::Int(arg2.into());
let val_3 = unsafe { jni::objects::JObject::from_raw(arg3.into().jni_object().clone())};
let val_4 = unsafe { jni::objects::JObject::from_raw(arg4.unwrap().into().jni_object().clone())};
let val_5 = jni::objects::JValueGen::Int(arg5.unwrap().into());
let cls = &jni.find_class("org/bukkit/event/inventory/InventoryClickEvent")?;
let res = jni.new_object(cls,
"(Lorg/bukkit/inventory/InventoryView;Lorg/bukkit/event/inventory/InventoryType$SlotType;ILorg/bukkit/event/inventory/ClickType;Lorg/bukkit/event/inventory/InventoryAction;I)V",&[jni::objects::JValueGen::from(&val_0),jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2),jni::objects::JValueGen::from(&val_3),jni::objects::JValueGen::from(&val_4),jni::objects::JValueGen::from(&val_5)])?;
crate::event::inventory::InventoryClickEvent::from_raw(&jni,res
)}
	pub fn handlers(&mut self) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getHandlers","()Lorg/bukkit/event/HandlerList;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::event::HandlerList::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn slot(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getSlot","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
#[deprecated]
	pub fn set_cursor(&mut self,arg0: impl Into<&'mc crate::inventory::ItemStack<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let res = self.jni_ref().call_method(&self.jni_object(),"setCursor","(Lorg/bukkit/inventory/ItemStack;)V",&[jni::objects::JValueGen::from(&val_0)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn cursor(&mut self) 
-> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getCursor","()Lorg/bukkit/inventory/ItemStack;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::inventory::ItemStack::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn slot_type(&mut self) 
-> Result<crate::event::inventory::InventoryTypeSlotType<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getSlotType","()Lorg/bukkit/event/inventory/InventoryType$SlotType;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::event::inventory::InventoryTypeSlotType::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn handler_list(jni: blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let cls = &jni.find_class("org/bukkit/event/HandlerList")?;
let res = jni.call_static_method(cls,"getHandlerList",
"()Lorg/bukkit/event/HandlerList;",&[])?;
let mut obj = res.l()?;
crate::event::HandlerList::from_raw(&jni,obj
)}
	pub fn action(&mut self) 
-> Result<crate::event::inventory::InventoryAction<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getAction","()Lorg/bukkit/event/inventory/InventoryAction;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };let variant = self.0.call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;let variant_str = self.0    .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?    .to_string_lossy()    .to_string();
crate::event::inventory::InventoryAction::from_raw(&self.jni_ref(),raw_obj
, crate::event::inventory::InventoryAction::from_string(variant_str).unwrap()
)}
	pub fn current_item(&mut self) 
-> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getCurrentItem","()Lorg/bukkit/inventory/ItemStack;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::inventory::ItemStack::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn is_right_click(&mut self) 
-> Result<bool, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"isRightClick","()Z",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn is_left_click(&mut self) 
-> Result<bool, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"isLeftClick","()Z",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn is_shift_click(&mut self) 
-> Result<bool, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"isShiftClick","()Z",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn set_current_item(&mut self,arg0: impl Into<&'mc crate::inventory::ItemStack<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let res = self.jni_ref().call_method(&self.jni_object(),"setCurrentItem","(Lorg/bukkit/inventory/ItemStack;)V",&[jni::objects::JValueGen::from(&val_0)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn clicked_inventory(&mut self) 
-> Result<crate::inventory::Inventory<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getClickedInventory","()Lorg/bukkit/inventory/Inventory;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::inventory::Inventory::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn raw_slot(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getRawSlot","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn hotbar_button(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getHotbarButton","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn click(&mut self) 
-> Result<crate::event::inventory::ClickType<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getClick","()Lorg/bukkit/event/inventory/ClickType;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };let variant = self.0.call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;let variant_str = self.0    .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?    .to_string_lossy()    .to_string();
crate::event::inventory::ClickType::from_raw(&self.jni_ref(),raw_obj
, crate::event::inventory::ClickType::from_string(variant_str).unwrap()
)}
	pub fn set_result(&mut self,arg0: impl Into<&'mc crate::event::EventResult<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let res = self.jni_ref().call_method(&self.jni_object(),"setResult","(Lorg/bukkit/event/Event$Result;)V",&[jni::objects::JValueGen::from(&val_0)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn result(&mut self) 
-> Result<crate::event::EventResult<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getResult","()Lorg/bukkit/event/Event$Result;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::event::EventResult::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn is_cancelled(&mut self) 
-> Result<bool, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"isCancelled","()Z",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn set_cancelled(&mut self,arg0: bool) 
-> Result<(), Box<dyn std::error::Error>>

{// -2
let val_0 = jni::objects::JValueGen::Bool(arg0.into());
let res = self.jni_ref().call_method(&self.jni_object(),"setCancelled","(Z)V",&[jni::objects::JValueGen::from(&val_0)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn who_clicked(&mut self) 
-> Result<crate::entity::HumanEntity<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getWhoClicked","()Lorg/bukkit/entity/HumanEntity;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::entity::HumanEntity::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn inventory(&mut self) 
-> Result<crate::inventory::Inventory<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getInventory","()Lorg/bukkit/inventory/Inventory;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::inventory::Inventory::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn viewers(&mut self) 
-> Result<blackboxmc_java::JavaList<'mc, orgHumanEntity>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getViewers","()Ljava/util/List;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
blackboxmc_java::JavaList::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn view(&mut self) 
-> Result<crate::inventory::InventoryView<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getView","()Lorg/bukkit/inventory/InventoryView;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::inventory::InventoryView::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn event_name(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getEventName","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn is_asynchronous(&mut self) 
-> Result<bool, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"isAsynchronous","()Z",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn wait(&mut self,arg0: std::option::Option<i64>,arg1: std::option::Option<i32>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_0 = jni::objects::JValueGen::Long(arg0.unwrap().into());
let val_1 = jni::objects::JValueGen::Int(arg1.unwrap().into());
let res = self.jni_ref().call_method(&self.jni_object(),"wait","(JI)V",&[jni::objects::JValueGen::from(&val_0),jni::objects::JValueGen::from(&val_1)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn equals(&mut self,arg0: jni::objects::JObject<'mc>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_0 = arg0;
let res = self.jni_ref().call_method(&self.jni_object(),"equals","(Ljava/lang/Object;)Z",&[jni::objects::JValueGen::from(&val_0)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn to_string(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"toString","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn hash_code(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"hashCode","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn class(&mut self) 
-> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getClass","()Ljava/lang/Class;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(unsafe {jni::objects::JClass::from_raw(res.as_jni().l)})}
	pub fn notify(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notify","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn notify_all(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notifyAll","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
}
impl<'mc> Into<crate::event::inventory::InventoryInteractEvent<'mc>> for InventoryClickEvent<'mc>{
   fn into(self) -> crate::event::inventory::InventoryInteractEvent<'mc> {
       crate::event::inventory::InventoryInteractEvent::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
pub struct InventoryDragEvent<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for InventoryDragEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> InventoryDragEvent<'mc> {
pub fn from_raw(env: &blackboxmc_general::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>
) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate InventoryDragEvent from null object.")
    .into());
}
let (valid, name) = env.validate_name(&obj, "InventoryDragEvent")?;
if !valid {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a InventoryDragEvent object, got {}",
        name
    )
    .into())
} else {
    Ok(Self(env.clone(), obj
))
}
}
	pub fn new(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: impl Into<&'mc crate::inventory::InventoryView<'mc>>,arg1: impl Into<&'mc crate::inventory::ItemStack<'mc>>,arg2: impl Into<&'mc crate::inventory::ItemStack<'mc>>,arg3: bool,arg4: impl Into<&'mc blackboxmc_java::JavaMap<'mc, javaInteger, orgItemStack>>) 
-> Result<crate::event::inventory::InventoryDragEvent<'mc>, Box<dyn std::error::Error>>

{let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let val_1 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone())};
let val_2 = unsafe { jni::objects::JObject::from_raw(arg2.into().jni_object().clone())};
// -2
let val_3 = jni::objects::JValueGen::Bool(arg3.into());
let val_4 = unsafe { jni::objects::JObject::from_raw(arg4.into().jni_object().clone())};
let cls = &jni.find_class("org/bukkit/event/inventory/InventoryDragEvent")?;
let res = jni.new_object(cls,
"(Lorg/bukkit/inventory/InventoryView;Lorg/bukkit/inventory/ItemStack;Lorg/bukkit/inventory/ItemStack;ZLjava/util/Map;)V",&[jni::objects::JValueGen::from(&val_0),jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2),jni::objects::JValueGen::from(&val_3),jni::objects::JValueGen::from(&val_4)])?;
crate::event::inventory::InventoryDragEvent::from_raw(&jni,res
)}
	pub fn handlers(&mut self) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getHandlers","()Lorg/bukkit/event/HandlerList;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::event::HandlerList::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn get_type(&mut self) 
-> Result<crate::event::inventory::DragType<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getType","()Lorg/bukkit/event/inventory/DragType;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };let variant = self.0.call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;let variant_str = self.0    .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?    .to_string_lossy()    .to_string();
crate::event::inventory::DragType::from_raw(&self.jni_ref(),raw_obj
, crate::event::inventory::DragType::from_string(variant_str).unwrap()
)}
	pub fn set_cursor(&mut self,arg0: impl Into<&'mc crate::inventory::ItemStack<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let res = self.jni_ref().call_method(&self.jni_object(),"setCursor","(Lorg/bukkit/inventory/ItemStack;)V",&[jni::objects::JValueGen::from(&val_0)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn cursor(&mut self) 
-> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getCursor","()Lorg/bukkit/inventory/ItemStack;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::inventory::ItemStack::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn handler_list(jni: blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let cls = &jni.find_class("org/bukkit/event/HandlerList")?;
let res = jni.call_static_method(cls,"getHandlerList",
"()Lorg/bukkit/event/HandlerList;",&[])?;
let mut obj = res.l()?;
crate::event::HandlerList::from_raw(&jni,obj
)}
	pub fn new_items(&mut self) 
-> Result<blackboxmc_java::JavaMap<'mc, javaInteger, orgItemStack>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getNewItems","()Ljava/util/Map;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
blackboxmc_java::JavaMap::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn raw_slots(&mut self) 
-> Result<blackboxmc_java::JavaSet<'mc, javaInteger>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getRawSlots","()Ljava/util/Set;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
blackboxmc_java::JavaSet::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn inventory_slots(&mut self) 
-> Result<blackboxmc_java::JavaSet<'mc, javaInteger>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getInventorySlots","()Ljava/util/Set;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
blackboxmc_java::JavaSet::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn old_cursor(&mut self) 
-> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getOldCursor","()Lorg/bukkit/inventory/ItemStack;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::inventory::ItemStack::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn set_result(&mut self,arg0: impl Into<&'mc crate::event::EventResult<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let res = self.jni_ref().call_method(&self.jni_object(),"setResult","(Lorg/bukkit/event/Event$Result;)V",&[jni::objects::JValueGen::from(&val_0)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn result(&mut self) 
-> Result<crate::event::EventResult<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getResult","()Lorg/bukkit/event/Event$Result;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::event::EventResult::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn is_cancelled(&mut self) 
-> Result<bool, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"isCancelled","()Z",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn set_cancelled(&mut self,arg0: bool) 
-> Result<(), Box<dyn std::error::Error>>

{// -2
let val_0 = jni::objects::JValueGen::Bool(arg0.into());
let res = self.jni_ref().call_method(&self.jni_object(),"setCancelled","(Z)V",&[jni::objects::JValueGen::from(&val_0)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn who_clicked(&mut self) 
-> Result<crate::entity::HumanEntity<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getWhoClicked","()Lorg/bukkit/entity/HumanEntity;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::entity::HumanEntity::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn inventory(&mut self) 
-> Result<crate::inventory::Inventory<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getInventory","()Lorg/bukkit/inventory/Inventory;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::inventory::Inventory::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn viewers(&mut self) 
-> Result<blackboxmc_java::JavaList<'mc, orgHumanEntity>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getViewers","()Ljava/util/List;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
blackboxmc_java::JavaList::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn view(&mut self) 
-> Result<crate::inventory::InventoryView<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getView","()Lorg/bukkit/inventory/InventoryView;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::inventory::InventoryView::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn event_name(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getEventName","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn is_asynchronous(&mut self) 
-> Result<bool, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"isAsynchronous","()Z",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn wait(&mut self,arg0: std::option::Option<i64>,arg1: std::option::Option<i32>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_0 = jni::objects::JValueGen::Long(arg0.unwrap().into());
let val_1 = jni::objects::JValueGen::Int(arg1.unwrap().into());
let res = self.jni_ref().call_method(&self.jni_object(),"wait","(JI)V",&[jni::objects::JValueGen::from(&val_0),jni::objects::JValueGen::from(&val_1)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn equals(&mut self,arg0: jni::objects::JObject<'mc>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_0 = arg0;
let res = self.jni_ref().call_method(&self.jni_object(),"equals","(Ljava/lang/Object;)Z",&[jni::objects::JValueGen::from(&val_0)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn to_string(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"toString","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn hash_code(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"hashCode","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn class(&mut self) 
-> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getClass","()Ljava/lang/Class;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(unsafe {jni::objects::JClass::from_raw(res.as_jni().l)})}
	pub fn notify(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notify","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn notify_all(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notifyAll","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
}
impl<'mc> Into<crate::event::inventory::InventoryInteractEvent<'mc>> for InventoryDragEvent<'mc>{
   fn into(self) -> crate::event::inventory::InventoryInteractEvent<'mc> {
       crate::event::inventory::InventoryInteractEvent::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
pub struct FurnaceBurnEvent<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for FurnaceBurnEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> FurnaceBurnEvent<'mc> {
pub fn from_raw(env: &blackboxmc_general::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>
) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate FurnaceBurnEvent from null object.")
    .into());
}
let (valid, name) = env.validate_name(&obj, "FurnaceBurnEvent")?;
if !valid {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a FurnaceBurnEvent object, got {}",
        name
    )
    .into())
} else {
    Ok(Self(env.clone(), obj
))
}
}
	pub fn new(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: impl Into<&'mc crate::block::Block<'mc>>,arg1: impl Into<&'mc crate::inventory::ItemStack<'mc>>,arg2: i32) 
-> Result<crate::event::inventory::FurnaceBurnEvent<'mc>, Box<dyn std::error::Error>>

{let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let val_1 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone())};
let val_2 = jni::objects::JValueGen::Int(arg2.into());
let cls = &jni.find_class("org/bukkit/event/inventory/FurnaceBurnEvent")?;
let res = jni.new_object(cls,
"(Lorg/bukkit/block/Block;Lorg/bukkit/inventory/ItemStack;I)V",&[jni::objects::JValueGen::from(&val_0),jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2)])?;
crate::event::inventory::FurnaceBurnEvent::from_raw(&jni,res
)}
	pub fn handlers(&mut self) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getHandlers","()Lorg/bukkit/event/HandlerList;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::event::HandlerList::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn is_cancelled(&mut self) 
-> Result<bool, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"isCancelled","()Z",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn fuel(&mut self) 
-> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getFuel","()Lorg/bukkit/inventory/ItemStack;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::inventory::ItemStack::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn burn_time(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getBurnTime","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn set_burn_time(&mut self,arg0: i32) 
-> Result<(), Box<dyn std::error::Error>>

{let val_0 = jni::objects::JValueGen::Int(arg0.into());
let res = self.jni_ref().call_method(&self.jni_object(),"setBurnTime","(I)V",&[jni::objects::JValueGen::from(&val_0)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn set_cancelled(&mut self,arg0: bool) 
-> Result<(), Box<dyn std::error::Error>>

{// -2
let val_0 = jni::objects::JValueGen::Bool(arg0.into());
let res = self.jni_ref().call_method(&self.jni_object(),"setCancelled","(Z)V",&[jni::objects::JValueGen::from(&val_0)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn handler_list(jni: blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let cls = &jni.find_class("org/bukkit/event/HandlerList")?;
let res = jni.call_static_method(cls,"getHandlerList",
"()Lorg/bukkit/event/HandlerList;",&[])?;
let mut obj = res.l()?;
crate::event::HandlerList::from_raw(&jni,obj
)}
	pub fn is_burning(&mut self) 
-> Result<bool, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"isBurning","()Z",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn set_burning(&mut self,arg0: bool) 
-> Result<(), Box<dyn std::error::Error>>

{// -2
let val_0 = jni::objects::JValueGen::Bool(arg0.into());
let res = self.jni_ref().call_method(&self.jni_object(),"setBurning","(Z)V",&[jni::objects::JValueGen::from(&val_0)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn block(&mut self) 
-> Result<crate::block::Block<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getBlock","()Lorg/bukkit/block/Block;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::block::Block::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn event_name(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getEventName","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn is_asynchronous(&mut self) 
-> Result<bool, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"isAsynchronous","()Z",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn wait(&mut self,arg0: std::option::Option<i64>,arg1: std::option::Option<i32>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_0 = jni::objects::JValueGen::Long(arg0.unwrap().into());
let val_1 = jni::objects::JValueGen::Int(arg1.unwrap().into());
let res = self.jni_ref().call_method(&self.jni_object(),"wait","(JI)V",&[jni::objects::JValueGen::from(&val_0),jni::objects::JValueGen::from(&val_1)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn equals(&mut self,arg0: jni::objects::JObject<'mc>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_0 = arg0;
let res = self.jni_ref().call_method(&self.jni_object(),"equals","(Ljava/lang/Object;)Z",&[jni::objects::JValueGen::from(&val_0)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn to_string(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"toString","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn hash_code(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"hashCode","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn class(&mut self) 
-> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getClass","()Ljava/lang/Class;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(unsafe {jni::objects::JClass::from_raw(res.as_jni().l)})}
	pub fn notify(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notify","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn notify_all(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notifyAll","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
}
impl<'mc> Into<crate::event::Cancellable<'mc>> for FurnaceBurnEvent<'mc>{
   fn into(self) -> crate::event::Cancellable<'mc> {
       crate::event::Cancellable::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
impl<'mc> Into<crate::event::block::BlockEvent<'mc>> for FurnaceBurnEvent<'mc>{
   fn into(self) -> crate::event::block::BlockEvent<'mc> {
       crate::event::block::BlockEvent::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
pub struct InventoryCloseEvent<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for InventoryCloseEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> InventoryCloseEvent<'mc> {
pub fn from_raw(env: &blackboxmc_general::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>
) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate InventoryCloseEvent from null object.")
    .into());
}
let (valid, name) = env.validate_name(&obj, "InventoryCloseEvent")?;
if !valid {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a InventoryCloseEvent object, got {}",
        name
    )
    .into())
} else {
    Ok(Self(env.clone(), obj
))
}
}
	pub fn new(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: impl Into<&'mc crate::inventory::InventoryView<'mc>>) 
-> Result<crate::event::inventory::InventoryCloseEvent<'mc>, Box<dyn std::error::Error>>

{let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let cls = &jni.find_class("org/bukkit/event/inventory/InventoryCloseEvent")?;
let res = jni.new_object(cls,
"(Lorg/bukkit/inventory/InventoryView;)V",&[jni::objects::JValueGen::from(&val_0)])?;
crate::event::inventory::InventoryCloseEvent::from_raw(&jni,res
)}
	pub fn handlers(&mut self) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getHandlers","()Lorg/bukkit/event/HandlerList;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::event::HandlerList::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn player(&mut self) 
-> Result<crate::entity::HumanEntity<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getPlayer","()Lorg/bukkit/entity/HumanEntity;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::entity::HumanEntity::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn handler_list(jni: blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let cls = &jni.find_class("org/bukkit/event/HandlerList")?;
let res = jni.call_static_method(cls,"getHandlerList",
"()Lorg/bukkit/event/HandlerList;",&[])?;
let mut obj = res.l()?;
crate::event::HandlerList::from_raw(&jni,obj
)}
	pub fn inventory(&mut self) 
-> Result<crate::inventory::Inventory<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getInventory","()Lorg/bukkit/inventory/Inventory;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::inventory::Inventory::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn viewers(&mut self) 
-> Result<blackboxmc_java::JavaList<'mc, orgHumanEntity>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getViewers","()Ljava/util/List;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
blackboxmc_java::JavaList::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn view(&mut self) 
-> Result<crate::inventory::InventoryView<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getView","()Lorg/bukkit/inventory/InventoryView;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::inventory::InventoryView::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn event_name(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getEventName","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn is_asynchronous(&mut self) 
-> Result<bool, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"isAsynchronous","()Z",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn wait(&mut self,arg0: std::option::Option<i64>,arg1: std::option::Option<i32>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_0 = jni::objects::JValueGen::Long(arg0.unwrap().into());
let val_1 = jni::objects::JValueGen::Int(arg1.unwrap().into());
let res = self.jni_ref().call_method(&self.jni_object(),"wait","(JI)V",&[jni::objects::JValueGen::from(&val_0),jni::objects::JValueGen::from(&val_1)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn equals(&mut self,arg0: jni::objects::JObject<'mc>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_0 = arg0;
let res = self.jni_ref().call_method(&self.jni_object(),"equals","(Ljava/lang/Object;)Z",&[jni::objects::JValueGen::from(&val_0)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn to_string(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"toString","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn hash_code(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"hashCode","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn class(&mut self) 
-> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getClass","()Ljava/lang/Class;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(unsafe {jni::objects::JClass::from_raw(res.as_jni().l)})}
	pub fn notify(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notify","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn notify_all(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notifyAll","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
}
impl<'mc> Into<crate::event::inventory::InventoryEvent<'mc>> for InventoryCloseEvent<'mc>{
   fn into(self) -> crate::event::inventory::InventoryEvent<'mc> {
       crate::event::inventory::InventoryEvent::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
pub struct PrepareInventoryResultEvent<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for PrepareInventoryResultEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> PrepareInventoryResultEvent<'mc> {
pub fn from_raw(env: &blackboxmc_general::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>
) -> Result<Self, Box<dyn std::error::Error>> {
if obj.is_null() {
    return Err(eyre::eyre!(
        "Tried to instantiate PrepareInventoryResultEvent from null object.")
    .into());
}
let (valid, name) = env.validate_name(&obj, "PrepareInventoryResultEvent")?;
if !valid {
    Err(eyre::eyre!(
        "Invalid argument passed. Expected a PrepareInventoryResultEvent object, got {}",
        name
    )
    .into())
} else {
    Ok(Self(env.clone(), obj
))
}
}
	pub fn new(jni: blackboxmc_general::SharedJNIEnv<'mc>,arg0: impl Into<&'mc crate::inventory::InventoryView<'mc>>,arg1: impl Into<&'mc crate::inventory::ItemStack<'mc>>) 
-> Result<crate::event::inventory::PrepareInventoryResultEvent<'mc>, Box<dyn std::error::Error>>

{let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let val_1 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone())};
let cls = &jni.find_class("org/bukkit/event/inventory/PrepareInventoryResultEvent")?;
let res = jni.new_object(cls,
"(Lorg/bukkit/inventory/InventoryView;Lorg/bukkit/inventory/ItemStack;)V",&[jni::objects::JValueGen::from(&val_0),jni::objects::JValueGen::from(&val_1)])?;
crate::event::inventory::PrepareInventoryResultEvent::from_raw(&jni,res
)}
	pub fn handlers(&mut self) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getHandlers","()Lorg/bukkit/event/HandlerList;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::event::HandlerList::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn set_result(&mut self,arg0: impl Into<&'mc crate::inventory::ItemStack<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone())};
let res = self.jni_ref().call_method(&self.jni_object(),"setResult","(Lorg/bukkit/inventory/ItemStack;)V",&[jni::objects::JValueGen::from(&val_0)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn result(&mut self) 
-> Result<crate::inventory::ItemStack<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getResult","()Lorg/bukkit/inventory/ItemStack;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::inventory::ItemStack::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn handler_list(jni: blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::event::HandlerList<'mc>, Box<dyn std::error::Error>>

{let cls = &jni.find_class("org/bukkit/event/HandlerList")?;
let res = jni.call_static_method(cls,"getHandlerList",
"()Lorg/bukkit/event/HandlerList;",&[])?;
let mut obj = res.l()?;
crate::event::HandlerList::from_raw(&jni,obj
)}
	pub fn inventory(&mut self) 
-> Result<crate::inventory::Inventory<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getInventory","()Lorg/bukkit/inventory/Inventory;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::inventory::Inventory::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn viewers(&mut self) 
-> Result<blackboxmc_java::JavaList<'mc, orgHumanEntity>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getViewers","()Ljava/util/List;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
blackboxmc_java::JavaList::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn view(&mut self) 
-> Result<crate::inventory::InventoryView<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getView","()Lorg/bukkit/inventory/InventoryView;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
crate::inventory::InventoryView::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn event_name(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getEventName","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn is_asynchronous(&mut self) 
-> Result<bool, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"isAsynchronous","()Z",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn wait(&mut self,arg0: std::option::Option<i64>,arg1: std::option::Option<i32>) 
-> Result<(), Box<dyn std::error::Error>>

{let val_0 = jni::objects::JValueGen::Long(arg0.unwrap().into());
let val_1 = jni::objects::JValueGen::Int(arg1.unwrap().into());
let res = self.jni_ref().call_method(&self.jni_object(),"wait","(JI)V",&[jni::objects::JValueGen::from(&val_0),jni::objects::JValueGen::from(&val_1)]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn equals(&mut self,arg0: jni::objects::JObject<'mc>) 
-> Result<bool, Box<dyn std::error::Error>>

{let val_0 = arg0;
let res = self.jni_ref().call_method(&self.jni_object(),"equals","(Ljava/lang/Object;)Z",&[jni::objects::JValueGen::from(&val_0)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.z().unwrap())}
	pub fn to_string(&mut self) 
-> Result<String, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"toString","()Ljava/lang/String;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string())}
	pub fn hash_code(&mut self) 
-> Result<i32, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"hashCode","()I",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(res.i().unwrap())}
	pub fn class(&mut self) 
-> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"getClass","()Ljava/lang/Class;",&[]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(unsafe {jni::objects::JClass::from_raw(res.as_jni().l)})}
	pub fn notify(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notify","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
	pub fn notify_all(&mut self) 
-> Result<(), Box<dyn std::error::Error>>

{let res = self.jni_ref().call_method(&self.jni_object(),"notifyAll","()V",&[]);
self.jni_ref().translate_error(res)?;
Ok(())}
}
impl<'mc> Into<crate::event::inventory::InventoryEvent<'mc>> for PrepareInventoryResultEvent<'mc>{
   fn into(self) -> crate::event::inventory::InventoryEvent<'mc> {
       crate::event::inventory::InventoryEvent::from_raw(&self.jni_ref(), self.1).unwrap()
   }
}
