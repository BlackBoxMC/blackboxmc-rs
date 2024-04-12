#![allow(deprecated)]
use blackboxmc_general::JNIRaw;
use blackboxmc_general::JNIInstantiatable;
use color_eyre::eyre::Result;/*org/bukkit/util/mod.rs*/

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
    
impl<'mc> FileUtilTrait<'mc> for FileUtil<'mc> {}
pub trait FileUtilTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {

	fn new(jni: &blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::util::FileUtil<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()V");
let cls = jni.find_class("org/bukkit/util/FileUtil"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.new_object(cls,
sig.as_str(),vec![]);
let res = 
jni.translate_error_no_gen(res)?;
crate::util::FileUtil::from_raw(&jni,res
)}

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
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
    
impl<'mc> VoxelShapeTrait<'mc> for VoxelShape<'mc> {}
pub trait VoxelShapeTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {
/// Converts this shape into a collection of {@link BoundingBox} equivalent
/// to the shape: a bounding box intersects with this block shape if it
/// intersects with any of the shape's bounding boxes.
	fn bounding_boxes(&self) 
-> Result<Vec<crate::util::BoundingBox<'mc>>, Box<dyn std::error::Error>>

{let sig = String::from("()Ljava/util/Collection;");
let res = self.jni_ref().call_method(&self.jni_object(),"getBoundingBoxes",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
let mut new_vec = Vec::new();
let col = blackboxmc_java::util::JavaCollection::from_raw(&self.jni_ref(),res.l()?)?;let iter = col.iterator()?;
while iter.has_next()? {            let obj = iter.next()?;
new_vec.push(crate::util::BoundingBox::from_raw(&self.jni_ref(),obj,)?);
};
Ok(
new_vec
)}
/// Checks if the given bounding box intersects this block shape.
	fn overlaps(&self,other: impl Into<crate::util::BoundingBox<'mc>>) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/util/BoundingBox;)Z");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(other.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"overlaps",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
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
    
impl<'mc> BlockVectorTrait<'mc> for BlockVector<'mc> {}
pub trait BlockVectorTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {
/// Construct the vector with provided float components.
	fn new(jni: &blackboxmc_general::SharedJNIEnv<'mc>,x: std::option::Option<f32>,y: std::option::Option<f32>,z: std::option::Option<f32>) 
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
/// Checks if another object is equivalent.
	fn equals(&self,obj: jni::objects::JObject<'mc>) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("(Ljava/lang/Object;)Z");
let val_1 = jni::objects::JValueGen::Object(obj);
let res = self.jni_ref().call_method(&self.jni_object(),"equals",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
/// Returns a hash code for this vector.
	fn hash_code(&self) 
-> Result<i32, Box<dyn std::error::Error>>

{let sig = String::from("()I");
let res = self.jni_ref().call_method(&self.jni_object(),"hashCode",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.i()?
)}
/// Get a new block vector.
	fn clone(&self) 
-> Result<crate::util::BlockVector<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/util/BlockVector;");
let res = self.jni_ref().call_method(&self.jni_object(),"clone",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::util::BlockVector::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}

	fn deserialize(jni: &blackboxmc_general::SharedJNIEnv<'mc>,val_args: impl Into<blackboxmc_java::util::JavaMap<'mc>>) 
-> Result<crate::util::BlockVector<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("(Ljava/util/Map;)Lorg/bukkit/util/BlockVector;");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(val_args.into().jni_object().clone())});
let cls = jni.find_class("org/bukkit/util/BlockVector"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.call_static_method(cls,"deserialize",
sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
jni.translate_error(res)?;
let obj = res.l()?;
crate::util::BlockVector::from_raw(&jni,obj
)}

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
impl<'mc> Into<crate::util::Vector<'mc>> for BlockVector<'mc>{

fn into(self) -> crate::util::Vector<'mc> {

crate::util::Vector::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting BlockVector into crate::util::Vector")

   }
}
impl<'mc> crate::util::VectorTrait<'mc> for BlockVector<'mc> {}
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
    
impl<'mc> BlockTransformerTrait<'mc> for BlockTransformer<'mc> {}
pub trait BlockTransformerTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {
/// Transforms a block in a structure.
/// NOTE: The usage of {@link BlockData#createBlockState()} can provide even
/// more flexibility to return the exact block state you might want to
/// return.
	fn transform(&self,region: impl Into<crate::generator::LimitedRegion<'mc>>,x: i32,y: i32,z: i32,current: impl Into<crate::block::BlockState<'mc>>,state: impl Into<crate::util::BlockTransformerTransformationState<'mc>>) 
-> Result<crate::block::BlockState<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/generator/LimitedRegion;IIILorg/bukkit/block/BlockState;Lorg/bukkit/util/BlockTransformer/TransformationState;)Lorg/bukkit/block/BlockState;");
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

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
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
    
impl<'mc> BoundingBoxTrait<'mc> for BoundingBox<'mc> {}
pub trait BoundingBoxTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {
/// Creates a new bounding box from the given corner coordinates.
	fn new(jni: &blackboxmc_general::SharedJNIEnv<'mc>,x1: std::option::Option<f64>,y1: std::option::Option<f64>,z1: std::option::Option<f64>,x2: std::option::Option<f64>,y2: std::option::Option<f64>,z2: std::option::Option<f64>) 
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
/// Creates a new bounding box using the given center and extents.
	fn of(jni: &blackboxmc_general::SharedJNIEnv<'mc>,center: impl Into<crate::Location<'mc>>,x: std::option::Option<f64>,y: std::option::Option<f64>,z: std::option::Option<f64>) 
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
/// Resizes this bounding box.
	fn resize(&self,x1: f64,y1: f64,z1: f64,x2: f64,y2: f64,z2: f64) 
-> Result<crate::util::BoundingBox<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("(DDDDDD)Lorg/bukkit/util/BoundingBox;");
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
/// Gets the minimum x value.
	fn min_x(&self) 
-> Result<f64, Box<dyn std::error::Error>>

{let sig = String::from("()D");
let res = self.jni_ref().call_method(&self.jni_object(),"getMinX",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.d()?
)}
/// Gets the minimum y value.
	fn min_y(&self) 
-> Result<f64, Box<dyn std::error::Error>>

{let sig = String::from("()D");
let res = self.jni_ref().call_method(&self.jni_object(),"getMinY",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.d()?
)}
/// Gets the minimum z value.
	fn min_z(&self) 
-> Result<f64, Box<dyn std::error::Error>>

{let sig = String::from("()D");
let res = self.jni_ref().call_method(&self.jni_object(),"getMinZ",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.d()?
)}
/// Gets the minimum corner as vector.
	fn min(&self) 
-> Result<crate::util::Vector<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/util/Vector;");
let res = self.jni_ref().call_method(&self.jni_object(),"getMin",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::util::Vector::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
/// Gets the maximum x value.
	fn max_x(&self) 
-> Result<f64, Box<dyn std::error::Error>>

{let sig = String::from("()D");
let res = self.jni_ref().call_method(&self.jni_object(),"getMaxX",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.d()?
)}
/// Gets the maximum y value.
	fn max_y(&self) 
-> Result<f64, Box<dyn std::error::Error>>

{let sig = String::from("()D");
let res = self.jni_ref().call_method(&self.jni_object(),"getMaxY",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.d()?
)}
/// Gets the maximum z value.
	fn max_z(&self) 
-> Result<f64, Box<dyn std::error::Error>>

{let sig = String::from("()D");
let res = self.jni_ref().call_method(&self.jni_object(),"getMaxZ",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.d()?
)}
/// Gets the maximum corner as vector.
	fn max(&self) 
-> Result<crate::util::Vector<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/util/Vector;");
let res = self.jni_ref().call_method(&self.jni_object(),"getMax",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::util::Vector::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
/// Gets the width of the bounding box in the x direction.
	fn width_x(&self) 
-> Result<f64, Box<dyn std::error::Error>>

{let sig = String::from("()D");
let res = self.jni_ref().call_method(&self.jni_object(),"getWidthX",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.d()?
)}
/// Gets the width of the bounding box in the z direction.
	fn width_z(&self) 
-> Result<f64, Box<dyn std::error::Error>>

{let sig = String::from("()D");
let res = self.jni_ref().call_method(&self.jni_object(),"getWidthZ",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.d()?
)}
/// Gets the height of the bounding box.
	fn height(&self) 
-> Result<f64, Box<dyn std::error::Error>>

{let sig = String::from("()D");
let res = self.jni_ref().call_method(&self.jni_object(),"getHeight",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.d()?
)}
/// Gets the volume of the bounding box.
	fn volume(&self) 
-> Result<f64, Box<dyn std::error::Error>>

{let sig = String::from("()D");
let res = self.jni_ref().call_method(&self.jni_object(),"getVolume",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.d()?
)}
/// Gets the x coordinate of the center of the bounding box.
	fn center_x(&self) 
-> Result<f64, Box<dyn std::error::Error>>

{let sig = String::from("()D");
let res = self.jni_ref().call_method(&self.jni_object(),"getCenterX",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.d()?
)}
/// Gets the y coordinate of the center of the bounding box.
	fn center_y(&self) 
-> Result<f64, Box<dyn std::error::Error>>

{let sig = String::from("()D");
let res = self.jni_ref().call_method(&self.jni_object(),"getCenterY",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.d()?
)}
/// Gets the z coordinate of the center of the bounding box.
	fn center_z(&self) 
-> Result<f64, Box<dyn std::error::Error>>

{let sig = String::from("()D");
let res = self.jni_ref().call_method(&self.jni_object(),"getCenterZ",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.d()?
)}
/// Gets the center of the bounding box.
	fn center(&self) 
-> Result<crate::util::Vector<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/util/Vector;");
let res = self.jni_ref().call_method(&self.jni_object(),"getCenter",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::util::Vector::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
/// Copies another bounding box.
	fn copy(&self,other: impl Into<crate::util::BoundingBox<'mc>>) 
-> Result<crate::util::BoundingBox<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/util/BoundingBox;)Lorg/bukkit/util/BoundingBox;");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(other.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"copy",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
crate::util::BoundingBox::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
/// Expands this bounding box by the given values in the corresponding
/// directions.
/// 
/// Negative values will shrink the bounding box in the corresponding
/// direction. Shrinking will be limited to the point where the affected
/// opposite faces would meet if the they shrank at uniform speeds.
	fn expand(&self,negative_x: f64,negative_y: std::option::Option<f64>,negative_z: std::option::Option<f64>,positive_x: std::option::Option<f64>,positive_y: std::option::Option<f64>,positive_z: std::option::Option<f64>) 
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
/// Expands this bounding box in the specified direction.
/// 
/// Negative values will expand the bounding box in the negative direction,
/// positive values will expand it in the positive direction. The magnitudes
/// of the direction components determine the corresponding amounts of
/// expansion.
	fn expand_directional(&self,dir_x: f64,dir_y: std::option::Option<f64>,dir_z: std::option::Option<f64>) 
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
/// Expands this bounding box to contain (or border) the specified position.
	fn union(&self,pos_x: f64,pos_y: std::option::Option<f64>,pos_z: std::option::Option<f64>) 
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
/// Resizes this bounding box to represent the intersection of this and the
/// given bounding box.
	fn intersection(&self,other: impl Into<crate::util::BoundingBox<'mc>>) 
-> Result<crate::util::BoundingBox<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/util/BoundingBox;)Lorg/bukkit/util/BoundingBox;");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(other.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"intersection",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
crate::util::BoundingBox::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
/// Shifts this bounding box by the given amounts.
	fn shift(&self,shift_x: f64,shift_y: std::option::Option<f64>,shift_z: std::option::Option<f64>) 
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
/// Checks if this bounding box overlaps with the bounding box that is
/// defined by the given corners.
/// 
/// Bounding boxes that are only intersecting at the borders are not
/// considered overlapping.
	fn overlaps(&self,min: impl Into<crate::util::Vector<'mc>>,max: std::option::Option<impl Into<crate::util::Vector<'mc>>>) 
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
/// Checks if this bounding box contains the specified position.
/// 
/// Positions exactly on the minimum borders of the bounding box are
/// considered to be inside the bounding box, while positions exactly on the
/// maximum borders are considered to be outside. This allows bounding boxes
/// to reside directly next to each other with positions always only residing
/// in exactly one of them.
	fn contains(&self,x: f64,y: std::option::Option<f64>,z: std::option::Option<f64>) 
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
/// Calculates the intersection of this bounding box with the specified line
/// segment.
/// 
/// Intersections at edges and corners yield one of the affected block faces
/// as hit result, but it is not defined which of them.
	fn ray_trace(&self,start: impl Into<crate::util::Vector<'mc>>,direction: impl Into<crate::util::Vector<'mc>>,max_distance: f64) 
-> Result<Option<crate::util::RayTraceResult<'mc>>, Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/util/Vector;Lorg/bukkit/util/Vector;D)Lorg/bukkit/util/RayTraceResult;");
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

	fn hash_code(&self) 
-> Result<i32, Box<dyn std::error::Error>>

{let sig = String::from("()I");
let res = self.jni_ref().call_method(&self.jni_object(),"hashCode",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.i()?
)}

	fn equals(&self,obj: jni::objects::JObject<'mc>) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("(Ljava/lang/Object;)Z");
let val_1 = jni::objects::JValueGen::Object(obj);
let res = self.jni_ref().call_method(&self.jni_object(),"equals",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}

#[doc(hidden)]
	fn internal_to_string(&self) 
-> Result<String, Box<dyn std::error::Error>>

{let sig = String::from("()Ljava/lang/String;");
let res = self.jni_ref().call_method(&self.jni_object(),"toString",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string()
)}
/// Creates a copy of this bounding box.
	fn clone(&self) 
-> Result<crate::util::BoundingBox<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/util/BoundingBox;");
let res = self.jni_ref().call_method(&self.jni_object(),"clone",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::util::BoundingBox::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}

	fn serialize(&self) 
-> Result<blackboxmc_java::util::JavaMap<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Ljava/util/Map;");
let res = self.jni_ref().call_method(&self.jni_object(),"serialize",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
blackboxmc_java::util::JavaMap::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}

	fn deserialize(jni: &blackboxmc_general::SharedJNIEnv<'mc>,val_args: impl Into<blackboxmc_java::util::JavaMap<'mc>>) 
-> Result<crate::util::BoundingBox<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("(Ljava/util/Map;)Lorg/bukkit/util/BoundingBox;");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(val_args.into().jni_object().clone())});
let cls = jni.find_class("org/bukkit/util/BoundingBox"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.call_static_method(cls,"deserialize",
sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
jni.translate_error(res)?;
let obj = res.l()?;
crate::util::BoundingBox::from_raw(&jni,obj
)}

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}

        impl<'mc> std::string::ToString for BoundingBox<'mc> {
            fn to_string(&self) -> String {
                match BoundingBoxTrait::internal_to_string(self) {
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

crate::configuration::serialization::ConfigurationSerializable::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting BoundingBox into crate::configuration::serialization::ConfigurationSerializable")

   }
}
impl<'mc> crate::configuration::serialization::ConfigurationSerializableTrait<'mc> for BoundingBox<'mc> {}
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
    
impl<'mc> ChatPaginatorChatPageTrait<'mc> for ChatPaginatorChatPage<'mc> {}
pub trait ChatPaginatorChatPageTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {

	fn new(jni: &blackboxmc_general::SharedJNIEnv<'mc>,lines: impl Into<String>,page_number: i32,total_pages: i32) 
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

	fn page_number(&self) 
-> Result<i32, Box<dyn std::error::Error>>

{let sig = String::from("()I");
let res = self.jni_ref().call_method(&self.jni_object(),"getPageNumber",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.i()?
)}

	fn total_pages(&self) 
-> Result<i32, Box<dyn std::error::Error>>

{let sig = String::from("()I");
let res = self.jni_ref().call_method(&self.jni_object(),"getTotalPages",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.i()?
)}

	fn lines(&self) 
-> Result<String, Box<dyn std::error::Error>>

{let sig = String::from("()Ljava/lang/String;");
let res = self.jni_ref().call_method(&self.jni_object(),"getLines",sig.as_str(),vec![]);
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
    
impl<'mc> ConsumerTrait<'mc> for Consumer<'mc> {}
pub trait ConsumerTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {
/// Performs this operation on the given argument.
	fn accept(&self,t: jni::objects::JObject<'mc>) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("(LT;)V");
let val_1 = jni::objects::JValueGen::Object(t);
let res = self.jni_ref().call_method(&self.jni_object(),"accept",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
impl<'mc> Into<blackboxmc_java::util::function::JavaConsumer<'mc>> for Consumer<'mc>{

fn into(self) -> blackboxmc_java::util::function::JavaConsumer<'mc> {

blackboxmc_java::util::function::JavaConsumer::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting Consumer into blackboxmc_java::util::function::JavaConsumer")

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
    
impl<'mc> ChatPaginatorTrait<'mc> for ChatPaginator<'mc> {}
pub trait ChatPaginatorTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {

	fn new(jni: &blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::util::ChatPaginator<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()V");
let cls = jni.find_class("org/bukkit/util/ChatPaginator"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.new_object(cls,
sig.as_str(),vec![]);
let res = 
jni.translate_error_no_gen(res)?;
crate::util::ChatPaginator::from_raw(&jni,res
)}
/// Breaks a raw string up into pages using a provided width and height.
	fn paginate(jni: &blackboxmc_general::SharedJNIEnv<'mc>,unpaginated_string: impl Into<String>,page_number: i32,line_length: std::option::Option<i32>,page_height: std::option::Option<i32>) 
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
/// Breaks a raw string up into a series of lines. Words are wrapped using
/// spaces as decimeters and the newline character is respected.
	fn word_wrap(jni: &blackboxmc_general::SharedJNIEnv<'mc>,raw_string: impl Into<String>,line_length: i32) 
-> Result<String, Box<dyn std::error::Error>>

{let sig = String::from("(Ljava/lang/String;I)Ljava/lang/String;");
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

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
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
    
impl<'mc> BlockIteratorTrait<'mc> for BlockIterator<'mc> {}
pub trait BlockIteratorTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {
/// Constructs the BlockIterator.
/// 
/// This considers all blocks as 1x1x1 in size.
	fn new(jni: &blackboxmc_general::SharedJNIEnv<'mc>,world: impl Into<crate::World<'mc>>,start: std::option::Option<impl Into<crate::util::Vector<'mc>>>,direction: std::option::Option<impl Into<crate::util::Vector<'mc>>>,y_offset: std::option::Option<f64>,max_distance: std::option::Option<i32>) 
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
/// Returns true if the iteration has more elements
	fn has_next(&self) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("()Z");
let res = self.jni_ref().call_method(&self.jni_object(),"hasNext",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
/// Returns the next Block in the trace
	fn next(&self) 
-> Result<crate::block::Block<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/block/Block;");
let res = self.jni_ref().call_method(&self.jni_object(),"next",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::block::Block::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}

	fn remove(&self) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("()V");
let res = self.jni_ref().call_method(&self.jni_object(),"remove",sig.as_str(),vec![]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
impl<'mc> Into<blackboxmc_java::util::JavaIterator<'mc>> for BlockIterator<'mc>{

fn into(self) -> blackboxmc_java::util::JavaIterator<'mc> {

blackboxmc_java::util::JavaIterator::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting BlockIterator into blackboxmc_java::util::JavaIterator")

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
    
impl<'mc> VectorTrait<'mc> for Vector<'mc> {}
pub trait VectorTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {
/// Construct the vector with provided float components.
	fn new(jni: &blackboxmc_general::SharedJNIEnv<'mc>,x: std::option::Option<f32>,y: std::option::Option<f32>,z: std::option::Option<f32>) 
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
/// Adds a vector to this one
	fn add(&self,vec: impl Into<crate::util::Vector<'mc>>) 
-> Result<crate::util::Vector<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/util/Vector;)Lorg/bukkit/util/Vector;");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(vec.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"add",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
crate::util::Vector::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
/// Subtracts a vector from this one.
	fn subtract(&self,vec: impl Into<crate::util::Vector<'mc>>) 
-> Result<crate::util::Vector<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/util/Vector;)Lorg/bukkit/util/Vector;");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(vec.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"subtract",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
crate::util::Vector::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
/// Performs scalar multiplication, multiplying all components with a
/// scalar.
	fn multiply(&self,m: f32) 
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
/// Divides the vector by another.
	fn divide(&self,vec: impl Into<crate::util::Vector<'mc>>) 
-> Result<crate::util::Vector<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/util/Vector;)Lorg/bukkit/util/Vector;");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(vec.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"divide",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
crate::util::Vector::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
/// Copies another vector
	fn copy(&self,vec: impl Into<crate::util::Vector<'mc>>) 
-> Result<crate::util::Vector<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/util/Vector;)Lorg/bukkit/util/Vector;");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(vec.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"copy",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
crate::util::Vector::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
/// Gets the magnitude of the vector, defined as sqrt(x^2+y^2+z^2). The
/// value of this method is not cached and uses a costly square-root
/// function, so do not repeatedly call this method to get the vector's
/// magnitude. NaN will be returned if the inner result of the sqrt()
/// function overflows, which will be caused if the length is too long.
	fn length(&self) 
-> Result<f64, Box<dyn std::error::Error>>

{let sig = String::from("()D");
let res = self.jni_ref().call_method(&self.jni_object(),"length",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.d()?
)}
/// Gets the magnitude of the vector squared.
	fn length_squared(&self) 
-> Result<f64, Box<dyn std::error::Error>>

{let sig = String::from("()D");
let res = self.jni_ref().call_method(&self.jni_object(),"lengthSquared",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.d()?
)}
/// Get the distance between this vector and another. The value of this
/// method is not cached and uses a costly square-root function, so do not
/// repeatedly call this method to get the vector's magnitude. NaN will be
/// returned if the inner result of the sqrt() function overflows, which
/// will be caused if the distance is too long.
	fn distance(&self,o: impl Into<crate::util::Vector<'mc>>) 
-> Result<f64, Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/util/Vector;)D");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(o.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"distance",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.d()?
)}
/// Get the squared distance between this vector and another.
	fn distance_squared(&self,o: impl Into<crate::util::Vector<'mc>>) 
-> Result<f64, Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/util/Vector;)D");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(o.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"distanceSquared",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.d()?
)}
/// Gets the angle between this vector and another in radians.
	fn angle(&self,other: impl Into<crate::util::Vector<'mc>>) 
-> Result<f32, Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/util/Vector;)F");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(other.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"angle",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.f()?
)}
/// Sets this vector to the midpoint between this vector and another.
	fn midpoint(&self,other: impl Into<crate::util::Vector<'mc>>) 
-> Result<crate::util::Vector<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/util/Vector;)Lorg/bukkit/util/Vector;");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(other.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"midpoint",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
crate::util::Vector::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
/// Gets a new midpoint vector between this vector and another.
	fn get_midpoint(&self,other: impl Into<crate::util::Vector<'mc>>) 
-> Result<crate::util::Vector<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/util/Vector;)Lorg/bukkit/util/Vector;");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(other.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"getMidpoint",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
crate::util::Vector::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
/// Calculates the dot product of this vector with another. The dot product
/// is defined as x1*x2+y1*y2+z1*z2. The returned value is a scalar.
	fn dot(&self,other: impl Into<crate::util::Vector<'mc>>) 
-> Result<f64, Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/util/Vector;)D");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(other.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"dot",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.d()?
)}
/// Calculates the cross product of this vector with another. The cross
/// product is defined as:
/// <ul>
/// <li>x = y1 * z2 - y2 * z1
/// <li>y = z1 * x2 - z2 * x1
/// <li>z = x1 * y2 - x2 * y1
/// </ul>
	fn cross_product(&self,o: impl Into<crate::util::Vector<'mc>>) 
-> Result<crate::util::Vector<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/util/Vector;)Lorg/bukkit/util/Vector;");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(o.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"crossProduct",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
crate::util::Vector::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
/// Calculates the cross product of this vector with another without mutating
/// the original. The cross product is defined as:
/// <ul>
/// <li>x = y1 * z2 - y2 * z1
/// <li>y = z1 * x2 - z2 * x1
/// <li>z = x1 * y2 - x2 * y1
/// </ul>
	fn get_cross_product(&self,o: impl Into<crate::util::Vector<'mc>>) 
-> Result<crate::util::Vector<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/util/Vector;)Lorg/bukkit/util/Vector;");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(o.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"getCrossProduct",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
crate::util::Vector::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
/// Converts this vector to a unit vector (a vector with length of 1).
	fn normalize(&self) 
-> Result<crate::util::Vector<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/util/Vector;");
let res = self.jni_ref().call_method(&self.jni_object(),"normalize",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::util::Vector::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
/// Zero this vector's components.
	fn zero(&self) 
-> Result<crate::util::Vector<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/util/Vector;");
let res = self.jni_ref().call_method(&self.jni_object(),"zero",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::util::Vector::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
/// Check whether or not each component of this vector is equal to 0.
	fn is_zero(&self) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("()Z");
let res = self.jni_ref().call_method(&self.jni_object(),"isZero",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
/// Returns whether this vector is in an axis-aligned bounding box.
/// 
/// The minimum and maximum vectors given must be truly the minimum and
/// maximum X, Y and Z components.
	fn is_in_aabb(&self,min: impl Into<crate::util::Vector<'mc>>,max: impl Into<crate::util::Vector<'mc>>) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/util/Vector;Lorg/bukkit/util/Vector;)Z");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(min.into().jni_object().clone())});
let val_2 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(max.into().jni_object().clone())});
let res = self.jni_ref().call_method(&self.jni_object(),"isInAABB",sig.as_str(),vec![jni::objects::JValueGen::from(val_1),jni::objects::JValueGen::from(val_2)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
/// Returns whether this vector is within a sphere.
	fn is_in_sphere(&self,origin: impl Into<crate::util::Vector<'mc>>,radius: f64) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/util/Vector;D)Z");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(origin.into().jni_object().clone())});
let val_2 = jni::objects::JValueGen::Double(radius);
let res = self.jni_ref().call_method(&self.jni_object(),"isInSphere",sig.as_str(),vec![jni::objects::JValueGen::from(val_1),jni::objects::JValueGen::from(val_2)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
/// Returns if a vector is normalized
	fn is_normalized(&self) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("()Z");
let res = self.jni_ref().call_method(&self.jni_object(),"isNormalized",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
/// Rotates the vector around the x axis.
/// 
/// This piece of math is based on the standard rotation matrix for vectors
/// in three dimensional space. This matrix can be found here:
/// <a href="https://en.wikipedia.org/wiki/Rotation_matrix#Basic_rotations">Rotation
/// Matrix</a>.
	fn rotate_around_x(&self,angle: f64) 
-> Result<crate::util::Vector<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("(D)Lorg/bukkit/util/Vector;");
let val_1 = jni::objects::JValueGen::Double(angle);
let res = self.jni_ref().call_method(&self.jni_object(),"rotateAroundX",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
crate::util::Vector::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
/// Rotates the vector around the y axis.
/// 
/// This piece of math is based on the standard rotation matrix for vectors
/// in three dimensional space. This matrix can be found here:
/// <a href="https://en.wikipedia.org/wiki/Rotation_matrix#Basic_rotations">Rotation
/// Matrix</a>.
	fn rotate_around_y(&self,angle: f64) 
-> Result<crate::util::Vector<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("(D)Lorg/bukkit/util/Vector;");
let val_1 = jni::objects::JValueGen::Double(angle);
let res = self.jni_ref().call_method(&self.jni_object(),"rotateAroundY",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
crate::util::Vector::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
/// Rotates the vector around the z axis
/// 
/// This piece of math is based on the standard rotation matrix for vectors
/// in three dimensional space. This matrix can be found here:
/// <a href="https://en.wikipedia.org/wiki/Rotation_matrix#Basic_rotations">Rotation
/// Matrix</a>.
	fn rotate_around_z(&self,angle: f64) 
-> Result<crate::util::Vector<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("(D)Lorg/bukkit/util/Vector;");
let val_1 = jni::objects::JValueGen::Double(angle);
let res = self.jni_ref().call_method(&self.jni_object(),"rotateAroundZ",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
crate::util::Vector::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
/// Rotates the vector around a given arbitrary axis in 3 dimensional space.
/// 
/// Rotation will follow the general Right-Hand-Rule, which means rotation
/// will be counterclockwise when the axis is pointing towards the observer.
/// 
/// This method will always make sure the provided axis is a unit vector, to
/// not modify the length of the vector when rotating. If you are experienced
/// with the scaling of a non-unit axis vector, you can use
/// {@link Vector#rotateAroundNonUnitAxis(Vector, double)}.
	fn rotate_around_axis(&self,axis: impl Into<crate::util::Vector<'mc>>,angle: f64) 
-> Result<crate::util::Vector<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/util/Vector;D)Lorg/bukkit/util/Vector;");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(axis.into().jni_object().clone())});
let val_2 = jni::objects::JValueGen::Double(angle);
let res = self.jni_ref().call_method(&self.jni_object(),"rotateAroundAxis",sig.as_str(),vec![jni::objects::JValueGen::from(val_1),jni::objects::JValueGen::from(val_2)]);
let res = 
self.jni_ref().translate_error(res)?;
crate::util::Vector::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
/// Rotates the vector around a given arbitrary axis in 3 dimensional space.
/// 
/// Rotation will follow the general Right-Hand-Rule, which means rotation
/// will be counterclockwise when the axis is pointing towards the observer.
/// 
/// Note that the vector length will change accordingly to the axis vector
/// length. If the provided axis is not a unit vector, the rotated vector
/// will not have its previous length. The scaled length of the resulting
/// vector will be related to the axis vector. If you are not perfectly sure
/// about the scaling of the vector, use
/// {@link Vector#rotateAroundAxis(Vector, double)}
	fn rotate_around_non_unit_axis(&self,axis: impl Into<crate::util::Vector<'mc>>,angle: f64) 
-> Result<crate::util::Vector<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/util/Vector;D)Lorg/bukkit/util/Vector;");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(axis.into().jni_object().clone())});
let val_2 = jni::objects::JValueGen::Double(angle);
let res = self.jni_ref().call_method(&self.jni_object(),"rotateAroundNonUnitAxis",sig.as_str(),vec![jni::objects::JValueGen::from(val_1),jni::objects::JValueGen::from(val_2)]);
let res = 
self.jni_ref().translate_error(res)?;
crate::util::Vector::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
/// Gets the X component.
	fn x(&self) 
-> Result<f64, Box<dyn std::error::Error>>

{let sig = String::from("()D");
let res = self.jni_ref().call_method(&self.jni_object(),"getX",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.d()?
)}
/// Gets the floored value of the X component, indicating the block that
/// this vector is contained with.
	fn block_x(&self) 
-> Result<i32, Box<dyn std::error::Error>>

{let sig = String::from("()I");
let res = self.jni_ref().call_method(&self.jni_object(),"getBlockX",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.i()?
)}
/// Gets the Y component.
	fn y(&self) 
-> Result<f64, Box<dyn std::error::Error>>

{let sig = String::from("()D");
let res = self.jni_ref().call_method(&self.jni_object(),"getY",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.d()?
)}
/// Gets the floored value of the Y component, indicating the block that
/// this vector is contained with.
	fn block_y(&self) 
-> Result<i32, Box<dyn std::error::Error>>

{let sig = String::from("()I");
let res = self.jni_ref().call_method(&self.jni_object(),"getBlockY",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.i()?
)}
/// Gets the Z component.
	fn z(&self) 
-> Result<f64, Box<dyn std::error::Error>>

{let sig = String::from("()D");
let res = self.jni_ref().call_method(&self.jni_object(),"getZ",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.d()?
)}
/// Gets the floored value of the Z component, indicating the block that
/// this vector is contained with.
	fn block_z(&self) 
-> Result<i32, Box<dyn std::error::Error>>

{let sig = String::from("()I");
let res = self.jni_ref().call_method(&self.jni_object(),"getBlockZ",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.i()?
)}
/// Set the X component.
	fn set_x(&self,x: f32) 
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
/// Set the Y component.
	fn set_y(&self,y: f32) 
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
/// Set the Z component.
	fn set_z(&self,z: f32) 
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
/// Checks to see if two objects are equal.
/// 
/// Only two Vectors can ever return true. This method uses a fuzzy match
/// to account for floating point errors. The epsilon can be retrieved
/// with epsilon.
	fn equals(&self,obj: jni::objects::JObject<'mc>) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("(Ljava/lang/Object;)Z");
let val_1 = jni::objects::JValueGen::Object(obj);
let res = self.jni_ref().call_method(&self.jni_object(),"equals",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}
/// Returns a hash code for this vector
	fn hash_code(&self) 
-> Result<i32, Box<dyn std::error::Error>>

{let sig = String::from("()I");
let res = self.jni_ref().call_method(&self.jni_object(),"hashCode",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.i()?
)}
/// Get a new vector.
	fn clone(&self) 
-> Result<crate::util::Vector<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/util/Vector;");
let res = self.jni_ref().call_method(&self.jni_object(),"clone",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::util::Vector::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
/// Returns this vector's components as x,y,z.
#[doc(hidden)]
	fn internal_to_string(&self) 
-> Result<String, Box<dyn std::error::Error>>

{let sig = String::from("()Ljava/lang/String;");
let res = self.jni_ref().call_method(&self.jni_object(),"toString",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
self.jni_ref().get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?.to_string_lossy().to_string()
)}
/// Gets a Location version of this vector.
	fn to_location(&self,world: impl Into<crate::World<'mc>>,yaw: std::option::Option<f32>,pitch: std::option::Option<f32>) 
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
/// Get the block vector of this vector.
	fn to_block_vector(&self) 
-> Result<crate::util::BlockVector<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/util/BlockVector;");
let res = self.jni_ref().call_method(&self.jni_object(),"toBlockVector",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::util::BlockVector::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
/// Get this vector as a JOML {@link Vector3f}.
	fn to_vector3f(&self) 
-> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/joml/Vector3f;");
let res = self.jni_ref().call_method(&self.jni_object(),"toVector3f",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.l()?
)}
/// Get this vector as a JOML {@link Vector3d}.
	fn to_vector3d(&self) 
-> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/joml/Vector3d;");
let res = self.jni_ref().call_method(&self.jni_object(),"toVector3d",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.l()?
)}
/// Get this vector as a JOML {@link Vector3i}.
	fn to_vector3i(&self,rounding_mode: std::option::Option<i32>) 
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
/// Check if each component of this Vector is finite.
	fn check_finite(&self) 
-> Result<(), Box<dyn std::error::Error>>

{let sig = String::from("()V");
let res = self.jni_ref().call_method(&self.jni_object(),"checkFinite",sig.as_str(),vec![]);
self.jni_ref().translate_error(res)?;
Ok(
()
)}
/// Get the threshold used for equals().
	fn epsilon(jni: &blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<f64, Box<dyn std::error::Error>>

{let sig = String::from("()D");
let cls = jni.find_class("org/bukkit/util/Vector"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.call_static_method(cls,"getEpsilon",
sig.as_str(),vec![]);
let res = 
jni.translate_error(res)?;
Ok(
res.d()?
)}
/// Gets the minimum components of two vectors.
	fn get_minimum(jni: &blackboxmc_general::SharedJNIEnv<'mc>,v1: impl Into<crate::util::Vector<'mc>>,v2: impl Into<crate::util::Vector<'mc>>) 
-> Result<crate::util::Vector<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/util/Vector;Lorg/bukkit/util/Vector;)Lorg/bukkit/util/Vector;");
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
/// Gets the maximum components of two vectors.
	fn get_maximum(jni: &blackboxmc_general::SharedJNIEnv<'mc>,v1: impl Into<crate::util::Vector<'mc>>,v2: impl Into<crate::util::Vector<'mc>>) 
-> Result<crate::util::Vector<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/util/Vector;Lorg/bukkit/util/Vector;)Lorg/bukkit/util/Vector;");
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
/// Gets a random vector with components having a random value between 0
/// and 1.
	fn random(jni: &blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::util::Vector<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/util/Vector;");
let cls = jni.find_class("org/bukkit/util/Vector"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.call_static_method(cls,"getRandom",
sig.as_str(),vec![]);
let res = 
jni.translate_error(res)?;
let obj = res.l()?;
crate::util::Vector::from_raw(&jni,obj
)}
/// Gets a vector with components that match the provided JOML {@link Vector3ic}.
	fn from_joml(jni: &blackboxmc_general::SharedJNIEnv<'mc>,vector: jni::objects::JObject<'mc>) 
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

	fn serialize(&self) 
-> Result<blackboxmc_java::util::JavaMap<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Ljava/util/Map;");
let res = self.jni_ref().call_method(&self.jni_object(),"serialize",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
blackboxmc_java::util::JavaMap::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}

	fn deserialize(jni: &blackboxmc_general::SharedJNIEnv<'mc>,val_args: impl Into<blackboxmc_java::util::JavaMap<'mc>>) 
-> Result<crate::util::Vector<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("(Ljava/util/Map;)Lorg/bukkit/util/Vector;");
let val_1 = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw(val_args.into().jni_object().clone())});
let cls = jni.find_class("org/bukkit/util/Vector"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.call_static_method(cls,"deserialize",
sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
jni.translate_error(res)?;
let obj = res.l()?;
crate::util::Vector::from_raw(&jni,obj
)}

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}

        impl<'mc> std::string::ToString for Vector<'mc> {
            fn to_string(&self) -> String {
                match VectorTrait::internal_to_string(self) {
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

crate::configuration::serialization::ConfigurationSerializable::from_raw(&self.jni_ref(), self.jni_object()).expect("Error converting Vector into crate::configuration::serialization::ConfigurationSerializable")

   }
}
impl<'mc> crate::configuration::serialization::ConfigurationSerializableTrait<'mc> for Vector<'mc> {}
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
    
impl<'mc> EntityTransformerTrait<'mc> for EntityTransformer<'mc> {}
pub trait EntityTransformerTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {
/// Transforms a entity in a structure.
	fn transform(&self,region: impl Into<crate::generator::LimitedRegion<'mc>>,x: i32,y: i32,z: i32,entity: impl Into<crate::entity::Entity<'mc>>,allowed_to_spawn: bool) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("(Lorg/bukkit/generator/LimitedRegion;IIILorg/bukkit/entity/Entity;Z)Z");
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

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
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
    
impl<'mc> BiomeSearchResultTrait<'mc> for BiomeSearchResult<'mc> {}
pub trait BiomeSearchResultTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {
/// Return the biome which was found.
	fn biome(&self) 
-> Result<crate::block::Biome<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/block/Biome;");
let res = self.jni_ref().call_method(&self.jni_object(),"getBiome",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::block::Biome::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
/// Return the location of the biome.
	fn location(&self) 
-> Result<crate::Location<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/Location;");
let res = self.jni_ref().call_method(&self.jni_object(),"getLocation",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::Location::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
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
    
impl<'mc> TransformationTrait<'mc> for Transformation<'mc> {}
pub trait TransformationTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {

	fn new(jni: &blackboxmc_general::SharedJNIEnv<'mc>,translation: jni::objects::JObject<'mc>,left_rotation: jni::objects::JObject<'mc>,scale: jni::objects::JObject<'mc>,right_rotation: jni::objects::JObject<'mc>) 
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
/// Gets the translation component of this transformation.
	fn translation(&self) 
-> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/joml/Vector3f;");
let res = self.jni_ref().call_method(&self.jni_object(),"getTranslation",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.l()?
)}
/// Gets the left rotation component of this transformation.
	fn left_rotation(&self) 
-> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/joml/Quaternionf;");
let res = self.jni_ref().call_method(&self.jni_object(),"getLeftRotation",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.l()?
)}
/// Gets the scale component of this transformation.
	fn scale(&self) 
-> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/joml/Vector3f;");
let res = self.jni_ref().call_method(&self.jni_object(),"getScale",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.l()?
)}
/// Gets the right rotation component of this transformation.
	fn right_rotation(&self) 
-> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/joml/Quaternionf;");
let res = self.jni_ref().call_method(&self.jni_object(),"getRightRotation",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.l()?
)}

	fn hash_code(&self) 
-> Result<i32, Box<dyn std::error::Error>>

{let sig = String::from("()I");
let res = self.jni_ref().call_method(&self.jni_object(),"hashCode",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.i()?
)}

	fn equals(&self,obj: jni::objects::JObject<'mc>) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("(Ljava/lang/Object;)Z");
let val_1 = jni::objects::JValueGen::Object(obj);
let res = self.jni_ref().call_method(&self.jni_object(),"equals",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}

#[doc(hidden)]
	fn internal_to_string(&self) 
-> Result<String, Box<dyn std::error::Error>>

{let sig = String::from("()Ljava/lang/String;");
let res = self.jni_ref().call_method(&self.jni_object(),"toString",sig.as_str(),vec![]);
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

        impl<'mc> std::string::ToString for Transformation<'mc> {
            fn to_string(&self) -> String {
                match TransformationTrait::internal_to_string(self) {
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
    
impl<'mc> CachedServerIconTrait<'mc> for CachedServerIcon<'mc> {}
pub trait CachedServerIconTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
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
    
impl<'mc> NumberConversionsTrait<'mc> for NumberConversions<'mc> {}
pub trait NumberConversionsTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {

	fn floor(jni: &blackboxmc_general::SharedJNIEnv<'mc>,num: f64) 
-> Result<i32, Box<dyn std::error::Error>>

{let sig = String::from("(D)I");
let val_1 = jni::objects::JValueGen::Double(num);
let cls = jni.find_class("org/bukkit/util/NumberConversions"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.call_static_method(cls,"floor",
sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
jni.translate_error(res)?;
Ok(
res.i()?
)}

	fn ceil(jni: &blackboxmc_general::SharedJNIEnv<'mc>,num: f64) 
-> Result<i32, Box<dyn std::error::Error>>

{let sig = String::from("(D)I");
let val_1 = jni::objects::JValueGen::Double(num);
let cls = jni.find_class("org/bukkit/util/NumberConversions"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.call_static_method(cls,"ceil",
sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
jni.translate_error(res)?;
Ok(
res.i()?
)}

	fn round(jni: &blackboxmc_general::SharedJNIEnv<'mc>,num: f64) 
-> Result<i32, Box<dyn std::error::Error>>

{let sig = String::from("(D)I");
let val_1 = jni::objects::JValueGen::Double(num);
let cls = jni.find_class("org/bukkit/util/NumberConversions"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.call_static_method(cls,"round",
sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
jni.translate_error(res)?;
Ok(
res.i()?
)}

	fn square(jni: &blackboxmc_general::SharedJNIEnv<'mc>,num: f64) 
-> Result<f64, Box<dyn std::error::Error>>

{let sig = String::from("(D)D");
let val_1 = jni::objects::JValueGen::Double(num);
let cls = jni.find_class("org/bukkit/util/NumberConversions"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.call_static_method(cls,"square",
sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
jni.translate_error(res)?;
Ok(
res.d()?
)}

	fn to_int(jni: &blackboxmc_general::SharedJNIEnv<'mc>,object: jni::objects::JObject<'mc>) 
-> Result<i32, Box<dyn std::error::Error>>

{let sig = String::from("(Ljava/lang/Object;)I");
let val_1 = jni::objects::JValueGen::Object(object);
let cls = jni.find_class("org/bukkit/util/NumberConversions"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.call_static_method(cls,"toInt",
sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
jni.translate_error(res)?;
Ok(
res.i()?
)}

	fn to_float(jni: &blackboxmc_general::SharedJNIEnv<'mc>,object: jni::objects::JObject<'mc>) 
-> Result<f32, Box<dyn std::error::Error>>

{let sig = String::from("(Ljava/lang/Object;)F");
let val_1 = jni::objects::JValueGen::Object(object);
let cls = jni.find_class("org/bukkit/util/NumberConversions"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.call_static_method(cls,"toFloat",
sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
jni.translate_error(res)?;
Ok(
res.f()?
)}

	fn to_double(jni: &blackboxmc_general::SharedJNIEnv<'mc>,object: jni::objects::JObject<'mc>) 
-> Result<f64, Box<dyn std::error::Error>>

{let sig = String::from("(Ljava/lang/Object;)D");
let val_1 = jni::objects::JValueGen::Object(object);
let cls = jni.find_class("org/bukkit/util/NumberConversions"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.call_static_method(cls,"toDouble",
sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
jni.translate_error(res)?;
Ok(
res.d()?
)}

	fn to_long(jni: &blackboxmc_general::SharedJNIEnv<'mc>,object: jni::objects::JObject<'mc>) 
-> Result<i64, Box<dyn std::error::Error>>

{let sig = String::from("(Ljava/lang/Object;)J");
let val_1 = jni::objects::JValueGen::Object(object);
let cls = jni.find_class("org/bukkit/util/NumberConversions"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.call_static_method(cls,"toLong",
sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
jni.translate_error(res)?;
Ok(
res.j()?
)}

	fn to_short(jni: &blackboxmc_general::SharedJNIEnv<'mc>,object: jni::objects::JObject<'mc>) 
-> Result<i16, Box<dyn std::error::Error>>

{let sig = String::from("(Ljava/lang/Object;)S");
let val_1 = jni::objects::JValueGen::Object(object);
let cls = jni.find_class("org/bukkit/util/NumberConversions"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.call_static_method(cls,"toShort",
sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
jni.translate_error(res)?;
Ok(
res.s()?
)}

	fn to_byte(jni: &blackboxmc_general::SharedJNIEnv<'mc>,object: jni::objects::JObject<'mc>) 
-> Result<i8, Box<dyn std::error::Error>>

{let sig = String::from("(Ljava/lang/Object;)B");
let val_1 = jni::objects::JValueGen::Object(object);
let cls = jni.find_class("org/bukkit/util/NumberConversions"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.call_static_method(cls,"toByte",
sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
jni.translate_error(res)?;
Ok(
res.b()?
)}

	fn is_finite(jni: &blackboxmc_general::SharedJNIEnv<'mc>,f: f32) 
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

	fn check_finite(jni: &blackboxmc_general::SharedJNIEnv<'mc>,d: f32,message: impl Into<String>) 
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

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
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
    
impl<'mc> EulerAngleTrait<'mc> for EulerAngle<'mc> {}
pub trait EulerAngleTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {
/// Creates a EularAngle with each axis set to the
/// passed angle in radians
	fn new(jni: &blackboxmc_general::SharedJNIEnv<'mc>,x: f64,y: f64,z: f64) 
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
/// Returns the angle on the x axis in radians
	fn x(&self) 
-> Result<f64, Box<dyn std::error::Error>>

{let sig = String::from("()D");
let res = self.jni_ref().call_method(&self.jni_object(),"getX",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.d()?
)}
/// Returns the angle on the y axis in radians
	fn y(&self) 
-> Result<f64, Box<dyn std::error::Error>>

{let sig = String::from("()D");
let res = self.jni_ref().call_method(&self.jni_object(),"getY",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.d()?
)}
/// Returns the angle on the z axis in radians
	fn z(&self) 
-> Result<f64, Box<dyn std::error::Error>>

{let sig = String::from("()D");
let res = self.jni_ref().call_method(&self.jni_object(),"getZ",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.d()?
)}
/// Return a EulerAngle which is the result of changing
/// the x axis to the passed angle
	fn set_x(&self,x: f64) 
-> Result<crate::util::EulerAngle<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("(D)Lorg/bukkit/util/EulerAngle;");
let val_1 = jni::objects::JValueGen::Double(x);
let res = self.jni_ref().call_method(&self.jni_object(),"setX",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
crate::util::EulerAngle::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
/// Return a EulerAngle which is the result of changing
/// the y axis to the passed angle
	fn set_y(&self,y: f64) 
-> Result<crate::util::EulerAngle<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("(D)Lorg/bukkit/util/EulerAngle;");
let val_1 = jni::objects::JValueGen::Double(y);
let res = self.jni_ref().call_method(&self.jni_object(),"setY",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
crate::util::EulerAngle::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
/// Return a EulerAngle which is the result of changing
/// the z axis to the passed angle
	fn set_z(&self,z: f64) 
-> Result<crate::util::EulerAngle<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("(D)Lorg/bukkit/util/EulerAngle;");
let val_1 = jni::objects::JValueGen::Double(z);
let res = self.jni_ref().call_method(&self.jni_object(),"setZ",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
crate::util::EulerAngle::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
/// Creates a new EulerAngle which is the result of adding
/// the x, y, z components to this EulerAngle
	fn add(&self,x: f64,y: f64,z: f64) 
-> Result<crate::util::EulerAngle<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("(DDD)Lorg/bukkit/util/EulerAngle;");
let val_1 = jni::objects::JValueGen::Double(x);
let val_2 = jni::objects::JValueGen::Double(y);
let val_3 = jni::objects::JValueGen::Double(z);
let res = self.jni_ref().call_method(&self.jni_object(),"add",sig.as_str(),vec![jni::objects::JValueGen::from(val_1),jni::objects::JValueGen::from(val_2),jni::objects::JValueGen::from(val_3)]);
let res = 
self.jni_ref().translate_error(res)?;
crate::util::EulerAngle::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
/// Creates a new EulerAngle which is the result of subtracting
/// the x, y, z components to this EulerAngle
	fn subtract(&self,x: f64,y: f64,z: f64) 
-> Result<crate::util::EulerAngle<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("(DDD)Lorg/bukkit/util/EulerAngle;");
let val_1 = jni::objects::JValueGen::Double(x);
let val_2 = jni::objects::JValueGen::Double(y);
let val_3 = jni::objects::JValueGen::Double(z);
let res = self.jni_ref().call_method(&self.jni_object(),"subtract",sig.as_str(),vec![jni::objects::JValueGen::from(val_1),jni::objects::JValueGen::from(val_2),jni::objects::JValueGen::from(val_3)]);
let res = 
self.jni_ref().translate_error(res)?;
crate::util::EulerAngle::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}

	fn equals(&self,o: jni::objects::JObject<'mc>) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("(Ljava/lang/Object;)Z");
let val_1 = jni::objects::JValueGen::Object(o);
let res = self.jni_ref().call_method(&self.jni_object(),"equals",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}

	fn hash_code(&self) 
-> Result<i32, Box<dyn std::error::Error>>

{let sig = String::from("()I");
let res = self.jni_ref().call_method(&self.jni_object(),"hashCode",sig.as_str(),vec![]);
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
    
impl<'mc> RayTraceResultTrait<'mc> for RayTraceResult<'mc> {}
pub trait RayTraceResultTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {
/// Creates a RayTraceResult.
	fn new(jni: &blackboxmc_general::SharedJNIEnv<'mc>,hit_position: impl Into<crate::util::Vector<'mc>>,hit_entity: std::option::Option<impl Into<crate::entity::Entity<'mc>>>,hit_block_face: std::option::Option<impl Into<crate::block::BlockFace<'mc>>>) 
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
/// Gets the exact position of the hit.
	fn hit_position(&self) 
-> Result<crate::util::Vector<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/util/Vector;");
let res = self.jni_ref().call_method(&self.jni_object(),"getHitPosition",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::util::Vector::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
/// Gets the hit block.
	fn hit_block(&self) 
-> Result<Option<crate::block::Block<'mc>>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/block/Block;");
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
/// Gets the hit block face.
	fn hit_block_face(&self) 
-> Result<Option<crate::block::BlockFace<'mc>>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/block/BlockFace;");
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
/// Gets the hit entity.
	fn hit_entity(&self) 
-> Result<Option<crate::entity::Entity<'mc>>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/entity/Entity;");
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

	fn hash_code(&self) 
-> Result<i32, Box<dyn std::error::Error>>

{let sig = String::from("()I");
let res = self.jni_ref().call_method(&self.jni_object(),"hashCode",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.i()?
)}

	fn equals(&self,obj: jni::objects::JObject<'mc>) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("(Ljava/lang/Object;)Z");
let val_1 = jni::objects::JValueGen::Object(obj);
let res = self.jni_ref().call_method(&self.jni_object(),"equals",sig.as_str(),vec![jni::objects::JValueGen::from(val_1)]);
let res = 
self.jni_ref().translate_error(res)?;
Ok(
res.z()?
)}

#[doc(hidden)]
	fn internal_to_string(&self) 
-> Result<String, Box<dyn std::error::Error>>

{let sig = String::from("()Ljava/lang/String;");
let res = self.jni_ref().call_method(&self.jni_object(),"toString",sig.as_str(),vec![]);
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

        impl<'mc> std::string::ToString for RayTraceResult<'mc> {
            fn to_string(&self) -> String {
                match RayTraceResultTrait::internal_to_string(self) {
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
    
impl<'mc> StringUtilTrait<'mc> for StringUtil<'mc> {}
pub trait StringUtilTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {

	fn new(jni: &blackboxmc_general::SharedJNIEnv<'mc>) 
-> Result<crate::util::StringUtil<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()V");
let cls = jni.find_class("org/bukkit/util/StringUtil"); let cls = jni.translate_error_with_class(cls)?;
let res = jni.new_object(cls,
sig.as_str(),vec![]);
let res = 
jni.translate_error_no_gen(res)?;
crate::util::StringUtil::from_raw(&jni,res
)}
/// This method uses a region to check case-insensitive equality. This
/// means the internal array does not need to be copied like a
/// toLowerCase() call would.
	fn starts_with_ignore_case(jni: &blackboxmc_general::SharedJNIEnv<'mc>,string: impl Into<String>,prefix: impl Into<String>) 
-> Result<bool, Box<dyn std::error::Error>>

{let sig = String::from("(Ljava/lang/String;Ljava/lang/String;)Z");
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

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
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
    
impl<'mc> BlockTransformerTransformationStateTrait<'mc> for BlockTransformerTransformationState<'mc> {}
pub trait BlockTransformerTransformationStateTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {
/// Creates a clone of the original block state that a structure wanted
/// to place and caches it for the current transformer.
	fn original(&self) 
-> Result<crate::block::BlockState<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/block/BlockState;");
let res = self.jni_ref().call_method(&self.jni_object(),"getOriginal",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::block::BlockState::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
/// Creates a clone of the block state that was at the location of the
/// currently modified block at the start of the transformation process
/// and caches it for the current transformer.
	fn world(&self) 
-> Result<crate::block::BlockState<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/block/BlockState;");
let res = self.jni_ref().call_method(&self.jni_object(),"getWorld",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::block::BlockState::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
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
    
impl<'mc> StructureSearchResultTrait<'mc> for StructureSearchResult<'mc> {}
pub trait StructureSearchResultTrait<'mc>: blackboxmc_general::JNIRaw<'mc> + blackboxmc_general::JNIInstantiatable<'mc> {
/// Return the structure which was found.
	fn structure(&self) 
-> Result<crate::generator::structure::Structure<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/generator/structure/Structure;");
let res = self.jni_ref().call_method(&self.jni_object(),"getStructure",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::generator::structure::Structure::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}
/// Return the location of the structure.
	fn location(&self) 
-> Result<crate::Location<'mc>, Box<dyn std::error::Error>>

{let sig = String::from("()Lorg/bukkit/Location;");
let res = self.jni_ref().call_method(&self.jni_object(),"getLocation",sig.as_str(),vec![]);
let res = 
self.jni_ref().translate_error(res)?;
crate::Location::from_raw(&self.jni_ref(),unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }
)}

    fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    
}
pub mod noise;
pub mod permissions;
pub mod io;
