use crate::JNIRaw;
pub struct NumberConversions<'mc>(
    pub(crate) jni::JNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> crate::JNIRaw<'mc> for NumberConversions<'mc> {
    fn jni_ref(&self) -> jni::JNIEnv<'mc> {
        unsafe { self.0.unsafe_clone() }
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> NumberConversions<'mc> {
    pub fn from_raw(env: jni::JNIEnv<'mc>, obj: jni::objects::JObject<'mc>) -> Self {
        Self(env, obj)
    }
    pub fn round(mut jni: jni::JNIEnv<'mc>, arg0: f64) -> Result<i32, Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Double(arg0.into());
        let cls = &jni.find_class("int")?;
        let res = jni.call_static_method(
            cls,
            "round",
            "(D)I",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(res.i().unwrap())
    }
    pub fn to_int(
        mut jni: jni::JNIEnv<'mc>,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        let val_0 = arg0;
        let cls = &jni.find_class("int")?;
        let res = jni.call_static_method(
            cls,
            "toInt",
            "(Ljava/lang/Object;)I",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(res.i().unwrap())
    }
    pub fn square(mut jni: jni::JNIEnv<'mc>, arg0: f64) -> Result<f64, Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Double(arg0.into());
        let cls = &jni.find_class("double")?;
        let res = jni.call_static_method(
            cls,
            "square",
            "(D)D",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(res.d().unwrap())
    }
    pub fn to_double(
        mut jni: jni::JNIEnv<'mc>,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<f64, Box<dyn std::error::Error>> {
        let val_0 = arg0;
        let cls = &jni.find_class("double")?;
        let res = jni.call_static_method(
            cls,
            "toDouble",
            "(Ljava/lang/Object;)D",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(res.d().unwrap())
    }
    pub fn to_float(
        mut jni: jni::JNIEnv<'mc>,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<f32, Box<dyn std::error::Error>> {
        let val_0 = arg0;
        let cls = &jni.find_class("float")?;
        let res = jni.call_static_method(
            cls,
            "toFloat",
            "(Ljava/lang/Object;)F",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(res.f().unwrap())
    }
    pub fn to_long(
        mut jni: jni::JNIEnv<'mc>,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<i64, Box<dyn std::error::Error>> {
        let val_0 = arg0;
        let cls = &jni.find_class("long")?;
        let res = jni.call_static_method(
            cls,
            "toLong",
            "(Ljava/lang/Object;)J",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(res.j().unwrap())
    }
    pub fn to_short(
        mut jni: jni::JNIEnv<'mc>,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<i16, Box<dyn std::error::Error>> {
        let val_0 = arg0;
        let cls = &jni.find_class("short")?;
        let res = jni.call_static_method(
            cls,
            "toShort",
            "(Ljava/lang/Object;)S",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(res.s().unwrap())
    }
    pub fn to_byte(
        mut jni: jni::JNIEnv<'mc>,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<i8, Box<dyn std::error::Error>> {
        let val_0 = arg0;
        let cls = &jni.find_class("byte")?;
        let res = jni.call_static_method(
            cls,
            "toByte",
            "(Ljava/lang/Object;)B",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(res.b().unwrap())
    }
    pub fn floor(mut jni: jni::JNIEnv<'mc>, arg0: f64) -> Result<i32, Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Double(arg0.into());
        let cls = &jni.find_class("int")?;
        let res = jni.call_static_method(
            cls,
            "floor",
            "(D)I",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(res.i().unwrap())
    }
    pub fn ceil(mut jni: jni::JNIEnv<'mc>, arg0: f64) -> Result<i32, Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Double(arg0.into());
        let cls = &jni.find_class("int")?;
        let res = jni.call_static_method(
            cls,
            "ceil",
            "(D)I",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(res.i().unwrap())
    }
    pub fn wait(
        &mut self,
        arg0: std::option::Option<i64>,
        arg1: std::option::Option<i32>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Long(arg0.unwrap().into());
        let val_1 = jni::objects::JValueGen::Int(arg1.unwrap().into());
        self.jni_ref().call_method(
            &self.jni_object(),
            "wait",
            "(JI)V",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        )?;
        Ok(())
    }
    pub fn equals(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "equals",
            "(Ljava/lang/Object;)Z",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(res.z().unwrap())
    }
    pub fn to_string(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "toString",
            "()Ljava/lang/String;",
            &[],
        )?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    pub fn hash_code(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hashCode", "()I", &[])?;
        Ok(res.i().unwrap())
    }
    pub fn class(&mut self) -> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getClass",
            "()Ljava/lang/Class;",
            &[],
        )?;
        Ok(unsafe { jni::objects::JClass::from_raw(res.as_jni().l) })
    }
    pub fn notify(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.jni_ref()
            .call_method(&self.jni_object(), "notify", "()V", &[])?;
        Ok(())
    }
    pub fn notify_all(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.jni_ref()
            .call_method(&self.jni_object(), "notifyAll", "()V", &[])?;
        Ok(())
    }
}
pub struct BlockIterator<'mc>(
    pub(crate) jni::JNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> crate::JNIRaw<'mc> for BlockIterator<'mc> {
    fn jni_ref(&self) -> jni::JNIEnv<'mc> {
        unsafe { self.0.unsafe_clone() }
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> BlockIterator<'mc> {
    pub fn from_raw(env: jni::JNIEnv<'mc>, obj: jni::objects::JObject<'mc>) -> Self {
        Self(env, obj)
    }
    pub fn remove(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.jni_ref()
            .call_method(&self.jni_object(), "remove", "()V", &[])?;
        Ok(())
    }
    pub fn has_next(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasNext", "()Z", &[])?;
        Ok(res.z().unwrap())
    }
    pub fn next(&mut self) -> Result<crate::bukkit::block::Block<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "next",
            "()Lorg/bukkit/block/Block;",
            &[],
        )?;
        let ret = {
            crate::bukkit::block::Block(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn wait(
        &mut self,
        arg0: std::option::Option<i64>,
        arg1: std::option::Option<i32>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Long(arg0.unwrap().into());
        let val_1 = jni::objects::JValueGen::Int(arg1.unwrap().into());
        self.jni_ref().call_method(
            &self.jni_object(),
            "wait",
            "(JI)V",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        )?;
        Ok(())
    }
    pub fn equals(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "equals",
            "(Ljava/lang/Object;)Z",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(res.z().unwrap())
    }
    pub fn to_string(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "toString",
            "()Ljava/lang/String;",
            &[],
        )?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    pub fn hash_code(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hashCode", "()I", &[])?;
        Ok(res.i().unwrap())
    }
    pub fn class(&mut self) -> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getClass",
            "()Ljava/lang/Class;",
            &[],
        )?;
        Ok(unsafe { jni::objects::JClass::from_raw(res.as_jni().l) })
    }
    pub fn notify(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.jni_ref()
            .call_method(&self.jni_object(), "notify", "()V", &[])?;
        Ok(())
    }
    pub fn notify_all(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.jni_ref()
            .call_method(&self.jni_object(), "notifyAll", "()V", &[])?;
        Ok(())
    }
}
pub struct Transformation<'mc>(
    pub(crate) jni::JNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> crate::JNIRaw<'mc> for Transformation<'mc> {
    fn jni_ref(&self) -> jni::JNIEnv<'mc> {
        unsafe { self.0.unsafe_clone() }
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Transformation<'mc> {
    pub fn from_raw(env: jni::JNIEnv<'mc>, obj: jni::objects::JObject<'mc>) -> Self {
        Self(env, obj)
    }
    pub fn equals(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "equals",
            "(Ljava/lang/Object;)Z",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(res.z().unwrap())
    }
    pub fn to_string(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "toString",
            "()Ljava/lang/String;",
            &[],
        )?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    pub fn hash_code(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hashCode", "()I", &[])?;
        Ok(res.i().unwrap())
    }
    pub fn wait(
        &mut self,
        arg0: std::option::Option<i64>,
        arg1: std::option::Option<i32>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Long(arg0.unwrap().into());
        let val_1 = jni::objects::JValueGen::Int(arg1.unwrap().into());
        self.jni_ref().call_method(
            &self.jni_object(),
            "wait",
            "(JI)V",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        )?;
        Ok(())
    }
    pub fn class(&mut self) -> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getClass",
            "()Ljava/lang/Class;",
            &[],
        )?;
        Ok(unsafe { jni::objects::JClass::from_raw(res.as_jni().l) })
    }
    pub fn notify(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.jni_ref()
            .call_method(&self.jni_object(), "notify", "()V", &[])?;
        Ok(())
    }
    pub fn notify_all(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.jni_ref()
            .call_method(&self.jni_object(), "notifyAll", "()V", &[])?;
        Ok(())
    }
}
pub struct RayTraceResult<'mc>(
    pub(crate) jni::JNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> crate::JNIRaw<'mc> for RayTraceResult<'mc> {
    fn jni_ref(&self) -> jni::JNIEnv<'mc> {
        unsafe { self.0.unsafe_clone() }
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> RayTraceResult<'mc> {
    pub fn from_raw(env: jni::JNIEnv<'mc>, obj: jni::objects::JObject<'mc>) -> Self {
        Self(env, obj)
    }
    pub fn hit_position(
        &mut self,
    ) -> Result<crate::bukkit::util::Vector<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getHitPosition",
            "()Lorg/bukkit/util/Vector;",
            &[],
        )?;
        let ret = {
            crate::bukkit::util::Vector(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn hit_block(
        &mut self,
    ) -> Result<crate::bukkit::block::Block<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getHitBlock",
            "()Lorg/bukkit/block/Block;",
            &[],
        )?;
        let ret = {
            crate::bukkit::block::Block(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn hit_block_face(
        &mut self,
    ) -> Result<crate::bukkit::block::BlockFace<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getHitBlockFace",
            "()Lorg/bukkit/block/BlockFace;",
            &[],
        )?;
        let ret = {
            let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
            let variant = self
                .0
                .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
            let variant_str = self
                .0
                .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
                .to_string_lossy()
                .to_string();
            crate::bukkit::block::BlockFace(
                self.jni_ref(),
                raw_obj,
                crate::bukkit::block::BlockFace::from_string(variant_str).unwrap(),
            )
        };
        Ok(ret)
    }
    pub fn hit_entity(
        &mut self,
    ) -> Result<crate::bukkit::entity::Entity<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getHitEntity",
            "()Lorg/bukkit/entity/Entity;",
            &[],
        )?;
        let ret = {
            crate::bukkit::entity::Entity(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn equals(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "equals",
            "(Ljava/lang/Object;)Z",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(res.z().unwrap())
    }
    pub fn to_string(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "toString",
            "()Ljava/lang/String;",
            &[],
        )?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    pub fn hash_code(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hashCode", "()I", &[])?;
        Ok(res.i().unwrap())
    }
    pub fn wait(
        &mut self,
        arg0: std::option::Option<i64>,
        arg1: std::option::Option<i32>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Long(arg0.unwrap().into());
        let val_1 = jni::objects::JValueGen::Int(arg1.unwrap().into());
        self.jni_ref().call_method(
            &self.jni_object(),
            "wait",
            "(JI)V",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        )?;
        Ok(())
    }
    pub fn class(&mut self) -> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getClass",
            "()Ljava/lang/Class;",
            &[],
        )?;
        Ok(unsafe { jni::objects::JClass::from_raw(res.as_jni().l) })
    }
    pub fn notify(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.jni_ref()
            .call_method(&self.jni_object(), "notify", "()V", &[])?;
        Ok(())
    }
    pub fn notify_all(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.jni_ref()
            .call_method(&self.jni_object(), "notifyAll", "()V", &[])?;
        Ok(())
    }
}
/// An instantiatable struct that implements CachedServerIcon. Needed for returning it from Java.
pub struct CachedServerIcon<'mc>(
    pub(crate) jni::JNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> CachedServerIcon<'mc> {}
impl<'mc> crate::JNIRaw<'mc> for CachedServerIcon<'mc> {
    fn jni_ref(&self) -> jni::JNIEnv<'mc> {
        unsafe { self.0.unsafe_clone() }
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
pub struct ChatPaginatorChatPage<'mc>(
    pub(crate) jni::JNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> crate::JNIRaw<'mc> for ChatPaginatorChatPage<'mc> {
    fn jni_ref(&self) -> jni::JNIEnv<'mc> {
        unsafe { self.0.unsafe_clone() }
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> ChatPaginatorChatPage<'mc> {
    pub fn from_raw(env: jni::JNIEnv<'mc>, obj: jni::objects::JObject<'mc>) -> Self {
        Self(env, obj)
    }
    pub fn page_number(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getPageNumber", "()I", &[])?;
        Ok(res.i().unwrap())
    }
    pub fn total_pages(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getTotalPages", "()I", &[])?;
        Ok(res.i().unwrap())
    }
    pub fn wait(
        &mut self,
        arg0: std::option::Option<i64>,
        arg1: std::option::Option<i32>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Long(arg0.unwrap().into());
        let val_1 = jni::objects::JValueGen::Int(arg1.unwrap().into());
        self.jni_ref().call_method(
            &self.jni_object(),
            "wait",
            "(JI)V",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        )?;
        Ok(())
    }
    pub fn equals(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "equals",
            "(Ljava/lang/Object;)Z",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(res.z().unwrap())
    }
    pub fn to_string(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "toString",
            "()Ljava/lang/String;",
            &[],
        )?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    pub fn hash_code(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hashCode", "()I", &[])?;
        Ok(res.i().unwrap())
    }
    pub fn class(&mut self) -> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getClass",
            "()Ljava/lang/Class;",
            &[],
        )?;
        Ok(unsafe { jni::objects::JClass::from_raw(res.as_jni().l) })
    }
    pub fn notify(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.jni_ref()
            .call_method(&self.jni_object(), "notify", "()V", &[])?;
        Ok(())
    }
    pub fn notify_all(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.jni_ref()
            .call_method(&self.jni_object(), "notifyAll", "()V", &[])?;
        Ok(())
    }
}
/// An instantiatable struct that implements VoxelShape. Needed for returning it from Java.
pub struct VoxelShape<'mc>(
    pub(crate) jni::JNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> VoxelShape<'mc> {
    pub fn from_raw(env: jni::JNIEnv<'mc>, obj: jni::objects::JObject<'mc>) -> Self {
        Self(env, obj)
    }
    pub fn overlaps(
        &mut self,
        arg0: crate::bukkit::util::BoundingBox<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "overlaps",
            "(Lorg/bukkit/util/BoundingBox;)Z",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(res.z().unwrap())
    }
}
impl<'mc> crate::JNIRaw<'mc> for VoxelShape<'mc> {
    fn jni_ref(&self) -> jni::JNIEnv<'mc> {
        unsafe { self.0.unsafe_clone() }
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
/// An instantiatable struct that implements Consumer. Needed for returning it from Java.
pub struct Consumer<'mc>(
    pub(crate) jni::JNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> Consumer<'mc> {
    pub fn from_raw(env: jni::JNIEnv<'mc>, obj: jni::objects::JObject<'mc>) -> Self {
        Self(env, obj)
    }
    pub fn accept(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = arg0;
        self.jni_ref().call_method(
            &self.jni_object(),
            "accept",
            "(Ljava/lang/Object;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
}
impl<'mc> crate::JNIRaw<'mc> for Consumer<'mc> {
    fn jni_ref(&self) -> jni::JNIEnv<'mc> {
        unsafe { self.0.unsafe_clone() }
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
/// An instantiatable struct that implements StructureSearchResult. Needed for returning it from Java.
pub struct StructureSearchResult<'mc>(
    pub(crate) jni::JNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> StructureSearchResult<'mc> {
    pub fn from_raw(env: jni::JNIEnv<'mc>, obj: jni::objects::JObject<'mc>) -> Self {
        Self(env, obj)
    }
    pub fn structure(
        &mut self,
    ) -> Result<crate::bukkit::generator::structure::Structure<'mc>, Box<dyn std::error::Error>>
    {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getStructure",
            "()Lorg/bukkit/generator/structure/Structure;",
            &[],
        )?;
        let ret = {
            crate::bukkit::generator::structure::Structure(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn location(&mut self) -> Result<crate::bukkit::Location<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getLocation",
            "()Lorg/bukkit/Location;",
            &[],
        )?;
        let ret = {
            crate::bukkit::Location(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
}
impl<'mc> crate::JNIRaw<'mc> for StructureSearchResult<'mc> {
    fn jni_ref(&self) -> jni::JNIEnv<'mc> {
        unsafe { self.0.unsafe_clone() }
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
pub struct BoundingBox<'mc>(
    pub(crate) jni::JNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> crate::JNIRaw<'mc> for BoundingBox<'mc> {
    fn jni_ref(&self) -> jni::JNIEnv<'mc> {
        unsafe { self.0.unsafe_clone() }
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> BoundingBox<'mc> {
    pub fn from_raw(env: jni::JNIEnv<'mc>, obj: jni::objects::JObject<'mc>) -> Self {
        Self(env, obj)
    }
    pub fn height(&mut self) -> Result<f64, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getHeight", "()D", &[])?;
        Ok(res.d().unwrap())
    }
    pub fn deserialize(
        mut jni: jni::JNIEnv<'mc>,
        arg0: std::collections::HashMap<String, jni::objects::JObject<'mc>>,
    ) -> Result<crate::bukkit::util::BoundingBox<'mc>, Box<dyn std::error::Error>> {
        let raw_val_0 = jni.new_object("java/util/HashMap", "()V", &[]).unwrap();
        for (k, v) in arg0 {
            let map_val_0 = jni::objects::JObject::from(jni.new_string(k).unwrap());
            let map_val_1 = v;
            jni.call_method(
                &raw_val_0,
                "put",
                "(Ljava/Lang/ObjectLjava/Lang/Object)V",
                &[
                    jni::objects::JValueGen::from(&map_val_0),
                    jni::objects::JValueGen::from(&map_val_1),
                ],
            )?;
        }
        let val_0 = jni::objects::JValueGen::Object(raw_val_0);
        let cls = &jni.find_class("org/bukkit/util/BoundingBox")?;
        let res = jni.call_static_method(
            cls,
            "deserialize",
            "(Ljava/util/Map;)Lorg/bukkit/util/BoundingBox;",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        let ret = {
            let obj = res.l()?;
            crate::bukkit::util::BoundingBox(jni, obj)
        };
        Ok(ret)
    }
    pub fn min_x(&mut self) -> Result<f64, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getMinX", "()D", &[])?;
        Ok(res.d().unwrap())
    }
    pub fn min_y(&mut self) -> Result<f64, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getMinY", "()D", &[])?;
        Ok(res.d().unwrap())
    }
    pub fn min_z(&mut self) -> Result<f64, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getMinZ", "()D", &[])?;
        Ok(res.d().unwrap())
    }
    pub fn min(&mut self) -> Result<crate::bukkit::util::Vector<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getMin",
            "()Lorg/bukkit/util/Vector;",
            &[],
        )?;
        let ret = {
            crate::bukkit::util::Vector(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn max_x(&mut self) -> Result<f64, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getMaxX", "()D", &[])?;
        Ok(res.d().unwrap())
    }
    pub fn max_y(&mut self) -> Result<f64, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getMaxY", "()D", &[])?;
        Ok(res.d().unwrap())
    }
    pub fn max_z(&mut self) -> Result<f64, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getMaxZ", "()D", &[])?;
        Ok(res.d().unwrap())
    }
    pub fn max(&mut self) -> Result<crate::bukkit::util::Vector<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getMax",
            "()Lorg/bukkit/util/Vector;",
            &[],
        )?;
        let ret = {
            crate::bukkit::util::Vector(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn width_x(&mut self) -> Result<f64, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getWidthX", "()D", &[])?;
        Ok(res.d().unwrap())
    }
    pub fn width_z(&mut self) -> Result<f64, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getWidthZ", "()D", &[])?;
        Ok(res.d().unwrap())
    }
    pub fn volume(&mut self) -> Result<f64, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getVolume", "()D", &[])?;
        Ok(res.d().unwrap())
    }
    pub fn center_x(&mut self) -> Result<f64, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getCenterX", "()D", &[])?;
        Ok(res.d().unwrap())
    }
    pub fn center_y(&mut self) -> Result<f64, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getCenterY", "()D", &[])?;
        Ok(res.d().unwrap())
    }
    pub fn center_z(&mut self) -> Result<f64, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getCenterZ", "()D", &[])?;
        Ok(res.d().unwrap())
    }
    pub fn center(
        &mut self,
    ) -> Result<crate::bukkit::util::Vector<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getCenter",
            "()Lorg/bukkit/util/Vector;",
            &[],
        )?;
        let ret = {
            crate::bukkit::util::Vector(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn intersection(
        &mut self,
        arg0: crate::bukkit::util::BoundingBox<'mc>,
    ) -> Result<crate::bukkit::util::BoundingBox<'mc>, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "intersection",
            "(Lorg/bukkit/util/BoundingBox;)Lorg/bukkit/util/BoundingBox;",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        let ret = {
            crate::bukkit::util::BoundingBox(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn ray_trace(
        &mut self,
        arg0: crate::bukkit::util::Vector<'mc>,
        arg1: crate::bukkit::util::Vector<'mc>,
        arg2: f64,
    ) -> Result<crate::bukkit::util::RayTraceResult<'mc>, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.1.clone()) };
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg1.1.clone()) };
        let val_2 = jni::objects::JValueGen::Double(arg2.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "rayTrace",
            "(Lorg/bukkit/util/Vector;Lorg/bukkit/util/Vector;D)Lorg/bukkit/util/RayTraceResult;",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        )?;
        let ret = {
            crate::bukkit::util::RayTraceResult(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn equals(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "equals",
            "(Ljava/lang/Object;)Z",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(res.z().unwrap())
    }
    pub fn to_string(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "toString",
            "()Ljava/lang/String;",
            &[],
        )?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    pub fn hash_code(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hashCode", "()I", &[])?;
        Ok(res.i().unwrap())
    }
    pub fn clone(&mut self) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "clone", "()Ljava/lang/Object;", &[])?;
        Ok(res.l().unwrap())
    }
    pub fn copy(
        &mut self,
        arg0: crate::bukkit::util::BoundingBox<'mc>,
    ) -> Result<crate::bukkit::util::BoundingBox<'mc>, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "copy",
            "(Lorg/bukkit/util/BoundingBox;)Lorg/bukkit/util/BoundingBox;",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        let ret = {
            crate::bukkit::util::BoundingBox(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn resize(
        &mut self,
        arg0: f64,
        arg1: f64,
        arg2: f64,
        arg3: f64,
        arg4: f64,
        arg5: f64,
    ) -> Result<crate::bukkit::util::BoundingBox<'mc>, Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Double(arg0.into());
        let val_1 = jni::objects::JValueGen::Double(arg1.into());
        let val_2 = jni::objects::JValueGen::Double(arg2.into());
        let val_3 = jni::objects::JValueGen::Double(arg3.into());
        let val_4 = jni::objects::JValueGen::Double(arg4.into());
        let val_5 = jni::objects::JValueGen::Double(arg5.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "resize",
            "(DDDDDD)Lorg/bukkit/util/BoundingBox;",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
                jni::objects::JValueGen::from(&val_3),
                jni::objects::JValueGen::from(&val_4),
                jni::objects::JValueGen::from(&val_5),
            ],
        )?;
        let ret = {
            crate::bukkit::util::BoundingBox(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn wait(
        &mut self,
        arg0: std::option::Option<i64>,
        arg1: std::option::Option<i32>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Long(arg0.unwrap().into());
        let val_1 = jni::objects::JValueGen::Int(arg1.unwrap().into());
        self.jni_ref().call_method(
            &self.jni_object(),
            "wait",
            "(JI)V",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        )?;
        Ok(())
    }
    pub fn class(&mut self) -> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getClass",
            "()Ljava/lang/Class;",
            &[],
        )?;
        Ok(unsafe { jni::objects::JClass::from_raw(res.as_jni().l) })
    }
    pub fn notify(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.jni_ref()
            .call_method(&self.jni_object(), "notify", "()V", &[])?;
        Ok(())
    }
    pub fn notify_all(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.jni_ref()
            .call_method(&self.jni_object(), "notifyAll", "()V", &[])?;
        Ok(())
    }
}
pub struct BlockVector<'mc>(
    pub(crate) jni::JNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> crate::JNIRaw<'mc> for BlockVector<'mc> {
    fn jni_ref(&self) -> jni::JNIEnv<'mc> {
        unsafe { self.0.unsafe_clone() }
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> BlockVector<'mc> {
    pub fn from_raw(env: jni::JNIEnv<'mc>, obj: jni::objects::JObject<'mc>) -> Self {
        Self(env, obj)
    }
    pub fn deserialize_with_map(
        mut jni: jni::JNIEnv<'mc>,
        arg0: std::option::Option<std::collections::HashMap<String, jni::objects::JObject<'mc>>>,
    ) -> Result<crate::bukkit::util::Vector<'mc>, Box<dyn std::error::Error>> {
        let raw_val_0 = jni.new_object("java/util/HashMap", "()V", &[]).unwrap();
        for (k, v) in arg0.unwrap() {
            let map_val_0 = jni::objects::JObject::from(jni.new_string(k).unwrap());
            let map_val_1 = v;
            jni.call_method(
                &raw_val_0,
                "put",
                "(Ljava/Lang/ObjectLjava/Lang/Object)V",
                &[
                    jni::objects::JValueGen::from(&map_val_0),
                    jni::objects::JValueGen::from(&map_val_1),
                ],
            )?;
        }
        let val_0 = jni::objects::JValueGen::Object(raw_val_0);
        let cls = &jni.find_class("org/bukkit/util/Vector")?;
        let res = jni.call_static_method(
            cls,
            "deserialize",
            "(Ljava/util/Map;)Lorg/bukkit/util/Vector;",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        let ret = {
            let obj = res.l()?;
            crate::bukkit::util::Vector(jni, obj)
        };
        Ok(ret)
    }
    pub fn equals(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "equals",
            "(Ljava/lang/Object;)Z",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(res.z().unwrap())
    }
    pub fn hash_code(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hashCode", "()I", &[])?;
        Ok(res.i().unwrap())
    }
    pub fn clone(&mut self) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "clone", "()Ljava/lang/Object;", &[])?;
        Ok(res.l().unwrap())
    }
    pub fn is_normalized(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isNormalized", "()Z", &[])?;
        Ok(res.z().unwrap())
    }
    pub fn x(&mut self) -> Result<f64, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getX", "()D", &[])?;
        Ok(res.d().unwrap())
    }
    pub fn y(&mut self) -> Result<f64, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getY", "()D", &[])?;
        Ok(res.d().unwrap())
    }
    pub fn z(&mut self) -> Result<f64, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getZ", "()D", &[])?;
        Ok(res.d().unwrap())
    }
    pub fn block_x(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getBlockX", "()I", &[])?;
        Ok(res.i().unwrap())
    }
    pub fn block_y(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getBlockY", "()I", &[])?;
        Ok(res.i().unwrap())
    }
    pub fn block_z(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getBlockZ", "()I", &[])?;
        Ok(res.i().unwrap())
    }
    pub fn subtract(
        &mut self,
        arg0: crate::bukkit::util::Vector<'mc>,
    ) -> Result<crate::bukkit::util::Vector<'mc>, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "subtract",
            "(Lorg/bukkit/util/Vector;)Lorg/bukkit/util/Vector;",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        let ret = {
            crate::bukkit::util::Vector(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn length_squared(&mut self) -> Result<f64, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "lengthSquared", "()D", &[])?;
        Ok(res.d().unwrap())
    }
    pub fn distance_squared(
        &mut self,
        arg0: crate::bukkit::util::Vector<'mc>,
    ) -> Result<f64, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "distanceSquared",
            "(Lorg/bukkit/util/Vector;)D",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(res.d().unwrap())
    }
    pub fn check_finite(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.jni_ref()
            .call_method(&self.jni_object(), "checkFinite", "()V", &[])?;
        Ok(())
    }
    pub fn is_zero(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isZero", "()Z", &[])?;
        Ok(res.z().unwrap())
    }
    pub fn divide(
        &mut self,
        arg0: crate::bukkit::util::Vector<'mc>,
    ) -> Result<crate::bukkit::util::Vector<'mc>, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "divide",
            "(Lorg/bukkit/util/Vector;)Lorg/bukkit/util/Vector;",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        let ret = {
            crate::bukkit::util::Vector(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn angle(
        &mut self,
        arg0: crate::bukkit::util::Vector<'mc>,
    ) -> Result<f32, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "angle",
            "(Lorg/bukkit/util/Vector;)F",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(res.f().unwrap())
    }
    pub fn midpoint(
        &mut self,
        arg0: crate::bukkit::util::Vector<'mc>,
    ) -> Result<crate::bukkit::util::Vector<'mc>, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "midpoint",
            "(Lorg/bukkit/util/Vector;)Lorg/bukkit/util/Vector;",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        let ret = {
            crate::bukkit::util::Vector(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn get_midpoint(
        &mut self,
        arg0: crate::bukkit::util::Vector<'mc>,
    ) -> Result<crate::bukkit::util::Vector<'mc>, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getMidpoint",
            "(Lorg/bukkit/util/Vector;)Lorg/bukkit/util/Vector;",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        let ret = {
            crate::bukkit::util::Vector(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn cross_product(
        &mut self,
        arg0: crate::bukkit::util::Vector<'mc>,
    ) -> Result<crate::bukkit::util::Vector<'mc>, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "crossProduct",
            "(Lorg/bukkit/util/Vector;)Lorg/bukkit/util/Vector;",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        let ret = {
            crate::bukkit::util::Vector(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn get_cross_product(
        &mut self,
        arg0: crate::bukkit::util::Vector<'mc>,
    ) -> Result<crate::bukkit::util::Vector<'mc>, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getCrossProduct",
            "(Lorg/bukkit/util/Vector;)Lorg/bukkit/util/Vector;",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        let ret = {
            crate::bukkit::util::Vector(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn is_in_aabb(
        &mut self,
        arg0: crate::bukkit::util::Vector<'mc>,
        arg1: crate::bukkit::util::Vector<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.1.clone()) };
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg1.1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "isInAABB",
            "(Lorg/bukkit/util/Vector;Lorg/bukkit/util/Vector;)Z",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        )?;
        Ok(res.z().unwrap())
    }
    pub fn is_in_sphere(
        &mut self,
        arg0: crate::bukkit::util::Vector<'mc>,
        arg1: f64,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.1.clone()) };
        let val_1 = jni::objects::JValueGen::Double(arg1.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "isInSphere",
            "(Lorg/bukkit/util/Vector;D)Z",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        )?;
        Ok(res.z().unwrap())
    }
    pub fn epsilon(mut jni: jni::JNIEnv<'mc>) -> Result<f64, Box<dyn std::error::Error>> {
        let cls = &jni.find_class("double")?;
        let res = jni.call_static_method(cls, "getEpsilon", "()D", &[])?;
        Ok(res.d().unwrap())
    }
    pub fn rotate_around_x(
        &mut self,
        arg0: f64,
    ) -> Result<crate::bukkit::util::Vector<'mc>, Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Double(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "rotateAroundX",
            "(D)Lorg/bukkit/util/Vector;",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        let ret = {
            crate::bukkit::util::Vector(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn rotate_around_y(
        &mut self,
        arg0: f64,
    ) -> Result<crate::bukkit::util::Vector<'mc>, Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Double(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "rotateAroundY",
            "(D)Lorg/bukkit/util/Vector;",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        let ret = {
            crate::bukkit::util::Vector(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn rotate_around_z(
        &mut self,
        arg0: f64,
    ) -> Result<crate::bukkit::util::Vector<'mc>, Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Double(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "rotateAroundZ",
            "(D)Lorg/bukkit/util/Vector;",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        let ret = {
            crate::bukkit::util::Vector(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn rotate_around_axis(
        &mut self,
        arg0: crate::bukkit::util::Vector<'mc>,
        arg1: f64,
    ) -> Result<crate::bukkit::util::Vector<'mc>, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.1.clone()) };
        let val_1 = jni::objects::JValueGen::Double(arg1.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "rotateAroundAxis",
            "(Lorg/bukkit/util/Vector;D)Lorg/bukkit/util/Vector;",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        )?;
        let ret = {
            crate::bukkit::util::Vector(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn rotate_around_non_unit_axis(
        &mut self,
        arg0: crate::bukkit::util::Vector<'mc>,
        arg1: f64,
    ) -> Result<crate::bukkit::util::Vector<'mc>, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.1.clone()) };
        let val_1 = jni::objects::JValueGen::Double(arg1.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "rotateAroundNonUnitAxis",
            "(Lorg/bukkit/util/Vector;D)Lorg/bukkit/util/Vector;",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        )?;
        let ret = {
            crate::bukkit::util::Vector(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn to_location_with_world(
        &mut self,
        arg0: std::option::Option<crate::bukkit::World<'mc>>,
        arg1: std::option::Option<f32>,
        arg2: std::option::Option<f32>,
    ) -> Result<crate::bukkit::Location<'mc>, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.unwrap().1.clone()) };
        let val_1 = jni::objects::JValueGen::Float(arg1.unwrap().into());
        let val_2 = jni::objects::JValueGen::Float(arg2.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "toLocation",
            "(Lorg/bukkit/World;FF)Lorg/bukkit/Location;",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        )?;
        let ret = {
            crate::bukkit::Location(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn to_block_vector(
        &mut self,
    ) -> Result<crate::bukkit::util::BlockVector<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "toBlockVector",
            "()Lorg/bukkit/util/BlockVector;",
            &[],
        )?;
        let ret = {
            crate::bukkit::util::BlockVector(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn get_minimum(
        mut jni: jni::JNIEnv<'mc>,
        arg0: crate::bukkit::util::Vector<'mc>,
        arg1: crate::bukkit::util::Vector<'mc>,
    ) -> Result<crate::bukkit::util::Vector<'mc>, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.1.clone()) };
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg1.1.clone()) };
        let cls = &jni.find_class("org/bukkit/util/Vector")?;
        let res = jni.call_static_method(
            cls,
            "getMinimum",
            "(Lorg/bukkit/util/Vector;Lorg/bukkit/util/Vector;)Lorg/bukkit/util/Vector;",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        )?;
        let ret = {
            let obj = res.l()?;
            crate::bukkit::util::Vector(jni, obj)
        };
        Ok(ret)
    }
    pub fn get_maximum(
        mut jni: jni::JNIEnv<'mc>,
        arg0: crate::bukkit::util::Vector<'mc>,
        arg1: crate::bukkit::util::Vector<'mc>,
    ) -> Result<crate::bukkit::util::Vector<'mc>, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.1.clone()) };
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg1.1.clone()) };
        let cls = &jni.find_class("org/bukkit/util/Vector")?;
        let res = jni.call_static_method(
            cls,
            "getMaximum",
            "(Lorg/bukkit/util/Vector;Lorg/bukkit/util/Vector;)Lorg/bukkit/util/Vector;",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        )?;
        let ret = {
            let obj = res.l()?;
            crate::bukkit::util::Vector(jni, obj)
        };
        Ok(ret)
    }
    pub fn random(
        mut jni: jni::JNIEnv<'mc>,
    ) -> Result<crate::bukkit::util::Vector<'mc>, Box<dyn std::error::Error>> {
        let cls = &jni.find_class("org/bukkit/util/Vector")?;
        let res = jni.call_static_method(cls, "getRandom", "()Lorg/bukkit/util/Vector;", &[])?;
        let ret = {
            let obj = res.l()?;
            crate::bukkit::util::Vector(jni, obj)
        };
        Ok(ret)
    }
    pub fn add(
        &mut self,
        arg0: crate::bukkit::util::Vector<'mc>,
    ) -> Result<crate::bukkit::util::Vector<'mc>, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "add",
            "(Lorg/bukkit/util/Vector;)Lorg/bukkit/util/Vector;",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        let ret = {
            crate::bukkit::util::Vector(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn length(&mut self) -> Result<f64, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "length", "()D", &[])?;
        Ok(res.d().unwrap())
    }
    pub fn to_string(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "toString",
            "()Ljava/lang/String;",
            &[],
        )?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    pub fn dot(
        &mut self,
        arg0: crate::bukkit::util::Vector<'mc>,
    ) -> Result<f64, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "dot",
            "(Lorg/bukkit/util/Vector;)D",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(res.d().unwrap())
    }
    pub fn copy(
        &mut self,
        arg0: crate::bukkit::util::Vector<'mc>,
    ) -> Result<crate::bukkit::util::Vector<'mc>, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "copy",
            "(Lorg/bukkit/util/Vector;)Lorg/bukkit/util/Vector;",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        let ret = {
            crate::bukkit::util::Vector(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn normalize(
        &mut self,
    ) -> Result<crate::bukkit::util::Vector<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "normalize",
            "()Lorg/bukkit/util/Vector;",
            &[],
        )?;
        let ret = {
            crate::bukkit::util::Vector(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn zero(&mut self) -> Result<crate::bukkit::util::Vector<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "zero",
            "()Lorg/bukkit/util/Vector;",
            &[],
        )?;
        let ret = {
            crate::bukkit::util::Vector(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn distance(
        &mut self,
        arg0: crate::bukkit::util::Vector<'mc>,
    ) -> Result<f64, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "distance",
            "(Lorg/bukkit/util/Vector;)D",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(res.d().unwrap())
    }
    pub fn wait(
        &mut self,
        arg0: std::option::Option<i64>,
        arg1: std::option::Option<i32>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Long(arg0.unwrap().into());
        let val_1 = jni::objects::JValueGen::Int(arg1.unwrap().into());
        self.jni_ref().call_method(
            &self.jni_object(),
            "wait",
            "(JI)V",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        )?;
        Ok(())
    }
    pub fn class(&mut self) -> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getClass",
            "()Ljava/lang/Class;",
            &[],
        )?;
        Ok(unsafe { jni::objects::JClass::from_raw(res.as_jni().l) })
    }
    pub fn notify(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.jni_ref()
            .call_method(&self.jni_object(), "notify", "()V", &[])?;
        Ok(())
    }
    pub fn notify_all(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.jni_ref()
            .call_method(&self.jni_object(), "notifyAll", "()V", &[])?;
        Ok(())
    }
}
pub struct EulerAngle<'mc>(
    pub(crate) jni::JNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> crate::JNIRaw<'mc> for EulerAngle<'mc> {
    fn jni_ref(&self) -> jni::JNIEnv<'mc> {
        unsafe { self.0.unsafe_clone() }
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> EulerAngle<'mc> {
    pub fn from_raw(env: jni::JNIEnv<'mc>, obj: jni::objects::JObject<'mc>) -> Self {
        Self(env, obj)
    }
    pub fn x(&mut self) -> Result<f64, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getX", "()D", &[])?;
        Ok(res.d().unwrap())
    }
    pub fn y(&mut self) -> Result<f64, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getY", "()D", &[])?;
        Ok(res.d().unwrap())
    }
    pub fn z(&mut self) -> Result<f64, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getZ", "()D", &[])?;
        Ok(res.d().unwrap())
    }
    pub fn set_x(
        &mut self,
        arg0: f64,
    ) -> Result<crate::bukkit::util::EulerAngle<'mc>, Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Double(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setX",
            "(D)Lorg/bukkit/util/EulerAngle;",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        let ret = {
            crate::bukkit::util::EulerAngle(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn set_y(
        &mut self,
        arg0: f64,
    ) -> Result<crate::bukkit::util::EulerAngle<'mc>, Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Double(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setY",
            "(D)Lorg/bukkit/util/EulerAngle;",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        let ret = {
            crate::bukkit::util::EulerAngle(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn set_z(
        &mut self,
        arg0: f64,
    ) -> Result<crate::bukkit::util::EulerAngle<'mc>, Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Double(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setZ",
            "(D)Lorg/bukkit/util/EulerAngle;",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        let ret = {
            crate::bukkit::util::EulerAngle(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn subtract(
        &mut self,
        arg0: f64,
        arg1: f64,
        arg2: f64,
    ) -> Result<crate::bukkit::util::EulerAngle<'mc>, Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Double(arg0.into());
        let val_1 = jni::objects::JValueGen::Double(arg1.into());
        let val_2 = jni::objects::JValueGen::Double(arg2.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "subtract",
            "(DDD)Lorg/bukkit/util/EulerAngle;",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        )?;
        let ret = {
            crate::bukkit::util::EulerAngle(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn add(
        &mut self,
        arg0: f64,
        arg1: f64,
        arg2: f64,
    ) -> Result<crate::bukkit::util::EulerAngle<'mc>, Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Double(arg0.into());
        let val_1 = jni::objects::JValueGen::Double(arg1.into());
        let val_2 = jni::objects::JValueGen::Double(arg2.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "add",
            "(DDD)Lorg/bukkit/util/EulerAngle;",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        )?;
        let ret = {
            crate::bukkit::util::EulerAngle(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn equals(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "equals",
            "(Ljava/lang/Object;)Z",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(res.z().unwrap())
    }
    pub fn hash_code(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hashCode", "()I", &[])?;
        Ok(res.i().unwrap())
    }
    pub fn wait(
        &mut self,
        arg0: std::option::Option<i64>,
        arg1: std::option::Option<i32>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Long(arg0.unwrap().into());
        let val_1 = jni::objects::JValueGen::Int(arg1.unwrap().into());
        self.jni_ref().call_method(
            &self.jni_object(),
            "wait",
            "(JI)V",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        )?;
        Ok(())
    }
    pub fn to_string(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "toString",
            "()Ljava/lang/String;",
            &[],
        )?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    pub fn class(&mut self) -> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getClass",
            "()Ljava/lang/Class;",
            &[],
        )?;
        Ok(unsafe { jni::objects::JClass::from_raw(res.as_jni().l) })
    }
    pub fn notify(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.jni_ref()
            .call_method(&self.jni_object(), "notify", "()V", &[])?;
        Ok(())
    }
    pub fn notify_all(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.jni_ref()
            .call_method(&self.jni_object(), "notifyAll", "()V", &[])?;
        Ok(())
    }
}
pub struct StringUtil<'mc>(
    pub(crate) jni::JNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> crate::JNIRaw<'mc> for StringUtil<'mc> {
    fn jni_ref(&self) -> jni::JNIEnv<'mc> {
        unsafe { self.0.unsafe_clone() }
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> StringUtil<'mc> {
    pub fn from_raw(env: jni::JNIEnv<'mc>, obj: jni::objects::JObject<'mc>) -> Self {
        Self(env, obj)
    }
    pub fn starts_with_ignore_case(
        mut jni: jni::JNIEnv<'mc>,
        arg0: String,
        arg1: String,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JObject::from(jni.new_string(arg0).unwrap());
        let val_1 = jni::objects::JObject::from(jni.new_string(arg1).unwrap());
        let cls = &jni.find_class("boolean")?;
        let res = jni.call_static_method(
            cls,
            "startsWithIgnoreCase",
            "(Ljava/lang/String;Ljava/lang/String;)Z",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        )?;
        Ok(res.z().unwrap())
    }
    pub fn wait(
        &mut self,
        arg0: std::option::Option<i64>,
        arg1: std::option::Option<i32>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Long(arg0.unwrap().into());
        let val_1 = jni::objects::JValueGen::Int(arg1.unwrap().into());
        self.jni_ref().call_method(
            &self.jni_object(),
            "wait",
            "(JI)V",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        )?;
        Ok(())
    }
    pub fn equals(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "equals",
            "(Ljava/lang/Object;)Z",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(res.z().unwrap())
    }
    pub fn to_string(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "toString",
            "()Ljava/lang/String;",
            &[],
        )?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    pub fn hash_code(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hashCode", "()I", &[])?;
        Ok(res.i().unwrap())
    }
    pub fn class(&mut self) -> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getClass",
            "()Ljava/lang/Class;",
            &[],
        )?;
        Ok(unsafe { jni::objects::JClass::from_raw(res.as_jni().l) })
    }
    pub fn notify(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.jni_ref()
            .call_method(&self.jni_object(), "notify", "()V", &[])?;
        Ok(())
    }
    pub fn notify_all(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.jni_ref()
            .call_method(&self.jni_object(), "notifyAll", "()V", &[])?;
        Ok(())
    }
}
pub struct ChatPaginator<'mc>(
    pub(crate) jni::JNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> crate::JNIRaw<'mc> for ChatPaginator<'mc> {
    fn jni_ref(&self) -> jni::JNIEnv<'mc> {
        unsafe { self.0.unsafe_clone() }
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> ChatPaginator<'mc> {
    pub fn from_raw(env: jni::JNIEnv<'mc>, obj: jni::objects::JObject<'mc>) -> Self {
        Self(env, obj)
    }
    pub fn paginate_with_string(
        mut jni: jni::JNIEnv<'mc>,
        arg0: String,
        arg1: std::option::Option<i32>,
        arg2: std::option::Option<i32>,
        arg3: std::option::Option<i32>,
    ) -> Result<crate::bukkit::util::ChatPaginatorChatPage<'mc>, Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JObject::from(jni.new_string(arg0).unwrap());
        let val_1 = jni::objects::JValueGen::Int(arg1.unwrap().into());
        let val_2 = jni::objects::JValueGen::Int(arg2.unwrap().into());
        let val_3 = jni::objects::JValueGen::Int(arg3.unwrap().into());
        let cls = &jni.find_class("org/bukkit/util/ChatPaginator$ChatPage")?;
        let res = jni.call_static_method(
            cls,
            "paginate",
            "(Ljava/lang/String;III)Lorg/bukkit/util/ChatPaginator$ChatPage;",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
                jni::objects::JValueGen::from(&val_3),
            ],
        )?;
        let ret = {
            let obj = res.l()?;
            crate::bukkit::util::ChatPaginatorChatPage(jni, obj)
        };
        Ok(ret)
    }
    pub fn wait(
        &mut self,
        arg0: std::option::Option<i64>,
        arg1: std::option::Option<i32>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Long(arg0.unwrap().into());
        let val_1 = jni::objects::JValueGen::Int(arg1.unwrap().into());
        self.jni_ref().call_method(
            &self.jni_object(),
            "wait",
            "(JI)V",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        )?;
        Ok(())
    }
    pub fn equals(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "equals",
            "(Ljava/lang/Object;)Z",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(res.z().unwrap())
    }
    pub fn to_string(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "toString",
            "()Ljava/lang/String;",
            &[],
        )?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    pub fn hash_code(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hashCode", "()I", &[])?;
        Ok(res.i().unwrap())
    }
    pub fn class(&mut self) -> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getClass",
            "()Ljava/lang/Class;",
            &[],
        )?;
        Ok(unsafe { jni::objects::JClass::from_raw(res.as_jni().l) })
    }
    pub fn notify(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.jni_ref()
            .call_method(&self.jni_object(), "notify", "()V", &[])?;
        Ok(())
    }
    pub fn notify_all(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.jni_ref()
            .call_method(&self.jni_object(), "notifyAll", "()V", &[])?;
        Ok(())
    }
}
pub struct Vector<'mc>(
    pub(crate) jni::JNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> crate::JNIRaw<'mc> for Vector<'mc> {
    fn jni_ref(&self) -> jni::JNIEnv<'mc> {
        unsafe { self.0.unsafe_clone() }
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Vector<'mc> {
    pub fn from_raw(env: jni::JNIEnv<'mc>, obj: jni::objects::JObject<'mc>) -> Self {
        Self(env, obj)
    }
    pub fn is_normalized(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isNormalized", "()Z", &[])?;
        Ok(res.z().unwrap())
    }
    pub fn x(&mut self) -> Result<f64, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getX", "()D", &[])?;
        Ok(res.d().unwrap())
    }
    pub fn y(&mut self) -> Result<f64, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getY", "()D", &[])?;
        Ok(res.d().unwrap())
    }
    pub fn z(&mut self) -> Result<f64, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getZ", "()D", &[])?;
        Ok(res.d().unwrap())
    }
    pub fn block_x(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getBlockX", "()I", &[])?;
        Ok(res.i().unwrap())
    }
    pub fn block_y(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getBlockY", "()I", &[])?;
        Ok(res.i().unwrap())
    }
    pub fn block_z(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getBlockZ", "()I", &[])?;
        Ok(res.i().unwrap())
    }
    pub fn subtract(
        &mut self,
        arg0: crate::bukkit::util::Vector<'mc>,
    ) -> Result<crate::bukkit::util::Vector<'mc>, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "subtract",
            "(Lorg/bukkit/util/Vector;)Lorg/bukkit/util/Vector;",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        let ret = {
            crate::bukkit::util::Vector(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn length_squared(&mut self) -> Result<f64, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "lengthSquared", "()D", &[])?;
        Ok(res.d().unwrap())
    }
    pub fn distance_squared(
        &mut self,
        arg0: crate::bukkit::util::Vector<'mc>,
    ) -> Result<f64, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "distanceSquared",
            "(Lorg/bukkit/util/Vector;)D",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(res.d().unwrap())
    }
    pub fn check_finite(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.jni_ref()
            .call_method(&self.jni_object(), "checkFinite", "()V", &[])?;
        Ok(())
    }
    pub fn deserialize(
        mut jni: jni::JNIEnv<'mc>,
        arg0: std::collections::HashMap<String, jni::objects::JObject<'mc>>,
    ) -> Result<crate::bukkit::util::Vector<'mc>, Box<dyn std::error::Error>> {
        let raw_val_0 = jni.new_object("java/util/HashMap", "()V", &[]).unwrap();
        for (k, v) in arg0 {
            let map_val_0 = jni::objects::JObject::from(jni.new_string(k).unwrap());
            let map_val_1 = v;
            jni.call_method(
                &raw_val_0,
                "put",
                "(Ljava/Lang/ObjectLjava/Lang/Object)V",
                &[
                    jni::objects::JValueGen::from(&map_val_0),
                    jni::objects::JValueGen::from(&map_val_1),
                ],
            )?;
        }
        let val_0 = jni::objects::JValueGen::Object(raw_val_0);
        let cls = &jni.find_class("org/bukkit/util/Vector")?;
        let res = jni.call_static_method(
            cls,
            "deserialize",
            "(Ljava/util/Map;)Lorg/bukkit/util/Vector;",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        let ret = {
            let obj = res.l()?;
            crate::bukkit::util::Vector(jni, obj)
        };
        Ok(ret)
    }
    pub fn is_zero(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isZero", "()Z", &[])?;
        Ok(res.z().unwrap())
    }
    pub fn divide(
        &mut self,
        arg0: crate::bukkit::util::Vector<'mc>,
    ) -> Result<crate::bukkit::util::Vector<'mc>, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "divide",
            "(Lorg/bukkit/util/Vector;)Lorg/bukkit/util/Vector;",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        let ret = {
            crate::bukkit::util::Vector(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn angle(
        &mut self,
        arg0: crate::bukkit::util::Vector<'mc>,
    ) -> Result<f32, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "angle",
            "(Lorg/bukkit/util/Vector;)F",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(res.f().unwrap())
    }
    pub fn midpoint(
        &mut self,
        arg0: crate::bukkit::util::Vector<'mc>,
    ) -> Result<crate::bukkit::util::Vector<'mc>, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "midpoint",
            "(Lorg/bukkit/util/Vector;)Lorg/bukkit/util/Vector;",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        let ret = {
            crate::bukkit::util::Vector(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn get_midpoint(
        &mut self,
        arg0: crate::bukkit::util::Vector<'mc>,
    ) -> Result<crate::bukkit::util::Vector<'mc>, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getMidpoint",
            "(Lorg/bukkit/util/Vector;)Lorg/bukkit/util/Vector;",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        let ret = {
            crate::bukkit::util::Vector(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn cross_product(
        &mut self,
        arg0: crate::bukkit::util::Vector<'mc>,
    ) -> Result<crate::bukkit::util::Vector<'mc>, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "crossProduct",
            "(Lorg/bukkit/util/Vector;)Lorg/bukkit/util/Vector;",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        let ret = {
            crate::bukkit::util::Vector(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn get_cross_product(
        &mut self,
        arg0: crate::bukkit::util::Vector<'mc>,
    ) -> Result<crate::bukkit::util::Vector<'mc>, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getCrossProduct",
            "(Lorg/bukkit/util/Vector;)Lorg/bukkit/util/Vector;",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        let ret = {
            crate::bukkit::util::Vector(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn is_in_aabb(
        &mut self,
        arg0: crate::bukkit::util::Vector<'mc>,
        arg1: crate::bukkit::util::Vector<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.1.clone()) };
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg1.1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "isInAABB",
            "(Lorg/bukkit/util/Vector;Lorg/bukkit/util/Vector;)Z",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        )?;
        Ok(res.z().unwrap())
    }
    pub fn is_in_sphere(
        &mut self,
        arg0: crate::bukkit::util::Vector<'mc>,
        arg1: f64,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.1.clone()) };
        let val_1 = jni::objects::JValueGen::Double(arg1.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "isInSphere",
            "(Lorg/bukkit/util/Vector;D)Z",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        )?;
        Ok(res.z().unwrap())
    }
    pub fn epsilon(mut jni: jni::JNIEnv<'mc>) -> Result<f64, Box<dyn std::error::Error>> {
        let cls = &jni.find_class("double")?;
        let res = jni.call_static_method(cls, "getEpsilon", "()D", &[])?;
        Ok(res.d().unwrap())
    }
    pub fn rotate_around_x(
        &mut self,
        arg0: f64,
    ) -> Result<crate::bukkit::util::Vector<'mc>, Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Double(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "rotateAroundX",
            "(D)Lorg/bukkit/util/Vector;",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        let ret = {
            crate::bukkit::util::Vector(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn rotate_around_y(
        &mut self,
        arg0: f64,
    ) -> Result<crate::bukkit::util::Vector<'mc>, Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Double(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "rotateAroundY",
            "(D)Lorg/bukkit/util/Vector;",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        let ret = {
            crate::bukkit::util::Vector(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn rotate_around_z(
        &mut self,
        arg0: f64,
    ) -> Result<crate::bukkit::util::Vector<'mc>, Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Double(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "rotateAroundZ",
            "(D)Lorg/bukkit/util/Vector;",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        let ret = {
            crate::bukkit::util::Vector(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn rotate_around_axis(
        &mut self,
        arg0: crate::bukkit::util::Vector<'mc>,
        arg1: f64,
    ) -> Result<crate::bukkit::util::Vector<'mc>, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.1.clone()) };
        let val_1 = jni::objects::JValueGen::Double(arg1.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "rotateAroundAxis",
            "(Lorg/bukkit/util/Vector;D)Lorg/bukkit/util/Vector;",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        )?;
        let ret = {
            crate::bukkit::util::Vector(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn rotate_around_non_unit_axis(
        &mut self,
        arg0: crate::bukkit::util::Vector<'mc>,
        arg1: f64,
    ) -> Result<crate::bukkit::util::Vector<'mc>, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.1.clone()) };
        let val_1 = jni::objects::JValueGen::Double(arg1.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "rotateAroundNonUnitAxis",
            "(Lorg/bukkit/util/Vector;D)Lorg/bukkit/util/Vector;",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        )?;
        let ret = {
            crate::bukkit::util::Vector(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn to_location_with_world(
        &mut self,
        arg0: std::option::Option<crate::bukkit::World<'mc>>,
        arg1: std::option::Option<f32>,
        arg2: std::option::Option<f32>,
    ) -> Result<crate::bukkit::Location<'mc>, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.unwrap().1.clone()) };
        let val_1 = jni::objects::JValueGen::Float(arg1.unwrap().into());
        let val_2 = jni::objects::JValueGen::Float(arg2.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "toLocation",
            "(Lorg/bukkit/World;FF)Lorg/bukkit/Location;",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        )?;
        let ret = {
            crate::bukkit::Location(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn to_block_vector(
        &mut self,
    ) -> Result<crate::bukkit::util::BlockVector<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "toBlockVector",
            "()Lorg/bukkit/util/BlockVector;",
            &[],
        )?;
        let ret = {
            crate::bukkit::util::BlockVector(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn get_minimum(
        mut jni: jni::JNIEnv<'mc>,
        arg0: crate::bukkit::util::Vector<'mc>,
        arg1: crate::bukkit::util::Vector<'mc>,
    ) -> Result<crate::bukkit::util::Vector<'mc>, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.1.clone()) };
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg1.1.clone()) };
        let cls = &jni.find_class("org/bukkit/util/Vector")?;
        let res = jni.call_static_method(
            cls,
            "getMinimum",
            "(Lorg/bukkit/util/Vector;Lorg/bukkit/util/Vector;)Lorg/bukkit/util/Vector;",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        )?;
        let ret = {
            let obj = res.l()?;
            crate::bukkit::util::Vector(jni, obj)
        };
        Ok(ret)
    }
    pub fn get_maximum(
        mut jni: jni::JNIEnv<'mc>,
        arg0: crate::bukkit::util::Vector<'mc>,
        arg1: crate::bukkit::util::Vector<'mc>,
    ) -> Result<crate::bukkit::util::Vector<'mc>, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.1.clone()) };
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg1.1.clone()) };
        let cls = &jni.find_class("org/bukkit/util/Vector")?;
        let res = jni.call_static_method(
            cls,
            "getMaximum",
            "(Lorg/bukkit/util/Vector;Lorg/bukkit/util/Vector;)Lorg/bukkit/util/Vector;",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        )?;
        let ret = {
            let obj = res.l()?;
            crate::bukkit::util::Vector(jni, obj)
        };
        Ok(ret)
    }
    pub fn random(
        mut jni: jni::JNIEnv<'mc>,
    ) -> Result<crate::bukkit::util::Vector<'mc>, Box<dyn std::error::Error>> {
        let cls = &jni.find_class("org/bukkit/util/Vector")?;
        let res = jni.call_static_method(cls, "getRandom", "()Lorg/bukkit/util/Vector;", &[])?;
        let ret = {
            let obj = res.l()?;
            crate::bukkit::util::Vector(jni, obj)
        };
        Ok(ret)
    }
    pub fn add(
        &mut self,
        arg0: crate::bukkit::util::Vector<'mc>,
    ) -> Result<crate::bukkit::util::Vector<'mc>, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "add",
            "(Lorg/bukkit/util/Vector;)Lorg/bukkit/util/Vector;",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        let ret = {
            crate::bukkit::util::Vector(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn equals(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "equals",
            "(Ljava/lang/Object;)Z",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(res.z().unwrap())
    }
    pub fn length(&mut self) -> Result<f64, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "length", "()D", &[])?;
        Ok(res.d().unwrap())
    }
    pub fn to_string(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "toString",
            "()Ljava/lang/String;",
            &[],
        )?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    pub fn hash_code(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hashCode", "()I", &[])?;
        Ok(res.i().unwrap())
    }
    pub fn clone(
        &mut self,
    ) -> Result<crate::bukkit::util::Vector<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "clone",
            "()Lorg/bukkit/util/Vector;",
            &[],
        )?;
        let ret = {
            crate::bukkit::util::Vector(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn dot(
        &mut self,
        arg0: crate::bukkit::util::Vector<'mc>,
    ) -> Result<f64, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "dot",
            "(Lorg/bukkit/util/Vector;)D",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(res.d().unwrap())
    }
    pub fn copy(
        &mut self,
        arg0: crate::bukkit::util::Vector<'mc>,
    ) -> Result<crate::bukkit::util::Vector<'mc>, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "copy",
            "(Lorg/bukkit/util/Vector;)Lorg/bukkit/util/Vector;",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        let ret = {
            crate::bukkit::util::Vector(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn normalize(
        &mut self,
    ) -> Result<crate::bukkit::util::Vector<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "normalize",
            "()Lorg/bukkit/util/Vector;",
            &[],
        )?;
        let ret = {
            crate::bukkit::util::Vector(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn zero(&mut self) -> Result<crate::bukkit::util::Vector<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "zero",
            "()Lorg/bukkit/util/Vector;",
            &[],
        )?;
        let ret = {
            crate::bukkit::util::Vector(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn distance(
        &mut self,
        arg0: crate::bukkit::util::Vector<'mc>,
    ) -> Result<f64, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "distance",
            "(Lorg/bukkit/util/Vector;)D",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(res.d().unwrap())
    }
    pub fn wait(
        &mut self,
        arg0: std::option::Option<i64>,
        arg1: std::option::Option<i32>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Long(arg0.unwrap().into());
        let val_1 = jni::objects::JValueGen::Int(arg1.unwrap().into());
        self.jni_ref().call_method(
            &self.jni_object(),
            "wait",
            "(JI)V",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        )?;
        Ok(())
    }
    pub fn class(&mut self) -> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getClass",
            "()Ljava/lang/Class;",
            &[],
        )?;
        Ok(unsafe { jni::objects::JClass::from_raw(res.as_jni().l) })
    }
    pub fn notify(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.jni_ref()
            .call_method(&self.jni_object(), "notify", "()V", &[])?;
        Ok(())
    }
    pub fn notify_all(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.jni_ref()
            .call_method(&self.jni_object(), "notifyAll", "()V", &[])?;
        Ok(())
    }
}
pub struct FileUtil<'mc>(
    pub(crate) jni::JNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> crate::JNIRaw<'mc> for FileUtil<'mc> {
    fn jni_ref(&self) -> jni::JNIEnv<'mc> {
        unsafe { self.0.unsafe_clone() }
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> FileUtil<'mc> {
    pub fn from_raw(env: jni::JNIEnv<'mc>, obj: jni::objects::JObject<'mc>) -> Self {
        Self(env, obj)
    }
    pub fn wait(
        &mut self,
        arg0: std::option::Option<i64>,
        arg1: std::option::Option<i32>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Long(arg0.unwrap().into());
        let val_1 = jni::objects::JValueGen::Int(arg1.unwrap().into());
        self.jni_ref().call_method(
            &self.jni_object(),
            "wait",
            "(JI)V",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        )?;
        Ok(())
    }
    pub fn equals(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "equals",
            "(Ljava/lang/Object;)Z",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(res.z().unwrap())
    }
    pub fn to_string(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "toString",
            "()Ljava/lang/String;",
            &[],
        )?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    pub fn hash_code(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hashCode", "()I", &[])?;
        Ok(res.i().unwrap())
    }
    pub fn class(&mut self) -> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getClass",
            "()Ljava/lang/Class;",
            &[],
        )?;
        Ok(unsafe { jni::objects::JClass::from_raw(res.as_jni().l) })
    }
    pub fn notify(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.jni_ref()
            .call_method(&self.jni_object(), "notify", "()V", &[])?;
        Ok(())
    }
    pub fn notify_all(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.jni_ref()
            .call_method(&self.jni_object(), "notifyAll", "()V", &[])?;
        Ok(())
    }
}
pub mod noise;
pub mod permissions;
