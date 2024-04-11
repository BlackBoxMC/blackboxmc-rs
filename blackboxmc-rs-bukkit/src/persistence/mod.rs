#![allow(deprecated)]
use blackboxmc_general::JNIRaw;
use blackboxmc_general::JNIInstantiatable;
use color_eyre::eyre::Result;
#[repr(C)]
pub struct PersistentDataType<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for PersistentDataType<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for PersistentDataType<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate PersistentDataType from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/persistence/PersistentDataType")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a PersistentDataType object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> PersistentDataType<'mc> {
pub fn from_extendable(
    env: &blackboxmc_general::SharedJNIEnv<'mc>,
    plugin: &'mc crate::plugin::Plugin,
    address: i32,
    lib_name: String,
    name: String,
) -> Result<Self, Box<dyn std::error::Error>> {
    let obj = unsafe { plugin.new_extendable(address, "PersistentDataType", name, lib_name) }?;
    Self::from_raw(env, obj)
}
	pub fn primitive_type(&self) 
-> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Ljni::objects::JClass;");
let res = self.jni_ref().call_method(&self.jni_object(),"getPrimitiveType",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
unsafe {jni::objects::JClass::from_raw(res.as_jni().l)}
)}
	pub fn complex_type(&self) 
-> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Ljni::objects::JClass;");
let res = self.jni_ref().call_method(&self.jni_object(),"getComplexType",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
unsafe {jni::objects::JClass::from_raw(res.as_jni().l)}
)}
	pub fn to_primitive(&self,complex: jni::objects::JObject<'mc>,context: impl Into<crate::persistence::PersistentDataAdapterContext<'mc>>) 
-> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("(Ljava/lang/Object;Lorg/bukkit/persistence/PersistentDataAdapterContext;)Ljni::objects::JObject;");
let val_1 = jni::objects::JValueGen::Object(complex);
let val_2 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(context.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"toPrimitive",sig.as_str(),vec![jni::objects::JValueGen::from(val_1),jni::objects::JValueGen::from(val_2)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.l()?
)}
	pub fn from_primitive(&self,primitive: jni::objects::JObject<'mc>,context: impl Into<crate::persistence::PersistentDataAdapterContext<'mc>>) 
-> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("(Ljava/lang/Object;Lorg/bukkit/persistence/PersistentDataAdapterContext;)Ljni::objects::JObject;");
let val_1 = jni::objects::JValueGen::Object(primitive);
let val_2 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(context.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"fromPrimitive",sig.as_str(),vec![jni::objects::JValueGen::from(val_1),jni::objects::JValueGen::from(val_2)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.l()?
)}

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
#[repr(C)]
pub struct ListPersistentDataType<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for ListPersistentDataType<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for ListPersistentDataType<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate ListPersistentDataType from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/persistence/ListPersistentDataType")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a ListPersistentDataType object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> ListPersistentDataType<'mc> {
	pub fn element_type(&self) 
-> Result<crate::persistence::PersistentDataType<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lcrate::persistence::PersistentDataType;");
let res = self.jni_ref().call_method(&self.jni_object(),"elementType",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::persistence::PersistentDataType::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn primitive_type(&self) 
-> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Ljni::objects::JClass;");
let res = self.jni_ref().call_method(&self.jni_object(),"getPrimitiveType",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
unsafe {jni::objects::JClass::from_raw(res.as_jni().l)}
)}
	pub fn complex_type(&self) 
-> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Ljni::objects::JClass;");
let res = self.jni_ref().call_method(&self.jni_object(),"getComplexType",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
unsafe {jni::objects::JClass::from_raw(res.as_jni().l)}
)}
	pub fn to_primitive(&self,complex: jni::objects::JObject<'mc>,context: impl Into<crate::persistence::PersistentDataAdapterContext<'mc>>) 
-> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("(Ljava/lang/Object;Lorg/bukkit/persistence/PersistentDataAdapterContext;)Ljni::objects::JObject;");
let val_1 = jni::objects::JValueGen::Object(complex);
let val_2 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(context.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"toPrimitive",sig.as_str(),vec![jni::objects::JValueGen::from(val_1),jni::objects::JValueGen::from(val_2)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.l()?
)}
	pub fn from_primitive(&self,primitive: jni::objects::JObject<'mc>,context: impl Into<crate::persistence::PersistentDataAdapterContext<'mc>>) 
-> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("(Ljava/lang/Object;Lorg/bukkit/persistence/PersistentDataAdapterContext;)Ljni::objects::JObject;");
let val_1 = jni::objects::JValueGen::Object(primitive);
let val_2 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(context.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"fromPrimitive",sig.as_str(),vec![jni::objects::JValueGen::from(val_1),jni::objects::JValueGen::from(val_2)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.l()?
)}

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
impl<'mc> Into<crate::persistence::PersistentDataType<'mc>> for ListPersistentDataType<'mc>{

fn into(self) -> crate::persistence::PersistentDataType<'mc> {

crate::persistence::PersistentDataType::from_raw(&self.jni_ref(), self.1).expect("Error converting ListPersistentDataType into crate::persistence::PersistentDataType")

   }
}
#[repr(C)]
pub struct PersistentDataTypeBooleanPersistentDataType<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for PersistentDataTypeBooleanPersistentDataType<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for PersistentDataTypeBooleanPersistentDataType<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate PersistentDataTypeBooleanPersistentDataType from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/persistence/PersistentDataType/BooleanPersistentDataType")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a PersistentDataTypeBooleanPersistentDataType object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> PersistentDataTypeBooleanPersistentDataType<'mc> {
	pub fn new(jni: &blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::persistence::PersistentDataTypeBooleanPersistentDataType<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()V");
let cls = jni.find_class("org/bukkit/persistence/PersistentDataType/BooleanPersistentDataType"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.new_object(cls,
sig.as_str(),vec![]);
let res = 
jni.translate_error_no_gen(res)?;
crate::persistence::PersistentDataTypeBooleanPersistentDataType::from_raw(&jni,res
)}
	pub fn primitive_type(&self) 
-> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Ljni::objects::JClass;");
let res = self.jni_ref().call_method(&self.jni_object(),"getPrimitiveType",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
unsafe {jni::objects::JClass::from_raw(res.as_jni().l)}
)}
	pub fn complex_type(&self) 
-> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Ljni::objects::JClass;");
let res = self.jni_ref().call_method(&self.jni_object(),"getComplexType",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
unsafe {jni::objects::JClass::from_raw(res.as_jni().l)}
)}
	pub fn to_primitive(&self,complex: bool,context: impl Into<crate::persistence::PersistentDataAdapterContext<'mc>>) 
-> Result<i8, Box<dyn std::error::Error>>

{let sig = String::from("(Ljava/lang/Boolean;Lorg/bukkit/persistence/PersistentDataAdapterContext;)Li8;");
let val_1 = jni::objects::JValueGen::Object(self.jni_ref().new_object("java/lang/Boolean", "(Z)V", vec![complex.into()])?);
let val_2 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(context.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"toPrimitive",sig.as_str(),vec![jni::objects::JValueGen::from(val_1),jni::objects::JValueGen::from(val_2)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.b()?
)}
	pub fn from_primitive(&self,primitive: i8,context: impl Into<crate::persistence::PersistentDataAdapterContext<'mc>>) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("(Ljava/lang/Byte;Lorg/bukkit/persistence/PersistentDataAdapterContext;)Lbool;");
let val_1 = jni::objects::JValueGen::Object(self.jni_ref().new_object("java/lang/Byte", "(Ljava/Lang/Object;)V", vec![primitive.into()])?);
let val_2 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(context.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"fromPrimitive",sig.as_str(),vec![jni::objects::JValueGen::from(val_1),jni::objects::JValueGen::from(val_2)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
// SUPER CLASS: Object

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
impl<'mc> Into<crate::persistence::PersistentDataType<'mc>> for PersistentDataTypeBooleanPersistentDataType<'mc>{

fn into(self) -> crate::persistence::PersistentDataType<'mc> {

crate::persistence::PersistentDataType::from_raw(&self.jni_ref(), self.1).expect("Error converting PersistentDataTypeBooleanPersistentDataType into crate::persistence::PersistentDataType")

   }
}
#[repr(C)]
pub struct ListPersistentDataTypeProvider<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for ListPersistentDataTypeProvider<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for ListPersistentDataTypeProvider<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate ListPersistentDataTypeProvider from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/persistence/ListPersistentDataTypeProvider")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a ListPersistentDataTypeProvider object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> ListPersistentDataTypeProvider<'mc> {
	pub fn bytes(&self) 
-> Result<crate::persistence::ListPersistentDataType<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lcrate::persistence::ListPersistentDataType;");
let res = self.jni_ref().call_method(&self.jni_object(),"bytes",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::persistence::ListPersistentDataType::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn shorts(&self) 
-> Result<crate::persistence::ListPersistentDataType<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lcrate::persistence::ListPersistentDataType;");
let res = self.jni_ref().call_method(&self.jni_object(),"shorts",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::persistence::ListPersistentDataType::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn integers(&self) 
-> Result<crate::persistence::ListPersistentDataType<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lcrate::persistence::ListPersistentDataType;");
let res = self.jni_ref().call_method(&self.jni_object(),"integers",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::persistence::ListPersistentDataType::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn longs(&self) 
-> Result<crate::persistence::ListPersistentDataType<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lcrate::persistence::ListPersistentDataType;");
let res = self.jni_ref().call_method(&self.jni_object(),"longs",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::persistence::ListPersistentDataType::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn floats(&self) 
-> Result<crate::persistence::ListPersistentDataType<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lcrate::persistence::ListPersistentDataType;");
let res = self.jni_ref().call_method(&self.jni_object(),"floats",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::persistence::ListPersistentDataType::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn doubles(&self) 
-> Result<crate::persistence::ListPersistentDataType<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lcrate::persistence::ListPersistentDataType;");
let res = self.jni_ref().call_method(&self.jni_object(),"doubles",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::persistence::ListPersistentDataType::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn booleans(&self) 
-> Result<crate::persistence::ListPersistentDataType<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lcrate::persistence::ListPersistentDataType;");
let res = self.jni_ref().call_method(&self.jni_object(),"booleans",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::persistence::ListPersistentDataType::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn strings(&self) 
-> Result<crate::persistence::ListPersistentDataType<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lcrate::persistence::ListPersistentDataType;");
let res = self.jni_ref().call_method(&self.jni_object(),"strings",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::persistence::ListPersistentDataType::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn byte_arrays(&self) 
-> Result<crate::persistence::ListPersistentDataType<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lcrate::persistence::ListPersistentDataType;");
let res = self.jni_ref().call_method(&self.jni_object(),"byteArrays",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::persistence::ListPersistentDataType::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn integer_arrays(&self) 
-> Result<crate::persistence::ListPersistentDataType<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lcrate::persistence::ListPersistentDataType;");
let res = self.jni_ref().call_method(&self.jni_object(),"integerArrays",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::persistence::ListPersistentDataType::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn long_arrays(&self) 
-> Result<crate::persistence::ListPersistentDataType<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lcrate::persistence::ListPersistentDataType;");
let res = self.jni_ref().call_method(&self.jni_object(),"longArrays",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::persistence::ListPersistentDataType::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn data_containers(&self) 
-> Result<crate::persistence::ListPersistentDataType<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lcrate::persistence::ListPersistentDataType;");
let res = self.jni_ref().call_method(&self.jni_object(),"dataContainers",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::persistence::ListPersistentDataType::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn list_type_from(&self,element_type: impl Into<crate::persistence::PersistentDataType<'mc>>) 
-> Result<crate::persistence::ListPersistentDataType<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/persistence/PersistentDataType;)Lcrate::persistence::ListPersistentDataType;");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(element_type.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"listTypeFrom",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
crate::persistence::ListPersistentDataType::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
// SUPER CLASS: Object

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
#[repr(C)]
pub struct PersistentDataTypePrimitivePersistentDataType<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for PersistentDataTypePrimitivePersistentDataType<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for PersistentDataTypePrimitivePersistentDataType<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate PersistentDataTypePrimitivePersistentDataType from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/persistence/PersistentDataType/PrimitivePersistentDataType")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a PersistentDataTypePrimitivePersistentDataType object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> PersistentDataTypePrimitivePersistentDataType<'mc> {
	pub fn primitive_type(&self) 
-> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Ljni::objects::JClass;");
let res = self.jni_ref().call_method(&self.jni_object(),"getPrimitiveType",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
unsafe {jni::objects::JClass::from_raw(res.as_jni().l)}
)}
	pub fn complex_type(&self) 
-> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Ljni::objects::JClass;");
let res = self.jni_ref().call_method(&self.jni_object(),"getComplexType",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
unsafe {jni::objects::JClass::from_raw(res.as_jni().l)}
)}
	pub fn to_primitive(&self,complex: jni::objects::JObject<'mc>,context: impl Into<crate::persistence::PersistentDataAdapterContext<'mc>>) 
-> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("(Ljava/lang/Object;Lorg/bukkit/persistence/PersistentDataAdapterContext;)Ljni::objects::JObject;");
let val_1 = jni::objects::JValueGen::Object(complex);
let val_2 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(context.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"toPrimitive",sig.as_str(),vec![jni::objects::JValueGen::from(val_1),jni::objects::JValueGen::from(val_2)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.l()?
)}
	pub fn from_primitive(&self,primitive: jni::objects::JObject<'mc>,context: impl Into<crate::persistence::PersistentDataAdapterContext<'mc>>) 
-> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("(Ljava/lang/Object;Lorg/bukkit/persistence/PersistentDataAdapterContext;)Ljni::objects::JObject;");
let val_1 = jni::objects::JValueGen::Object(primitive);
let val_2 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(context.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"fromPrimitive",sig.as_str(),vec![jni::objects::JValueGen::from(val_1),jni::objects::JValueGen::from(val_2)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.l()?
)}
// SUPER CLASS: Object

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
impl<'mc> Into<crate::persistence::PersistentDataType<'mc>> for PersistentDataTypePrimitivePersistentDataType<'mc>{

fn into(self) -> crate::persistence::PersistentDataType<'mc> {

crate::persistence::PersistentDataType::from_raw(&self.jni_ref(), self.1).expect("Error converting PersistentDataTypePrimitivePersistentDataType into crate::persistence::PersistentDataType")

   }
}
#[repr(C)]
pub struct PersistentDataHolder<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for PersistentDataHolder<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for PersistentDataHolder<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate PersistentDataHolder from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/persistence/PersistentDataHolder")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a PersistentDataHolder object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> PersistentDataHolder<'mc> {
	pub fn persistent_data_container(&self) 
-> Result<crate::persistence::PersistentDataContainer<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lcrate::persistence::PersistentDataContainer;");
let res = self.jni_ref().call_method(&self.jni_object(),"getPersistentDataContainer",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::persistence::PersistentDataContainer::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
#[repr(C)]
pub struct PersistentDataContainer<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for PersistentDataContainer<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for PersistentDataContainer<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate PersistentDataContainer from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/persistence/PersistentDataContainer")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a PersistentDataContainer object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> PersistentDataContainer<'mc> {
	pub fn set(&self,key: impl Into<crate::NamespacedKey<'mc>>,val_type: impl Into<crate::persistence::PersistentDataType<'mc>>,value: jni::objects::JObject<'mc>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/NamespacedKey;Lorg/bukkit/persistence/PersistentDataType;Ljava/lang/Object;)L();");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(key.into().jni_object().clone())});
let val_2 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(val_type.into().jni_object().clone())});
let val_3 = jni::objects::JValueGen::Object(value);
let res = self.jni_ref().call_method(&self.jni_object(),"set",sig.as_str(),vec![jni::objects::JValueGen::from(val_1),jni::objects::JValueGen::from(val_2),jni::objects::JValueGen::from(val_3)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
	pub fn has_with_key(&self,key: impl Into<crate::NamespacedKey<'mc>>,val_type: std::option::Option<impl Into<crate::persistence::PersistentDataType<'mc>>>) 
-> Result<bool, Box<dyn std::error::Error>>

{let mut args = Vec::new();
let mut sig = String::from("(");
sig += "Lorg/bukkit/NamespacedKey;";
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(key.into().jni_object().clone())});
args.push(val_1);
if let Some(a) = val_type {
sig += "Lorg/bukkit/persistence/PersistentDataType;";
let val_2 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(a.into().jni_object().clone())});
args.push(val_2);
}
sig += ")Z";
let res = self.jni_ref().call_method(&self.jni_object(),"has",sig.as_str(),args);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
	pub fn get(&self,key: impl Into<crate::NamespacedKey<'mc>>,val_type: impl Into<crate::persistence::PersistentDataType<'mc>>) 
-> Result<Option<jni::objects::JObject<'mc>>, Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/NamespacedKey;Lorg/bukkit/persistence/PersistentDataType;)Ljni::objects::JObject;");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(key.into().jni_object().clone())});
let val_2 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(val_type.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"get",sig.as_str(),vec![jni::objects::JValueGen::from(val_1),jni::objects::JValueGen::from(val_2)]);
let res = 
self.jni_ref().translate_error(res)?;
if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {return Ok(None);}
Ok(
Some(
res.l()?
)
)}
	pub fn get_or_default(&self,key: impl Into<crate::NamespacedKey<'mc>>,val_type: impl Into<crate::persistence::PersistentDataType<'mc>>,default_value: jni::objects::JObject<'mc>) 
-> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/NamespacedKey;Lorg/bukkit/persistence/PersistentDataType;Ljava/lang/Object;)Ljni::objects::JObject;");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(key.into().jni_object().clone())});
let val_2 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(val_type.into().jni_object().clone())});
let val_3 = jni::objects::JValueGen::Object(default_value);
let res = self.jni_ref().call_method(&self.jni_object(),"getOrDefault",sig.as_str(),vec![jni::objects::JValueGen::from(val_1),jni::objects::JValueGen::from(val_2),jni::objects::JValueGen::from(val_3)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.l()?
)}
	pub fn keys(&self) 
-> Result<blackboxmc_java::util::JavaSet<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lblackboxmc_java::util::Set;");
let res = self.jni_ref().call_method(&self.jni_object(),"getKeys",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
blackboxmc_java::util::JavaSet::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn remove(&self,key: impl Into<crate::NamespacedKey<'mc>>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/NamespacedKey;)L();");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(key.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"remove",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
	pub fn is_empty(&self) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("()Lbool;");
let res = self.jni_ref().call_method(&self.jni_object(),"isEmpty",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
	pub fn copy_to(&self,other: impl Into<crate::persistence::PersistentDataContainer<'mc>>,replace: bool) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/persistence/PersistentDataContainer;Z)L();");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(other.into().jni_object().clone())});
let val_2 = jni::objects::JValueGen::Bool(replace.into());
let res = self.jni_ref().call_method(&self.jni_object(),"copyTo",sig.as_str(),vec![jni::objects::JValueGen::from(val_1),jni::objects::JValueGen::from(val_2)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
	pub fn adapter_context(&self) 
-> Result<crate::persistence::PersistentDataAdapterContext<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lcrate::persistence::PersistentDataAdapterContext;");
let res = self.jni_ref().call_method(&self.jni_object(),"getAdapterContext",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::persistence::PersistentDataAdapterContext::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
#[repr(C)]
pub struct PersistentDataAdapterContext<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for PersistentDataAdapterContext<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for PersistentDataAdapterContext<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate PersistentDataAdapterContext from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/persistence/PersistentDataAdapterContext")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a PersistentDataAdapterContext object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> PersistentDataAdapterContext<'mc> {
	pub fn new_persistent_data_container(&self) 
-> Result<crate::persistence::PersistentDataContainer<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lcrate::persistence::PersistentDataContainer;");
let res = self.jni_ref().call_method(&self.jni_object(),"newPersistentDataContainer",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::persistence::PersistentDataContainer::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
