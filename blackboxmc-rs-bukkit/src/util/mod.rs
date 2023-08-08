#![allow(deprecated)]
use blackboxmc_general::JNIRaw;
use color_eyre::eyre::Result;
/// Utils for casting number types to other number types
pub struct NumberConversions<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for NumberConversions<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> NumberConversions<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate NumberConversions from null object.").into(),
            );
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

    pub fn square(
        jni: blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: f64,
    ) -> Result<f64, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Double(arg0.into());
        let cls = &jni.find_class("double")?;
        let res = jni.call_static_method(
            cls,
            "square",
            "(D)D",
            &[jni::objects::JValueGen::from(&val_1)],
        )?;
        Ok(res.d().unwrap())
    }

    pub fn round(
        jni: blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: f64,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Double(arg0.into());
        let cls = &jni.find_class("int")?;
        let res = jni.call_static_method(
            cls,
            "round",
            "(D)I",
            &[jni::objects::JValueGen::from(&val_1)],
        )?;
        Ok(res.i().unwrap())
    }

    pub fn to_int(
        jni: blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let cls = &jni.find_class("int")?;
        let res = jni.call_static_method(
            cls,
            "toInt",
            "(Ljava/lang/Object;)I",
            &[jni::objects::JValueGen::from(&val_1)],
        )?;
        Ok(res.i().unwrap())
    }

    pub fn check_finite_with_double(
        jni: blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: f32,
        arg1: std::option::Option<impl Into<&'mc String>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Float(arg0.into());
        let val_2 = jni::objects::JObject::from(jni.new_string(arg1.unwrap().into()).unwrap());
        let cls = &jni.find_class("void")?;
        let res = jni.call_static_method(
            cls,
            "checkFinite",
            "(FLjava/lang/String;)V",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        )?;
        Ok(())
    }

    pub fn to_double(
        jni: blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<f64, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let cls = &jni.find_class("double")?;
        let res = jni.call_static_method(
            cls,
            "toDouble",
            "(Ljava/lang/Object;)D",
            &[jni::objects::JValueGen::from(&val_1)],
        )?;
        Ok(res.d().unwrap())
    }

    pub fn to_float(
        jni: blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<f32, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let cls = &jni.find_class("float")?;
        let res = jni.call_static_method(
            cls,
            "toFloat",
            "(Ljava/lang/Object;)F",
            &[jni::objects::JValueGen::from(&val_1)],
        )?;
        Ok(res.f().unwrap())
    }

    pub fn to_short(
        jni: blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<i16, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let cls = &jni.find_class("short")?;
        let res = jni.call_static_method(
            cls,
            "toShort",
            "(Ljava/lang/Object;)S",
            &[jni::objects::JValueGen::from(&val_1)],
        )?;
        Ok(res.s().unwrap())
    }

    pub fn to_long(
        jni: blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<i64, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let cls = &jni.find_class("long")?;
        let res = jni.call_static_method(
            cls,
            "toLong",
            "(Ljava/lang/Object;)J",
            &[jni::objects::JValueGen::from(&val_1)],
        )?;
        Ok(res.j().unwrap())
    }

    pub fn to_byte(
        jni: blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<i8, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let cls = &jni.find_class("byte")?;
        let res = jni.call_static_method(
            cls,
            "toByte",
            "(Ljava/lang/Object;)B",
            &[jni::objects::JValueGen::from(&val_1)],
        )?;
        Ok(res.b().unwrap())
    }

    pub fn floor(
        jni: blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: f64,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Double(arg0.into());
        let cls = &jni.find_class("int")?;
        let res = jni.call_static_method(
            cls,
            "floor",
            "(D)I",
            &[jni::objects::JValueGen::from(&val_1)],
        )?;
        Ok(res.i().unwrap())
    }

    pub fn ceil(
        jni: blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: f64,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Double(arg0.into());
        let cls = &jni.find_class("int")?;
        let res = jni.call_static_method(
            cls,
            "ceil",
            "(D)I",
            &[jni::objects::JValueGen::from(&val_1)],
        )?;
        Ok(res.i().unwrap())
    }

    pub fn is_finite_with_double(
        jni: blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: std::option::Option<f32>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Float(arg0.unwrap().into());
        let cls = &jni.find_class("boolean")?;
        let res = jni.call_static_method(
            cls,
            "isFinite",
            "(F)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        )?;
        Ok(res.z().unwrap())
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
/// This class performs ray tracing and iterates along blocks on a line
pub struct BlockIterator<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for BlockIterator<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> BlockIterator<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate BlockIterator from null object.").into());
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
    pub fn new_with_location(
        jni: blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<&'mc crate::World<'mc>>,
        arg1: impl Into<&'mc crate::util::Vector<'mc>>,
        arg2: std::option::Option<impl Into<&'mc crate::util::Vector<'mc>>>,
        arg3: std::option::Option<f64>,
        arg4: std::option::Option<i32>,
    ) -> Result<crate::util::BlockIterator<'mc>, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let val_2 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone()) };
        let val_3 =
            unsafe { jni::objects::JObject::from_raw(arg2.unwrap().into().jni_object().clone()) };
        let val_4 = jni::objects::JValueGen::Double(arg3.unwrap().into());
        let val_5 = jni::objects::JValueGen::Int(arg4.unwrap().into());
        let cls = &jni.find_class("org/bukkit/util/BlockIterator")?;
        let res = jni.new_object(
            cls,
            "(Lorg/bukkit/World;Lorg/bukkit/util/Vector;Lorg/bukkit/util/Vector;DI)V",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
                jni::objects::JValueGen::from(&val_3),
                jni::objects::JValueGen::from(&val_4),
                jni::objects::JValueGen::from(&val_5),
            ],
        )?;
        crate::util::BlockIterator::from_raw(&jni, res)
    }
    pub fn new_with_living_entity(
        jni: blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<&'mc crate::Location<'mc>>,
        arg1: std::option::Option<f64>,
    ) -> Result<crate::util::BlockIterator<'mc>, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let val_2 = jni::objects::JValueGen::Double(arg1.unwrap().into());
        let cls = &jni.find_class("org/bukkit/util/BlockIterator")?;
        let res = jni.new_object(
            cls,
            "(Lorg/bukkit/Location;D)V",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        )?;
        crate::util::BlockIterator::from_raw(&jni, res)
    }

    pub fn remove(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "remove", "()V", &[]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Returns true if the iteration has more elements
    pub fn has_next(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasNext", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    /// Returns the next Block in the trace
    pub fn next(&mut self) -> Result<crate::block::Block<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "next",
            "()Lorg/bukkit/block/Block;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::block::Block::from_raw(&self.jni_ref(), unsafe {
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
impl<'mc> Into<blackboxmc_java::JavaIterator<'mc>> for BlockIterator<'mc> {
    fn into(self) -> blackboxmc_java::JavaIterator<'mc> {
        blackboxmc_java::JavaIterator::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
/// Represents an arbitrary affine transformation.
pub struct Transformation<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for Transformation<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Transformation<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate Transformation from null object.").into(),
            );
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
    pub unsafe fn new_with_vector3f(
        jni: blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: jni::objects::JObject<'mc>,
        arg1: jni::objects::JObject<'mc>,
        arg2: jni::objects::JObject<'mc>,
        arg3: std::option::Option<jni::objects::JObject<'mc>>,
    ) -> Result<crate::util::Transformation<'mc>, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let val_2 = arg1;
        let val_3 = arg2;
        let val_4 = arg3.unwrap();
        let cls = &jni.find_class("org/bukkit/util/Transformation")?;
        let res = jni.new_object(
            cls,
            "(Lorg/joml/Vector3f;Lorg/joml/Quaternionf;Lorg/joml/Vector3f;Lorg/joml/Quaternionf;)V",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
                jni::objects::JValueGen::from(&val_3),
                jni::objects::JValueGen::from(&val_4),
            ],
        )?;
        crate::util::Transformation::from_raw(&jni, res)
    }
    /// Gets the translation component of this transformation.
    pub fn translation(
        &mut self,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getTranslation",
            "()Lorg/joml/Vector3f;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
    /// Gets the left rotation component of this transformation.
    pub fn left_rotation(
        &mut self,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getLeftRotation",
            "()Lorg/joml/Quaternionf;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
    /// Gets the scale component of this transformation.
    pub fn scale(&mut self) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getScale",
            "()Lorg/joml/Vector3f;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
    /// Gets the right rotation component of this transformation.
    pub fn right_rotation(
        &mut self,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getRightRotation",
            "()Lorg/joml/Quaternionf;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
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
/// The hit result of a ray trace.
/// <p>Only the hit position is guaranteed to always be available. The availability of the other attributes depends on what got hit and on the context in which the ray trace was performed.</p>
pub struct RayTraceResult<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for RayTraceResult<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> RayTraceResult<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate RayTraceResult from null object.").into(),
            );
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
    pub fn new_with_vector(
        jni: blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<&'mc crate::util::Vector<'mc>>,
        arg1: impl Into<&'mc crate::block::Block<'mc>>,
        arg2: std::option::Option<impl Into<&'mc crate::block::BlockFace<'mc>>>,
    ) -> Result<crate::util::RayTraceResult<'mc>, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let val_2 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone()) };
        let val_3 =
            unsafe { jni::objects::JObject::from_raw(arg2.unwrap().into().jni_object().clone()) };
        let cls = &jni.find_class("org/bukkit/util/RayTraceResult")?;
        let res = jni.new_object(
            cls,
            "(Lorg/bukkit/util/Vector;Lorg/bukkit/block/Block;Lorg/bukkit/block/BlockFace;)V",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
                jni::objects::JValueGen::from(&val_3),
            ],
        )?;
        crate::util::RayTraceResult::from_raw(&jni, res)
    }
    /// Gets the exact position of the hit.
    pub fn hit_position(&mut self) -> Result<crate::util::Vector<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getHitPosition",
            "()Lorg/bukkit/util/Vector;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::util::Vector::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Gets the hit block.
    /// Gets the hit block face.
    pub fn hit_block(&mut self) -> Result<crate::block::Block<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getHitBlock",
            "()Lorg/bukkit/block/Block;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::block::Block::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Gets the hit block face.
    pub fn hit_block_face(
        &mut self,
    ) -> Result<crate::block::BlockFace<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getHitBlockFace",
            "()Lorg/bukkit/block/BlockFace;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant = self
            .0
            .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
        let variant_str = self
            .0
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::block::BlockFace::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::block::BlockFace::from_string(variant_str).unwrap(),
        )
    }
    /// Gets the hit entity.
    pub fn hit_entity(&mut self) -> Result<crate::entity::Entity<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getHitEntity",
            "()Lorg/bukkit/entity/Entity;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::entity::Entity::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
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
/// This is a cached version of a server-icon. It's internal representation and implementation is undefined.
///
/// This is a representation of an abstract class.
pub struct CachedServerIcon<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> CachedServerIcon<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate CachedServerIcon from null object.").into(),
            );
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
impl<'mc> JNIRaw<'mc> for CachedServerIcon<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}

pub struct ChatPaginatorChatPage<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for ChatPaginatorChatPage<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> ChatPaginatorChatPage<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate ChatPaginatorChatPage from null object."
            )
            .into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/util/ChatPaginatorChatPage")?;
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
    pub fn new(
        jni: blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: Vec<impl Into<String>>,
        arg1: i32,
        arg2: i32,
    ) -> Result<crate::util::ChatPaginatorChatPage<'mc>, Box<dyn std::error::Error>> {
        let val_2 = jni::objects::JValueGen::Int(arg1.into());
        let val_3 = jni::objects::JValueGen::Int(arg2.into());
        let cls = &jni.find_class("org/bukkit/util/ChatPaginator$ChatPage")?;
        let res = jni.new_object(
            cls,
            "(Ljava/lang/String;II)V",
            &[
                jni::objects::JValueGen::from(&val_2),
                jni::objects::JValueGen::from(&val_3),
            ],
        )?;
        crate::util::ChatPaginatorChatPage::from_raw(&jni, res)
    }

    pub fn page_number(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getPageNumber", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }

    pub fn total_pages(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getTotalPages", "()I", &[]);
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
/// A shape made out of voxels. For example, used to represent the detailed collision shape of blocks.
///
/// This is a representation of an abstract class.
pub struct VoxelShape<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> VoxelShape<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate VoxelShape from null object.").into());
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
    /// Checks if the given bounding box intersects this block shape.
    pub fn overlaps(
        &mut self,
        arg0: impl Into<&'mc crate::util::BoundingBox<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "overlaps",
            "(Lorg/bukkit/util/BoundingBox;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    /// Converts this shape into a collection of <a title="class in org.bukkit.util" href="BoundingBox.html"><code>BoundingBox</code></a> equivalent to the shape: a bounding box intersects with this block shape if it intersects with any of the shape's bounding boxes.
    pub fn bounding_boxes(
        &mut self,
    ) -> Result<Vec<crate::util::BoundingBox<'mc>>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getBoundingBoxes",
            "()Ljava/util/Collection;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let mut col = blackboxmc_java::JavaCollection::from_raw(&self.jni_ref(), res.l()?)?;
        let mut iter = blackboxmc_java::JavaIterator::from_raw(&self.jni_ref(), col.iterator()?)?;
        while iter.has_next()? {
            let obj = iter.next()?;
            new_vec.push(crate::util::BoundingBox::from_raw(&self.jni_ref(), obj)?);
        }
        Ok(new_vec)
    }
}
impl<'mc> JNIRaw<'mc> for VoxelShape<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
/// Represents an operation that accepts a single input argument and returns no result.
///
/// This is a representation of an abstract class.
pub struct Consumer<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
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
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Consumer from null object.").into());
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
    /// Performs this operation on the given argument.
    pub fn accept(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "accept",
            "(Ljava/lang/Object;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
}
impl<'mc> JNIRaw<'mc> for Consumer<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
/// Holds the result of searching for a structure.
///
/// This is a representation of an abstract class.
pub struct StructureSearchResult<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> StructureSearchResult<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate StructureSearchResult from null object."
            )
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
    /// Return the structure which was found.
    pub fn structure(
        &mut self,
    ) -> Result<crate::generator::structure::Structure<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getStructure",
            "()Lorg/bukkit/generator/structure/Structure;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::generator::structure::Structure::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Return the location of the structure.
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
}
impl<'mc> JNIRaw<'mc> for StructureSearchResult<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
/// A mutable axis aligned bounding box (AABB).
/// <p>This basically represents a rectangular box (specified by minimum and maximum corners) that can for example be used to describe the position and extents of an object (such as an entity, block, or rectangular region) in 3D space. Its edges and faces are parallel to the axes of the cartesian coordinate system.</p>
/// <p>The bounding box may be degenerate (one or more sides having the length 0).</p>
/// <p>Because bounding boxes are mutable, storing them long term may be dangerous if they get modified later. If you want to keep around a bounding box, it may be wise to call <a href="#clone()"><code>clone()</code></a> in order to get a copy.</p>
pub struct BoundingBox<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for BoundingBox<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> BoundingBox<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate BoundingBox from null object.").into());
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
    pub fn new(
        jni: blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: std::option::Option<f64>,
        arg1: std::option::Option<f64>,
        arg2: std::option::Option<f64>,
        arg3: std::option::Option<f64>,
        arg4: std::option::Option<f64>,
        arg5: std::option::Option<f64>,
    ) -> Result<crate::util::BoundingBox<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Double(arg0.unwrap().into());
        let val_2 = jni::objects::JValueGen::Double(arg1.unwrap().into());
        let val_3 = jni::objects::JValueGen::Double(arg2.unwrap().into());
        let val_4 = jni::objects::JValueGen::Double(arg3.unwrap().into());
        let val_5 = jni::objects::JValueGen::Double(arg4.unwrap().into());
        let val_6 = jni::objects::JValueGen::Double(arg5.unwrap().into());
        let cls = &jni.find_class("org/bukkit/util/BoundingBox")?;
        let res = jni.new_object(
            cls,
            "(DDDDDD)V",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
                jni::objects::JValueGen::from(&val_3),
                jni::objects::JValueGen::from(&val_4),
                jni::objects::JValueGen::from(&val_5),
                jni::objects::JValueGen::from(&val_6),
            ],
        )?;
        crate::util::BoundingBox::from_raw(&jni, res)
    }
    /// Expands this bounding box to contain (or border) the specified position.
    /// Expands this bounding box to contain (or border) the specified position.
    /// Expands this bounding box to contain (or border) the specified position.
    /// Expands this bounding box to contain both this and the given bounding box.
    pub fn union_with_bounding_box(
        &mut self,
        arg0: std::option::Option<impl Into<&'mc crate::Location<'mc>>>,
    ) -> Result<crate::util::BoundingBox<'mc>, Box<dyn std::error::Error>> {
        let val_1 =
            unsafe { jni::objects::JObject::from_raw(arg0.unwrap().into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "union",
            "(Lorg/bukkit/Location;)Lorg/bukkit/util/BoundingBox;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::util::BoundingBox::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Expands this bounding box to contain (or border) the specified position.
    /// Expands this bounding box to contain (or border) the specified position.
    /// Expands this bounding box to contain (or border) the specified position.
    /// Expands this bounding box to contain both this and the given bounding box.
    pub fn union_with_vector(
        &mut self,
        arg0: std::option::Option<f64>,
        arg1: std::option::Option<f64>,
        arg2: std::option::Option<f64>,
    ) -> Result<crate::util::BoundingBox<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Double(arg0.unwrap().into());
        let val_2 = jni::objects::JValueGen::Double(arg1.unwrap().into());
        let val_3 = jni::objects::JValueGen::Double(arg2.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "union",
            "(DDD)Lorg/bukkit/util/BoundingBox;",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
                jni::objects::JValueGen::from(&val_3),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::util::BoundingBox::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// <span class="descfrm-type-label">Description copied from interface:&nbsp;<code><a href="../configuration/serialization/ConfigurationSerializable.html#serialize()">ConfigurationSerializable</a></code></span>
    /// Creates a Map representation of this class.
    /// <p>This class must provide a method to restore this class, as defined in the <a title="interface in org.bukkit.configuration.serialization" href="../configuration/serialization/ConfigurationSerializable.html"><code>ConfigurationSerializable</code></a> interface javadocs.</p>
    pub fn serialize(
        &mut self,
    ) -> Result<blackboxmc_java::JavaMap<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "serialize", "()Ljava/util/Map;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::JavaMap::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn deserialize(
        jni: blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<&'mc blackboxmc_java::JavaMap<'mc>>,
    ) -> Result<crate::util::BoundingBox<'mc>, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let cls = &jni.find_class("org/bukkit/util/BoundingBox")?;
        let res = jni.call_static_method(
            cls,
            "deserialize",
            "(Ljava/util/Map;)Lorg/bukkit/util/BoundingBox;",
            &[jni::objects::JValueGen::from(&val_1)],
        )?;
        let obj = res.l()?;
        crate::util::BoundingBox::from_raw(&jni, obj)
    }
    /// Gets the height of the bounding box.
    pub fn height(&mut self) -> Result<f64, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getHeight", "()D", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.d().unwrap())
    }
    /// Gets the minimum x value.
    /// Gets the minimum y value.
    /// Gets the minimum z value.
    /// Gets the minimum corner as vector.
    pub fn min(&mut self) -> Result<crate::util::Vector<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getMin",
            "()Lorg/bukkit/util/Vector;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::util::Vector::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Gets the maximum x value.
    /// Gets the maximum y value.
    /// Gets the maximum z value.
    /// Gets the maximum corner as vector.
    pub fn max(&mut self) -> Result<crate::util::Vector<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getMax",
            "()Lorg/bukkit/util/Vector;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::util::Vector::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Gets the width of the bounding box in the z direction.
    pub fn width_z(&mut self) -> Result<f64, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getWidthZ", "()D", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.d().unwrap())
    }
    /// Gets the minimum x value.
    pub fn min_x(&mut self) -> Result<f64, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getMinX", "()D", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.d().unwrap())
    }
    /// Gets the minimum y value.
    pub fn min_y(&mut self) -> Result<f64, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getMinY", "()D", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.d().unwrap())
    }
    /// Gets the minimum z value.
    pub fn min_z(&mut self) -> Result<f64, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getMinZ", "()D", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.d().unwrap())
    }
    /// Gets the maximum x value.
    pub fn max_x(&mut self) -> Result<f64, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getMaxX", "()D", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.d().unwrap())
    }
    /// Gets the maximum y value.
    pub fn max_y(&mut self) -> Result<f64, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getMaxY", "()D", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.d().unwrap())
    }
    /// Gets the maximum z value.
    pub fn max_z(&mut self) -> Result<f64, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getMaxZ", "()D", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.d().unwrap())
    }
    /// Gets the width of the bounding box in the x direction.
    pub fn width_x(&mut self) -> Result<f64, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getWidthX", "()D", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.d().unwrap())
    }
    /// Gets the volume of the bounding box.
    pub fn volume(&mut self) -> Result<f64, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getVolume", "()D", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.d().unwrap())
    }
    /// Gets the x coordinate of the center of the bounding box.
    pub fn center_x(&mut self) -> Result<f64, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getCenterX", "()D", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.d().unwrap())
    }
    /// Gets the y coordinate of the center of the bounding box.
    pub fn center_y(&mut self) -> Result<f64, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getCenterY", "()D", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.d().unwrap())
    }
    /// Gets the z coordinate of the center of the bounding box.
    pub fn center_z(&mut self) -> Result<f64, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getCenterZ", "()D", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.d().unwrap())
    }
    /// Gets the x coordinate of the center of the bounding box.
    /// Gets the y coordinate of the center of the bounding box.
    /// Gets the z coordinate of the center of the bounding box.
    /// Gets the center of the bounding box.
    pub fn center(&mut self) -> Result<crate::util::Vector<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getCenter",
            "()Lorg/bukkit/util/Vector;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::util::Vector::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Expands this bounding box in the specified direction.
    /// <p>Negative values will expand the bounding box in the negative direction, positive values will expand it in the positive direction. The magnitudes of the direction components determine the corresponding amounts of expansion.</p>
    /// Expands this bounding box in the specified direction.
    /// <p>Negative values will expand the bounding box in the negative direction, positive values will expand it in the positive direction. The magnitude of the direction vector determines the amount of expansion.</p>
    pub fn expand_directional_with_vector(
        &mut self,
        arg0: std::option::Option<f64>,
        arg1: std::option::Option<f64>,
        arg2: std::option::Option<f64>,
    ) -> Result<crate::util::BoundingBox<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Double(arg0.unwrap().into());
        let val_2 = jni::objects::JValueGen::Double(arg1.unwrap().into());
        let val_3 = jni::objects::JValueGen::Double(arg2.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "expandDirectional",
            "(DDD)Lorg/bukkit/util/BoundingBox;",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
                jni::objects::JValueGen::from(&val_3),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::util::BoundingBox::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Resizes this bounding box to represent the intersection of this and the given bounding box.
    pub fn intersection(
        &mut self,
        arg0: impl Into<&'mc crate::util::BoundingBox<'mc>>,
    ) -> Result<crate::util::BoundingBox<'mc>, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "intersection",
            "(Lorg/bukkit/util/BoundingBox;)Lorg/bukkit/util/BoundingBox;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::util::BoundingBox::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Checks if this bounding box overlaps with the given bounding box.
    /// <p>Bounding boxes that are only intersecting at the borders are not considered overlapping.</p>
    /// Checks if this bounding box overlaps with the bounding box that is defined by the given corners.
    /// <p>Bounding boxes that are only intersecting at the borders are not considered overlapping.</p>
    pub fn overlaps_with_bounding_box(
        &mut self,
        arg0: std::option::Option<impl Into<&'mc crate::util::Vector<'mc>>>,
        arg1: std::option::Option<impl Into<&'mc crate::util::Vector<'mc>>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 =
            unsafe { jni::objects::JObject::from_raw(arg0.unwrap().into().jni_object().clone()) };
        let val_2 =
            unsafe { jni::objects::JObject::from_raw(arg1.unwrap().into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "overlaps",
            "(Lorg/bukkit/util/Vector;Lorg/bukkit/util/Vector;)Z",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    /// Calculates the intersection of this bounding box with the specified line segment.
    /// <p>Intersections at edges and corners yield one of the affected block faces as hit result, but it is not defined which of them.</p>
    pub fn ray_trace(
        &mut self,
        arg0: impl Into<&'mc crate::util::Vector<'mc>>,
        arg1: impl Into<&'mc crate::util::Vector<'mc>>,
        arg2: f64,
    ) -> Result<crate::util::RayTraceResult<'mc>, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let val_2 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone()) };
        let val_3 = jni::objects::JValueGen::Double(arg2.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "rayTrace",
            "(Lorg/bukkit/util/Vector;Lorg/bukkit/util/Vector;D)Lorg/bukkit/util/RayTraceResult;",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
                jni::objects::JValueGen::from(&val_3),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::util::RayTraceResult::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
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
    /// Creates a copy of this bounding box.
    pub fn clone(&mut self) -> Result<crate::util::BoundingBox<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "clone",
            "()Lorg/bukkit/util/BoundingBox;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::util::BoundingBox::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Creates a new bounding box using the coordinates of the given vectors as corners.
    /// Creates a new bounding box using the coordinates of the given locations as corners.
    /// Creates a new bounding box using the coordinates of the given blocks as corners.
    /// <p>The bounding box will be sized to fully contain both blocks.</p>
    /// Creates a new 1x1x1 sized bounding box containing the given block.
    /// Creates a new bounding box using the given center and extents.
    /// Creates a new bounding box using the given center and extents.
    pub fn of_with_block(
        jni: blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<&'mc crate::Location<'mc>>,
        arg1: std::option::Option<impl Into<&'mc crate::Location<'mc>>>,
    ) -> Result<crate::util::BoundingBox<'mc>, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let val_2 =
            unsafe { jni::objects::JObject::from_raw(arg1.unwrap().into().jni_object().clone()) };
        let cls = &jni.find_class("org/bukkit/util/BoundingBox")?;
        let res = jni.call_static_method(
            cls,
            "of",
            "(Lorg/bukkit/Location;Lorg/bukkit/Location;)Lorg/bukkit/util/BoundingBox;",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        )?;
        let obj = res.l()?;
        crate::util::BoundingBox::from_raw(&jni, obj)
    }
    /// Creates a new bounding box using the coordinates of the given vectors as corners.
    /// Creates a new bounding box using the coordinates of the given locations as corners.
    /// Creates a new bounding box using the coordinates of the given blocks as corners.
    /// <p>The bounding box will be sized to fully contain both blocks.</p>
    /// Creates a new 1x1x1 sized bounding box containing the given block.
    /// Creates a new bounding box using the given center and extents.
    /// Creates a new bounding box using the given center and extents.
    pub fn of_with_location(
        jni: blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<&'mc crate::util::Vector<'mc>>,
        arg1: f64,
        arg2: f64,
        arg3: std::option::Option<f64>,
    ) -> Result<crate::util::BoundingBox<'mc>, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let val_2 = jni::objects::JValueGen::Double(arg1.into());
        let val_3 = jni::objects::JValueGen::Double(arg2.into());
        let val_4 = jni::objects::JValueGen::Double(arg3.unwrap().into());
        let cls = &jni.find_class("org/bukkit/util/BoundingBox")?;
        let res = jni.call_static_method(
            cls,
            "of",
            "(Lorg/bukkit/util/Vector;DDD)Lorg/bukkit/util/BoundingBox;",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
                jni::objects::JValueGen::from(&val_3),
                jni::objects::JValueGen::from(&val_4),
            ],
        )?;
        let obj = res.l()?;
        crate::util::BoundingBox::from_raw(&jni, obj)
    }
    /// Checks if this bounding box contains the specified position.
    /// <p>Positions exactly on the minimum borders of the bounding box are considered to be inside the bounding box, while positions exactly on the maximum borders are considered to be outside. This allows bounding boxes to reside directly next to each other with positions always only residing in exactly one of them.</p>
    /// Checks if this bounding box contains the specified position.
    /// <p>Positions exactly on the minimum borders of the bounding box are considered to be inside the bounding box, while positions exactly on the maximum borders are considered to be outside. This allows bounding boxes to reside directly next to each other with positions always only residing in exactly one of them.</p>
    /// Checks if this bounding box fully contains the given bounding box.
    /// Checks if this bounding box fully contains the bounding box that is defined by the given corners.
    pub fn contains_with_bounding_box(
        &mut self,
        arg0: std::option::Option<impl Into<&'mc crate::util::Vector<'mc>>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 =
            unsafe { jni::objects::JObject::from_raw(arg0.unwrap().into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "contains",
            "(Lorg/bukkit/util/Vector;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    /// Checks if this bounding box contains the specified position.
    /// <p>Positions exactly on the minimum borders of the bounding box are considered to be inside the bounding box, while positions exactly on the maximum borders are considered to be outside. This allows bounding boxes to reside directly next to each other with positions always only residing in exactly one of them.</p>
    /// Checks if this bounding box contains the specified position.
    /// <p>Positions exactly on the minimum borders of the bounding box are considered to be inside the bounding box, while positions exactly on the maximum borders are considered to be outside. This allows bounding boxes to reside directly next to each other with positions always only residing in exactly one of them.</p>
    /// Checks if this bounding box fully contains the given bounding box.
    /// Checks if this bounding box fully contains the bounding box that is defined by the given corners.
    pub fn contains_with_vector(
        &mut self,
        arg0: f64,
        arg1: std::option::Option<f64>,
        arg2: std::option::Option<f64>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Double(arg0.into());
        let val_2 = jni::objects::JValueGen::Double(arg1.unwrap().into());
        let val_3 = jni::objects::JValueGen::Double(arg2.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "contains",
            "(DDD)Z",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
                jni::objects::JValueGen::from(&val_3),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    /// Copies another bounding box.
    pub fn copy(
        &mut self,
        arg0: impl Into<&'mc crate::util::BoundingBox<'mc>>,
    ) -> Result<crate::util::BoundingBox<'mc>, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "copy",
            "(Lorg/bukkit/util/BoundingBox;)Lorg/bukkit/util/BoundingBox;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::util::BoundingBox::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Expands this bounding box by the given values in the corresponding directions.
    /// <p>Negative values will shrink the bounding box in the corresponding direction. Shrinking will be limited to the point where the affected opposite faces would meet if the they shrank at uniform speeds.</p>
    /// Expands this bounding box uniformly by the given values in both positive and negative directions.
    /// <p>Negative values will shrink the bounding box. Shrinking will be limited to the bounding box's current size.</p>
    /// Expands this bounding box uniformly by the given values in both positive and negative directions.
    /// <p>Negative values will shrink the bounding box. Shrinking will be limited to the bounding box's current size.</p>
    /// Expands this bounding box uniformly by the given value in all directions.
    /// <p>A negative value will shrink the bounding box. Shrinking will be limited to the bounding box's current size.</p>
    /// Expands this bounding box in the specified direction.
    /// <p>The magnitude of the direction will scale the expansion. A negative expansion value will shrink the bounding box in this direction. Shrinking will be limited to the bounding box's current size.</p>
    /// Expands this bounding box in the specified direction.
    /// <p>The magnitude of the direction will scale the expansion. A negative expansion value will shrink the bounding box in this direction. Shrinking will be limited to the bounding box's current size.</p>
    /// Expands this bounding box in the direction specified by the given block face.
    /// <p>A negative expansion value will shrink the bounding box in this direction. Shrinking will be limited to the bounding box's current size.</p>
    /// Expands this bounding box in the specified direction.
    /// <p>Negative values will expand the bounding box in the negative direction, positive values will expand it in the positive direction. The magnitudes of the direction components determine the corresponding amounts of expansion.</p>
    /// Expands this bounding box in the specified direction.
    /// <p>Negative values will expand the bounding box in the negative direction, positive values will expand it in the positive direction. The magnitude of the direction vector determines the amount of expansion.</p>
    pub fn expand_with_double(
        &mut self,
        arg0: f64,
        arg1: f64,
        arg2: f64,
        arg3: f64,
        arg4: f64,
        arg5: std::option::Option<f64>,
    ) -> Result<crate::util::BoundingBox<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Double(arg0.into());
        let val_2 = jni::objects::JValueGen::Double(arg1.into());
        let val_3 = jni::objects::JValueGen::Double(arg2.into());
        let val_4 = jni::objects::JValueGen::Double(arg3.into());
        let val_5 = jni::objects::JValueGen::Double(arg4.into());
        let val_6 = jni::objects::JValueGen::Double(arg5.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "expand",
            "(DDDDDD)Lorg/bukkit/util/BoundingBox;",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
                jni::objects::JValueGen::from(&val_3),
                jni::objects::JValueGen::from(&val_4),
                jni::objects::JValueGen::from(&val_5),
                jni::objects::JValueGen::from(&val_6),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::util::BoundingBox::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Expands this bounding box by the given values in the corresponding directions.
    /// <p>Negative values will shrink the bounding box in the corresponding direction. Shrinking will be limited to the point where the affected opposite faces would meet if the they shrank at uniform speeds.</p>
    /// Expands this bounding box uniformly by the given values in both positive and negative directions.
    /// <p>Negative values will shrink the bounding box. Shrinking will be limited to the bounding box's current size.</p>
    /// Expands this bounding box uniformly by the given values in both positive and negative directions.
    /// <p>Negative values will shrink the bounding box. Shrinking will be limited to the bounding box's current size.</p>
    /// Expands this bounding box uniformly by the given value in all directions.
    /// <p>A negative value will shrink the bounding box. Shrinking will be limited to the bounding box's current size.</p>
    /// Expands this bounding box in the specified direction.
    /// <p>The magnitude of the direction will scale the expansion. A negative expansion value will shrink the bounding box in this direction. Shrinking will be limited to the bounding box's current size.</p>
    /// Expands this bounding box in the specified direction.
    /// <p>The magnitude of the direction will scale the expansion. A negative expansion value will shrink the bounding box in this direction. Shrinking will be limited to the bounding box's current size.</p>
    /// Expands this bounding box in the direction specified by the given block face.
    /// <p>A negative expansion value will shrink the bounding box in this direction. Shrinking will be limited to the bounding box's current size.</p>
    /// Expands this bounding box in the specified direction.
    /// <p>Negative values will expand the bounding box in the negative direction, positive values will expand it in the positive direction. The magnitudes of the direction components determine the corresponding amounts of expansion.</p>
    /// Expands this bounding box in the specified direction.
    /// <p>Negative values will expand the bounding box in the negative direction, positive values will expand it in the positive direction. The magnitude of the direction vector determines the amount of expansion.</p>
    pub fn expand_with_vector(
        &mut self,
        arg0: impl Into<&'mc crate::block::BlockFace<'mc>>,
        arg1: std::option::Option<f64>,
    ) -> Result<crate::util::BoundingBox<'mc>, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let val_2 = jni::objects::JValueGen::Double(arg1.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "expand",
            "(Lorg/bukkit/block/BlockFace;D)Lorg/bukkit/util/BoundingBox;",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::util::BoundingBox::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Shifts this bounding box by the given amounts.
    /// Shifts this bounding box by the given amounts.
    /// Shifts this bounding box by the given amounts.
    pub fn shift_with_location(
        &mut self,
        arg0: std::option::Option<impl Into<&'mc crate::util::Vector<'mc>>>,
    ) -> Result<crate::util::BoundingBox<'mc>, Box<dyn std::error::Error>> {
        let val_1 =
            unsafe { jni::objects::JObject::from_raw(arg0.unwrap().into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "shift",
            "(Lorg/bukkit/util/Vector;)Lorg/bukkit/util/BoundingBox;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::util::BoundingBox::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Shifts this bounding box by the given amounts.
    /// Shifts this bounding box by the given amounts.
    /// Shifts this bounding box by the given amounts.
    pub fn shift_with_double(
        &mut self,
        arg0: f64,
        arg1: f64,
        arg2: std::option::Option<f64>,
    ) -> Result<crate::util::BoundingBox<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Double(arg0.into());
        let val_2 = jni::objects::JValueGen::Double(arg1.into());
        let val_3 = jni::objects::JValueGen::Double(arg2.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "shift",
            "(DDD)Lorg/bukkit/util/BoundingBox;",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
                jni::objects::JValueGen::from(&val_3),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::util::BoundingBox::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Resizes this bounding box.
    pub fn resize(
        &mut self,
        arg0: f64,
        arg1: f64,
        arg2: f64,
        arg3: f64,
        arg4: f64,
        arg5: f64,
    ) -> Result<crate::util::BoundingBox<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Double(arg0.into());
        let val_2 = jni::objects::JValueGen::Double(arg1.into());
        let val_3 = jni::objects::JValueGen::Double(arg2.into());
        let val_4 = jni::objects::JValueGen::Double(arg3.into());
        let val_5 = jni::objects::JValueGen::Double(arg4.into());
        let val_6 = jni::objects::JValueGen::Double(arg5.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "resize",
            "(DDDDDD)Lorg/bukkit/util/BoundingBox;",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
                jni::objects::JValueGen::from(&val_3),
                jni::objects::JValueGen::from(&val_4),
                jni::objects::JValueGen::from(&val_5),
                jni::objects::JValueGen::from(&val_6),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::util::BoundingBox::from_raw(&self.jni_ref(), unsafe {
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
impl<'mc> Into<crate::configuration::serialization::ConfigurationSerializable<'mc>>
    for BoundingBox<'mc>
{
    fn into(self) -> crate::configuration::serialization::ConfigurationSerializable<'mc> {
        crate::configuration::serialization::ConfigurationSerializable::from_raw(
            &self.jni_ref(),
            self.1,
        )
        .unwrap()
    }
}
/// A vector with a hash function that floors the X, Y, Z components, a la BlockVector in WorldEdit. BlockVectors can be used in hash sets and hash maps. Be aware that BlockVectors are mutable, but it is important that BlockVectors are never changed once put into a hash set or hash map.
pub struct BlockVector<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for BlockVector<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> BlockVector<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate BlockVector from null object.").into());
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
    pub fn new(
        jni: blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: std::option::Option<f64>,
        arg1: std::option::Option<f64>,
        arg2: std::option::Option<f64>,
    ) -> Result<crate::util::BlockVector<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Double(arg0.unwrap().into());
        let val_2 = jni::objects::JValueGen::Double(arg1.unwrap().into());
        let val_3 = jni::objects::JValueGen::Double(arg2.unwrap().into());
        let cls = &jni.find_class("org/bukkit/util/BlockVector")?;
        let res = jni.new_object(
            cls,
            "(DDD)V",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
                jni::objects::JValueGen::from(&val_3),
            ],
        )?;
        crate::util::BlockVector::from_raw(&jni, res)
    }
    pub fn new_with_float(
        jni: blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: i32,
        arg1: i32,
        arg2: std::option::Option<i32>,
    ) -> Result<crate::util::BlockVector<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let val_2 = jni::objects::JValueGen::Int(arg1.into());
        let val_3 = jni::objects::JValueGen::Int(arg2.unwrap().into());
        let cls = &jni.find_class("org/bukkit/util/BlockVector")?;
        let res = jni.new_object(
            cls,
            "(III)V",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
                jni::objects::JValueGen::from(&val_3),
            ],
        )?;
        crate::util::BlockVector::from_raw(&jni, res)
    }

    pub fn deserialize_with_map(
        jni: blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: std::option::Option<impl Into<&'mc blackboxmc_java::JavaMap<'mc>>>,
    ) -> Result<crate::util::Vector<'mc>, Box<dyn std::error::Error>> {
        let val_1 =
            unsafe { jni::objects::JObject::from_raw(arg0.unwrap().into().jni_object().clone()) };
        let cls = &jni.find_class("org/bukkit/util/Vector")?;
        let res = jni.call_static_method(
            cls,
            "deserialize",
            "(Ljava/util/Map;)Lorg/bukkit/util/Vector;",
            &[jni::objects::JValueGen::from(&val_1)],
        )?;
        let obj = res.l()?;
        crate::util::Vector::from_raw(&jni, obj)
    }
    /// Checks if another object is equivalent.
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
    /// Returns a hash code for this vector.
    pub fn hash_code(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hashCode", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }
    /// Get a new block vector.
    pub fn clone(&mut self) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "clone", "()Ljava/lang/Object;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }

    pub fn get_minimum(
        jni: blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<&'mc crate::util::Vector<'mc>>,
        arg1: impl Into<&'mc crate::util::Vector<'mc>>,
    ) -> Result<crate::util::Vector<'mc>, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let val_2 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone()) };
        let cls = &jni.find_class("org/bukkit/util/Vector")?;
        let res = jni.call_static_method(
            cls,
            "getMinimum",
            "(Lorg/bukkit/util/Vector;Lorg/bukkit/util/Vector;)Lorg/bukkit/util/Vector;",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        )?;
        let obj = res.l()?;
        crate::util::Vector::from_raw(&jni, obj)
    }

    pub fn get_maximum(
        jni: blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<&'mc crate::util::Vector<'mc>>,
        arg1: impl Into<&'mc crate::util::Vector<'mc>>,
    ) -> Result<crate::util::Vector<'mc>, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let val_2 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone()) };
        let cls = &jni.find_class("org/bukkit/util/Vector")?;
        let res = jni.call_static_method(
            cls,
            "getMaximum",
            "(Lorg/bukkit/util/Vector;Lorg/bukkit/util/Vector;)Lorg/bukkit/util/Vector;",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        )?;
        let obj = res.l()?;
        crate::util::Vector::from_raw(&jni, obj)
    }

    pub fn is_normalized(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isNormalized", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn subtract(
        &mut self,
        arg0: impl Into<&'mc crate::util::Vector<'mc>>,
    ) -> Result<crate::util::Vector<'mc>, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "subtract",
            "(Lorg/bukkit/util/Vector;)Lorg/bukkit/util/Vector;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::util::Vector::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn divide(
        &mut self,
        arg0: impl Into<&'mc crate::util::Vector<'mc>>,
    ) -> Result<crate::util::Vector<'mc>, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "divide",
            "(Lorg/bukkit/util/Vector;)Lorg/bukkit/util/Vector;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::util::Vector::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn is_zero(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isZero", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn multiply_with_int(
        &mut self,
        arg0: std::option::Option<f32>,
    ) -> Result<crate::util::Vector<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Float(arg0.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "multiply",
            "(F)Lorg/bukkit/util/Vector;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::util::Vector::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn multiply_with_double(
        &mut self,
        arg0: std::option::Option<impl Into<&'mc crate::util::Vector<'mc>>>,
    ) -> Result<crate::util::Vector<'mc>, Box<dyn std::error::Error>> {
        let val_1 =
            unsafe { jni::objects::JObject::from_raw(arg0.unwrap().into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "multiply",
            "(Lorg/bukkit/util/Vector;)Lorg/bukkit/util/Vector;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::util::Vector::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn check_finite(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "checkFinite", "()V", &[]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn serialize(
        &mut self,
    ) -> Result<blackboxmc_java::JavaMap<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "serialize", "()Ljava/util/Map;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::JavaMap::from_raw(&self.jni_ref(), unsafe {
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

    pub fn set_x_with_float(
        &mut self,
        arg0: std::option::Option<f64>,
    ) -> Result<crate::util::Vector<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Double(arg0.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setX",
            "(D)Lorg/bukkit/util/Vector;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::util::Vector::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn set_x_with_int(
        &mut self,
        arg0: std::option::Option<i32>,
    ) -> Result<crate::util::Vector<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setX",
            "(I)Lorg/bukkit/util/Vector;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::util::Vector::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn block_x(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getBlockX", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }

    pub fn set_y_with_float(
        &mut self,
        arg0: std::option::Option<f64>,
    ) -> Result<crate::util::Vector<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Double(arg0.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setY",
            "(D)Lorg/bukkit/util/Vector;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::util::Vector::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn set_y_with_int(
        &mut self,
        arg0: std::option::Option<i32>,
    ) -> Result<crate::util::Vector<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setY",
            "(I)Lorg/bukkit/util/Vector;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::util::Vector::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn block_y(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getBlockY", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }

    pub fn set_z_with_float(
        &mut self,
        arg0: std::option::Option<f64>,
    ) -> Result<crate::util::Vector<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Double(arg0.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setZ",
            "(D)Lorg/bukkit/util/Vector;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::util::Vector::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn set_z_with_int(
        &mut self,
        arg0: std::option::Option<i32>,
    ) -> Result<crate::util::Vector<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setZ",
            "(I)Lorg/bukkit/util/Vector;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::util::Vector::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn block_z(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getBlockZ", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }

    pub fn length_squared(&mut self) -> Result<f64, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "lengthSquared", "()D", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.d().unwrap())
    }

    pub fn distance_squared(
        &mut self,
        arg0: impl Into<&'mc crate::util::Vector<'mc>>,
    ) -> Result<f64, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "distanceSquared",
            "(Lorg/bukkit/util/Vector;)D",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.d().unwrap())
    }

    pub fn angle(
        &mut self,
        arg0: impl Into<&'mc crate::util::Vector<'mc>>,
    ) -> Result<f32, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "angle",
            "(Lorg/bukkit/util/Vector;)F",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.f().unwrap())
    }

    pub fn midpoint(
        &mut self,
        arg0: impl Into<&'mc crate::util::Vector<'mc>>,
    ) -> Result<crate::util::Vector<'mc>, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "midpoint",
            "(Lorg/bukkit/util/Vector;)Lorg/bukkit/util/Vector;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::util::Vector::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn get_midpoint(
        &mut self,
        arg0: impl Into<&'mc crate::util::Vector<'mc>>,
    ) -> Result<crate::util::Vector<'mc>, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getMidpoint",
            "(Lorg/bukkit/util/Vector;)Lorg/bukkit/util/Vector;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::util::Vector::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn cross_product(
        &mut self,
        arg0: impl Into<&'mc crate::util::Vector<'mc>>,
    ) -> Result<crate::util::Vector<'mc>, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "crossProduct",
            "(Lorg/bukkit/util/Vector;)Lorg/bukkit/util/Vector;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::util::Vector::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn get_cross_product(
        &mut self,
        arg0: impl Into<&'mc crate::util::Vector<'mc>>,
    ) -> Result<crate::util::Vector<'mc>, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getCrossProduct",
            "(Lorg/bukkit/util/Vector;)Lorg/bukkit/util/Vector;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::util::Vector::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn is_in_aabb(
        &mut self,
        arg0: impl Into<&'mc crate::util::Vector<'mc>>,
        arg1: impl Into<&'mc crate::util::Vector<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let val_2 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "isInAABB",
            "(Lorg/bukkit/util/Vector;Lorg/bukkit/util/Vector;)Z",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn is_in_sphere(
        &mut self,
        arg0: impl Into<&'mc crate::util::Vector<'mc>>,
        arg1: f64,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let val_2 = jni::objects::JValueGen::Double(arg1.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "isInSphere",
            "(Lorg/bukkit/util/Vector;D)Z",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn epsilon(
        jni: blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<f64, Box<dyn std::error::Error>> {
        let cls = &jni.find_class("double")?;
        let res = jni.call_static_method(cls, "getEpsilon", "()D", &[])?;
        Ok(res.d().unwrap())
    }

    pub fn rotate_around_x(
        &mut self,
        arg0: f64,
    ) -> Result<crate::util::Vector<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Double(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "rotateAroundX",
            "(D)Lorg/bukkit/util/Vector;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::util::Vector::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn rotate_around_y(
        &mut self,
        arg0: f64,
    ) -> Result<crate::util::Vector<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Double(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "rotateAroundY",
            "(D)Lorg/bukkit/util/Vector;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::util::Vector::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn rotate_around_z(
        &mut self,
        arg0: f64,
    ) -> Result<crate::util::Vector<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Double(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "rotateAroundZ",
            "(D)Lorg/bukkit/util/Vector;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::util::Vector::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn rotate_around_axis(
        &mut self,
        arg0: impl Into<&'mc crate::util::Vector<'mc>>,
        arg1: f64,
    ) -> Result<crate::util::Vector<'mc>, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let val_2 = jni::objects::JValueGen::Double(arg1.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "rotateAroundAxis",
            "(Lorg/bukkit/util/Vector;D)Lorg/bukkit/util/Vector;",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::util::Vector::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn rotate_around_non_unit_axis(
        &mut self,
        arg0: impl Into<&'mc crate::util::Vector<'mc>>,
        arg1: f64,
    ) -> Result<crate::util::Vector<'mc>, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let val_2 = jni::objects::JValueGen::Double(arg1.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "rotateAroundNonUnitAxis",
            "(Lorg/bukkit/util/Vector;D)Lorg/bukkit/util/Vector;",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::util::Vector::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn to_location_with_world(
        &mut self,
        arg0: std::option::Option<impl Into<&'mc crate::World<'mc>>>,
        arg1: std::option::Option<f32>,
        arg2: std::option::Option<f32>,
    ) -> Result<crate::Location<'mc>, Box<dyn std::error::Error>> {
        let val_1 =
            unsafe { jni::objects::JObject::from_raw(arg0.unwrap().into().jni_object().clone()) };
        let val_2 = jni::objects::JValueGen::Float(arg1.unwrap().into());
        let val_3 = jni::objects::JValueGen::Float(arg2.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "toLocation",
            "(Lorg/bukkit/World;FF)Lorg/bukkit/Location;",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
                jni::objects::JValueGen::from(&val_3),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::Location::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn to_block_vector(
        &mut self,
    ) -> Result<crate::util::BlockVector<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "toBlockVector",
            "()Lorg/bukkit/util/BlockVector;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::util::BlockVector::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn to_vector3f(
        &mut self,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "toVector3f",
            "()Lorg/joml/Vector3f;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }

    pub fn to_vector3d(
        &mut self,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "toVector3d",
            "()Lorg/joml/Vector3d;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }

    pub fn to_vector3i(
        &mut self,
        arg0: std::option::Option<i32>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "toVector3i",
            "(I)Lorg/joml/Vector3i;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }

    pub fn random(
        jni: blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<crate::util::Vector<'mc>, Box<dyn std::error::Error>> {
        let cls = &jni.find_class("org/bukkit/util/Vector")?;
        let res = jni.call_static_method(cls, "getRandom", "()Lorg/bukkit/util/Vector;", &[])?;
        let obj = res.l()?;
        crate::util::Vector::from_raw(&jni, obj)
    }

    pub unsafe fn from_joml_with_vector3f(
        jni: blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: std::option::Option<jni::objects::JObject<'mc>>,
    ) -> Result<crate::util::Vector<'mc>, Box<dyn std::error::Error>> {
        let val_1 = arg0.unwrap();
        let cls = &jni.find_class("org/bukkit/util/Vector")?;
        let res = jni.call_static_method(
            cls,
            "fromJOML",
            "(Lorg/joml/Vector3i;)Lorg/bukkit/util/Vector;",
            &[jni::objects::JValueGen::from(&val_1)],
        )?;
        let obj = res.l()?;
        crate::util::Vector::from_raw(&jni, obj)
    }

    pub unsafe fn from_joml_with_vector3d(
        jni: blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: std::option::Option<jni::objects::JObject<'mc>>,
    ) -> Result<crate::util::Vector<'mc>, Box<dyn std::error::Error>> {
        let val_1 = arg0.unwrap();
        let cls = &jni.find_class("org/bukkit/util/Vector")?;
        let res = jni.call_static_method(
            cls,
            "fromJOML",
            "(Lorg/joml/Vector3d;)Lorg/bukkit/util/Vector;",
            &[jni::objects::JValueGen::from(&val_1)],
        )?;
        let obj = res.l()?;
        crate::util::Vector::from_raw(&jni, obj)
    }

    pub fn add(
        &mut self,
        arg0: impl Into<&'mc crate::util::Vector<'mc>>,
    ) -> Result<crate::util::Vector<'mc>, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "add",
            "(Lorg/bukkit/util/Vector;)Lorg/bukkit/util/Vector;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::util::Vector::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn length(&mut self) -> Result<f64, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "length", "()D", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.d().unwrap())
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

    pub fn dot(
        &mut self,
        arg0: impl Into<&'mc crate::util::Vector<'mc>>,
    ) -> Result<f64, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "dot",
            "(Lorg/bukkit/util/Vector;)D",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.d().unwrap())
    }

    pub fn copy(
        &mut self,
        arg0: impl Into<&'mc crate::util::Vector<'mc>>,
    ) -> Result<crate::util::Vector<'mc>, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "copy",
            "(Lorg/bukkit/util/Vector;)Lorg/bukkit/util/Vector;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::util::Vector::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn normalize(&mut self) -> Result<crate::util::Vector<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "normalize",
            "()Lorg/bukkit/util/Vector;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::util::Vector::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn zero(&mut self) -> Result<crate::util::Vector<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "zero",
            "()Lorg/bukkit/util/Vector;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::util::Vector::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn distance(
        &mut self,
        arg0: impl Into<&'mc crate::util::Vector<'mc>>,
    ) -> Result<f64, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "distance",
            "(Lorg/bukkit/util/Vector;)D",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.d().unwrap())
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
impl<'mc> Into<crate::util::Vector<'mc>> for BlockVector<'mc> {
    fn into(self) -> crate::util::Vector<'mc> {
        crate::util::Vector::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
/// EulerAngle is used to represent 3 angles, one for each axis (x, y, z). The angles are in radians
pub struct EulerAngle<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for EulerAngle<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> EulerAngle<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate EulerAngle from null object.").into());
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
    pub fn new(
        jni: blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: f64,
        arg1: f64,
        arg2: f64,
    ) -> Result<crate::util::EulerAngle<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Double(arg0.into());
        let val_2 = jni::objects::JValueGen::Double(arg1.into());
        let val_3 = jni::objects::JValueGen::Double(arg2.into());
        let cls = &jni.find_class("org/bukkit/util/EulerAngle")?;
        let res = jni.new_object(
            cls,
            "(DDD)V",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
                jni::objects::JValueGen::from(&val_3),
            ],
        )?;
        crate::util::EulerAngle::from_raw(&jni, res)
    }
    /// Creates a new EulerAngle which is the result of subtracting the x, y, z components to this EulerAngle
    pub fn subtract(
        &mut self,
        arg0: f64,
        arg1: f64,
        arg2: f64,
    ) -> Result<crate::util::EulerAngle<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Double(arg0.into());
        let val_2 = jni::objects::JValueGen::Double(arg1.into());
        let val_3 = jni::objects::JValueGen::Double(arg2.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "subtract",
            "(DDD)Lorg/bukkit/util/EulerAngle;",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
                jni::objects::JValueGen::from(&val_3),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::util::EulerAngle::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Returns the angle on the x axis in radians
    pub fn x(&mut self) -> Result<f64, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getX", "()D", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.d().unwrap())
    }
    /// Returns the angle on the y axis in radians
    pub fn y(&mut self) -> Result<f64, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getY", "()D", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.d().unwrap())
    }
    /// Returns the angle on the z axis in radians
    pub fn z(&mut self) -> Result<f64, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getZ", "()D", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.d().unwrap())
    }
    /// Return a EulerAngle which is the result of changing the x axis to the passed angle
    pub fn set_x(
        &mut self,
        arg0: f64,
    ) -> Result<crate::util::EulerAngle<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Double(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setX",
            "(D)Lorg/bukkit/util/EulerAngle;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::util::EulerAngle::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Return a EulerAngle which is the result of changing the y axis to the passed angle
    pub fn set_y(
        &mut self,
        arg0: f64,
    ) -> Result<crate::util::EulerAngle<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Double(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setY",
            "(D)Lorg/bukkit/util/EulerAngle;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::util::EulerAngle::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Return a EulerAngle which is the result of changing the z axis to the passed angle
    pub fn set_z(
        &mut self,
        arg0: f64,
    ) -> Result<crate::util::EulerAngle<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Double(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setZ",
            "(D)Lorg/bukkit/util/EulerAngle;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::util::EulerAngle::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Creates a new EulerAngle which is the result of adding the x, y, z components to this EulerAngle
    pub fn add(
        &mut self,
        arg0: f64,
        arg1: f64,
        arg2: f64,
    ) -> Result<crate::util::EulerAngle<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Double(arg0.into());
        let val_2 = jni::objects::JValueGen::Double(arg1.into());
        let val_3 = jni::objects::JValueGen::Double(arg2.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "add",
            "(DDD)Lorg/bukkit/util/EulerAngle;",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
                jni::objects::JValueGen::from(&val_3),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::util::EulerAngle::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
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

    pub fn hash_code(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hashCode", "()I", &[]);
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

pub struct StringUtil<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for StringUtil<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> StringUtil<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate StringUtil from null object.").into());
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
    pub fn new(
        jni: blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<crate::util::StringUtil<'mc>, Box<dyn std::error::Error>> {
        let cls = &jni.find_class("org/bukkit/util/StringUtil")?;
        let res = jni.new_object(cls, "()V", &[])?;
        crate::util::StringUtil::from_raw(&jni, res)
    }
    /// This method uses a region to check case-insensitive equality. This means the internal array does not need to be copied like a toLowerCase() call would.
    pub fn starts_with_ignore_case(
        jni: blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<&'mc String>,
        arg1: impl Into<&'mc String>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(jni.new_string(arg0.into()).unwrap());
        let val_2 = jni::objects::JObject::from(jni.new_string(arg1.into()).unwrap());
        let cls = &jni.find_class("boolean")?;
        let res = jni.call_static_method(
            cls,
            "startsWithIgnoreCase",
            "(Ljava/lang/String;Ljava/lang/String;)Z",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        )?;
        Ok(res.z().unwrap())
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
/// The ChatPaginator takes a raw string of arbitrary length and breaks it down into an array of strings appropriate for displaying on the Minecraft player console.
pub struct ChatPaginator<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for ChatPaginator<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> ChatPaginator<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate ChatPaginator from null object.").into());
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
    pub fn new(
        jni: blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<crate::util::ChatPaginator<'mc>, Box<dyn std::error::Error>> {
        let cls = &jni.find_class("org/bukkit/util/ChatPaginator")?;
        let res = jni.new_object(cls, "()V", &[])?;
        crate::util::ChatPaginator::from_raw(&jni, res)
    }
    /// Breaks a raw string up into pages using the default width and height.
    /// Breaks a raw string up into pages using a provided width and height.
    pub fn paginate_with_string(
        jni: blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<&'mc String>,
        arg1: std::option::Option<i32>,
        arg2: std::option::Option<i32>,
        arg3: std::option::Option<i32>,
    ) -> Result<crate::util::ChatPaginatorChatPage<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(jni.new_string(arg0.into()).unwrap());
        let val_2 = jni::objects::JValueGen::Int(arg1.unwrap().into());
        let val_3 = jni::objects::JValueGen::Int(arg2.unwrap().into());
        let val_4 = jni::objects::JValueGen::Int(arg3.unwrap().into());
        let cls = &jni.find_class("org/bukkit/util/ChatPaginator$ChatPage")?;
        let res = jni.call_static_method(
            cls,
            "paginate",
            "(Ljava/lang/String;III)Lorg/bukkit/util/ChatPaginator$ChatPage;",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
                jni::objects::JValueGen::from(&val_3),
                jni::objects::JValueGen::from(&val_4),
            ],
        )?;
        let obj = res.l()?;
        crate::util::ChatPaginatorChatPage::from_raw(&jni, obj)
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
/// Represents a mutable vector. Because the components of Vectors are mutable, storing Vectors long term may be dangerous if passing code modifies the Vector later. If you want to keep around a Vector, it may be wise to call <code>clone()</code> in order to get a copy.
pub struct Vector<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for Vector<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Vector<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Vector from null object.").into());
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
    pub fn new(
        jni: blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: std::option::Option<f64>,
        arg1: std::option::Option<f64>,
        arg2: std::option::Option<f64>,
    ) -> Result<crate::util::Vector<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Double(arg0.unwrap().into());
        let val_2 = jni::objects::JValueGen::Double(arg1.unwrap().into());
        let val_3 = jni::objects::JValueGen::Double(arg2.unwrap().into());
        let cls = &jni.find_class("org/bukkit/util/Vector")?;
        let res = jni.new_object(
            cls,
            "(DDD)V",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
                jni::objects::JValueGen::from(&val_3),
            ],
        )?;
        crate::util::Vector::from_raw(&jni, res)
    }
    pub fn new_with_int(
        jni: blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: i32,
        arg1: i32,
        arg2: std::option::Option<i32>,
    ) -> Result<crate::util::Vector<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let val_2 = jni::objects::JValueGen::Int(arg1.into());
        let val_3 = jni::objects::JValueGen::Int(arg2.unwrap().into());
        let cls = &jni.find_class("org/bukkit/util/Vector")?;
        let res = jni.new_object(
            cls,
            "(III)V",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
                jni::objects::JValueGen::from(&val_3),
            ],
        )?;
        crate::util::Vector::from_raw(&jni, res)
    }
    /// Gets the minimum components of two vectors.
    pub fn get_minimum(
        jni: blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<&'mc crate::util::Vector<'mc>>,
        arg1: impl Into<&'mc crate::util::Vector<'mc>>,
    ) -> Result<crate::util::Vector<'mc>, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let val_2 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone()) };
        let cls = &jni.find_class("org/bukkit/util/Vector")?;
        let res = jni.call_static_method(
            cls,
            "getMinimum",
            "(Lorg/bukkit/util/Vector;Lorg/bukkit/util/Vector;)Lorg/bukkit/util/Vector;",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        )?;
        let obj = res.l()?;
        crate::util::Vector::from_raw(&jni, obj)
    }
    /// Gets the maximum components of two vectors.
    pub fn get_maximum(
        jni: blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<&'mc crate::util::Vector<'mc>>,
        arg1: impl Into<&'mc crate::util::Vector<'mc>>,
    ) -> Result<crate::util::Vector<'mc>, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let val_2 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone()) };
        let cls = &jni.find_class("org/bukkit/util/Vector")?;
        let res = jni.call_static_method(
            cls,
            "getMaximum",
            "(Lorg/bukkit/util/Vector;Lorg/bukkit/util/Vector;)Lorg/bukkit/util/Vector;",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        )?;
        let obj = res.l()?;
        crate::util::Vector::from_raw(&jni, obj)
    }
    /// Returns if a vector is normalized
    pub fn is_normalized(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isNormalized", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    /// Subtracts a vector from this one.
    pub fn subtract(
        &mut self,
        arg0: impl Into<&'mc crate::util::Vector<'mc>>,
    ) -> Result<crate::util::Vector<'mc>, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "subtract",
            "(Lorg/bukkit/util/Vector;)Lorg/bukkit/util/Vector;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::util::Vector::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Divides the vector by another.
    pub fn divide(
        &mut self,
        arg0: impl Into<&'mc crate::util::Vector<'mc>>,
    ) -> Result<crate::util::Vector<'mc>, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "divide",
            "(Lorg/bukkit/util/Vector;)Lorg/bukkit/util/Vector;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::util::Vector::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Check whether or not each component of this vector is equal to 0.
    pub fn is_zero(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isZero", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    /// Multiplies the vector by another.
    /// Performs scalar multiplication, multiplying all components with a scalar.
    /// Performs scalar multiplication, multiplying all components with a scalar.
    /// Performs scalar multiplication, multiplying all components with a scalar.
    pub fn multiply_with_int(
        &mut self,
        arg0: std::option::Option<f32>,
    ) -> Result<crate::util::Vector<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Float(arg0.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "multiply",
            "(F)Lorg/bukkit/util/Vector;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::util::Vector::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Multiplies the vector by another.
    /// Performs scalar multiplication, multiplying all components with a scalar.
    /// Performs scalar multiplication, multiplying all components with a scalar.
    /// Performs scalar multiplication, multiplying all components with a scalar.
    pub fn multiply_with_double(
        &mut self,
        arg0: std::option::Option<impl Into<&'mc crate::util::Vector<'mc>>>,
    ) -> Result<crate::util::Vector<'mc>, Box<dyn std::error::Error>> {
        let val_1 =
            unsafe { jni::objects::JObject::from_raw(arg0.unwrap().into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "multiply",
            "(Lorg/bukkit/util/Vector;)Lorg/bukkit/util/Vector;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::util::Vector::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Check if each component of this Vector is finite.
    pub fn check_finite(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "checkFinite", "()V", &[]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// <span class="descfrm-type-label">Description copied from interface:&nbsp;<code><a href="../configuration/serialization/ConfigurationSerializable.html#serialize()">ConfigurationSerializable</a></code></span>
    /// Creates a Map representation of this class.
    /// <p>This class must provide a method to restore this class, as defined in the <a title="interface in org.bukkit.configuration.serialization" href="../configuration/serialization/ConfigurationSerializable.html"><code>ConfigurationSerializable</code></a> interface javadocs.</p>
    pub fn serialize(
        &mut self,
    ) -> Result<blackboxmc_java::JavaMap<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "serialize", "()Ljava/util/Map;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::JavaMap::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn deserialize(
        jni: blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<&'mc blackboxmc_java::JavaMap<'mc>>,
    ) -> Result<crate::util::Vector<'mc>, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let cls = &jni.find_class("org/bukkit/util/Vector")?;
        let res = jni.call_static_method(
            cls,
            "deserialize",
            "(Ljava/util/Map;)Lorg/bukkit/util/Vector;",
            &[jni::objects::JValueGen::from(&val_1)],
        )?;
        let obj = res.l()?;
        crate::util::Vector::from_raw(&jni, obj)
    }
    /// Gets the X component.
    pub fn x(&mut self) -> Result<f64, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getX", "()D", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.d().unwrap())
    }
    /// Gets the Y component.
    pub fn y(&mut self) -> Result<f64, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getY", "()D", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.d().unwrap())
    }
    /// Gets the Z component.
    pub fn z(&mut self) -> Result<f64, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getZ", "()D", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.d().unwrap())
    }
    /// Set the X component.
    /// Set the X component.
    /// Set the X component.
    pub fn set_x_with_float(
        &mut self,
        arg0: std::option::Option<f64>,
    ) -> Result<crate::util::Vector<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Double(arg0.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setX",
            "(D)Lorg/bukkit/util/Vector;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::util::Vector::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Set the X component.
    /// Set the X component.
    /// Set the X component.
    pub fn set_x_with_int(
        &mut self,
        arg0: std::option::Option<i32>,
    ) -> Result<crate::util::Vector<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setX",
            "(I)Lorg/bukkit/util/Vector;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::util::Vector::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Gets the floored value of the X component, indicating the block that this vector is contained with.
    pub fn block_x(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getBlockX", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }
    /// Set the Y component.
    /// Set the Y component.
    /// Set the Y component.
    pub fn set_y_with_float(
        &mut self,
        arg0: std::option::Option<f64>,
    ) -> Result<crate::util::Vector<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Double(arg0.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setY",
            "(D)Lorg/bukkit/util/Vector;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::util::Vector::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Set the Y component.
    /// Set the Y component.
    /// Set the Y component.
    pub fn set_y_with_int(
        &mut self,
        arg0: std::option::Option<i32>,
    ) -> Result<crate::util::Vector<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setY",
            "(I)Lorg/bukkit/util/Vector;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::util::Vector::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Gets the floored value of the Y component, indicating the block that this vector is contained with.
    pub fn block_y(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getBlockY", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }
    /// Set the Z component.
    /// Set the Z component.
    /// Set the Z component.
    pub fn set_z_with_float(
        &mut self,
        arg0: std::option::Option<f64>,
    ) -> Result<crate::util::Vector<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Double(arg0.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setZ",
            "(D)Lorg/bukkit/util/Vector;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::util::Vector::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Set the Z component.
    /// Set the Z component.
    /// Set the Z component.
    pub fn set_z_with_int(
        &mut self,
        arg0: std::option::Option<i32>,
    ) -> Result<crate::util::Vector<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setZ",
            "(I)Lorg/bukkit/util/Vector;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::util::Vector::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Gets the floored value of the Z component, indicating the block that this vector is contained with.
    pub fn block_z(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getBlockZ", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }
    /// Gets the magnitude of the vector squared.
    pub fn length_squared(&mut self) -> Result<f64, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "lengthSquared", "()D", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.d().unwrap())
    }
    /// Get the squared distance between this vector and another.
    pub fn distance_squared(
        &mut self,
        arg0: impl Into<&'mc crate::util::Vector<'mc>>,
    ) -> Result<f64, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "distanceSquared",
            "(Lorg/bukkit/util/Vector;)D",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.d().unwrap())
    }
    /// Gets the angle between this vector and another in radians.
    pub fn angle(
        &mut self,
        arg0: impl Into<&'mc crate::util::Vector<'mc>>,
    ) -> Result<f32, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "angle",
            "(Lorg/bukkit/util/Vector;)F",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.f().unwrap())
    }
    /// Sets this vector to the midpoint between this vector and another.
    pub fn midpoint(
        &mut self,
        arg0: impl Into<&'mc crate::util::Vector<'mc>>,
    ) -> Result<crate::util::Vector<'mc>, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "midpoint",
            "(Lorg/bukkit/util/Vector;)Lorg/bukkit/util/Vector;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::util::Vector::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Gets a new midpoint vector between this vector and another.
    pub fn get_midpoint(
        &mut self,
        arg0: impl Into<&'mc crate::util::Vector<'mc>>,
    ) -> Result<crate::util::Vector<'mc>, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getMidpoint",
            "(Lorg/bukkit/util/Vector;)Lorg/bukkit/util/Vector;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::util::Vector::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Calculates the cross product of this vector with another. The cross product is defined as:
    /// <ul>
    /// <li>x = y1 * z2 - y2 * z1</li>
    /// <li>y = z1 * x2 - z2 * x1</li>
    /// <li>z = x1 * y2 - x2 * y1</li>
    /// </ul>
    pub fn cross_product(
        &mut self,
        arg0: impl Into<&'mc crate::util::Vector<'mc>>,
    ) -> Result<crate::util::Vector<'mc>, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "crossProduct",
            "(Lorg/bukkit/util/Vector;)Lorg/bukkit/util/Vector;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::util::Vector::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Calculates the cross product of this vector with another without mutating the original. The cross product is defined as:
    /// <ul>
    /// <li>x = y1 * z2 - y2 * z1</li>
    /// <li>y = z1 * x2 - z2 * x1</li>
    /// <li>z = x1 * y2 - x2 * y1</li>
    /// </ul>
    pub fn get_cross_product(
        &mut self,
        arg0: impl Into<&'mc crate::util::Vector<'mc>>,
    ) -> Result<crate::util::Vector<'mc>, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getCrossProduct",
            "(Lorg/bukkit/util/Vector;)Lorg/bukkit/util/Vector;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::util::Vector::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Returns whether this vector is in an axis-aligned bounding box.
    /// <p>The minimum and maximum vectors given must be truly the minimum and maximum X, Y and Z components.</p>
    pub fn is_in_aabb(
        &mut self,
        arg0: impl Into<&'mc crate::util::Vector<'mc>>,
        arg1: impl Into<&'mc crate::util::Vector<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let val_2 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "isInAABB",
            "(Lorg/bukkit/util/Vector;Lorg/bukkit/util/Vector;)Z",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    /// Returns whether this vector is within a sphere.
    pub fn is_in_sphere(
        &mut self,
        arg0: impl Into<&'mc crate::util::Vector<'mc>>,
        arg1: f64,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let val_2 = jni::objects::JValueGen::Double(arg1.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "isInSphere",
            "(Lorg/bukkit/util/Vector;D)Z",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    /// Get the threshold used for equals().
    pub fn epsilon(
        jni: blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<f64, Box<dyn std::error::Error>> {
        let cls = &jni.find_class("double")?;
        let res = jni.call_static_method(cls, "getEpsilon", "()D", &[])?;
        Ok(res.d().unwrap())
    }
    /// Rotates the vector around the x axis.
    /// <p>This piece of math is based on the standard rotation matrix for vectors in three dimensional space. This matrix can be found here: <a href="https://en.wikipedia.org/wiki/Rotation_matrix#Basic_rotations">Rotation Matrix</a>.</p>
    pub fn rotate_around_x(
        &mut self,
        arg0: f64,
    ) -> Result<crate::util::Vector<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Double(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "rotateAroundX",
            "(D)Lorg/bukkit/util/Vector;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::util::Vector::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Rotates the vector around the y axis.
    /// <p>This piece of math is based on the standard rotation matrix for vectors in three dimensional space. This matrix can be found here: <a href="https://en.wikipedia.org/wiki/Rotation_matrix#Basic_rotations">Rotation Matrix</a>.</p>
    pub fn rotate_around_y(
        &mut self,
        arg0: f64,
    ) -> Result<crate::util::Vector<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Double(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "rotateAroundY",
            "(D)Lorg/bukkit/util/Vector;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::util::Vector::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Rotates the vector around the z axis
    /// <p>This piece of math is based on the standard rotation matrix for vectors in three dimensional space. This matrix can be found here: <a href="https://en.wikipedia.org/wiki/Rotation_matrix#Basic_rotations">Rotation Matrix</a>.</p>
    pub fn rotate_around_z(
        &mut self,
        arg0: f64,
    ) -> Result<crate::util::Vector<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Double(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "rotateAroundZ",
            "(D)Lorg/bukkit/util/Vector;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::util::Vector::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Rotates the vector around a given arbitrary axis in 3 dimensional space.
    /// <p>Rotation will follow the general Right-Hand-Rule, which means rotation will be counterclockwise when the axis is pointing towards the observer.</p>
    /// <p>This method will always make sure the provided axis is a unit vector, to not modify the length of the vector when rotating. If you are experienced with the scaling of a non-unit axis vector, you can use <a href="#rotateAroundNonUnitAxis(org.bukkit.util.Vector,double)"><code>rotateAroundNonUnitAxis(Vector, double)</code></a>.</p>
    pub fn rotate_around_axis(
        &mut self,
        arg0: impl Into<&'mc crate::util::Vector<'mc>>,
        arg1: f64,
    ) -> Result<crate::util::Vector<'mc>, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let val_2 = jni::objects::JValueGen::Double(arg1.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "rotateAroundAxis",
            "(Lorg/bukkit/util/Vector;D)Lorg/bukkit/util/Vector;",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::util::Vector::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Rotates the vector around a given arbitrary axis in 3 dimensional space.
    /// <p>Rotation will follow the general Right-Hand-Rule, which means rotation will be counterclockwise when the axis is pointing towards the observer.</p>
    /// <p>Note that the vector length will change accordingly to the axis vector length. If the provided axis is not a unit vector, the rotated vector will not have its previous length. The scaled length of the resulting vector will be related to the axis vector. If you are not perfectly sure about the scaling of the vector, use <a href="#rotateAroundAxis(org.bukkit.util.Vector,double)"><code>rotateAroundAxis(Vector, double)</code></a></p>
    pub fn rotate_around_non_unit_axis(
        &mut self,
        arg0: impl Into<&'mc crate::util::Vector<'mc>>,
        arg1: f64,
    ) -> Result<crate::util::Vector<'mc>, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let val_2 = jni::objects::JValueGen::Double(arg1.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "rotateAroundNonUnitAxis",
            "(Lorg/bukkit/util/Vector;D)Lorg/bukkit/util/Vector;",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::util::Vector::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Gets a Location version of this vector with yaw and pitch being 0.
    /// Gets a Location version of this vector.
    pub fn to_location_with_world(
        &mut self,
        arg0: std::option::Option<impl Into<&'mc crate::World<'mc>>>,
        arg1: std::option::Option<f32>,
        arg2: std::option::Option<f32>,
    ) -> Result<crate::Location<'mc>, Box<dyn std::error::Error>> {
        let val_1 =
            unsafe { jni::objects::JObject::from_raw(arg0.unwrap().into().jni_object().clone()) };
        let val_2 = jni::objects::JValueGen::Float(arg1.unwrap().into());
        let val_3 = jni::objects::JValueGen::Float(arg2.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "toLocation",
            "(Lorg/bukkit/World;FF)Lorg/bukkit/Location;",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
                jni::objects::JValueGen::from(&val_3),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::Location::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Get the block vector of this vector.
    pub fn to_block_vector(
        &mut self,
    ) -> Result<crate::util::BlockVector<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "toBlockVector",
            "()Lorg/bukkit/util/BlockVector;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::util::BlockVector::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Get this vector as a JOML <code>Vector3f</code>.
    pub fn to_vector3f(
        &mut self,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "toVector3f",
            "()Lorg/joml/Vector3f;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
    /// Get this vector as a JOML <code>Vector3d</code>.
    pub fn to_vector3d(
        &mut self,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "toVector3d",
            "()Lorg/joml/Vector3d;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
    /// Get this vector as a JOML <code>Vector3i</code>.
    /// Get this vector as a JOML <code>Vector3i</code> with its components floored.
    pub fn to_vector3i(
        &mut self,
        arg0: std::option::Option<i32>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "toVector3i",
            "(I)Lorg/joml/Vector3i;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
    /// Gets a random vector with components having a random value between 0 and 1.
    pub fn random(
        jni: blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<crate::util::Vector<'mc>, Box<dyn std::error::Error>> {
        let cls = &jni.find_class("org/bukkit/util/Vector")?;
        let res = jni.call_static_method(cls, "getRandom", "()Lorg/bukkit/util/Vector;", &[])?;
        let obj = res.l()?;
        crate::util::Vector::from_raw(&jni, obj)
    }
    /// Gets a vector with components that match the provided JOML <code>Vector3f</code>.
    /// Gets a vector with components that match the provided JOML <code>Vector3d</code>.
    /// Gets a vector with components that match the provided JOML <code>Vector3i</code>.
    pub unsafe fn from_joml_with_vector3f(
        jni: blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: std::option::Option<jni::objects::JObject<'mc>>,
    ) -> Result<crate::util::Vector<'mc>, Box<dyn std::error::Error>> {
        let val_1 = arg0.unwrap();
        let cls = &jni.find_class("org/bukkit/util/Vector")?;
        let res = jni.call_static_method(
            cls,
            "fromJOML",
            "(Lorg/joml/Vector3i;)Lorg/bukkit/util/Vector;",
            &[jni::objects::JValueGen::from(&val_1)],
        )?;
        let obj = res.l()?;
        crate::util::Vector::from_raw(&jni, obj)
    }
    /// Gets a vector with components that match the provided JOML <code>Vector3f</code>.
    /// Gets a vector with components that match the provided JOML <code>Vector3d</code>.
    /// Gets a vector with components that match the provided JOML <code>Vector3i</code>.
    pub unsafe fn from_joml_with_vector3d(
        jni: blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: std::option::Option<jni::objects::JObject<'mc>>,
    ) -> Result<crate::util::Vector<'mc>, Box<dyn std::error::Error>> {
        let val_1 = arg0.unwrap();
        let cls = &jni.find_class("org/bukkit/util/Vector")?;
        let res = jni.call_static_method(
            cls,
            "fromJOML",
            "(Lorg/joml/Vector3d;)Lorg/bukkit/util/Vector;",
            &[jni::objects::JValueGen::from(&val_1)],
        )?;
        let obj = res.l()?;
        crate::util::Vector::from_raw(&jni, obj)
    }
    /// Adds a vector to this one
    pub fn add(
        &mut self,
        arg0: impl Into<&'mc crate::util::Vector<'mc>>,
    ) -> Result<crate::util::Vector<'mc>, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "add",
            "(Lorg/bukkit/util/Vector;)Lorg/bukkit/util/Vector;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::util::Vector::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Checks to see if two objects are equal.
    /// <p>Only two Vectors can ever return true. This method uses a fuzzy match to account for floating point errors. The epsilon can be retrieved with epsilon.</p>
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
    /// Gets the magnitude of the vector, defined as sqrt(x^2+y^2+z^2). The value of this method is not cached and uses a costly square-root function, so do not repeatedly call this method to get the vector's magnitude. NaN will be returned if the inner result of the sqrt() function overflows, which will be caused if the length is too long.
    /// Gets the magnitude of the vector squared.
    pub fn length(&mut self) -> Result<f64, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "length", "()D", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.d().unwrap())
    }
    /// Returns this vector's components as x,y,z.
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
    /// Returns a hash code for this vector
    pub fn hash_code(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hashCode", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }
    /// Get a new vector.
    pub fn clone(&mut self) -> Result<crate::util::Vector<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "clone",
            "()Lorg/bukkit/util/Vector;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::util::Vector::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Calculates the dot product of this vector with another. The dot product is defined as x1*x2+y1*y2+z1*z2. The returned value is a scalar.
    pub fn dot(
        &mut self,
        arg0: impl Into<&'mc crate::util::Vector<'mc>>,
    ) -> Result<f64, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "dot",
            "(Lorg/bukkit/util/Vector;)D",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.d().unwrap())
    }
    /// Copies another vector
    pub fn copy(
        &mut self,
        arg0: impl Into<&'mc crate::util::Vector<'mc>>,
    ) -> Result<crate::util::Vector<'mc>, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "copy",
            "(Lorg/bukkit/util/Vector;)Lorg/bukkit/util/Vector;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::util::Vector::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Converts this vector to a unit vector (a vector with length of 1).
    pub fn normalize(&mut self) -> Result<crate::util::Vector<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "normalize",
            "()Lorg/bukkit/util/Vector;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::util::Vector::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Zero this vector's components.
    pub fn zero(&mut self) -> Result<crate::util::Vector<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "zero",
            "()Lorg/bukkit/util/Vector;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::util::Vector::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Get the distance between this vector and another. The value of this method is not cached and uses a costly square-root function, so do not repeatedly call this method to get the vector's magnitude. NaN will be returned if the inner result of the sqrt() function overflows, which will be caused if the distance is too long.
    /// Get the squared distance between this vector and another.
    pub fn distance(
        &mut self,
        arg0: impl Into<&'mc crate::util::Vector<'mc>>,
    ) -> Result<f64, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "distance",
            "(Lorg/bukkit/util/Vector;)D",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.d().unwrap())
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
impl<'mc> Into<crate::configuration::serialization::ConfigurationSerializable<'mc>>
    for Vector<'mc>
{
    fn into(self) -> crate::configuration::serialization::ConfigurationSerializable<'mc> {
        crate::configuration::serialization::ConfigurationSerializable::from_raw(
            &self.jni_ref(),
            self.1,
        )
        .unwrap()
    }
}
/// Class containing file utilities
pub struct FileUtil<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for FileUtil<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> FileUtil<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate FileUtil from null object.").into());
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
    pub fn new(
        jni: blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<crate::util::FileUtil<'mc>, Box<dyn std::error::Error>> {
        let cls = &jni.find_class("org/bukkit/util/FileUtil")?;
        let res = jni.new_object(cls, "()V", &[])?;
        crate::util::FileUtil::from_raw(&jni, res)
    }
    /// This method copies one file to another location
    pub unsafe fn copy(
        jni: blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: jni::objects::JObject<'mc>,
        arg1: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let val_2 = arg1;
        let cls = &jni.find_class("boolean")?;
        let res = jni.call_static_method(
            cls,
            "copy",
            "(Ljava/io/File;Ljava/io/File;)Z",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        )?;
        Ok(res.z().unwrap())
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
pub mod noise;
pub mod permissions;
