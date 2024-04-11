#![allow(deprecated)]
use blackboxmc_general::JNIRaw;
use blackboxmc_general::JNIInstantiatable;
use color_eyre::eyre::Result;
#[repr(C)]
pub struct FileUtil<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for FileUtil<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for FileUtil<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate FileUtil from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/util/FileUtil")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a FileUtil object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> FileUtil<'mc> {
	pub fn new(jni: &blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::util::FileUtil<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()V");
let cls = jni.find_class("org/bukkit/util/FileUtil"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.new_object(cls,
sig.as_str(),vec![]);
let res = 
jni.translate_error_no_gen(res)?;
crate::util::FileUtil::from_raw(&jni,res
)}
// SUPER CLASS: Object

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
#[repr(C)]
pub struct VoxelShape<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for VoxelShape<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for VoxelShape<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate VoxelShape from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/util/VoxelShape")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a VoxelShape object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> VoxelShape<'mc> {
	pub fn bounding_boxes(&self) 
-> Result<Vec<jni::objects::JObject<'mc>>, Box<dyn std::error::Error>>

{let sig = String::from("()LVec;");
let res = self.jni_ref().call_method(&self.jni_object(),"getBoundingBoxes",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
let mut new_vec = Vec::new();
let col = blackboxmc_java::util::JavaCollection::from_raw(&self.jni_ref(),res.l()?)?;let iter = col.iterator()?;
while iter.has_next()? {            let obj = iter.next()?;
new_vec.push(obj);
};
Ok(
new_vec
)}
	pub fn overlaps(&self,other: impl Into<crate::util::BoundingBox<'mc>>) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/util/BoundingBox;)Lbool;");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(other.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"overlaps",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
#[repr(C)]
pub struct BlockVector<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for BlockVector<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for BlockVector<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate BlockVector from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/util/BlockVector")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a BlockVector object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> BlockVector<'mc> {
	pub fn new_with_x(jni: &blackboxmc_general::SharedJNIEnv<'mc>,x: std::option::Option<f32>,y: std::option::Option<f32>,z: std::option::Option<f32>) 
-> Result<crate::util::BlockVector<'mc>, Box<dyn std::error::Error>>

{let mut args = Vec::new();
let mut sig = String::from("(");
if let Some(a) = x {
sig += "F";
let val_1 = jni::objects::JValueGen::Float(a);
args.push(val_1);
}
if let Some(a) = y {
sig += "F";
let val_2 = jni::objects::JValueGen::Float(a);
args.push(val_2);
}
if let Some(a) = z {
sig += "F";
let val_3 = jni::objects::JValueGen::Float(a);
args.push(val_3);
}
sig += ")V";
let cls = jni.find_class("org/bukkit/util/BlockVector"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.new_object(cls,
sig.as_str(),args);
let res = 
jni.translate_error_no_gen(res)?;
crate::util::BlockVector::from_raw(&jni,res
)}
	pub fn equals(&self,obj: jni::objects::JObject<'mc>) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("(Ljava/lang/Object;)Lbool;");
let val_1 = jni::objects::JValueGen::Object(obj);
let res = self.jni_ref().call_method(&self.jni_object(),"equals",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
	pub fn hash_code(&self) 
-> Result<i32, Box<dyn std::error::Error>>

{let sig = String::from("()Li32;");
let res = self.jni_ref().call_method(&self.jni_object(),"hashCode",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.i()?
)}
	pub fn clone(&self) 
-> Result<crate::util::BlockVector<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lcrate::util::BlockVector;");
let res = self.jni_ref().call_method(&self.jni_object(),"clone",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::util::BlockVector::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn deserialize(jni: &blackboxmc_general::SharedJNIEnv<'mc>,val_args: impl Into<blackboxmc_java::util::JavaMap<'mc>>) 
-> Result<crate::util::BlockVector<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("(Ljava/util/Map;)Lcrate::util::BlockVector;");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(val_args.into().jni_object().clone())});
let cls = jni.find_class("org/bukkit/util/BlockVector"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.call_static_method(cls,"deserialize",
sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
jni.translate_error(res)?;
let obj = res.l()?;
crate::util::BlockVector::from_raw(&jni,obj
)}
// SUPER CLASS: Vector

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
impl<'mc> Into<crate::util::Vector<'mc>> for BlockVector<'mc>{

fn into(self) -> crate::util::Vector<'mc> {

crate::util::Vector::from_raw(&self.jni_ref(), self.1).expect("Error converting BlockVector into crate::util::Vector")

   }
}
#[repr(C)]
pub struct BlockTransformer<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for BlockTransformer<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for BlockTransformer<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate BlockTransformer from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/util/BlockTransformer")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a BlockTransformer object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> BlockTransformer<'mc> {
	pub fn transform(&self,region: impl Into<crate::generator::LimitedRegion<'mc>>,x: i32,y: i32,z: i32,current: impl Into<crate::block::BlockState<'mc>>,state: impl Into<crate::util::BlockTransformerTransformationState<'mc>>) 
-> Result<crate::block::BlockState<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/generator/LimitedRegion;IIILorg/bukkit/block/BlockState;Lorg/bukkit/util/BlockTransformer/TransformationState;)Lcrate::block::BlockState;");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(region.into().jni_object().clone())});
let val_2 = jni::objects::JValueGen::Int(x);
let val_3 = jni::objects::JValueGen::Int(y);
let val_4 = jni::objects::JValueGen::Int(z);
let val_5 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(current.into().jni_object().clone())});
let val_6 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(state.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"transform",sig.as_str(),vec![jni::objects::JValueGen::from(val_1),jni::objects::JValueGen::from(val_2),jni::objects::JValueGen::from(val_3),jni::objects::JValueGen::from(val_4),jni::objects::JValueGen::from(val_5),jni::objects::JValueGen::from(val_6)]);
let res = 
self.jni_ref().translate_error(res)?;
crate::block::BlockState::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
#[repr(C)]
pub struct BoundingBox<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for BoundingBox<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for BoundingBox<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate BoundingBox from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/util/BoundingBox")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a BoundingBox object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> BoundingBox<'mc> {
	pub fn new_with_x1(jni: &blackboxmc_general::SharedJNIEnv<'mc>,x1: std::option::Option<f64>,y1: std::option::Option<f64>,z1: std::option::Option<f64>,x2: std::option::Option<f64>,y2: std::option::Option<f64>,z2: std::option::Option<f64>) 
-> Result<crate::util::BoundingBox<'mc>, Box<dyn std::error::Error>>

{let mut args = Vec::new();
let mut sig = String::from("(");
if let Some(a) = x1 {
sig += "D";
let val_1 = jni::objects::JValueGen::Double(a);
args.push(val_1);
}
if let Some(a) = y1 {
sig += "D";
let val_2 = jni::objects::JValueGen::Double(a);
args.push(val_2);
}
if let Some(a) = z1 {
sig += "D";
let val_3 = jni::objects::JValueGen::Double(a);
args.push(val_3);
}
if let Some(a) = x2 {
sig += "D";
let val_4 = jni::objects::JValueGen::Double(a);
args.push(val_4);
}
if let Some(a) = y2 {
sig += "D";
let val_5 = jni::objects::JValueGen::Double(a);
args.push(val_5);
}
if let Some(a) = z2 {
sig += "D";
let val_6 = jni::objects::JValueGen::Double(a);
args.push(val_6);
}
sig += ")V";
let cls = jni.find_class("org/bukkit/util/BoundingBox"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.new_object(cls,
sig.as_str(),args);
let res = 
jni.translate_error_no_gen(res)?;
crate::util::BoundingBox::from_raw(&jni,res
)}
	pub fn of_with_corner1(jni: &blackboxmc_general::SharedJNIEnv<'mc>,corner1: impl Into<crate::block::Block<'mc>>,corner2: std::option::Option<impl Into<crate::block::Block<'mc>>>) 
-> Result<crate::util::BoundingBox<'mc>, Box<dyn std::error::Error>>

{let mut args = Vec::new();
let mut sig = String::from("(");
sig += "Lorg/bukkit/block/Block;";
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(corner1.into().jni_object().clone())});
args.push(val_1);
if let Some(a) = corner2 {
sig += "Lorg/bukkit/block/Block;";
let val_2 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(a.into().jni_object().clone())});
args.push(val_2);
}
sig += ")Lorg/bukkit/util/BoundingBox;";
let cls = jni.find_class("org/bukkit/util/BoundingBox"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.call_static_method(cls,"of",
sig.as_str(),args);
let res = 
jni.translate_error(res)?;
let obj = res.l()?;
crate::util::BoundingBox::from_raw(&jni,obj
)}
	pub fn of_with_center(jni: &blackboxmc_general::SharedJNIEnv<'mc>,center: impl Into<crate::Location<'mc>>,x: std::option::Option<f64>,y: std::option::Option<f64>,z: std::option::Option<f64>) 
-> Result<crate::util::BoundingBox<'mc>, Box<dyn std::error::Error>>

{let mut args = Vec::new();
let mut sig = String::from("(");
sig += "Lorg/bukkit/Location;";
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(center.into().jni_object().clone())});
args.push(val_1);
if let Some(a) = x {
sig += "D";
let val_2 = jni::objects::JValueGen::Double(a);
args.push(val_2);
}
if let Some(a) = y {
sig += "D";
let val_3 = jni::objects::JValueGen::Double(a);
args.push(val_3);
}
if let Some(a) = z {
sig += "D";
let val_4 = jni::objects::JValueGen::Double(a);
args.push(val_4);
}
sig += ")Lorg/bukkit/util/BoundingBox;";
let cls = jni.find_class("org/bukkit/util/BoundingBox"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.call_static_method(cls,"of",
sig.as_str(),args);
let res = 
jni.translate_error(res)?;
let obj = res.l()?;
crate::util::BoundingBox::from_raw(&jni,obj
)}
	pub fn resize(&self,x1: f64,y1: f64,z1: f64,x2: f64,y2: f64,z2: f64) 
-> Result<crate::util::BoundingBox<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("(DDDDDD)Lcrate::util::BoundingBox;");
let val_1 = jni::objects::JValueGen::Double(x1);
let val_2 = jni::objects::JValueGen::Double(y1);
let val_3 = jni::objects::JValueGen::Double(z1);
let val_4 = jni::objects::JValueGen::Double(x2);
let val_5 = jni::objects::JValueGen::Double(y2);
let val_6 = jni::objects::JValueGen::Double(z2);
let res = self.jni_ref().call_method(&self.jni_object(),"resize",sig.as_str(),vec![jni::objects::JValueGen::from(val_1),jni::objects::JValueGen::from(val_2),jni::objects::JValueGen::from(val_3),jni::objects::JValueGen::from(val_4),jni::objects::JValueGen::from(val_5),jni::objects::JValueGen::from(val_6)]);
let res = 
self.jni_ref().translate_error(res)?;
crate::util::BoundingBox::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn min_x(&self) 
-> Result<f64, Box<dyn std::error::Error>>

{let sig = String::from("()Lf64;");
let res = self.jni_ref().call_method(&self.jni_object(),"getMinX",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.d()?
)}
	pub fn min_y(&self) 
-> Result<f64, Box<dyn std::error::Error>>

{let sig = String::from("()Lf64;");
let res = self.jni_ref().call_method(&self.jni_object(),"getMinY",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.d()?
)}
	pub fn min_z(&self) 
-> Result<f64, Box<dyn std::error::Error>>

{let sig = String::from("()Lf64;");
let res = self.jni_ref().call_method(&self.jni_object(),"getMinZ",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.d()?
)}
	pub fn min(&self) 
-> Result<crate::util::Vector<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lcrate::util::Vector;");
let res = self.jni_ref().call_method(&self.jni_object(),"getMin",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::util::Vector::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn max_x(&self) 
-> Result<f64, Box<dyn std::error::Error>>

{let sig = String::from("()Lf64;");
let res = self.jni_ref().call_method(&self.jni_object(),"getMaxX",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.d()?
)}
	pub fn max_y(&self) 
-> Result<f64, Box<dyn std::error::Error>>

{let sig = String::from("()Lf64;");
let res = self.jni_ref().call_method(&self.jni_object(),"getMaxY",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.d()?
)}
	pub fn max_z(&self) 
-> Result<f64, Box<dyn std::error::Error>>

{let sig = String::from("()Lf64;");
let res = self.jni_ref().call_method(&self.jni_object(),"getMaxZ",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.d()?
)}
	pub fn max(&self) 
-> Result<crate::util::Vector<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lcrate::util::Vector;");
let res = self.jni_ref().call_method(&self.jni_object(),"getMax",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::util::Vector::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn width_x(&self) 
-> Result<f64, Box<dyn std::error::Error>>

{let sig = String::from("()Lf64;");
let res = self.jni_ref().call_method(&self.jni_object(),"getWidthX",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.d()?
)}
	pub fn width_z(&self) 
-> Result<f64, Box<dyn std::error::Error>>

{let sig = String::from("()Lf64;");
let res = self.jni_ref().call_method(&self.jni_object(),"getWidthZ",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.d()?
)}
	pub fn height(&self) 
-> Result<f64, Box<dyn std::error::Error>>

{let sig = String::from("()Lf64;");
let res = self.jni_ref().call_method(&self.jni_object(),"getHeight",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.d()?
)}
	pub fn volume(&self) 
-> Result<f64, Box<dyn std::error::Error>>

{let sig = String::from("()Lf64;");
let res = self.jni_ref().call_method(&self.jni_object(),"getVolume",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.d()?
)}
	pub fn center_x(&self) 
-> Result<f64, Box<dyn std::error::Error>>

{let sig = String::from("()Lf64;");
let res = self.jni_ref().call_method(&self.jni_object(),"getCenterX",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.d()?
)}
	pub fn center_y(&self) 
-> Result<f64, Box<dyn std::error::Error>>

{let sig = String::from("()Lf64;");
let res = self.jni_ref().call_method(&self.jni_object(),"getCenterY",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.d()?
)}
	pub fn center_z(&self) 
-> Result<f64, Box<dyn std::error::Error>>

{let sig = String::from("()Lf64;");
let res = self.jni_ref().call_method(&self.jni_object(),"getCenterZ",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.d()?
)}
	pub fn center(&self) 
-> Result<crate::util::Vector<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lcrate::util::Vector;");
let res = self.jni_ref().call_method(&self.jni_object(),"getCenter",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::util::Vector::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn copy(&self,other: impl Into<crate::util::BoundingBox<'mc>>) 
-> Result<crate::util::BoundingBox<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/util/BoundingBox;)Lcrate::util::BoundingBox;");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(other.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"copy",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
crate::util::BoundingBox::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn expand_with_direction(&self,direction: impl Into<crate::util::Vector<'mc>>,expansion: std::option::Option<f64>) 
-> Result<crate::util::BoundingBox<'mc>, Box<dyn std::error::Error>>

{let mut args = Vec::new();
let mut sig = String::from("(");
sig += "Lorg/bukkit/util/Vector;";
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(direction.into().jni_object().clone())});
args.push(val_1);
if let Some(a) = expansion {
sig += "D";
let val_2 = jni::objects::JValueGen::Double(a);
args.push(val_2);
}
sig += ")Lorg/bukkit/util/BoundingBox;";
let res = self.jni_ref().call_method(&self.jni_object(),"expand",sig.as_str(),args);
let res = 
self.jni_ref().translate_error(res)?;
crate::util::BoundingBox::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn expand_with_block_face(&self,block_face: impl Into<crate::block::BlockFace<'mc>>,expansion: std::option::Option<f64>) 
-> Result<crate::util::BoundingBox<'mc>, Box<dyn std::error::Error>>

{let mut args = Vec::new();
let mut sig = String::from("(");
sig += "Lorg/bukkit/block/BlockFace;";
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(block_face.into().jni_object().clone())});
args.push(val_1);
if let Some(a) = expansion {
sig += "D";
let val_2 = jni::objects::JValueGen::Double(a);
args.push(val_2);
}
sig += ")Lorg/bukkit/util/BoundingBox;";
let res = self.jni_ref().call_method(&self.jni_object(),"expand",sig.as_str(),args);
let res = 
self.jni_ref().translate_error(res)?;
crate::util::BoundingBox::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn expand_with_x(&self,x: f64,y: std::option::Option<f64>,z: std::option::Option<f64>) 
-> Result<crate::util::BoundingBox<'mc>, Box<dyn std::error::Error>>

{let mut args = Vec::new();
let mut sig = String::from("(");
sig += "D";
let val_1 = jni::objects::JValueGen::Double(x);
args.push(val_1);
if let Some(a) = y {
sig += "D";
let val_2 = jni::objects::JValueGen::Double(a);
args.push(val_2);
}
if let Some(a) = z {
sig += "D";
let val_3 = jni::objects::JValueGen::Double(a);
args.push(val_3);
}
sig += ")Lorg/bukkit/util/BoundingBox;";
let res = self.jni_ref().call_method(&self.jni_object(),"expand",sig.as_str(),args);
let res = 
self.jni_ref().translate_error(res)?;
crate::util::BoundingBox::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn expand_with_dir_x(&self,dir_x: f64,dir_y: std::option::Option<f64>,dir_z: std::option::Option<f64>,expansion: std::option::Option<f64>) 
-> Result<crate::util::BoundingBox<'mc>, Box<dyn std::error::Error>>

{let mut args = Vec::new();
let mut sig = String::from("(");
sig += "D";
let val_1 = jni::objects::JValueGen::Double(dir_x);
args.push(val_1);
if let Some(a) = dir_y {
sig += "D";
let val_2 = jni::objects::JValueGen::Double(a);
args.push(val_2);
}
if let Some(a) = dir_z {
sig += "D";
let val_3 = jni::objects::JValueGen::Double(a);
args.push(val_3);
}
if let Some(a) = expansion {
sig += "D";
let val_4 = jni::objects::JValueGen::Double(a);
args.push(val_4);
}
sig += ")Lorg/bukkit/util/BoundingBox;";
let res = self.jni_ref().call_method(&self.jni_object(),"expand",sig.as_str(),args);
let res = 
self.jni_ref().translate_error(res)?;
crate::util::BoundingBox::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn expand_with_negative_x(&self,negative_x: f64,negative_y: std::option::Option<f64>,negative_z: std::option::Option<f64>,positive_x: std::option::Option<f64>,positive_y: std::option::Option<f64>,positive_z: std::option::Option<f64>) 
-> Result<crate::util::BoundingBox<'mc>, Box<dyn std::error::Error>>

{let mut args = Vec::new();
let mut sig = String::from("(");
sig += "D";
let val_1 = jni::objects::JValueGen::Double(negative_x);
args.push(val_1);
if let Some(a) = negative_y {
sig += "D";
let val_2 = jni::objects::JValueGen::Double(a);
args.push(val_2);
}
if let Some(a) = negative_z {
sig += "D";
let val_3 = jni::objects::JValueGen::Double(a);
args.push(val_3);
}
if let Some(a) = positive_x {
sig += "D";
let val_4 = jni::objects::JValueGen::Double(a);
args.push(val_4);
}
if let Some(a) = positive_y {
sig += "D";
let val_5 = jni::objects::JValueGen::Double(a);
args.push(val_5);
}
if let Some(a) = positive_z {
sig += "D";
let val_6 = jni::objects::JValueGen::Double(a);
args.push(val_6);
}
sig += ")Lorg/bukkit/util/BoundingBox;";
let res = self.jni_ref().call_method(&self.jni_object(),"expand",sig.as_str(),args);
let res = 
self.jni_ref().translate_error(res)?;
crate::util::BoundingBox::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn expand_directional_with_dir_x(&self,dir_x: f64,dir_y: std::option::Option<f64>,dir_z: std::option::Option<f64>) 
-> Result<crate::util::BoundingBox<'mc>, Box<dyn std::error::Error>>

{let mut args = Vec::new();
let mut sig = String::from("(");
sig += "D";
let val_1 = jni::objects::JValueGen::Double(dir_x);
args.push(val_1);
if let Some(a) = dir_y {
sig += "D";
let val_2 = jni::objects::JValueGen::Double(a);
args.push(val_2);
}
if let Some(a) = dir_z {
sig += "D";
let val_3 = jni::objects::JValueGen::Double(a);
args.push(val_3);
}
sig += ")Lorg/bukkit/util/BoundingBox;";
let res = self.jni_ref().call_method(&self.jni_object(),"expandDirectional",sig.as_str(),args);
let res = 
self.jni_ref().translate_error(res)?;
crate::util::BoundingBox::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn union_with_other(&self,other: impl Into<crate::util::BoundingBox<'mc>>) 
-> Result<crate::util::BoundingBox<'mc>, Box<dyn std::error::Error>>

{let mut args = Vec::new();
let mut sig = String::from("(");
sig += "Lorg/bukkit/util/BoundingBox;";
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(other.into().jni_object().clone())});
args.push(val_1);
sig += ")Lorg/bukkit/util/BoundingBox;";
let res = self.jni_ref().call_method(&self.jni_object(),"union",sig.as_str(),args);
let res = 
self.jni_ref().translate_error(res)?;
crate::util::BoundingBox::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn union_with_pos_x(&self,pos_x: f64,pos_y: std::option::Option<f64>,pos_z: std::option::Option<f64>) 
-> Result<crate::util::BoundingBox<'mc>, Box<dyn std::error::Error>>

{let mut args = Vec::new();
let mut sig = String::from("(");
sig += "D";
let val_1 = jni::objects::JValueGen::Double(pos_x);
args.push(val_1);
if let Some(a) = pos_y {
sig += "D";
let val_2 = jni::objects::JValueGen::Double(a);
args.push(val_2);
}
if let Some(a) = pos_z {
sig += "D";
let val_3 = jni::objects::JValueGen::Double(a);
args.push(val_3);
}
sig += ")Lorg/bukkit/util/BoundingBox;";
let res = self.jni_ref().call_method(&self.jni_object(),"union",sig.as_str(),args);
let res = 
self.jni_ref().translate_error(res)?;
crate::util::BoundingBox::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn intersection(&self,other: impl Into<crate::util::BoundingBox<'mc>>) 
-> Result<crate::util::BoundingBox<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/util/BoundingBox;)Lcrate::util::BoundingBox;");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(other.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"intersection",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
crate::util::BoundingBox::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn shift_with_shift_x(&self,shift_x: f64,shift_y: std::option::Option<f64>,shift_z: std::option::Option<f64>) 
-> Result<crate::util::BoundingBox<'mc>, Box<dyn std::error::Error>>

{let mut args = Vec::new();
let mut sig = String::from("(");
sig += "D";
let val_1 = jni::objects::JValueGen::Double(shift_x);
args.push(val_1);
if let Some(a) = shift_y {
sig += "D";
let val_2 = jni::objects::JValueGen::Double(a);
args.push(val_2);
}
if let Some(a) = shift_z {
sig += "D";
let val_3 = jni::objects::JValueGen::Double(a);
args.push(val_3);
}
sig += ")Lorg/bukkit/util/BoundingBox;";
let res = self.jni_ref().call_method(&self.jni_object(),"shift",sig.as_str(),args);
let res = 
self.jni_ref().translate_error(res)?;
crate::util::BoundingBox::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn overlaps_with_min(&self,min: impl Into<crate::util::Vector<'mc>>,max: std::option::Option<impl Into<crate::util::Vector<'mc>>>) 
-> Result<bool, Box<dyn std::error::Error>>

{let mut args = Vec::new();
let mut sig = String::from("(");
sig += "Lorg/bukkit/util/Vector;";
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(min.into().jni_object().clone())});
args.push(val_1);
if let Some(a) = max {
sig += "Lorg/bukkit/util/Vector;";
let val_2 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(a.into().jni_object().clone())});
args.push(val_2);
}
sig += ")Z";
let res = self.jni_ref().call_method(&self.jni_object(),"overlaps",sig.as_str(),args);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
	pub fn contains_with_other(&self,other: impl Into<crate::util::BoundingBox<'mc>>) 
-> Result<bool, Box<dyn std::error::Error>>

{let mut args = Vec::new();
let mut sig = String::from("(");
sig += "Lorg/bukkit/util/BoundingBox;";
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(other.into().jni_object().clone())});
args.push(val_1);
sig += ")Z";
let res = self.jni_ref().call_method(&self.jni_object(),"contains",sig.as_str(),args);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
	pub fn contains_with_min(&self,min: impl Into<crate::util::Vector<'mc>>,max: std::option::Option<impl Into<crate::util::Vector<'mc>>>) 
-> Result<bool, Box<dyn std::error::Error>>

{let mut args = Vec::new();
let mut sig = String::from("(");
sig += "Lorg/bukkit/util/Vector;";
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(min.into().jni_object().clone())});
args.push(val_1);
if let Some(a) = max {
sig += "Lorg/bukkit/util/Vector;";
let val_2 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(a.into().jni_object().clone())});
args.push(val_2);
}
sig += ")Z";
let res = self.jni_ref().call_method(&self.jni_object(),"contains",sig.as_str(),args);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
	pub fn contains_with_x(&self,x: f64,y: std::option::Option<f64>,z: std::option::Option<f64>) 
-> Result<bool, Box<dyn std::error::Error>>

{let mut args = Vec::new();
let mut sig = String::from("(");
sig += "D";
let val_1 = jni::objects::JValueGen::Double(x);
args.push(val_1);
if let Some(a) = y {
sig += "D";
let val_2 = jni::objects::JValueGen::Double(a);
args.push(val_2);
}
if let Some(a) = z {
sig += "D";
let val_3 = jni::objects::JValueGen::Double(a);
args.push(val_3);
}
sig += ")Z";
let res = self.jni_ref().call_method(&self.jni_object(),"contains",sig.as_str(),args);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
	pub fn ray_trace(&self,start: impl Into<crate::util::Vector<'mc>>,direction: impl Into<crate::util::Vector<'mc>>,max_distance: f64) 
-> Result<Option<crate::util::RayTraceResult<'mc>>, Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/util/Vector;Lorg/bukkit/util/Vector;D)Lcrate::util::RayTraceResult;");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(start.into().jni_object().clone())});
let val_2 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(direction.into().jni_object().clone())});
let val_3 = jni::objects::JValueGen::Double(max_distance);
let res = self.jni_ref().call_method(&self.jni_object(),"rayTrace",sig.as_str(),vec![jni::objects::JValueGen::from(val_1),jni::objects::JValueGen::from(val_2),jni::objects::JValueGen::from(val_3)]);
let res = 
self.jni_ref().translate_error(res)?;
if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {return Ok(None);}
Ok(
Some(
crate::util::RayTraceResult::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)?
)
)}
	pub fn hash_code(&self) 
-> Result<i32, Box<dyn std::error::Error>>

{let sig = String::from("()Li32;");
let res = self.jni_ref().call_method(&self.jni_object(),"hashCode",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.i()?
)}
	pub fn equals(&self,obj: jni::objects::JObject<'mc>) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("(Ljava/lang/Object;)Lbool;");
let val_1 = jni::objects::JValueGen::Object(obj);
let res = self.jni_ref().call_method(&self.jni_object(),"equals",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
#[doc(hidden)]
	pub fn internal_to_string(&self) 
-> Result<String, Box<dyn std::error::Error>>

{let sig = String::from("()LString;");
let res = self.jni_ref().call_method(&self.jni_object(),"toString",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string()
)}
	pub fn clone(&self) 
-> Result<crate::util::BoundingBox<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lcrate::util::BoundingBox;");
let res = self.jni_ref().call_method(&self.jni_object(),"clone",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::util::BoundingBox::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn serialize(&self) 
-> Result<blackboxmc_java::util::JavaMap<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lblackboxmc_java::util::Map;");
let res = self.jni_ref().call_method(&self.jni_object(),"serialize",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
blackboxmc_java::util::JavaMap::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn deserialize(jni: &blackboxmc_general::SharedJNIEnv<'mc>,val_args: impl Into<blackboxmc_java::util::JavaMap<'mc>>) 
-> Result<crate::util::BoundingBox<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("(Ljava/util/Map;)Lcrate::util::BoundingBox;");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(val_args.into().jni_object().clone())});
let cls = jni.find_class("org/bukkit/util/BoundingBox"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.call_static_method(cls,"deserialize",
sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
jni.translate_error(res)?;
let obj = res.l()?;
crate::util::BoundingBox::from_raw(&jni,obj
)}
// SUPER CLASS: Object

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}

        impl<'mc> std::string::ToString for BoundingBox<'mc> {
            fn to_string(&self) -> String {
                match &self.internal_to_string() {
                    Ok(a) => a.clone(),
                    Err(err) => format!(
                        "Error calling BoundingBox.toString: {}",
                        err
                    ),
                }
            }
        }
        
impl<'mc> Into<crate::configuration::serialization::ConfigurationSerializable<'mc>> for BoundingBox<'mc>{

fn into(self) -> crate::configuration::serialization::ConfigurationSerializable<'mc> {

crate::configuration::serialization::ConfigurationSerializable::from_raw(&self.jni_ref(), self.1).expect("Error converting BoundingBox into crate::configuration::serialization::ConfigurationSerializable")

   }
}
#[repr(C)]
pub struct ChatPaginatorChatPage<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for ChatPaginatorChatPage<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for ChatPaginatorChatPage<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate ChatPaginatorChatPage from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/util/ChatPaginator/ChatPage")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a ChatPaginatorChatPage object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> ChatPaginatorChatPage<'mc> {
	pub fn new(jni: &blackboxmc_general::SharedJNIEnv<'mc>,lines: impl Into<String>,page_number: i32,total_pages: i32) 
-> Result<crate::util::ChatPaginatorChatPage<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("(Ljava/lang/String;II)V");
let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(jni.new_string(lines.into())?));
let val_2 = jni::objects::JValueGen::Int(page_number);
let val_3 = jni::objects::JValueGen::Int(total_pages);
let cls = jni.find_class("org/bukkit/util/ChatPaginator/ChatPage"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.new_object(cls,
sig.as_str(),vec![jni::objects::JValueGen::from(val_1),jni::objects::JValueGen::from(val_2),jni::objects::JValueGen::from(val_3)]);
let res = 
jni.translate_error_no_gen(res)?;
crate::util::ChatPaginatorChatPage::from_raw(&jni,res
)}
	pub fn page_number(&self) 
-> Result<i32, Box<dyn std::error::Error>>

{let sig = String::from("()Li32;");
let res = self.jni_ref().call_method(&self.jni_object(),"getPageNumber",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.i()?
)}
	pub fn total_pages(&self) 
-> Result<i32, Box<dyn std::error::Error>>

{let sig = String::from("()Li32;");
let res = self.jni_ref().call_method(&self.jni_object(),"getTotalPages",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.i()?
)}
	pub fn lines(&self) 
-> Result<String, Box<dyn std::error::Error>>

{let sig = String::from("()LString;");
let res = self.jni_ref().call_method(&self.jni_object(),"getLines",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string()
)}
// SUPER CLASS: Object

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
#[repr(C)]
pub struct Consumer<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for Consumer<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for Consumer<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate Consumer from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/util/Consumer")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a Consumer object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> Consumer<'mc> {
pub fn from_extendable(
    env: &blackboxmc_general::SharedJNIEnv<'mc>,
    plugin: &'mc crate::plugin::Plugin,
    address: i32,
    lib_name: String,
    name: String,
) -> Result<Self, Box<dyn std::error::Error>> {
    let obj = unsafe { plugin.new_extendable(address, "Consumer", name, lib_name) }?;
    Self::from_raw(env, obj)
}
	pub fn accept(&self,t: jni::objects::JObject<'mc>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(Ljava/lang/Object;)L();");
let val_1 = jni::objects::JValueGen::Object(t);
let res = self.jni_ref().call_method(&self.jni_object(),"accept",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
impl<'mc> Into<blackboxmc_java::util::function::JavaConsumer<'mc>> for Consumer<'mc>{

fn into(self) -> blackboxmc_java::util::function::JavaConsumer<'mc> {

blackboxmc_java::util::function::JavaConsumer::from_raw(&self.jni_ref(), self.1).expect("Error converting Consumer into blackboxmc_java::util::function::JavaConsumer")

   }
}
#[repr(C)]
pub struct ChatPaginator<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for ChatPaginator<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for ChatPaginator<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate ChatPaginator from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/util/ChatPaginator")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a ChatPaginator object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> ChatPaginator<'mc> {
	pub fn new(jni: &blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::util::ChatPaginator<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()V");
let cls = jni.find_class("org/bukkit/util/ChatPaginator"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.new_object(cls,
sig.as_str(),vec![]);
let res = 
jni.translate_error_no_gen(res)?;
crate::util::ChatPaginator::from_raw(&jni,res
)}
	pub fn paginate_with_unpaginated_string(jni: &blackboxmc_general::SharedJNIEnv<'mc>,unpaginated_string: impl Into<String>,page_number: i32,line_length: std::option::Option<i32>,page_height: std::option::Option<i32>) 
-> Result<crate::util::ChatPaginatorChatPage<'mc>, Box<dyn std::error::Error>>

{let mut args = Vec::new();
let mut sig = String::from("(");
sig += "Ljava/lang/String;";
let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(jni.new_string(unpaginated_string.into())?));
args.push(val_1);
sig += "I";
let val_2 = jni::objects::JValueGen::Int(page_number);
args.push(val_2);
if let Some(a) = line_length {
sig += "I";
let val_3 = jni::objects::JValueGen::Int(a);
args.push(val_3);
}
if let Some(a) = page_height {
sig += "I";
let val_4 = jni::objects::JValueGen::Int(a);
args.push(val_4);
}
sig += ")Lorg/bukkit/util/ChatPaginator/ChatPage;";
let cls = jni.find_class("org/bukkit/util/ChatPaginator"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.call_static_method(cls,"paginate",
sig.as_str(),args);
let res = 
jni.translate_error(res)?;
let obj = res.l()?;
crate::util::ChatPaginatorChatPage::from_raw(&jni,obj
)}
	pub fn word_wrap(jni: &blackboxmc_general::SharedJNIEnv<'mc>,raw_string: impl Into<String>,line_length: i32) 
-> Result<String, Box<dyn std::error::Error>>

{let sig = String::from("(Ljava/lang/String;I)LString;");
let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(jni.new_string(raw_string.into())?));
let val_2 = jni::objects::JValueGen::Int(line_length);
let cls = jni.find_class("org/bukkit/util/ChatPaginator"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.call_static_method(cls,"wordWrap",
sig.as_str(),vec![jni::objects::JValueGen::from(val_1),jni::objects::JValueGen::from(val_2)]);
let res = 
jni.translate_error(res)?;
Ok(
jni.get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string()
)}
// SUPER CLASS: Object

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
#[repr(C)]
pub struct BlockIterator<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for BlockIterator<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for BlockIterator<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate BlockIterator from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/util/BlockIterator")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a BlockIterator object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> BlockIterator<'mc> {
	pub fn new_with_entity(jni: &blackboxmc_general::SharedJNIEnv<'mc>,entity: impl Into<crate::entity::LivingEntity<'mc>>,max_distance: std::option::Option<i32>) 
-> Result<crate::util::BlockIterator<'mc>, Box<dyn std::error::Error>>

{let mut args = Vec::new();
let mut sig = String::from("(");
sig += "Lorg/bukkit/entity/LivingEntity;";
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(entity.into().jni_object().clone())});
args.push(val_1);
if let Some(a) = max_distance {
sig += "I";
let val_2 = jni::objects::JValueGen::Int(a);
args.push(val_2);
}
sig += ")V";
let cls = jni.find_class("org/bukkit/util/BlockIterator"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.new_object(cls,
sig.as_str(),args);
let res = 
jni.translate_error_no_gen(res)?;
crate::util::BlockIterator::from_raw(&jni,res
)}
	pub fn new_with_loc(jni: &blackboxmc_general::SharedJNIEnv<'mc>,loc: impl Into<crate::Location<'mc>>,y_offset: std::option::Option<f64>,max_distance: std::option::Option<i32>) 
-> Result<crate::util::BlockIterator<'mc>, Box<dyn std::error::Error>>

{let mut args = Vec::new();
let mut sig = String::from("(");
sig += "Lorg/bukkit/Location;";
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(loc.into().jni_object().clone())});
args.push(val_1);
if let Some(a) = y_offset {
sig += "D";
let val_2 = jni::objects::JValueGen::Double(a);
args.push(val_2);
}
if let Some(a) = max_distance {
sig += "I";
let val_3 = jni::objects::JValueGen::Int(a);
args.push(val_3);
}
sig += ")V";
let cls = jni.find_class("org/bukkit/util/BlockIterator"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.new_object(cls,
sig.as_str(),args);
let res = 
jni.translate_error_no_gen(res)?;
crate::util::BlockIterator::from_raw(&jni,res
)}
	pub fn new_with_world(jni: &blackboxmc_general::SharedJNIEnv<'mc>,world: impl Into<crate::World<'mc>>,start: std::option::Option<impl Into<crate::util::Vector<'mc>>>,direction: std::option::Option<impl Into<crate::util::Vector<'mc>>>,y_offset: std::option::Option<f64>,max_distance: std::option::Option<i32>) 
-> Result<crate::util::BlockIterator<'mc>, Box<dyn std::error::Error>>

{let mut args = Vec::new();
let mut sig = String::from("(");
sig += "Lorg/bukkit/World;";
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(world.into().jni_object().clone())});
args.push(val_1);
if let Some(a) = start {
sig += "Lorg/bukkit/util/Vector;";
let val_2 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(a.into().jni_object().clone())});
args.push(val_2);
}
if let Some(a) = direction {
sig += "Lorg/bukkit/util/Vector;";
let val_3 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(a.into().jni_object().clone())});
args.push(val_3);
}
if let Some(a) = y_offset {
sig += "D";
let val_4 = jni::objects::JValueGen::Double(a);
args.push(val_4);
}
if let Some(a) = max_distance {
sig += "I";
let val_5 = jni::objects::JValueGen::Int(a);
args.push(val_5);
}
sig += ")V";
let cls = jni.find_class("org/bukkit/util/BlockIterator"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.new_object(cls,
sig.as_str(),args);
let res = 
jni.translate_error_no_gen(res)?;
crate::util::BlockIterator::from_raw(&jni,res
)}
	pub fn has_next(&self) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("()Lbool;");
let res = self.jni_ref().call_method(&self.jni_object(),"hasNext",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
	pub fn next(&self) 
-> Result<crate::block::Block<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lcrate::block::Block;");
let res = self.jni_ref().call_method(&self.jni_object(),"next",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::block::Block::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn remove(&self) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("()L();");
let res = self.jni_ref().call_method(&self.jni_object(),"remove",sig.as_str(),vec![]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
// SUPER CLASS: Object

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
impl<'mc> Into<blackboxmc_java::util::JavaIterator<'mc>> for BlockIterator<'mc>{

fn into(self) -> blackboxmc_java::util::JavaIterator<'mc> {

blackboxmc_java::util::JavaIterator::from_raw(&self.jni_ref(), self.1).expect("Error converting BlockIterator into blackboxmc_java::util::JavaIterator")

   }
}
#[repr(C)]
pub struct Vector<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for Vector<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for Vector<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate Vector from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/util/Vector")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a Vector object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> Vector<'mc> {
	pub fn new_with_x(jni: &blackboxmc_general::SharedJNIEnv<'mc>,x: std::option::Option<f32>,y: std::option::Option<f32>,z: std::option::Option<f32>) 
-> Result<crate::util::Vector<'mc>, Box<dyn std::error::Error>>

{let mut args = Vec::new();
let mut sig = String::from("(");
if let Some(a) = x {
sig += "F";
let val_1 = jni::objects::JValueGen::Float(a);
args.push(val_1);
}
if let Some(a) = y {
sig += "F";
let val_2 = jni::objects::JValueGen::Float(a);
args.push(val_2);
}
if let Some(a) = z {
sig += "F";
let val_3 = jni::objects::JValueGen::Float(a);
args.push(val_3);
}
sig += ")V";
let cls = jni.find_class("org/bukkit/util/Vector"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.new_object(cls,
sig.as_str(),args);
let res = 
jni.translate_error_no_gen(res)?;
crate::util::Vector::from_raw(&jni,res
)}
	pub fn add(&self,vec: impl Into<crate::util::Vector<'mc>>) 
-> Result<crate::util::Vector<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/util/Vector;)Lcrate::util::Vector;");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(vec.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"add",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
crate::util::Vector::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn subtract(&self,vec: impl Into<crate::util::Vector<'mc>>) 
-> Result<crate::util::Vector<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/util/Vector;)Lcrate::util::Vector;");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(vec.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"subtract",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
crate::util::Vector::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn multiply_with_m(&self,m: f32) 
-> Result<crate::util::Vector<'mc>, Box<dyn std::error::Error>>

{let mut args = Vec::new();
let mut sig = String::from("(");
sig += "F";
let val_1 = jni::objects::JValueGen::Float(m);
args.push(val_1);
sig += ")Lorg/bukkit/util/Vector;";
let res = self.jni_ref().call_method(&self.jni_object(),"multiply",sig.as_str(),args);
let res = 
self.jni_ref().translate_error(res)?;
crate::util::Vector::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn divide(&self,vec: impl Into<crate::util::Vector<'mc>>) 
-> Result<crate::util::Vector<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/util/Vector;)Lcrate::util::Vector;");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(vec.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"divide",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
crate::util::Vector::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn copy(&self,vec: impl Into<crate::util::Vector<'mc>>) 
-> Result<crate::util::Vector<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/util/Vector;)Lcrate::util::Vector;");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(vec.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"copy",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
crate::util::Vector::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn length(&self) 
-> Result<f64, Box<dyn std::error::Error>>

{let sig = String::from("()Lf64;");
let res = self.jni_ref().call_method(&self.jni_object(),"length",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.d()?
)}
	pub fn length_squared(&self) 
-> Result<f64, Box<dyn std::error::Error>>

{let sig = String::from("()Lf64;");
let res = self.jni_ref().call_method(&self.jni_object(),"lengthSquared",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.d()?
)}
	pub fn distance(&self,o: impl Into<crate::util::Vector<'mc>>) 
-> Result<f64, Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/util/Vector;)Lf64;");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(o.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"distance",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.d()?
)}
	pub fn distance_squared(&self,o: impl Into<crate::util::Vector<'mc>>) 
-> Result<f64, Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/util/Vector;)Lf64;");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(o.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"distanceSquared",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.d()?
)}
	pub fn angle(&self,other: impl Into<crate::util::Vector<'mc>>) 
-> Result<f32, Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/util/Vector;)Lf32;");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(other.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"angle",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.f()?
)}
	pub fn midpoint(&self,other: impl Into<crate::util::Vector<'mc>>) 
-> Result<crate::util::Vector<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/util/Vector;)Lcrate::util::Vector;");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(other.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"midpoint",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
crate::util::Vector::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn get_midpoint(&self,other: impl Into<crate::util::Vector<'mc>>) 
-> Result<crate::util::Vector<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/util/Vector;)Lcrate::util::Vector;");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(other.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"getMidpoint",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
crate::util::Vector::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn dot(&self,other: impl Into<crate::util::Vector<'mc>>) 
-> Result<f64, Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/util/Vector;)Lf64;");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(other.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"dot",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.d()?
)}
	pub fn cross_product(&self,o: impl Into<crate::util::Vector<'mc>>) 
-> Result<crate::util::Vector<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/util/Vector;)Lcrate::util::Vector;");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(o.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"crossProduct",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
crate::util::Vector::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn get_cross_product(&self,o: impl Into<crate::util::Vector<'mc>>) 
-> Result<crate::util::Vector<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/util/Vector;)Lcrate::util::Vector;");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(o.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"getCrossProduct",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
crate::util::Vector::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn normalize(&self) 
-> Result<crate::util::Vector<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lcrate::util::Vector;");
let res = self.jni_ref().call_method(&self.jni_object(),"normalize",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::util::Vector::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn zero(&self) 
-> Result<crate::util::Vector<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lcrate::util::Vector;");
let res = self.jni_ref().call_method(&self.jni_object(),"zero",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::util::Vector::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn is_zero(&self) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("()Lbool;");
let res = self.jni_ref().call_method(&self.jni_object(),"isZero",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
	pub fn is_in_aabb(&self,min: impl Into<crate::util::Vector<'mc>>,max: impl Into<crate::util::Vector<'mc>>) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/util/Vector;Lorg/bukkit/util/Vector;)Lbool;");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(min.into().jni_object().clone())});
let val_2 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(max.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"isInAABB",sig.as_str(),vec![jni::objects::JValueGen::from(val_1),jni::objects::JValueGen::from(val_2)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
	pub fn is_in_sphere(&self,origin: impl Into<crate::util::Vector<'mc>>,radius: f64) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/util/Vector;D)Lbool;");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(origin.into().jni_object().clone())});
let val_2 = jni::objects::JValueGen::Double(radius);
let res = self.jni_ref().call_method(&self.jni_object(),"isInSphere",sig.as_str(),vec![jni::objects::JValueGen::from(val_1),jni::objects::JValueGen::from(val_2)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
	pub fn is_normalized(&self) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("()Lbool;");
let res = self.jni_ref().call_method(&self.jni_object(),"isNormalized",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
	pub fn rotate_around_x(&self,angle: f64) 
-> Result<crate::util::Vector<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("(D)Lcrate::util::Vector;");
let val_1 = jni::objects::JValueGen::Double(angle);
let res = self.jni_ref().call_method(&self.jni_object(),"rotateAroundX",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
crate::util::Vector::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn rotate_around_y(&self,angle: f64) 
-> Result<crate::util::Vector<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("(D)Lcrate::util::Vector;");
let val_1 = jni::objects::JValueGen::Double(angle);
let res = self.jni_ref().call_method(&self.jni_object(),"rotateAroundY",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
crate::util::Vector::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn rotate_around_z(&self,angle: f64) 
-> Result<crate::util::Vector<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("(D)Lcrate::util::Vector;");
let val_1 = jni::objects::JValueGen::Double(angle);
let res = self.jni_ref().call_method(&self.jni_object(),"rotateAroundZ",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
crate::util::Vector::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn rotate_around_axis(&self,axis: impl Into<crate::util::Vector<'mc>>,angle: f64) 
-> Result<crate::util::Vector<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/util/Vector;D)Lcrate::util::Vector;");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(axis.into().jni_object().clone())});
let val_2 = jni::objects::JValueGen::Double(angle);
let res = self.jni_ref().call_method(&self.jni_object(),"rotateAroundAxis",sig.as_str(),vec![jni::objects::JValueGen::from(val_1),jni::objects::JValueGen::from(val_2)]);
let res = 
self.jni_ref().translate_error(res)?;
crate::util::Vector::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn rotate_around_non_unit_axis(&self,axis: impl Into<crate::util::Vector<'mc>>,angle: f64) 
-> Result<crate::util::Vector<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/util/Vector;D)Lcrate::util::Vector;");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(axis.into().jni_object().clone())});
let val_2 = jni::objects::JValueGen::Double(angle);
let res = self.jni_ref().call_method(&self.jni_object(),"rotateAroundNonUnitAxis",sig.as_str(),vec![jni::objects::JValueGen::from(val_1),jni::objects::JValueGen::from(val_2)]);
let res = 
self.jni_ref().translate_error(res)?;
crate::util::Vector::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn x(&self) 
-> Result<f64, Box<dyn std::error::Error>>

{let sig = String::from("()Lf64;");
let res = self.jni_ref().call_method(&self.jni_object(),"getX",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.d()?
)}
	pub fn block_x(&self) 
-> Result<i32, Box<dyn std::error::Error>>

{let sig = String::from("()Li32;");
let res = self.jni_ref().call_method(&self.jni_object(),"getBlockX",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.i()?
)}
	pub fn y(&self) 
-> Result<f64, Box<dyn std::error::Error>>

{let sig = String::from("()Lf64;");
let res = self.jni_ref().call_method(&self.jni_object(),"getY",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.d()?
)}
	pub fn block_y(&self) 
-> Result<i32, Box<dyn std::error::Error>>

{let sig = String::from("()Li32;");
let res = self.jni_ref().call_method(&self.jni_object(),"getBlockY",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.i()?
)}
	pub fn z(&self) 
-> Result<f64, Box<dyn std::error::Error>>

{let sig = String::from("()Lf64;");
let res = self.jni_ref().call_method(&self.jni_object(),"getZ",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.d()?
)}
	pub fn block_z(&self) 
-> Result<i32, Box<dyn std::error::Error>>

{let sig = String::from("()Li32;");
let res = self.jni_ref().call_method(&self.jni_object(),"getBlockZ",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.i()?
)}
	pub fn set_x_with_x(&self,x: f32) 
-> Result<crate::util::Vector<'mc>, Box<dyn std::error::Error>>

{let mut args = Vec::new();
let mut sig = String::from("(");
sig += "F";
let val_1 = jni::objects::JValueGen::Float(x);
args.push(val_1);
sig += ")Lorg/bukkit/util/Vector;";
let res = self.jni_ref().call_method(&self.jni_object(),"setX",sig.as_str(),args);
let res = 
self.jni_ref().translate_error(res)?;
crate::util::Vector::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn set_y_with_y(&self,y: f32) 
-> Result<crate::util::Vector<'mc>, Box<dyn std::error::Error>>

{let mut args = Vec::new();
let mut sig = String::from("(");
sig += "F";
let val_1 = jni::objects::JValueGen::Float(y);
args.push(val_1);
sig += ")Lorg/bukkit/util/Vector;";
let res = self.jni_ref().call_method(&self.jni_object(),"setY",sig.as_str(),args);
let res = 
self.jni_ref().translate_error(res)?;
crate::util::Vector::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn set_z_with_z(&self,z: f32) 
-> Result<crate::util::Vector<'mc>, Box<dyn std::error::Error>>

{let mut args = Vec::new();
let mut sig = String::from("(");
sig += "F";
let val_1 = jni::objects::JValueGen::Float(z);
args.push(val_1);
sig += ")Lorg/bukkit/util/Vector;";
let res = self.jni_ref().call_method(&self.jni_object(),"setZ",sig.as_str(),args);
let res = 
self.jni_ref().translate_error(res)?;
crate::util::Vector::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn equals(&self,obj: jni::objects::JObject<'mc>) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("(Ljava/lang/Object;)Lbool;");
let val_1 = jni::objects::JValueGen::Object(obj);
let res = self.jni_ref().call_method(&self.jni_object(),"equals",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
	pub fn hash_code(&self) 
-> Result<i32, Box<dyn std::error::Error>>

{let sig = String::from("()Li32;");
let res = self.jni_ref().call_method(&self.jni_object(),"hashCode",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.i()?
)}
	pub fn clone(&self) 
-> Result<crate::util::Vector<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lcrate::util::Vector;");
let res = self.jni_ref().call_method(&self.jni_object(),"clone",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::util::Vector::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
#[doc(hidden)]
	pub fn internal_to_string(&self) 
-> Result<String, Box<dyn std::error::Error>>

{let sig = String::from("()LString;");
let res = self.jni_ref().call_method(&self.jni_object(),"toString",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string()
)}
	pub fn to_location_with_world(&self,world: impl Into<crate::World<'mc>>,yaw: std::option::Option<f32>,pitch: std::option::Option<f32>) 
-> Result<crate::Location<'mc>, Box<dyn std::error::Error>>

{let mut args = Vec::new();
let mut sig = String::from("(");
sig += "Lorg/bukkit/World;";
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(world.into().jni_object().clone())});
args.push(val_1);
if let Some(a) = yaw {
sig += "F";
let val_2 = jni::objects::JValueGen::Float(a);
args.push(val_2);
}
if let Some(a) = pitch {
sig += "F";
let val_3 = jni::objects::JValueGen::Float(a);
args.push(val_3);
}
sig += ")Lorg/bukkit/Location;";
let res = self.jni_ref().call_method(&self.jni_object(),"toLocation",sig.as_str(),args);
let res = 
self.jni_ref().translate_error(res)?;
crate::Location::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn to_block_vector(&self) 
-> Result<crate::util::BlockVector<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lcrate::util::BlockVector;");
let res = self.jni_ref().call_method(&self.jni_object(),"toBlockVector",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::util::BlockVector::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn to_vector3f(&self) 
-> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Ljni::objects::JObject;");
let res = self.jni_ref().call_method(&self.jni_object(),"toVector3f",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.l()?
)}
	pub fn to_vector3d(&self) 
-> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Ljni::objects::JObject;");
let res = self.jni_ref().call_method(&self.jni_object(),"toVector3d",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.l()?
)}
	pub fn to_vector3i_with_rounding_mode(&self,rounding_mode: std::option::Option<i32>) 
-> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>>

{let mut args = Vec::new();
let mut sig = String::from("(");
if let Some(a) = rounding_mode {
sig += "I";
let val_1 = jni::objects::JValueGen::Int(a);
args.push(val_1);
}
sig += ")Lorg/joml/Vector3i;";
let res = self.jni_ref().call_method(&self.jni_object(),"toVector3i",sig.as_str(),args);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.l()?
)}
	pub fn check_finite(&self) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("()L();");
let res = self.jni_ref().call_method(&self.jni_object(),"checkFinite",sig.as_str(),vec![]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
	pub fn epsilon(jni: &blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<f64, Box<dyn std::error::Error>>

{let sig = String::from("()Lf64;");
let cls = jni.find_class("org/bukkit/util/Vector"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.call_static_method(cls,"getEpsilon",
sig.as_str(),vec![]);
let res = 
jni.translate_error(res)?;
Ok(
res.d()?
)}
	pub fn get_minimum(jni: &blackboxmc_general::SharedJNIEnv<'mc>,v1: impl Into<crate::util::Vector<'mc>>,v2: impl Into<crate::util::Vector<'mc>>) 
-> Result<crate::util::Vector<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/util/Vector;Lorg/bukkit/util/Vector;)Lcrate::util::Vector;");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(v1.into().jni_object().clone())});
let val_2 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(v2.into().jni_object().clone())});
let cls = jni.find_class("org/bukkit/util/Vector"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.call_static_method(cls,"getMinimum",
sig.as_str(),vec![jni::objects::JValueGen::from(val_1),jni::objects::JValueGen::from(val_2)]);
let res = 
jni.translate_error(res)?;
let obj = res.l()?;
crate::util::Vector::from_raw(&jni,obj
)}
	pub fn get_maximum(jni: &blackboxmc_general::SharedJNIEnv<'mc>,v1: impl Into<crate::util::Vector<'mc>>,v2: impl Into<crate::util::Vector<'mc>>) 
-> Result<crate::util::Vector<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/util/Vector;Lorg/bukkit/util/Vector;)Lcrate::util::Vector;");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(v1.into().jni_object().clone())});
let val_2 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(v2.into().jni_object().clone())});
let cls = jni.find_class("org/bukkit/util/Vector"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.call_static_method(cls,"getMaximum",
sig.as_str(),vec![jni::objects::JValueGen::from(val_1),jni::objects::JValueGen::from(val_2)]);
let res = 
jni.translate_error(res)?;
let obj = res.l()?;
crate::util::Vector::from_raw(&jni,obj
)}
	pub fn random(jni: &blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::util::Vector<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lcrate::util::Vector;");
let cls = jni.find_class("org/bukkit/util/Vector"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.call_static_method(cls,"getRandom",
sig.as_str(),vec![]);
let res = 
jni.translate_error(res)?;
let obj = res.l()?;
crate::util::Vector::from_raw(&jni,obj
)}
	pub fn from_joml_with_vector(jni: &blackboxmc_general::SharedJNIEnv<'mc>,vector: jni::objects::JObject<'mc>) 
-> Result<crate::util::Vector<'mc>, Box<dyn std::error::Error>>

{let mut args = Vec::new();
let mut sig = String::from("(");
sig += "Lorg/joml/Vector3ic;";
let val_1 = jni::objects::JValueGen::Object(vector);
args.push(val_1);
sig += ")Lorg/bukkit/util/Vector;";
let cls = jni.find_class("org/bukkit/util/Vector"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.call_static_method(cls,"fromJOML",
sig.as_str(),args);
let res = 
jni.translate_error(res)?;
let obj = res.l()?;
crate::util::Vector::from_raw(&jni,obj
)}
	pub fn serialize(&self) 
-> Result<blackboxmc_java::util::JavaMap<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lblackboxmc_java::util::Map;");
let res = self.jni_ref().call_method(&self.jni_object(),"serialize",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
blackboxmc_java::util::JavaMap::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn deserialize(jni: &blackboxmc_general::SharedJNIEnv<'mc>,val_args: impl Into<blackboxmc_java::util::JavaMap<'mc>>) 
-> Result<crate::util::Vector<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("(Ljava/util/Map;)Lcrate::util::Vector;");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(val_args.into().jni_object().clone())});
let cls = jni.find_class("org/bukkit/util/Vector"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.call_static_method(cls,"deserialize",
sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
jni.translate_error(res)?;
let obj = res.l()?;
crate::util::Vector::from_raw(&jni,obj
)}
// SUPER CLASS: Object

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}

        impl<'mc> std::string::ToString for Vector<'mc> {
            fn to_string(&self) -> String {
                match &self.internal_to_string() {
                    Ok(a) => a.clone(),
                    Err(err) => format!(
                        "Error calling Vector.toString: {}",
                        err
                    ),
                }
            }
        }
        
impl<'mc> Into<crate::configuration::serialization::ConfigurationSerializable<'mc>> for Vector<'mc>{

fn into(self) -> crate::configuration::serialization::ConfigurationSerializable<'mc> {

crate::configuration::serialization::ConfigurationSerializable::from_raw(&self.jni_ref(), self.1).expect("Error converting Vector into crate::configuration::serialization::ConfigurationSerializable")

   }
}
#[repr(C)]
pub struct EntityTransformer<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for EntityTransformer<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for EntityTransformer<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate EntityTransformer from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/util/EntityTransformer")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a EntityTransformer object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> EntityTransformer<'mc> {
	pub fn transform(&self,region: impl Into<crate::generator::LimitedRegion<'mc>>,x: i32,y: i32,z: i32,entity: impl Into<crate::entity::Entity<'mc>>,allowed_to_spawn: bool) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/generator/LimitedRegion;IIILorg/bukkit/entity/Entity;Z)Lbool;");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(region.into().jni_object().clone())});
let val_2 = jni::objects::JValueGen::Int(x);
let val_3 = jni::objects::JValueGen::Int(y);
let val_4 = jni::objects::JValueGen::Int(z);
let val_5 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(entity.into().jni_object().clone())});
let val_6 = jni::objects::JValueGen::Bool(allowed_to_spawn.into());
let res = self.jni_ref().call_method(&self.jni_object(),"transform",sig.as_str(),vec![jni::objects::JValueGen::from(val_1),jni::objects::JValueGen::from(val_2),jni::objects::JValueGen::from(val_3),jni::objects::JValueGen::from(val_4),jni::objects::JValueGen::from(val_5),jni::objects::JValueGen::from(val_6)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
#[repr(C)]
pub struct BiomeSearchResult<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for BiomeSearchResult<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for BiomeSearchResult<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate BiomeSearchResult from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/util/BiomeSearchResult")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a BiomeSearchResult object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> BiomeSearchResult<'mc> {
	pub fn biome(&self) 
-> Result<crate::block::Biome<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lcrate::block::Biome;");
let res = self.jni_ref().call_method(&self.jni_object(),"getBiome",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::block::Biome::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn location(&self) 
-> Result<crate::Location<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lcrate::Location;");
let res = self.jni_ref().call_method(&self.jni_object(),"getLocation",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::Location::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
#[repr(C)]
pub struct Transformation<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for Transformation<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for Transformation<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate Transformation from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/util/Transformation")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a Transformation object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> Transformation<'mc> {
	pub fn new_with_translation(jni: &blackboxmc_general::SharedJNIEnv<'mc>,translation: jni::objects::JObject<'mc>,left_rotation: jni::objects::JObject<'mc>,scale: jni::objects::JObject<'mc>,right_rotation: jni::objects::JObject<'mc>) 
-> Result<crate::util::Transformation<'mc>, Box<dyn std::error::Error>>

{let mut args = Vec::new();
let mut sig = String::from("(");
sig += "Lorg/joml/Vector3f;";
let val_1 = jni::objects::JValueGen::Object(translation);
args.push(val_1);
sig += "Lorg/joml/Quaternionf;";
let val_2 = jni::objects::JValueGen::Object(left_rotation);
args.push(val_2);
sig += "Lorg/joml/Vector3f;";
let val_3 = jni::objects::JValueGen::Object(scale);
args.push(val_3);
sig += "Lorg/joml/Quaternionf;";
let val_4 = jni::objects::JValueGen::Object(right_rotation);
args.push(val_4);
sig += ")V";
let cls = jni.find_class("org/bukkit/util/Transformation"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.new_object(cls,
sig.as_str(),args);
let res = 
jni.translate_error_no_gen(res)?;
crate::util::Transformation::from_raw(&jni,res
)}
	pub fn translation(&self) 
-> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Ljni::objects::JObject;");
let res = self.jni_ref().call_method(&self.jni_object(),"getTranslation",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.l()?
)}
	pub fn left_rotation(&self) 
-> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Ljni::objects::JObject;");
let res = self.jni_ref().call_method(&self.jni_object(),"getLeftRotation",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.l()?
)}
	pub fn scale(&self) 
-> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Ljni::objects::JObject;");
let res = self.jni_ref().call_method(&self.jni_object(),"getScale",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.l()?
)}
	pub fn right_rotation(&self) 
-> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Ljni::objects::JObject;");
let res = self.jni_ref().call_method(&self.jni_object(),"getRightRotation",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.l()?
)}
	pub fn hash_code(&self) 
-> Result<i32, Box<dyn std::error::Error>>

{let sig = String::from("()Li32;");
let res = self.jni_ref().call_method(&self.jni_object(),"hashCode",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.i()?
)}
	pub fn equals(&self,obj: jni::objects::JObject<'mc>) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("(Ljava/lang/Object;)Lbool;");
let val_1 = jni::objects::JValueGen::Object(obj);
let res = self.jni_ref().call_method(&self.jni_object(),"equals",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
#[doc(hidden)]
	pub fn internal_to_string(&self) 
-> Result<String, Box<dyn std::error::Error>>

{let sig = String::from("()LString;");
let res = self.jni_ref().call_method(&self.jni_object(),"toString",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string()
)}
// SUPER CLASS: Object

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}

        impl<'mc> std::string::ToString for Transformation<'mc> {
            fn to_string(&self) -> String {
                match &self.internal_to_string() {
                    Ok(a) => a.clone(),
                    Err(err) => format!(
                        "Error calling Transformation.toString: {}",
                        err
                    ),
                }
            }
        }
        
#[repr(C)]
pub struct CachedServerIcon<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for CachedServerIcon<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for CachedServerIcon<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate CachedServerIcon from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/util/CachedServerIcon")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a CachedServerIcon object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> CachedServerIcon<'mc> {

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
#[repr(C)]
pub struct NumberConversions<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for NumberConversions<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for NumberConversions<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate NumberConversions from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/util/NumberConversions")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a NumberConversions object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> NumberConversions<'mc> {
	pub fn floor(jni: &blackboxmc_general::SharedJNIEnv<'mc>,num: f64) 
-> Result<i32, Box<dyn std::error::Error>>

{let sig = String::from("(D)Li32;");
let val_1 = jni::objects::JValueGen::Double(num);
let cls = jni.find_class("org/bukkit/util/NumberConversions"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.call_static_method(cls,"floor",
sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
jni.translate_error(res)?;
Ok(
res.i()?
)}
	pub fn ceil(jni: &blackboxmc_general::SharedJNIEnv<'mc>,num: f64) 
-> Result<i32, Box<dyn std::error::Error>>

{let sig = String::from("(D)Li32;");
let val_1 = jni::objects::JValueGen::Double(num);
let cls = jni.find_class("org/bukkit/util/NumberConversions"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.call_static_method(cls,"ceil",
sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
jni.translate_error(res)?;
Ok(
res.i()?
)}
	pub fn round(jni: &blackboxmc_general::SharedJNIEnv<'mc>,num: f64) 
-> Result<i32, Box<dyn std::error::Error>>

{let sig = String::from("(D)Li32;");
let val_1 = jni::objects::JValueGen::Double(num);
let cls = jni.find_class("org/bukkit/util/NumberConversions"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.call_static_method(cls,"round",
sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
jni.translate_error(res)?;
Ok(
res.i()?
)}
	pub fn square(jni: &blackboxmc_general::SharedJNIEnv<'mc>,num: f64) 
-> Result<f64, Box<dyn std::error::Error>>

{let sig = String::from("(D)Lf64;");
let val_1 = jni::objects::JValueGen::Double(num);
let cls = jni.find_class("org/bukkit/util/NumberConversions"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.call_static_method(cls,"square",
sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
jni.translate_error(res)?;
Ok(
res.d()?
)}
	pub fn to_int(jni: &blackboxmc_general::SharedJNIEnv<'mc>,object: jni::objects::JObject<'mc>) 
-> Result<i32, Box<dyn std::error::Error>>

{let sig = String::from("(Ljava/lang/Object;)Li32;");
let val_1 = jni::objects::JValueGen::Object(object);
let cls = jni.find_class("org/bukkit/util/NumberConversions"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.call_static_method(cls,"toInt",
sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
jni.translate_error(res)?;
Ok(
res.i()?
)}
	pub fn to_float(jni: &blackboxmc_general::SharedJNIEnv<'mc>,object: jni::objects::JObject<'mc>) 
-> Result<f32, Box<dyn std::error::Error>>

{let sig = String::from("(Ljava/lang/Object;)Lf32;");
let val_1 = jni::objects::JValueGen::Object(object);
let cls = jni.find_class("org/bukkit/util/NumberConversions"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.call_static_method(cls,"toFloat",
sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
jni.translate_error(res)?;
Ok(
res.f()?
)}
	pub fn to_double(jni: &blackboxmc_general::SharedJNIEnv<'mc>,object: jni::objects::JObject<'mc>) 
-> Result<f64, Box<dyn std::error::Error>>

{let sig = String::from("(Ljava/lang/Object;)Lf64;");
let val_1 = jni::objects::JValueGen::Object(object);
let cls = jni.find_class("org/bukkit/util/NumberConversions"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.call_static_method(cls,"toDouble",
sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
jni.translate_error(res)?;
Ok(
res.d()?
)}
	pub fn to_long(jni: &blackboxmc_general::SharedJNIEnv<'mc>,object: jni::objects::JObject<'mc>) 
-> Result<i64, Box<dyn std::error::Error>>

{let sig = String::from("(Ljava/lang/Object;)Li64;");
let val_1 = jni::objects::JValueGen::Object(object);
let cls = jni.find_class("org/bukkit/util/NumberConversions"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.call_static_method(cls,"toLong",
sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
jni.translate_error(res)?;
Ok(
res.j()?
)}
	pub fn to_short(jni: &blackboxmc_general::SharedJNIEnv<'mc>,object: jni::objects::JObject<'mc>) 
-> Result<i16, Box<dyn std::error::Error>>

{let sig = String::from("(Ljava/lang/Object;)Li16;");
let val_1 = jni::objects::JValueGen::Object(object);
let cls = jni.find_class("org/bukkit/util/NumberConversions"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.call_static_method(cls,"toShort",
sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
jni.translate_error(res)?;
Ok(
res.s()?
)}
	pub fn to_byte(jni: &blackboxmc_general::SharedJNIEnv<'mc>,object: jni::objects::JObject<'mc>) 
-> Result<i8, Box<dyn std::error::Error>>

{let sig = String::from("(Ljava/lang/Object;)Li8;");
let val_1 = jni::objects::JValueGen::Object(object);
let cls = jni.find_class("org/bukkit/util/NumberConversions"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.call_static_method(cls,"toByte",
sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
jni.translate_error(res)?;
Ok(
res.b()?
)}
	pub fn is_finite_with_f(jni: &blackboxmc_general::SharedJNIEnv<'mc>,f: f32) 
-> Result<bool, Box<dyn std::error::Error>>

{let mut args = Vec::new();
let mut sig = String::from("(");
sig += "F";
let val_1 = jni::objects::JValueGen::Float(f);
args.push(val_1);
sig += ")Z";
let cls = jni.find_class("org/bukkit/util/NumberConversions"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.call_static_method(cls,"isFinite",
sig.as_str(),args);
let res = 
jni.translate_error(res)?;
Ok(
res.z()?
)}
	pub fn check_finite_with_d(jni: &blackboxmc_general::SharedJNIEnv<'mc>,d: f32,message: impl Into<String>) 
-> Result<(), Box<dyn std::error::Error>>

{let mut args = Vec::new();
let mut sig = String::from("(");
sig += "F";
let val_1 = jni::objects::JValueGen::Float(d);
args.push(val_1);
sig += "Ljava/lang/String;";
let val_2 = jni::objects::JValueGen::Object(jni::objects::JObject::from(jni.new_string(message.into())?));
args.push(val_2);
sig += ")V";
let cls = jni.find_class("org/bukkit/util/NumberConversions"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.call_static_method(cls,"checkFinite",
sig.as_str(),args);
jni.translate_error(res)?;
Ok(
()
)}
// SUPER CLASS: Object

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
#[repr(C)]
pub struct EulerAngle<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for EulerAngle<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for EulerAngle<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate EulerAngle from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/util/EulerAngle")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a EulerAngle object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> EulerAngle<'mc> {
	pub fn new(jni: &blackboxmc_general::SharedJNIEnv<'mc>,x: f64,y: f64,z: f64) 
-> Result<crate::util::EulerAngle<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("(DDD)V");
let val_1 = jni::objects::JValueGen::Double(x);
let val_2 = jni::objects::JValueGen::Double(y);
let val_3 = jni::objects::JValueGen::Double(z);
let cls = jni.find_class("org/bukkit/util/EulerAngle"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.new_object(cls,
sig.as_str(),vec![jni::objects::JValueGen::from(val_1),jni::objects::JValueGen::from(val_2),jni::objects::JValueGen::from(val_3)]);
let res = 
jni.translate_error_no_gen(res)?;
crate::util::EulerAngle::from_raw(&jni,res
)}
	pub fn x(&self) 
-> Result<f64, Box<dyn std::error::Error>>

{let sig = String::from("()Lf64;");
let res = self.jni_ref().call_method(&self.jni_object(),"getX",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.d()?
)}
	pub fn y(&self) 
-> Result<f64, Box<dyn std::error::Error>>

{let sig = String::from("()Lf64;");
let res = self.jni_ref().call_method(&self.jni_object(),"getY",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.d()?
)}
	pub fn z(&self) 
-> Result<f64, Box<dyn std::error::Error>>

{let sig = String::from("()Lf64;");
let res = self.jni_ref().call_method(&self.jni_object(),"getZ",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.d()?
)}
	pub fn set_x(&self,x: f64) 
-> Result<crate::util::EulerAngle<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("(D)Lcrate::util::EulerAngle;");
let val_1 = jni::objects::JValueGen::Double(x);
let res = self.jni_ref().call_method(&self.jni_object(),"setX",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
crate::util::EulerAngle::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn set_y(&self,y: f64) 
-> Result<crate::util::EulerAngle<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("(D)Lcrate::util::EulerAngle;");
let val_1 = jni::objects::JValueGen::Double(y);
let res = self.jni_ref().call_method(&self.jni_object(),"setY",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
crate::util::EulerAngle::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn set_z(&self,z: f64) 
-> Result<crate::util::EulerAngle<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("(D)Lcrate::util::EulerAngle;");
let val_1 = jni::objects::JValueGen::Double(z);
let res = self.jni_ref().call_method(&self.jni_object(),"setZ",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
crate::util::EulerAngle::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn add(&self,x: f64,y: f64,z: f64) 
-> Result<crate::util::EulerAngle<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("(DDD)Lcrate::util::EulerAngle;");
let val_1 = jni::objects::JValueGen::Double(x);
let val_2 = jni::objects::JValueGen::Double(y);
let val_3 = jni::objects::JValueGen::Double(z);
let res = self.jni_ref().call_method(&self.jni_object(),"add",sig.as_str(),vec![jni::objects::JValueGen::from(val_1),jni::objects::JValueGen::from(val_2),jni::objects::JValueGen::from(val_3)]);
let res = 
self.jni_ref().translate_error(res)?;
crate::util::EulerAngle::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn subtract(&self,x: f64,y: f64,z: f64) 
-> Result<crate::util::EulerAngle<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("(DDD)Lcrate::util::EulerAngle;");
let val_1 = jni::objects::JValueGen::Double(x);
let val_2 = jni::objects::JValueGen::Double(y);
let val_3 = jni::objects::JValueGen::Double(z);
let res = self.jni_ref().call_method(&self.jni_object(),"subtract",sig.as_str(),vec![jni::objects::JValueGen::from(val_1),jni::objects::JValueGen::from(val_2),jni::objects::JValueGen::from(val_3)]);
let res = 
self.jni_ref().translate_error(res)?;
crate::util::EulerAngle::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn equals(&self,o: jni::objects::JObject<'mc>) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("(Ljava/lang/Object;)Lbool;");
let val_1 = jni::objects::JValueGen::Object(o);
let res = self.jni_ref().call_method(&self.jni_object(),"equals",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
	pub fn hash_code(&self) 
-> Result<i32, Box<dyn std::error::Error>>

{let sig = String::from("()Li32;");
let res = self.jni_ref().call_method(&self.jni_object(),"hashCode",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.i()?
)}
// SUPER CLASS: Object

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
#[repr(C)]
pub struct RayTraceResult<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for RayTraceResult<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for RayTraceResult<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate RayTraceResult from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/util/RayTraceResult")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a RayTraceResult object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> RayTraceResult<'mc> {
	pub fn new_with_hit_position(jni: &blackboxmc_general::SharedJNIEnv<'mc>,hit_position: impl Into<crate::util::Vector<'mc>>,hit_entity: std::option::Option<impl Into<crate::entity::Entity<'mc>>>,hit_block_face: std::option::Option<impl Into<crate::block::BlockFace<'mc>>>) 
-> Result<crate::util::RayTraceResult<'mc>, Box<dyn std::error::Error>>

{let mut args = Vec::new();
let mut sig = String::from("(");
sig += "Lorg/bukkit/util/Vector;";
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(hit_position.into().jni_object().clone())});
args.push(val_1);
if let Some(a) = hit_entity {
sig += "Lorg/bukkit/entity/Entity;";
let val_2 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(a.into().jni_object().clone())});
args.push(val_2);
}
if let Some(a) = hit_block_face {
sig += "Lorg/bukkit/block/BlockFace;";
let val_3 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(a.into().jni_object().clone())});
args.push(val_3);
}
sig += ")V";
let cls = jni.find_class("org/bukkit/util/RayTraceResult"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.new_object(cls,
sig.as_str(),args);
let res = 
jni.translate_error_no_gen(res)?;
crate::util::RayTraceResult::from_raw(&jni,res
)}
	pub fn hit_position(&self) 
-> Result<crate::util::Vector<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lcrate::util::Vector;");
let res = self.jni_ref().call_method(&self.jni_object(),"getHitPosition",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::util::Vector::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn hit_block(&self) 
-> Result<Option<crate::block::Block<'mc>>, Box<dyn std::error::Error>>

{let sig = String::from("()Lcrate::block::Block;");
let res = self.jni_ref().call_method(&self.jni_object(),"getHitBlock",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {return Ok(None);}
Ok(
Some(
crate::block::Block::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)?
)
)}
	pub fn hit_block_face(&self) 
-> Result<Option<crate::block::BlockFace<'mc>>, Box<dyn std::error::Error>>

{let sig = String::from("()Lcrate::block::BlockFace;");
let res = self.jni_ref().call_method(&self.jni_object(),"getHitBlockFace",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {return Ok(None);}
Ok(
Some(
crate::block::BlockFace::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)?
)
)}
	pub fn hit_entity(&self) 
-> Result<Option<crate::entity::Entity<'mc>>, Box<dyn std::error::Error>>

{let sig = String::from("()Lcrate::entity::Entity;");
let res = self.jni_ref().call_method(&self.jni_object(),"getHitEntity",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {return Ok(None);}
Ok(
Some(
crate::entity::Entity::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)?
)
)}
	pub fn hash_code(&self) 
-> Result<i32, Box<dyn std::error::Error>>

{let sig = String::from("()Li32;");
let res = self.jni_ref().call_method(&self.jni_object(),"hashCode",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.i()?
)}
	pub fn equals(&self,obj: jni::objects::JObject<'mc>) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("(Ljava/lang/Object;)Lbool;");
let val_1 = jni::objects::JValueGen::Object(obj);
let res = self.jni_ref().call_method(&self.jni_object(),"equals",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
#[doc(hidden)]
	pub fn internal_to_string(&self) 
-> Result<String, Box<dyn std::error::Error>>

{let sig = String::from("()LString;");
let res = self.jni_ref().call_method(&self.jni_object(),"toString",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string()
)}
// SUPER CLASS: Object

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}

        impl<'mc> std::string::ToString for RayTraceResult<'mc> {
            fn to_string(&self) -> String {
                match &self.internal_to_string() {
                    Ok(a) => a.clone(),
                    Err(err) => format!(
                        "Error calling RayTraceResult.toString: {}",
                        err
                    ),
                }
            }
        }
        
#[repr(C)]
pub struct StringUtil<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for StringUtil<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for StringUtil<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate StringUtil from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/util/StringUtil")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a StringUtil object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> StringUtil<'mc> {
	pub fn new(jni: &blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::util::StringUtil<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()V");
let cls = jni.find_class("org/bukkit/util/StringUtil"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.new_object(cls,
sig.as_str(),vec![]);
let res = 
jni.translate_error_no_gen(res)?;
crate::util::StringUtil::from_raw(&jni,res
)}
	pub fn starts_with_ignore_case(jni: &blackboxmc_general::SharedJNIEnv<'mc>,string: impl Into<String>,prefix: impl Into<String>) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("(Ljava/lang/String;Ljava/lang/String;)Lbool;");
let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(jni.new_string(string.into())?));
let val_2 = jni::objects::JValueGen::Object(jni::objects::JObject::from(jni.new_string(prefix.into())?));
let cls = jni.find_class("org/bukkit/util/StringUtil"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.call_static_method(cls,"startsWithIgnoreCase",
sig.as_str(),vec![jni::objects::JValueGen::from(val_1),jni::objects::JValueGen::from(val_2)]);
let res = 
jni.translate_error(res)?;
Ok(
res.z()?
)}
// SUPER CLASS: Object

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
#[repr(C)]
pub struct BlockTransformerTransformationState<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for BlockTransformerTransformationState<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for BlockTransformerTransformationState<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate BlockTransformerTransformationState from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/util/BlockTransformer/TransformationState")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a BlockTransformerTransformationState object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> BlockTransformerTransformationState<'mc> {
	pub fn original(&self) 
-> Result<crate::block::BlockState<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lcrate::block::BlockState;");
let res = self.jni_ref().call_method(&self.jni_object(),"getOriginal",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::block::BlockState::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn world(&self) 
-> Result<crate::block::BlockState<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lcrate::block::BlockState;");
let res = self.jni_ref().call_method(&self.jni_object(),"getWorld",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::block::BlockState::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
#[repr(C)]
pub struct StructureSearchResult<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);

    impl<'mc> JNIRaw<'mc> for StructureSearchResult<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        
self.0.clone()
}
fn jni_object(&self) -> jni::objects::JObject<'mc> {
unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
}
}
impl<'mc> JNIInstantiatable<'mc> for StructureSearchResult<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    "Tried to instantiate StructureSearchResult from null object.")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, "org/bukkit/util/StructureSearchResult")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a StructureSearchResult object, got {}",
                    name
                )
                .into())
            } else {
    Ok(Self(env.clone(), obj))
            }
        }
    }
    
impl<'mc> StructureSearchResult<'mc> {
	pub fn structure(&self) 
-> Result<crate::generator::structure::Structure<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lcrate::generator::structure::Structure;");
let res = self.jni_ref().call_method(&self.jni_object(),"getStructure",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::generator::structure::Structure::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
	pub fn location(&self) 
-> Result<crate::Location<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lcrate::Location;");
let res = self.jni_ref().call_method(&self.jni_object(),"getLocation",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::Location::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
pub mod noise;
pub mod permissions;
pub mod io;
